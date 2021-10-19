use ffmpeg_next::sys::*;
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Clone, Copy)]
struct Stream {
    st: *mut AVStream,
    enc: *mut AVCodecContext,
    frame: *mut AVFrame,
}

const FPS: i32 = 30;

impl Stream {
    unsafe fn free(mut self) {
        avcodec_free_context(&mut self.enc);
        av_frame_free(&mut self.frame);
    }

    unsafe fn make(oc: *mut AVFormatContext, codec_id: AVCodecID) -> Self {
        let codec = avcodec_find_encoder(codec_id);
        let st = avformat_new_stream(oc, std::ptr::null_mut());
        (*st).id = ((*oc).nb_streams - 1) as i32;

        let c = avcodec_alloc_context3(codec);
        assert!(!c.is_null(), "Could not alloc an encoding context");

        (*c).codec_id = codec_id;
        (*c).bit_rate = 400000;
        (*c).width = 1920;
        (*c).height = 1080;
        (*st).time_base = AVRational { num: 1, den: FPS };
        (*c).time_base = (*st).time_base;

        (*c).gop_size = 12;
        (*c).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;

        if (*(*oc).oformat).flags & AVFMT_GLOBALHEADER != 0 {
            (*(*oc).oformat).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
        }

        let opts: *mut *mut AVDictionary = &mut std::ptr::null_mut();
        let response = avcodec_open2(c, codec, opts);
        assert!(response >= 0, "Could not open videeo codec");

        let mut frame = av_frame_alloc();
        (*frame).format = (*c).pix_fmt as i32;
        (*frame).width = (*c).width;
        (*frame).height = (*c).height;

        let response = av_frame_get_buffer(frame, 0);
        assert!(response >= 0, "Could not allocate frame data");

        let response = avcodec_parameters_from_context((*st).codecpar, c);
        assert!(response >= 0, "Could not copy the stream parameters");

        Stream { st, enc: c, frame }
    }

    unsafe fn get_video_frame(&mut self, i: i32) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.frame);

        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        let y_size = (*self.enc).height * (*self.frame).linesize[0] + (*self.enc).width;
        let cb_cr_size = (*self.enc).height * (*self.frame).linesize[1] + (*self.enc).width;

        let y_pixels = std::slice::from_raw_parts_mut((*self.frame).data[0], y_size as usize);

        for y in 0..(*self.enc).height {
            for x in 0..(*self.enc).width {
                y_pixels[(y * (*self.frame).linesize[0] + x) as usize] = (x + y + i * 3) as u8;
            }
        }

        let cb_pixels = std::slice::from_raw_parts_mut((*self.frame).data[1], cb_cr_size as usize);
        let cr_pixels = std::slice::from_raw_parts_mut((*self.frame).data[2], cb_cr_size as usize);

        for y in 0..(*self.enc).height / 2 {
            for x in 0..(*self.enc).width / 2 {
                cb_pixels[(y * (*self.frame).linesize[1] + x) as usize] = (128 + y + i * 2) as u8;
                cr_pixels[(y * (*self.frame).linesize[2] + x) as usize] = (64 + x + i * 5) as u8;
            }
        }

        (*self.frame).pts = i as i64;

        self.frame
    }
}

#[inline(always)]
pub const fn FFMPEG_AVERROR(e: std::os::raw::c_int) -> std::os::raw::c_int {
    -e
}

pub unsafe fn test() {
    let filename = CString::new("new.mp4").unwrap();
    let mut oc: *mut AVFormatContext = std::ptr::null_mut();

    let alloc_res = avformat_alloc_output_context2(
        &mut oc,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        filename.as_ptr(),
    );

    assert!(
        alloc_res >= 0,
        "Could not deduce output format from file extension: using MPEG.\n"
    );

    let fmt = (*oc).oformat;
    let mut stream = Stream::make(oc, (*fmt).video_codec);

    av_dump_format(oc, 0, filename.as_ptr(), 1);
    let ret = avio_open(&mut (*oc).pb, filename.as_ptr(), 2);
    assert!(
        ret >= 0,
        "Could not open output file {filename}",
        filename = filename.to_str().unwrap_or_default()
    );

    avformat_write_header(oc, std::ptr::null_mut());

    (0..1000).for_each(|fr| {
        let frame = stream.get_video_frame(fr);
        let mut status: i32 = avcodec_send_frame(stream.enc, frame);

        while status >= 0 {
            let mut pkt = av_packet_alloc();
            status = avcodec_receive_packet(stream.enc, pkt);

            match status {
                status if status == AVERROR_EOF || status == FFMPEG_AVERROR(EAGAIN) => break,
                status if status < 0 => break,
                _ => {
                    av_packet_rescale_ts(pkt, (*stream.enc).time_base, (*stream.st).time_base);
                    (*pkt).stream_index = (*stream.st).index;

                    status = av_interleaved_write_frame(oc, pkt);
                    av_packet_unref(pkt);
                }
            }
        }
    });

    avcodec_send_frame(stream.enc, std::ptr::null_mut());
    av_write_trailer(oc);

    stream.free();

    avio_closep(&mut (*oc).pb);
    avformat_free_context(oc);
}
