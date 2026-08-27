use std::hint::black_box;
use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 30;

#[inline(never)]
fn fma_kernel(a: &[f32], b: &[f32], c: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());
    assert_eq!(a.len(), out.len());

    for i in 0..a.len() {
        out[i] = a[i] * b[i] + c[i];
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
        fma_kernel(&a, &b, &c, &mut out);
    }

    let mut timings = Vec::with_capacity(RUNS);

    for _ in 0..RUNS {
        let start = Instant::now();

        fma_kernel(
            black_box(&a),
            black_box(&b),
            black_box(&c),
            black_box(&mut out),
        );

        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    println!("kernel=fma");
    println!("language=rust");
    println!("implementation=automatic");
    println!("elements={N}");
    println!("warmup={WARMUP}");
    println!("runs={RUNS}");
    println!("min_ns={}", timings[0]);
    println!("median_ns={}", timings[RUNS / 2]);
    println!("checksum={:.6}", checksum(&out));
}
