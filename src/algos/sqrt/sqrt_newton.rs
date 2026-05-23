//! `sqrt_newton` — Newton–Raphson integer square root of
//! `raw · 10^SCALE`, with a single round step.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the logical
//! value is `r / 10^SCALE`, so the square-root raw storage is
//! `round(sqrt(r · 10^SCALE))`. The radicand is formed exactly in a
//! wider work integer `W` (so the multiply by `10^SCALE` cannot
//! overflow), the exact integer square root is taken via the integer
//! wide-kernel surface ([`crate::int::types::traits::BigInt::isqrt`] —
//! the int `isqrt` policy's Newton limb kernel, with an `f64` seed when
//! `std` is available), and a single round-to-nearest
//! step lands the result on the type's last representable place. The
//! result is within 0.5 ULP under any of the six
//! [`crate::support::rounding::RoundingMode`] values.
//!
//! # Generic over the storage and work widths
//!
//! The kernel is parameterised over `(S, W)` via the
//! [`crate::int::types::traits::BigInt`] trait, which exposes the
//! uniform surface (`ZERO` / `ONE` / `TEN`, `pow`, `isqrt`,
//! `resize_to`, the standard arithmetic ops) every wide signed integer
//! in the family ships. `S` is the storage type backing the decimal and
//! `W` is the next-up width used to form the radicand without overflow.
//! There are **no** per-tier shims: the policy layer binds the concrete
//! `W` for each storage width when it dispatches here.
//!
//! Returns `S::ZERO` for `raw <= 0` (the saturate-not-panic policy
//! matching the typed `sqrt_strict` surface).

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Newton integer square-root kernel for the wide-integer family.
///
/// `S` is the storage type backing `D<Int<N>, SCALE>` and `W` is the
/// next-up width used to form `raw · 10^scale` without overflow. See the
/// module docs for the full pipeline.
#[inline]
#[must_use]
pub(crate) fn sqrt_newton<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: BigInt,
    W: BigInt,
{
    if raw <= S::ZERO {
        return S::ZERO;
    }
    let n: W = raw.resize_to::<W>() * W::TEN.pow(scale);
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
    q.resize_to::<S>()
}
