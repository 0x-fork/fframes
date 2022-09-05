#![feature(async_closure)]
use fframes_editor_controller::{prelude::*, setup_wasm_editor};
use podcast_example::PodcastVideo;

setup_wasm_editor!(PodcastVideo, {});
