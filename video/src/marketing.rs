use std::string;

use rumotion_core::Animation::{self, AnimationRuntime, SpringOptions, Tween};
pub use rumotion_core::{AudioData, Frame, RumotionContext, Video::Video, Windows};
use svgr_macro::{self, svgr};

struct SpectrumValue {
    color: String,
    val: f32,
}

pub struct MarketingVideo {
    spring: Animation::AnimationRuntime,
    final_animation: Animation::SteppedAnimation,
}

impl Video for MarketingVideo {
    const FPS: i64 = 60;

    fn make() -> Self {
        let spring = Animation::Easing::Spring(SpringOptions {
            mass: 1.0,
            stiffness: 100.0,
            damping: 10.0,
        });

        MarketingVideo {
            spring: Animation::make_runtime(&spring),
            final_animation: Animation::SteppedAnimation::make_from_tweens(vec![Tween {
                start: 16,
                from: 48.0,
                to: 48.0 * 4.0,
                easing: spring,
            }]),
        }
    }

    fn render_frame(&self, frame: &Frame::Frame, ctx: RumotionContext::RumotionContext) -> String {
        const BAR_SIZE: usize = 96;
        const BAR_SIZE_F32: f32 = BAR_SIZE as f32;

        let audio_visualization = AudioData::visualize_audio_frame(
            frame,
            &AudioData::VisualizeFrameInput {
                audio: ctx.get_audio_data("marketing"),
                sample_size: AudioData::SampleSize::S128,
                ctx: &ctx,
            },
        );
        let pretty_spectrum = [
            SpectrumValue {
                val: audio_visualization[3],
                color: "#F59E0B".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[7],
                color: "#3B82F6".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[2],
                color: "#10B981".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[1],
                color: "#ffffff".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[4],
                color: "#EF4444".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[5],
                color: "#EC4899".to_owned(),
            },
            SpectrumValue {
                val: audio_visualization[6],
                color: "#6366F1".to_owned(),
            },
        ];

        const BAR_MARGIN: usize = 16;
        const BAR_WIDTH_WITH_MARGIN: usize = BAR_SIZE + BAR_MARGIN;
        // viewbox width - space that all bars will take - right margin
        let spectrum_space_len = 1920 - BAR_WIDTH_WITH_MARGIN * pretty_spectrum.len() - BAR_MARGIN;

        svgr!(
          <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            width="1920"
            height="1080"
          >
            <pattern id="pattern-hex"  x="48" y="-30" width="112" height="190" patternUnits="userSpaceOnUse" viewBox="56 -254 112 190">
              <g id="hexagon">
                <path stroke-width="4" stroke="#6b6b6b" fill="#000" d="M168-127.1c0.5,0,1,0.1,1.3,0.3l53.4,30.5c0.7,0.4,1.3,1.4,1.3,2.2v61c0,0.8-0.6,1.8-1.3,2.2L169.3-0.3 c-0.7,0.4-1.9,0.4-2.6,0l-53.4-30.5c-0.7-0.4-1.3-1.4-1.3-2.2v-61c0-0.8,0.6-1.8,1.3-2.2l53.4-30.5C167-127,167.5-127.1,168-127.1 L168-127.1z"></path>
                <path stroke-width="4" stroke="#6b6b6b" fill="#000" d="M112-222.5c0.5,0,1,0.1,1.3,0.3l53.4,30.5c0.7,0.4,1.3,1.4,1.3,2.2v61c0,0.8-0.6,1.8-1.3,2.2l-53.4,30.5 c-0.7,0.4-1.9,0.4-2.6,0l-53.4-30.5c-0.7-0.4-1.3-1.4-1.3-2.2v-61c0-0.8,0.6-1.8,1.3-2.2l53.4-30.5 C111-222.4,111.5-222.5,112-222.5L112-222.5z"></path>
                <path stroke-width="4" stroke="#6b6b6b" fill="#000" d="M168-317.8c0.5,0,1,0.1,1.3,0.3l53.4,30.5c0.7,0.4,1.3,1.4,1.3,2.2v61c0,0.8-0.6,1.8-1.3,2.2L169.3-191 c-0.7,0.4-1.9,0.4-2.6,0l-53.4-30.5c-0.7-0.4-1.3-1.4-1.3-2.2v-61c0-0.8,0.6-1.8,1.3-2.2l53.4-30.5 C167-317.7,167.5-317.8,168-317.8L168-317.8z"></path>
              </g>
            </pattern>

            <rect x="0" y="0" width="100%" height="100%" fill="#000" />
            <rect x="0" y="0" width="100%" height="100%" fill="url(#pattern-hex)" />


           {
              pretty_spectrum
              .iter()
              .enumerate()
              .map(|(i, SpectrumValue { val, color})|  {
                let bar_height = match val {
                  height if height.is_nan() => BAR_SIZE_F32,
                  height if height < &BAR_SIZE_F32 => BAR_SIZE_F32,
                  height if height > &920.0 => 920.0,
                  height => height.to_owned()
                };

                svgr_macro::svgr!(
                  <filter id={format!("{}-shadow", i)} x="-100%" y="-100%" width="300%" height="300%">
                    <feGaussianBlur in="SourceAlpha" stdDeviation="10.4"/>
                    <feOffset dx="0" dy="3" result="offsetblur"/>
                    <feFlood flood-color={color}  flood-opacity="0.5" />
                    <feComposite in2="offsetblur" operator="in"/>
                    <feMerge>
                      <feMergeNode/>
                      <feMergeNode in="SourceGraphic"/>
                    </feMerge>
                  </filter>

                  <rect
                    y={(1080 / 2) as f32 - bar_height / 2.0}
                    x={frame.animate_runtime(
                      16.0,
                      ((spectrum_space_len / 2) + (i * BAR_WIDTH_WITH_MARGIN)) as f64,
                      944.9,
                      &self.spring,
                    )}
                    transform-origin="center center"
                    fill={color}
                    filter={format!("url(#{}-shadow)", i)}
                    height={bar_height}
                    width={BAR_SIZE}
                    rx={BAR_SIZE / 2}
                    ry={BAR_SIZE / 2}
                  />
               )})
              .collect::<Vec<String>>().join("\n")
            }

            <circle cx="944" fill="#fff" cy="540" r={frame.animate_or(&self.final_animation, 48.0 as f64)} />
            <g opacity={frame.animate_runtime(16.0, 0.0, 100.0, &self.spring)}>
              <text x="49%" y="50%" font-family="'Bubble Bobble'" font-size="87" dominant-baseline="middle" text-anchor="middle">"fframe"</text>
              <text x="49%" y="55%"  dominant-baseline="middle" font-size="18" text-anchor="middle">"Video creation framework."</text>
            </g>
          </svg>
        )
    }
}
