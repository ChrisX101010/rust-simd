#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

SOURCE="$ROOT/zig/src/ffi/vector_add.zig"
VENDOR="$ROOT/rust/vendor/zig/vector_add.zig"

mkdir -p "$(dirname "$VENDOR")"

cp "$SOURCE" "$VENDOR"

echo "Synchronized:"
echo "  $SOURCE"
echo "→ $VENDOR"
