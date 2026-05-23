//! `PartialEq` impls between every decimal width and primitive numeric
//! types. Wires the macro emissions in `src/macros/equalities.rs` to
//! each concrete width.
//!
//! # Semantics — EXACT value equality
//!
//! Both surfaces ride the shared 1.3 const comparator
//! (`Int::cmp_cross_scaled` for integers, `Int::cmp_f64_exact` for
//! floats); no quotient/remainder bespoke path, no `from_f64`/`to_f64`
//! round-trip.
//!
//! - **Integers** (`i8`-`i128`, `u8`-`u128`, `isize`, `usize`): exact
//! mathematical equality. A primitive `n` is the scale-0 value `n`, so
//! `d == n` compares `d`'s storage (scale `SCALE`) against `n` (scale 0)
//! via the cross-scale comparator. Exact and overflow-free.
//!
//! Examples:
//! - `D38s12::from(5) == 5_i32` -> `true`
//! - `5.5` stored in `D38s12` `== 5_i32` -> `false` (`5.5 != 5`)
//! - a negative `D38s12` `== 0_u32` -> `false`
//!
//! - **Floats** (`f32`, `f64`): EXACT value equality. The decimal's
//! rational value `bits / 10^SCALE` is compared against the float's
//! exact dyadic value `m · 2^e` by cross-multiplying to integers. `NaN`
//! and `±inf` always compare unequal.
//!
//! This is distinct from the lossy `TryFrom<f64>` / `to_f64`
//! conversions: `f64` cannot represent `1.1` exactly (the nearest `f64`
//! is `1.1000000000000000888...`), so `D::from_str("1.1") == 1.1_f64` is
//! `false`. Callers wanting the *rounded* float should convert with
//! `from_f64` and compare decimals.
//!
//! Each impl provides both directions (`D38<S> == T` and `T == D38<S>`) so
//! comparisons are symmetric at the call site.

use crate::types::widths::{D18, D38};

// Cross-equality with primitive integer types is emitted by the
// `decl_eq_all_integers!` macro family in `src/macros/equalities.rs`.
// The same surface is generated for every decimal width.
crate::macros::equalities::decl_eq_all_integers!(D38);
crate::macros::equalities::decl_eq_all_integers!(D18);

#[cfg(any(feature = "d76", feature = "wide"))]
use crate::types::widths::D76;
#[cfg(any(feature = "d153", feature = "wide"))]
use crate::types::widths::D153;
#[cfg(any(feature = "d307", feature = "wide"))]
use crate::types::widths::D307;

#[cfg(any(feature = "d76", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(D76);
#[cfg(any(feature = "d153", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(D153);
#[cfg(any(feature = "d307", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(D307);

// Float equality requires the f64 bridge (`from_f64` / `to_f64`),
// which is gated on `std`. Float impls are emitted for every width
// below.
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D38, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D38, f64);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D18, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D18, f64);

// Wide tiers share the same exact-equality surface, so the same macro
// applies unchanged.
#[cfg(all(feature = "std", any(feature = "d76", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D76, f32);
#[cfg(all(feature = "std", any(feature = "d76", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D76, f64);
#[cfg(all(feature = "std", any(feature = "d153", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D153, f32);
#[cfg(all(feature = "std", any(feature = "d153", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D153, f64);
#[cfg(all(feature = "std", any(feature = "d307", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D307, f32);
#[cfg(all(feature = "std", any(feature = "d307", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D307, f64);
