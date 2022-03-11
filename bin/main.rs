mod encoder;
use fframes::media_provider::ImageData;
use fframes::Duration;
use fframes::{fframes_context, frame, video::Video};
use fframes_logger::FFramesLoggerVariant;
use rayon::prelude::*;
use renderer_error::FFramesError;
use std::ops::Range;
use uuid::Uuid;

use crate::encoder::{Encoder, EncoderFrame};
use crate::renderer_error::FFramesResult;

mod concatenator;
mod ffmpeg_helper;
pub mod fframes_logger;
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
    media_dir: &'a str,
    logger: FFramesLoggerVariant,
}

fn render<'a, TVideo: Video + Sync + Sized>(
    video: TVideo,
    output: &'a str,
    options: RenderOptions<'a>,
) -> Result<(), FFramesError> {
    let fps = TVideo::FPS;
    let mut logger = fframes_logger::make_logger(options.logger);

    let media_provider =
        media_processor::load_media_from_folder(&mut logger, options.media_dir).unwrap();

    let mut opt = usvg::Options::default();

    opt.fontdb.load_system_fonts();
    media_provider.fonts.values().for_each(|font_path| {
        opt.fontdb
            .load_font_file(font_path)
            .unwrap_or_else(|_| println!("Can not load a font"));
    });

    let cloned_provider = media_provider.clone();
    opt.image_href_resolver = usvg::ImageHrefResolver {
        // we forbid passing custom data in base64 format so just skip this
        resolve_data: Box::new(|_, _, _| None),
        resolve_string: Box::new(move |href: &str, _| {
            cloned_provider
                .images
                .get(href)
                .map(|(_, image)| match image {
                    ImageData::RawJpg(data) => usvg::ImageKind::JPEG(data.to_owned()),
                    ImageData::RawPng(data) => usvg::ImageKind::PNG(data.to_owned()),
                    ImageData::None => {
                        panic!("Somehow missing image data during rendering phase. Failing")
                    }
                })
        }),
    };

    let duration_in_frames = match TVideo::DURATION {
        Duration::FromAudio(audio) => {
            let main_audio = media_provider
                .audio
                .get(audio)
                .ok_or(FFramesError::MissingRequiredMedia(audio.to_owned()))?;

            main_audio.samples.len() / main_audio.sample_rate as usize * fps as usize
        }
        Duration::Seconds(seconds) => seconds * fps,
        Duration::Frames(frames) => frames,
    };

    logger.init_frames_rendering(duration_in_frames);

    let ctx = fframes_context::FFramesContext {
        mode: fframes::FFramesMode::Renderer,
        fps,
        media_provider,
    };

    let session = Uuid::new_v4();
    let directory = std::env::temp_dir().join(format!("fframes-{session}"));
    std::fs::create_dir(&directory)?;

    let opt_ref = &opt.to_ref();
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
                fps as i32,
                file.as_str(),
                &mut |encoder| {
                    let mut last_svg = "".to_owned();
                    let mut frame = EncoderFrame::make(&encoder.video_stream);
                    let mut pixmap =
                        tiny_skia::Pixmap::new(TVideo::WIDTH as u32, TVideo::HEIGHT as u32).unwrap();

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

                            logger.log_frame(index, thread_number, &svg);
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
        concatenator::concat_files(files.as_slice(), output)?;
    }

    logger.success(output, directory.to_str());

    Ok(())
}

fn main() {
    render(
        video::marketing::MarketingVideo::make(),
        "out.mp4",
        RenderOptions {
            media_dir: "/Users/dmtrkovalenko/dev/fframes/editor-wasm/media",
            ..Default::default()
        },
    )
    .unwrap();
}
