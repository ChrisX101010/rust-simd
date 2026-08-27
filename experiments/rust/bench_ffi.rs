
use std::hint::black_box;
use std::time::Instant;

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

fn rust_vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
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

fn run<F>(mut f: F, a: &[f32], b: &[f32], out: &mut [f32], runs: usize) -> Vec<u128>
where
    F: FnMut(&[f32], &[f32], &mut [f32]),
{
    for _ in 0..10 {
        f(a, b, out);
    }

    let mut timings = Vec::with_capacity(runs);

    for _ in 0..runs {
        let start = Instant::now();

        f(
            black_box(a),
            black_box(b),
            black_box(out),
        );

        timings.push(start.elapsed().as_nanos());
    }

    timings
}

fn main() {
    let sizes = [
        1usize,
        8,
        64,
        1_024,
        65_536,
        1_048_576,
        16_777_216,
    ];

    const RUNS: usize = 50;

    println!("kernel,implementation,elements,runs,min_ns,median_ns,checksum");

    for n in sizes {
        let a: Vec<f32> = (0..n)
            .map(|i| (i % 1000) as f32)
            .collect();

        let b: Vec<f32> = (0..n)
            .map(|i| ((i * 3) % 1000) as f32)
            .collect();

        let mut out = vec![0.0f32; n];

        let cases: [(&str, fn(&[f32], &[f32], &mut [f32])); 3] = [
            ("rust_direct", rust_vector_add),
            ("zig_scalar_ffi", zig_scalar),
            ("zig_simd_ffi", zig_simd),
        ];

        for (name, function) in cases {
            out.fill(0.0);

            let timings = run(
                function,
                &a,
                &b,
                &mut out,
                RUNS,
            );

            let min = *timings.iter().min().unwrap();

            let mut sorted = timings;
            sorted.sort_unstable();

            let median = sorted[sorted.len() / 2];

            let checksum: f64 =
                out.iter().map(|&x| x as f64).sum();

            println!(
                "vector_add,{name},{n},{RUNS},{min},{median},{checksum:.3}"
            );
        }
    }
}
