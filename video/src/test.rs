use fframes::*;
use svgr_macro::svgr;

pub fn render_frame(frame: &frame::Frame, ctx: fframes_context::FFramesContext) -> String {
    let audio_visualization = audio_data::visualize_audio_frame(
        frame,
        &audio_data::VisualizeFrameInput {
            audio: ctx.get_audio_data("marketing"),
            sample_size: audio_data::SampleSize::S16,
            ctx: &ctx,
        },
    );

    svgr!(
      <svg
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        width="1920"
        height="1080"
      >

       {
          audio_visualization
          .iter()
          .enumerate()
          .map(|(i, val)|  {
            let bar_height = match val {
              height if height.is_nan() || height < &0.0 => 0.0,
              height => height.to_owned()
            };

            svgr_macro::svgr!(


              <rect
                y={(1080 / 2) as f32 - bar_height / 2.0}
                x={i * 24}
                transform-origin="center center"
                height={bar_height}
                width={16}
                fill="tomato"
                rx={16 / 4}
                ry={16 / 4}
              />
           )})
          .collect::<Vec<String>>().join("\n")
        }
      </svg>
    )
}
