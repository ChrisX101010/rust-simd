#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "========================================"
echo " rust-simd release verification"
echo "========================================"

cd "$ROOT"

echo
echo "[1/8] Formatting"
cargo fmt --manifest-path "$ROOT/rust/Cargo.toml" -- --check

echo
echo "[2/8] Core check"
cargo check --manifest-path "$ROOT/rust/Cargo.toml" --locked

echo
echo "[3/8] Core tests"
cargo test --manifest-path "$ROOT/rust/Cargo.toml"

echo
echo "[4/8] Release tests"
cargo test --manifest-path "$ROOT/rust/Cargo.toml" --release

echo
echo "[5/8] Clippy"
cargo clippy \
    --manifest-path "$ROOT/rust/Cargo.toml" \
    --all-targets \
    -- \
    -D warnings

echo
echo "[6/8] Package"

mkdir -p "$ROOT/rust/target/package"
rm -f "$ROOT/rust/target/package/rust-simd-"*.crate

cargo package \
    --manifest-path "$ROOT/rust/Cargo.toml" \
    --allow-dirty

PACKAGE="$(
    find "$ROOT/rust/target/package" \
        -maxdepth 1 \
        -type f \
        -name 'rust-simd-*.crate' \
        | sort \
        | tail -n 1
)"

if [[ -z "$PACKAGE" ]]; then
    echo "ERROR: package was not created"
    exit 1
fi

echo
echo "Package: $PACKAGE"

echo
echo "[7/8] Published package inspection"

LISTING="$(tar -tf "$PACKAGE")"

echo "$LISTING"

for forbidden in \
    "build.rs.zig-backend" \
    "src/ffi.rs" \
    "vendor/" \
    "src/bin/"
do
    if grep -Fq "$forbidden" <<<"$LISTING"; then
        echo
        echo "ERROR: forbidden published path detected: $forbidden"
        exit 1
    fi
done

for required in \
    "src/lib.rs" \
    "src/backend.rs" \
    "src/error.rs" \
    "src/simd.rs"
do
    if ! grep -Fq "$required" <<<"$LISTING"; then
        echo
        echo "ERROR: required published path missing: $required"
        exit 1
    fi
done

echo
echo "[8/8] Offline package consumer"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/project"
tar -xf "$PACKAGE" -C "$TMP/project"

PACKAGE_DIR="$(
    find "$TMP/project" \
        -mindepth 1 \
        -maxdepth 1 \
        -type d \
        -name 'rust-simd-*' \
        | head -n 1
)"

if [[ -z "$PACKAGE_DIR" ]]; then
    echo "ERROR: extracted package directory not found"
    exit 1
fi

mkdir -p "$TMP/consumer/src"

cat > "$TMP/consumer/Cargo.toml" <<MANIFEST
[package]
name = "rust-simd-release-consumer"
version = "0.1.0"
edition = "2024"

[dependencies]
rust-simd = { path = "$PACKAGE_DIR" }
MANIFEST

cat > "$TMP/consumer/src/main.rs" <<'RS'
use rust_simd::{BackendKind, Engine};

fn main() {
    let engine = Engine::auto();

    let a = [1.0f32, 2.0, 3.0, 4.0];
    let b = [5.0f32, 6.0, 7.0, 8.0];
    let c = [1.0f32; 4];

    let mut out = [0.0f32; 4];

    engine.vector_add(&a, &b, &mut out);
    assert_eq!(out, [6.0, 8.0, 10.0, 12.0]);

    engine.fma(&a, &b, &c, &mut out);
    assert_eq!(out, [6.0, 13.0, 22.0, 33.0]);

    assert_eq!(engine.reduce_sum(&a), 10.0);
    assert_eq!(engine.dot(&a, &b), 70.0);

    assert!(matches!(
        engine.backend(),
        BackendKind::Scalar
            | BackendKind::Avx2
            | BackendKind::Avx2Fma
    ));

    println!("release consumer: PASS");
    println!("backend: {}", engine.backend_name());
}
RS

(
    cd "$TMP/consumer"
    cargo run --offline
)

echo
echo "========================================"
echo " RELEASE VERIFICATION PASSED"
echo "========================================"
