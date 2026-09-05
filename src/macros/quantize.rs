// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated `quantize` / `quantize_with` for all decimal widths.
//!
//! `quantize` sets the quantum: it changes `SCALE` at a fixed storage width.
//! [`requantize`](crate::macros::requantize) moves both axes at once.
//!
//! **Surface only.** Every method here is a thin delegation: the
//! computation lives in [`crate::algos::quantize::quantize_pow10`], and
//! which kernel runs for a given `(N, SCALE, TARGET_SCALE)` cell is
//! [`crate::policy::quantize`]'s to decide. `quantize_with` forwards to
//! that policy and owns nothing but the tier-named scale-up overflow
//! panic; the no-arg `quantize` forwards to `quantize_with` with the
//! crate's `DEFAULT_ROUNDING_MODE`, which is `HalfToEven` unless a
//! `rounding-*` Cargo feature selects something else.
//!
//! The methods are ordinary `fn`, not `const fn`: the wide integer's
//! `Div` / `Rem` operators are not `const`.
//!
//! # The `rescale` alias
//!
//! This operation shipped in 0.5.0 as `rescale` — the name the decimal
//! arithmetic specification marks as deprecated in favour of `quantize`.
//! `rescale` / `rescale_with` are kept as delegating aliases and are
//! removed in 0.6.0.

/// Emits `quantize` (no-arg, uses `DEFAULT_ROUNDING_MODE`) and
/// `quantize_with` (explicit mode) methods for `$Type<SCALE>` with
/// storage `$Storage`, plus the deprecated `rescale` aliases.
macro_rules! decl_decimal_quantize {
    // Wide storage. Not `const` — the wide integer's `Div`/`Rem`
    // operators are not const fns.
    (wide $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Quantizes to `TARGET_SCALE` using the crate's default
            /// rounding mode (`HalfToEven`, or whatever a `rounding-*`
            /// Cargo feature selects). Delegates to [`Self::quantize_with`].
            #[inline]
            #[must_use]
            pub fn quantize<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize_with::<TARGET_SCALE>($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Builder-style alias for [`Self::quantize`].
            ///
            /// Returns a new value at `TARGET_SCALE` using the crate's
            /// default rounding mode. Use [`Self::quantize_with`] when
            /// you need to pass an explicit [`RoundingMode`].
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            #[must_use]
            pub fn with_scale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize::<TARGET_SCALE>()
            }

            /// Quantizes to `TARGET_SCALE` using the supplied rounding
            /// mode.
            ///
            /// - `TARGET_SCALE == SCALE`: bit-identity.
            /// - `TARGET_SCALE > SCALE`: scale-up multiplies by
            /// `10^(TARGET - SCALE)`; lossless; panics on overflow.
            /// - `TARGET_SCALE < SCALE`: scale-down divides by
            /// `10^(SCALE - TARGET)` with the requested rounding rule.
            #[inline]
            #[must_use]
            pub fn quantize_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                use $crate::policy::quantize::QuantizePolicy as _;
                match self.quantize_impl::<TARGET_SCALE>(mode) {
                    Some(bits) => $Type::<TARGET_SCALE>::from_bits(bits),
                    // Scale-up only; the dispatcher never returns `None`
                    // for a scale-down.
                    None => panic!(concat!(stringify!($Type), "::quantize: scale-up overflow")),
                }
            }

            /// Deprecated alias for [`Self::quantize`].
            ///
            /// Delegates unchanged; removed in 0.6.0.
            #[inline]
            #[must_use]
            #[deprecated(
                since = "0.5.1",
                note = "renamed to `quantize`; `rescale` is removed in 0.6.0"
            )]
            pub fn rescale<const TARGET_SCALE: u32>(self) -> $Type<TARGET_SCALE> {
                self.quantize::<TARGET_SCALE>()
            }

            /// Deprecated alias for [`Self::quantize_with`].
            ///
            /// Delegates unchanged; removed in 0.6.0.
            #[inline]
            #[must_use]
            #[deprecated(
                since = "0.5.1",
                note = "renamed to `quantize_with`; `rescale_with` is removed in 0.6.0"
            )]
            pub fn rescale_with<const TARGET_SCALE: u32>(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $Type<TARGET_SCALE> {
                self.quantize_with::<TARGET_SCALE>(mode)
            }
        }
    };

}

pub(crate) use decl_decimal_quantize;
