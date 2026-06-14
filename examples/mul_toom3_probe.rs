// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! samply hot-path probe for the Toom-Cook 3-way integer multiply.
//!
//! Runs `mul_toom3_slice` at N=64 limbs (the widest shipped int tier) in a
//! tight `black_box`-guarded loop so the optimiser cannot fold it away and
//! the profiler captures the kernel's own self-time (per-level zeroing,
//! eval/interpolation passes, the limb leaves). Reusable for any before/after
//! Toom-3 profile.
//!
//! Build + profile (coordinator-side, headless samply):
//!   cargo build --release --features "wide bench-alt" --example mul_toom3_probe
//!   research/run_samply.sh \
//!     target/release/examples/mul_toom3_probe.exe "" trace/prof_toom3.json.gz
//!   python research/samply_symbolize.py trace/prof_toom3.json.gz \
//!     target/release/examples/mul_toom3_probe.exe 40

use std::hint::black_box;

use decimal_scaled::__bench_internals::mul_toom3_slice;

const N: usize = 64;
const ITERS: u64 = 400_000;

fn fill(seed: u64) -> [u64; N] {
    let mut out = [0u64; N];
    let mut state = seed;
    for x in out.iter_mut() {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        *x = z ^ (z >> 31);
    }
    out
}

fn main() {
    let a = fill(1009);
    let b = fill(7919);
    let mut out = [0u64; 2 * N];
    let mut acc = 0u64;
    for _ in 0..ITERS {
        for v in out.iter_mut() {
            *v = 0;
        }
        mul_toom3_slice(black_box(&a), black_box(&b), black_box(&mut out));
        acc = acc.wrapping_add(out[0]).wrapping_add(out[2 * N - 1]);
    }
    // defeat DCE: print the accumulator
    println!("acc={acc}");
}
