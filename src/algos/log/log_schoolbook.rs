// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook base-`b` logarithm — naive `ln(x)/ln(b)` composition.
//!
//! `log_schoolbook` is the textbook formula:
//!
//! ```text
//! log_b(x) = ln(x) / ln(b)
//! ```
//!
//! Both `ln` calls use the narrow-tier schoolbook `ln` kernel from
//! [`crate::algos::ln::ln_schoolbook`], which evaluates the atanh series
//! in the 256-bit `Fixed` intermediate with `SCHOOLBOOK_GUARD` guard digits.
//! The division of the two `Fixed` results is performed at the same working
//! scale before rounding back to storage.
//!
//! ## Why not call `ln_with` on self
//!
//! Calling a dispatched method on a value of the same decimal type
//! re-enters the decimal policy (inversion — forbidden by the layering
//! law in `docs/ARCHITECTURE.md`).  Instead, `ln_schoolbook_fixed` is
//! called directly on the `Fixed` representations.
//!
//! ## Correctness
//!
//! Guard digits match `ln_schoolbook` (`SCHOOLBOOK_GUARD = 30`).  The
//! correctly-rounded unit test cross-checks against `log_ln_divide` (the
//! production kernel) and asserts `delta = 0` for all tested tiers.
//!
//! ## Scope
//!
//! Registered as the unrouted `Algorithm::Schoolbook` variant in
//! [`crate::policy::log`].  `select` never returns it.

use crate::algos::ln::ln_schoolbook::{SCHOOLBOOK_GUARD, ln_schoolbook_fixed};
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── Wide tier — generic over the tier core `C: WideTrigCore` ─────────

/// Schoolbook `log_b(x)` for a wide tier — the textbook quotient
/// `log_b(x) = ln(x) / ln(b)`, both natural logs evaluated by the leaf
/// [`WideTrigCore::ln_fixed`] in the guard-digit work integer and divided
/// via [`WideTrigCore::div`] before correctly-rounded narrowing with Ziv
/// escalation. Composes the C-generic `ln` leaf directly (no inversion).
/// Registered as the unrouted `Schoolbook` arm of the wide `policy::log`
/// tiers.
///
/// # Panics
///
/// Panics if `raw_x <= 0`, `raw_b <= 0`, or `b == 1` (undefined log).
#[inline]
#[must_use]
pub(crate) fn log_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw_x: C::Storage,
    raw_b: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw_x <= C::storage_zero() {
        panic!("wide-tier log schoolbook: x must be positive");
    }
    if raw_b <= C::storage_zero() {
        panic!("wide-tier log schoolbook: base must be positive");
    }
    if raw_b == C::storage_one(SCALE) {
        panic!("wide-tier log schoolbook: base must not be 1");
    }
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard_digits| {
        let working_scale = SCALE + guard_digits;
        let ln_x = C::ln_fixed::<SCALE>(C::to_work_scaled(raw_x, guard_digits), working_scale);
        let ln_b = C::ln_fixed::<SCALE>(C::to_work_scaled(raw_b, guard_digits), working_scale);
        C::div(ln_x, ln_b, working_scale)
    })
}

/// `log_b(x)` via naive `ln(x)/ln(b)` on the 256-bit `Fixed` intermediate.
///
/// Accepts raw `Int<2>` storage for `x` and `b` at `scale`, evaluates both
/// natural logs at `working_scale = scale + working_digits`, divides, and
/// rounds back to `scale`.
///
/// # Panics
///
/// Panics if `x <= 0` or `b <= 0` or `b == 1` (undefined log).
#[allow(dead_code)]
pub(crate) fn log_schoolbook_with(
    raw_x: Int<2>,
    raw_b: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<2> {
    let x_raw_i128 = raw_x.as_i128();
    let b_raw_i128 = raw_b.as_i128();
    assert!(x_raw_i128 > 0, "log_schoolbook: x must be positive");
    assert!(b_raw_i128 > 0, "log_schoolbook: base must be positive");
    let one_scaled = 10_i128.pow(scale);
    assert!(b_raw_i128 != one_scaled, "log_schoolbook: base must not be 1");

    // Exact-integer pin: if x == b^k exactly, the result is the integer k.
    // Derived from the nearest-rounded ln ratio; skip for non-integer bases.
    // (Avoids the ln(x)/ln(b) round-off bumping a directed mode by 1 LSB
    // at exact powers.)
    if x_raw_i128 % one_scaled == 0 && b_raw_i128 % one_scaled == 0 {
        let x_int = x_raw_i128 / one_scaled;
        let b_int = b_raw_i128 / one_scaled;
        if b_int >= 2 {
            // Try exponent = 1, 2, … up to log2(i128::MAX) ~127
            let mut power: i128 = b_int;
            let mut exponent: i128 = 1;
            while power <= x_raw_i128 / one_scaled {
                if power == x_int {
                    return Int::<2>::from_i128(exponent * one_scaled);
                }
                match power.checked_mul(b_int) {
                    Some(next_power) => power = next_power,
                    None => break,
                }
                exponent += 1;
            }
        }
    }

    let working_scale = scale + working_digits;
    let guard_pow = 10u128.pow(working_digits);

    // Lift both operands to working_scale.
    let x_working_value = Fixed::from_u128_mag(x_raw_i128 as u128, false).mul_u128(guard_pow);
    let b_working_value = Fixed::from_u128_mag(b_raw_i128 as u128, false).mul_u128(guard_pow);

    // Compute ln(x) and ln(b) at working_scale via the schoolbook atanh kernel.
    let ln_x = ln_schoolbook_fixed(x_working_value, working_scale);
    let ln_b = ln_schoolbook_fixed(b_working_value, working_scale);

    // log_b(x) = ln(x) / ln(b), rounded to storage scale.
    Int::<2>::from_i128(
        ln_x.div(ln_b, working_scale)
            .round_to_i128_with(working_scale, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale(
                    "log_schoolbook",
                    scale,
                )
            }),
    )
}

/// `D38` schoolbook `log_b(x)` (strict variant, fixed to `SCHOOLBOOK_GUARD`
/// working digits).
#[allow(dead_code)]
pub(crate) fn log_schoolbook_strict<const SCALE: u32>(
    raw_x: Int<2>,
    raw_b: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    log_schoolbook_with(raw_x, raw_b, SCALE, SCHOOLBOOK_GUARD, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::ln::ln_series_2limb::log;
    use crate::support::rounding::RoundingMode;
    use crate::int::types::Int;

    const MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Trunc,
        RoundingMode::Floor, RoundingMode::Ceiling,
        RoundingMode::AwayFromZero, RoundingMode::ZeroFiveUp,
    ];

    #[track_caller]
    fn check<const S: u32>(x_raw: i128, b_raw: i128, mode: RoundingMode) {
        let x_storage = Int::<2>::from_i128(x_raw);
        let b_storage = Int::<2>::from_i128(b_raw);
        let got = log_schoolbook_strict::<S>(x_storage, b_storage, mode);
        let expected = log::<S>(x_storage, b_storage, mode).expect("reference in range");
        assert_eq!(got, expected,
            "log schoolbook D38<{}> x={} b={} mode={:?}: {:?} != {:?}",
            S, x_raw, b_raw, mode, got, expected);
    }

    #[test]
    fn log_schoolbook_matches_log_ln_divide_d38_s12() {
        let one: i128 = 10_i128.pow(12);
        // (x, b) pairs: exact powers and non-integer results.
        let cases = [
            (2*one, 2*one), (4*one, 2*one), (8*one, 2*one),
            (10*one, 10*one), (3*one, 2*one), (one+one/2, 2*one),
        ];
        for (x_raw, b_raw) in cases {
            for mode in MODES { check::<12>(x_raw, b_raw, mode); }
        }
    }

    #[test]
    fn log_schoolbook_matches_log_ln_divide_d38_s19() {
        let one: i128 = 10_i128.pow(19);
        let cases = [
            (2*one, 2*one), (4*one, 2*one), (3*one, 2*one), (10*one, 10*one),
        ];
        for (x_raw, b_raw) in cases {
            for mode in MODES { check::<19>(x_raw, b_raw, mode); }
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
        // (x, b) pairs: exact powers + non-integer results, all positive, b != 1.
        const CASES: [(i128, i128); 5] = [
            (2_000_000_000, 2_000_000_000),
            (4_000_000_000, 2_000_000_000),
            (8_000_000_000, 2_000_000_000),
            (3_000_000_000, 2_000_000_000),
            (10_000_000_000, 10_000_000_000),
        ];

        #[test]
        fn log_schoolbook_matches_routed() {
            for &(x_units, base_units) in &CASES {
                let x_storage = raw9(x_units);
                let b_storage = raw9(base_units);
                for mode in MODES {
                    assert_eq!(
                        crate::algos::log::log_schoolbook::log_schoolbook::<Core, S>(
                            x_storage, b_storage, mode),
                        D::<Int<3>, S>(x_storage)
                            .log_with(D::<Int<3>, S>(b_storage), mode).0,
                        "D57 log schoolbook != routed at x={x_units} b={base_units} mode={mode:?}"
                    );
                }
            }
        }
    }
}
