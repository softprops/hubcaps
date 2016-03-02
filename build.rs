extern crate syntex;
extern crate serde_codegen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    generate(Path::new("src/rep.rs.in"), &Path::new(&out_dir).join("rep.rs"));
}

fn generate(src: &Path, dst: &PathBuf) {
    let mut registry = syntex::Registry::new();
    serde_codegen::register(&mut registry);
    registry.expand("", &src, &dst).unwrap();
}
