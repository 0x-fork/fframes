pub use fframes::{audio_data, fframes_context, frame, video::Video};
use fframes::{Animation, AudioMap};
use svgr_macro::{self, svgr};

pub struct TestVideo {}

impl Video for TestVideo {
    const FPS: usize = 30;
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;
    const DURATION: fframes::Duration = fframes::Duration::Seconds(100);

    fn audio(&self) -> AudioMap {
        AudioMap::none()
    }

    fn make() -> Self {
        TestVideo {}
    }

    fn render_frame(&self, frame: &frame::Frame, ctx: &fframes_context::FFramesContext) -> String {
        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width={Self::WIDTH}
            height={Self::HEIGHT}
          >
            <rect width={Self::WIDTH} height={Self::HEIGHT} x="0" y="0" fill="white" />
            <text x="100" y="100" font-size="100">

              <rect
                x={frame.animate(fframes::timeline!(
                  on 24., val 10.0 => 12.2, Animation::Easing::Linear(0.2),
                  on 24., val 10.0 => 12.2, Animation::Easing::Linear(0.2),
                  on 24., val 10.0 => 12.2, Animation::Easing::Linear(0.2)
                ))}
              />

              {format!("Hey! Frame number: {}, second: {}", frame.index + 1, frame.get_current_second())}
            </text>
          </svg>
        )
    }
}
