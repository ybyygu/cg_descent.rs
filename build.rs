// build.rs
// :PROPERTIES:
// :header-args: :tangle build.rs
// :END:

// [[file:~/Workspace/Programming/rust-libs/cg_descent.rs/cg_descent.note::*build.rs][build.rs:1]]
use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(false)
        .file("lib/cg_descent.c")
        .include("lib")
        .compile("libcgd.a");

    // println!("cargo:rustc-link-lib=lbfgs");

    let bindings = bindgen::Builder::default()
        .header("lib/cg_user.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// build.rs:1 ends here
