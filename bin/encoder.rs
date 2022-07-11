use ffmpeg_next::{channel_layout, sys::*};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    slice::from_raw_parts,
};

use crate::{
    ffmpeg_action, ffmpeg_loggable_action,
    renderer_error::{self, AVError, AVResult},
};

#[inline(always)]
#[allow(non_snake_case)]
pub const fn FFMPEG_AVERROR(e: std::os::raw::c_int) -> std::os::raw::c_int {
    -e
}

extern "C" {
    pub fn av_error_to_string(err: i32) -> *mut c_char;
    pub fn fill_yuv_image(frame: *mut AVFrame, frame_index: i32, width: i32, height: i32) -> i32;
    pub fn make_stereo_layout_channel(c: *mut AVCodecContext, codec: *mut AVCodec) -> i32;
}

#[derive(Debug, Clone)]
pub struct EncoderOptions<'a> {
    pub preferred_codec: &'a str,
}

impl Default for EncoderOptions<'_> {
    fn default() -> Self {
        Self {
            preferred_codec: "libx264",
        }
    }
}

#[derive(Clone, Copy)]
pub enum StreamVariant {
    Video,
    Audio,
}

#[derive(Clone, Copy)]
pub struct Stream {
    pub(crate) st: *mut AVStream,
    pub(crate) enc: *mut AVCodecContext,
    pub(crate) variant: StreamVariant,
}

impl Stream {
    unsafe fn free(mut self) {
        avcodec_free_context(&mut self.enc);
    }

    pub unsafe fn get_frames_in_stream(&self) -> i64 {
        (*self.st).nb_frames
    }

    unsafe fn make(
        preferred_codec_name: &str,
        codec_id: AVCodecID,
        oc: *mut AVFormatContext,
    ) -> Result<
        (*mut AVCodec, AVCodecID, *mut AVStream, *mut AVCodecContext),
        Result<Stream, AVError>,
    > {
        let codec_name = CString::new(preferred_codec_name).unwrap();
        let mut codec = avcodec_find_encoder_by_name(codec_name.as_ptr());
        if codec.is_null() {
            codec = avcodec_find_encoder(codec_id);

            let found_codec_name = CStr::from_ptr((*codec).name);
            eprintln!(
                "Warning: Can not find codec {preferred_codec_name}, continue with {codec_name}",
                codec_name = found_codec_name.to_str().unwrap()
            );
        }
        let codec_id = (*codec).id;

        let st = avformat_new_stream(oc, std::ptr::null_mut());
        (*st).id = ((*oc).nb_streams - 1) as i32;
        let c = avcodec_alloc_context3(codec);
        if c.is_null() {
            return Err(Err(AVError::CantAllocateCtx));
        }
        Ok((codec, codec_id, st, c))
    }

    pub(crate) unsafe fn make_video(
        width: i32,
        height: i32,
        fps: i32,
        oc: *mut AVFormatContext,
        preferred_codec_name: &str,
        codec_id: AVCodecID,
    ) -> AVResult<Self> {
        let (codec, codec_id, st, c) = match Self::make(preferred_codec_name, codec_id, oc) {
            Ok(value) => value,
            Err(value) => return value,
        };

        (*c).codec_id = codec_id;
        (*c).width = width;
        (*c).height = height;
        (*st).time_base = AVRational { num: 1, den: fps };
        (*c).time_base = (*st).time_base;

        (*c).gop_size = 40;
        (*c).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;
        (*c).qmin = 10;
        (*c).qmax = 51;
        (*c).qcompress = 0.6;
        (*c).max_qdiff = 4;
        (*c).bit_rate_tolerance = 0;

        if (*(*oc).oformat).flags & AVFMT_GLOBALHEADER != 0 {
            (*(*oc).oformat).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
        }

        let opts: *mut *mut AVDictionary = &mut std::ptr::null_mut();

        let crf = CString::new("crf").unwrap();
        let crfval = CString::new("23").unwrap();
        av_dict_set(opts, crf.as_ptr(), crfval.as_ptr(), 0);

        ffmpeg_loggable_action!(avcodec_open2(c, codec, opts));
        ffmpeg_loggable_action!(avcodec_parameters_from_context((*st).codecpar, c));

        Ok(Stream {
            st,
            enc: c,
            variant: StreamVariant::Video,
        })
    }

    pub(crate) unsafe fn make_audio(
        sample_rate: i32,
        oc: *mut AVFormatContext,
        preferred_codec_name: &str,
        codec_id: AVCodecID,
    ) -> AVResult<Self> {
        let (codec, codec_id, st, c) = match Self::make(preferred_codec_name, codec_id, oc) {
            Ok(value) => value,
            Err(value) => return value,
        };

        let a = from_raw_parts((*codec).sample_fmts, 4);
        (*c).sample_fmt = AVSampleFormat::AV_SAMPLE_FMT_FLTP;
        (*c).sample_rate = sample_rate;
        (*st).time_base = AVRational {
            num: 1,
            den: sample_rate,
        };
        // TODO verify that codec supports 44100 sample_rate
        make_stereo_layout_channel(c, codec);

        // TODO pass user options
        let opts: *mut *mut AVDictionary = &mut std::ptr::null_mut();

        ffmpeg_loggable_action!(avcodec_open2(c, codec, opts));
        ffmpeg_loggable_action!(avcodec_parameters_from_context((*st).codecpar, c));

        Ok(Stream {
            st,
            enc: c,
            variant: StreamVariant::Audio,
        })
    }
}

pub struct Encoder {
    pub(crate) video_stream: Stream,
    pub(crate) audio_stream: Option<Stream>,
    pub(crate) b_frames_count: i32,
    pub(crate) oc: *mut AVFormatContext,
}

impl Encoder {
    pub unsafe fn with_output<T, F: FnMut(&mut Encoder) -> T>(
        width: i32,
        height: i32,
        fps: i32,
        filename: &str,
        preferred_codec: &str,
        function: &mut F,
    ) -> AVResult<T> {
        av_log_set_level(AV_LOG_FATAL);

        let c_filename = CString::new(filename).unwrap();
        let mut oc: *mut AVFormatContext = std::ptr::null_mut();

        ffmpeg_action!(
            avformat_alloc_output_context2(
                &mut oc,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                c_filename.as_ptr(),
            ),
            AVError::UnknownExtension(filename.to_owned())
        );

        let fmt = (*oc).oformat;
        let video_stream =
            Stream::make_video(width, height, fps, oc, preferred_codec, (*fmt).video_codec)?;

        av_dump_format(oc, 0, c_filename.as_ptr(), 1);

        ffmpeg_action!(
            avio_open(&mut (*oc).pb, c_filename.as_ptr(), 2),
            AVError::CantOpenFile(filename.to_owned())
        );

        avformat_write_header(oc, std::ptr::null_mut());

        let mut encoder = Encoder {
            b_frames_count: 0,
            video_stream,
            oc,
            audio_stream: None,
        };

        let res = function(&mut encoder);

        avcodec_send_frame(video_stream.enc, std::ptr::null_mut());
        av_write_trailer(oc);

        video_stream.free();

        avio_closep(&mut (*oc).pb);
        avformat_free_context(oc);

        Ok(res)
    }

    pub unsafe fn send_customizeable_frame_packet<F: Fn(*mut AVPacket) -> i32>(
        &mut self,
        stream: &Stream,
        frame: EncoderFrame,
        customize_frame: F,
    ) -> AVResult<()> {
        let frame = frame.0;
        let avcodec_send_frame = avcodec_send_frame(stream.enc, frame);
        let mut status = avcodec_send_frame;
        if status == FFMPEG_AVERROR(EAGAIN) {
            self.b_frames_count += 1;
        }

        if status < 0 {
            let error_description = av_error_to_string(status);

            return Err(renderer_error::AVError::CantWriteFrame(
                CString::from_raw(error_description)
                    .to_str()
                    .unwrap_or("Unknown libav error.")
                    .to_owned(),
            ));
        }

        let packet = av_packet_alloc();
        while status >= 0 {
            status = avcodec_receive_packet(stream.enc, packet);

            match status {
                status if status == AVERROR_EOF => break,
                status if status == FFMPEG_AVERROR(EAGAIN) => {
                    self.b_frames_count += 1;
                    break;
                }
                _ => status = customize_frame(packet),
            }
        }

        Ok(())
    }

    pub unsafe fn send_frame(&mut self, stream: &Stream, frame: EncoderFrame) -> AVResult<()> {
        let video_stream = self.video_stream.st;
        let oc = self.oc;

        self.send_customizeable_frame_packet(stream, frame, |packet| {
            av_packet_rescale_ts(packet, (*stream.enc).time_base, (*stream.st).time_base);

            (*packet).stream_index = (*video_stream).index;
            av_interleaved_write_frame(oc, packet)
        })
    }
}

#[derive(Clone, Copy)]
pub struct EncoderFrame(*mut AVFrame);

impl EncoderFrame {
    pub fn make(stream: &Stream) -> Self {
        unsafe {
            let mut frame = av_frame_alloc();

            match stream.variant {
                StreamVariant::Video => {
                    (*frame).format = (*stream.enc).pix_fmt as i32;
                    (*frame).width = (*stream.enc).width;
                    (*frame).height = (*stream.enc).height;
                }
                StreamVariant::Audio => {
                    (*frame).format = (*stream.enc).sample_fmt as i32;
                    (*frame).channel_layout = (*stream.enc).channel_layout;
                    (*frame).sample_rate = (*stream.enc).sample_rate;
                    (*frame).nb_samples = if ((*(*stream.enc).codec).capabilities
                        & AV_CODEC_CAP_VARIABLE_FRAME_SIZE as i32)
                        != 0
                    {
                        10000
                    } else {
                        (*stream.enc).frame_size
                    };
                }
            }

            let response = av_frame_get_buffer(frame, 0);
            assert!(response >= 0, "Could not allocate frame data");

            EncoderFrame(frame)
        }
    }

    pub fn set_index(self, new_index: i64) {
        unsafe {
            (*self.0).pts = new_index;
        }
    }

    #[inline(always)]
    fn get_rgb(pixmap: &[u8], i: usize) -> (i32, i32, i32) {
        let r = pixmap[4 * i] as i32;
        let g = pixmap[4 * i + 1] as i32;
        let b = pixmap[4 * i + 2] as i32;

        (r, g, b)
    }

    pub unsafe fn from_audio_data(&mut self, frame_index: i64, audio_data: &[i16]) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.0);
        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        (*self.0).pts = frame_index;
        if audio_data.len() == 0 {
            return self.0;
        }

        let new_data = audio_data
            .into_iter()
            .map(|data| *data as f32)
            .collect::<Vec<f32>>();

        let mut test =
            std::slice::from_raw_parts(new_data.as_ptr() as *mut u8, new_data.len() * 4).to_vec();

        (*self.0).data[0] = test.as_mut_ptr();

        self.0
    }

    pub unsafe fn from_rgba_pixmap(&mut self, frame_index: i64, rgb_pixels: &[u8]) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.0);
        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        let height = (*self.0).height as usize;
        let width = (*self.0).width as usize;
        let av_frame = self.0;

        let frame_size: usize = height * (*av_frame).linesize[0] as usize + width;
        
        let y_pixels = std::slice::from_raw_parts_mut((*av_frame).data[0], frame_size);
        let cb_pixels = std::slice::from_raw_parts_mut((*av_frame).data[1], frame_size / 4);
        let cr_pixels = std::slice::from_raw_parts_mut((*av_frame).data[2], frame_size / 4);

        let mut y_index = 0;
        let mut cb_cr_pixel_index = 0;

        // fill_yuv_image(av_frame, frame_index as i32, width, height);

        let mut cb_assign = 0;

        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = EncoderFrame::get_rgb(&rgb_pixels, y * width + x);

                y_pixels[(y * (*av_frame).linesize[0] as usize + x) as usize] =
                    (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;
            }
        }

        for y in 0..height / 2 {
            for x in 0..width / 2 {
                let (r, g, b) = EncoderFrame::get_rgb(&rgb_pixels, y * width * 2 + x * 2);

                cb_pixels[(y * (*av_frame).linesize[1] as usize + x) as usize] =
                    (128 + ((-38 * r - 74 * g + 112 * b) >> 8)) as u8;
                cr_pixels[(y * (*av_frame).linesize[2] as usize + x) as usize] =
                    (128 + ((112 * r - 94 * g - 18 * b) >> 8)) as u8;
            }
        }

        // for y in 0..height {
        //     if y % 2 == 0 {
        //         let mut x = 0;
        //         while x < width {
        //             let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);

        //             y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;
        //             y_index += 1;

        //             cb_pixels[cb_cr_pixel_index] =
        //                 (128 + ((-38 * r - 74 * g + 112 * b) >> 8)) as u8;
        //             cr_pixels[cb_cr_pixel_index] = (128 + ((112 * r - 94 * g - 18 * b) >> 8)) as u8;

        //             cb_cr_pixel_index += 1;

        //             let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);
        //             y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;

        //             y_index += 1;

        //             x += 2;
        //         }
        //     } else {
        //         for _x in 0..width {
        //             let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);

        //             y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;
        //             y_index += 1;
        //         }
        //     }
        // }

        (*av_frame).pts = frame_index;
        av_frame
    }

    pub fn free(&mut self) {
        unsafe {
            av_frame_free(&mut self.0);
        }
    }
}

unsafe impl Send for Encoder {}
unsafe impl Sync for Encoder {}
unsafe impl Send for EncoderFrame {}
unsafe impl Sync for EncoderFrame {}
