use rust_simd::{backend_name, dot, fma, reduce_sum, vector_add};

fn main() {
    let a = [1.0f32, 2.0, 3.0, 4.0];
    let b = [5.0f32, 6.0, 7.0, 8.0];

    let mut added = [0.0f32; 4];
    vector_add(&a, &b, &mut added);

    assert_eq!(added, [6.0, 8.0, 10.0, 12.0]);

    let mut fused = [0.0f32; 4];
    fma(&a, &b, &[1.0; 4], &mut fused);

    assert_eq!(fused, [6.0, 13.0, 22.0, 33.0]);

    let sum = reduce_sum(&a);
    assert!((sum - 10.0).abs() <= 1.0e-6);

    let product = dot(&a, &b);
    assert!((product - 70.0).abs() <= 1.0e-6);

    println!("public API consumer: PASS");
    println!("selected backend: {}", backend_name());
}
