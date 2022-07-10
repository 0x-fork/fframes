use rayon::prelude::*;
use std::{ops::Range, path::PathBuf, sync::Arc};

use fframes::{fframes_context, frame, video::Video};
use uuid::Uuid;

use crate::{
    concatenator,
    encoder::{Encoder, EncoderFrame, EncoderOptions},
    fframes_logger::FFramesLogger,
    gpu::GpuRenderingBackend,
    renderer_error::{FFramesError, FFramesResult},
    RenderOptions,
};

pub trait FFramesRenderBackend {
    fn render<'a, TVideo: Video + Sync + Sized>(
        &self,
        output: &'a str,
        video: TVideo,
        logger: Arc<dyn FFramesLogger>,
        usvg_options: &usvg::OptionsRef,
        duration_in_frames: usize,
        render_options: EncoderOptions<'a>,
        ctx: fframes::FFramesContext,
    ) -> FFramesResult<()>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum RenderBackendVariant {
    Gpu,
    Cpu,
}

impl RenderBackendVariant {
    pub fn make_backend(&self) -> impl FFramesRenderBackend {
        match self {
            RenderBackendVariant::Gpu => GpuRenderingBackend {},
            RenderBackendVariant::Cpu => GpuRenderingBackend {},
        }
    }
}

impl Default for RenderBackendVariant {
    fn default() -> Self {
        RenderBackendVariant::Cpu
    }
}

#[derive(Default)]
pub struct CpuRenderingBackend {}

fn split_ffmpeg_chunks(duration_in_frames: usize, chunk_size: usize) -> Vec<Range<usize>> {
    let mut chunks = vec![];
    let mut prev_chunk = 0;

    while prev_chunk < duration_in_frames {
        if duration_in_frames - prev_chunk > chunk_size {
            chunks.push(prev_chunk..prev_chunk + chunk_size);
            prev_chunk += chunk_size;
        } else {
            let last_chunk = duration_in_frames - prev_chunk;
            chunks.push(prev_chunk..prev_chunk + last_chunk);
            prev_chunk += last_chunk;
        }
    }

    chunks
}

pub fn divide_round_up(a: usize, b: usize) -> usize {
    (a + (b - 1)) / b
}

impl FFramesRenderBackend for CpuRenderingBackend {
    fn render<'a, TVideo: Video + Sync + Sized>(
        &self,
        output: &'a str,
        video: TVideo,
        logger: Arc<dyn FFramesLogger>,
        usvg_options: &usvg::OptionsRef,
        duration_in_frames: usize,
        encoder_options: EncoderOptions<'a>,
        ctx: fframes::FFramesContext,
    ) -> FFramesResult<()> {
        let session = Uuid::new_v4();
        // let directory = std::env::temp_dir().join(format!("fframes-{session}"));
        let directory = PathBuf::from("/Users/dmtrkovalenko/dev/fframes/bin/some");
        // std::fs::create_dir(&directory)?;

        let files = split_ffmpeg_chunks(
            duration_in_frames,
            divide_round_up(duration_in_frames, rayon::current_num_threads()),
        )
        .par_iter()
        .enumerate()
        .map(|(thread_number, chunk_range)| {
            let file = directory
                .join(format!("{thread_number}.mp4"))
                .into_os_string()
                .into_string()
                .unwrap();

            unsafe {
                Encoder::with_output(
                    TVideo::WIDTH as i32,
                    TVideo::HEIGHT as i32,
                    TVideo::FPS as i32,
                    file.as_str(),
                    "libx264",
                    &mut |encoder| {
                        let mut last_svg = "".to_owned();
                        let mut frame = EncoderFrame::make(&encoder.video_stream);
                        let mut pixmap =
                            tiny_skia::Pixmap::new(TVideo::WIDTH as u32, TVideo::HEIGHT as u32)
                                .unwrap();

                        chunk_range
                            .to_owned()
                            .into_iter()
                            .enumerate()
                            .try_for_each(|(index, fr)| {
                                let svg = video.render_frame(
                                    &frame::Frame {
                                        fps: TVideo::FPS,
                                        index: fr as i64,
                                    },
                                    &ctx,
                                );

                                // pixmap.save_png(directory.join(format!("{index}.png")));

                                logger.log_frame(index, thread_number, &svg);
                                if svg != last_svg {
                                    let rtree = usvg::Tree::from_str(&svg, usvg_options).unwrap();
                                    resvg::render(
                                        &rtree,
                                        usvg::FitTo::Original,
                                        tiny_skia::Transform::default(),
                                        pixmap.as_mut(),
                                    )
                                    .unwrap();

                                    last_svg = svg;
                                }

                                encoder
                                    .send_frame(frame.from_rgba_pixmap(index as i64, pixmap.data()))
                            })?;

                        let frames_to_generate = chunk_range.end - chunk_range.start;
                        let submitted_frames = encoder.video_stream.get_frames_in_stream() as usize;

                        if submitted_frames < frames_to_generate {
                            let intra_frames_to_add = frames_to_generate - submitted_frames;

                            for _ in chunk_range.end..chunk_range.end + intra_frames_to_add {
                                encoder.send_frame(frame.frame)?;
                            }
                        }

                        frame.free();
                        Ok(())
                    },
                )
            }
            .and_then(std::convert::identity)
            .map_err(|av_err| FFramesError::RenderChunkError(thread_number, av_err))?;

            Ok(file)
        })
        .collect::<FFramesResult<Vec<_>>>()?;

        unsafe {
            concatenator::concat_video_files_with_audio(files.as_slice(), output, &ctx)?;
        }

        logger.success(output, directory.to_str());
        Ok(())
    }
}
