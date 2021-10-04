mod encoder;
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

    unsafe {
        encoder::test();
    }

    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.fontdb.set_generic_families();
    opt.fontdb
        .load_font_file("/Users/dmitrijkovalenko/dev/rumotion/video/media/Bubble.ttf")
        .unwrap_or_else(|_| println!("Can not load a font"));

    println!("decoding audio");
    // let mut audio_hash = HashMap::new();
    // audio_hash.insert(
    //     "marketing".to_owned(),
    //     load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/marketing.mp3"),
    // );
    // audio_hash.insert(
    //     "me".to_owned(),
    //     load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/me.mp3"),
    // );
    // audio_hash.insert(
    //     "vlad".to_owned(),
    //     load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/vlad.mp3"),
    // );
    // audio_hash.insert(
    //     "guest".to_owned(),
    //     load_audio("/Users/dmitrijkovalenko/dev/rumotion/video/media/guest.mp3"),
    // );

    println!("audio decoding completed");

    let video = video::podcast::PodcastVideo::make();

    // let final_audio = audio_hash.get("me").unwrap();
    // let duration_in_frames = final_audio.samples.len() / final_audio.sample_rate as usize
    //     * video::podcast::PodcastVideo::FPS as usize;

    // let ctx = RumotionContext::RumotionContext {
    //     fps: video::podcast::PodcastVideo::FPS,
    //     audio: &audio_hash,
    //     fft_hash: None,
    // };

    // println!("rendering {} frames", duration_in_frames);

    (0..30).into_iter().for_each(|f| {
        // let svg = video.render_frame(
        //     &Frame::Frame {
        //         fps: video::podcast::PodcastVideo::FPS,
        //         index: frame as i64,
        //     },
        //     ctx.clone(),
        // );

        // let rtree = usvg::Tree::from_str(&svg, &opt).unwrap();
        // let mut pixmap = tiny_skia::Pixmap::new(1920, 1080).unwrap();
        // resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();

        // let mut frame_ptr = unsafe { ffmpeg_next::sys::av_frame_alloc() };
        // let mut frame: &mut AVFrame = unsafe { frame_ptr.as_mut() }.unwrap();

        // for y in 0..frame.height {
        //     for x in 0..frame.width {
        //         unsafe {
        //             {
        //                 let mut this = frame.data[0].offset((y * frame.linesize[0] + x) as isize);
        //                 let val = (x + y + f * 3) as *mut u8;
        //                 let thin = &mut this;
        //                 // SAFETY: In case of a thin pointer, this operations is identical
        //                 // to a simple assignment. In case of a fat pointer, with the current
        //                 // fat pointer layout implementation, the first field of such a
        //                 // pointer is always the data pointer, which is likewise assigned.
        //                 unsafe { *thin = val };
        //                 this
        //             };
        //         }
        //     }
        // }
        // frame.pts = f as i64;
        // frame.width = 1;
        // // frame.format = encoder.format() as i32;

        // let keker = unsafe { ffmpeg_frame::Frame::wrap(frame) };
        // print!("{:#?}", frame);
        // unsafe {
        //     encoder.send_frame(&keker).unwrap();
        // }
    });
}
