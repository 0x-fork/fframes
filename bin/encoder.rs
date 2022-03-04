use ffmpeg_next::sys::*;
use std::{ffi::CString, os::raw::c_char};

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
}

#[derive(Clone, Copy)]
pub struct Stream {
    st: *mut AVStream,
    enc: *mut AVCodecContext,
}

impl Stream {
    unsafe fn free(mut self) {
        avcodec_free_context(&mut self.enc);
    }

    pub unsafe fn get_frames_in_stream(&self) -> i64 {
        (*self.st).nb_frames
    }

    unsafe fn make(
        width: i32,
        height: i32,
        fps: i32,
        oc: *mut AVFormatContext,
        codec_id: AVCodecID,
    ) -> AVResult<Self> {
        let codec = avcodec_find_encoder(codec_id);
        let st = avformat_new_stream(oc, std::ptr::null_mut());
        (*st).id = ((*oc).nb_streams - 1) as i32;

        let c = avcodec_alloc_context3(codec);
        if c.is_null() {
            return Err(AVError::CantAllocateCtx);
        }

        (*c).codec_id = codec_id;
        (*c).width = width;
        (*c).height = height;
        (*st).time_base = AVRational { num: 1, den: fps };
        (*c).time_base = (*st).time_base;

        (*c).gop_size = 12;
        (*c).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;
        (*c).qmin = 10;
        (*c).qmax = 51;

        if (*(*oc).oformat).flags & AVFMT_GLOBALHEADER != 0 {
            (*(*oc).oformat).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
        }

        let opts: *mut *mut AVDictionary = &mut std::ptr::null_mut();

        let crf = CString::new("crf").unwrap();
        let crfval = CString::new("28").unwrap();
        av_dict_set(opts, crf.as_ptr(), crfval.as_ptr(), 0);

        ffmpeg_loggable_action!(avcodec_open2(c, codec, opts));
        ffmpeg_loggable_action!(avcodec_parameters_from_context((*st).codecpar, c));

        Ok(Stream { st, enc: c })
    }
}

pub struct Encoder {
    pub video_stream: Stream,
    pub b_frames_count: i32,
    oc: *mut AVFormatContext,
}

impl Encoder {
    pub unsafe fn with_output<T, F: FnMut(&mut Encoder) -> T>(
        width: i32,
        height: i32,
        fps: i32,
        filename: &str,
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
        let video_stream = Stream::make(width, height, fps, oc, (*fmt).video_codec)?;

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
        };

        let res = function(&mut encoder);

        avcodec_send_frame(video_stream.enc, std::ptr::null_mut());
        av_write_trailer(oc);

        video_stream.free();

        avio_closep(&mut (*oc).pb);
        avformat_free_context(oc);

        Ok(res)
    }

    pub unsafe fn send_frame(&mut self, frame: *mut AVFrame) -> AVResult<()> {
        let mut status = avcodec_send_frame(self.video_stream.enc, frame);
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

        while status >= 0 {
            let mut packet = av_packet_alloc();
            status = avcodec_receive_packet(self.video_stream.enc, packet);

            match status {
                status if status == AVERROR_EOF => break,
                status if status == FFMPEG_AVERROR(EAGAIN) => {
                    self.b_frames_count += 1;
                    break;
                }
                _ => {
                    av_packet_rescale_ts(
                        packet,
                        (*self.video_stream.enc).time_base,
                        (*self.video_stream.st).time_base,
                    );

                    (*packet).stream_index = (*self.video_stream.st).index;

                    status = av_interleaved_write_frame(self.oc, packet);
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct EncoderFrame {
    height: i32,
    width: i32,
    pub frame: *mut AVFrame,
}

impl EncoderFrame {
    pub fn make(stream: &Stream) -> Self {
        unsafe {
            let mut frame = av_frame_alloc();
            (*frame).format = (*stream.enc).pix_fmt as i32;
            (*frame).width = (*stream.enc).width;
            (*frame).height = (*stream.enc).height;

            let response = av_frame_get_buffer(frame, 0);
            assert!(response >= 0, "Could not allocate frame data");

            EncoderFrame {
                frame,
                width: (*stream.enc).width,
                height: (*stream.enc).height,
            }
        }
    }

    pub fn set_index(self, new_index: i64) {
        unsafe {
            (*self.frame).pts = new_index;
        }
    }

    #[inline(always)]
    fn get_rgb(pixmap: &[u8], i: usize) -> (i32, i32, i32) {
        let r = pixmap[4 * i] as i32;
        let g = pixmap[4 * i + 1] as i32;
        let b = pixmap[4 * i + 2] as i32;

        (r, g, b)
    }

    pub unsafe fn from_rgba_pixmap(&self, frame_index: i64, rgb_pixels: &[u8]) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.frame);
        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        let frame_size: usize = self.height as usize * self.width as usize;
        let y_pixels = std::slice::from_raw_parts_mut((*self.frame).data[0], frame_size);
        let cb_pixels = std::slice::from_raw_parts_mut((*self.frame).data[1], frame_size / 2);
        let cr_pixels = std::slice::from_raw_parts_mut((*self.frame).data[2], frame_size / 2);

        let mut y_index = 0;
        let mut cb_cr_pixel_index = 0;

        for y in 0..self.height {
            if y % 2 == 0 {
                let mut x = 0;
                while x < self.width {
                    let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);

                    y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;
                    y_index += 1;

                    cb_pixels[cb_cr_pixel_index] =
                        (128 + ((-38 * r - 74 * g + 112 * b) >> 8)) as u8;
                    cr_pixels[cb_cr_pixel_index] = (128 + ((112 * r - 94 * g - 18 * b) >> 8)) as u8;

                    cb_cr_pixel_index += 1;

                    let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);
                    y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;

                    y_index += 1;

                    x += 2;
                }
            } else {
                for _x in 0..self.width {
                    let (r, g, b) = Self::get_rgb(rgb_pixels, y_index);

                    y_pixels[y_index] = (16 + (66 * r + 129 * g + 25 * b) >> 8) as u8;
                    y_index += 1;
                }
            }
        }

        (*self.frame).pts = frame_index;
        self.frame
    }

    pub unsafe fn get_video_frame(&self, i: i64) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.frame);

        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        let y_size = self.height * (*self.frame).linesize[0] + self.width;
        let cb_cr_size = self.height * (*self.frame).linesize[1] + self.width;

        let y_pixels = std::slice::from_raw_parts_mut((*self.frame).data[0], y_size as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                y_pixels[(y * (*self.frame).linesize[0] + x) as usize] =
                    (x + y + i as i32 * 3) as u8;
            }
        }

        let cb_pixels = std::slice::from_raw_parts_mut((*self.frame).data[1], cb_cr_size as usize);
        let cr_pixels = std::slice::from_raw_parts_mut((*self.frame).data[2], cb_cr_size as usize);

        for y in 0..self.height / 2 {
            for x in 0..self.width / 2 {
                cb_pixels[(y * (*self.frame).linesize[1] + x) as usize] =
                    (128 + y + i as i32 * 2) as u8;
                cr_pixels[(y * (*self.frame).linesize[2] + x) as usize] =
                    (64 + x + i as i32 * 5) as u8;
            }
        }

        (*self.frame).pts = i + 1;

        self.frame
    }

    fn free(&mut self) {
        unsafe {
            av_frame_free(&mut self.frame);
        }
    }
}

unsafe impl Send for Encoder {}
unsafe impl Sync for Encoder {}
unsafe impl Send for EncoderFrame {}
unsafe impl Sync for EncoderFrame {}
