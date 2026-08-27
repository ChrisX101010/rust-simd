pub mod ffi;

pub use ffi::{
    fma_zig, fma_zig_simd, reduce_sum_zig, reduce_sum_zig_simd, reduce_sum_zig_simd_4acc,
    vector_add_zig, vector_add_zig_simd,
};

pub fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_add_basic() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [10.0, 20.0, 30.0, 40.0];
        let mut out = [0.0; 4];

        vector_add(&a, &b, &mut out);

        assert_eq!(out, [11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn vector_add_edge_lengths() {
        for n in [0, 1, 7, 8, 9, 16, 31, 32, 33] {
            let a = vec![1.0f32; n];
            let b = vec![2.0f32; n];
            let mut out = vec![0.0f32; n];

            vector_add(&a, &b, &mut out);

            assert_eq!(out, vec![3.0f32; n]);
        }
    }

    #[test]
    #[should_panic]
    fn vector_add_rejects_mismatched_lengths() {
        let a = [1.0f32, 2.0];
        let b = [3.0f32];
        let mut out = [0.0f32, 0.0];

        vector_add(&a, &b, &mut out);
    }
}
