extern crate glob;
extern crate serde_codegen;

use std::env;
use std::path::Path;
use std::fs;

fn main() {
    // Don't re-run this script unless one of the inputs has changed.
    for entry in glob::glob("src/**/*.rs.in").expect("Failed to read glob pattern") {
        println!("cargo:rerun-if-changed={}", entry.unwrap().display());
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Switch to our `src` directory so that we have the right base for our
    // globs, and so that we won't need to strip `src/` off every path.
    env::set_current_dir("src").unwrap();

    for entry in glob::glob("**/*.rs.in").expect("Failed to read glob pattern") {
        match entry {
            Ok(src) => {
                let mut dst = Path::new(&out_dir).join(&src);

                // Change ".rs.in" to ".rs".
                dst.set_file_name(src.file_stem().expect("Failed to get file stem"));
                dst.set_extension("rs");

                // Make sure our target directory exists.  We only need
                // this if there are extra nested sudirectories under src/.
                fs::create_dir_all(dst.parent().unwrap()).unwrap();

                // Process our source file.
                serde_codegen::expand(&src, &dst).unwrap();
            }
            Err(e) => {
                panic!("Error globbing: {}", e);
            }
        }
    }
}
