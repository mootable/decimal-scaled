// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log1p` algorithm family — `log1p(t) = ln(1 + t)`, domain `t > -1`.
//!
//! Two generic kernels, one per argument regime; there is no per-tier
//! variant and no narrow/wide split. `crate::policy::log1p` chooses
//! between them by value and supplies each width's work integer and
//! guard.
//!
//! Variants:
//!
//! - [`log1p_artanh`] — the Goldberg/Higham reformulation
//!   `log1p(t) = 2·artanh(u)`, `u = t / (2 + t)`, reaching the result
//!   without ever forming `1 + t`. Wraps the width-generic working-scale
//!   kernel [`crate::algos::exp::exp_generic::log1p_fixed`] already
//!   shipped for `acosh` / `atanh`. The series has NO range reduction,
//!   so it is the small-`|t|` kernel: its ratio is `u²`, which stops
//!   converging usefully as `|u| → 1` (`t → -1` or `t → ∞`).
//! - [`log1p_with_ln`] — the composition for everything else: form
//!   `1 + t` at the working scale (EXACT in fixed point) and run the
//!   width-generic `ln` kernel, which carries its own sqrt range
//!   reduction.
//!
//! # Why two kernels, and why `log1p` is not more accurate than `ln`
//!
//! In binary floating point `log1p` exists because `1 + t` loses every
//! significant digit of a tiny `t`. In this crate's FIXED-POINT
//! representation `1 + t` is exactly representable at the working scale,
//! so that cancellation cannot occur and `log1p` carries **no accuracy
//! advantage** over `ln(1 + t)`. The function is provided for API parity
//! and standards conformance (C `log1p`, IEEE 754-2019 `logp1`).
//!
//! The split is therefore about **cost and convergence**, not accuracy:
//! for a tiny `t` the artanh series terminates in a couple of terms
//! while `ln` would burn its whole sqrt-reduction chain, and near the
//! domain edge the artanh series does not converge at all while `ln`
//! is uniformly good.
//!
//! The same split is already in the tree: the wide `acosh_strict` uses
//! `log1p_fixed` near 1 and the `ln` kernel away from it.

pub(crate) mod log1p_artanh;
pub(crate) mod log1p_with_ln;

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Directed-rounding post-adjust for the sub-resolution band near
/// `t = 0` — the direct analogue of
/// [`wide_trig_core::adjust_ln_near_one`], read at `log1p`'s own
/// argument so the gap `δ` IS `raw` and no subtraction is needed.
///
/// Concavity gives `log1p(t) < t` STRICTLY for every `t ≠ 0`, and
/// `ln(1 + t)` is transcendental for algebraic `1 + t ≠ 1`, so the value
/// never lands exactly on a storage grid line. For a tiny `t` the
/// deficit `t − log1p(t) ≈ t²/2` can sit far below any REACHABLE working
/// scale (`t = 10^−SCALE` leaves it at ~`10^−2·SCALE`, past the Ziv
/// precision horizon at the wide tiers), so the kernel rounds to exactly
/// the linear term `t` and a downward mode then keeps `t` though the
/// true value is strictly below it.
///
/// Because `log1p(t) < t`, a CORRECT downward result can never equal
/// `t`, so `result == raw` is unambiguously the sub-resolution
/// overshoot — step down one LSB. `log1p(0) = 0` is exact and excluded;
/// nearest modes (the fraction is `1⁻`, so they round to `t` anyway) and
/// `Ceiling` (`t` IS the correct ceiling) are already right. `Floor`
/// steps down for both signs; `Trunc` (toward zero) steps down only for
/// `t > 0`, since for `t < 0` truncation moves UP and `t` is then the
/// correct answer.
///
/// A no-op unless the result is exactly `raw`, so every cell whose
/// deciding digit the walker actually reaches passes through untouched.
///
/// [`wide_trig_core::adjust_ln_near_one`]: crate::algos::support::wide_trig_core::adjust_ln_near_one
#[inline]
pub(crate) fn adjust_near_zero<St: BigInt>(result: St, raw: St, mode: RoundingMode) -> St {
    if crate::support::rounding::is_nearest_mode(mode) {
        return result;
    }
    if raw == <St as BigInt>::ZERO {
        return result; // log1p(0) = 0 is exact
    }
    if result != raw {
        return result; // only the sub-resolution linear-term overshoot
    }
    match mode {
        RoundingMode::Floor => result - <St as BigInt>::ONE,
        RoundingMode::Trunc if raw > <St as BigInt>::ZERO => result - <St as BigInt>::ONE,
        _ => result,
    }
}

/// Panics unless `t > -1`, i.e. unless the raw storage value exceeds
/// `-10^SCALE`. The family-level precondition, shared by both kernels.
///
/// Mirrors the `ln` kernels' `argument must be positive` guard:
/// `log1p`'s domain violation is precisely `1 + t <= 0`, which is `ln`'s
/// own out-of-domain condition on the value it is asked for.
///
/// `10^scale` is representable in every tier at every admissible scale
/// (`MAX_SCALE` is one below the tier's digit capacity), so building the
/// bound cannot itself overflow.
#[inline]
pub(crate) fn guard_domain<St: BigInt>(raw: St, scale: u32) {
    if raw <= -crate::consts::pow10::dispatch::<St>(scale) {
        panic!("log1p: argument must be greater than -1");
    }
}

#[cfg(test)]
mod tests;
