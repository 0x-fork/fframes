use fframes::{audio_data, media_provider::MediaProvider, Subtitles};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs, io,
    path::Path,
    sync::{Arc, Mutex},
};

pub fn load_media_from_folder(folder_path: &str) -> io::Result<MediaProvider> {
    let audio_hash = Arc::new(Mutex::new(HashMap::new()));
    let subtitles_hash = Arc::new(Mutex::new(HashMap::new()));
    let image_hash = Arc::new(Mutex::new(HashMap::new()));

    let folder_path = Path::new(folder_path);

    if !folder_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, // todo change to NotADirectory when this https://github.com/rust-lang/rust/issues/86442 will be stable
            "resources_dir must be a folder",
        ));
    }

    fs::read_dir(folder_path)?
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
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(
            |path| match path.file_name().map(|os_str| os_str.to_str()).flatten() {
                Some(filename) if filename.ends_with(".mp3") => {
                    let (sample_rate, samples) = media_loader::decode_mp3(&path);

                    audio_hash.lock().unwrap().insert(
                        filename.to_owned(),
                        audio_data::AudioData {
                            sample_rate,
                            samples,
                            max_magnitude: 0.0,
                        },
                    );
                }
                Some(filename) if filename.ends_with(".vtt") => {
                    subtitles_hash
                        .lock()
                        .unwrap()
                        .insert(filename.to_owned(), Subtitles::from_file(&path));
                }

                Some(filename) => println!("Can not process resource {filename}"),
                None => (),
            },
        );

    let audio = audio_hash.lock().unwrap();
    let images = image_hash.lock().unwrap();
    let subtitles = subtitles_hash.lock().unwrap();

    Ok(MediaProvider {
        audio: audio.clone(),
        images: images.clone(),
        subtitles: subtitles.clone(),
    })
}
