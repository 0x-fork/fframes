mod encoder;
use encoder::EncoderOptions;
use fframes::media_provider::ImageData;
use fframes::{fframes_context, frame, video::Video};
use fframes::{AudioData, Duration};
use fframes_logger::FFramesLoggerVariant;

use gpu::GpuRenderingBackend;
use lyon::geom::euclid::default;
use render_backend::{CpuRenderingBackend, FFramesRenderBackend, RenderBackendVariant};
use renderer_error::FFramesError;
use std::ops::Range;
use uuid::Uuid;

use crate::encoder::{Encoder, EncoderFrame};
use crate::renderer_error::FFramesResult;

mod concatenator;
mod ffmpeg_helper;
pub mod fframes_logger;
mod gpu;
mod media_processor;
pub mod render_backend;
mod renderer_error;
mod stream;

#[derive(Debug, Clone, Default)]
pub struct RenderOptions<'a, TBackend: FFramesRenderBackend> {
    media_dir: &'a str,
    logger: FFramesLoggerVariant,
    encoder_options: EncoderOptions<'a>,
    render_backend: TBackend,
    /// Preferred codec ot use. If not allowed to use will use default codec for the container which may not be the most efficient.
    /// Because libav by default ignores non-system codecs like hevc or x264.
    preferred_codec: &'a str,
}

fn render<'a, TVideo: Video + Sync + Sized, TBackend: FFramesRenderBackend>(
    video: TVideo,
    output: &'a str,
    options: RenderOptions<'a, TBackend>,
) -> Result<(), FFramesError> {
    let fps = TVideo::FPS;
    let mut logger = fframes_logger::make_logger(options.logger);

    let media_provider =
        media_processor::load_media_from_folder(&mut logger, options.media_dir).unwrap();

    let mut opt = usvg::Options::default();

    opt.fontdb.load_system_fonts();
    media_provider.fonts.values().for_each(|font_path| {
        opt.fontdb
            .load_font_file(font_path)
            .unwrap_or_else(|_| println!("Can not load a font"));
    });

    let cloned_provider = media_provider.clone();
    opt.image_href_resolver = usvg::ImageHrefResolver {
        // we forbid passing custom data in base64 format so just skip this
        resolve_data: Box::new(|_, _, _| None),
        resolve_string: Box::new(move |href: &str, _| {
            cloned_provider
                .images
                .get(href)
                .map(|(_, image)| match image {
                    ImageData::RawJpg(data) => usvg::ImageKind::JPEG(data.to_owned()),
                    ImageData::RawPng(data) => usvg::ImageKind::PNG(data.to_owned()),
                    ImageData::None => {
                        panic!("Somehow missing image data during rendering phase. Failing")
                    }
                })
        }),
    };

    let duration_in_frames = match TVideo::DURATION {
        Duration::FromAudio(audio) => {
            let main_audio = media_provider
                .audio
                .get(audio)
                .ok_or(FFramesError::MissingRequiredMedia(audio.to_owned()))?;

            match main_audio {
                AudioData::Preloaded(data) => {
                    data.samples.len() / data.sample_rate as usize * fps as usize
                }
                _ => 0,
            }
        }
        Duration::Seconds(seconds) => seconds * fps,
        Duration::Frames(frames) => frames,
    };

    logger.init_frames_rendering(duration_in_frames);

    let ctx = fframes_context::FFramesContext {
        sample_rate: 44100,
        mode: fframes::FFramesMode::Renderer,
        fps,
        media_provider,
    };

    options.render_backend.render(
        output,
        video,
        logger,
        &opt.to_ref(),
        duration_in_frames,
        options.encoder_options,
        ctx,
    )?;

    Ok(())
}

fn main() {
    render(
        video::marketing::MarketingVideo::make(),
        "out.mp4",
        RenderOptions {
            media_dir: "/Users/dmtrkovalenko/dev/fframes/editor-wasm/media",
            logger: FFramesLoggerVariant::Compact,
            preferred_codec: "libx264",
            render_backend: CpuRenderingBackend {},
            ..Default::default()
        },
    )
    .unwrap();
}
