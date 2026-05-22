//! Float-bridge Newton seeds for integer roots — the one place the
//! `std` vs `no_std` divergence for the seeded integer `isqrt` / `icbrt`
//! kernels lives.
//!
//! The wide-integer Newton root loops in [`crate::algos::sqrt`] and
//! [`crate::algos::cbrt`] converge from any positive seed, but the
//! *number* of iterations — each a multi-limb Knuth divide, the
//! dominant cost — depends entirely on seed quality:
//!
//! - **`std`** — `f64::sqrt` / `f64::cbrt` are in scope. Seeding from
//!   `f64::sqrt(n.as_f64())` lands within ~2⁻⁵² of the true root, so
//!   Newton needs ~2 iterations instead of ~7. The float intrinsics are
//!   `std`-only (the `f32`/`f64` inherent transcendental methods are not
//!   in `core`).
//! - **`no_std`** — no float intrinsics, so we fall back to the
//!   classical 1-bit seed (`Int::isqrt`, or `1 << ⌈bits/3⌉` for cbrt).
//!   Bit-identical result, just more iterations.
//!
//! Both paths return the **same** `⌊√n⌋` / `⌊∛n⌋` — only the seed
//! source, and hence the iteration count, differs. Hoisting the choice
//! here lets the root kernels stay cfg-free: they call
//! [`isqrt`] / [`icbrt`] and never name a float method directly.

use crate::wide_int::BigInt;

/// Whether the `std` float intrinsics (`f64::sqrt` / `f64::cbrt`, used
/// as Newton seeds) are available in this build.
///
/// The wide-tier kernels that seed via the top-64-bit `f64` bridge
/// (e.g. [`crate::algos::cbrt::generic_wide`]) branch on this const so
/// the seed code const-folds away on `no_std`.
pub(crate) const FLOAT_SEED_AVAILABLE: bool = cfg!(feature = "std");

/// `f64::cbrt`, available only under `std`.
///
/// On `no_std` this returns a placeholder — every caller gates the
/// f64-seed branch behind [`FLOAT_SEED_AVAILABLE`], so the `no_std`
/// body const-folds out and this value is never observed.
#[inline]
#[must_use]
pub(crate) fn cbrt_f64(x: f64) -> f64 {
    #[cfg(feature = "std")]
    {
        x.cbrt()
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = x;
        1.0
    }
}

/// `⌊√n⌋` for `n > 0`, computed in the wide integer `W`.
///
/// Under `std` the Newton loop is seeded from `f64::sqrt(n.as_f64())`
/// (~53-bit seed, one unconditional AM-GM pre-step to guarantee the
/// loop precondition `seed ≥ ⌈√n⌉`); under `no_std` it falls back to
/// the trait-level 1-bit-seed `isqrt`. The returned `⌊√n⌋` is identical
/// either way.
#[inline]
#[must_use]
pub(crate) fn isqrt<W: BigInt>(n: W) -> W {
    #[cfg(feature = "std")]
    {
        let seed_f64 = n.to_f64().sqrt();
        let seed = W::from_f64_val(seed_f64);
        let x0 = if seed <= W::ZERO { W::ONE } else { seed };
        // Unconditional first Newton step. AM-GM ⇒ result ≥ ⌈√n⌉.
        let mut x = (x0 + n / x0) >> 1u32;
        loop {
            let y = (x + n / x) >> 1u32;
            if y >= x {
                break x;
            }
            x = y;
        }
    }
    #[cfg(not(feature = "std"))]
    {
        n.isqrt()
    }
}

/// `⌊∛n⌋` for `n > 0`, computed in the wide integer `W`.
///
/// Under `std` the Newton cube-root loop is seeded from
/// `f64::cbrt(n.as_f64())` with one unconditional pre-step (AM-GM on
/// `(x, x, n/x²)` ⇒ `≥ ⌈∛n⌉`); under `no_std` it seeds the loop with
/// the classical `1` and lets the monotone-decrease loop settle. Both
/// return the same `⌊∛n⌋`.
///
/// Callers needing the *full generic-kernel* rounding (the
/// `generic_wide::cbrt` half-step logic) supply this as the loop core;
/// the bespoke `D57<20>` kernel uses it directly.
#[inline]
#[must_use]
pub(crate) fn icbrt<W: BigInt>(n: W) -> W {
    let three = W::ONE + W::ONE + W::ONE;
    #[cfg(feature = "std")]
    let x0 = {
        let seed_f64 = n.to_f64().cbrt();
        let seed = W::from_f64_val(seed_f64);
        if seed <= W::ZERO { W::ONE } else { seed }
    };
    #[cfg(not(feature = "std"))]
    let x0 = W::ONE;

    // Unconditional first Newton step. AM-GM ⇒ result ≥ ⌈∛n⌉.
    let mut x = (x0 + x0 + n / (x0 * x0)) / three;
    if x <= W::ZERO {
        x = W::ONE;
    }
    loop {
        let y = (x + x + n / (x * x)) / three;
        if y >= x {
            break x;
        }
        x = y;
    }
}
