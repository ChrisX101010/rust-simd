
use std::hint::black_box;
use std::time::Instant;

unsafe extern "C" {
    fn simd_ffi_add_one(x: u64) -> u64;
}

const CALLS: usize = 1_000_000;
const ROUNDS: usize = 20;

fn main() {
    let mut timings = Vec::with_capacity(ROUNDS);
    let mut sink = 0u64;

    for round in 0..ROUNDS {
        let start = Instant::now();

        let mut value = round as u64;

        for _ in 0..CALLS {
            value = unsafe {
                simd_ffi_add_one(black_box(value))
            };
        }

        sink = black_box(value);
        timings.push(start.elapsed().as_nanos());
    }

    timings.sort_unstable();

    let median_total = timings[timings.len() / 2];
    let median_call = median_total as f64 / CALLS as f64;

    println!("calls_per_round={CALLS}");
    println!("rounds={ROUNDS}");
    println!("median_total_ns={median_total}");
    println!("median_call_ns={median_call:.3}");
    println!("sink={sink}");
}
