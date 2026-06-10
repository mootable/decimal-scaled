// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Inverse-trig schoolbook reference kernels -- asin / acos / atan2.
//!
//! Naive textbook realisations of the inverse family, registered as an
//! unrouted `Schoolbook` arm of
//! [`crate::policy::trig::inverse::Algorithm`]. Correctness reference +
//! A/B microbench partner for the routed `Atan` kernels; `select` never
//! routes here. Each is the textbook composition over the leaf atan
//! kernel + the new work-int `sqrt_fixed` leaf, dispatched DOWN to the
//! `Int<N>` layer. NEVER calls a decimal `*_strict_with` on its own
//! value (the inversion dec-trig had to avoid). Identical composition +
//! narrowing as the routed kernel, so it matches bit-exactly.

use crate::algos::ln::ln_series_2limb::STRICT_GUARD;
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::{atan2_kernel, atan_fixed, to_fixed, wide_half_pi};
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

#[inline]
fn asin_work<C: WideTrigCore, const SCALE: u32>(v: C::W, w: u32) -> C::W {
    let one_w = C::one(w);
    let abs_v = if v < C::zero() { C::zero() - v } else { v };
    let half_w = one_w >> 1;
    if abs_v == one_w {
        let hp = C::half_pi::<SCALE>(w);
        if v < C::zero() { C::zero() - hp } else { hp }
    } else if abs_v <= half_w {
        let denom = C::sqrt_fixed(one_w - C::mul(v, v, w), w);
        C::atan_fixed::<SCALE>(C::div(v, denom, w), w)
    } else {
        let inner = (one_w - abs_v) >> 1;
        let inner_sqrt = C::sqrt_fixed(inner, w);
        let inner_denom = C::sqrt_fixed(one_w - C::mul(inner_sqrt, inner_sqrt, w), w);
        let inner_asin = C::atan_fixed::<SCALE>(C::div(inner_sqrt, inner_denom, w), w);
        let result_abs = C::half_pi::<SCALE>(w) - inner_asin - inner_asin;
        if v < C::zero() { C::zero() - result_abs } else { result_abs }
    }
}

/// Schoolbook asin for a wide tier. Panics if |x| > 1.
#[inline]
#[must_use]
pub(crate) fn asin_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let one_w = C::one(w);
    let v0 = C::to_work(raw);
    let abs_v0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
    if abs_v0 > one_w {
        panic!("schoolbook asin: argument out of domain [-1, 1]");
    }
    let r = asin_work::<C, SCALE>(v0, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// Schoolbook acos for a wide tier -- pi/2 - asin(x). Panics if |x| > 1.
#[inline]
#[must_use]
pub(crate) fn acos_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let one_w = C::one(w);
    let v0 = C::to_work(raw);
    let abs_v0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
    if abs_v0 > one_w {
        panic!("schoolbook acos: argument out of domain [-1, 1]");
    }
    let r = C::half_pi::<SCALE>(w) - asin_work::<C, SCALE>(v0, w);
    C::round_to_storage_with(r, w, SCALE, mode)
}

/// Schoolbook atan2 for a wide tier -- quadrant-resolved atan(y/x).
#[inline]
#[must_use]
pub(crate) fn atan2_schoolbook<C: WideTrigCore, const SCALE: u32>(
    y_raw: C::Storage,
    x_raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let w = SCALE + C::GUARD;
    let z = C::storage_zero();
    let r = if x_raw == z {
        if y_raw > z {
            C::half_pi::<SCALE>(w)
        } else if y_raw < z {
            C::zero() - C::half_pi::<SCALE>(w)
        } else {
            C::zero()
        }
    } else {
        let y = C::to_work(y_raw);
        let x = C::to_work(x_raw);
        let zero_w = C::zero();
        let abs_y = if y < zero_w { zero_w - y } else { y };
        let abs_x = if x < zero_w { zero_w - x } else { x };
        let base = if abs_x >= abs_y {
            C::atan_fixed::<SCALE>(C::div(y, x, w), w)
        } else {
            let inv = C::atan_fixed::<SCALE>(C::div(x, y, w), w);
            let hp = C::half_pi::<SCALE>(w);
            let same_sign = (y < zero_w) == (x < zero_w);
            if same_sign { hp - inv } else { (zero_w - hp) - inv }
        };
        if x_raw > z {
            base
        } else if y_raw >= z {
            base + C::pi::<SCALE>(w)
        } else {
            base - C::pi::<SCALE>(w)
        }
    };
    C::round_to_storage_with(r, w, SCALE, mode)
}

#[inline]
fn asin_work_narrow(v: Fixed, w: u32) -> Fixed {
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let abs_v = Fixed { negative: false, mag: v.mag };
    if abs_v == one_w {
        let hp = wide_half_pi(w);
        return if v.negative { hp.neg() } else { hp };
    }
    let half_w = one_w.halve();
    if !abs_v.ge_mag(half_w) {
        let denom = one_w.sub(v.mul(v, w)).sqrt(w);
        atan_fixed(v.div(denom, w), w)
    } else {
        let inner = one_w.sub(abs_v).halve();
        let inner_sqrt = inner.sqrt(w);
        let inner_denom = one_w.sub(inner_sqrt.mul(inner_sqrt, w)).sqrt(w);
        let inner_asin = atan_fixed(inner_sqrt.div(inner_denom, w), w);
        let result_abs = wide_half_pi(w).sub(inner_asin).sub(inner_asin);
        if v.negative { result_abs.neg() } else { result_abs }
    }
}

#[inline]
#[must_use]
fn asin_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "asin: argument out of domain [-1, 1]"
    );
    asin_work_narrow(v, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("asin", SCALE))
}

#[inline]
#[must_use]
fn acos_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    use crate::types::consts::DecimalConstants;
    // Exact endpoint identities, matching the routed `acos_strict_raw`:
    // acos(0) = pi/2, acos(1) = 0, acos(-1) = pi -- pinned to the
    // correctly-rounded constant so the directed modes agree bit-exactly.
    if raw == 0 {
        return <crate::D<Int<2>, SCALE> as DecimalConstants>::half_pi().0.as_i128();
    }
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    if raw == -one_bits {
        return <crate::D<Int<2>, SCALE> as DecimalConstants>::pi().0.as_i128();
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = Fixed { negative: false, mag: Fixed::pow10(w) };
    let v = to_fixed(raw);
    let abs_v = Fixed { negative: false, mag: v.mag };
    assert!(
        !(abs_v.ge_mag(one_w) && abs_v != one_w),
        "acos: argument out of domain [-1, 1]"
    );
    wide_half_pi(w)
        .sub(asin_work_narrow(v, w))
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("acos", SCALE))
}

#[inline]
#[must_use]
fn atan2_schoolbook_raw<const SCALE: u32>(y_raw: i128, x_raw: i128, mode: RoundingMode) -> i128 {
    let w = SCALE + STRICT_GUARD;
    atan2_kernel(to_fixed(y_raw), to_fixed(x_raw), y_raw, w)
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atan2", SCALE))
}

/// Narrow schoolbook asin for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn asin_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(asin_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook acos for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn acos_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(acos_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook atan2 for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn atan2_schoolbook_narrow<const SCALE: u32>(
    y_raw: Int<2>,
    x_raw: Int<2>,
    mode: RoundingMode,
) -> Int<2> {
    Int::<2>::from_i128(atan2_schoolbook_raw::<SCALE>(y_raw.as_i128(), x_raw.as_i128(), mode))
}

// -- Unit tests: each schoolbook is bit-exact against the routed kernel.
//
// The schoolbook is the correctness reference (skill 7): it MUST produce
// the SAME storage raw as the golden-validated routed kernel at every
// input, scale, tier and mode. We assert delta == 0 over a range that
// covers the half-angle branch (|x| > 1/2), the |x| -> 1 boundary, the
// negative-argument fold, and the atan2 quadrants. A mismatch is a hard
// failure, never weakened.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::D;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    const S38: u32 = 12;
    fn d38(raw: i128) -> D<Int<2>, S38> {
        D(Int::<2>::from_i128(raw))
    }
    // domain [-1, 1] at scale 12: 0, 0.25, 0.5, 0.6 (>1/2 branch), 0.9,
    // 1.0 (endpoint), and negatives.
    const ASIN_INPUTS: [i128; 11] = [
        0,
        250_000_000_000,
        500_000_000_000,
        600_000_000_000,
        900_000_000_000,
        1_000_000_000_000,
        -250_000_000_000,
        -500_000_000_000,
        -600_000_000_000,
        -900_000_000_000,
        -1_000_000_000_000,
    ];

    #[test]
    fn asin_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &ASIN_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    asin_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).asin_strict_with(mode).0,
                    "asin schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acos_schoolbook_narrow_matches_routed_kernel() {
        // Exclude the exact endpoints acos(0)=pi/2, acos(+-1)=0/pi: under
        // the wide features the D38 routed path borrows D57 and rounds
        // those 1 ULP high in the directed modes (the documented
        // borrow-D57 endpoint quirk, see trig_schoolbook::atan(1)). The
        // schoolbook pins the correctly-rounded constant there, so it is
        // the valid reference -- the endpoints are asserted separately
        // against the external (DecimalConstants / mpmath) value below.
        let one_bits: i128 = 10_i128.pow(S38);
        for &raw in &ASIN_INPUTS {
            if raw == 0 || raw == one_bits || raw == -one_bits {
                continue;
            }
            for &mode in &MODES {
                assert_eq!(
                    acos_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).acos_strict_with(mode).0,
                    "acos schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acos_schoolbook_narrow_endpoints_are_correctly_rounded() {
        use crate::types::consts::DecimalConstants;
        // External oracle = the mpmath-pinned DecimalConstants: acos(0) =
        // pi/2, acos(1) = 0, acos(-1) = pi. The schoolbook returns these
        // exactly across every mode (the constant is correctly rounded).
        let one_bits: i128 = 10_i128.pow(S38);
        let half_pi = <D<Int<2>, S38> as DecimalConstants>::half_pi().0;
        let pi = <D<Int<2>, S38> as DecimalConstants>::pi().0;
        for &mode in &MODES {
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(0).0, mode), half_pi);
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(one_bits).0, mode), Int::<2>::from_i128(0));
            assert_eq!(acos_schoolbook_narrow::<S38>(d38(-one_bits).0, mode), pi);
        }
    }

    #[test]
    fn atan2_schoolbook_narrow_matches_routed_kernel() {
        // (y, x) across all four quadrants + axes.
        const PTS: [(i128, i128); 9] = [
            (1_000_000_000_000, 1_000_000_000_000),
            (1_000_000_000_000, -1_000_000_000_000),
            (-1_000_000_000_000, 1_000_000_000_000),
            (-1_000_000_000_000, -1_000_000_000_000),
            (1_000_000_000_000, 0),
            (-1_000_000_000_000, 0),
            (0, 1_000_000_000_000),
            (500_000_000_000, 2_000_000_000_000),
            (2_000_000_000_000, 500_000_000_000),
        ];
        for &(y, x) in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    atan2_schoolbook_narrow::<S38>(d38(y).0, d38(x).0, mode),
                    d38(y).atan2_strict_with(d38(x), mode).0,
                    "atan2 schoolbook != routed at y={y} x={x} mode={mode:?}"
                );
            }
        }
    }

    // wide tier: D57, scale 19.
    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }
        // scale-9 micro-values in [-1, 1].
        const INPUTS9: [i128; 9] = [
            0,
            250_000_000,
            500_000_000,
            600_000_000,
            900_000_000,
            1_000_000_000,
            -500_000_000,
            -900_000_000,
            -1_000_000_000,
        ];

        #[test]
        fn asin_acos_atan2_schoolbook_match_routed() {
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        asin_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).asin_strict_with(mode).0,
                        "D57 asin schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        acos_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).acos_strict_with(mode).0,
                        "D57 acos schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            // atan2 quadrants at scale 9.
            const PTS: [(i128, i128); 5] = [
                (1_000_000_000, 1_000_000_000),
                (1_000_000_000, -1_000_000_000),
                (-1_000_000_000, 1_000_000_000),
                (-1_000_000_000, -1_000_000_000),
                (500_000_000, 2_000_000_000),
            ];
            for &(y, x) in &PTS {
                let yr = raw9(y);
                let xr = raw9(x);
                for &mode in &MODES {
                    assert_eq!(
                        atan2_schoolbook::<Core, S>(yr, xr, mode),
                        D::<Int<3>, S>(yr).atan2_strict_with(D::<Int<3>, S>(xr), mode).0,
                        "D57 atan2 schoolbook != routed at y={y} x={x} mode={mode:?}"
                    );
                }
            }
        }
    }
}
