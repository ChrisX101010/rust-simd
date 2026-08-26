# Rust-Zig SIMD Lab

Experimental research project comparing Rust and Zig implementations
of identical computational kernels.

## Goals

1. Measure Rust compiler autovectorization.
2. Compare explicit SIMD implementations.
3. Compare Rust and Zig implementations of identical kernels.
4. Measure the cost of Rust/Zig FFI.
5. Inspect generated assembly and SIMD instructions.
6. Identify cases where explicit low-level information provides an
   optimization advantage over compiler-driven optimization.

## Initial target

- OS: Linux under WSL2
- Architecture: x86-64
- CPU: AMD Ryzen 7 4800H
- SIMD baseline: AVX2/FMA
- Rust: stable
- Zig: 0.16.0

## Status

Experimental.
