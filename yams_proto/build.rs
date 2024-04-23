use flatbuffers_build::BuilderOptions;

fn main() {
    BuilderOptions::new_with_files(["schemas/message.fbs"])
        .set_symlink_directory("src/fb_api")
        .compile()
        .expect("flatbuffer compilation failed")
}
