use std::{fmt, process::ExitCode};

use rust_simd::Engine;

const LENGTHS: &[usize] = &[
    0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023,
    1024, 1025, 4095, 4096, 4097, 16_383, 16_384, 16_385,
];

#[derive(Debug, Clone)]
struct VerificationFailure {
    backend: &'static str,
    operation: &'static str,
    length: usize,
    index: Option<usize>,
    actual: f32,
    expected: f64,
    error: f64,
    tolerance: f64,
}

impl fmt::Display for VerificationFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SIMD verification: FAILED")?;
        writeln!(f)?;
        writeln!(f, "  backend            {}", self.backend)?;
        writeln!(f, "  operation          {}", self.operation)?;
        writeln!(f, "  length             {}", self.length)?;

        if let Some(index) = self.index {
            writeln!(f, "  index              {index}")?;
        }

        writeln!(f, "  actual             {}", self.actual)?;
        writeln!(f, "  expected           {}", self.expected)?;
        writeln!(f, "  absolute error     {}", self.error)?;
        writeln!(f, "  tolerance          {}", self.tolerance)?;

        Ok(())
    }
}

pub fn run(args: &[String]) -> ExitCode {
    if !args.is_empty() {
        eprintln!("error: verify currently takes no arguments");
        return ExitCode::from(2);
    }

    let engines = available_engines();

    println!("cargo-simd verify");
    println!();
    println!("available backends:");

    for engine in &engines {
        println!("  - {}", engine.backend_name());
    }

    println!();
    println!("running differential verification...");

    for &length in LENGTHS {
        if let Err(failure) = verify_length(length, &engines) {
            eprintln!();
            eprintln!("{failure}");
            return ExitCode::from(1);
        }
    }

    println!("running structured numerical cases...");

    if let Err(failure) = verify_numerical_cases(&engines) {
        eprintln!();
        eprintln!("{failure}");
        return ExitCode::from(1);
    }

    println!();
    println!("verification summary");
    println!("  lengths tested     {}", LENGTHS.len());
    println!("  backends tested    {}", engines.len());
    println!("  numerical cases    PASS");
    println!("  vector_add         PASS");
    println!("  fma                PASS");
    println!("  reduce_sum         PASS");
    println!("  dot                PASS");
    println!();
    println!("SIMD verification: PASS");

    ExitCode::SUCCESS
}

fn available_engines() -> Vec<Engine> {
    let mut engines = vec![Engine::scalar()];

    if let Ok(engine) = Engine::avx2() {
        engines.push(engine);
    }

    if let Ok(engine) = Engine::avx2_fma() {
        engines.push(engine);
    }

    if let Ok(engine) = Engine::neon() {
        engines.push(engine);
    }

    if let Ok(engine) = Engine::wasm_simd128() {
        engines.push(engine);
    }

    engines
}

fn verify_length(length: usize, engines: &[Engine]) -> Result<(), VerificationFailure> {
    let a = make_data(length, 0x1234_5678_9abc_def0);

    let b = make_data(length, 0xfedc_ba98_7654_3210);

    let c = make_data(length, 0x0f0f_f0f0_55aa_aa55);

    let sum_reference = a.iter().map(|&value| value as f64).sum::<f64>();

    let dot_reference = a
        .iter()
        .zip(&b)
        .map(|(&left, &right)| (left as f64) * (right as f64))
        .sum::<f64>();

    for engine in engines {
        verify_vector_add(*engine, &a, &b)?;

        verify_fma(*engine, &a, &b, &c)?;

        check_close(
            engine.backend_name(),
            "reduce_sum",
            length,
            None,
            engine.reduce_sum(&a),
            sum_reference,
        )?;

        check_close(
            engine.backend_name(),
            "dot",
            length,
            None,
            engine.dot(&a, &b),
            dot_reference,
        )?;
    }

    Ok(())
}

fn verify_vector_add(engine: Engine, a: &[f32], b: &[f32]) -> Result<(), VerificationFailure> {
    let mut output = vec![0.0_f32; a.len()];

    engine.vector_add(a, b, &mut output);

    for index in 0..a.len() {
        let expected = a[index] + b[index];

        if !same_float(output[index], expected) {
            return Err(VerificationFailure {
                backend: engine.backend_name(),
                operation: "vector_add",
                length: a.len(),
                index: Some(index),
                actual: output[index],
                expected: expected as f64,
                error: (output[index] as f64 - expected as f64).abs(),
                tolerance: 0.0,
            });
        }
    }

    Ok(())
}

fn verify_fma(engine: Engine, a: &[f32], b: &[f32], c: &[f32]) -> Result<(), VerificationFailure> {
    let mut output = vec![0.0_f32; a.len()];

    engine.fma(a, b, c, &mut output);

    for index in 0..a.len() {
        let reference = (a[index] as f64) * (b[index] as f64) + c[index] as f64;

        check_close(
            engine.backend_name(),
            "fma",
            a.len(),
            Some(index),
            output[index],
            reference,
        )?;
    }

    Ok(())
}

fn verify_numerical_cases(engines: &[Engine]) -> Result<(), VerificationFailure> {
    verify_finite_case(engines, "zeros", &[0.0; 33], &[0.0; 33], &[0.0; 33])?;

    verify_finite_case(engines, "ones", &[1.0; 33], &[1.0; 33], &[1.0; 33])?;

    let alternating_a: Vec<f32> = (0..65)
        .map(|index| if index % 2 == 0 { 1000.0 } else { -1000.0 })
        .collect();

    let alternating_b: Vec<f32> = (0..65)
        .map(|index| if index % 3 == 0 { -0.25 } else { 0.25 })
        .collect();

    let alternating_c = vec![0.125_f32; 65];

    verify_finite_case(
        engines,
        "alternating-sign",
        &alternating_a,
        &alternating_b,
        &alternating_c,
    )?;

    let small_a = vec![1.0e-20_f32; 65];
    let small_b = vec![2.0e-10_f32; 65];
    let small_c = vec![1.0e-20_f32; 65];

    verify_finite_case(engines, "small-finite", &small_a, &small_b, &small_c)?;

    let large_a = vec![1.0e10_f32; 65];
    let large_b = vec![1.0e-5_f32; 65];
    let large_c = vec![1.0_f32; 65];

    verify_finite_case(engines, "large-finite", &large_a, &large_b, &large_c)?;

    verify_non_finite_elementwise(engines)?;

    Ok(())
}

fn verify_finite_case(
    engines: &[Engine],
    _case_name: &'static str,
    a: &[f32],
    b: &[f32],
    c: &[f32],
) -> Result<(), VerificationFailure> {
    let sum_reference = a.iter().map(|&value| value as f64).sum::<f64>();

    let dot_reference = a
        .iter()
        .zip(b)
        .map(|(&left, &right)| (left as f64) * (right as f64))
        .sum::<f64>();

    for engine in engines {
        verify_vector_add(*engine, a, b)?;

        verify_fma(*engine, a, b, c)?;

        check_close(
            engine.backend_name(),
            "reduce_sum",
            a.len(),
            None,
            engine.reduce_sum(a),
            sum_reference,
        )?;

        check_close(
            engine.backend_name(),
            "dot",
            a.len(),
            None,
            engine.dot(a, b),
            dot_reference,
        )?;
    }

    Ok(())
}

fn verify_non_finite_elementwise(engines: &[Engine]) -> Result<(), VerificationFailure> {
    let a = [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        0.0,
        -0.0,
        1.0,
        -1.0,
    ];

    let b = [1.0, 1.0, 2.0, -0.0, 0.0, f32::INFINITY, f32::NEG_INFINITY];

    for engine in engines {
        let mut output = [0.0_f32; 7];

        engine.vector_add(&a, &b, &mut output);

        for index in 0..a.len() {
            let expected = a[index] + b[index];

            if !same_float(output[index], expected) {
                return Err(VerificationFailure {
                    backend: engine.backend_name(),
                    operation: "vector_add/non-finite",
                    length: a.len(),
                    index: Some(index),
                    actual: output[index],
                    expected: expected as f64,
                    error: f64::NAN,
                    tolerance: 0.0,
                });
            }
        }
    }

    Ok(())
}

fn check_close(
    backend: &'static str,
    operation: &'static str,
    length: usize,
    index: Option<usize>,
    value: f32,
    reference: f64,
) -> Result<(), VerificationFailure> {
    if reference.is_nan() {
        if value.is_nan() {
            return Ok(());
        }

        return Err(VerificationFailure {
            backend,
            operation,
            length,
            index,
            actual: value,
            expected: reference,
            error: f64::NAN,
            tolerance: 0.0,
        });
    }

    if reference.is_infinite() {
        if (value as f64) == reference {
            return Ok(());
        }

        return Err(VerificationFailure {
            backend,
            operation,
            length,
            index,
            actual: value,
            expected: reference,
            error: f64::INFINITY,
            tolerance: 0.0,
        });
    }

    let error = (value as f64 - reference).abs();

    let tolerance = reference.abs() * 1.0e-4 + 1.0e-2;

    if error <= tolerance {
        return Ok(());
    }

    Err(VerificationFailure {
        backend,
        operation,
        length,
        index,
        actual: value,
        expected: reference,
        error,
        tolerance,
    })
}

fn same_float(actual: f32, expected: f32) -> bool {
    if expected.is_nan() {
        actual.is_nan()
    } else {
        actual.to_bits() == expected.to_bits() || actual == expected
    }
}

fn make_data(length: usize, seed: u64) -> Vec<f32> {
    (0..length)
        .map(|index| pseudo_random_value(index, seed))
        .collect()
}

fn pseudo_random_value(index: usize, seed: u64) -> f32 {
    let mut value = seed
        .wrapping_add(index as u64)
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1);

    value ^= value >> 33;
    value ^= value << 17;
    value ^= value >> 29;

    let signed = (value % 20_001) as i32 - 10_000;

    signed as f32 * 0.001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_data_generation() {
        assert_eq!(pseudo_random_value(42, 123), pseudo_random_value(42, 123),);
    }

    #[test]
    fn different_seeds_change_generated_data() {
        assert_ne!(pseudo_random_value(42, 123), pseudo_random_value(42, 456),);
    }

    #[test]
    fn scalar_verification_handles_tail_length() {
        assert!(verify_length(33, &[Engine::scalar()],).is_ok());
    }

    #[test]
    fn scalar_verification_handles_empty_input() {
        assert!(verify_length(0, &[Engine::scalar()],).is_ok());
    }

    #[test]
    fn scalar_numerical_cases_pass() {
        assert!(verify_numerical_cases(&[Engine::scalar()],).is_ok());
    }

    #[test]
    fn close_check_rejects_large_error() {
        let failure = check_close("test", "dot", 4, None, 100.0, 1.0);

        assert!(failure.is_err());
    }

    #[test]
    fn nan_comparison_uses_classification() {
        assert!(same_float(f32::NAN, f32::NAN,));
    }

    #[test]
    fn signed_zero_is_numerically_equal() {
        assert!(same_float(0.0, -0.0,));
    }
}
