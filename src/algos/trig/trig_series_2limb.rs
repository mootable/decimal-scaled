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

use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::support::fixed::Fixed;
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
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
    let thresh_exp = SCALE.saturating_sub(SCALE.div_ceil(3));
    10_i128.pow(thresh_exp)
}

/// π at working scale `w`, sourced DIRECTLY from the per-scale const
/// table (`consts::pi_const_n`) — `floor(π·10^w)` rounded half-to-even.
/// The ungated NARROW band covers `0..=512`, so this reads in every build
/// (default / no_std included); no per-call rescale, no embedded raw.
pub(crate) fn wide_pi(w: u32) -> Fixed {
    debug_assert!(w <= 75, "wide_pi: working scale {w} exceeds Fixed capacity");
    let words = crate::consts::pi_const_n::<4>(w, crate::support::rounding::RoundingMode::HalfToEven)
        .limbs_le();
    Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
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

/// Largest working scale the narrow `Fixed` trig kernels escalate to: the
/// embedded π (`wide_pi`) is exact to 75 digits and `10^75` sits safely inside
/// the 256-bit `Fixed`, so directed-mode escalation stops here.
const MAX_FIXED_TRIG_W: u32 = 75;

/// Guard increment per directed-mode Ziv escalation step.
const TRIG_GUARD_STEP: u32 = 8;

/// Directed-mode rounding with Ziv escalation for the narrow `Fixed` trig
/// kernels — the narrow-tier analogue of the wide path's
/// `round_to_storage_directed`. Nearest modes round ONCE at `base_guard` (the
/// common, benchmarked path — byte-identical to a plain `round_to_i128_with`).
/// Directed modes (Trunc / Floor / Ceiling) need the residual SIGN: when the
/// value lands exactly on a storage grid line at the base working scale, the
/// true value may be a sub-resolution residual to one side. A near-extremum
/// `cos`/`sin` is the canonical case — `cos`/`sin` near `kπ` equal `±1 ∓ δ²/2`,
/// and for an argument within the input granularity of `kπ` the `δ²/2`
/// deviation falls *below* the base guard, so at the base working scale the
/// kernel rounds to exactly `±1` and a directed mode cannot see which side the
/// true value lies on (the golden `cos`/`sin` near-`kπ` cells). Re-evaluate
/// `eval(working_digits)` at increasing guard until the residual resolves,
/// capped at [`MAX_FIXED_TRIG_W`] (where the value is treated as exact).
#[inline]
fn round_fixed_trig_directed(
    scale: u32,
    base_guard: u32,
    mode: RoundingMode,
    fn_name: &'static str,
    mut eval: impl FnMut(u32) -> Fixed,
) -> i128 {
    let overflow = || crate::support::diagnostics::overflow_panic_with_scale(fn_name, scale);
    if is_nearest_mode(mode) {
        let w = scale + base_guard;
        return eval(base_guard)
            .round_to_i128_with(w, scale, mode)
            .unwrap_or_else(|| overflow());
    }
    let max_guard = MAX_FIXED_TRIG_W.saturating_sub(scale).max(base_guard);
    let mut guard = base_guard;
    loop {
        let w = scale + guard;
        let (v, exact) = eval(guard)
            .round_to_i128_with_exact(w, scale, mode)
            .unwrap_or_else(|| (overflow(), true));
        if !exact || guard >= max_guard {
            return v;
        }
        guard += TRIG_GUARD_STEP;
    }
}

#[inline]
#[must_use]
pub(crate) fn sin_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    if raw.abs() <= small_x_linear_threshold::<SCALE>() && is_nearest_mode(mode) {
        return raw;
    }
    round_fixed_trig_directed(SCALE, STRICT_GUARD, mode, "sin", |g| {
        sin_fixed(to_fixed_w(raw, g), SCALE + g)
    })
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
    round_fixed_trig_directed(SCALE, STRICT_GUARD, mode, "cos", |g| {
        let w = SCALE + g;
        sin_fixed(to_fixed_w(raw, g).add(wide_half_pi(w)), w)
    })
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
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw == -one_bits {
        return -<crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
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
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
    }
    if raw == -one_bits {
        return -<crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::quarter_pi().0.as_i128();
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
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::pi().0.as_i128();
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
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <crate::D<crate::int::types::Int<2>, SCALE> as DecimalConstants>::pi().0.as_i128();
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
// delegates one line to each `policy::trig` dispatch, whose body
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
    // Integer-regime: the result carries too many integer digits for the
    // 256-bit `Fixed`'s `e^|x|` reassembly — route through the wider
    // `WNarrow` work integer (correctly-rounded, never-exact directed).
    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(raw, scale, w) {
        return crate::algos::exp::exp_series_2limb::sinh_wide_narrow_raw(
            raw,
            scale,
            working_digits,
            mode,
        );
    }
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
    // Integer-regime: route through the wider `WNarrow` work integer (see
    // `sinh_with_raw`).
    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(raw, scale, w) {
        return crate::algos::exp::exp_series_2limb::cosh_wide_narrow_raw(
            raw,
            scale,
            working_digits,
            mode,
        );
    }
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
    let result = ex
        .add(enx)
        .halve()
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::cosh", scale)
        });
    // cosh(x) > 1 strictly for x != 0 (raw == 0 returned 1.0 exactly above).
    // Near the minimum the +x²/2 excess underflows the working scale, so the
    // kernel rounds to exactly 1.0 and a directed-up mode cannot see that the
    // true value sits just above the grid line — re-decide analytically.
    let one_raw = 10_i128.pow(scale);
    if result == one_raw {
        return crate::support::rounding::tiny_above_line_directed(one_raw, 1, mode);
    }
    result
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
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    // tanh is odd; the input sign is reapplied to the non-negative tanh(|x|).
    let neg = raw < 0;
    // Large |x| via the NEGATIVE-exponent identity tanh(|x|) = (1 − m)/(1 + m),
    // m = e^(−2|x|) = (e^|x| − e^−|x|)/(e^|x| + e^−|x|) ÷ (e^|x|/e^−|x|) — exact.
    // Forming the dominant e^(+|x|) directly overflows the 256-bit `Fixed` once
    // |x| ≳ (256·ln2 − w·ln10) (≈ 44 at w = 58), BELOW the all-nines saturation
    // onset |x| ≳ 1.1513·w (`thr_x`) — leaving a panic GAP between them. The
    // identity sidesteps it: for large |x|, m is TINY (|x| = 48 → m = e^−96 ≈
    // 1e−42, trivially inside `Fixed` at scale w), and is formed by `exp_fixed`
    // on the NEGATIVE argument −2|x| whose `2^k` reassembly shifts DOWN, never
    // the overflowing up-shift. So this is the correctly-rounded tanh across the
    // whole large-|x| range. Mirrors `exp_generic::tanh_pos` (the wide path).
    let thr_x = (w as i128) * 1152 / 1000 + 2;
    // Largest working value below 1 (value 1 − 10^−w): the all-nines saturation
    // the caller rounds to ±1 (nearest/away/up) or ±(1−10^−w) (Floor/Trunc).
    let saturated = one_w.sub(Fixed::from_u128_mag(1, false));
    let th = if raw.abs() / 10_i128.pow(scale) > thr_x {
        // Past the onset 2·e^(−2|x|) < 10^−w: skip forming e^(−2|x|) entirely.
        saturated
    } else {
        let v = to_fixed_w(raw, working_digits);
        let av = Fixed {
            negative: false,
            mag: v.mag,
        };
        let m = exp_fixed(av.double().neg(), w);
        if m.is_zero() {
            // |x| just under `thr_x`: m underflowed the working scale, so tanh
            // is all-nines too — same saturation, keeping the boundary continuous.
            saturated
        } else {
            one_w.sub(m).div(one_w.add(m), w)
        }
    };
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
    // atanh(x) = ½·ln((1+x)/(1-x)), value-gated on |x|:
    //
    //  - |x| ≤ 0.98 (1−|x| ≥ 0.02): the 1-log RATIO form ½·ln((1+x)/(1-x)) —
    //    ONE `ln_fixed` (~2× cheaper than the two-log gap form). The ratio
    //    R = (1+|x|)/(1-|x|) ≤ 99 there, so `R·10^w` fits the 256-bit `Fixed`
    //    at any working scale up to the 75-digit ceiling (R < 2²⁵⁶/10⁷⁵ ≈ 115;
    //    the strict path's w = SCALE+30 ≤ 68 has ~10⁷× more headroom). The
    //    division's ≤0.5-ULP-at-w error propagates to atanh as ≤0.25·10⁻ʷ,
    //    absorbed by the ≥30 guard digits, so every mode rounds correctly.
    //
    //  - |x| > 0.98 (near ±1): the GAP form ½·(ln(1+x) − ln(1-x)) — TWO logs,
    //    taken separately so the overflowing ratio is never formed (the
    //    `dab34171` correctness fix: near |x|=1 the ratio reaches ~10^(2·digits),
    //    e.g. atanh(1−10⁻²⁸) at D38 s28 → ~10⁸⁶, past 2²⁵⁶). `1+x` and `1-x`
    //    each lie in (0, 2) and fit; `ln_fixed` handles arguments below 1.
    //
    // Gate in `Fixed` magnitudes: `50·(1−|x|)·10^w ≥ 10^w` ⟺ `1−|x| ≥ 0.02`
    // (the `50·(1−|x|)·10^w ≤ 5·10⁷⁶` intermediate fits 2²⁵⁶).
    if one_w.sub(ax).mul_u128(50).ge_mag(one_w) {
        // Ratio form: R = (1+x)/(1-x) at working scale w (both 1±x are positive
        // for |x| < 1, so R > 0; R < 1 for x < 0). One `ln_fixed`.
        let r = one_w.add(v).div(one_w.sub(v), w);
        return ln_fixed(r, w)
            .halve()
            .round_to_i128_with(w, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale("D38::atanh", scale)
            });
    }
    let ln_num = ln_fixed(one_w.add(v), w);
    let ln_den = ln_fixed(one_w.sub(v), w);
    ln_num
        .sub(ln_den)
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
    let thresh_exp = scale.saturating_sub(scale.div_ceil(3));
    10_i128.pow(thresh_exp)
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

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
    ];

    /// FAST sinh/cosh in `Fixed`, no gate — catching any overflow panic.
    fn fast_hyper_raw(raw: i128, scale: u32, mode: RoundingMode, is_cosh: bool) -> Option<i128> {
        let w = scale + STRICT_GUARD;
        std::panic::catch_unwind(|| {
            let v = to_fixed_w(raw, STRICT_GUARD);
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
            let res = if is_cosh {
                ex.add(enx).halve()
            } else {
                let sh = ex.sub(enx).halve();
                if raw < 0 { sh.neg() } else { sh }
            };
            res.round_to_i128_with(w, scale, mode)
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
                    let w = scale + STRICT_GUARD;
                    // Gate: stays fast unless the integer-regime digit gate fires.
                    if crate::algos::exp::exp_series_2limb::hyper_needs_wide_narrow(raw, scale, w) {
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
                            Ok(v) => v,
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
