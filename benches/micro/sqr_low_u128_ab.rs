// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A/B for the `int::policy::sqr_low` limb-width (`u64` vs `u128`) axis.
//!
//! Ranks the two limb widths of the truncated-low symmetric square kernel
//! `sqr_low_limb::<N, L>` (`src/int/algos/sqr/sqr_low_limb.rs`) at the even
//! widths the wide-tier exp/powf Smith squaring runs on (the work integers
//! Int<128>/Int<192>/Int<256> and the storage widths 32/48/64). `u128` packs
//! the operand into N/2 u128 limbs (half the limbs, ≈¼ the partial products,
//! at the cost of a wider 128×128→256 inner step). The winner per even `N`
//! becomes that cell's `LimbSize` in `int::policy::sqr_low::limb_size`.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench sqr_low_u128_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{sqr_low_u128, sqr_low_u64};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// A seeded operand plus a label for its `BenchmarkId`.
#[derive(Clone)]
struct Operand<const N: usize> {
    label: &'static str,
    x: [u64; N],
}

/// Deterministic limb fill (splitmix64 over a seeded counter).
fn fill<const N: usize>(seed: u64) -> [u64; N] {
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

/// Low / mid / high seeded operands at width `N`.
fn operand_set<const N: usize>() -> Vec<Operand<N>> {
    vec![
        Operand { label: "low", x: fill::<N>(3) },
        Operand { label: "mid", x: fill::<N>(7) },
        Operand { label: "high", x: fill::<N>(1009) },
    ]
}

/// u64 base-2^64 truncated-low symmetric square candidate.
fn u64_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    let mut out = [0u64; N];
    sqr_low_u64::<N>(&op.x, &mut out);
    out
}

/// u128-packed truncated-low symmetric square candidate.
fn u128_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    let mut out = [0u64; N];
    sqr_low_u128::<N>(&op.x, &mut out);
    out
}

/// N-way (here 2-way) comparison at one width.
fn compare_width<const N: usize>(c: &mut Criterion, width_label: &str) {
    // Correctness cross-check: the two candidates must agree before timing.
    for op in operand_set::<N>() {
        assert_eq!(
            u64_run::<N>(op.clone()),
            u128_run::<N>(op.clone()),
            "u64 vs u128 low-sqr disagree at width {N} ({})",
            op.label
        );
    }
    compare_all(
        c,
        &format!("sqr_low/{width_label}"),
        |op: &Operand<N>| op.label.to_string(),
        operand_set::<N>(),
        vec![
            ("u64", u64_run::<N> as fn(Operand<N>) -> [u64; N]),
            ("u128", u128_run::<N>),
        ],
    );
}

/// Bench the even widths the wide exp/powf squaring touches: the guard-digit
/// work integers and the storage widths (N=64 is both a D307 work integer and
/// D1232 storage).
fn bench_sqr_low(c: &mut Criterion) {
    compare_width::<128>(c, "Int128");
    compare_width::<192>(c, "Int192");
    compare_width::<256>(c, "Int256");
    compare_width::<32>(c, "Int32");
    compare_width::<48>(c, "Int48");
    compare_width::<64>(c, "Int64");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_sqr_low(&mut c);
    c.final_summary();
}
