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

fn zig_scalar(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    unsafe {
        vector_add_f32(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

fn zig_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    unsafe {
        vector_add_f32_simd(
            a.as_ptr(),
            b.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        );
    }
}

fn expected(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter()
        .zip(b)
        .map(|(&x, &y)| x + y)
        .collect()
}

fn run_case(n: usize) {
    let a: Vec<f32> = (0..n)
        .map(|i| (i % 1000) as f32)
        .collect();

    let b: Vec<f32> = (0..n)
        .map(|i| ((i * 3) % 1000) as f32)
        .collect();

    let expected = expected(&a, &b);

    let mut scalar = vec![0.0f32; n];
    let mut simd = vec![0.0f32; n];

    zig_scalar(&a, &b, &mut scalar);
    zig_simd(&a, &b, &mut simd);

    assert_eq!(scalar, expected, "scalar FFI mismatch for length {n}");
    assert_eq!(simd, expected, "SIMD FFI mismatch for length {n}");
    assert_eq!(scalar, simd, "scalar/SIMD disagreement for length {n}");
}

#[test]
fn ffi_handles_simd_boundaries() {
    for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33] {
        run_case(n);
    }
}

#[test]
fn ffi_handles_large_input() {
    run_case(100_003);
}

#[test]
#[should_panic]
fn safe_wrapper_rejects_mismatched_lengths() {
    let a = [1.0f32, 2.0];
    let b = [3.0f32];
    let mut out = [0.0f32, 0.0];

    zig_simd(&a, &b, &mut out);
}
