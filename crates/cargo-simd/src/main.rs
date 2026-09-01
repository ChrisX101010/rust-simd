mod doctor;
mod orchestrator;
mod policy;
mod project;
mod report;
mod system;
mod verify;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("simd") {
        args.remove(0);
    }

    match args.first().map(String::as_str) {
        Some("doctor") => doctor::run(&args[1..]),

        Some("verify") => verify::run(&args[1..]),

        Some("build") => orchestrator::run_build(&args[1..]),

        Some("test") => orchestrator::run_test(&args[1..]),

        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            ExitCode::SUCCESS
        }

        Some("version") | Some("--version") | Some("-V") => {
            println!("cargo-simd {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }

        Some(command) => {
            eprintln!("error: unknown cargo-simd command `{command}`");
            eprintln!();

            print_help();

            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!(
        "\
cargo-simd — lightweight SIMD-aware Rust development tooling

USAGE:
    cargo simd <COMMAND>

COMMANDS:
    doctor    Inspect hardware, SIMD, project, and resource capabilities
    verify    Differentially verify available SIMD backends
    build     Build with a resource-aware Cargo job budget
    test      Test with resource-aware build/test concurrency
    help      Print this help
    version   Print cargo-simd version

VERIFY:
    cargo simd verify
    cargo simd verify --json
    cargo simd verify --backend scalar
    cargo simd verify --backend avx2
    cargo simd verify --backend avx2+fma
    cargo simd verify --json --backend scalar

BUILD:
    cargo simd build
    cargo simd build --release
    cargo simd build --dry-run

TEST:
    cargo simd test
    cargo simd test --workspace
    cargo simd test --nextest
    cargo simd test --dry-run

NETWORK:
    Build/test orchestration is offline by default.
    Use --online to explicitly allow Cargo network access.

OPTIONAL INTEGRATIONS:
    cargo-nextest
        Enhanced test scheduling.
        Install with:
            cargo install cargo-nextest --locked

EXAMPLES:
    cargo simd doctor
    cargo simd doctor --deep
    cargo simd verify
    cargo simd verify --json
    cargo simd build --release
    cargo simd test --workspace
"
    );
}
