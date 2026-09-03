// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Bespoke `atan_strict` kernel slot for `D57<SCALE>`, routed across the
//! tier's FULL scale range (`SCALE ∈ 0..=56`).
//!
//! The module carries NO precondition suffix because it has no scale
//! precondition: it is valid and routed across the tier's whole range. It
//! was once `atan_tang_3limb_s44_56`, named for a lower edge that was
//! asserted rather than measured; the bisection (below) dropped that edge
//! to 0 and the suffix was removed with it.
//!
//! At deep storage scales the wide-tier `atan_fixed` runs an
//! `O(log working_scale)` halving chain (each `atan(x) = 2·atan(x/(1+√(1+x²)))`
//! costs one wide sqrt + one wide div + one wide mul) followed by a
//! Taylor evaluation on the post-halving residual. With `GUARD = 30` the
//! routed range's `working_scale = SCALE + GUARD` is `30..=86`, all of
//! which lands in the `working_scale < 110` arm of the halving count in
//! [`crate::algos::trig::trig_generic::atan_fixed`] — so the chain the
//! generic path runs is **6** halvings from `SCALE = 30` up (5 below it),
//! not the 7 an earlier draft of this header claimed (7 needs
//! `working_scale >= 110`, i.e. `SCALE >= 80`, which D57 cannot reach: its
//! `MAX_SCALE` is 56). Those wide sqrts before the Taylor loop
//! runs ~30 terms — and every iteration of every
//! kernel goes through the same `Int<16> / Int<16>` Knuth divide that
//! dominates wide arithmetic at this width. This kernel collapses the
//! halving chain into a single table lookup using the atan addition
//! formula:
//!
//! ```text
//! atan(x) = atan(c_j) + atan(y),  c_j = j / M,  j ∈ [0, M),
//!                                 y    = (x − c_j) / (1 + c_j · x).
//! ```
//!
//! With `M = 512` and `x ∈ [0, 1]` (the existing reciprocal-fold for
//! `|x| > 1` is preserved), choosing `j = round(x · M)` gives
//! `|y| ≤ 1/(2M) = 1/1024 ≈ 9.8·10⁻⁴`. The Taylor remainder then
//! converges in ~15 terms at `working_scale ≤ 87`, vs the 5–6 halvings +
//! ~30 terms the generic path runs.
//!
//! The slot is exposed through `crate::policy::trig::forward::select_atan`
//! for every `SCALE ∈ 0..=56`. It is reached through `select_atan`, NOT the
//! shared `select`: at this tier the `Tang` arm realises as a different
//! kernel per function (this one for atan, `sincos_tang` for sin/cos), so
//! the two bands are separate empirical questions and do not share an edge.
//!
//! # The lower band edge (44) was ASSERTED, not measured — and has now
//! # been bisected away
//!
//! An earlier draft of this header justified the edge by claiming lower
//! scales "keep using the generic `atan_series` which is already cheaper
//! there (fewer halvings, faster Knuth dispatch)". Neither half survives
//! reading the code it refers to:
//!
//! - **"fewer halvings"** — the count in
//!   [`crate::algos::trig::trig_generic::atan_fixed`] steps only at
//!   `working_scale = 60`, i.e. `SCALE = 30`. Every scale in `30..=56`
//!   runs the SAME 6 halvings, so the generic path at `SCALE = 43` does
//!   exactly the work it does at `SCALE = 44` where this kernel takes
//!   over. Below 30 it runs 5 — one fewer, not "fewer" in any sense that
//!   scales with the gap.
//! - **"faster Knuth dispatch"** — the D57 trig work integer is `Int<16>`
//!   at EVERY scale of the tier (`types::widths`), so the divide width is
//!   scale-INDEPENDENT. Only the operand magnitudes shrink.
//!
//! Meanwhile this kernel gets CHEAPER as SCALE falls: its table
//! reconstruction reads a `ceil((w·3.322 + 64)/64)`-limb prefix
//! (`support::atan_tang_table::reconstruct`) — 3 limbs at `SCALE = 0`
//! against 6 at `SCALE = 56` — and its Taylor loop is bounded by
//! `working_scale`. It pays NO halvings at any scale.
//!
//! So the band edge was re-bisected downward rather than trusted, and it
//! came back at the SCALE floor: the arm is now `(3, 0..=56)`. This matches
//! the precedent in the same matcher — the D462 arm `(24, 0..=461)`, the
//! one trig band whose edge had ever actually been bisected, which also
//! came back as the FULL scale range.
//!
//! The bisection (`benches/micro/atan_d57_band_bisect.rs`, three
//! independent `-Core 22` pinned runs) put this kernel ahead at every
//! probed scale in `0..=43` — the newly claimed region — 3/3, worst cell
//! s0 at 1.68x. `18..=22` was ranked THREE-way rather than interpolated,
//! because production runs a narrow-GUARD Series there (GUARD=10, working
//! scale `SCALE + 10`) while this kernel is fixed at `SCALE + 30` and so
//! carries ~20 more working digits: it still wins (1.66x / 1.65x / 1.56x
//! over `narrow_g10` at s18), so the win region is continuous through the
//! band rather than stepping over a hole in it.
//!
//! Nothing below 44 was a validity limit: `atan_tang_table::reconstruct`
//! asserts only an UPPER bound (`p_full <= ATAN_TANG_LIMBS`) and takes
//! `p = p_full.max(1)`, and the baked value is read as a high-limb PREFIX
//! of a 7168-bit expansion — so a lower scale reads a SHORTER prefix and
//! is strictly cheaper. Bit-identity to the canonical `atan_series` was
//! walled over 12,224 comparisons (all 57 scales x up to 27 inputs x all
//! 8 rounding modes), 0 mismatches.
//!
//! ## Correctness
//!
//! Error budget at `working_scale` (in LSB-of-`working_scale`):
//!
//! - Reciprocal-fold `1/x` (when `|x| > 1`): ≤ 0.5 LSB.
//! - Table index quantisation `c_j = j/M`: exact (integer division
//!   of `one(working_scale)` by small `M`, ≤ 0.5 LSB).
//! - `y = (x − c_j) / (1 + c_j · x)`: 1 mul + 1 div + 2 add/sub
//!   → ≤ 1.5 LSB.
//! - Taylor on `|y| ≤ 1/(2M) ≈ 10⁻³`: ~15 rounded muls → ≤ 7.5 LSB.
//! - Table lookup `atan(c_j)`: precomputed by the generic
//!   `atan_fixed` at the same `working_scale`, ≤ 1 LSB after rounding.
//! - One outer add (`atan(c_j) + atan(y)`): ≤ 0.5 LSB.
//!
//! Total ≤ ~11 LSB-of-`working_scale` = ~11·10⁻³⁰ at storage scale. The strict
//! contract requires ≤ 0.5 LSB-of-storage = 0.5·10⁻ᴿᴱ — a margin of
//! 28+ orders of magnitude even at `SCALE = 57`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::algos::support::atan_tang_table;
use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Table size — number of `atan(j / M)` entries per working scale.
/// Power of two so the index quantisation step `1/M` keeps the cheap
/// integer-division path. Larger M shrinks the post-table residual
/// `|y| ≤ 1/(2M)` and so shaves Taylor iterations.
///
/// Mirrors the tuning from the D57 exp lookup (the collapsed
/// per-tier 45..=56 exp table): same `Int<16>`-wide work integer,
/// same Knuth-dispatch arithmetic cost per slot. `M = 512` strikes
/// the balance — the post-table Taylor remainder is small enough that
/// the inner loop runs in ~15 iterations, and a call touches exactly
/// one table slot (computed on the stack via [`table_entry`]).
///
const M: u32 = atan_tang_table::ATAN_TANG_M;

/// `atan(idx / M)` at `working_scale` — the single table slot the
/// kernel needs (`idx ∈ [0, M]`). idx = 0 → atan(0) = 0.
///
/// Reads the value from the BAKED binary Tang table
/// [`atan_tang_table::atan_table_entry_baked`]: the `M + 1` values
/// `atan(j/M)` are precomputed ONCE by a flint/Arb oracle as binary
/// fixed-point `round(atan(j/M) · 2^B)` — every retained bit pinned by a
/// rigorous interval bound (committed rodata) — then SLICED
/// to the tier's needed precision and reconstructed to `working_scale`
/// per call — one multiply + one shift. This replaces the previous
/// per-call `core::atan_fixed` halving-chain Series recompute, which the
/// samply probe showed dominated the kernel (~74% of total time at
/// D57<56>). `pow10_w` is `10^working_scale` in the work integer, supplied
/// by the caller from the kernel's baked `core::one(working_scale)` table
/// lookup.
#[inline]
fn table_entry(working_scale: u32, idx: usize, pow10_w: core::W) -> core::W {
    atan_tang_table::atan_table_entry_baked::<core::W>(working_scale, idx, M, pow10_w)
}

/// `atan(x)` strict kernel for `D57<SCALE>`, routed for `SCALE ∈ 0..=56`.
///
/// Stages:
/// 1. Fold sign and `|x| > 1` to `|x| ≤ 1` via `atan(1/|x|)` + π/2.
/// 2. Pick `j = round(|x| · M)`, `c_j = j / M`. Use the atan addition
///    formula to reduce: `y = (|x| − c_j) / (1 + c_j · |x|)`, with
///    `|y| ≤ 1/(2M)`.
/// 3. `atan(|x|) = table[j] + atan_taylor(y)`. The Taylor loop now
///    runs against a residual three orders of magnitude smaller than
///    the unreduced argument, so it terminates in ~15 iterations.
/// 4. Reassemble: apply the π/2-fold and the sign back to recover
///    `atan(x)`.
#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: Int<3>, mode: RoundingMode) -> Int<3> {
    // atan(0) = 0 short-circuit.
    if raw == Int::<3>::ZERO {
        return Int::<3>::ZERO;
    }

    let working_scale = SCALE + core::GUARD;
    let working_value = core::to_work(raw);
    let one_w = core::one(working_scale);
    let pow10_w = one_w;

    // Stage 1: sign + reciprocal fold so the table-reduced argument
    // sits in [0, 1].
    let sign_neg = working_value < core::zero();
    let mut x = if sign_neg { -working_value } else { working_value };
    let add_half_pi = x > one_w;
    if add_half_pi {
        x = core::div_cached(one_w, x, pow10_w);
    }

    // Stage 2: pick the nearest table entry. `j` is in [0, M].
    // x · M / one_w → integer in [0, M]. We compute it via
    // `round_to_nearest_int(x · M, working_scale)` so the rounding is half-away
    // from zero (matching the existing core helper).
    let x_times_m = x * core::lit(M as u128);
    let j_signed = core::round_to_nearest_int(x_times_m, working_scale);
    // Clamp j to [0, M-1] — at x = 1.0 exactly the round would
    // produce M, which is out of the table's range. Folding j = M
    // into j = M - 1 keeps |y| ≤ 1/M ≈ 2·10⁻³, still well below the
    // Taylor convergence band.
    let j_idx: u32 = if j_signed >= M as i128 {
        M - 1
    } else if j_signed < 0 {
        // x ∈ [0, 1] so j_signed should be ≥ 0; guard just in case.
        0
    } else {
        j_signed as u32
    };

    // c_j at the working scale.
    let cj_w = if j_idx == 0 {
        core::zero()
    } else {
        (one_w * core::lit(j_idx as u128)) / core::lit(M as u128)
    };

    // y = (x − c_j) / (1 + c_j · x). At j_idx = 0, y = x itself.
    let y = if j_idx == 0 {
        x
    } else {
        let numer = x - cj_w;
        let denom = one_w + core::mul(cj_w, x, working_scale);
        core::div_cached(numer, denom, pow10_w)
    };

    // Stage 3: Taylor on the small residual y. atan(y) =
    //   y − y³/3 + y⁵/5 − …
    //
    // For M = 512, |y| ≤ 1/(2M) ≈ 9.8·10⁻⁴, so |y²| ≤ ~10⁻⁶. Each
    // pair of terms shrinks by |y|² / (2·term_index+1), so the loop exits
    // on a zero term in ~15 iterations at `working_scale ≤ 87`. Mirrors
    // [`core::atan_taylor`]; the `÷10^working_scale` reduce goes through
    // the fast MG `core::mul` (`round_div_pow10`).
    let atan_y = {
        let y_squared = core::mul(y, y, working_scale);
        let mut sum = y;
        let mut term = y;
        let mut term_index: u128 = 1;
        loop {
            term = core::mul(term, y_squared, working_scale);
            let contrib = term / core::lit(2 * term_index + 1);
            if contrib == core::zero() {
                break;
            }
            if term_index % 2 == 1 {
                sum = sum - contrib;
            } else {
                sum = sum + contrib;
            }
            term_index += 1;
            if term_index > 200 {
                break;
            }
        }
        sum
    };

    // atan(|x|) = table[j_idx] + atan(y).
    let atan_abs_x = table_entry(working_scale, j_idx as usize, pow10_w) + atan_y;

    // Stage 4: undo the reciprocal fold then the sign.
    let mut result = if add_half_pi {
        core::half_pi::<SCALE>(working_scale) - atan_abs_x
    } else {
        atan_abs_x
    };
    if sign_neg {
        result = -result;
    }

    // Near-tie escape — see `wide_trig_core::tan_series` / the asin(3e-60)
    // family: a fixed-working-scale single shot cannot see a deciding digit
    // below the working scale. Clear-of-band residuals keep the single-shot
    // cost; the band falls to the Ziv-escalating generic kernel (rare).
    match crate::algos::support::wide_trig_core::round_to_storage_clear_of_tie_g::<Int<3>, _>(
        result, working_scale, SCALE, mode, Int::<3>::MAX, Int::<3>::MIN,
    ) {
        Some(rounded) => rounded,
        None => crate::algos::support::wide_trig_core::atan_series::<
            crate::types::widths::wide_trig_d57::Core,
            SCALE,
        >(raw, mode),
    }
}
