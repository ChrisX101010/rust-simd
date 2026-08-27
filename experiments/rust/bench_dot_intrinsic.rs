use std::arch::x86_64::{
    _mm256_fmadd_ps,
    _mm256_loadu_ps,
    _mm256_storeu_ps,
};
use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[target_feature(enable = "avx2,fma")]
unsafe fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = _mm256_setzero_ps();
    let mut acc1 = _mm256_setzero_ps();
    let mut acc2 = _mm256_setzero_ps();
    let mut acc3 = _mm256_setzero_ps();

    while i + 32 <= a.len() {
        acc0 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a.as_ptr().add(i)),
            _mm256_loadu_ps(b.as_ptr().add(i)),
            acc0,
        );

        acc1 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a.as_ptr().add(i + 8)),
            _mm256_loadu_ps(b.as_ptr().add(i + 8)),
            acc1,
        );

        acc2 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a.as_ptr().add(i + 16)),
            _mm256_loadu_ps(b.as_ptr().add(i + 16)),
            acc2,
        );

        acc3 = _mm256_fmadd_ps(
            _mm256_loadu_ps(a.as_ptr().add(i + 24)),
            _mm256_loadu_ps(b.as_ptr().add(i + 24)),
            acc3,
        );

        i += 32;
    }

    let mut lanes = [0.0f32; 8];

    _mm256_storeu_ps(lanes.as_mut_ptr(), acc0);
    let mut total: f32 = lanes.iter().sum();

    _mm256_storeu_ps(lanes.as_mut_ptr(), acc1);
    total += lanes.iter().sum::<f32>();

    _mm256_storeu_ps(lanes.as_mut_ptr(), acc2);
    total += lanes.iter().sum::<f32>();

    _mm256_storeu_ps(lanes.as_mut_ptr(), acc3);
    total += lanes.iter().sum::<f32>();

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}

use std::arch::x86_64::_mm256_setzero_ps;

fn main() {
    let a: Vec<f32> = (0..N)
        .map(|i| ((i % 997) as f32) * 0.001)
        .collect();

    let b: Vec<f32> = (0..N)
        .map(|i| ((i % 991) as f32) * 0.002)
        .collect();

    let mut result = 0.0f32;

    for _ in 0..WARMUP {
        result = unsafe { dot(black_box(&a), black_box(&b)) };
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        result = unsafe { dot(black_box(&a), black_box(&b)) };

        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    println!("kernel=dot");
    println!("language=rust");
    println!("implementation=x86_avx2_fma_4acc");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
