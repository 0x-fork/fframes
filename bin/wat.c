#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>

#include <libavutil/avassert.h>
#include <libavutil/channel_layout.h>
#include <libavutil/opt.h>
#include <libavutil/mathematics.h>
#include <libavutil/timestamp.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>

void fill_yuv_image(AVFrame *pict, int frame_index,
                    int width, int height)
{
  int x, y, i;

  i = frame_index;
  /* Y */
  for (y = 0; y < height; y++)
    for (x = 0; x < width; x++)
      pict->data[0][y * pict->linesize[0] + x] = x + y + i * 3;

  printf("WTFOUI JOIJIOJIOJ %d \n", pict->data[0][y * pict->linesize[0] + x]);

  /* Cb and Cr */
  for (y = 0; y < height / 2; y++)
  {
    for (x = 0; x < width / 2; x++)
    {
      pict->data[1][y * pict->linesize[1] + x] = 128 + y + i * 2;
      pict->data[2][y * pict->linesize[2] + x] = 64 + x + i * 5;
    }
  }
}

int send_frame(AVCodecContext *c, AVFrame *frame)
{
  int ret = avcodec_send_frame(c, frame);
  
  if (ret < 0)
  {
    fprintf(stderr, "Error sending a frame to the encoder: %s\n",
            av_err2str(ret));
    exit(1);
  }

  return ret;
}