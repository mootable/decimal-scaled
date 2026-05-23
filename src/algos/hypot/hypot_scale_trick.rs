//! `hypot_scale_trick` — the overflow-avoiding hypotenuse algorithm.
//!
//! Computes `sqrt(a² + b²)` as
//!
//! ```text
//! hypot(a, b) = max(|a|, |b|) · sqrt(1 + (min(|a|, |b|) / max(|a|, |b|))²)
//! ```
//!
//! The `min/max` ratio lies in `[0, 1]`, so `1 + ratio²` stays in
//! `[1, 2]` — the inner square root never overflows. The outer multiply
//! by `large` overflows only when the true hypotenuse genuinely exceeds
//! the type's range.
//!
//! `hypot(0, 0) = 0` (bit-exact); `hypot(0, x) = |x|` (the inner sqrt of
//! `1 + 0` is exactly 1).
//!
//! # Layering
//!
//! This is a decimal-level algorithm (§1a): it composes the tier's own
//! decimal operator surface (`abs`, `>=`, `==`, `/`, `*`, `+`,
//! `Self::ONE`, `Self::ZERO`) and the `sqrt` surface
//! ([`DecimalTranscendental::sqrt_strict_with`]). It never calls
//! `hypot_strict_with` on its own type — the inherent method delegates
//! *down* to [`crate::policy::hypot`], which calls this fn; the call
//! graph points strictly down (method → policy → algorithm → sqrt
//! surface + operators).

use crate::support::rounding::RoundingMode;
use crate::types::traits::{DecimalArithmetic, DecimalTranscendental};

/// `sqrt(a² + b²)` via the scale-ratio trick. Generic over any decimal
/// tier: the bound supplies the operator surface (`DecimalArithmetic`)
/// and the inner `sqrt` surface (`DecimalTranscendental`). `mode`
/// applies to the inner square-root step.
#[inline]
pub(crate) fn hypot_scale_trick<T>(a: T, b: T, mode: RoundingMode) -> T
where
    T: DecimalArithmetic + DecimalTranscendental,
{
    let a = a.abs();
    let b = b.abs();
    let (large, small) = if a >= b { (a, b) } else { (b, a) };
    if large == T::ZERO {
        T::ZERO
    } else {
        let ratio = small / large;
        let one_plus_sq = T::ONE + ratio * ratio;
        large * one_plus_sq.sqrt_strict_with(mode)
    }
}
