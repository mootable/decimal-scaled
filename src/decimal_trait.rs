//! The [`Decimal`] trait — width-generic surface across the
//! `decimal-scaled` type family.
//!
//! At present, only `D128<SCALE>` implements this trait. As the family
//! grows (per the design plan: D32, D64, D256, D512, D1024), each new
//! width's macro-generated impl will also implement `Decimal`, letting
//! downstream code write generic numeric helpers like
//!
//! ```ignore
//! fn average<D: Decimal>(values: &[D]) -> D {
//!     let mut sum = D::ZERO;
//!     for v in values { sum = /* width-generic add */; }
//!     sum
//! }
//! ```
//!
//! The trait is deliberately minimal in Phase 1. Phase 4 will add
//! width-generic helpers (`sum`, `mean`, `total_cmp`, etc.) once
//! multiple concrete widths exist to validate the abstraction.
//!
//! For most users the concrete type (`D128<SCALE>` or one of its
//! aliases like `D128s12`) is the canonical surface. Reach for
//! `Decimal` only when writing code that must work across widths.

use crate::core_type::D128;

/// A scaled fixed-point decimal type with a compile-time `SCALE` and a
/// fixed-width integer `Storage`.
///
/// Every implementor exposes:
///
/// - An associated [`Self::Storage`] type — the underlying integer
///   representation. For `D128<SCALE>` this is `i128`.
/// - The compile-time [`Self::SCALE`] of the value, equal to the
///   const-generic parameter.
/// - The width-specific [`Self::MAX_SCALE`] cap, equal to the largest
///   `SCALE` such that `10^SCALE` fits in `Self::Storage`.
/// - `ZERO`, `ONE`, `MAX`, `MIN` constants typed as `Self`.
/// - Round-trip constructors / accessors `from_bits` and `to_bits`,
///   plus `multiplier()` returning `10^SCALE`.
///
/// # Precision
///
/// N/A: this is a trait definition, no arithmetic is performed.
pub trait Decimal: Copy + PartialEq + Eq {
    /// Underlying integer storage type (e.g. `i128` for `D128<SCALE>`).
    type Storage: Copy + PartialEq + Eq;

    /// The decimal scale of this type, equal to the const-generic
    /// parameter. One LSB of storage represents `10^-SCALE`.
    const SCALE: u32;

    /// The maximum legal `SCALE` for this width. Equal to the largest
    /// `k` such that `10^k` fits in `Self::Storage`. For `D128`,
    /// this is `38`.
    const MAX_SCALE: u32;

    /// The additive identity (logical value `0`).
    const ZERO: Self;

    /// The multiplicative identity (logical value `1`).
    const ONE: Self;

    /// The largest representable value (storage equal to
    /// `Self::Storage::MAX`).
    const MAX: Self;

    /// The smallest representable value (storage equal to
    /// `Self::Storage::MIN`).
    const MIN: Self;

    /// Returns `10^SCALE`, the factor that converts a logical integer
    /// to its storage representation.
    fn multiplier() -> Self::Storage;

    /// Constructs from a raw storage value.
    fn from_bits(raw: Self::Storage) -> Self;

    /// Returns the raw storage value.
    fn to_bits(self) -> Self::Storage;

    /// Returns the decimal scale of this value (equal to
    /// [`Self::SCALE`]; provided for ergonomic method-call syntax).
    fn scale(self) -> u32;
}

impl<const SCALE: u32> Decimal for D128<SCALE> {
    type Storage = i128;

    const SCALE: u32 = SCALE;
    const MAX_SCALE: u32 = 38;
    const ZERO: Self = D128::<SCALE>::ZERO;
    const ONE: Self = D128::<SCALE>::ONE;
    const MAX: Self = D128::<SCALE>::MAX;
    const MIN: Self = D128::<SCALE>::MIN;

    #[inline]
    fn multiplier() -> i128 {
        D128::<SCALE>::multiplier()
    }

    #[inline]
    fn from_bits(raw: i128) -> Self {
        D128::<SCALE>::from_bits(raw)
    }

    #[inline]
    fn to_bits(self) -> i128 {
        self.0
    }

    #[inline]
    fn scale(self) -> u32 {
        SCALE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_type::D128s12;

    /// Trait-dispatch accessors agree with inherent methods.
    #[test]
    fn trait_dispatch_matches_inherent() {
        let v: D128s12 = D128s12::from_bits(1_500_000_000_000);
        assert_eq!(<D128s12 as Decimal>::SCALE, 12);
        assert_eq!(<D128s12 as Decimal>::MAX_SCALE, 38);
        assert_eq!(<D128s12 as Decimal>::multiplier(), 1_000_000_000_000_i128);
        assert_eq!(<D128s12 as Decimal>::to_bits(v), 1_500_000_000_000);
        assert_eq!(<D128s12 as Decimal>::scale(v), 12);
    }

    /// Trait constants line up with inherent constants.
    #[test]
    fn trait_constants_match_inherent() {
        assert_eq!(<D128s12 as Decimal>::ZERO, D128s12::ZERO);
        assert_eq!(<D128s12 as Decimal>::ONE, D128s12::ONE);
        assert_eq!(<D128s12 as Decimal>::MAX, D128s12::MAX);
        assert_eq!(<D128s12 as Decimal>::MIN, D128s12::MIN);
    }

    /// Trait works in width-generic context.
    fn return_zero<D: Decimal>() -> D {
        D::ZERO
    }

    #[test]
    fn width_generic_zero() {
        let z: D128s12 = return_zero();
        assert_eq!(z, D128s12::ZERO);
    }

    /// Storage associated type is reachable.
    fn double_bits<D: Decimal<Storage = i128>>(d: D) -> i128 {
        d.to_bits() * 2
    }

    #[test]
    fn width_generic_with_storage_bound() {
        let v = D128s12::from_bits(7);
        assert_eq!(double_bits(v), 14);
    }
}
