use std::fs;
use std::process::Command;
use std::thread;

const KIB: u64 = 1024;

#[derive(Debug)]
pub struct SystemInfo {
    pub operating_system: &'static str,
    pub logical_cpus: usize,
    pub memory_total_bytes: Option<u64>,
    pub memory_available_bytes: Option<u64>,
    pub rustc_version: Option<String>,
    pub cargo_version: Option<String>,
}

pub fn inspect() -> SystemInfo {
    let (memory_total_bytes, memory_available_bytes) = memory_info();

    SystemInfo {
        operating_system: std::env::consts::OS,
        logical_cpus: thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1),
        memory_total_bytes,
        memory_available_bytes,
        rustc_version: command_version("rustc"),
        cargo_version: command_version("cargo"),
    }
}

fn command_version(program: &str) -> Option<String> {
    let output = Command::new(program).arg("--version").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    Some(text.trim().to_owned())
}

fn memory_info() -> (Option<u64>, Option<u64>) {
    let Ok(contents) = fs::read_to_string("/proc/meminfo") else {
        return (None, None);
    };

    let mut total = None;
    let mut available = None;

    for line in contents.lines() {
        if let Some(value) = parse_meminfo_line(line, "MemTotal:") {
            total = Some(value);
        }

        if let Some(value) = parse_meminfo_line(line, "MemAvailable:") {
            available = Some(value);
        }
    }

    (total, available)
}

fn parse_meminfo_line(line: &str, key: &str) -> Option<u64> {
    let rest = line.strip_prefix(key)?;

    let kib = rest.split_whitespace().next()?.parse::<u64>().ok()?;

    kib.checked_mul(KIB)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_linux_meminfo_value() {
        assert_eq!(
            parse_meminfo_line("MemTotal:       16384000 kB", "MemTotal:"),
            Some(16_777_216_000)
        );
    }

    #[test]
    fn rejects_wrong_meminfo_key() {
        assert_eq!(parse_meminfo_line("MemFree: 100 kB", "MemTotal:"), None);
    }
}
