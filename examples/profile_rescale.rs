// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! samply hot-path probe for the high-scale decimal-mul ÷10^SCALE rescale
//! (the Newton-reciprocal path, SCALE>38). Loops the real production
//! `dec_mul_widen_divide::<N, SCALE>` at D924_s923 (N=48, the biggest bbc
//! regression cell) in a `black_box`-guarded loop so the profiler captures
//! the rescale's own self-time (precompute Knuth-for-r, the mag·r>>k divide,
//! the correction) post the pow10-table wire-in. Reusable.
//!
//! Build + profile:
//!   cargo build --release --features "wide x-wide xx-wide macros bench-alt" --example profile_rescale
//!   research/run_samply.sh target/release/examples/profile_rescale.exe "" trace/prof_rescale.json.gz
//!   python research/samply_symbolize.py trace/prof_rescale.json.gz target/release/examples/profile_rescale.exe 40

use core::hint::black_box;

use decimal_scaled::__bench_internals::{dec_mul_widen_divide, int_from_mag_limbs};
use decimal_scaled::{Int, RoundingMode};

const N: usize = 48; // D924
const SCALE: u32 = 923; // s923 (the biggest bbc rescale-regression cell)

fn operand(seed: u64) -> Int<N> {
    let mut mag = [0u64; N];
    for (i, slot) in mag.iter_mut().enumerate() {
        *slot = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add((i as u64).wrapping_mul(0x1357_9BDF));
    }
    // keep the top limb small so a·b/10^scale stays representable
    mag[N - 1] &= 0xFFFF;
    int_from_mag_limbs::<N>(&mag)
}

fn main() {
    let iters: u64 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(40_000);
    let a = operand(1009);
    let b = operand(2027);
    let mut acc = 0u64;
    for _ in 0..iters {
        let r = dec_mul_widen_divide::<N, SCALE>(black_box(a), black_box(b), RoundingMode::HalfToEven);
        acc = acc.wrapping_add(r.unsigned_abs().as_limbs()[0]);
    }
    println!("acc={acc}");
}
