use std::{array, collections::HashMap, ops::Range};

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
/// The resolved audio_map contain each audio file position and duration in {1/{ctx.sample_rate}} units
pub struct ResolvedAudioMap(pub HashMap<&'static str, Range<usize>>);

impl ResolvedAudioMap {
    pub fn calc_stream_duration_in_seconds(&self, ctx: &FFramesContext) -> usize {
        self.0
            .iter()
            .map(|(_, range)| (range.start + range.end) / ctx.sample_rate)
            .max()
            .unwrap_or(0)
    }
}

impl AudioMap {
    pub fn resolve(&self, ctx: &FFramesContext) -> Option<ResolvedAudioMap> {
        self.0
            .as_ref()
            .map(|hash_map| {
                hash_map
                    .into_iter()
                    .map(|(f, (start_ts, end_ts))| {
                        let start_sample = start_ts.to_seconds(f, ctx) * ctx.sample_rate;
                        let end_sample = end_ts.to_seconds(f, ctx) * ctx.sample_rate;

                        (*f, start_sample..end_sample)
                    })
                    .collect::<HashMap<_, _>>()
            })
            .map(ResolvedAudioMap)
    }

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

fn get_audio_duration((file, duration): (&&str, &AudioDuration), ctx: &FFramesContext) -> usize {
    let (start, end) = duration;

    start.to_seconds(file, ctx) + end.to_seconds(file, ctx)
}

impl<const N: usize> From<[(&'static str, AudioDuration); N]> for AudioMap {
    fn from(arr: [(&'static str, AudioDuration); N]) -> Self {
        AudioMap(Some(IntoIterator::into_iter(arr).collect()))
    }
}
