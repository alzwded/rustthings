extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // compile some C code as a static library, which will be linked
    // into our project
    cc::Build::new()
        .file("src/cstuff/test.c")
        .include("src") // -I equivalent
        .compile("libtest.a"); // -o equivalent

    println!("cargo:rustc-link-lib=dl"); // we need to link to this because of dlopen/dlsym
    println!("cargo:rerun-if-changed=src/cstuff/test.c"); // rebuild C code; the .h file is
                                                   // automatically picked up because
                                                   // of parse_callbacks

    // use bindgen to generate some bindings; it expects a header
    // that `include ""'s everything you want in rust
    let bindings = bindgen::Builder::default()
        .header("src/cstuff/test.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) // triggers auto rebuilds
        .generate()
        .expect("Unable to generate bindings"); // die if failed

    // determine the output path
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // write out the bindings
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
