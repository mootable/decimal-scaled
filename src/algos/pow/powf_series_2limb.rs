// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Floating-point power series kernel — `powf` computed as
//! `exp(y · ln(x))` on the 256-bit `Fixed` guard-digit intermediate.
//!
//! The narrow `Int<2>`-storage series path: it serves the narrow
//! D18 / D38 tier, composing the `ln` and `exp` series kernels in the
//! wider `Fixed` intermediate. The `ln`, the multiplication, and the
//! `exp` all share the wide guard-digit working scale so no precision
//! is dropped between stages.
//!
//! Fast path preserved verbatim from the typed surface:
//! - A non-positive base saturates to `0` (matches the f64-bridge
//!   NaN-to-ZERO policy for negative bases with arbitrary fractional
//!   exponents).
//! - Integer-valued exponents with `|n| <= INT_FAST_PATH_THRESHOLD`
//!   route to `powi_raw_checked::<SCALE>` — exact via square-and-multiply
//!   on raw, ~10–500× faster than the `exp(y·ln(x))` chain. When a partial
//!   power leaves the storage range it returns `None` and the call defers
//!   to the overflow-safe composition (so an in-range reciprocal such as
//!   `powf(10, -2) = 0.01` computes instead of panicking on its `10² = 100`
//!   intermediate).
//!
//! Returns the raw `i128` storage at the input's scale.

use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::support::fixed::Fixed;
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Integer-exponent fast-path threshold for `powf_strict`.
///
/// At `|n| <= 64`, the square-and-multiply `powi(n)` costs at most
/// ~12 multiplications (2·log2(64)) — comfortably cheaper than the
/// `exp(y · ln(x))` chain (one `ln_fixed` + one `mul` + one
/// `exp_fixed`, each ~hundreds of i128 ops on the 256-bit `Fixed`
/// intermediate). Above 64 the integer path's cost grows
/// logarithmically while the transcendental path's cost is constant,
/// so a fixed threshold is sufficient.
pub(crate) const INT_FAST_PATH_THRESHOLD: i32 = 64;

/// `(a · b) / 10^SCALE` rounded under `mode`, formed in the 256-bit
/// `Int<4>` widening so the product never overflows mid-step, then
/// narrowed back to `i128` storage. Returns `None` when the rounded
/// result does not fit `i128` — the signal [`powi_raw_checked`] uses to
/// defer to the overflow-safe `exp(y·ln x)` composition rather than
/// compute a partial power that has left the storage range.
///
/// Bit-identical to the prior `mul_widen_divide::<2, SCALE>` for every
/// in-range result: the value and rounding are width-independent, so the
/// narrowed `i128` matches exactly.
#[inline]
fn mul_div_scale_checked<const SCALE: u32>(a: i128, b: i128, mode: RoundingMode) -> Option<i128> {
    crate::algos::mul::mul_widen_divide::mul_widen_divide::<4, SCALE>(
        Int::<4>::from_i128(a),
        Int::<4>::from_i128(b),
        mode,
    )
    .try_to_i128()
}

/// Integer-exponent square-and-multiply on raw `i128` storage at `SCALE`,
/// returning `None` if any partial power `base^k` (`k ≤ |n|`) leaves the
/// `i128` storage range. Uses `mul_widen_divide` directly (not the decimal
/// `Mul` operator, which would re-enter the mul policy from inside an
/// algorithm fn — the layering inversion). `one_s` is `10^SCALE`.
///
/// Every intermediate (`acc`, `b`) is bounded by `base^|n|`, so a `None`
/// means the full integer power `base^|n|` is itself out of range. The
/// caller then routes to the composition, which panics on a genuinely
/// out-of-range result (positive `n`) and computes the in-range reciprocal
/// `base^-|n|` overflow-safely (negative `n`) — fixing the integer fast
/// path's spurious panic on `powf(10, -2)` (`10² = 100` overflows storage
/// while `1/100 = 0.01` is representable). When `Some`, the result is the
/// exact integer power (or its reciprocal), bit-identical to before.
///
/// For negative `n`, returns `one_s / base^|n|` via the decimal
/// `div_widen_scale` (a genuine downward cross-tier call to the int layer).
#[inline]
fn powi_raw_checked<const SCALE: u32>(base: i128, n: i32, mode: RoundingMode) -> Option<i128> {
    let one_s: Int<2> = Int::<2>::TEN.pow(SCALE);
    if n == 0 {
        return Some(one_s.as_i128());
    }
    let pos_n = n.unsigned_abs();
    // Square-and-multiply; every partial product is range-checked so an
    // out-of-storage intermediate signals `None` instead of panicking.
    let mut acc: i128 = one_s.as_i128();
    let mut b: i128 = base;
    let mut e = pos_n;
    while e > 0 {
        if e & 1 == 1 {
            acc = mul_div_scale_checked::<SCALE>(acc, b, mode)?;
        }
        e >>= 1;
        if e > 0 {
            b = mul_div_scale_checked::<SCALE>(b, b, mode)?;
        }
    }
    if n > 0 {
        Some(acc)
    } else {
        // Reciprocal: one_s / base^|n|. `acc = base^|n|` fits i128 here and
        // `1/acc ≤ 1`, so the quotient certainly fits storage.
        Some(
            crate::algos::div::div_widen_scale::div_widen_scale::<2>(
                one_s,
                Int::<2>::from_i128(acc),
                Int::<2>::TEN.pow(SCALE),
                mode,
            )
            .as_i128(),
        )
    }
}

/// Returns `Some(n)` if `exp_raw` (at `SCALE`) represents an exact
/// integer value `n` that fits `i32` and `|n| <= INT_FAST_PATH_THRESHOLD`.
#[inline]
fn exp_as_small_int<const SCALE: u32>(exp_raw: i128) -> Option<i32> {
    let mult = 10_i128.pow(SCALE);
    if exp_raw % mult != 0 {
        return None;
    }
    let q = exp_raw / mult;
    if !(i32::MIN as i128..=i32::MAX as i128).contains(&q) {
        return None;
    }
    let n = q as i32;
    if n.unsigned_abs() <= INT_FAST_PATH_THRESHOLD as u32 {
        Some(n)
    } else {
        None
    }
}

/// `base^exp` with caller-chosen `working_digits` above the storage scale.
///
/// Both `base` and `exp` are raw storage at `scale`. Non-positive `base`
/// saturates to `0`.
#[inline]
#[must_use]
pub(crate) fn powf_with<const SCALE: u32>(
    base: Int<2>,
    exp: Int<2>,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<2> {
    Int::<2>::from_i128(powf_with_raw::<SCALE>(
        base.as_i128(),
        exp.as_i128(),
        working_digits,
        mode,
    ))
}

/// `i128` core of [`powf_with`].
#[inline]
fn powf_with_raw<const SCALE: u32>(
    base: i128,
    exp: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if base <= 0 {
        return 0;
    }
    if let Some(n) = exp_as_small_int::<SCALE>(exp) {
        if let Some(v) = powi_raw_checked::<SCALE>(base, n, mode) {
            return v;
        }
        // `base^|n|` left the storage range. When the base is an exact
        // integer the result is still an exact rational — pin its
        // correctly-directed-rounded value (`10^SCALE / base^|n|` for a
        // negative `n`) so a directed mode is not 1 LSB off, rather than
        // defer to the to-nearest `exp(y·ln x)` composition. A fractional
        // base or a genuinely out-of-range positive power returns `None`
        // and falls through to the composition (which panics on overflow).
        if let Some(v) = crate::algos::pow::powi_exact::powi_exact_pin::<Int<2>, SCALE>(
            Int::<2>::from_i128(base),
            Int::<2>::from_i128(exp),
            Int::<2>::MAX,
            mode,
        ) {
            return v.as_i128();
        }
    }
    let w = SCALE + working_digits;
    let pow = 10u128.pow(working_digits);
    let ln_x = ln_fixed(Fixed::from_u128_mag(base as u128, false).mul_u128(pow), w);
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    exp_fixed(y_w.mul(ln_x, w), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf kernel", SCALE)
        })
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
#[inline]
#[must_use]
pub(crate) fn powf_strict<const SCALE: u32>(base: Int<2>, exp: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(powf_strict_raw::<SCALE>(base.as_i128(), exp.as_i128(), mode))
}

/// `i128` core of [`powf_strict`].
#[inline]
fn powf_strict_raw<const SCALE: u32>(base: i128, exp: i128, mode: RoundingMode) -> i128 {
    if base <= 0 {
        return 0;
    }
    if let Some(n) = exp_as_small_int::<SCALE>(exp) {
        if let Some(v) = powi_raw_checked::<SCALE>(base, n, mode) {
            return v;
        }
        // `base^|n|` left the storage range. When the base is an exact
        // integer the result is still an exact rational — pin its
        // correctly-directed-rounded value (`10^SCALE / base^|n|` for a
        // negative `n`) so a directed mode is not 1 LSB off, rather than
        // defer to the to-nearest `exp(y·ln x)` composition. A fractional
        // base or a genuinely out-of-range positive power returns `None`
        // and falls through to the composition (which panics on overflow).
        if let Some(v) = crate::algos::pow::powi_exact::powi_exact_pin::<Int<2>, SCALE>(
            Int::<2>::from_i128(base),
            Int::<2>::from_i128(exp),
            Int::<2>::MAX,
            mode,
        ) {
            return v.as_i128();
        }
    }
    let w = SCALE + STRICT_GUARD;
    let pow = 10u128.pow(STRICT_GUARD);
    let ln_x = ln_fixed(Fixed::from_u128_mag(base as u128, false).mul_u128(pow), w);
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    exp_fixed(y_w.mul(ln_x, w), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("powf kernel", SCALE)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // D38<37>: storage MAX ≈ 17.01 (full i128 range), so bases 10..17 are
    // representable but their squares are not — the integer fast path's
    // `base^|n|` intermediate overflows storage even when the result fits.
    const S: u32 = 37;
    const ONE: i128 = 10_i128.pow(S);
    const M: RoundingMode = RoundingMode::HalfToEven;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    fn powf(base: i128, exp: i128) -> i128 {
        powf_strict_raw::<S>(base * ONE, exp * ONE, M)
    }

    #[test]
    fn in_range_reciprocal_with_overflowing_intermediate_computes() {
        // `10^-2 = 0.01` is representable, but `10² = 100` overflows i128
        // storage at scale 37. The fast path must defer to the exact pin,
        // not panic on the intermediate.
        assert_eq!(powf(10, -2), ONE / 100);
        assert_eq!(powf(5, -3), ONE / 125); // 1/125 = 0.008
        assert_eq!(powf(16, -2), ONE / 256); // 1/256 = 0.00390625
    }

    #[track_caller]
    fn check_directed_exact<const SC: u32>(base: i128, exp: i128, divisor: i128) {
        let one = 10_i128.pow(SC);
        for mode in MODES {
            assert_eq!(
                powf_strict_raw::<SC>(base * one, exp * one, mode),
                one / divisor,
                "base={base} exp={exp} scale={SC} mode={mode:?}"
            );
        }
    }

    #[test]
    fn overflowing_intermediate_reciprocal_is_directed_exact() {
        // The defect: these reciprocals' scaled `base^|n|` intermediate
        // overflows i128, so the integer fast path returned `None` and
        // DEFERRED to the to-nearest `exp(y·ln x)` composition — which lands
        // ~1 ULP low, so `Floor` / `Trunc` rounded to the value one LSB BELOW
        // the exact power (e.g. `0.00999…9` for `0.01`). The exact integer pin
        // must return the on-grid value for EVERY mode, since each power lands
        // exactly on a storage grid line. Each base is representable at its
        // scale yet its scaled power overflows i128 (so the pin, not
        // `powi_raw_checked`, decides). Bases ≤ 17 at scale 37, larger at 36.
        check_directed_exact::<37>(10, -2, 100); // 0.01
        check_directed_exact::<37>(16, -2, 256); // 0.00390625
        check_directed_exact::<37>(4, -3, 64); // 0.015625
        check_directed_exact::<37>(5, -3, 125); // 0.008
        check_directed_exact::<36>(20, -2, 400); // 0.0025
        check_directed_exact::<36>(25, -2, 625); // 0.0016
        check_directed_exact::<36>(25, -3, 15_625); // 0.000064
    }

    #[test]
    fn exact_small_powers_unchanged() {
        // Cases the fast path always handled — must stay bit-identical.
        assert_eq!(powf(2, 3), 8 * ONE);
        assert_eq!(powf(2, -3), ONE / 8); // 0.125
        assert_eq!(powf(4, -2), ONE / 16); // 0.0625
        assert_eq!(powf(17, 1), 17 * ONE); // at the in-range edge
    }

    #[test]
    #[should_panic(expected = "result out of range")]
    fn out_of_range_positive_power_panics() {
        // `10² = 100` exceeds the storage MAX (≈17) — must panic, not wrap.
        let _ = powf(10, 2);
    }
}
