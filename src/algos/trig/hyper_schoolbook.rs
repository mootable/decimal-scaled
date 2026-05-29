//! Hyperbolic + inverse-hyperbolic schoolbook reference kernels.
//!
//! Naive textbook realisations of sinh / cosh / tanh / asinh / acosh /
//! atanh, registered as unrouted `Schoolbook` arms of the hyperbolic and
//! inverse-hyperbolic trig policies. Correctness reference + A/B
//! microbench partner; `select` never routes here. Each is the textbook
//! identity dispatched DOWN to the `Int<N>` work int. NEVER calls a
//! decimal `*_strict_with` on its own value. Identical composition +
//! narrowing as the routed kernel, so it matches bit-exactly.

use crate::algos::exp::exp_series_2limb::exp_fixed;
use crate::algos::ln::ln_series_2limb::{STRICT_GUARD, ln_fixed};
use crate::algos::support::fixed::Fixed;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::algos::trig::trig_series_2limb::to_fixed;
use crate::int::types::Int;
use crate::support::rounding::RoundingMode;

/// Schoolbook sinh for a wide tier -- (e^|x| - e^-|x|)/2 (odd).
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_w(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_w(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        let sh = C::sinh_pos_wide::<SCALE>(av, w);
        if neg { C::zero() - sh } else { sh }
    })
}

/// Schoolbook cosh for a wide tier -- (e^|x| + e^-|x|)/2 (even).
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let k_lift = C::exp_result_int_digits(C::to_work_w(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_w(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        C::cosh_pos_wide::<SCALE>(av, w)
    })
}

/// Schoolbook tanh for a wide tier -- (e^|x| - e^-|x|)/(e^|x| + e^-|x|).
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    let neg = raw < C::storage_zero();
    let k_lift = C::exp_result_int_digits(C::to_work_w(raw, 0), SCALE);
    let base_guard = C::GUARD + k_lift;
    C::round_to_storage_directed(base_guard, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let v = C::to_work_w(raw, guard);
        let av = if v < C::zero() { C::zero() - v } else { v };
        let th = C::tanh_pos_wide::<SCALE>(av, w);
        if neg { C::zero() - th } else { th }
    })
}

/// Schoolbook asinh for a wide tier -- ln(x + sqrt(x^2 + 1)) (odd).
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    if raw == C::storage_zero() {
        return C::storage_zero();
    }
    let neg = raw < C::storage_zero();
    C::round_to_storage_directed(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_w(raw, guard);
        let ax = if v < C::zero() { C::zero() - v } else { v };
        let inner = if ax >= one_w {
            let inv = C::div(one_w, ax, w);
            let root = C::sqrt_fixed(one_w + C::mul(inv, inv, w), w);
            C::ln_fixed::<SCALE>(ax, w) + C::ln_fixed::<SCALE>(one_w + root, w)
        } else {
            let root = C::sqrt_fixed(C::mul(ax, ax, w) + one_w, w);
            C::ln_fixed::<SCALE>(ax + root, w)
        };
        if neg { C::zero() - inner } else { inner }
    })
}

/// Schoolbook acosh for a wide tier -- ln(x + sqrt(x^2 - 1)), x >= 1.
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    {
        let w0 = SCALE + C::GUARD;
        if C::to_work(raw) < C::one(w0) {
            panic!("schoolbook acosh: argument must be >= 1");
        }
    }
    C::round_to_storage_directed_near_special(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_w(raw, guard);
        let two_w = one_w + one_w;
        if v >= two_w {
            let inv = C::div(one_w, v, w);
            let root = C::sqrt_fixed(one_w - C::mul(inv, inv, w), w);
            C::ln_fixed::<SCALE>(v, w) + C::ln_fixed::<SCALE>(one_w + root, w)
        } else {
            let t = v - one_w;
            let root = C::sqrt_fixed(C::mul(t, t + two_w, w), w);
            C::log1p_fixed(t + root, w)
        }
    })
}

/// Schoolbook atanh for a wide tier -- (1/2) ln((1+x)/(1-x)), |x| < 1.
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage {
    {
        let w0 = SCALE + C::GUARD;
        let v0 = C::to_work(raw);
        let ax0 = if v0 < C::zero() { C::zero() - v0 } else { v0 };
        if ax0 >= C::one(w0) {
            panic!("schoolbook atanh: argument out of domain (-1, 1)");
        }
    }
    C::round_to_storage_directed_near_special(C::GUARD, SCALE, mode, &mut |guard| {
        let w = SCALE + guard;
        let one_w = C::one(w);
        let v = C::to_work_w(raw, guard);
        (C::ln_fixed::<SCALE>(one_w + v, w) - C::ln_fixed::<SCALE>(one_w - v, w)) >> 1
    })
}

// -- Narrow tier -- Int<2> storage, math in the 256-bit Fixed ---------

#[inline]
fn one_fixed(w: u32) -> Fixed {
    Fixed { negative: false, mag: Fixed::pow10(w) }
}

#[inline]
#[must_use]
fn sinh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let neg = raw < 0;
    let av = Fixed { negative: false, mag: v.mag };
    let ex = exp_fixed(av, w);
    let one_w = one_fixed(w);
    let enx = one_w.div(ex, w);
    let sh = ex.sub(enx).halve();
    let sh = if neg { sh.neg() } else { sh };
    sh.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("sinh", SCALE))
}

#[inline]
#[must_use]
fn cosh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 10_i128.pow(SCALE);
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let av = Fixed { negative: false, mag: v.mag };
    let ex = exp_fixed(av, w);
    let one_w = one_fixed(w);
    let enx = one_w.div(ex, w);
    ex.add(enx)
        .halve()
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("cosh", SCALE))
}

#[inline]
#[must_use]
fn tanh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let v = to_fixed(raw);
    let neg = raw < 0;
    let av = Fixed { negative: false, mag: v.mag };
    let ex = exp_fixed(av, w);
    let one_w = one_fixed(w);
    let enx = one_w.div(ex, w);
    let th = ex.sub(enx).div(ex.add(enx), w);
    let th = if neg { th.neg() } else { th };
    th.round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("tanh", SCALE))
}

#[inline]
#[must_use]
fn asinh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    let ax = Fixed { negative: false, mag: v.mag };
    let inner = if ax.ge_mag(one_w) {
        let inv = one_w.div(ax, w);
        let root = one_w.add(inv.mul(inv, w)).sqrt(w);
        ln_fixed(ax, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = ax.mul(ax, w).add(one_w).sqrt(w);
        ln_fixed(ax.add(root), w)
    };
    let signed = if raw < 0 { inner.neg() } else { inner };
    signed
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("asinh", SCALE))
}

#[inline]
#[must_use]
fn acosh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    let one_bits: i128 = 10_i128.pow(SCALE);
    if raw == one_bits {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    assert!(!v.negative && v.ge_mag(one_w), "acosh: argument must be >= 1");
    let two_w = one_w.double();
    let inner = if v.ge_mag(two_w) {
        let inv = one_w.div(v, w);
        let root = one_w.sub(inv.mul(inv, w)).sqrt(w);
        ln_fixed(v, w).add(ln_fixed(one_w.add(root), w))
    } else {
        let root = v.mul(v, w).sub(one_w).sqrt(w);
        ln_fixed(v.add(root), w)
    };
    inner
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("acosh", SCALE))
}

#[inline]
#[must_use]
fn atanh_schoolbook_raw<const SCALE: u32>(raw: i128, mode: RoundingMode) -> i128 {
    if raw == 0 {
        return 0;
    }
    let w = SCALE + STRICT_GUARD;
    let one_w = one_fixed(w);
    let v = to_fixed(raw);
    let ax = Fixed { negative: false, mag: v.mag };
    assert!(!ax.ge_mag(one_w), "atanh: argument out of domain (-1, 1)");
    let ratio = one_w.add(v).div(one_w.sub(v), w);
    ln_fixed(ratio, w)
        .halve()
        .round_to_i128_with(w, SCALE, mode)
        .unwrap_or_else(|| crate::support::diagnostics::overflow_panic_with_scale("atanh", SCALE))
}

/// Narrow schoolbook sinh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn sinh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(sinh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook cosh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn cosh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(cosh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook tanh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn tanh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(tanh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook asinh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn asinh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(asinh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook acosh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn acosh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(acosh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

/// Narrow schoolbook atanh for Int<2> storage.
#[inline]
#[must_use]
pub(crate) fn atanh_schoolbook_narrow<const SCALE: u32>(raw: Int<2>, mode: RoundingMode) -> Int<2> {
    Int::<2>::from_i128(atanh_schoolbook_raw::<SCALE>(raw.as_i128(), mode))
}

// -- Unit tests: each schoolbook is bit-exact against the routed kernel.
//
// Reference correctness (skill 7): the schoolbook MUST produce the SAME
// storage raw as the routed kernel at every input, scale, tier and mode
// (delta == 0). A mismatch is a hard failure, never weakened.
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
    const HYP_INPUTS: [i128; 9] = [
        0,
        1_000_000_000,
        500_000_000_000,
        1_000_000_000_000,
        2_500_000_000_000,
        -1_000_000_000,
        -500_000_000_000,
        -1_000_000_000_000,
        -2_500_000_000_000,
    ];

    #[test]
    fn sinh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    sinh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).sinh_strict_with(mode).0,
                    "sinh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn cosh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    cosh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).cosh_strict_with(mode).0,
                    "cosh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn tanh_schoolbook_narrow_matches_routed_kernel() {
        for &raw in &HYP_INPUTS {
            for &mode in &MODES {
                assert_eq!(
                    tanh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).tanh_strict_with(mode).0,
                    "tanh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn asinh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 9] = [
            0,
            500_000_000_000,
            1_000_000_000_000,
            2_500_000_000_000,
            5_000_000_000_000,
            -500_000_000_000,
            -1_000_000_000_000,
            -2_500_000_000_000,
            -5_000_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    asinh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).asinh_strict_with(mode).0,
                    "asinh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn acosh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 5] = [
            1_000_000_000_000,
            1_200_000_000_000,
            2_000_000_000_000,
            3_000_000_000_000,
            5_000_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    acosh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).acosh_strict_with(mode).0,
                    "acosh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[test]
    fn atanh_schoolbook_narrow_matches_routed_kernel() {
        const PTS: [i128; 7] = [
            0,
            250_000_000_000,
            500_000_000_000,
            900_000_000_000,
            -250_000_000_000,
            -500_000_000_000,
            -900_000_000_000,
        ];
        for &raw in &PTS {
            for &mode in &MODES {
                assert_eq!(
                    atanh_schoolbook_narrow::<S38>(d38(raw).0, mode),
                    d38(raw).atanh_strict_with(mode).0,
                    "atanh schoolbook != routed at raw={raw} mode={mode:?}"
                );
            }
        }
    }

    #[cfg(any(feature = "d57", feature = "wide"))]
    mod wide_d57 {
        use super::*;
        use crate::types::widths::wide_trig_d57::Core;

        const S: u32 = 19;
        fn raw9(units: i128) -> Int<3> {
            Int::<3>::from_i128(units * 10_i128.pow(10))
        }

        #[test]
        fn forward_hyper_schoolbook_match_routed() {
            const INPUTS9: [i128; 7] = [
                0,
                1_000_000,
                500_000_000,
                1_000_000_000,
                2_500_000_000,
                -1_000_000_000,
                -2_500_000_000,
            ];
            for &u in &INPUTS9 {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        sinh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).sinh_strict_with(mode).0,
                        "D57 sinh schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        cosh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).cosh_strict_with(mode).0,
                        "D57 cosh schoolbook != routed at units={u} mode={mode:?}"
                    );
                    assert_eq!(
                        tanh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).tanh_strict_with(mode).0,
                        "D57 tanh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }

        #[test]
        fn inverse_hyper_schoolbook_match_routed() {
            const SINPUTS: [i128; 7] = [
                0,
                500_000_000,
                1_000_000_000,
                2_500_000_000,
                -500_000_000,
                -1_000_000_000,
                -2_500_000_000,
            ];
            for &u in &SINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        asinh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).asinh_strict_with(mode).0,
                        "D57 asinh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            const TINPUTS: [i128; 5] = [
                0,
                250_000_000,
                500_000_000,
                900_000_000,
                -500_000_000,
            ];
            for &u in &TINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        atanh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).atanh_strict_with(mode).0,
                        "D57 atanh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
            const AINPUTS: [i128; 4] = [
                1_000_000_000,
                1_200_000_000,
                2_000_000_000,
                3_000_000_000,
            ];
            for &u in &AINPUTS {
                let r = raw9(u);
                for &mode in &MODES {
                    assert_eq!(
                        acosh_schoolbook::<Core, S>(r, mode),
                        D::<Int<3>, S>(r).acosh_strict_with(mode).0,
                        "D57 acosh schoolbook != routed at units={u} mode={mode:?}"
                    );
                }
            }
        }
    }

}
