use crate::audio_map::AudioMap;
use crate::{fframes_context, frame};

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

    fn audio(&self) -> AudioMap;
    fn render_frame(&self, frame: &frame::Frame, ctx: &fframes_context::FFramesContext) -> String;
}
