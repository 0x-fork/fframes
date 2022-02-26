use colored::Colorize;
use std::fmt;

pub enum AVError {
    MissingVideoStreamInFile(String),
    CantOpenFile(String),
    CantAllocateCtx,
    CantWriteFrame(String),
    UnknownExtension(String),
    FFmpegError(i32, String),
}

impl fmt::Display for AVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::MissingVideoStreamInFile(file) =>
                    format!("Missing video stream in file {file}"),
                Self::CantOpenFile(file) => format!("Missing video stream in file {file}"),
                Self::CantAllocateCtx => "Can not allocate encoding context".to_owned(),
                _ => "wtf".to_owned(),
            }
        )
    }
}

pub type AVResult<T> = Result<T, AVError>;

pub enum FFramesError {
    FFmpegError(AVError),
    RenderChunkError(usize, AVError),
    MediaError(std::io::Error),
}

impl fmt::Debug for FFramesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{header}\n{error}",
            header = "Failure".red().bold(),
            error = match self {
                Self::RenderChunkError(chunk, error) => format!(
                    "Rendering chunk {chunk} failed.\nReason: {error}",
                    chunk = chunk.to_string().cyan()
                ),
                Self:: FFmpegError(err) => format!("libav error:, {err}"),
                _ => "wtf".to_owned(),
            }
        )
    }
}

pub type FFramesResult<T> = Result<T, FFramesError>;

impl From<AVError> for FFramesError {
    fn from(ffmpeg_error: AVError) -> Self {
        Self::FFmpegError(ffmpeg_error)
    }
}

impl From<std::io::Error> for FFramesError {
    fn from(io_error: std::io::Error) -> Self {
        Self::MediaError(io_error)
    }
}
