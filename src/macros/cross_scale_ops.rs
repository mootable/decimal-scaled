// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Macro-generated cross-scale / cross-width constructor + comparator
//! methods (`mul_of`, `add_of`, `sub_of`, `div_of`, `rem_of`,
//! `max_of`, `min_of`, `clamp_of`, `cmp_of`, `eq_of`, `ne_of`,
//! `lt_of`, `le_of`, `gt_of`, `ge_of`) emitted on every decimal width.
//!
//! The constructors take operands of any width (≤ Self's storage)
//! and any SCALE, widen them to Self's storage, rescale to Self's
//! SCALE, then apply the matching same-width same-SCALE operator.
//! The comparators widen both sides to the wider storage and rescale
//! to the higher SCALE before comparing (lossless when both operands
//! fit Self's storage, which is enforced by the `WidthLE` bound).
//!
//! Each constructor has a `*_with(…, mode)` sibling that takes an
//! explicit [`crate::RoundingMode`]; the no-mode form delegates to it
//! with the crate's `DEFAULT_ROUNDING_MODE`. Same convention as
//! `rescale` / `rescale_with`.
//!
//! Same body for native and wide storage — only the rescale path
//! changes (the wide arm goes through the wide-int helpers via the
//! per-width `rescale_with` already in scope).

/// Emits the cross-scale-op surface on `$Type<SCALE>` with storage
/// `$Storage`. Used for every decimal width.
macro_rules! decl_decimal_cross_scale_ops {
    ($Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            // ── Multiply ─────────────────────────────────────────────

            /// Constructs `Self` as `a × b`, where `a` and `b` may have
            /// any width ≤ `Self`'s and any SCALE. Both operands are
            /// widened to `Self`'s storage, rescaled to `Self`'s
            /// `SCALE` (UP-rescale is exact; DOWN uses the crate's
            /// default rounding mode), then multiplied. Panics on overflow
            /// in both debug and release, matching the same-width
            /// same-SCALE [`Mul`](core::ops::Mul) operator.
            ///
            /// See [`Self::mul_of_with`] for an explicit-rounding form.
            #[inline]
            #[must_use]
            pub fn mul_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::mul_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::mul_of`] but with an explicit rounding mode
            /// for any DOWN-rescale of `a` / `b` to `Self`'s `SCALE`.
            #[inline]
            #[must_use]
            pub fn mul_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                let a_t: Self = a_w.rescale_with::<SCALE>(mode);
                let b_t: Self = b_w.rescale_with::<SCALE>(mode);
                a_t * b_t
            }

            // ── Add / Sub / Rem ──────────────────────────────────────

            /// Constructs `Self` as `a + b` (cross-width / cross-SCALE).
            /// See [`Self::mul_of`] for semantics.
            #[inline]
            #[must_use]
            pub fn add_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::add_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::add_of`] but with explicit rounding for the
            /// input rescale step.
            #[inline]
            #[must_use]
            pub fn add_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                let a_t: Self = a_w.rescale_with::<SCALE>(mode);
                let b_t: Self = b_w.rescale_with::<SCALE>(mode);
                a_t + b_t
            }

            /// Constructs `Self` as `a - b` (cross-width / cross-SCALE).
            #[inline]
            #[must_use]
            pub fn sub_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::sub_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::sub_of`] but with explicit rounding for the
            /// input rescale step.
            #[inline]
            #[must_use]
            pub fn sub_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                let a_t: Self = a_w.rescale_with::<SCALE>(mode);
                let b_t: Self = b_w.rescale_with::<SCALE>(mode);
                a_t - b_t
            }

            /// Constructs `Self` as `a / b` (cross-width / cross-SCALE).
            #[inline]
            #[must_use]
            pub fn div_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::div_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::div_of`] but with explicit rounding for both
            /// the input rescale step *and* the truncating divide.
            #[inline]
            #[must_use]
            pub fn div_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                let a_t: Self = a_w.rescale_with::<SCALE>(mode);
                let b_t: Self = b_w.rescale_with::<SCALE>(mode);
                a_t / b_t
            }

            /// Constructs `Self` as `a % b` (cross-width / cross-SCALE).
            #[inline]
            #[must_use]
            pub fn rem_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::rem_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::rem_of`] but with explicit rounding for the
            /// input rescale step.
            #[inline]
            #[must_use]
            pub fn rem_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                let a_t: Self = a_w.rescale_with::<SCALE>(mode);
                let b_t: Self = b_w.rescale_with::<SCALE>(mode);
                a_t % b_t
            }

            // ── max / min / clamp ────────────────────────────────────

            /// Returns the larger of `a` and `b` as `Self`. Comparison
            /// is done at the wider operand's `SCALE` (lossless via
            /// UP-rescale on both sides); the winner is then rescaled
            /// to `Self`'s `SCALE` using the crate's default rounding
            /// mode.
            #[inline]
            #[must_use]
            pub fn max_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::max_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::max_of`] but with an explicit rounding mode
            /// for the final rescale to `Self`'s `SCALE`.
            #[inline]
            #[must_use]
            pub fn max_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                // Lossless widen each into Self's storage.
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                // Compare at the larger of S1 / S2 — that scale-up is
                // exact on both sides, so the comparison is exact.
                if S1 >= S2 {
                    let b_at_s1: $crate::D<$Storage, S1> = b_w.rescale_with::<S1>(mode);
                    let winner: $crate::D<$Storage, S1> =
                        if a_w >= b_at_s1 { a_w } else { b_at_s1 };
                    winner.rescale_with::<SCALE>(mode)
                } else {
                    let a_at_s2: $crate::D<$Storage, S2> = a_w.rescale_with::<S2>(mode);
                    let winner: $crate::D<$Storage, S2> =
                        if a_at_s2 >= b_w { a_at_s2 } else { b_w };
                    winner.rescale_with::<SCALE>(mode)
                }
            }

            /// Returns the smaller of `a` and `b` as `Self`. See
            /// [`Self::max_of`] for the comparison + rescale semantics.
            #[inline]
            #[must_use]
            pub fn min_of<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                Self::min_of_with(a, b, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Like [`Self::min_of`] but with an explicit rounding mode
            /// for the final rescale to `Self`'s `SCALE`.
            #[inline]
            #[must_use]
            pub fn min_of_with<W1, W2, const S1: u32, const S2: u32>(
                a: $crate::D<W1, S1>,
                b: $crate::D<W2, S2>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
            {
                let a_w: $crate::D<$Storage, S1> =
                    $crate::D::<$Storage, S1>(<W1 as $crate::WidthLE<$Storage>>::widen_into(a.0));
                let b_w: $crate::D<$Storage, S2> =
                    $crate::D::<$Storage, S2>(<W2 as $crate::WidthLE<$Storage>>::widen_into(b.0));
                if S1 >= S2 {
                    let b_at_s1: $crate::D<$Storage, S1> = b_w.rescale_with::<S1>(mode);
                    let winner: $crate::D<$Storage, S1> =
                        if a_w <= b_at_s1 { a_w } else { b_at_s1 };
                    winner.rescale_with::<SCALE>(mode)
                } else {
                    let a_at_s2: $crate::D<$Storage, S2> = a_w.rescale_with::<S2>(mode);
                    let winner: $crate::D<$Storage, S2> =
                        if a_at_s2 <= b_w { a_at_s2 } else { b_w };
                    winner.rescale_with::<SCALE>(mode)
                }
            }

            /// Clamps `value` to the inclusive range `[lo, hi]`,
            /// returning the result as `Self`. All three operands may
            /// have any width ≤ `Self`'s and any SCALE. The comparison
            /// is exact (each pairwise compare runs at the higher of
            /// the two scales, lossless); the result is then rescaled
            /// to `Self`'s `SCALE` using the crate's default rounding
            /// mode.
            ///
            /// Panics if `lo > hi` (matching `core::cmp::Ord::clamp`).
            #[inline]
            #[must_use]
            pub fn clamp_of<W1, W2, W3, const SV: u32, const SLO: u32, const SHI: u32>(
                value: $crate::D<W1, SV>,
                lo: $crate::D<W2, SLO>,
                hi: $crate::D<W3, SHI>,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
                W3: $crate::WidthLE<$Storage>,
            {
                Self::clamp_of_with(
                    value,
                    lo,
                    hi,
                    $crate::support::rounding::DEFAULT_ROUNDING_MODE,
                )
            }

            /// Like [`Self::clamp_of`] but with an explicit rounding
            /// mode for the rescale to `Self`'s `SCALE`.
            #[inline]
            #[must_use]
            pub fn clamp_of_with<W1, W2, W3, const SV: u32, const SLO: u32, const SHI: u32>(
                value: $crate::D<W1, SV>,
                lo: $crate::D<W2, SLO>,
                hi: $crate::D<W3, SHI>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self
            where
                W1: $crate::WidthLE<$Storage>,
                W2: $crate::WidthLE<$Storage>,
                W3: $crate::WidthLE<$Storage>,
            {
                // Pairwise: max(value, lo) then min(.., hi). Each step
                // delegates to max_of_with / min_of_with which already
                // pick the higher-scale common space for comparison.
                let v_after_lo: Self = Self::max_of_with(value, lo, mode);
                Self::min_of_with(v_after_lo, hi, mode)
            }

            // ── convert (cross-width + cross-scale, fallible) ────────

            /// Converts a source decimal of any storage width `W1` and
            /// any `SCALE` `S1` to `Self` (the target tier at the target
            /// `SCALE`), using the crate's default rounding mode for any
            /// scale-down step.
            ///
            /// Unlike the [`Self::mul_of`] family, the source width may be
            /// **wider** than `Self`'s — the conversion is fallible and
            /// returns [`ConvertError`] rather than requiring
            /// [`WidthLE`](crate::WidthLE).
            ///
            /// See [`Self::convert_from_with`] for the explicit-mode form,
            /// the value-preserving width/scale ordering, and the full
            /// list of error conditions.
            ///
            /// [`ConvertError`]: crate::support::error::ConvertError
            #[inline]
            pub fn convert_from<W1, const S1: u32>(
                src: $crate::D<W1, S1>,
            ) -> ::core::result::Result<Self, $crate::support::error::ConvertError>
            where
                W1: $crate::int::types::BigInt,
            {
                Self::convert_from_with(src, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Converts a source decimal of any storage width `W1` and
            /// any `SCALE` `S1` to `Self`, using `mode` for any
            /// scale-down rounding.
            ///
            /// The conversion composes a cross-width and a cross-scale
            /// step on the stored magnitude, ordered to be
            /// value-preserving: when `Self`'s storage is at least as
            /// wide as the source's, the magnitude is widened first and
            /// the scale change happens at the target width; when `Self`'s
            /// storage is narrower, the scale change happens at the source
            /// width first (so a value that is too large at `S1` but fits
            /// after a scale-down is not spuriously rejected) and the
            /// magnitude is narrowed afterwards. The branch is chosen by a
            /// compile-time limb-count comparison, so no nightly
            /// `generic_const_exprs` is required.
            ///
            /// # Rounding
            ///
            /// A scale-up (`S1 < SCALE`) is exact. A scale-down
            /// (`S1 > SCALE`) discards low fractional digits and rounds
            /// them per `mode`; this is **not** an error.
            ///
            /// # Errors
            ///
            /// Returns [`ConvertError::Overflow`] when the scaled
            /// magnitude does not fit `Self`'s storage — either a
            /// scale-up overflows the working width, or the
            /// (correctly-ordered) rescaled magnitude does not fit the
            /// narrower target storage.
            ///
            /// [`ConvertError::Overflow`]: crate::support::error::ConvertError::Overflow
            #[inline]
            pub fn convert_from_with<W1, const S1: u32>(
                src: $crate::D<W1, S1>,
                mode: $crate::support::rounding::RoundingMode,
            ) -> ::core::result::Result<Self, $crate::support::error::ConvertError>
            where
                W1: $crate::int::types::BigInt,
            {
                let mag: $Storage = $crate::int::convert::convert_magnitude::<W1, $Storage>(
                    src.0, S1, SCALE, mode,
                )?;
                ::core::result::Result::Ok(Self::from_bits(mag))
            }

            // ── Comparators (cmp_of / eq_of / ne_of / lt_of / le_of / gt_of / ge_of) ─

            /// Compares `self` against `other` of any width ≤ `Self`'s
            /// and any SCALE. Comparison is exact: both sides are
            /// widened to `Self`'s storage and UP-rescaled to the
            /// higher of `SCALE` / `S2` (both UP-rescales are lossless),
            /// then the storage `Ord` is invoked.
            #[inline]
            #[must_use]
            pub fn cmp_of<W2, const S2: u32>(
                self,
                other: $crate::D<W2, S2>,
            ) -> ::core::cmp::Ordering
            where
                W2: $crate::WidthLE<$Storage>,
            {
                let other_w: $crate::D<$Storage, S2> = $crate::D::<$Storage, S2>(
                    <W2 as $crate::WidthLE<$Storage>>::widen_into(other.0),
                );
                if SCALE >= S2 {
                    let other_at: Self = other_w.rescale::<SCALE>();
                    ::core::cmp::Ord::cmp(&self.0, &other_at.0)
                } else {
                    let self_at: $crate::D<$Storage, S2> = self.rescale::<S2>();
                    ::core::cmp::Ord::cmp(&self_at.0, &other_w.0)
                }
            }

            /// Returns `true` iff `self == other` (semantically; cross-
            /// width / cross-SCALE). See [`Self::cmp_of`] for the
            /// exactness contract.
            #[inline]
            #[must_use]
            pub fn eq_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) == ::core::cmp::Ordering::Equal
            }

            /// Returns `true` iff `self != other` (semantically).
            #[inline]
            #[must_use]
            pub fn ne_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) != ::core::cmp::Ordering::Equal
            }

            /// Returns `true` iff `self < other` (semantically).
            #[inline]
            #[must_use]
            pub fn lt_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) == ::core::cmp::Ordering::Less
            }

            /// Returns `true` iff `self <= other` (semantically).
            #[inline]
            #[must_use]
            pub fn le_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) != ::core::cmp::Ordering::Greater
            }

            /// Returns `true` iff `self > other` (semantically).
            #[inline]
            #[must_use]
            pub fn gt_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) == ::core::cmp::Ordering::Greater
            }

            /// Returns `true` iff `self >= other` (semantically).
            #[inline]
            #[must_use]
            pub fn ge_of<W2, const S2: u32>(self, other: $crate::D<W2, S2>) -> bool
            where
                W2: $crate::WidthLE<$Storage>,
            {
                self.cmp_of(other) != ::core::cmp::Ordering::Less
            }
        }
    };
}

pub(crate) use decl_decimal_cross_scale_ops;
