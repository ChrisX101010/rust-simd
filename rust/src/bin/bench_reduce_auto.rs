use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn reduce_sum(data: &[f32]) -> f32 {
    data.iter().copied().sum()
}

fn main() {
    let data: Vec<f32> = (0..N).map(|i| ((i % 997) as f32) * 0.001).collect();

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
    println!("implementation=automatic");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
