use crate::ffi::{
    dot_zig,
    fma_zig,
    reduce_sum_zig,
    vector_add_zig,
};

#[cfg(rust_zig_simd_native)]
use crate::ffi::{
    dot_zig_simd,
    fma_zig_simd,
    reduce_sum_zig_simd,
    vector_add_zig_simd,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Backend {
    Scalar,
    #[cfg(rust_zig_simd_native)]
    Simd,
}

#[inline]
pub(crate) fn select_backend() -> Backend {
    #[cfg(rust_zig_simd_native)]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                return Backend::Simd;
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            return Backend::Simd;
        }
    }

    Backend::Scalar
}

pub(crate) fn backend_name() -> &'static str {
    match select_backend() {
        Backend::Scalar => "scalar",
        #[cfg(rust_zig_simd_native)]
        Backend::Simd => "simd",
    }
}

#[inline]
pub(crate) fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    match select_backend() {
        Backend::Scalar => vector_add_zig(a, b, out),
        #[cfg(rust_zig_simd_native)]
        Backend::Simd => vector_add_zig_simd(a, b, out),
    }
}

#[inline]
pub(crate) fn fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    match select_backend() {
        Backend::Scalar => fma_zig(a, b, c, out),
        #[cfg(rust_zig_simd_native)]
        Backend::Simd => fma_zig_simd(a, b, c, out),
    }
}

#[inline]
pub(crate) fn reduce_sum(data: &[f32]) -> f32 {
    match select_backend() {
        Backend::Scalar => reduce_sum_zig(data),
        #[cfg(rust_zig_simd_native)]
        Backend::Simd => reduce_sum_zig_simd(data),
    }
}

#[inline]
pub(crate) fn dot(a: &[f32], b: &[f32]) -> f32 {
    match select_backend() {
        Backend::Scalar => dot_zig(a, b),
        #[cfg(rust_zig_simd_native)]
        Backend::Simd => dot_zig_simd(a, b),
    }
}
