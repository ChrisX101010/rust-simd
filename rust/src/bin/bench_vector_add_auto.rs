use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
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
        vector_add(&a, &b, &mut out);
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        vector_add(
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
    println!("implementation=automatic");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("total_ns={total}");
    println!("min_ns={min}");
    println!("median_ns={median}");
    println!("checksum={checksum:.3}");
}
