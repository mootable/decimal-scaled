// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! A/B for the `int::policy::mul_low` limb-width (`u64` vs `u128`) axis.
//!
//! Ranks the two limb widths of the truncated-low schoolbook kernel
//! `mul_low_limb::<N, L>` (`src/int/algos/mul/mul_schoolbook.rs`) at the even
//! widths the wide-tier exp/powf Taylor multiply runs on (the work integers
//! Int<128>/Int<192>/Int<256> and the storage widths 32/48/64). `u128` packs
//! each operand into N/2 u128 limbs (half the limbs, ~1/4 the partial
//! products, at the cost of a wider 128x128->256 inner step). The winner per
//! even `N` becomes that cell's `LimbSize` in
//! `int::policy::mul_low::limb_size`.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench mul_low_u128_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{mul_low_u128, mul_low_u64};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// A seeded equal-length operand pair plus a label for its `BenchmarkId`.
#[derive(Clone)]
struct Operands<const N: usize> {
    label: &'static str,
    a: [u64; N],
    b: [u64; N],
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

/// Low / mid / high seeded operand pairs at width `N`.
fn operand_set<const N: usize>() -> Vec<Operands<N>> {
    vec![
        Operands { label: "low", a: fill::<N>(3), b: fill::<N>(5) },
        Operands { label: "mid", a: fill::<N>(7), b: fill::<N>(13) },
        Operands { label: "high", a: fill::<N>(1009), b: fill::<N>(7919) },
    ]
}

/// u64 base-2^64 truncated-low schoolbook candidate.
fn u64_run<const N: usize>(ops: Operands<N>) -> [u64; N] {
    let mut out = [0u64; N];
    mul_low_u64::<N>(&ops.a, &ops.b, &mut out);
    out
}

/// u128-packed truncated-low schoolbook candidate.
fn u128_run<const N: usize>(ops: Operands<N>) -> [u64; N] {
    let mut out = [0u64; N];
    mul_low_u128::<N>(&ops.a, &ops.b, &mut out);
    out
}

/// N-way (here 2-way) comparison at one width.
fn compare_width<const N: usize>(c: &mut Criterion, width_label: &str) {
    // Correctness cross-check: the two candidates must agree before timing.
    for ops in operand_set::<N>() {
        assert_eq!(
            u64_run::<N>(ops.clone()),
            u128_run::<N>(ops.clone()),
            "u64 vs u128 low-mul disagree at width {N} ({})",
            ops.label
        );
    }
    compare_all(
        c,
        &format!("mul_low/{width_label}"),
        |ops: &Operands<N>| ops.label.to_string(),
        operand_set::<N>(),
        vec![
            ("u64", u64_run::<N> as fn(Operands<N>) -> [u64; N]),
            ("u128", u128_run::<N>),
        ],
    );
}

/// Bench the even widths the wide exp/powf Taylor multiply touches: the
/// guard-digit work integers and the storage widths (N=64 is both a D307 work
/// integer and D1232 storage).
fn bench_mul_low(c: &mut Criterion) {
    compare_width::<128>(c, "Int128");
    compare_width::<192>(c, "Int192");
    compare_width::<256>(c, "Int256");
    compare_width::<32>(c, "Int32");
    compare_width::<48>(c, "Int48");
    compare_width::<64>(c, "Int64");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_mul_low(&mut c);
    c.final_summary();
}
