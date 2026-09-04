// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Narrow-tier Tang natural logarithm — the table-reduced core for the
//! D18 / D38 (`Int<2>`-storage) `ln` family, evaluated in the same 256-bit
//! `Fixed` intermediate the Series path uses.
//!
//! # What this changes, and why it is a different ALGORITHM
//!
//! [`crate::algos::ln::ln_series_2limb::ln_fixed`] range-reduces to a
//! mantissa `m ∈ [1, 2)` and evaluates `ln(m) = 2·artanh(t)` with
//! `t = (m−1)/(m+1) ≤ 1/3`. Each artanh term buys `log10(3) ≈ 0.477`
//! decimal digits, so the series runs `≈ w/0.954` terms — LINEAR in the
//! working scale, and every term pays a 512-bit product plus a rescale by
//! `10^w`.
//!
//! Tang reduces one step further, against a baked table: pick
//! `c = 1 + idx/M` (`M = 128`) closest to `m`, and evaluate
//!
//! ```text
//! ln(m) = ln(c) + 2·artanh(t),   t = (m − c)/(m + c),   |t| ≤ 1/(2M+1)
//! ```
//!
//! with `ln(c) = L_idx` read from the table rather than computed. Now each
//! term buys `log10(257) ≈ 2.41` digits, so the term count falls by ~5x for
//! the same `w`. That is the whole difference: the series is the same
//! series, evaluated at a far smaller argument.
//!
//! # The table in a narrow build
//!
//! `ln_tang_table` is unconditional, but its array is sized by
//! `_wide-support`: the wide tiers compile the full 112-limb entry, a
//! narrow-only build compiles the leading 5 limbs of the SAME values
//! (5,160 bytes). Because the narrow array is a PREFIX rather than a
//! second computation, and the reader always takes the top `p` limbs, a
//! narrow build and a wide build return bit-identical results here.
//!
//! # Correctness shell
//!
//! This file holds ONLY the working-scale core. The exact `1.0` pin, the
//! linear `ln(1+x) ≈ x` band, the clear-of-tie single shot and the Ziv
//! escalation live once in `ln_series_2limb::ln_strict_raw_with`, which
//! this entry point calls with [`ln_tang_fixed`] as its core. Restating
//! that shell here would put the near-tie terminal in two places.

use crate::algos::ln::ln_series_2limb::{
    STRICT_GUARD, fixed_from_int256, ln_with_core, wide_ln2,
};
use crate::algos::support::fixed::Fixed;
use crate::algos::support::ln_tang_table::{
    LN_TANG_M, ln_table_entry_baked, ln_table_fits, ln_table_limbs_for,
};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Limb count of [`Work`], and so its bit width — declared together so the
/// two cannot drift apart.
const WORK_LIMBS: usize = 12;
const WORK_BITS: u64 = (WORK_LIMBS as u64) * 64;

/// The work integer the table reconstruction runs in.
///
/// [`ln_table_entry_baked`] forms `slot_hi · 10^w`, which spans
/// `64·ln_table_limbs_for(w) + w·log2(10)` bits. At the narrow family's
/// widest working scale (`SCALE 38 + STRICT_GUARD 30 = 68`) that is
/// `320 + 226 = 546` bits, so `Int<8>`'s 512 is NOT enough and `Int<12>`
/// (768) is.
///
/// That sizing was not obvious and was not eyeballed: an earlier revision
/// of this file used `Int<8>`, correctly, against a working scale of 48 —
/// and [`work_fits`] turned the move to a wider guard into a BUILD failure
/// rather than a silent 34-bit overflow of the product. Which is the
/// entire argument for it being a `const fn`.
type Work = Int<WORK_LIMBS>;

/// Whether the table reconstruction product fits [`Work`] at working
/// scale `w`.
///
/// A `const fn` on purpose: the ln family's working scale is
/// `SCALE + STRICT_GUARD`, const-foldable at every call, so a guard or
/// scale change that would overflow `Work` must fail the BUILD. The
/// alternative — discovering it as a wrong answer or a panic in release —
/// is the failure mode the build-max sizing bug already cost this
/// campaign once.
const fn work_fits(w: u32) -> bool {
    let product_bits = 64 * ln_table_limbs_for(w) as u64 + (w as u64) * 3322 / 1000 + 1;
    product_bits <= WORK_BITS
}

/// `ln` of a positive `working_value` at `working_scale`, by Tang table
/// reduction. Drop-in for `ln_series_2limb::ln_fixed`.
///
/// Range-reduces `v = 2^k · m` with `m ∈ [1, 2)`, snaps `m` to the nearest
/// table node `c = 1 + idx/M`, and returns
/// `k·ln2 + L_idx + 2·artanh((m−c)/(m+c))`.
pub(crate) fn ln_tang_fixed(working_value: Fixed, working_scale: u32) -> Fixed {
    debug_assert!(
        working_scale >= 7,
        "ln_tang_fixed: `c` is formed by an exact >>7, which needs 2^7 | 10^w"
    );
    let one = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let two = one.double();

    // ── Binary range reduction: v = 2^k · m, m ∈ [1, 2) ──────────────
    // Identical to the Series path — Tang replaces what happens AFTER
    // this, not the reduction itself.
    let mut k: i32 = working_value.bit_length() as i32 - one.bit_length() as i32;
    let mantissa = loop {
        let candidate = if k >= 0 {
            working_value.shr(k as u32)
        } else {
            working_value.shl((-k) as u32)
        };
        if candidate.ge_mag(two) {
            k += 1;
        } else if !candidate.ge_mag(one) {
            k -= 1;
        } else {
            break candidate;
        }
    };

    // ── Table node: idx = round((m − 1)·M), so c = 1 + idx/M ─────────
    // `m − 1 ∈ [0, 1)` so `idx ∈ [0, M]`; the clamp is defensive against
    // a boundary rounding, not an expected path.
    let node_index = mantissa
        .sub(one)
        .mul_u128(u128::from(LN_TANG_M))
        .round_to_nearest_int(working_scale)
        .clamp(0, i128::from(LN_TANG_M)) as u128;

    // `c = (M + idx)/M · 10^w`. M = 128 = 2^7 and `w >= 7`, so `10^w` is
    // divisible by 128 and the shift is EXACT — no rounding enters `c`,
    // which matters because `c` is the point the table value is pinned to.
    let node = one.mul_u128(u128::from(LN_TANG_M) + node_index).shr(7);

    // ── t = (m − c)/(m + c), |t| ≤ 1/(2M+1) ──────────────────────────
    let t = mantissa
        .sub(node)
        .div(mantissa.add(node), working_scale);

    // ── artanh(t) = t + t³/3 + t⁵/5 + … ──────────────────────────────
    // Same recurrence as the Series path; it simply terminates ~5x
    // sooner because |t| ≤ 1/257 rather than ≤ 1/3.
    let t_squared = t.mul(t, working_scale);
    let mut sum = t;
    let mut term = t;
    let mut term_index: u128 = 1;
    loop {
        term = term.mul(t_squared, working_scale);
        let contribution = term.div_small(2 * term_index + 1);
        if contribution.is_zero() {
            break;
        }
        sum = sum.add(contribution);
        term_index += 1;
        if term_index > 400 {
            break;
        }
    }
    let ln_node_offset = sum.double();

    // ── L_idx = ln(1 + idx/M) from the baked table ───────────────────
    // `idx == 0` is `ln(1) = 0`; the reader short-circuits it, but the
    // pow10 lookup is skipped here too so the common path stays cheap.
    let ln_node = if node_index == 0 {
        Fixed::ZERO
    } else {
        let pow10_w = crate::consts::pow10::dispatch::<Work>(working_scale);
        fixed_from_int256(ln_table_entry_baked::<Work>(
            working_scale,
            node_index as usize,
            pow10_w,
        ))
    };

    // ── k·ln2 + L_idx + 2·artanh(t) ──────────────────────────────────
    let ln_two = wide_ln2(working_scale);
    let k_ln_two = if k >= 0 {
        ln_two.mul_u128(k as u128)
    } else {
        ln_two.mul_u128((-k) as u128).neg()
    };
    k_ln_two.add(ln_node).add(ln_node_offset)
}

/// Strict `ln` at `SCALE` via the Tang core. `None` = out of storage range.
///
/// The two `const` assertions are the point of this wrapper: the working
/// scale is `SCALE + STRICT_GUARD`, known at compile time, so "this build's
/// table is too small" and "the reconstruction would overflow the work
/// integer" are BUILD errors here, not release panics or wrong digits at
/// run time. The runtime `assert!` inside `ln_table_entry_baked` remains
/// for callers whose working scale is genuinely dynamic — the Ziv walker
/// does not route through this path, but nothing structurally prevents a
/// future one.
#[inline]
#[must_use]
pub(crate) fn ln<const N: usize, const SCALE: u32>(
    raw: Int<2>,
    mode: RoundingMode,
) -> Option<Int<2>> {
    const {
        // GUARDED BY `N`, and the guard is load-bearing.
        //
        // `policy::ln::tang_routed::<N, SCALE>` is monomorphised for every
        // `(N, SCALE)` the crate instantiates, and the monomorphisation
        // collector walks its `match N` narrow arm even where `N` is a WIDE
        // width — the arm is unreachable at run time, but it is still
        // collected, so this function is instantiated at D230's SCALE 229 and
        // D307's 306. At SCALE 306 the working scale is 336, needing 19 limbs
        // and a 2,333-bit product: no `Work` width satisfies that, and an
        // unguarded assertion fires for a cell this kernel never runs at.
        //
        // That is not hypothetical. It is what broke golden 33884588763 on the
        // d230 and d307 shards, having passed `cargo check` — which stops
        // before codegen and never evaluates a `const {}` block at all.
        //
        // `N <= 2` is exactly the set of cells `tang_routed` can dispatch here,
        // so the bound is asserted over the domain it actually describes.
        if N <= 2 {
            assert_narrow_tang_fits(SCALE);
        }
    }
    ln_with_core::<SCALE>(raw, mode, ln_tang_fixed)
}

/// The narrow Tang core's compile-time width bounds at storage `scale`.
///
/// Call it ONLY from inside `const { if N <= 2 { … } }`. Both bounds hold
/// for every cell the narrow arm serves and fail loudly for cells it does
/// not — see [`ln`] for why the `N` guard is not optional.
///
/// Shared by the `ln` and `log` narrow entries so the bound is stated once;
/// two copies would be two chances to get the domain wrong, which is the
/// mistake that produced the E0080 in the first place.
pub(crate) const fn assert_narrow_tang_fits(scale: u32) {
    assert!(
        ln_table_fits(scale + STRICT_GUARD),
        "ln Tang: this build's ln_tang_table is too narrow for SCALE + STRICT_GUARD"
    );
    assert!(
        work_fits(scale + STRICT_GUARD),
        "ln Tang: slot_hi * 10^w overflows the Work integer at SCALE + STRICT_GUARD"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tang must agree with the Series kernel it is an alternative to.
    ///
    /// Both are correctly rounded, so where the shell resolves on the
    /// single shot they must produce the SAME storage integer — this is
    /// the bit-identity wall an algorithm swap has to clear. Anchors span
    /// the reduction: exact powers of two (mantissa 1, the degenerate
    /// node-0 case), values landing between nodes, and values either side
    /// of 1.
    #[test]
    fn tang_matches_series_at_d38_s19() {
        const S: u32 = 19;
        let one = 10_i128.pow(S);
        let cases = [
            2 * one,          // exact power of two: m = 1, idx = 0
            7 * one,          // m = 1.75 — the bbc `ln_nd` operand
            3 * one,          // m = 1.5, exactly on node 64
            one / 2,          // 0.5, k = -1
            one + one / 3,    // 1.333…
            9 * one + one / 7,
        ];
        for raw in cases {
            let series = crate::algos::ln::ln_series_2limb::ln::<S>(
                Int::<2>::from_i128(raw),
                RoundingMode::HalfToEven,
            );
            let tang = ln::<2, S>(Int::<2>::from_i128(raw), RoundingMode::HalfToEven);
            assert_eq!(tang, series, "ln raw={raw}");
        }
    }

    /// The node snap must be exact: `c` is formed by an exact `>>7`, so
    /// `ln_tang_fixed` at a mantissa sitting ON a node must give a zero
    /// artanh argument and return the table value unmodified.
    #[test]
    fn node_snap_is_exact_on_a_table_node() {
        const W: u32 = 49; // SCALE 19 + STRICT_GUARD 30
        let one = Fixed {
            negative: false,
            mag: Fixed::pow10(W),
        };
        // m = 1 + 64/128 = 1.5 exactly, the midpoint node.
        let m = one.mul_u128(192).shr(7);
        let tang = ln_tang_fixed(m, W);
        let series = crate::algos::ln::ln_series_2limb::ln_fixed(m, W);
        // Compare the WORKING-SCALE magnitudes, not the rounded integer:
        // both values are ~0.405, so `round_to_nearest_int` maps any
        // disagreement below 0.5 to zero and the test would pass while the
        // cores differed in the 3rd digit. The two compute the same real
        // number by different reductions, so they may differ only in the
        // last few working units out of 10^29.
        let diff = tang.sub(series);
        assert!(
            diff.mag[1] == 0 && diff.mag[0] < 1_000,
            "node-snap ln(1.5): Tang and Series disagree by {:?} working units at w={W}",
            diff.mag
        );
    }
}
