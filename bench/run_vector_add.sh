#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RAW="$ROOT/results/raw/vector_add"
BUILD="$ROOT/bench/build/vector_add"
mkdir -p "$BUILD"
ROUND_COUNT="${ROUND_COUNT:-5}"

mkdir -p "$RAW"

echo "Building Rust automatic SIMD benchmark..."
RUSTFLAGS="-C target-cpu=native" \
    cargo build --release \
    --manifest-path "$ROOT/rust/Cargo.toml" \
    --bin bench_vector_add_auto

echo "Building Rust explicit SIMD benchmark..."
rustc +nightly \
    "$ROOT/experiments/rust/bench_vector_add_simd.rs" \
    -C opt-level=3 \
    -C target-cpu=native \
    -o "$BUILD/bench_vector_add_rust_simd"

echo "Building Zig scalar benchmark..."
zig build-exe \
    "$ROOT/zig/src/bench_scalar.zig" \
    -O ReleaseFast \
    -mcpu=native \
    -femit-bin="$BUILD/bench_vector_add_zig_scalar"

echo "Building Zig explicit SIMD benchmark..."
zig build-exe \
    "$ROOT/zig/src/bench_simd.zig" \
    -O ReleaseFast \
    -mcpu=native \
    -femit-bin="$BUILD/bench_vector_add_zig_simd"

echo
echo "Running $ROUND_COUNT complete benchmark rounds..."

for round in $(seq 1 "$ROUND_COUNT"); do
    dir="$RAW/round-$round"
    mkdir -p "$dir"

    echo "Round $round/$ROUND_COUNT"

    RUSTFLAGS="-C target-cpu=native" \
        "$ROOT/rust/target/release/bench_vector_add_auto" \
        > "$dir/rust_auto.txt"

    "$BUILD/bench_vector_add_rust_simd" \
        > "$dir/rust_explicit_simd.txt"

    "$BUILD/bench_vector_add_zig_scalar" \
        > "$dir/zig_scalar.txt" 2>&1

    "$BUILD/bench_vector_add_zig_simd" \
        > "$dir/zig_explicit_simd.txt" 2>&1
done

echo
echo "Benchmark complete."
echo "Raw results: $RAW"
