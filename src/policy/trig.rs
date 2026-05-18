//! Trigonometric policy — narrow tier only (D9 / D18 / D38).
//!
//! Covers sin / cos / tan and the inverse family
//! (atan / asin / acos / atan2). Wide tier remains macro-emitted
//! (same scope rationale as [`crate::policy::ln`]).

use crate::algos::trig;
use crate::core_type::{D9, D18, D38};
use crate::rounding::RoundingMode;

pub(crate) trait TrigPolicy: Sized {
    fn sin_impl(self, mode: RoundingMode) -> Self;
    fn sin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cos_impl(self, mode: RoundingMode) -> Self;
    fn cos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tan_impl(self, mode: RoundingMode) -> Self;
    fn tan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan_impl(self, mode: RoundingMode) -> Self;
    fn atan_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asin_impl(self, mode: RoundingMode) -> Self;
    fn asin_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acos_impl(self, mode: RoundingMode) -> Self;
    fn acos_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self;
    fn atan2_with_impl(self, other: Self, working_digits: u32, mode: RoundingMode) -> Self;
}

impl<const SCALE: u32> TrigPolicy for D9<SCALE> {
    #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::sin_strict_d9(self, mode) }
    #[inline] fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::sin_with_d9(self, wd, mode) }
    #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::cos_strict_d9(self, mode) }
    #[inline] fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::cos_with_d9(self, wd, mode) }
    #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::tan_strict_d9(self, mode) }
    #[inline] fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::tan_with_d9(self, wd, mode) }
    #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::atan_strict_d9(self, mode) }
    #[inline] fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::atan_with_d9(self, wd, mode) }
    #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::asin_strict_d9(self, mode) }
    #[inline] fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::asin_with_d9(self, wd, mode) }
    #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::acos_strict_d9(self, mode) }
    #[inline] fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::acos_with_d9(self, wd, mode) }
    #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { trig::widen_to_d38::atan2_strict_d9(self, other, mode) }
    #[inline] fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::atan2_with_d9(self, other, wd, mode) }
}

impl<const SCALE: u32> TrigPolicy for D18<SCALE> {
    #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::sin_strict_d18(self, mode) }
    #[inline] fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::sin_with_d18(self, wd, mode) }
    #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::cos_strict_d18(self, mode) }
    #[inline] fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::cos_with_d18(self, wd, mode) }
    #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::tan_strict_d18(self, mode) }
    #[inline] fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::tan_with_d18(self, wd, mode) }
    #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::atan_strict_d18(self, mode) }
    #[inline] fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::atan_with_d18(self, wd, mode) }
    #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::asin_strict_d18(self, mode) }
    #[inline] fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::asin_with_d18(self, wd, mode) }
    #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { trig::widen_to_d38::acos_strict_d18(self, mode) }
    #[inline] fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::acos_with_d18(self, wd, mode) }
    #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { trig::widen_to_d38::atan2_strict_d18(self, other, mode) }
    #[inline] fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self { trig::widen_to_d38::atan2_with_d18(self, other, wd, mode) }
}

impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::sin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::sin_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::cos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::cos_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::tan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::tan_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atan_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::asin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::asin_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::acos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::acos_with::<SCALE>(self.0, wd, mode)) }
    #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atan2_strict::<SCALE>(self.0, other.0, mode)) }
    #[inline] fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atan2_with::<SCALE>(self.0, other.0, wd, mode)) }
}
