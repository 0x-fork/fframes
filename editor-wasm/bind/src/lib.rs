use fframes::{
    AudioTimestamp, fframes_context, frame, subtitles::Subtitles, video::Video,
};
use fframes_editor_controller::setup_wasm_editor;
use lazy_static::lazy_static;
use std::{any::type_name, collections::HashMap, sync::Mutex};
use video::marketing::MarketingVideo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;


setup_wasm_editor!(MarketingVideo);
