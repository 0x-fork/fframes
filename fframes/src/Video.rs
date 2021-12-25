use std::collections::HashMap;

use crate::{AudioData, FFramesContext, Frame};

pub struct EnvContext {
    pub audio: HashMap<String, AudioData::AudioData>,
}

pub enum Duration {
    FromAudio(&'static str),
    Seconds(usize),
    Frames(usize),
}

pub trait Video: Sized {
    const FPS: usize;
    const WIDTH: usize;
    const HEIGHT: usize;
    const DURATION: Duration;

    fn make() -> Self
    where
        Self: Sync + Sized;
    fn render_frame(&self, frame: &Frame::Frame, ctx: FFramesContext::FFramesContext) -> String;
}
