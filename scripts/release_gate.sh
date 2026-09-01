#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "========================================"
echo " rust-simd family release gate"
echo "========================================"

echo
echo "[1/12] format"
cargo fmt --all -- --check

echo
echo "[2/12] check"
cargo check --workspace --locked

echo
echo "[3/12] debug tests"
cargo test --workspace --locked

echo
echo "[4/12] release tests"
cargo test --workspace --release --locked

echo
echo "[5/12] clippy"
cargo clippy \
  --workspace \
  --all-targets \
  --locked \
  -- \
  -D warnings

echo
echo "[6/12] differential verification"
cargo run \
  -p cargo-simd \
  --release \
  -- \
  simd verify

echo
echo "[7/12] resource controller"
cargo run \
  -p cargo-simd \
  --release \
  -- \
  simd build --dry-run --release

cargo run \
  -p cargo-simd \
  --release \
  -- \
  simd test --dry-run --workspace

echo
echo "[8/12] dependency footprint"

CORE_LINES="$(
    cargo tree \
      -p rust-simd \
      --edges normal \
      --prefix none \
      | wc -l
)"

TOOL_LINES="$(
    cargo tree \
      -p cargo-simd \
      --edges normal \
      --prefix none \
      | wc -l
)"

echo "rust-simd tree lines: $CORE_LINES"
echo "cargo-simd tree lines: $TOOL_LINES"

test "$CORE_LINES" -eq 1
test "$TOOL_LINES" -eq 2

echo
echo "[9/12] package"

cargo package \
  --manifest-path rust/Cargo.toml \
  --allow-dirty

CRATE_FILE="$ROOT/target/package/rust-simd-0.4.0.crate"

test -f "$CRATE_FILE"

if tar -tzf "$CRATE_FILE" | \
   grep -Eq '/(src/bin|vendor|bench|target)/|build\.rs\.zig-backend|src/ffi\.rs'
then
    echo "ERROR: unwanted package content"
    exit 1
fi

echo
echo "[10/12] cross-target warning-free checks"

RUSTFLAGS="-Dwarnings" \
cargo check \
  -p rust-simd \
  --target aarch64-unknown-linux-gnu \
  --locked

RUSTFLAGS="-Dwarnings" \
cargo check \
  -p rust-simd \
  --target wasm32-unknown-unknown \
  --locked

RUSTFLAGS="-Dwarnings" \
cargo check \
  -p rust-simd \
  --target x86_64-unknown-linux-musl \
  --locked

RUSTFLAGS="-Dwarnings -Ctarget-feature=+simd128" \
cargo check \
  -p rust-simd \
  --target wasm32-unknown-unknown \
  --locked

echo
echo "[11/12] WASM SIMD128 smoke build"

RUSTFLAGS="-Dwarnings -Ctarget-feature=+simd128" \
cargo build \
  --manifest-path integration/wasm-smoke/Cargo.toml \
  --target wasm32-wasip1 \
  --release

echo
echo "[12/12] repository cleanliness"

git diff --check

echo
echo "========================================"
echo " RELEASE GATE PASSED"
echo "========================================"

echo
echo "rust-simd package:"
ls -lh "$CRATE_FILE"

echo
echo "cargo-simd binary:"
cargo build -p cargo-simd --release --locked
ls -lh target/release/cargo-simd
