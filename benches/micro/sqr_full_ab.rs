// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! N-way A/B for the `int::policy::sqr` truncated-low square (`x²` mod 2^BITS).
//!
//! Ranks every candidate the production square dispatch chooses between,
//! across the full width axis `N ∈ {2,3,4,6,8,12,16,24,32,48,64}`:
//!
//! - `half_product` -> `sqr_half_product` (const comba symmetric square — the
//!   `int::policy::sqr` production arm; ≈N²/4 limb-multiplies).
//! - `schoolbook`   -> `sqr_schoolbook` (const full `x·x` truncated to the low
//!   N limbs via `mul_low_fixed`; ≈N²/2 limb-multiplies — the "route the
//!   square through mul (x·x)" baseline, the bit-identicality reference).
//! - `low_u64`      -> `sqr_low_limb::<N, u64>` (the NON-const row-idiom u64
//!   truncated-low symmetric square; the `int::policy::sqr_low` U64 arm).
//! - `low_u128`     -> `sqr_low_limb::<N, u128>` (u128-packed, EVEN N only; the
//!   `int::policy::sqr_low` U128 arm — the LimbSize axis).
//!
//! Two regions are read from this map:
//!   (a) half_product vs schoolbook (the CONST candidates `int::policy::sqr`
//!       must choose between — its dispatch is `const fn`, so the non-const
//!       `sqr_low_limb` u128 path is INELIGIBLE there), and
//!   (b) the u64 -> u128 `LimbSize` crossover per even N (owned by the
//!       separate non-const `int::policy::sqr_low` policy).
//!
//! A candidate is eligible at a cell only where it is bit-identical to
//! `half_product` (the truncated-low square is exact — every arm must agree;
//! any mismatch is flagged by the pre-timing `assert_eq!`).
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench sqr_full_ab`

use criterion::Criterion;
use decimal_scaled::Uint;
use decimal_scaled::__bench_internals::{
    sqr_half_product, sqr_low_u128, sqr_low_u64, sqr_schoolbook,
};

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

/// Const comba half-product square (production `int::policy::sqr` arm).
fn half_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    *sqr_half_product::<N>(Uint::<N>::from_limbs(op.x)).as_limbs()
}

/// Const full `x·x` truncated-low square (the "route through mul" baseline).
fn school_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    *sqr_schoolbook::<N>(Uint::<N>::from_limbs(op.x)).as_limbs()
}

/// Non-const row-idiom truncated-low symmetric square at `L = u64`.
fn low_u64_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    let mut out = [0u64; N];
    sqr_low_u64::<N>(&op.x, &mut out);
    out
}

/// Non-const u128-packed truncated-low symmetric square (even N only).
fn low_u128_run<const N: usize>(op: Operand<N>) -> [u64; N] {
    let mut out = [0u64; N];
    sqr_low_u128::<N>(&op.x, &mut out);
    out
}

/// N-way comparison at one ODD width (no u128 arm — packing needs even N).
fn compare_odd<const N: usize>(c: &mut Criterion, width_label: &str) {
    for op in operand_set::<N>() {
        let r = half_run::<N>(op.clone());
        assert_eq!(school_run::<N>(op.clone()), r, "school != half at N={N} ({})", op.label);
        assert_eq!(low_u64_run::<N>(op.clone()), r, "low_u64 != half at N={N} ({})", op.label);
    }
    compare_all(
        c,
        &format!("sqr_full/{width_label}"),
        |op: &Operand<N>| op.label.to_string(),
        operand_set::<N>(),
        vec![
            ("half_product", half_run::<N> as fn(Operand<N>) -> [u64; N]),
            ("schoolbook", school_run::<N>),
            ("low_u64", low_u64_run::<N>),
        ],
    );
}

/// N-way comparison at one EVEN width — adds the `u128`-packed arm.
fn compare_even<const N: usize>(c: &mut Criterion, width_label: &str) {
    for op in operand_set::<N>() {
        let r = half_run::<N>(op.clone());
        assert_eq!(school_run::<N>(op.clone()), r, "school != half at N={N} ({})", op.label);
        assert_eq!(low_u64_run::<N>(op.clone()), r, "low_u64 != half at N={N} ({})", op.label);
        assert_eq!(low_u128_run::<N>(op.clone()), r, "low_u128 != half at N={N} ({})", op.label);
    }
    compare_all(
        c,
        &format!("sqr_full/{width_label}"),
        |op: &Operand<N>| op.label.to_string(),
        operand_set::<N>(),
        vec![
            ("half_product", half_run::<N> as fn(Operand<N>) -> [u64; N]),
            ("schoolbook", school_run::<N>),
            ("low_u64", low_u64_run::<N>),
            ("low_u128", low_u128_run::<N>),
        ],
    );
}

/// Sweep the full width axis. Coarse `{2,4,8,16,32,64}` plus the bisection
/// points `{3,6,12,24,48}` that localize the half_product/schoolbook edge
/// (if any) and the u64/u128 LimbSize edge.
fn bench_sqr_full(c: &mut Criterion) {
    compare_even::<2>(c, "Int128");
    compare_odd::<3>(c, "Int192");
    compare_even::<4>(c, "Int256");
    compare_even::<6>(c, "Int384");
    compare_even::<8>(c, "Int512");
    // Bisection of the half_product<->schoolbook const crossover: N=8/12 favour
    // the unrolled schoolbook, N>=16 the comba half_product; N=10/14 localize
    // the upper edge.
    compare_even::<10>(c, "Int640");
    compare_even::<12>(c, "Int768");
    compare_even::<14>(c, "Int896");
    compare_even::<16>(c, "Int1024");
    compare_even::<24>(c, "Int1536");
    compare_even::<32>(c, "Int2048");
    compare_even::<48>(c, "Int3072");
    compare_even::<64>(c, "Int4096");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_sqr_full(&mut c);
    c.final_summary();
}
