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
                Self::CantOpenFile(file) => format!("Missing video stream in file {}", file.cyan()),
                Self::CantAllocateCtx => "Can not allocate encoding context".to_owned(),
                Self::FFmpegError(code, description) =>
                    format!("libav error {code}: {description}"),
                Self::CantWriteFrame(file) =>
                    format!("Can not write frame to file {}", file.cyan()),
                Self::UnknownExtension(file) => format!(
                    "Can not deduce file format of output file {} from extension.",
                    file.cyan().bold()
                ),
            }
        )
    }
}

pub type AVResult<T> = Result<T, AVError>;

pub enum FFramesError {
    FFmpegError(AVError),
    RenderChunkError(usize, AVError),
    MediaError(std::io::Error),
    MissingRequiredMedia(String),
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
                    chunk = chunk.to_string().cyan().bold()
                ),
                Self::FFmpegError(err) => format!("{err}"),
                Self::MediaError(err) =>
                    format!("{}\n{}", "Can't load or process media".bold(), err),
                Self::MissingRequiredMedia(required_media) => format!(
                    "Missing required media {}. Verify that you provided correct media_dir.",
                    required_media.magenta().bold()
                ),
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
