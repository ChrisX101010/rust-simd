unsafe extern "C" {
    fn fma_f32(a: *const f32, b: *const f32, c: *const f32, out: *mut f32, len: usize);

    fn fma_f32_simd(a: *const f32, b: *const f32, c: *const f32, out: *mut f32, len: usize);

    fn vector_add_f32(a: *const f32, b: *const f32, out: *mut f32, len: usize);

    fn vector_add_f32_simd(a: *const f32, b: *const f32, out: *mut f32, len: usize);

    fn reduce_sum_f32(data: *const f32, len: usize) -> f32;

    fn reduce_sum_f32_simd(data: *const f32, len: usize) -> f32;

    fn reduce_sum_f32_simd_4acc(data: *const f32, len: usize) -> f32;
}

/// Calls the Zig scalar implementation through the C-compatible ABI.
pub fn vector_add_zig(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        vector_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// Calls the Zig explicit-SIMD implementation through the C-compatible ABI.
///
/// The Rust-facing API is safe because the wrapper validates all slice
/// length invariants before crossing the unsafe FFI boundary.
pub fn vector_add_zig_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        vector_add_f32_simd(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), out.len());
    }
}

/// Computes `a * b + c` through the Zig backend.
pub fn fma_zig(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), c.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        fma_f32(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

/// Computes `a * b + c` through the Zig explicit-SIMD backend.
pub fn fma_zig_simd(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), c.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        fma_f32_simd(
            a.as_ptr(),
            b.as_ptr(),
            c.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

/// Reduces a slice through the Zig scalar backend.
pub fn reduce_sum_zig(data: &[f32]) -> f32 {
    unsafe { reduce_sum_f32(data.as_ptr(), data.len()) }
}

/// Reduces a slice through the Zig SIMD backend.
pub fn reduce_sum_zig_simd(data: &[f32]) -> f32 {
    unsafe { reduce_sum_f32_simd(data.as_ptr(), data.len()) }
}

/// Reduces a slice through the Zig SIMD multi-accumulator backend.
pub fn reduce_sum_zig_simd_4acc(data: &[f32]) -> f32 {
    unsafe { reduce_sum_f32_simd_4acc(data.as_ptr(), data.len()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter().zip(b).map(|(&x, &y)| x + y).collect()
    }

    fn check_case(n: usize) {
        let a: Vec<f32> = (0..n).map(|i| (i % 1000) as f32).collect();

        let b: Vec<f32> = (0..n).map(|i| ((i * 3) % 1000) as f32).collect();

        let expected = expected(&a, &b);

        let mut scalar = vec![0.0f32; n];
        let mut simd = vec![0.0f32; n];

        vector_add_zig(&a, &b, &mut scalar);
        vector_add_zig_simd(&a, &b, &mut simd);

        assert_eq!(scalar, expected);
        assert_eq!(simd, expected);
        assert_eq!(scalar, simd);
    }

    #[test]
    fn ffi_boundary_lengths() {
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33] {
            check_case(n);
        }
    }

    #[test]
    fn ffi_large_input() {
        check_case(100_003);
    }

    #[test]
    fn reduction_ffi_matches_reference() {
        for n in [0, 1, 7, 8, 9, 15, 16, 31, 32, 33, 100_003] {
            let data: Vec<f32> = (0..n).map(|i| ((i % 997) as f32) * 0.001).collect();

            let reference: f64 = data.iter().map(|&x| x as f64).sum();

            let scalar = reduce_sum_zig(&data) as f64;
            let simd = reduce_sum_zig_simd(&data) as f64;
            let simd_4acc = reduce_sum_zig_simd_4acc(&data) as f64;

            for (name, value) in [("scalar", scalar), ("simd", simd), ("simd_4acc", simd_4acc)] {
                let error = (value - reference).abs();

                assert!(
                    error <= reference.abs() * 1.0e-5 + 1.0e-2,
                    "{name} reduction error too large: value={value}, reference={reference}, error={error}"
                );
            }
        }
    }

    #[test]
    fn fma_ffi_matches_reference() {
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 100_003] {
            let a: Vec<f32> = (0..n).map(|i| (i % 997) as f32 * 0.001).collect();

            let b: Vec<f32> = (0..n).map(|i| (i % 991) as f32 * 0.002).collect();

            let c: Vec<f32> = (0..n).map(|i| (i % 983) as f32 * 0.003).collect();

            let mut scalar = vec![0.0f32; n];
            let mut simd = vec![0.0f32; n];

            fma_zig(&a, &b, &c, &mut scalar);
            fma_zig_simd(&a, &b, &c, &mut simd);

            for i in 0..n {
                let reference = a[i] * b[i] + c[i];

                assert!(
                    (scalar[i] - reference).abs() <= 1.0e-6,
                    "scalar FMA mismatch at {i}: got {}, expected {}",
                    scalar[i],
                    reference
                );

                assert!(
                    (simd[i] - reference).abs() <= 1.0e-6,
                    "SIMD FMA mismatch at {i}: got {}, expected {}",
                    simd[i],
                    reference
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = "input lengths must match")]
    fn mismatched_inputs_are_rejected() {
        let a = [1.0f32, 2.0];
        let b = [3.0f32];
        let mut out = [0.0f32, 0.0];

        vector_add_zig_simd(&a, &b, &mut out);
    }
}
