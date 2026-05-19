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
//! `crate::algos::*` or `crate::algos::fixed_d38::*` references left.

use crate::algos::ln::fixed_d38::{STRICT_GUARD, wide_ln2};
use crate::algos::fixed_d38::Fixed;
use crate::support::rounding::RoundingMode;

/// `e` raised to a working-scale value `v_w`, returned at the same
/// working scale `w`.
///
/// Range-reduces `v = k·ln(2) + s` with `|s| ≤ ln(2)/2`, halves `s`
/// `n` further times (`s_red = s / 2^n`), evaluates the Taylor
/// series for `exp(s_red)` on the much smaller argument, then squares
/// the result `n` times to recover `exp(s) = (exp(s_red))^(2^n)` —
/// classic Brent–Salamin "argument reduction + squaring" trick. `n`
/// is tuned so the Taylor cost (one mul + one div_small per term)
/// trades evenly against the `n` post-squarings (one wide mul each).
///
/// At `w = 44` decimal digits (D38 SCALE 19 + STRICT_GUARD = 25) the
/// naïve series wants ~25 iterations; halving with `n = 5` cuts that
/// to ~10 and adds five squarings — net ~30 % fewer wide multiplies.
///
/// Finally `2^k · exp(s)` is reassembled by shifting the working
/// value (so the `2^k` factor never amplifies a rounding error).
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

    // Argument halvings: pick `n` such that `(n+1)² ≤ 3w+1` — the
    // standard tuning where one extra halving saves roughly two
    // Taylor iterations but costs one final squaring. For w ≤ 44
    // this lands at n ∈ {4, 5, 6}.
    let p_bits = w.saturating_mul(3).saturating_add(1);
    let mut n: u32 = 1;
    while (n + 1) * (n + 1) <= p_bits {
        n += 1;
    }
    let s_red = s.shr(n);

    // Taylor series exp(s_red) = 1 + s_red + s_red²/2! + … on the
    // halved argument — `term` carries s_redⁱ/i!.
    let mut sum = one_w.add(s_red);
    let mut term = s_red;
    let mut i: u128 = 2;
    loop {
        term = term.mul(s_red, w).div_small(i);
        if term.is_zero() {
            break;
        }
        sum = sum.add(term);
        i += 1;
        if i > 400 {
            break;
        }
    }

    // Undo the n halvings: exp(s) = (exp(s_red))^(2^n) — `n` repeated
    // squarings.
    for _ in 0..n {
        sum = sum.mul(sum, w);
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
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("exp kernel", scale))
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
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("exp kernel", SCALE))
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
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("D38::exp2", scale))
}

#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    exp2_with(raw, SCALE, STRICT_GUARD, mode)
}
