use rust_simd::{BackendKind, Capabilities, Engine};

fn close(actual: f32, expected: f32) {
    let tolerance = expected.abs() * 1.0e-4 + 1.0e-3;

    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual} expected={expected} tolerance={tolerance}"
    );
}

fn main() {
    let capabilities = Capabilities::detect();
    let engine = Engine::auto();

    assert!(
        capabilities.has_wasm_simd128(),
        "SIMD128 must be enabled for this smoke test"
    );

    assert_eq!(
        engine.backend(),
        BackendKind::WasmSimd128,
        "automatic dispatch did not select WASM SIMD128"
    );

    let lengths = [
        0_usize, 1, 2, 3, 4, 5, 7, 8, 15, 16, 17, 31, 32, 33,
        127, 128, 129, 1023, 1024, 1025,
    ];

    for length in lengths {
        let a: Vec<f32> = (0..length)
            .map(|i| ((i * 17 + 3) % 101) as f32 * 0.125 - 5.0)
            .collect();

        let b: Vec<f32> = (0..length)
            .map(|i| ((i * 29 + 7) % 97) as f32 * 0.0625 - 3.0)
            .collect();

        let c: Vec<f32> = (0..length)
            .map(|i| ((i * 11 + 5) % 89) as f32 * 0.03125 - 1.0)
            .collect();

        let mut out = vec![0.0_f32; length];

        engine.vector_add(&a, &b, &mut out);

        for i in 0..length {
            assert_eq!(out[i], a[i] + b[i]);
        }

        /*
         * WASM SIMD128 has no strict fused f32 multiply-add
         * matching the public mul_add contract, so rust-simd
         * deliberately uses the scalar FMA path here.
         */
        engine.fma(&a, &b, &c, &mut out);

        for i in 0..length {
            assert_eq!(out[i], a[i].mul_add(b[i], c[i]));
        }

        let sum_reference =
            a.iter().map(|&x| x as f64).sum::<f64>() as f32;

        close(
            engine.reduce_sum(&a),
            sum_reference,
        );

        let dot_reference = a
            .iter()
            .zip(&b)
            .map(|(&x, &y)| (x as f64) * (y as f64))
            .sum::<f64>() as f32;

        close(
            engine.dot(&a, &b),
            dot_reference,
        );
    }

    println!(
        "WASM SIMD128 runtime: PASS backend={}",
        engine.backend_name()
    );
}
