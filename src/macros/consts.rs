//! Macro-generated `DecimalConstants` impls for every decimal width.
//!
//! The constants are stored once as `i128` literals at `SCALE_REF = 37`
//! in `src/consts.rs` (D38's path). For other widths the macro
//! delegates rescaling to D38 (via `D38::<37>::from_bits(...)
//! .rescale::<SCALE>()`) and then narrows or widens the resulting
//! `i128` to the target storage. Narrowing overflows silently in
//! release builds and panics in debug; downstream callers
//! requesting a SCALE that does not fit the chosen constant should
//! pick a wider type.
//!
//! # Known precision gap (wide tier, follow-up)
//!
//! Wide tiers (D76, D153, D307) widen the same 37-digit `i128`
//! reference into their wider storage. At `SCALE ≤ 37` they get the
//! full 0.5 ULP contract; at `SCALE > 37` the rescale appends
//! trailing zeros — no extra precision — and at `SCALE > 38` the
//! rescale path's intermediate `i128 * 10^k` overflows and panics.
//! Practical impact:
//!
//! - `D76<S>::pi()` works correctly for `S ≤ 37`; at `S = 50` / `76`
//!   (the D76 max) it panics on rescale-up.
//! - `D153<S>::pi()` and `D307<S>::pi()` have the same panic past
//!   `S = 37`.
//!
//! Closing this needs per-width raw constants stored in the storage
//! type itself (e.g. a 75-digit raw `Int256` for D76 built via
//! `Int256::from_str_radix`), with the precision growing to 153
//! digits for D153 and 307 for D307. Recorded as a substantial
//! follow-up — verifying ~308 digits of each constant for D307 is
//! the bulk of the work.

/// Emits `DecimalConstants` for a decimal type. Requires the
/// `D38::<SCALE_REF>::from_bits(_).rescale::<SCALE>()` path to be
/// available (i.e. the rescale macro is already invoked for D38).
///
/// - `decl_decimal_consts!(D9, i32)` — *native* storage; the `i128`
/// reference bits narrow via an `as`-cast.
/// - `decl_decimal_consts!(wide D76, I256)` — *wide* storage; the
/// `i128` reference bits widen via the `WideInt` cast. Because the
/// reference constants are only carried to `SCALE_REF = 35` digits,
/// wide widths gain no extra precision above that scale (the trailing
/// digits are zero-extended) — the value is still correct, just not
/// more precise than the D38 reference.
macro_rules! decl_decimal_consts {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $crate::types::consts::DecimalConstants for $Type<SCALE> {
            #[inline]
            fn pi() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::pi_at_target::<SCALE>()))
            }
            #[inline]
            fn tau() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::tau_at_target::<SCALE>()))
            }
            #[inline]
            fn half_pi() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::half_pi_at_target::<SCALE>()))
            }
            #[inline]
            fn quarter_pi() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::quarter_pi_at_target::<SCALE>()))
            }
            #[inline]
            fn golden() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::golden_at_target::<SCALE>()))
            }
            #[inline]
            fn e() -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::e_at_target::<SCALE>()))
            }
            #[inline]
            fn pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::pi_at_target_with::<SCALE>(mode)))
            }
            #[inline]
            fn tau_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::tau_at_target_with::<SCALE>(mode)))
            }
            #[inline]
            fn half_pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::half_pi_at_target_with::<SCALE>(mode)))
            }
            #[inline]
            fn quarter_pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::quarter_pi_at_target_with::<SCALE>(mode)))
            }
            #[inline]
            fn golden_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::golden_at_target_with::<SCALE>(mode)))
            }
            #[inline]
            fn e_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self(<$Storage>::from_i128($crate::types::consts::e_at_target_with::<SCALE>(mode)))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $crate::types::consts::DecimalConstants for $Type<SCALE> {
            #[inline]
            fn pi() -> Self {
                Self($crate::types::consts::pi_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn tau() -> Self {
                Self($crate::types::consts::tau_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn half_pi() -> Self {
                Self($crate::types::consts::half_pi_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn quarter_pi() -> Self {
                Self($crate::types::consts::quarter_pi_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn golden() -> Self {
                Self($crate::types::consts::golden_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn e() -> Self {
                Self($crate::types::consts::e_at_target::<SCALE>() as $Storage)
            }
            #[inline]
            fn pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::pi_at_target_with::<SCALE>(mode) as $Storage)
            }
            #[inline]
            fn tau_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::tau_at_target_with::<SCALE>(mode) as $Storage)
            }
            #[inline]
            fn half_pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::half_pi_at_target_with::<SCALE>(mode) as $Storage)
            }
            #[inline]
            fn quarter_pi_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::quarter_pi_at_target_with::<SCALE>(mode) as $Storage)
            }
            #[inline]
            fn golden_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::golden_at_target_with::<SCALE>(mode) as $Storage)
            }
            #[inline]
            fn e_with(mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::types::consts::e_at_target_with::<SCALE>(mode) as $Storage)
            }
        }
    };
}

pub(crate) use decl_decimal_consts;
