use std::{cmp::Ordering, collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use rumotion_core::{RumotionContext, Video::Video};
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

lazy_static! {
    static ref AUDIO_CACHE: Mutex<HashMap<String, video::AudioData::AudioData>> =
        Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn cache_audio(file: String, input: &[f32]) {
    console_error_panic_hook::set_once();
    let audio_data = video::AudioData::AudioData {
        media_id: file.clone(),
        sample_rate: 44100,
        samples: input.to_vec(),
        max_magnitude: input
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(&0.0)
            .to_owned()
            .sqrt(),
    };

    let mut audio_cache = AUDIO_CACHE.lock().unwrap();
    audio_cache.insert(file, audio_data);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn render_frame(frame: i64) -> String {
    let video = video::podcast::PodcastVideo::make();
    // video::render_frame(frame, AUDIO_CACHE.lock().unwrap().get("test").unwrap())
    video.render_frame(
        &video::Frame::Frame {
            fps: 30,
            index: frame,
        },
        RumotionContext::RumotionContext {
            fft_hash: None,
            fps: 30,
            audio: &AUDIO_CACHE.lock().unwrap(),
        },
    )
}
