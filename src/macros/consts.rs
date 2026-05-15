//! Macro-generated `DecimalConsts` impls for every decimal width.
//!
//! The constants are stored once as `i128` literals at `SCALE_REF = 37`
//! in `src/consts.rs` (D128's path). For other widths the macro
//! delegates rescaling to D128 (via `D128::<37>::from_bits(...)
//! .rescale::<SCALE>()`) and then narrows or widens the resulting
//! `i128` to the target storage. Narrowing overflows silently in
//! release builds and panics in debug; downstream callers
//! requesting a SCALE that does not fit the chosen constant should
//! pick a wider type.
//!
//! # Known precision gap (wide tier, follow-up)
//!
//! Wide tiers (D256, D512, D1024) widen the same 37-digit `i128`
//! reference into their wider storage. At `SCALE ≤ 37` they get the
//! full 0.5 ULP contract; at `SCALE > 37` the rescale appends
//! trailing zeros — no extra precision — and at `SCALE > 38` the
//! rescale path's intermediate `i128 * 10^k` overflows and panics.
//! Practical impact:
//!
//! - D256<S>::pi() works correctly for `S ≤ 37`; at `S = 50` / `76`
//!   (the D256 max) it panics on rescale-up.
//! - D512<S>::pi() and D1024<S>::pi() have the same panic past
//!   `S = 37`.
//!
//! Closing this needs per-width raw constants stored in the storage
//! type itself (e.g. a 75-digit raw `Int256` for D256 built via
//! `Int256::from_str_radix`), with the precision growing to 153
//! digits for D512 and 307 for D1024. Recorded as a substantial
//! follow-up — verifying ~308 digits of each constant for D1024 is
//! the bulk of the work.

/// Emits `DecimalConsts` for a decimal type. Requires the
/// `D128::<SCALE_REF>::from_bits(_).rescale::<SCALE>()` path to be
/// available (i.e. the rescale macro is already invoked for D128).
///
/// - `decl_decimal_consts!(D32, i32)` — *native* storage; the `i128`
/// reference bits narrow via an `as`-cast.
/// - `decl_decimal_consts!(wide D256, I256)` — *wide* storage; the
/// `i128` reference bits widen via the `WideInt` cast. Because the
/// reference constants are only carried to `SCALE_REF = 35` digits,
/// wide widths gain no extra precision above that scale (the trailing
/// digits are zero-extended) — the value is still correct, just not
/// more precise than the D128 reference.
macro_rules! decl_decimal_consts {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $crate::consts::DecimalConsts for $Type<SCALE> {
            #[inline]
            fn pi() -> Self {
                Self(<$Storage>::from_i128($crate::consts::pi_at_target::<SCALE>()))
            }
            #[inline]
            fn tau() -> Self {
                Self(<$Storage>::from_i128($crate::consts::tau_at_target::<SCALE>()))
            }
            #[inline]
            fn half_pi() -> Self {
                Self(<$Storage>::from_i128($crate::consts::half_pi_at_target::<SCALE>()))
            }
            #[inline]
            fn quarter_pi() -> Self {
                Self(<$Storage>::from_i128($crate::consts::quarter_pi_at_target::<SCALE>()))
            }
            #[inline]
            fn golden() -> Self {
                Self(<$Storage>::from_i128($crate::consts::golden_at_target::<SCALE>()))
            }
            #[inline]
            fn e() -> Self {
                Self(<$Storage>::from_i128($crate::consts::e_at_target::<SCALE>()))
            }
        }
    };

    // Native (primitive integer) storage.
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $crate::consts::DecimalConsts for $Type<SCALE> {
            #[inline]
            fn pi() -> Self {
                let bits = $crate::consts::pi_at_target::<SCALE>();
                Self(bits as $Storage)
            }
            #[inline]
            fn tau() -> Self {
                let bits = $crate::consts::tau_at_target::<SCALE>();
                Self(bits as $Storage)
            }
            #[inline]
            fn half_pi() -> Self {
                let bits = $crate::consts::half_pi_at_target::<SCALE>();
                Self(bits as $Storage)
            }
            #[inline]
            fn quarter_pi() -> Self {
                let bits = $crate::consts::quarter_pi_at_target::<SCALE>();
                Self(bits as $Storage)
            }
            #[inline]
            fn golden() -> Self {
                let bits = $crate::consts::golden_at_target::<SCALE>();
                Self(bits as $Storage)
            }
            #[inline]
            fn e() -> Self {
                let bits = $crate::consts::e_at_target::<SCALE>();
                Self(bits as $Storage)
            }
        }
    };
}

pub(crate) use decl_decimal_consts;
