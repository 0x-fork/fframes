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

use crate::fframes_logger::FFramesLogger;

pub fn load_media_from_folder(
    logger: &Arc<dyn FFramesLogger>,
    folder_path: &str,
) -> io::Result<MediaProvider> {
    let audio_hash = Arc::new(Mutex::new(HashMap::new()));
    let subtitles_hash = Arc::new(Mutex::new(HashMap::new()));
    let image_hash = Arc::new(Mutex::new(HashMap::new()));
    let fonts_hash = Arc::new(Mutex::new(HashMap::new()));

    let folder_path = Path::new(folder_path);

    if !folder_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, // todo change to NotADirectory when this https://github.com/rust-lang/rust/issues/86442 will be stable
            "resources_dir must be a folder",
        ));
    }

    let media_files = fs::read_dir(folder_path)?
        .filter_map(|path_buf| {
            path_buf
                .ok()
                .map(|path_buf| {
                    let path = path_buf.path();
                    if path.is_dir() {
                        None
                    } else {
                        Some(path)
                    }
                })
                .flatten()
        })
        .collect::<Vec<_>>();

    logger.init_media_processing(media_files.len());

    media_files.into_par_iter().for_each(|path| {
        match path.file_name().map(|os_str| os_str.to_str()).flatten() {
            Some(filename) => {
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
                            .insert(filename.to_owned(), Subtitles::from_file(&path));
                    }
                    filename if filename.ends_with(".ttf") || filename.ends_with(".woff") => {
                        path.to_str().map(|font_path| {
                            fonts_hash
                                .lock()
                                .unwrap()
                                .insert(filename.to_owned(), font_path.to_owned());
                        });
                    }
                    filename if filename.ends_with(".png") => {
                        image_hash.lock().unwrap().insert(
                            filename.to_owned(),
                            (
                                filename.to_owned(),
                                ImageData::RawPng(Arc::new(fs::read(&path).unwrap())),
                            ),
                        );
                    }
                    filename if filename.ends_with(".jpg") || filename.ends_with(".jpeg") => {
                        image_hash.lock().unwrap().insert(
                            filename.to_owned(),
                            (
                                filename.to_owned(),
                                ImageData::RawJpg(Arc::new(fs::read(&path).unwrap())),
                            ),
                        );
                    }
                    filename => logger.log_unprocessed_media_file(filename),
                }
            }
            _ => (),
        }

        logger.log_processed_media(&path);
    });

    let audio = audio_hash.lock().unwrap();
    let images = image_hash.lock().unwrap();
    let subtitles = subtitles_hash.lock().unwrap();
    let fonts = fonts_hash.lock().unwrap();

    Ok(MediaProvider {
        audio: audio.clone(),
        fonts: fonts.clone(),
        images: images.clone(),
        subtitles: subtitles.clone(),
    })
}
