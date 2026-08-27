#![feature(portable_simd)]

use std::hint::black_box;
use std::simd::{f32x8, num::SimdFloat};
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn reduce_sum(data: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = f32x8::splat(0.0);
    let mut acc1 = f32x8::splat(0.0);
    let mut acc2 = f32x8::splat(0.0);
    let mut acc3 = f32x8::splat(0.0);

    while i + 32 <= data.len() {
        acc0 += f32x8::from_slice(&data[i..]);
        acc1 += f32x8::from_slice(&data[i + 8..]);
        acc2 += f32x8::from_slice(&data[i + 16..]);
        acc3 += f32x8::from_slice(&data[i + 24..]);

        i += 32;
    }

    let acc = acc0 + acc1 + acc2 + acc3;
    let mut total = acc.reduce_sum();

    while i < data.len() {
        total += data[i];
        i += 1;
    }

    total
}

fn main() {
    let data: Vec<f32> = (0..N)
        .map(|i| ((i % 997) as f32) * 0.001)
        .collect();

    let mut result = 0.0f32;

    for _ in 0..WARMUP {
        result = reduce_sum(black_box(&data));
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        result = reduce_sum(black_box(&data));

        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    println!("kernel=reduce_sum");
    println!("language=rust");
    println!("implementation=explicit_simd_4acc");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
