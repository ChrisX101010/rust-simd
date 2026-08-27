use std::arch::x86_64::{
    _mm256_add_ps,
    _mm256_fmadd_ps,
    _mm256_loadu_ps,
    _mm256_mul_ps,
    _mm256_storeu_ps,
};
use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[target_feature(enable = "avx2,fma")]
unsafe fn fma_kernel(
    a: &[f32],
    b: &[f32],
    c: &[f32],
    out: &mut [f32],
) {
    let mut i = 0;

    while i + 8 <= a.len() {
        let av = _mm256_loadu_ps(a.as_ptr().add(i));
        let bv = _mm256_loadu_ps(b.as_ptr().add(i));
        let cv = _mm256_loadu_ps(c.as_ptr().add(i));

        let result = _mm256_fmadd_ps(av, bv, cv);

        _mm256_storeu_ps(out.as_mut_ptr().add(i), result);
        i += 8;
    }

    while i < a.len() {
        out[i] = a[i] * b[i] + c[i];
        i += 1;
    }
}

fn checksum(data: &[f32]) -> f64 {
    data.iter().map(|&x| x as f64).sum()
}

fn main() {
    let a: Vec<f32> = (0..N).map(|i| (i % 997) as f32 * 0.001).collect();
    let b: Vec<f32> = (0..N).map(|i| (i % 991) as f32 * 0.002).collect();
    let c: Vec<f32> = (0..N).map(|i| (i % 983) as f32 * 0.003).collect();

    let mut out = vec![0.0f32; N];

    for _ in 0..WARMUP {
        unsafe {
            fma_kernel(&a, &b, &c, &mut out);
        }
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        unsafe {
            fma_kernel(
                black_box(&a),
                black_box(&b),
                black_box(&c),
                black_box(&mut out),
            );
        }

        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    println!("kernel=fma");
    println!("language=rust");
    println!("implementation=x86_avx2_fma_intrinsic");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("checksum={:.6}", checksum(&out));

    // Keep these imports from being accidentally optimized out of
    // the source-level experiment description.
    let _ = (_mm256_mul_ps, _mm256_add_ps);
}
