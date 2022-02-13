use std::collections::HashMap;

use crate::{audio_data, subtitles};

#[derive(Clone)]
pub struct MediaProvider {
    pub audio: HashMap<String, audio_data::AudioData>,
    pub images: HashMap<String, String>,
    pub subtitles: HashMap<String, subtitles::Subtitles>,
}
