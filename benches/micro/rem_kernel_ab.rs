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
    dec_rem_int_layer, dec_rem_native, int_from_mag_limbs, int_wrapping_rem_slice, rem_native,
    rem_schoolbook, rem_via_div_rem,
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
///
/// A macro, not a generic fn: `dec_rem_int_layer` carries a `pub(crate)`
/// `ComputeInt` bound a bench (a separate crate) cannot name, so the body
/// must monomorphise at a concrete width `$n` — where `Int<$n>: ComputeInt`
/// is discharged by the impl — rather than be checked for all `N`. Mirrors
/// `hypot_ab`'s `hypot_cell!`.
macro_rules! dec_rem_cell {
    ($c:expr, $n:literal, $label:literal) => {{
        for p in operand_set::<$n>() {
            assert_eq!(
                dec_rem_native::<$n>(p.clone().a, p.clone().b),
                dec_rem_int_layer::<$n>(p.clone().a, p.clone().b),
                "dec native vs int_layer disagree {} {}",
                $label,
                p.label
            );
        }
        compare_all(
            $c,
            concat!("dec_rem/", $label),
            |p: &Pair<$n>| p.label.to_string(),
            operand_set::<$n>(),
            vec![
                (
                    "native",
                    (|p: Pair<$n>| dec_rem_native::<$n>(p.a, p.b)) as fn(Pair<$n>) -> Int<$n>,
                ),
                ("int_layer", |p: Pair<$n>| dec_rem_int_layer::<$n>(p.a, p.b)),
            ],
        );
    }};
}

/// Build `k * 10^scale` in an `N`-limb magnitude (the exact live full_matrix
/// decimal operand: a small integer scaled by `10^SCALE`). Iterative *10 so
/// no const-eval of `10^SCALE` is needed.
fn k_times_pow10<const N: usize>(k: u64, scale: u32) -> Int<N> {
    let mut limbs = [0u64; N];
    limbs[0] = k;
    for _ in 0..scale {
        let mut carry: u128 = 0;
        for l in limbs.iter_mut() {
            let v = (*l as u128) * 10 + carry;
            *l = v as u64;
            carry = v >> 64;
        }
    }
    int_from_mag_limbs::<N>(&limbs)
}

/// Wide decimal-remainder dispatch seam (`src/policy/rem.rs` -> `rem_int_layer`
/// for `N >= 3`). The regression-recovery A/B: the recovered operator/Knuth
/// `rem_int_layer` vs the OLD `Int::wrapping_rem` shift-subtract path it
/// replaced. Two shapes:
///   * `short_circuit` -- the live full_matrix operand `2 * 10^SCALE %
///     1 * 10^SCALE` (small quotient over a wide `10^SCALE` divisor): the
///     shape that exposed the ~25x regression.
///   * `balanced` -- two full-width random magnitudes (general divmod): the
///     case that must NOT regress.
macro_rules! dec_rem_wide_cell {
    ($c:expr, $n:literal, $label:literal, $scale:expr) => {{
        let two = k_times_pow10::<$n>(2, $scale);
        let one = k_times_pow10::<$n>(1, $scale);
        let bal_a = synth::<$n>(7919, $n);
        let bal_b = synth::<$n>(104729, $n);
        // correctness: new path agrees with the old wrapping_rem on both shapes.
        assert_eq!(
            dec_rem_int_layer::<$n>(two, one),
            int_wrapping_rem_slice::<$n>(two, one),
            "dec rem short_circuit disagree {}",
            $label
        );
        assert_eq!(
            dec_rem_int_layer::<$n>(bal_a, bal_b),
            int_wrapping_rem_slice::<$n>(bal_a, bal_b),
            "dec rem balanced disagree {}",
            $label
        );

        let inputs = vec![
            Pair { label: "short_circuit", a: two, b: one },
            Pair { label: "balanced", a: bal_a, b: bal_b },
        ];
        compare_all(
            $c,
            concat!("dec_rem_wide/", $label),
            |p: &Pair<$n>| p.label.to_string(),
            inputs,
            vec![
                (
                    "operator_knuth",
                    (|p: Pair<$n>| dec_rem_int_layer::<$n>(p.a, p.b)) as fn(Pair<$n>) -> Int<$n>,
                ),
                ("old_wrapping_rem", |p: Pair<$n>| int_wrapping_rem_slice::<$n>(p.a, p.b)),
            ],
        );
    }};
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        Int<1> => |c: &mut Criterion| compare_narrow::<1>(c, "Int64_D18"),
        Int<2> => |c: &mut Criterion| compare_narrow::<2>(c, "Int128_D38"),
        Int<1> => |c: &mut Criterion| dec_rem_cell!(c, 1, "D18"),
        Int<2> => |c: &mut Criterion| dec_rem_cell!(c, 2, "D38"),
        Int<4> => |c: &mut Criterion| compare_wide::<4>(c, "Int256_D76"),
        Int<8> => |c: &mut Criterion| compare_wide::<8>(c, "Int512_D153"),
        Int<64> => |c: &mut Criterion| compare_wide::<64>(c, "Int4096_D1232"),
        // Wide decimal-rem regression recovery: operator/Knuth vs the old
        // wrapping_rem shift-subtract, on the live `k * 10^SCALE` shape.
        Int<16> => |c: &mut Criterion| dec_rem_wide_cell!(c, 16, "D307_s153", 153),
        Int<32> => |c: &mut Criterion| dec_rem_wide_cell!(c, 32, "D616_s308", 308),
        Int<48> => |c: &mut Criterion| dec_rem_wide_cell!(c, 48, "D924_s462", 462),
        Int<64> => |c: &mut Criterion| dec_rem_wide_cell!(c, 64, "D1232_s616", 616),
    );
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
