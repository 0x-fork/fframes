## Requirements

[Rust](https://www.rust-lang.org/learn/get-started) and [NodeJS](https://nodejs.org/en/download/) (for local development) toolchains.

It is also required to have ffmpeg v4 installed, yes unfortunately the latest v5 is not supported yet. Install the 4.4.2 version from your favorite package manager or built it from source:

```bash
git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg
git checkout tags/n4.4.2

./configure --enable-shared --enable-libx264 --enable-libx265 --enable-gpl
make # build ffmpeg v4
make install # install c libraries globally
```
