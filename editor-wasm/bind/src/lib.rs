use fframes::{AudioData::AudioData, FFramesContext, Frame, Subtitles::Subtitles, Video::Video};
use fframes_editor_controller::setup_wasm_editor;
use lazy_static::lazy_static;
use std::{any::type_name, cmp::Ordering, collections::HashMap, sync::Mutex};
use video::marketing::MarketingVideo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

setup_wasm_editor!(MarketingVideo);
