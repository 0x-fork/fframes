use crate::{audio_data, media_provider, subtitles, ResolvedAudioMap};

#[derive(Clone, Copy, Debug)]
pub enum FFramesMode {
    Editor,
    EditorTimelinePreview,
    Renderer,
}

#[derive(Clone, Debug)]
pub struct FFramesContext {
    pub fps: usize,
    pub sample_rate: usize,
    pub mode: FFramesMode,
    pub media_provider: media_provider::MediaProvider,
    // pub resolve_lazy_audio_during_render: Option<Arc<dyn Fn(usize) -> Vec<f32>>>,
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

    /// This method takes all the information we
    pub fn get_mixed_audio_data_in_fltp(
        &self,
        audio_map: &ResolvedAudioMap,
        start_sample: usize,
        frame_size: usize,
    ) -> Vec<f32> {
        let mut audio_data = vec![0.0; frame_size];

        audio_map.0.iter().for_each(|(f, sample_range)| {
            if sample_range.contains(&start_sample) {
                let start_of_this_frame_in_file = start_sample - sample_range.start;

                self.get_audio_data(f)
                    .get_range(
                        start_of_this_frame_in_file..start_of_this_frame_in_file + frame_size,
                    )
                    .map(|data| {
                        data.into_iter().enumerate().for_each(|(i, sample)| {
                            let fltp_sample = *sample as f32 / i16::MAX as f32;
                            let filled_sample = audio_data[i];

                            if filled_sample == 0. {
                                audio_data[i] = fltp_sample
                            } else {
                                audio_data[i] =
                                    filled_sample + fltp_sample - (filled_sample * fltp_sample)
                            }
                        });
                    });
            }
        });

        audio_data
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
