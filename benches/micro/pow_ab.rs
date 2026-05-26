// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam N-way A/B for the integer exponentiation policy
//! (`src/int/policy/pow.rs`).
//!
//! Decision being mapped: per storage width `N` and exponent magnitude,
//! which kernel computes `base^exp mod 2^BITS` fastest while remaining
//! bit-identical to the reference:
//!
//! - `square_and_multiply` -> the WIRED kernel: binary exponentiation by
//!   squaring (O(log exp) limb operations — one square per exponent bit plus
//!   one multiply per set bit).
//! - `schoolbook`          -> the registered reference: naive repeated
//!   multiply (exp - 1 sequential multiplies, O(exp) limb operations).
//!
//! Both are `const fn`, both compute via the const `sqr_low_fixed` /
//! `mul_low_fixed` u64 kernels, and both are numerically identical modulo
//! `2^BITS`. The CONTINUOUS axis here is the EXPONENT (int pow has no scale
//! axis; the base magnitude does not change the operation count — only the
//! exponent does). At `exp == 2` both perform one product (a tie); the
//! square-and-multiply edge opens as the exponent grows. We sweep:
//!
//! - `e2`, `e3`, `e4`, `e7`, `e10`, `e16`, `e31` — small fixed exponents
//!   spanning the range `pow` is used with (root iterations: `k`, `k-1` ≤
//!   ~10) up past the worst small-exp case.
//!
//! Every candidate is asserted bit-identical to the reference across the
//! exponent spread at every width BEFORE timing — the validity wall. The
//! harness `black_box`-guards inputs/outputs.
//!
//! Run: `cargo bench --features "wide x-wide xx-wide bench-alt" --bench pow_ab`

use criterion::Criterion;
use decimal_scaled::Uint;
use decimal_scaled::__bench_internals::{pow_schoolbook, pow_square_and_multiply};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// Deterministic limb fill for a `Uint<N>` base.
fn synth<const N: usize>(seed: u64) -> Uint<N> {
    let mut mag = [0u64; N];
    for (i, limb) in mag.iter_mut().enumerate() {
        *limb = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(i as u64 * 0x1357_9BDF)
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    Uint::<N>::from_limbs(mag)
}

/// A `(base, exp)` input at a labelled exponent class.
#[derive(Clone)]
struct PowIn<const N: usize> {
    label: &'static str,
    base: Uint<N>,
    exp: u32,
}

fn inputs<const N: usize>() -> Vec<PowIn<N>> {
    let exps: &[(&'static str, u32)] = &[
        ("e2", 2),
        ("e3", 3),
        ("e4", 4),
        ("e7", 7),
        ("e10", 10),
        ("e16", 16),
        ("e31", 31),
    ];
    exps.iter()
        .map(|&(label, exp)| PowIn { label, base: synth::<N>(0xABCD ^ exp as u64), exp })
        .collect()
}

fn run_sqm<const N: usize>(i: PowIn<N>) -> Uint<N> {
    pow_square_and_multiply::<N>(i.base, i.exp)
}
fn run_schoolbook<const N: usize>(i: PowIn<N>) -> Uint<N> {
    pow_schoolbook::<N>(i.base, i.exp)
}

fn cell<const N: usize>(c: &mut Criterion, label: &str) {
    for i in inputs::<N>() {
        assert_eq!(
            run_sqm::<N>(i.clone()),
            run_schoolbook::<N>(i.clone()),
            "pow square_and_multiply vs schoolbook {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("pow/{label}"),
        |i: &PowIn<N>| i.label.to_string(),
        inputs::<N>(),
        vec![
            ("square_and_multiply", Box::new(run_sqm::<N>) as Box<dyn Fn(PowIn<N>) -> Uint<N>>),
            ("schoolbook", Box::new(run_schoolbook::<N>)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    cell::<1>(c, "n1_D18");
    cell::<2>(c, "n2_D38");
    cell::<3>(c, "n3_D57");
    cell::<4>(c, "n4_D76");
    cell::<6>(c, "n6_D115");
    cell::<8>(c, "n8_D153");
    cell::<12>(c, "n12_D230");
    cell::<16>(c, "n16_D307");
    cell::<24>(c, "n24_D462");
    cell::<32>(c, "n32_D616");
    cell::<48>(c, "n48_D924");
    cell::<64>(c, "n64_D1232");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
