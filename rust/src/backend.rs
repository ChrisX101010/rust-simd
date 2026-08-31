use std::sync::OnceLock;

use crate::simd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Scalar,
    Avx2,
    Avx2Fma,
}

impl BackendKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Avx2 => "avx2",
            Self::Avx2Fma => "avx2+fma",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BackendAvailability {
    Available,
    Unsupported,
}

#[derive(Debug, Clone, Copy)]
pub struct Engine {
    kind: BackendKind,
}

impl Engine {
    /// Creates an engine using the best backend supported by the current CPU.
    pub fn auto() -> Self {
        Self {
            kind: select_backend(),
        }
    }

    /// Creates a scalar engine.
    pub const fn scalar() -> Self {
        Self {
            kind: BackendKind::Scalar,
        }
    }

    /// Creates an AVX2 engine when AVX2 is supported.
    pub fn avx2() -> crate::Result<Self> {
        if backend_available(BackendKind::Avx2) == BackendAvailability::Available {
            Ok(Self {
                kind: BackendKind::Avx2,
            })
        } else {
            Err(crate::SimdError::UnsupportedBackend {
                backend: BackendKind::Avx2.name(),
            })
        }
    }

    /// Creates an AVX2+FMA engine when both instruction sets are supported.
    pub fn avx2_fma() -> crate::Result<Self> {
        if backend_available(BackendKind::Avx2Fma) == BackendAvailability::Available {
            Ok(Self {
                kind: BackendKind::Avx2Fma,
            })
        } else {
            Err(crate::SimdError::UnsupportedBackend {
                backend: BackendKind::Avx2Fma.name(),
            })
        }
    }

    /// Returns the backend used by this engine.
    pub const fn backend(self) -> BackendKind {
        self.kind
    }

    /// Returns the human-readable backend name.
    pub const fn backend_name(self) -> &'static str {
        self.kind.name()
    }

    /// Performs checked element-wise vector addition.
    ///
    /// The supplied slices must have matching lengths.
    #[inline]
    pub fn try_vector_add(self, a: &[f32], b: &[f32], out: &mut [f32]) -> crate::Result<()> {
        crate::error::validate_binary_inputs(a, b, Some(out))?;
        self.vector_add_unchecked_contract(a, b, out);
        Ok(())
    }

    /// Performs element-wise vector addition.
    ///
    /// # Panics
    ///
    /// Panics if the slices have incompatible lengths.
    #[inline]
    pub fn vector_add(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        self.try_vector_add(a, b, out)
            .expect("invalid vector_add arguments");
    }

    /// Performs checked fused multiply-add.
    ///
    /// Computes `out[i] = a[i] * b[i] + c[i]`.
    #[inline]
    pub fn try_fma(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) -> crate::Result<()> {
        crate::error::validate_fma_inputs(a, b, c, out)?;
        self.fma_unchecked_contract(a, b, c, out);
        Ok(())
    }

    /// Performs element-wise fused multiply-add.
    ///
    /// # Panics
    ///
    /// Panics if the slices have incompatible lengths.
    #[inline]
    pub fn fma(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
        self.try_fma(a, b, c, out).expect("invalid fma arguments");
    }

    /// Computes a sum reduction.
    #[inline]
    pub fn reduce_sum(self, data: &[f32]) -> f32 {
        match self.kind {
            BackendKind::Scalar => scalar_reduce_sum(data),

            BackendKind::Avx2 | BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY:
                    // An AVX2 engine can only be constructed after runtime
                    // AVX2 capability detection succeeds.
                    unsafe { simd::reduce_sum_avx2_4acc(data) }
                }

                #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
                {
                    scalar_reduce_sum(data)
                }
            }
        }
    }

    /// Computes a checked dot product.
    #[inline]
    pub fn try_dot(self, a: &[f32], b: &[f32]) -> crate::Result<f32> {
        crate::error::validate_binary_inputs(a, b, None)?;
        Ok(self.dot_unchecked_contract(a, b))
    }

    /// Computes the dot product.
    ///
    /// # Panics
    ///
    /// Panics if the slices have different lengths.
    #[inline]
    pub fn dot(self, a: &[f32], b: &[f32]) -> f32 {
        self.try_dot(a, b).expect("invalid dot arguments")
    }

    #[inline]
    fn vector_add_unchecked_contract(self, a: &[f32], b: &[f32], out: &mut [f32]) {
        match self.kind {
            BackendKind::Scalar => scalar_vector_add(a, b, out),

            BackendKind::Avx2 | BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY:
                    // This path is reachable only for an Engine created through
                    // a validated AVX2 constructor or automatic dispatch.
                    unsafe { simd::vector_add_avx2(a, b, out) }
                }

                #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
                {
                    scalar_vector_add(a, b, out)
                }
            }
        }
    }

    #[inline]
    fn fma_unchecked_contract(self, a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
        match self.kind {
            BackendKind::Scalar | BackendKind::Avx2 => {
                scalar_fma(a, b, c, out);
            }

            BackendKind::Avx2Fma => {
                #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
                {
                    // SAFETY:
                    // This path is reachable only after AVX2+FMA detection.
                    unsafe { simd::fma_avx2_fma(a, b, c, out) }
                }

                #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
                {
                    scalar_fma(a, b, c, out);
                }
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
                    // SAFETY:
                    // An AVX2 engine can only be constructed after runtime
                    // AVX2 capability detection succeeds.
                    unsafe { simd::dot_avx2(a, b) }
                }

                #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
                {
                    scalar_dot(a, b)
                }
            }
        }
    }
}

static AUTO_ENGINE: OnceLock<Engine> = OnceLock::new();

#[inline]
pub(crate) fn auto_engine() -> &'static Engine {
    AUTO_ENGINE.get_or_init(Engine::auto)
}

fn select_backend() -> BackendKind {
    if backend_available(BackendKind::Avx2Fma) == BackendAvailability::Available {
        return BackendKind::Avx2Fma;
    }

    if backend_available(BackendKind::Avx2) == BackendAvailability::Available {
        return BackendKind::Avx2;
    }

    BackendKind::Scalar
}

fn backend_available(kind: BackendKind) -> BackendAvailability {
    match kind {
        BackendKind::Scalar => BackendAvailability::Available,

        BackendKind::Avx2 => {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                if std::arch::is_x86_feature_detected!("avx2") {
                    return BackendAvailability::Available;
                }
            }

            BackendAvailability::Unsupported
        }

        BackendKind::Avx2Fma => {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                if std::arch::is_x86_feature_detected!("avx2")
                    && std::arch::is_x86_feature_detected!("fma")
                {
                    return BackendAvailability::Available;
                }
            }

            BackendAvailability::Unsupported
        }
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
