//! `cbrt_newton` — Newton–Raphson integer cube root of
//! `mag · 10^(2·SCALE)`, with a single round step. Sign-preserving.
//!
//! For a `D<Int<N>, SCALE>` value with raw storage `r`, the logical
//! value is `r / 10^SCALE`, so the cube-root raw storage is
//! `round(cbrt(r) · 10^SCALE) = round(cbrt(r · 10^(3·SCALE)) / 10^SCALE)`.
//! Working with `n = |r| · 10^(2·SCALE)` keeps the radicand exact in a
//! wider work integer `W`, takes the exact integer cube root with the
//! seeded Newton loop ([`crate::policy::float_seed::icbrt`]: an `f64`
//! `cbrt` seed when `std` is available, the classical 1-bit seed
//! otherwise), and a single half-step lands the result on the type's
//! last representable place. The result is within 0.5 ULP under any of
//! the six [`crate::support::rounding::RoundingMode`] values.
//!
//! # Generic over the storage and work widths
//!
//! Mirrors [`crate::algos::sqrt::sqrt_newton`]: parameterised over
//! `(S, W)` via the [`crate::int::types::traits::BigInt`] trait. `S` is
//! the storage type backing the decimal and `W` is the work width wide
//! enough to hold `|raw| · 10^(2·scale)` without overflow — one width
//! step *beyond* the obvious next-up, because of the `10^(2·SCALE)`
//! factor (see the policy's per-tier `W` selection). There are **no**
//! per-tier shims: the policy layer binds the concrete `W` for each
//! storage width when it dispatches here.
//!
//! The Newton seed (the only `std`/`no_std` divergence) lives in
//! [`crate::policy::float_seed::icbrt`], so this body is cfg-free.
//!
//! Returns `S::ZERO` for `raw == 0`; the sign of a non-zero input is
//! preserved (`cbrt(-x) = -cbrt(x)`), with the rounding mode resolving
//! direction relative to the signed result.

use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Newton integer cube-root kernel for the wide-integer family.
///
/// `S` is the storage type backing `D<Int<N>, SCALE>` and `W` is the
/// work width wide enough to form `|raw| · 10^(2·scale)` without
/// overflow. See the module docs for the full pipeline.
#[inline]
#[must_use]
pub(crate) fn cbrt_newton<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: BigInt,
    W: BigInt,
{
    if raw == S::ZERO {
        return S::ZERO;
    }
    let zero = W::ZERO;
    let one = W::ONE;

    let widened: W = raw.resize_to::<W>();
    let negative = widened < zero;
    let mag = if negative { -widened } else { widened };
    let n: W = mag * W::TEN.pow(2 * scale);

    // Floor cube root, seeded via `f64::cbrt` under `std` and the
    // classical seed under `no_std` — both return the exact `⌊∛n⌋`.
    let q: W = crate::policy::float_seed::icbrt::<W>(n);

    // ── single half-step round (all six modes) ───────────────────────
    let eight_n = n << 3u32;
    let t = q + q + one;
    let cube = t * t * t;
    let halfway_geq = eight_n >= cube;
    let halfway_gt = eight_n > cube;
    let tie = halfway_geq && !halfway_gt;
    let two_q = q + q;
    let eight_q_cubed = if q == zero { zero } else { two_q * two_q * two_q };
    let residual_nonzero = eight_n > eight_q_cubed;
    let q_is_odd = (q % (one + one)) != zero;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && q_is_odd),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => negative && residual_nonzero,
        RoundingMode::Ceiling => !negative && residual_nonzero,
    };
    let q = if bump { q + one } else { q };
    let signed = if negative { -q } else { q };
    signed.resize_to::<S>()
}
