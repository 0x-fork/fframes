use fframes::{AudioData::AudioData, FFramesContext, Frame, Subtitles::Subtitles, Video::Video};
use fframes_editor_controller::setup_wasm_editor;
use lazy_static::lazy_static;
use std::{any::type_name, cmp::Ordering, collections::HashMap, sync::Mutex};
use video::marketing::MarketingVideo;
use video::podcast::PodcastVideo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "fframes-editor")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn load_audio_wasm_callback(path: &str) -> Result<JsValue, JsValue>;
}

setup_wasm_editor!(PodcastVideo);

// lazy_static! {
//     static ref VIDEO: video::marketing::MarketingVideo = video::marketing::MarketingVideo::make();
//     static ref AUDIO_CACHE: Mutex<HashMap<String, AudioData>> = Mutex::new(HashMap::new());
//     static ref SUBTITLES_CACHE: Mutex<HashMap<String, Subtitles>> = Mutex::new(HashMap::new());
// }

// #[wasm_bindgen]
// pub fn cache_audio(file: String, input: &[f32]) {
//     let audio_data = AudioData {
//         sample_rate: 44100,
//         samples: input.to_vec(),
//         max_magnitude: input
//             .iter()
//             .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
//             .unwrap_or(&0.0)
//             .to_owned()
//             .sqrt(),
//     };

//     let mut audio_cache = AUDIO_CACHE.lock().unwrap();
//     audio_cache.insert(file, audio_data);
// }

// #[wasm_bindgen]
// pub fn cache_subtitles(file: String, content: String) {
//     console_error_panic_hook::set_once();

//     SUBTITLES_CACHE
//         .lock()
//         .unwrap()
//         .insert(file, Subtitles::from_str(content.as_str()));
// }

// #[wasm_bindgen]
// pub fn render_frame(frame: i64) -> String {
//     VIDEO.render_frame(
//         &Frame::Frame {
//             fps: 30,
//             index: frame,
//         },
//         FFramesContext::FFramesContext {
//             fps: 30,
//             audio: &AUDIO_CACHE.lock().unwrap(),
//             subtitles: &SUBTITLES_CACHE.lock().unwrap(),
//         },
//     )
// }
