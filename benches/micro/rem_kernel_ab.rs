// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the remainder policy (`src/int/policy/rem.rs`).
//!
//! Decision being modelled: per storage width `N`, which registered `rem`
//! `Algorithm` arm is fastest:
//!
//! - `native`     -> `rem_native` (hardware `u128 % u128`, valid `N <= 2`).
//! - `via_div_rem`-> `rem_via_div_rem` (the general division-policy path).
//! - `schoolbook` -> `rem_schoolbook` (the binary shift-subtract baseline).
//!
//! For each width the bench feeds three operand pairs (low / mid / high
//! magnitude). Inputs and outputs are `black_box`-guarded by the harness so
//! no kernel const-folds away.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench rem_kernel_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::__bench_internals::{
    dec_rem_int_layer, dec_rem_native, int_from_mag_limbs, rem_native, rem_schoolbook,
    rem_via_div_rem,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};
use crate::ab_sweep as decimal_scaled_ab_sweep;

#[derive(Clone)]
struct Pair<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

/// Deterministic limb fill.
fn synth<const N: usize>(seed: u64, used: usize) -> Int<N> {
    let mut mag = [0u64; N];
    let used = used.min(N);
    for i in 0..used {
        mag[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(i as u64 * 0x1357_9BDF)
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    // ensure the top used limb is non-zero so the effective width is `used`.
    if used > 0 && mag[used - 1] == 0 {
        mag[used - 1] = 0x8000_0000_0000_0001;
    }
    int_from_mag_limbs::<N>(&mag)
}

/// Operand set for width N: dividend uses all N limbs; divisor uses about
/// half the limbs (a representative "both multi-limb" decimal-rem shape) so
/// the divide engine sees a real multi-limb divisor at wide N.
fn operand_set<const N: usize>() -> Vec<Pair<N>> {
    let half = (N / 2).max(1);
    vec![
        Pair { label: "small_div", a: synth::<N>(7, N), b: synth::<N>(3, 1.min(N)) },
        Pair { label: "mid_div", a: synth::<N>(1009, N), b: synth::<N>(13, half) },
        Pair { label: "near_div", a: synth::<N>(7919, N), b: synth::<N>(104729, N) },
    ]
}

fn native_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_native::<N>(p.a, p.b)
}
fn viadiv_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_via_div_rem::<N>(p.a, p.b)
}
fn school_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_schoolbook::<N>(p.a, p.b)
}


/// Narrow widths (N <= 2): native is a candidate.
fn compare_narrow<const N: usize>(c: &mut Criterion, label: &str) {
    for p in operand_set::<N>() {
        assert_eq!(native_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "native vs via_div_rem disagree {label} {}", p.label);
    }
    compare_all(
        c,
        &format!("rem_kernel/{label}"),
        |p: &Pair<N>| p.label.to_string(),
        operand_set::<N>(),
        vec![
            ("native", native_run::<N> as fn(Pair<N>) -> Int<N>),
            ("via_div_rem", viadiv_run::<N>),
        ],
    );
}

/// Wide widths (N >= 3): native is invalid; compare via_div_rem vs the
/// shift-subtract schoolbook baseline.
fn compare_wide<const N: usize>(c: &mut Criterion, label: &str) {
    for p in operand_set::<N>() {
        assert_eq!(school_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "schoolbook vs via_div_rem disagree {label} {}", p.label);
    }
    compare_all(
        c,
        &format!("rem_kernel/{label}"),
        |p: &Pair<N>| p.label.to_string(),
        operand_set::<N>(),
        vec![
            ("via_div_rem", viadiv_run::<N> as fn(Pair<N>) -> Int<N>),
            ("schoolbook", school_run::<N>),
        ],
    );
}

/// Decimal-rem dispatch seam (`src/policy/rem.rs` -> `rem_int_layer`).
/// Narrow widths only: native hardware `%` vs the generic int-layer path.
fn compare_dec_rem<const N: usize>(c: &mut Criterion, label: &str) {
    for p in operand_set::<N>() {
        assert_eq!(dec_rem_native::<N>(p.clone().a, p.clone().b),
            dec_rem_int_layer::<N>(p.clone().a, p.clone().b),
            "dec native vs int_layer disagree {label} {}", p.label);
    }
    compare_all(
        c,
        &format!("dec_rem/{label}"),
        |p: &Pair<N>| p.label.to_string(),
        operand_set::<N>(),
        vec![
            ("native", (|p: Pair<N>| dec_rem_native::<N>(p.a, p.b)) as fn(Pair<N>) -> Int<N>),
            ("int_layer", |p: Pair<N>| dec_rem_int_layer::<N>(p.a, p.b)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        Int<1> => |c: &mut Criterion| compare_narrow::<1>(c, "Int64_D18"),
        Int<2> => |c: &mut Criterion| compare_narrow::<2>(c, "Int128_D38"),
        Int<1> => |c: &mut Criterion| compare_dec_rem::<1>(c, "D18"),
        Int<2> => |c: &mut Criterion| compare_dec_rem::<2>(c, "D38"),
        Int<4> => |c: &mut Criterion| compare_wide::<4>(c, "Int256_D76"),
        Int<8> => |c: &mut Criterion| compare_wide::<8>(c, "Int512_D153"),
        Int<64> => |c: &mut Criterion| compare_wide::<64>(c, "Int4096_D1232"),
    );
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
