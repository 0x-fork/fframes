use ffmpeg_next::sys::*;
use std::ffi::CString;

use crate::{
    ffmpeg_action,
    renderer_error::{FFmpegError, FFmpegResult},
};

unsafe fn open_file_video_stream(
    filename: &str,
    input_format_ctx: &mut *mut AVFormatContext,
) -> FFmpegResult<*mut AVStream> {
    let input_file = CString::new(filename).unwrap();

    ffmpeg_action!(
        avformat_open_input(
            input_format_ctx,
            input_file.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ),
        FFmpegError::CantOpenFile(filename.to_owned())
    );

    ffmpeg_action!(
        avformat_find_stream_info(*input_format_ctx, std::ptr::null_mut()),
        FFmpegError::CantOpenFile(filename.to_owned())
    );

    let streams = std::slice::from_raw_parts_mut(
        (*(*input_format_ctx)).streams,
        (*(*input_format_ctx)).nb_streams as usize,
    );

    let mut input_video_stream = std::ptr::null_mut();
    for stream in streams {
        let codec = (*stream.to_owned()).codec;

        if (*codec).codec_type == AVMediaType::AVMEDIA_TYPE_VIDEO {
            input_video_stream = *stream;
            break;
        }
    }

    if input_video_stream.is_null() {
        Err(FFmpegError::MissingVideoStreamInFile(filename.to_owned()))
    } else {
        Ok(input_video_stream)
    }
}

unsafe fn copy_codec_params(
    codec: *mut AVCodecContext,
    input_format_ctx: *mut AVFormatContext,
    input_video_stream: *mut AVStream,
    output_video_stream: *mut AVStream,
) {
    (*codec).bit_rate = (*input_format_ctx).bit_rate;
    (*codec).codec_id = (*(*input_video_stream).codec).codec_id;
    (*codec).codec_type = (*(*input_video_stream).codec).codec_type;

    (*codec).time_base = (*input_video_stream).time_base;
    (*output_video_stream).time_base = (*codec).time_base;
    
    (*codec).width = (*(*input_video_stream).codec).width;
    (*codec).height = (*(*input_video_stream).codec).height;
    (*codec).pix_fmt = (*(*input_video_stream).codec).pix_fmt;

    (*codec).flags = (*(*input_video_stream).codec).flags;
    (*codec).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;

    (*codec).me_range = (*(*input_video_stream).codec).me_range;
    (*codec).max_qdiff = (*(*input_video_stream).codec).max_qdiff;
    (*codec).gop_size = (*(*input_video_stream).codec).gop_size; // maybe hardcode to 12?

    (*codec).qmin = (*(*input_video_stream).codec).qmin;
    (*codec).qmax = (*(*input_video_stream).codec).qmax;
    (*codec).qcompress = (*(*input_video_stream).codec).qcompress;

    (*codec).extradata = (*(*input_video_stream).codec).extradata;
    (*codec).extradata_size = (*(*input_video_stream).codec).extradata_size;
    avcodec_parameters_from_context((*output_video_stream).codecpar, codec);
}

pub unsafe fn concat_files(files: &[String], output: &str) -> Result<(), FFmpegError> {
    let mut input_format_ctx: *mut AVFormatContext = std::ptr::null_mut();
    let mut output_format_ctx: *mut AVFormatContext = std::ptr::null_mut();

    let input_video_stream = open_file_video_stream(&files[0], &mut input_format_ctx)?;
    let output_file = CString::new(output).unwrap();

    avformat_alloc_output_context2(
        &mut output_format_ctx,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        output_file.as_ptr(),
    );

    let output_video_stream = avformat_new_stream(output_format_ctx, std::ptr::null_mut());
    let codec = (*output_video_stream).codec;

    copy_codec_params(
        codec,
        input_format_ctx,
        input_video_stream,
        output_video_stream,
    );

    if (*(*output_format_ctx).oformat).flags & AVFMT_GLOBALHEADER != 0 {
        (*(*output_format_ctx).oformat).flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32
    }

    avio_open(
        &mut (*output_format_ctx).pb,
        output_file.as_ptr(),
        AVIO_FLAG_WRITE,
    );

    avformat_close_input(&mut input_format_ctx);
    avformat_write_header(output_format_ctx, std::ptr::null_mut());

    let mut last_pts = 0;
    let mut last_dts = 0;
    let mut start_time = 0;

    for (i, file) in files.into_iter().enumerate() {
        let c_filename = CString::new(file.as_str()).unwrap();
        let mut input_format_ctx = std::ptr::null_mut();

        let input_video_stream = open_file_video_stream(&file, &mut input_format_ctx)?;
        av_dump_format(input_format_ctx, 0, c_filename.as_ptr(), 0);

        let mut delta = 0;

        loop {
            let packet = std::ptr::null_mut();
            av_init_packet(packet);

            (*packet).size = 0;
            (*packet).data = std::ptr::null_mut();

            let res = av_read_frame(input_format_ctx, packet);
            if res < 0 {
                break;
            }

            (*packet).flags |= AV_PKT_FLAG_KEY;

            // This calculates the delta in pts based on the duration when this file must be appeared
            delta = av_rescale_q(start_time, AV_TIME_BASE_Q, (*output_video_stream).time_base);

            (*packet).pts += delta;
            (*packet).dts += delta;

            if i != 0 && (*packet).dts <= last_dts {
                // This can happen if first frames dts is negative
                // just make +1 and hope 🤞 it won't broke in the final video
                (*packet).dts = last_dts + 1
            }
            if i != 0 && (*packet).pts <= last_pts {
                // This can happen if first frames pts is negative
                // just make +1 and hope 🤞 it won't broke in the final video
                (*packet).pts = last_pts + 1
            }

            last_dts = (*packet).dts;
            last_pts = (*packet).pts;

            av_packet_rescale_ts(
                packet,
                (*input_video_stream).time_base,
                (*output_video_stream).time_base,
            );
            av_interleaved_write_frame(output_format_ctx, packet);
        }

        start_time += (*input_format_ctx).duration;
        avformat_close_input(&mut input_format_ctx);
    }

    av_write_trailer(output_format_ctx);
    avcodec_close(codec);

    avio_close((*output_format_ctx).pb);
    avformat_free_context(output_format_ctx);

    Ok(())
}
