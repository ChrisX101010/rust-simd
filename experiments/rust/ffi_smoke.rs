
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

fn vector_add_zig_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
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

fn main() {
    let a = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = [10.0f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];

    let mut out_scalar = [0.0f32; 8];
    let mut out_simd = [0.0f32; 8];

    unsafe {
        vector_add_f32(
            a.as_ptr(),
            b.as_ptr(),
            out_scalar.as_mut_ptr(),
            a.len(),
        );
    }

    vector_add_zig_simd(&a, &b, &mut out_simd);

    let expected = [
        11.0f32, 22.0, 33.0, 44.0,
        55.0, 66.0, 77.0, 88.0,
    ];

    assert_eq!(out_scalar, expected);
    assert_eq!(out_simd, expected);

    // Keep the import above explicit while we develop the wrapper.

    println!("Rust ↔ Zig FFI smoke test: PASS");
}
