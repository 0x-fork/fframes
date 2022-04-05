use crate::{Animation, Frame};
use svgr_macro::svgr;

mod fframes {
    pub use crate::*;
}

pub struct Ctx;

impl Ctx { 
  pub fn get_image_link(&self, a: &str) -> String {
    a.to_owned()
  }
}

#[test]
pub fn macro_animations() {
    let frame = Frame { fps: 50, index: 75 };
    let ctx = Ctx {};

    let a = || {
        svgr!(
           <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width="1920"
            height="1080"
          >

                   <image
                     width="900"
                     height="900"
                     xlink:href={ctx.get_image_link("code.png")}
                    //  x={frame.animate(&self.code_animation)}
                     y="10"
                   />
            <rect
                x={frame.animate(fframes::timeline!(
                  on 0., val 10.0 => 12.2, Animation::Easing::Linear(0.2),
                  on 10., val 10.0 => 12.2, Animation::Easing::Linear(0.2),
                  on 12., val 10.0 => 12.2, Animation::Easing::Linear(0.2)
                ))}
            />
          </svg>
        );
    };

    assert_eq!(a(), r"".to_string());
}
