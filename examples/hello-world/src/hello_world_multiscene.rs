pub use fframes::{audio_data, fframes_context, frame, video::Video};
use fframes::{AudioMap, Color, Scene};
use svgr_macro::{self, svgr};

pub struct HelloWorldMultisceneVideo {}

#[derive(Debug)]
struct SceneOne {}

impl Scene for SceneOne {
    fn duration(&self) -> fframes::Duration {
        fframes::Duration::Seconds(15)
    }

    fn render_frame(&self, _frame: frame::Frame, _ctx: &fframes_context::FFramesContext) -> String {
        svgr!(
          <text font-family="DM Sans" x="100" y="300" font-size="150"> "Hello Scene 1" </text>
        )
    }
}

#[derive(Debug)]
struct SceneTwo {}

impl Scene for SceneTwo {
    fn duration(&self) -> fframes::Duration {
        fframes::Duration::Seconds(15)
    }

    fn render_frame(&self, frame: frame::Frame, _ctx: &fframes_context::FFramesContext) -> String {
        svgr!(
          <text font-family="DM Sans" x="100" y={frame.animate
            (fframes::timeline!(on 0., val 300. => 320., fframes::Easing::Linear(0.2)))} font-size="150"> "Hello Scene 2" </text>
        )
    }
}

impl Video for HelloWorldMultisceneVideo {
    const FPS: usize = 30;
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

    fn audio(&self) -> AudioMap {
        AudioMap::none()
    }

    fn define_scenes(&self) -> fframes::Scenes {
        let vec: Vec<Box<dyn Scene>> = vec![Box::new(SceneTwo {}), Box::new(SceneOne {})];

        fframes::Scenes::from(vec)
    }

    fn render_frame(&self, frame: frame::Frame, _ctx: &fframes_context::FFramesContext) -> String {
        const BACKGROUND_EASING: fframes::Easing = fframes::Easing::Linear(5.);

        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width={Self::WIDTH}
            height={Self::HEIGHT}
          >
            <rect
              width={Self::WIDTH}
              height={Self::HEIGHT}
              x="0"
              y="0"
              fill={
                frame.animate(fframes::timeline!(
                  on 0., val Color::hex("#fff") => Color::hex("#f8fafc"), &BACKGROUND_EASING,
                  on 5., val Color::hex("#f8fafc") => Color::hex("#fff7ed"), &BACKGROUND_EASING,
                  on 10., val Color::hex("#fff7ed") => Color::hex("#fef2f2"), &BACKGROUND_EASING,
                  on 15., val Color::hex("#fef2f2") => Color::hex("#f7fee7"), &BACKGROUND_EASING,
                  on 20., val Color::hex("#f7fee7") => Color::hex("#ecfdf5"), &BACKGROUND_EASING,
                  on 25., val Color::hex("#ecfdf5") => Color::hex("#faf5ff"), &BACKGROUND_EASING
                ))
              }
            />

            {_ctx.render_scenes(&frame)}

            <text font-weight="500" font-family="JetBrains Mono" x="100" y="440" font-size="74" fill="#4b5563">
              {format!("This frame index: {}, second: {:.2}", frame.index, frame.get_current_second())}
            </text>
          </svg>
        )
    }
}
