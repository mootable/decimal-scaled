//! `PartialEq` impls between `D128<SCALE>` and primitive numeric types.
//!
//! # Semantics
//!
//! - **Integers** (`i8`-`i128`, `u8`-`u128`, `isize`, `usize`): exact
//! mathematical equality. `d == n` holds iff `d.to_bits() == n * 10^SCALE`,
//! i.e. `d` represents the integer `n` with no fractional part. The
//! comparison is computed without overflow by splitting `d.to_bits()` into
//! quotient and remainder modulo `10^SCALE`; the value is equal to `n` iff
//! the remainder is zero and the quotient equals `n`.
//!
//! Examples:
//! - `D128s12::from_int(5) == 5_i32` -> `true`
//! - `D128s12::from_bits(5_500_000_000_000) == 5_i32` -> `false` (5.5 != 5)
//! - `D128s12::from_bits(-1) == 0_u32` -> `false` (negative value)
//!
//! - **Floats** (`f32`, `f64`): equality holds iff `f` is finite and converts
//! to and from `D128` losslessly relative to the f64 representation. NaN and
//! ±inf always compare unequal. A `D128` value larger than `2^53` cannot
//! match any `f64` exactly and will compare unequal except when the float's
//! stored value happens to round-trip.
//!
//! Note that f64 cannot represent decimals like `1.1` exactly; the nearest
//! f64 to `1.1` is `1.1000000000000000888...`. The implementation treats
//! that nearest f64 as equal to `D128s12::from_bits(1_100_000_000_000)`
//! because the round-trip through `from_f64_lossy`/`to_f64_lossy` agrees.
//! Callers who need true rational equality should convert and compare
//! explicitly.
//!
//! Each impl provides both directions (`D128<S> == T` and `T == D128<S>`) so
//! comparisons are symmetric at the call site.

use crate::core_type::{D128, D32, D64};

// Cross-equality with primitive integer types is emitted by the
// `decl_eq_all_integers!` macro family — see
// `src/decimal_equalities_macro.rs`. The same surface is generated for
// every decimal width.
crate::macros::equalities::decl_eq_all_integers!(D128);
crate::macros::equalities::decl_eq_all_integers!(D64);
crate::macros::equalities::decl_eq_all_integers!(D32);

#[cfg(any(feature = "d256", feature = "wide"))]
use crate::core_type::D256;
#[cfg(any(feature = "d512", feature = "wide"))]
use crate::core_type::D512;
#[cfg(any(feature = "d1024", feature = "wide"))]
use crate::core_type::D1024;

#[cfg(any(feature = "d256", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(wide D256);
#[cfg(any(feature = "d512", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(wide D512);
#[cfg(any(feature = "d1024", feature = "wide"))]
crate::macros::equalities::decl_eq_all_integers!(wide D1024);

// Float equality requires the f64 bridge (`from_f64_lossy` /
// `to_f64_lossy`), which is only present when `std` is on and
// `strict` is off. Gate the float impls accordingly. Float impls are
// emitted for D128 only at present — the D32/D64 conversion bridge
// covers will follow in a later commit.
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D128, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D128, f64);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D64, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D64, f64);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D32, f32);
#[cfg(feature = "std")]
crate::macros::equalities::decl_eq_float!(D32, f64);

// Wide tiers share the same float-bridge surface, so the same macro
// applies unchanged.
#[cfg(all(feature = "std", any(feature = "d256", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D256, f32);
#[cfg(all(feature = "std", any(feature = "d256", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D256, f64);
#[cfg(all(feature = "std", any(feature = "d512", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D512, f32);
#[cfg(all(feature = "std", any(feature = "d512", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D512, f64);
#[cfg(all(feature = "std", any(feature = "d1024", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D1024, f32);
#[cfg(all(feature = "std", any(feature = "d1024", feature = "wide")))]
crate::macros::equalities::decl_eq_float!(D1024, f64);

