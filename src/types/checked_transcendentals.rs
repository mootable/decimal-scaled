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
//! forms have no panic contract to opt out of — they saturate).
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

    /// Checked [`ln_with`](crate::types::widths::D38::ln_with):
    /// natural logarithm, `None` instead of a panic.
    ///
    /// Returns `None` when `self <= 0` (the domain wall) or when the
    /// correctly-rounded result does not fit the storage range (possible
    /// only near a tier's maximum scale). Otherwise
    /// `Some(self.ln_with(mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (kernel seam not yet reached).
    /// Domain errors return `None` at every tier.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let two = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     two.checked_ln_with(RoundingMode::HalfToEven),
    ///     Some(two.ln_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ZERO.checked_ln_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_ln_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_ln_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(10i64).unwrap().checked_ln().is_some());
    /// assert_eq!(D38::<12>::try_from(-1i64).unwrap().checked_ln(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_ln(self) -> Option<Self> {
        self.checked_ln_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log_with`: logarithm in an arbitrary `base`,
    /// `None` instead of a panic.
    ///
    /// Returns `None` when `self <= 0`, `base <= 0`, or `base == 1`
    /// (the domain walls the default form panics on), or when the
    /// result does not fit the storage range. Otherwise
    /// `Some(self.log_with(base, mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (wide kernel-shell seam not yet
    /// reached). Domain errors return `None` at every tier.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let eight = D38::<10>::try_from(8i64).unwrap();
    /// let two = D38::<10>::try_from(2i64).unwrap();
    /// assert!(eight.checked_log_with(two, RoundingMode::HalfToEven).is_some());
    /// assert_eq!(eight.checked_log_with(D38::<10>::ONE, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log_with(self, base: Self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO
            || base.0 <= Int::<N>::ZERO
            || base.0 == Self::unit_bits()
        {
            return None;
        }
        crate::policy::log::checked_dispatch::<N, SCALE>(self.0, base.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let x = D38::<10>::try_from(100i64).unwrap();
    /// assert!(x.checked_log(D38::<10>::try_from(10i64).unwrap()).is_some());
    /// assert_eq!(x.checked_log(D38::<10>::ZERO), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log(self, base: Self) -> Option<Self> {
        self.checked_log_with(base, DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log2_with`: base-2 logarithm, `None` instead of
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
    ///     eight.checked_log2_with(RoundingMode::HalfToEven),
    ///     Some(eight.log2_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<10>::ZERO.checked_log2_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log2_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_log2_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log2_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(4i64).unwrap().checked_log2().is_some());
    /// assert_eq!(D38::<10>::try_from(-4i64).unwrap().checked_log2(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log2(self) -> Option<Self> {
        self.checked_log2_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log10_with`: base-10 logarithm, `None` instead
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
    /// assert!(hundred.checked_log10_with(RoundingMode::HalfToEven).is_some());
    /// assert_eq!(D38::<10>::ZERO.checked_log10_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log10_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= Int::<N>::ZERO {
            return None;
        }
        crate::policy::ln::checked_log10_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_log10_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(1000i64).unwrap().checked_log10().is_some());
    /// assert_eq!(D38::<10>::ZERO.checked_log10(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log10(self) -> Option<Self> {
        self.checked_log10_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `log1p_with`: `ln(1 + self)`, `None` instead of a
    /// domain panic.
    ///
    /// Returns `None` when `self <= -1` — the domain wall the default
    /// form panics on (`log1p: argument must be greater than -1`).
    /// Otherwise `Some(self.log1p_with(mode))`, bit-identical.
    ///
    /// There is no out-of-range case: `ln(1 + t)` is bounded by the
    /// storage range wherever `1 + t` is representable, so every
    /// in-domain argument returns `Some` at every tier. Unlike the
    /// logarithms above, this needs no wide-tier caveat.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_log1p_with(RoundingMode::HalfToEven),
    ///     Some(one.log1p_with(RoundingMode::HalfToEven)),
    /// );
    /// // `t = -1` is the wall — `ln(0)` is undefined.
    /// assert_eq!(
    ///     D38::<12>::try_from(-1i64).unwrap().checked_log1p_with(RoundingMode::HalfToEven),
    ///     None,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log1p_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 <= -Self::unit_bits() {
            return None;
        }
        Some(Self(crate::policy::log1p::dispatch::<N, SCALE>(
            self.0, mode,
        )))
    }

    /// Default-mode sibling of [`Self::checked_log1p_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_log1p().is_some());
    /// assert_eq!(D38::<12>::try_from(-1i64).unwrap().checked_log1p(), None);
    /// assert_eq!(D38::<12>::try_from(-2i64).unwrap().checked_log1p(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_log1p(self) -> Option<Self> {
        self.checked_log1p_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Exponentials ──────────────────────────────────────────────

    /// Checked `exp_with`: `e^self`, `None` instead of a panic.
    ///
    /// `exp` has no domain wall; `None` means the correctly-rounded
    /// result does not fit the storage range — the same condition on
    /// which the default form panics. Otherwise
    /// `Some(self.exp_with(mode))`, bit-identical.
    ///
    /// Out-of-range detection: exact on D18/D38; on the wide tiers an
    /// out-of-range result still panics (kernel seam not yet reached).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_exp_with(RoundingMode::HalfToEven),
    ///     Some(one.exp_with(RoundingMode::HalfToEven)),
    /// );
    /// // e^120 has 53 integer digits — far outside D38's 38.
    /// assert_eq!(D38::<12>::try_from(120i64).unwrap().checked_exp_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::exp::checked_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_exp_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_exp().is_some());
    /// assert_eq!(D38::<12>::try_from(120i64).unwrap().checked_exp(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp(self) -> Option<Self> {
        self.checked_exp_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `exp2_with`: `2^self`, `None` instead of a panic.
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
    ///     ten.checked_exp2_with(RoundingMode::HalfToEven),
    ///     Some(ten.exp2_with(RoundingMode::HalfToEven)),
    /// );
    /// // 2^95 has 29 integer digits — outside D38<12>'s 26.
    /// assert_eq!(D38::<12>::try_from(95i64).unwrap().checked_exp2_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp2_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::exp::checked_exp2_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_exp2_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(10i64).unwrap().checked_exp2().is_some());
    /// assert_eq!(D38::<12>::try_from(95i64).unwrap().checked_exp2(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_exp2(self) -> Option<Self> {
        self.checked_exp2_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `expm1_with`: `e^self − 1`, `None` instead of a
    /// panic.
    ///
    /// `expm1` is total over its argument, so there is no domain wall
    /// and `None` is reserved for a result that does not fit the storage
    /// range — the same condition on which the default form panics.
    /// Otherwise `Some(self.expm1_with(mode))`, bit-identical.
    ///
    /// Out-of-range detection: the kernel seam that reports overflow as
    /// an `Option` has not been threaded through `expm1`, so at every
    /// tier an out-of-range result still PANICS rather than returning
    /// `None`, and this form is `Some` whenever the default form
    /// returns at all. That is the same gap the wide-tier note on
    /// [`Self::checked_exp_with`] records — `exp` has the seam on
    /// D18/D38, `expm1` does not have it yet — so this pair completes
    /// the documented surface rather than adding a stronger guarantee.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_expm1_with(RoundingMode::HalfToEven),
    ///     Some(one.expm1_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_expm1_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::expm1::dispatch::<N, SCALE>(
            self.0, mode,
        )))
    }

    /// Default-mode sibling of [`Self::checked_expm1_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_expm1().is_some());
    /// assert!(D38::<12>::try_from(-1i64).unwrap().checked_expm1().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_expm1(self) -> Option<Self> {
        self.checked_expm1_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Power ─────────────────────────────────────────────────────

    /// Checked `powf_with`: `self^exp`, `None` instead of a
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
    ///     three.checked_powf_with(two, RoundingMode::HalfToEven),
    ///     Some(three.powf_with(two, RoundingMode::HalfToEven)),
    /// );
    /// // Non-positive base saturates to zero, as the default form does.
    /// let half = D38::<10>::ONE / two;
    /// assert_eq!(
    ///     (-three).checked_powf_with(half, RoundingMode::HalfToEven),
    ///     Some(D38::<10>::ZERO),
    /// );
    /// // 10^30 has 31 integer digits — out of D38<10>'s 28.
    /// let ten = D38::<10>::try_from(10i64).unwrap();
    /// let thirty = D38::<10>::try_from(30i64).unwrap();
    /// assert_eq!(ten.checked_powf_with(thirty, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_powf_with(self, exp: Self, mode: RoundingMode) -> Option<Self> {
        crate::policy::pow::checked_dispatch::<N, SCALE>(self.0, exp.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_powf_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let x = D38::<10>::try_from(2i64).unwrap();
    /// assert!(x.checked_powf(D38::<10>::try_from(8i64).unwrap()).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_powf(self, exp: Self) -> Option<Self> {
        self.checked_powf_with(exp, DEFAULT_ROUNDING_MODE)
    }

    // ── Roots ─────────────────────────────────────────────────────

    /// Checked `sqrt_with`. Always `Some`: the strict square
    /// root is total — negative inputs saturate to zero (the policy's
    /// documented behaviour, not a panic), and the result `√v ≤
    /// max(v, 1)` always fits the storage range. The checked form
    /// exists for surface uniformity.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let nine = D38::<10>::try_from(9i64).unwrap();
    /// assert_eq!(
    ///     nine.checked_sqrt_with(RoundingMode::HalfToEven),
    ///     Some(nine.sqrt_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sqrt_with(self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        Some(Self(crate::policy::sqrt::dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_sqrt_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(2i64).unwrap().checked_sqrt().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sqrt(self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_sqrt_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cbrt_with`. Always `Some`: the cube root is
    /// total over the signed domain and `∛v` never exceeds `max(|v|,
    /// 1)`, so it always fits the storage range.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<10>::try_from(-27i64).unwrap();
    /// assert_eq!(
    ///     x.checked_cbrt_with(RoundingMode::HalfToEven),
    ///     Some(x.cbrt_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cbrt_with(self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        Some(Self(crate::policy::cbrt::dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_cbrt_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<10>::try_from(8i64).unwrap().checked_cbrt().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cbrt(self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_cbrt_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `hypot_with`: `√(self² + other²)`, `None` instead
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
    ///     three.checked_hypot_with(four, RoundingMode::HalfToEven),
    ///     Some(three.hypot_with(four, RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<10>::MAX.checked_hypot_with(D38::<10>::MAX, RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_hypot_with(self, other: Self, mode: RoundingMode) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        crate::policy::hypot::checked_dispatch::<N, SCALE>(self.0, other.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_hypot_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let a = D38::<10>::try_from(5i64).unwrap();
    /// assert!(a.checked_hypot(a).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_hypot(self, other: Self) -> Option<Self>
    where
        Limbs<N>: ComputeLimbs,
    {
        self.checked_hypot_with(other, DEFAULT_ROUNDING_MODE)
    }

    // ── Trigonometry (forward) ────────────────────────────────────

    /// Checked `sin_with`. Always `Some`: `sin` is total and
    /// `|sin x| <= 1`, which fits every tier's range at every valid
    /// scale (each tier keeps >= ~10 of integer headroom at its
    /// maximum scale).
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_sin_with(RoundingMode::HalfToEven),
    ///     Some(one.sin_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sin_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::sin_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_sin_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_sin().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sin(self) -> Option<Self> {
        self.checked_sin_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cos_with`. Always `Some`: `cos` is total and
    /// `|cos x| <= 1` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_cos_with(RoundingMode::HalfToEven),
    ///     Some(one.cos_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cos_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::cos_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_cos_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_cos().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cos(self) -> Option<Self> {
        self.checked_cos_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `tan_with`: `None` instead of a panic.
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
    ///     one.checked_tan_with(RoundingMode::HalfToEven),
    ///     Some(one.tan_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tan_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::tan_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_tan_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_tan().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tan(self) -> Option<Self> {
        self.checked_tan_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Trigonometry (inverse) ────────────────────────────────────

    /// Checked `asin_with`: `None` instead of a domain panic.
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
    ///     half.checked_asin_with(RoundingMode::HalfToEven),
    ///     Some(half.asin_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_asin_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asin_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 > one || self.0 < -one {
            return None;
        }
        crate::policy::trig::checked_asin_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_asin_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_asin().is_some());
    /// assert_eq!(D38::<12>::try_from(-2i64).unwrap().checked_asin(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asin(self) -> Option<Self> {
        self.checked_asin_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `acos_with`: `None` instead of a domain panic.
    ///
    /// Returns `None` when `|self| > 1`. The result `0 <= acos x <= π`
    /// always fits the storage range, so there is no out-of-range case.
    /// Otherwise bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     half.checked_acos_with(RoundingMode::HalfToEven),
    ///     Some(half.acos_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_acos_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acos_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 > one || self.0 < -one {
            return None;
        }
        crate::policy::trig::checked_acos_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_acos_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_acos().is_some());
    /// assert_eq!(D38::<12>::try_from(2i64).unwrap().checked_acos(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acos(self) -> Option<Self> {
        self.checked_acos_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atan_with`. Always `Some`: `atan` is total and
    /// `|atan x| < π/2` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(5i64).unwrap();
    /// assert_eq!(
    ///     x.checked_atan_with(RoundingMode::HalfToEven),
    ///     Some(x.atan_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan_with(self, mode: RoundingMode) -> Option<Self> {
        Some(Self(crate::policy::trig::atan_dispatch::<N, SCALE>(self.0, mode)))
    }

    /// Default-mode sibling of [`Self::checked_atan_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(3i64).unwrap().checked_atan().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan(self) -> Option<Self> {
        self.checked_atan_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atan2_with`. Always `Some`: `atan2` is total
    /// (including the `(0, 0)` origin, which yields `0`) and `|atan2(y,
    /// x)| <= π` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let y = D38::<12>::ONE;
    /// let x = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     y.checked_atan2_with(x, RoundingMode::HalfToEven),
    ///     Some(y.atan2_with(x, RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan2_with(self, other: Self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_atan2_dispatch::<N, SCALE>(self.0, other.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_atan2_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_atan2(D38::<12>::ONE).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atan2(self, other: Self) -> Option<Self> {
        self.checked_atan2_with(other, DEFAULT_ROUNDING_MODE)
    }

    // ── Hyperbolics ───────────────────────────────────────────────

    /// Checked `sinh_with`: `None` instead of a panic.
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
    ///     one.checked_sinh_with(RoundingMode::HalfToEven),
    ///     Some(one.sinh_with(RoundingMode::HalfToEven)),
    /// );
    /// // sinh(40) ~ 1.2e17 exceeds D18<6>'s range but fits the D38 work width.
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_sinh_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sinh_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_sinh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_sinh_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_sinh().is_some());
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_sinh(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_sinh(self) -> Option<Self> {
        self.checked_sinh_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `cosh_with`: `None` instead of a panic.
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
    ///     one.checked_cosh_with(RoundingMode::HalfToEven),
    ///     Some(one.cosh_with(RoundingMode::HalfToEven)),
    /// );
    /// // cosh(40) ~ 1.2e17 exceeds D18<6>'s range but fits the D38 work width.
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_cosh_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cosh_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_cosh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_cosh_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_cosh().is_some());
    /// assert_eq!(D18::<6>::try_from(40).unwrap().checked_cosh(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_cosh(self) -> Option<Self> {
        self.checked_cosh_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `tanh_with`. Always `Some`: `tanh` is total and
    /// `|tanh x| <= 1` fits every tier's range at every valid scale.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let one = D38::<12>::ONE;
    /// assert_eq!(
    ///     one.checked_tanh_with(RoundingMode::HalfToEven),
    ///     Some(one.tanh_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tanh_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_tanh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_tanh_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::ONE.checked_tanh().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_tanh(self) -> Option<Self> {
        self.checked_tanh_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `asinh_with`. Always `Some`: `asinh` is total and
    /// `|asinh x| <= max(|x|, 1)` always fits the storage range when
    /// `x` does.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(3i64).unwrap();
    /// assert_eq!(
    ///     x.checked_asinh_with(RoundingMode::HalfToEven),
    ///     Some(x.asinh_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asinh_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_asinh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_asinh_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(2i64).unwrap().checked_asinh().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_asinh(self) -> Option<Self> {
        self.checked_asinh_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `acosh_with`: `None` instead of a domain panic.
    ///
    /// Returns `None` when `self < 1` (the default form's domain wall).
    /// The result `acosh x < ln(2x) <= x` always fits the storage range
    /// when `x` does. Otherwise bit-identical `Some`.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let two = D38::<12>::try_from(2i64).unwrap();
    /// assert_eq!(
    ///     two.checked_acosh_with(RoundingMode::HalfToEven),
    ///     Some(two.acosh_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ZERO.checked_acosh_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acosh_with(self, mode: RoundingMode) -> Option<Self> {
        if self.0 < Self::unit_bits() {
            return None;
        }
        crate::policy::trig::checked_acosh_dispatch::<N, SCALE>(self.0, mode).map(Self)
    }

    /// Default-mode sibling of [`Self::checked_acosh_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(3i64).unwrap().checked_acosh().is_some());
    /// assert_eq!(D38::<12>::ZERO.checked_acosh(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_acosh(self) -> Option<Self> {
        self.checked_acosh_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `atanh_with`: `None` instead of a panic.
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
    ///     half.checked_atanh_with(RoundingMode::HalfToEven),
    ///     Some(half.atanh_with(RoundingMode::HalfToEven)),
    /// );
    /// assert_eq!(D38::<12>::ONE.checked_atanh_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atanh_with(self, mode: RoundingMode) -> Option<Self> {
        let one = Self::unit_bits();
        if self.0 >= one || self.0 <= -one {
            return None;
        }
        crate::policy::trig::checked_atanh_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_atanh_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// let half = D38::<12>::ONE / D38::<12>::try_from(2i64).unwrap();
    /// assert!(half.checked_atanh().is_some());
    /// assert_eq!(D38::<12>::ONE.checked_atanh(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_atanh(self) -> Option<Self> {
        self.checked_atanh_with(DEFAULT_ROUNDING_MODE)
    }

    // ── Angle conversion ──────────────────────────────────────────

    /// Checked `to_degrees_with`: `None` instead of a panic.
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
    ///     one.checked_to_degrees_with(RoundingMode::HalfToEven),
    ///     Some(one.to_degrees_with(RoundingMode::HalfToEven)),
    /// );
    /// // MAX·(180/π) leaves D18's range but fits the D38 work width.
    /// assert_eq!(D18::<6>::MAX.checked_to_degrees_with(RoundingMode::HalfToEven), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_degrees_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_to_degrees_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_to_degrees_with`].
    ///
    /// ```
    /// use decimal_scaled::{D18, D38};
    /// assert!(D38::<12>::ONE.checked_to_degrees().is_some());
    /// assert_eq!(D18::<6>::MAX.checked_to_degrees(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_degrees(self) -> Option<Self> {
        self.checked_to_degrees_with(DEFAULT_ROUNDING_MODE)
    }

    /// Checked `to_radians_with`. Always `Some`: the conversion
    /// multiplies by `π/180 < 1`, so the result is strictly smaller in
    /// magnitude than the (representable) input.
    ///
    /// ```
    /// use decimal_scaled::{D38, RoundingMode};
    /// let x = D38::<12>::try_from(180i64).unwrap();
    /// assert_eq!(
    ///     x.checked_to_radians_with(RoundingMode::HalfToEven),
    ///     Some(x.to_radians_with(RoundingMode::HalfToEven)),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_radians_with(self, mode: RoundingMode) -> Option<Self> {
        crate::policy::trig::checked_to_radians_dispatch::<N, SCALE>(self.0, mode)
            .map(Self)
    }

    /// Default-mode sibling of [`Self::checked_to_radians_with`].
    ///
    /// ```
    /// use decimal_scaled::D38;
    /// assert!(D38::<12>::try_from(90i64).unwrap().checked_to_radians().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn checked_to_radians(self) -> Option<Self> {
        self.checked_to_radians_with(DEFAULT_ROUNDING_MODE)
    }
}
