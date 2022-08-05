use std::{collections::HashMap, sync::Arc};

use crate::{audio_data, subtitles};

#[derive(Clone, Debug)]
pub enum ImageData {
    RawPng(Arc<Vec<u8>>),
    RawJpg(Arc<Vec<u8>>),
    None,
    Base64(String),
}

#[derive(Clone, Debug)]
pub struct MediaProvider {
    pub audio: HashMap<String, audio_data::AudioData>,
    pub images: HashMap<String, (String, ImageData)>,
    pub subtitles: HashMap<String, subtitles::Subtitles>,
    pub fonts: HashMap<String, String>,
}
