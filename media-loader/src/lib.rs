use minimp3::{Decoder as Mp3Decoder, Error as Mp3Error, Frame as Mp3Frame};
use std::fs::File;

pub fn decode_mp3(audio_path: &str) -> (i32, Vec<f32>) {
    let mut decoder = Mp3Decoder::new(File::open(audio_path).unwrap());

    let mut sample_rate = 0;
    let mut mono_samples = Vec::with_capacity(100_000_000_000);
    loop {
        match decoder.next_frame() {
            Ok(Mp3Frame {
                data: samples_of_frame,
                sample_rate: sample_rate_of_frame,
                channels,
                ..
            }) => {
                // Sampling rate can not change over mp3 file
                sample_rate = sample_rate_of_frame;

                match channels {
                    1 => samples_of_frame
                        .iter()
                        .for_each(|sample| mono_samples.push(*sample as f32)),
                    channels => {
                        for (i, sample) in samples_of_frame.iter().enumerate().step_by(channels) {
                            // prevent overflow
                            let i32sample = *sample as f32;

                            mono_samples.push((i32sample + samples_of_frame[i + 1] as f32) / 2.0);
                        }
                    }
                }
            }
            Err(Mp3Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    (sample_rate, mono_samples)
}
