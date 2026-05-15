//! Macro-generated impls for the *basics* of every decimal width.
//!
//! What this macro emits, for a given `(Type, Storage, MAX_SCALE)`:
//!
//! - The inherent constructor / accessor / multiplier methods.
//! - The `SCALE` associated const + `scale()` method.
//! - The `ZERO`, `ONE`, `MAX`, `MIN` constants.
//! - The [`crate::decimal_trait::Decimal`] trait impl, delegating to
//! the inherent surface.
//!
//! Invoke once per width (see `core_type.rs` for the per-width
//! invocations).
//!
//! Two front-end arms exist:
//!
//! - `decl_decimal_basics!(D38, i128, 38)` — *native* storage. The
//! storage type is a primitive signed integer that supports the
//! `(10 as $Storage)` literal cast and a const-fn `pow`.
//! - `decl_decimal_basics!(wide D76, I256, 76)` — *wide* storage. The
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
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty, $max_scale:literal) => {
        $crate::macros::basics::decl_decimal_basics! {
            @impl $Type, $Storage, $max_scale,
            multiplier = { (10 as $Storage).pow(SCALE) },
            zero = { 0 }
        }
    };

    // Shared implementation body. `$mult` is the const expression for
    // `10^SCALE`; `$zero` is the const expression for the storage zero.
    (@impl $Type:ident, $Storage:ty, $max_scale:literal,
     multiplier = { $mult:expr }, zero = { $zero:expr }) => {
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
        }

        impl<const SCALE: u32> $crate::decimal_trait::Decimal for $Type<SCALE> {
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
        }
    };
}

pub(crate) use decl_decimal_basics;
