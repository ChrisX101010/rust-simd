#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT"

OUT="$ROOT/results/assembly"
mkdir -p "$OUT"

echo "========================================"
echo " rust-simd assembly verification"
echo "========================================"

echo
echo "Building release assembly..."

cargo rustc \
    --manifest-path "$ROOT/rust/Cargo.toml" \
    --release \
    --lib \
    -- \
    --emit=asm \
    -C target-cpu=generic

ASM_FILE="$(
    find "$ROOT/rust/target/release/deps" \
        -maxdepth 1 \
        -type f \
        -name 'rust_simd-*.s' \
        | sort \
        | tail -n 1
)"

if [[ -z "$ASM_FILE" ]]; then
    echo "ERROR: assembly file not found"
    exit 1
fi

cp "$ASM_FILE" "$OUT/rust-simd-release.s"

echo
echo "Assembly: $ASM_FILE"

echo
echo "=== SIMD instructions ==="

grep -E \
    'vaddps|vmulps|vfmadd|vmovups|vmovaps|vextractf128|vperm' \
    "$ASM_FILE" \
    | head -n 100 || true

echo
echo "=== Summary ==="

printf 'vaddps:    '
grep -c 'vaddps' "$ASM_FILE" || true

printf 'vmulps:    '
grep -c 'vmulps' "$ASM_FILE" || true

printf 'vfmadd:    '
grep -c 'vfmadd' "$ASM_FILE" || true

printf 'vmovups:   '
grep -c 'vmovups' "$ASM_FILE" || true

echo
echo "ASSEMBLY VERIFICATION COMPLETE"
