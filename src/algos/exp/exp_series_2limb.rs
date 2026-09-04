// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

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

use crate::algos::pow::powi_exact::ExactPin;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::narrow_ziv::{self, WZiv};
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
    // `scale ≥ 33` makes `10^scale·10^6 > u128::MAX`). Returning
    // `u32::MAX` ("does not fit") whenever the DENOMINATOR
    // overflows is wrong: a SMALL argument at a HIGH scale
    // (e.g. `exp(0.1)` at scale 37, `raw = 10^36`) has a tiny result that
    // fits the fast path, yet `10^37·10^6` overflows and would force
    // the cell onto the expensive wide `WNarrow` path — mis-routing the
    // D38/cosh/sinh high-scale exp cells.
    //
    // The integer-digit count of `e^x` is governed by the MAGNITUDE of
    // `x = raw / 10^scale`, which never overflows even when `raw·434295`
    // and `10^scale·10^6` individually do. We want, exactly as before,
    //   `ceil(raw · 434295 / (10^scale · 10^6)) + 1`.
    // Compute it overflow-free by first dividing `raw` by `10^scale`
    // (split into `integer_part` and `fraction_remainder`), then forming
    // the `·434295/10^6` product on the BOUNDED pieces:
    //
    //   raw·434295 / 10^scale = q·434295 + (r·434295)/10^scale
    //
    // where `q = ⌊x⌋`. The result int-digit count is past the 22-digit
    // fast band once the integer part is ≳ 50, so capping it at 60 keeps
    // `q·434295` inside u128 without ever mis-classifying an in-band cell —
    // and the remainder term `r·434295 < 10^scale·434295` is divided back
    // down by `10^scale`, never overflowing because `r < 10^scale ≤ 10^38`
    // and `434295 < 10^6` give `r·434295 < 10^44`… which DOES overflow for
    // large scale, so divide the remainder toward the reduced scale first:
    // drop the low digits that cannot affect the `/10^6` ceil. Keeping the
    // top 12 significant digits of the fraction (`10^6` precision ×6 guard)
    // is exact for the comparison; do it by reducing `r`/`10^scale` to
    // `fraction_e7 = r·10^7 / 10^scale` (the fraction ×10^7, ≤ 10^7), all
    // in u128.
    let one_scaled = match 10u128.checked_pow(scale) {
        Some(power) => power,
        // `scale > 38` cannot occur for an `i128`-storage tier; an
        // enormous scale means `x < 1`, single integer digit.
        None => return 1,
    };
    let raw_u128 = raw as u128;
    let integer_part = raw_u128 / one_scaled; // ⌊raw / 10^scale⌋
    let fraction_remainder = raw_u128 % one_scaled; // < 10^scale
    // Past 50 the count certainly exceeds the 22-digit band; cap at 60
    // so q·434295 stays in u128 and never under-states an in-band cell.
    let integer_part_capped = integer_part.min(60);
    // Fraction of x scaled by 10^7 (one guard digit beyond the 10^6 in
    // log10 e): r/10^scale ∈ [0,1) ⇒ fraction_e7 = ⌊r·10^7 / 10^scale⌋
    // ∈ [0, 10^7). Form it overflow-free: if scale ≤ 7, r·10^(7−scale);
    // else r / 10^(scale−7).
    let fraction_e7 = if scale <= 7 {
        fraction_remainder * 10u128.pow(7 - scale)
    } else {
        fraction_remainder / 10u128.pow(scale - 7)
    };
    // x·10^7 ≈ q·10^7 + fraction_e7, then ·434295 / 10^6, ceil, +1.
    // numerator = (q·10^7 + fraction_e7)·434295, all bounded.
    let x_e7 = integer_part_capped * 10_000_000 + fraction_e7; // x · 10^7 (capped)
    let numerator = x_e7 * 434_295; // / 10^7 / 10^6 = / 10^13 gives x·log10 e
    (numerator.div_ceil(10u128.pow(13)).min(u32::MAX as u128 - 1) as u32) + 1
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
/// count against [`FAST_MAX_RESULT_DIGITS`]; `working_scale` is unused (kept
/// for the existing callers' signature) — the squaring/`2^k`-reassembly peak
/// is computed in the full 512-bit product inside `Fixed::mul`, so it never
/// overflows; only the rounded result's guard-digit budget bounds the fast
/// path, and that is purely a function of the result magnitude.
#[inline]
fn narrow_fixed_fits(raw: i128, scale: u32, working_scale: u32) -> bool {
    let _ = working_scale;
    exp_result_int_digits(raw, scale) <= FAST_MAX_RESULT_DIGITS
}

/// `e` raised to a `working_value`, returned at the same `working_scale`.
///
/// Range-reduces `v = k·ln(2) + s` with `|s| ≤ ln(2)/2`, halves the reduced
/// argument `halvings` further times (`halved_arg = s / 2^n`), evaluates the
/// Taylor series for `exp(halved_arg)` on the much smaller argument, then
/// squares the result `halvings` times to recover
/// `exp(s) = (exp(halved_arg))^(2^n)` — classic Brent–Salamin "argument
/// reduction + squaring" trick. `halvings` is tuned so the Taylor cost (one
/// mul + one div_small per term) trades evenly against the post-squarings
/// (one wide mul each).
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
pub(crate) fn exp_fixed(working_value: Fixed, working_scale: u32) -> Fixed {
    let one_at_working_scale = Fixed {
        negative: false,
        mag: Fixed::pow10(working_scale),
    };
    // Deep-underflow pre-gate — BEFORE the `k` range-reduction divide. For
    // a deep negative argument that divide (and the `k·ln 2` product after
    // it) overflows the 256-bit intermediates: `|v| ≥ (w+1)·ln 10` makes
    // `e^v < 10^-(w+1)`, strictly below the working resolution, while
    // `|k| ≈ |v|/ln 2` is far past what `mul_u128` / the `2^k` shift can
    // carry — pre-fix, `exp(-1.5e38)` died in an internal `div_u512_by_
    // pow10` invariant instead of returning its in-range 0. The threshold
    // `(w+1)·2.302586` over-approximates `(w+1)·ln 10` (so the gate only
    // fires on provable sub-resolution values), built as `(w+1)·2_302_586 ·
    // 10^(w−6)` — within `U256` for every working scale this kernel serves
    // (`w ≤ 68`). The returned ZERO is exactly what the ungated body
    // produces for any deep negative it can carry (`sum >> |k|` underflows
    // to zero), so every caller — exp's nearest-mode fast path (directed
    // modes route to the wider work integer before this kernel) and powf's
    // composition — sees the value it always did.
    if working_value.negative && working_scale >= 6 {
        let threshold = Fixed {
            negative: false,
            mag: Fixed::pow10(working_scale - 6),
        }
        .mul_u128(((working_scale as u128) + 1) * 2_302_586);
        if working_value.ge_mag(threshold) {
            return Fixed::ZERO;
        }
    }
    let ln2 = wide_ln2(working_scale);

    // k = round(v / ln 2); s = v - k·ln(2), |s| <= ln(2)/2.
    let k = working_value.div(ln2, working_scale).round_to_nearest_int(working_scale);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    let reduced_arg = working_value.sub(k_ln2);

    // Argument halvings: pick `n` such that `(n+1)² ≤ 3w+1` — the
    // standard tuning where one extra halving saves roughly two
    // Taylor iterations but costs one final squaring. For w ≤ 44
    // this lands at n ∈ {4, 5, 6}.
    let level_bound = working_scale.saturating_mul(3).saturating_add(1);
    let mut halvings: u32 = 1;
    while (halvings + 1) * (halvings + 1) <= level_bound {
        halvings += 1;
    }
    let halved_arg = reduced_arg.shr(halvings);

    // Taylor series exp(s_red) = 1 + s_red + s_red²/2! + … on the
    // halved argument — `term` carries s_redⁱ/i!.
    let mut sum = one_at_working_scale.add(halved_arg);
    let mut term = halved_arg;
    let mut term_index: u128 = 2;
    loop {
        term = term.mul(halved_arg, working_scale).div_small(term_index);
        if term.is_zero() {
            break;
        }
        sum = sum.add(term);
        term_index += 1;
        if term_index > 400 {
            break;
        }
    }

    // Undo the halvings: exp(s) = (exp(halved_arg))^(2^n) — `halvings`
    // repeated squarings.
    for _ in 0..halvings {
        sum = sum.mul(sum, working_scale);
    }

    // exp(v) = 2^k · exp(s).
    if k >= 0 {
        // Saturating narrowing: a `k` past `u32` (not formable by the gated
        // callers, but cheap to make total) must FAIL the range assert, not
        // wrap into a small shift that silently passes it.
        let shift = u32::try_from(k).unwrap_or(u32::MAX);
        assert!(
            (sum.bit_length() as u64) + (shift as u64) <= 256,
            "D38::exp: result out of range"
        );
        sum.shl(shift)
    } else {
        // `shr` is total for any shift ≥ 256 (the magnitude underflows to
        // zero), so clamp rather than truncate: a wrapped `(-k) as u32`
        // could land on a SMALL shift and return a wrongly large value.
        sum.shr(k.unsigned_abs().min(256) as u32)
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
fn exp_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<i128> {
    use crate::algos::exp::exp_generic;

    let working_scale = scale + working_digits;
    let negative_input = raw < 0;
    let abs_working_value = WNarrow::from_i128(raw.unsigned_abs() as i128)
        * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let working_value = if negative_input { -abs_working_value } else { abs_working_value };

    // `try_exp_fixed`: a deep argument the generic kernel proves out of
    // range is this kernel's `None` (the policy dispatch wrapper applies
    // the default form's contractual panic; the `checked_` surface
    // propagates it), same as the post-narrowing fit check below.
    let exp_x = exp_generic::try_exp_fixed::<WNarrow>(working_value, working_scale)?;
    narrow_round_mag(exp_x, working_digits, mode, true, false)
}

/// Whether the sub-storage residual of the non-negative working
/// magnitude `mag` (at `shift` digits above storage) is clear of the
/// mode's deciding boundary by more than the near-tie band
/// (`divisor/1000`) — the [`WNarrow`] sibling of
/// [`Fixed::round_to_i128_clear_of_tie`]'s band check. `false` = the
/// strict caller must escalate through the Ziv walker.
#[inline]
fn wnarrow_residual_clear(mag: WNarrow, shift: u32, mode: RoundingMode) -> bool {
    let divisor = crate::consts::pow10::dispatch::<WNarrow>(shift);
    let (_quotient, remainder) = mag.div_rem(divisor);
    let band = if shift >= 3 {
        crate::consts::pow10::dispatch::<WNarrow>(shift - 3)
    } else {
        WNarrow::ZERO
    };
    let distance = if crate::support::rounding::is_nearest_mode(mode) {
        let half = divisor >> 1;
        if remainder < half { half - remainder } else { remainder - half }
    } else {
        let complement = divisor - remainder;
        if remainder < complement { remainder } else { complement }
    };
    distance > band
}

/// One `WZiv` exp probe at working scale `scale + guard_digits` (`WZiv` and
/// [`WNarrow`] are the same `Int<24>`).
fn exp_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    crate::algos::exp::exp_generic::exp_fixed::<WZiv>(
        narrow_ziv::lift(raw, guard_digits), scale + guard_digits)
}

/// Strict-path integer-regime / directed `e^x` — the wide-[`WNarrow`]
/// single shot with the near-tie protected terminal: a clear residual
/// keeps the cheap single-shot cost; a near-tie escalates through the
/// never-exact Ziv walker (`e^x` is transcendental for every `x ≠ 0`).
fn exp_wide_narrow_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    use crate::algos::exp::exp_generic;

    let working_scale = scale + STRICT_GUARD;
    let negative_input = raw < 0;
    let abs_working_value = WNarrow::from_i128(raw.unsigned_abs() as i128)
        * crate::consts::pow10::dispatch::<WNarrow>(STRICT_GUARD);
    let working_value = if negative_input { -abs_working_value } else { abs_working_value };
    let exp_x = exp_generic::try_exp_fixed::<WNarrow>(working_value, working_scale)?;
    let single_shot = narrow_round_mag(exp_x, STRICT_GUARD, mode, true, false);
    if wnarrow_residual_clear(exp_x, STRICT_GUARD, mode) {
        return single_shot;
    }
    narrow_ziv::walk_checked_never_exact(single_shot, STRICT_GUARD, scale, mode, |guard_digits| {
        exp_ziv(raw, scale, guard_digits)
    })
}

/// Narrows a non-negative [`WNarrow`] working-scale magnitude `mag`
/// (`= value · 10^w`, `value > 0` and irrational at a non-trivial
/// argument) to a signed `i128` storage value at scale `w − shift` under
/// `mode`. `never_exact` mirrors the wide directed path: a zero working
/// residual is treated as a present positive sub-resolution fraction
/// (bumps Ceiling, not Floor/Trunc). `result_is_negative` reapplies an odd
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
    result_is_negative: bool,
) -> Option<i128> {
    use crate::support::rounding::{is_nearest_mode, should_bump};
    let divisor = crate::consts::pow10::dispatch::<WNarrow>(shift);
    let (quotient, remainder) = mag.div_rem(divisor);
    let result_is_positive = !result_is_negative;
    // Last decimal digit of the quotient magnitude. This is a wide `%`
    // (O(limbs)) where the tie break previously read one bit.
    let q_mod_10 = (quotient % WNarrow::from_i128(10)).as_i128().unsigned_abs() as u8;
    let bump = if remainder != WNarrow::ZERO {
        if is_nearest_mode(mode) {
            let complement = divisor - remainder;
            let remainder_cmp = remainder.cmp(&complement);
            should_bump(mode, remainder_cmp, q_mod_10, result_is_positive)
        } else {
            match mode {
                RoundingMode::Ceiling => result_is_positive,
                RoundingMode::Floor => !result_is_positive,
                RoundingMode::AwayFromZero => true,
                RoundingMode::ZeroFiveUp => matches!(q_mod_10, 0 | 5),
                _ => false, // Trunc
            }
        }
    } else if never_exact {
        // Present-and-positive sub-resolution residual.
        match mode {
            RoundingMode::Ceiling => result_is_positive,
            RoundingMode::Floor => !result_is_positive,
            RoundingMode::AwayFromZero => true,
            RoundingMode::ZeroFiveUp => matches!(q_mod_10, 0 | 5),
            _ => false,
        }
    } else {
        false
    };
    let magnitude = if bump { quotient + WNarrow::ONE } else { quotient };
    // Result-type fit check (mirrors `Fixed::round_to_i128_with`): the
    // non-negative quotient `magnitude` must fit the signed `i128`.
    // A positive result fits iff `magnitude <= i128::MAX`; a negative result
    // iff `magnitude <= 2^127` (= `|i128::MIN|`). Both are bounded by
    // `bit_length <= 127`, with the single extra `2^127` value allowed
    // only for the negative side.
    let magnitude_bits = magnitude.bit_length();
    if magnitude_bits > 128 {
        return None;
    }
    if magnitude_bits == 128 {
        // The only 128-bit magnitude that fits is exactly `2^127`, and
        // only as a negative result (`i128::MIN`).
        let two_pow_127 = WNarrow::ONE << 127;
        if !(result_is_negative && magnitude == two_pow_127) {
            return None;
        }
    } else if magnitude_bits == 127 && !result_is_negative {
        // `2^126 <= magnitude < 2^127`: a positive result fits iff
        // `magnitude <= i128::MAX = 2^127 - 1`. bit_length 127 already
        // guarantees `magnitude < 2^127`, so it fits.
    }
    let signed = if result_is_negative { -magnitude } else { magnitude };
    Some(signed.to_i128())
}

/// `sinh(x)` / `cosh(x)` magnitude `(e^|x| ∓ e^-|x|)/2` at
/// `working_scale`, computed in the wide [`WNarrow`] work integer. Returns
/// the non-negative `sinh(|x|)` / `cosh(|x|)`; the odd-function sign is
/// reapplied by the caller via [`narrow_round_mag`].
#[inline]
fn hyper_pos_wide_narrow(
    abs_working_value: WNarrow, working_scale: u32, is_cosh: bool) -> WNarrow {
    use crate::algos::exp::exp_generic;
    let exp_x = exp_generic::exp_fixed::<WNarrow>(abs_working_value, working_scale);
    let one_at_working_scale = crate::consts::pow10::dispatch::<WNarrow>(working_scale);
    // `exp_x = e^|x|·10^w`. The reciprocal at the same scale is
    // `e^-|x|·10^w = 10^(2w) / exp_x`. For the integer-regime |x| this is a
    // tiny positive value (≪ 1 ULP-of-storage), formed in the wide integer
    // to avoid the `Fixed` overflow.
    let (exp_neg_x, _remainder) =
        (one_at_working_scale * one_at_working_scale).div_rem(exp_x);
    let two = WNarrow::from_i128(2);
    if is_cosh {
        (exp_x + exp_neg_x).div_rem(two).0
    } else {
        (exp_x - exp_neg_x).div_rem(two).0
    }
}

/// Narrow integer-regime `sinh(x)` via the wide [`WNarrow`] work integer,
/// for when the result exceeds the 256-bit `Fixed`'s headroom. `sinh` is
/// odd. A kept alternative: the strict narrow path reaches that regime
/// through `trig_series_2limb::sinh_ziv` instead, so this is currently
/// exercised only by `trig_series_2limb::hyper_fast_path_validity`.
pub(crate) fn sinh_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let working_scale = scale + working_digits;
    let is_negative = raw < 0;
    let abs_working_value = WNarrow::from_i128(raw.unsigned_abs() as i128)
        * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let sinh_magnitude = hyper_pos_wide_narrow(abs_working_value, working_scale, false);
    narrow_round_mag(sinh_magnitude, working_digits, mode, true, is_negative).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::sinh", scale)
    })
}

/// Narrow integer-regime `cosh(x)` via the wide [`WNarrow`] work integer,
/// for when the result exceeds the 256-bit `Fixed`'s headroom. `cosh` is
/// even (always non-negative). A kept alternative on the same footing as
/// [`sinh_wide_narrow_raw`]: the strict narrow path reaches that regime
/// through `trig_series_2limb::cosh_ziv` instead.
pub(crate) fn cosh_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> i128 {
    let working_scale = scale + working_digits;
    let abs_working_value = WNarrow::from_i128(raw.unsigned_abs() as i128)
        * crate::consts::pow10::dispatch::<WNarrow>(working_digits);
    let cosh_magnitude = hyper_pos_wide_narrow(abs_working_value, working_scale, true);
    narrow_round_mag(cosh_magnitude, working_digits, mode, true, false).unwrap_or_else(|| {
        crate::support::diagnostics::overflow_panic_with_scale("D38::cosh", scale)
    })
}

/// Whether the narrow `sinh`/`cosh` result for `raw` at `scale` exceeds
/// the 256-bit `Fixed`'s headroom and must route through [`WNarrow`].
/// `sinh(x)`/`cosh(x) ≈ e^|x|/2`, so the result's integer-digit count
/// matches `e^|x|`'s — reuse the exp gate on `|raw|`.
#[inline]
pub(crate) fn hyper_needs_wide_narrow(raw: i128, scale: u32, working_scale: u32) -> bool {
    !narrow_fixed_fits(raw.unsigned_abs() as i128, scale, working_scale)
}

/// Const-folded at `working_digits = STRICT_GUARD`.
/// `None` = result out of storage range.
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    exp_strict_raw::<SCALE>(raw.as_i128(), mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`exp_strict`].
#[inline]
fn exp_strict_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> Option<i128> {
    if raw == 0 {
        return Some(10_i128.pow(SCALE));
    }
    let working_scale = SCALE + STRICT_GUARD;
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
    // narrow exp — stays on the fast path. Both terminals are near-tie
    // protected (clear-of-tie single shot, Ziv walker behind it).
    if !narrow_fixed_fits(raw, SCALE, working_scale)
        || !crate::support::rounding::is_nearest_mode(mode)
    {
        return exp_wide_narrow_strict_raw(raw, SCALE, mode);
    }
    let negative_input = raw < 0;
    let working_value =
        Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(STRICT_GUARD));
    let working_value = if negative_input { working_value.neg() } else { working_value };
    let exp_working_value = exp_fixed(working_value, working_scale);
    match exp_working_value.round_to_i128_clear_of_tie(working_scale, SCALE, mode) {
        Some(rounded) => rounded,
        None => narrow_ziv::walk_checked_never_exact(
            exp_working_value.round_to_i128_with(working_scale, SCALE, mode),
            STRICT_GUARD,
            SCALE,
            mode,
            |guard_digits| exp_ziv(raw, SCALE, guard_digits),
        ),
    }
}

// ── exp2 kernel (D38, Fixed fallback) ─────────────────────────────

/// Exact-power pin for the D38 `exp2`. When `raw` is an exact integer
/// `k`, `exp2(k) = 2^k` is an exact algebraic point — a *dyadic
/// rational*, never a transcendental residual. Returns the
/// **correctly-rounded** storage value of `2^k` under `mode`, computed
/// from exact integer arithmetic, so the `exp(k·ln 2)` series round-off
/// can never bump it across a tie or grid line. [`ExactPin::Defer`] only
/// when `raw` is not an exact integer (the genuinely transcendental case
/// the series kernel handles); a positive `2^k` past the decimal range is
/// [`ExactPin::OutOfRange`] — the exact ladder's overflow is PROOF,
/// detected once here: the kernel returns `None` for it (the policy
/// dispatch wrapper applies the default form's contractual panic, the
/// `checked_` surface propagates) rather than deferring to the
/// `exp(k·ln 2)` composition, whose to-nearest approximation can
/// directed-round (Floor / Trunc) back INSIDE the range at an out-by-one
/// boundary (`exp2(127)` at scale 0 is `i128::MAX + 1`). See the
/// wide-tier `exp2_exact_pow`.
#[inline]
fn exp2_exact_pin(raw: i128, scale: u32, mode: RoundingMode) -> ExactPin<i128> {
    let one_scaled = match 10i128.checked_pow(scale) {
        Some(value) => value,
        None => return ExactPin::Defer,
    };
    if raw % one_scaled != 0 {
        return ExactPin::Defer;
    }
    let exponent = raw / one_scaled;
    if exponent == 0 {
        return ExactPin::Value(one_scaled);
    }
    let abs_exponent = exponent.unsigned_abs();
    if exponent > 0 {
        // 2^k · 10^scale — exact integer when representable; the ladder's
        // overflow is the out-of-range proof.
        let mut power: i128 = one_scaled;
        for _ in 0..abs_exponent {
            power = match power.checked_mul(2) {
                Some(doubled) => doubled,
                None => return ExactPin::OutOfRange,
            };
        }
        ExactPin::Value(power)
    } else if abs_exponent <= scale as u128 {
        // 2^-|k| = 5^|k| · 10^(scale − |k|) — exact, no rounding. These
        // checked steps cannot fail for any real scale (the value is
        // `<= 10^scale`); kept checked-and-deferring for totality.
        let mut power = match 10i128.checked_pow(scale - abs_exponent as u32) {
            Some(value) => value,
            None => return ExactPin::Defer,
        };
        for _ in 0..abs_exponent {
            power = match power.checked_mul(5) {
                Some(next_power) => next_power,
                None => return ExactPin::Defer,
            };
        }
        ExactPin::Value(power)
    } else {
        // |k| > scale: `2^k · 10^scale = 5^scale / 2^(|k|−scale)` is a
        // proper dyadic fraction in `(0, 1)` storage units. Round it
        // exactly under `mode` (`exp2(-1) = 0.5` is the half-to-even tie
        // → 0; `exp2(-146)` is a sub-resolution positive → Ceiling → 1).
        let numerator = match 5u128.checked_pow(scale) {
            Some(value) => value, // 5^38 < 2^89, fits u128
            None => return ExactPin::Defer,
        };
        let shift = abs_exponent as u32 - scale; // shift amount, ≥ 1
        ExactPin::Value(round_pow2_fraction(numerator, shift, mode))
    }
}

/// Correctly-rounded storage value of the dyadic fraction
/// `numerator / 2^shift` (`numerator > 0`, `shift ≥ 1`) — a
/// strictly-positive result in `[0, numerator/2]`.
///
/// `quotient = numerator >> shift`, `remainder = numerator & (2^shift − 1)`;
/// the half-way divisor is `2^shift`, so the tie compares `2·r` against
/// `2^shift`. When `shift ≥ 128` the quotient is `0` and the whole of
/// `numerator` is the (sub-half) residual — a tiny positive value that
/// `Ceiling`, `AwayFromZero` and `ZeroFiveUp` round up (`0` is a
/// `ZeroFiveUp` pivot digit) and the remaining modes truncate away.
#[inline]
fn round_pow2_fraction(numerator: u128, shift: u32, mode: RoundingMode) -> i128 {
    if shift >= 128 {
        // num < 2^128 ≤ 2^p, so q = 0 and r = num > 0 but < 2^(p-1)
        // (half), i.e. a sub-resolution positive residual.
        let bump = crate::support::rounding::should_bump(
            mode,
            ::core::cmp::Ordering::Less, // r strictly below half
            0,                           // q == 0, so its last digit is 0
            true,                        // result positive
        );
        return i128::from(bump);
    }
    let quotient = (numerator >> shift) as i128;
    let remainder = numerator & ((1u128 << shift) - 1);
    if remainder == 0 {
        return quotient;
    }
    let half = 1u128 << (shift - 1);
    let remainder_cmp = remainder.cmp(&half);
    // `quotient` is non-negative here (`numerator > 0`, arithmetic shift right).
    let q_mod_10 = (quotient % 10) as u8;
    let bump = crate::support::rounding::should_bump(mode, remainder_cmp, q_mod_10, true);
    quotient + i128::from(bump)
}

/// The guard-parameterised `i128` core of the narrow `exp2`, kept for the
/// sweep tests that drive it across guards. `None` = result out of range.
#[inline]
fn exp2_with_raw(raw: i128, scale: u32, working_digits: u32, mode: RoundingMode) -> Option<i128> {
    if raw == 0 {
        return Some(10_i128.pow(scale));
    }
    // Exact-power pin: `exp2(integer k) = 2^k` is an exact algebraic
    // point (integer for `k >= 0`, `5^|k|·10^(scale−|k|)` for `k < 0`).
    // Emitting it directly stops the `exp(k·ln 2)` round-off from
    // bumping a directed mode by one LSB at the exact power. A proven
    // out-of-range positive power is the kernel's `None` (the policy
    // dispatch wrapper applies the default form's panic; the `checked_`
    // surface propagates) — never deferred to the composition.
    match exp2_exact_pin(raw, scale, mode) {
        ExactPin::Value(pinned) => return Some(pinned),
        ExactPin::OutOfRange => return None,
        ExactPin::Defer => {}
    }
    // Integer-regime gate (mirrors `exp_strict_raw`): `2^x = e^(x·ln 2)` whose
    // result carries `k_lift` integer digits leaves the flat-`w` 256-bit
    // `Fixed` too few fractional guard digits to round correctly (e.g. 2^93
    // has 28 integer digits — the exp2_d38_s9 mis-round). Route those cells to
    // the wider work integer, which lifts the working scale by `k_lift` exactly
    // as the wide-tier `exp2_guarded` does. Small results stay on the fast path.
    let abs_raw = raw.unsigned_abs();
    if exp2_result_int_digits(abs_raw, scale) > FAST_MAX_RESULT_DIGITS {
        return exp2_wide_narrow_raw(raw, scale, working_digits, mode);
    }
    let working_scale = scale + working_digits;
    let negative_input = raw < 0;
    let working_value =
        Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(working_digits));
    let working_value = if negative_input { working_value.neg() } else { working_value };
    let arg_at_working_scale = working_value.mul(wide_ln2(working_scale), working_scale);
    exp_fixed(arg_at_working_scale, working_scale)
        .round_to_i128_with(working_scale, scale, mode)
}

/// Integer-digit count of `2^x` for the non-negative storage magnitude
/// `abs_raw` at `scale` (`x = abs_raw / 10^scale`). `int_digits(2^x) =
/// floor(x · log10 2) + 1`; for `x == 0` it is one digit. `log10 2 ≈
/// 301030 / 1_000_000`, rounded UP (`div_ceil`) so the count is never
/// under-stated — the safe direction for the [`FAST_MAX_RESULT_DIGITS`]
/// gate (errs toward the wide path). Mirrors [`exp_result_int_digits`]'s
/// overflow-free `q`/`r` split so no intermediate exceeds `u128`.
#[inline]
fn exp2_result_int_digits(abs_raw: u128, scale: u32) -> u32 {
    let one_scaled = match 10u128.checked_pow(scale) {
        Some(power) => power,
        None => return 1,
    };
    let integer_part = abs_raw / one_scaled; // integer part of |x|
    let fraction_remainder = abs_raw % one_scaled; // < 10^scale
    let integer_part_capped = integer_part.min(180); // past here the count far exceeds the band
    let fraction_e7 = if scale <= 7 {
        fraction_remainder * 10u128.pow(7 - scale)
    } else {
        fraction_remainder / 10u128.pow(scale - 7)
    };
    let x_e7 = integer_part_capped * 10_000_000 + fraction_e7; // |x| · 10^7 (capped)
    let numerator = x_e7 * 301_030; // / 10^7 / 10^6 = / 10^13 gives |x|·log10 2
    (numerator.div_ceil(10u128.pow(13)).min(u32::MAX as u128 - 1) as u32) + 1
}

/// LOWER bound on the integer-digit count of `2^x` for the non-negative
/// storage magnitude `abs_raw` at `scale` — the floor counterpart of the
/// (deliberately over-stating) [`exp2_result_int_digits`]. Every rounding
/// here errs DOWN: the fraction `fraction_e7` is floored, `301_029/10^6`
/// under-approximates `log10 2`, and the final division is floored — so the
/// returned count never exceeds the true `⌊x·log10 2⌋ + 1`. That is the
/// safe direction for the [`exp2_wide_narrow_raw`] overflow gate: a cell
/// it fires on is PROVABLY out of range, never a representable one.
///
/// The integer-part cap matches the sibling's: at `q ≥ 180` the count is
/// already ≈ 55 — past the 40-digit `i128` ceiling at EVERY scale — so
/// capping keeps `x_e7 · 301_029` inside `u128` without weakening the gate.
#[inline]
fn exp2_result_int_digits_floor(abs_raw: u128, scale: u32) -> u32 {
    let one_scaled = match 10u128.checked_pow(scale) {
        Some(power) => power,
        None => return 1,
    };
    let integer_part = abs_raw / one_scaled; // integer part of |x|
    let fraction_remainder = abs_raw % one_scaled; // < 10^scale
    let integer_part_capped = integer_part.min(180);
    let fraction_e7 = if scale <= 7 {
        fraction_remainder * 10u128.pow(7 - scale)
    } else {
        fraction_remainder / 10u128.pow(scale - 7)
    };
    let x_e7 = integer_part_capped * 10_000_000 + fraction_e7; // ≤ |x| · 10^7
    let numerator = x_e7 * 301_029; // / 10^13 under-states |x|·log10 2
    ((numerator / 10u128.pow(13)).min(u32::MAX as u128 - 1) as u32) + 1
}

/// Integer-regime / large-result `2^x` for the narrow tier, evaluated in the
/// wider [`WNarrow`] work integer via the width-generic
/// [`exp_generic::exp_fixed`], then narrowed with correct rounding. The
/// working scale is lifted by the result's integer-digit count `k_lift` so the
/// argument `x·ln 2` AND the `e^(x·ln 2)` evaluation keep enough fractional
/// guard past the many integer digits — the narrow analogue of the wide-tier
/// `exp2_guarded`'s `GUARD + k_lift`. [`narrow_round_mag`]'s `never_exact`
/// gives the directed modes the sub-resolution residual a transcendental needs.
fn exp2_wide_narrow_raw(
    raw: i128,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<i128> {
    let (exp_x, lifted_guard_digits) = exp2_wide_narrow_eval(raw, scale, working_digits)?;
    narrow_round_mag(exp_x, lifted_guard_digits, mode, true, false)
}

/// The [`exp2_wide_narrow_raw`] evaluation: the
/// `(value · 10^(scale + lifted_guard_digits), lifted_guard_digits)` pair
/// before the strict near-tie protected narrowing terminal.
fn exp2_wide_narrow_eval(
    raw: i128,
    scale: u32,
    working_digits: u32,
) -> Option<(WNarrow, u32)> {
    use crate::algos::exp::exp_generic;

    let is_negative = raw < 0;
    let abs_raw = raw.unsigned_abs();
    // Storage overflow gate — BEFORE any working-scale arithmetic. The
    // `k_lift` lift below grows the working scale `w` with the result's
    // integer-digit count, so a deep-overflow argument inflates every
    // dividend downstream (`x·ln 2`, the kernel's own `k` range-reduction
    // divide) far past what the build's divide scratch provisions for
    // in-range work — tripping an INTERNAL kernel assertion instead of
    // the contractual out-of-range signal. Analytic bound, exact for
    // every scale: the result `2^x ≥ 10^(d−1)` for `d = int_digits(2^x)`,
    // so its storage value is `≥ 10^(d−1+scale)`; `i128` holds values
    // `< 1.8·10^38 < 10^39`, hence `d + scale ≥ 40` PROVES the result
    // cannot be stored — return the kernel's `None` (the policy dispatch
    // wrapper applies the default form's contractual panic; the
    // `checked_` surface propagates it). `d` is the floor lower bound
    // (never over-stated), so no representable cell can fire; cells
    // between the true edge and this bound still flow to the kernel,
    // whose post-narrowing fit check ([`narrow_round_mag`] → `None`)
    // signals the SAME `None` — and for those cells `d + scale ≤ 39`
    // bounds the lift (`w ≤ scale + working_digits + 41 − scale`),
    // keeping every dividend inside the scratch at every scale.
    if raw > 0 && exp2_result_int_digits_floor(abs_raw, scale).saturating_add(scale) >= 40 {
        return None;
    }
    // The lift exists to keep guard digits ABOVE the result's integer
    // digits. A negative argument has none (`2^x < 1`), so its lift is 0 —
    // lifting by `int_digits(2^|x|)` would re-inflate `w` without bound for
    // a deep-underflow argument (the mirror of the overflow band above),
    // with zero precision benefit: the `2^k` reassembly shifts DOWN.
    let k_lift = if is_negative { 0 } else { exp2_result_int_digits(abs_raw, scale) };
    let lifted_guard_digits = working_digits + k_lift;
    let working_scale = scale + lifted_guard_digits;
    // x · ln 2 at the working scale, formed in the wide work integer:
    //   x_w = x·10^w = abs_raw·10^guard ;  ln2_w = ln 2·10^w ;
    //   (x_w · ln2_w) / 10^w = x·ln 2 · 10^w.
    let x_working_value = WNarrow::from_i128(abs_raw as i128)
        * crate::consts::pow10::dispatch::<WNarrow>(lifted_guard_digits);
    let ln2_at_working_scale =
        crate::consts::ln2_by_working_scale::<WNarrow>(working_scale, RoundingMode::HalfToEven);
    let product = x_working_value * ln2_at_working_scale;
    let exp_arg_magnitude = if working_scale <= 38 {
        crate::algos::support::mg_divide::div_wide_pow10::<WNarrow>(
            product, working_scale, RoundingMode::HalfToEven)
    } else {
        crate::algos::support::mg_divide::div_wide_pow10_chain::<WNarrow>(
            product,
            working_scale,
            RoundingMode::HalfToEven,
        )
    };
    let exp_arg = if is_negative { -exp_arg_magnitude } else { exp_arg_magnitude };
    // `try_exp_fixed`: see [`exp_wide_narrow_raw`] — an out-of-range
    // verdict from the generic kernel propagates as this kernel's `None`.
    let exp_x = exp_generic::try_exp_fixed::<WNarrow>(exp_arg, working_scale)?;
    Some((exp_x, lifted_guard_digits))
}

/// One `WZiv` `exp(x·ln 2)` probe at working scale `scale + guard_digits`.
/// No `k_lift` is applied at the probe — the walker's escalation cap
/// already subtracts the result's integer-digit count, so the probe
/// keeps its guard digits without an explicit lift.
fn exp2_ziv(raw: i128, scale: u32, guard_digits: u32) -> WZiv {
    use crate::algos::exp::exp_generic as eg;
    let working_scale = scale + guard_digits;
    let exp_arg = eg::mul::<WZiv>(
        narrow_ziv::lift(raw, guard_digits),
        narrow_ziv::ln2_w(working_scale),
        working_scale);
    eg::exp_fixed::<WZiv>(exp_arg, working_scale)
}

/// `None` = result out of storage range (see [`exp2_with_raw`]). The strict
/// terminal is near-tie protected on both branches (the exact-power pin
/// already removes every rational `2^x`; `2^x` is irrational for every
/// other on-grid `x`, so the never-exact walker polarity is sound).
#[inline]
#[must_use]
pub(crate) fn exp2_strict<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    exp2_strict_raw(raw.as_i128(), SCALE, mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`exp2_strict`].
fn exp2_strict_raw(raw: i128, scale: u32, mode: RoundingMode) -> Option<i128> {
    if raw == 0 {
        return Some(10_i128.pow(scale));
    }
    match exp2_exact_pin(raw, scale, mode) {
        ExactPin::Value(pinned) => return Some(pinned),
        ExactPin::OutOfRange => return None,
        ExactPin::Defer => {}
    }
    let abs_raw = raw.unsigned_abs();
    if exp2_result_int_digits(abs_raw, scale) > FAST_MAX_RESULT_DIGITS {
        // Integer-regime: the lifted wide single shot, near-tie protected.
        let (exp_x, lifted_guard_digits) = exp2_wide_narrow_eval(raw, scale, STRICT_GUARD)?;
        let single_shot = narrow_round_mag(exp_x, lifted_guard_digits, mode, true, false);
        if wnarrow_residual_clear(exp_x, lifted_guard_digits, mode) {
            return single_shot;
        }
        return narrow_ziv::walk_checked_never_exact(
            single_shot, STRICT_GUARD, scale, mode, |guard_digits| {
                exp2_ziv(raw, scale, guard_digits)
            });
    }
    let working_scale = scale + STRICT_GUARD;
    let negative_input = raw < 0;
    let working_value = Fixed::from_u128_mag(abs_raw, false).mul_u128(10u128.pow(STRICT_GUARD));
    let working_value = if negative_input { working_value.neg() } else { working_value };
    let arg_at_working_scale = working_value.mul(wide_ln2(working_scale), working_scale);
    let exp_working_value = exp_fixed(arg_at_working_scale, working_scale);
    match exp_working_value.round_to_i128_clear_of_tie(working_scale, scale, mode) {
        Some(rounded) => rounded,
        None => narrow_ziv::walk_checked_never_exact(
            exp_working_value.round_to_i128_with(working_scale, scale, mode),
            STRICT_GUARD,
            scale,
            mode,
            |guard_digits| exp2_ziv(raw, scale, guard_digits),
        ),
    }
}

// Deep-underflow directed-rounding guard (golden exp.golden:4748):
// `exp(-62.17530480440519)` ≈ 9.945e-28 (mpmath/flint-validated) is a
// strictly positive SUB-RESOLUTION result at scales >= 14, so the
// correctly-rounded storage value is 1 ULP under Ceiling and 0 under every
// other mode. The directed modes escalate through the never-exact Ziv
// walker (the residual sits inside the near-tie band at every probe
// depth); the walker's cap-clamped deepest probe runs the generic exp
// kernel past its internal squaring peak (`k = -90`, `w_ext = 231` in
// `Int<24>` — the 2·w_ext-digit peak tops the sign bit), handing back a
// NEGATIVE probe — which would invert
// Ceiling to 0 and Floor to -1 if trusted. The walker returns the clean base
// narrowing at an unresolved cap (`wide_trig_core::
// round_to_storage_directed_impl_g`); this pins the whole band's verdict
// at the kernel layer for every mode.
#[cfg(test)]
mod deep_underflow_directed {
    use super::*;

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Ceiling,
        RoundingMode::Floor,
        RoundingMode::Trunc,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    /// The result is a sub-resolution POSITIVE, so every mode that moves a
    /// discarded remainder away from zero lands on 1 storage ULP: `Ceiling`
    /// (toward +∞), `AwayFromZero` (anything discarded), and `ZeroFiveUp`
    /// (the retained digit is `0`, one of its two bump digits). The rest
    /// truncate to 0.
    fn rounds_up_to_one_ulp(mode: RoundingMode) -> bool {
        matches!(
            mode,
            RoundingMode::Ceiling | RoundingMode::AwayFromZero | RoundingMode::ZeroFiveUp
        )
    }

    /// `-62.17530480440519` lifted onto the storage grid at `scale`
    /// (exact for every `scale >= 14`).
    fn raw_at(scale: u32) -> i128 {
        -6_217_530_480_440_519 * 10_i128.pow(scale - 14)
    }

    #[test]
    fn exp_deep_underflow_rounds_correctly_all_modes() {
        // const-generic SCALE forces a literal per scale; same band, same
        // predicate (result magnitude strictly below the storage LSB).
        let cells: [(u32, fn(RoundingMode) -> Option<i128>); 3] = [
            (17, |mode| exp_strict_raw::<17>(raw_at(17), mode)),
            (18, |mode| exp_strict_raw::<18>(raw_at(18), mode)),
            (19, |mode| exp_strict_raw::<19>(raw_at(19), mode)),
        ];
        for (scale, run) in cells {
            for mode in ALL_MODES {
                let want = i128::from(rounds_up_to_one_ulp(mode));
                assert_eq!(
                    run(mode),
                    Some(want),
                    "exp(-62.17530480440519) scale={scale} mode={mode:?}"
                );
            }
        }
    }
}

// ── Fast-path validity wall ────────────────────────────────────────
// The narrow exp gate (`exp_strict_raw`) routes a cell
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
        let working_scale = scale + STRICT_GUARD;
        let negative_input = raw < 0;
        let working_value =
            Fixed::from_u128_mag(raw.unsigned_abs(), false).mul_u128(10u128.pow(STRICT_GUARD));
        let working_value = if negative_input { working_value.neg() } else { working_value };
        std::panic::catch_unwind(|| exp_fixed(working_value, working_scale)
            .round_to_i128_with(working_scale, scale, mode))
            .unwrap_or(None)
    }

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

    /// Mirror the production gate exactly: `true` ⇒ this cell stays on the
    /// fast path (so fast MUST equal wide). Directed modes always route wide.
    fn gate_keeps_fast(raw: i128, scale: u32, mode: RoundingMode) -> bool {
        let working_scale = scale + STRICT_GUARD;
        narrow_fixed_fits(raw, scale, working_scale)
            && crate::support::rounding::is_nearest_mode(mode)
    }

    // For EVERY D38 cell the production gate routes to the fast path,
    // assert it is bit-identical to the wide reference. A fine 0.1-step
    // |x| grid over the whole representable range, both signs, every
    // mode, scale 0..=38 — so an unbenched scale cannot silently get a
    // wrong fast result.
    #[test]
    fn fast_path_bit_identical_to_wide_d38() {
        std::panic::set_hook(Box::new(|_| {}));
        let mut checked = 0u64;
        for scale in 0u32..=38 {
            let one_scaled = 10f64.powi(scale as i32);
            let mut x_tenths = 1u64;
            while x_tenths <= 1000 {
                let x_value = x_tenths as f64 / 10.0;
                for sign in [1i128, -1] {
                    let raw_float = sign as f64 * x_value * one_scaled;
                    if raw_float.abs() >= (i128::MAX as f64) {
                        x_tenths += 1;
                        continue;
                    }
                    let raw = raw_float as i128;
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
                            Ok(Some(value)) => value,
                            // Wide reference itself overflows i128 — the
                            // narrow tier cannot represent the result; both
                            // paths report out of range, not a fast-vs-wide
                            // question.
                            Ok(None) | Err(_) => continue,
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
                x_tenths += 1;
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

    // Integer-regime exp2 guard (the golden exp2_d38_s9 defect): 2^x
    // whose result has many integer digits (2^93 ≈ 10^28) leaves the flat-`w`
    // 256-bit `Fixed` too few fractional guard digits, mis-rounding the
    // last ULPs. The gate routes such cells to `exp2_wide_narrow_raw`.
    // Pin the exposing cell (class "Low": every mode → floor, except Ceiling
    // and AwayFromZero → floor+1) plus a small integer-regime sweep checking
    // the rounding order stays consistent (floor ≤ nearest ≤ ceil,
    // ceil − floor ≤ 1). Guards the fix in the fast default build, parallel
    // to the atanh near-1 test; the golden floor is confirmed by flint (Arb)
    // and mpmath.
    //
    // 93.013986656 has a non-integer exponent, so 2^93.013986656 is
    // irrational and can never land exactly on the scale-9 grid: the
    // discard is non-zero by theorem, not by measurement. The value is
    // positive with residual 0.4301 (below half) and a floor ending in 8,
    // so AwayFromZero (bumps on any non-zero discard) lands at floor+1,
    // while ZeroFiveUp (bumps only on a 0/5 last digit) stays at the floor
    // since 8 is not a pivot digit.
    #[test]
    fn exp2_integer_regime_matches_golden_floor() {
        const S9: u32 = 9;
        let raw = 93_013_986_656_i128; // 93.013986656 at scale 9
        let golden_floor: i128 = 9_999_999_994_134_964_658_924_521_484_307_802_708;
        for &mode in &MODES {
            let got = exp2_with_raw(raw, S9, STRICT_GUARD, mode);
            let want = if matches!(mode, RoundingMode::Ceiling | RoundingMode::AwayFromZero) {
                golden_floor + 1
            } else {
                golden_floor
            };
            assert_eq!(got, Some(want), "exp2(93.013986656) s9 mode={mode:?}");
        }
        // Integer-regime inputs whose result still fits i128 at scale 9:
        // 2^x·10^9 < i128::MAX ≈ 1.7·10^38 ⇒ x ≲ 97. (2^100·10^9 ≈ 1.3·10^39
        // overflows and correctly panics, so keep the sweep below that.)
        for &sweep_raw in &[50_000_000_000_i128, 70_000_000_000, 90_000_000_000] {
            let half_even = exp2_with_raw(sweep_raw, S9, STRICT_GUARD, RoundingMode::HalfToEven)
                .expect("in range");
            let floor = exp2_with_raw(sweep_raw, S9, STRICT_GUARD, RoundingMode::Floor)
                .expect("in range");
            let ceiling = exp2_with_raw(sweep_raw, S9, STRICT_GUARD, RoundingMode::Ceiling)
                .expect("in range");
            assert!(
                floor <= half_even && half_even <= ceiling,
                "exp2 rounding order violated at raw={sweep_raw}"
            );
            assert!(
                ceiling - floor <= 1,
                "exp2 floor/ceil differ by >1 at raw={sweep_raw}"
            );
        }
    }
}
