pub mod ffi;

pub use ffi::{
    dot_zig, dot_zig_simd, fma_zig, fma_zig_simd, reduce_sum_zig, reduce_sum_zig_simd,
    reduce_sum_zig_simd_4acc, vector_add_zig, vector_add_zig_simd,
};

/// Element-wise vector addition.
pub fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

/// Computes `a * b + c` using the optimized backend.
pub fn fma(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    fma_zig_simd(a, b, c, out);
}

/// Computes a sum reduction using the optimized backend.
pub fn reduce_sum(data: &[f32]) -> f32 {
    reduce_sum_zig_simd(data)
}

/// Computes a dot product using the optimized backend.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    dot_zig_simd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_fma_matches_reference() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [2.0f32, 3.0, 4.0, 5.0];
        let c = [10.0f32; 4];
        let mut out = [0.0f32; 4];

        fma(&a, &b, &c, &mut out);

        assert_eq!(out, [12.0, 16.0, 22.0, 30.0]);
    }

    #[test]
    fn public_reduce_sum_matches_reference() {
        let data = [1.0f32, 2.0, 3.0, 4.0];

        let result = reduce_sum(&data);

        assert!((result - 10.0).abs() <= 1.0e-6);
    }

    #[test]
    fn public_dot_matches_reference() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [5.0f32, 6.0, 7.0, 8.0];

        let result = dot(&a, &b);

        assert!((result - 70.0).abs() <= 1.0e-6);
    }

    #[test]
    fn public_vector_add_basic() {
        let a = [1.0f32, 2.0, 3.0, 4.0];
        let b = [10.0f32, 20.0, 30.0, 40.0];
        let mut out = [0.0f32; 4];

        vector_add(&a, &b, &mut out);

        assert_eq!(out, [11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    #[should_panic(expected = "input lengths must match")]
    fn public_dot_rejects_mismatched_lengths() {
        let a = [1.0f32, 2.0];
        let b = [3.0f32];

        let _ = dot(&a, &b);
    }
}
