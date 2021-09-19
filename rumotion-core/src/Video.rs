use std::collections::HashMap;

use crate::{AudioData, Frame, RumotionContext};

pub struct EnvContext {
    pub audio: HashMap<String, AudioData::AudioData>,
}

pub trait Video {
    const FPS: i64;

    fn make() -> Self
    where
        Self: Sync + Sized;
    fn render_frame(&self, frame: &Frame::Frame, ctx: RumotionContext::RumotionContext) -> String;
}
