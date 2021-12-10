use std::{cmp::Ordering, collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use rumotion_core::{AudioData::AudioData, RumotionContext, Subtitles::Subtitles, Video::Video};
use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

lazy_static! {
    static ref AUDIO_CACHE: Mutex<HashMap<String, AudioData>> = Mutex::new(HashMap::new());
    static ref SUBTITLES_CACHE: Mutex<HashMap<String, Subtitles>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub fn cache_audio(file: String, input: &[f32]) {
    let audio_data = AudioData {
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

#[wasm_bindgen]
pub fn cache_subtitles(file: String, content: String) {
    console_error_panic_hook::set_once();

    SUBTITLES_CACHE
        .lock()
        .unwrap()
        .insert(file, Subtitles::from_str(content.as_str()));
}

#[wasm_bindgen]
pub fn render_frame(frame: i64) -> String {
    let video = video::marketing::MarketingVideo::make();
    video.render_frame(
        &video::Frame::Frame {
            fps: 30,
            index: frame,
        },
        RumotionContext::RumotionContext {
            fps: 30,
            audio: &AUDIO_CACHE.lock().unwrap(),
            subtitles: &SUBTITLES_CACHE.lock().unwrap(),
        },
    )
}
