//! Macro-generated arithmetic operator overloads for the decimal
//! widths that use a *uniform* mul/div pattern (D18, and the wide
//! tier D76 / D153 / D307).
//!
//! Every decimal width stores its value in an `Int<N>` and carries the
//! mul/div widening step in the next size up (`Int<1>`→`Int<2>` for D18,
//! `Int<2>`→`Int<4>` for the wide tier, …); the only thing that changes
//! across widths is *how* the `10^SCALE` literal and the width casts are
//! spelled.
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
//!
//! ## Overflow contract (how it is implemented)
//!
//! The decimal operators layer Rust's standard integer-overflow contract
//! on top of the `Int<N>` storage, whose own operators and kernels are
//! deliberately *modular* (wrapping) so the bignum algorithms stay
//! composable. The contract lives only here, at the decimal-operator
//! layer:
//!
//! * **Runtime (non-`const`) operators** — `Add` / `Sub` / `Neg` / `Rem`
//!   and the `Mul` / `Div` value paths — use the profile switch
//!   `if cfg!(debug_assertions) { checked … .expect("…overflow") } else {
//!   wrapping … }`: debug panics, release wraps. Built entirely from
//!   `Int<N>`'s existing `checked_*` / `wrapping_*` methods plus `core`
//!   `panic!` — no `std` dependency, so `no_std` still builds.
//! * **`const fn` paths** — any decimal-arith helper that is `const`
//!   (e.g. `abs`) cannot branch on `cfg!(debug_assertions)` and still be
//!   profile-correct, so it takes the stricter rule: an *unconditional*
//!   `checked_* … .expect(…)`. In a `const` context that lowers to a
//!   compile-time evaluation error on overflow (profile-independent),
//!   exactly matching `std`'s `const` integer arithmetic; at runtime it
//!   panics in both profiles.


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
/// Strict: identical bit-for-bit result to [`round_with_mode_wide!`]
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

/// Rounds a widened division residual using a strategy-pattern dispatch
/// over [`crate::support::rounding::should_bump`], expressed in terms of
/// the `Int<N>` storage `$W`. Uses `<$W>::from_i128(0/1)` for the small
/// constants and the type's operators throughout.
// Always available: D18 / D38 (default features) route their Div /
// checked_div / wrapping_div through this rounding step too.
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
pub(crate) use round_with_mode_wide;

/// Narrows a `$Wider`-typed result down to `$Storage`, applying the
/// runtime decimal-operator overflow contract: debug panics with
/// `$msg`, release wraps via `resize` (two's-complement truncation —
/// the historical behaviour, so release bit-patterns are unchanged).
///
/// The range test mirrors `checked_mul` / `checked_div`: out of range
/// iff `value > $Storage::MAX` or `value < $Storage::MIN` (compared in
/// `$Wider`).
macro_rules! narrow_or_panic {
    ($value:expr, $Storage:ty, $Wider:ty, $msg:expr) => {{
        let value: $Wider = $value;
        if cfg!(debug_assertions) {
            let storage_max: $Wider = <$Storage>::MAX.resize::<$Wider>();
            let storage_min: $Wider = <$Storage>::MIN.resize::<$Wider>();
            if value > storage_max || value < storage_min {
                panic!($msg);
            }
        }
        value.resize::<$Storage>()
    }};
}
pub(crate) use narrow_or_panic;

/// Generates the standard arithmetic operator overloads for a decimal
/// width `$Type<SCALE>`.
///
/// Invoked as `decl_decimal_arithmetic!(wide D76, Int<4>, Int<8>)`: the
/// storage is an `Int<N>` and the widening type the next size up. The
/// `BigInt` cast carries the width casts and `from_str_radix` builds the
/// `10^SCALE` factor (`Int<N>` has no `as` literal cast).
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
            ///
            /// Routes through the `MulPolicy` per-type policy trait.
            #[inline]
            pub fn mul_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                use $crate::policy::mul::MulPolicy as _;
                self.mul_impl(rhs, mode)
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
            ///
            /// Routes through the `DivPolicy` per-type policy trait.
            #[inline]
            pub fn div_with(self, rhs: Self, mode: $crate::support::rounding::RoundingMode) -> Self {
                use $crate::policy::div::DivPolicy as _;
                self.div_impl(rhs, mode)
            }
        }

        impl<const SCALE: u32> ::core::ops::DivAssign for $Type<SCALE> {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    };

    // Add / Sub / Neg / Rem and their assign forms — identical across the
    // `Int<N>` storage widths (the `core::ops` impls on the wide integers
    // match the primitive integer surface).
    //
    // Each operator routes through the corresponding policy trait method
    // (`AddPolicy::add_impl`, etc.) defined in `src/policy/`. The policy's
    // `const { select }` block folds per monomorphisation so the dispatch is
    // zero-cost in release. See `docs/ARCHITECTURE.md` → "Policy file
    // structure".
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::Add for $Type<SCALE> {
            type Output = Self;
            /// Add two values of the same scale.
            ///
            /// Follows Rust's standard integer-overflow contract: panics
            /// in debug builds, wraps (two's-complement) in release.
            /// Routes through the `AddPolicy` per-type policy trait.
            #[inline]
            fn add(self, rhs: Self) -> Self {
                use $crate::policy::add::AddPolicy as _;
                self.add_impl(rhs)
            }
        }

        impl<const SCALE: u32> ::core::ops::AddAssign for $Type<SCALE> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Sub for $Type<SCALE> {
            type Output = Self;
            /// Subtract two values of the same scale.
            ///
            /// Panics on overflow in debug builds, wraps in release.
            /// Routes through the `SubPolicy` per-type policy trait.
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                use $crate::policy::sub::SubPolicy as _;
                self.sub_impl(rhs)
            }
        }

        impl<const SCALE: u32> ::core::ops::SubAssign for $Type<SCALE> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<const SCALE: u32> ::core::ops::Neg for $Type<SCALE> {
            type Output = Self;
            /// Negate a value. Panics on overflow in debug builds
            /// (`-MIN` is unrepresentable in two's-complement), wraps in
            /// release (`-MIN == MIN`).
            /// Routes through the `NegPolicy` per-type policy trait.
            #[inline]
            fn neg(self) -> Self {
                use $crate::policy::neg::NegPolicy as _;
                self.neg_impl()
            }
        }

        impl<const SCALE: u32> ::core::ops::Rem for $Type<SCALE> {
            type Output = Self;
            /// Remainder of two values at the same scale. Because both
            /// operands share the scale factor, the storage-level
            /// remainder is the answer with no rescaling.
            ///
            /// Panics on the `MIN % -ONE` overflow boundary in debug
            /// builds, wraps in release (matching `i128::wrapping_rem`).
            /// Division by zero always panics.
            /// Routes through the `RemPolicy` per-type policy trait.
            #[inline]
            fn rem(self, rhs: Self) -> Self {
                use $crate::policy::rem::RemPolicy as _;
                self.rem_impl(rhs)
            }
        }

        impl<const SCALE: u32> ::core::ops::RemAssign for $Type<SCALE> {
            #[inline]
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
    };
}

pub(crate) use decl_decimal_arithmetic;
