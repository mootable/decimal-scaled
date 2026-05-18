//! Floating-point power policy — narrow tier only (same scope
//! rationale as [`crate::policy::ln`] / [`crate::policy::exp`]).

use crate::algos::pow;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

pub(crate) trait PowPolicy: Sized {
    /// `self^exp` (strict, const-folded `SCALE + STRICT_GUARD`).
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self;

    /// `self^exp` with caller-chosen working digits.
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

impl<const SCALE: u32> PowPolicy for D9<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_strict_d9(self, exp, mode)
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_with_d9(self, exp, working_digits, mode)
    }
}

impl<const SCALE: u32> PowPolicy for D18<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_strict_d18(self, exp, mode)
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        pow::widen_to_d38::powf_with_d18(self, exp, working_digits, mode)
    }
}

impl<const SCALE: u32> PowPolicy for D38<SCALE> {
    #[inline]
    fn powf_impl(self, exp: Self, mode: RoundingMode) -> Self {
        Self(pow::fixed_d38::powf_strict::<SCALE>(self.0, exp.0, mode))
    }
    #[inline]
    fn powf_with_impl(self, exp: Self, working_digits: u32, mode: RoundingMode) -> Self {
        Self(pow::fixed_d38::powf_with::<SCALE>(
            self.0,
            exp.0,
            working_digits,
            mode,
        ))
    }
}
