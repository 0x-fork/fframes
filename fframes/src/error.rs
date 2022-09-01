use std::fmt;

pub enum FFramesCoreError {
    CanNotProcessAudioDuration(String),
    MissingDurationOrScenes,
}

impl fmt::Debug for FFramesCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FFramesCoreError::CanNotProcessAudioDuration(file) => format!("Can not get the duration based on the Duration::FromString. The file {} is not a valid audio file.", file),
                FFramesCoreError::MissingDurationOrScenes => "Can not infer the duration of video. The implementation of Video trait must have either const DURATION or the define_scenes method implemented.".to_owned(),
            }
        )
    }
}

pub type Result<T> = std::result::Result<T, FFramesCoreError>;
