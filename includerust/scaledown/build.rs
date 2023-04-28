extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = PathBuf::from("target/include");

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_include_version(true)
        .with_crate(&crate_dir)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(out_path.join("imageproc_bindings.h"));

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::Cxx)
        .with_include_version(true)
        .with_crate(&crate_dir)
        .generate()
        .expect("Unable to generate C++ bindings")
        .write_to_file(out_path.join("imageproc_bindings.hxx"));
}
