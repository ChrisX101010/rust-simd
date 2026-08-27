unsafe extern "C" {
    fn vector_add_f32(
        a: *const f32,
        b: *const f32,
        out: *mut f32,
        len: usize,
    );

    fn vector_add_f32_simd(
        a: *const f32,
        b: *const f32,
        out: *mut f32,
        len: usize,
    );
}

/// Calls the Zig scalar implementation through the C-compatible ABI.
pub fn vector_add_zig(
    a: &[f32],
    b: &[f32],
    out: &mut [f32],
) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        vector_add_f32(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

/// Calls the Zig explicit-SIMD implementation through the C-compatible ABI.
///
/// The Rust-facing API is safe because the wrapper validates all slice
/// length invariants before crossing the unsafe FFI boundary.
pub fn vector_add_zig_simd(
    a: &[f32],
    b: &[f32],
    out: &mut [f32],
) {
    assert_eq!(a.len(), b.len(), "input lengths must match");
    assert_eq!(a.len(), out.len(), "output length must match");

    unsafe {
        vector_add_f32_simd(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter()
            .zip(b)
            .map(|(&x, &y)| x + y)
            .collect()
    }

    fn check_case(n: usize) {
        let a: Vec<f32> = (0..n)
            .map(|i| (i % 1000) as f32)
            .collect();

        let b: Vec<f32> = (0..n)
            .map(|i| ((i * 3) % 1000) as f32)
            .collect();

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
    #[should_panic(expected = "input lengths must match")]
    fn mismatched_inputs_are_rejected() {
        let a = [1.0f32, 2.0];
        let b = [3.0f32];
        let mut out = [0.0f32, 0.0];

        vector_add_zig_simd(&a, &b, &mut out);
    }
}
