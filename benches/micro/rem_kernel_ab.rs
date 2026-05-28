// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the remainder policy (`src/int/policy/rem.rs`).
//!
//! Decision being modelled: per storage width `N` × operand MAGNITUDE, which
//! registered `rem` `Algorithm` arm is fastest while bit-identical to the
//! reference:
//!
//! - `native`      -> `rem_native` (hardware `u128 % u128`, valid `N <= 2`).
//! - `small_fast`  -> `rem_small_fast` (value-gated single-word hardware `%`
//!   with a `via_div_rem` fallback — valid at EVERY `N`; the recovery of
//!   v0.4.4's "Fast Path A").
//! - `via_div_rem` -> `rem_via_div_rem` (the general division-policy path —
//!   the current baseline for `N >= 3`).
//! - `schoolbook`  -> `rem_schoolbook` (the binary shift-subtract baseline).
//!
//! The KEY axis for rem is operand magnitude (the bbc regression is at
//! SCALE-0 = small/integer operands at wide tiers). Per width we feed
//! several magnitude shapes: `s0_word` (both operands fit one word — the
//! bbc scale-0 shape), `mid_div` (half-width divisor), `full_div`
//! (full-width both). Inputs/outputs are `black_box`-guarded by the harness.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench rem_kernel_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::__bench_internals::{
    dec_rem_int_layer, dec_rem_int_layer_divmod, dec_rem_native, int_from_mag_limbs,
    int_wrapping_rem_slice, rem_native, rem_native_direct, rem_schoolbook, rem_small_fast,
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

/// Deterministic limb fill over the first `used` limbs.
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

/// Magnitude-sweep operand set for width N. The magnitude axis is what rem
/// keys on (the scale-0 bbc regression is at small/integer operands):
///   * `s0_word`  — both operands fit ONE 64-bit limb (the scale-0 shape).
///   * `s0_u128`  — both operands fit a 128-bit word (two limbs) — the wider
///                  small-operand case the single-word fast path still covers.
///   * `mid_div`  — full-width dividend, ~half-width divisor.
///   * `full_div` — both operands full N-limb (the case that must NOT regress).
fn operand_set<const N: usize>() -> Vec<Pair<N>> {
    let half = (N / 2).max(1);
    let mut v = vec![
        Pair { label: "s0_word", a: synth::<N>(7, 1.min(N)), b: synth::<N>(3, 1.min(N)) },
    ];
    if N >= 2 {
        v.push(Pair { label: "s0_u128", a: synth::<N>(1009, 2.min(N)), b: synth::<N>(13, 1.min(N)) });
    }
    if N >= 3 {
        v.push(Pair { label: "mid_div", a: synth::<N>(7919, N), b: synth::<N>(101, half) });
    }
    v.push(Pair { label: "full_div", a: synth::<N>(104729, N), b: synth::<N>(7919, N) });
    v
}

fn native_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_native::<N>(p.a, p.b)
}
fn native_direct_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_native_direct::<N>(p.a, p.b)
}
fn smallfast_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_small_fast::<N>(p.a, p.b)
}
fn viadiv_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_via_div_rem::<N>(p.a, p.b)
}
fn school_run<const N: usize>(p: Pair<N>) -> Int<N> {
    rem_schoolbook::<N>(p.a, p.b)
}

/// Narrow widths (N <= 2): native / native_direct / small_fast are all
/// candidates against via_div_rem.
fn compare_narrow<const N: usize>(c: &mut Criterion, label: &str) {
    for p in operand_set::<N>() {
        assert_eq!(native_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "native vs via_div_rem disagree {label} {}", p.label);
        assert_eq!(native_direct_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "native_direct vs via_div_rem disagree {label} {}", p.label);
        assert_eq!(smallfast_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "small_fast vs via_div_rem disagree {label} {}", p.label);
    }
    compare_all(
        c,
        &format!("rem_kernel/{label}"),
        |p: &Pair<N>| p.label.to_string(),
        operand_set::<N>(),
        vec![
            ("native", native_run::<N> as fn(Pair<N>) -> Int<N>),
            ("native_direct", native_direct_run::<N>),
            ("small_fast", smallfast_run::<N>),
            ("via_div_rem", viadiv_run::<N>),
        ],
    );
}

/// Wide widths (N >= 3): native is invalid (magnitude exceeds one u128 for
/// the full-width shape). Candidates are small_fast (value-gated word fast
/// path + fallback), via_div_rem (baseline) and the schoolbook reference.
fn compare_wide<const N: usize>(c: &mut Criterion, label: &str) {
    for p in operand_set::<N>() {
        assert_eq!(smallfast_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "small_fast vs via_div_rem disagree {label} {}", p.label);
        assert_eq!(school_run::<N>(p.clone()), viadiv_run::<N>(p.clone()),
            "schoolbook vs via_div_rem disagree {label} {}", p.label);
    }
    compare_all(
        c,
        &format!("rem_kernel/{label}"),
        |p: &Pair<N>| p.label.to_string(),
        operand_set::<N>(),
        vec![
            ("small_fast", smallfast_run::<N> as fn(Pair<N>) -> Int<N>),
            ("via_div_rem", viadiv_run::<N>),
            ("schoolbook", school_run::<N>),
        ],
    );
}

/// Decimal-rem dispatch seam (`src/policy/rem.rs` -> `rem_int_layer`).
/// Narrow widths only: native hardware `%` vs the generic int-layer path.
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
/// for `N >= 3`). The recovered operator/Knuth `rem_int_layer` vs the OLD
/// `Int::wrapping_rem` shift-subtract path it replaced.
macro_rules! dec_rem_wide_cell {
    ($c:expr, $n:literal, $label:literal, $scale:expr) => {{
        let two = k_times_pow10::<$n>(2, $scale);
        let one = k_times_pow10::<$n>(1, $scale);
        let bal_a = synth::<$n>(7919, $n);
        let bal_b = synth::<$n>(104729, $n);
        // scale-0 small-operand shape: the EXACT bbc cell (`D<N>::<0>(2) %
        // D<N>::<0>(1)`) — both operands are tiny single-word integers, the
        // dominant scale-0 decimal-`rem` regression.
        let s0_two = k_times_pow10::<$n>(2, 0);
        let s0_one = k_times_pow10::<$n>(1, 0);
        // The EXACT scaled bbc `rem` cell: `2.0 % 3.5` at this SCALE, i.e.
        // `2·10^scale % 35·10^(scale-1)`. The dividend `|2·10^s|` is SMALLER
        // than the divisor `|3.5·10^s|`, so `x % b == x` — and the scaled
        // divisor crosses the 128-bit line at s38+, so the u128 fast path
        // MISSES and (before the magnitude short-circuit) it fell into a full
        // multi-limb Knuth divmod. This is the dominant rem regression cell.
        let bbc_x = k_times_pow10::<$n>(2, $scale);
        let bbc_b = if $scale >= 1 {
            k_times_pow10::<$n>(35, $scale - 1)
        } else {
            k_times_pow10::<$n>(3, 0)
        };
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
        // The fast path must be bit-identical to the divmod-only path on the
        // s0 small shape (it short-circuits) AND the balanced shape (it
        // doesn't) — the validity wall.
        assert_eq!(
            dec_rem_int_layer::<$n>(s0_two, s0_one),
            dec_rem_int_layer_divmod::<$n>(s0_two, s0_one),
            "dec rem fastpath s0 disagree {}",
            $label
        );
        assert_eq!(
            dec_rem_int_layer::<$n>(bal_a, bal_b),
            dec_rem_int_layer_divmod::<$n>(bal_a, bal_b),
            "dec rem fastpath balanced disagree {}",
            $label
        );
        // The magnitude short-circuit on the real bbc `2.0 % 3.5` cell must be
        // bit-identical to the pure divmod (both yield `rem == dividend`).
        assert_eq!(
            dec_rem_int_layer::<$n>(bbc_x, bbc_b),
            dec_rem_int_layer_divmod::<$n>(bbc_x, bbc_b),
            "dec rem bbc_xb (2.0 % 3.5) disagree {}",
            $label
        );

        let inputs = vec![
            Pair { label: "s0_small", a: s0_two, b: s0_one },
            Pair { label: "bbc_xb", a: bbc_x, b: bbc_b },
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
                    "fastpath",
                    (|p: Pair<$n>| dec_rem_int_layer::<$n>(p.a, p.b)) as fn(Pair<$n>) -> Int<$n>,
                ),
                ("divmod_only", |p: Pair<$n>| dec_rem_int_layer_divmod::<$n>(p.a, p.b)),
                ("old_wrapping_rem", |p: Pair<$n>| int_wrapping_rem_slice::<$n>(p.a, p.b)),
            ],
        );
    }};
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        // Narrow tiers: native / native_direct / small_fast vs via_div_rem.
        Int<1> => |c: &mut Criterion| compare_narrow::<1>(c, "Int64_D18"),
        Int<2> => |c: &mut Criterion| compare_narrow::<2>(c, "Int128_D38"),
        // Wide tiers: full magnitude sweep, small_fast vs via_div_rem vs school.
        Int<3> => |c: &mut Criterion| compare_wide::<3>(c, "Int192_D57"),
        Int<4> => |c: &mut Criterion| compare_wide::<4>(c, "Int256_D76"),
        Int<6> => |c: &mut Criterion| compare_wide::<6>(c, "Int384_D115"),
        Int<8> => |c: &mut Criterion| compare_wide::<8>(c, "Int512_D153"),
        Int<12> => |c: &mut Criterion| compare_wide::<12>(c, "Int768_D230"),
        Int<16> => |c: &mut Criterion| compare_wide::<16>(c, "Int1024_D307"),
        Int<24> => |c: &mut Criterion| compare_wide::<24>(c, "Int1536_D462"),
        Int<32> => |c: &mut Criterion| compare_wide::<32>(c, "Int2048_D616"),
        Int<48> => |c: &mut Criterion| compare_wide::<48>(c, "Int3072_D924"),
        Int<64> => |c: &mut Criterion| compare_wide::<64>(c, "Int4096_D1232"),
        // Decimal-rem dispatch seam (kept).
        Int<1> => |c: &mut Criterion| dec_rem_cell!(c, 1, "D18"),
        Int<2> => |c: &mut Criterion| dec_rem_cell!(c, 2, "D38"),
        // Wide decimal-rem cells: the s0_small input is the bbc scale-0
        // regression cell (D57 5.34× worst), the short_circuit/balanced
        // inputs guard the non-small shapes against regression. The scale
        // arg drives the short_circuit operand size only.
        Int<3> => |c: &mut Criterion| dec_rem_wide_cell!(c, 3, "D57_s28", 28),
        Int<4> => |c: &mut Criterion| dec_rem_wide_cell!(c, 4, "D76_s38", 38),
        Int<6> => |c: &mut Criterion| dec_rem_wide_cell!(c, 6, "D115_s57", 57),
        Int<8> => |c: &mut Criterion| dec_rem_wide_cell!(c, 8, "D153_s76", 76),
        Int<12> => |c: &mut Criterion| dec_rem_wide_cell!(c, 12, "D230_s115", 115),
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
