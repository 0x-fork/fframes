use std::iter::FromIterator;

use crate::{
    audio_data, media_provider, subtitles, video::ResolvedScenesTimeline, Frame, ResolvedAudioMap,
    Svgr,
};

#[derive(Clone, Copy, Debug)]
pub enum FFramesMode {
    Editor,
    EditorTimelinePreview,
    Renderer,
}

#[derive(Debug)]
pub struct FFramesContext<'a> {
    pub fps: usize,
    pub sample_rate: usize,
    pub mode: FFramesMode,
    pub media_provider: media_provider::MediaProvider,
    pub duration_in_frames: usize,
    pub scenes: Option<&'a ResolvedScenesTimeline>,
}

impl<'a> FFramesContext<'a> {
    pub fn get_audio_data(&self, filename: &str) -> &audio_data::AudioData {
        // crate::log!("{:?}", self.media_provider.audio);
        match self.media_provider.audio.get(filename) {
            Some(data) => data,
            None => panic!("Audio data not found for {file}, please make sure that media folder contains {file}", file=filename)
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
        match (self.mode, self.media_provider.images.get(filename)) {
            (FFramesMode::EditorTimelinePreview, Some(data)) if data.base64.is_some() => {
                // safe to unwrap because of leading if
                data.base64.as_ref().unwrap().to_owned()
            }
            (_, Some(data)) => data.link.to_owned(),
            _ => panic!(
                "Image {file} not found! Please make sure that media folder contains {file}",
                file = filename
            ),
        }
    }

    pub fn render_scenes(&self, global_frame: &Frame) -> Svgr {
        if let Some(scenes) = self.scenes.as_ref() {
            Svgr::from_iter(scenes.0.iter().filter_map(|(range, scene)| {
                if range.contains(&global_frame.index) {
                    Some(scene.render_frame(
                        Frame {
                            fps: global_frame.fps,
                            global_index: global_frame.index,
                            index: global_frame.index - range.start,
                        },
                        self,
                    ))
                } else {
                    None
                }
            }))
        } else {
            Svgr::default()
        }
    }

    /// This is internal method that is used by the renderer which mixes audio data and returns the final as fltp in a vector.
    #[allow(clippy::option_map_unit_fn)]
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
                        data.iter().enumerate().for_each(|(i, sample)| {
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
}
