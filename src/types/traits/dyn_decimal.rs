//! Runtime-polymorphic façade over every concrete decimal width.
//!
//! `decimal-scaled` is built around the typed, monomorphised
//! [`crate::Decimal`] trait: it gives zero-cost generic code at the
//! price of every concrete width / scale pair being a distinct type at
//! compile time. That works beautifully for code that knows its
//! widths and scales statically — and exposes nothing useful when you
//! need a single uniform handle for runtime-chosen values (config-driven
//! width selection, plugin types, a `Vec<Box<…>>` of mixed decimals).
//!
//! This module fills that gap with a deliberately small, object-safe
//! trait [`DynDecimal`] and a tagged-union [`DecimalWidth`] /
//! [`RawStorage`] pair so callers can identify and unpack the underlying
//! value. Construction is the same as for the typed API; the boxed
//! handle is acquired by coercion:
//!
//! ```ignore
//! use decimal_scaled::{D38, DynDecimal};
//! let lhs: Box<dyn DynDecimal> = Box::new(D38::<2>::from_i32(150));
//! let rhs: Box<dyn DynDecimal> = Box::new(D38::<5>::from_i32(2));
//! let sum = lhs.add(&*rhs).expect("same width: D38");
//! // sum is a D38 at the wider scale (5).
//! ```
//!
//! # Semantics of binary operations
//!
//! Every binary op (`add`, `sub`, `mul`, `div`, `rem`, `eq_dyn`,
//! `cmp_dyn`) returns `None` (or `false`) when the two operands have
//! **different widths**: `D38` and `D76` cannot mix without an explicit
//! widening step.
//!
//! When the widths match but the **scales differ**, the operands are
//! losslessly rescaled to the wider of the two scales, the typed op is
//! performed at that scale, and the result is returned at that scale.
//! This mirrors the rule a programmer would follow with the typed API
//! ("rescale up so no fractional digits are lost"). Arithmetic overflow
//! during the rescale or the op itself returns `None` rather than
//! panicking.
//!
//! # Downcasting back to the typed surface
//!
//! Use [`DynDecimal::as_any`] and `Any::downcast_ref` once you know the
//! concrete `Dxx<S>` you expect:
//!
//! ```ignore
//! let typed: &D38<5> = boxed.as_any().downcast_ref::<D38<5>>().unwrap();
//! ```
//!
//! # Cost
//!
//! Each binary op allocates one or two `Box<dyn DynDecimal>` for the
//! result (and possibly intermediate rescaled operands). Use the typed
//! [`crate::Decimal`] surface in hot paths.
//!
//! # Scope
//!
//! The `dyn` feature ships impls for the narrow-tier widths only —
//! [`D18`], [`D38`]. The wide and extra-wide tiers
//! (D57/D76/D115/D153/D230/D307/D462/D616/D924/D1232) are deliberately
//! excluded: their per-scale monomorphisation footprint
//! (`MAX_SCALE + 1` instantiations per op per width, up to 1233 for
//! `D1232`) would dominate compile time, and the dyn façade is the
//! wrong tool for the compute-bound code those tiers serve. Reach for
//! the typed [`crate::Decimal`] surface there. The [`DecimalWidth`] and
//! [`RawStorage`] enums still carry variants for the wider tiers so
//! that the API is forward-compatible if those impls are added later.
//!
//! [`D18`]: crate::D18
//! [`D38`]: crate::D38

#![cfg(feature = "dyn")]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use core::any::Any;
use core::cmp::Ordering;

use crate::support::rounding::RoundingMode;

/// Discriminator for the concrete decimal width carried by a
/// [`DynDecimal`] value.
///
/// Two values are guaranteed to share a concrete `Dxx<S>` family (i.e.
/// both are some scale of `D38`, or both some scale of `D76`, etc.) if
/// and only if their [`width`](DynDecimal::width) values compare equal.
/// Use this to dispatch on the storage tier before unpacking.
///
/// Variants are gated by the same Cargo features that gate the
/// corresponding decimal width — they are only present in this enum
/// when the underlying `Dxx` type is enabled in the build.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[non_exhaustive]
pub enum DecimalWidth {
    /// 64-bit storage; corresponds to [`crate::D18`].
    D18,
    /// 128-bit storage; corresponds to [`crate::D38`].
    D38,
    /// 192-bit storage; corresponds to [`crate::D57`]. Gated by `d57` / `wide`.
    #[cfg(any(feature = "d57", feature = "wide"))]
    D57,
    /// 256-bit storage; corresponds to [`crate::D76`]. Gated by `d76` / `wide`.
    #[cfg(any(feature = "d76", feature = "wide"))]
    D76,
    /// 384-bit storage; corresponds to [`crate::D115`]. Gated by `d115` / `wide`.
    #[cfg(any(feature = "d115", feature = "wide"))]
    D115,
    /// 512-bit storage; corresponds to [`crate::D153`]. Gated by `d153` / `wide`.
    #[cfg(any(feature = "d153", feature = "wide"))]
    D153,
    /// 768-bit storage; corresponds to [`crate::D230`]. Gated by `d230` / `wide`.
    #[cfg(any(feature = "d230", feature = "wide"))]
    D230,
    /// 1024-bit storage; corresponds to [`crate::D307`]. Gated by `d307` / `wide` / `x-wide`.
    #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
    D307,
    /// 1536-bit storage; corresponds to [`crate::D462`]. Gated by `d462` / `x-wide`.
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    D462,
    /// 2048-bit storage; corresponds to [`crate::D616`]. Gated by `d616` / `x-wide`.
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    D616,
    /// 3072-bit storage; corresponds to [`crate::D924`]. Gated by `d924` / `xx-wide`.
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    D924,
    /// 4096-bit storage; corresponds to [`crate::D1232`]. Gated by `d1232` / `xx-wide`.
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    D1232,
}

/// Tagged-union view of the raw storage integer behind a [`DynDecimal`]
/// value, with the scale stripped.
///
/// Use this to inspect or transport the underlying integer when the
/// concrete decimal type is not statically known. The variant matches
/// the value's [`DecimalWidth`] one-to-one.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RawStorage {
    /// 64-bit signed integer — D18.
    I64(i64),
    /// 128-bit signed integer — D38.
    I128(i128),
    /// 192-bit signed integer — D57. Gated by `d57` / `wide`.
    #[cfg(any(feature = "d57", feature = "wide"))]
    Int192(crate::int::types::Int<3>),
    /// 256-bit signed integer — D76. Gated by `d76` / `wide`.
    #[cfg(any(feature = "d76", feature = "wide"))]
    Int256(crate::int::types::Int<4>),
    /// 384-bit signed integer — D115. Gated by `d115` / `wide`.
    #[cfg(any(feature = "d115", feature = "wide"))]
    Int384(crate::int::types::Int<6>),
    /// 512-bit signed integer — D153. Gated by `d153` / `wide`.
    #[cfg(any(feature = "d153", feature = "wide"))]
    Int512(crate::int::types::Int<8>),
    /// 768-bit signed integer — D230. Gated by `d230` / `wide`.
    #[cfg(any(feature = "d230", feature = "wide"))]
    Int768(crate::int::types::Int<12>),
    /// 1024-bit signed integer — D307. Gated by `d307` / `wide` / `x-wide`.
    #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
    Int1024(crate::int::types::Int<16>),
    /// 1536-bit signed integer — D462. Gated by `d462` / `x-wide`.
    #[cfg(any(feature = "d462", feature = "x-wide"))]
    Int1536(crate::int::types::Int<24>),
    /// 2048-bit signed integer — D616. Gated by `d616` / `x-wide`.
    #[cfg(any(feature = "d616", feature = "x-wide"))]
    Int2048(crate::int::types::Int<32>),
    /// 3072-bit signed integer — D924. Gated by `d924` / `xx-wide`.
    #[cfg(any(feature = "d924", feature = "xx-wide"))]
    Int3072(crate::int::types::Int<48>),
    /// 4096-bit signed integer — D1232. Gated by `d1232` / `xx-wide`.
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    Int4096(crate::int::types::Int<64>),
}

/// Object-safe, width-erased view of a decimal value.
///
/// Implemented by every concrete `Dxx<S>` shipped with the crate. See
/// the module-level documentation for the cross-width / cross-scale
/// semantics and the cost model.
///
/// The trait is intentionally narrower than [`crate::Decimal`]: it
/// covers identity, sign, comparison, arithmetic, rescale, the float
/// bridge, and `Display`. Transcendental functions and constants live
/// only on the typed surface — use [`DynDecimal::as_any`] to downcast
/// to a concrete `Dxx<S>` and call them there.
pub trait DynDecimal: 'static {
    // ── Identity ────────────────────────────────────────────────────

    /// Returns the storage tier this value lives in.
    fn width(&self) -> DecimalWidth;

    /// Returns the decimal scale of this value.
    fn scale_dyn(&self) -> u32;

    /// Returns the maximum legal scale for this value's width.
    fn max_scale(&self) -> u32;

    /// Returns the raw storage integer (scale stripped).
    fn raw_storage(&self) -> RawStorage;

    /// Returns this value as a `&dyn Any` for downcasting to a
    /// concrete `Dxx<S>`.
    fn as_any(&self) -> &dyn Any;

    /// Heap-clones into a fresh `Box<dyn DynDecimal>`.
    fn clone_box(&self) -> Box<dyn DynDecimal>;

    // ── Predicates ──────────────────────────────────────────────────

    /// Returns `true` if this value is the additive identity for its type.
    fn is_zero(&self) -> bool;

    /// Returns `true` if this value is the multiplicative identity for its type.
    fn is_one(&self) -> bool;

    /// Returns `true` if `self > 0`.
    fn is_positive(&self) -> bool;

    /// Returns `true` if `self < 0`.
    fn is_negative(&self) -> bool;

    // ── Sign / unary ────────────────────────────────────────────────

    /// Returns `+1`, `0`, or `-1` (each at the same width/scale as `self`).
    fn signum(&self) -> Box<dyn DynDecimal>;

    /// Returns `|self|`.
    fn abs(&self) -> Box<dyn DynDecimal>;

    /// Returns `-self`.
    fn neg(&self) -> Box<dyn DynDecimal>;

    // ── Binary arithmetic ──────────────────────────────────────────

    /// `self + rhs`. Returns `None` if widths differ, if the auto-rescale
    /// to the wider scale overflows, or if the sum overflows.
    fn add(&self, rhs: &dyn DynDecimal) -> Option<Box<dyn DynDecimal>>;

    /// `self - rhs`. Same width / overflow contract as [`Self::add`].
    fn sub(&self, rhs: &dyn DynDecimal) -> Option<Box<dyn DynDecimal>>;

    /// `self * rhs`. Same width / overflow contract as [`Self::add`].
    fn mul(&self, rhs: &dyn DynDecimal) -> Option<Box<dyn DynDecimal>>;

    /// `self / rhs`. Returns `None` on width mismatch, rescale overflow,
    /// product overflow, or division by zero.
    fn div(&self, rhs: &dyn DynDecimal) -> Option<Box<dyn DynDecimal>>;

    /// `self % rhs`. Same contract as [`Self::div`].
    fn rem(&self, rhs: &dyn DynDecimal) -> Option<Box<dyn DynDecimal>>;

    // ── Rescale ─────────────────────────────────────────────────────

    /// Rescale to `target_scale` using the crate-default rounding mode.
    /// Returns `None` if `target_scale > max_scale()` or the scale-up
    /// multiplication overflows.
    fn rescale_to(&self, target_scale: u32) -> Option<Box<dyn DynDecimal>>;

    /// Rescale with an explicit rounding mode. See [`Self::rescale_to`].
    fn rescale_to_with(&self, target_scale: u32, mode: RoundingMode)
    -> Option<Box<dyn DynDecimal>>;

    // ── Comparison ──────────────────────────────────────────────────

    /// Equality after width check + lossless rescale to the wider scale.
    /// Different widths are never equal.
    fn eq_dyn(&self, rhs: &dyn DynDecimal) -> bool;

    /// Ordering after width check + lossless rescale to the wider scale.
    /// Different widths return `None`.
    fn cmp_dyn(&self, rhs: &dyn DynDecimal) -> Option<Ordering>;

    // ── Conversion ──────────────────────────────────────────────────

    /// Canonical decimal string. Equivalent to `format!("{}", self)`.
    fn display(&self) -> String;

    /// Lossy conversion to `f64`. Available only with the `std` feature.
    #[cfg(feature = "std")]
    fn to_f64(&self) -> f64;

    /// Conversion to `i64` using the crate-default rounding mode.
    /// Saturates on overflow; see [`crate::DecimalConvert::to_int`].
    fn to_int(&self) -> i64;
}

// ── Per-width impl emission ───────────────────────────────────────────
//
// Each invocation enumerates every legal `SCALE` for the width so the
// match arms in `add` / `sub` / `mul` / `div` / `rem` / `rescale_to_with`
// / `eq_dyn` / `cmp_dyn` cover the full range. Out-of-range scales fall
// through to `None` (or `false` for `eq_dyn`).

crate::macros::dyn_bridge::decl_decimal_dyn_impl!(
    D18, crate::int::types::Int<1>, D18, I64, i64, 18,
    scales = [0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18]
);

crate::macros::dyn_bridge::decl_decimal_dyn_impl!(
    D38, crate::int::types::Int<2>, D38, I128, i128, 38,
    scales = [
        0 1 2 3 4 5 6 7 8 9
        10 11 12 13 14 15 16 17 18 19
        20 21 22 23 24 25 26 27 28 29
        30 31 32 33 34 35 36 37 38
    ]
);
