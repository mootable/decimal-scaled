//! Trigonometric policy.
//!
//! Narrow tier (D9 / D18 / D38) covers forward + inverse + atan2 on
//! the `Fixed` 256-bit intermediate. Wide tier (D56 .. D1231) covers
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

// D38 — see `crate::policy::ln` for the borrow-D56 rationale.
//
// When D56 is available, sin / cos / tan / atan / asin / acos / atan2
// all route through `borrow_d56`. The `_with` variants collapse to
// strict because the D56 wide_kernel has no runtime-`working_digits`
// path. `fixed_d38::*` is retained as an alternate kernel.

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for D38<SCALE> {
    #[inline] fn sin_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::sin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::sin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn cos_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::cos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::cos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn tan_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::tan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::tan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::atan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::atan_strict::<SCALE>(self.0, mode)) }
    #[inline] fn asin_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::asin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn asin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::asin_strict::<SCALE>(self.0, mode)) }
    #[inline] fn acos_impl(self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::acos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn acos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::acos_strict::<SCALE>(self.0, mode)) }
    #[inline] fn atan2_impl(self, other: Self, mode: RoundingMode) -> Self { Self(trig::borrow_d56::atan2_strict::<SCALE>(self.0, other.0, mode)) }
    #[inline] fn atan2_with_impl(self, other: Self, _wd: u32, mode: RoundingMode) -> Self { Self(trig::borrow_d56::atan2_strict::<SCALE>(self.0, other.0, mode)) }
}

#[cfg(not(any(feature = "d56", feature = "wide")))]
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

// ── Wide tiers — width default ─────────────────────────────────────
//
// sin / cos / tan / atan route through `trig::wide_kernel`; the
// inverse family (asin / acos / atan2) delegates to the macro-emitted
// inherent `*_strict_with` methods on each `Dxx<SCALE>` (those compose
// the same per-tier `atan_fixed` / `sqrt_fixed` / `half_pi` primitives
// internally and aren't easier as free functions). `*_with_impl`
// ignores `working_digits` (see module docs).

/// Emits the `TrigPolicy` impl for one wide tier. `$T` is the typed
/// decimal ident (e.g. `D56`) prefixed inside with `crate::core_type::`;
/// `$sin` / `$cos` / `$tan` / `$atan` are the corresponding
/// `trig::wide_kernel::*` paths.
macro_rules! impl_wide_trig {
    ($T:ident, $sin:path, $cos:path, $tan:path, $atan:path) => {
        impl<const SCALE: u32> TrigPolicy for crate::core_type::$T<SCALE> {
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
            // Inverse family delegates to the inherent method shells —
            // wide-tier asin / acos / atan2 stay macro-emitted (no
            // separate algos kernel today).
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
        }
    };
}

// D56 — hand-rolled (not via `impl_wide_trig!`) so the `atan_impl`
// arm can route `SCALE ∈ 44..=57` through the bespoke
// `algos::trig::lookup_d56_s44_57_atan` kernel before falling back to
// the generic `wide_kernel::atan_strict_d56`. Lower scales (and every
// other forward op at every scale) still hit the wide kernel. Inverse
// family (asin / acos / atan2) continues to delegate to the macro-
// emitted inherent `*_strict_with` methods, matching the wide-tier
// shape `impl_wide_trig!` produces for the sibling tiers.

#[cfg(any(feature = "d56", feature = "wide"))]
impl<const SCALE: u32> TrigPolicy for crate::core_type::D56<SCALE> {
    #[inline]
    fn sin_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::sin_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn sin_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_sincos::sin_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::sin_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn cos_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::cos_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn cos_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_sincos::cos_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::cos_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn tan_impl(self, mode: RoundingMode) -> Self {
        Self(trig::wide_kernel::tan_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn tan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        Self(trig::wide_kernel::tan_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn atan_impl(self, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_atan::atan_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::atan_strict_d56(self.0, mode, SCALE))
    }
    #[inline]
    fn atan_with_impl(self, _wd: u32, mode: RoundingMode) -> Self {
        if matches!(SCALE, 44..=57) {
            return Self(trig::lookup_d56_s44_57_atan::atan_strict::<SCALE>(self.0, mode));
        }
        Self(trig::wide_kernel::atan_strict_d56(self.0, mode, SCALE))
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
}

#[cfg(any(feature = "d76", feature = "wide"))]
impl_wide_trig!(
    D76,
    trig::wide_kernel::sin_strict_d76,
    trig::wide_kernel::cos_strict_d76,
    trig::wide_kernel::tan_strict_d76,
    trig::wide_kernel::atan_strict_d76
);

#[cfg(any(feature = "d114", feature = "wide"))]
impl_wide_trig!(
    D114,
    trig::wide_kernel::sin_strict_d114,
    trig::wide_kernel::cos_strict_d114,
    trig::wide_kernel::tan_strict_d114,
    trig::wide_kernel::atan_strict_d114
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

#[cfg(any(feature = "d461", feature = "x-wide"))]
impl_wide_trig!(
    D461,
    trig::wide_kernel::sin_strict_d461,
    trig::wide_kernel::cos_strict_d461,
    trig::wide_kernel::tan_strict_d461,
    trig::wide_kernel::atan_strict_d461
);

#[cfg(any(feature = "d615", feature = "x-wide"))]
impl_wide_trig!(
    D615,
    trig::wide_kernel::sin_strict_d615,
    trig::wide_kernel::cos_strict_d615,
    trig::wide_kernel::tan_strict_d615,
    trig::wide_kernel::atan_strict_d615
);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
impl_wide_trig!(
    D923,
    trig::wide_kernel::sin_strict_d923,
    trig::wide_kernel::cos_strict_d923,
    trig::wide_kernel::tan_strict_d923,
    trig::wide_kernel::atan_strict_d923
);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
impl_wide_trig!(
    D1231,
    trig::wide_kernel::sin_strict_d1231,
    trig::wide_kernel::cos_strict_d1231,
    trig::wide_kernel::tan_strict_d1231,
    trig::wide_kernel::atan_strict_d1231
);
