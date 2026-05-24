// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Pilot A/B: the wide-tier exp/powf Taylor work-multiply.
//!
//! The wide exp/powf path (`macros::wide_transcendental::exp_fixed`) runs
//! a Taylor loop whose hot step is `mul_cached(term, s_red, pow10_w)`,
//! which is `round_div((term * s_red), pow10_w)`. The `term * s_red` is
//! `Int<N>::wrapping_mul` -> the truncated-low base-2^64 schoolbook
//! kernel `mul_low_fixed::<N>` (`src/int/algos/mul/mul_schoolbook.rs`),
//! an O(N^2) low-half multiply on the tier's WORK integer:
//!
//!   * D616  -> work integer `Int<128>` (N = 128 u64 limbs)
//!   * D924  -> work integer `Int<192>` (N = 192 u64 limbs)
//!   * D1232 -> work integer `Int<256>` (N = 256 u64 limbs)
//!
//! (The decimal STORAGE widths 32 / 48 / 64 limbs are NOT where this
//! multiply runs; the Taylor series operates in the wider guard-digit
//! work integer `W`. Both bands are benched below so the decision is not
//! based on the wrong width.)
//!
//! Candidate: `mul_low_fixed_u128::<N>`, the same truncated-low
//! schoolbook packed into `N/2` u128 limbs (the base-2^128 shape 0.4.4
//! ran). Half the limbs, ~1/4 the partial products, at the cost of a
//! wider 128x128->256 inner step.
//!
//! DECISION RULE (from the pilot brief): u128 must win by >= 1.15x at the
//! wide work widths (N = 128 / 192 / 256) to justify wiring it into exp.
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

/// Bench every width: the three wide-tier WORK integers (the decisive
/// band) and, for completeness, the three storage widths.
fn bench_mul_low(c: &mut Criterion) {
    // --- decisive band: the exp/powf Taylor work integers ---
    compare_width::<128>(c, "Int128_D616_work");
    compare_width::<192>(c, "Int192_D924_work");
    compare_width::<256>(c, "Int256_D1232_work");
    // --- reference band: the decimal storage widths ---
    compare_width::<32>(c, "Int32_D616_store");
    compare_width::<48>(c, "Int48_D924_store");
    compare_width::<64>(c, "Int64_D1232_store");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_mul_low(&mut c);
    c.final_summary();
}
