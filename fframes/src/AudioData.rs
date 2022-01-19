use crate::{FFramesContext, Frame, WindowFunctions};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct AudioData {
    pub max_magnitude: f32,
    pub samples: Vec<f32>,
    pub sample_rate: i32,
}

pub enum SampleSize {
    S2,
    S4,
    S8,
    S16,
    S32,
    S64,
    S128,
    S256,
    S512,
    S1024,
}

fn get_fft_size_number(variant: &SampleSize) -> usize {
    match variant {
        SampleSize::S2 => 2,
        SampleSize::S4 => 4,
        SampleSize::S8 => 8,
        SampleSize::S16 => 16,
        SampleSize::S32 => 32,
        SampleSize::S64 => 64,
        SampleSize::S128 => 128,
        SampleSize::S256 => 256,
        SampleSize::S512 => 512,
        SampleSize::S1024 => 1024,
    }
}

pub struct VisualizeFrameInput<'a> {
    pub audio: &'a AudioData,
    pub sample_size: SampleSize,
    pub ctx: &'a FFramesContext::FFramesContext<'a>,
}

fn apply_fft_to_frame(
    sample_size: &SampleSize,
    window: Option<WindowFunctions::Window>,
    start_index: usize,
    samples: &Vec<f32>,
) -> Vec<microfft::Complex32> {
    let apply_window = |samples: &[f32]| -> Vec<f32> {
        if let Some(window_function) = window {
            WindowFunctions::apply_window_function(window_function, samples)
        } else {
            samples.to_vec()
        }
    };

    match sample_size {
        SampleSize::S2 => {
            let mut buffer: [_; 2] = apply_window(&samples[start_index..start_index + 2])
                .try_into()
                .unwrap();
            microfft::real::rfft_2(&mut buffer).to_vec()
        }
        SampleSize::S4 => {
            let mut buffer: [_; 4] = apply_window(&samples[start_index..start_index + 4])
                .try_into()
                .unwrap();
            microfft::real::rfft_4(&mut buffer).to_vec()
        }
        SampleSize::S8 => {
            let mut buffer: [_; 8] = apply_window(&samples[start_index..start_index + 8])
                .try_into()
                .unwrap();
            microfft::real::rfft_8(&mut buffer).to_vec()
        }
        SampleSize::S16 => {
            let mut buffer: [_; 16] = apply_window(&samples[start_index..start_index + 16])
                .try_into()
                .unwrap();
            microfft::real::rfft_16(&mut buffer).to_vec()
        }
        SampleSize::S32 => {
            let mut buffer: [_; 32] = apply_window(&samples[start_index..start_index + 32])
                .try_into()
                .unwrap();

            microfft::real::rfft_32(&mut buffer).to_vec()
        }
        SampleSize::S64 => {
            let mut buffer: [_; 64] = apply_window(&samples[start_index..start_index + 64])
                .try_into()
                .unwrap();
            microfft::real::rfft_64(&mut buffer).to_vec()
        }
        SampleSize::S128 => {
            let mut buffer: [_; 128] = apply_window(&samples[start_index..start_index + 128])
                .try_into()
                .unwrap();

            microfft::real::rfft_128(&mut buffer).to_vec()
        }
        SampleSize::S256 => {
            let mut buffer: [_; 256] = apply_window(&samples[start_index..start_index + 256])
                .try_into()
                .unwrap();
            microfft::real::rfft_256(&mut buffer).to_vec()
        }
        SampleSize::S512 => {
            let mut buffer: [_; 512] = apply_window(&samples[start_index..start_index + 512])
                .try_into()
                .unwrap();
            microfft::real::rfft_512(&mut buffer).to_vec()
        }
        SampleSize::S1024 => {
            let mut buffer: [_; 1024] = apply_window(&samples[start_index..start_index + 1024])
                .try_into()
                .unwrap();
            microfft::real::rfft_1024(&mut buffer).to_vec()
        }
    }
}

fn convert_fft_result_to_magnitude(num: &microfft::Complex32) -> f32 {
    let magnitude = (num.re * num.re + num.im + num.im).sqrt();

    magnitude
}

pub fn get_visualization(
    frame: &i64,
    VisualizeFrameInput {
        sample_size,
        ctx,
        audio,
    }: &VisualizeFrameInput,
) -> Vec<f32> {
    // if ctx.fft_hash.contains_key(frame) {
    //     return ctx.fft_hash.get(frame).unwrap().to_owned();
    // }

    let sample_start = *frame as i128 * audio.sample_rate as i128 / ctx.fps as i128;
    let fft_size = get_fft_size_number(sample_size);


    let res = apply_fft_to_frame(
        &sample_size,
        Some(WindowFunctions::Window::Hamming),
        sample_start as usize,
        &audio.samples,
    )
    .iter()
    .map(|x| convert_fft_result_to_magnitude(x) / fft_size as f32)
    .collect::<Vec<f32>>();

    res
}

pub fn visualize_audio_frame(frame: &Frame::Frame, input: &VisualizeFrameInput) -> Vec<f32> {
    let res = match frame.index {
        0 => get_visualization(&frame.index, input),
        1 => get_visualization(&frame.index, input),
        2 => get_visualization(&frame.index, input),
        frame => {
            let frames_to_smooth = [
                get_visualization(&(frame - 1), input),
                get_visualization(&frame, input),
                get_visualization(&(frame + 1), input),
            ];

            (0..frames_to_smooth[1].len())
                .into_iter()        
                .map(|frame| {
                    frames_to_smooth.iter().map(|arr| arr[frame]).sum::<f32>()
                        / frames_to_smooth.len() as f32
                })
                .collect()
        }
    };

    res
}
