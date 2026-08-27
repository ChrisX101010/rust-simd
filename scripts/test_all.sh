#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "========================================"
echo " Rust-Zig SIMD Control Lab"
echo " Full Validation Suite"
echo "========================================"

echo
echo "[1/7] Rust portable/default tests"
cargo clean --manifest-path "$ROOT/rust/Cargo.toml"
cargo test --manifest-path "$ROOT/rust/Cargo.toml"

echo
echo "[2/7] Rust native-target tests"
cargo clean --manifest-path "$ROOT/rust/Cargo.toml"
RUST_ZIG_SIMD_NATIVE=1 \
    cargo test --manifest-path "$ROOT/rust/Cargo.toml"

echo
echo "[3/7] Zig unit tests"
zig build test --build-file "$ROOT/zig/build.zig"

echo
echo "[4/7] Rust nightly portable-SIMD experiment"
rustc +nightly \
    "$ROOT/experiments/rust/vector_add_portable_simd.rs" \
    -C opt-level=3 \
    -C target-cpu=native \
    -o /tmp/rust_simd_portable_simd

/tmp/rust_simd_portable_simd

echo
echo "[5/7] Standalone FFI integration tests"

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
echo "[6/7] External consumer test"

cargo run \
    --manifest-path "$ROOT/consumer-test/Cargo.toml"

echo
echo "[7/7] Rust package validation"
(
    cd "$ROOT/rust"
    cargo package
)

echo
echo "========================================"
echo " ALL TESTS PASSED"
echo "========================================"
