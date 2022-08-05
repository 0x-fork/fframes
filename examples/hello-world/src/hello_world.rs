use fframes::AudioMap;
pub use fframes::{audio_data, fframes_context, frame, video::Video};
use svgr_macro::{self, svgr};

pub struct HelloWorldVideo {}

impl Video for HelloWorldVideo {
    const FPS: usize = 30;
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const DURATION: fframes::Duration = fframes::Duration::Seconds(60);

    fn audio(&self) -> AudioMap {
        AudioMap::none()
    }

    fn make() -> Self {
        HelloWorldVideo {}
    }

    fn render_frame(&self, frame: &frame::Frame, _ctx: &fframes_context::FFramesContext) -> String {
        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width={Self::WIDTH}
            height={Self::HEIGHT}
          >
            <rect width={Self::WIDTH} height={Self::HEIGHT} x="0" y="0" fill="white" />
            <text x="100" y="100" font-size="100">
              {format!("Hey! Frame number: {}, second: {}", frame.index + 1, frame.get_current_second())}
            </text>
          </svg>
        )
    }
}
