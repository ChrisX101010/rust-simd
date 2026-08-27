use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn dot(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    a.iter().zip(b).map(|(&x, &y)| x * y).sum()
}

fn main() {
    let a: Vec<f32> = (0..N).map(|i| ((i % 997) as f32) * 0.001).collect();

    let b: Vec<f32> = (0..N).map(|i| ((i % 991) as f32) * 0.002).collect();

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
    println!("implementation=automatic");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("result={result:.6}");
}
