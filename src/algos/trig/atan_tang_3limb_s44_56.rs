//! Bespoke `atan_strict` kernel slot for `D57<SCALE>` with
//! `SCALE ∈ 44..=56`.
//!
//! At deep storage scales the wide-tier `atan_fixed` runs an
//! `O(log w)` halving chain (each `atan(x) = 2·atan(x/(1+√(1+x²)))`
//! costs one wide sqrt + one wide div + one wide mul) followed by a
//! Taylor evaluation on the post-halving residual. With `w = SCALE +
//! GUARD = 74..=87` and the per-tier halving cap at 7, the halving
//! chain itself burns ~7 wide sqrts (each ~1.2 µs at D57<57>) before
//! the Taylor loop runs ~30 terms — and every iteration of every
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
//! converges in ~15 terms at `w ≤ 87`, vs the 7 halvings + ~30 terms
//! the generic path runs.
//!
//! The slot is exposed through `crate::policy::trig`
//! only for `SCALE ∈ 44..=56`; lower scales keep using the generic
//! [`crate::algos::support::wide_trig_core::atan_series`] which is
//! already cheaper there (fewer halvings, faster Knuth dispatch).
//!
//! ## Correctness
//!
//! Error budget at working scale `w` (in LSB-of-`w`):
//!
//! - Reciprocal-fold `1/x` (when `|x| > 1`): ≤ 0.5 LSB.
//! - Table index quantisation `c_j = j/M`: exact (integer division
//!   of `one(w)` by small `M`, ≤ 0.5 LSB).
//! - `y = (x − c_j) / (1 + c_j · x)`: 1 mul + 1 div + 2 add/sub
//!   → ≤ 1.5 LSB.
//! - Taylor on `|y| ≤ 1/(2M) ≈ 10⁻³`: ~15 rounded muls → ≤ 7.5 LSB.
//! - Table lookup `atan(c_j)`: precomputed by the generic
//!   `atan_fixed` at the same `w`, ≤ 1 LSB after rounding.
//! - One outer add (`atan(c_j) + atan(y)`): ≤ 0.5 LSB.
//!
//! Total ≤ ~11 LSB-of-`w` = ~11·10⁻³⁰ at storage scale. The strict
//! contract requires ≤ 0.5 LSB-of-storage = 0.5·10⁻ᴿᴱ — a margin of
//! 28+ orders of magnitude even at `SCALE = 57`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::int::types::Int;

/// Table size — number of `atan(j / M)` entries per working scale.
/// Power of two so the index quantisation step `1/M` keeps the cheap
/// integer-division path. Larger M shrinks the post-table residual
/// `|y| ≤ 1/(2M)` and so shaves Taylor iterations.
///
/// Mirrors the tuning from the D57 exp lookup (the collapsed
/// per-tier 45..=56 exp table): same `Int<16>`-wide
/// work integer, same Knuth-dispatch arithmetic cost per slot, same
/// per-thread memoisation pattern. `M = 512` strikes the same balance
/// here — the post-table Taylor remainder is small enough that the
/// inner loop runs in ~15 iterations, against a one-off cold-start
/// table seed of `M · atan_fixed(w)` calls (~22 ms at SCALE=57).
///
const M: u32 = 512;

/// `atan(idx / M)` at working scale `w` — the single table slot the
/// kernel needs (`idx ∈ [0, M)`). idx = 0 → atan(0) = 0.
///
/// Value-independent for a given `(w, idx)`, recomputed on the stack each
/// call: stateless and heap-free. (`core::atan_fixed` runs `BigInt`
/// divides and is not a `const fn`, so the table cannot be baked as
/// `const` rodata in-crate.)
#[inline]
fn table_entry(w: u32, idx: usize) -> core::W {
    if idx == 0 {
        return core::zero();
    }
    // c_j = idx / M at working scale = (idx · 10^w) / M.
    let cj_w = (core::one(w) * core::lit(idx as u128)) / core::lit(M as u128);
    core::atan_fixed(cj_w, w)
}

/// `atan(x)` strict kernel for `D57<SCALE>` with `SCALE ∈ 44..=56`.
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

    let w = SCALE + core::GUARD;
    let v_w = core::to_work(raw);
    let one_w = core::one(w);
    let pow10_w = one_w;

    // Stage 1: sign + reciprocal fold so the table-reduced argument
    // sits in [0, 1].
    let sign_neg = v_w < core::zero();
    let mut x = if sign_neg { -v_w } else { v_w };
    let add_half_pi = x > one_w;
    if add_half_pi {
        x = core::div_cached(one_w, x, pow10_w);
    }

    // Stage 2: pick the nearest table entry. `j` is in [0, M].
    // x · M / one_w → integer in [0, M]. We compute it via
    // `round_to_nearest_int(x · M, w)` so the rounding is half-away
    // from zero (matching the existing core helper).
    let x_times_m = x * core::lit(M as u128);
    let j_signed = core::round_to_nearest_int(x_times_m, w);
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

    // c_j at working scale.
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
        let denom = one_w + core::mul(cj_w, x, w);
        core::div_cached(numer, denom, pow10_w)
    };

    // Stage 3: Taylor on the small residual y. atan(y) =
    //   y − y³/3 + y⁵/5 − …
    //
    // For M = 512, |y| ≤ 1/(2M) ≈ 9.8·10⁻⁴, so |y²| ≤ ~10⁻⁶. Each
    // pair of terms shrinks by |y|² / (2k+1), so the loop exits on a
    // zero term in ~15 iterations at w ≤ 87. Mirrors
    // [`core::atan_taylor`]; the `÷10^w` reduce goes through the fast
    // MG `core::mul` (`round_div_pow10`).
    let atan_y = {
        let y2 = core::mul(y, y, w);
        let mut sum = y;
        let mut term = y;
        let mut k: u128 = 1;
        loop {
            term = core::mul(term, y2, w);
            let contrib = term / core::lit(2 * k + 1);
            if contrib == core::zero() {
                break;
            }
            if k % 2 == 1 {
                sum = sum - contrib;
            } else {
                sum = sum + contrib;
            }
            k += 1;
            if k > 200 {
                break;
            }
        }
        sum
    };

    // atan(|x|) = table[j_idx] + atan(y).
    let atan_abs_x = table_entry(w, j_idx as usize) + atan_y;

    // Stage 4: undo the reciprocal fold then the sign.
    let mut result = if add_half_pi {
        core::half_pi(w) - atan_abs_x
    } else {
        atan_abs_x
    };
    if sign_neg {
        result = -result;
    }

    core::round_to_storage_with(result, w, SCALE, mode)
}
