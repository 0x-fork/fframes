use ffmpeg_next::sys::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;

#[derive(Clone, Copy)]
struct Stream {
    st: *mut AVStream,
    enc: *mut AVCodecContext,
    frame: *mut AVFrame,
    samples_count: i32,
}

const FPS: i32 = 30;

impl Stream {
    unsafe fn free(mut self) {
        avcodec_free_context(&mut self.enc);
        av_frame_free(&mut self.frame);
    }

    unsafe fn make(oc: *mut AVFormatContext, codec_id: AVCodecID) -> Self {
        let codec = avcodec_find_encoder(codec_id);
        let c = avcodec_alloc_context3(codec);
        let st = avformat_new_stream(oc, codec);

        (*st).id = ((*oc).nb_streams - 1) as i32;
        (*st).time_base = AVRational { num: 1, den: FPS };

        (*c).codec_id = codec_id;
        (*c).bit_rate = 400000;
        (*c).height = 352;
        (*c).width = 288;
        (*c).gop_size = 12;
        (*c).pix_fmt = AVPixelFormat::AV_PIX_FMT_YUV420P;

        (*c).time_base = (*st).time_base;

        let mut frame = av_frame_alloc();
        (*frame).format = (*c).pix_fmt as i32;
        (*frame).width = 352;
        (*frame).height = 288;

        av_frame_get_buffer(frame, 0);

        let opts: *mut *mut AVDictionary = &mut std::ptr::null_mut();
        avcodec_open2(c, codec, opts);

        avcodec_parameters_from_context((*st).codecpar, c);
        Stream {
            st,
            enc: c,
            frame,
            samples_count: 0,
        }
    }

    unsafe fn get_video_frame(self, i: i32) -> *mut AVFrame {
        let is_writable = av_frame_make_writable(self.frame);

        if is_writable < 0 {
            panic!("Can not reuse frame allocations");
        }

        // fill_yuv_image(self.frame, i, (*self.enc).width, (*self.enc).height);

        for y in 0..(*self.enc).height {
            for x in 0..(*self.enc).width {
                let slice = std::slice::from_raw_parts_mut((*self.frame).data[0], 999999);

                slice[(y * (*self.frame).linesize[0] + x) as usize] = (x + y + i * 3) as u8;

                // set_custom_ptr_value(
                //     (*stream.frame).data[0].offset((y * (*stream.frame).linesize[0] + x) as isize),
                //     ,
                // );
            }
        }

        // println!("{:?}", (*stream.frame).data);

        for y in 0..(*self.enc).height / 2 {
            for x in 0..(*self.enc).width / 2 {
                let c = std::slice::from_raw_parts_mut((*self.frame).data[1], 999999);
                let b = std::slice::from_raw_parts_mut((*self.frame).data[2], 999999);

                c[(y * (*self.frame).linesize[1] + x) as usize] = (128 + y + i * 2) as u8;
                b[(y * (*self.frame).linesize[2] + x) as usize] = (64 + y + i * 5) as u8;
            }
        }

        // println!(
        //     "{:?}",
        //     std::slice::from_raw_parts_mut((*stream.frame).data[0], 2000)
        // );

        (*self.frame).pts = i as i64;

        self.frame
    }
}

fn set_custom_ptr_value<T>(mut pointer: *mut T, val: *mut u8) -> *mut T {
    let thin = &mut pointer as *mut *mut T as *mut *mut u8;
    // SAFETY: In case of a thin pointer, this operations is identical
    // to a simple assignment. In case of a fat pointer, with the current
    // fat pointer layout implementation, the first field of such a
    // pointer is always the data pointer, which is likewise assigned.
    unsafe { *thin = val };
    pointer
}

unsafe fn set_arr(data: *mut u8, offset: isize, value: u8) {
    let ptr = data.offset(offset as isize) as *mut u8;
    *ptr = value;
}

extern "C" {
    fn fill_yuv_image(pict: *mut AVFrame, frame_index: i32, width: i32, height: i32);
    fn send_frame(c: *mut AVCodecContext, frame: *mut AVFrame) -> i32;
    fn create_video(filename: *const c_char);
}

#[test]
fn kek() {
    unsafe {
        test();
    }

    assert!(1 == 1);
}

#[inline(always)]
pub const fn FFMPEG_AVERROR(e: std::os::raw::c_int) -> std::os::raw::c_int {
    -e
}

pub unsafe fn test() {
    let filename = CString::new("my.mp4").unwrap();
    let test_filename = CString::new("raw_c.mp4").unwrap();

    create_video(test_filename.as_ptr());
    // panic!("FUCK OFF");

    let mut oc: *mut AVFormatContext = std::ptr::null_mut();

    let alloc_res = avformat_alloc_output_context2(
        &mut oc,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        filename.as_ptr(),
    );

    if alloc_res < 0 {
        panic!("Could not determine output format from file extension, please ensure you passed correct output file.");
    }

    let fmt = (*oc).oformat;
    let stream = Stream::make(oc, (*fmt).video_codec);
    
    avio_open(&mut (*oc).pb, filename.as_ptr(), 2);
    avformat_write_header(oc, std::ptr::null_mut());

    (0..1000).for_each(|fr| {
        println!("status {}", fr);
        let mut status: i32 = avcodec_send_frame(stream.enc, stream.get_video_frame(fr));

        while status >= 0 {
            let mut pkt = av_packet_alloc();
            status = avcodec_receive_packet(stream.enc, pkt);

            match status {
                status if status == AVERROR_EOF || status == FFMPEG_AVERROR(EAGAIN) => break,
                status if status < 0 => break,
                _ => {
                    println!("rescaling");
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

    println!("stream");

    // stream.free();

    println!("avio");
    // avio_closep(&mut (*oc).pb);
    println!("avio context");
    // avformat_free_context(oc);
}
