// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Exact integer-power pin for `powf` — one generic kernel for every width.
//!
//! When the exponent is an exact integer `n` and the base is an exact
//! positive integer `b`, `b^n` is an **exact rational** — `b^n` for `n >= 0`,
//! `1 / b^|n|` for `n < 0`. Its correctly-rounded storage value is fixed by
//! integer arithmetic alone, so emitting it directly stops the
//! `exp(n · ln b)` composition's to-nearest round-off from landing one LSB
//! off under a directed mode (`Floor` / `Trunc` / `Ceiling`) at these
//! algebraic-exact points (e.g. `10^-2 = 0.01`, `25^-3 = 0.000064`). The
//! composition is correct to nearest, but a deferred reciprocal whose scaled
//! integer power overflows storage (`powf_series_2limb::powi_raw_checked`
//! returns `None`) was being computed through it, ~1 ULP low — so `Floor` /
//! `Trunc` rounded to `0.00999…9` instead of the exact `0.01`.
//!
//! This is the power analogue of `exp::exp_series_2limb::exp2_exact_pin` +
//! `round_pow2_fraction`: a single `St: BigInt`-generic pin called by both
//! the narrow `Fixed` kernel (`powf_series_2limb`, `St = Int<2>`) and the
//! wide schoolbook kernel (`pow_schoolbook`, `St = C::Storage`).
//!
//! Returns `None` (defer to the composition) whenever the pin does not apply:
//! a non-integer exponent, a non-integer base, `|n|` above the fast-path
//! threshold, or a positive power that genuinely overflows storage (the
//! composition then panics uniformly, per the overflow contract).

use crate::int::types::traits::BigInt;
use crate::support::rounding::{should_bump, RoundingMode};

/// Correctly-rounded storage value of `b^n` at scale `SCALE`, when `b` is an
/// exact positive integer and `n` an exact integer with
/// `|n| <= INT_FAST_PATH_THRESHOLD`. `base` / `exp` are raw storage
/// (`value · 10^SCALE`); `storage_max` is the tier's representable maximum
/// (`Int<N>::MAX` for the narrow path, `C::storage_max()` for the wide path),
/// used only to reject a positive power that has left the decimal range.
///
/// Returns `None` to signal "this pin does not apply — defer to the
/// `exp(n · ln b)` composition": a fractional base or exponent (the genuinely
/// transcendental case), `|n|` past the threshold, or a positive power out of
/// range. For `n < 0` the result is always in `(0, 1]` and so always
/// representable.
#[inline]
pub(crate) fn powi_exact_pin<St: BigInt, const SCALE: u32>(
    base: St,
    exp: St,
    storage_max: St,
    mode: RoundingMode,
) -> Option<St> {
    // `10^SCALE` — the raw value `1.0`, sourced from the baked table so it is
    // exact at every tier (`St::TEN.pow` would wrap; `pow10::dispatch` does
    // not).
    let one_s = crate::consts::pow10::dispatch::<St>(SCALE);

    // The exponent must be an exact integer `n` (`exp` an exact multiple of
    // `10^SCALE`) with `|n|` inside the integer fast-path threshold.
    let (n_big, e_rem) = exp.div_rem(one_s);
    if e_rem != St::ZERO {
        return None;
    }
    let thresh =
        St::from_i128(crate::algos::pow::powf_series_2limb::INT_FAST_PATH_THRESHOLD as i128);
    if n_big > thresh || n_big < St::ZERO - thresh {
        return None;
    }
    let n = n_big.to_i128() as i32;

    // The base must be an exact positive integer `b >= 1`.
    if base <= St::ZERO {
        return None;
    }
    let (bv, b_rem) = base.div_rem(one_s);
    if b_rem != St::ZERO {
        return None; // fractional base — defer to the composition
    }

    // `b^0 = 1` and `1^n = 1` are exactly `1.0` for every mode.
    if n == 0 || bv == St::ONE {
        return Some(one_s);
    }
    let k = n.unsigned_abs();

    if n > 0 {
        // `b^n · 10^SCALE` — an exact integer when it fits the decimal range.
        // `checked_pow` / `checked_mul` reject a storage overflow; the
        // explicit `> storage_max` guard rejects a value that fits the raw
        // integer but exceeds the tier's decimal maximum. Either way `None`
        // defers to the composition, which panics on the genuine overflow.
        let p = bv.checked_pow(k)?;
        let v = p.checked_mul(one_s)?;
        if v > storage_max {
            return None;
        }
        Some(v)
    } else {
        // `b^-k = 1 / b^k`, stored as `round(10^SCALE / b^k)` — a strictly
        // positive value in `(0, 1]`.
        match bv.checked_pow(k) {
            Some(d) => {
                // `d = b^k` fits storage; round `10^SCALE / d` under `mode`.
                let (q, r) = one_s.div_rem(d);
                if r == St::ZERO {
                    return Some(q); // exact — no rounding
                }
                // Half-comparison `2r vs d`, formed as `r vs d − r` to avoid
                // overflowing `2r` (`r < d <= MAX`). `q` is the truncated
                // quotient; the result is positive.
                let cmp = r.cmp(&(d - r));
                let q_is_odd = q.bit(0);
                let bump = should_bump(mode, cmp, q_is_odd, true);
                Some(if bump { q + St::ONE } else { q })
            }
            None => {
                // `b^k` overflowed storage ⟹ `b^k > MAX >= 2·10^SCALE`
                // (every tier keeps at least one integer digit, so
                // `10^SCALE <= MAX / 2`), hence `0 < 1/b^k < ½` LSB:
                // a sub-resolution positive, strictly below the half
                // boundary. Only `Ceiling` rounds it up to one LSB.
                let bump = should_bump(mode, core::cmp::Ordering::Less, false, true);
                Some(if bump { St::ONE } else { St::ZERO })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::types::Int;

    // D38<S>: storage MAX ≈ 1.7·10^38 (full i128). Each base below is
    // representable at its chosen scale (`base · 10^S <= MAX`), but its
    // scaled power `base^|n| · 10^S` overflows i128 — exactly the case the
    // narrow integer fast path defers on.
    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    /// Pin called on already-scaled raw storage at scale `SC`.
    fn pin_raw<const SC: u32>(base_raw: i128, exp_raw: i128, mode: RoundingMode) -> Option<i128> {
        powi_exact_pin::<Int<2>, SC>(
            Int::<2>::from_i128(base_raw),
            Int::<2>::from_i128(exp_raw),
            Int::<2>::MAX,
            mode,
        )
        .map(|v| v.as_i128())
    }

    /// Pin on integer `base`/`exp` values, scaled to `SC` internally.
    fn pin<const SC: u32>(base: i128, exp: i128, mode: RoundingMode) -> Option<i128> {
        let one = 10_i128.pow(SC);
        pin_raw::<SC>(base * one, exp * one, mode)
    }

    #[track_caller]
    fn check_exact<const SC: u32>(base: i128, exp: i128, divisor: i128) {
        let one = 10_i128.pow(SC);
        for mode in MODES {
            assert_eq!(
                pin::<SC>(base, exp, mode),
                Some(one / divisor),
                "base={base} exp={exp} scale={SC} mode={mode:?}"
            );
        }
    }

    #[test]
    fn exact_reciprocals_are_mode_independent() {
        // Exact powers: the rational lands exactly on a grid line, so every
        // mode returns the same value (no round-off to misdirect). Bases ≤ 17
        // at scale 37; the larger bases at scale 36 (where they remain ≤ MAX).
        check_exact::<37>(10, -2, 100); // 0.01
        check_exact::<37>(16, -2, 256); // 0.00390625
        check_exact::<37>(4, -3, 64); // 0.015625
        check_exact::<37>(5, -3, 125); // 0.008
        check_exact::<36>(20, -2, 400); // 0.0025
        check_exact::<36>(25, -2, 625); // 0.0016
        check_exact::<36>(25, -3, 15_625); // 0.000064
    }

    #[test]
    fn inexact_reciprocal_rounds_each_direction() {
        // 1/3 = 0.333…3̅ at scale 37: q = floor(10^37 / 3), remainder 1 (< half),
        // so the directed/nearest split is the LSB.
        let q = 10_i128.pow(37) / 3; // 333…3 (37 threes)
        // remainder 10^37 mod 3 == 1, strictly below half → round down except Ceiling.
        assert_eq!(pin::<37>(3, -1, RoundingMode::Floor), Some(q));
        assert_eq!(pin::<37>(3, -1, RoundingMode::Trunc), Some(q));
        assert_eq!(pin::<37>(3, -1, RoundingMode::HalfToEven), Some(q));
        assert_eq!(pin::<37>(3, -1, RoundingMode::Ceiling), Some(q + 1));
    }

    #[test]
    fn positive_powers_are_exact_integers() {
        let one = 10_i128.pow(37);
        for mode in MODES {
            assert_eq!(pin::<37>(2, 3, mode), Some(8 * one));
            assert_eq!(pin::<37>(17, 1, mode), Some(17 * one)); // in-range edge
        }
    }

    #[test]
    fn positive_overflow_defers() {
        // 10² = 100 exceeds the D38<37> decimal range → defer (None) so the
        // composition can panic uniformly.
        assert_eq!(pin::<37>(10, 2, RoundingMode::HalfToEven), None);
    }

    #[test]
    fn non_integer_base_or_exp_defers() {
        // 2.5 (fractional base) and 0.5 (fractional exponent) are not this
        // pin's job — defer to the composition.
        let one = 10_i128.pow(37);
        assert_eq!(pin_raw::<37>(one * 5 / 2, -2 * one, RoundingMode::Floor), None);
        assert_eq!(pin_raw::<37>(2 * one, one / 2, RoundingMode::Floor), None);
    }
}
