//! Exponential series kernel — `exp_fixed` evaluated on the 256-bit
//! `Fixed` guard-digit intermediate, parameterised by working-digit
//! guard.
//!
//! The narrow `Int<2>`-storage series path: it serves the narrow
//! D18 / D38 tier, where the result must be correctly rounded but the
//! storage is too small to host the guard digits directly, so the
//! Taylor evaluation runs in the wider `Fixed` intermediate. Returns
//! the raw `i128` storage at the input's scale; the typed method shell
//! handles the panic-on-overflow message.
//!
//! Hosts the shared `Fixed` exp primitive used by the `policy::exp`
//! defaults (`exp_fixed`) so the typed-shell file has no
//! `crate::algos::*` or `crate::algos::support::fixed::*` references left.

use crate::algos::support::fixed::Fixed;
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, wide_ln2};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Work integer for the narrow integer-regime / MAX-scale exp fallback.
///
/// `Int<24>` is 1536 bits ≈ 462 decimal digits — far wider than the
/// 256-bit `Fixed` (~77 digits) the normal narrow path runs in. The
/// largest D38 result fits 38 storage digits, and at the strict working
/// scale `w = SCALE + STRICT_GUARD ≤ 68` the internal `exp_fixed` peak
/// (`≈ 2·w_ext`, `w_ext = w + extra`, `extra ≈ result_int_digits`) tops
/// out near `2·(68 + 60) ≈ 256` digits, so `Int<24>` holds it with a
/// comfortable margin for every D38 (and D18) cell. The work width is the
/// fixed [`WNarrow`] type, NOT a const work-width parameter — it is a
/// concrete wider integer the generic [`exp_generic::exp_fixed`] runs in.
type WNarrow = Int<24>;

/// Integer-digit count of `e^x` for the storage value `raw` at `scale`
/// (`x = raw / 10^scale`). For `x ≤ 0` (`e^x ≤ 1`) the result has a
/// single integer digit (`0` or `1`). For `x > 0`, `e^x` has
/// `floor(x·log10 e) + 1` integer digits, computed in exact `i128`
/// arithmetic from the rational bound `log10 e ≈ 434295 / 1_000_000`
/// (rounded UP via `div_ceil`, so the digit count is never UNDER-stated).
/// Over-stating is the safe direction for the [`narrow_fixed_fits`] gate:
/// it errs toward routing a borderline cell to the wider work integer.
#[inline]
fn exp_result_int_digits(raw: i128, scale: u32) -> u32 {
    if raw <= 0 {
        return 1;
    }
    let one_s = 10i128.pow(scale);
    let num = (raw as u128) * 434_295;
    let den = (one_s as u128) * 1_000_000;
    (num.div_ceil(den) as u32) + 1
}

/// Whether the 256-bit `Fixed` has headroom to compute `e^x` correctly at
/// working scale `w` for the storage value `raw` at `scale`.
///
/// `exp_fixed`'s internal peak is the `2·w_ext` squaring (`w_ext = w +
/// extra`, `extra ≈ result_int_digits` from the `2^k` amplification lift)
/// — the same true peak the wide-tier gate models. The `Fixed` holds ~76
/// decimal digits (256 bits · log10 2). We require that peak plus an
/// 8-digit margin to stay under that, so the normal fast path keeps every
/// cell it can round correctly and only the genuine integer-regime /
/// near-overflow cells take the wider [`WNarrow`] route.
#[inline]
fn narrow_fixed_fits(raw: i128, scale: u32, w: u32) -> bool {
    let result_digits = exp_result_int_digits(raw, scale);
    // Mirror `exp_generic::exp_fixed`'s `extra` lift: `extra ≈
    // result_digits + margin`, `w_ext = w + extra`, squaring peak
    // `2·w_ext`.
    let extra = result_digits + 12 + (result_digits >> 2);
    let w_ext = w + extra;
    let need = 2 * w_ext + 8;
    // Fixed capacity in decimal digits: 256 · log10(2) ≈ 76.
    need < 76
}

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
    let one_w = Fixed {
        negative: false,
        mag: Fixed::pow10(w),
    };
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
        assert!(
            sum.bit_length() + shift <= 256,
            "D38::exp: result overflows the representable range"
        );
        sum.shl(shift)
    } else {
        sum.shr((-k) as u32)
    }
}

/// Narrow integer-regime / MAX-scale `e^x` fallback, evaluated in the
/// wider [`WNarrow`] (`Int<24>`) work integer instead of the 256-bit
/// `Fixed`, then narrowed back to `i128` storage with correctly-rounded
/// directed / nearest rounding.
///
/// Used when the result carries too many integer digits for the `Fixed`
/// to hold the `exp_fixed` peak ([`narrow_fixed_fits`] is false), or when
/// a directed mode needs the never-exact treatment of a sub-resolution
/// `e^(negative)` (`exp(-76)·10^0 ≈ 0` must round Ceiling up to `1`, not
/// truncate to `0`). The wider work integer gives the `2^k` reassembly
/// the headroom the flat-`w` `Fixed` lacks, and [`exp_generic::exp_fixed`]
/// already returns the smallest positive working value (`10^-w`) for a
/// deep-underflow `e^(negative)` so the sign is preserved into the
/// rounding. `exp_generic::exp_fixed::<WNarrow>` is the SAME range-reduce
/// → squaring-Taylor → `2^k`-reassemble algorithm as the per-tier wide
/// `exp_fixed`, just run in the wider `Int<24>` — one generic kernel, no
/// per-tier copy.
fn exp_wide_narrow_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    use crate::algos::exp::exp_generic;

    let w = scale + working_digits;
    let negative_input = raw < 0;
    let v_mag = WNarrow::from_i128(raw.unsigned_abs() as i128) * WNarrow::TEN.pow(working_digits);
    let v_w = if negative_input { -v_mag } else { v_mag };

    let ex = exp_generic::exp_fixed::<WNarrow>(v_w, w);
    narrow_round_mag(ex, working_digits, mode, true, false)
}

/// Narrows a non-negative [`WNarrow`] working-scale magnitude `mag`
/// (`= value · 10^w`, `value > 0` and irrational at a non-trivial
/// argument) to a signed `i128` storage value at scale `w − shift` under
/// `mode`. `never_exact` mirrors the wide directed path: a zero working
/// residual is treated as a present positive sub-resolution fraction
/// (bumps Ceiling, not Floor/Trunc). `result_neg` reapplies an odd
/// function's sign AFTER rounding the magnitude.
#[inline]
fn narrow_round_mag(
    mag: WNarrow,
    shift: u32,
    mode: RoundingMode,
    never_exact: bool,
    result_neg: bool,
) -> i128 {
    use crate::support::rounding::{is_nearest_mode, should_bump};
    let divisor = WNarrow::TEN.pow(shift);
    let (q, rem) = mag.div_rem(divisor);
    let result_positive = !result_neg;
    let bump = if rem != WNarrow::ZERO {
        if is_nearest_mode(mode) {
            let comp = divisor - rem;
            let cmp_r = rem.cmp(&comp);
            should_bump(mode, cmp_r, q.bit(0), result_positive)
        } else {
            match mode {
                RoundingMode::Ceiling => result_positive,
                RoundingMode::Floor => !result_positive,
                _ => false, // Trunc
            }
        }
    } else if never_exact {
        // Present-and-positive sub-resolution residual.
        match mode {
            RoundingMode::Ceiling => result_positive,
            RoundingMode::Floor => !result_positive,
            _ => false,
        }
    } else {
        false
    };
    let q_mag = if bump { q + WNarrow::ONE } else { q };
    let signed = if result_neg { -q_mag } else { q_mag };
    signed.to_i128()
}

/// `sinh(x)` / `cosh(x)` magnitude `(e^|x| ∓ e^-|x|)/2` at working scale
/// `w`, computed in the wide [`WNarrow`] work integer. Returns the
/// non-negative `sinh(|x|)` / `cosh(|x|)`; the odd-function sign is
/// reapplied by the caller via [`narrow_round_mag`].
#[inline]
fn hyper_pos_wide_narrow(av_w: WNarrow, w: u32, is_cosh: bool) -> WNarrow {
    use crate::algos::exp::exp_generic;
    let ex = exp_generic::exp_fixed::<WNarrow>(av_w, w);
    let one_w = WNarrow::TEN.pow(w);
    // `ex = e^|x|·10^w`. The reciprocal at the same scale is `e^-|x|·10^w
    // = 10^(2w) / ex`. For the integer-regime |x| this is a tiny positive
    // value (≪ 1 ULP-of-storage), formed in the wide integer to avoid the
    // `Fixed` overflow.
    let (enx, _r) = (one_w * one_w).div_rem(ex);
    let two = WNarrow::from_i128(2);
    if is_cosh {
        (ex + enx).div_rem(two).0
    } else {
        (ex - enx).div_rem(two).0
    }
}

/// Narrow integer-regime `sinh(x)` via the wide [`WNarrow`] work integer.
/// Routed from [`crate::algos::trig::trig_series_2limb::sinh_with_raw`]
/// when the result exceeds the 256-bit `Fixed`'s headroom. `sinh` is odd.
pub(crate) fn sinh_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let w = scale + working_digits;
    let neg = raw < 0;
    let av = WNarrow::from_i128(raw.unsigned_abs() as i128) * WNarrow::TEN.pow(working_digits);
    let sh = hyper_pos_wide_narrow(av, w, false);
    narrow_round_mag(sh, working_digits, mode, true, neg)
}

/// Narrow integer-regime `cosh(x)` via the wide [`WNarrow`] work integer.
/// Routed from [`crate::algos::trig::trig_series_2limb::cosh_with_raw`]
/// when the result exceeds the 256-bit `Fixed`'s headroom. `cosh` is even
/// (always non-negative).
pub(crate) fn cosh_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let w = scale + working_digits;
    let av = WNarrow::from_i128(raw.unsigned_abs() as i128) * WNarrow::TEN.pow(working_digits);
    let ch = hyper_pos_wide_narrow(av, w, true);
    narrow_round_mag(ch, working_digits, mode, true, false)
}

/// Whether the narrow `sinh`/`cosh` result for `raw` at `scale` exceeds
/// the 256-bit `Fixed`'s headroom and must route through [`WNarrow`].
/// `sinh(x)`/`cosh(x) ≈ e^|x|/2`, so the result's integer-digit count
/// matches `e^|x|`'s — reuse the exp gate on `|raw|`.
#[inline]
pub(crate) fn hyper_needs_wide_narrow(raw: i128, scale: u32, w: u32) -> bool {
    !narrow_fixed_fits(raw.unsigned_abs() as i128, scale, w)
}

/// `e^x` with caller-chosen `working_digits` above the storage scale.
#[inline]
#[must_use]
pub(crate) fn exp_with(raw: Int<2>, scale: u32, working_digits: u32, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(exp_with_raw(raw.as_i128(), scale, working_digits, mode))
}

/// `i128` core of [`exp_with`].
#[inline]
fn exp_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale); // ONE for this scale
    }
    let w = scale + working_digits;
    // Integer-regime / MAX-scale / sub-resolution-directed cells exceed the
    // 256-bit `Fixed`'s headroom (the `2^k` reassembly peak overflows) or
    // need the never-exact directed rounding the flat-`w` `Fixed` path
    // lacks — route them through the wider `WNarrow` work integer.
    if !narrow_fixed_fits(raw, scale, w) || !crate::support::rounding::is_nearest_mode(mode) {
        return exp_wide_narrow_raw(raw, scale, working_digits, mode);
    }
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("exp kernel", scale)
        })
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(exp_strict_raw::<SCALE>(raw.as_i128(), mode))
}

/// `i128` core of [`exp_strict`].
#[inline]
fn exp_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    // See [`exp_with_raw`]: integer-regime / MAX-scale / directed-mode
    // cells route through the wider `WNarrow` work integer.
    if !narrow_fixed_fits(raw, SCALE, w) || !crate::support::rounding::is_nearest_mode(mode) {
        return exp_wide_narrow_raw(raw, SCALE, STRICT_GUARD, mode);
    }
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(STRICT_GUARD));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    exp_fixed(v_w, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("exp kernel", SCALE)
        })
}

// ── exp2 kernel (D38, Fixed fallback) ─────────────────────────────

/// Exact-power pin for the D38 `exp2`. When `raw` is an exact integer
/// `k`, `exp2(k) = 2^k` is an exact algebraic point — a *dyadic
/// rational*, never a transcendental residual. Returns the
/// **correctly-rounded** storage value of `2^k` under `mode`, computed
/// from exact integer arithmetic, so the `exp(k·ln 2)` series round-off
/// can never bump it across a tie or grid line. Returns `None` only when
/// `raw` is not an exact integer (the genuinely transcendental case the
/// series kernel handles) or when a representable result overflows
/// `i128`. See the wide-tier `exp2_exact_pow`.
#[inline]
fn exp2_exact_pin(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    let one_s = 10i128.checked_pow(scale)?;
    if raw % one_s != 0 {
        return None;
    }
    let k = raw / one_s;
    if k == 0 {
        return Some(one_s);
    }
    let kk = k.unsigned_abs();
    if k > 0 {
        // 2^k · 10^scale — exact integer when representable.
        let mut v: i128 = one_s;
        for _ in 0..kk {
            v = v.checked_mul(2)?;
        }
        Some(v)
    } else if kk <= scale as u128 {
        // 2^-|k| = 5^|k| · 10^(scale − |k|) — exact, no rounding.
        let mut v = 10i128.checked_pow(scale - kk as u32)?;
        for _ in 0..kk {
            v = v.checked_mul(5)?;
        }
        Some(v)
    } else {
        // |k| > scale: `2^k · 10^scale = 5^scale / 2^(|k|−scale)` is a
        // proper dyadic fraction in `(0, 1)` storage units. Round it
        // exactly under `mode` (`exp2(-1) = 0.5` is the half-to-even tie
        // → 0; `exp2(-146)` is a sub-resolution positive → Ceiling → 1).
        let num = 5u128.checked_pow(scale)?; // 5^38 < 2^89, fits u128
        let p = kk as u32 - scale; // shift amount, ≥ 1
        Some(round_pow2_fraction(num, p, mode))
    }
}

/// Correctly-rounded storage value of the dyadic fraction `num / 2^p`
/// (`num > 0`, `p ≥ 1`) — a strictly-positive result in `[0, num/2]`.
///
/// `q = num >> p`, remainder `r = num & (2^p − 1)`; the half-way divisor
/// is `2^p`, so the tie compares `2·r` against `2^p`. When `p ≥ 128`
/// the quotient is `0` and the whole of `num` is the (sub-half) residual
/// — a tiny positive value that only `Ceiling` rounds up.
#[inline]
fn round_pow2_fraction(num: u128, p: u32, mode: RoundingMode) -> i128 {
    if p >= 128 {
        // num < 2^128 ≤ 2^p, so q = 0 and r = num > 0 but < 2^(p-1)
        // (half), i.e. a sub-resolution positive residual.
        let bump = crate::support::rounding::should_bump(
            mode,
            ::core::cmp::Ordering::Less, // r strictly below half
            false,                       // q == 0 is even
            true,                        // result positive
        );
        return i128::from(bump);
    }
    let q = (num >> p) as i128;
    let r = num & ((1u128 << p) - 1);
    if r == 0 {
        return q;
    }
    let half = 1u128 << (p - 1);
    let cmp_r = r.cmp(&half);
    let q_is_odd = (q & 1) == 1;
    let bump = crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, true);
    q + i128::from(bump)
}

/// `2^x = exp(x · ln 2)` on the `Fixed` intermediate. Used by
/// `policy::exp::exp2_dispatch` when the D57 borrow path is not available.
#[inline]
#[must_use]
pub(crate) fn exp2_with(raw: Int<2>, scale: u32, working_digits: u32, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(exp2_with_raw(raw.as_i128(), scale, working_digits, mode))
}

/// `i128` core of [`exp2_with`].
#[inline]
fn exp2_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(scale);
    }
    // Exact-power pin: `exp2(integer k) = 2^k` is an exact algebraic
    // point (integer for `k >= 0`, `5^|k|·10^(scale−|k|)` for `k < 0`).
    // Emitting it directly stops the `exp(k·ln 2)` round-off from
    // bumping a directed mode by one LSB at the exact power.
    if let Some(pinned) = exp2_exact_pin(raw, scale, mode) {
        return pinned;
    }
    let w = scale + working_digits;
    let negative_input = raw < 0;
    let v_w = Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    let arg_w = v_w.mul(wide_ln2(w), w);
    exp_fixed(arg_w, w)
        .round_to_i128_with(w, scale, mode)
        .unwrap_or_else(|| {
            crate::support::diagnostics::overflow_panic_with_scale("D38::exp2", scale)
        })
}

#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    exp2_with(raw, SCALE, STRICT_GUARD, mode)
}
