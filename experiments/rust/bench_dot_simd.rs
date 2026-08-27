#![feature(portable_simd)]

use std::hint::black_box;
use std::simd::{f32x8, num::SimdFloat};
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn dot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    let mut i = 0;

    let mut acc0 = f32x8::splat(0.0);
    let mut acc1 = f32x8::splat(0.0);
    let mut acc2 = f32x8::splat(0.0);
    let mut acc3 = f32x8::splat(0.0);

    while i + 32 <= a.len() {
        let a0 = f32x8::from_slice(&a[i..]);
        let b0 = f32x8::from_slice(&b[i..]);

        let a1 = f32x8::from_slice(&a[i + 8..]);
        let b1 = f32x8::from_slice(&b[i + 8..]);

        let a2 = f32x8::from_slice(&a[i + 16..]);
        let b2 = f32x8::from_slice(&b[i + 16..]);

        let a3 = f32x8::from_slice(&a[i + 24..]);
        let b3 = f32x8::from_slice(&b[i + 24..]);

        acc0 += a0 * b0;
        acc1 += a1 * b1;
        acc2 += a2 * b2;
        acc3 += a3 * b3;

        i += 32;
    }

    let mut total = (acc0 + acc1 + acc2 + acc3).reduce_sum();

    while i < a.len() {
        total += a[i] * b[i];
        i += 1;
    }

    total
}

fn main() {
    let a: Vec<f32> = (0..N)
        .map(|i| ((i % 997) as f32) * 0.001)
        .collect();

    let b: Vec<f32> = (0..N)
        .map(|i| ((i % 991) as f32) * 0.002)
        .collect();

    let mut result = 0.0f32;

    for _ in 0..WARMUP {
        result = dot(black_box(&a), black_box(&b));
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        result = dot(black_box(&a), black_box(&b));

        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    println!("kernel=dot");
    println!("language=rust");
    println!("implementation=explicit_simd_4acc");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
