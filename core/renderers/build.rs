use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Path to cengine/build relative to this crate
    let cengine_build = PathBuf::from(&manifest_dir).join("../cengine/build");

    // Tell rustc where to look for libcengine.a
    println!("cargo:rustc-link-search=native={}", cengine_build.display());

    // Link the static library: libcengine.a  (lib + cengine + .a)
    println!("cargo:rustc-link-lib=static=cengine");

    // Link GLFW + system libs (Linux)
    println!("cargo:rustc-link-lib=static=glad");
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=pthread");
}
