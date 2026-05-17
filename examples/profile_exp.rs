//! Tight-loop driver for profiling the wide-tier strict `exp_strict`
//! path under samply / flamegraph.
//!
//! Each iteration calls `D307s150::from_int(2).exp_strict()` —
//! roughly 90 µs of work per call at the time of writing — wrapped
//! in `black_box` so the optimiser cannot hoist or fold the result.
//!
//! Run under samply (writes to trace/):
//!   cargo build --release --features wide,x-wide,xx-wide,macros --example profile_exp
//!   samply record -o trace/exp_samply.json.gz target/release/examples/profile_exp.exe
//!
//! Run under cargo-flamegraph (elevated terminal on Windows):
//!   flamegraph -o trace/exp_flamegraph.svg -- target/release/examples/profile_exp.exe 10000
//!
//! The `trace/` folder is in .git/info/exclude — profiler artefacts
//! never land in the repo.
//!
//! The 10000-iter default keeps a run at ~1 second of CPU — long
//! enough for samply's 1 ms sampling cadence to land ~1000 stack
//! samples in the exp path, short enough that thermal throttling
//! and SMT noise don't dominate.

use core::hint::black_box;
use decimal_scaled::D307;

type Work = D307<150>;

fn main() {
    let iters: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10_000);
    let mut acc = Work::from_int(0);
    for i in 0..iters {
        // Vary the argument so each call goes down the same logical
        // path but does not hit the same memoised state — defensive
        // against any constant-folding the compiler might attempt.
        let x = Work::from_int(2) + Work::from_int(i as i128) / Work::from_int(1_000_000);
        acc = acc + x.exp_strict();
    }
    println!("{} iters; acc = {}", iters, black_box(acc));
}
