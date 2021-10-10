fn main() {
    cc::Build::new()
        .file("../encode_video.c")
        .compile("wat");
}
