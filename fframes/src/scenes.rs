use std::{collections::HashMap, fmt::Debug};

pub enum Overlap {
    Previous(f64),
    Next(f64),
    PreviousAndNext((f64, f64)),
    None,
}

pub fn seconds_to_frames(seconds: &f64, fps: usize) -> usize {
    (seconds * fps as f64) as usize
}

impl Overlap {
    pub(crate) fn to_frames(&self, fps: usize) -> (usize, usize) {
        match self {
            Overlap::Previous(sec) => (seconds_to_frames(sec, fps), 0),
            Overlap::Next(sec) => (0, seconds_to_frames(sec, fps)),
            Overlap::PreviousAndNext((prev, next)) => {
                (seconds_to_frames(prev, fps), seconds_to_frames(next, fps))
            }
            Overlap::None => (0, 0),
        }
    }
}

pub trait Scene: Debug + Sync + Send {
    // const DURATION: Duration;
    fn duration(&self) -> crate::video::Duration;
    fn render_frame(&self, frame: crate::frame::Frame, ctx: &crate::FFramesContext) -> String;

    fn overlap(&self) -> Overlap {
        Overlap::None
    }
}

#[macro_export]
macro_rules! ff {
    ($(on $start: expr, val $from:expr => $to:expr, $easing:expr),+) => {
        fframes::animation::SteppedAnimation::make_from_tweens(vec![
           $(
            fframes::animation::Tween {
                start: $start,
                from: $from,
                to: $to,
                easing: &$easing,
            }
           ),+
    ])
    };
}

pub struct Scenes(pub(crate) Option<Vec<Box<dyn Scene>>>);

impl From<Vec<Box<dyn Scene>>> for Scenes {
    fn from(arr: Vec<Box<dyn Scene>>) -> Self {
        Self(Some(arr))
    }
}
