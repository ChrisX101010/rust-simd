// ============================================================
// x86 / x86_64 AVX2
// ============================================================

#[cfg(target_arch = "x86")]
use core::arch::x86::{
    __m256, _mm256_add_ps, _mm256_fmadd_ps, _mm256_loadu_ps, _mm256_mul_ps, _mm256_setzero_ps,
    _mm256_storeu_ps,
};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m256, _mm256_add_ps, _mm256_fmadd_ps, _mm256_loadu_ps, _mm256_mul_ps, _mm256_setzero_ps,
    _mm256_storeu_ps,
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn load_avx2(ptr: *const f32) -> __m256 {
    // SAFETY: caller guarantees that at least eight f32 values
    // are readable from ptr. loadu has no alignment requirement.
    unsafe { _mm256_loadu_ps(ptr) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn store_avx2(ptr: *mut f32, value: __m256) {
    // SAFETY: caller guarantees that at least eight f32 values
    // are writable from ptr. storeu has no alignment requirement.
    unsafe { _mm256_storeu_ps(ptr, value) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn vector_add_avx2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 8 <= a.len() {
        let av = unsafe { load_avx2(a.as_ptr().add(i)) };
        let bv = unsafe { load_avx2(b.as_ptr().add(i)) };

        let result = _mm256_add_ps(av, bv);

        unsafe {
            store_avx2(out.as_mut_ptr().add(i), result);
        }

        i += 8;
    }

    while i < a.len() {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
pub(crate) unsafe fn fma_avx2_fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 8 <= a.len() {
        let av = unsafe { load_avx2(a.as_ptr().add(i)) };
        let bv = unsafe { load_avx2(b.as_ptr().add(i)) };
        let cv = unsafe { load_avx2(c.as_ptr().add(i)) };

        let result = _mm256_fmadd_ps(av, bv, cv);

        unsafe {
            store_avx2(out.as_mut_ptr().add(i), result);
        }

        i += 8;
    }

    while i < a.len() {
        out[i] = a[i].mul_add(b[i], c[i]);
        i += 1;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn reduce_sum_avx2_4acc(data: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = _mm256_setzero_ps();
    let mut acc1 = _mm256_setzero_ps();
    let mut acc2 = _mm256_setzero_ps();
    let mut acc3 = _mm256_setzero_ps();

    while i + 32 <= data.len() {
        let v0 = unsafe { load_avx2(data.as_ptr().add(i)) };
        let v1 = unsafe { load_avx2(data.as_ptr().add(i + 8)) };
        let v2 = unsafe { load_avx2(data.as_ptr().add(i + 16)) };
        let v3 = unsafe { load_avx2(data.as_ptr().add(i + 24)) };

        acc0 = _mm256_add_ps(acc0, v0);
        acc1 = _mm256_add_ps(acc1, v1);
        acc2 = _mm256_add_ps(acc2, v2);
        acc3 = _mm256_add_ps(acc3, v3);

        i += 32;
    }

    let combined = _mm256_add_ps(_mm256_add_ps(acc0, acc1), _mm256_add_ps(acc2, acc3));

    let mut lanes = [0.0_f32; 8];

    unsafe {
        store_avx2(lanes.as_mut_ptr(), combined);
    }

    let mut total = lanes.into_iter().sum::<f32>();

    while i < data.len() {
        total += data[i];
        i += 1;
    }

    total
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn dot_avx2_4acc(a: &[f32], b: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = _mm256_setzero_ps();
    let mut acc1 = _mm256_setzero_ps();
    let mut acc2 = _mm256_setzero_ps();
    let mut acc3 = _mm256_setzero_ps();

    while i + 32 <= a.len() {
        let a0 = unsafe { load_avx2(a.as_ptr().add(i)) };
        let a1 = unsafe { load_avx2(a.as_ptr().add(i + 8)) };
        let a2 = unsafe { load_avx2(a.as_ptr().add(i + 16)) };
        let a3 = unsafe { load_avx2(a.as_ptr().add(i + 24)) };

        let b0 = unsafe { load_avx2(b.as_ptr().add(i)) };
        let b1 = unsafe { load_avx2(b.as_ptr().add(i + 8)) };
        let b2 = unsafe { load_avx2(b.as_ptr().add(i + 16)) };
        let b3 = unsafe { load_avx2(b.as_ptr().add(i + 24)) };

        acc0 = _mm256_add_ps(acc0, _mm256_mul_ps(a0, b0));
        acc1 = _mm256_add_ps(acc1, _mm256_mul_ps(a1, b1));
        acc2 = _mm256_add_ps(acc2, _mm256_mul_ps(a2, b2));
        acc3 = _mm256_add_ps(acc3, _mm256_mul_ps(a3, b3));

        i += 32;
    }

    let combined = _mm256_add_ps(_mm256_add_ps(acc0, acc1), _mm256_add_ps(acc2, acc3));

    let mut lanes = [0.0_f32; 8];

    unsafe {
        store_avx2(lanes.as_mut_ptr(), combined);
    }

    let mut total = lanes.into_iter().sum::<f32>();

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}

// ============================================================
// AArch64 NEON
// ============================================================

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    float32x4_t, vaddq_f32, vaddvq_f32, vdupq_n_f32, vfmaq_f32, vld1q_f32, vmulq_f32, vst1q_f32,
};

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn load_neon(ptr: *const f32) -> float32x4_t {
    // SAFETY: caller guarantees four readable f32 values.
    unsafe { vld1q_f32(ptr) }
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn store_neon(ptr: *mut f32, value: float32x4_t) {
    // SAFETY: caller guarantees four writable f32 values.
    unsafe { vst1q_f32(ptr, value) }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn vector_add_neon(a: &[f32], b: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 4 <= a.len() {
        let av = unsafe { load_neon(a.as_ptr().add(i)) };
        let bv = unsafe { load_neon(b.as_ptr().add(i)) };

        let result = vaddq_f32(av, bv);

        unsafe {
            store_neon(out.as_mut_ptr().add(i), result);
        }

        i += 4;
    }

    while i < a.len() {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn fma_neon(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 4 <= a.len() {
        let av = unsafe { load_neon(a.as_ptr().add(i)) };
        let bv = unsafe { load_neon(b.as_ptr().add(i)) };
        let cv = unsafe { load_neon(c.as_ptr().add(i)) };

        // vfmaq_f32(accumulator, lhs, rhs)
        let result = vfmaq_f32(cv, av, bv);

        unsafe {
            store_neon(out.as_mut_ptr().add(i), result);
        }

        i += 4;
    }

    while i < a.len() {
        out[i] = a[i].mul_add(b[i], c[i]);
        i += 1;
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn reduce_sum_neon_4acc(data: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = vdupq_n_f32(0.0);
    let mut acc1 = vdupq_n_f32(0.0);
    let mut acc2 = vdupq_n_f32(0.0);
    let mut acc3 = vdupq_n_f32(0.0);

    while i + 16 <= data.len() {
        let v0 = unsafe { load_neon(data.as_ptr().add(i)) };
        let v1 = unsafe { load_neon(data.as_ptr().add(i + 4)) };
        let v2 = unsafe { load_neon(data.as_ptr().add(i + 8)) };
        let v3 = unsafe { load_neon(data.as_ptr().add(i + 12)) };

        acc0 = vaddq_f32(acc0, v0);
        acc1 = vaddq_f32(acc1, v1);
        acc2 = vaddq_f32(acc2, v2);
        acc3 = vaddq_f32(acc3, v3);

        i += 16;
    }

    let combined = vaddq_f32(vaddq_f32(acc0, acc1), vaddq_f32(acc2, acc3));

    let mut total = vaddvq_f32(combined);

    while i < data.len() {
        total += data[i];
        i += 1;
    }

    total
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn dot_neon_4acc(a: &[f32], b: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = vdupq_n_f32(0.0);
    let mut acc1 = vdupq_n_f32(0.0);
    let mut acc2 = vdupq_n_f32(0.0);
    let mut acc3 = vdupq_n_f32(0.0);

    while i + 16 <= a.len() {
        let a0 = unsafe { load_neon(a.as_ptr().add(i)) };
        let a1 = unsafe { load_neon(a.as_ptr().add(i + 4)) };
        let a2 = unsafe { load_neon(a.as_ptr().add(i + 8)) };
        let a3 = unsafe { load_neon(a.as_ptr().add(i + 12)) };

        let b0 = unsafe { load_neon(b.as_ptr().add(i)) };
        let b1 = unsafe { load_neon(b.as_ptr().add(i + 4)) };
        let b2 = unsafe { load_neon(b.as_ptr().add(i + 8)) };
        let b3 = unsafe { load_neon(b.as_ptr().add(i + 12)) };

        acc0 = vaddq_f32(acc0, vmulq_f32(a0, b0));
        acc1 = vaddq_f32(acc1, vmulq_f32(a1, b1));
        acc2 = vaddq_f32(acc2, vmulq_f32(a2, b2));
        acc3 = vaddq_f32(acc3, vmulq_f32(a3, b3));

        i += 16;
    }

    let combined = vaddq_f32(vaddq_f32(acc0, acc1), vaddq_f32(acc2, acc3));

    let mut total = vaddvq_f32(combined);

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}

// ============================================================
// WebAssembly SIMD128
// ============================================================

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use core::arch::wasm32::{f32x4_add, f32x4_mul, f32x4_splat, v128, v128_load, v128_store};

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[inline]
unsafe fn load_wasm(ptr: *const f32) -> v128 {
    // SAFETY: caller guarantees four readable f32 values.
    unsafe { v128_load(ptr.cast::<v128>()) }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[inline]
unsafe fn store_wasm(ptr: *mut f32, value: v128) {
    // SAFETY: caller guarantees four writable f32 values.
    unsafe { v128_store(ptr.cast::<v128>(), value) }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(crate) unsafe fn vector_add_wasm_simd128(a: &[f32], b: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 4 <= a.len() {
        let av = unsafe { load_wasm(a.as_ptr().add(i)) };
        let bv = unsafe { load_wasm(b.as_ptr().add(i)) };

        let result = f32x4_add(av, bv);

        unsafe {
            store_wasm(out.as_mut_ptr().add(i), result);
        }

        i += 4;
    }

    while i < a.len() {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(crate) unsafe fn reduce_sum_wasm_simd128_4acc(data: &[f32]) -> f32 {
    let mut i = 0;

    let zero = f32x4_splat(0.0);

    let mut acc0 = zero;
    let mut acc1 = zero;
    let mut acc2 = zero;
    let mut acc3 = zero;

    while i + 16 <= data.len() {
        let v0 = unsafe { load_wasm(data.as_ptr().add(i)) };
        let v1 = unsafe { load_wasm(data.as_ptr().add(i + 4)) };
        let v2 = unsafe { load_wasm(data.as_ptr().add(i + 8)) };
        let v3 = unsafe { load_wasm(data.as_ptr().add(i + 12)) };

        acc0 = f32x4_add(acc0, v0);
        acc1 = f32x4_add(acc1, v1);
        acc2 = f32x4_add(acc2, v2);
        acc3 = f32x4_add(acc3, v3);

        i += 16;
    }

    let combined = f32x4_add(f32x4_add(acc0, acc1), f32x4_add(acc2, acc3));

    let mut lanes = [0.0_f32; 4];

    unsafe {
        store_wasm(lanes.as_mut_ptr(), combined);
    }

    let mut total = lanes.into_iter().sum::<f32>();

    while i < data.len() {
        total += data[i];
        i += 1;
    }

    total
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(crate) unsafe fn dot_wasm_simd128_4acc(a: &[f32], b: &[f32]) -> f32 {
    let mut i = 0;

    let zero = f32x4_splat(0.0);

    let mut acc0 = zero;
    let mut acc1 = zero;
    let mut acc2 = zero;
    let mut acc3 = zero;

    while i + 16 <= a.len() {
        let a0 = unsafe { load_wasm(a.as_ptr().add(i)) };
        let a1 = unsafe { load_wasm(a.as_ptr().add(i + 4)) };
        let a2 = unsafe { load_wasm(a.as_ptr().add(i + 8)) };
        let a3 = unsafe { load_wasm(a.as_ptr().add(i + 12)) };

        let b0 = unsafe { load_wasm(b.as_ptr().add(i)) };
        let b1 = unsafe { load_wasm(b.as_ptr().add(i + 4)) };
        let b2 = unsafe { load_wasm(b.as_ptr().add(i + 8)) };
        let b3 = unsafe { load_wasm(b.as_ptr().add(i + 12)) };

        acc0 = f32x4_add(acc0, f32x4_mul(a0, b0));
        acc1 = f32x4_add(acc1, f32x4_mul(a1, b1));
        acc2 = f32x4_add(acc2, f32x4_mul(a2, b2));
        acc3 = f32x4_add(acc3, f32x4_mul(a3, b3));

        i += 16;
    }

    let combined = f32x4_add(f32x4_add(acc0, acc1), f32x4_add(acc2, acc3));

    let mut lanes = [0.0_f32; 4];

    unsafe {
        store_wasm(lanes.as_mut_ptr(), combined);
    }

    let mut total = lanes.into_iter().sum::<f32>();

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}
