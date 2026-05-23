//! Macro-generated `PartialEq` impls between a decimal type and the
//! primitive integer / float types.
//!
//! Each macro takes a target `$Type` (e.g. `D18`, `D38`) and emits a
//! pair of `PartialEq` impls (both directions).
//!
//! # Semantics — exact value equality, via the shared 1.3 comparator
//!
//! Both the integer and the float surface compute EXACT mathematical
//! value equality, riding the same const cross-scale comparator the
//! `D == D` impls use (`Int::cmp_cross_scaled` / `Int::cmp_f64_exact`).
//!
//! - **Integers**: a primitive `n` is the scale-0 value `n`, so `d == n`
//! compares the decimal's storage (scale `SCALE`) against `n` (scale 0)
//! with [`Int::cmp_cross_scaled`]. `n` is widened into an `Int<2>` (two
//! limbs hold any `i128` / `u128`); the comparator scales the decimal
//! *down* with remainder, so the result is exact and overflow-free.
//!
//! - **Floats**: `d == f` is EXACT value equality between `d`'s rational
//! value `bits / 10^SCALE` and `f`'s exact dyadic value `m · 2^e`,
//! cross-multiplied to integers by [`Int::cmp_f64_exact`]. This is
//! distinct from the lossy `TryFrom<f64>` / `to_f64` round-trip: e.g.
//! `D::from_str("1.1") == 1.1_f64` is `false`, because `1.1_f64` is not
//! exactly `1.1`. `NaN` and `±inf` always compare unequal.

/// Emits `PartialEq<$Src> for $Type<SCALE>` and the reciprocal direction
/// for any signed integer source type that fits in `i128`.
macro_rules! decl_eq_signed_int {
    // The storage tier is irrelevant: the comparator is cross-width, so a
    // single body serves every `$Type`.
    ($Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                let rhs = $crate::int::types::Int::<2>::from_i128(*other as i128);
                self.to_bits().cmp_cross_scaled(rhs, SCALE) == ::core::cmp::Ordering::Equal
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for $Src {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
}

/// Emits `PartialEq<i128> for $Type<SCALE>` and the reciprocal.
macro_rules! decl_eq_i128 {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<i128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &i128) -> bool {
                let rhs = $crate::int::types::Int::<2>::from_i128(*other);
                self.to_bits().cmp_cross_scaled(rhs, SCALE) == ::core::cmp::Ordering::Equal
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for i128 {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
}

/// Emits `PartialEq<$Src> for $Type<SCALE>` for unsigned sources that
/// fit in `u128` (`u8` through `u64`, `usize`).
macro_rules! decl_eq_unsigned_int {
    ($Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                let rhs = $crate::int::types::Int::<2>::from_u128(*other as u128);
                self.to_bits().cmp_cross_scaled(rhs, SCALE) == ::core::cmp::Ordering::Equal
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for $Src {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
}

/// Emits `PartialEq<u128> for $Type<SCALE>` and the reciprocal.
macro_rules! decl_eq_u128 {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<u128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &u128) -> bool {
                let rhs = $crate::int::types::Int::<2>::from_u128(*other);
                self.to_bits().cmp_cross_scaled(rhs, SCALE) == ::core::cmp::Ordering::Equal
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for u128 {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
}

/// Emits `PartialEq<$Src> for $Type<SCALE>` for float sources `f32`
/// and `f64`. EXACT value equality: the decimal's rational value is
/// compared against the float's exact dyadic value via
/// [`Int::cmp_f64_exact`]. `NaN` and `±inf` are always unequal. This is
/// NOT a lossy round-trip — see the module docs.
///
/// Only available with the `std` float bridge present.
#[cfg(feature = "std")]
macro_rules! decl_eq_float {
    ($Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                if !other.is_finite() {
                    return false;
                }
                let f = *other as f64;
                self.to_bits().cmp_f64_exact(SCALE, f) == ::core::cmp::Ordering::Equal
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for $Src {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
}


/// One-line invoker: emits the full signed/unsigned/i128/u128 cross-
/// equality surface for a decimal type.
macro_rules! decl_eq_all_integers {
    ($Type:ident) => {
        $crate::macros::equalities::decl_eq_signed_int!($Type, i8);
        $crate::macros::equalities::decl_eq_signed_int!($Type, i16);
        $crate::macros::equalities::decl_eq_signed_int!($Type, i32);
        $crate::macros::equalities::decl_eq_signed_int!($Type, i64);
        $crate::macros::equalities::decl_eq_signed_int!($Type, isize);
        $crate::macros::equalities::decl_eq_i128!($Type);
        $crate::macros::equalities::decl_eq_unsigned_int!($Type, u8);
        $crate::macros::equalities::decl_eq_unsigned_int!($Type, u16);
        $crate::macros::equalities::decl_eq_unsigned_int!($Type, u32);
        $crate::macros::equalities::decl_eq_unsigned_int!($Type, u64);
        $crate::macros::equalities::decl_eq_unsigned_int!($Type, usize);
        $crate::macros::equalities::decl_eq_u128!($Type);
    };
}

pub(crate) use decl_eq_all_integers;
pub(crate) use decl_eq_i128;
pub(crate) use decl_eq_signed_int;
pub(crate) use decl_eq_u128;
pub(crate) use decl_eq_unsigned_int;

#[cfg(feature = "std")]
pub(crate) use decl_eq_float;
