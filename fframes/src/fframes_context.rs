use crate::{
    audio_data, media_provider,
    subtitles::{self},
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum FFramesMode {
    Editor,
    EditorTimelinePreview,
    Renderer,
}

#[derive(Clone)]
pub struct FFramesContext {
    pub fps: usize,
    pub mode: FFramesMode,
    pub media_provider: media_provider::MediaProvider,
}

impl FFramesContext {
    pub fn get_audio_data(&self, filename: &str) -> &audio_data::AudioData {
        match self.media_provider.audio.get(filename) {
            Some(data) => data,
            None => panic!("Audio data not found for {file}, please make sure that media folder contains {file}.mp3", file=filename)
        }
    }

    pub fn get_subtitles(&self, filename: &str) -> &subtitles::Subtitles {
        match self.media_provider.subtitles.get(filename) {
            Some(data) => data,
            None => panic!("Subtitles {file} not found! Please make sure that media folder contains {file}.vtt", file=filename)
        }
    }

    pub fn get_image_link(&self, filename: &str) -> &str {
        match self.media_provider.images.get(filename) {
            Some(data) => data,
            None => panic!(
                "Image {file} not found! Please make sure that media folder contains {file}.vtt",
                file = filename
            ),
        }
    }
}
