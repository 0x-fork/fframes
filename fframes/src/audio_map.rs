use std::{array, collections::HashMap};

use crate::FFramesContext;

pub enum AudioTimestamp {
    Frame(usize),
    Second(usize),
    /// Plays audio till the end of file
    Eof,
}

impl AudioTimestamp {
    fn to_seconds(&self, filename: &str, ctx: &FFramesContext) -> usize {
        match self {
            AudioTimestamp::Frame(frame) => frame * ctx.fps,
            AudioTimestamp::Second(seconds) => *seconds,
            AudioTimestamp::Eof => ctx.get_audio_data(filename).duration_in_seconds(),
        }
    }
}

type AudioDuration = (AudioTimestamp, AudioTimestamp);

pub struct AudioMap(pub Option<HashMap<&'static str, AudioDuration>>);

fn get_audio_duration((file, duration): (&&str, &AudioDuration), ctx: &FFramesContext) -> usize {
    let (start, end) = duration;

    start.to_seconds(file, ctx) + end.to_seconds(file, ctx)
}

impl AudioMap {
    pub fn none() -> Self {
        AudioMap(None)
    }

    pub fn calc_stream_duration_in_seconds(&self, ctx: &FFramesContext) -> usize {
        let inner = self.0.as_ref();
        match inner {
            None => 0,
            Some(audio_map) => audio_map
                .into_iter()
                .map(|val| get_audio_duration(val, &ctx))
                .max()
                .unwrap_or(0),
        }
    }
}

impl<const N: usize> From<[(&'static str, AudioDuration); N]> for AudioMap {
    fn from(arr: [(&'static str, AudioDuration); N]) -> Self {
        AudioMap(Some(array::IntoIter::new(arr).collect()))
    }
}
