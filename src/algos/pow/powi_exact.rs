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
//! The pin DEFERS to the composition ([`ExactPin::Defer`]) ONLY when it
//! cannot prove anything: a non-integer exponent, a non-integer base, or
//! `|n|` above the fast-path threshold. A positive integer power that
//! overflows storage is a PROOF of out-of-range ([`ExactPin::OutOfRange`]):
//! the seamed narrow kernel propagates it as its `None` (the policy dispatch
//! wrapper applies the overflow contract's panic; the `checked_` surface
//! propagates), and the default [`powi_exact_pin`] wrapper panics directly
//! for the wide shells. Deferring instead would be wrong — the composition's
//! to-nearest approximation could be directed-rounded (Floor / Trunc) back
//! inside the range at an out-by-one boundary.

use crate::int::types::traits::BigInt;
use crate::support::rounding::{should_bump, RoundingMode};

/// Verdict of an exact-power pin: the pin either DECIDES the cell — an
/// exact value, or proof the exact result exceeds the decimal range —
/// or declines and defers to the `exp(n · ln b)` composition.
pub(crate) enum ExactPin<V> {
    /// The correctly-rounded exact storage value.
    Value(V),
    /// Exact integer arithmetic proved the result exceeds the decimal
    /// range. The kernel seam returns `None` for it (the default policy
    /// dispatch wrapper applies the overflow contract's panic); the
    /// unseamed default wrappers panic directly.
    OutOfRange,
    /// The pin does not apply — defer to the composition.
    Defer,
}

/// Default (panicking) form of [`powi_exact_pin_checked`] for the unseamed
/// wide shells: `Value` → `Some`, `Defer` → `None`, and a proven
/// out-of-range applies the overflow contract's panic directly.
#[inline]
pub(crate) fn powi_exact_pin<St: BigInt, const SCALE: u32>(
    base: St,
    exp: St,
    storage_max: St,
    mode: RoundingMode,
) -> Option<St> {
    match powi_exact_pin_checked::<St, SCALE>(base, exp, storage_max, mode) {
        ExactPin::Value(v) => Some(v),
        ExactPin::OutOfRange => {
            crate::support::diagnostics::overflow_panic_with_scale("powf kernel", SCALE)
        }
        ExactPin::Defer => None,
    }
}

/// Correctly-rounded storage value of `b^n` at scale `SCALE`, when `b` is an
/// exact positive integer and `n` an exact integer with
/// `|n| <= INT_FAST_PATH_THRESHOLD`. `base` / `exp` are raw storage
/// (`value · 10^SCALE`); `storage_max` is the tier's representable maximum
/// (`Int<N>::MAX` for the narrow path, `C::storage_max()` for the wide path),
/// used only to reject a positive power that has left the decimal range.
///
/// [`ExactPin::Defer`] signals "this pin does not apply — defer to the
/// `exp(n · ln b)` composition": a fractional base or exponent (the genuinely
/// transcendental case) or `|n|` past the threshold. A positive power out of
/// range is [`ExactPin::OutOfRange`] (the overflow is proof) — detected once
/// here; each wrapper applies its policy (seam `None` / contract panic). For
/// `n < 0` the result is always in `(0, 1]` and so always representable.
#[inline]
pub(crate) fn powi_exact_pin_checked<St: BigInt, const SCALE: u32>(
    base: St,
    exp: St,
    storage_max: St,
    mode: RoundingMode,
) -> ExactPin<St> {
    // `10^SCALE` — the raw value `1.0`, sourced from the baked table so it is
    // exact at every tier (`St::TEN.pow` would wrap; `pow10::dispatch` does
    // not).
    let one_s = crate::consts::pow10::dispatch::<St>(SCALE);

    // The exponent must be an exact integer `n` (`exp` an exact multiple of
    // `10^SCALE`) with `|n|` inside the integer fast-path threshold.
    let (n_big, e_rem) = exp.div_rem(one_s);
    if e_rem != St::ZERO {
        return ExactPin::Defer;
    }
    let thresh =
        St::from_i128(crate::algos::pow::powf_series_2limb::INT_FAST_PATH_THRESHOLD as i128);
    if n_big > thresh || n_big < St::ZERO - thresh {
        return ExactPin::Defer;
    }
    let n = n_big.to_i128() as i32;

    // The base must be an exact positive integer `b >= 1`.
    if base <= St::ZERO {
        return ExactPin::Defer;
    }
    let (bv, b_rem) = base.div_rem(one_s);
    if b_rem != St::ZERO {
        return ExactPin::Defer; // fractional base — defer to the composition
    }

    // `b^0 = 1` and `1^n = 1` are exactly `1.0` for every mode.
    if n == 0 || bv == St::ONE {
        return ExactPin::Value(one_s);
    }
    let k = n.unsigned_abs();

    if n > 0 {
        // `b^n · 10^SCALE` — an exact integer when it fits the decimal range.
        // A `checked_pow` / `checked_mul` overflow, or a value past
        // `storage_max`, is PROOF the exact `b^n` exceeds the decimal range
        // (integer `b >= 2`, `n > 0`: the power is monotone): report
        // `OutOfRange` per the overflow contract (the default wrappers panic
        // in debug AND release; the `checked_` seam propagates `None`) rather
        // than deferring to the `exp(n·ln b)` composition, whose to-nearest
        // approximation can directed-round (Floor / Trunc) back INSIDE the
        // range at an out-by-one boundary (the `exp2(127)` hair case:
        // `2^127` at scale 0 is `i128::MAX + 1`).
        match bv
            .checked_pow(k)
            .and_then(|p| p.checked_mul(one_s))
            .filter(|v| *v <= storage_max)
        {
            Some(v) => ExactPin::Value(v),
            None => ExactPin::OutOfRange,
        }
    } else {
        // `b^-k = 1 / b^k`, stored as `round(10^SCALE / b^k)` — a strictly
        // positive value in `(0, 1]`.
        match bv.checked_pow(k) {
            Some(d) => {
                // `d = b^k` fits storage; round `10^SCALE / d` under `mode`.
                let (q, r) = one_s.div_rem(d);
                if r == St::ZERO {
                    return ExactPin::Value(q); // exact — no rounding
                }
                // Half-comparison `2r vs d`, formed as `r vs d − r` to avoid
                // overflowing `2r` (`r < d <= MAX`). `q` is the truncated
                // quotient; the result is positive.
                let cmp = r.cmp(&(d - r));
                let q_is_odd = q.bit(0);
                let bump = should_bump(mode, cmp, q_is_odd, true);
                ExactPin::Value(if bump { q + St::ONE } else { q })
            }
            None => {
                // `b^k` overflowed storage ⟹ `b^k > MAX >= 2·10^SCALE`
                // (every tier keeps at least one integer digit, so
                // `10^SCALE <= MAX / 2`), hence `0 < 1/b^k < ½` LSB:
                // a sub-resolution positive, strictly below the half
                // boundary. Only `Ceiling` rounds it up to one LSB.
                let bump = should_bump(mode, core::cmp::Ordering::Less, false, true);
                ExactPin::Value(if bump { St::ONE } else { St::ZERO })
            }
        }
    }
}

/// `Some(n)` when `exp` (raw storage at `SCALE`) is an exact integer with
/// `|n|` inside the integer fast-path threshold — the shared exponent gate of
/// [`powi_exact_pin`] and the fractional-base chain.
pub(crate) fn exp_as_small_int_raw<St: BigInt, const SCALE: u32>(exp: St) -> Option<i32> {
    let one_s = crate::consts::pow10::dispatch::<St>(SCALE);
    let (n_big, e_rem) = exp.div_rem(one_s);
    if e_rem != St::ZERO {
        return None;
    }
    let thresh =
        St::from_i128(crate::algos::pow::powf_series_2limb::INT_FAST_PATH_THRESHOLD as i128);
    if n_big > thresh || n_big < St::ZERO - thresh {
        return None;
    }
    Some(n_big.to_i128() as i32)
}

/// `10^d` in `St`, or `None` past the width (checked ×10 loop — this is a
/// cold pin path; `d` is bounded by `SCALE·(k+1)` in practice).
fn checked_pow10<St: BigInt>(d: u32) -> Option<St> {
    let ten = St::from_i128(10);
    let mut v = St::ONE;
    for _ in 0..d {
        v = v.checked_mul(ten)?;
    }
    Some(v)
}

/// `floor(10^e / d)` with remainder, by STREAMING long division — the
/// power-of-ten dividend is never materialised, so the reciprocal pin works
/// at MAX_SCALE where `10^(SCALE+f·k)` itself exceeds the width.
enum Pow10Div<St> {
    /// The quotient and remainder; quotient verified `<= storage_max`.
    Q(St, St),
    /// The quotient exceeds the decimal range — PROOF of overflow (the
    /// quotient only grows as digits stream in).
    OutOfRange,
    /// `d` is too close to the width for the digit step (`r·10` overflowed) —
    /// not a proof of anything; the caller defers.
    Wide,
}

fn div_pow10_small<St: BigInt>(e: u32, d: St, storage_max: St) -> Pow10Div<St> {
    let ten = St::from_i128(10);
    let (mut q, mut r) = St::ONE.div_rem(d);
    for _ in 0..e {
        let r10 = match r.checked_mul(ten) {
            Some(v) => v,
            None => return Pow10Div::Wide,
        };
        let (digit, rr) = r10.div_rem(d); // digit <= 9: r < d ⇒ r·10 < 10·d
        q = match q.checked_mul(ten).and_then(|v| v.checked_add(digit)) {
            Some(v) => v,
            None => return Pow10Div::OutOfRange,
        };
        if q > storage_max {
            return Pow10Div::OutOfRange;
        }
        r = rr;
    }
    Pow10Div::Q(q, r)
}

/// Correctly-rounded `base^n` for a TERMINATING-DECIMAL base — the
/// fractional-base sibling of [`powi_exact_pin`], reached when that pin
/// declines (non-integer base). The base is REDUCED to its significant
/// digits first: `base = m / 10^f` with the trailing zeros of the raw
/// stripped, so `m` is small for real literals (`2.5 -> m = 25, f = 1`) and
/// `m^k` never touches a `2·SCALE` product. Then `base^n = m^k / 10^(f·k)`
/// (or its reciprocal) is placed on the `SCALE` grid by EXACT integer
/// arithmetic — a shift when it terminates, a single half-compared rounding
/// otherwise (ties included), at every scale.
///
/// `None` (defer to the guarded composition) only when `m^k` or a needed
/// power of ten exceeds the width — never a proof of anything. A POSITIVE
/// exact result beyond the decimal range panics per the overflow contract
/// (the proof is in hand), mirroring [`powi_exact_pin`].
pub(crate) fn powi_terminating_pin<St: BigInt, const SCALE: u32>(
    base: St,
    n: i32,
    storage_max: St,
    mode: RoundingMode,
) -> Option<St> {
    debug_assert!(n != 0);
    if base <= St::ZERO {
        return None;
    }
    // Reduce: base raw = m · 10^z, m not divisible by 10; f = significant
    // fraction length of the VALUE (f <= SCALE; f == 0 means integer base,
    // which powi_exact_pin already owns but is handled here for totality).
    let ten = St::from_i128(10);
    let mut m = base;
    let mut z = 0u32;
    loop {
        let (q, r) = m.div_rem(ten);
        if r != St::ZERO || z >= SCALE {
            break;
        }
        m = q;
        z += 1;
    }
    let f = SCALE - z;
    let k = n.unsigned_abs();
    let mk = m.checked_pow(k)?; // base's significant digits ^ k — small for real literals
    let fk = f.checked_mul(k)?;

    // Round the exact rational `num_pow10 / den` (or `mk · 10^shift`) onto the
    // SCALE grid; `positive` results only (base > 0).
    let place = |q: St, r: St, d: St| -> St {
        if r == St::ZERO {
            return q;
        }
        let cmp = r.cmp(&(d - r));
        let bump = should_bump(mode, cmp, q.bit(0), true);
        if bump {
            q + St::ONE
        } else {
            q
        }
    };

    let v = if n > 0 {
        // base^k = mk / 10^fk; at SCALE the raw is mk · 10^(SCALE - fk) when
        // that shift is non-negative (exact), else a single rounded division.
        if fk <= SCALE {
            mk.checked_mul(checked_pow10::<St>(SCALE - fk)?)?
        } else {
            match checked_pow10::<St>(fk - SCALE) {
                Some(d) => {
                    let (q, r) = mk.div_rem(d);
                    place(q, r, d)
                }
                None => {
                    // The divisor exceeds the width while mk fits: the result
                    // is at least one order below ½ LSB — a sub-resolution
                    // positive; only Ceiling rounds it up.
                    let bump = should_bump(mode, core::cmp::Ordering::Less, false, true);
                    if bump {
                        St::ONE
                    } else {
                        St::ZERO
                    }
                }
            }
        }
    } else {
        // base^-k = 10^fk / mk; the raw at SCALE is 10^(SCALE + fk) / mk,
        // single-rounded — exact directed/tie handling for terminating AND
        // non-terminating reciprocals alike (1.5^-1 = 0.666...). When the
        // power-of-ten numerator exceeds the width (MAX_SCALE cells), the
        // division streams digit-by-digit instead.
        let e = SCALE.checked_add(fk)?;
        match checked_pow10::<St>(e) {
            Some(num) => {
                let (q, r) = num.div_rem(mk);
                place(q, r, mk)
            }
            None => match div_pow10_small::<St>(e, mk, storage_max) {
                Pow10Div::Q(q, r) => place(q, r, mk),
                Pow10Div::OutOfRange => crate::support::diagnostics::overflow_panic_with_scale(
                    "powf kernel",
                    SCALE,
                ),
                Pow10Div::Wide => return None,
            },
        }
    };
    if v > storage_max {
        // Exact arithmetic put the correctly-rounded result beyond the
        // decimal range — proof of overflow; the contract panics.
        crate::support::diagnostics::overflow_panic_with_scale("powf kernel", SCALE)
    }
    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::types::Int;

    #[test]
    fn terminating_pin_exact_ties_and_reciprocals() {
        let pin = |base: i128, n: i32, mode: RoundingMode| {
            powi_terminating_pin::<Int<2>, 2>(Int::<2>::from_i128(base), n, Int::<2>::MAX, mode)
                .map(|v| v.as_i128())
        };
        for mode in MODES {
            // 2.5^2 = 6.25 at scale 2 — exact at every mode.
            assert_eq!(pin(250, 2, mode), Some(625), "{mode:?} 2.5^2");
            // 0.5^-2 = 4 — exact reciprocal.
            assert_eq!(pin(50, -2, mode), Some(400), "{mode:?} 0.5^-2");
        }
        // 1.5^3 = 3.375 at scale 2 — an exact TIE the single rounding decides.
        assert_eq!(pin(150, 3, RoundingMode::HalfToEven), Some(338)); // 337 odd -> up
        assert_eq!(pin(150, 3, RoundingMode::HalfTowardZero), Some(337));
        assert_eq!(pin(150, 3, RoundingMode::Trunc), Some(337));
        assert_eq!(pin(150, 3, RoundingMode::Ceiling), Some(338));
        // 0.5^-2 = 4 at MAX-scale-like cells: 10^(SCALE+2) exceeds i128, so
        // the reciprocal streams its long division (the gate4 residue).
        let pin37 = |base: i128, n: i32, mode: RoundingMode| {
            powi_terminating_pin::<Int<2>, 37>(Int::<2>::from_i128(base), n, Int::<2>::MAX, mode)
                .map(|v| v.as_i128())
        };
        let half_raw = 5 * 10_i128.pow(36);
        for mode in MODES {
            assert_eq!(pin37(half_raw, -2, mode), Some(4 * 10_i128.pow(37)), "{mode:?} 0.5^-2 @37");
        }
        // 1.5^-1 = 0.666... — non-terminating reciprocal, residual above half.
        assert_eq!(pin(150, -1, RoundingMode::Floor), Some(66));
        assert_eq!(pin(150, -1, RoundingMode::HalfToEven), Some(67));
        assert_eq!(pin(150, -1, RoundingMode::Ceiling), Some(67));
    }

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
    #[should_panic(expected = "powf kernel")]
    fn positive_overflow_panics() {
        // 10² = 100 exceeds the D38<37> decimal range; the exact arithmetic is
        // PROOF of the overflow, so the pin panics per the contract (deferring
        // let the directed-down composition round the approximation back into
        // range — the exp2(127) hair case).
        let _ = pin::<37>(10, 2, RoundingMode::HalfToEven);
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
