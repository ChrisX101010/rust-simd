use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_simd::{BackendKind, Engine};

unsafe extern "C" {
    fn vector_add_f32_simd(a: *const f32, b: *const f32, out: *mut f32, len: usize);

    fn fma_f32_muladd_simd(a: *const f32, b: *const f32, c: *const f32, out: *mut f32, len: usize);

    fn reduce_sum_f32_simd_4acc(data: *const f32, len: usize) -> f32;

    fn dot_f32_simd(a: *const f32, b: *const f32, len: usize) -> f32;
}

const SIZES: &[usize] = &[1, 8, 32, 256, 4096, 65536, 1048576, 16777216];

const WARMUPS: usize = 10;
const SAMPLES: usize = 31;

#[derive(Debug, Clone, Copy)]
struct Stats {
    min: f64,
    median: f64,
    p95: f64,
    mean: f64,
}

impl Stats {
    fn from_samples(mut samples: Vec<f64>) -> Self {
        assert!(!samples.is_empty());

        samples.sort_by(f64::total_cmp);

        let min = samples[0];
        let median = percentile(&samples, 0.50);
        let p95 = percentile(&samples, 0.95);
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;

        Self {
            min,
            median,
            p95,
            mean,
        }
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let last = sorted.len() - 1;
    let index = p * last as f64;

    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        return sorted[lower];
    }

    let weight = index - lower as f64;

    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}

fn make_data(n: usize, seed: u32) -> Vec<f32> {
    (0..n)
        .map(|i| {
            let value = ((i as u32).wrapping_mul(1_664_525).wrapping_add(seed)) % 10_000;

            value as f32 * 0.001
        })
        .collect()
}

fn sample<F>(mut operation: F) -> Stats
where
    F: FnMut(),
{
    for _ in 0..WARMUPS {
        operation();
    }

    let mut samples = Vec::with_capacity(SAMPLES);

    for _ in 0..SAMPLES {
        let start = Instant::now();
        operation();
        let elapsed = start.elapsed();

        samples.push(elapsed.as_secs_f64() * 1e9);
    }

    Stats::from_samples(samples)
}

fn throughput_gbps(bytes: usize, ns: f64) -> f64 {
    if ns <= 0.0 {
        return 0.0;
    }

    bytes as f64 / ns
}

fn reduction_tolerance(reference: f64) -> f64 {
    reference.abs() * 1.0e-4 + 1.0e-2
}

fn assert_close(name: &str, value: f32, reference: f64) {
    let error = (value as f64 - reference).abs();
    let tolerance = reduction_tolerance(reference);

    assert!(
        error <= tolerance,
        "{name}: value={value}, reference={reference}, error={error}, tolerance={tolerance}"
    );
}

fn available_engines() -> Vec<Engine> {
    let mut engines = vec![Engine::scalar()];

    if let Ok(engine) = Engine::avx2() {
        engines.push(engine);
    }

    if let Ok(engine) = Engine::avx2_fma() {
        engines.push(engine);
    }

    engines
}

fn validate_vector_add(engines: &[Engine], a: &[f32], b: &[f32]) {
    let n = a.len();

    let expected: Vec<f32> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();

    for engine in engines {
        let mut out = vec![0.0f32; n];

        engine.vector_add(a, b, &mut out);

        assert_eq!(
            out,
            expected,
            "vector_add mismatch for {}",
            engine.backend_name()
        );
    }

    let mut zig = vec![0.0f32; n];

    // SAFETY:
    // `a`, `b`, and `zig` are live slices with exactly `n` elements.
    unsafe {
        vector_add_f32_simd(a.as_ptr(), b.as_ptr(), zig.as_mut_ptr(), n);
    }

    assert_eq!(zig, expected);
}

fn validate_fma(engines: &[Engine], a: &[f32], b: &[f32], c: &[f32]) {
    let n = a.len();

    for engine in engines {
        let mut out = vec![0.0f32; n];

        engine.fma(a, b, c, &mut out);

        for i in 0..n {
            let reference = (a[i] as f64) * (b[i] as f64) + c[i] as f64;

            assert_close(engine.backend_name(), out[i], reference);
        }
    }

    let mut zig = vec![0.0f32; n];

    // SAFETY:
    // All input pointers refer to valid buffers containing `n` elements.
    unsafe {
        fma_f32_muladd_simd(a.as_ptr(), b.as_ptr(), c.as_ptr(), zig.as_mut_ptr(), n);
    }

    for i in 0..n {
        let reference = (a[i] as f64) * (b[i] as f64) + c[i] as f64;

        assert_close("zig", zig[i], reference);
    }
}

fn validate_reduce(engines: &[Engine], data: &[f32]) {
    let reference: f64 = data.iter().map(|&x| x as f64).sum();

    for engine in engines {
        assert_close(engine.backend_name(), engine.reduce_sum(data), reference);
    }

    // SAFETY:
    // `data` contains exactly `data.len()` live f32 elements.
    let zig = unsafe { reduce_sum_f32_simd_4acc(data.as_ptr(), data.len()) };

    assert_close("zig", zig, reference);
}

fn validate_dot(engines: &[Engine], a: &[f32], b: &[f32]) {
    let reference: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x as f64) * (y as f64))
        .sum();

    for engine in engines {
        assert_close(engine.backend_name(), engine.dot(a, b), reference);
    }

    // SAFETY:
    // Both pointers refer to live buffers with exactly `a.len()` elements.
    let zig = unsafe { dot_f32_simd(a.as_ptr(), b.as_ptr(), a.len()) };

    assert_close("zig", zig, reference);
}

fn bench_vector_add(engine: Engine, a: &[f32], b: &[f32]) -> Stats {
    let mut out = vec![0.0f32; a.len()];

    sample(|| {
        engine.vector_add(black_box(a), black_box(b), black_box(&mut out));

        black_box(&out);
    })
}

fn bench_vector_add_auto(a: &[f32], b: &[f32]) -> Stats {
    let engine = Engine::auto();
    bench_vector_add(engine, a, b)
}

fn bench_vector_add_zig(a: &[f32], b: &[f32]) -> Stats {
    let mut out = vec![0.0f32; a.len()];

    sample(|| {
        // SAFETY:
        // The buffers are valid for exactly `a.len()` elements.
        unsafe {
            vector_add_f32_simd(
                black_box(a.as_ptr()),
                black_box(b.as_ptr()),
                black_box(out.as_mut_ptr()),
                a.len(),
            );
        }

        black_box(&out);
    })
}

fn bench_fma(engine: Engine, a: &[f32], b: &[f32], c: &[f32]) -> Stats {
    let mut out = vec![0.0f32; a.len()];

    sample(|| {
        engine.fma(
            black_box(a),
            black_box(b),
            black_box(c),
            black_box(&mut out),
        );

        black_box(&out);
    })
}

fn bench_fma_auto(a: &[f32], b: &[f32], c: &[f32]) -> Stats {
    bench_fma(Engine::auto(), a, b, c)
}

fn bench_fma_zig(a: &[f32], b: &[f32], c: &[f32]) -> Stats {
    let mut out = vec![0.0f32; a.len()];

    sample(|| {
        // SAFETY:
        // All buffers contain exactly `a.len()` f32 elements.
        unsafe {
            fma_f32_muladd_simd(
                black_box(a.as_ptr()),
                black_box(b.as_ptr()),
                black_box(c.as_ptr()),
                black_box(out.as_mut_ptr()),
                a.len(),
            );
        }

        black_box(&out);
    })
}

fn bench_reduce(engine: Engine, data: &[f32]) -> Stats {
    sample(|| {
        black_box(engine.reduce_sum(black_box(data)));
    })
}

fn bench_reduce_auto(data: &[f32]) -> Stats {
    bench_reduce(Engine::auto(), data)
}

fn bench_reduce_zig(data: &[f32]) -> Stats {
    sample(|| {
        // SAFETY:
        // `data` contains exactly `data.len()` f32 elements.
        let result = unsafe { reduce_sum_f32_simd_4acc(black_box(data.as_ptr()), data.len()) };

        black_box(result);
    })
}

fn bench_dot(engine: Engine, a: &[f32], b: &[f32]) -> Stats {
    sample(|| {
        black_box(engine.dot(black_box(a), black_box(b)));
    })
}

fn bench_dot_auto(a: &[f32], b: &[f32]) -> Stats {
    bench_dot(Engine::auto(), a, b)
}

fn bench_dot_zig(a: &[f32], b: &[f32]) -> Stats {
    sample(|| {
        // SAFETY:
        // Both pointers refer to buffers with exactly `a.len()` elements.
        let result = unsafe { dot_f32_simd(black_box(a.as_ptr()), black_box(b.as_ptr()), a.len()) };

        black_box(result);
    })
}

fn print_result(kernel: &str, n: usize, backend: &str, stats: Stats, bytes: usize) {
    let throughput = throughput_gbps(bytes, stats.median);

    println!(
        "{kernel:12} n={n:>9} backend={backend:>10} \
         median={:>11.3} ns p95={:>11.3} ns min={:>11.3} ns \
         mean={:>11.3} ns throughput={:>8.3} GB/s",
        stats.median, stats.p95, stats.min, stats.mean, throughput,
    );
}

fn print_csv_header() {
    println!("kernel,size,backend,min_ns,median_ns,p95_ns,mean_ns,throughput_gb_s");
}

fn print_csv(kernel: &str, n: usize, backend: &str, stats: Stats, bytes: usize) {
    println!(
        "CSV,{kernel},{n},{backend},{:.3},{:.3},{:.3},{:.3},{:.6}",
        stats.min,
        stats.median,
        stats.p95,
        stats.mean,
        throughput_gbps(bytes, stats.median),
    );
}

fn main() {
    let engines = available_engines();
    let auto = Engine::auto();

    println!("rust-simd controlled benchmark");
    println!("automatic backend: {}", auto.backend_name());
    println!("warmups: {WARMUPS}");
    println!("samples: {SAMPLES}");
    println!("rustc: {}", env!("RUSTC_VERSION"));

    println!();
    println!("available backends:");

    for engine in &engines {
        println!("  {}", engine.backend_name());
    }

    println!();
    println!("human-readable results");
    println!();

    for &n in SIZES {
        let a = make_data(n, 0x1234);
        let b = make_data(n, 0x5678);

        validate_vector_add(&engines, &a, &b);

        let bytes = n
            .saturating_mul(std::mem::size_of::<f32>())
            .saturating_mul(3);

        for engine in &engines {
            let stats = bench_vector_add(*engine, &a, &b);

            print_result("vector_add", n, engine.backend_name(), stats, bytes);
        }

        let stats = bench_vector_add_auto(&a, &b);

        print_result("vector_add", n, "auto", stats, bytes);

        let zig = bench_vector_add_zig(&a, &b);

        print_result("vector_add", n, "zig-simd", zig, bytes);

        println!();
    }

    for &n in SIZES {
        let a = make_data(n, 0x1111);
        let b = make_data(n, 0x2222);
        let c = make_data(n, 0x3333);

        validate_fma(&engines, &a, &b, &c);

        let bytes = n
            .saturating_mul(std::mem::size_of::<f32>())
            .saturating_mul(4);

        for engine in &engines {
            let stats = bench_fma(*engine, &a, &b, &c);

            print_result("fma", n, engine.backend_name(), stats, bytes);
        }

        let stats = bench_fma_auto(&a, &b, &c);

        print_result("fma", n, "auto", stats, bytes);

        let zig = bench_fma_zig(&a, &b, &c);

        print_result("fma", n, "zig-simd", zig, bytes);

        println!();
    }

    for &n in SIZES {
        let data = make_data(n, 0x4444);

        validate_reduce(&engines, &data);

        let bytes = n.saturating_mul(std::mem::size_of::<f32>());

        for engine in &engines {
            let stats = bench_reduce(*engine, &data);

            print_result("reduce_sum", n, engine.backend_name(), stats, bytes);
        }

        let stats = bench_reduce_auto(&data);

        print_result("reduce_sum", n, "auto", stats, bytes);

        let zig = bench_reduce_zig(&data);

        print_result("reduce_sum", n, "zig-simd", zig, bytes);

        println!();
    }

    for &n in SIZES {
        let a = make_data(n, 0x5555);
        let b = make_data(n, 0x6666);

        validate_dot(&engines, &a, &b);

        let bytes = n
            .saturating_mul(std::mem::size_of::<f32>())
            .saturating_mul(2);

        for engine in &engines {
            let stats = bench_dot(*engine, &a, &b);

            print_result("dot", n, engine.backend_name(), stats, bytes);
        }

        let stats = bench_dot_auto(&a, &b);

        print_result("dot", n, "auto", stats, bytes);

        let zig = bench_dot_zig(&a, &b);

        print_result("dot", n, "zig-simd", zig, bytes);

        println!();
    }

    println!("CSV results");
    print_csv_header();

    // A compact machine-readable summary for the largest size.
    let n = *SIZES.last().expect("benchmark sizes must not be empty");

    let a = make_data(n, 0x1234);
    let b = make_data(n, 0x5678);
    let c = make_data(n, 0x3333);

    for engine in &engines {
        let stats = bench_vector_add(*engine, &a, &b);
        let bytes = n * std::mem::size_of::<f32>() * 3;
        print_csv("vector_add", n, engine.backend_name(), stats, bytes);
    }

    let auto_stats = bench_vector_add_auto(&a, &b);
    print_csv(
        "vector_add",
        n,
        "auto",
        auto_stats,
        n * std::mem::size_of::<f32>() * 3,
    );

    let zig_stats = bench_vector_add_zig(&a, &b);
    print_csv(
        "vector_add",
        n,
        "zig-simd",
        zig_stats,
        n * std::mem::size_of::<f32>() * 3,
    );

    for engine in &engines {
        let stats = bench_fma(*engine, &a, &b, &c);
        print_csv(
            "fma",
            n,
            engine.backend_name(),
            stats,
            n * std::mem::size_of::<f32>() * 4,
        );
    }

    let auto_stats = bench_fma_auto(&a, &b, &c);
    print_csv(
        "fma",
        n,
        "auto",
        auto_stats,
        n * std::mem::size_of::<f32>() * 4,
    );

    let zig_stats = bench_fma_zig(&a, &b, &c);
    print_csv(
        "fma",
        n,
        "zig-simd",
        zig_stats,
        n * std::mem::size_of::<f32>() * 4,
    );

    let data = make_data(n, 0x4444);

    for engine in &engines {
        let stats = bench_reduce(*engine, &data);
        print_csv(
            "reduce_sum",
            n,
            engine.backend_name(),
            stats,
            n * std::mem::size_of::<f32>(),
        );
    }

    let auto_stats = bench_reduce_auto(&data);
    print_csv(
        "reduce_sum",
        n,
        "auto",
        auto_stats,
        n * std::mem::size_of::<f32>(),
    );

    let zig_stats = bench_reduce_zig(&data);
    print_csv(
        "reduce_sum",
        n,
        "zig-simd",
        zig_stats,
        n * std::mem::size_of::<f32>(),
    );

    let a = make_data(n, 0x5555);
    let b = make_data(n, 0x6666);

    for engine in &engines {
        let stats = bench_dot(*engine, &a, &b);
        print_csv(
            "dot",
            n,
            engine.backend_name(),
            stats,
            n * std::mem::size_of::<f32>() * 2,
        );
    }

    let auto_stats = bench_dot_auto(&a, &b);
    print_csv(
        "dot",
        n,
        "auto",
        auto_stats,
        n * std::mem::size_of::<f32>() * 2,
    );

    let zig_stats = bench_dot_zig(&a, &b);
    print_csv(
        "dot",
        n,
        "zig-simd",
        zig_stats,
        n * std::mem::size_of::<f32>() * 2,
    );

    println!();
    println!("all differential correctness checks passed");
}
