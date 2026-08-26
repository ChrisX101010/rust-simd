#![feature(portable_simd)]

use std::simd::f32x8;

const N: usize = 16 * 1024 * 1024;

fn vector_add_simd(a: &[f32], b: &[f32], out: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());

    let mut i = 0;

    while i + 8 <= a.len() {
        let av = f32x8::from_slice(&a[i..]);
        let bv = f32x8::from_slice(&b[i..]);

        let cv = av + bv;

        cv.copy_to_slice(&mut out[i..]);

        i += 8;
    }

    while i < a.len() {
        out[i] = a[i] + b[i];
        i += 1;
    }
}

fn main() {
    let a: Vec<f32> = (0..N)
        .map(|i| (i % 1000) as f32)
        .collect();

    let b: Vec<f32> = (0..N)
        .map(|i| ((i * 3) % 1000) as f32)
        .collect();

    let mut out = vec![0.0; N];

    vector_add_simd(&a, &b, &mut out);

    let checksum: f64 = out.iter().map(|&x| x as f64).sum();

    println!("elements: {N}");
    println!("checksum: {checksum:.3}");
}
