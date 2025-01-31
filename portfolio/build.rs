use build::MarkdownBuilder;
use std::{env, path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = path::Path::new(&out_dir);
    let content_dir = path::Path::new("content");

    for artifact in MarkdownBuilder::from_path(&content_dir).synthesize() {
        artifact.emit(&out_dir);
    }
}
