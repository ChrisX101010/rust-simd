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
