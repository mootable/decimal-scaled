//! D38 trigonometric kernels — sin / cos / tan / atan / asin / acos /
//! atan2 plus the hyperbolic family on the `Fixed` 256-bit
//! intermediate via `sin_fixed` / `atan_fixed`.
//!
//! Width-level specialisation for D38. Hosts the shared `Fixed`
//! primitives (`sin_fixed`, `atan_fixed`, `atan2_kernel`, `to_fixed`,
//! `wide_pi`, `wide_half_pi`, `small_x_linear_threshold`) plus every
//! D38 strict-trig + hyperbolic kernel. The typed-shell file in
//! `src/types/trig.rs` has no `crate::algos::*` or
//! `crate::algos::support::fixed_d38::*` references left — each public method
//! delegates one line through `policy::trig::TrigPolicy::*_impl`,
//! whose default body lives here.
//!
//! Fast paths preserved:
//! - `raw == 0` returns the appropriate identity (sin: 0, cos: 1, tan: 0).
//! - `|raw| <= small_x_linear_threshold` returns `raw` itself
//!   (linear band where the result is exact at storage precision).

use crate::algos::exp::fixed_d38::exp_fixed;
use crate::algos::support::fixed_d38::Fixed;
use crate::algos::ln::fixed_d38::{STRICT_GUARD, ln_fixed};
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, is_nearest_mode};
use crate::types::consts::DecimalConstants;
use crate::types::widths::D38;

// ── Int<2> entry points ─────────────────────────────────────────────
//
// The decimal storage type is `Int<2>`; the hand-tuned trig kernels do
// their math in `i128`. These thin wrappers bridge `Int<2> → i128` at
// the algorithm boundary — the `*_raw` cores below are unchanged — so
// `i128` never escapes this module. The policy layer forwards `self.0`
// (an `Int<2>`) straight to these entry points.
macro_rules! int2_trig {
    (strict $pub:ident, $core:ident) => {
        pub(crate) fn $pub<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
            Int::<2>::from_i128($core::<SCALE>(raw.as_i128(), mode))
        }
    };
    (with $pub:ident, $core:ident) => {
        pub(crate) fn $pub<const SCALE: u32>(
            raw: Int<2>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> Int<2> {
            Int::<2>::from_i128($core::<SCALE>(raw.as_i128(), working_digits, mode))
        }
    };
    (with_scale $pub:ident, $core:ident) => {
        pub(crate) fn $pub(
            raw: Int<2>,
            scale: u32,
            working_digits: u32,
            mode: RoundingMode,
        ) -> Int<2> {
            Int::<2>::from_i128($core(raw.as_i128(), scale, working_digits, mode))
        }
    };
    (atan2_strict $pub:ident, $core:ident) => {
        pub(crate) fn $pub<const SCALE: u32>(
            y_raw: Int<2>,
            x_raw: Int<2>,
            mode: RoundingMode,
        ) -> Int<2> {
            Int::<2>::from_i128($core::<SCALE>(y_raw.as_i128(), x_raw.as_i128(), mode))
        }
    };
    (atan2_with $pub:ident, $core:ident) => {
        pub(crate) fn $pub<const SCALE: u32>(
            y_raw: Int<2>,
            x_raw: Int<2>,
            working_digits: u32,
            mode: RoundingMode,
        ) -> Int<2> {
            Int::<2>::from_i128($core::<SCALE>(
                y_raw.as_i128(),
                x_raw.as_i128(),
                working_digits,
                mode,
            ))
        }
    };
}

int2_trig!(strict sin_strict, sin_strict_raw);
int2_trig!(strict cos_strict, cos_strict_raw);
int2_trig!(strict tan_strict, tan_strict_raw);
int2_trig!(strict atan_strict, atan_strict_raw);
int2_trig!(strict asin_strict, asin_strict_raw);
int2_trig!(strict acos_strict, acos_strict_raw);
int2_trig!(atan2_strict atan2_strict, atan2_strict_raw);
int2_trig!(with sin_with, sin_with_raw);
int2_trig!(with cos_with, cos_with_raw);
int2_trig!(with tan_with, tan_with_raw);
int2_trig!(with atan_with, atan_with_raw);
int2_trig!(with asin_with, asin_with_raw);
int2_trig!(with acos_with, acos_with_raw);
int2_trig!(atan2_with atan2_with, atan2_with_raw);
int2_trig!(with_scale sinh_with, sinh_with_raw);
int2_trig!(with_scale cosh_with, cosh_with_raw);
int2_trig!(with_scale tanh_with, tanh_with_raw);
int2_trig!(with_scale asinh_with, asinh_with_raw);
int2_trig!(with_scale acosh_with, acosh_with_raw);
int2_trig!(with_scale atanh_with, atanh_with_raw);
int2_trig!(with_scale to_degrees_with, to_degrees_with_raw);
int2_trig!(with_scale to_radians_with, to_radians_with_raw);

// ── Shared Fixed primitives ────────────────────────────────────────

/// Threshold below which the linear small-x fast paths fire for the
/// odd trig functions (`atan`, `sin`, `tan`, `sinh`, `tanh`, `asin`,
/// `asinh`, `atanh`).
///
/// All these functions have a Taylor series `f(x) = x + c·x³ + …`
/// where `|c| ≤ 1/3`. For `|x| < (1.5·10⁻ˢᶜᴬᴸᴱ)^(1/3) ≈ 10^(−⌈SCALE/3⌉)`
/// the cubic correction is bounded by `0.5·ULP` and `f(x) == x`
/// exactly at the storage scale. The threshold returned here is the
/// conservative integer `10^(SCALE − ⌈(SCALE+2)/3⌉)` in storage
/// units (one decimal digit safety margin from the exact bound).
#[inline]
pub(crate) const fn small_x_linear_threshold<const SCALE: u32>() -> i128 {
    let thresh_exp = SCALE.saturating_sub((SCALE + 2) / 3);
    10_i128.pow(thresh_exp)
}

/// π at working scale `w`, sourced from the crate-wide 75-digit
/// `consts::PI_RAW` (Int<4> holding `π × 10^75`).
pub(crate) fn wide_pi(w: u32) -> Fixed {
    debug_assert!(
        w <= 75,
        "wide_pi: working scale {w} exceeds embedded 75-digit π"
    );
    let words = crate::types::consts::PI_RAW.limbs_le();
    let pi_at_75 = Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
    };
    if w == 75 {
        pi_at_75
    } else {
        pi_at_75.rescale_down(75, w)
    }
}

/// τ = 2π at working scale `w`.
fn wide_tau(w: u32) -> Fixed {
    wide_pi(w).double()
}

/// π/2 at working scale `w`.
pub(crate) fn wide_half_pi(w: u32) -> Fixed {
    wide_pi(w).halve()
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^STRICT_GUARD`, carrying the sign.
pub(crate) fn to_fixed(raw: i128) -> Fixed {
    to_fixed_w(raw, STRICT_GUARD)
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^working_digits`, carrying the sign. Used by the `_approx`
/// variants where the guard width is chosen at runtime.
pub(crate) fn to_fixed_w(raw: i128, working_digits: u32) -> Fixed {
    let m = Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(working_digits));
    if raw < 0 { m.neg() } else { m }
}

/// Shared `atan2` body factored out so the `_strict` and `_approx`
/// dispatchers can compose it at their chosen working scale `w`.
/// `y_raw` keeps the original sign of the y-argument for the x-zero
/// branch where the wide y value would have been signed-zero.
pub(crate) fn atan2_kernel(y: Fixed, x: Fixed, y_raw: i128, w: u32) -> Fixed {
    if x.is_zero() {
        return if y_raw > 0 {
            wide_half_pi(w)
        } else if y_raw < 0 {
            wide_half_pi(w).neg()
        } else {
            Fixed::ZERO
        };
    }
    // Max-branch: feed atan_fixed the |smaller|/|larger| ratio so the
    // argument-halving cascade doesn't blow up when |y| ≫ |x|.
    let abs_y_ge_abs_x = y.ge_mag(x);
    let base = if !abs_y_ge_abs_x {
        atan_fixed(y.div(x, w), w)
    } else {
        let inv = atan_fixed(x.div(y, w), w);
        let hp = wide_half_pi(w);
        let same_sign = y.negative == x.negative;
        if same_sign {
            hp.sub(inv)
        } else {
            hp.neg().sub(inv)
        }
    };
    if !x.negative {
        base
    } else if !y.negative {
        base.add(wide_pi(w))
    } else {
        base.sub(wide_pi(w))
    }
}

/// Taylor series for `sin` on a reduced non-negative argument
/// `r ∈ [0, π/4]`, evaluated at working scale `w`.
fn sin_taylor(r: Fixed, w: u32) -> Fixed {
    let r2 = r.mul(r, w);
    let mut sum = r;
    let mut term = r; // term = r^(2k-1)
    let mut k: u128 = 1;
    loop {
        // term_k = term_{k-1} · r² / ((2k)(2k+1)); sign alternates.
        term = term.mul(r2, w).div_small((2 * k) * (2 * k + 1));
        if term.is_zero() {
            break;
        }
        if k % 2 == 1 {
            sum = sum.sub(term);
        } else {
            sum = sum.add(term);
        }
        k += 1;
        if k > 200 {
            break;
        }
    }
    sum
}

/// Taylor series for `cos` on a reduced non-negative argument
/// `r ∈ [0, π/4]`, evaluated at working scale `w`.
///
/// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
///
/// Mirrors the wide-tier `cos_taylor`: at the same `r` cos converges
/// faster than sin because the leading `1` dominates the small
/// even-power corrections. Used as the upper-half branch of
/// `sin_fixed` and `sin_cos_fixed` when the reduced argument exceeds
/// π/4 — splitting `[0, π/2]` at π/4 roughly halves the worst-case
/// Taylor term count.
fn cos_taylor(r: Fixed, w: u32) -> Fixed {
    let r2 = r.mul(r, w);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let mut sum = one_w;
    let mut term = one_w;
    let mut k: u128 = 1;
    loop {
        // term_k = term_{k-1} · r² / ((2k-1)(2k)); sign alternates.
        term = term.mul(r2, w).div_small((2 * k - 1) * (2 * k));
        if term.is_zero() {
            break;
        }
        if k % 2 == 1 {
            sum = sum.sub(term);
        } else {
            sum = sum.add(term);
        }
        k += 1;
        if k > 200 {
            break;
        }
    }
    sum
}

/// Sine of a working-scale value `v_w`, at working scale `w`.
///
/// Reduces `v` modulo τ via `q = round(v/τ)`, folds the remainder into
/// `[0, π/2]` tracking sign and the `π − x` reflection, then routes
/// to `sin_taylor` for `r ≤ π/4` or `cos_taylor(π/2 − r)` for the
/// upper half — the π/4 split roughly halves the Taylor term count
/// versus a single `[0, π/2]` series.
pub(crate) fn sin_fixed(v_w: Fixed, w: u32) -> Fixed {
    let tau = wide_tau(w);
    let pi = wide_pi(w);
    let half_pi = wide_half_pi(w);
    let quarter_pi = half_pi.halve();

    // r = v - round(v/τ)·τ ∈ [-π, π].
    let q = v_w.div(tau, w).round_to_nearest_int(w);
    let q_tau = if q >= 0 {
        tau.mul_u128(q as u128)
    } else {
        tau.mul_u128((-q) as u128).neg()
    };
    let r = v_w.sub(q_tau);

    // Fold |r| ∈ [0, π] into [0, π/2] via sin(π − x) = sin(x).
    let sign = r.negative;
    let abs_r = Fixed {
        negative: false,
        mag: r.mag,
    };
    let reduced = if abs_r.ge_mag(half_pi) {
        pi.sub(abs_r)
    } else {
        abs_r
    };
    // Pick the faster-converging branch at π/4.
    let s = if reduced.ge_mag(quarter_pi) {
        // sin(reduced) = cos(π/2 − reduced); the cos arg ∈ [0, π/4].
        cos_taylor(half_pi.sub(reduced), w)
    } else {
        sin_taylor(reduced, w)
    };
    if sign { s.neg() } else { s }
}

/// Joint sine + cosine of a working-scale value `v_w`, at working
/// scale `w`.
///
/// Shares the mod-τ argument reduction between sin and cos — one
/// reduction (1 wide divide + 1 round-to-int + 1 multiply-back +
/// 1 sub), then two Taylor evaluations on the reduced argument.
///
/// The naive `sin_fixed(v) + sin_fixed(v + π/2)` pays the reduction
/// twice. Recovering `cos` via the Pythagorean identity
/// `|cos| = √(1 − sin²)` was tried — a 256-bit Fixed sqrt is far
/// more expensive than a second Taylor at this width, so the joint
/// kernel sticks with two Taylors after the shared reduction.
pub(crate) fn sin_cos_fixed(v_w: Fixed, w: u32) -> (Fixed, Fixed) {
    let tau = wide_tau(w);
    let pi = wide_pi(w);
    let half_pi = wide_half_pi(w);
    let quarter_pi = half_pi.halve();

    let q = v_w.div(tau, w).round_to_nearest_int(w);
    let q_tau = if q >= 0 {
        tau.mul_u128(q as u128)
    } else {
        tau.mul_u128((-q) as u128).neg()
    };
    let r = v_w.sub(q_tau);

    // Sin: fold |r| ∈ [0, π] to [0, π/2] via sin(π − x) = sin(x);
    // sign comes from the residue sign.
    let sin_neg = r.negative;
    let abs_r = Fixed {
        negative: false,
        mag: r.mag,
    };
    let cos_neg = abs_r.ge_mag(half_pi); // |r| > π/2 ⇒ cos negative.
    let sin_reduced = if cos_neg { pi.sub(abs_r) } else { abs_r };
    let s_abs = if sin_reduced.ge_mag(quarter_pi) {
        cos_taylor(half_pi.sub(sin_reduced), w)
    } else {
        sin_taylor(sin_reduced, w)
    };

    // Cos: |cos(r)| = sin(π/2 − sin_reduced) — same π/4 split.
    // sin_reduced is in [0, π/2], so π/2 − sin_reduced is also in
    // [0, π/2] and the π/4 branch logic is just inverted.
    let cos_reduced = half_pi.sub(sin_reduced);
    let c_abs = if cos_reduced.ge_mag(quarter_pi) {
        cos_taylor(half_pi.sub(cos_reduced), w)
    } else {
        sin_taylor(cos_reduced, w)
    };

    let sin_result = if sin_neg { s_abs.neg() } else { s_abs };
    let cos_result = if cos_neg { c_abs.neg() } else { c_abs };
    (sin_result, cos_result)
}

/// Taylor series for `atan` on a reduced non-negative argument
/// `x ∈ [0, ~1/8]`, evaluated at working scale `w`.
fn atan_taylor(x: Fixed, w: u32) -> Fixed {
    let x2 = x.mul(x, w);
    let mut sum = x;
    let mut term = x; // term = x^(2k-1)
    let mut k: u128 = 1;
    loop {
        term = term.mul(x2, w);
        let contrib = term.div_small(2 * k + 1);
        if contrib.is_zero() {
            break;
        }
        if k % 2 == 1 {
            sum = sum.sub(contrib);
        } else {
            sum = sum.add(contrib);
        }
        k += 1;
        if k > 300 {
            break;
        }
    }
    sum
}

/// Arctangent of a working-scale value `v_w`, at working scale `w`,
/// result in `(−π/2, π/2)`.
///
/// Odd-function fold to `x ≥ 0`; reciprocal reduction
/// `atan(x) = π/2 − atan(1/x)` for `x > 1`; up to 8 rounds of
/// argument halving `atan(x) = 2·atan(x / (1 + √(1+x²)))`; then the
/// series.
pub(crate) fn atan_fixed(v_w: Fixed, w: u32) -> Fixed {
    #[cfg(feature = "perf-trace")]
    let _atan_span = ::tracing::info_span!("atan_fixed").entered();

    #[cfg(feature = "perf-trace")]
    let _setup_span = ::tracing::info_span!("setup").entered();
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let sign = v_w.negative;
    let mut x = Fixed {
        negative: false,
        mag: v_w.mag,
    };
    let mut add_half_pi = false;
    if x.ge_mag(one_w) && x != one_w {
        x = one_w.div(x, w); // atan(x) = π/2 − atan(1/x)
        add_half_pi = true;
    }
    #[cfg(feature = "perf-trace")]
    drop(_setup_span);

    // Adaptive argument halvings: atan(x) = 2·atan(x / (1 + √(1+x²))).
    // Halve only while |x| > ~0.2 (the Taylor convergence target);
    // matches g_math's adaptive-halvings approach. Skips halvings
    // entirely when the input is already small. Hard cap at 8 halvings
    // as a safety net against pathological edge cases.
    #[cfg(feature = "perf-trace")]
    let _halvings_span = ::tracing::info_span!("halvings").entered();
    let halving_threshold = one_w.div_small(5); // 0.2 at scale w
    let mut halvings: u32 = 0;
    while x.ge_mag(halving_threshold) && halvings < 8 {
        let x2 = x.mul(x, w);
        let denom = one_w.add(one_w.add(x2).sqrt(w));
        x = x.div(denom, w);
        halvings += 1;
    }
    #[cfg(feature = "perf-trace")]
    drop(_halvings_span);

    #[cfg(feature = "perf-trace")]
    let _taylor_span = ::tracing::info_span!("taylor").entered();
    let mut result = atan_taylor(x, w);
    #[cfg(feature = "perf-trace")]
    drop(_taylor_span);

    #[cfg(feature = "perf-trace")]
    let _reasm_span = ::tracing::info_span!("reassemble").entered();
    result = result.shl(halvings);
    if add_half_pi {
        result = wide_half_pi(w).sub(result);
    }
    if sign { result.neg() } else { result }
}

// ── sin ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn sin_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    sin_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sin", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn sin_with_raw<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + working_digits;
    sin_fixed(to_fixed_w(raw, working_digits), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sin", SCALE))
}

// ── cos ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn cos_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let arg = to_fixed(raw).add(wide_half_pi(w));
    sin_fixed(arg, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cos", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn cos_with_raw<const SCALE: u32>(
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
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cos", SCALE))
}

// ── tan ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn tan_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    let (sin_w, cos_w) = sin_cos_fixed(to_fixed(raw), w);
    assert!(
        !cos_w.is_zero(),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    sin_w
        .div(cos_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tan", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn tan_with_raw<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + working_digits;
    let (sin_w, cos_w) = sin_cos_fixed(to_fixed_w(raw, working_digits), w);
    assert!(
        !cos_w.is_zero(),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    sin_w
        .div(cos_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tan", SCALE))
}

// ── atan ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return <D38<SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw == -one_bits {
        return -<D38<SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    atan_fixed(to_fixed(raw), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn atan_with_raw<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return <D38<SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw == -one_bits {
        return -<D38<SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + working_digits;
    atan_fixed(to_fixed_w(raw, working_digits), w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan", SCALE))
}

// ── asin ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn asin_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed(raw);
    let abs_v = Fixed {
        negative: false,
        mag: v.mag,
    };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "asin: argument out of domain [-1, 1]"
    );
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        let hp = if v.negative { hp.neg() } else { hp };
        return hp.round_to_i128_with(w, SCALE, mode).unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE)
        });
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
        if v.negative {
            result_abs.neg()
        } else {
            result_abs
        }
    };
    asin_w
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn asin_with_raw<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let w = SCALE + working_digits;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed_w(raw, working_digits);
    let abs_v = Fixed {
        negative: false,
        mag: v.mag,
    };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "asin: argument out of domain [-1, 1]"
    );
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        let hp = if v.negative { hp.neg() } else { hp };
        return hp.round_to_i128_with(w, SCALE, mode).unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE)
        });
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
        if v.negative {
            result_abs.neg()
        } else {
            result_abs
        }
    };
    asin_w
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE))
}

// ── acos ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn acos_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return <D38<SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <D38<SCALE> as DecimalConstants>::pi().0.as_i128();
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed(raw);
    let abs_v = Fixed {
        negative: false,
        mag: v.mag,
    };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "acos: argument out of domain [-1, 1]"
    );
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
        if v.negative {
            result_abs.neg()
        } else {
            result_abs
        }
    };
    wide_half_pi(w)
        .sub(asin_w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("acos", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn acos_with_raw<const SCALE: u32>(
    raw: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return <D38<SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <D38<SCALE> as DecimalConstants>::pi().0.as_i128();
    }
    let w = SCALE + working_digits;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed_w(raw, working_digits);
    let abs_v = Fixed {
        negative: false,
        mag: v.mag,
    };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "acos: argument out of domain [-1, 1]"
    );
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
        if v.negative {
            result_abs.neg()
        } else {
            result_abs
        }
    };
    wide_half_pi(w)
        .sub(asin_w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("acos", SCALE))
}

// ── atan2 ──────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan2_strict_raw<const SCALE: u32>(y_raw: i128, x_raw: i128, mode: RoundingMode) -> i128 {
    let w = SCALE + STRICT_GUARD;
    atan2_kernel(to_fixed(y_raw), to_fixed(x_raw), y_raw, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan2", SCALE))
}

#[inline]
#[must_use]
pub(crate) fn atan2_with_raw<const SCALE: u32>(
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
    .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan2", SCALE))
}

// ── Hyperbolic family ─────────────────────────────────────────────
//
// sinh / cosh / tanh / asinh / acosh / atanh kernels on the `Fixed`
// 256-bit intermediate. The typed-shell file in `src/types/trig.rs`
// delegates one line to each `TrigPolicy::*_impl`, whose default body
// resolves to the matching kernel below.

#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    sinh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn sinh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) {
        // sinh(x) = x + x³/6 + … : within the linear band the cubic is
        // below one ULP yet strictly positive, so the true value sits
        // just *above* the grid line `raw` (in magnitude). Nearest modes
        // return `raw`; the directed modes need the analytic decision —
        // no finite-precision exp path can resolve the sub-ULP cubic.
        return crate::support::rounding::tiny_odd_expanding_directed(raw, 0, 1, mode);
    }
    let w = scale + working_digits;
    let v = to_fixed_w(raw, working_digits);
    // Evaluate at |v| so the dominant `e^|x|` term is computed directly
    // and accurately; the reciprocal gives the tiny `e^-|x|`. (Computing
    // `exp(-|x|)` directly and reciprocating would amplify the small
    // term's relative error into a large absolute error.) sinh is odd,
    // so the input sign is reapplied to the non-negative `sinh(|x|)`.
    let neg = raw < 0;
    let av = Fixed {
        negative: false,
        mag: v.mag,
    };
    let ex = exp_fixed(av, w);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let enx = one_w.div(ex, w);
    let sh = ex.sub(enx).halve();
    let sh = if neg { sh.neg() } else { sh };
    sh.round_to_i128_with(w, scale, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::sinh", scale)
    })
}

#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    cosh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn cosh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale);
    }
    let w = scale + working_digits;
    let v = to_fixed_w(raw, working_digits);
    // cosh is even; evaluate at |v| so the dominant `e^|x|` term is
    // computed directly (see `sinh_with` for why the sign matters: a
    // negative argument would otherwise reciprocate the small
    // `e^-|x|` and amplify its relative error).
    let av = Fixed {
        negative: false,
        mag: v.mag,
    };
    let ex = exp_fixed(av, w);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let enx = one_w.div(ex, w);
    ex.add(enx)
        .halve()
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::cosh", scale)
        })
}

#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    tanh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn tanh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) {
        // tanh(x) = x − x³/3 + … : within the linear band the cubic is
        // below one ULP yet strictly positive, so the true value sits
        // just inside the grid line `raw`. Nearest modes return `raw`;
        // the directed modes need the analytic decision below — no
        // finite-precision exp path can resolve the sub-ULP cubic.
        return crate::support::rounding::tiny_odd_compressing_directed(raw, 0, 1, mode);
    }
    let w = scale + working_digits;
    let v = to_fixed_w(raw, working_digits);
    // tanh is odd; evaluate at |v| (dominant `e^|x|` direct, see
    // `sinh_with`) and reapply the input sign.
    let neg = raw < 0;
    let av = Fixed {
        negative: false,
        mag: v.mag,
    };
    let ex = exp_fixed(av, w);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let enx = one_w.div(ex, w);
    let th = ex.sub(enx).div(ex.add(enx), w);
    let th = if neg { th.neg() } else { th };
    th.round_to_i128_with(w, scale, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::tanh", scale)
    })
}

#[inline]
#[must_use]
pub(crate) fn asinh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    asinh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn asinh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) && is_nearest_mode(mode) {
        return raw;
    }
    let w = scale + working_digits;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed_w(raw, working_digits);
    let ax = Fixed {
        negative: false,
        mag: v.mag,
    };
    let inner = if ax.ge_mag(one_w) {
        let inv = one_w.div(ax, w);
        let root = one_w.add(inv.mul(inv, w)).sqrt(w);
        ln_fixed(ax, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = ax.mul(ax, w).add(one_w).sqrt(w);
        ln_fixed(ax.add(root), w)
    };
    let signed = if raw < 0 { inner.neg() } else { inner };
    signed
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::asinh", scale)
        })
}

#[inline]
#[must_use]
pub(crate) fn acosh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    acosh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn acosh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    let one_bits: i128 = 10_i128.pow(scale);
    if raw == one_bits {
        return 0;
    }
    let w = scale + working_digits;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed_w(raw, working_digits);
    assert!(
        !v.negative && v.ge_mag(one_w),
        "D38::acosh: argument must be >= 1"
    );
    let two_w = one_w.double();
    let inner = if v.ge_mag(two_w) {
        let inv = one_w.div(v, w);
        let root = one_w.sub(inv.mul(inv, w)).sqrt(w);
        ln_fixed(v, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = v.mul(v, w).sub(one_w).sqrt(w);
        ln_fixed(v.add(root), w)
    };
    inner.round_to_i128_with(w, scale, mode).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::acosh", scale)
    })
}

#[inline]
#[must_use]
pub(crate) fn atanh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    atanh_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn atanh_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) && is_nearest_mode(mode) {
        return raw;
    }
    let w = scale + working_digits;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let v = to_fixed_w(raw, working_digits);
    let ax = Fixed {
        negative: false,
        mag: v.mag,
    };
    assert!(
        !ax.ge_mag(one_w),
        "D38::atanh: argument out of domain (-1, 1)"
    );
    let ratio = one_w.add(v).div(one_w.sub(v), w);
    ln_fixed(ratio, w)
        .halve()
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::atanh", scale)
        })
}

// ── Angle conversions ─────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn to_degrees_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    to_degrees_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn to_degrees_with_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = scale + working_digits;
    to_fixed_w(raw, working_digits)
        .mul_u128(180)
        .div(wide_pi(w), w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::to_degrees", scale)
        })
}

#[inline]
#[must_use]
pub(crate) fn to_radians_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    to_radians_with(raw, SCALE, STRICT_GUARD, mode)
}

#[inline]
#[must_use]
pub(crate) fn to_radians_with_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = scale + working_digits;
    to_fixed_w(raw, working_digits)
        .mul(wide_pi(w), w)
        .div_small(180)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::to_radians", scale)
        })
}

// ── Runtime-scale small-x threshold ───────────────────────────────

/// Runtime-scale companion to [`small_x_linear_threshold`]. Used by
/// the `_with` kernels where `scale` is a runtime value rather than
/// a const generic.
#[inline]
fn small_x_linear_threshold_scale(scale: u32) -> i128 {
    let thresh_exp = scale.saturating_sub((scale + 2) / 3);
    10_i128.pow(thresh_exp)
}
