// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: direct leading-term-dropped Taylor, no argument reduction, not wired

//! `expm1` by the bare leading-term-dropped Taylor series.
//!
//! `expm1(x) = x + x^2/2! + x^3/3! + ...` evaluated directly at the working
//! scale, with NO argument reduction, NO `ln 2` constant, NO division to find a
//! range-reduction quotient, and NO reassembly. The whole kernel is the series
//! loop already in `exp_generic::expm1_fixed`; this file supplies the domain
//! gate, the regime classifier and the peak-fit wall around it.
//!
//! # Why this is the accurate kernel near zero
//!
//! Correct rounding needs absolute accuracy `1/2 * 10^-SCALE`, i.e.
//! `1/2 * 10^guard` WORKING units — a bound that does not shrink as the result
//! shrinks. Routing through `exp` instead costs `n = sqrt(3w)` repeated
//! squarings, each of which doubles the accumulated error, giving `~1.5 * 2^n`
//! units of noise; this series accumulates only `~T` units for `T` terms. The
//! guard requirement is therefore
//!
//! * via `exp`:   `guard > 0.301 * sqrt(3w) + 0.5`  (`~9` digits at `w = 300`)
//! * this kernel: `guard >~ 3`
//!
//! — six to eight digits of extra headroom at every scale, which is headroom the
//! Ziv walker would otherwise spend escalating.
//!
//! # Validity
//!
//! * `|x| <= DIRECT_BAND` (= 1). The series alternates for `x < 0`, so the
//!   cancellation loss is `max term / |sum|`; at `x = -1` that is
//!   `1/0.63212`, i.e. 0.66 bits. Past the band the loss grows like `e^|x|`.
//! * `2*w*log2(10) + 2 <= BITS - 2` — the `term * s` product spans `2w` digits
//!   before its `/10^w`, and `mul` returns the low bits.
//!
//! Reference: J.-M. Muller, *Elementary Functions* 3rd ed. (2016), 4.4;
//! N. J. Higham, *Accuracy and Stability of Numerical Algorithms* 2nd ed.
//! (2002), 1.14.1.

#![allow(dead_code)]

use super::expm1_support as sup;
use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// `expm1(v)` for a working-scale value `v_w` at scale `w`, by the direct
/// series.
///
/// `None` means the value cannot be produced in `S` at this `w`: either the
/// argument is outside the series' band (the caller must route to a reducing
/// candidate) or the internal product would wrap the work integer. The
/// `Option` is the `try_exp_fixed` contract — detect once here, let the policy
/// wrapper apply the overflow policy.
pub(crate) fn expm1_series_fixed<S: BigInt>(v_w: S, w: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    if v_w == S::ZERO {
        // expm1(0) = 0 exactly — the ONLY exact case (for algebraic x != 0,
        // e^x is transcendental by Lindemann-Weierstrass, so e^x - 1 never
        // lands on a storage grid line).
        return Some(S::ZERO);
    }
    match sup::regime::<S>(v_w, w) {
        sup::Regime::Overflow => return None,
        sup::Regime::MinusOne => return Some(sup::just_above_minus_one::<S>(w)),
        sup::Regime::Fits => {}
    }
    // Band gate: |v| <= DIRECT_BAND. Tested on the working-scale integer so no
    // division is needed on the hot path.
    if sup::abs::<S>(v_w) > eg::lit::<S>(sup::DIRECT_BAND) * eg::one::<S>(w) {
        return None;
    }
    // Peak: |term| <= 10^w and |s| <= 10^w, so the `term * s` product before
    // the `/10^w` spans at most `2w` digits.
    if !sup::peak_fits::<S>(2 * sup::scale_bits(w)) {
        return None;
    }
    Some(eg::expm1_fixed::<S>(v_w, w))
}
