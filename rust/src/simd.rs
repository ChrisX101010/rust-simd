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
unsafe fn load(ptr: *const f32) -> __m256 {
    unsafe { _mm256_loadu_ps(ptr) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
unsafe fn store(ptr: *mut f32, value: __m256) {
    unsafe { _mm256_storeu_ps(ptr, value) }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn vector_add_avx2(a: &[f32], b: &[f32], out: &mut [f32]) {
    let mut i = 0;

    while i + 8 <= a.len() {
        let av = unsafe { load(a.as_ptr().add(i)) };
        let bv = unsafe { load(b.as_ptr().add(i)) };
        let result = _mm256_add_ps(av, bv);

        unsafe {
            store(out.as_mut_ptr().add(i), result);
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
        let av = unsafe { load(a.as_ptr().add(i)) };
        let bv = unsafe { load(b.as_ptr().add(i)) };
        let cv = unsafe { load(c.as_ptr().add(i)) };

        let result = _mm256_fmadd_ps(av, bv, cv);

        unsafe {
            store(out.as_mut_ptr().add(i), result);
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
        let v0 = unsafe { load(data.as_ptr().add(i)) };
        let v1 = unsafe { load(data.as_ptr().add(i + 8)) };
        let v2 = unsafe { load(data.as_ptr().add(i + 16)) };
        let v3 = unsafe { load(data.as_ptr().add(i + 24)) };

        acc0 = _mm256_add_ps(acc0, v0);
        acc1 = _mm256_add_ps(acc1, v1);
        acc2 = _mm256_add_ps(acc2, v2);
        acc3 = _mm256_add_ps(acc3, v3);

        i += 32;
    }

    let combined0 = _mm256_add_ps(acc0, acc1);
    let combined1 = _mm256_add_ps(acc2, acc3);
    let combined = _mm256_add_ps(combined0, combined1);

    let mut lanes = [0.0f32; 8];

    unsafe {
        store(lanes.as_mut_ptr(), combined);
    }

    let mut total = 0.0f32;

    for lane in lanes {
        total += lane;
    }

    while i < data.len() {
        total += data[i];
        i += 1;
    }

    total
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn dot_avx2(a: &[f32], b: &[f32]) -> f32 {
    let mut i = 0;
    let mut acc = _mm256_setzero_ps();

    while i + 8 <= a.len() {
        let av = unsafe { load(a.as_ptr().add(i)) };
        let bv = unsafe { load(b.as_ptr().add(i)) };

        acc = _mm256_add_ps(acc, _mm256_mul_ps(av, bv));

        i += 8;
    }

    let mut lanes = [0.0f32; 8];

    unsafe {
        store(lanes.as_mut_ptr(), acc);
    }

    let mut total = 0.0f32;

    for lane in lanes {
        total += lane;
    }

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}
