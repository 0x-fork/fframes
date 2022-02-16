mod encoder;
use fframes::Duration;
use fframes::{audio_data, fframes_context, frame, video::Video};
use rayon::prelude::*;
use renderer_error::FFramesError;
use std::ffi::CString;
use std::ops::Range;

use crate::encoder::{Encoder, EncoderFrame};

mod concatenator;
mod ffmpeg_helper;
mod media_processor;
mod renderer_error;

fn load_audio(path: &str) -> audio_data::AudioData {
    let (sample_reate, samples) = media_loader::decode_mp3(path);

    audio_data::AudioData {
        sample_rate: sample_reate,
        samples,
        max_magnitude: 0.0,
    }
}

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

    println!("rendering {} frames", duration_in_frames);

    let output = "out.mp4";
    let files = vec!["some-0.mp4", "some-1.mp4"]
        .into_iter()
        .map(|file| std::ffi::CStr::as_ptr(&CString::new(file).unwrap()))
        .collect::<Vec<_>>()
        .as_ptr();

    let opt_ref = &opt.to_ref();

    let files = split_ffmpeg_chunks(
        duration_in_frames,
        divide_round_up(duration_in_frames, rayon::current_num_threads()),
    )
    .par_iter()
    .enumerate()
    .map(|(i, chunk_range)| unsafe {
        let file = format!("some-{}.mp4", i);
        Encoder::with_output(1920, 1080, fps as i32, &file.as_str(), &mut |encoder| {
            let mut last_svg = "".to_owned();
            let frame = EncoderFrame::make(&encoder.video_stream);
            let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();

            chunk_range
                .to_owned()
                .into_iter()
                .enumerate()
                .for_each(|(index, fr)| {
                    let svg = video.render_frame(
                        &frame::Frame {
                            fps,
                            index: fr as i64,
                        },
                        ctx.clone(),
                    );

                    if svg != last_svg {
                        let rtree = usvg::Tree::from_str(&svg, opt_ref).unwrap();
                        resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

                        last_svg = svg;
                    }

                    encoder.send_frame(frame.from_rgba_pixmap(index as i64, pixmap.data()));
                });

            let frames_to_generate = chunk_range.end - chunk_range.start;
            let submitted_frames = encoder.video_stream.get_frames_in_stream() as usize;

            if submitted_frames < frames_to_generate {
                let intra_frames_to_add = frames_to_generate - submitted_frames;

                for _ in chunk_range.end..chunk_range.end + intra_frames_to_add {
                    // frame.set_index(intra_frame as i64);
                    encoder.send_frame(frame.frame);
                }
            }
        });

        file
    })
    .collect::<Vec<String>>();
    

    unsafe { concatenator::concat_files(files.as_slice(), output).map_err(Into::into) }
}

fn main() {
    // render(video::marketing::MarketingVideo::make());
    render(
        video::test_video::TestVideo::make(),
        RenderOptions {
            // resources_dir: "/Users/dmtrkovalenko/dev/fframes/editor-wasm/media",
            resources_dir: "/Users/dmtrkovalenko/dev/fframes/fframes-editor-controller",
        },
    ).unwrap();
}
