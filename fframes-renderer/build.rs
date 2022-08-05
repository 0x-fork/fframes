fn main() {
    cc::Build::new()
        .file("ffmpeg_helper.c")
        .compile("ffmpeg_helper");
}
