mod encoder;
use fframes::Duration;
use fframes::{fframes_context, frame, video::Video};
use rayon::prelude::*;
use renderer_error::FFramesError;
use std::ops::Range;
use uuid::Uuid;

use crate::encoder::{Encoder, EncoderFrame};
use crate::renderer_error::{AVResult, FFramesResult};

mod concatenator;
mod ffmpeg_helper;
mod media_processor;
mod renderer_error;

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

#[derive(Default, Debug)]
pub struct RenderOptions<'a> {
    resources_dir: &'a str,
}

fn render<TVideo: Video + Sync + Sized>(
    video: TVideo,
    options: RenderOptions,
) -> Result<(), FFramesError> {
    let fps = TVideo::FPS;
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb
        .load_font_file("/Users/dmtrkovalneko/dev/fframes/video/media/Bubble.ttf")
        .unwrap_or_else(|_| println!("Can not load a font"));

    let media_provider = media_processor::load_media_from_folder(options.resources_dir).unwrap();

    let duration_in_frames = match TVideo::DURATION {
        Duration::FromAudio(audio) => {
            let main_audio = media_provider.audio.get(audio).unwrap();
            main_audio.samples.len() / main_audio.sample_rate as usize * fps as usize
        }
        Duration::Seconds(seconds) => seconds * fps,
        Duration::Frames(frames) => frames,
    };

    let ctx = fframes_context::FFramesContext {
        mode: fframes::FFramesMode::Renderer,
        fps,
        media_provider,
    };

    let session = Uuid::new_v4();
    let directory = std::env::temp_dir().join(format!("fframes-{session}"));
    std::fs::create_dir(&directory)?;

    println!("rendering {} frames", duration_in_frames);

    let output = "out.mp4";
    let opt_ref = &opt.to_ref();

    let files = split_ffmpeg_chunks(
        duration_in_frames,
        divide_round_up(duration_in_frames, rayon::current_num_threads()),
    )
    .par_iter()
    .enumerate()
    .map(|(i, chunk_range)| {
        let file = directory
            .join(format!("{i}.mp4"))
            .into_os_string()
            .into_string()
            .unwrap();

        unsafe {
            Encoder::with_output(1920, 1080, fps as i32, file.as_str(), &mut |encoder| {
                let mut last_svg = "".to_owned();
                let frame = EncoderFrame::make(&encoder.video_stream);
                let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();

                if (i == 3) {
                    return Err(renderer_error::AVError::UnknownExtension("f".to_owned()));
                }

                chunk_range
                    .to_owned()
                    .into_iter()
                    .enumerate()
                    .try_for_each(|(index, fr)| {
                        let svg = video.render_frame(
                            &frame::Frame {
                                fps,
                                index: fr as i64,
                            },
                            ctx.clone(),
                        );

                        if svg != last_svg {
                            let rtree = usvg::Tree::from_str(&svg, opt_ref).unwrap();
                            resvg::render(
                                &rtree,
                                usvg::FitTo::Original,
                                tiny_skia::Transform::default(),
                                pixmap.as_mut(),
                            )
                            .unwrap();

                            last_svg = svg;
                        }

                        encoder.send_frame(frame.from_rgba_pixmap(index as i64, pixmap.data()))
                    })?;

                let frames_to_generate = chunk_range.end - chunk_range.start;
                let submitted_frames = encoder.video_stream.get_frames_in_stream() as usize;

                if submitted_frames < frames_to_generate {
                    let intra_frames_to_add = frames_to_generate - submitted_frames;

                    for _ in chunk_range.end..chunk_range.end + intra_frames_to_add {
                        encoder.send_frame(frame.frame)?;
                    }
                }

                Ok(())
            })
        }
        .and_then(std::convert::identity)
        .map_err(|av_err| FFramesError::RenderChunkError(i, av_err))?;

        Ok(file)
    })
    .collect::<FFramesResult<Vec<_>>>()?;

    unsafe {
        concatenator::concat_files(files.as_slice(), output)?;
    }

    println!(
        "resources_dir: {}",
        directory.to_str().unwrap_or("unknown directory")
    );

    Ok(())
}

fn main() {
    // render(video::marketing::MarketingVideo::make());
    let result = render(
        video::test_video::TestVideo::make(),
        RenderOptions {
            // resources_dir: "/Users/dmtrkovalenko/dev/fframes/editor-wasm/media",
            resources_dir: "/Users/dmtrkovalenko/dev/fframes/fframes-editor-controller",
        },
    )
    .unwrap();
}
