// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Pilot A/B: the wide-tier exp/powf Smith-squaring work-square.
//!
//! The wide exp/powf path (`macros::wide_transcendental::exp_fixed`) runs a
//! Smith range-reduction loop whose hot step is `squared.wrapping_sqr_low_u128()`
//! → `round_div_pow10(…)`. The square is the truncated-low symmetric square
//! kernel `sqr_low_limb::<N, L>` (`src/int/algos/sqr/sqr_low_limb.rs`), an
//! O(N²/2) low-half square on the tier's WORK integer:
//!
//!   * D616  → work integer `Int<128>` (N = 128 u64 limbs)
//!   * D924  → work integer `Int<192>` (N = 192 u64 limbs)
//!   * D1232 → work integer `Int<256>` (N = 256 u64 limbs)
//!
//! (The decimal STORAGE widths 32 / 48 / 64 limbs are NOT where this square
//! runs; the Smith loop operates in the wider guard-digit work integer. Both
//! bands are benched so the `LimbSize` decision is not based on the wrong
//! width — mirroring the multiply sibling `mul_low_u128_ab`.)
//!
//! Candidate: `sqr_low_u128::<N>`, the same truncated-low symmetric square
//! packed into N/2 u128 limbs (the base-2^128 shape 0.4.4's u128-limb work
//! integer ran). Half the limbs, ≈¼ the partial products, at the cost of a
//! wider 128×128→256 inner step.
//!
//! DECISION RULE: u128 must win at the wide work widths (N = 128 / 192 / 256)
//! to justify routing `U128` in `int::policy::sqr_low`; carve any losing even
//! cell back to `U64` in that policy's `limb_size` arm.
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

/// Bench every width: the three wide-tier WORK integers (the decisive band)
/// and, for completeness, the three storage widths.
fn bench_sqr_low(c: &mut Criterion) {
    // --- decisive band: the exp/powf Smith-squaring work integers ---
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
    bench_sqr_low(&mut c);
    c.final_summary();
}
