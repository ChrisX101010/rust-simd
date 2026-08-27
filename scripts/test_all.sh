#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "========================================"
echo " Rust-Zig SIMD Control Lab"
echo " Full Validation Suite"
echo "========================================"

echo
echo "[1/5] Rust stable library + FFI tests"
cargo test --manifest-path "$ROOT/rust/Cargo.toml"

echo
echo "[2/5] Zig unit tests"
zig build test --build-file "$ROOT/zig/build.zig"

echo
echo "[3/5] Rust nightly portable-SIMD experiment"
rustc +nightly \
    "$ROOT/experiments/rust/vector_add_portable_simd.rs" \
    -C opt-level=3 \
    -C target-cpu=native \
    -o /tmp/rust_zig_simd_portable_simd

/tmp/rust_zig_simd_portable_simd

echo
echo "[4/5] Standalone FFI integration tests"

mkdir -p "$ROOT/bench/build/ffi"

zig build-lib \
    "$ROOT/zig/src/ffi/vector_add.zig" \
    -O ReleaseFast \
    -mcpu=native \
    -static \
    -femit-bin="$ROOT/bench/build/ffi/libsimd.a"

rustc \
    --edition=2024 \
    --test \
    "$ROOT/experiments/rust/ffi_tests.rs" \
    -L native="$ROOT/bench/build/ffi" \
    -l static=simd \
    -o "$ROOT/bench/build/ffi/ffi_tests"

"$ROOT/bench/build/ffi/ffi_tests"

echo
echo "[5/5] External consumer test"

cargo run \
    --manifest-path "$ROOT/consumer-test/Cargo.toml"

echo
echo "========================================"
echo " ALL TESTS PASSED"
echo "========================================"
