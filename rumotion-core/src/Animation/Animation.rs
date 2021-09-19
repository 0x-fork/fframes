use super::Spring;
use std::ops::Range;

pub enum AnimationRuntime {
    Linear(f32),
    SpringRuntime(Spring::SpringRuntime),
}

impl AnimationRuntime {
    pub fn solve_spring(runtime: &Spring::SpringRuntime, t: &f32) -> f32 {
        let progress = if runtime.m_zeta < 1.0 {
            // Under-damped
            libm::expf(-t * &runtime.m_zeta * &runtime.w0)
                * (&runtime.a * libm::cosf(&runtime.wd * t)
                    + &runtime.b * libm::sinf(&runtime.wd * t))
        } else {
            // Critically damped
            (&runtime.a + &runtime.b * t) * libm::expf(-t * &runtime.w0)
        };

        // Map range from [1..0] to [0..1].
        1.0 - progress
    }

    pub fn get_duration(&self) -> &f32 {
        match &self {
            AnimationRuntime::Linear(duration) => duration,
            AnimationRuntime::SpringRuntime(runtime) => &4.0,
        }
    }

    pub fn solve(&self, t: &f32) -> f32 {
        match &self {
            &AnimationRuntime::Linear(duration) => t / duration,
            &AnimationRuntime::SpringRuntime(spring) => Self::solve_spring(&spring, t),
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
            AnimationRuntime::SpringRuntime(Spring::SpringRuntime::from_options(&options))
        }
        Easing::Linear(duration) => AnimationRuntime::Linear(*duration),
    }
}

pub struct Tween {
    pub start: i64,
    pub from: f64,
    pub to: f64,
    pub easing: Easing,
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
            .map(|tween| KeyFrame {
                to: tween.to,
                from: tween.from,
                seconds_range: (tween.start as f32..tween.start as f32 + 200.0),
                animation_runtime: make_runtime(&tween.easing),
            })
            .collect::<Vec<KeyFrame>>();

        SteppedAnimation { keyframes }
    }
}
