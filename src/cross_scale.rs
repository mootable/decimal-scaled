// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Nightly-gated auto-inferred cross-scale operations.
//!
//! Free functions [`cross::mul`], [`cross::add`], [`cross::sub`],
//! [`cross::div`], [`cross::rem`] that accept two decimals at the
//! same storage width but possibly different `SCALE`s, and return a
//! decimal at `max(SCALE_a, SCALE_b)`. The output SCALE is computed
//! by [`max_const`] inside a `generic_const_exprs` clause, so the
//! call site doesn't have to spell it out.
//!
//! Each function has a `_with(mode)` sibling for explicit
//! [`crate::RoundingMode`]. The no-mode form delegates with the
//! crate's `DEFAULT_ROUNDING_MODE`.
//!
//! # Stability
//!
//! Requires nightly Rust. The `cross-scale-ops` feature flag
//! activates the `feature(generic_const_exprs)` directive in
//! `lib.rs`. `generic_const_exprs` is an incomplete feature
//! ([rust-lang/rust#76560](https://github.com/rust-lang/rust/issues/76560));
//! we cap the surface to the same-width case where the const-eval
//! requirements are minimal.
//!
//! # Cross-width
//!
//! For mixed widths, use the stable explicit-target form on the
//! wider type, e.g. `D38::<12>::mul_of(d18_at_s6, d76_at_s20)`. The
//! `cross::mul(a, b)` shape requires the storage types to match.

#![cfg(feature = "cross-scale-ops")]

use crate::support::rounding::{DEFAULT_ROUNDING_MODE, RoundingMode};
use crate::types::unified::D;

/// `const fn` `max` for `u32`, usable inside a
/// `generic_const_exprs` clause.
#[inline]
pub const fn max_const(a: u32, b: u32) -> u32 {
    if a >= b { a } else { b }
}

/// Same-width cross-scale multiply. Returns a decimal at the wider
/// of the two operand SCALEs.
///
/// ```ignore
/// # // (Doctest is `ignore`d because the feature requires nightly.)
/// use decimal_scaled::{D38, cross};
/// let a: D38<6> = D38::<6>::try_from(7).unwrap();
/// let b: D38<12> = D38::<12>::try_from(11).unwrap();
/// let c = cross::mul(a, b); // D38<12>, value = 77
/// ```
#[inline]
pub fn mul<W, const S1: u32, const S2: u32>(a: D<W, S1>, b: D<W, S2>) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Mul<Output = D<W, { max_const(S1, S2) }>>,
{
    let a_t = <D<W, S1> as WidenScale<W, S1, { max_const(S1, S2) }>>::widen_scale(a);
    let b_t = <D<W, S2> as WidenScale<W, S2, { max_const(S1, S2) }>>::widen_scale(b);
    a_t * b_t
}

/// Like [`mul`] but with explicit rounding mode for the
/// (always-exact, when widening up) rescale of inputs. The argument
/// is preserved for API symmetry with the other `_with` forms in the
/// crate; rescaling to `max_const(S1, S2)` is exact for both inputs.
#[inline]
pub fn mul_with<W, const S1: u32, const S2: u32>(
    a: D<W, S1>,
    b: D<W, S2>,
    _mode: RoundingMode,
) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Mul<Output = D<W, { max_const(S1, S2) }>>,
{
    mul(a, b)
}

/// Same-width cross-scale add.
#[inline]
pub fn add<W, const S1: u32, const S2: u32>(a: D<W, S1>, b: D<W, S2>) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Add<Output = D<W, { max_const(S1, S2) }>>,
{
    let a_t = <D<W, S1> as WidenScale<W, S1, { max_const(S1, S2) }>>::widen_scale(a);
    let b_t = <D<W, S2> as WidenScale<W, S2, { max_const(S1, S2) }>>::widen_scale(b);
    a_t + b_t
}

/// Same-width cross-scale subtract.
#[inline]
pub fn sub<W, const S1: u32, const S2: u32>(a: D<W, S1>, b: D<W, S2>) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Sub<Output = D<W, { max_const(S1, S2) }>>,
{
    let a_t = <D<W, S1> as WidenScale<W, S1, { max_const(S1, S2) }>>::widen_scale(a);
    let b_t = <D<W, S2> as WidenScale<W, S2, { max_const(S1, S2) }>>::widen_scale(b);
    a_t - b_t
}

/// Same-width cross-scale divide.
#[inline]
pub fn div<W, const S1: u32, const S2: u32>(a: D<W, S1>, b: D<W, S2>) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Div<Output = D<W, { max_const(S1, S2) }>>,
{
    let a_t = <D<W, S1> as WidenScale<W, S1, { max_const(S1, S2) }>>::widen_scale(a);
    let b_t = <D<W, S2> as WidenScale<W, S2, { max_const(S1, S2) }>>::widen_scale(b);
    a_t / b_t
}

/// Same-width cross-scale remainder.
#[inline]
pub fn rem<W, const S1: u32, const S2: u32>(a: D<W, S1>, b: D<W, S2>) -> D<W, { max_const(S1, S2) }>
where
    W: Copy,
    D<W, S1>: WidenScale<W, S1, { max_const(S1, S2) }>,
    D<W, S2>: WidenScale<W, S2, { max_const(S1, S2) }>,
    D<W, { max_const(S1, S2) }>: core::ops::Rem<Output = D<W, { max_const(S1, S2) }>>,
{
    let a_t = <D<W, S1> as WidenScale<W, S1, { max_const(S1, S2) }>>::widen_scale(a);
    let b_t = <D<W, S2> as WidenScale<W, S2, { max_const(S1, S2) }>>::widen_scale(b);
    a_t % b_t
}

/// Trait shim: "widen the SCALE of a same-width decimal".
///
/// Implemented for every concrete `(storage, source_scale,
/// target_scale)` triple via a blanket impl that calls the per-width
/// inherent `rescale_with`. Used only inside this module to keep the
/// `generic_const_exprs` bounds clean.
pub trait WidenScale<W, const S_FROM: u32, const S_TO: u32> {
    fn widen_scale(self) -> D<W, S_TO>;
}

// Implementations per concrete storage. Each width's `rescale_with`
// is an inherent method (emitted by `decl_decimal_rescale!` on the
// `D<Int<N>, SCALE>` alias), so we have to spell out the storage type
// to dispatch. Every decimal tier is `Int<N>`-backed; the always-built
// tiers are D18 (`Int<1>`) and D38 (`Int<2>`), the rest follow the
// same feature gates as their `decl_decimal_full!` emissions in
// `types/widths.rs`.

macro_rules! impl_widen_scale {
    ($Storage:ty) => {
        impl<const S_FROM: u32, const S_TO: u32> WidenScale<$Storage, S_FROM, S_TO>
            for D<$Storage, S_FROM>
        {
            #[inline]
            fn widen_scale(self) -> D<$Storage, S_TO> {
                // The math: `cross::mul` etc. always pick the
                // MAXIMUM of (S_FROM, S_TO) so S_TO >= S_FROM by
                // construction. The rescale-UP path is always exact;
                // rounding-mode is irrelevant. We still pass
                // DEFAULT_ROUNDING_MODE so the rescale_with signature
                // is satisfied.
                <D<$Storage, S_FROM>>::rescale_with::<S_TO>(self, DEFAULT_ROUNDING_MODE)
            }
        }
    };
}

impl_widen_scale!(crate::int::types::Int<1>);
impl_widen_scale!(crate::int::types::Int<2>);

#[cfg(any(feature = "d57", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<3>);
#[cfg(any(feature = "d76", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<4>);
#[cfg(any(feature = "d115", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<6>);
#[cfg(any(feature = "d153", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<8>);
#[cfg(any(feature = "d230", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<12>);
#[cfg(any(feature = "d307", feature = "wide"))]
impl_widen_scale!(crate::int::types::Int<16>);
#[cfg(any(feature = "d462", feature = "x-wide"))]
impl_widen_scale!(crate::int::types::Int<24>);
#[cfg(any(feature = "d616", feature = "x-wide"))]
impl_widen_scale!(crate::int::types::Int<32>);
#[cfg(any(feature = "d924", feature = "xx-wide"))]
impl_widen_scale!(crate::int::types::Int<48>);
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
impl_widen_scale!(crate::int::types::Int<64>);
