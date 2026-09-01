use std::process::ExitCode;

use rust_simd::Capabilities;

use crate::policy;
use crate::project;
use crate::system;

pub fn run(args: &[String]) -> ExitCode {
    let deep = args.iter().any(|arg| arg == "--deep");

    for arg in args {
        if arg != "--deep" {
            eprintln!("error: unsupported doctor option `{arg}`");
            return ExitCode::from(2);
        }
    }

    let system = system::inspect();
    let project = project::inspect(deep);
    let recommendation = policy::recommend(&system);
    let capabilities = Capabilities::detect();

    println!("cargo-simd doctor");
    println!();

    println!("SYSTEM");
    println!(
        "  architecture       {}",
        capabilities.architecture().name()
    );
    println!("  operating system   {}", system.operating_system);
    println!("  logical CPUs       {}", system.logical_cpus);
    println!(
        "  memory total       {}",
        format_optional_bytes(system.memory_total_bytes)
    );
    println!(
        "  memory available   {}",
        format_optional_bytes(system.memory_available_bytes)
    );

    println!();
    println!("RUST");
    println!(
        "  rustc              {}",
        system.rustc_version.as_deref().unwrap_or("unknown")
    );
    println!(
        "  cargo              {}",
        system.cargo_version.as_deref().unwrap_or("unknown")
    );

    println!();
    println!("SIMD");
    println!(
        "  vector model       {}",
        capabilities.vector_model().name()
    );
    println!(
        "  selected backend   {}",
        capabilities.best_backend().name()
    );
    println!("  AVX2 available     {}", yes_no(capabilities.has_avx2()));
    println!("  FMA available      {}", yes_no(capabilities.has_fma()));
    println!("  NEON available     {}", yes_no(capabilities.has_neon()));
    println!(
        "  WASM SIMD128       {}",
        yes_no(capabilities.has_wasm_simd128())
    );

    println!();
    println!("PROJECT");

    match project.root.as_deref() {
        Some(root) => {
            println!("  root               {}", root.display());
            println!("  Cargo.lock         {}", yes_no(project.has_lockfile));

            println!(
                "  target directory   {}",
                project
                    .target_directory
                    .as_deref()
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|| "not present".to_owned())
            );

            if deep {
                println!(
                    "  target size        {}",
                    format_optional_bytes(project.target_size_bytes)
                );
            } else {
                println!("  target size        skipped (use --deep)");
            }
        }

        None => {
            println!("  root               no Cargo project detected");
        }
    }

    println!();
    println!("RECOMMENDATION");
    println!("  policy             {}", recommendation.policy);
    println!("  build jobs         {}", recommendation.budget.build_jobs);
    println!(
        "  test threads       {}",
        recommendation.budget.test_threads
    );
    println!("  reason             {}", recommendation.reason);

    println!();
    println!("STATUS");
    println!("  ✓ toolchain inspection");
    println!("  ✓ shared SIMD capability model");
    println!("  ✓ resource policy");
    println!("  ✓ project inspection");

    if !deep {
        println!();
        println!("Run `cargo simd doctor --deep` for target-size analysis.");
    }

    ExitCode::SUCCESS
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn format_optional_bytes(bytes: Option<u64>) -> String {
    bytes
        .map(format_bytes)
        .unwrap_or_else(|| "unknown".to_owned())
}

fn format_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;

    let bytes = bytes as f64;

    if bytes >= GIB {
        return format!("{:.2} GiB", bytes / GIB);
    }

    if bytes >= MIB {
        return format!("{:.2} MiB", bytes / MIB);
    }

    if bytes >= KIB {
        return format!("{:.2} KiB", bytes / KIB);
    }

    format!("{bytes:.0} B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_gibibytes() {
        assert_eq!(format_bytes(2 * 1024 * 1024 * 1024), "2.00 GiB");
    }

    #[test]
    fn formats_mebibytes() {
        assert_eq!(format_bytes(128 * 1024 * 1024), "128.00 MiB");
    }
}
