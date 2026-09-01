mod backend;
mod capabilities;
mod error;
mod simd;

pub use backend::{BackendKind, Engine};
pub use capabilities::{Architecture, Capabilities, VectorModel};
pub use error::{Result, SimdError};

/// Returns an engine using the best supported backend for this process.
#[inline]
pub fn engine() -> Engine {
    *backend::auto_engine()
}

/// Returns the automatically selected backend.
#[inline]
pub fn backend() -> BackendKind {
    engine().backend()
}

/// Returns the name of the automatically selected backend.
#[inline]
pub fn backend_name() -> &'static str {
    engine().backend_name()
}

/// Checked element-wise vector addition using automatic dispatch.
#[inline]
pub fn try_vector_add(a: &[f32], b: &[f32], out: &mut [f32]) -> Result<()> {
    engine().try_vector_add(a, b, out)
}

/// Element-wise vector addition using automatic dispatch.
#[inline]
pub fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    engine().vector_add(a, b, out);
}

/// Checked fused multiply-add using automatic dispatch.
#[inline]
pub fn try_fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) -> Result<()> {
    engine().try_fma(a, b, c, out)
}

/// Element-wise fused multiply-add using automatic dispatch.
#[inline]
pub fn fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    engine().fma(a, b, c, out);
}

/// Computes the sum using automatic dispatch.
#[inline]
pub fn reduce_sum(data: &[f32]) -> f32 {
    engine().reduce_sum(data)
}

/// Checked dot product using automatic dispatch.
#[inline]
pub fn try_dot(a: &[f32], b: &[f32]) -> Result<f32> {
    engine().try_dot(a, b)
}

/// Computes the dot product using automatic dispatch.
#[inline]
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    engine().dot(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automatic_backend_is_known() {
        assert!(matches!(
            backend(),
            BackendKind::Scalar
                | BackendKind::Avx2
                | BackendKind::Avx2Fma
                | BackendKind::Neon
                | BackendKind::WasmSimd128
        ));
    }

    #[test]
    fn scalar_engine_is_available() {
        assert_eq!(Engine::scalar().backend(), BackendKind::Scalar);
    }

    #[test]
    fn vector_add_basic() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [10.0f32, 20.0, 30.0, 40.0];
        let mut out = [0.0f32; 4];

        vector_add(&a, &b, &mut out);

        assert_eq!(out, [11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn scalar_engine_vector_add() {
        let engine = Engine::scalar();

        let a = [1.0f32, 2.0, 3.0];
        let b = [10.0f32, 20.0, 30.0];
        let mut out = [0.0f32; 3];

        engine.vector_add(&a, &b, &mut out);

        assert_eq!(out, [11.0, 22.0, 33.0]);
    }

    #[test]
    fn scalar_engine_checked_vector_add() {
        let engine = Engine::scalar();

        let a = [1.0f32, 2.0];
        let b = [3.0f32];
        let mut out = [0.0f32; 2];

        assert!(matches!(
            engine.try_vector_add(&a, &b, &mut out),
            Err(SimdError::InputLengthMismatch { left: 2, right: 1 })
        ));
    }

    #[test]
    fn scalar_engine_fma() {
        let engine = Engine::scalar();

        let a = [1.0f32, 2.0, 3.0];
        let b = [2.0f32, 3.0, 4.0];
        let c = [10.0f32; 3];
        let mut out = [0.0f32; 3];

        engine.fma(&a, &b, &c, &mut out);

        assert_eq!(out, [12.0, 16.0, 22.0]);
    }

    #[test]
    fn scalar_engine_reduce() {
        let engine = Engine::scalar();

        assert!((engine.reduce_sum(&[1.0, 2.0, 3.0, 4.0]) - 10.0).abs() <= 1e-6);
    }

    #[test]
    fn scalar_engine_dot() {
        let engine = Engine::scalar();

        assert!((engine.dot(&[1.0, 2.0], &[3.0, 4.0]) - 11.0).abs() <= 1e-6);
    }

    #[test]
    fn avx2_constructor_reports_cpu_support() {
        match Engine::avx2() {
            Ok(engine) => assert_eq!(engine.backend(), BackendKind::Avx2),
            Err(error) => assert!(matches!(error, SimdError::UnsupportedBackend { .. })),
        }
    }

    #[test]
    fn avx2_fma_constructor_reports_cpu_support() {
        match Engine::avx2_fma() {
            Ok(engine) => assert_eq!(engine.backend(), BackendKind::Avx2Fma),
            Err(error) => assert!(matches!(error, SimdError::UnsupportedBackend { .. })),
        }
    }

    #[test]
    fn neon_constructor_reports_platform_support() {
        match Engine::neon() {
            Ok(engine) => {
                assert_eq!(engine.backend(), BackendKind::Neon);
            }
            Err(error) => {
                assert!(matches!(error, SimdError::UnsupportedBackend { .. }));
            }
        }
    }

    #[test]
    fn wasm_simd128_constructor_reports_build_support() {
        match Engine::wasm_simd128() {
            Ok(engine) => {
                assert_eq!(engine.backend(), BackendKind::WasmSimd128);
            }
            Err(error) => {
                assert!(matches!(error, SimdError::UnsupportedBackend { .. }));
            }
        }
    }

    #[test]
    fn large_vector_add() {
        let n = 100_003;

        let a: Vec<f32> = (0..n).map(|i| (i % 997) as f32).collect();
        let b: Vec<f32> = (0..n).map(|i| (i % 991) as f32).collect();
        let mut out = vec![0.0f32; n];

        vector_add(&a, &b, &mut out);

        for i in 0..n {
            assert_eq!(out[i], a[i] + b[i]);
        }
    }
}
