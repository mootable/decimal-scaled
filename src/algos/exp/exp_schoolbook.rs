// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Schoolbook exponential — direct Maclaurin series reference.
//!
//! `exp_schoolbook` is the naive textbook implementation of `eˣ`:
//!
//! 1. **Range-reduce**: write `x = k·ln(2) + s` with `|s| ≤ ln(2)/2`
//!    using the pre-embedded `ln(2)` constant.  This keeps the argument
//!    to the series small enough for the Maclaurin series to converge
//!    within a bounded number of terms at the working precision.
//! 2. **Direct Maclaurin series**: compute
//!    `exp(s) = 1 + s + s²/2! + s³/3! + …`
//!    term-by-term, dividing each accumulated term by the loop counter
//!    `i`, until the contribution rounds to zero at the working precision.
//!    No Smith/Brent argument-halving squarings — the loop runs to
//!    convergence on the range-reduced argument directly.
//! 3. **Reconstruct**: `exp(x) = 2^k · exp(s)`, applied as a bit-shift on
//!    the `Fixed` magnitude (no floating-point, no libm).
//!
//! All integer work uses the `Fixed` 256-bit sign-magnitude work type,
//! whose multiply and divide dispatch down to `Int<N>` arithmetic.
//! The `ln(2)` constant is sourced from
//! [`crate::algos::ln::ln_series_2limb`], which embeds a 75-digit
//! reference.
//!
//! ## Correctness
//!
//! `SCHOOLBOOK_GUARD = 30` matches the strict-series guard so the working
//! precision is identical and the result is correctly-rounded (delta = 0)
//! for all tiers the narrow `Fixed` intermediate covers.
//!
//! ## Scope
//!
//! Registered as the unrouted `Algorithm::Schoolbook` variant in
//! [`crate::policy::exp`].  `select` never returns it — production
//! traffic never reaches this kernel.

use crate::algos::ln::ln_series_2limb::wide_ln2;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Guard digits for the schoolbook path — matches `STRICT_GUARD` in
/// `exp_series_2limb` so the working precision is identical.
pub(crate) const SCHOOLBOOK_GUARD: u32 = 30;

/// `eˣ` via direct Maclaurin series on the 256-bit `Fixed` intermediate,
/// returned at working scale `w`.
///
/// Range-reduces `x = k·ln(2) + s` with `|s| ≤ ln(2)/2`, evaluates
/// `exp(s) = Σ sⁱ/i!` term-by-term until terms vanish at scale `w`, then
/// reconstructs `exp(x) = 2^k · exp(s)` by shifting the `Fixed` magnitude.
/// No Smith squarings; the loop terminates by convergence on the
/// range-reduced argument.
///
/// # Panics
///
/// Panics if the reconstructed value overflows a 256-bit `Fixed`.
pub(crate) fn exp_schoolbook_fixed(v_w: Fixed, w: u32) -> Fixed {
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let ln2 = wide_ln2(w);

    // Range reduction: k = round(v / ln 2); s = v - k·ln(2), |s| <= ln(2)/2.
    let k = v_w.div(ln2, w).round_to_nearest_int(w);
    let k_ln2 = if k >= 0 {
        ln2.mul_u128(k as u128)
    } else {
        ln2.mul_u128((-k) as u128).neg()
    };
    let s = v_w.sub(k_ln2);

    // Direct Maclaurin series: exp(s) = 1 + s + s²/2! + s³/3! + …
    // term[i] = s^i / i!  is computed iteratively as  term[i] = term[i-1]*s/i.
    let mut sum = one_w.add(s); // 1 + s (i=0 and i=1 terms)
    let mut term = s; // term[1] = s
    let mut i: u128 = 2;
    loop {
        term = term.mul(s, w).div_small(i);
        if term.is_zero() {
            break;
        }
        sum = sum.add(term);
        i += 1;
        // Safety cap — at w=68 the loop needs ~110 iterations for |s|<=ln2/2.
        // 300 is far above any reachable working scale, so this is defensive.
        if i > 300 {
            break;
        }
    }

    // Reconstruct exp(x) = 2^k · exp(s).
    if k >= 0 {
        let shift = k as u32;
        assert!(
            sum.bit_length() + shift <= 256,
            "exp_schoolbook: result overflows the representable range"
        );
        sum.shl(shift)
    } else {
        sum.shr((-k) as u32)
    }
}

// ── Wide tier — generic over the tier core `C: WideTrigCore` ─────────

/// Schoolbook `eˣ` for a wide tier — direct Maclaurin series via the
/// range-reduced leaf [`WideTrigCore::exp_fixed`] (the wide-tier
/// realisation of the `1 + s + s²/2! + …` textbook series with the Smith
/// `r/2^n` reduction), rounded correctly with Ziv escalation. Mirrors
/// `exp_series`; registered as the unrouted `Schoolbook` arm of the wide
/// `policy::exp` tiers and a correctness/microbench reference.
#[inline]
#[must_use]
pub(crate) fn exp_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }
    // `exp(x)` for `x != 0` is transcendental — never exactly on a storage grid
    // line (`raw == 0` is pinned above) — so a zero working residual is a
    // sub-resolution artifact; the never-exact narrowing keeps Ceiling correct
    // on inputs whose deciding residual is below the work-int resolution.
    C::round_to_storage_directed_never_exact(C::GUARD, SCALE, mode, &mut |guard| {
        C::exp_fixed(C::to_work_w(raw, guard), SCALE + guard)
    })
}

/// `D38` schoolbook `eˣ` with explicit working digits and rounding mode.
///
/// Accepts raw `Int<2>` storage at `scale`, evaluates in a `Fixed`
/// intermediate at `w = scale + working_digits`, and rounds back to `scale`.
#[allow(dead_code)]
pub(crate) fn exp_schoolbook_with(
    raw: Int<2>,
    scale: u32,
    working_digits: u32,
    mode: RoundingMode,
) -> Int<2> {
    let raw_i = raw.as_i128();
    if raw_i == 0 {
        return Int::<2>::from_i128(10_i128.pow(scale));
    }
    let w = scale + working_digits;
    let negative_input = raw_i < 0;
    let v_w = Fixed::from_u128_mag(raw_i.unsigned_abs(), false)
        .mul_u128(10u128.pow(working_digits));
    let v_w = if negative_input { v_w.neg() } else { v_w };
    Int::<2>::from_i128(
        exp_schoolbook_fixed(v_w, w)
            .round_to_i128_with(w, scale, mode)
            .unwrap_or_else(|| {
                crate::support::diagnostics::overflow_panic_with_scale(
                    "exp_schoolbook",
                    scale,
                )
            }),
    )
}

/// `D38` schoolbook `eˣ` (strict variant, fixed to `SCHOOLBOOK_GUARD`
/// working digits).
#[allow(dead_code)]
pub(crate) fn exp_schoolbook_strict<const SCALE: u32>(
    raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    exp_schoolbook_with(raw, SCALE, SCHOOLBOOK_GUARD, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::exp::exp_series_2limb::exp_strict;
    use crate::support::rounding::RoundingMode;
    use crate::int::types::Int;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven, RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero, RoundingMode::Trunc,
        RoundingMode::Floor, RoundingMode::Ceiling,
    ];

    #[track_caller]
    fn check<const S: u32>(raw_i: i128, mode: RoundingMode) {
        let raw = Int::<2>::from_i128(raw_i);
        let got = exp_schoolbook_strict::<S>(raw, mode);
        let expected = exp_strict::<S>(raw, mode);
        assert_eq!(got, expected,
            "exp schoolbook D38<{}> raw={} mode={:?}: {:?} != {:?}",
            S, raw_i, mode, got, expected);
    }

    #[test]
    fn exp_schoolbook_matches_series_d38_s12() {
        // Boundary: zero, near-1, ln(2), small, large, negative.
        for raw_i in [0_i128, 500_000_000_000, 1_000_000_000_000,
                      -500_000_000_000, 2_000_000_000_000, 693_147_180_560,
                      -1_000_000_000_000, 3_000_000_000_000, 100_000_000_000] {
            for mode in MODES { check::<12>(raw_i, mode); }
        }
    }

    #[test]
    fn exp_schoolbook_matches_series_d38_s19() {
        let one: i128 = 10_i128.pow(19);
        for raw_i in [0, one / 2, one, -(one / 2), 2 * one, -one,
                      one * 693_147_180 / 1_000_000_000] {
            for mode in MODES { check::<19>(raw_i, mode); }
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
        const INPUTS9: [i128; 7] = [
            0, 500_000_000, 1_000_000_000, -500_000_000,
            2_000_000_000, -1_000_000_000, 693_147_180,
        ];

        #[test]
        fn exp_schoolbook_matches_routed() {
            for &u in &INPUTS9 {
                let r = raw9(u);
                for mode in MODES {
                    assert_eq!(
                        crate::algos::exp::exp_schoolbook::exp_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).exp_strict_with(mode).0,
                        "D57 exp schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }
}
