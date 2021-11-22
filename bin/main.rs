mod encoder;
use ffmpeg_next::sys::exit;
use rayon::prelude::*;
use rumotion_core::{AudioData, Frame, RumotionContext, Video::Video};
use std::ffi::CString;
use std::ops::Range;
use std::thread;
use std::{borrow::BorrowMut, collections::HashMap, sync::mpsc};
use usvg::SystemFontDB;

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
    opt.fontdb.set_generic_families();
    opt.fontdb
        .load_font_file("/Users/dmitrijkovalenko/dev/rumotion/video/media/Bubble.ttf")
        .unwrap_or_else(|_| println!("Can not load a font"));

    println!("decoding audio");
    let mut audio_hash = HashMap::new();
    // audio_hash.insert(
    //     "marketing".to_owned(),
    //     load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/marketing.mp3"),
    // );
    audio_hash.insert(
        "me".to_owned(),
        load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/me.mp3"),
    );
    audio_hash.insert(
        "vlad".to_owned(),
        load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/vlad.mp3"),
    );
    audio_hash.insert(
        "guest".to_owned(),
        load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/guest.mp3"),
    );
    audio_hash.insert(
        "final".to_owned(),
        load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/final.mp3"),
    );

    println!("audio decoding completed");

    let fps = TVideo::FPS;

    let final_audio = audio_hash.get("final").unwrap();
    let duration_in_frames =
        final_audio.samples.len() / final_audio.sample_rate as usize * fps as usize;

    let ctx = RumotionContext::RumotionContext {
        fps,
        audio: &audio_hash,
        fft_hash: None,
    };

    let duration_in_frames = 630;
    println!("rendering {} frames", duration_in_frames);

    let output = "out.mp4";
    let files = vec!["some-0.mp4", "some-1.mp4"]
        .into_iter()
        .map(|file| std::ffi::CStr::as_ptr(&CString::new(file).unwrap()))
        .collect::<Vec<_>>()
        .as_ptr();

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
                    let frame = EncoderFrame::make(&encoder.video_stream);
                    let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();

                    chunk_range
                        .to_owned()
                        .into_iter()
                        .enumerate()
                        .for_each(|(index, fr)| {
                            println!("{}", fr);
                            let svg = video.render_frame(
                                &Frame::Frame {
                                    fps,
                                    index: fr as i64,
                                },
                                ctx.clone(),
                            );

                            let rtree = usvg::Tree::from_str(&svg, &opt).unwrap();
                            resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

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
    render(video::test_video::TestVideo::make());
}
