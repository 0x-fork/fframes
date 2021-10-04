fn main() {
    cc::Build::new()
        .file("wat.c")
        .file("../encode_video.c")
        .compile("wat");
}
