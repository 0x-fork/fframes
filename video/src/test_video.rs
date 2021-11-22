pub use rumotion_core::{AudioData, Frame, RumotionContext, Video::Video, Windows};
use svgr_macro::{self, svgr};

pub struct TestVideo {}

impl Video for TestVideo {
    const FPS: i64 = 30;

    fn make() -> Self {
        TestVideo {}
    }

    fn render_frame(&self, frame: &Frame::Frame, ctx: RumotionContext::RumotionContext) -> String {
        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width="1920"
            height="1080"
          >
            <rect width="1920" height="1080" x="0" y="0" fill="white" />
            <text x="100" y="100" font-size="100">
              {format!("Frame number: {}, second: {}", frame.index + 1, frame.getCurrentSecond())}
            </text>
          </svg>
        )
    }
}
