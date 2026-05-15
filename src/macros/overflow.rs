//! Macro-generated overflow-aware arithmetic variants for the decimal
//! widths that use a *uniform* mul/div pattern (D32, D64, and the wide
//! tier D256 / D512 / D1024).
//!
//! Emits the four standard families (`checked_*`, `wrapping_*`,
//! `saturating_*`, `overflowing_*`) for `add`, `sub`, `neg`, `mul`,
//! `div`, `rem`.
//!
//! Add / sub / neg / rem delegate to the storage type's `checked_*` /
//! `wrapping_*` / `saturating_*` / `overflowing_*` intrinsics, which
//! the wide integers expose with the same names and `const`-ness as the
//! primitive integers — so those families live in a shared `@common`
//! arm. Mul / div widen to `$Wider` for the intermediate; only the
//! widening *spelling* differs (native `as`-casts vs the `WideInt` cast),
//! so they are written inline per front-end arm.
//!
//! D128 is the exception: its overflow mul/div go through the
//! hand-rolled `mg_divide` path and are not generated here.

/// Emits overflow variants for a decimal type.
///
/// - `decl_decimal_overflow_variants!(D32, i32, i64)` — *native*
/// storage; `$Wider` is a primitive integer.
/// - `decl_decimal_overflow_variants!(wide D256, I256, I512)` — *wide*
/// storage; `$Wider` is the next size up.
macro_rules! decl_decimal_overflow_variants {
    // Wide storage.
    (wide $Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::overflow::decl_decimal_overflow_variants!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            // ----- mul (uses widening) ------------------------------

            #[inline]
            #[must_use]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let prod = a.checked_mul(b)?;
                let scaled = prod / m;
                let storage_max: $Wider = <$Storage>::MAX.resize::<$Wider>();
                let storage_min: $Wider = <$Storage>::MIN.resize::<$Wider>();
                if scaled > storage_max || scaled < storage_min {
                    None
                } else {
                    Some(Self(scaled.resize::<$Storage>()))
                }
            }

            #[inline]
            #[must_use]
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let prod = a.wrapping_mul(b);
                let scaled = prod / m;
                Self(scaled.resize::<$Storage>())
            }

            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                match self.checked_mul(rhs) {
                    Some(v) => v,
                    None => {
                        let neg_result =
                            self.0.is_negative() ^ rhs.0.is_negative();
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            #[inline]
            #[must_use]
            pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                match self.checked_mul(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_mul(rhs), true),
                }
            }

            // ----- div ----------------------------------------------

            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                if rhs == Self::ZERO {
                    return None;
                }
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let scaled_numer = a.checked_mul(m)?;
                let result = scaled_numer / b;
                let storage_max: $Wider = <$Storage>::MAX.resize::<$Wider>();
                let storage_min: $Wider = <$Storage>::MIN.resize::<$Wider>();
                if result > storage_max || result < storage_min {
                    None
                } else {
                    Some(Self(result.resize::<$Storage>()))
                }
            }

            #[inline]
            #[must_use]
            pub fn wrapping_div(self, rhs: Self) -> Self {
                let a: $Wider = self.0.resize::<$Wider>();
                let b: $Wider = rhs.0.resize::<$Wider>();
                let m: $Wider = $Type::<SCALE>::multiplier().resize::<$Wider>();
                let scaled_numer = a.wrapping_mul(m);
                let result = scaled_numer / b;
                Self(result.resize::<$Storage>())
            }

            #[inline]
            #[must_use]
            pub fn saturating_div(self, rhs: Self) -> Self {
                match self.checked_div(rhs) {
                    Some(v) => v,
                    None => {
                        let neg_result =
                            self.0.is_negative() ^ rhs.0.is_negative();
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            #[inline]
            #[must_use]
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                match self.checked_div(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_div(rhs), true),
                }
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::overflow::decl_decimal_overflow_variants!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            // ----- mul (uses widening) ------------------------------

            #[inline]
            #[must_use]
            pub fn checked_mul(self, rhs: Self) -> Option<Self> {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let prod = a.checked_mul(b)?;
                let scaled = prod / m;
                if scaled > <$Storage>::MAX as $Wider || scaled < <$Storage>::MIN as $Wider {
                    None
                } else {
                    Some(Self(scaled as $Storage))
                }
            }

            #[inline]
            #[must_use]
            pub fn wrapping_mul(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let prod = a.wrapping_mul(b);
                let scaled = prod / m;
                Self(scaled as $Storage)
            }

            #[inline]
            #[must_use]
            pub fn saturating_mul(self, rhs: Self) -> Self {
                match self.checked_mul(rhs) {
                    Some(v) => v,
                    None => {
                        // Sign of (a * b) is sign(a) XOR sign(b).
                        let neg_result = (self.0 < 0) ^ (rhs.0 < 0);
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            #[inline]
            #[must_use]
            pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                match self.checked_mul(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_mul(rhs), true),
                }
            }

            // ----- div ----------------------------------------------

            #[inline]
            #[must_use]
            pub fn checked_div(self, rhs: Self) -> Option<Self> {
                if rhs.0 == 0 {
                    return None;
                }
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let scaled_numer = a.checked_mul(m)?;
                let result = scaled_numer / b;
                if result > <$Storage>::MAX as $Wider || result < <$Storage>::MIN as $Wider {
                    None
                } else {
                    Some(Self(result as $Storage))
                }
            }

            #[inline]
            #[must_use]
            pub fn wrapping_div(self, rhs: Self) -> Self {
                let a = self.0 as $Wider;
                let b = rhs.0 as $Wider;
                let m = (10 as $Wider).pow(SCALE);
                let scaled_numer = a.wrapping_mul(m);
                let result = scaled_numer / b;
                Self(result as $Storage)
            }

            #[inline]
            #[must_use]
            pub fn saturating_div(self, rhs: Self) -> Self {
                match self.checked_div(rhs) {
                    Some(v) => v,
                    None => {
                        // Sign of (a / b) is sign(a) XOR sign(b).
                        let neg_result = (self.0 < 0) ^ (rhs.0 < 0);
                        if neg_result { Self::MIN } else { Self::MAX }
                    }
                }
            }

            #[inline]
            #[must_use]
            pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                match self.checked_div(rhs) {
                    Some(v) => (v, false),
                    None => (self.wrapping_div(rhs), true),
                }
            }
        }
    };

    // Shared: add / sub / neg / rem and their overflow families.
    // the wide integers expose these intrinsics with the same names and
    // `const`-ness as the primitive integers.
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            // ----- add ----------------------------------------------

            #[inline]
            #[must_use]
            pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                match self.0.checked_add(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            #[inline]
            #[must_use]
            pub const fn wrapping_add(self, rhs: Self) -> Self {
                Self(self.0.wrapping_add(rhs.0))
            }

            #[inline]
            #[must_use]
            pub const fn saturating_add(self, rhs: Self) -> Self {
                Self(self.0.saturating_add(rhs.0))
            }

            #[inline]
            #[must_use]
            pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_add(rhs.0);
                (Self(v), of)
            }

            // ----- sub ----------------------------------------------

            #[inline]
            #[must_use]
            pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                match self.0.checked_sub(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            #[inline]
            #[must_use]
            pub const fn wrapping_sub(self, rhs: Self) -> Self {
                Self(self.0.wrapping_sub(rhs.0))
            }

            #[inline]
            #[must_use]
            pub const fn saturating_sub(self, rhs: Self) -> Self {
                Self(self.0.saturating_sub(rhs.0))
            }

            #[inline]
            #[must_use]
            pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_sub(rhs.0);
                (Self(v), of)
            }

            // ----- neg ----------------------------------------------

            #[inline]
            #[must_use]
            pub const fn checked_neg(self) -> Option<Self> {
                match self.0.checked_neg() {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            #[inline]
            #[must_use]
            pub const fn wrapping_neg(self) -> Self {
                Self(self.0.wrapping_neg())
            }

            #[inline]
            #[must_use]
            pub const fn saturating_neg(self) -> Self {
                Self(self.0.saturating_neg())
            }

            #[inline]
            #[must_use]
            pub const fn overflowing_neg(self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_neg();
                (Self(v), of)
            }

            // ----- rem ----------------------------------------------

            #[inline]
            #[must_use]
            pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                match self.0.checked_rem(rhs.0) {
                    Some(v) => Some(Self(v)),
                    None => None,
                }
            }

            #[inline]
            #[must_use]
            pub const fn wrapping_rem(self, rhs: Self) -> Self {
                Self(self.0.wrapping_rem(rhs.0))
            }

            #[inline]
            #[must_use]
            pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                let (v, of) = self.0.overflowing_rem(rhs.0);
                (Self(v), of)
            }
        }
    };
}

pub(crate) use decl_decimal_overflow_variants;
