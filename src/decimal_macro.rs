//! Macro-generated impls for the *basics* of every decimal width.
//!
//! Phase 1 of the multi-width plan introduces this macro skeleton with
//! D128 as its only consumer. The bulk of D128's surface (arithmetic,
//! display, num_traits, overflow variants, equalities, conversions,
//! transcendentals) stays hand-coded in its own modules for now;
//! incremental macro extraction of those surfaces lands as new widths
//! join the family in Phase 3+.
//!
//! What this macro emits, for a given `(Type, Storage, MAX_SCALE)`:
//!
//! - The inherent constructor / accessor / multiplier methods.
//! - The `SCALE` associated const + `scale()` method.
//! - The `ZERO`, `ONE`, `MAX`, `MIN` constants.
//! - The [`crate::decimal_trait::Decimal`] trait impl, delegating to
//!   the inherent surface.
//!
//! Invoke once per width (see `core_type.rs` for the D128 invocation).

/// Generates the basic inherent surface and the `Decimal` trait impl
/// for a decimal type `$Type<SCALE>` backed by `$Storage`, with the
/// given `$max_scale`.
///
/// `$Storage` must be a signed integer primitive that supports
/// `MAX`, `MIN`, and a const-fn `pow(u32)`. Today only `i128` is
/// supplied; `i32` / `i64` follow in Phase 3, and the wide widths
/// (`BInt<N>`) follow in Phase 5.
macro_rules! decl_decimal_basics {
    ($Type:ident, $Storage:ty, $max_scale:literal) => {
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
                (10 as $Storage).pow(SCALE)
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
            pub const ZERO: Self = Self(0);

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
