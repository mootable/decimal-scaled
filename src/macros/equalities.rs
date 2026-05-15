//! Macro-generated `PartialEq` impls between a decimal type and the
//! primitive integer / float types.
//!
//! Each macro takes a target `$Type` (e.g. `D9`, `D18`, `D38`) and
//! emits a pair of `PartialEq` impls (both directions). Comparisons
//! against primitives go through `i128` arithmetic to keep one path
//! per source type regardless of the target's storage width.

/// Emits `PartialEq<$Src> for $Type<SCALE>` and the reciprocal direction
/// for any signed integer source type that fits in `i128`.
macro_rules! decl_eq_signed_int {
    ($Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                let m = Self::multiplier() as i128;
                let self_bits = self.to_bits() as i128;
                self_bits % m == 0 && self_bits / m == *other as i128
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for $Src {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
    // Wide storage: arithmetic stays in the wide integer; the quotient
    // narrows to `i128` for the primitive comparison.
    (wide $Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                let m = Self::multiplier();
                let bits = self.to_bits();
                let r = bits % m;
                if !r.is_zero() {
                    return false;
                }
                match (bits / m).to_i128_checked() {
                    ::core::option::Option::Some(v) => v == *other as i128,
                    ::core::option::Option::None => false,
                }
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

/// Emits `PartialEq<i128> for $Type<SCALE>` and the reciprocal. Has its
/// own definition because the destination integer math is already
/// i128 — no widening cast on the source.
macro_rules! decl_eq_i128 {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<i128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &i128) -> bool {
                let m = Self::multiplier() as i128;
                let self_bits = self.to_bits() as i128;
                self_bits % m == 0 && self_bits / m == *other
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for i128 {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
    (wide $Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<i128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &i128) -> bool {
                let m = Self::multiplier();
                let bits = self.to_bits();
                let r = bits % m;
                if !r.is_zero() {
                    return false;
                }
                match (bits / m).to_i128_checked() {
                    ::core::option::Option::Some(v) => v == *other,
                    ::core::option::Option::None => false,
                }
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
/// fit in `i128` (`u8` through `u64`, `usize`). A negative decimal
/// value is never equal to an unsigned primitive.
macro_rules! decl_eq_unsigned_int {
    ($Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                if self.to_bits() < 0 {
                    return false;
                }
                let m = Self::multiplier() as i128;
                let self_bits = self.to_bits() as i128;
                self_bits % m == 0 && self_bits / m == *other as i128
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for $Src {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
    (wide $Type:ident, $Src:ty) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Src> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &$Src) -> bool {
                let bits = self.to_bits();
                if bits.is_negative() {
                    return false;
                }
                let m = Self::multiplier();
                let r = bits % m;
                if !r.is_zero() {
                    return false;
                }
                match (bits / m).to_u128_checked() {
                    ::core::option::Option::Some(v) => v == *other as u128,
                    ::core::option::Option::None => false,
                }
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

/// Emits `PartialEq<u128> for $Type<SCALE>` and the reciprocal. u128
/// exceeds the i128 range, so we cast the storage's quotient to u128
/// after a sign-check.
macro_rules! decl_eq_u128 {
    ($Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<u128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &u128) -> bool {
                if self.to_bits() < 0 {
                    return false;
                }
                let m = Self::multiplier() as i128;
                let self_bits = self.to_bits() as i128;
                if self_bits % m != 0 {
                    return false;
                }
                (self_bits / m) as u128 == *other
            }
        }
        impl<const SCALE: u32> ::core::cmp::PartialEq<$Type<SCALE>> for u128 {
            #[inline]
            fn eq(&self, other: &$Type<SCALE>) -> bool {
                other == self
            }
        }
    };
    (wide $Type:ident) => {
        impl<const SCALE: u32> ::core::cmp::PartialEq<u128> for $Type<SCALE> {
            #[inline]
            fn eq(&self, other: &u128) -> bool {
                let bits = self.to_bits();
                if bits.is_negative() {
                    return false;
                }
                let m = Self::multiplier();
                let r = bits % m;
                if !r.is_zero() {
                    return false;
                }
                match (bits / m).to_u128_checked() {
                    ::core::option::Option::Some(v) => v == *other,
                    ::core::option::Option::None => false,
                }
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
/// and `f64`. Equality holds when the float is finite and round-trips
/// through `from_f64_lossy`/`to_f64_lossy` exactly. NaN and ±inf are
/// always unequal.
///
/// Only available when the lossy f64 bridge is present (i.e. not
/// in `--features strict` mode).
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
                let from_f = $Type::<SCALE>::from_f64_lossy(f);
                from_f.to_bits() == self.to_bits() && self.to_f64_lossy() == f
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
    (wide $Type:ident) => {
        $crate::macros::equalities::decl_eq_signed_int!(wide $Type, i8);
        $crate::macros::equalities::decl_eq_signed_int!(wide $Type, i16);
        $crate::macros::equalities::decl_eq_signed_int!(wide $Type, i32);
        $crate::macros::equalities::decl_eq_signed_int!(wide $Type, i64);
        $crate::macros::equalities::decl_eq_signed_int!(wide $Type, isize);
        $crate::macros::equalities::decl_eq_i128!(wide $Type);
        $crate::macros::equalities::decl_eq_unsigned_int!(wide $Type, u8);
        $crate::macros::equalities::decl_eq_unsigned_int!(wide $Type, u16);
        $crate::macros::equalities::decl_eq_unsigned_int!(wide $Type, u32);
        $crate::macros::equalities::decl_eq_unsigned_int!(wide $Type, u64);
        $crate::macros::equalities::decl_eq_unsigned_int!(wide $Type, usize);
        $crate::macros::equalities::decl_eq_u128!(wide $Type);
    };
}

pub(crate) use decl_eq_all_integers;
pub(crate) use decl_eq_i128;
pub(crate) use decl_eq_signed_int;
pub(crate) use decl_eq_u128;
pub(crate) use decl_eq_unsigned_int;

#[cfg(feature = "std")]
pub(crate) use decl_eq_float;
