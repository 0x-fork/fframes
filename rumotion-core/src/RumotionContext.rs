use crate::AudioData;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RumotionContext<'a> {
    pub fps: i64,
    pub audio: &'a HashMap<String, AudioData::AudioData>,
    pub fft_hash: Option<&'a HashMap<i64, Vec<f32>>>,
}

impl RumotionContext<'_> {
    pub fn get_audio_data(&self, key: &str) -> &AudioData::AudioData {
        match &self.audio.get(key) {
            Some(data) => data,
            None => panic!("Audio data not found for {file}, please make sure that media folder contains {file}.mp3", file=key)
        }
    }
}
