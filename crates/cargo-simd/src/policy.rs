use std::fmt;

use crate::system::SystemInfo;

const GIB: u64 = 1024 * 1024 * 1024;

const BUILD_GIB_PER_JOB: f64 = 1.5;
const TEST_GIB_PER_THREAD: f64 = 0.75;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    LowResource,
    Balanced,
    Performance,
}

impl fmt::Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LowResource => write!(f, "low-resource"),
            Self::Balanced => write!(f, "balanced"),
            Self::Performance => write!(f, "performance"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceBudget {
    pub build_jobs: usize,
    pub test_threads: usize,
}

#[derive(Debug)]
pub struct Recommendation {
    pub policy: Policy,
    pub budget: ResourceBudget,
    pub reason: &'static str,
}

pub fn recommend(system: &SystemInfo) -> Recommendation {
    let cpus = system.logical_cpus.max(1);

    let build_jobs = memory_limited_slots(system.memory_available_bytes, BUILD_GIB_PER_JOB, cpus);

    let test_threads =
        memory_limited_slots(system.memory_available_bytes, TEST_GIB_PER_THREAD, cpus);

    let low_total_memory = system
        .memory_total_bytes
        .is_some_and(|bytes| bytes < 8 * GIB);

    let low_available_memory = system
        .memory_available_bytes
        .is_some_and(|bytes| bytes < 3 * GIB);

    if low_total_memory || low_available_memory {
        return Recommendation {
            policy: Policy::LowResource,
            budget: ResourceBudget {
                build_jobs: build_jobs.min(4),
                test_threads: test_threads.min(6),
            },
            reason: "memory pressure favors conservative process concurrency",
        };
    }

    let high_memory = system
        .memory_total_bytes
        .is_some_and(|bytes| bytes >= 32 * GIB);

    if high_memory && cpus >= 12 {
        return Recommendation {
            policy: Policy::Performance,
            budget: ResourceBudget {
                build_jobs,
                test_threads,
            },
            reason: "system has enough CPU and memory for aggressive concurrency",
        };
    }

    Recommendation {
        policy: Policy::Balanced,
        budget: ResourceBudget {
            build_jobs,
            test_threads,
        },
        reason: "balanced CPU and memory utilization is recommended",
    }
}

fn memory_limited_slots(
    available_bytes: Option<u64>,
    gib_per_slot: f64,
    cpu_limit: usize,
) -> usize {
    let Some(bytes) = available_bytes else {
        return cpu_limit.max(1);
    };

    let available_gib = bytes as f64 / GIB as f64;

    let memory_slots = (available_gib / gib_per_slot).floor() as usize;

    memory_slots.max(1).min(cpu_limit.max(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn system(cpus: usize, total_gib: u64, available_gib: u64) -> SystemInfo {
        SystemInfo {
            operating_system: "linux",
            logical_cpus: cpus,
            memory_total_bytes: Some(total_gib * GIB),
            memory_available_bytes: Some(available_gib * GIB),
            rustc_version: None,
            cargo_version: None,
        }
    }

    #[test]
    fn chooses_low_resource_for_small_machine() {
        let recommendation = recommend(&system(16, 7, 6));

        assert_eq!(recommendation.policy, Policy::LowResource);

        assert!(recommendation.budget.build_jobs <= 4);
        assert!(recommendation.budget.test_threads <= 6);
    }

    #[test]
    fn chooses_balanced_for_normal_machine() {
        let recommendation = recommend(&system(8, 16, 8));

        assert_eq!(recommendation.policy, Policy::Balanced);

        assert!(recommendation.budget.build_jobs <= 8);
        assert!(recommendation.budget.test_threads <= 8);
    }

    #[test]
    fn chooses_performance_for_large_machine() {
        let recommendation = recommend(&system(16, 64, 48));

        assert_eq!(recommendation.policy, Policy::Performance);

        assert_eq!(recommendation.budget.build_jobs, 16);
        assert_eq!(recommendation.budget.test_threads, 16);
    }

    #[test]
    fn memory_never_produces_zero_slots() {
        let slots = memory_limited_slots(Some(128 * 1024 * 1024), BUILD_GIB_PER_JOB, 16);

        assert_eq!(slots, 1);
    }
}
