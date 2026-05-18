//! D38 trigonometric kernels — sin / cos / tan on the `Fixed`
//! 256-bit intermediate via `sin_fixed`.
//!
//! Width-level specialisation for D38. Fast paths preserved:
//! - `raw == 0` returns the appropriate identity (sin: 0, cos: 1, tan: 0).
//! - `|raw| <= small_x_linear_threshold` returns `raw` itself
//!   (linear band where the result is exact at storage precision).

use crate::consts::DecimalConstants;
use crate::core_type::D38;
use crate::log_exp_strict::STRICT_GUARD;
use crate::rounding::RoundingMode;
use crate::trig_strict::{
    atan2_kernel, atan_fixed, sin_fixed, small_x_linear_threshold, to_fixed, to_fixed_w,
    wide_half_pi,
};

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

// ── atan ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return <D38<SCALE> as DecimalConstants>::quarter_pi().0;
    }
    if raw == -one_bits {
        return -<D38<SCALE> as DecimalConstants>::quarter_pi().0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    atan_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("atan: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn atan_with<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return <D38<SCALE> as DecimalConstants>::quarter_pi().0;
    }
    if raw == -one_bits {
        return -<D38<SCALE> as DecimalConstants>::quarter_pi().0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    let w = SCALE + working_digits;
    atan_fixed(to_fixed_w(raw, working_digits), w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("atan: result out of range")
}

// ── asin ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn asin_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() {
        return raw;
    }
    use crate::d_w128_kernels::Fixed;
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "asin: argument out of domain [-1, 1]");
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        let hp = if v.negative { hp.neg() } else { hp };
        return hp
            .round_to_i128_with(w, SCALE, mode)
            .expect("asin: result out of range");
    }
    let half_w = one_w.halve();
    let asin_w = if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    };
    asin_w
        .round_to_i128_with(w, SCALE, mode)
        .expect("asin: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn asin_with<const SCALE: u32>(
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
    use crate::d_w128_kernels::Fixed;
    let w = SCALE + working_digits;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed_w(raw, working_digits);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "asin: argument out of domain [-1, 1]");
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        let hp = if v.negative { hp.neg() } else { hp };
        return hp
            .round_to_i128_with(w, SCALE, mode)
            .expect("asin: result out of range");
    }
    let half_w = one_w.halve();
    let asin_w = if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    };
    asin_w
        .round_to_i128_with(w, SCALE, mode)
        .expect("asin: result out of range")
}

// ── acos ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn acos_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return <D38<SCALE> as DecimalConstants>::half_pi().0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <D38<SCALE> as DecimalConstants>::pi().0;
    }
    use crate::d_w128_kernels::Fixed;
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "acos: argument out of domain [-1, 1]");
    let half_w = one_w.halve();
    let asin_w = if abs_v == one_w {
        let hp = wide_half_pi(w);
        if v.negative { hp.neg() } else { hp }
    } else if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    };
    wide_half_pi(w)
        .sub(asin_w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("acos: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn acos_with<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return <D38<SCALE> as DecimalConstants>::half_pi().0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <D38<SCALE> as DecimalConstants>::pi().0;
    }
    use crate::d_w128_kernels::Fixed;
    let w = SCALE + working_digits;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed_w(raw, working_digits);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(!(abs_v.ge_mag(one_w) && abs_v != one_w), "acos: argument out of domain [-1, 1]");
    let half_w = one_w.halve();
    let asin_w = if abs_v == one_w {
        let hp = wide_half_pi(w);
        if v.negative { hp.neg() } else { hp }
    } else if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    };
    wide_half_pi(w)
        .sub(asin_w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("acos: result out of range")
}

// ── atan2 ──────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan2_strict<const SCALE: u32>(
    y_raw: i128,
    x_raw: i128,
    mode: RoundingMode,
) -> i128 {
    let w = SCALE + STRICT_GUARD;
    atan2_kernel(to_fixed(y_raw), to_fixed(x_raw), y_raw, w)
        .round_to_i128_with(w, SCALE, mode)
        .expect("atan2: result out of range")
}

#[inline]
#[must_use]
pub(crate) fn atan2_with<const SCALE: u32>(
    y_raw: i128,
    x_raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let w = SCALE + working_digits;
    atan2_kernel(
        to_fixed_w(y_raw, working_digits),
        to_fixed_w(x_raw, working_digits),
        y_raw,
        w,
    )
        .round_to_i128_with(w, SCALE, mode)
        .expect("atan2: result out of range")
}
