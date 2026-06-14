//! `checked_*` siblings of the strict transcendental family.
//!
//! One generic `impl` over `(N, SCALE)` — a single source serving every
//! width tier (D18 .. D1232), per the overflow contract in
//! `docs/ARCHITECTURE.md` ("Overflow & domain behaviour"): the default
//! strict form panics on a domain error or an out-of-range result; the
//! `checked_` form returns `None` instead. The two forms run the SAME
//! policy-dispatched kernel, so an in-range `checked_*` result is
//! bit-identical to the default form's result.
//!
//! # Shape
//!
//! Every strict transcendental gets the pair
//!
//! - `checked_<fn>_strict_with(self, .., mode) -> Option<Self>`
//! - `checked_<fn>_strict(self, ..) -> Option<Self>` — the default-mode
//!   sibling, delegating with [`DEFAULT_ROUNDING_MODE`].
//!
//! Only the **strict** forms get checked siblings: the architecture's
//! claim covers the strict transcendentals (the f64-bridge `*_fast`
//! forms have no panic contract to opt out of — they saturate — and the
//! `*_approx` forms trade away the strict guarantee by construction).
//!
//! # What `None` covers, per method class
//!
//! - **Total methods** (`sqrt`, `cbrt`, `sin`, `cos`, `atan`, `atan2`,
//!   `tanh`, `asinh`, `to_radians`): the default form cannot panic — the
//!   result is mathematically bounded well inside every tier's range at
//!   every valid scale (each method's doc carries the bound) — so the
//!   checked form always returns `Some`.
//! - **Domain-checked methods** (`asin`, `acos`, `acosh`, `ln`, `log`,
//!   `log2`, `log10`, `atanh`): `None` exactly on the inputs the default
//!   form rejects with a domain panic.
//! - **Range-checked methods** (`exp`, `ln`, `hypot`, …): `None` when
//!   the correctly-rounded result does not fit the storage range — the
//!   same single detection point whose `unwrap` is the default form's
//!   panic (see the per-policy `checked_dispatch` primitives).
//!
//! A method's doc states which of these apply. Where the out-of-range
//! seam has not yet been threaded through a kernel family, the doc says
//! so explicitly: those methods still panic on an out-of-range result
//! (identically to the default form — never a silent wrong value).
//!
//! [`DEFAULT_ROUNDING_MODE`]: crate::support::rounding::DEFAULT_ROUNDING_MODE

use crate::int::types::compute_limbs::{ComputeLimbs, Limbs};
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, DEFAULT_ROUNDING_MODE};

// `private_bounds`: the sqrt / cbrt / hypot methods carry the same
// `Limbs<N>: ComputeLimbs` scratch bound their policy dispatchers do.
// The trait is crate-internal plumbing; at every concrete `Dxx<S>` the
// bound is auto-satisfied and invisible to callers, so the unnameable
// bound never surfaces in downstream code.
#[allow(private_bounds)]
impl<const N: usize, const SCALE: u32> crate::D<Int<N>, SCALE> {
    /// Raw-storage value of `1` at this scale (`10^SCALE`), the unit the
    /// domain walls compare against.
    #[inline]
    fn unit_bits() -> Int<N> {
        const { Int::<N>::TEN.pow(SCALE) }
    }

    // ── Logarithms ────────────────────────────────────────────────

    /// Checked [`ln_strict_with`](crate::types::widths::D38::ln_strict_with):
    /// natural logarithm, `None` instead of a panic.
    ///
    /// Returns `None` when `self <= 0` (the domain wall) or when the
    /// correctly-rounded result does not fit the storage range (possible
    /// only near a tier's maximum scale). Otherwise
    /// `Some(self.ln_strict_with(mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (kernel seam not yet reached).
    /// Domain errors return `None` at every tier.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let two = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     two.checked_ln_strict_with(RoundingMode::HalfToEven),
    ///     Some(two.ln_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ZERO.checked_ln_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_ln_strict_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_ln_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(10i64).unwrap().checked_ln_strict().is_some());
    /// assert_eq!(D38::<12>::try_from(-1i64).unwrap().checked_ln_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_ln_strict(self) -> Option<Self> {
        self.checked_ln_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log_strict_with`: logarithm in an arbitrary `base`,
    /// `None` instead of a panic.
    ///
    /// Returns `None` when `self <= 0`, `base <= 0`, or `base == 1`
    /// (the domain walls the default form panics on), or when the
    /// result does not fit the storage range. Otherwise
    /// `Some(self.log_strict_with(base, mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet
    /// reached). Domain errors return `None` at every tier.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let eight = D38::<10>::try_from(8i64).unwrap();
    /// let two = D38::<10>::try_from(2i64).unwrap();
    /// assert!(eight.checked_log_strict_with(two, RoundingMode::HalfToEven).is_some());
    /// assert_eq!(eight.checked_log_strict_with(D38::<10>::ONE, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log_strict_with(self, base: Self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO
            || base.0 <= Int::<N>::ZERO
            || base.0 == Self::unit_bits()
        {
            return None;
        }
        crate::policy::log::checked_dispatch::<N, SCALE>(self.0, base.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let x = D38::<10>::try_from(100i64).unwrap();
    /// assert!(x.checked_log_strict(D38::<10>::try_from(10i64).unwrap()).is_some());
    /// assert_eq!(x.checked_log_strict(D38::<10>::ZERO), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log_strict(self, base: Self) -> Option<Self> {
        self.checked_log_strict_with(base, DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log2_strict_with`: base-2 logarithm, `None` instead of
    /// a panic.
    ///
    /// Returns `None` when `self <= 0`, or when the result does not fit
    /// the storage range. Otherwise bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let eight = D38::<10>::try_from(8i64).unwrap();
    /// assert_eq!(
    ///     eight.checked_log2_strict_with(RoundingMode::HalfToEven),
    ///     Some(eight.log2_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<10>::ZERO.checked_log2_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log2_strict_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_log2_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log2_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(4i64).unwrap().checked_log2_strict().is_some());
    /// assert_eq!(D38::<10>::try_from(-4i64).unwrap().checked_log2_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log2_strict(self) -> Option<Self> {
        self.checked_log2_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log10_strict_with`: base-10 logarithm, `None` instead
    /// of a panic.
    ///
    /// Returns `None` when `self <= 0`, or when the result does not fit
    /// the storage range. Otherwise bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let hundred = D38::<10>::try_from(100i64).unwrap();
    /// assert!(hundred.checked_log10_strict_with(RoundingMode::HalfToEven).is_some());
    /// assert_eq!(D38::<10>::ZERO.checked_log10_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log10_strict_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_log10_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log10_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(1000i64).unwrap().checked_log10_strict().is_some());
    /// assert_eq!(D38::<10>::ZERO.checked_log10_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log10_strict(self) -> Option<Self> {
        self.checked_log10_strict_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Exponentials ──────────────────────────────────────────────

    /// Checked `exp_strict_with`: `e^self`, `None` instead of a panic.
    ///
    /// `exp` has no domain wall; `None` means the correctly-rounded
    /// result does not fit the storage range — the same condition on
    /// which the default form panics. Otherwise
    /// `Some(self.exp_strict_with(mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (kernel seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_exp_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.exp_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// // e^120 has 53 integer digits — far outside D38's 38.
    /// assert_eq!(D38::<12>::try_from(120i64).unwrap().checked_exp_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::exp::checked_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_exp_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_exp_strict().is_some());
    /// assert_eq!(D38::<12>::try_from(120i64).unwrap().checked_exp_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp_strict(self) -> Option<Self> {
        self.checked_exp_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `exp2_strict_with`: `2^self`, `None` instead of a panic.
    ///
    /// No domain wall; `None` means the result does not fit the storage
    /// range. Otherwise bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let ten = D38::<12>::try_from(10i64).unwrap();
    /// assert_eq!(
    ///     ten.checked_exp2_strict_with(RoundingMode::HalfToEven),
    ///     Some(ten.exp2_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// // 2^95 has 29 integer digits — outside D38<12>'s 26.
    /// assert_eq!(D38::<12>::try_from(95i64).unwrap().checked_exp2_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp2_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::exp::checked_exp2_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_exp2_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(10i64).unwrap().checked_exp2_strict().is_some());
    /// assert_eq!(D38::<12>::try_from(95i64).unwrap().checked_exp2_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp2_strict(self) -> Option<Self> {
        self.checked_exp2_strict_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Power ─────────────────────────────────────────────────────

    /// Checked `powf_strict_with`: `self^exp`, `None` instead of a
    /// panic.
    ///
    /// `powf` has no domain panic: a non-positive base saturates to
    /// zero (the kernel's documented behaviour at every tier), so
    /// `checked_powf` returns `Some(ZERO)` there, matching the default
    /// form. `None` means the result does not fit the storage range.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let three = D38::<10>::try_from(3i64).unwrap();
    /// let two = D38::<10>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     three.checked_powf_strict_with(two, RoundingMode::HalfToEven),
    ///     Some(three.powf_strict_with(two, RoundingMode::HalfToEven)),
    /// );
    /// // Non-positive base saturates to zero, as the default form does.
    /// let half = D38::<10>::ONE / two;
    /// assert_eq!(
    ///     (-three).checked_powf_strict_with(half, RoundingMode::HalfToEven),
    ///     Some(D38::<10>::ZERO),
    /// );
    /// // 10^30 has 31 integer digits — out of D38<10>'s 28.
    /// let ten = D38::<10>::try_from(10i64).unwrap();
    /// let thirty = D38::<10>::try_from(30i64).unwrap();
    /// assert_eq!(ten.checked_powf_strict_with(thirty, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_powf_strict_with(self, exp: Self, mode: RoundingMode) -> Option<Self> {
        crate::policy::pow::checked_dispatch::<N, SCALE>(self.0, exp.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_powf_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let x = D38::<10>::try_from(2i64).unwrap();
    /// assert!(x.checked_powf_strict(D38::<10>::try_from(8i64).unwrap()).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_powf_strict(self, exp: Self) -> Option<Self> {
        self.checked_powf_strict_with(exp, DEFAULT_ROUNDING_MODE)
    }

    // ── Roots ─────────────────────────────────────────────────────

    /// Checked `sqrt_strict_with`. Always `Some`: the strict square
    /// root is total — negative inputs saturate to zero (the policy's
    /// documented behaviour, not a panic), and the result `√v ≤
    /// max(v, 1)` always fits the storage range. The checked form
    /// exists for surface uniformity.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let nine = D38::<10>::try_from(9i64).unwrap();
    /// assert_eq!(
    ///     nine.checked_sqrt_strict_with(RoundingMode::HalfToEven),
    ///     Some(nine.sqrt_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sqrt_strict_with(self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        Some(Self(crate::policy::sqrt::dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_sqrt_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(2i64).unwrap().checked_sqrt_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sqrt_strict(self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_sqrt_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cbrt_strict_with`. Always `Some`: the cube root is
    /// total over the signed domain and `∛v` never exceeds `max(|v|,
    /// 1)`, so it always fits the storage range.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<10>::try_from(-27i64).unwrap();
    /// assert_eq!(
    ///     x.checked_cbrt_strict_with(RoundingMode::HalfToEven),
    ///     Some(x.cbrt_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cbrt_strict_with(self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        Some(Self(crate::policy::cbrt::dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_cbrt_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(8i64).unwrap().checked_cbrt_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cbrt_strict(self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_cbrt_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `hypot_strict_with`: `√(self² + other²)`, `None` instead
    /// of a panic.
    ///
    /// No domain wall; `None` means the result does not fit the storage
    /// range (possible only when both operands are near the range
    /// limit). Otherwise bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let three = D38::<10>::try_from(3i64).unwrap();
    /// let four = D38::<10>::try_from(4i64).unwrap();
    /// assert_eq!(
    ///     three.checked_hypot_strict_with(four, RoundingMode::HalfToEven),
    ///     Some(three.hypot_strict_with(four, RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<10>::MAX.checked_hypot_strict_with(D38::<10>::MAX, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_hypot_strict_with(self, other: Self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        crate::policy::hypot::checked_dispatch::<N, SCALE>(self.0, other.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_hypot_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let a = D38::<10>::try_from(5i64).unwrap();
    /// assert!(a.checked_hypot_strict(a).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_hypot_strict(self, other: Self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_hypot_strict_with(other, DEFAULT_ROUNDING_MODE)
    }

    // ── Trigonometry (forward) ────────────────────────────────────

    /// Checked `sin_strict_with`. Always `Some`: `sin` is total and
    /// `|sin x| <= 1`, which fits every tier's range at every valid
    /// scale (each tier keeps >= ~10 of integer headroom at its
    /// maximum scale).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_sin_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.sin_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sin_strict_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::sin_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_sin_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_sin_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sin_strict(self) -> Option<Self> {
        self.checked_sin_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cos_strict_with`. Always `Some`: `cos` is total and
    /// `|cos x| <= 1` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_cos_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.cos_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cos_strict_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::cos_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_cos_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_cos_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cos_strict(self) -> Option<Self> {
        self.checked_cos_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `tan_strict_with`: `None` instead of a panic.
    ///
    /// The default form panics when the argument's cosine rounds to
    /// zero at the working precision (an odd multiple of π/2 to within
    /// the kernel's resolution) and when the result does not fit the
    /// storage range (near those asymptotes). Both conditions are
    /// detected inside the kernels at every tier; this checked form
    /// currently panics on them identically to the default form
    /// (kernel seam not yet reached). For every other input
    /// it returns bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_tan_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.tan_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tan_strict_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::tan_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_tan_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_tan_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tan_strict(self) -> Option<Self> {
        self.checked_tan_strict_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Trigonometry (inverse) ────────────────────────────────────

    /// Checked `asin_strict_with`: `None` instead of a domain panic.
    ///
    /// Returns `None` when `|self| > 1` (the default form's domain
    /// wall). The result `|asin x| <= π/2` always fits the storage
    /// range, so there is no out-of-range case. Otherwise bit-identical
    /// `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     half.checked_asin_strict_with(RoundingMode::HalfToEven),
    ///     Some(half.asin_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_asin_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asin_strict_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 > one || self.0 < -one {
            return None;
        }
        crate::policy::trig::checked_asin_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_asin_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_asin_strict().is_some());
    /// assert_eq!(D38::<12>::try_from(-2i64).unwrap().checked_asin_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asin_strict(self) -> Option<Self> {
        self.checked_asin_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `acos_strict_with`: `None` instead of a domain panic.
    ///
    /// Returns `None` when `|self| > 1`. The result `0 <= acos x <= π`
    /// always fits the storage range, so there is no out-of-range case.
    /// Otherwise bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     half.checked_acos_strict_with(RoundingMode::HalfToEven),
    ///     Some(half.acos_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_acos_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acos_strict_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 > one || self.0 < -one {
            return None;
        }
        crate::policy::trig::checked_acos_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_acos_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_acos_strict().is_some());
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_acos_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acos_strict(self) -> Option<Self> {
        self.checked_acos_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atan_strict_with`. Always `Some`: `atan` is total and
    /// `|atan x| < π/2` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(5i64).unwrap();
    /// assert_eq!(
    ///     x.checked_atan_strict_with(RoundingMode::HalfToEven),
    ///     Some(x.atan_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan_strict_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::atan_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_atan_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(3i64).unwrap().checked_atan_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan_strict(self) -> Option<Self> {
        self.checked_atan_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atan2_strict_with`. Always `Some`: `atan2` is total
    /// (including the `(0, 0)` origin, which yields `0`) and `|atan2(y,
    /// x)| <= π` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let y = D38::<12>::ONE;
    /// let x = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     y.checked_atan2_strict_with(x, RoundingMode::HalfToEven),
    ///     Some(y.atan2_strict_with(x, RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan2_strict_with(self, other: Self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_atan2_dispatch::<N, SCALE>(self.0, other.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_atan2_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_atan2_strict(D38::<12>::ONE).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan2_strict(self, other: Self) -> Option<Self> {
        self.checked_atan2_strict_with(other, DEFAULT_ROUNDING_MODE)
    }

    // ── Hyperbolics ───────────────────────────────────────────────

    /// Checked `sinh_strict_with`: `None` instead of a panic.
    ///
    /// `sinh` has no domain wall; `None` means the result does not fit
    /// the storage range (it grows like `e^|x|/2`). Otherwise
    /// bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18 (a result that fits the
    /// D38 work width but not D18 storage is `None`); detection deeper
    /// in the kernels (D38 and the wide tiers) still panics (kernel seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D18, D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_sinh_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.sinh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// // sinh(40) ~ 1.2e17 exceeds D18<6>'s range but fits the D38 work width.
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_sinh_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sinh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_sinh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_sinh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_sinh_strict().is_some());
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_sinh_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sinh_strict(self) -> Option<Self> {
        self.checked_sinh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cosh_strict_with`: `None` instead of a panic.
    ///
    /// `cosh` has no domain wall; `None` means the result does not fit
    /// the storage range. Otherwise bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18 (a result that fits the
    /// D38 work width but not D18 storage is `None`); detection deeper
    /// in the kernels (D38 and the wide tiers) still panics (kernel seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D18, D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_cosh_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.cosh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// // cosh(40) ~ 1.2e17 exceeds D18<6>'s range but fits the D38 work width.
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_cosh_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cosh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_cosh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_cosh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_cosh_strict().is_some());
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_cosh_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cosh_strict(self) -> Option<Self> {
        self.checked_cosh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `tanh_strict_with`. Always `Some`: `tanh` is total and
    /// `|tanh x| <= 1` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_tanh_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.tanh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tanh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_tanh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_tanh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_tanh_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tanh_strict(self) -> Option<Self> {
        self.checked_tanh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `asinh_strict_with`. Always `Some`: `asinh` is total and
    /// `|asinh x| <= max(|x|, 1)` always fits the storage range when
    /// `x` does.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(3i64).unwrap();
    /// assert_eq!(
    ///     x.checked_asinh_strict_with(RoundingMode::HalfToEven),
    ///     Some(x.asinh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asinh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_asinh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_asinh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(2i64).unwrap().checked_asinh_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asinh_strict(self) -> Option<Self> {
        self.checked_asinh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `acosh_strict_with`: `None` instead of a domain panic.
    ///
    /// Returns `None` when `self < 1` (the default form's domain wall).
    /// The result `acosh x < ln(2x) <= x` always fits the storage range
    /// when `x` does. Otherwise bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let two = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     two.checked_acosh_strict_with(RoundingMode::HalfToEven),
    ///     Some(two.acosh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ZERO.checked_acosh_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acosh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 < Self::unit_bits() {
            return None;
        }
        crate::policy::trig::checked_acosh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_acosh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(3i64).unwrap().checked_acosh_strict().is_some());
    /// assert_eq!(D38::<12>::ZERO.checked_acosh_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acosh_strict(self) -> Option<Self> {
        self.checked_acosh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atanh_strict_with`: `None` instead of a panic.
    ///
    /// Returns `None` when `|self| >= 1` (the default form's domain
    /// wall — `atanh` diverges at ±1). An out-of-range result (the
    /// logarithmic blow-up just inside ±1 at a near-maximum scale) is
    /// `None` on D18 when it fits the D38 work width; detection deeper
    /// in the kernels still panics, identically to the default form
    /// (kernel seam not yet reached). Otherwise bit-identical
    /// `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     half.checked_atanh_strict_with(RoundingMode::HalfToEven),
    ///     Some(half.atanh_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ONE.checked_atanh_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atanh_strict_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 >= one || self.0 <= -one {
            return None;
        }
        crate::policy::trig::checked_atanh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_atanh_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert!(half.checked_atanh_strict().is_some());
    /// assert_eq!(D38::<12>::ONE.checked_atanh_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atanh_strict(self) -> Option<Self> {
        self.checked_atanh_strict_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Angle conversion ──────────────────────────────────────────

    /// Checked `to_degrees_strict_with`: `None` instead of a panic.
    ///
    /// No domain wall; `None` means `self · (180/π)` does not fit the
    /// storage range (the result is ~57.3× the input). Otherwise
    /// bit-identical `Some`.
    ///
    /// Out-of-range detection: exact on D18 (a result that fits the
    /// D38 work width but not D18 storage is `None`); detection deeper
    /// in the kernels (D38 and the wide tiers) still panics (kernel seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D18, D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_to_degrees_strict_with(RoundingMode::HalfToEven),
    ///     Some(one.to_degrees_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// // MAX·(180/π) leaves D18's range but fits the D38 work width.
    /// assert_eq!(D18::<6>::MAX.checked_to_degrees_strict_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_degrees_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_to_degrees_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_to_degrees_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_to_degrees_strict().is_some());
    /// assert_eq!(D18::<6>::MAX.checked_to_degrees_strict(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_degrees_strict(self) -> Option<Self> {
        self.checked_to_degrees_strict_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `to_radians_strict_with`. Always `Some`: the conversion
    /// multiplies by `π/180 < 1`, so the result is strictly smaller in
    /// magnitude than the (representable) input.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(180i64).unwrap();
    /// assert_eq!(
    ///     x.checked_to_radians_strict_with(RoundingMode::HalfToEven),
    ///     Some(x.to_radians_strict_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_radians_strict_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_to_radians_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_to_radians_strict_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(90i64).unwrap().checked_to_radians_strict().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_radians_strict(self) -> Option<Self> {
        self.checked_to_radians_strict_with(DEFAULT_ROUNDING_MODE)
    }
}
