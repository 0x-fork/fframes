use crate::{
    audio_data,
    media_provider::{self, ImageData},
    subtitles::{self},
};

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
            None => panic!(
                "Subtitles {file} not found! Please make sure that media folder contains {file}",
                file = filename
            ),
        }
    }

    pub fn get_image_link(&self, filename: &str) -> String {
        match self.media_provider.images.get(filename) {
            Some((link, _)) => link.to_owned(),
            None => panic!(
                "Image {file} not found! Please make sure that media folder contains {file}",
                file = filename
            ),
        }
    }

    // pub fn get_image_data(&self, filename: &str) -> Vec<u8> {
    //     match self.media_provider.images.get(filename) {
    //         Some(data) => match data.to_owned() {
    //             ImageData::Url(_) => panic!("Received image url instead of blob data. Likely mixed up the rendering and editing media provider."),
    //             ImageData::ImageData(data) => data
    //         },
    //         None => panic!(
    //             "Image {file} not found! Please make sure that media folder contains {file}",
    //             file = filename
    //         ),
    //     }
    // }
}
