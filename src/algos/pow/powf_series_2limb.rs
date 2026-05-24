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
//!   route to `powi_raw::<SCALE>` — exact via square-and-multiply on raw
//!   ~10–500× faster than the `exp(y·ln(x))` chain.
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

/// Integer-exponent square-and-multiply on raw `i128` storage at
/// `SCALE`, using `mul_widen_divide` directly instead of routing through
/// the decimal `Mul` operator (which would re-enter the mul policy from
/// inside an algorithm fn - the layering inversion). `ONE_S` is
/// `10^SCALE`; passing it avoids recomputing the constant inside the loop.
///
/// For negative `n`, returns `ONE_S / base^|n|` via the same path, but
/// uses the decimal `div_widen_scale` for the final reciprocal since
/// that is a genuine downward cross-tier call to the int layer.
#[inline]
fn powi_raw<const SCALE: u32>(base: i128, n: i32, mode: RoundingMode) -> i128 {
    let one_s: Int<2> = Int::<2>::TEN.pow(SCALE);
    if n == 0 {
        return one_s.as_i128();
    }
    let pos_n = n.unsigned_abs();
    // Square-and-multiply using mul_widen_divide kernel directly.
    let mut acc: Int<2> = one_s;
    let mut b: Int<2> = Int::<2>::from_i128(base);
    let mut e = pos_n;
    while e > 0 {
        if e & 1 == 1 {
            acc = crate::algos::mul::mul_widen_divide::mul_widen_divide::<2, SCALE>(
                acc, b, mode,
            );
        }
        e >>= 1;
        if e > 0 {
            b = crate::algos::mul::mul_widen_divide::mul_widen_divide::<2, SCALE>(
                b, b, mode,
            );
        }
    }
    if n > 0 {
        acc.as_i128()
    } else {
        // Reciprocal: one_s / acc using div_widen_scale kernel.
        crate::algos::div::div_widen_scale::div_widen_scale::<2>(
            one_s, acc, Int::<2>::TEN.pow(SCALE), mode,
        ).as_i128()
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
        return powi_raw::<SCALE>(base, n, mode);
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
        return powi_raw::<SCALE>(base, n, mode);
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
