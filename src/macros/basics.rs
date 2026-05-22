//! Macro-generated impls for the *basics* of every decimal width.
//!
//! What this macro emits, for a given `(Type, Storage, MAX_SCALE)`:
//!
//! - The inherent constructor / accessor / multiplier methods.
//! - The `SCALE` associated const + `scale()` method.
//! - The `ZERO`, `ONE`, `MAX`, `MIN` constants.
//! - The [`crate::types::traits::Decimal`] trait impl, delegating to
//! the inherent surface.
//!
//! Invoke once per width (see `types/widths.rs` for the per-width
//! invocations).
//!
//! Two front-end arms exist:
//!
//! - `decl_decimal_basics!(D38, i128, 37)` — *native* storage. The
//!   third argument is `MAX_SCALE = name - 1` (the v0.4.0 cap). The
//! storage type is a primitive signed integer that supports the
//! `(10 as $Storage)` literal cast and a const-fn `pow`.
//! - `decl_decimal_basics!(wide D76, I256, 75)` — *wide* storage. The
//! storage type is a hand-rolled wide integer fixed-width integer, which has no `as`
//! cast from integer literals; the `10` and `0` constants are built
//! via the const-fn `from_str_radix` instead.
//!
//! Both arms forward to a shared `@impl` arm so the bulk of the surface
//! is defined exactly once.

macro_rules! decl_decimal_basics {
    // Wide storage: literal `10` / `0` are constructed via
    // the const-fn `from_str_radix` because the wide integers have no
    // `as`-cast from a primitive literal.
    (wide $Type:ident, $Storage:ty, $max_scale:literal) => {
        $crate::macros::basics::decl_decimal_basics! {
            @impl $Type, $Storage, $max_scale,
            multiplier = {
                match <$Storage>::from_str_radix("10", 10) {
                    ::core::result::Result::Ok(v) => v,
                    ::core::result::Result::Err(_) => {
                        panic!("wide decimal: invalid base-10 multiplier literal")
                    }
                }
                .pow(SCALE)
            },
            zero = {
                match <$Storage>::from_str_radix("0", 10) {
                    ::core::result::Result::Ok(v) => v,
                    ::core::result::Result::Err(_) => {
                        panic!("wide decimal: invalid zero literal")
                    }
                }
            },
            one_lsb = { <$Storage>::from_u128(1) }
        }
    };


    // Shared implementation body. `$mult` is the const expression for
    // `10^SCALE`; `$zero` is the const expression for the storage zero;
    // `$one_lsb` is the const expression for the smallest positive
    // storage value (`1` cast / constructed into the storage type).
    (@impl $Type:ident, $Storage:ty, $max_scale:literal,
     multiplier = { $mult:expr }, zero = { $zero:expr },
     one_lsb = { $one_lsb:expr }) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Constructs from a raw storage bit pattern.
            ///
            /// The integer is interpreted directly as the internal storage:
            /// `raw` represents the logical value `raw * 10^(-SCALE)`. This
            /// is the inverse of [`Self::to_bits`].
            ///
            /// # Precision
            ///
            /// Strict: all arithmetic is integer-only; result is bit-exact.
            #[inline]
            pub const fn from_bits(raw: $Storage) -> Self {
                Self(raw)
            }

            /// Returns the raw storage value.
            ///
            /// The returned integer encodes the logical value
            /// `self * 10^SCALE`. This is the inverse of [`Self::from_bits`].
            ///
            /// # Precision
            ///
            /// Strict: all arithmetic is integer-only; result is bit-exact.
            #[inline]
            pub const fn to_bits(self) -> $Storage {
                self.0
            }

            /// Returns `10^SCALE`, the factor that converts a logical
            /// integer value to its storage representation. Equals the
            /// bit pattern of [`Self::ONE`].
            ///
            /// # Precision
            ///
            /// Strict: all arithmetic is integer-only; result is bit-exact.
            ///
            /// # Overflow
            ///
            /// `10^SCALE` overflows the storage type at `SCALE > MAX_SCALE`.
            /// Calling with an overflowing scale panics at compile time
            /// when the const item is evaluated.
            #[inline]
            pub const fn multiplier() -> $Storage {
                $mult
            }

            /// The decimal scale of this type, equal to the `SCALE`
            /// const-generic parameter. One LSB of storage represents
            /// `10^-SCALE`. Use in type-level / const contexts; prefer
            /// [`Self::scale`] when an instance is in hand.
            pub const SCALE: u32 = SCALE;

            /// Returns the decimal scale of this value, equal to the
            /// `SCALE` const-generic parameter. The value is determined
            /// entirely by the type; the method exists for ergonomic
            /// method-call syntax.
            #[inline]
            pub const fn scale(self) -> u32 {
                SCALE
            }

            /// The additive identity. Stored as zero bits.
            ///
            /// # Precision
            ///
            /// N/A: constant value, no arithmetic performed.
            pub const ZERO: Self = Self($zero);

            /// The multiplicative identity. Stored as `10^SCALE` bits.
            ///
            /// # Precision
            ///
            /// N/A: constant value, no arithmetic performed.
            pub const ONE: Self = Self(Self::multiplier());

            /// The largest representable value: the storage type's `MAX`.
            ///
            /// Arithmetic that overflows this bound panics in debug
            /// builds and wraps in release builds.
            pub const MAX: Self = Self(<$Storage>::MAX);

            /// The smallest representable value: the storage type's `MIN`.
            ///
            /// Mirror of [`Self::MAX`]. Note that `-MIN` panics in debug
            /// builds because two's-complement `MIN` has no positive
            /// counterpart.
            pub const MIN: Self = Self(<$Storage>::MIN);

            /// Smallest representable positive value: 1 LSB = `10^-SCALE`.
            ///
            /// Provided as an analogue to `f64::EPSILON` for generic
            /// numeric code that wants the smallest non-zero positive
            /// step. Differs from the f64 definition ("difference
            /// between 1.0 and the next-larger f64"): on a
            /// fixed-scale decimal the LSB is uniform across the
            /// representable range. There are no subnormals.
            ///
            /// Useful when you need a "smallest positive step" value
            /// without writing `Self::from_bits(<storage>::from_u128(1))`
            /// out longhand — particularly with wide-tier storage
            /// where the literal `1` isn't directly the wide-int type.
            pub const EPSILON: Self = Self($one_lsb);

            /// Smallest positive value (equal to [`Self::EPSILON`]).
            ///
            /// Provided as an analogue to `f64::MIN_POSITIVE` for
            /// generic numeric code. Unlike `f64`, fixed-scale decimal
            /// types have no subnormals, so `MIN_POSITIVE` and
            /// `EPSILON` are the same value.
            pub const MIN_POSITIVE: Self = Self($one_lsb);
        }

        impl<const SCALE: u32> $crate::types::traits::arithmetic::DecimalArithmetic
            for $Type<SCALE>
        {
            type Storage = $Storage;

            const SCALE: u32 = SCALE;
            const MAX_SCALE: u32 = $max_scale;
            const ZERO: Self = $Type::<SCALE>::ZERO;
            const ONE: Self = $Type::<SCALE>::ONE;
            const MAX: Self = $Type::<SCALE>::MAX;
            const MIN: Self = $Type::<SCALE>::MIN;

            #[inline]
            fn multiplier() -> $Storage {
                $Type::<SCALE>::multiplier()
            }

            // Sign — delegate to inherent.
            #[inline]
            fn abs(self) -> Self {
                $Type::<SCALE>::abs(self)
            }
            #[inline]
            fn signum(self) -> Self {
                $Type::<SCALE>::signum(self)
            }
            #[inline]
            fn is_positive(self) -> bool {
                $Type::<SCALE>::is_positive(self)
            }
            #[inline]
            fn is_negative(self) -> bool {
                $Type::<SCALE>::is_negative(self)
            }

            // Integer-shape predicates.
            #[inline]
            fn is_nan(self) -> bool {
                $Type::<SCALE>::is_nan(self)
            }
            #[inline]
            fn is_infinite(self) -> bool {
                $Type::<SCALE>::is_infinite(self)
            }
            #[inline]
            fn is_finite(self) -> bool {
                $Type::<SCALE>::is_finite(self)
            }

            // Integer methods.
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                $Type::<SCALE>::div_euclid(self, rhs)
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                $Type::<SCALE>::rem_euclid(self, rhs)
            }
            #[inline]
            fn div_floor(self, rhs: Self) -> Self {
                $Type::<SCALE>::div_floor(self, rhs)
            }
            #[inline]
            fn div_ceil(self, rhs: Self) -> Self {
                $Type::<SCALE>::div_ceil(self, rhs)
            }
            #[inline]
            fn abs_diff(self, rhs: Self) -> Self {
                $Type::<SCALE>::abs_diff(self, rhs)
            }
            #[inline]
            fn midpoint(self, rhs: Self) -> Self {
                $Type::<SCALE>::midpoint(self, rhs)
            }
            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                $Type::<SCALE>::mul_add(self, a, b)
            }

            // Pow.
            #[inline]
            fn pow(self, exp: u32) -> Self {
                $Type::<SCALE>::pow(self, exp)
            }
            #[inline]
            fn powi(self, exp: i32) -> Self {
                $Type::<SCALE>::powi(self, exp)
            }
            #[inline]
            fn checked_pow(self, exp: u32) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_pow(self, exp)
            }
            #[inline]
            fn wrapping_pow(self, exp: u32) -> Self {
                $Type::<SCALE>::wrapping_pow(self, exp)
            }
            #[inline]
            fn saturating_pow(self, exp: u32) -> Self {
                $Type::<SCALE>::saturating_pow(self, exp)
            }
            #[inline]
            fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                $Type::<SCALE>::overflowing_pow(self, exp)
            }

            // Overflow-variant arithmetic.
            #[inline]
            fn checked_add(self, rhs: Self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_add(self, rhs)
            }
            #[inline]
            fn checked_sub(self, rhs: Self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_sub(self, rhs)
            }
            #[inline]
            fn checked_mul(self, rhs: Self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_mul(self, rhs)
            }
            #[inline]
            fn checked_div(self, rhs: Self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_div(self, rhs)
            }
            #[inline]
            fn checked_neg(self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_neg(self)
            }
            #[inline]
            fn checked_rem(self, rhs: Self) -> ::core::option::Option<Self> {
                $Type::<SCALE>::checked_rem(self, rhs)
            }
            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self {
                $Type::<SCALE>::wrapping_add(self, rhs)
            }
            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self {
                $Type::<SCALE>::wrapping_sub(self, rhs)
            }
            #[inline]
            fn wrapping_mul(self, rhs: Self) -> Self {
                $Type::<SCALE>::wrapping_mul(self, rhs)
            }
            #[inline]
            fn wrapping_div(self, rhs: Self) -> Self {
                $Type::<SCALE>::wrapping_div(self, rhs)
            }
            #[inline]
            fn wrapping_neg(self) -> Self {
                $Type::<SCALE>::wrapping_neg(self)
            }
            #[inline]
            fn wrapping_rem(self, rhs: Self) -> Self {
                $Type::<SCALE>::wrapping_rem(self, rhs)
            }
            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                $Type::<SCALE>::saturating_add(self, rhs)
            }
            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self {
                $Type::<SCALE>::saturating_sub(self, rhs)
            }
            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self {
                $Type::<SCALE>::saturating_mul(self, rhs)
            }
            #[inline]
            fn saturating_div(self, rhs: Self) -> Self {
                $Type::<SCALE>::saturating_div(self, rhs)
            }
            #[inline]
            fn saturating_neg(self) -> Self {
                $Type::<SCALE>::saturating_neg(self)
            }
            #[inline]
            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_add(self, rhs)
            }
            #[inline]
            fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_sub(self, rhs)
            }
            #[inline]
            fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_mul(self, rhs)
            }
            #[inline]
            fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_div(self, rhs)
            }
            #[inline]
            fn overflowing_neg(self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_neg(self)
            }
            #[inline]
            fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                $Type::<SCALE>::overflowing_rem(self, rhs)
            }
        }

        impl<const SCALE: u32> $crate::types::traits::convert::DecimalConvert for $Type<SCALE> {
            // Round-trip.
            #[inline]
            fn from_bits(raw: $Storage) -> Self {
                $Type::<SCALE>::from_bits(raw)
            }
            #[inline]
            fn to_bits(self) -> $Storage {
                self.0
            }
            #[inline]
            fn scale(self) -> u32 {
                SCALE
            }

            // Integer conversion.
            #[inline]
            fn from_i32(value: i32) -> Self {
                $Type::<SCALE>::from_i32(value)
            }
            #[inline]
            fn to_int(self) -> i64 {
                $Type::<SCALE>::to_int(self)
            }
            #[inline]
            fn to_int_with(self, mode: $crate::support::rounding::RoundingMode) -> i64 {
                $Type::<SCALE>::to_int_with(self, mode)
            }

            // Float bridge (lossy).
            #[cfg(feature = "std")]
            #[inline]
            fn from_f64(value: f64) -> Self {
                $Type::<SCALE>::from_f64(value)
            }
            #[cfg(feature = "std")]
            #[inline]
            fn from_f64_with(value: f64, mode: $crate::support::rounding::RoundingMode) -> Self {
                $Type::<SCALE>::from_f64_with(value, mode)
            }
            #[cfg(feature = "std")]
            #[inline]
            fn to_f64(self) -> f64 {
                $Type::<SCALE>::to_f64(self)
            }
            #[cfg(feature = "std")]
            #[inline]
            fn to_f32(self) -> f32 {
                $Type::<SCALE>::to_f32(self)
            }
        }
    };
}

pub(crate) use decl_decimal_basics;
