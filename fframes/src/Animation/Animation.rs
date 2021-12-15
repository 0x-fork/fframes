use super::Spring;
use std::ops::Range;

pub enum AnimationRuntime {
    Linear(f32),
    SpringRuntime(Spring::SpringRuntime, f32),
}

impl AnimationRuntime {
    pub fn get_duration(&self) -> f32 {
        match &self {
            AnimationRuntime::Linear(duration) => *duration,
            AnimationRuntime::SpringRuntime(_spring, duration) => *duration,
        }
    }

    pub fn solve(&self, t: &f32) -> f32 {
        match &self {
            &AnimationRuntime::Linear(duration) => t / duration,
            &AnimationRuntime::SpringRuntime(spring, _) => spring.solve(t),
        }
    }
}

pub enum Easing {
    Linear(f32),
    // Inspired by https://webkit.org/demos/spring/spring.js. Copyright (C) 2016 Apple Inc. All rights reserved.
    Spring(Spring::SpringOptions),
}

pub struct LinearRuntime {}

pub fn make_runtime(easing: &Easing) -> AnimationRuntime {
    match easing {
        Easing::Spring(options) => {
            let spring_runtime = Spring::SpringRuntime::from_options(&options);
            let duration = spring_runtime.get_duration();

            AnimationRuntime::SpringRuntime(spring_runtime, duration)
        }
        Easing::Linear(duration) => AnimationRuntime::Linear(*duration),
    }
}

pub struct Tween<'a> {
    pub start: f32,
    pub to: f64,
    pub from: f64,
    pub easing: &'a Easing,
}

pub(crate) struct KeyFrame {
    pub(crate) seconds_range: Range<f32>,
    pub(crate) from: f64,
    pub(crate) to: f64,
    pub(crate) animation_runtime: AnimationRuntime,
}

pub struct SteppedAnimation {
    pub(crate) keyframes: Vec<KeyFrame>,
}

impl SteppedAnimation {
    pub fn make_from_tweens(tweens: Vec<Tween>) -> Self {
        let keyframes = tweens
            .iter()
            .enumerate()
            .flat_map(|(i, tween)| {
                let animation_runtime = make_runtime(&tween.easing);
                let keyframe = KeyFrame {
                    to: tween.to,
                    from: tween.from,
                    seconds_range: (tween.start..tween.start + animation_runtime.get_duration()),
                    animation_runtime,
                };

                match tweens.get(i + 1) {
                    None => vec![keyframe],
                    Some(next_tween) if next_tween.start <= keyframe.seconds_range.end => {
                        vec![keyframe]
                    }
                    Some(next_tween) => {
                        let filler_keyframe_range = keyframe.seconds_range.end..next_tween.start;
                        let filler_keyframe = KeyFrame {
                            from: keyframe.to,
                            to: keyframe.to,
                            animation_runtime: AnimationRuntime::Linear(
                                filler_keyframe_range.end - filler_keyframe_range.start,
                            ),
                            seconds_range: filler_keyframe_range,
                        };

                        vec![keyframe, filler_keyframe]
                    }
                }
            })
            .collect::<Vec<KeyFrame>>();

        SteppedAnimation { keyframes }
    }
}
