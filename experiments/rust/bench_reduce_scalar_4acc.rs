use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn reduce_sum(data: &[f32]) -> f32 {
    let mut i = 0;

    let mut acc0 = 0.0f32;
    let mut acc1 = 0.0f32;
    let mut acc2 = 0.0f32;
    let mut acc3 = 0.0f32;

    while i + 4 <= data.len() {
        acc0 += data[i];
        acc1 += data[i + 1];
        acc2 += data[i + 2];
        acc3 += data[i + 3];

        i += 4;
    }

    let mut total = acc0 + acc1 + acc2 + acc3;

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
    println!("implementation=scalar_4acc");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
