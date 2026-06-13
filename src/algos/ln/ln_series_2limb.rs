// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Natural-logarithm series kernel — `ln_fixed` evaluated on the
//! 256-bit `Fixed` guard-digit intermediate with a configurable
//! working-scale guard.
//!
//! The narrow `Int<2>`-storage series path: it serves the narrow
//! D18 / D38 tier, evaluating the log series in the wider `Fixed`
//! intermediate because the narrow storage cannot host the guard
//! digits a correctly-rounded result needs. Captures the four-variant
//! matrix entry shape (`strict` vs `approx`, each with an
//! explicit-rounding-mode sibling) as a single kernel parameterised by
//! `working_digits`.
//!
//! Hosts the shared `Fixed` ln primitives used by every D38 strict-
//! ln callsite plus the `policy::ln` defaults — `STRICT_GUARD`,
//! `ln_fixed`, `wide_ln2`, `wide_ln10` — so the typed-shell file
//! has no `crate::algos::*` or `crate::algos::support::fixed::*` references
//! left.
//!
//! Fast paths preserved verbatim from the typed surface:
//! - `self == 10^SCALE` (i.e. logical 1.0) returns `Self::ZERO`.
//! - `|self - 1| <= 10^(SCALE - ceil(SCALE/2))` returns `(self - 1)`
//!   directly — the linear `ln(1+x) ≈ x` band where the result is
//!   exact at storage precision.
//!
//! Panics on `raw <= 0` (the typed method's contract).

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::narrow_ziv::{self, WZiv};
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, is_nearest_mode};

/// Guard digits added below the storage scale for the D38 strict log
/// family. The 256-bit `Fixed` intermediate runs at
/// `w = SCALE + STRICT_GUARD`; with `SCALE <= 38` that caps `w` at 68
/// digits, comfortably inside the 75-digit window the embedded
/// `ln 2` / `ln 10` constants cover.
pub(crate) const STRICT_GUARD: u32 = 30;

// `ln(2)` and `ln(10)` are embedded at 75 fractional digits — the same
// reference scale `wide_pi` uses — so callers running at the maximum
// strict working scale `W = SCALE + STRICT_GUARD = 38 + 30 = 68` always
// rescale **down** (never up, which would wrap `from_w − to_w` as `u32`
// and silently produce a wrong constant). The 75-digit window covers
// every supported strict scale; widening it further is wasted work
// because the working scale is capped by `D38<SCALE>`'s storage range
// (no input that fits `i128` at `SCALE > 38` exists).
//
// Half-to-even rounded ln(2) × 10^75 and ln(10) × 10^75; both fit an
// `Int<4>` (max ≈ 5.78 × 10⁷⁶). The next-digit rounding is documented
// in-line so the truncation step is auditable from this file alone.

/// Repacks an `Int<4>` (internally `[u64; 4]`) into a
/// `Fixed` magnitude (`[u128; 2]`) sourced at scale `75`.
#[inline]
fn fixed_from_int256(raw: Int<4>) -> Fixed {
    let words = raw.limbs_le();
    Fixed {
        negative: false,
        mag: [
            (words[0] as u128) | ((words[1] as u128) << 64),
            (words[2] as u128) | ((words[3] as u128) << 64),
        ],
    }
}

/// `ln(2)` as a `Fixed` at working scale `w` (`w <= 75`). Sourced from
/// the 75-digit reference and rescaled **down** to `w`.
///
/// Caller-side precondition: `w <= 75`. The D38 strict log family runs
/// at `w = SCALE + STRICT_GUARD`, capped at `38 + 30 = 68`, so every
/// strict call site is comfortably inside the bound. A debug-assert
/// documents the invariant for any future caller.
pub(crate) fn wide_ln2(w: u32) -> Fixed {
    debug_assert!(w <= 75, "wide_ln2: working scale {w} exceeds Fixed capacity");
    fixed_from_int256(crate::consts::ln2_const_n::<4>(
        w,
        crate::support::rounding::RoundingMode::HalfToEven,
    ))
}

/// `ln(10)` as a `Fixed` at working scale `w` (`w <= 75`). Sourced from
/// the 75-digit reference and rescaled **down** to `w`.
///
/// Caller-side precondition: `w <= 75`. See [`wide_ln2`].
pub(crate) fn wide_ln10(w: u32) -> Fixed {
    debug_assert!(w <= 75, "wide_ln10: working scale {w} exceeds Fixed capacity");
    fixed_from_int256(crate::consts::ln10_const_n::<4>(
        w,
        crate::support::rounding::RoundingMode::HalfToEven,
    ))
}

/// Natural logarithm of a positive working-scale value `v_w`, returned
/// at the same working scale `w`.
///
/// Range-reduces `v = 2^k · m` with `m ∈ [1,2)` — the mantissa is
/// recomputed exactly from `v_w` once `k` is known — then evaluates
/// `ln(m) = 2·artanh((m-1)/(m+1))` (`t ∈ [0,1/3]`, fast convergence)
/// and returns `k·ln(2) + ln(m)`.
pub(crate) fn ln_fixed(v_w: Fixed, w: u32) -> Fixed {
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
    let two_w = one_w.double();

    // Range reduction: find k with v ∈ [2^k, 2^(k+1)); m_w = v_w / 2^k.
    let mut k: i32 = v_w.bit_length() as i32 - one_w.bit_length() as i32;
    let m_w = loop {
        let m = if k >= 0 {
            v_w.shr(k as u32)
        } else {
            v_w.shl((-k) as u32)
        };
        if m.ge_mag(two_w) {
            k += 1;
        } else if !m.ge_mag(one_w) {
            k -= 1;
        } else {
            break m;
        }
    };

    // t = (m - 1) / (m + 1) ∈ [0, 1/3]; artanh(t) = t + t³/3 + t⁵/5 + …
    let t = m_w.sub(one_w).div(m_w.add(one_w), w);
    let t2 = t.mul(t, w);
    let mut sum = t;
    let mut term = t;
    let mut j: u128 = 1;
    loop {
        term = term.mul(t2, w);
        let contrib = term.div_small(2 * j + 1);
        if contrib.is_zero() {
            break;
        }
        sum = sum.add(contrib);
        j += 1;
        if j > 400 {
            break;
        }
    }
    let ln_m = sum.double();

    let ln2 = wide_ln2(w);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    k_ln2.add(ln_m)
}

/// D38 natural log with explicit `working_digits` and rounding mode.
/// Called by both `ln_strict_with` (with `working_digits = STRICT_GUARD`)
/// and `ln_approx_with` (with the caller's value).
///
/// Returns the raw `i128` storage at the input's scale; `None` when the
/// correctly-rounded result does not fit the storage (the policy wrapper
/// panics / the `checked_` surface propagates).
#[inline]
#[must_use]
pub(crate) fn ln_with(
    raw: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<Int<2>> {
    ln_with_raw(raw.as_i128(), scale, working_digits, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`ln_with`].
#[inline]
fn ln_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(scale);
    if raw == one_bits {
        return Some(0);
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(scale.saturating_sub(1) / 2);
    if delta.abs() <= ln1p_band && is_nearest_mode(mode) {
        // ln(1 + δ/10^S)·10^S = δ − δ²/(2·10^S) + … . The leading omitted term
        // is δ²/(2·10^S); at the band edge |δ| = 10^k it equals 10^(2k−S)/2, so
        // the linear value `δ` is the nearest-rounded result only while
        // 2k − S < 0 STRICTLY (the term is below half an LSB and the round is
        // not a tie). The band exponent k = ⌊(S−1)/2⌋ gives 2k − S ≤ −1, i.e.
        // ≤ 0.05 LSB — strictly clear of the half-ULP tie for EVERY S. The old
        // k = ⌊S/2⌋ put 2k − S = 0 for even S, so the edge term was exactly
        // 0.5 LSB and `δ` misrounded the tie (ln(0.999) at s6/s18/s28). Directed
        // modes need the true residual sign (the value sits sub-LSB to one side
        // of `δ`), so they fall through to the full working-scale kernel below.
        return Some(delta);
    }
    let w = scale + working_digits;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    ln_fixed(v_w, w).round_to_i128_with(w, scale, mode)
}

/// Strict variant — fixed to `STRICT_GUARD` working digits. Equivalent
/// to `ln_with(raw, scale, STRICT_GUARD, mode)` but keeps the working
/// scale `w = SCALE + STRICT_GUARD` const-folded so LLVM specialises
/// one optimal kernel per `SCALE`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    ln_strict_raw::<SCALE>(raw.as_i128(), mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`ln_strict`].
#[inline]
fn ln_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return Some(0);
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(SCALE.saturating_sub(1) / 2);
    if delta.abs() <= ln1p_band && is_nearest_mode(mode) {
        // See `ln_with_raw`: the band exponent ⌊(S−1)/2⌋ keeps the omitted
        // quadratic term ≤ 0.05 LSB — strictly clear of the half-ULP tie for
        // every S (a ⌊S/2⌋ band hits exactly 0.5 LSB at the edge for even S).
        // The linear approximation `δ` is then the correctly nearest-rounded
        // result inside the band; directed modes need the residual sign and
        // fall through to the full kernel.
        return Some(delta);
    }
    let w = SCALE + STRICT_GUARD;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
    let lv = ln_fixed(v_w, w);
    match lv.round_to_i128_clear_of_tie(w, SCALE, mode) {
        Some(r) => r,
        // Near-tie: the directed deep-near-1 family (`ln(1 ± k·10^-S)`
        // leaves the `δ²/2` deviation below the fixed working scale once
        // `δ² < 2·10^(S−30)`) and any nearest near-half — escalate. The
        // walker resolves every constructible member (deviation depth
        // ≤ 2·38 = 76 ≪ the `WZiv` reach).
        None => narrow_ziv::walk_checked(
            lv.round_to_i128_with(w, SCALE, mode),
            STRICT_GUARD,
            SCALE,
            mode,
            |g| ln_ziv(raw, SCALE, g),
        ),
    }
}

/// One `WZiv` ln probe at working scale `scale + g`.
fn ln_ziv(raw: i128, scale: u32, g: u32) -> WZiv {
    let w = scale + g;
    eg::ln_fixed::<WZiv>(narrow_ziv::lift(raw, g), w, narrow_ziv::ln2_w(w))
}

/// One `WZiv` `ln(x)/ln(b)` ratio probe at working scale `scale + g`.
/// `b_num2` selects the base: positive = the storage `b_raw` (general
/// log), `-2` / `-10` = the constant bases (log2 / log10, whose scaled
/// storage form can overflow `i128` at the maximal scale).
fn log_ratio_ziv(x_raw: i128, b_sel: i128, scale: u32, g: u32) -> WZiv {
    let w = scale + g;
    let ln2 = narrow_ziv::ln2_w(w);
    let lx = eg::ln_fixed::<WZiv>(narrow_ziv::lift(x_raw, g), w, ln2);
    let lb = match b_sel {
        -2 => ln2,
        -10 => narrow_ziv::ln10_w(w),
        b_raw => eg::ln_fixed::<WZiv>(narrow_ziv::lift(b_raw, g), w, ln2),
    };
    eg::div::<WZiv>(lx, lb, w)
}

/// Greatest common divisor (Euclid) on `u128`.
pub(crate) fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// `base^e` in [`WZiv`], `None` when the result would exceed the bounded
/// bit budget (1500 bits — inside `Int<24>`'s 1536 with sign headroom).
/// Square-and-multiply; the bit-length pre-check bounds every
/// intermediate, so no step can overflow.
pub(crate) fn pow_bounded(base: u128, e: u128) -> Option<WZiv> {
    if base == 0 {
        return None; // domain-asserted positive inputs; 0 never verifies
    }
    let bl = (128 - base.leading_zeros()) as u128;
    // Saturating: `e` can be as large as `10^scale` (an irreducible
    // candidate denominator), so the budget product must not wrap.
    if bl.saturating_mul(e) > 1500 {
        return None;
    }
    let mut acc = WZiv::from_i128(1);
    let mut b = WZiv::from_u128(base);
    let mut k = e;
    while k > 0 {
        if k & 1 == 1 {
            acc = acc * b;
        }
        k >>= 1;
        if k > 0 {
            b = b * b;
        }
    }
    Some(acc)
}

/// Exact rational-power pin for the strict log near-tie terminal.
///
/// `num2` is the boundary CANDIDATE in half-ULPs at `scale` (the working
/// ratio doubled and rounded to the nearest integer): even = a grid
/// candidate (a directed near-grid tie, e.g. `log_4(8) = 3/2` on the
/// scale-19 grid), odd = a half candidate (a nearest near-half tie, e.g.
/// `log_4(32) = 5/2` at scale 0). Verifies `log_(bn/bd)(xn/xd) ==
/// num2/(2·10^scale)` EXACTLY via the integer identity `x^q == b^p`
/// (`p/q` = the candidate in lowest terms; `pow_bounded` caps the
/// verification at sizes [`WZiv`] holds — an unverifiable candidate
/// defers to the walker). On a verified exact value the result is
/// decided by exact integer arithmetic: a grid candidate IS the result;
/// a half candidate applies the mode's tie rule. This is what makes the
/// constructible exact-rational families deterministic — the working
/// residual at any finite scale is kernel noise around the boundary,
/// which no escalation can settle.
fn log_rational_pow_pin(
    xn: u128,
    xd: u128,
    bn: u128,
    bd: u128,
    scale: u32,
    num2: i128,
    mode: RoundingMode,
) -> Option<i128> {
    if num2 == 0 {
        return None; // r = 0 ⇔ x == 1, pinned upstream
    }
    let den: u128 = 2 * 10u128.pow(scale);
    let neg = num2 < 0;
    let num = num2.unsigned_abs();
    let g = gcd_u128(num, den);
    let p = num / g;
    let q = den / g;
    // log_b(x) = ±p/q ⇔ x^q == b^(±p): for the negative sign the base
    // fraction inverts.
    let (tn, td) = if neg { (bd, bn) } else { (bn, bd) };
    let lx_n = pow_bounded(xn, q)?;
    let lx_d = pow_bounded(xd, q)?;
    let rb_n = pow_bounded(tn, p)?;
    let rb_d = pow_bounded(td, p)?;
    if lx_n != rb_n || lx_d != rb_d {
        return None;
    }
    // Exact value num2/(2·10^scale): fold the half-ULP form to storage.
    let q_mag = num / 2;
    if num & 1 == 0 {
        return Some(if neg { -(q_mag as i128) } else { q_mag as i128 });
    }
    // Exactly on the half between q_mag and q_mag + 1 (magnitude side):
    // the mode's tie rule, by exact integer arithmetic.
    let bump = crate::support::rounding::should_bump(
        mode,
        core::cmp::Ordering::Equal,
        q_mag & 1 == 1,
        !neg,
    );
    let m = (q_mag + u128::from(bump)) as i128;
    Some(if neg { -m } else { m })
}

/// Reduces `a / b` to lowest terms.
pub(crate) fn reduce_fraction(a: u128, b: u128) -> (u128, u128) {
    let g = gcd_u128(a, b);
    (a / g, b / g)
}

// ── log / log2 / log10 kernels (D38, Fixed fallback) ──────────────

/// Exact-integer-logarithm pin for the D38 log family.
///
/// Returns `Some(k · 10^scale)` (the exact storage representation of the
/// integer `k`) when `value == base^k` exactly at the storage scale, i.e.
/// when the true `log_base(value)` is the exact integer `k`. Off this
/// allow-list the logarithm is irrational (Lindemann–Weierstrass), so the
/// residual is genuinely non-zero and the caller's working-scale kernel
/// runs. Pinning the exact points stops the `ln(value)/ln(base)` round-off
/// from landing a hair off the storage grid line and bumping by one LSB
/// under a directed mode (`Trunc`/`Floor`/`Ceiling`).
///
/// `value_raw` is the storage integer (`x · 10^scale`); `base_int` is
/// the *integer* base (`2` for `log2`, `10` for `log10`, or the integer
/// part of a general `log` base — pass `0` for a non-integer base, which
/// never has an exact integer power). Passing `base_int` rather than the
/// scaled `base · 10^scale` avoids overflowing `i128` when forming
/// `10^(scale+1)` at the maximal scale (the D38 max-scale ln panic).
///
/// The candidate `k` is derived by the caller from the nearest-rounded
/// result. All arithmetic is `i128`; an intermediate that overflows
/// `i128` cannot be a representable exact power, so the check returns
/// `None`.
#[inline]
fn log_exact_int_pin(value_raw: i128, base_int: i128, scale: u32, k: i128) -> Option<i128> {
    let one_s = 10i128.checked_pow(scale)?;
    if k == 0 {
        return (value_raw == one_s).then_some(0);
    }
    // A non-integer base (only the near-1 ill-conditioning probes hit
    // this) raised to an integer `k` is not an integer matching `value`
    // at storage scale, so no exact pin applies.
    if base_int == 0 {
        return None;
    }
    let kk = k.unsigned_abs();
    let exact = if k > 0 {
        // value == base^|k|: compare `base_int^|k|` against the integer
        // part of `value`, requiring `value` to be that exact integer.
        if value_raw % one_s != 0 {
            return None;
        }
        let value_int = value_raw / one_s;
        let mut pow: i128 = 1;
        let mut ok = true;
        for _ in 0..kk {
            match pow.checked_mul(base_int) {
                Some(p) => pow = p,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        ok && pow == value_int
    } else {
        // value == 1 / base^|k|: `value · base^|k| == 1` exactly, i.e.
        // `value_raw · base_int^|k| == 10^scale`. Drive `value_raw` up by
        // `base_int` each step (staying bounded near `10^scale`) and
        // require exact divisibility is not needed — multiplication is
        // exact integer; the product must hit `one_s` precisely.
        let mut cur = value_raw;
        let mut ok = true;
        for _ in 0..kk {
            match cur.checked_mul(base_int) {
                Some(p) => cur = p,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        ok && cur == one_s
    };
    if exact { k.checked_mul(one_s) } else { None }
}

/// `log_base(v) = ln(v) / ln(base)`, both carried in the `Fixed` wide.
/// Used by `policy::log::dispatch` when the D57 borrow path is not
/// available (no `d57` / `wide` feature).
#[inline]
#[must_use]
pub(crate) fn log_with(
    raw: Int<2>,
    base_raw: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<Int<2>> {
    log_with_raw(
        raw.as_i128(),
        base_raw.as_i128(),
        scale,
        working_digits,
        mode,
    )
    .map(Int::<2>::from_i128)
}

/// `i128` core of [`log_with`].
#[inline]
fn log_with_raw(
    raw: i128,
    base_raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<i128> {
    assert!(raw > 0, "D38::log: argument must be positive");
    assert!(base_raw > 0, "D38::log: base must be positive");
    let w = scale + working_digits;
    let pow = 10u128.pow(working_digits);
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(pow);
    let b_w = Fixed::from_u128_mag(base_raw as u128, false).mul_u128(pow);
    let ln_b = ln_fixed(b_w, w);
    assert!(
        !ln_b.is_zero(),
        "D38::log: base must not equal 1 (ln(1) is zero)"
    );
    let ratio = ln_fixed(v_w, w).div(ln_b, w);
    // Exact-power pin: `value == base^k` ⇒ result is exactly `k`.
    // Reduce the storage `base_raw` to its integer base (`base_raw /
    // 10^scale`) here, without forming `base · 10^scale`, so the pin's
    // integer-domain check never carries (and never overflows) the
    // scale factor — `0` flags a non-integer base (no exact pin).
    let k = ratio.round_to_nearest_int(w);
    let base_int = match 10i128.checked_pow(scale) {
        Some(one_s) if base_raw % one_s == 0 => base_raw / one_s,
        _ => 0,
    };
    if let Some(pinned) = log_exact_int_pin(raw, base_int, scale, k) {
        return Some(pinned);
    }
    ratio.round_to_i128_with(w, scale, mode)
}

/// Const-folded strict variant of [`log_with`] with the near-tie
/// protected terminal: the exact-integer pin, then the clear-of-tie
/// single shot, then (near a boundary) the exact rational-power pin —
/// `log_4(8) = 3/2` exactly on the scale-19 grid, `log_4(32) = 5/2`
/// exactly on the scale-0 half — and finally the Ziv walker for the
/// genuinely transcendental near-ties. `None` = result out of storage
/// range.
#[inline]
#[must_use]
pub(crate) fn log_strict<const SCALE: u32>(raw: Int<2>, base_raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    log_strict_raw(raw.as_i128(), base_raw.as_i128(), SCALE, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`log_strict`].
fn log_strict_raw(raw: i128, base_raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "D38::log: argument must be positive");
    assert!(base_raw > 0, "D38::log: base must be positive");
    let w = scale + STRICT_GUARD;
    let pow = 10u128.pow(STRICT_GUARD);
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(pow);
    let b_w = Fixed::from_u128_mag(base_raw as u128, false).mul_u128(pow);
    let ln_b = ln_fixed(b_w, w);
    assert!(
        !ln_b.is_zero(),
        "D38::log: base must not equal 1 (ln(1) is zero)"
    );
    let ratio = ln_fixed(v_w, w).div(ln_b, w);
    // Exact-power pin (see `log_with_raw`).
    let k = ratio.round_to_nearest_int(w);
    let base_int = match 10i128.checked_pow(scale) {
        Some(one_s) if base_raw % one_s == 0 => base_raw / one_s,
        _ => 0,
    };
    if let Some(pinned) = log_exact_int_pin(raw, base_int, scale, k) {
        return Some(pinned);
    }
    match ratio.round_to_i128_clear_of_tie(w, scale, mode) {
        Some(r) => r,
        None => {
            // Near a boundary: try the exact rational-power pin before
            // walking (an exact rational log never resolves by escalation
            // — the residual at every depth is kernel noise around the
            // boundary).
            if let Some(num2) = ratio.double().round_to_i128_with(w, scale, RoundingMode::HalfToEven)
            {
                let one_s = 10u128.pow(scale);
                let (xn, xd) = reduce_fraction(raw as u128, one_s);
                let (bn, bd) = reduce_fraction(base_raw as u128, one_s);
                if let Some(pinned) =
                    log_rational_pow_pin(xn, xd, bn, bd, scale, num2, mode)
                {
                    return Some(pinned);
                }
            }
            narrow_ziv::walk_checked(
                ratio.round_to_i128_with(w, scale, mode),
                STRICT_GUARD,
                scale,
                mode,
                |g| log_ratio_ziv(raw, base_raw, scale, g),
            )
        }
    }
}

/// `log2(v) = ln(v) / ln(2)`, `Fixed`-wide fallback for D38.
#[inline]
#[must_use]
pub(crate) fn log2_with(raw: Int<2>, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<Int<2>> {
    log2_with_raw(raw.as_i128(), scale, working_digits, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`log2_with`].
#[inline]
fn log2_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "D38::log2: argument must be positive");
    let w = scale + working_digits;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    let ratio = ln_fixed(v_w, w).div(wide_ln2(w), w);
    // Exact-power pin: `value == 2^k` ⇒ result is exactly `k`.
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 2, scale, k) {
        return Some(pinned);
    }
    ratio.round_to_i128_with(w, scale, mode)
}

/// `None` = result out of storage range. The strict terminal is
/// near-tie protected; `log2` of an on-grid rational is integer (the
/// exact pin) or IRRATIONAL (`2^(p/q)` is rational only for `q | p`), so
/// no rational-power pin is needed — the walker covers the rest.
#[inline]
#[must_use]
pub(crate) fn log2_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    log2_strict_raw(raw.as_i128(), SCALE, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`log2_strict`].
fn log2_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "D38::log2: argument must be positive");
    let w = scale + STRICT_GUARD;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
    let ratio = ln_fixed(v_w, w).div(wide_ln2(w), w);
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 2, scale, k) {
        return Some(pinned);
    }
    match ratio.round_to_i128_clear_of_tie(w, scale, mode) {
        Some(r) => r,
        None => narrow_ziv::walk_checked(
            ratio.round_to_i128_with(w, scale, mode),
            STRICT_GUARD,
            scale,
            mode,
            |g| log_ratio_ziv(raw, -2, scale, g),
        ),
    }
}

/// `log10(v) = ln(v) / ln(10)`, `Fixed`-wide fallback for D38.
#[inline]
#[must_use]
pub(crate) fn log10_with(raw: Int<2>, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<Int<2>> {
    log10_with_raw(raw.as_i128(), scale, working_digits, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`log10_with`].
#[inline]
fn log10_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "D38::log10: argument must be positive");
    let w = scale + working_digits;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    let ratio = ln_fixed(v_w, w).div(wide_ln10(w), w);
    // Exact-power pin: `value == 10^k` ⇒ result is exactly `k`.
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 10, scale, k) {
        return Some(pinned);
    }
    ratio.round_to_i128_with(w, scale, mode)
}

/// `None` = result out of storage range. Near-tie protected like
/// [`log2_strict`] (`log10` of an on-grid rational is integer or
/// irrational).
#[inline]
#[must_use]
pub(crate) fn log10_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    log10_strict_raw(raw.as_i128(), SCALE, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`log10_strict`].
fn log10_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    assert!(raw > 0, "D38::log10: argument must be positive");
    let w = scale + STRICT_GUARD;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
    let ratio = ln_fixed(v_w, w).div(wide_ln10(w), w);
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 10, scale, k) {
        return Some(pinned);
    }
    match ratio.round_to_i128_clear_of_tie(w, scale, mode) {
        Some(r) => r,
        None => narrow_ziv::walk_checked(
            ratio.round_to_i128_with(w, scale, mode),
            STRICT_GUARD,
            scale,
            mode,
            |g| log_ratio_ziv(raw, -10, scale, g),
        ),
    }
}

// ── Near-tie pins (see `trig_series_2limb::near_tie_pins`) ───────────
#[cfg(test)]
mod near_tie_pins {
    use super::*;

    #[test]
    fn ln_directed_near_one_deep_d38_s38() {
        // ln(1 + 1e-38) = δ − δ²/2 + … with δ = 1 ULP exactly: the true
        // value is δ·(1 − 5e-39), strictly below the grid line δ by a
        // deviation (depth 76.3) under every reachable fixed working
        // scale. Concavity decides: Floor/Trunc → 0, Ceiling → 1.
        // failing-first: the single shot at w = 68 saw a zero residual
        // and returned δ = 1 for Floor/Trunc. Oracle: the exact rational
        // partial + the strictly-negative tail (narrow_tie_derive.py).
        let raw = Int::<2>::from_i128(10_i128.pow(38) + 1);
        assert_eq!(
            ln_strict::<38>(raw, RoundingMode::Floor).map(|v| v.as_i128()),
            Some(0),
            "ln Floor"
        );
        assert_eq!(
            ln_strict::<38>(raw, RoundingMode::Trunc).map(|v| v.as_i128()),
            Some(0),
            "ln Trunc"
        );
        assert_eq!(
            ln_strict::<38>(raw, RoundingMode::Ceiling).map(|v| v.as_i128()),
            Some(1),
            "ln Ceiling"
        );
        // below 1: ln(1 − 1e-38) = −δ − δ²/2: strictly below −δ.
        let raw_lo = Int::<2>::from_i128(10_i128.pow(38) - 1);
        assert_eq!(
            ln_strict::<38>(raw_lo, RoundingMode::Floor).map(|v| v.as_i128()),
            Some(-2),
            "ln(1−ulp) Floor"
        );
        assert_eq!(
            ln_strict::<38>(raw_lo, RoundingMode::Trunc).map(|v| v.as_i128()),
            Some(-1),
            "ln(1−ulp) Trunc"
        );
    }

    #[test]
    fn ln_directed_deep_band_sweep() {
        // deep zone: deviation delta^2/(2*10^S) below 10^-(S+30) working
        // resolution <=> delta^2 < 2*10^(S-30)... in working units at w:
        // dev_working = delta^2/(2*10^S)*10^30 < 1 <=> delta^2 < 2*10^(S-30)
        let mut bad = 0u32;
        for s in 31u32..=38 {
            let one: i128 = 10_i128.pow(s);
            let max_d = ((2.0 * 10f64.powi(s as i32 - 30)).sqrt() as i128).max(1);
            let mut d: i128 = 1;
            while d <= max_d {
                let raw = Int::<2>::from_i128(one + d);
                let fl = ln_strict_dyn(s, raw, RoundingMode::Floor);
                let ce = ln_strict_dyn(s, raw, RoundingMode::Ceiling);
                if fl != Some(d - 1) || ce != Some(d) {
                    println!("LIVE ln(1+{d}ulp) S={s}: Floor={fl:?} (want {}), Ceil={ce:?} (want {d})", d-1);
                    bad += 1;
                }
                let raw_lo = Int::<2>::from_i128(one - d);
                let fl2 = ln_strict_dyn(s, raw_lo, RoundingMode::Floor);
                let tr2 = ln_strict_dyn(s, raw_lo, RoundingMode::Trunc);
                if fl2 != Some(-d - 1) || tr2 != Some(-d) {
                    println!("LIVE ln(1-{d}ulp) S={s}: Floor={fl2:?} (want {}), Trunc={tr2:?} (want {})", -d-1, -d);
                    bad += 1;
                }
                d = if d < 20 { d + 1 } else { d + d / 3 };
            }
        }
        assert_eq!(bad, 0, "ln deep directed band has live misrounds");
    }

    fn ln_strict_dyn(s: u32, raw: Int<2>, mode: RoundingMode) -> Option<i128> {
        // The STRICT path (const-scale): the fixed near-tie terminal.
        let v = match s {
            31 => ln_strict::<31>(raw, mode),
            32 => ln_strict::<32>(raw, mode),
            33 => ln_strict::<33>(raw, mode),
            34 => ln_strict::<34>(raw, mode),
            35 => ln_strict::<35>(raw, mode),
            36 => ln_strict::<36>(raw, mode),
            37 => ln_strict::<37>(raw, mode),
            38 => ln_strict::<38>(raw, mode),
            _ => unreachable!(),
        };
        v.map(|v| v.as_i128())
    }

    #[test]
    fn log_fractional_base_high_scale_budget_product_does_not_wrap() {
        // the rational-pin budget product `bl * e` can wrap u128
        // for an irreducible candidate denominator (q ~ 2*10^37) on
        // log(0.1096614350149675660535769418, 0.0182017066872921546105935121)
        // at D38<37> (the golden log.golden:18 panic). The saturating
        // budget must DEFER (walker), never wrap.
        let xr: i128 = 1096614350149675660535769418_i128 * 1_000_000_000;
        let br: i128 = 182017066872921546105935121_i128 * 1_000_000_000;
        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ] {
            let r = log_strict_raw(xr, br, 37, mode);
            assert!(r.is_some(), "log fractional-base s37 mode={mode:?}");
        }
    }

    #[test]
    fn log_rational_exact_d38() {
        // log_4(8) = 3/2 EXACTLY (8² = 4³): on the storage grid at
        // SCALE 19, so every mode must return exactly 1.5. failing-first:
        // the ln(8)/ln(4) single shot lands a hair off the grid line and
        // the directed modes stepped 1 LSB off the exact power.
        let one19 = 10_i128.pow(19);
        let x = Int::<2>::from_i128(8 * one19);
        let b = Int::<2>::from_i128(4 * one19);
        for mode in [
            RoundingMode::HalfToEven,
            RoundingMode::HalfAwayFromZero,
            RoundingMode::HalfTowardZero,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
            RoundingMode::Trunc,
        ] {
            assert_eq!(
                log_strict_raw(x.as_i128(), b.as_i128(), 19, mode),
                Some(15 * one19 / 10),
                "log_4(8) mode={mode:?}"
            );
        }
    }

    #[test]
    fn log_rational_half_tie_s0() {
        // log_4(32) = 5/2 EXACTLY (32² = 4⁵): at SCALE 0 the true value
        // sits EXACTLY on the half between 2 and 3 — a genuine tie that
        // only exact integer arithmetic can certify (the working-scale
        // residual is kernel noise around the half). HalfToEven → 2
        // (even), HalfAwayFromZero → 3, HalfTowardZero → 2.
        let x = Int::<2>::from_i128(32);
        let b = Int::<2>::from_i128(4);
        assert_eq!(
            log_strict_raw(x.as_i128(), b.as_i128(), 0, RoundingMode::HalfToEven),
            Some(2),
            "log_4(32) HalfToEven"
        );
        assert_eq!(
            log_strict_raw(x.as_i128(), b.as_i128(), 0, RoundingMode::HalfAwayFromZero),
            Some(3),
            "log_4(32) HalfAwayFromZero"
        );
        assert_eq!(
            log_strict_raw(x.as_i128(), b.as_i128(), 0, RoundingMode::HalfTowardZero),
            Some(2),
            "log_4(32) HalfTowardZero"
        );
        // log_16(8) = 3/4 (8⁴ = 16³): a half-tie at SCALE 1 (7.5 tenths).
        let x8 = Int::<2>::from_i128(80);
        let b16 = Int::<2>::from_i128(160);
        assert_eq!(
            log_strict_raw(x8.as_i128(), b16.as_i128(), 1, RoundingMode::HalfToEven),
            Some(8),
            "log_16(8) s1 HalfToEven"
        );
        assert_eq!(
            log_strict_raw(x8.as_i128(), b16.as_i128(), 1, RoundingMode::HalfTowardZero),
            Some(7),
            "log_16(8) s1 HalfTowardZero"
        );
    }
}
