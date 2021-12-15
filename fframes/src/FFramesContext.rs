use crate::{
    AudioData,
    Subtitles::{self},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct FFramesContext<'a> {
    pub fps: usize,
    pub audio: &'a HashMap<String, AudioData::AudioData>,
    pub subtitles: &'a HashMap<String, Subtitles::Subtitles>,
}

impl FFramesContext<'_> {
    pub fn get_audio_data(&self, filename: &str) -> &AudioData::AudioData {
        match self.audio.get(filename) {
            Some(data) => data,
            None => panic!("Audio data not found for {file}, please make sure that media folder contains {file}.mp3", file=filename)
        }
    }

    pub fn get_subtitles(&self, filename: &str) -> &Subtitles::Subtitles {
        match self.subtitles.get(filename) {
            Some(data) => data,
            None => panic!("Subtitles {file} not found! Please make sure that media folder contains {file}.vtt", file=filename)
        }
    }
}
