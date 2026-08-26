#![feature(portable_simd)]

use std::hint::black_box;
use std::simd::f32x8;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn vector_add_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    let mut i = 0;

    while i + 8 <= a.len() {
        let av = f32x8::from_slice(&a[i..]);
        let bv = f32x8::from_slice(&b[i..]);

        (av + bv).copy_to_slice(&mut out[i..]);

        i += 8;
    }

    while i < a.len() {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

fn checksum(data: &[f32]) -> f64 {
    data.iter().map(|&x| x as f64).sum()
}

fn main() {
    let a: Vec<f32> = (0..N)
        .map(|i| (i % 1000) as f32)
        .collect();

    let b: Vec<f32> = (0..N)
        .map(|i| ((i * 3) % 1000) as f32)
        .collect();

    let mut out = vec![0.0f32; N];

    for _ in 0..WARMUP {
        vector_add_simd(&a, &b, &mut out);
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        vector_add_simd(
            black_box(&a),
            black_box(&b),
            black_box(&mut out),
        );

        timings.push(start.elapsed().as_nanos());
    }

    let checksum = checksum(&out);

    let total: u128 = timings.iter().sum();
    let min = *timings.iter().min().unwrap();

    let mut sorted = timings;
    sorted.sort_unstable();

    let median = sorted[sorted.len() / 2];

    println!("kernel=vector_add");
    println!("language=rust");
    println!("implementation=explicit_simd");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("total_ns={total}");
    println!("min_ns={min}");
    println!("median_ns={median}");
    println!("checksum={checksum:.3}");
}
