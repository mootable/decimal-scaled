//! `hypot_isqrt` — `sqrt(a² + b²)` via the integer-layer `isqrt`.
//!
//! For two `D<Int<N>, SCALE>` values with raw storages `a` and `b`, the
//! logical hypotenuse is `sqrt((a/10^SCALE)² + (b/10^SCALE)²)`, whose raw
//! storage is
//!
//! ```text
//! round( sqrt(a² + b²) )
//! ```
//!
//! — both operands carry the same `10^SCALE` factor, so it divides out of
//! the root and no rescale is needed (contrast [`crate::algos::sqrt`],
//! which forms `raw · 10^SCALE`). The radicand `a² + b²` is formed exactly
//! in a wider work integer `W` (so the squares and their sum cannot
//! overflow), the exact integer square root is taken via the integer
//! wide-kernel surface ([`crate::int::types::traits::BigInt::isqrt`] — the
//! same int `isqrt` dispatch [`crate::algos::sqrt::sqrt_newton`] uses), and
//! a single round-to-nearest step lands the result on the type's last
//! representable place. This routes the root **down** through the integer
//! layer instead of calling the decimal `sqrt` surface on the tier's own
//! value.
//!
//! # Generic over the storage and work widths
//!
//! The kernel is parameterised over `(S, W)` via the
//! [`crate::int::types::traits::BigInt`] trait, exactly as
//! [`crate::algos::sqrt::sqrt_newton`] is. `S` is the storage type backing
//! the decimal; `W` is the next-up width used to form `a² + b²` without
//! overflow. There are **no** per-tier shims: the policy layer binds the
//! concrete `W` for each storage width when it dispatches here.
//!
//! # Semantics preserved
//!
//! - `hypot(0, 0) = 0` (the radicand is zero, `isqrt(0) = 0`);
//! - `hypot(0, x) = |x|` (`isqrt(x²) = |x|`, no rounding bump);
//! - overflow is reported (the kernel returns [`None`]) only when the true
//!   result `round(sqrt(a² + b²))` does not fit `S` — the in-`S` value is
//!   widened back to `W` and compared, so a `None` means the storage type
//!   genuinely cannot hold the hypotenuse. The caller turns `None` into the
//!   uniform out-of-range panic, matching the prior scale-trick's
//!   panic-on-overflow.

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `sqrt(a² + b²)` for the wide-integer family, taking the root through
/// the integer-layer `isqrt`.
///
/// `S` is the storage type backing `D<Int<N>, SCALE>` and `W` is the
/// next-up width used to form `a² + b²` without overflow. The rounding
/// step is identical to [`crate::algos::sqrt::sqrt_newton`]. Returns
/// [`None`] when the rounded root does not fit `S` (true overflow); the
/// caller maps that to the out-of-range panic.
#[inline]
#[must_use]
pub(crate) fn hypot_isqrt<S, W>(a: S, b: S, mode: RoundingMode) -> Option<S>
where
    S: BigInt,
    W: BigInt,
{
    let aw: W = a.resize_to::<W>();
    let bw: W = b.resize_to::<W>();
    let n: W = aw * aw + bw * bw;
    if n <= W::ZERO {
        return Some(S::ZERO);
    }
    let q: W = n.isqrt();
    let diff: W = n - q * q;
    let halfway_round_up = diff > q;
    let diff_nonzero = diff != W::ZERO;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    let q = if bump { q + W::ONE } else { q };
    // Narrow to storage, then verify the value round-trips: a mismatch
    // means the rounded root genuinely exceeds `S`'s range (true overflow).
    let narrowed: S = q.resize_to::<S>();
    if narrowed.resize_to::<W>() == q {
        Some(narrowed)
    } else {
        None
    }
}
