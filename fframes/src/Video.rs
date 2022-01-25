use std::{array, collections::HashMap, hash::Hash};

use crate::{AudioData, FFramesContext, Frame};

pub struct EnvContext {
    pub audio: HashMap<String, AudioData::AudioData>,
}
pub enum Duration {
    FromAudio(&'static str),
    Seconds(usize),
    Frames(usize),
}

pub enum AudioTimestamp {
    Frame(usize),
    Second(usize),
    /// Plays audio till the end of file
    Eof,
}

pub struct AudioMap(pub Option<HashMap<&'static str, (AudioTimestamp, AudioTimestamp)>>);

impl AudioMap {
    pub fn none() -> Self {
        AudioMap(None)
    }
}

impl<const N: usize> From<[(&'static str, (AudioTimestamp, AudioTimestamp)); N]> for AudioMap {
    fn from(arr: [(&'static str, (AudioTimestamp, AudioTimestamp)); N]) -> Self {
        AudioMap(Some(array::IntoIter::new(arr).collect()))
    }
}

pub trait Video: Sized {
    const FPS: usize;
    const WIDTH: usize;
    const HEIGHT: usize;
    const DURATION: Duration;

    fn make() -> Self
    where
        Self: Sync + Sized;

    fn audio() -> AudioMap;
    fn render_frame(&self, frame: &Frame::Frame, ctx: FFramesContext::FFramesContext) -> String;
}
