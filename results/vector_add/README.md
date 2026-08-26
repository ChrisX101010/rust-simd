# Experiment A — vector_add

## Kernel

```text
C[i] = A[i] + B[i]
```

## Environment

- CPU: AMD Ryzen 7 4800H
- Architecture: x86-64
- OS: Linux under WSL2
- Rust stable: 1.95.0
- Rust nightly: installed for portable SIMD
- Zig: 0.16.0
- Native CPU target: AMD Zen 2 / znver2

## Implementations

### Rust — automatic vectorization

Ordinary safe Rust slice loop. The optimized native assembly contains packed floating-point instructions including vaddps using YMM registers.

### Zig — scalar implementation

Ordinary Zig slice loop. In our Zig 0.16.0 test, the generated kernel contained scalar floating-point additions using vaddss.

### Zig — explicit SIMD

Uses @Vector(8, f32). The generated kernel contains packed vaddps instructions.

### Rust — explicit portable SIMD

Uses nightly std::simd::f32x8. Correctness has been verified.

## Correctness

All tested implementations produced:

16760315880.000

## Preliminary timing

Zig scalar: 14.282719 ms/run

Zig explicit SIMD: 13.084886 ms/run

These timings are exploratory and are not yet suitable for the final cross-language performance claim.

## Current observations

1. Rust/LLVM automatically vectorizes the ordinary safe Rust loop.
2. Zig 0.16.0 produced scalar code for the ordinary loop in this test.
3. Explicit Zig @Vector(8, f32) produced packed SIMD.
4. Rust portable SIMD is functionally correct.
5. Explicit SIMD provides a programmer-directed optimization path when automatic vectorization does not produce the desired result.

## Research question

Where does compiler-driven SIMD optimization succeed or fail across Rust and Zig, and how do ownership, aliasing, memory layout, dependencies, and explicit SIMD control affect the resulting machine code and performance?

## Next steps

1. Complete the Rust portable-SIMD assembly inspection.
2. Build a common benchmark harness.
3. Freeze Experiment A as the baseline.
4. Add FMA, reduction, stencil, branch, and alias-sensitive kernels.
5. Compare automatic and explicit optimization across Rust and Zig.
6. Investigate the optimization gaps revealed by the experiments.
