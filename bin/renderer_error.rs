use fframes::FFramesContext;
use handlebars::RenderError;

#[derive(Debug)]
pub enum FFmpegError {
    MissingVideoStreamInFile(String),
    CantOpenFile(String),
    CantAllocateCtx,
    CantOpenCodec,
    UnknownExtension(String),
}

pub type FFmpegResult<T> = Result<T, FFmpegError>;

#[derive(Debug)]
pub enum FFramesError {
    FFmpegError(FFmpegError),
    MediaError(std::io::Error)
}

impl From<FFmpegError> for FFramesError {
    fn from(ffmpeg_error: FFmpegError) -> Self {
        Self::FFmpegError(ffmpeg_error)
    }
}

impl From<std::io::Error> for FFramesError{
    fn from(io_error: std::io::Error) -> Self {
        Self::MediaError(io_error)
    }
}
