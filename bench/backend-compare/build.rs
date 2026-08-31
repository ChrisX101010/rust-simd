use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set"));

    // bench/backend-compare -> bench -> repository root
    let repo_root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("failed to locate repository root");

    let zig_source = repo_root.join("zig/src/ffi/vector_add.zig");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let library = out_dir.join("libsimd.a");

    println!("cargo:rerun-if-changed={}", zig_source.display());

    if !zig_source.is_file() {
        panic!(
            "Zig backend source does not exist: {}",
            zig_source.display()
        );
    }

    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("failed to query rustc version");

    if !rustc_version.status.success() {
        panic!("rustc --version failed");
    }

    let rustc_version = String::from_utf8_lossy(&rustc_version.stdout);
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version.trim());

    let status = Command::new("zig")
        .arg("build-lib")
        .arg(&zig_source)
        .arg("-O")
        .arg("ReleaseFast")
        .arg("-mcpu=native")
        .arg("-static")
        .arg(format!("-femit-bin={}", library.display()))
        .status()
        .expect("failed to execute Zig compiler");

    if !status.success() {
        panic!("Zig backend benchmark compilation failed");
    }

    if !library.is_file() {
        panic!(
            "Zig benchmark library was not created: {}",
            library.display()
        );
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=simd");
}
