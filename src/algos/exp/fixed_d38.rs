//! D38 exponential kernel — `exp_fixed` on the 256-bit `Fixed`
//! intermediate, parameterised by working-digit guard.
//!
//! Width-level specialisation for D38, capturing the hand-tuned path
//! that has shipped since before the algorithm library existed.
//! Returns the raw `i128` storage at the input's scale; the typed
//! method shell handles the panic-on-overflow message.
//!
//! Hosts the shared `Fixed` exp primitive used by the `ExpPolicy`
//! defaults (`exp_fixed`) so the typed-shell file has no
//! `crate::algos::*` or `crate::d_w128_kernels::*` references left.

use crate::algos::ln::fixed_d38::{STRICT_GUARD, wide_ln2};
use crate::d_w128_kernels::Fixed;
use crate::rounding::RoundingMode;

/// `e` raised to a working-scale value `v_w`, returned at the same
/// working scale `w`.
///
/// Range-reduces `v = k·ln(2) + s` with `|s| ≤ ln(2)/2`, evaluates the
/// Taylor series for `exp(s)`, then reassembles `2^k · exp(s)` by
/// shifting the working-scale value (so the `2^k` factor never
/// amplifies a rounding error).
///
/// # Panics
///
/// Panics if `2^k · exp(s)` cannot fit a 256-bit working value — i.e.
/// the caller's result would overflow its representable range.
pub(crate) fn exp_fixed(v_w: Fixed, w: u32) -> Fixed {
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let ln2 = wide_ln2(w);

    // k = round(v / ln 2); s = v - k·ln(2), |s| <= ln(2)/2.
    let k = v_w.div(ln2, w).round_to_nearest_int(w);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    let s = v_w.sub(k_ln2);

    // Taylor series exp(s) = 1 + s + s²/2! + … — `term` carries sⁿ/n!.
    let mut sum = one_w;
    let mut term = one_w;
    let mut n: u128 = 1;
    loop {
        term = term.mul(s, w).div_small(n);
        if term.is_zero() {
            break;
        }
        sum = sum.add(term);
        n += 1;
        if n > 400 {
            break;
        }
    }

    // exp(v) = 2^k · exp(s).
    if k >= 0 {
        let shift = k as u32;
        assert!(sum.bit_length() + shift <= 256, "D38::exp: result overflows the representable range");
        sum.shl(shift)
    } else {
        sum.shr((-k) as u32)
    }
}

/// `e^x` with caller-chosen `working_digits` above the storage scale.
#[inline]
#[must_use]
pub(crate) fn exp_with(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale); // ONE for this scale
    }
    let w = scale + working_digits;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("exp kernel", scale))
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(STRICT_GUARD));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("exp kernel", SCALE))
}

// ── exp2 kernel (D38, Fixed fallback) ─────────────────────────────

/// `2^x = exp(x · ln 2)` on the `Fixed` intermediate. Used by
/// `ExpPolicy::exp2_impl` when the D57 borrow path is not available.
#[inline]
#[must_use]
pub(crate) fn exp2_with(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale);
    }
    let w = scale + working_digits;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false)
        .mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    let arg_w = v_w.mul(wide_ln2(w), w);
    exp_fixed(arg_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::diagnostics::overflow_panic_with_scale("D38::exp2", scale))
}

#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    exp2_with(raw, SCALE, STRICT_GUARD, mode)
}
