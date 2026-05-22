//! Macro-generated arithmetic operator overloads for the decimal
//! widths that use a *uniform* mul/div pattern (D9, D18, and the wide
//! tier D76 / D153 / D307).
//!
//! For D9 / D18 the storage type is a primitive (`i32` / `i64`) and a
//! native wider integer (`i64` / `i128`) carries the mul/div widening
//! step. For D76 / D153 / D307 the storage type is a hand-rolled wide integer
//! fixed-width integer and the widening type is the next size up
//! up; the only thing that changes is *how* the `10^SCALE` literal and
//! the width casts are spelled.
//!
//! D38 is the exception: its mul/div go through the hand-rolled
//! 256-bit `mg_divide` path and are not generated here.
//!
//! Add / Sub / Neg / Rem (and their `*Assign` forms) are identical for
//! both storage kinds and live in the shared `@common` arm. Only Mul
//! and Div differ, so they are written inline in each front-end arm —
//! that keeps `self` / `rhs` in the same macro hygiene context as the
//! method signature.
//!
//! Overflow semantics mirror Rust's default integer arithmetic:
//! debug-mode panic on overflow, release-mode wrap. Explicit-overflow
//! variants (`checked_*`, `saturating_*`, `wrapping_*`) live in a
//! companion module.

/// Rounds `n / m` (truncating-toward-zero quotient) according to
/// `$mode` (a [`RoundingMode`]) for *primitive* signed integer types
/// (`i32` / `i64` / `i128`).
///
/// Mode-specific behaviour is delegated to
/// [`crate::support::rounding::should_bump`], which receives the three
/// pre-computed inputs every mode needs: the `|r|` vs `|m|−|r|`
/// ordering (the round-up test without the `2·|r|` overflow risk),
/// the parity of the truncated quotient, and the result sign. The
/// caller bumps the quotient by ±1 in the result direction.
///
/// Passing `crate::support::rounding::DEFAULT_ROUNDING_MODE` yields the
/// crate-wide default (IEEE-754 round-half-to-even unless a
/// `rounding-*` feature overrides it).
///
/// [`RoundingMode`]: crate::support::rounding::RoundingMode
macro_rules! round_with_mode_native {
    ($n:expr, $m:expr, $mode:expr) => {{
        let n = $n;
        let m = $m;
        let mode = $mode;
        let q = n / m;
        let r = n % m;
        if r == 0 {
            q
        } else {
            let abs_r = if r < 0 { -r } else { r };
            let abs_m = if m < 0 { -m } else { m };
            let comp = abs_m - abs_r;
            let cmp_r = abs_r.cmp(&comp);
            let q_is_odd = (q & 1) != 0;
            let result_positive = (n < 0) == (m < 0);
            if $crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                if result_positive { q + 1 } else { q - 1 }
            } else {
                q
            }
        }
    }};
}
pub(crate) use round_with_mode_native;

/// Divides a signed `i128` magnitude-bearing numerator by an unsigned
/// `u64` divisor magnitude using two hardware `divq` instructions (one
/// when the high half of the magnitude is zero), then applies `mode`
/// to the truncated quotient via the shared `should_bump` strategy.
///
/// **Why this exists:** the obvious `n_i128 / m_i128` lowers to LLVM's
/// `__divti3` soft-call (≈ 10 ns on x86-64) even when the divisor fits
/// `u64`. At D18 SCALE ≥ 10 the rebalance divisor is `10^SCALE ≤ 10^18 <
/// 2^64`, so a u128/u64 schoolbook divide in base 2^64 — exactly the
/// trick `mg_divide`'s SCALE ≤ 19 fast path uses — replaces the soft
/// call with two hardware divides, cutting the D18 mul/div cost ~60%.
///
/// Returns the signed quotient. The caller asserts the result fits the
/// destination storage type's range; for D18 the divisor is `10^SCALE`
/// (mul) or `rhs.0.unsigned_abs() as u64` (div) and the quotient fits
/// `i64` by construction.
///
/// # Algorithm reference
///
/// Knuth, *The Art of Computer Programming, Vol. 2: Seminumerical
/// Algorithms*, Section 4.3.1, Algorithm D ("Schoolbook" division of
/// nonnegative integers). The two-limb case reduces to two single-limb
/// divides over base `2^64`.
///
/// # Precision
///
/// Strict: identical bit-for-bit result to `round_with_mode_native!`
/// at the same `(n, m, mode)`. Proof: both compute
/// `n.signum() * (|n| / m_mag)` for the truncated quotient (Rust signed
/// `/` truncates toward zero, identical to `(-|n|/m_mag) * sign(n)`
/// when `m > 0`), and both feed the same `(cmp_r, q_is_odd,
/// result_positive)` triple to `should_bump`.
#[inline(always)]
pub(crate) fn i128_divrem_by_u64_with_mode(
    n: i128,
    m_mag: u64,
    mode: crate::support::rounding::RoundingMode,
) -> i128 {
    debug_assert!(m_mag != 0, "i128_divrem_by_u64_with_mode: m_mag = 0");
    let n_neg = n < 0;
    let un = n.unsigned_abs();
    let (q_mag, r_mag) = {
        let hi = (un >> 64) as u64;
        let lo = un as u64;
        if hi == 0 {
            // Single-limb dividend — one hardware `divq`.
            let q = lo / m_mag;
            let r = lo % m_mag;
            (q as u128, r)
        } else {
            // Two-limb schoolbook divide in base 2^64. Two hardware
            // divides.
            let q_hi = hi / m_mag;
            let r_hi = hi % m_mag;
            let cur = ((r_hi as u128) << 64) | (lo as u128);
            // The divisor fits u64, so the quotient of (cur / m_mag)
            // also fits u64 (cur < m_mag * 2^64).
            let q_lo_u128 = cur / (m_mag as u128);
            let r = cur - q_lo_u128 * (m_mag as u128);
            let q = ((q_hi as u128) << 64) | (q_lo_u128 & u128::from(u64::MAX));
            (q, r as u64)
        }
    };

    if r_mag == 0 {
        // No remainder — exact. Restore sign.
        return if n_neg {
            -(q_mag as i128)
        } else {
            q_mag as i128
        };
    }

    // `should_bump` needs the same three pre-computed inputs the macro
    // builds. `m_mag` is the divisor magnitude, never zero, never
    // negative.
    let abs_r = r_mag as u128;
    let abs_m = m_mag as u128;
    let comp = abs_m - abs_r;
    let cmp_r = abs_r.cmp(&comp);
    let q_is_odd = (q_mag & 1) != 0;
    let result_positive = !n_neg;
    let bump = crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive);
    let bumped_mag = if bump { q_mag + 1 } else { q_mag };
    if n_neg {
        -(bumped_mag as i128)
    } else {
        bumped_mag as i128
    }
}

/// Wide-storage counterpart of [`round_with_mode_native!`] — the same
/// strategy-pattern dispatch over [`crate::support::rounding::should_bump`],
/// adapted to a hand-rolled wide integer `$W`. Uses
/// `<$W>::from_i128(0/1)` for the small constants and the type's
/// operators throughout.
#[cfg(any(
    feature = "d76",
    feature = "d153",
    feature = "d307",
    feature = "wide",
    feature = "x-wide"
))]
macro_rules! round_with_mode_wide {
    ($n:expr, $m:expr, $W:ty, $mode:expr) => {{
        let n = $n;
        let m = $m;
        let mode = $mode;
        // Single divmod call instead of `n / m` + `n % m` (which
        // would do the full multi-limb divide twice).
        let (q, r) = n.div_rem(m);
        let zero = <$W>::from_i128(0);
        if r == zero {
            q
        } else {
            let one = <$W>::from_i128(1);
            let abs_r = if r < zero { -r } else { r };
            let abs_m = if m < zero { -m } else { m };
            let comp = abs_m - abs_r;
            let cmp_r = abs_r.cmp(&comp);
            let q_is_odd = {
                let two = <$W>::from_i128(2);
                (q % two) != zero
            };
            let result_positive = (n < zero) == (m < zero);
            if $crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
                if result_positive { q + one } else { q - one }
            } else {
                q
            }
        }
    }};
}
#[cfg(any(
    feature = "d76",
    feature = "d153",
    feature = "d307",
    feature = "wide",
    feature = "x-wide"
))]
pub(crate) use round_with_mode_wide;

/// Generates the standard arithmetic operator overloads for a decimal
/// width `$Type<SCALE>`.
///
/// - `decl_decimal_arithmetic!(D9, i32, i64)` — *native* storage; the
/// widening type is a primitive integer, `as`-casts and the
/// `(10 as $Wider)` literal carry the mul/div step.
/// - `decl_decimal_arithmetic!(wide D76, I256, I512)` — *wide*
/// storage; the widening type is a hand-rolled wide integer, the `BigInt` cast
/// carries the width casts and `from_str_radix` builds the
/// `10^SCALE` factor.
macro_rules! decl_decimal_arithmetic {
    // Wide storage.
    (wide $Type:ident, $Storage:ty, $Wider:ty) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);

        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `$Wider`
            /// to hold `a · b` exactly, divides by `10^SCALE` using the
            /// crate-default [`RoundingMode`] (IEEE-754 round-to-nearest;
            /// within 0.5 ULP), and narrows back to `$Storage`. See
            /// [`Self::mul_with`] to choose a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> ::core::ops::MulAssign for $Type<SCALE> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Div for $Type<SCALE> {
            type Output = Self;
            /// Divide two values of the same scale using the crate-default
            /// [`RoundingMode`] (within 0.5 ULP). Numerator is widened to
            /// `$Wider`, multiplied by `10^SCALE`, then divided by `b`
            /// preserving the `value · 10^SCALE` form. See
            /// [`Self::div_with`] for a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Multiply two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Result is
            /// within 0.5 ULP for the half-* family and bounded by the
            /// directed-rounding rule otherwise.
            ///
            /// For `SCALE ≤ 38` the divide-by-`10^SCALE` step routes
            /// through the Möller-Granlund magic-divide kernel shared
            /// with D38 — avoiding the generic schoolbook divide for
            /// the common case. Larger scales fall through to the
            /// slower `n / (10^SCALE)` path.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                // Fast path: if the product fits `$Storage` exactly,
                // skip the widen → mg_divide-in-`$Wider` → narrow
                // chain. The check is one `leading_zeros` per operand
                // on the storage type's `[u64; L]` limbs (4 ops for
                // Int256, 6 for Int384, …); negligible vs the
                // 4 × L² limb mul that follows. When the path is
                // taken we save (a) one set of `$Storage → $Wider`
                // resizes, (b) one `$Wider → $Storage` resize at
                // exit, and (c) the wider half of the MG divide's
                // entry / exit `mag_into_u128` / `from_mag_sign_u128`
                // limb copy.
                //
                // The condition `lz_a + lz_b > $Storage::BITS` is
                // sufficient for the unsigned magnitude product to
                // fit in `$Storage::BITS - 1` bits (i.e. the signed
                // sign-bit slot stays clear). `leading_zeros` on the
                // signed `$Storage` is computed over
                // `unsigned_abs(self).leading_zeros()` so it already
                // accounts for the sign-bit asymmetry; `.MIN` is the
                // only value with magnitude `2^(BITS - 1)`, and at
                // `lz = 0` the test fails so MIN takes the slow
                // path.
                let lz_a = self.0.leading_zeros();
                let lz_b = rhs.0.leading_zeros();
                if lz_a + lz_b > <$Storage>::BITS {
                    let n: $Storage = self.0.wrapping_mul(rhs.0);
                    let scaled = if SCALE == 0 {
                        n
                    } else if SCALE <= 38 {
                        $crate::algos::mg_divide::div_wide_pow10_with::<$Storage, { <$Storage as $crate::int::types::traits::BigInt>::U128_LIMBS }>(n, SCALE, mode)
                    } else {
                        // Newton vs MG chain dispatch: cells in the
                        // bench-validated matrix (Int2048 ≥ s200,
                        // Int3072 ≥ s200, Int4096 ≥ s400) route to
                        // Newton; everything else stays on MG. See
                        // [`crate::algos::newton_reciprocal::dispatch_wide_pow10_with`].
                        $crate::algos::newton_reciprocal::dispatch_wide_pow10_with::<$Storage, { <$Storage as $crate::int::types::traits::BigInt>::U128_LIMBS }>(n, SCALE, mode)
                    };
                    return Self(scaled);
                }

                // `widen_mul` does the `$Storage × $Storage → $Wider`
                // product in one step — no Int{2W} wrapping mul with
                // half-empty operands, and no double trip through the
                // 64-limb `BigInt::to_mag_sign` buffer.
                let n: $Wider = self.0.widen_mul::<$Wider>(rhs.0);
                let scaled = if SCALE == 0 {
                    n
                } else if SCALE <= 38 {
                    $crate::algos::mg_divide::div_wide_pow10_with::<$Wider, { <$Wider as $crate::int::types::traits::BigInt>::U128_LIMBS }>(n, SCALE, mode)
                } else {
                    // Width-dispatch as above; the slow path's `$Wider`
                    // numerator hits the same matrix (e.g. D307's
                    // `$Wider = Int2048` routes Newton at SCALE ≥ 200).
                    $crate::algos::newton_reciprocal::dispatch_wide_pow10_with::<$Wider, { <$Wider as $crate::int::types::traits::BigInt>::U128_LIMBS }>(n, SCALE, mode)
                };
                Self(scaled.resize::<$Storage>())
            }

            /// Divide two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            ///
            /// The divisor here is the runtime operand `rhs.0`, not
            /// `10^SCALE`, so the MG magic-divide doesn't apply; the
            /// final step uses the wide integer's schoolbook
            /// `limbs_divmod` (which has its own hardware fast paths
            /// for sub-word divisors). Scaling the numerator uses the
            /// type's `multiplier()` const (already evaluated at the
            /// `$Storage` width) widened to `$Wider`, avoiding the
            /// per-call `pow(SCALE)` on the wider type.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                // Fast path: when `self * 10^SCALE` fits `$Storage`
                // exactly (`leading_zeros(self) + leading_zeros(10^SCALE) >
                // $Storage::BITS`), skip the widen-to-$Wider chain
                // and divide in $Storage. The divisor `rhs` already
                // fits $Storage by construction. Saves one
                // $Storage→$Wider resize on `rhs`, one $Wider→$Storage
                // resize on the result, and shrinks the Knuth divmod
                // from $Wider-limbs to $Storage-limbs.
                //
                // `$Type::<SCALE>::multiplier()` is a `const` -- its
                // `leading_zeros()` collapses at compile time when
                // SCALE is a const, so the branch's predicate is one
                // `leading_zeros` call on `self.0`.
                let mult: $Storage = $Type::<SCALE>::multiplier();
                let lz_n = self.0.leading_zeros();
                let lz_m = mult.leading_zeros();
                if lz_n + lz_m > <$Storage>::BITS {
                    let n: $Storage = self.0.wrapping_mul(mult);
                    let result =
                        $crate::macros::arithmetic::round_with_mode_wide!(n, rhs.0, $Storage, mode);
                    return Self(result);
                }

                let b: $Wider = rhs.0.resize::<$Wider>();
                // `self.0 * multiplier()` both fit `$Storage` for any
                // representable `SCALE`, so the full product fits
                // `$Wider` exactly; `widen_mul` avoids the
                // resize-to-`$Wider` round trip on both operands.
                let n: $Wider = self.0.widen_mul::<$Wider>($Type::<SCALE>::multiplier());
                let result =
                    $crate::macros::arithmetic::round_with_mode_wide!(n, b, $Wider, mode);
                Self(result.resize::<$Storage>())
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // Native (primitive integer) storage — `i64`-widening branch (D9).
    //
    // Storage is `i32`, widening is `i64`. The rebalance divisor
    // `10^SCALE` (SCALE <= 9) and the runtime divisor `rhs.0`
    // (`|rhs.0| <= i32::MAX`) both fit a single 32-bit word, so the
    // `i64 / i64` divide LLVM emits is a single hardware `idivq` —
    // already optimal. No `i128_divrem_by_u64` fast path applies.
    ($Type:ident, $Storage:ty, i64) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@native_i64_wider $Type, $Storage);
    };

    // Native (primitive integer) storage — `i128`-widening branch (D18).
    //
    // Storage is `i64`, widening is `i128`. The naive `i128 / i128`
    // divide lowers to LLVM's `__divti3` soft-call (≈10 ns) even
    // though both the rebalance divisor `10^SCALE` (SCALE <= 18, fits
    // u64) and the runtime divisor `rhs.0.unsigned_abs()` (i64
    // magnitude, fits u64) are u64. Routing through
    // `i128_divrem_by_u64_with_mode` replaces the soft-call with
    // two hardware `divq` instructions, cutting D18 mul/div ~60%.
    ($Type:ident, $Storage:ty, i128) => {
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@common $Type, $Storage);
        $crate::macros::arithmetic::decl_decimal_arithmetic!(@native_i128_wider $Type, $Storage);
    };

    // i64-widening body: original code, unchanged.
    (@native_i64_wider $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `i64`
            /// to hold `a · b` exactly, divides by `10^SCALE` using the
            /// crate-default [`RoundingMode`] (IEEE-754 round-to-nearest;
            /// within 0.5 ULP), and narrows back to `$Storage`. See
            /// [`Self::mul_with`] to choose a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> ::core::ops::MulAssign for $Type<SCALE> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Div for $Type<SCALE> {
            type Output = Self;
            /// Divide two values of the same scale using the crate-default
            /// `RoundingMode` (within 0.5 ULP).
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Multiply two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let a = self.0 as i64;
                let b = rhs.0 as i64;
                let m = (10i64).pow(SCALE);
                let n = a * b;
                let scaled =
                    $crate::macros::arithmetic::round_with_mode_native!(n, m, mode);
                Self(scaled as $Storage)
            }

            /// Divide two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let a = self.0 as i64;
                let b = rhs.0 as i64;
                let m = (10i64).pow(SCALE);
                let n = a * m;
                let result =
                    $crate::macros::arithmetic::round_with_mode_native!(n, b, mode);
                Self(result as $Storage)
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // i128-widening body: u128/u64 schoolbook fast path.
    (@native_i128_wider $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::Mul for $Type<SCALE> {
            type Output = Self;
            /// Multiply two values of the same scale. Widens to `i128`
            /// to hold `a · b` exactly, divides by `10^SCALE` via the
            /// `u128/u64` schoolbook fast path (hardware `divq`, not
            /// LLVM `__divti3`), and narrows back to `$Storage`. See
            /// [`Self::mul_with`] to choose a non-default rounding mode.
            ///
            /// [`RoundingMode`]: $crate::support::rounding::RoundingMode
            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> ::core::ops::MulAssign for $Type<SCALE> {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Div for $Type<SCALE> {
            type Output = Self;
            /// Divide two values of the same scale. The numerator
            /// `a · 10^SCALE` is widened to `i128`; the final divide
            /// by `rhs.0` (an `i64`-storage operand) takes the
            /// `u128/u64` hardware-divide fast path. Within 0.5 ULP.
            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_with(rhs, $crate::support::rounding::DEFAULT_ROUNDING_MODE)
            }
        }

        impl<const SCALE: u32> $Type<SCALE> {
            /// Multiply two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            ///
            /// For `SCALE = 0` the multiplier is 1; the i128 product is
            /// returned directly. For `SCALE >= 1` the divide-by-
            /// `10^SCALE` step uses
            /// `i128_divrem_by_u64_with_mode`, replacing the
            /// `__divti3` soft-call with two hardware `divq` instructions.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let a = self.0 as i128;
                let b = rhs.0 as i128;
                let n = a * b;
                let scaled: i128 = if SCALE == 0 {
                    n
                } else {
                    // SCALE in 1..=18 implies 10^SCALE <= 10^18 < 2^60,
                    // fits u64. Compile-time constant, const-folded.
                    let m_mag: u64 = (10u64).pow(SCALE);
                    $crate::macros::arithmetic::i128_divrem_by_u64_with_mode(n, m_mag, mode)
                };
                Self(scaled as $Storage)
            }

            /// Divide two values of the same scale, rounding the
            /// scale-narrowing step according to `mode`. Within 0.5 ULP
            /// for the half-* family.
            ///
            /// The numerator is `a · 10^SCALE` as `i128`; the divisor
            /// is `rhs.0` cast to its unsigned-magnitude `u64`. The
            /// final divide is the same `u128/u64` schoolbook fast path
            /// that [`Self::mul_with`] uses.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                let a = self.0 as i128;
                let b = rhs.0 as i128;
                let m = (10i128).pow(SCALE);
                let n = a * m;
                // `rhs.0` is `i64`-sized storage; its magnitude fits
                // `u64`. Splitting sign from magnitude lets the
                // schoolbook divide take its single-instruction `divq`
                // fast path instead of the `__divti3` soft-call.
                let b_neg = b < 0;
                let b_mag: u64 = if b_neg {
                    // Two's-complement: for i64::MIN this is 2^63, fits u64.
                    (rhs.0 as i64).unsigned_abs()
                } else {
                    rhs.0 as u64
                };
                let q =
                    $crate::macros::arithmetic::i128_divrem_by_u64_with_mode(n, b_mag, mode);
                let result = if b_neg { -q } else { q };
                Self(result as $Storage)
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // Add / Sub / Neg / Rem and their assign forms — identical for
    // native and wide storage (the `core::ops` impls on the wide integers
    // match the primitive integer surface).
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::Add for $Type<SCALE> {
            type Output = Self;
            /// Add two values of the same scale.
            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::AddAssign for $Type<SCALE> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 = self.0 + rhs.0;
            }
        }

        impl<const SCALE: u32> ::core::ops::Sub for $Type<SCALE> {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::SubAssign for $Type<SCALE> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 = self.0 - rhs.0;
            }
        }

        impl<const SCALE: u32> ::core::ops::Neg for $Type<SCALE> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::Rem for $Type<SCALE> {
            type Output = Self;
            /// Remainder of two values at the same scale. Because both
            /// operands share the scale factor, the storage-level
            /// remainder is the answer with no rescaling.
            #[inline]
            fn rem(self, rhs: Self) -> Self {
                Self(self.0 % rhs.0)
            }
        }

        impl<const SCALE: u32> ::core::ops::RemAssign for $Type<SCALE> {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                self.0 = self.0 % rhs.0;
            }
        }
    };
}

pub(crate) use decl_decimal_arithmetic;
