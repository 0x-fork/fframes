use super::Spring;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub enum AnimationRuntime {
    /// No animation, used internally for filling gaps keyframe
    Static(f32, f32),
    Linear(f32),
    SpringRuntime(Spring::SpringRuntime, f32),
}

impl AnimationRuntime {
    pub fn from_easing(easing: &Easing) -> Self {
        match easing {
            Easing::Spring(options) => {
                let spring_runtime = Spring::SpringRuntime::from_options(&options);
                let duration = spring_runtime.get_duration();

                AnimationRuntime::SpringRuntime(spring_runtime, duration)
            }
            Easing::Linear(duration) => AnimationRuntime::Linear(*duration),
            Easing::Spring2(mass, stiffness, damping) => {
                let spring_runtime = Spring::SpringRuntime::from_options(&crate::SpringOptions {
                    mass: *mass,
                    stiffness: *stiffness,
                    damping: *damping,
                });

                let duration = spring_runtime.get_duration();
                AnimationRuntime::SpringRuntime(spring_runtime, duration)
            }
        }
    }

    pub fn get_duration(&self) -> f32 {
        match &self {
            AnimationRuntime::Linear(duration) => *duration,
            AnimationRuntime::SpringRuntime(_spring, duration) => *duration,
            AnimationRuntime::Static(_, _) => todo!(),
        }
    }

    pub fn solve(&self, t: &f32) -> f32 {
        match &self {
            &AnimationRuntime::Linear(duration) => t / duration,
            &AnimationRuntime::SpringRuntime(spring, _) => spring.solve(t),
            AnimationRuntime::Static(permanent_value, _) => permanent_value.to_owned(),
        }
    }
}

/// Animation easing. Different variants of how value changes over time.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Easing {
    /// Specifies an animation with the same speed from start to end.
    /// calculates as Linear(duration): f(current_time) = current_time / duration
    Linear(f32),
    // Inspired by https://webkit.org/demos/spring/spring.js. Copyright (C) 2016 Apple Inc. All rights reserved.
    Spring(Spring::SpringOptions),
    /// Specifies an animation that calculates value based on spring physics.
    /// Learn more about spring physics: https://www.joshwcomeau.com/animation/a-friendly-introduction-to-spring-physics/
    Spring2(f32, f32, f32),
}

pub struct LinearRuntime {}

#[derive(Clone, Copy, Debug)]
pub struct Tween<'a> {
    pub start: f32,
    pub to: f32,
    pub from: f32,
    pub easing: &'a Easing,
}

#[derive(Clone, Debug)]
pub(crate) struct KeyFrame {
    pub(crate) seconds_range: Range<f32>,
    pub(crate) from: f32,
    pub(crate) to: f32,
    pub(crate) animation_runtime: AnimationRuntime,
}

pub struct SteppedAnimation {
    pub(crate) keyframes: Vec<KeyFrame>,
}

impl SteppedAnimation {
    pub fn make_from_tweens(tweens: Vec<Tween>) -> Self {
        let mut sorted_tweens = tweens;
        sorted_tweens.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        let mut keyframes = sorted_tweens
            .iter()
            .enumerate()
            .flat_map(|(i, tween)| {
                let animation_runtime = AnimationRuntime::from_easing(&tween.easing);
                let keyframe = KeyFrame {
                    to: tween.to,
                    from: tween.from,
                    seconds_range: (tween.start..tween.start + animation_runtime.get_duration()),
                    animation_runtime,
                };

                match sorted_tweens.get(i + 1) {
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

        if sorted_tweens[0].start > 0. {
            keyframes.insert(
                0,
                KeyFrame {
                    from: sorted_tweens[0].from,
                    to: sorted_tweens[0].from,
                    seconds_range: 0f32..sorted_tweens[0].start,
                    animation_runtime: AnimationRuntime::Static(
                        sorted_tweens[0].from,
                        sorted_tweens[0].start,
                    ),
                },
            );
        }

        let last_keyframe = &keyframes[keyframes.len() - 1];
        if last_keyframe.to < f32::MAX {
            let last_filling_keyframe = KeyFrame {
                from: last_keyframe.to,
                to: last_keyframe.to,
                animation_runtime: AnimationRuntime::Static(
                    last_keyframe.to,
                    f32::MAX - last_keyframe.seconds_range.end,
                ),
                seconds_range: last_keyframe.seconds_range.end..f32::MAX,
            };

            keyframes.push(last_filling_keyframe);
        }

        SteppedAnimation { keyframes }
    }
}

#[macro_export]
macro_rules! timeline {
    ($(on $start: expr, val $from:expr => $to:expr, $easing:expr),+) => {
        fframes::Animation::SteppedAnimation::make_from_tweens(vec![
           $(
            fframes::Animation::Tween {
                start: $start,
                from: $from,
                to: $to,
                easing: &$easing,
            }
           ),+
    ])
    };
}
