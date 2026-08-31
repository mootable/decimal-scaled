// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `requantize` — change storage width and `SCALE` in one call, any direction.
//!
//! [`quantize`](crate::macros::quantize) sets the quantum at a fixed width;
//! `widen`/`narrow` change width one tier at a fixed `SCALE`. `requantize` moves
//! both axes at once, to any width and any scale, in either direction.
//!
//! # Order matters
//!
//! Scaling UP multiplies, so doing it at a narrow width can overflow where the
//! target width would have held the value comfortably. `requantize` therefore
//! works at whichever width is safe:
//!
//! - **growing** (target width >= source): widen the storage first
//!   (sign-extending, infallible), then rescale at the wider width;
//! - **shrinking**: rescale at the source width first, then narrow.
//!
//! This is possible as ONE generic method because
//! [`rescale_bigint`](crate::int::convert::rescale_bigint) is generic over the
//! integer width, so the same call serves both branches.
//!
//! # Overflow
//!
//! Panics with the crate's standard overflow wording when the value does not fit
//! the target — the same contract as any other operation.

macro_rules! decl_decimal_requantize {
    ($Type:ident, $SrcLimbs:literal) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Changes storage width and `SCALE` together, using the crate's
            /// default rounding mode for any scale-down step.
            ///
            /// Panics if the value does not fit the target width.
            #[inline]
            #[must_use]
            pub fn requantize<const N: usize, const TARGET_SCALE: u32>(
                self,
            ) -> $crate::D<$crate::int::types::Int<N>, TARGET_SCALE> {
                self.requantize_with::<N, TARGET_SCALE>(
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// [`Self::requantize`] with an explicit rounding mode for the
            /// scale-down step.
            ///
            /// Panics if the value does not fit the target width.
            #[inline]
            #[must_use]
            pub fn requantize_with<const N: usize, const TARGET_SCALE: u32>(
                self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> $crate::D<$crate::int::types::Int<N>, TARGET_SCALE> {
                if N >= $SrcLimbs {
                    // Growing: widen first so the scale-up cannot overflow a
                    // width the target could have held.
                    let widened = self.0.resize_n::<N>();
                    let out = $crate::int::convert::rescale_bigint(
                        widened, SCALE, TARGET_SCALE, mode,
                    )
                    .expect("attempt to requantize with overflow");
                    $crate::D(out)
                } else {
                    // Shrinking: rescale at the source width, then narrow.
                    let scaled = $crate::int::convert::rescale_bigint(
                        self.0, SCALE, TARGET_SCALE, mode,
                    )
                    .expect("attempt to requantize with overflow");
                    let narrowed = scaled
                        .try_narrow::<N>()
                        .expect("attempt to requantize with overflow");
                    $crate::D(narrowed)
                }
            }
        }
    };
}

pub(crate) use decl_decimal_requantize;
