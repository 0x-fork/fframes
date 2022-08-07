use crate::{animation, AnimationRuntime};

pub struct Frame {
    pub index: i64,
    pub fps: usize,
}

pub struct AnimateRuntimeInput<'a> {
    pub on: f32,
    pub from: f32,
    pub to: f32,
    pub animation_runtime: &'a AnimationRuntime,
}

impl Frame {
    pub fn get_current_second(&self) -> f32 {
        self.index as f32 / self.fps as f32
    }

    /// Calculates animation in runtime.
    /// Unlike frame.animate(fframes::timeline!()) you can pass dynamic value in the start/from/to properties.
    /// But it is required to prepare easing function in advance.
    ///
    /// Use frame.animate! if possible.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fframes::{animation, AnimationRuntime, Frame};
    ///
    /// let frame = Frame { index: 0, fps: 60 };
    /// const RUNTIME: AnimationRuntime = AnimationRuntime::from_easing(animation::Easing::Linear(2.0))
    ///
    /// let value = frame.animate_runtime(AnimateRuntimeInput {  on: 3.2, from: 1000, to: 2000, animation_runtime: &RUNTIME);
    /// ```
    pub fn animate_runtime(
        &self,
        AnimateRuntimeInput {
            on,
            from,
            to,
            animation_runtime,
        }: AnimateRuntimeInput,
    ) -> f32 {
        let duration = animation_runtime.get_duration();

        match &self.get_current_second() {
            second if second < &on => from,
            second if second > &(on + duration) => to,
            second => {
                let progress = animation_runtime.solve(&(second - on));

                let animation_range = to - from;
                from + animation_range * progress
            }
        }
    }

    /// Returns the current value of the animation at the current second.
    /// # Panics
    ///
    /// Panics if current value can't be calculate, this may happen if seconds are negative or timeline is broken.
    ///
    /// # Timeline
    ///
    /// Timeline is defined using fframes::timeline! macros. All the gaps between frames are filled automatically.
    ///
    /// ## Example
    ///
    /// In this example we have the 2 transition and 5 states. Value based on seconds:
    /// * from 0 to 2.3 -> 1400
    /// * from 2.3 to 2.9 -> transition from 1400 to 770 (duration calculates based on spring duration)
    /// * from 2.9 to 4.8 -> 770
    /// * from 4.8 to 5.4 -> transition from 770 to 1400 (duration calculates based on spring duration)
    /// * from 5.4 to end of file -> 1400
    ///
    /// ```rust
    /// svgr!(
    ///   <rect
    ///     y={frame.animate(fframes::timeline!(
    ///         on 2.3, val 1400. => 770., Spring(1.85, 130.0, 16.0),
    ///         on 4.8, val 770. => 1400., Spring(1.85, 130.0, 16.0)
    ///     ))}
    ///   />
    /// );
    /// ```
    pub fn animate(&self, animation: &animation::Steppedanimation) -> f32 {
        let current_second = &self.get_current_second();

        let keyframe = animation
            .keyframes
            .iter()
            .rev()
            .find(|keyframe| keyframe.seconds_range.contains(current_second));

        match keyframe {
            None => panic!("frame.animate can not get the value for frame {}. It may mean that Steppedanimation is not correctly filled out./", self.index),
            Some(keyframe) => {
                let progress = keyframe
                    .animation_runtime
                    .solve(&(current_second - keyframe.seconds_range.start));

                let animation_range = keyframe.to - keyframe.from;

                keyframe.from + animation_range * progress
            }
        }
    }
}
