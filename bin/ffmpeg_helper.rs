use crate::renderer_error::{FFmpegError, FFmpegResult};

pub fn av_result(result: i32) -> Result<(), i32> {
    if result < 0 {
        Err(result)
    } else {
        Ok(())
    }
}

#[macro_export]
macro_rules! ffmpeg_action {
    ($x:expr, $err:expr) => {
        let res = $x;

        if (res < 0) {
            return Err($err);
        }
    };
}
