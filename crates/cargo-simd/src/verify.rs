use std::process::ExitCode;

use rust_simd::Engine;

use crate::report::{CheckStatus, FailureRecord, VerificationReport};

const LENGTHS: &[usize] = &[
    0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023,
    1024, 1025, 4095, 4096, 4097, 16_383, 16_384, 16_385,
];

#[derive(Debug, Clone)]
struct VerificationFailure {
    backend: &'static str,
    operation: &'static str,
    case_name: Option<&'static str>,
    length: usize,
    index: Option<usize>,
    actual: f32,
    expected: f64,
    error: f64,
    tolerance: f64,
}

impl From<VerificationFailure> for FailureRecord {
    fn from(failure: VerificationFailure) -> Self {
        Self {
            backend: failure.backend.to_owned(),
            operation: failure.operation.to_owned(),
            case_name: failure.case_name.map(str::to_owned),
            length: failure.length,
            index: failure.index,
            actual: failure.actual,
            expected: failure.expected,
            error: failure.error,
            tolerance: failure.tolerance,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct VerifyOptions {
    json: bool,
    backend: Option<String>,
}

pub fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return ExitCode::SUCCESS;
    }

    let options = match parse_options(args) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };

    let engines = match select_engines(options.backend.as_deref()) {
        Ok(engines) => engines,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(2);
        }
    };

    let report = run_verification(&engines, options.backend.clone());

    if options.json {
        println!("{}", report.to_json());
    } else {
        print!("{}", report.to_text());
    }

    if report.passed() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn parse_options(args: &[String]) -> Result<VerifyOptions, String> {
    let mut options = VerifyOptions::default();
    let mut index = 0;

    while index < args.len() {
        let argument = &args[index];

        match argument.as_str() {
            "--json" => {
                options.json = true;
                index += 1;
            }

            "--backend" => {
                if options.backend.is_some() {
                    return Err("`--backend` may only be specified once".to_owned());
                }

                let Some(value) = args.get(index + 1) else {
                    return Err("`--backend` requires a backend name".to_owned());
                };

                if value.starts_with('-') {
                    return Err("`--backend` requires a backend name".to_owned());
                }

                options.backend = Some(value.clone());
                index += 2;
            }

            _ if argument.starts_with("--backend=") => {
                if options.backend.is_some() {
                    return Err("`--backend` may only be specified once".to_owned());
                }

                let value = argument
                    .strip_prefix("--backend=")
                    .expect("prefix was checked");

                if value.is_empty() {
                    return Err("`--backend` requires a backend name".to_owned());
                }

                options.backend = Some(value.to_owned());
                index += 1;
            }

            _ => {
                return Err(format!(
                    "unknown verify argument `{argument}`; use `cargo simd verify --help`"
                ));
            }
        }
    }

    Ok(options)
}

fn select_engines(backend: Option<&str>) -> Result<Vec<Engine>, String> {
    let Some(backend) = backend else {
        return Ok(available_engines());
    };

    let engine = match backend {
        "scalar" => Engine::scalar(),

        "avx2" => Engine::avx2().map_err(|_| unavailable_backend_error("avx2"))?,

        "avx2+fma" => Engine::avx2_fma().map_err(|_| unavailable_backend_error("avx2+fma"))?,

        "neon" => Engine::neon().map_err(|_| unavailable_backend_error("neon"))?,

        "wasm-simd128" => {
            Engine::wasm_simd128().map_err(|_| unavailable_backend_error("wasm-simd128"))?
        }

        other => {
            return Err(format!(
                "unknown backend `{other}`; expected one of: scalar, avx2, avx2+fma, neon, wasm-simd128"
            ));
        }
    };

    Ok(vec![engine])
}

fn unavailable_backend_error(backend: &str) -> String {
    format!("backend `{backend}` is not available on this machine/build")
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

fn run_verification(engines: &[Engine], backend_filter: Option<String>) -> VerificationReport {
    let backend_names = engines
        .iter()
        .map(|engine| engine.backend_name().to_owned())
        .collect();

    let mut report = VerificationReport::new(backend_filter, backend_names, LENGTHS.to_vec());

    for &length in LENGTHS {
        match verify_length(length, engines) {
            Ok(()) => {
                report.lengths_completed += 1;
            }

            Err(failure) => {
                mark_operation_failure(&mut report, failure.operation);
                report.failures.push(failure.into());
                return report;
            }
        }
    }

    report.vector_add = CheckStatus::Pass;
    report.fma = CheckStatus::Pass;
    report.reduce_sum = CheckStatus::Pass;
    report.dot = CheckStatus::Pass;

    match verify_numerical_cases(engines) {
        Ok(()) => {
            report.numerical_cases = CheckStatus::Pass;
        }

        Err(failure) => {
            report.numerical_cases = CheckStatus::Fail;
            mark_operation_failure(&mut report, failure.operation);
            report.failures.push(failure.into());
        }
    }

    report
}

fn mark_operation_failure(report: &mut VerificationReport, operation: &str) {
    if operation.starts_with("vector_add") {
        report.vector_add = CheckStatus::Fail;
    } else if operation == "fma" {
        report.fma = CheckStatus::Fail;
    } else if operation == "reduce_sum" {
        report.reduce_sum = CheckStatus::Fail;
    } else if operation == "dot" {
        report.dot = CheckStatus::Fail;
    }
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
            None,
            length,
            None,
            engine.reduce_sum(&a),
            sum_reference,
        )?;

        check_close(
            engine.backend_name(),
            "dot",
            None,
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
                case_name: None,
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
            None,
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
    case_name: &'static str,
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
        verify_vector_add(*engine, a, b).map_err(|failure| with_case(failure, case_name))?;

        verify_fma(*engine, a, b, c).map_err(|failure| with_case(failure, case_name))?;

        check_close(
            engine.backend_name(),
            "reduce_sum",
            Some(case_name),
            a.len(),
            None,
            engine.reduce_sum(a),
            sum_reference,
        )?;

        check_close(
            engine.backend_name(),
            "dot",
            Some(case_name),
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
                    case_name: Some("non-finite"),
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

fn with_case(mut failure: VerificationFailure, case_name: &'static str) -> VerificationFailure {
    failure.case_name = Some(case_name);
    failure
}

fn check_close(
    backend: &'static str,
    operation: &'static str,
    case_name: Option<&'static str>,
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
            case_name,
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
            case_name,
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
        case_name,
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

fn print_help() {
    println!(
        "\
cargo-simd verify — differential SIMD backend verification

USAGE:
    cargo simd verify [OPTIONS]

OPTIONS:
    --json
        Emit a machine-readable JSON report to stdout.

    --backend <BACKEND>
        Verify only one backend.

        Supported backend names:
            scalar
            avx2
            avx2+fma
            neon
            wasm-simd128

    -h, --help
        Print this help.

EXIT CODES:
    0    Verification passed
    1    Verification failed
    2    Invalid arguments or unavailable backend

EXAMPLES:
    cargo simd verify
    cargo simd verify --json
    cargo simd verify --backend scalar
    cargo simd verify --backend avx2
    cargo simd verify --backend avx2+fma
    cargo simd verify --json --backend scalar
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_data_generation() {
        assert_eq!(pseudo_random_value(42, 123), pseudo_random_value(42, 123));
    }

    #[test]
    fn different_seeds_change_generated_data() {
        assert_ne!(pseudo_random_value(42, 123), pseudo_random_value(42, 456));
    }

    #[test]
    fn scalar_verification_handles_tail_length() {
        assert!(verify_length(33, &[Engine::scalar()]).is_ok());
    }

    #[test]
    fn scalar_verification_handles_empty_input() {
        assert!(verify_length(0, &[Engine::scalar()]).is_ok());
    }

    #[test]
    fn scalar_numerical_cases_pass() {
        assert!(verify_numerical_cases(&[Engine::scalar()]).is_ok());
    }

    #[test]
    fn close_check_rejects_large_error() {
        let failure = check_close("test", "dot", None, 4, None, 100.0, 1.0);

        assert!(failure.is_err());
    }

    #[test]
    fn nan_comparison_uses_classification() {
        assert!(same_float(f32::NAN, f32::NAN));
    }

    #[test]
    fn signed_zero_is_numerically_equal() {
        assert!(same_float(0.0, -0.0));
    }

    #[test]
    fn parser_accepts_json() {
        let args = vec!["--json".to_owned()];

        let options = parse_options(&args).expect("JSON option should parse");

        assert!(options.json);
        assert_eq!(options.backend, None);
    }

    #[test]
    fn parser_accepts_backend() {
        let args = vec!["--backend".to_owned(), "scalar".to_owned()];

        let options = parse_options(&args).expect("backend option should parse");

        assert_eq!(options.backend.as_deref(), Some("scalar"));
    }

    #[test]
    fn parser_accepts_backend_equals_form() {
        let args = vec!["--backend=scalar".to_owned()];

        let options = parse_options(&args).expect("backend option should parse");

        assert_eq!(options.backend.as_deref(), Some("scalar"));
    }

    #[test]
    fn parser_rejects_unknown_argument() {
        let args = vec!["--wat".to_owned()];

        assert!(parse_options(&args).is_err());
    }

    #[test]
    fn parser_rejects_duplicate_backend() {
        let args = vec![
            "--backend".to_owned(),
            "scalar".to_owned(),
            "--backend".to_owned(),
            "scalar".to_owned(),
        ];

        assert!(parse_options(&args).is_err());
    }

    #[test]
    fn scalar_backend_can_be_selected() {
        let engines = select_engines(Some("scalar")).expect("scalar is always available");

        assert_eq!(engines.len(), 1);
        assert_eq!(engines[0].backend_name(), "scalar");
    }

    #[test]
    fn unknown_backend_is_rejected() {
        assert!(select_engines(Some("definitely-not-a-backend")).is_err());
    }

    #[test]
    fn passing_scalar_report_is_structured() {
        let report = run_verification(&[Engine::scalar()], Some("scalar".to_owned()));

        assert!(report.passed());
        assert_eq!(report.lengths_completed, LENGTHS.len());
        assert_eq!(report.backends, vec!["scalar".to_owned()]);
    }
}
