
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR is not set")
    );

    let zig_source = manifest_dir.join("../zig/src/ffi/vector_add.zig");

    println!(
        "cargo:rerun-if-changed={}",
        zig_source.display()
    );

    let out_dir = PathBuf::from(
        env::var("OUT_DIR")
            .expect("OUT_DIR is not set")
    );

    let library = out_dir.join("libsimd.a");

    let emit_bin = format!("-femit-bin={}", library.display());

    let status = Command::new("zig")
        .arg("build-lib")
        .arg(&zig_source)
        .arg("-O")
        .arg("ReleaseFast")
        .arg("-mcpu=native")
        .arg("-static")
        .arg(&emit_bin)
        .status()
        .expect("failed to execute Zig compiler");

    if !status.success() {
        panic!("Zig failed to build the FFI library");
    }

    if !library.exists() {
        panic!(
            "Zig reported success, but the library was not created: {}",
            library.display()
        );
    }

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.display()
    );

    println!("cargo:rustc-link-lib=static=simd");
}
