use fframes::AudioMap;
pub use fframes::{audio_data, fframes_context, frame, video::Video};
use svgr_macro::{self, svgr};

pub struct TestVideo {}

impl Video for TestVideo {
    const FPS: usize = 30;
    const HEIGHT: usize = 1920;
    const WIDTH: usize = 1080;
    const DURATION: fframes::Duration = fframes::Duration::Frames(100);

    fn audio() -> AudioMap {
        AudioMap::none()
    }

    fn make() -> Self {
        TestVideo {}
    }

    fn render_frame(&self, frame: &frame::Frame, ctx: fframes_context::FFramesContext) -> String {
        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width="1920"
            height="1080"
          >
            <rect width="1920" height="1080" x="0" y="0" fill="white" />
            <text x="100" y="100" font-size="100">
              {format!("Frame number: {}, second: {}", frame.index + 1, frame.get_current_second())}
            </text>
          </svg>
        )
    }
}
