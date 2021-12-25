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
            static ref AUDIO_CACHE: Mutex<HashMap<String, AudioData>> = Mutex::new(HashMap::new());
            static ref SUBTITLES_CACHE: Mutex<HashMap<String, Subtitles>> =
                Mutex::new(HashMap::new());
        }

        #[wasm_bindgen]
        pub struct VideoMetadata {
            duration: f64,
        }

        #[wasm_bindgen]
        impl VideoMetadata {
            #[wasm_bindgen(getter)]
            pub fn duration(&self) -> f64 {
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
        }

        async fn get_duration_in_seconds() -> f64 {
            match $x::DURATION {
                fframes::Duration::Seconds(seconds) => seconds as f64,
                fframes::Duration::Frames(frames) => (frames * $x::FPS) as f64,
                fframes::Duration::FromAudio(audio) => unsafe {
                    load_audio_wasm_callback(audio)
                        .await
                        .unwrap()
                        .as_f64()
                        .unwrap()
                },
            }
        }

        #[wasm_bindgen]
        pub async fn prepare() -> Result<VideoMetadata, JsValue> {
            console_error_panic_hook::set_once();

            let duration = get_duration_in_seconds().await;
            // ("kek".to_owned(), VideoMetadata {})
            Ok(VideoMetadata { duration })
        }

        #[wasm_bindgen]
        pub fn add_audio_source(file: String, input: &[f32]) {
            let audio_data = AudioData {
                sample_rate: 44100,
                samples: input.to_vec(),
                max_magnitude: input
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                    .unwrap_or(&0.0)
                    .to_owned()
                    .sqrt(),
            };

            let mut audio_cache = AUDIO_CACHE.lock().unwrap();
            audio_cache.insert(file, audio_data);
        }

        #[wasm_bindgen]
        pub fn add_subtitles_source(file: String, content: String) {
            SUBTITLES_CACHE
                .lock()
                .unwrap()
                .insert(file, Subtitles::from_str(content.as_str()));
        }

        #[wasm_bindgen]
        pub fn render_frame(frame: i64) -> String {
            VIDEO.render_frame(
                &Frame::Frame {
                    fps: 30,
                    index: frame,
                },
                FFramesContext::FFramesContext {
                    fps: 30,
                    audio: &AUDIO_CACHE.lock().unwrap(),
                    subtitles: &SUBTITLES_CACHE.lock().unwrap(),
                },
            )
        }
    };
}
