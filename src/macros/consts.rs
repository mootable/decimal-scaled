//! Macro-generated `DecimalConsts` impls for narrow decimal widths.
//!
//! The constants are stored once as `i128` literals at `SCALE_REF = 35`
//! in `src/consts.rs` (D128's path). For narrower widths the macro
//! delegates rescaling to D128 (via `D128::<35>::from_bits(...)
//! .rescale::<SCALE>()`) and then narrows the resulting `i128` to the
//! target storage. Narrowing overflows silently in release builds and
//! panics in debug; downstream callers requesting a SCALE that does
//! not fit the chosen constant should pick a wider type.

/// Emits `DecimalConsts` for a decimal type. Requires the
/// `D128::<SCALE_REF>::from_bits(_).rescale::<SCALE>()` path to be
/// available (i.e. the rescale macro is already invoked for D128).
macro_rules! decl_decimal_consts {
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
