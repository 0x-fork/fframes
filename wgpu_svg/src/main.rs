use clap::*;
use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation::geometry_builder::*;
use lyon::tessellation::{self, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator};
use usvg::prelude::*;
use wgpu::include_wgsl;
// use winit::dpi::PhysicalSize;
// use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop};
// use winit::window::{Window, WindowBuilder};
use std::num::NonZeroU32;

use futures::executor::block_on;

use wgpu::util::DeviceExt;

use std::f64::NAN;

const WINDOW_SIZE: f32 = 1024.0;
pub const FALLBACK_COLOR: usvg::Color = usvg::Color {
    red: 0,
    green: 0,
    blue: 0,
};

// This example renders a very tiny subset of SVG (only filled and stroke paths with solid color
// patterns and transforms).
//
// Parsing is done via the usvg crate. In this very simple example, paths are all tessellated directly
// into a static mesh during parsing.
// vertices embed a primitive ID which lets the vertex shader fetch the per-path information such like
// the color from uniform buffer objects.
// No occlusion culling optimization here (see the wgpu example).
//
// Most of the code in this example is related to working with the GPU.

fn main() {
    // Grab some parameters from the command line.

    env_logger::init();
    let texture_size = 1024u32;

    let app = App::new("Lyon svg_render example")
        .version("0.1")
        .arg(
            Arg::with_name("MSAA")
                .long("msaa")
                .short("m")
                .help("Enable msaa")
                .value_name("SAMPLES")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("SVG or SVGZ file")
                .value_name("INPUT")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("TESS_ONLY")
                .help("Perform the tessellation and exit without rendering")
                .value_name("TESS_ONLY")
                .long("tessellate-only")
                .short("t")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

    let msaa_samples = if app.is_present("MSAA") { 4 } else { 4 };

    // Parse and tessellate the geometry

    let filename = app.value_of("INPUT").unwrap();

    let mut fill_tess = FillTessellator::new();
    let mut stroke_tess = StrokeTessellator::new();
    let mut mesh: VertexBuffers<_, u32> = VertexBuffers::new();
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();

    let file_data = std::fs::read(filename).unwrap();
    let rtree = usvg::Tree::from_data(&file_data, &opt).unwrap();
    let mut transforms = Vec::new();
    let mut primitives = Vec::new();

    let mut prev_transform = usvg::Transform {
        a: NAN,
        b: NAN,
        c: NAN,
        d: NAN,
        e: NAN,
        f: NAN,
    };
    let view_box = rtree.svg_node().view_box;
    for node in rtree.root().descendants() {
        if let usvg::NodeKind::Path(ref p) = *node.borrow() {
            let t = node.transform();
            if t != prev_transform {
                transforms.push(GpuTransform {
                    data0: [t.a as f32, t.b as f32, t.c as f32, t.d as f32],
                    data1: [t.e as f32, t.f as f32, 0.0, 0.0],
                });
            }
            prev_transform = t;

            let transform_idx = transforms.len() as u32 - 1;

            if let Some(ref fill) = p.fill {
                // fall back to always use color fill
                // no gradients (yet?)
                let color = match fill.paint {
                    usvg::Paint::Color(c) => c,
                    _ => FALLBACK_COLOR,
                };

                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    color,
                    fill.opacity.value() as f32,
                ));

                fill_tess
                    .tessellate(
                        convert_path(p),
                        &FillOptions::tolerance(0.01),
                        &mut BuffersBuilder::new(
                            &mut mesh,
                            VertexCtor {
                                prim_id: primitives.len() as u32 - 1,
                            },
                        ),
                    )
                    .expect("Error during tesselation!");
            }

            if let Some(ref stroke) = p.stroke {
                let (stroke_color, stroke_opts) = convert_stroke(stroke);
                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    stroke_color,
                    stroke.opacity.value() as f32,
                ));

                let _ = stroke_tess.tessellate(
                    convert_path(p),
                    &stroke_opts.with_tolerance(0.01),
                    &mut BuffersBuilder::new(
                        &mut mesh,
                        VertexCtor {
                            prim_id: primitives.len() as u32 - 1,
                        },
                    ),
                );
            }
        }
    }

    if app.is_present("TESS_ONLY") {
        return;
    }

    println!(
        "Finished tesselation: {} vertices, {} indices",
        mesh.vertices.len(),
        mesh.indices.len()
    );

    println!("Use arrow keys to pan, pageup and pagedown to zoom.");

    // Initialize wgpu and send some data to the GPU.

    let vb_width = view_box.rect.size().width() as f32;
    let vb_height = view_box.rect.size().height() as f32;
    let scale = vb_width / vb_height;

    let (width, height) = if scale < 1.0 {
        (WINDOW_SIZE, WINDOW_SIZE * scale)
    } else {
        (WINDOW_SIZE, WINDOW_SIZE / scale)
    };

    let pan = [vb_width / -2.0, vb_height / -2.0];
    let zoom = 2.0 / f32::max(vb_width, vb_height);
    let mut z = SceneGlobals {
        zoom,
        pan,
        wireframe: false,
        size_changed: true,
        render: false,
    };

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

    // create an adapter
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();

    // create a device and a queue
    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
        },
        // trace_path can be used for API call tracing
        None,
    ))
    .unwrap();

    let mut surface_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: width as u32,
        height: width as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&mesh.vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&mesh.indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let prim_buffer_byte_size = (primitives.len() * std::mem::size_of::<GpuPrimitive>()) as u64;
    let transform_buffer_byte_size =
        (transforms.len() * std::mem::size_of::<GpuTransform>()) as u64;
    let globals_buffer_byte_size = std::mem::size_of::<GpuGlobals>() as u64;

    let prims_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Prims ssbo"),
        size: prim_buffer_byte_size,
        usage: wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let transforms_ssbo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Transforms ssbo"),
        size: transform_buffer_byte_size,
        usage: wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let globals_ubo = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Globals ubo"),
        size: globals_buffer_byte_size,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let vs_module = device.create_shader_module(&include_wgsl!("../shaders/geometry.vs.wgsl"));
    let fs_module = device.create_shader_module(&include_wgsl!("../shaders/geometry.fs.wgsl"));
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(prim_buffer_byte_size),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(transform_buffer_byte_size),
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(globals_ubo.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(prims_ssbo.as_entire_buffer_binding()),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Buffer(transforms_ssbo.as_entire_buffer_binding()),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
        label: None,
    });

    let mut render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vs_module,
            entry_point: "main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<GpuVertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        format: wgpu::VertexFormat::Float32x2,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        offset: 8,
                        format: wgpu::VertexFormat::Uint32,
                        shader_location: 1,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &fs_module,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            polygon_mode: wgpu::PolygonMode::Fill,
            front_face: wgpu::FrontFace::Ccw,
            strip_index_format: None,
            cull_mode: None,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: msaa_samples,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    };

    let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    // TODO: this isn't what we want: we'd need the equivalent of VK_POLYGON_MODE_LINE,
    // but it doesn't seem to be exposed by wgpu?
    render_pipeline_descriptor.primitive.topology = wgpu::PrimitiveTopology::LineList;
    let wireframe_render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    queue.write_buffer(&transforms_ssbo, 0, bytemuck::cast_slice(&transforms));

    queue.write_buffer(&prims_ssbo, 0, bytemuck::cast_slice(&primitives));

    // The main loop.
    scene.size_changed = false;
    surface_desc.width = width as u32;
    surface_desc.height = width as u32;

    let texture_desc = wgpu::TextureDescriptor {
        label: Some("Multisampled frame descriptor"),
        size: wgpu::Extent3d {
            width: surface_desc.width,
            height: surface_desc.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: msaa_samples,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    let texture = device.create_texture(&texture_desc);
    let texture_view = texture.create_view(&Default::default());
    let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Multisampled frame descriptor"),
        size: wgpu::Extent3d {
            width: surface_desc.width,
            height: surface_desc.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
    });

    let msaa_texture_view = msaa_texture.create_view(&Default::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Encoder"),
    });

    queue.write_buffer(
        &globals_ubo,
        0,
        bytemuck::cast_slice(&[GpuGlobals {
            aspect_ratio: 1.,
            zoom: [scene.zoom, scene.zoom],
            pan: scene.pan,
            _pad: 0.0,
        }]),
    );

    let u32_size = std::mem::size_of::<u32>() as u32;

    let output_buffer_size = (u32_size * texture_size * texture_size) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsages::COPY_DST
            // this tells wpgu that we want to read this buffer from the cpu
            | wgpu::BufferUsages::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    let output_buffer = device.create_buffer(&output_buffer_desc);

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &texture_view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: false,
                },
                resolve_target: Some(&msaa_texture_view),
            }],
            depth_stencil_attachment: None,
        });

        if scene.wireframe {
            pass.set_pipeline(&wireframe_render_pipeline);
        } else {
            pass.set_pipeline(&render_pipeline);
        }
        pass.set_bind_group(0, &bind_group, &[]);
        pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.set_vertex_buffer(0, vbo.slice(..));

        pass.draw_indexed(0..(mesh.indices.len() as u32), 0, 0..1);
    }

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &msaa_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(u32_size * texture_size),
                rows_per_image: NonZeroU32::new(texture_size),
            },
        },
        texture_desc.size,
    );

    queue.submit(Some(encoder.finish()));

    // We need to scope the mapping variables so that we can
    // unmap the buffer
    {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
        device.poll(wgpu::Maintain::Wait);
        block_on(mapping).unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer =
            ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, data).unwrap();
        buffer.save("image.png").unwrap();
    }
    output_buffer.unmap();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub prim_id: u32,
}

// A 2x3 matrix (last two members of data1 unused).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuTransform {
    pub data0: [f32; 4],
    pub data1: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuPrimitive {
    pub transform: u32,
    pub color: u32,
    pub _pad: [u32; 2],
}

impl GpuPrimitive {
    pub fn new(transform_idx: u32, color: usvg::Color, alpha: f32) -> Self {
        GpuPrimitive {
            transform: transform_idx,
            color: ((color.red as u32) << 24)
                + ((color.green as u32) << 16)
                + ((color.blue as u32) << 8)
                + (alpha * 255.0) as u32,
            _pad: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

pub struct VertexCtor {
    pub prim_id: u32,
}

impl FillVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

// Default scene has all values set to zero
#[derive(Copy, Clone, Debug)]
pub struct SceneGlobals {
    pub zoom: f32,
    pub pan: [f32; 2],
    pub wireframe: bool,
    pub size_changed: bool,
    pub render: bool,
}

/// Some glue between usvg's iterators and lyon's.

fn point(x: &f64, y: &f64) -> Point {
    Point::new((*x) as f32, (*y) as f32)
}

pub struct PathConvIter<'a> {
    iter: std::slice::Iter<'a, usvg::PathSegment>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(usvg::PathSegment::MoveTo { x, y }) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = point(x, y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = point(x, y);
                    self.needs_end = true;
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(usvg::PathSegment::LineTo { x, y }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            }) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = point(x, y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1: point(x1, y1),
                    ctrl2: point(x2, y2),
                    to: self.prev,
                })
            }
            Some(usvg::PathSegment::ClosePath) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data.iter(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

pub fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint {
        usvg::Paint::Color(c) => c,
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap {
        usvg::LineCap::Butt => tessellation::LineCap::Butt,
        usvg::LineCap::Square => tessellation::LineCap::Square,
        usvg::LineCap::Round => tessellation::LineCap::Round,
    };
    let linejoin = match s.linejoin {
        usvg::LineJoin::Miter => tessellation::LineJoin::Miter,
        usvg::LineJoin::Bevel => tessellation::LineJoin::Bevel,
        usvg::LineJoin::Round => tessellation::LineJoin::Round,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width.value() as f32)
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}
unsafe impl bytemuck::Pod for GpuPrimitive {}
unsafe impl bytemuck::Zeroable for GpuPrimitive {}
unsafe impl bytemuck::Pod for GpuTransform {}
unsafe impl bytemuck::Zeroable for GpuTransform {}
