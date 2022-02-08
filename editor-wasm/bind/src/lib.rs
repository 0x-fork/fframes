use fframes::{
    audio_data::AudioData, AudioTimestamp, fframes_context, frame, subtitles::Subtitles, video::Video,
};
use fframes_editor_controller::setup_wasm_editor;
use lazy_static::lazy_static;
use std::{any::type_name, cmp::Ordering, collections::HashMap, hash::Hash, sync::Mutex};
use video::marketing::MarketingVideo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;


setup_wasm_editor!(MarketingVideo);
