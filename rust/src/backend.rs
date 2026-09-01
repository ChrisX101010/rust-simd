use std::sync::OnceLock;

use crate::capabilities::Capabilities;

#[cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    all(target_arch = "wasm32", target_feature = "simd128")
))]
use crate::simd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackendKind {
    Scalar,
    Avx2,
    Avx2Fma,
    Neon,
    WasmSimd128,
}

impl BackendKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Avx2 => "avx2",
            Self::Avx2Fma => "avx2+fma",
            Self::Neon => "neon",
            Self::WasmSimd128 => "wasm-simd128",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Engine {
    kind: BackendKind,
}

impl Engine {
    /// Creates an engine using the fastest implemented backend
    /// supported by this process.
    #[must_use]
    pub fn auto() -> Self {
        Self {
            kind: Capabilities::detect().best_backend(),
        }
    }

    #[must_use]
    pub const fn scalar() -> Self {
        Self {
            kind: BackendKind::Scalar,
        }
    }

    pub fn avx2() -> crate::Result<Self> {
        Self::for_backend(BackendKind::Avx2)
    }

    pub fn avx2_fma() -> crate::Result<Self> {
        Self::for_backend(BackendKind::Avx2Fma)
    }

    pub fn neon() -> crate::Result<Self> {
        Self::for_backend(BackendKind::Neon)
    }

    pub fn wasm_simd128() -> crate::Result<Self> {
        Self::for_backend(BackendKind::WasmSimd128)
    }

    fn for_backend(kind: BackendKind) -> crate::Result<Self> {
        if backend_available(Capabilities::detect(), kind) {
            Ok(Self { kind })
        } else {
            Err(crate::SimdError::UnsupportedBackend {
                backend: kind.name(),
            })
        }
    }

    #[must_use]
    pub const fn backend(self) -> BackendKind {
        self.kind
    }

    #[must_use]
    pub const fn backend_name(self) -> &'static str {
        self.kind.name()
    }

    #[inline]
    pub fn try_vector_add(self, a: &[f32], b: &[f32], out: &mut [f32]) -> crate::Result<()> {
        crate::error::validate_binary_inputs(a, b, Some(out))?;

        self.vector_add_unchecked_contract(a, b, out);

        Ok(())
    }

    #[inline]
    pub fn vector_add(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        self.try_vector_add(a, b, out)
            .expect("invalid vector_add arguments");
    }

    #[inline]
    pub fn try_fma(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) -> crate::Result<()> {
        crate::error::validate_fma_inputs(a, b, c, out)?;

        self.fma_unchecked_contract(a, b, c, out);

        Ok(())
    }

    #[inline]
    pub fn fma(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
        self.try_fma(a, b, c, out).expect("invalid fma arguments");
    }

    #[inline]
    #[must_use]
    pub fn reduce_sum(self, data: &[f32]) -> f32 {
        match self.kind {
            BackendKind::Scalar => scalar_reduce_sum(data),

            BackendKind::Avx2 | BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY: Engine construction proves AVX2
                    // support before this backend can be selected.
                    return unsafe { simd::reduce_sum_avx2_4acc(data) };
                }

                #[allow(unreachable_code)]
                scalar_reduce_sum(data)
            }

            BackendKind::Neon => {
                #[cfg(target_arch = "aarch64")]
                {
                    // SAFETY: Engine construction proves NEON
                    // support before this backend can be selected.
                    return unsafe { simd::reduce_sum_neon_4acc(data) };
                }

                #[allow(unreachable_code)]
                scalar_reduce_sum(data)
            }

            BackendKind::WasmSimd128 => {
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    // SAFETY: this code only exists in a
                    // simd128-enabled WebAssembly build.
                    return unsafe { simd::reduce_sum_wasm_simd128_4acc(data) };
                }

                #[allow(unreachable_code)]
                scalar_reduce_sum(data)
            }
        }
    }

    #[inline]
    pub fn try_dot(self, a: &[f32], b: &[f32]) -> crate::Result<f32> {
        crate::error::validate_binary_inputs(a, b, None)?;

        Ok(self.dot_unchecked_contract(a, b))
    }

    #[inline]
    #[must_use]
    pub fn dot(self, a: &[f32], b: &[f32]) -> f32 {
        self.try_dot(a, b).expect("invalid dot arguments")
    }

    #[inline]
    fn vector_add_unchecked_contract(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        match self.kind {
            BackendKind::Scalar => {
                scalar_vector_add(a, b, out);
            }

            BackendKind::Avx2 | BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY: backend selection proves AVX2.
                    unsafe {
                        simd::vector_add_avx2(a, b, out);
                    }

                    return;
                }

                #[allow(unreachable_code)]
                scalar_vector_add(a, b, out);
            }

            BackendKind::Neon => {
                #[cfg(target_arch = "aarch64")]
                {
                    // SAFETY: backend selection proves NEON.
                    unsafe {
                        simd::vector_add_neon(a, b, out);
                    }

                    return;
                }

                #[allow(unreachable_code)]
                scalar_vector_add(a, b, out);
            }

            BackendKind::WasmSimd128 => {
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    // SAFETY: this backend only exists in a
                    // simd128-enabled WebAssembly build.
                    unsafe {
                        simd::vector_add_wasm_simd128(a, b, out);
                    }

                    return;
                }

                #[allow(unreachable_code)]
                scalar_vector_add(a, b, out);
            }
        }
    }

    #[inline]
    fn fma_unchecked_contract(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
        match self.kind {
            BackendKind::Scalar | BackendKind::Avx2 | BackendKind::WasmSimd128 => {
                // Baseline WebAssembly SIMD128 has no strict
                // fused f32 multiply-add instruction matching
                // Rust f32::mul_add semantics, so preserve the
                // numerical contract through the scalar kernel.
                scalar_fma(a, b, c, out);
            }

            BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY: backend selection proves AVX2+FMA.
                    unsafe {
                        simd::fma_avx2_fma(a, b, c, out);
                    }

                    return;
                }

                #[allow(unreachable_code)]
                scalar_fma(a, b, c, out);
            }

            BackendKind::Neon => {
                #[cfg(target_arch = "aarch64")]
                {
                    // SAFETY: backend selection proves NEON.
                    unsafe {
                        simd::fma_neon(a, b, c, out);
                    }

                    return;
                }

                #[allow(unreachable_code)]
                scalar_fma(a, b, c, out);
            }
        }
    }

    #[inline]
    fn dot_unchecked_contract(self, a: &[f32], b: &[f32]) -> f32 {
        match self.kind {
            BackendKind::Scalar => scalar_dot(a, b),

            BackendKind::Avx2 | BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY: backend selection proves AVX2.
                    return unsafe { simd::dot_avx2_4acc(a, b) };
                }

                #[allow(unreachable_code)]
                scalar_dot(a, b)
            }

            BackendKind::Neon => {
                #[cfg(target_arch = "aarch64")]
                {
                    // SAFETY: backend selection proves NEON.
                    return unsafe { simd::dot_neon_4acc(a, b) };
                }

                #[allow(unreachable_code)]
                scalar_dot(a, b)
            }

            BackendKind::WasmSimd128 => {
                #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
                {
                    // SAFETY: this backend only exists in a
                    // simd128-enabled WebAssembly build.
                    return unsafe { simd::dot_wasm_simd128_4acc(a, b) };
                }

                #[allow(unreachable_code)]
                scalar_dot(a, b)
            }
        }
    }
}

static AUTO_ENGINE: OnceLock<Engine> = OnceLock::new();

#[inline]
pub(crate) fn auto_engine() -> &'static Engine {
    AUTO_ENGINE.get_or_init(Engine::auto)
}

fn backend_available(capabilities: Capabilities, kind: BackendKind) -> bool {
    match kind {
        BackendKind::Scalar => true,

        BackendKind::Avx2 => capabilities.has_avx2(),

        BackendKind::Avx2Fma => capabilities.has_avx2() && capabilities.has_fma(),

        BackendKind::Neon => capabilities.has_neon(),

        BackendKind::WasmSimd128 => capabilities.has_wasm_simd128(),
    }
}

#[inline]
fn scalar_vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    for ((av, bv), outv) in a.iter().zip(b.iter()).zip(out.iter_mut()) {
        *outv = av + bv;
    }
}

#[inline]
fn scalar_fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    for (((av, bv), cv), outv) in a.iter().zip(b.iter()).zip(c.iter()).zip(out.iter_mut()) {
        *outv = av.mul_add(*bv, *cv);
    }
}

#[inline]
fn scalar_reduce_sum(data: &[f32]) -> f32 {
    data.iter().map(|&value| value as f64).sum::<f64>() as f32
}

#[inline]
fn scalar_dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(av, bv)| (*av as f64) * (*bv as f64))
        .sum::<f64>() as f32
}
