//! Exponential policy — narrow tier only (same scope rationale as
//! [`crate::policy::ln`]).

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
