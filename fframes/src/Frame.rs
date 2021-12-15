use crate::Animation;

pub struct Frame {
    pub index: i64,
    pub fps: usize,
}

impl Frame {
    pub fn get_current_second(&self) -> f32 {
        self.index as f32 / self.fps as f32
    }

    pub fn animate_runtime(
        &self,
        start: f32,
        from: f64,
        to: f64,
        animation_runtime: &Animation::AnimationRuntime,
    ) -> f64 {
        let duration = animation_runtime.get_duration();

        match &self.get_current_second() {
            second if second < &start => from,
            second if second > &(start + duration) => to,
            second => {
                let progress = animation_runtime.solve(&(second - start));

                let animation_range = to - from;

                from + animation_range * progress as f64
            }
        }
    }

    pub fn animate_or(&self, animation: &Animation::SteppedAnimation, default_value: f64) -> f64 {
        let current_second = &self.get_current_second();
        let keyframe = animation
            .keyframes
            .iter()
            .rev()
            .find(|keyframe| keyframe.seconds_range.contains(current_second));

        match keyframe {
            None => default_value,
            Some(keyframe) => {
                let progress = keyframe
                    .animation_runtime
                    .solve(&(current_second - keyframe.seconds_range.start));

                let animation_range = keyframe.to - keyframe.from;

                keyframe.from + animation_range * progress as f64
            }
        }
    }
}
