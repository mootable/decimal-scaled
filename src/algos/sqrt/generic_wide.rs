//! Generic wide-integer square-root kernel — `isqrt(raw · 10^SCALE)`
//! with a single round step.
//!
//! For a `Dxx<SCALE>` value with raw storage `r`, the logical value is
//! `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand is formed exactly in a
//! wider integer (so the multiply by `10^SCALE` cannot overflow), the
//! exact integer square root is taken with `isqrt`, and a single
//! round-to-nearest step lands the result on the type's last
//! representable place. The result is within 0.5 ULP under any of the
//! six [`crate::support::rounding::RoundingMode`] values.
//!
//! # Generic kernel + per-tier shims
//!
//! The kernel is parameterised over `(Storage, SqrtWide)` via the
//! [`crate::int::types::traits::BigInt`] trait, which exposes the unifrom
//! surface (`ZERO` / `ONE` / `TEN`, `pow`, `isqrt`, `resize_to`, the
//! standard arithmetic ops) every wide signed integer in the family
//! ships. The kernel collapses to a single function:
//!
//! - [`sqrt`] — `pub(crate) fn sqrt<S, W>(raw: S, scale, mode) -> S`
//!   where `S: BigInt` is the storage type and
//!   `W: BigInt` is the next-up width used to form the radicand
//!   without overflow.
//!
//! Per-tier free functions ([`sqrt_d57`], [`sqrt_d76`], …) remain as
//! thin shims so the policy callers don't need to know about the
//! `(Storage, SqrtWide)` pairing — each shim picks the right `W` for
//! its tier and forwards to [`sqrt`].
//!
//! Each function returns `Storage::ZERO` for `raw <= 0` (the
//! saturate-not-panic policy matching the typed `sqrt_strict` surface).

use crate::support::rounding::RoundingMode;
use crate::int::types::traits::BigInt;

/// Generic square-root kernel for the wide-integer family.
///
/// `S` is the storage type backing `Dxx<SCALE>` and `W` is the
/// next-up width used to form `raw · 10^scale` without overflow.
/// See the module docs for the full pipeline.
#[inline]
#[must_use]
pub(crate) fn sqrt<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: BigInt,
    W: BigInt,
{
    if raw <= S::ZERO {
        return S::ZERO;
    }
    let n: W = raw.resize_to::<W>() * W::TEN.pow(scale);
    let q: W = n.isqrt();
    let diff: W = n - q * q;
    let halfway_round_up = diff > q;
    let diff_nonzero = diff != W::ZERO;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    let q = if bump { q + W::ONE } else { q };
    q.resize_to::<S>()
}

/// Emits a `sqrt_<tier>(raw, scale, mode) -> Storage` shim around
/// the generic [`sqrt`] kernel.
macro_rules! decl_sqrt_kernel_shim {
    ($name:ident, $Storage:ty, $SqrtWide:ty) => {
        /// Square-root kernel shim for one wide tier. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, scale: u32, mode: RoundingMode) -> $Storage {
            sqrt::<$Storage, $SqrtWide>(raw, scale, mode)
        }
    };
}

use crate::int::types::Int;

#[cfg(any(feature = "d57", feature = "wide"))]
decl_sqrt_kernel_shim!(sqrt_d57, Int<3>, Int<6>);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_sqrt_kernel_shim!(sqrt_d76, Int<4>, Int<8>);

#[cfg(any(feature = "d115", feature = "wide"))]
decl_sqrt_kernel_shim!(sqrt_d115, Int<6>, Int<12>);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_sqrt_kernel_shim!(sqrt_d153, Int<8>, Int<16>);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_sqrt_kernel_shim!(sqrt_d230, Int<12>, Int<24>);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_sqrt_kernel_shim!(
    sqrt_d307,
    Int<16>,
    Int<32>
);

#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_sqrt_kernel_shim!(
    sqrt_d462,
    Int<24>,
    Int<48>
);

#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_sqrt_kernel_shim!(
    sqrt_d616,
    Int<32>,
    Int<64>
);

#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_sqrt_kernel_shim!(
    sqrt_d924,
    Int<48>,
    Int<96>
);

#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_sqrt_kernel_shim!(
    sqrt_d1232,
    Int<64>,
    Int<128>
);
