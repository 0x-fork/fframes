use colored::*;
use core::fmt::Debug;
use indicatif::ProgressBar;
use std::sync::Arc;

pub trait FFramesLogger: Sync + Send {
    fn init(&self, all_frames: usize);
    fn success(&self, output_path: &str, temp_files_dir: Option<&str>);
    fn log_frame(&self, index: usize, thread_number: usize, svg: &str);
}

struct CompactFFramesLogger {
    progress_bar: ProgressBar,
}

impl FFramesLogger for CompactFFramesLogger {
    fn init(&self, frames_count: usize) {
        println!("Rendering {} frames", frames_count.to_string().cyan());
    }

    fn log_frame(&self, _index: usize, _thread_number: usize, _svg: &str) {
        self.progress_bar.inc(1);
    }

    fn success(&self, output_path: &str, temp_files_dir: Option<&str>) {
        println!(
            "{} Watch your video: \n$ {ffplay} {output_path}\n\n{temp_files_slug}",
            "Success!".green().bold(),
            ffplay = "ffplay".bold(),
            temp_files_slug = temp_files_dir
                .map(|path| format!("Generated files {path}\n"))
                .unwrap_or_default()
        );
    }
}

struct SilentLogger;

impl FFramesLogger for SilentLogger {
    fn init(&self, _all_frames: usize) {}

    fn success(&self, output_path: &str, _temp_files_dir: Option<&str>) {
        println!("Success. Your video {}", output_path);
    }

    fn log_frame(&self, _index: usize, _thread_number: usize, _svg: &str) {}
}

impl Debug for dyn FFramesLogger {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Logger")
    }
}

#[derive(Debug)]
/// Different options for logging rendering process.
pub enum FFramesLoggerVariant {
    /// Doesn't show progress of rendering, only the output. Slightly faster.
    Silent,
    /// Renders one progress bar showing rendering progress frame by frame
    Compact,
    /// Pass custom logger functionality by implementing FFramesLogger trait
    Custom(Arc<dyn FFramesLogger>),
}

impl Default for FFramesLoggerVariant {
    fn default() -> Self {
        Self::Compact
    }
}

pub fn make_logger(variant: FFramesLoggerVariant, frames_count: usize) -> Arc<dyn FFramesLogger> {
    match variant {
        FFramesLoggerVariant::Silent => Arc::new(SilentLogger) as Arc<dyn FFramesLogger>,
        FFramesLoggerVariant::Compact => Arc::new(CompactFFramesLogger {
            progress_bar: ProgressBar::new(frames_count as u64),
        }) as Arc<dyn FFramesLogger>,
        FFramesLoggerVariant::Custom(logger) => logger,
    }
}
