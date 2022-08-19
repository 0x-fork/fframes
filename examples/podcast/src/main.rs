use fframes_renderer::render_backend::GpuRenderingBackend;
pub use fframes_renderer::{fframes_logger, render, render_backend, RenderOptions};
use podcast_example::PodcastVideo;

fn main() {
    render(
        PodcastVideo {},
        "out.mp4",
        RenderOptions {
            // media_dir: "./media",
            media_dir: "/Users/dmtrkovalenko/dev/fframes/examples/podcast/media",
            logger: fframes_logger::FFramesLoggerVariant::Compact,
            render_backend: GpuRenderingBackend {},
            preferred_codec: "libx264",
            ..Default::default()
        },
    )
    .unwrap();
}
