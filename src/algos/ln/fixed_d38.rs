//! D38 natural-logarithm kernel — `ln_fixed` on the 256-bit `Fixed`
//! intermediate with a configurable working-scale guard.
//!
//! Width-level specialisation for D38: the hand-tuned ln path that
//! has shipped since before the algorithm library existed. Captures
//! the four-variant matrix entry shape (`strict` vs `approx`, each
//! with an explicit-rounding-mode sibling) as a single kernel
//! parameterised by `working_digits`.
//!
//! Hosts the shared `Fixed` ln primitives used by every D38 strict-
//! ln callsite plus the `LnPolicy` defaults — `STRICT_GUARD`,
//! `ln_fixed`, `wide_ln2`, `wide_ln10` — so the typed-shell file
//! has no `crate::algos::*` or `crate::algos::fixed_d38::*` references
//! left.
//!
//! Fast paths preserved verbatim from the typed surface:
//! - `self == 10^SCALE` (i.e. logical 1.0) returns `Self::ZERO`.
//! - `|self - 1| <= 10^(SCALE - ceil(SCALE/2))` returns `(self - 1)`
//!   directly — the linear `ln(1+x) ≈ x` band where the result is
//!   exact at storage precision.
//!
//! Panics on `raw <= 0` (the typed method's contract).

use crate::algos::fixed_d38::Fixed;
use crate::support::rounding::{is_nearest_mode, RoundingMode};

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
// `Int256` (max ≈ 5.78 × 10⁷⁶). The next-digit rounding is documented
// in-line so the truncation step is auditable from this file alone.

/// `ln(2) × 10^75`, half-to-even rounded (76th frac digit is 4 — round
/// down). Source: high-precision evaluation of the natural logarithm.
const LN2_S75: &str =
    "693147180559945309417232121458176568075500134360255254120680009493393621969";

/// `ln(10) × 10^75`, half-to-even rounded (76th frac digit is 3 — round
/// down). Source: high-precision evaluation of the natural logarithm.
const LN10_S75: &str =
    "2302585092994045684017991454684364207601101488628772976033327900967572609677";

const LN2_RAW: crate::wide_int::Int256 =
    match crate::wide_int::Int256::from_str_radix(LN2_S75, 10) {
        Ok(v) => v,
        Err(_) => panic!("algos::ln::fixed_d38: LN2_S75 not parseable"),
    };

const LN10_RAW: crate::wide_int::Int256 =
    match crate::wide_int::Int256::from_str_radix(LN10_S75, 10) {
        Ok(v) => v,
        Err(_) => panic!("algos::ln::fixed_d38: LN10_S75 not parseable"),
    };

/// Repacks an `Int256` reference (internally `[u64; 4]`) into a
/// `Fixed` magnitude (`[u128; 2]`) sourced at scale `75`.
#[inline]
fn fixed_from_int256(raw: crate::wide_int::Int256) -> Fixed {
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
    debug_assert!(w <= 75, "wide_ln2: working scale {w} exceeds embedded 75-digit ln 2");
    let ln2_at_75 = fixed_from_int256(LN2_RAW);
    if w == 75 { ln2_at_75 } else { ln2_at_75.rescale_down(75, w) }
}

/// `ln(10)` as a `Fixed` at working scale `w` (`w <= 75`). Sourced from
/// the 75-digit reference and rescaled **down** to `w`.
///
/// Caller-side precondition: `w <= 75`. See [`wide_ln2`].
pub(crate) fn wide_ln10(w: u32) -> Fixed {
    debug_assert!(w <= 75, "wide_ln10: working scale {w} exceeds embedded 75-digit ln 10");
    let ln10_at_75 = fixed_from_int256(LN10_RAW);
    if w == 75 { ln10_at_75 } else { ln10_at_75.rescale_down(75, w) }
}

/// Natural logarithm of a positive working-scale value `v_w`, returned
/// at the same working scale `w`.
///
/// Range-reduces `v = 2^k · m` with `m ∈ [1,2)` — the mantissa is
/// recomputed exactly from `v_w` once `k` is known — then evaluates
/// `ln(m) = 2·artanh((m-1)/(m+1))` (`t ∈ [0,1/3]`, fast convergence)
/// and returns `k·ln(2) + ln(m)`.
pub(crate) fn ln_fixed(v_w: Fixed, w: u32) -> Fixed {
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
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
/// Returns the raw `i128` storage at the input's scale.
#[inline]
#[must_use]
pub(crate) fn ln_with(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(scale);
    if raw == one_bits {
        return 0;
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(scale.saturating_sub((scale + 1) / 2));
    if delta.abs() <= ln1p_band && is_nearest_mode(mode) {
        // ln(1 + δ/10^S)·10^S = δ − δ²/(2·10^S) + … : within the band
        // the quadratic term is below half an LSB, so the nearest-rounded
        // result is exactly `delta`. Directed modes need the true residual
        // sign (the value sits sub-LSB to one side of `delta`), so they
        // fall through to the full working-scale kernel below.
        return delta;
    }
    let w = scale + working_digits;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    ln_fixed(v_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("ln kernel", scale))
}

/// Strict variant — fixed to `STRICT_GUARD` working digits. Equivalent
/// to `ln_with(raw, scale, STRICT_GUARD, mode)` but keeps the working
/// scale `w = SCALE + STRICT_GUARD` const-folded so LLVM specialises
/// one optimal kernel per `SCALE`.
#[inline]
#[must_use]
pub(crate) fn ln_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    assert!(raw > 0, "ln kernel: argument must be positive");
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    let delta = raw - one_bits;
    let ln1p_band: i128 = 10_i128.pow(SCALE.saturating_sub((SCALE + 1) / 2));
    if delta.abs() <= ln1p_band && is_nearest_mode(mode) {
        // See `ln_with`: the linear approximation is correctly rounded to
        // nearest inside the band but loses the residual sign that directed
        // modes require, so those fall through to the full kernel.
        return delta;
    }
    let w = SCALE + STRICT_GUARD;
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(STRICT_GUARD));
    ln_fixed(v_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("ln kernel", SCALE))
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
    if exact {
        k.checked_mul(one_s)
    } else {
        None
    }
}


/// `log_base(v) = ln(v) / ln(base)`, both carried in the `Fixed` wide.
/// Used by `LnPolicy::log_impl` when the D57 borrow path is not
/// available (no `d57` / `wide` feature).
#[inline]
#[must_use]
pub(crate) fn log_with(
    raw: i128,
    base_raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    assert!(raw > 0, "D38::log: argument must be positive");
    assert!(base_raw > 0, "D38::log: base must be positive");
    let w = scale + working_digits;
    let pow = 10u128.pow(working_digits);
    let v_w = Fixed::from_u128_mag(raw as u128, false).mul_u128(pow);
    let b_w = Fixed::from_u128_mag(base_raw as u128, false).mul_u128(pow);
    let ln_b = ln_fixed(b_w, w);
    assert!(!ln_b.is_zero(), "D38::log: base must not equal 1 (ln(1) is zero)");
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
        return pinned;
    }
    ratio
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("D38::log", scale))
}

/// Const-folded strict variant of [`log_with`].
#[inline]
#[must_use]
pub(crate) fn log_strict<const SCALE: u32>(
    raw: i128,
    base_raw: i128,
    mode: RoundingMode,
) -> i128 {
    log_with(raw, base_raw, SCALE, STRICT_GUARD, mode)
}

/// `log2(v) = ln(v) / ln(2)`, `Fixed`-wide fallback for D38.
#[inline]
#[must_use]
pub(crate) fn log2_with(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    assert!(raw > 0, "D38::log2: argument must be positive");
    let w = scale + working_digits;
    let v_w =
        Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    let ratio = ln_fixed(v_w, w).div(wide_ln2(w), w);
    // Exact-power pin: `value == 2^k` ⇒ result is exactly `k`.
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 2, scale, k) {
        return pinned;
    }
    ratio
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("D38::log2", scale))
}

#[inline]
#[must_use]
pub(crate) fn log2_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    log2_with(raw, SCALE, STRICT_GUARD, mode)
}

/// `log10(v) = ln(v) / ln(10)`, `Fixed`-wide fallback for D38.
#[inline]
#[must_use]
pub(crate) fn log10_with(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    assert!(raw > 0, "D38::log10: argument must be positive");
    let w = scale + working_digits;
    let v_w =
        Fixed::from_u128_mag(raw as u128, false).mul_u128(10u128.pow(working_digits));
    let ratio = ln_fixed(v_w, w).div(wide_ln10(w), w);
    // Exact-power pin: `value == 10^k` ⇒ result is exactly `k`.
    let k = ratio.round_to_nearest_int(w);
    if let Some(pinned) = log_exact_int_pin(raw, 10, scale, k) {
        return pinned;
    }
    ratio
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("D38::log10", scale))
}

#[inline]
#[must_use]
pub(crate) fn log10_strict<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    log10_with(raw, SCALE, STRICT_GUARD, mode)
}
