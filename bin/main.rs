mod encoder;
use ffmpeg_next::sys::exit;
use fframes::Duration;
use fframes::Subtitles::Subtitles;
use fframes::{AudioData, FFramesContext, Frame, Video::Video};
use rayon::prelude::*;
use std::ffi::CString;
use std::ops::Range;
use std::thread;
use std::{borrow::BorrowMut, collections::HashMap, sync::mpsc};

use crate::encoder::{Encoder, EncoderFrame};

fn load_audio(path: &str) -> AudioData::AudioData {
    let (sample_reate, samples) = media_loader::decode_mp3(path);

    AudioData::AudioData {
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

fn render<TVideo: Video + Sync + Sized>(video: TVideo) {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb
        .load_font_file("/Users/dmtrkovalneko/dev/fframes/video/media/Bubble.ttf")
        .unwrap_or_else(|_| println!("Can not load a font"));

    println!("decoding audio");
    let mut audio_hash = HashMap::new();
    // audio_hash.insert(
    //     "marketing".to_owned(),
    //     load_audio("/Users/dmtrkovalenko/dev/fframes/editor-wasm/media/marketing.mp3"),
    // );
    audio_hash.insert(
        "me.mp3".to_owned(),
        load_audio("/Users/dmtrkovalenko/goose_duck/me.mp3"),
    );
    audio_hash.insert(
        "vlad.mp3".to_owned(),
        load_audio("/Users/dmtrkovalenko/goose_duck/vlad.mp3"),
    );
    audio_hash.insert(
        "guest.mp3".to_owned(),
        load_audio("/Users/dmtrkovalenko/goose_duck/guest.mp3"),
    );
    audio_hash.insert(
        "final.mp3".to_owned(),
        load_audio("/Users/dmtrkovalenko/goose_duck/final.mp3"),
    );

    let mut subtitles_hash = HashMap::new();
    // subtitles_hash.insert(
    //     "subtitles".to_owned(),
    //     Subtitles::from_file("/Users/dmitrijkovalenko/dev/fframes/editor-wasm/media/subtitles.vtt"),
    // );

    println!("audio decoding completed");

    let fps = TVideo::FPS;

    let duration_in_frames = match TVideo::DURATION {
        Duration::FromAudio(audio) => {
            let main_audio = audio_hash.get(audio).unwrap();
            main_audio.samples.len() / main_audio.sample_rate as usize * fps as usize
        }
        Duration::Seconds(seconds) => seconds * fps,
        Duration::Frames(frames) => frames,
    };

    let ctx = FFramesContext::FFramesContext {
        fps,
        audio: &audio_hash,
        subtitles: &subtitles_hash,
    };

    println!("rendering {} frames", duration_in_frames);

    let output = "out.mp4";
    let files = vec!["some-0.mp4", "some-1.mp4"]
        .into_iter()
        .map(|file| std::ffi::CStr::as_ptr(&CString::new(file).unwrap()))
        .collect::<Vec<_>>()
        .as_ptr();
    let opt_ref = &opt.to_ref();
    unsafe {
        split_ffmpeg_chunks(
            duration_in_frames,
            divide_round_up(duration_in_frames, rayon::current_num_threads()),
        )
        .par_iter()
        .enumerate()
        .for_each(|(i, chunk_range)| {
            println!("{:?}", chunk_range);
            Encoder::with_output(
                1920,
                1080,
                fps as i32,
                format!("some-{}.mp4", i).as_str(),
                &mut |encoder| {
                    let mut last_svg = "".to_owned();
                    let frame = EncoderFrame::make(&encoder.video_stream);
                    let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();

                    chunk_range
                        .to_owned()
                        .into_iter()
                        .enumerate()
                        .for_each(|(index, fr)| {
                            let svg = video.render_frame(
                                &Frame::Frame {
                                    fps,
                                    index: fr as i64,
                                },
                                ctx.clone(),
                            );
                            

                            if svg != last_svg {
                                let rtree = usvg::Tree::from_str(&svg, opt_ref).unwrap();
                                resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut())
                                    .unwrap();

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
                },
            )
        });

        encoder::concat_files(
            std::ffi::CStr::as_ptr(&CString::new(output).unwrap()),
            files,
        );
    }
}

fn main() {
    // render(video::marketing::MarketingVideo::make());
    render(video::podcast::PodcastVideo::make());
}
