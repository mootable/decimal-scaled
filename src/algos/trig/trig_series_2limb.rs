// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Trigonometric series kernels — sin / cos / tan / atan / asin / acos /
//! atan2 plus the hyperbolic family, evaluated on the 256-bit `Fixed`
//! guard-digit intermediate via `sin_fixed` / `atan_fixed`.
//!
//! The narrow `Int<2>`-storage series path: it serves the narrow
//! D18 / D38 tier, running the trig series in the wider `Fixed`
//! intermediate. Hosts the shared `Fixed` primitives (`sin_fixed`,
//! `atan_fixed`, `atan2_kernel`, `to_fixed`, `wide_pi`,
//! `wide_half_pi`, `small_x_linear_threshold`) plus every narrow-tier
//! strict-trig + hyperbolic kernel. The typed-shell file in
//! `src/types/trig.rs` has no `crate::algos::*` or
//! `crate::algos::support::fixed::*` references left — each public method
//! delegates one line through `policy::trig` dispatch,
//! whose default body lives here.
//!
//! Fast paths preserved:
//! - `raw == 0` returns the appropriate identity (sin: 0, cos: 1, tan: 0).
//! - `|raw| <= small_x_linear_threshold` returns `raw` itself
//!   (linear band where the result is exact at storage precision).

use crate::algos::exp::exp_generic as eg;
use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::narrow_ziv::{self, WZiv};
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
use crate::algos::trig::trig_generic as tg;
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, is_nearest_mode};
use crate::types::consts::DecimalConstants;

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
}

int2_trig!(strict sin_strict, sin_strict_raw);
int2_trig!(strict cos_strict, cos_strict_raw);
int2_trig!(strict tan_strict, tan_strict_raw);
int2_trig!(strict atan_strict, atan_strict_raw);
int2_trig!(strict asin_strict, asin_strict_raw);
int2_trig!(strict acos_strict, acos_strict_raw);
int2_trig!(atan2_strict atan2_strict, atan2_strict_raw);
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
///
/// At `SCALE == 0` the cubic-only bound `(1.5)^(1/3) ≈ 1.14` storage
/// units would admit `x = 1` radian, but the function is nowhere near
/// linear that far from the origin (`tan(1) = 1.557`, `asin(1) = π/2`),
/// so the only argument the linear identity holds for is the
/// already-pinned `x = 0`. A plain `saturating_sub` clamps the band
/// exponent to `0` and wrongly returns `10^0 = 1`; we special-case
/// `SCALE == 0` to `0` so no nonzero storage value short-circuits and the
/// full working-scale kernel runs. For `SCALE ≥ 1` the golden-validated
/// band stands.
#[inline]
pub(crate) const fn small_x_linear_threshold<const SCALE: u32>() -> i128 {
    if SCALE == 0 {
        return 0;
    }
    let threshold_exponent = SCALE.saturating_sub(SCALE.div_ceil(3));
    10_i128.pow(threshold_exponent)
}

/// π at `working_scale`, sourced DIRECTLY from the per-scale const
/// table (`consts::pi_const_n`) — `floor(π·10^working_scale)` rounded
/// half-to-even.
/// The ungated NARROW band covers `0..=512`, so this reads in every build
/// (default / no_std included); no per-call rescale, no embedded raw.
pub(crate) fn wide_pi(working_scale: u32) -> Fixed {
    debug_assert!(
        working_scale <= 75,
        "wide_pi: working scale {working_scale} exceeds Fixed capacity"
    );
    let words = crate::consts::pi_const_n::<4>(
        working_scale,
        crate::support::rounding::RoundingMode::HalfToEven,
    )
    .limbs_le();
    Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
    }
}

/// τ = 2π at `working_scale`.
fn wide_tau(working_scale: u32) -> Fixed {
    wide_pi(working_scale).double()
}

/// π/2 at `working_scale`.
pub(crate) fn wide_half_pi(working_scale: u32) -> Fixed {
    wide_pi(working_scale).halve()
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^STRICT_GUARD`, carrying the sign.
pub(crate) fn to_fixed(raw: i128) -> Fixed {
    to_fixed_w(raw, STRICT_GUARD)
}

/// Builds a working-scale `Fixed` from a signed `D38` raw value `r`:
/// `r · 10^working_digits`, carrying the sign. Used wherever the guard
/// width is a runtime value rather than the const `STRICT_GUARD` — the
/// Ziv escalation rungs and the angle-conversion kernels.
pub(crate) fn to_fixed_w(raw: i128, working_digits: u32) -> Fixed {
    let magnitude =
        Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(working_digits));
    if raw < 0 { magnitude.neg() } else { magnitude }
}

/// Shared `atan2` body factored out so a dispatcher can compose it at
/// its chosen `working_scale`.
/// `y_raw` keeps the original sign of the y-argument for the x-zero
/// branch where the wide y value would have been signed-zero.
pub(crate) fn atan2_kernel(y: Fixed, x: Fixed, y_raw: i128, working_scale: u32) -> Fixed {
    if x.is_zero() {
        return if y_raw > 0 {
            wide_half_pi(working_scale)
        } else if y_raw < 0 {
            wide_half_pi(working_scale).neg()
        } else {
            Fixed::ZERO
        };
    }
    // Max-branch: feed atan_fixed the |smaller|/|larger| ratio so the
    // argument-halving cascade doesn't blow up when |y| ≫ |x|.
    let abs_y_ge_abs_x = y.ge_mag(x);
    let base = if !abs_y_ge_abs_x {
        atan_fixed(y.div(x, working_scale), working_scale)
    } else {
        let atan_x_over_y = atan_fixed(x.div(y, working_scale), working_scale);
        let half_pi = wide_half_pi(working_scale);
        let same_sign = y.negative == x.negative;
        if same_sign {
            half_pi.sub(atan_x_over_y)
        } else {
            half_pi.neg().sub(atan_x_over_y)
        }
    };
    if !x.negative {
        base
    } else if !y.negative {
        base.add(wide_pi(working_scale))
    } else {
        base.sub(wide_pi(working_scale))
    }
}

/// Taylor series for `sin` on a reduced non-negative argument
/// `r ∈ [0, π/4]`, evaluated at `working_scale`.
fn sin_taylor(reduced_arg: Fixed, working_scale: u32) -> Fixed {
    let reduced_arg_squared = reduced_arg.mul(reduced_arg, working_scale);
    let mut sum = reduced_arg;
    let mut term = reduced_arg; // term = r^(2k-1)
    let mut term_index: u128 = 1;
    loop {
        // term_k = term_{k-1} · r² / ((2k)(2k+1)); sign alternates.
        term = term
            .mul(reduced_arg_squared, working_scale)
            .div_small((2 * term_index) * (2 * term_index + 1));
        if term.is_zero() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum.sub(term);
        } else {
            sum = sum.add(term);
        }
        term_index += 1;
        if term_index > 200 {
            break;
        }
    }
    sum
}

/// Taylor series for `cos` on a reduced non-negative argument
/// `r ∈ [0, π/4]`, evaluated at `working_scale`.
///
/// `cos(r) = 1 − r²/2! + r⁴/4! − r⁶/6! + …`
///
/// Mirrors the wide-tier `cos_taylor`: at the same `r` cos converges
/// faster than sin because the leading `1` dominates the small
/// even-power corrections. Used as the upper-half branch of
/// `sin_fixed` and `sin_cos_fixed` when the reduced argument exceeds
/// π/4 — splitting `[0, π/2]` at π/4 roughly halves the worst-case
/// Taylor term count.
fn cos_taylor(reduced_arg: Fixed, working_scale: u32) -> Fixed {
    let reduced_arg_squared = reduced_arg.mul(reduced_arg, working_scale);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let mut sum = one_w;
    let mut term = one_w;
    let mut term_index: u128 = 1;
    loop {
        // term_k = term_{k-1} · r² / ((2k-1)(2k)); sign alternates.
        term = term
            .mul(reduced_arg_squared, working_scale)
            .div_small((2 * term_index - 1) * (2 * term_index));
        if term.is_zero() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum.sub(term);
        } else {
            sum = sum.add(term);
        }
        term_index += 1;
        if term_index > 200 {
            break;
        }
    }
    sum
}

/// Sine of a `working_value`, at `working_scale`.
///
/// Reduces the value modulo τ via `tau_multiple = round(v/τ)`, folds the
/// remainder into
/// `[0, π/2]` tracking sign and the `π − x` reflection, then routes
/// to `sin_taylor` for `r ≤ π/4` or `cos_taylor(π/2 − r)` for the
/// upper half — the π/4 split roughly halves the Taylor term count
/// versus a single `[0, π/2]` series.
pub(crate) fn sin_fixed(working_value: Fixed, working_scale: u32) -> Fixed {
    let tau = wide_tau(working_scale);
    let pi = wide_pi(working_scale);
    let half_pi = wide_half_pi(working_scale);
    let quarter_pi = half_pi.halve();

    // r = v - round(v/τ)·τ ∈ [-π, π].
    let tau_multiple = working_value
        .div(tau, working_scale)
        .round_to_nearest_int(working_scale);
    let tau_offset = if tau_multiple >= 0 {
        tau.mul_u128(tau_multiple as u128)
    } else {
        tau.mul_u128((-tau_multiple) as u128).neg()
    };
    let residue = working_value.sub(tau_offset);

    // Fold |r| ∈ [0, π] into [0, π/2] via sin(π − x) = sin(x).
    let sin_neg = residue.negative;
    let abs_residue = Fixed {
        negative: false,
        mag: residue.mag,
    };
    let reduced = if abs_residue.ge_mag(half_pi) {
        pi.sub(abs_residue)
    } else {
        abs_residue
    };
    // Pick the faster-converging branch at π/4.
    let sin_abs = if reduced.ge_mag(quarter_pi) {
        // sin(reduced) = cos(π/2 − reduced); the cos arg ∈ [0, π/4].
        cos_taylor(half_pi.sub(reduced), working_scale)
    } else {
        sin_taylor(reduced, working_scale)
    };
    if sin_neg { sin_abs.neg() } else { sin_abs }
}

/// Joint sine + cosine of a `working_value`, at `working_scale`.
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
pub(crate) fn sin_cos_fixed(working_value: Fixed, working_scale: u32) -> (Fixed, Fixed) {
    let tau = wide_tau(working_scale);
    let pi = wide_pi(working_scale);
    let half_pi = wide_half_pi(working_scale);
    let quarter_pi = half_pi.halve();

    let tau_multiple = working_value
        .div(tau, working_scale)
        .round_to_nearest_int(working_scale);
    let tau_offset = if tau_multiple >= 0 {
        tau.mul_u128(tau_multiple as u128)
    } else {
        tau.mul_u128((-tau_multiple) as u128).neg()
    };
    let residue = working_value.sub(tau_offset);

    // Sin: fold |r| ∈ [0, π] to [0, π/2] via sin(π − x) = sin(x);
    // sign comes from the residue sign.
    let sin_neg = residue.negative;
    let abs_residue = Fixed {
        negative: false,
        mag: residue.mag,
    };
    let cos_neg = abs_residue.ge_mag(half_pi); // |r| > π/2 ⇒ cos negative.
    let sin_reduced = if cos_neg {
        pi.sub(abs_residue)
    } else {
        abs_residue
    };
    let sin_abs = if sin_reduced.ge_mag(quarter_pi) {
        cos_taylor(half_pi.sub(sin_reduced), working_scale)
    } else {
        sin_taylor(sin_reduced, working_scale)
    };

    // Cos: |cos(r)| = sin(π/2 − sin_reduced) — same π/4 split.
    // sin_reduced is in [0, π/2], so π/2 − sin_reduced is also in
    // [0, π/2] and the π/4 branch logic is just inverted.
    let cos_reduced = half_pi.sub(sin_reduced);
    let cos_abs = if cos_reduced.ge_mag(quarter_pi) {
        cos_taylor(half_pi.sub(cos_reduced), working_scale)
    } else {
        sin_taylor(cos_reduced, working_scale)
    };

    let sin_result = if sin_neg { sin_abs.neg() } else { sin_abs };
    let cos_result = if cos_neg { cos_abs.neg() } else { cos_abs };
    (sin_result, cos_result)
}

/// Taylor series for `atan` on a reduced non-negative argument
/// `x ∈ [0, ~1/8]`, evaluated at `working_scale`.
fn atan_taylor(reduced_arg: Fixed, working_scale: u32) -> Fixed {
    let reduced_arg_squared = reduced_arg.mul(reduced_arg, working_scale);
    let mut sum = reduced_arg;
    let mut term = reduced_arg; // term = x^(2k-1)
    let mut term_index: u128 = 1;
    loop {
        term = term.mul(reduced_arg_squared, working_scale);
        let contrib = term.div_small(2 * term_index + 1);
        if contrib.is_zero() {
            break;
        }
        if term_index % 2 == 1 {
            sum = sum.sub(contrib);
        } else {
            sum = sum.add(contrib);
        }
        term_index += 1;
        if term_index > 300 {
            break;
        }
    }
    sum
}

/// Arctangent of a `working_value`, at `working_scale`,
/// result in `(−π/2, π/2)`.
///
/// Odd-function fold to `x ≥ 0`; reciprocal reduction
/// `atan(x) = π/2 − atan(1/x)` for `x > 1`; up to 8 rounds of
/// argument halving `atan(x) = 2·atan(x / (1 + √(1+x²)))`; then the
/// series.
pub(crate) fn atan_fixed(working_value: Fixed, working_scale: u32) -> Fixed {
    #[cfg(feature = "perf-trace")]
    let _atan_span = ::tracing::info_span!("atan_fixed").entered();

    #[cfg(feature = "perf-trace")]
    let _setup_span = ::tracing::info_span!("setup").entered();
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let sign_neg = working_value.negative;
    let mut x = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    let mut add_half_pi = false;
    if x.ge_mag(one_w) && x != one_w {
        x = one_w.div(x, working_scale); // atan(x) = π/2 − atan(1/x)
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
    let halving_threshold = one_w.div_small(5); // 0.2 at the working scale
    let mut halvings: u32 = 0;
    while x.ge_mag(halving_threshold) && halvings < 8 {
        let x_squared = x.mul(x, working_scale);
        let denom = one_w.add(one_w.add(x_squared).sqrt(working_scale));
        x = x.div(denom, working_scale);
        halvings += 1;
    }
    #[cfg(feature = "perf-trace")]
    drop(_halvings_span);

    #[cfg(feature = "perf-trace")]
    let _taylor_span = ::tracing::info_span!("taylor").entered();
    let mut result = atan_taylor(x, working_scale);
    #[cfg(feature = "perf-trace")]
    drop(_taylor_span);

    #[cfg(feature = "perf-trace")]
    let _reasm_span = ::tracing::info_span!("reassemble").entered();
    result = result.shl(halvings);
    if add_half_pi {
        result = wide_half_pi(working_scale).sub(result);
    }
    if sign_neg { result.neg() } else { result }
}

// ── Near-tie Ziv escalation (the narrow walker recomputes) ─────────
//
// The strict kernels narrow their `Fixed` single shot through
// `Fixed::round_to_i128_clear_of_tie`: a residual clear of the mode's
// deciding boundary exits at today's cost, a near-tie escalates through
// the SAME generic Ziv walkers the wide tiers run (via
// `support::narrow_ziv`), recomputing in the wider `WZiv = Int<24>` work
// integer with the width-generic kernels (`trig_generic`,
// `exp_generic`). This escalation handles the constructible cases a
// directed-only escalation hard-capped at the 75-digit
// `Fixed` constant window would miss: a still-exact residual at the
// cap treated as exact (mis-rounding e.g. `sin(1e-38)` Floor at D38<38>, whose
// `x³/6` deviation sits at fraction depth 115), and the nearest
// modes left entirely single-shot (mis-rounding the even-function exact-half
// family, e.g. `cosh(1e-19)` HalfToEven at D38<38>). `WZiv` probes to
// ~192 digits, past every constructible narrow-tier deciding depth
// (≤ 3·38 = 114); see `narrow_ziv`.

/// One `WZiv` sin probe at working scale `SCALE + guard_digits`.
fn sin_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    tg::sin_fixed::<WZiv>(
        narrow_ziv::lift(raw, guard_digits),
        working_scale,
        narrow_ziv::pi_w(working_scale),
    )
}

/// One `WZiv` cos probe at working scale `SCALE + guard_digits`.
fn cos_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    tg::cos_fixed::<WZiv>(
        narrow_ziv::lift(raw, guard_digits),
        working_scale,
        narrow_ziv::pi_w(working_scale),
    )
}

/// One `WZiv` tan probe (the sin/cos ratio) at working scale
/// `SCALE + guard_digits`.
fn tan_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let (sin_value, cos_value) = tg::sin_cos_fixed::<WZiv>(
        narrow_ziv::lift(raw, guard_digits),
        working_scale,
        narrow_ziv::pi_w(working_scale),
    );
    assert!(
        cos_value != WZiv::from_i128(0),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    eg::div::<WZiv>(sin_value, cos_value, working_scale)
}

/// One `WZiv` atan probe at working scale `SCALE + guard_digits`.
fn atan_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    tg::atan_fixed::<WZiv>(
        narrow_ziv::lift(raw, guard_digits),
        working_scale,
        narrow_ziv::pi_w(working_scale),
    )
}

/// The asin composition on a `WZiv` working value — the generic mirror
/// of the `Fixed` body (`atan(x/√(1−x²))` below ½, the half-angle
/// reduction above).
pub(crate) fn asin_work_ziv(working_value: WZiv, working_scale: u32) -> WZiv {
    let zero = WZiv::from_i128(0);
    let one_w = eg::one::<WZiv>(working_scale);
    let pi = narrow_ziv::pi_w(working_scale);
    let half_pi = pi >> 1;
    let is_negative = working_value < zero;
    let abs_working_value = if is_negative { -working_value } else { working_value };
    if abs_working_value == one_w {
        return if is_negative { -half_pi } else { half_pi };
    }
    let half_w = one_w >> 1;
    let asin_abs = if abs_working_value < half_w {
        let denom = eg::sqrt_fixed::<WZiv>(
            one_w - eg::mul::<WZiv>(abs_working_value, abs_working_value, working_scale),
            working_scale,
        );
        tg::atan_fixed::<WZiv>(
            eg::div::<WZiv>(abs_working_value, denom, working_scale),
            working_scale,
            pi,
        )
    } else {
        let inner = (one_w - abs_working_value) >> 1;
        let inner_sqrt = eg::sqrt_fixed::<WZiv>(inner, working_scale);
        let inner_denom = eg::sqrt_fixed::<WZiv>(
            one_w - eg::mul::<WZiv>(inner_sqrt, inner_sqrt, working_scale),
            working_scale,
        );
        let inner_asin = tg::atan_fixed::<WZiv>(
            eg::div::<WZiv>(inner_sqrt, inner_denom, working_scale),
            working_scale,
            pi,
        );
        half_pi - inner_asin - inner_asin
    };
    if is_negative { -asin_abs } else { asin_abs }
}

/// One `WZiv` asin probe at working scale `SCALE + guard_digits`.
pub(crate) fn asin_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    asin_work_ziv(narrow_ziv::lift(raw, guard_digits), working_scale)
}

/// One `WZiv` acos probe (`π/2 − asin`) at working scale
/// `SCALE + guard_digits`.
pub(crate) fn acos_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    (narrow_ziv::pi_w(working_scale) >> 1)
        - asin_work_ziv(narrow_ziv::lift(raw, guard_digits), working_scale)
}

/// One `WZiv` atan2 probe (quadrant-resolved max-branch ratio) at
/// working scale `SCALE + guard_digits` — the generic mirror of
/// [`atan2_kernel`].
pub(crate) fn atan2_ziv(y_raw: i128, x_raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let zero = WZiv::from_i128(0);
    let pi = narrow_ziv::pi_w(working_scale);
    let half_pi = pi >> 1;
    if x_raw == 0 {
        return if y_raw > 0 {
            half_pi
        } else if y_raw < 0 {
            -half_pi
        } else {
            zero
        };
    }
    let y = narrow_ziv::lift(y_raw, guard_digits);
    let x = narrow_ziv::lift(x_raw, guard_digits);
    let abs_y = if y < zero { -y } else { y };
    let abs_x = if x < zero { -x } else { x };
    let base = if abs_x >= abs_y {
        tg::atan_fixed::<WZiv>(eg::div::<WZiv>(y, x, working_scale), working_scale, pi)
    } else {
        let atan_x_over_y =
            tg::atan_fixed::<WZiv>(eg::div::<WZiv>(x, y, working_scale), working_scale, pi);
        let same_sign = (y < zero) == (x < zero);
        if same_sign {
            half_pi - atan_x_over_y
        } else {
            (-half_pi) - atan_x_over_y
        }
    };
    if x_raw > 0 {
        base
    } else if y_raw >= 0 {
        base + pi
    } else {
        base - pi
    }
}

/// Directed-mode post-adjust for the ±1-bounded forward functions — the
/// raw-`i128` mirror of the wide `adjust_bounded_extremum`. `|sin| < 1`
/// and `|cos| < 1` STRICTLY for every representable argument (`π/2` is
/// irrational, so no grid argument hits the extremum), so a downward
/// mode can never correctly produce exactly ±1: `result == ±1` under
/// Floor / Trunc (toward zero on the matching sign) is the
/// sub-resolution overshoot — step one LSB inside. A no-op everywhere
/// else.
fn adjust_bounded_extremum_raw(result: i128, scale: u32, mode: RoundingMode) -> i128 {
    if is_nearest_mode(mode) {
        return result;
    }
    let one = 10_i128.pow(scale);
    // `ZeroFiveUp` pivots on the last digit of the toward-zero result,
    // whose magnitude is `10^scale − 1`: a `9` for every `scale >= 1`, so
    // it never bumps and tracks `Trunc`. At `scale == 0` that digit is
    // `0`, a pivot, so it bumps straight back onto `±1` — the `_` arm.
    // `AwayFromZero` keeps the full `±1` magnitude, also the `_` arm.
    let zero_five_up_truncates = scale >= 1;
    if result == one {
        match mode {
            RoundingMode::Floor | RoundingMode::Trunc => one - 1,
            RoundingMode::ZeroFiveUp if zero_five_up_truncates => one - 1,
            _ => result,
        }
    } else if result == -one {
        match mode {
            RoundingMode::Ceiling | RoundingMode::Trunc => -one + 1,
            RoundingMode::ZeroFiveUp if zero_five_up_truncates => -one + 1,
            _ => result,
        }
    } else {
        result
    }
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
    let working_scale = SCALE + STRICT_GUARD;
    let rounded = match sin_fixed(to_fixed(raw), working_scale)
        .round_to_i128_clear_of_tie(working_scale, SCALE, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("sin", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            sin_ziv(raw, SCALE, guard_digits)
        }),
    };
    adjust_bounded_extremum_raw(rounded, SCALE, mode)
}

// ── cos ────────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn cos_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let working_scale = SCALE + STRICT_GUARD;
    let cos_value = sin_fixed(
        to_fixed(raw).add(wide_half_pi(working_scale)),
        working_scale,
    );
    let rounded = match cos_value.round_to_i128_clear_of_tie(working_scale, SCALE, mode) {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("cos", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            cos_ziv(raw, SCALE, guard_digits)
        }),
    };
    adjust_bounded_extremum_raw(rounded, SCALE, mode)
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
    let working_scale = SCALE + STRICT_GUARD;
    let (sin_w, cos_w) = sin_cos_fixed(to_fixed(raw), working_scale);
    assert!(
        !cos_w.is_zero(),
        "tan: cosine is zero (argument is an odd multiple of pi/2)"
    );
    match sin_w
        .div(cos_w, working_scale)
        .round_to_i128_clear_of_tie(working_scale, SCALE, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("tan", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            tan_ziv(raw, SCALE, guard_digits)
        }),
    }
}

// ── atan ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    // atan(±1) = ±π/4: the baked constant is rounded HALF-TO-EVEN, so the
    // pin is correct only for the nearest modes (an exact half-tie is
    // impossible for an irrational, so half-away agrees). Directed modes
    // fall through to the guarded computation + mode-aware rounding.
    if is_nearest_mode(mode) {
        if raw == one_bits {
            return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
        }
        if raw == -one_bits {
            return -<crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
        }
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    let working_scale = SCALE + STRICT_GUARD;
    match atan_fixed(to_fixed(raw), working_scale)
        .round_to_i128_clear_of_tie(working_scale, SCALE, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("atan", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            atan_ziv(raw, SCALE, guard_digits)
        }),
    }
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
    let working_scale = SCALE + STRICT_GUARD;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let working_value = to_fixed(raw);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    assert!(
        !(abs_working_value.ge_mag(one_w) && abs_working_value != one_w),
        "asin: argument out of domain [-1, 1]"
    );
    let asin_w = if abs_working_value == one_w {
        let half_pi = wide_half_pi(working_scale);
        if working_value.negative { half_pi.neg() } else { half_pi }
    } else {
        let half_w = one_w.halve();
        if !abs_working_value.ge_mag(half_w) {
            let denom = one_w
                .sub(working_value.mul(working_value, working_scale))
                .sqrt(working_scale);
            atan_fixed(working_value.div(denom, working_scale), working_scale)
        } else {
            let inner = one_w.sub(abs_working_value).halve();
            let inner_sqrt = inner.sqrt(working_scale);
            let inner_denom = one_w
                .sub(inner_sqrt.mul(inner_sqrt, working_scale))
                .sqrt(working_scale);
            let inner_asin =
                atan_fixed(inner_sqrt.div(inner_denom, working_scale), working_scale);
            let result_abs = wide_half_pi(working_scale)
                .sub(inner_asin)
                .sub(inner_asin);
            if working_value.negative {
                result_abs.neg()
            } else {
                result_abs
            }
        }
    };
    match asin_w.round_to_i128_clear_of_tie(working_scale, SCALE, mode) {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            asin_ziv(raw, SCALE, guard_digits)
        }),
    }
}

// ── acos ───────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn acos_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    // acos(0) = π/2 and acos(−1) = π are IRRATIONAL: the baked constants
    // are half-even-rounded, so those pins hold for nearest modes only;
    // directed modes fall through to the mode-aware computation.
    // acos(1) = 0 is EXACT — mode-independent, pinned for every mode.
    if raw == 0 && is_nearest_mode(mode) {
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits && is_nearest_mode(mode) {
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::pi().0.as_i128();
    }
    let working_scale = SCALE + STRICT_GUARD;
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let working_value = to_fixed(raw);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    assert!(
        !(abs_working_value.ge_mag(one_w) && abs_working_value != one_w),
        "acos: argument out of domain [-1, 1]"
    );
    let half_w = one_w.halve();
    let asin_w = if abs_working_value == one_w {
        let half_pi = wide_half_pi(working_scale);
        if working_value.negative { half_pi.neg() } else { half_pi }
    } else if !abs_working_value.ge_mag(half_w) {
        let denom = one_w
            .sub(working_value.mul(working_value, working_scale))
            .sqrt(working_scale);
        atan_fixed(working_value.div(denom, working_scale), working_scale)
    } else {
        let inner = one_w.sub(abs_working_value).halve();
        let inner_sqrt = inner.sqrt(working_scale);
        let inner_denom = one_w
            .sub(inner_sqrt.mul(inner_sqrt, working_scale))
            .sqrt(working_scale);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, working_scale), working_scale);
        let result_abs = wide_half_pi(working_scale)
            .sub(inner_asin)
            .sub(inner_asin);
        if working_value.negative {
            result_abs.neg()
        } else {
            result_abs
        }
    };
    match wide_half_pi(working_scale)
        .sub(asin_w)
        .round_to_i128_clear_of_tie(working_scale, SCALE, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("acos", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            acos_ziv(raw, SCALE, guard_digits)
        }),
    }
}

// ── atan2 ──────────────────────────────────────────────────────────

#[inline]
#[must_use]
pub(crate) fn atan2_strict_raw<const SCALE: u32>(y_raw: i128, x_raw: i128, mode: RoundingMode) -> i128 {
    let working_scale = SCALE + STRICT_GUARD;
    match atan2_kernel(to_fixed(y_raw), to_fixed(x_raw), y_raw, working_scale)
        .round_to_i128_clear_of_tie(working_scale, SCALE, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("atan2", SCALE)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, SCALE, mode, |guard_digits| {
            atan2_ziv(y_raw, x_raw, SCALE, guard_digits)
        }),
    }
}

// ── Hyperbolic family ─────────────────────────────────────────────
//
// sinh / cosh / tanh / asinh / acosh / atanh kernels on the `Fixed`
// 256-bit intermediate. The typed-shell file in `src/types/trig.rs`
// delegates one line to each `policy::trig` dispatch, whose body
// resolves to the matching kernel below.

#[inline]
#[must_use]
pub(crate) fn sinh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sinh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One signed `Fixed` sinh evaluation at
/// `working_scale = scale + working_digits` —
/// the `(e^|x| − e^-|x|)/2` identity body shared by the strict terminal
/// and its Ziv rungs. Evaluates at `|v|` so the dominant `e^|x|` term is
/// computed directly and accurately; the reciprocal gives the tiny
/// `e^-|x|` (computing `exp(-|x|)` directly and reciprocating would
/// amplify the small term's relative error into a large absolute
/// error). sinh is odd: the input sign is reapplied to the non-negative
/// `sinh(|x|)`.
fn sinh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> Fixed {
    let working_value = to_fixed_w(raw, working_digits);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    let exp_x = exp_fixed(abs_working_value, working_scale);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let exp_neg_x = one_w.div(exp_x, working_scale);
    let sinh_value = exp_x.sub(exp_neg_x).halve();
    if raw < 0 { sinh_value.neg() } else { sinh_value }
}

/// One `WZiv` sinh probe at working scale `scale + guard_digits`.
fn sinh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let abs_working_value = if working_value < WZiv::from_i128(0) {
        -working_value
    } else {
        working_value
    };
    let sinh_value = eg::sinh_pos::<WZiv>(abs_working_value, working_scale);
    if raw < 0 { -sinh_value } else { sinh_value }
}

/// Strict-path `i128` core of [`sinh_strict`]: the `Fixed` fast shot
/// narrows through the clear-of-tie terminal; a near-tie (and the
/// integer-regime cells, whose base probe is a wider-work single
/// shot) escalates through the Ziv walker.
fn sinh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) {
        // sinh(x) = x + x³/6 + … : within the linear band the cubic is
        // below one ULP yet strictly positive, so the true value sits
        // just *above* the grid line `raw` (in magnitude) — the analytic
        // decision, exact at every depth.
        return crate::support::rounding::tiny_odd_expanding_directed(
            raw,
            0,
            1,
            (raw.unsigned_abs() % 10) as u8,
            mode,
        );
    }
    let working_scale = scale + STRICT_GUARD;
    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(raw, scale, working_scale) {
        return narrow_ziv::walk(STRICT_GUARD, scale, mode, |guard_digits| {
            sinh_ziv(raw, scale, guard_digits)
        });
    }
    match sinh_eval_fixed(raw, STRICT_GUARD, working_scale)
        .round_to_i128_clear_of_tie(working_scale, scale, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::sinh", scale)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, scale, mode, |guard_digits| {
            sinh_ziv(raw, scale, guard_digits)
        }),
    }
}

#[inline]
#[must_use]
pub(crate) fn cosh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cosh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One non-negative `Fixed` cosh evaluation at
/// `working_scale = scale + working_digits` — the `(e^|x| + e^-|x|)/2`
/// identity body shared by
/// the strict terminal and its Ziv rungs. cosh is even; evaluating at `|v|`
/// keeps the dominant `e^|x|` term direct (see [`sinh_eval_fixed`]).
fn cosh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> Fixed {
    let working_value = to_fixed_w(raw, working_digits);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    let exp_x = exp_fixed(abs_working_value, working_scale);
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let exp_neg_x = one_w.div(exp_x, working_scale);
    exp_x.add(exp_neg_x).halve()
}

/// One `WZiv` cosh probe at working scale `scale + guard_digits`.
fn cosh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let abs_working_value = if working_value < WZiv::from_i128(0) {
        -working_value
    } else {
        working_value
    };
    eg::cosh_pos::<WZiv>(abs_working_value, working_scale)
}

/// Strict-path `i128` core of [`cosh_strict`]. `cosh(x) > 1` is
/// transcendental for every `x ≠ 0` (the `never_exact` walker polarity:
/// the exact-half/grid families — `cosh(1e-19)` at D38<38> lands
/// `1 + x²/2` EXACTLY on the half with the `x⁴/24` tail at depth 77 —
/// always carry a strictly positive tail). The `Fixed` fast shot now
/// serves every mode; near-ties (including every directed cell the old
/// path detoured through the wide single shot for) escalate through the
/// walker, which resolves the deciding digit instead of assuming it.
fn cosh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale);
    }
    let working_scale = scale + STRICT_GUARD;
    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(raw, scale, working_scale) {
        return narrow_ziv::walk_never_exact(STRICT_GUARD, scale, mode, |guard_digits| {
            cosh_ziv(raw, scale, guard_digits)
        });
    }
    match cosh_eval_fixed(raw, STRICT_GUARD, working_scale)
        .round_to_i128_clear_of_tie(working_scale, scale, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::cosh", scale)
        }),
        None => narrow_ziv::walk_never_exact(STRICT_GUARD, scale, mode, |guard_digits| {
            cosh_ziv(raw, scale, guard_digits)
        }),
    }
}

#[inline]
#[must_use]
pub(crate) fn tanh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(tanh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One `WZiv` tanh probe at working scale `scale + guard_digits`.
fn tanh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let abs_working_value = if working_value < WZiv::from_i128(0) {
        -working_value
    } else {
        working_value
    };
    let tanh_value = eg::tanh_pos::<WZiv>(abs_working_value, working_scale);
    if raw < 0 { -tanh_value } else { tanh_value }
}

/// Strict-path `i128` core of [`tanh_strict`]: the linear band and the
/// all-nines saturation region stay ANALYTIC (the band's sub-ULP cubic
/// and the saturation's `1 − 10^-w` shape are proven, so no tie check
/// is needed there); the middle region narrows through the clear-of-tie
/// terminal with the Ziv walker behind it.
fn tanh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) {
        // tanh(x) = x − x³/3 + … : within the linear band the cubic is
        // below one ULP yet strictly positive, so the true value sits just
        // inside the grid line `raw`. Nearest modes return `raw`; the
        // directed modes need this analytic decision — no finite-precision
        // exp path can resolve the sub-ULP cubic.
        return crate::support::rounding::tiny_odd_compressing_directed(
            raw,
            0,
            1,
            (raw.unsigned_abs() % 10) as u8,
            mode,
        );
    }
    let working_scale = scale + STRICT_GUARD;
    match tanh_eval_fixed(raw, STRICT_GUARD, working_scale) {
        // Saturated all-nines: tanh(|x|) ∈ (1 − 10^-working_scale, 1)
        // analytically — every mode rounds the all-nines value correctly
        // (nearest → 1, Floor/Trunc → 1 − 10^-S, Ceiling → 1); no tie to
        // resolve.
        (tanh_value, true) => tanh_value
            .round_to_i128_with(working_scale, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("D38::tanh", scale)
            }),
        (tanh_value, false) => {
            match tanh_value.round_to_i128_clear_of_tie(working_scale, scale, mode) {
                Some(narrowed) => narrowed.unwrap_or_else(|| {
                    crate::support::diagnostics::overflow_panic_with_scale("D38::tanh", scale)
                }),
                None => narrow_ziv::walk(STRICT_GUARD, scale, mode, |guard_digits| {
                    tanh_ziv(raw, scale, guard_digits)
                }),
            }
        }
    }
}

/// One signed `Fixed` tanh evaluation at
/// `working_scale = scale + working_digits`,
/// returning `(value, saturated)` — `saturated == true` is the analytic
/// all-nines region (`|x|` past the `2·e^(−2|x|) < 10^-working_scale`
/// onset, or the
/// `m` underflow just inside it), where the value is exactly the
/// largest working value below 1 and needs no tie analysis.
fn tanh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> (Fixed, bool) {
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let is_negative = raw < 0;
    // Large |x| via the NEGATIVE-exponent identity tanh(|x|) = (1 − m)/(1 + m),
    // m = e^(−2|x|) — exact and overflow-safe across the whole large-|x|
    // range: forming e^(+|x|) directly overflows the 256-bit `Fixed` once
    // |x| ≳ 256·ln2 − working_scale·ln10, BELOW the all-nines saturation
    // onset |x| ≳ 1.1513·working_scale. Mirrors `exp_generic::tanh_pos`
    // (the wide path).
    let scale = working_scale - working_digits;
    let saturation_bound = (working_scale as i128) * 1152 / 1000 + 2;
    let saturated = one_w.sub(Fixed::from_u128_mag(1, false));
    let (tanh_value, is_saturated) = if raw.abs() / 10_i128.pow(scale) > saturation_bound {
        (saturated, true)
    } else {
        let working_value = to_fixed_w(raw, working_digits);
        let abs_working_value = Fixed {
            negative: false,
            mag: working_value.mag,
        };
        let m = exp_fixed(abs_working_value.double().neg(), working_scale);
        if m.is_zero() {
            (saturated, true)
        } else {
            (one_w.sub(m).div(one_w.add(m), working_scale), false)
        }
    };
    (
        if is_negative {
            tanh_value.neg()
        } else {
            tanh_value
        },
        is_saturated,
    )
}

#[inline]
#[must_use]
pub(crate) fn asinh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(asinh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One signed `Fixed` asinh evaluation at
/// `working_scale = scale + working_digits`
/// — `ln(|x| + √(x²+1))` (the reciprocal form above 1 keeps the `x²`
/// product inside the 256-bit `Fixed`), shared by the strict terminal
/// and its Ziv rungs. asinh is odd; the sign is reapplied.
fn asinh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> Fixed {
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let working_value = to_fixed_w(raw, working_digits);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    let inner = if abs_working_value.ge_mag(one_w) {
        let reciprocal = one_w.div(abs_working_value, working_scale);
        let root = one_w
            .add(reciprocal.mul(reciprocal, working_scale))
            .sqrt(working_scale);
        ln_fixed(abs_working_value, working_scale)
            .add(ln_fixed(one_w.add(root), working_scale))
    } else {
        let root = abs_working_value
            .mul(abs_working_value, working_scale)
            .add(one_w)
            .sqrt(working_scale);
        ln_fixed(abs_working_value.add(root), working_scale)
    };
    if raw < 0 { inner.neg() } else { inner }
}

/// One `WZiv` asinh probe at working scale `scale + guard_digits`.
fn asinh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let zero = WZiv::from_i128(0);
    let one_w = eg::one::<WZiv>(working_scale);
    let ln2 = narrow_ziv::ln2_w(working_scale);
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let abs_working_value = if working_value < zero {
        -working_value
    } else {
        working_value
    };
    let inner = if abs_working_value >= one_w {
        let reciprocal = eg::div::<WZiv>(one_w, abs_working_value, working_scale);
        let root = eg::sqrt_fixed::<WZiv>(
            one_w + eg::mul::<WZiv>(reciprocal, reciprocal, working_scale),
            working_scale,
        );
        eg::ln_fixed::<WZiv>(abs_working_value, working_scale, ln2)
            + eg::ln_fixed::<WZiv>(one_w + root, working_scale, ln2)
    } else {
        let root = eg::sqrt_fixed::<WZiv>(
            eg::mul::<WZiv>(abs_working_value, abs_working_value, working_scale) + one_w,
            working_scale,
        );
        eg::ln_fixed::<WZiv>(abs_working_value + root, working_scale, ln2)
    };
    if raw < 0 { -inner } else { inner }
}

/// Strict-path `i128` core of [`asinh_strict`] — clear-of-tie terminal
/// with the Ziv walker behind it.
fn asinh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) && is_nearest_mode(mode) {
        return raw;
    }
    let working_scale = scale + STRICT_GUARD;
    match asinh_eval_fixed(raw, STRICT_GUARD, working_scale)
        .round_to_i128_clear_of_tie(working_scale, scale, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::asinh", scale)
        }),
        None => narrow_ziv::walk(STRICT_GUARD, scale, mode, |guard_digits| {
            asinh_ziv(raw, scale, guard_digits)
        }),
    }
}

#[inline]
#[must_use]
pub(crate) fn acosh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(acosh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One `Fixed` acosh evaluation at
/// `working_scale = scale + working_digits` —
/// `ln(x + √(x²−1))` (reciprocal form above 2), shared by the strict
/// terminal and its Ziv rungs. Asserts the `x ≥ 1` domain.
fn acosh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> Fixed {
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let working_value = to_fixed_w(raw, working_digits);
    assert!(
        !working_value.negative && working_value.ge_mag(one_w),
        "D38::acosh: argument must be >= 1"
    );
    let two_w = one_w.double();
    if working_value.ge_mag(two_w) {
        let reciprocal = one_w.div(working_value, working_scale);
        let root = one_w
            .sub(reciprocal.mul(reciprocal, working_scale))
            .sqrt(working_scale);
        ln_fixed(working_value, working_scale).add(ln_fixed(one_w.add(root), working_scale))
    } else {
        let root = working_value
            .mul(working_value, working_scale)
            .sub(one_w)
            .sqrt(working_scale);
        ln_fixed(working_value.add(root), working_scale)
    }
}

/// One `WZiv` acosh probe at working scale `scale + guard_digits`.
fn acosh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let one_w = eg::one::<WZiv>(working_scale);
    let ln2 = narrow_ziv::ln2_w(working_scale);
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let two_w = one_w + one_w;
    if working_value >= two_w {
        let reciprocal = eg::div::<WZiv>(one_w, working_value, working_scale);
        let root = eg::sqrt_fixed::<WZiv>(
            one_w - eg::mul::<WZiv>(reciprocal, reciprocal, working_scale),
            working_scale,
        );
        eg::ln_fixed::<WZiv>(working_value, working_scale, ln2)
            + eg::ln_fixed::<WZiv>(one_w + root, working_scale, ln2)
    } else {
        let root = eg::sqrt_fixed::<WZiv>(
            eg::mul::<WZiv>(working_value, working_value, working_scale) - one_w,
            working_scale,
        );
        eg::ln_fixed::<WZiv>(working_value + root, working_scale, ln2)
    }
}

/// Strict-path `i128` core of [`acosh_strict`] — clear-of-tie terminal
/// with the near-special walker (the wide acosh shape: a forced confirm
/// near the `x = 1` special point) behind it.
fn acosh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    let one_bits: i128 = 10_i128.pow(scale);
    if raw == one_bits {
        return 0;
    }
    let working_scale = scale + STRICT_GUARD;
    match acosh_eval_fixed(raw, STRICT_GUARD, working_scale)
        .round_to_i128_clear_of_tie(working_scale, scale, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::acosh", scale)
        }),
        None => narrow_ziv::walk_near_special(STRICT_GUARD, scale, mode, |guard_digits| {
            acosh_ziv(raw, scale, guard_digits)
        }),
    }
}

#[inline]
#[must_use]
pub(crate) fn atanh_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(atanh_strict_raw(raw.as_i128(), SCALE, mode))
}

/// One `WZiv` atanh probe at working scale `scale + guard_digits` — the
/// overflow-safe two-log GAP form `½·(ln(1+|x|) − ln(1−|x|))` (the
/// near-±1 ratio overflow never arises), sign reapplied.
fn atanh_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    let working_scale = scale + guard_digits;
    let zero = WZiv::from_i128(0);
    let one_w = eg::one::<WZiv>(working_scale);
    let ln2 = narrow_ziv::ln2_w(working_scale);
    let working_value = narrow_ziv::lift(raw, guard_digits);
    let abs_working_value = if working_value < zero {
        -working_value
    } else {
        working_value
    };
    let inner = (eg::ln_fixed::<WZiv>(one_w + abs_working_value, working_scale, ln2)
        - eg::ln_fixed::<WZiv>(one_w - abs_working_value, working_scale, ln2))
        >> 1;
    if raw < 0 { -inner } else { inner }
}

/// Strict-path `i128` core of [`atanh_strict`] — clear-of-tie terminal
/// with the near-special walker (the wide atanh shape: a forced confirm
/// near the ±1 special points) behind it. The exposing family is the
/// tiny-x partial (`atanh(1e-38)` at D38<38>: `x³/3` deviation at depth
/// 115, strictly positive → Ceiling must step up).
fn atanh_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold_scale(scale) && is_nearest_mode(mode) {
        return raw;
    }
    let working_scale = scale + STRICT_GUARD;
    match atanh_eval_fixed(raw, STRICT_GUARD, working_scale)
        .round_to_i128_clear_of_tie(working_scale, scale, mode)
    {
        Some(narrowed) => narrowed.unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::atanh", scale)
        }),
        None => narrow_ziv::walk_near_special(STRICT_GUARD, scale, mode, |guard_digits| {
            atanh_ziv(raw, scale, guard_digits)
        }),
    }
}

/// One signed `Fixed` atanh evaluation at
/// `working_scale = scale + working_digits` — the value-gated ratio/gap
/// split (the gate is derived at the branch below).
fn atanh_eval_fixed(raw: i128, working_digits: u32, working_scale: u32) -> Fixed {
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    let working_value = to_fixed_w(raw, working_digits);
    let abs_working_value = Fixed {
        negative: false,
        mag: working_value.mag,
    };
    assert!(
        !abs_working_value.ge_mag(one_w),
        "D38::atanh: argument out of domain (-1, 1)"
    );
    // atanh(x) = ½·ln((1+x)/(1-x)), value-gated on |x|:
    //
    //  - |x| ≤ 0.98 (1−|x| ≥ 0.02): the 1-log RATIO form ½·ln((1+x)/(1-x)) —
    //    ONE `ln_fixed` (~2× cheaper than the two-log gap form). The ratio
    //    R = (1+|x|)/(1-|x|) ≤ 99 there, so `R·10^ws` (ws = the working scale)
    //    fits the 256-bit `Fixed`
    //    at any working scale up to the 75-digit ceiling (R < 2²⁵⁶/10⁷⁵ ≈ 115;
    //    the strict path's working_scale = SCALE+30 ≤ 68 has ~10⁷× more
    //    headroom). The division's ≤0.5-ULP-at-working-scale error propagates
    //    to atanh as ≤0.25·10⁻ʷ,
    //    absorbed by the ≥30 guard digits, so every mode rounds correctly.
    //
    //  - |x| > 0.98 (near ±1): the GAP form ½·(ln(1+x) − ln(1-x)) — TWO logs,
    //    taken separately so the overflowing ratio is never formed (the
    //    `dab34171` correctness fix: near |x|=1 the ratio reaches ~10^(2·digits),
    //    e.g. atanh(1−10⁻²⁸) at D38 s28 → ~10⁸⁶, past 2²⁵⁶). `1+x` and `1-x`
    //    each lie in (0, 2) and fit; `ln_fixed` handles arguments below 1.
    //
    // Gate in `Fixed` magnitudes: `50·(1−|x|)·10^ws ≥ 10^ws` ⟺ `1−|x| ≥ 0.02`
    // (the `50·(1−|x|)·10^ws ≤ 5·10⁷⁶` intermediate fits 2²⁵⁶, where `ws` is
    // the working scale).
    if one_w.sub(abs_working_value).mul_u128(50).ge_mag(one_w) {
        let ratio = one_w
            .add(working_value)
            .div(one_w.sub(working_value), working_scale);
        ln_fixed(ratio, working_scale).halve()
    } else {
        let ln_num = ln_fixed(one_w.add(working_value), working_scale);
        let ln_den = ln_fixed(one_w.sub(working_value), working_scale);
        ln_num.sub(ln_den).halve()
    }
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
    let working_scale = scale + working_digits;
    to_fixed_w(raw, working_digits)
        .mul_u128(180)
        .div(wide_pi(working_scale), working_scale)
        .round_to_i128_with(working_scale, scale, mode)
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
    let working_scale = scale + working_digits;
    to_fixed_w(raw, working_digits)
        .mul(wide_pi(working_scale), working_scale)
        .div_small(180)
        .round_to_i128_with(working_scale, scale, mode)
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
    let threshold_exponent = scale.saturating_sub(scale.div_ceil(3));
    10_i128.pow(threshold_exponent)
}

// ── Near-tie pins: the narrow single-shot / capped-escalation defect ──
// The same constructible class seen on the wide
// tiers (asin(3e-60) D462<180>): an EXACT input whose Taylor partial lands
// exactly ON a rounding boundary (the grid line for directed modes, the
// half for nearest) with the deciding transcendental tail BELOW the fixed
// working scale SCALE + 30 (and, for sin/cos, below the 75-digit
// escalation cap). Oracle for every expected value: the exact rational
// Taylor partial + the strict tail sign (mpmath-confirmed,
// trace/narrow_tie_derive.py derivations):
//
//  directed tiny-x at D38<38>, raw = 1 (x = 1e-38):
//    sin(x)  = x − x³/6 + …      deviation −1.67e-115 → Floor/Trunc 0
//    cos(x)  = 1 − x²/2 + …      deviation −5e-77     → Floor/Trunc 10³⁸−1
//    tan(x)  = x + x³/3 + …      deviation +3.3e-77   → Ceiling 2
//    atan(x) = x − x³/3 + …      deviation −3.3e-77   → Floor/Trunc 0
//    asin(x) = x + x³/6 + …      deviation +1.67e-77  → Ceiling 2
//    asinh(x)= x − x³/6 + …      deviation −1.67e-77  → Floor/Trunc 0
//    atanh(x)= x + x³/3 + …      deviation +3.3e-77   → Ceiling 2
//
//  nearest exact-half (even functions; x = c·10^(−S/2), S even ≥ 34, the
//  x²/2 term is EXACTLY (c²/2) ULPs and the +x⁴/24 tail hides below w):
//    cosh(1e-19) D38<38> = 1 + 0.5 ULP + 4.2e-78 → every nearest mode
//      rounds UP to 10³⁸ + 1 (the computed exact half tied-to-even DOWN).
//    cos(1e-19)  D38<38> = 1 − 0.5 ULP + 4.2e-78 → every nearest mode
//      rounds UP to 10³⁸ (HalfTowardZero kept the floor pre-fix).
#[cfg(test)]
mod near_tie_pins {
    use super::*;

    const S38: u32 = 38;
    const ONE38: i128 = 10_i128.pow(38);

    #[test]
    fn sin_directed_tiny_x_d38_s38() {
        // a w = 75 escalation cap is below deviation
        // depth 115 and would treat the grid-exact residual as exact.
        assert_eq!(sin_strict_raw::<S38>(1, RoundingMode::Floor), 0, "sin Floor");
        assert_eq!(sin_strict_raw::<S38>(1, RoundingMode::Trunc), 0, "sin Trunc");
        assert_eq!(sin_strict_raw::<S38>(1, RoundingMode::Ceiling), 1, "sin Ceiling");
        // odd-function mirror: sin(−1e-38) = −(x − x³/6) ∈ (−1 ULP, 0).
        assert_eq!(sin_strict_raw::<S38>(-1, RoundingMode::Ceiling), 0, "sin(−x) Ceiling");
        assert_eq!(sin_strict_raw::<S38>(-1, RoundingMode::Floor), -1, "sin(−x) Floor");
    }

    #[test]
    fn cos_directed_tiny_x_d38_s38() {
        // deviation 5e-39 ULPs (depth 76.3) — below even the 75-digit cap.
        assert_eq!(cos_strict_raw::<S38>(1, RoundingMode::Floor), ONE38 - 1, "cos Floor");
        assert_eq!(cos_strict_raw::<S38>(1, RoundingMode::Trunc), ONE38 - 1, "cos Trunc");
        assert_eq!(cos_strict_raw::<S38>(1, RoundingMode::Ceiling), ONE38, "cos Ceiling");
    }

    #[test]
    fn tan_directed_tiny_x_d38_s38() {
        assert_eq!(tan_strict_raw::<S38>(1, RoundingMode::Ceiling), 2, "tan Ceiling");
        assert_eq!(tan_strict_raw::<S38>(1, RoundingMode::Floor), 1, "tan Floor");
        assert_eq!(tan_strict_raw::<S38>(-1, RoundingMode::Floor), -2, "tan(−x) Floor");
    }

    #[test]
    fn atan_directed_tiny_x_d38_s38() {
        assert_eq!(atan_strict_raw::<S38>(1, RoundingMode::Floor), 0, "atan Floor");
        assert_eq!(atan_strict_raw::<S38>(1, RoundingMode::Trunc), 0, "atan Trunc");
        assert_eq!(atan_strict_raw::<S38>(1, RoundingMode::Ceiling), 1, "atan Ceiling");
    }

    #[test]
    fn asin_directed_tiny_x_d38_s38() {
        assert_eq!(asin_strict_raw::<S38>(1, RoundingMode::Ceiling), 2, "asin Ceiling");
        assert_eq!(asin_strict_raw::<S38>(1, RoundingMode::Floor), 1, "asin Floor");
    }

    #[test]
    fn asinh_directed_tiny_x_d38_s38() {
        assert_eq!(asinh_strict_raw(1, S38, RoundingMode::Floor), 0, "asinh Floor");
        assert_eq!(asinh_strict_raw(1, S38, RoundingMode::Ceiling), 1, "asinh Ceiling");
    }

    #[test]
    fn atanh_directed_tiny_x_d38_s38() {
        assert_eq!(atanh_strict_raw(1, S38, RoundingMode::Ceiling), 2, "atanh Ceiling");
        assert_eq!(atanh_strict_raw(1, S38, RoundingMode::Floor), 1, "atanh Floor");
    }

    #[test]
    fn atan2_directed_tiny_ratio_d38_s38() {
        // atan2(1e-38, 1) = atan(1e-38) — the same partial-sum family
        // through the ratio path.
        assert_eq!(
            atan2_strict_raw::<S38>(1, ONE38, RoundingMode::Floor),
            0,
            "atan2 Floor"
        );
        assert_eq!(
            atan2_strict_raw::<S38>(1, ONE38, RoundingMode::Ceiling),
            1,
            "atan2 Ceiling"
        );
    }

    #[test]
    fn tan_directed_tiny_x_d18_s18() {
        // D18 (SCALE ≤ 18, widened through the same Int<2> kernels):
        // tan(1e-18) deviation 3.3e-55 is below the single shot w = 48.
        assert_eq!(tan_strict_raw::<18>(1, RoundingMode::Ceiling), 2, "tan D18 Ceiling");
        // control: sin at D18 resolved already via the 75-digit escalation
        // (deviation depth 55 ≤ 75) — must keep resolving post-fix.
        assert_eq!(sin_strict_raw::<18>(1, RoundingMode::Floor), 0, "sin D18 Floor");
    }

    #[test]
    fn cosh_nearest_exact_half_d38_s38() {
        // cosh(1e-19) = 1 + 0.5 ULP + 4.2e-78: the partial 1 + x²/2 lands
        // EXACTLY on the half (x²/2 = 0.5·10⁻³⁸ exact) and the positive
        // x⁴/24 tail (depth 77.4) decides UP — every nearest mode → +1.
        let raw = 10_i128.pow(19);
        for (mode, what) in [
            (RoundingMode::HalfToEven, "HalfToEven"),
            (RoundingMode::HalfAwayFromZero, "HalfAwayFromZero"),
            (RoundingMode::HalfTowardZero, "HalfTowardZero"),
        ] {
            assert_eq!(
                cosh_strict_raw(raw, S38, mode),
                ONE38 + 1,
                "cosh(1e-19) {what}"
            );
        }
        // the c = 3 family member: cosh(3e-19) = 1 + 4.5 ULP + tail → +5.
        assert_eq!(
            cosh_strict_raw(3 * raw, S38, RoundingMode::HalfToEven),
            ONE38 + 5,
            "cosh(3e-19) HalfToEven"
        );
    }

    #[test]
    fn cos_nearest_exact_half_d38_s38() {
        // cos(1e-19) = 1 − 0.5 ULP + 4.2e-78: above the half → UP to 1.0
        // under every nearest mode (HalfTowardZero kept the floor pre-fix).
        let raw = 10_i128.pow(19);
        for (mode, what) in [
            (RoundingMode::HalfToEven, "HalfToEven"),
            (RoundingMode::HalfAwayFromZero, "HalfAwayFromZero"),
            (RoundingMode::HalfTowardZero, "HalfTowardZero"),
        ] {
            assert_eq!(
                cos_strict_raw::<S38>(raw, mode),
                ONE38,
                "cos(1e-19) {what}"
            );
        }
    }

    #[test]
    fn cos_nearest_exact_half_d38_s36() {
        // second scale of the even-S family: cos(1e-18) at SCALE 36.
        let one36 = 10_i128.pow(36);
        assert_eq!(
            cos_strict_raw::<36>(10_i128.pow(18), RoundingMode::HalfTowardZero),
            one36,
            "cos(1e-18) s36 HalfTowardZero"
        );
        assert_eq!(
            cosh_strict_raw(10_i128.pow(18), 36, RoundingMode::HalfToEven),
            one36 + 1,
            "cosh(1e-18) s36 HalfToEven"
        );
    }

    #[test]
    fn public_path_d38_and_d18_directed_tiny_x() {
        // The policy/public route must agree with the fixed kernels in
        // every build (default: trig_series_2limb; wide: borrow-D57 for
        // the inverse family, walker-fixed by the wide campaign).
        let x38 = crate::D::<Int<2>, 38>(Int::<2>::from_i128(1));
        assert_eq!(x38.sin_strict_with(RoundingMode::Floor).0.as_i128(), 0, "public sin Floor");
        assert_eq!(x38.tan_strict_with(RoundingMode::Ceiling).0.as_i128(), 2, "public tan Ceiling");
        assert_eq!(x38.asin_strict_with(RoundingMode::Ceiling).0.as_i128(), 2, "public asin Ceiling");
        assert_eq!(
            x38.cosh_strict_with(RoundingMode::HalfToEven).0.as_i128(),
            10_i128.pow(38),
            "public cosh(1e-38) HalfToEven (1 + x²/2 = 1 + 5e-77 → 1)"
        );
        let x18 = crate::D::<Int<1>, 18>(Int::<1>::from_i128(1));
        assert_eq!(x18.tan_strict_with(RoundingMode::Ceiling).0.as_i128(), 2, "public D18 tan Ceiling");
    }
}

// ── sinh / cosh fast-path validity wall ────────────────────────────
// Mirror of `exp_series_2limb::fast_path_validity`: assert the narrow
// sinh/cosh fast 256-bit `Fixed` path is bit-identical to the wider
// `WNarrow` reference for every cell the production gate
// (`hyper_needs_wide_narrow`) keeps fast. The tightened exp digit-gate is
// shared (sinh/cosh route on `!narrow_fixed_fits(|raw|, ...)`), so this
// guards that recovering the common sinh/cosh cells did not regress
// correctness at any unbenched scale.
#[cfg(test)]
mod hyper_fast_path_validity {
    use super::*;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// FAST sinh/cosh in `Fixed`, no gate — catching any overflow panic.
    fn fast_hyper_raw(raw: i128, scale: u32, mode: RoundingMode, is_cosh: bool) -> Option<i128> {
        let working_scale = scale + STRICT_GUARD;
        std::panic::catch_unwind(|| {
            let working_value = to_fixed_w(raw, STRICT_GUARD);
            let abs_working_value = Fixed {
                negative: false,
                mag: working_value.mag,
            };
            let exp_x = exp_fixed(abs_working_value, working_scale);
            let one_w = Fixed {
                negative: false,
                mag: Fixed::pow10(working_scale),
            };
            let exp_neg_x = one_w.div(exp_x, working_scale);
            let result = if is_cosh {
                exp_x.add(exp_neg_x).halve()
            } else {
                let sinh_value = exp_x.sub(exp_neg_x).halve();
                if raw < 0 { sinh_value.neg() } else { sinh_value }
            };
            result.round_to_i128_with(working_scale, scale, mode)
        })
        .unwrap_or(None)
    }

    fn run(is_cosh: bool) {
        std::panic::set_hook(Box::new(|_| {}));
        let mut checked = 0u64;
        for scale in 0u32..=38 {
            let one_s = 10f64.powi(scale as i32);
            let mut x10 = 1u64;
            while x10 <= 1000 {
                let x = x10 as f64 / 10.0;
                for sign in [1i128, -1] {
                    let raw_f = sign as f64 * x * one_s;
                    if raw_f.abs() >= (i128::MAX as f64) {
                        x10 += 1;
                        continue;
                    }
                    let raw = raw_f as i128;
                    if raw == 0 || raw.abs() <= small_x_linear_threshold_scale(scale) {
                        continue; // linear band handled separately
                    }
                    let working_scale = scale + STRICT_GUARD;
                    // Gate: stays fast unless the integer-regime digit gate fires.
                    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(
                        raw,
                        scale,
                        working_scale,
                    ) {
                        continue;
                    }
                    for mode in MODES {
                        let wide = match std::panic::catch_unwind(|| {
                            if is_cosh {
                                crate::algos::exp::exp_series_2limb::cosh_wide_narrow_raw(
                                    raw,
                                    scale,
                                    STRICT_GUARD,
                                    mode,
                                )
                            } else {
                                crate::algos::exp::exp_series_2limb::sinh_wide_narrow_raw(
                                    raw,
                                    scale,
                                    STRICT_GUARD,
                                    mode,
                                )
                            }
                        }) {
                            Ok(value) => value,
                            Err(_) => continue,
                        };
                        let fast = fast_hyper_raw(raw, scale, mode, is_cosh);
                        assert_eq!(
                            fast,
                            Some(wide),
                            "{} fast != wide at scale={scale} raw={raw} mode={mode:?}",
                            if is_cosh { "cosh" } else { "sinh" }
                        );
                        checked += 1;
                    }
                }
                x10 += 1;
            }
        }
        assert!(checked > 50_000, "too few cells checked: {checked}");
    }

    #[test]
    fn sinh_fast_bit_identical_to_wide_d38() {
        run(false);
    }

    #[test]
    fn cosh_fast_bit_identical_to_wide_d38() {
        run(true);
    }
}
