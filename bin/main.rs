use rayon::prelude::*;
use rumotion_core::{AudioData, Frame, RumotionContext, Video::Video};
use std::collections::HashMap;
use usvg::SystemFontDB;

fn load_audio(path: &str) -> AudioData::AudioData {
    let (sample_reate, samples) = media_loader::decode_mp3(path);

    AudioData::AudioData {
        media_id: "kek".to_string(),
        sample_rate: sample_reate,
        samples,
        max_magnitude: 0.0,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

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

    println!("audio decoding completed");

    let video = video::podcast::PodcastVideo::make();

    let final_audio = audio_hash.get("me").unwrap();
    let duration_in_frames = final_audio.samples.len() / final_audio.sample_rate as usize
        * video::podcast::PodcastVideo::FPS as usize;

    let ctx = RumotionContext::RumotionContext {
        fps: video::podcast::PodcastVideo::FPS,
        audio: &audio_hash,
        fft_hash: None,
    };

    println!("rendering {} frames", duration_in_frames);

    (0..duration_in_frames).into_par_iter().for_each(|frame| {
        let svg = video.render_frame(
            &Frame::Frame {
                fps: video::podcast::PodcastVideo::FPS,
                index: frame as i64,
            },
            ctx.clone(),
        );

        let rtree = usvg::Tree::from_str(&svg, &opt).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();
        resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

        pixmap
            .save_png(format!(
                "{dir}/frame-{i}.png",
                dir = "/Users/dmitrijkovalenko/dev/rumotion/bin/out",
                i = frame
            ))
            .unwrap();
    });
}
