//! Correctly-rounded strict square root and cube root for the wide
//! decimal tiers (D256 / D512 / D1024).
//!
//! D128 and the narrow tiers get their roots from the 128/256/384-bit
//! integer machinery in `mg_divide.rs`, and D32 / D64 delegate into
//! D128. The wide tiers cannot widen into D128 — their scale range
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
//! - `<method>_strict` — always present unless the `no_strict` feature
//! is set. Integer-only; `no_std`-compatible.
//! - `<method>` — a dispatcher present only under
//! `#[cfg(all(feature = "strict", not(feature = "no_strict")))]`,
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
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn sqrt_strict(self) -> Self {
                let raw = self.to_bits();
                if raw <= $crate::macros::wide_roots::wide_lit!($Storage, "0") {
                    return Self::ZERO;
                }
                let one = $crate::macros::wide_roots::wide_lit!($SqrtWide, "1");
                let ten = $crate::macros::wide_roots::wide_lit!($SqrtWide, "10");
                // N = r · 10^SCALE, formed exactly one width up.
                let n: $SqrtWide = raw.resize::<$SqrtWide>() * ten.pow(SCALE);
                // q = floor(sqrt(N)). Round up to q+1 iff N is closer to
                // (q+1)² than to q², i.e. iff N − q² > q.
                let q = n.isqrt();
                let diff = n - q * q;
                let q = if diff > q { q + one } else { q };
                Self::from_bits(q.resize::<$Storage>())
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
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn cbrt_strict(self) -> Self {
                let raw = self.to_bits();
                let storage_zero = $crate::macros::wide_roots::wide_lit!($Storage, "0");
                if raw == storage_zero {
                    return Self::ZERO;
                }
                let zero = $crate::macros::wide_roots::wide_lit!($CbrtWide, "0");
                let one = $crate::macros::wide_roots::wide_lit!($CbrtWide, "1");
                let three = $crate::macros::wide_roots::wide_lit!($CbrtWide, "3");
                let ten = $crate::macros::wide_roots::wide_lit!($CbrtWide, "10");

                // Work on the magnitude two widths up; the radicand
                // `r · 10^(2·SCALE)` needs the extra room.
                let widened = raw.resize::<$CbrtWide>();
                let negative = widened < zero;
                let mag = if negative { -widened } else { widened };
                let n: $CbrtWide = mag * ten.pow(2 * SCALE);

                // q = floor(cbrt(N)) by integer Newton iteration:
                // x_{k+1} = (2·x_k + N / x_k²) / 3, started from an
                // overestimate so the sequence decreases monotonically.
                let sig_bits = <$CbrtWide>::BITS - n.leading_zeros();
                let mut x = one << sig_bits.div_ceil(3);
                loop {
                    let y = (x + x + n / (x * x)) / three;
                    if y >= x {
                        break;
                    }
                    x = y;
                }
                let q = x;

                // Round up to q+1 iff N is closer to (q+1)³ than to q³.
                // Multiplying by 8: 8·N ≥ (2q + 1)³.
                let eight_n = n << 3u32;
                let t = q + q + one;
                let q = if eight_n >= t * t * t { q + one } else { q };
                let signed = if negative { -q } else { q };
                Self::from_bits(signed.resize::<$Storage>())
            }

            /// Square root. With `strict` enabled this is the
            /// integer-only, correctly-rounded [`Self::sqrt_strict`].
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                self.sqrt_strict()
            }

            /// Cube root. With `strict` enabled this is the
            /// integer-only, correctly-rounded [`Self::cbrt_strict`].
            #[cfg(all(feature = "strict", not(feature = "no_strict")))]
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
            #[cfg(not(feature = "no_strict"))]
            #[inline]
            #[must_use]
            pub fn hypot_strict(self, other: Self) -> Self {
                let a = self.abs();
                let b = other.abs();
                let (large, small) = if a >= b { (a, b) } else { (b, a) };
                if large == Self::ZERO {
                    Self::ZERO
                } else {
                    let ratio = small / large;
                    let one_plus_sq = Self::ONE + ratio * ratio;
                    large * one_plus_sq.sqrt_strict()
                }
            }
        }
    };
}

pub(crate) use {decl_wide_roots, wide_lit};

#[cfg(all(test, not(feature = "no_strict")))]
mod tests {
    use crate::{D128, D256, D512, D1024};

    #[test]
    fn sqrt_perfect_squares_are_exact() {
        assert_eq!(D256::<6>::from_int(4).sqrt_strict(), D256::<6>::from_int(2));
        assert_eq!(D256::<6>::from_int(9).sqrt_strict(), D256::<6>::from_int(3));
        assert_eq!(
            D256::<6>::from_int(144).sqrt_strict(),
            D256::<6>::from_int(12)
        );
        assert_eq!(D512::<6>::from_int(25).sqrt_strict(), D512::<6>::from_int(5));
        assert_eq!(
            D1024::<6>::from_int(81).sqrt_strict(),
            D1024::<6>::from_int(9)
        );
    }

    #[test]
    fn sqrt_zero_and_negative_saturate() {
        assert_eq!(D256::<6>::ZERO.sqrt_strict(), D256::<6>::ZERO);
        assert_eq!(D256::<6>::from_int(-4).sqrt_strict(), D256::<6>::ZERO);
        assert_eq!(D1024::<6>::from_int(-1).sqrt_strict(), D1024::<6>::ZERO);
    }

    #[test]
    fn cbrt_perfect_cubes_are_exact() {
        assert_eq!(D256::<6>::from_int(8).cbrt_strict(), D256::<6>::from_int(2));
        assert_eq!(
            D256::<6>::from_int(27).cbrt_strict(),
            D256::<6>::from_int(3)
        );
        assert_eq!(
            D256::<6>::from_int(-8).cbrt_strict(),
            D256::<6>::from_int(-2)
        );
        assert_eq!(
            D512::<6>::from_int(125).cbrt_strict(),
            D512::<6>::from_int(5)
        );
        assert_eq!(
            D1024::<6>::from_int(-64).cbrt_strict(),
            D1024::<6>::from_int(-4)
        );
    }

    #[test]
    fn cbrt_zero_is_zero() {
        assert_eq!(D256::<6>::ZERO.cbrt_strict(), D256::<6>::ZERO);
        assert_eq!(D512::<6>::ZERO.cbrt_strict(), D512::<6>::ZERO);
        assert_eq!(D1024::<6>::ZERO.cbrt_strict(), D1024::<6>::ZERO);
    }

    /// The wide-tier roots are correctly rounded, so for any scale the
    /// D128 and D256 results must agree bit-for-bit (both land on the
    /// IEEE-754 round-to-nearest value).
    #[test]
    fn wide_roots_match_d128() {
        for raw in [2i64, 3, 5, 7, 10, 123, 1_000, 999_983] {
            let narrow = D128::<6>::from_int(raw);
            let wide: D256<6> = narrow.into();
            let narrow_sqrt: D256<6> = narrow.sqrt_strict().into();
            assert_eq!(wide.sqrt_strict(), narrow_sqrt, "sqrt mismatch for {raw}");
            let narrow_cbrt: D256<6> = narrow.cbrt_strict().into();
            assert_eq!(wide.cbrt_strict(), narrow_cbrt, "cbrt mismatch for {raw}");
        }
    }

    /// Exercises a scale beyond D128's range, where delegation is
    /// impossible and the wide path is the only implementation.
    #[test]
    fn sqrt_cbrt_at_wide_only_scale() {
        // D256<50>: 4.0 -> 2.0, 8.0 -> 2.0.
        assert_eq!(
            D256::<50>::from_int(4).sqrt_strict(),
            D256::<50>::from_int(2)
        );
        assert_eq!(
            D256::<50>::from_int(8).cbrt_strict(),
            D256::<50>::from_int(2)
        );
        // D1024<150>: well past any narrower tier.
        assert_eq!(
            D1024::<150>::from_int(9).sqrt_strict(),
            D1024::<150>::from_int(3)
        );
        assert_eq!(
            D1024::<150>::from_int(27).cbrt_strict(),
            D1024::<150>::from_int(3)
        );
    }
}
