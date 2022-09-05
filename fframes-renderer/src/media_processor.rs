use fframes::{
    audio_data,
    media_provider::{ImageData, MediaProvider},
    Subtitles,
};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{
    fframes_logger::FFramesLogger,
    renderer_error::{FFramesError, FFramesResult},
};

pub(crate) fn load_media_from_folder(
    logger: &Arc<dyn FFramesLogger>,
    folder_path: &str,
) -> FFramesResult<(
    MediaProvider,
    HashMap<String, Arc<usvgr::PreloadedImageData>>,
)> {
    let audio_hash = Arc::new(Mutex::new(HashMap::new()));
    let subtitles_hash = Arc::new(Mutex::new(HashMap::new()));
    let image_hash = Arc::new(Mutex::new(HashMap::new()));
    let fonts_hash = Arc::new(Mutex::new(HashMap::new()));
    let usvgr_image_data = Arc::new(Mutex::new(HashMap::new()));

    let folder_path = Path::new(folder_path);
    if !folder_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, // todo change to NotADirectory when this https://github.com/rust-lang/rust/issues/86442 will be stable
            "resources_dir must be a folder",
        )
        .into());
    }

    let media_files = fs::read_dir(folder_path)?
        .filter_map(|path_buf| {
            path_buf.ok().and_then(|path_buf| {
                let path = path_buf.path();
                if path.is_dir() {
                    None
                } else {
                    Some(path)
                }
            })
        })
        .collect::<Vec<_>>();

    logger.init_media_processing(media_files.len());

    media_files
        .into_par_iter()
        .try_for_each(|path| -> FFramesResult<()> {
            if let Some(filename) = path.file_name().and_then(|os_str| os_str.to_str()) {
                logger.log_media_processing_start(filename, &path);

                match filename {
                    filename if filename.ends_with(".mp3") => {
                        let (sample_rate, samples) = media_loader::decode_mp3(&path);

                        audio_hash.lock().unwrap().insert(
                            filename.to_owned(),
                            audio_data::AudioData::Preloaded(audio_data::PreloadedAudioData {
                                sample_rate,
                                samples,
                            }),
                        );
                    }
                    filename if filename.ends_with(".vtt") => {
                        subtitles_hash
                            .lock()
                            .unwrap()
                            .insert(filename.to_owned(), Subtitles::from_file(&path)?);
                    }
                    filename if filename.ends_with(".ttf") || filename.ends_with(".woff") => {
                        if let Some(font_path) = path.to_str() {
                            fonts_hash
                                .lock()
                                .unwrap()
                                .insert(filename.to_owned(), font_path.to_owned());
                        }
                    }
                    filename
                        if filename.ends_with(".png")
                            || filename.ends_with(".jpg")
                            || filename.ends_with(".jpeg") =>
                    {
                        let data = fs::read(&path)?;
                        let buffer = image::load_from_memory(data.as_slice())
                            .map_err(|e| FFramesError::ImageError((filename.to_owned(), e)))?;

                        usvgr_image_data.lock().unwrap().insert(
                            filename.to_owned(),
                            usvgr::PreloadedImageData::new(
                                if filename.ends_with(".png") {
                                    "png".to_owned()
                                } else {
                                    "jpeg".to_owned()
                                },
                                buffer.width(),
                                buffer.height(),
                                buffer.to_rgba8().into_raw(),
                            ),
                        );

                        image_hash.lock().unwrap().insert(
                            filename.to_owned(),
                            ImageData {
                                link: filename.to_owned(),
                                base64: None,
                            },
                        );
                    }
                    filename if filename == ".DS_Store" => (),
                    filename => {
                        logger.log_unprocessed_media_file(filename);
                    }
                };
            };

            logger.log_processed_media(&path);
            Ok(())
        })?;

    let audio = audio_hash.lock().unwrap();
    let images = image_hash.lock().unwrap();
    let subtitles = subtitles_hash.lock().unwrap();
    let fonts = fonts_hash.lock().unwrap();
    let usvgr_image_data = usvgr_image_data.lock().unwrap();

    Ok((
        MediaProvider {
            audio: audio.clone(),
            fonts: fonts.clone(),
            images: images.clone(),
            subtitles: subtitles.clone(),
        },
        usvgr_image_data.clone(),
    ))
}
