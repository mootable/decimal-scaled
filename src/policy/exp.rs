//! Exponential policy.
//!
//! Same cascade shape as [`crate::policy::ln`]: narrow tier on the
//! `Fixed` 256-bit intermediate, wide tier on per-tier `exp_strict`
//! kernels in [`crate::algos::exp::wide_kernel`]. The wide-tier macro
//! does not ship a runtime-`working_digits` variant of `exp_fixed`, so
//! [`ExpPolicy::exp_with_impl`] for wide tiers ignores the
//! caller-supplied digits and delegates to the strict path.

use crate::algos::exp;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

pub(crate) trait ExpPolicy: Sized {
    /// `e^self` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn exp_impl(self, mode: RoundingMode) -> Self;

    /// `e^self` with caller-chosen working digits.
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

impl<const SCALE: u32> ExpPolicy for D9<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        exp::widen_to_d38::exp_strict_d9(self, mode)
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        exp::widen_to_d38::exp_with_d9(self, working_digits, mode)
    }
}

impl<const SCALE: u32> ExpPolicy for D18<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        exp::widen_to_d38::exp_strict_d18(self, mode)
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        exp::widen_to_d38::exp_with_d18(self, working_digits, mode)
    }
}

// D38 — see `crate::policy::ln` for the borrow-D56 rationale.

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for D38<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::borrow_d56::exp_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::borrow_d56::exp_strict::<SCALE>(self.0, mode))
    }
}

#[cfg(not(any(feature = "d56", feature = "wide")))]
impl<const SCALE: u32> ExpPolicy for D38<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp_strict::<SCALE>(self.0, mode))
    }
    #[inline]
    fn exp_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::fixed_d38::exp_with(self.0, SCALE, working_digits, mode))
    }
}

// ── Wide tiers — width default: per-tier wide_kernel ────────────────
//
// `exp_with_impl` for wide tiers ignores `working_digits` and falls
// through to the strict path; see module-level docs for the rationale.

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D56<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 45..=57) {
            return Self(exp::lookup_d56_s45_57::exp_strict::<SCALE>(self.0, mode));
        }
        Self(exp::wide_kernel::exp_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 45..=57) {
            return Self(exp::lookup_d56_s45_57::exp_strict::<SCALE>(self.0, mode));
        }
        Self(exp::wide_kernel::exp_strict_d56(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D76<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d76(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d76(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d114", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D114<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d114(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d114(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d153", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D153<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d153(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d153(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d230", feature = "wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D230<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d230(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d230(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D307<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d307(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d307(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d461", feature = "x-wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D461<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d461(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d461(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d615", feature = "x-wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D615<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d615(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d615(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d923", feature = "xx-wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D923<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d923(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d923(self.0, mode, SCALE))
    }
}

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
impl<const SCALE: u32> ExpPolicy for crate::core_type::D1231<SCALE> {
    #[inline]
    fn exp_impl(self, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d1231(self.0, mode, SCALE))
    }
    #[inline]
    fn exp_with_impl(self, _working_digits: u32, mode: RoundingMode) -> Self {
        Self(exp::wide_kernel::exp_strict_d1231(self.0, mode, SCALE))
    }
}
