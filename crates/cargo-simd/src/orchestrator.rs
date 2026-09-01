use std::process::{Command, ExitCode};

use crate::policy::{self, Recommendation};
use crate::system;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandPlan {
    program: String,
    args: Vec<String>,
}

pub fn run_build(args: &[String]) -> ExitCode {
    let options = match CommonOptions::parse(args, false) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::from(2);
        }
    };

    let system = system::inspect();
    let recommendation = policy::recommend(&system);

    let plan = build_plan(recommendation.budget.build_jobs, &options.forwarded);

    execute(
        "build",
        &plan,
        &recommendation,
        options.dry_run,
        options.online,
    )
}

pub fn run_test(args: &[String]) -> ExitCode {
    let options = match CommonOptions::parse(args, true) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("error: {message}");
            return ExitCode::from(2);
        }
    };

    let system = system::inspect();
    let recommendation = policy::recommend(&system);

    let plan = if options.nextest {
        if !options.dry_run && !nextest_available() {
            eprintln!("cargo-simd: optional integration `cargo-nextest` is not installed.");
            eprintln!();
            eprintln!("Install it with:");
            eprintln!("    cargo install cargo-nextest --locked");
            eprintln!();
            eprintln!("Or use the built-in runner:");
            eprintln!("    cargo simd test");

            return ExitCode::from(2);
        }

        nextest_plan(
            recommendation.budget.build_jobs,
            recommendation.budget.test_threads,
            &options.forwarded,
        )
    } else {
        cargo_test_plan(
            recommendation.budget.build_jobs,
            recommendation.budget.test_threads,
            &options.forwarded,
        )
    };

    execute(
        if options.nextest {
            "test/nextest"
        } else {
            "test/cargo"
        },
        &plan,
        &recommendation,
        options.dry_run,
        options.online,
    )
}

#[derive(Debug)]
struct CommonOptions {
    dry_run: bool,
    online: bool,
    nextest: bool,
    forwarded: Vec<String>,
}

impl CommonOptions {
    fn parse(args: &[String], allow_nextest: bool) -> Result<Self, &'static str> {
        let mut dry_run = false;
        let mut online = false;
        let mut nextest = false;
        let mut forwarded = Vec::new();

        for arg in args {
            match arg.as_str() {
                "--dry-run" => dry_run = true,
                "--online" => online = true,
                "--nextest" if allow_nextest => {
                    nextest = true;
                }
                "--nextest" => {
                    return Err("`--nextest` is only valid for `cargo simd test`");
                }
                _ => forwarded.push(arg.clone()),
            }
        }

        validate_resource_overrides(&forwarded)?;

        Ok(Self {
            dry_run,
            online,
            nextest,
            forwarded,
        })
    }
}

fn validate_resource_overrides(args: &[String]) -> Result<(), &'static str> {
    for arg in args {
        if arg == "-j"
            || arg == "--jobs"
            || arg.starts_with("--jobs=")
            || (arg.starts_with("-j") && arg.len() > 2)
            || arg == "--build-jobs"
            || arg.starts_with("--build-jobs=")
            || arg == "--test-threads"
            || arg.starts_with("--test-threads=")
        {
            return Err("resource concurrency flags are managed by cargo-simd");
        }
    }

    Ok(())
}

fn build_plan(build_jobs: usize, forwarded: &[String]) -> CommandPlan {
    let mut args = vec![
        "build".to_owned(),
        "--jobs".to_owned(),
        build_jobs.to_string(),
    ];

    args.extend_from_slice(forwarded);

    CommandPlan {
        program: "cargo".to_owned(),
        args,
    }
}

fn cargo_test_plan(build_jobs: usize, test_threads: usize, forwarded: &[String]) -> CommandPlan {
    let separator = forwarded.iter().position(|arg| arg == "--");

    let (cargo_args, test_args) = match separator {
        Some(index) => (&forwarded[..index], &forwarded[index + 1..]),
        None => (forwarded, &[][..]),
    };

    let mut args = vec![
        "test".to_owned(),
        "--jobs".to_owned(),
        build_jobs.to_string(),
    ];

    args.extend_from_slice(cargo_args);

    args.push("--".to_owned());

    args.extend_from_slice(test_args);

    args.push("--test-threads".to_owned());
    args.push(test_threads.to_string());

    CommandPlan {
        program: "cargo".to_owned(),
        args,
    }
}

fn nextest_plan(build_jobs: usize, test_threads: usize, forwarded: &[String]) -> CommandPlan {
    let mut args = vec![
        "nextest".to_owned(),
        "run".to_owned(),
        "--build-jobs".to_owned(),
        build_jobs.to_string(),
        "--test-threads".to_owned(),
        test_threads.to_string(),
    ];

    args.extend_from_slice(forwarded);

    CommandPlan {
        program: "cargo".to_owned(),
        args,
    }
}

fn nextest_available() -> bool {
    Command::new("cargo")
        .args(["nextest", "--version"])
        .output()
        .is_ok_and(|output| output.status.success())
}

fn execute(
    operation: &str,
    plan: &CommandPlan,
    recommendation: &Recommendation,
    dry_run: bool,
    online: bool,
) -> ExitCode {
    println!("cargo-simd {operation}");
    println!();

    println!("RESOURCE BUDGET");
    println!("  policy             {}", recommendation.policy);
    println!("  build jobs         {}", recommendation.budget.build_jobs);
    println!(
        "  test threads       {}",
        recommendation.budget.test_threads
    );
    println!(
        "  network            {}",
        if online {
            "Cargo default / explicitly allowed"
        } else {
            "offline"
        }
    );
    println!("  reason             {}", recommendation.reason);

    println!();
    println!("COMMAND");
    println!("  {} {}", plan.program, plan.args.join(" "));

    if dry_run {
        println!();
        println!("dry-run: command not executed");
        return ExitCode::SUCCESS;
    }

    println!();

    let mut command = Command::new(&plan.program);
    command.args(&plan.args);

    if !online {
        command.env("CARGO_NET_OFFLINE", "true");
    }

    match command.status() {
        Ok(status) if status.success() => {
            println!();
            println!("cargo-simd {operation}: PASS");
            ExitCode::SUCCESS
        }

        Ok(status) => {
            eprintln!();
            eprintln!("cargo-simd {operation}: FAILED ({status})");

            ExitCode::from(1)
        }

        Err(error) => {
            eprintln!();
            eprintln!("cargo-simd {operation}: unable to launch Cargo: {error}");

            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_plan_applies_resource_budget() {
        let plan = build_plan(4, &["--release".to_owned()]);

        assert_eq!(plan.args, ["build", "--jobs", "4", "--release",]);
    }

    #[test]
    fn cargo_test_plan_limits_build_and_test_parallelism() {
        let plan = cargo_test_plan(4, 6, &["--workspace".to_owned()]);

        assert_eq!(
            plan.args,
            [
                "test",
                "--jobs",
                "4",
                "--workspace",
                "--",
                "--test-threads",
                "6",
            ]
        );
    }

    #[test]
    fn cargo_test_preserves_harness_arguments() {
        let forwarded = vec![
            "--workspace".to_owned(),
            "--".to_owned(),
            "--nocapture".to_owned(),
        ];

        let plan = cargo_test_plan(4, 6, &forwarded);

        assert_eq!(
            plan.args,
            [
                "test",
                "--jobs",
                "4",
                "--workspace",
                "--",
                "--nocapture",
                "--test-threads",
                "6",
            ]
        );
    }

    #[test]
    fn nextest_plan_has_separate_build_and_test_budgets() {
        let plan = nextest_plan(4, 6, &["--workspace".to_owned()]);

        assert_eq!(
            plan.args,
            [
                "nextest",
                "run",
                "--build-jobs",
                "4",
                "--test-threads",
                "6",
                "--workspace",
            ]
        );
    }

    #[test]
    fn rejects_manual_cargo_jobs_override() {
        let args = vec!["--jobs".to_owned(), "16".to_owned()];

        assert!(validate_resource_overrides(&args).is_err());
    }

    #[test]
    fn rejects_manual_test_threads_override() {
        let args = vec!["--test-threads=16".to_owned()];

        assert!(validate_resource_overrides(&args).is_err());
    }
}
