pub mod ffi;

pub use ffi::{vector_add_zig, vector_add_zig_simd};

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
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33] {
            let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
            let b: Vec<f32> = (0..n).map(|i| (i * 3) as f32).collect();
            let mut out = vec![0.0f32; n];

            vector_add(&a, &b, &mut out);

            for i in 0..n {
                assert_eq!(out[i], a[i] + b[i]);
            }
        }
    }

    #[test]
    #[should_panic]
    fn vector_add_rejects_mismatched_lengths() {
        let a = [1.0, 2.0];
        let b = [3.0];
        let mut out = [0.0, 0.0];

        vector_add(&a, &b, &mut out);
    }
}
