//! D38 trigonometric kernels — sin / cos / tan on the `Fixed`
//! 256-bit intermediate via `sin_fixed`.
//!
//! Width-level specialisation for D38. Fast paths preserved:
//! - `raw == 0` returns the appropriate identity (sin: 0, cos: 1, tan: 0).
//! - `|raw| <= small_x_linear_threshold` returns `raw` itself
//!   (linear band where the result is exact at storage precision).

use crate::log_exp_strict::STRICT_GUARD;
use crate::rounding::RoundingMode;
use crate::trig_strict::{sin_fixed, small_x_linear_threshold, to_fixed, to_fixed_w, wide_half_pi};

// ── sin ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn sin_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    sin_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("sin: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn sin_with<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + working_digits;
    sin_fixed(to_fixed_w(raw, working_digits), w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("sin: result out of range")
}

// ── cos ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn cos_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let arg = to_fixed(raw).add(wide_half_pi(w));
    sin_fixed(arg, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("cos: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn cos_with<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + working_digits;
    let arg = to_fixed_w(raw, working_digits).add(wide_half_pi(w));
    sin_fixed(arg, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("cos: result out of range")
}

// ── tan ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn tan_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let sin_w = sin_fixed(v, w);
    let cos_w = sin_fixed(v.add(wide_half_pi(w)), w);
    assert!(
        !cos_w.is_zero(),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    sin_w
        .div(cos_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("tan: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn tan_with<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + working_digits;
    let v = to_fixed_w(raw, working_digits);
    let sin_w = sin_fixed(v, w);
    let cos_w = sin_fixed(v.add(wide_half_pi(w)), w);
    assert!(
        !cos_w.is_zero(),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    sin_w
        .div(cos_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("tan: result out of range")
}
