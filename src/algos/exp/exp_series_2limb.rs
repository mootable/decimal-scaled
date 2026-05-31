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
    // `int_digits(e^x) = ceil(x · log10 e) + 1`, `x = raw / 10^scale`,
    // `log10 e ≈ 434295 / 1_000_000`. We need
    // `ceil(raw · 434295 / (10^scale · 1_000_000)) + 1`.
    //
    // Forming `raw · 434295` and `10^scale · 1_000_000` directly both
    // overflow `u128` for in-range cells (`raw ≈ 1.7e38`, and any
    // `scale ≥ 33` makes `10^scale·10^6 > u128::MAX`). The old code
    // returned `u32::MAX` ("does not fit") whenever the DENOMINATOR
    // overflowed — but that is wrong: a SMALL argument at a HIGH scale
    // (e.g. `exp(0.1)` at scale 37, `raw = 10^36`) has a tiny result that
    // fits the fast path, yet `10^37·10^6` overflows and the gate forced
    // the cell onto the expensive wide `WNarrow` path. That mis-routing is
    // exactly the D38/cosh/sinh high-scale exp regression.
    //
    // The integer-digit count of `e^x` is governed by the MAGNITUDE of
    // `x = raw / 10^scale`, which never overflows even when `raw·434295`
    // and `10^scale·10^6` individually do. We want, exactly as before,
    //   `ceil(raw · 434295 / (10^scale · 10^6)) + 1`.
    // Compute it overflow-free by first dividing `raw` by `10^scale`
    // (split into integer part `q` and remainder `r`), then forming the
    // `·434295/10^6` product on the BOUNDED pieces:
    //
    //   raw·434295 / 10^scale = q·434295 + (r·434295)/10^scale
    //
    // where `q = ⌊x⌋`. The result int-digit count is past the 22-digit
    // fast band once `q ≳ 50`, so capping `q` at 60 keeps `q·434295`
    // inside u128 without ever mis-classifying an in-band cell — and the
    // remainder term `r·434295 < 10^scale·434295` is divided back down by
    // `10^scale`, never overflowing because `r < 10^scale ≤ 10^38` and
    // `434295 < 10^6` give `r·434295 < 10^44`… which DOES overflow for
    // large scale, so divide `r` toward the reduced scale first: drop the
    // low digits of `r` that cannot affect the `/10^6` ceil. Keeping the
    // top 12 significant digits of the fraction (`10^6` precision ×6 guard)
    // is exact for the comparison; do it by reducing `r`/`10^scale` to
    // `r6 = r·10^7 / 10^scale` (the fraction ×10^7, ≤ 10^7), all in u128.
    let one_s = match 10u128.checked_pow(scale) {
        Some(p) => p,
        // `scale > 38` cannot occur for an `i128`-storage tier; an
        // enormous scale means `x < 1`, single integer digit.
        None => return 1,
    };
    let raw_u = raw as u128;
    let q = raw_u / one_s; // integer part of x = ⌊raw / 10^scale⌋
    let r = raw_u % one_s; // fractional remainder, r < 10^scale
    // Past q = 50 the count certainly exceeds the 22-digit band; cap at 60
    // so q·434295 stays in u128 and never under-states an in-band cell.
    let q_capped = q.min(60);
    // Fraction of x scaled by 10^7 (one guard digit beyond the 10^6 in
    // log10 e): r/10^scale ∈ [0,1) ⇒ r7 = ⌊r·10^7 / 10^scale⌋ ∈ [0, 10^7).
    // Form it overflow-free: if scale ≤ 7, r·10^(7−scale); else r / 10^(scale−7).
    let r7 = if scale <= 7 {
        r * 10u128.pow(7 - scale)
    } else {
        r / 10u128.pow(scale - 7)
    };
    // x·10^7 ≈ q·10^7 + r7, then ·434295 / 10^6, ceil, +1.
    // numerator = (q·10^7 + r7)·434295, all bounded (q ≤ 60, r7 < 10^7).
    let x_e7 = q_capped * 10_000_000 + r7; // x · 10^7 (q capped)
    let num = x_e7 * 434_295; // / 10^7 / 10^6 = / 10^13 gives x·log10 e
    (num.div_ceil(10u128.pow(13)).min(u32::MAX as u128 - 1) as u32) + 1
}

/// Largest `e^x` integer-digit count the fast 256-bit `Fixed` path rounds
/// correctly. Empirically (the `validity_probe`) the fast path first
/// diverges from the wide reference at `≥ 25` result integer digits (the
/// guard digits left above the `2^k`-reassembled integer part erode to too
/// few). `22` keeps a 3-digit margin below that wall, so every cell at or
/// below it is bit-identical to the wide path; above it the integer-regime
/// cell takes the wider [`WNarrow`] work integer.
const FAST_MAX_RESULT_DIGITS: u32 = 22;

/// Whether the 256-bit `Fixed` fast path computes `e^x` correctly for the
/// storage value `raw` at `scale` — i.e. the result is NOT in the
/// integer-regime where its many integer digits leave the `Fixed` too few
/// guard digits to round correctly. Keyed on the result's integer-digit
/// count against [`FAST_MAX_RESULT_DIGITS`]; `w` is unused (kept for the
/// existing callers' signature) — the squaring/`2^k`-reassembly peak is
/// computed in the full 512-bit product inside `Fixed::mul`, so it never
/// overflows; only the rounded result's guard-digit budget bounds the fast
/// path, and that is purely a function of the result magnitude.
#[inline]
fn narrow_fixed_fits(raw: i128, scale: u32, w: u32) -> bool {
    let _ = w;
    exp_result_int_digits(raw, scale) <= FAST_MAX_RESULT_DIGITS
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
    let v_mag = WNarrow::from_i128(raw.unsigned_abs() as i128) * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let v_w = if negative_input { -v_mag } else { v_mag };

    let ex = exp_generic::exp_fixed::<WNarrow>(v_w, w);
    narrow_round_mag(ex, working_digits, mode, true, false).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("exp kernel", scale)
    })
}

/// Narrows a non-negative [`WNarrow`] working-scale magnitude `mag`
/// (`= value · 10^w`, `value > 0` and irrational at a non-trivial
/// argument) to a signed `i128` storage value at scale `w − shift` under
/// `mode`. `never_exact` mirrors the wide directed path: a zero working
/// residual is treated as a present positive sub-resolution fraction
/// (bumps Ceiling, not Floor/Trunc). `result_neg` reapplies an odd
/// function's sign AFTER rounding the magnitude.
///
/// Returns `None` when the rounded storage value does not fit the `i128`
/// the narrow tier stores its result in — a genuine RESULT-TYPE overflow
/// (e.g. `exp(100)` at D38<35> is ~2.7e43·10^35, far beyond `i128::MAX`),
/// DISTINCT from the working-width (`exp_fixed` internal `2·w_ext`)
/// overflow the wider `WNarrow` work integer fixes. The caller turns the
/// `None` into the same overflow panic / saturation the `Fixed` path's
/// `round_to_i128_with` did, so `exp_strict` still panics on an
/// unrepresentable result rather than silently returning a wrapped value.
#[inline]
fn narrow_round_mag(
    mag: WNarrow,
    shift: u32,
    mode: RoundingMode,
    never_exact: bool,
    result_neg: bool,
) -> Option<i128> {
    use crate::support::rounding::{is_nearest_mode, should_bump};
    let divisor = crate::consts::pow10::dispatch::<WNarrow>(shift);
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
    // Result-type fit check (mirrors `Fixed::round_to_i128_with`): the
    // non-negative quotient magnitude `q_mag` must fit the signed `i128`.
    // A positive result fits iff `q_mag <= i128::MAX`; a negative result
    // iff `q_mag <= 2^127` (= `|i128::MIN|`). Both are bounded by
    // `bit_length <= 127`, with the single extra `2^127` value allowed
    // only for the negative side.
    let bl = q_mag.bit_length();
    if bl > 128 {
        return None;
    }
    if bl == 128 {
        // The only 128-bit magnitude that fits is exactly `2^127`, and
        // only as a negative result (`i128::MIN`).
        let two_pow_127 = WNarrow::ONE << 127;
        if !(result_neg && q_mag == two_pow_127) {
            return None;
        }
    } else if bl == 127 && !result_neg {
        // `2^126 <= q_mag < 2^127`: a positive result fits iff
        // `q_mag <= i128::MAX = 2^127 - 1`. bit_length 127 already
        // guarantees `q_mag < 2^127`, so it fits.
    }
    let signed = if result_neg { -q_mag } else { q_mag };
    Some(signed.to_i128())
}

/// `sinh(x)` / `cosh(x)` magnitude `(e^|x| ∓ e^-|x|)/2` at working scale
/// `w`, computed in the wide [`WNarrow`] work integer. Returns the
/// non-negative `sinh(|x|)` / `cosh(|x|)`; the odd-function sign is
/// reapplied by the caller via [`narrow_round_mag`].
#[inline]
fn hyper_pos_wide_narrow(av_w: WNarrow, w: u32, is_cosh: bool) -> WNarrow {
    use crate::algos::exp::exp_generic;
    let ex = exp_generic::exp_fixed::<WNarrow>(av_w, w);
    let one_w = crate::consts::pow10::dispatch::<WNarrow>(w);
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
    let av = WNarrow::from_i128(raw.unsigned_abs() as i128) * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let sh = hyper_pos_wide_narrow(av, w, false);
    narrow_round_mag(sh, working_digits, mode, true, neg).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::sinh", scale)
    })
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
    let av = WNarrow::from_i128(raw.unsigned_abs() as i128) * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let ch = hyper_pos_wide_narrow(av, w, true);
    narrow_round_mag(ch, working_digits, mode, true, false).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::cosh", scale)
    })
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
    // The wider `WNarrow` work integer is needed for the cells the fast
    // 256-bit `Fixed` path cannot round correctly:
    //  1. integer-regime — `e^x` carries so many integer digits the `Fixed`
    //     keeps too few guard digits (`!narrow_fixed_fits`); and
    //  2. ALL directed modes — the fast path's flat-`w` rounding lacks the
    //     never-exact treatment a directed mode needs for the sub-resolution
    //     transcendental residual (a near-1 `e^(tiny)` or a sub-resolution
    //     `e^(negative)` must round up under Ceiling, which the fast path
    //     cannot resolve). Directed exp is not the common/benched cell, so
    //     keeping it on the wide path costs nothing on the hot path.
    // Every other (NEAREST-mode, non-integer-regime) cell — the COMMON
    // narrow exp the regression was about — stays on the fast path.
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
    // See [`exp_with_raw`]: the integer-regime cells and ALL directed modes
    // route through the wider `WNarrow` work integer; every other
    // NEAREST-mode common cell stays on the fast `Fixed` path.
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

// ── Fast-path validity wall ────────────────────────────────────────
// The narrow exp gate (`exp_with_raw` / `exp_strict_raw`) routes a cell
// to the fast 256-bit `Fixed` path only where it is bit-identical to the
// trusted wider-`WNarrow` reference (the path the 8 mpmath golden cells
// validate). This test ASSERTS that validity wall across the full D38
// scale × |x| × mode space: for every cell the production gate keeps on
// the fast path, fast == wide. It is the consistency-wall guard that lets
// the gate stay tight (recover the common-cell speed) without a
// correctness regression — the same "bit-identical to the reference"
// pattern the `exp_series_tang_ab` Tang validity wall uses.
#[cfg(test)]
mod fast_path_validity {
    use super::*;

    /// FAST path with NO gate (pure `Fixed`), catching the overflow panic.
    fn fast_exp_raw_ungated(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
        if raw == 0 {
            return Some(10_i128.pow(scale));
        }
        let w = scale + STRICT_GUARD;
        let negative_input = raw < 0;
        let v_w =
            Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(STRICT_GUARD));
        let v_w = if negative_input { v_w.neg() } else { v_w };
        std::panic::catch_unwind(|| exp_fixed(v_w, w).round_to_i128_with(w, scale, mode))
            .unwrap_or(None)
    }

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
    ];

    /// Mirror the production gate exactly: `true` ⇒ this cell stays on the
    /// fast path (so fast MUST equal wide). Directed modes always route wide.
    fn gate_keeps_fast(raw: i128, scale: u32, mode: RoundingMode) -> bool {
        let w = scale + STRICT_GUARD;
        narrow_fixed_fits(raw, scale, w) && crate::support::rounding::is_nearest_mode(mode)
    }

    // For EVERY D38 cell the production gate routes to the fast path,
    // assert it is bit-identical to the wide reference. A fine 0.1-step
    // |x| grid over the whole representable range, both signs, all six
    // modes, scale 0..=38 — so an unbenched scale cannot silently get a
    // wrong fast result.
    #[test]
    fn fast_path_bit_identical_to_wide_d38() {
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
                    if raw == 0 {
                        continue;
                    }
                    for mode in MODES {
                        if !gate_keeps_fast(raw, scale, mode) {
                            continue; // routed to wide — not a fast-path claim
                        }
                        let wide = match std::panic::catch_unwind(|| {
                            exp_wide_narrow_raw(raw, scale, STRICT_GUARD, mode)
                        }) {
                            Ok(v) => v,
                            // Wide reference itself overflows i128 — the
                            // narrow tier cannot represent the result; both
                            // paths panic, not a fast-vs-wide question.
                            Err(_) => continue,
                        };
                        let fast = fast_exp_raw_ungated(raw, scale, mode);
                        assert_eq!(
                            fast,
                            Some(wide),
                            "fast != wide at scale={scale} raw={raw} mode={mode:?} (gate kept fast)"
                        );
                        checked += 1;
                    }
                }
                x10 += 1;
            }
        }
        assert!(checked > 100_000, "too few cells checked: {checked}");
    }

    // The genuine wide-only cells (integer-regime + every directed mode)
    // must actually be routed AWAY from the fast path — the gate's other
    // half. Spot-check the 8-golden-cell shapes plus a directed cell.
    #[test]
    fn wide_only_cells_are_routed_wide() {
        use RoundingMode::*;
        // integer-regime: routed wide for ALL modes (incl. nearest)
        for &raw in &[66i128, 85, 100] {
            assert!(
                !gate_keeps_fast(raw, 0, HalfToEven),
                "exp({raw}) s0 should route WIDE (integer regime)"
            );
        }
        // ALL directed modes route wide (the fast path lacks the never-exact
        // directed rounding the transcendental residual needs) — including a
        // deep-fractional near-1 result like exp(-1e-37) under Ceiling, the
        // golden d38 exp s37 cell.
        for mode in [Ceiling, Floor, Trunc] {
            assert!(
                !gate_keeps_fast(-1, 37, mode),
                "exp(-1e-37) s37 {mode:?} should route WIDE (directed)"
            );
            assert!(
                !gate_keeps_fast(2 * 10i128.pow(0), 0, mode),
                "exp(2) s0 {mode:?} should route WIDE (directed)"
            );
        }
        // ...but a normal nearest-mode common cell stays FAST.
        assert!(
            gate_keeps_fast(15 * 10i128.pow(18), 19, HalfToEven),
            "exp(1.5) D38 s19 HalfToEven should stay FAST (common cell)"
        );
        assert!(
            gate_keeps_fast(-1, 37, HalfToEven),
            "exp(-1e-37) s37 HalfToEven should stay FAST"
        );
    }

    // Focused timing A/B at the gate seam: the FAST `Fixed` path (what the
    // tightened gate now picks for the common nearest-mode cells) vs the
    // WIDE `WNarrow=Int<24>` path (what the over-broad 9.3 gate wrongly
    // picked for them). Run with `--ignored --nocapture` for the numbers;
    // ignored by default so it never slows the normal `--lib` run.
    #[test]
    #[ignore = "timing A/B — run with --ignored --nocapture"]
    fn timing_fast_vs_wide_common_cells() {
        use std::time::Instant;
        // (raw, scale, label) — all common nearest-mode cells the gate now
        // keeps fast; result stays in-range for both paths.
        let cells: &[(i128, u32, &str)] = &[
            (15 * 10i128.pow(18), 19, "exp(1.5) D38 s19"),
            (27 * 10i128.pow(18) / 10, 19, "exp(2.7) D38 s19"),
            (5 * 10i128.pow(30) / 10, 30, "exp(0.5) D38 s30"),
            (5 * 10i128.pow(37) / 10, 37, "exp(0.5) D38 s37"),
            (9 * 10i128.pow(9), 9, "exp(9) D18 s9"),
            (15 * 10i128.pow(17), 17, "exp(1.5) D18 s17"),
            // The high-scale small-|x| cells the corrected digit-gate now
            // keeps FAST (previously forced WIDE by the denominator-overflow
            // bug) — the bench-branch-compare `exp_D38_s37`/`cosh`/`sinh`
            // regression operands (`exp(0.1)`). The speedup column here is the
            // measured fast-vs-wide gain these cells recover.
            (10i128.pow(36), 37, "exp(0.1) D38 s37 *bbc*"),
            (10i128.pow(37), 38, "exp(0.1) D38 s38"),
            (10i128.pow(35), 36, "exp(0.1) D38 s36"),
            (15 * 10i128.pow(36) / 10, 37, "exp(1.5) D38 s37"),
        ];
        let mode = RoundingMode::HalfToEven;
        const ITERS: u32 = 200_000;
        for &(raw, scale, label) in cells {
            // warm + skip any cell whose result is out of i128 range
            if fast_exp_raw_ungated(raw, scale, mode).is_none() {
                continue;
            }
            let _ = exp_wide_narrow_raw(raw, scale, STRICT_GUARD, mode);
            let t0 = Instant::now();
            let mut acc = 0i128;
            for _ in 0..ITERS {
                acc = acc.wrapping_add(
                    fast_exp_raw_ungated(std::hint::black_box(raw), scale, mode).unwrap_or(0),
                );
            }
            let fast_ns = t0.elapsed().as_nanos() as f64 / ITERS as f64;
            let t1 = Instant::now();
            for _ in 0..ITERS {
                acc = acc.wrapping_add(exp_wide_narrow_raw(
                    std::hint::black_box(raw),
                    scale,
                    STRICT_GUARD,
                    mode,
                ));
            }
            let wide_ns = t1.elapsed().as_nanos() as f64 / ITERS as f64;
            std::hint::black_box(acc);
            println!(
                "{label:22}: fast={fast_ns:7.1}ns  wide={wide_ns:8.1}ns  speedup={:.2}x",
                wide_ns / fast_ns
            );
        }
    }
}
