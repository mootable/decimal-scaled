// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook natural logarithm — atanh series reference.
//!
//! `ln_schoolbook` is the naive textbook implementation of `ln(x)`:
//!
//! 1. **Exponent split**: write `x = 2^k · m` with `m ∈ [1, 2)` using
//!    the binary bit-length of the `Fixed` work value.  This keeps the
//!    series argument in a bounded range.
//! 2. **atanh series**: compute `ln(m) = 2·artanh(t)` where
//!    `t = (m−1)/(m+1) ∈ [0, 1/3)`:
//!
//!    ```text
//!    artanh(t) = t + t³/3 + t⁵/5 + …   (converges for |t| < 1)
//!    ```
//!
//!    Terms are computed iteratively: `term[k] = term[k-2] · t² / (2k+1)`
//!    until the contribution rounds to zero at working precision.
//! 3. **Reconstruct**: `ln(x) = k·ln(2) + ln(m)`, using the pre-embedded
//!    `ln(2)` constant from [`crate::algos::ln::ln_series_2limb`].
//!
//! All integer work uses the `Fixed` 256-bit sign-magnitude work type.
//! No floating-point, no libm.
//!
//! ## Correctness
//!
//! `SCHOOLBOOK_GUARD = 30` matches the strict-series guard so the working
//! precision is identical to the existing `ln_series_2limb` kernel.  The
//! result is correctly-rounded (delta = 0) for all tiers the narrow
//! `Fixed` intermediate covers.
//!
//! ## Scope
//!
//! Registered as the unrouted `Algorithm::Schoolbook` variant in
//! [`crate::policy::ln`].  `select` never returns it — production
//! traffic never reaches this kernel.

use crate::algos::ln::ln_series_2limb::wide_ln2;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, is_nearest_mode};

/// Guard digits for the schoolbook path — matches `STRICT_GUARD` in
/// `ln_series_2limb`.
pub(crate) const SCHOOLBOOK_GUARD: u32 = 30;

/// `ln(x)` via the atanh series on the 256-bit `Fixed` intermediate,
/// returned at `working_scale`.
///
/// Splits `x = 2^k · m` with `m ∈ [1, 2)`, evaluates
/// `ln(m) = 2·artanh((m−1)/(m+1))` term-by-term until terms vanish, then
/// returns `k·ln(2) + ln(m)`.  No Newton correction steps.
pub(crate) fn ln_schoolbook_fixed(working_value: Fixed, working_scale: u32) -> Fixed {
    let one_at_working_scale = Fixed { negative: false, mag: Fixed::pow10(working_scale) };
    let two_at_working_scale = one_at_working_scale.double();

    // Exponent split: find k such that 2^k <= v < 2^(k+1); mantissa_w = v / 2^k.
    let mut k: i32 =
        working_value.bit_length() as i32 - one_at_working_scale.bit_length() as i32;
    let mantissa_w = loop {
        let candidate_mantissa = if k >= 0 {
            working_value.shr(k as u32)
        } else {
            working_value.shl((-k) as u32)
        };
        if candidate_mantissa.ge_mag(two_at_working_scale) {
            k += 1;
        } else if !candidate_mantissa.ge_mag(one_at_working_scale) {
            k -= 1;
        } else {
            break candidate_mantissa;
        }
    };

    // t = (m - 1) / (m + 1) ∈ [0, 1/3); artanh(t) = t + t³/3 + t⁵/5 + …
    let atanh_arg = mantissa_w
        .sub(one_at_working_scale)
        .div(mantissa_w.add(one_at_working_scale), working_scale);
    let atanh_arg_sq = atanh_arg.mul(atanh_arg, working_scale);
    let mut sum = atanh_arg;
    let mut term = atanh_arg;
    let mut term_index: u128 = 1;
    loop {
        term = term.mul(atanh_arg_sq, working_scale);
        let contribution = term.div_small(2 * term_index + 1);
        if contribution.is_zero() {
            break;
        }
        sum = sum.add(contribution);
        term_index += 1;
        // Defensive cap — convergence for t<=1/3 at w=68 needs <80 steps.
        if term_index > 300 {
            break;
        }
    }
    let ln_mantissa = sum.double();

    let ln2 = wide_ln2(working_scale);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    k_ln2.add(ln_mantissa)
}

// ── Wide tier — generic over the tier core `C: WideTrigCore` ─────────

/// Schoolbook `ln(x)` for a wide tier — the atanh series with binary
/// exponent split via the leaf [`WideTrigCore::ln_fixed`] (the wide-tier
/// realisation of `ln(2^k·m) = k·ln2 + 2·artanh((m-1)/(m+1))`), rounded
/// correctly with Ziv escalation. Mirrors `ln_series`; registered as the
/// unrouted `Schoolbook` arm of the wide `policy::ln` tiers.
///
/// # Panics
///
/// Panics if `raw <= 0` (log of a non-positive value is undefined).
#[inline]
#[must_use]
pub(crate) fn ln_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw <= C::storage_zero() {
        panic!("wide-tier ln schoolbook: argument must be positive");
    }
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard_digits| {
        C::ln_fixed::<SCALE>(C::to_work_scaled(raw, guard_digits), SCALE + guard_digits)
    })
}

/// `D38` schoolbook `ln(x)` with explicit working digits and rounding mode.
#[allow(dead_code)]
pub(crate) fn ln_schoolbook_with(
    raw: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<2> {
    let raw_i128 = raw.as_i128();
    assert!(raw_i128 > 0, "ln_schoolbook: argument must be positive");
    let one_scaled: i128 = 10_i128.pow(scale);
    if raw_i128 == one_scaled {
        return Int::<2>::ZERO;
    }
    // Linear band fast-path: same as ln_series_2limb for nearest modes.
    let delta = raw_i128 - one_scaled;
    let ln1p_band: i128 = 10_i128.pow(scale.saturating_sub((scale + 1) >> 1));
    if delta.abs() <= ln1p_band && is_nearest_mode(mode) {
        return Int::<2>::from_i128(delta);
    }
    let working_scale = scale + working_digits;
    let working_value = Fixed::from_u128_mag(raw_i128 as u128, false)
        .mul_u128(10u128.pow(working_digits));
    Int::<2>::from_i128(
        ln_schoolbook_fixed(working_value, working_scale)
            .round_to_i128_with(working_scale, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale(
                    "ln_schoolbook",
                    scale,
                )
            }),
    )
}

/// `D38` schoolbook `ln(x)` (strict variant, fixed to `SCHOOLBOOK_GUARD`
/// working digits).
#[allow(dead_code)]
pub(crate) fn ln_schoolbook_strict<const SCALE: u32>(
    raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    ln_schoolbook_with(raw, SCALE, SCHOOLBOOK_GUARD, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::ln::ln_series_2limb::ln_strict;
    use crate::support::rounding::RoundingMode;
    use crate::int::types::Int;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Trunc,
        RoundingMode::Floor, RoundingMode::Ceiling,
        RoundingMode::AwayFromZero, RoundingMode::ZeroFiveUp,
    ];

    #[track_caller]
    fn check<const S: u32>(raw_i128: i128, mode: RoundingMode) {
        let raw = Int::<2>::from_i128(raw_i128);
        let got = ln_schoolbook_strict::<S>(raw, mode);
        let expected = ln_strict::<S>(raw, mode).expect("reference in range");
        assert_eq!(got, expected,
            "ln schoolbook D38<{}> raw={} mode={:?}: {:?} != {:?}",
            S, raw_i128, mode, got, expected);
    }

    #[test]
    fn ln_schoolbook_matches_series_d38_s12() {
        // Boundary: 1.0 (ln=0), 2, 0.5, e, 10, 3, near-1.
        for raw_i128 in [1_000_000_000_000_i128, 2_000_000_000_000, 500_000_000_000,
                         1_000_000_000, 10_000_000_000_000, 2_718_281_828_459,
                         1_100_000_000_000, 3_000_000_000_000, 100_000_000_000] {
            for mode in MODES { check::<12>(raw_i128, mode); }
        }
    }

    #[test]
    fn ln_schoolbook_matches_series_d38_s19() {
        let one: i128 = 10_i128.pow(19);
        for raw_i128 in [one, 2 * one, one / 2, 10 * one, 3 * one, one + one / 10] {
            for mode in MODES { check::<19>(raw_i128, mode); }
        }
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;
        use crate::D;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        // ln domain: positive only.
        const INPUTS9: [i128; 6] = [
            500_000_000, 1_000_000_000, 1_500_000_000,
            2_000_000_000, 3_000_000_000, 10_000_000_000,
        ];

        #[test]
        fn ln_schoolbook_matches_routed() {
            for &units in &INPUTS9 {
                let raw = raw9(units);
                for mode in MODES {
                    assert_eq!(
                        crate::algos::ln::ln_schoolbook::ln_schoolbook::<Core, S>(raw, mode),
                        D::<Int<3>, S>(raw).ln_strict_with(mode).0,
                        "D57 ln schoolbook != routed at units={units} mode={mode:?}"
                    );
                }
            }
        }
    }
}
