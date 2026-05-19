//! Trigonometric policy.
//!
//! Narrow tier (D9 / D18 / D38) covers forward + inverse + atan2 on
//! the `Fixed` 256-bit intermediate. Wide tier (D57 .. D1232) covers
//! the forward kernels (sin / cos / tan / atan) via per-tier kernels
//! in [`crate::algos::trig::wide_kernel`]; the inverse family
//! (asin / acos / atan2) for the wide tiers continues to delegate to
//! the macro-emitted inherent methods on the type, which compose
//! atan_fixed + sqrt_fixed + half_pi internally (no separate algos
//! kernel needed today).
//!
//! The wide-tier macro does not ship runtime-`working_digits` variants
//! of `sin_fixed` / `cos_fixed` / `tan_fixed` / `atan_fixed`, so the
//! `*_with_impl` methods for wide tiers ignore the caller-supplied
//! digits and delegate to the strict path (matching the precedent set
//! in [`crate::policy::ln`]).

use crate::algos::trig;
use crate::types::widths::{D9, D18, D38};
use crate::support::rounding::RoundingMode;

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

    // ── Hyperbolic family ──────────────────────────────────────────
    fn sinh_impl(self, mode: RoundingMode) -> Self;
    fn sinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn cosh_impl(self, mode: RoundingMode) -> Self;
    fn cosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn tanh_impl(self, mode: RoundingMode) -> Self;
    fn tanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn asinh_impl(self, mode: RoundingMode) -> Self;
    fn asinh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn acosh_impl(self, mode: RoundingMode) -> Self;
    fn acosh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn atanh_impl(self, mode: RoundingMode) -> Self;
    fn atanh_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;

    // ── Angle conversions ─────────────────────────────────────────
    fn to_degrees_impl(self, mode: RoundingMode) -> Self;
    fn to_degrees_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
    fn to_radians_impl(self, mode: RoundingMode) -> Self;
    fn to_radians_with_impl(self, working_digits: u32, mode: RoundingMode) -> Self;
}

/// Emits the narrow-tier `TrigPolicy` impl that widens to D38, calls
/// the D38 method, then narrows back. The forward family
/// (sin/cos/tan/atan/asin/acos/atan2) uses dedicated `widen_to_d38`
/// kernels; the hyperbolics and angle conversions widen via the same
/// `TryInto` shape the macro-emitted shells already use.
macro_rules! impl_narrow_trig {
    ($T:ident,
     $sin_s:path, $sin_w:path,
     $cos_s:path, $cos_w:path,
     $tan_s:path, $tan_w:path,
     $atan_s:path, $atan_w:path,
     $asin_s:path, $asin_w:path,
     $acos_s:path, $acos_w:path,
     $atan2_s:path, $atan2_w:path
    ) => {
        impl<const SCALE: u32> TrigPolicy for $T<SCALE> {
            #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { $sin_s(self, mode) }
            #[inline] fn sin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $sin_w(self, wd, mode) }
            #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { $cos_s(self, mode) }
            #[inline] fn cos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $cos_w(self, wd, mode) }
            #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { $tan_s(self, mode) }
            #[inline] fn tan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $tan_w(self, wd, mode) }
            #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { $atan_s(self, mode) }
            #[inline] fn atan_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $atan_w(self, wd, mode) }
            #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { $asin_s(self, mode) }
            #[inline] fn asin_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $asin_w(self, wd, mode) }
            #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { $acos_s(self, mode) }
            #[inline] fn acos_with_impl(self, wd: u32, mode: RoundingMode) -> Self { $acos_w(self, wd, mode) }
            #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { $atan2_s(self, other, mode) }
            #[inline] fn atan2_with_impl(self, other: Self, wd: u32, mode: RoundingMode) -> Self { $atan2_w(self, other, wd, mode) }

            // Hyperbolics and angle conversions widen → D38 → narrow.
            #[inline]
            fn sinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::sinh"), SCALE))
            }
            #[inline]
            fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.sinh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::sinh"), SCALE))
            }
            #[inline]
            fn cosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::cosh"), SCALE))
            }
            #[inline]
            fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.cosh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::cosh"), SCALE))
            }
            #[inline]
            fn tanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::tanh"), SCALE))
            }
            #[inline]
            fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.tanh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::tanh"), SCALE))
            }
            #[inline]
            fn asinh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::asinh"), SCALE))
            }
            #[inline]
            fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.asinh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::asinh"), SCALE))
            }
            #[inline]
            fn acosh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::acosh"), SCALE))
            }
            #[inline]
            fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.acosh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::acosh"), SCALE))
            }
            #[inline]
            fn atanh_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::atanh"), SCALE))
            }
            #[inline]
            fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.atanh_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::atanh"), SCALE))
            }
            #[inline]
            fn to_degrees_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::to_degrees"), SCALE))
            }
            #[inline]
            fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_degrees_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::to_degrees"), SCALE))
            }
            #[inline]
            fn to_radians_impl(self, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_strict_with(mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::to_radians"), SCALE))
            }
            #[inline]
            fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self {
                let wide: D38<SCALE> = self.into();
                ::core::convert::TryInto::try_into(wide.to_radians_approx_with(wd, mode))
                    .unwrap_or_else(|_| crate::support::diagnostics::overflow_panic_with_scale(
                        concat!(stringify!($T), "::to_radians"), SCALE))
            }
        }
    };
}

impl_narrow_trig!(
    D9,
    trig::widen_to_d38::sin_strict_d9, trig::widen_to_d38::sin_with_d9,
    trig::widen_to_d38::cos_strict_d9, trig::widen_to_d38::cos_with_d9,
    trig::widen_to_d38::tan_strict_d9, trig::widen_to_d38::tan_with_d9,
    trig::widen_to_d38::atan_strict_d9, trig::widen_to_d38::atan_with_d9,
    trig::widen_to_d38::asin_strict_d9, trig::widen_to_d38::asin_with_d9,
    trig::widen_to_d38::acos_strict_d9, trig::widen_to_d38::acos_with_d9,
    trig::widen_to_d38::atan2_strict_d9, trig::widen_to_d38::atan2_with_d9
);

impl_narrow_trig!(
    D18,
    trig::widen_to_d38::sin_strict_d18, trig::widen_to_d38::sin_with_d18,
    trig::widen_to_d38::cos_strict_d18, trig::widen_to_d38::cos_with_d18,
    trig::widen_to_d38::tan_strict_d18, trig::widen_to_d38::tan_with_d18,
    trig::widen_to_d38::atan_strict_d18, trig::widen_to_d38::atan_with_d18,
    trig::widen_to_d38::asin_strict_d18, trig::widen_to_d38::asin_with_d18,
    trig::widen_to_d38::acos_strict_d18, trig::widen_to_d38::acos_with_d18,
    trig::widen_to_d38::atan2_strict_d18, trig::widen_to_d38::atan2_with_d18
);

// D38 — see `crate::policy::ln` for the borrow-D57 rationale.
//
// When D57 is available, sin / cos / tan / atan / asin / acos / atan2
// all route through `borrow_d57`. The `_with` variants collapse to
// strict because the D57 wide_kernel has no runtime-`working_digits`
// path. `fixed_d38::*` is retained as an alternate kernel.

/// D38 hyperbolic + angle-conversion methods share one `Fixed` core
/// regardless of whether the forward trig path borrows D57 or runs
/// `fixed_d38`. Emit them once so both cfg branches stay short.
macro_rules! d38_hyperbolic_and_angle {
    () => {
        #[inline] fn sinh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::sinh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn sinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::sinh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn cosh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::cosh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn cosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::cosh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn tanh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::tanh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn tanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::tanh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn asinh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::asinh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn asinh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::asinh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn acosh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::acosh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn acosh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::acosh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn atanh_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atanh_strict::<SCALE>(self.0, mode)) }
        #[inline] fn atanh_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::atanh_with(self.0, SCALE, wd, mode)) }
        #[inline] fn to_degrees_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::to_degrees_strict::<SCALE>(self.0, mode)) }
        #[inline] fn to_degrees_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::to_degrees_with(self.0, SCALE, wd, mode)) }
        #[inline] fn to_radians_impl(self, mode: RoundingMode) -> Self { Self(trig::fixed_d38::to_radians_strict::<SCALE>(self.0, mode)) }
        #[inline] fn to_radians_with_impl(self, wd: u32, mode: RoundingMode) -> Self { Self(trig::fixed_d38::to_radians_with(self.0, SCALE, wd, mode)) }
    };
}

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::sin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::sin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::cos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::cos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::tan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::tan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::atan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::atan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::asin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::asin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::acos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::acos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { Self(trig::borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode)) }
    #[inline] fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d57::atan2_strict::<SCALE>(self.0, other.0, mode)) }

    d38_hyperbolic_and_angle!();
}

#[cfg(not(any(feature = "d57", feature = "wide")))]
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

    d38_hyperbolic_and_angle!();
}

// ── Wide tiers — width default ─────────────────────────────────────
//
// sin / cos / tan / atan route through `trig::wide_kernel`; the
// inverse family (asin / acos / atan2) delegates to the macro-emitted
// inherent `*_strict_with` methods on each `Dxx<SCALE>` (those compose
// the same per-tier `atan_fixed` / `sqrt_fixed` / `half_pi` primitives
// internally and aren't easier as free functions). `*_with_impl`
// ignores `working_digits` (see module docs).

/// Emits the `TrigPolicy` impl for one wide tier. `$T` is the typed
/// decimal ident (e.g. `D57`) prefixed inside with `crate::types::widths::`;
/// `$sin` / `$cos` / `$tan` / `$atan` are the corresponding
/// `trig::wide_kernel::*` paths.
macro_rules! impl_wide_trig {
    ($T:ident, $sin:path, $cos:path, $tan:path, $atan:path) => {
        impl<const SCALE: u32> TrigPolicy for crate::types::widths::$T<SCALE> {
            #[inline]
            fn sin_impl(self, mode: RoundingMode) -> Self {
                Self($sin(self.0, mode, SCALE))
            }
            #[inline]
            fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
                Self($sin(self.0, mode, SCALE))
            }
            #[inline]
            fn cos_impl(self, mode: RoundingMode) -> Self {
                Self($cos(self.0, mode, SCALE))
            }
            #[inline]
            fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
                Self($cos(self.0, mode, SCALE))
            }
            #[inline]
            fn tan_impl(self, mode: RoundingMode) -> Self {
                Self($tan(self.0, mode, SCALE))
            }
            #[inline]
            fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
                Self($tan(self.0, mode, SCALE))
            }
            #[inline]
            fn atan_impl(self, mode: RoundingMode) -> Self {
                Self($atan(self.0, mode, SCALE))
            }
            #[inline]
            fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
                Self($atan(self.0, mode, SCALE))
            }
            // Inverse family, hyperbolics, and angle conversions
            // delegate to the macro-emitted inherent `*_strict_with`
            // methods on the wide tier (those compose the per-tier
            // `atan_fixed` / `sqrt_fixed` / `half_pi` / `exp_fixed` /
            // `ln_fixed` primitives internally and aren't easier as
            // free functions).
            #[inline]
            fn asin_impl(self, mode: RoundingMode) -> Self { self.asin_strict_with(mode) }
            #[inline]
            fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.asin_strict_with(mode) }
            #[inline]
            fn acos_impl(self, mode: RoundingMode) -> Self { self.acos_strict_with(mode) }
            #[inline]
            fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.acos_strict_with(mode) }
            #[inline]
            fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { self.atan2_strict_with(other, mode) }
            #[inline]
            fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self { self.atan2_strict_with(other, mode) }

            #[inline]
            fn sinh_impl(self, mode: RoundingMode) -> Self { self.sinh_strict_with(mode) }
            #[inline]
            fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.sinh_strict_with(mode) }
            #[inline]
            fn cosh_impl(self, mode: RoundingMode) -> Self { self.cosh_strict_with(mode) }
            #[inline]
            fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.cosh_strict_with(mode) }
            #[inline]
            fn tanh_impl(self, mode: RoundingMode) -> Self { self.tanh_strict_with(mode) }
            #[inline]
            fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.tanh_strict_with(mode) }
            #[inline]
            fn asinh_impl(self, mode: RoundingMode) -> Self { self.asinh_strict_with(mode) }
            #[inline]
            fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.asinh_strict_with(mode) }
            #[inline]
            fn acosh_impl(self, mode: RoundingMode) -> Self { self.acosh_strict_with(mode) }
            #[inline]
            fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.acosh_strict_with(mode) }
            #[inline]
            fn atanh_impl(self, mode: RoundingMode) -> Self { self.atanh_strict_with(mode) }
            #[inline]
            fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.atanh_strict_with(mode) }

            #[inline]
            fn to_degrees_impl(self, mode: RoundingMode) -> Self { self.to_degrees_strict_with(mode) }
            #[inline]
            fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.to_degrees_strict_with(mode) }
            #[inline]
            fn to_radians_impl(self, mode: RoundingMode) -> Self { self.to_radians_strict_with(mode) }
            #[inline]
            fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.to_radians_strict_with(mode) }
        }
    };
}

// D57 — hand-rolled (not via `impl_wide_trig!`) so the `atan_impl`
// arm can route `SCALE ∈ 44..=56` through the bespoke
// `algos::trig::lookup_d57_s44_56_atan` kernel before falling back to
// the generic `wide_kernel::atan_strict_d57`. Lower scales (and every
// other forward op at every scale) still hit the wide kernel. Inverse
// family (asin / acos / atan2) continues to delegate to the macro-
// emitted inherent `*_strict_with` methods, matching the wide-tier
// shape `impl_wide_trig!` produces for the sibling tiers.

#[cfg(any(feature = "d57", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::types::widths::D57<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::sin_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::sin_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::cos_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::cos_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::wide_kernel::tan_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::wide_kernel::tan_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_atan::atan_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_atan::atan_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::atan_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 18..=22) {
            return Self(trig::lookup_d57_s18_22_atan::atan_strict::<SCALE>(self.0, mode));
        }
        if matches!(SCALE, 44..=56) {
            return Self(trig::lookup_d57_s44_56_atan::atan_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::atan_strict_d57(self.0, mode, SCALE))
    }
    #[inline]
    fn asin_impl(self, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.asin_strict_with(mode)
    }
    #[inline]
    fn acos_impl(self, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        self.acos_strict_with(mode)
    }
    #[inline]
    fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }
    #[inline]
    fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self {
        self.atan2_strict_with(other, mode)
    }

    // Hyperbolics + angle conversions — delegate to inherent shells.
    #[inline]
    fn sinh_impl(self, mode: RoundingMode) -> Self { self.sinh_strict_with(mode) }
    #[inline]
    fn sinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.sinh_strict_with(mode) }
    #[inline]
    fn cosh_impl(self, mode: RoundingMode) -> Self { self.cosh_strict_with(mode) }
    #[inline]
    fn cosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.cosh_strict_with(mode) }
    #[inline]
    fn tanh_impl(self, mode: RoundingMode) -> Self { self.tanh_strict_with(mode) }
    #[inline]
    fn tanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.tanh_strict_with(mode) }
    #[inline]
    fn asinh_impl(self, mode: RoundingMode) -> Self { self.asinh_strict_with(mode) }
    #[inline]
    fn asinh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.asinh_strict_with(mode) }
    #[inline]
    fn acosh_impl(self, mode: RoundingMode) -> Self { self.acosh_strict_with(mode) }
    #[inline]
    fn acosh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.acosh_strict_with(mode) }
    #[inline]
    fn atanh_impl(self, mode: RoundingMode) -> Self { self.atanh_strict_with(mode) }
    #[inline]
    fn atanh_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.atanh_strict_with(mode) }
    #[inline]
    fn to_degrees_impl(self, mode: RoundingMode) -> Self { self.to_degrees_strict_with(mode) }
    #[inline]
    fn to_degrees_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.to_degrees_strict_with(mode) }
    #[inline]
    fn to_radians_impl(self, mode: RoundingMode) -> Self { self.to_radians_strict_with(mode) }
    #[inline]
    fn to_radians_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { self.to_radians_strict_with(mode) }
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl_wide_trig!(
    D76,
    trig::wide_kernel::sin_strict_d76,
    trig::wide_kernel::cos_strict_d76,
    trig::wide_kernel::tan_strict_d76,
    trig::wide_kernel::atan_strict_d76
);

#[cfg(any(feature = "d115", feature = "wide"))]
impl_wide_trig!(
    D115,
    trig::wide_kernel::sin_strict_d115,
    trig::wide_kernel::cos_strict_d115,
    trig::wide_kernel::tan_strict_d115,
    trig::wide_kernel::atan_strict_d115
);

#[cfg(any(feature = "d153", feature = "wide"))]
impl_wide_trig!(
    D153,
    trig::wide_kernel::sin_strict_d153,
    trig::wide_kernel::cos_strict_d153,
    trig::wide_kernel::tan_strict_d153,
    trig::wide_kernel::atan_strict_d153
);

#[cfg(any(feature = "d230", feature = "wide"))]
impl_wide_trig!(
    D230,
    trig::wide_kernel::sin_strict_d230,
    trig::wide_kernel::cos_strict_d230,
    trig::wide_kernel::tan_strict_d230,
    trig::wide_kernel::atan_strict_d230
);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
impl_wide_trig!(
    D307,
    trig::wide_kernel::sin_strict_d307,
    trig::wide_kernel::cos_strict_d307,
    trig::wide_kernel::tan_strict_d307,
    trig::wide_kernel::atan_strict_d307
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
impl_wide_trig!(
    D462,
    trig::wide_kernel::sin_strict_d462,
    trig::wide_kernel::cos_strict_d462,
    trig::wide_kernel::tan_strict_d462,
    trig::wide_kernel::atan_strict_d462
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
impl_wide_trig!(
    D616,
    trig::wide_kernel::sin_strict_d616,
    trig::wide_kernel::cos_strict_d616,
    trig::wide_kernel::tan_strict_d616,
    trig::wide_kernel::atan_strict_d616
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl_wide_trig!(
    D924,
    trig::wide_kernel::sin_strict_d924,
    trig::wide_kernel::cos_strict_d924,
    trig::wide_kernel::tan_strict_d924,
    trig::wide_kernel::atan_strict_d924
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl_wide_trig!(
    D1232,
    trig::wide_kernel::sin_strict_d1232,
    trig::wide_kernel::cos_strict_d1232,
    trig::wide_kernel::tan_strict_d1232,
    trig::wide_kernel::atan_strict_d1232
);
