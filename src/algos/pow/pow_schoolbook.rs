// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook floating-point power — naive `exp(y · ln(x))` composition.
//!
//! `pow_schoolbook` is the textbook formula for `x^y`:
//!
//! ```text
//! x^y = exp(y · ln(x))
//! ```
//!
//! All three stages — `ln(x)`, the multiplication `y · ln(x)`, and
//! `exp(…)` — are performed in the 256-bit `Fixed` guard-digit
//! intermediate at `w = SCALE + SCHOOLBOOK_GUARD` working digits,
//! using:
//! - [`crate::algos::ln::ln_schoolbook::ln_schoolbook_fixed`] for the
//!   natural log;
//! - [`crate::algos::exp::exp_schoolbook::exp_schoolbook_fixed`] for the
//!   exponential.
//!
//! No intermediate rounding between stages: the full working-precision
//! result is rounded to storage only once, after `exp`.
//!
//! ## Why not call dispatched methods
//!
//! Calling `x.ln_strict_with(…)` or `x.exp_strict_with(…)` on a value
//! of the same decimal type re-enters the decimal policy (the layering
//! inversion — forbidden by `docs/ARCHITECTURE.md`).  The schoolbook
//! kernels are called directly on `Fixed` representations instead.
//!
//! ## Correctness
//!
//! Guard digits match the schoolbook exp/ln (`SCHOOLBOOK_GUARD = 30`).
//! The correctly-rounded unit test cross-checks against `powf_series_2limb`
//! (the production kernel) and asserts `delta = 0`.  For non-positive
//! base values the kernel returns `0` (matching the production NaN-to-ZERO
//! policy).
//!
//! ## Scope
//!
//! Registered as the unrouted `Algorithm::Schoolbook` variant in
//! [`crate::policy::pow`].  `select` never returns it.

use crate::algos::exp::exp_schoolbook::{SCHOOLBOOK_GUARD, exp_schoolbook_fixed};
use crate::algos::ln::ln_schoolbook::ln_schoolbook_fixed;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

// ── Wide tier — generic over the tier core `C: WideTrigCore` ─────────

/// Schoolbook `x^y` for a wide tier — the textbook composition
/// `x^y = exp(y · ln(x))`, evaluated in the guard-digit work integer:
/// `ln(x)` via [`WideTrigCore::ln_fixed`], the product `y · ln(x)` via
/// [`WideTrigCore::mul`], and `exp(·)` via [`WideTrigCore::exp_fixed`],
/// then correctly-rounded narrowing with Ziv escalation. Composes the
/// C-generic `exp`/`ln` leaves directly (no inversion). Returns the
/// storage `0` for a non-positive base (the production NaN-to-ZERO
/// policy). A correctness/microbench reference; not policy-routed today.
#[inline]
#[must_use]
#[allow(dead_code)]
pub(crate) fn pow_schoolbook<C: WideTrigCore, const SCALE: u32>(
    base: C::Storage,
    exponent: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if base <= C::storage_zero() {
        return C::storage_zero();
    }
    if exponent == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let ln_base = C::ln_fixed::<SCALE>(C::to_work_w(base, guard), w);
        let arg = C::mul(C::to_work_w(exponent, guard), ln_base, w);
        C::exp_fixed(arg, w)
    })
}

/// `x^y` via naive `exp(y · ln(x))` on the 256-bit `Fixed` intermediate.
///
/// Accepts raw `Int<2>` storage for `base` and `exp` at `scale`, evaluates
/// `exp(exp · ln(base))` at working scale `w = scale + working_digits`, and
/// rounds the result back to `scale`.
///
/// Returns `0` for a non-positive `base` (matching the production NaN-to-ZERO
/// policy for bases where `ln` is undefined).
#[allow(dead_code)]
pub(crate) fn pow_schoolbook_with(
    base: Int<2>,
    exponent: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<2> {
    let base_i = base.as_i128();
    if base_i <= 0 {
        return Int::<2>::ZERO;
    }
    let exp_i = exponent.as_i128();
    let one_s: i128 = 10_i128.pow(scale);
    // base^0 == 1.
    if exp_i == 0 {
        return Int::<2>::from_i128(one_s);
    }

    let w = scale + working_digits;
    let guard_pow = 10u128.pow(working_digits);

    // Lift base to working scale w.
    let base_w = Fixed::from_u128_mag(base_i as u128, false).mul_u128(guard_pow);

    // Compute ln(base) at working scale w.
    let ln_base = ln_schoolbook_fixed(base_w, w);

    // Lift exponent to working scale w (preserving sign).
    let negative_exp = exp_i < 0;
    let exp_w = Fixed::from_u128_mag(exp_i.unsigned_abs(), false).mul_u128(guard_pow);
    let exp_w = if negative_exp { exp_w.neg() } else { exp_w };

    // Multiply: arg = y · ln(base) at working scale w.
    let arg = exp_w.mul(ln_base, w);

    // exp(arg) and round to storage.
    Int::<2>::from_i128(
        exp_schoolbook_fixed(arg, w)
            .round_to_i128_with(w, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale(
                    "pow_schoolbook",
                    scale,
                )
            }),
    )
}

/// `D38` schoolbook `base^exp` (strict variant, fixed to `SCHOOLBOOK_GUARD`
/// working digits).
#[allow(dead_code)]
pub(crate) fn pow_schoolbook_strict<const SCALE: u32>(
    base: Int<2>,
    exponent: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    pow_schoolbook_with(base, exponent, SCALE, SCHOOLBOOK_GUARD, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::pow::powf_series_2limb::powf_strict;
    use crate::support::rounding::RoundingMode;
    use crate::int::types::Int;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Trunc,
        RoundingMode::Floor, RoundingMode::Ceiling,
    ];

    #[track_caller]
    fn check<const S: u32>(base: i128, exp: i128, mode: RoundingMode) {
        let rb = Int::<2>::from_i128(base);
        let re = Int::<2>::from_i128(exp);
        let got = pow_schoolbook_strict::<S>(rb, re, mode);
        let expected = powf_strict::<S>(rb, re, mode);
        assert_eq!(got, expected,
            "pow schoolbook D38<{}> base={} exp={} mode={:?}: {:?} != {:?}",
            S, base, exp, mode, got, expected);
    }

    #[test]
    fn pow_schoolbook_matches_powf_series_d38_s12() {
        let one: i128 = 10_i128.pow(12);
        // Non-integer exponents: production integer fast-path does not fire.
        let cases = [
            (2*one, one/2), (2*one, 3*one/2), (3*one, one/2),
            (2*one, -(one/2)), (4*one, 3*one/4), (3*one/2, 5*one/2),
        ];
        for (b, e) in cases {
            for mode in MODES { check::<12>(b, e, mode); }
        }
    }

    #[test]
    fn pow_schoolbook_matches_powf_series_d38_s19() {
        let one: i128 = 10_i128.pow(19);
        let cases = [
            (2*one, one/2), (2*one, 3*one/2), (3*one, one/2),
        ];
        for (b, e) in cases {
            for mode in MODES { check::<19>(b, e, mode); }
        }
    }
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;
        use crate::D;

        const S: u32 = 19;
        fn raw9(units_milli: i128) -> Int<3> {
            // value = units_milli / 1000, expressed at scale 19.
            Int::<3>::from_i128(units_milli * 10_i128.pow(16))
        }
        // (base, exp) at scale-9 milli-units; non-integer exponents so the
        // production integer fast-path does not fire.
        const CASES: [(i128, i128); 5] = [
            (2_000, 500),       // 2 ^ 0.5
            (2_000, 1_500),     // 2 ^ 1.5
            (3_000, 500),       // 3 ^ 0.5
            (4_000, 750),       // 4 ^ 0.75
            (2_000, -500),      // 2 ^ -0.5
        ];

        #[test]
        fn pow_schoolbook_matches_routed() {
            for &(b, e) in &CASES {
                let rb = raw9(b);
                let re = raw9(e);
                for mode in MODES {
                    assert_eq!(
                        crate::algos::pow::pow_schoolbook::pow_schoolbook::<Core, S>(rb, re, mode),
                        D::<Int<3>, S>(rb).powf_strict_with(D::<Int<3>, S>(re), mode).0,
                        "D57 pow schoolbook != routed at base={b} exp={e} mode={mode:?}"
                    );
                }
            }
        }
    }
}
