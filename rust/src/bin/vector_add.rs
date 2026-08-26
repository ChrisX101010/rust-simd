use std::time::Instant;

const N: usize = 16 * 1024 * 1024;
const WARMUP: usize = 5;
const RUNS: usize = 20;

fn vector_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    for i in 0..a.len() {
        out[i] = a[i] + b[i];
    }
}

fn main() {
    let a: Vec<f32> = (0..N).map(|i| (i % 1000) as f32).collect();
    let b: Vec<f32> = (0..N).map(|i| ((i * 3) % 1000) as f32).collect();
    let mut out = vec![0.0f32; N];

    for _ in 0..WARMUP {
        vector_add(&a, &b, &mut out);
    }

    let start = Instant::now();

    for _ in 0..RUNS {
        vector_add(&a, &b, &mut out);
    }

    let elapsed = start.elapsed();

    let checksum: f64 = out.iter().map(|&x| x as f64).sum();

    println!("elements:   {N}");
    println!("runs:       {RUNS}");
    println!("total:      {:?}", elapsed);
    println!("per run:    {:?}", elapsed / RUNS as u32);
    println!("checksum:   {checksum:.3}");
}
