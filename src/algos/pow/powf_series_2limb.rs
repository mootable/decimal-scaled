// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Floating-point power series kernel — `powf` computed as
//! `exp(y · ln(x))` on the 256-bit `Fixed` guard-digit intermediate.
//!
//! The narrow `Int<2>`-storage series path: it serves the narrow
//! D18 / D38 tier, composing the `ln` and `exp` series kernels in the
//! wider `Fixed` intermediate. The `ln`, the multiplication, and the
//! `exp` all share the wide guard-digit working scale so no precision
//! is dropped between stages.
//!
//! Fast path preserved verbatim from the typed surface:
//! - A non-positive base saturates to `0` (matches the f64-bridge
//!   NaN-to-ZERO policy for negative bases with arbitrary fractional
//!   exponents).
//! - Integer-valued exponents with `|n| <= INT_FAST_PATH_THRESHOLD`
//!   route to `powi_raw_checked::<SCALE>` — exact via square-and-multiply
//!   on raw, ~10–500× faster than the `exp(y·ln(x))` chain. When a partial
//!   power leaves the storage range it returns `None` and the call defers
//!   to the overflow-safe composition (so an in-range reciprocal such as
//!   `powf(10, -2) = 0.01` computes instead of panicking on its `10² = 100`
//!   intermediate).
//!
//! Returns the raw `i128` storage at the input's scale.

use crate::algos::exp::exp_generic as eg;
use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::narrow_ziv::{self, WZiv};
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Integer-exponent fast-path threshold for `powf_strict`.
///
/// At `|n| <= 64`, the square-and-multiply `powi(n)` costs at most
/// ~12 multiplications (2·log2(64)) — comfortably cheaper than the
/// `exp(y · ln(x))` chain (one `ln_fixed` + one `mul` + one
/// `exp_fixed`, each ~hundreds of i128 ops on the 256-bit `Fixed`
/// intermediate). Above 64 the integer path's cost grows
/// logarithmically while the transcendental path's cost is constant,
/// so a fixed threshold is sufficient.
pub(crate) const INT_FAST_PATH_THRESHOLD: i32 = 64;

/// Analytic storage-overflow gate on the `exp(y·ln x)` composition's
/// argument `arg = y·ln x` (a working-scale [`Fixed`] at scale `w`),
/// run BEFORE [`exp_fixed`]. Returns `true` when the result
/// `x^y = e^arg` provably cannot be stored: the kernel signals its
/// out-of-range `None` (the policy dispatch wrapper applies the default
/// form's contractual `"result out of range"` panic; the `checked_`
/// surface propagates the `None`).
///
/// Derivation, exact for every scale: `i128` holds values `< 1.8·10^38 <
/// 10^39`, so the storage value `e^arg · 10^scale` overflows once
/// `e^arg ≥ 10^(39−scale)`, i.e. `arg ≥ (39−scale)·ln 10`. The threshold
/// uses `2.302586 > ln 10`, so a fired cell satisfies the true bound — a
/// representable result can never fire. Cells between the true edge and
/// this threshold still flow to the kernel, whose `round_to_i128_with`
/// fit check signals the same `None`; for those `arg < (39−scale)·2.302586
/// + 1 ≤ 91` bounds the kernel's `2^k` reassembly (`k ≤ 132`), so the
/// kernel's internal shift arithmetic stays in range. Without this gate a
/// deep-overflow exponent reached the kernel's 256-bit reassembly
/// assertion (loud but non-contractual), and an extreme one could wrap
/// the `k` shift narrowing entirely.
///
/// The threshold magnitude `(39−scale)·2_302_586 · 10^(w−6)` fits `U256`
/// for every working scale this kernel serves (`w ≤ 68`); `w < 6` (no
/// real caller) skips the gate and leaves the kernel assert as backstop.
#[inline]
fn powf_overflow_gate(arg: Fixed, w: u32, scale: u32) -> bool {
    if arg.negative || w < 6 {
        return false; // y·ln x < 0 ⇒ x^y < 1: never a storage overflow.
    }
    let thr = Fixed {
        negative: false,
        mag: Fixed::pow10(w - 6),
    }
    .mul_u128(((39 - scale) as u128) * 2_302_586);
    arg.ge_mag(thr)
}

/// `(a · b) / 10^SCALE` rounded under `mode`, on `i128` storage. Returns
/// `None` when the rounded result does not fit `i128` — the signal
/// [`powi_raw_checked`] uses to defer to the overflow-safe `exp(y·ln x)`
/// composition rather than compute a partial power that has left the
/// storage range.
///
/// Delegates to [`mul_div_pow10_with`] — the tiered narrow engine the
/// decimal mul path runs on: a pure-`i128` multiply when the product
/// fits (the common band; at `SCALE == 0` the whole op is one
/// `checked_mul`), the `u128`-product hardware-divide band above it,
/// and the full 256-bit widening only when genuinely needed. The prior
/// shape paid the 256-bit `mul_widen_divide::<4, SCALE>` machinery
/// unconditionally — the dominant cost of the bbc powf D18<0> cell.
/// Value and rounding are engine-independent (the same correctly-rounded
/// `(a·b)/10^SCALE`), so results are bit-identical.
///
/// [`mul_div_pow10_with`]: crate::algos::support::mg_divide::mul_div_pow10_with
#[inline]
fn mul_div_scale_checked<const SCALE: u32>(a: i128, b: i128, mode: RoundingMode) -> Option<i128> {
    crate::algos::support::mg_divide::mul_div_pow10_with::<SCALE>(a, b, mode)
}

/// Integer-exponent square-and-multiply on raw `i128` storage at `SCALE`,
/// returning `None` if any partial power `base^k` (`k ≤ |n|`) leaves the
/// `i128` storage range. Uses `mul_widen_divide` directly (not the decimal
/// `Mul` operator, which would re-enter the mul policy from inside an
/// algorithm fn — the layering inversion). `one_s` is `10^SCALE`.
///
/// Every intermediate (`acc`, `b`) is bounded by `base^|n|`, so a `None`
/// means the full integer power `base^|n|` is itself out of range. The
/// caller then routes to the composition, which panics on a genuinely
/// out-of-range result (positive `n`) and computes the in-range reciprocal
/// `base^-|n|` overflow-safely (negative `n`) — fixing the integer fast
/// path's spurious panic on `powf(10, -2)` (`10² = 100` overflows storage
/// while `1/100 = 0.01` is representable). When `Some`, the result is the
/// exact integer power (or its reciprocal), bit-identical to before.
///
/// For negative `n`, returns `one_s / base^|n|` via the decimal
/// `div_widen_scale` (a genuine downward cross-tier call to the int layer).
#[inline]
fn powi_raw_checked<const SCALE: u32>(base: i128, n: i32, mode: RoundingMode) -> Option<i128> {
    let one_s: Int<2> = const { crate::consts::pow10::dispatch_int::<2>(SCALE) };
    if n == 0 {
        return Some(one_s.as_i128());
    }
    let pos_n = n.unsigned_abs();
    // Square-and-multiply; every partial product is range-checked so an
    // out-of-storage intermediate signals `None` instead of panicking.
    let mut acc: i128 = one_s.as_i128();
    let mut b: i128 = base;
    let mut e = pos_n;
    while e > 0 {
        if e & 1 == 1 {
            acc = mul_div_scale_checked::<SCALE>(acc, b, mode)?;
        }
        e >>= 1;
        if e > 0 {
            b = mul_div_scale_checked::<SCALE>(b, b, mode)?;
        }
    }
    if n > 0 {
        Some(acc)
    } else {
        // Reciprocal: one_s / base^|n|. `acc = base^|n|` fits i128 here and
        // `1/acc ≤ 1`, so the quotient certainly fits storage.
        Some(
            crate::algos::div::div_widen_scale::div_widen_scale::<2>(
                one_s,
                Int::<2>::from_i128(acc),
                const { crate::consts::pow10::dispatch_int::<2>(SCALE) },
                mode,
            )
            .as_i128(),
        )
    }
}

/// Returns `Some(n)` if `exp_raw` (at `SCALE`) represents an exact
/// integer value `n` that fits `i32` and `|n| <= INT_FAST_PATH_THRESHOLD`.
#[inline]
fn exp_as_small_int<const SCALE: u32>(exp_raw: i128) -> Option<i32> {
    let mult = const { crate::consts::pow10::dispatch_i128(SCALE) };
    if exp_raw % mult != 0 {
        return None;
    }
    let q = exp_raw / mult;
    if !(i32::MIN as i128..=i32::MAX as i128).contains(&q) {
        return None;
    }
    let n = q as i32;
    if n.unsigned_abs() <= INT_FAST_PATH_THRESHOLD as u32 {
        Some(n)
    } else {
        None
    }
}

/// `base^exp` with caller-chosen `working_digits` above the storage scale.
///
/// Both `base` and `exp` are raw storage at `scale`. Non-positive `base`
/// saturates to `0`.
#[inline]
#[must_use]
pub(crate) fn powf_with<const SCALE: u32>(
    base: Int<2>,
    exp: Int<2>,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<Int<2>> {
    powf_with_raw::<SCALE>(base.as_i128(), exp.as_i128(), working_digits, mode)
        .map(Int::<2>::from_i128)
}

/// `i128` core of [`powf_with`].
#[inline]
fn powf_with_raw<const SCALE: u32>(
    base: i128,
    exp: i128,
    working_digits: u32,
    mode: RoundingMode,
) -> Option<i128> {
    if base <= 0 {
        return Some(0);
    }
    if let Some(n) = exp_as_small_int::<SCALE>(exp) {
        if let Some(v) = powi_raw_checked::<SCALE>(base, n, mode) {
            return Some(v);
        }
        // `base^|n|` left the storage range. When the base is an exact
        // integer the result is still an exact rational — pin its
        // correctly-directed-rounded value (`10^SCALE / base^|n|` for a
        // negative `n`) so a directed mode is not 1 LSB off, rather than
        // defer to the to-nearest `exp(y·ln x)` composition. A fractional
        // base defers to the composition; a genuinely out-of-range positive
        // power is the pin's PROOF of overflow — signal the kernel's `None`
        // (the policy dispatch wrapper applies the default form's panic, the
        // `checked_` surface propagates) instead of deferring, whose
        // to-nearest approximation could directed-round back inside range.
        use crate::algos::pow::powi_exact::ExactPin;
        match crate::algos::pow::powi_exact::powi_exact_pin_checked::<Int<2>, SCALE>(
            Int::<2>::from_i128(base),
            Int::<2>::from_i128(exp),
            Int::<2>::MAX,
            mode,
        ) {
            ExactPin::Value(v) => return Some(v.as_i128()),
            ExactPin::OutOfRange => return None,
            ExactPin::Defer => {}
        }
    }
    let w = SCALE + working_digits;
    let pow = 10u128.pow(working_digits);
    let ln_x = ln_fixed(Fixed::from_u128_mag(base as u128, false).mul_u128(pow), w);
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    let arg = y_w.mul(ln_x, w);
    if powf_overflow_gate(arg, w, SCALE) {
        return None;
    }
    exp_fixed(arg, w).round_to_i128_with(w, SCALE, mode)
}

/// Strict variant — const-folded `working_digits = STRICT_GUARD`.
/// `None` = result out of storage range (non-positive bases saturate
/// to `Some(0)`, matching the default form's documented behaviour).
#[inline]
#[must_use]
pub(crate) fn powf_strict<const SCALE: u32>(base: Int<2>, exp: Int<2>, mode: RoundingMode) -> Option<Int<2>> {
    powf_strict_raw::<SCALE>(base.as_i128(), exp.as_i128(), mode).map(Int::<2>::from_i128)
}

/// `i128` core of [`powf_strict`].
#[inline]
fn powf_strict_raw<const SCALE: u32>(base: i128, exp: i128, mode: RoundingMode) -> Option<i128> {
    if base <= 0 {
        return Some(0);
    }
    if let Some(n) = exp_as_small_int::<SCALE>(exp) {
        if let Some(v) = powi_raw_checked::<SCALE>(base, n, mode) {
            return Some(v);
        }
        // `base^|n|` left the storage range. When the base is an exact
        // integer the result is still an exact rational — pin its
        // correctly-directed-rounded value (`10^SCALE / base^|n|` for a
        // negative `n`) so a directed mode is not 1 LSB off, rather than
        // defer to the to-nearest `exp(y·ln x)` composition. A fractional
        // base defers to the composition; a genuinely out-of-range positive
        // power is the pin's PROOF of overflow — signal the kernel's `None`
        // (the policy dispatch wrapper applies the default form's panic, the
        // `checked_` surface propagates) instead of deferring, whose
        // to-nearest approximation could directed-round back inside range.
        use crate::algos::pow::powi_exact::ExactPin;
        match crate::algos::pow::powi_exact::powi_exact_pin_checked::<Int<2>, SCALE>(
            Int::<2>::from_i128(base),
            Int::<2>::from_i128(exp),
            Int::<2>::MAX,
            mode,
        ) {
            ExactPin::Value(v) => return Some(v.as_i128()),
            ExactPin::OutOfRange => return None,
            ExactPin::Defer => {}
        }
    }
    let w = SCALE + STRICT_GUARD;
    let pow = 10u128.pow(STRICT_GUARD);
    let ln_x = ln_fixed(Fixed::from_u128_mag(base as u128, false).mul_u128(pow), w);
    let y_neg = exp < 0;
    let y_w = Fixed::from_u128_mag(exp.unsigned_abs(), false).mul_u128(pow);
    let y_w = if y_neg { y_w.neg() } else { y_w };
    let arg = y_w.mul(ln_x, w);
    if powf_overflow_gate(arg, w, SCALE) {
        return None;
    }
    let v = exp_fixed(arg, w);
    match v.round_to_i128_clear_of_tie(w, SCALE, mode) {
        Some(r) => r,
        // Near a boundary. The constructible family is the EXACT rational
        // power reached through the composition (`powf(4, 0.5) = 2`,
        // `powf(225, 0.5) = 15` — the composition's value lands a
        // kernel-error hair OFF the exact grid value, so a directed mode
        // stepped 1 LSB off the exact power). Exact integer arithmetic
        // (the rational-power pin) decides those — a composition's
        // SYSTEMATIC relative error keeps a depth-stable position, so no
        // escalation can settle an exactly-boundary-valued cell. A nearest
        // exact-HALF powf value is PROVEN impossible for on-grid `(x, y)`
        // (the 2-adic/5-adic argument in the pin's docs), so only grid
        // candidates can verify; the walker resolves the genuinely
        // transcendental near-ties.
        None => {
            if let Some(num2) =
                v.double().round_to_i128_with(w, SCALE, RoundingMode::HalfToEven)
            {
                if let Some(pinned) = powf_rational_pin(base, exp, SCALE, num2) {
                    return Some(pinned);
                }
            }
            narrow_ziv::walk_checked(
                v.round_to_i128_with(w, SCALE, mode),
                STRICT_GUARD,
                SCALE,
                mode,
                |g| powf_ziv(base, exp, SCALE, g),
            )
        }
    }
}

/// One `WZiv` `exp(y·ln x)` probe at working scale `scale + g`.
fn powf_ziv(base: i128, exp: i128, scale: u32, g: u32) -> WZiv {
    let w = scale + g;
    let ln_x = eg::ln_fixed::<WZiv>(narrow_ziv::lift(base, g), w, narrow_ziv::ln2_w(w));
    let arg = eg::mul::<WZiv>(narrow_ziv::lift(exp, g), ln_x, w);
    eg::exp_fixed::<WZiv>(arg, w)
}

/// Exact rational-power pin for the strict powf near-tie terminal — the
/// powf sibling of `ln_series_2limb::log_rational_pow_pin`. `num2` is
/// the boundary candidate in half-ULPs at `scale` (the working value
/// doubled and nearest-rounded). With the exponent `y = p/q` (the input
/// reduced — exact by construction) the claim `x^y == num2/(2·10^scale)`
/// is the integer identity `x^p == v^q` over the reduced fractions,
/// verified with the bounded checked ladder. Only an EVEN `num2` (a grid
/// candidate) can verify — an exact-half powf value is impossible for
/// on-grid `(x, y)`: `x^y = (2k+1)/(2·10^S)` forces `x = u^q` with
/// `den(u^p) = 2^(S+1)·5^S`, whose 2-/5-adic exponents demand
/// `p | S+1` AND `p | S` ⇒ `p = 1`, and then `den(x) = (2^(S+1)·5^S)^q`
/// cannot divide `10^S` — so the half-candidate arm is unreachable and
/// simply returns `None`. A verified grid value is returned for every
/// mode (it IS the exact result).
fn powf_rational_pin(base: i128, exp: i128, scale: u32, num2: i128) -> Option<i128> {
    use crate::algos::ln::ln_series_2limb::{gcd_u128, pow_bounded, reduce_fraction};
    if num2 <= 0 || num2 & 1 == 1 || exp == 0 || base <= 0 {
        // x > 0 ⇒ x^y > 0; half candidates can't verify (see above);
        // y == 0 is the exact-1 case the integer fast path already pins.
        return None;
    }
    let one_s = 10u128.pow(scale);
    // y = p/q in lowest terms (sign split off).
    let y_neg = exp < 0;
    let g = gcd_u128(exp.unsigned_abs(), one_s);
    let p = exp.unsigned_abs() / g;
    let q = one_s / g;
    // x and the candidate v as reduced fractions.
    let (xn, xd) = reduce_fraction(base as u128, one_s);
    let (vn, vd) = reduce_fraction((num2 / 2) as u128, one_s);
    // x^(±p/q) == v  ⇔  x^(±p) == v^q  ⇔ (positive y) xn^p == vn^q ∧
    // xd^p == vd^q; (negative y) xd^p == vn^q ∧ xn^p == vd^q.
    let (tn, td) = if y_neg { (xd, xn) } else { (xn, xd) };
    let lx_n = pow_bounded(tn, p)?;
    let lx_d = pow_bounded(td, p)?;
    let rv_n = pow_bounded(vn, q)?;
    let rv_d = pow_bounded(vd, q)?;
    if lx_n != rv_n || lx_d != rv_d {
        return None;
    }
    Some(num2 / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    // D38<37>: storage MAX ≈ 17.01 (full i128 range), so bases 10..17 are
    // representable but their squares are not — the integer fast path's
    // `base^|n|` intermediate overflows storage even when the result fits.
    const S: u32 = 37;
    const ONE: i128 = 10_i128.pow(S);
    const M: RoundingMode = RoundingMode::HalfToEven;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    fn powf(base: i128, exp: i128) -> i128 {
        powf_strict_raw::<S>(base * ONE, exp * ONE, M).expect("in range")
    }

    #[test]
    fn in_range_reciprocal_with_overflowing_intermediate_computes() {
        // `10^-2 = 0.01` is representable, but `10² = 100` overflows i128
        // storage at scale 37. The fast path must defer to the exact pin,
        // not panic on the intermediate.
        assert_eq!(powf(10, -2), ONE / 100);
        assert_eq!(powf(5, -3), ONE / 125); // 1/125 = 0.008
        assert_eq!(powf(16, -2), ONE / 256); // 1/256 = 0.00390625
    }

    #[track_caller]
    fn check_directed_exact<const SC: u32>(base: i128, exp: i128, divisor: i128) {
        let one = 10_i128.pow(SC);
        for mode in MODES {
            assert_eq!(
                powf_strict_raw::<SC>(base * one, exp * one, mode),
                Some(one / divisor),
                "base={base} exp={exp} scale={SC} mode={mode:?}"
            );
        }
    }

    #[test]
    fn overflowing_intermediate_reciprocal_is_directed_exact() {
        // The defect: these reciprocals' scaled `base^|n|` intermediate
        // overflows i128, so the integer fast path returned `None` and
        // DEFERRED to the to-nearest `exp(y·ln x)` composition — which lands
        // ~1 ULP low, so `Floor` / `Trunc` rounded to the value one LSB BELOW
        // the exact power (e.g. `0.00999…9` for `0.01`). The exact integer pin
        // must return the on-grid value for EVERY mode, since each power lands
        // exactly on a storage grid line. Each base is representable at its
        // scale yet its scaled power overflows i128 (so the pin, not
        // `powi_raw_checked`, decides). Bases ≤ 17 at scale 37, larger at 36.
        check_directed_exact::<37>(10, -2, 100); // 0.01
        check_directed_exact::<37>(16, -2, 256); // 0.00390625
        check_directed_exact::<37>(4, -3, 64); // 0.015625
        check_directed_exact::<37>(5, -3, 125); // 0.008
        check_directed_exact::<36>(20, -2, 400); // 0.0025
        check_directed_exact::<36>(25, -2, 625); // 0.0016
        check_directed_exact::<36>(25, -3, 15_625); // 0.000064
    }

    #[test]
    fn exact_small_powers_unchanged() {
        // Cases the fast path always handled — must stay bit-identical.
        assert_eq!(powf(2, 3), 8 * ONE);
        assert_eq!(powf(2, -3), ONE / 8); // 0.125
        assert_eq!(powf(4, -2), ONE / 16); // 0.0625
        assert_eq!(powf(17, 1), 17 * ONE); // at the in-range edge
    }

    // Near-tie pins (see `trig_series_2limb::near_tie_pins`): exact
    // algebraic powers reached through the exp(y·ln x) composition. The
    // composition's value lands a kernel-error hair OFF the exact grid
    // value, so a directed mode stepped 1 LSB off the exact power
    // (failing-first). The fixed terminal walks the near-tie and the
    // walker's at-cap grid snap restores the exact value. Oracle: exact
    // integer arithmetic (4^(1/2) = 2, 225^(1/2) = 15, 2.25^(1/2) = 1.5).
    #[test]
    fn sqrt_like_exact_powers_are_directed_exact() {
        let one19 = 10_i128.pow(19);
        let half = 5 * 10_i128.pow(18); // 0.5 at scale 19
        for mode in MODES {
            assert_eq!(
                powf_strict_raw::<19>(4 * one19, half, mode),
                Some(2 * one19),
                "powf(4, 0.5) mode={mode:?}"
            );
            assert_eq!(
                powf_strict_raw::<19>(225 * one19, half, mode),
                Some(15 * one19),
                "powf(225, 0.5) mode={mode:?}"
            );
            assert_eq!(
                powf_strict_raw::<19>(225 * one19 / 100, half, mode),
                Some(15 * one19 / 10),
                "powf(2.25, 0.5) mode={mode:?}"
            );
        }
    }

    #[test]
    fn out_of_range_positive_power_signals_none() {
        // `10² = 100` exceeds the storage MAX (≈17) — the kernel must
        // signal `None` (the policy dispatch wrapper turns it into the
        // default form's panic; the `checked_` surface propagates it),
        // never a wrapped value.
        assert_eq!(powf_strict_raw::<S>(10 * ONE, 2 * ONE, M), None);
    }
}
