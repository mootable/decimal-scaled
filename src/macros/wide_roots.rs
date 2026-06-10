// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Correctly-rounded strict square root and cube root for the wide
//! decimal tiers (D76 / D153 / D307).
//!
//! D38 and the narrow tiers get their roots from the 128/256/384-bit
//! integer machinery in `mg_divide.rs`, and D18 delegate into
//! D38. The wide tiers cannot widen into D38 — their scale range
//! exceeds it — so they compute roots directly on a hand-rolled wide integer one
//! or two sizes up.
//!
//! For a `D*<SCALE>` value with raw storage `r`, the logical value is
//! `r / 10^SCALE`, so:
//!
//! - the square-root raw storage is `round(sqrt(r · 10^SCALE))`;
//! - the cube-root raw storage is `round(cbrt(r · 10^(2·SCALE)))`.
//!
//! The radicand is formed exactly in a wider integer, the exact
//! integer root is taken, and a single round-to-nearest step lands the
//! result on the type's last representable place (within 0.5 ULP — the
//! IEEE-754 round-to-nearest result).
//!
//! Two surfaces are emitted per method, mirroring the rest of the
//! strict family:
//!
//! - `<method>_strict` — always present unless the `fast` feature
//! is set. Integer-only; `no_std`-compatible.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(all(feature = "strict", not(feature = "fast")))]`,
//! forwarding to `<method>_strict`. The wide tiers have no f64-bridge
//! transcendentals of their own, so there is no non-strict
//! `<method>` for these widths.

/// Builds a small wide-integer constant from a base-10 literal.
///
/// Building wide-integer constants in `const` context is done
/// integers have no `From<u8>`, so the const-fn `from_str_radix` is the
/// portable way to materialise a literal.
macro_rules! wide_lit {
    ($T:ty, $s:literal) => {
        match <$T>::from_str_radix($s, 10) {
            ::core::result::Result::Ok(v) => v,
            ::core::result::Result::Err(_) => {
                panic!(concat!("wide_roots: invalid base-10 literal ", $s))
            }
        }
    };
}

/// Emits the correctly-rounded strict `sqrt` / `cbrt` surface for a
/// wide decimal tier.
///
/// - `$Type` / `$Storage` — the decimal type and its wide storage.
/// - `$SqrtWide` — a hand-rolled wide integer wide enough to hold `r · 10^SCALE`.
/// - `$CbrtWide` — a hand-rolled wide integer wide enough to hold
/// `r · 10^(2·SCALE)`.
macro_rules! decl_wide_roots {
    ($Type:ident, $Storage:ty, $SqrtWide:ty, $CbrtWide:ty) => {
        impl<const SCALE: u32> $Type<SCALE> {
            /// Correctly-rounded square root.
            ///
            /// Negative inputs saturate to [`Self::ZERO`], matching the
            /// f64-bridge saturate-not-panic policy of the narrow tiers.
            ///
            /// # Precision
            ///
            /// Strict: integer-only; the result is the exact square
            /// root correctly rounded to the type's last place (within
            /// 0.5 ULP).
            #[inline]
            #[must_use]
            pub fn sqrt_strict(self) -> Self {
                self.sqrt_strict_with($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Square root under the supplied rounding mode. See the
            /// `D38::sqrt_strict_with` doc for the per-mode contract:
            /// ties are impossible for an integer radicand, so the
            /// three half-modes coincide.
            ///
            /// Body delegates to `policy::sqrt::SqrtPolicy::sqrt_impl`,
            /// which dispatches to the kernel registered for this
            /// `(width, SCALE)` cell in `crate::policy::sqrt`.
            #[inline]
            #[must_use]
            pub fn sqrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::policy::sqrt::dispatch::<_, SCALE>(self.0, mode))
            }

            /// Correctly-rounded cube root.
            ///
            /// Defined for the whole real line: `cbrt(-x) == -cbrt(x)`.
            ///
            /// # Precision
            ///
            /// Strict: integer-only; the result is the exact cube root
            /// correctly rounded to the type's last place (within 0.5
            /// ULP).
            #[inline]
            #[must_use]
            pub fn cbrt_strict(self) -> Self {
                self.cbrt_strict_with($crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Cube root under the supplied rounding mode. Sign is
            /// preserved; `Floor` / `Ceiling` bump magnitude only when
            /// the bump moves the signed result in their direction.
            ///
            /// Body delegates to `policy::cbrt::CbrtPolicy::cbrt_impl`.
            #[inline]
            #[must_use]
            pub fn cbrt_strict_with(self, mode: $crate::support::rounding::RoundingMode) -> Self {
                Self($crate::policy::cbrt::dispatch::<_, SCALE>(self.0, mode))
            }

            /// Square root. With `strict` enabled this is the
            /// integer-only, correctly-rounded [`Self::sqrt_strict`].
            #[cfg(all(feature = "strict", not(feature = "fast")))]
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                self.sqrt_strict()
            }

            /// Cube root. With `strict` enabled this is the
            /// integer-only, correctly-rounded [`Self::cbrt_strict`].
            #[cfg(all(feature = "strict", not(feature = "fast")))]
            #[inline]
            #[must_use]
            pub fn cbrt(self) -> Self {
                self.cbrt_strict()
            }

            /// `sqrt(self² + other²)` without intermediate overflow,
            /// computed integer-only via the correctly-rounded
            /// [`Self::sqrt_strict`]. Uses the scale-trick algorithm:
            ///
            /// ```text
            /// hypot(a, b) = max(|a|,|b|) · sqrt(1 + (min(|a|,|b|)/max(|a|,|b|))²)
            /// ```
            ///
            /// The `min/max` ratio lies in `[0, 1]`, so `ratio² + 1` is
            /// always in `[1, 2]` — the inner sqrt never overflows. The
            /// outer multiply by `large` only overflows when the true
            /// hypotenuse genuinely exceeds the type's range.
            ///
            /// `hypot(0, 0) = 0` (bit-exact); `hypot(0, x) = |x|`.
            #[inline]
            #[must_use]
            pub fn hypot_strict(self, other: Self) -> Self {
                self.hypot_strict_with(other, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }

            /// Hypot under the supplied rounding mode. The mode applies
            /// to the inner sqrt step.
            ///
            /// Body delegates *down* to
            /// `policy::hypot::HypotPolicy::hypot_impl`, which routes to
            /// the `algos::hypot::hypot_pythagoras` algorithm. The impl
            /// lives in the algorithm, not in this method.
            #[inline]
            #[must_use]
            pub fn hypot_strict_with(
                self,
                other: Self,
                mode: $crate::support::rounding::RoundingMode,
            ) -> Self {
                Self($crate::policy::hypot::dispatch::<_, SCALE>(self.0, other.0, mode))
            }
        }
    };
}

pub(crate) use {decl_wide_roots, wide_lit};

#[cfg(all(test, not(feature = "fast")))]
mod tests {

    #[test]
    fn sqrt_perfect_squares_are_exact() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(4).sqrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::from_int(2));
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(9).sqrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::from_int(3));
        assert_eq!(
            crate::D::<crate::int::types::Int<4>, 6>::from_int(144).sqrt_strict(),
            crate::D::<crate::int::types::Int<4>, 6>::from_int(12)
        );
        assert_eq!(
            crate::D::<crate::int::types::Int<8>, 6>::from_int(25).sqrt_strict(),
            crate::D::<crate::int::types::Int<8>, 6>::from_int(5)
        );
        assert_eq!(
            crate::D::<crate::int::types::Int<16>, 6>::from_int(81).sqrt_strict(),
            crate::D::<crate::int::types::Int<16>, 6>::from_int(9)
        );
    }

    #[test]
    fn sqrt_zero_and_negative_saturate() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.sqrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(-4).sqrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::from_int(-1).sqrt_strict(), crate::D::<crate::int::types::Int<16>, 6>::ZERO);
    }

    #[test]
    fn cbrt_perfect_cubes_are_exact() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(8).cbrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::from_int(2));
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(27).cbrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::from_int(3));
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::from_int(-8).cbrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::from_int(-2));
        assert_eq!(
            crate::D::<crate::int::types::Int<8>, 6>::from_int(125).cbrt_strict(),
            crate::D::<crate::int::types::Int<8>, 6>::from_int(5)
        );
        assert_eq!(
            crate::D::<crate::int::types::Int<16>, 6>::from_int(-64).cbrt_strict(),
            crate::D::<crate::int::types::Int<16>, 6>::from_int(-4)
        );
    }

    #[test]
    fn cbrt_zero_is_zero() {
        assert_eq!(crate::D::<crate::int::types::Int<4>, 6>::ZERO.cbrt_strict(), crate::D::<crate::int::types::Int<4>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<8>, 6>::ZERO.cbrt_strict(), crate::D::<crate::int::types::Int<8>, 6>::ZERO);
        assert_eq!(crate::D::<crate::int::types::Int<16>, 6>::ZERO.cbrt_strict(), crate::D::<crate::int::types::Int<16>, 6>::ZERO);
    }

    /// The wide-tier roots are correctly rounded, so for any scale the
    /// D38 and D76 results must agree bit-for-bit (both land on the
    /// IEEE-754 round-to-nearest value).
    #[test]
    fn wide_roots_match_d38() {
        for raw in [2i64, 3, 5, 7, 10, 123, 1_000, 999_983] {
            let narrow = crate::D::<crate::int::types::Int<2>, 6>::from_int(raw);
            let wide: crate::D<crate::int::types::Int<4>, 6> = narrow.into();
            let narrow_sqrt: crate::D<crate::int::types::Int<4>, 6> = narrow.sqrt_strict().into();
            assert_eq!(wide.sqrt_strict(), narrow_sqrt, "sqrt mismatch for {raw}");
            let narrow_cbrt: crate::D<crate::int::types::Int<4>, 6> = narrow.cbrt_strict().into();
            assert_eq!(wide.cbrt_strict(), narrow_cbrt, "cbrt mismatch for {raw}");
        }
    }

    /// Exercises a scale beyond D38's range, where delegation is
    /// impossible and the wide path is the only implementation.
    #[test]
    fn sqrt_cbrt_at_wide_only_scale() {
        // D76<50>: 4.0 -> 2.0, 8.0 -> 2.0.
        assert_eq!(crate::D::<crate::int::types::Int<4>, 50>::from_int(4).sqrt_strict(), crate::D::<crate::int::types::Int<4>, 50>::from_int(2));
        assert_eq!(crate::D::<crate::int::types::Int<4>, 50>::from_int(8).cbrt_strict(), crate::D::<crate::int::types::Int<4>, 50>::from_int(2));
        // D307<150>: well past any narrower tier.
        assert_eq!(
            crate::D::<crate::int::types::Int<16>, 150>::from_int(9).sqrt_strict(),
            crate::D::<crate::int::types::Int<16>, 150>::from_int(3)
        );
        assert_eq!(
            crate::D::<crate::int::types::Int<16>, 150>::from_int(27).cbrt_strict(),
            crate::D::<crate::int::types::Int<16>, 150>::from_int(3)
        );
    }
}
