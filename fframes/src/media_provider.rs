use std::{collections::HashMap, sync::Arc};

use crate::{audio_data, subtitles};

#[derive(Clone)]
pub enum ImageData {
    RawPng(Arc<Vec<u8>>),
    RawJpg(Arc<Vec<u8>>),
    None
}

#[derive(Clone)]
pub struct MediaProvider {
    pub audio: HashMap<String, audio_data::AudioData>,
    pub images: HashMap<String, (String, ImageData)>,
    pub subtitles: HashMap<String, subtitles::Subtitles>,
    pub fonts: HashMap<String, String>,
}
