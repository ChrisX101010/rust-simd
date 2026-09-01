# rust-simd

A lightweight SIMD runtime and Cargo tooling family for Rust.

`rust-simd` provides safe backend-neutral numerical kernels with runtime SIMD dispatch, while `cargo-simd` adds SIMD-aware diagnostics, verification, and resource-aware Cargo orchestration.

The project is designed around a small dependency footprint, explicit capability detection, scalar fallbacks, offline-first execution, and reproducible verification.

## Project family

```text
rust-simd family
├── rust-simd    lightweight SIMD runtime
└── cargo-simd   SIMD-aware Cargo developer tooling
```

The two crates share the same capability and backend model.

`cargo-simd` does not implement a separate SIMD detector or runtime. It uses `rust-simd` as the authoritative execution layer.

---

## rust-simd

`rust-simd` exposes a backend-neutral API for a small set of numerical kernels:

* element-wise vector addition
* fused multiply-add
* sum reduction
* dot product

Automatic dispatch selects the best implemented backend available to the current process.

```rust
use rust_simd::{Capabilities, Engine};

fn main() {
    let capabilities = Capabilities::detect();
    let engine = Engine::auto();

    println!(
        "architecture={} backend={}",
        capabilities.architecture().name(),
        engine.backend_name(),
    );

    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let b = [5.0_f32, 6.0, 7.0, 8.0];

    let mut out = [0.0_f32; 4];

    engine.vector_add(&a, &b, &mut out);

    assert_eq!(out, [6.0, 8.0, 10.0, 12.0]);

    let dot = engine.dot(&a, &b);

    assert!((dot - 70.0).abs() < 0.001);
}
```

### Backends

| Architecture         | Backend    | Implementation |
| -------------------- | ---------- | -------------- |
| Any supported target | Scalar     | Yes            |
| x86 / x86_64         | AVX2       | Yes            |
| x86 / x86_64         | AVX2 + FMA | Yes            |
| AArch64              | NEON       | Yes            |
| WebAssembly          | SIMD128    | Yes            |

The scalar backend is always available and acts as the portability fallback.

### Kernel coverage

| Operation          | Scalar |                 AVX2 |             AVX2+FMA |                 NEON |              WASM SIMD128 |
| ------------------ | -----: | -------------------: | -------------------: | -------------------: | ------------------------: |
| vector add         |    yes |                 SIMD |                 SIMD |                 SIMD |                      SIMD |
| fused multiply-add |    yes |      scalar fallback |             SIMD FMA |             SIMD FMA | scalar `mul_add` fallback |
| sum reduction      |    yes | SIMD, 4 accumulators | SIMD, 4 accumulators | SIMD, 4 accumulators |      SIMD, 4 accumulators |
| dot product        |    yes | SIMD, 4 accumulators | SIMD, 4 accumulators | SIMD, 4 accumulators |      SIMD, 4 accumulators |

WASM SIMD128 deliberately retains the scalar `f32::mul_add` path for FMA rather than replacing fused semantics with a non-fused multiply followed by add.

### Explicit backend selection

```rust
use rust_simd::Engine;

let scalar = Engine::scalar();

let avx2 = Engine::avx2();
let avx2_fma = Engine::avx2_fma();
let neon = Engine::neon();
let wasm = Engine::wasm_simd128();
```

Explicit constructors return an error if the requested backend is unavailable.

### Capabilities

```rust
use rust_simd::Capabilities;

let capabilities = Capabilities::detect();

println!("arch: {}", capabilities.architecture().name());
println!("AVX2: {}", capabilities.has_avx2());
println!("FMA: {}", capabilities.has_fma());
println!("NEON: {}", capabilities.has_neon());
println!("WASM SIMD128: {}", capabilities.has_wasm_simd128());
println!("best backend: {}", capabilities.best_backend().name());
```

`Capabilities` is the authoritative backend-selection model used by automatic dispatch.

---

## cargo-simd

`cargo-simd` is a Cargo custom subcommand built above `rust-simd`.

Install it with:

```bash
cargo install cargo-simd
```

Then use:

```bash
cargo simd doctor
cargo simd verify
cargo simd build
cargo simd test
```

### `cargo simd doctor`

Reports:

* architecture
* operating system
* logical CPU count
* available memory
* Rust and Cargo versions
* detected SIMD capabilities
* automatically selected backend
* Cargo workspace information
* recommended resource policy

Example:

```text
cargo-simd doctor

SYSTEM
  architecture       x86_64
  operating system   linux
  logical CPUs       16

SIMD
  vector model       fixed-width
  selected backend   avx2+fma
  AVX2 available     yes
  FMA available      yes

RECOMMENDATION
  policy             low-resource
  build jobs         4
  test threads       6
```

### `cargo simd verify`

Runs differential verification across every backend available on the current machine.

The verifier currently checks:

* 31 boundary and tail lengths
* vector addition
* fused multiply-add
* sum reduction
* dot product
* structured numerical cases
* zero and signed-zero handling
* finite large and small values
* alternating-sign inputs
* NaN classification for element-wise operations

Verification failures are reported as structured errors containing the backend, operation, input length, index where applicable, actual value, reference value, and tolerance.

Example:

```text
available backends:
  - scalar
  - avx2
  - avx2+fma

running differential verification...
running structured numerical cases...

verification summary
  lengths tested     31
  backends tested    3
  numerical cases    PASS
  vector_add         PASS
  fma                PASS
  reduce_sum         PASS
  dot                PASS

SIMD verification: PASS
```

### Resource-aware Cargo execution

`cargo-simd` observes the host CPU and available memory and derives separate budgets for Cargo build processes and test threads.

```bash
cargo simd build --release
cargo simd test --workspace
```

Example:

```text
RESOURCE BUDGET
  policy             low-resource
  build jobs         4
  test threads       6
  network            offline

COMMAND
  cargo test --jobs 4 --workspace -- --test-threads 6
```

Manual concurrency overrides such as `--jobs` and `--test-threads` are rejected when `cargo-simd` owns resource policy.

The goal is not to replace Cargo's scheduler. `cargo-simd` supplies a resource budget while Cargo and the Rust test harness continue to perform the actual scheduling.

### Offline-first

Commands launched through the resource controller run with Cargo networking disabled by default.

An explicit online mode can be used when network access is required.

`cargo-simd` does not automatically download, install, or execute optional third-party tooling.

### cargo-nextest integration

`cargo-simd` can delegate test execution to `cargo-nextest` when explicitly requested and installed.

This keeps specialist functionality in specialist tools instead of reimplementing it inside `cargo-simd`.

---

## Lightweight by design

The production runtime currently has no external crate dependencies.

```text
rust-simd
```

`cargo-simd` depends only on `rust-simd`:

```text
cargo-simd
└── rust-simd
```

The project intentionally avoids adding an async runtime, daemon, database, container runtime, network client, or mandatory external compiler to the production dependency graph.

Zig experiments and historical benchmark programs remain research material and are not part of the published `rust-simd` runtime package.

---

## Verification

The repository includes a local release gate:

```bash
./scripts/release_gate.sh
```

It checks:

1. rustfmt
2. workspace compilation
3. debug tests
4. release tests
5. Clippy with warnings denied
6. SIMD differential verification
7. resource-aware Cargo plans
8. dependency footprint
9. package integrity
10. warning-free cross-target compilation
11. WASM SIMD128 smoke build
12. repository diff cleanliness

Current local verification covers x86_64 AVX2/FMA execution plus cross-compilation of AArch64 NEON and WebAssembly SIMD128 implementations.

Native CI is used to provide architecture-specific runtime evidence that cannot be obtained from a single development machine.

---

## CI strategy

The repository separates different types of evidence.

### Native CI

Runs tests and verification on native hosted environments for:

* Linux x86_64
* Linux ARM64
* Windows x86_64
* Windows ARM64
* macOS Intel
* macOS ARM64

Linux ARM64 CI explicitly requires the NEON backend to be discovered and executed.

### Portability

Checks:

* x86_64 Linux GNU
* x86_64 Linux musl
* AArch64 Linux
* wasm32
* wasm32 with SIMD128

A separate WASI smoke program executes the SIMD128 backend under a WebAssembly runtime.

### Package integrity

Checks dependency footprint, frozen builds, crate contents, and release binary size.

### Real-world smoke tests

Scheduled/manual smoke tests exercise `cargo-simd` against external Rust projects and workspaces.

---

## Platform semantics

Native x86 and AArch64 use runtime feature detection before selecting instruction-set-specific kernels.

WebAssembly differs from native CPU dispatch. SIMD128 code is compiled when the target has `simd128` enabled and must be executed by a WebAssembly runtime supporting that feature.

A baseline WebAssembly build remains scalar.

---

## MSRV

The currently declared and tested minimum Rust version is:

```text
Rust 1.95
```

A lower MSRV may be established in a future release after dedicated CI validation.

---

## Repository layout

```text
.
├── rust/                         rust-simd crate
├── crates/
│   └── cargo-simd/              Cargo developer tooling
├── integration/
│   └── wasm-smoke/              WASM SIMD128 runtime harness
├── bench/                        research benchmarks
├── assembly/                     generated assembly research
├── results/                      benchmark / assembly results
└── scripts/
    └── release_gate.sh           local release verification
```

---

## Project scope

The project focuses on one coherent path:

```text
hardware capabilities
        ↓
SIMD backend selection
        ↓
safe numerical kernels
        ↓
differential verification
        ↓
resource-aware Cargo execution
        ↓
portable release evidence
```

It is not intended to replace specialized assembly inspection, dependency auditing, benchmarking, or test-runner tools.

Where useful, `cargo-simd` can integrate with specialist tools rather than duplicating their functionality.

---

## Current release line

### rust-simd 0.4

Introduces the multi-architecture backend model:

* unified `Capabilities` detection
* AVX2
* AVX2 + FMA
* AArch64 NEON
* WebAssembly SIMD128
* scalar portability fallback
* multi-accumulator reductions and dot products

### cargo-simd 0.1

Introduces:

* `cargo simd doctor`
* `cargo simd verify`
* `cargo simd build`
* `cargo simd test`
* resource-aware execution policy
* offline-first Cargo execution
* optional cargo-nextest integration

---

## License

MIT
