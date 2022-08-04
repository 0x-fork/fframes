#[macro_export]
macro_rules! setup_wasm_editor {
    ($x:tt) => {
        #[wasm_bindgen(module = "fframes-editor")]
        extern "C" {
            #[wasm_bindgen(catch)]
            async fn load_audio_wasm_callback(path: &str) -> Result<JsValue, JsValue>;
        }

        lazy_static! {
            static ref VIDEO: $x = $x::make();
            static ref AUDIO_MAP: Mutex<Option<HashMap<&'static str, (usize, usize)>>> = {
                Mutex::new($x::audio(&VIDEO).0.map(|audio_map| {
                    audio_map
                        .into_iter()
                        .map(|(name, (start_ts, end_ts))| {
                            let start = audio_ts_to_frame(start_ts, name);
                            let end = audio_ts_to_frame(end_ts, name);

                            (name, (start, start + end))
                        })
                        .collect::<HashMap<_, _>>()
                }))
            };
            static ref AUDIO_DURATIONS: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
            static ref MEDIA_PROVIDER: Mutex<fframes::media_provider::MediaProvider> =
                Mutex::new(fframes::media_provider::MediaProvider {
                    audio: HashMap::new(),
                    images: HashMap::new(),
                    subtitles: HashMap::new(),
                    fonts: HashMap::new()
                });
        }

        #[wasm_bindgen]
        pub struct VideoMetadata {
            duration: i32,
        }

        fn audio_ts_to_frame(audio_ts: AudioTimestamp, name: &str) -> usize {
            match audio_ts {
                AudioTimestamp::Frame(frame) => frame,
                AudioTimestamp::Eof => AUDIO_DURATIONS
                    .lock()
                    .unwrap()
                    .get(&name.to_owned())
                    .map(ToOwned::to_owned)
                    .unwrap_or_default() as usize,
                AudioTimestamp::Second(second) => second * $x::FPS,
            }
        }

        #[wasm_bindgen]
        impl VideoMetadata {
            #[wasm_bindgen(getter, js_name=durationInFrames)]
            pub fn duration_in_frames(&self) -> i32 {
                self.duration
            }

            #[wasm_bindgen(getter)]
            pub fn height(&self) -> f64 {
                $x::HEIGHT as f64
            }

            #[wasm_bindgen(getter)]
            pub fn width(&self) -> f64 {
                $x::WIDTH as f64
            }

            #[wasm_bindgen(getter = name)]
            pub fn name(&self) -> String {
                type_name::<$x>().to_owned()
            }

            #[wasm_bindgen(getter, js_name = audioMap)]
            /// Returns a resolved audio_map with audio timestamps converted to frames.
            /// The audio timestamp fallbacks to 0 if audio not loaded yet
            /// @returns {Record<string, [number, number]>}
            pub fn audio_map(&self) -> JsValue {
                JsValue::from_serde(&AUDIO_MAP.lock().unwrap().clone()).unwrap()
            }

            #[wasm_bindgen(getter)]
            pub fn fps(&self) -> f64 {
                $x::FPS as f64
            }
        }

        async fn get_duration_frames() -> i32 {
            match $x::DURATION {
                fframes::Duration::Frames(frames) => frames as i32,
                fframes::Duration::Seconds(seconds) => (seconds * $x::FPS) as i32,
                fframes::Duration::FromAudio(audio) => unsafe {
                    let duration_in_frames = (load_audio_wasm_callback(audio)
                        .await
                        .expect("Can not get the duration of audio")
                        .as_f64()
                        .expect("Can not convert the duration of audio to f64")
                        * $x::FPS as f64) as i32;

                    let mut durations_hash = AUDIO_DURATIONS.lock().unwrap();
                    durations_hash.insert(audio.to_owned(), duration_in_frames);

                    duration_in_frames
                },
            }
        }

        #[wasm_bindgen]
        pub async fn prepare() -> Result<VideoMetadata, JsValue> {
            console_error_panic_hook::set_once();

            let duration = get_duration_frames().await;
            Ok(VideoMetadata { duration })
        }

        #[wasm_bindgen]
        pub fn add_audio_source(file: String, input: &[i16]) {
            let audio_data = fframes::audio_data::AudioData::Preloaded(
                fframes::audio_data::PreloadedAudioData {
                    sample_rate: 44100,
                    samples: input.to_vec(),
                },
            );

            let mut media_provider = MEDIA_PROVIDER.lock().unwrap();
            media_provider.audio.insert(file.clone(), audio_data);

            let duration_in_frames = input.len() as f64 / 44100 as f64 * $x::FPS as f64;
            let mut durations_hash = AUDIO_DURATIONS.lock().unwrap();
            durations_hash.insert(file, duration_in_frames as i32);
        }

        #[wasm_bindgen]
        pub fn add_subtitles_source(file: String, content: String) -> usize {
            let parsed_subtitle = Subtitles::from_str(content.as_str());
            let phrases_count = parsed_subtitle.get_phrases_count();

            let mut media_provider = MEDIA_PROVIDER
                .lock()
                .unwrap()
                .subtitles
                .insert(file, parsed_subtitle);

            phrases_count
        }

        #[wasm_bindgen]
        pub fn add_image_source(file: String, url: String) {
            let mut media_provider = MEDIA_PROVIDER.lock().unwrap();
            media_provider
                .images
                .insert(file, (url, fframes::media_provider::ImageData::None));
        }

        #[wasm_bindgen]
        pub fn render_frame(frame: i64) -> String {
            VIDEO.render_frame(
                &frame::Frame {
                    fps: $x::FPS,
                    index: frame,
                },
                &fframes_context::FFramesContext {
                    mode: fframes_context::FFramesMode::Editor,
                    fps: $x::FPS,
                    sample_rate: 44100,
                    media_provider: MEDIA_PROVIDER.lock().unwrap().clone(),
                },
            )
        }
    };
}
