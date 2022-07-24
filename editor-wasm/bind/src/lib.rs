use fframes::{fframes_context, frame, subtitles::Subtitles, video::Video, AudioTimestamp};
use fframes_editor_controller::setup_wasm_editor;
use lazy_static::lazy_static;
use std::{any::type_name, collections::HashMap, sync::Mutex};
use video::goose_thoughts::GooseVideo;
use video::{marketing::MarketingVideo, test_video::TestVideo};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;

setup_wasm_editor!(TestVideo);
