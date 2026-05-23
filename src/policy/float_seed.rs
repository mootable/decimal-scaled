//! Float-bridge Newton seeds for integer roots — the one place the
//! `std` vs `no_std` divergence for the seeded integer `isqrt` / `icbrt`
//! kernels lives.
//!
//! The wide-integer Newton root loops in [`crate::algos::sqrt`] and
//! [`crate::algos::cbrt`] converge from any positive seed, but the
//! *number* of iterations — each a multi-limb Knuth divide, the
//! dominant cost — depends entirely on seed quality. The seed itself is
//! produced by the cross-algorithm seed leaf
//! ([`crate::algo_x_support::seed`]), which encapsulates **both** the
//! `std` hardware-`f64` bootstrap (~53 correct bits in one shot) and the
//! `no_std` pure-integer 1-bit fallback internally:
//!
//! - **`std`** — the leaf seeds from `f64::sqrt` / `f64::cbrt` of the top
//!   64 significant bits of `n`, scaled back to `n`'s magnitude, so Newton
//!   needs ~2 iterations instead of ~7.
//! - **`no_std`** — the leaf falls back to the classical 1-bit seed
//!   (`2^⌈bits/2⌉` for sqrt, `2^⌈bits/3⌉` for cbrt) — pure-integer math,
//!   never `libm` / `num_traits::Float`.
//!
//! Both paths return the **same** `⌊√n⌋` / `⌊∛n⌋` — only the seed
//! source, and hence the iteration count, differs. Hoisting the choice
//! into the leaf lets the root kernels stay cfg-free: they call
//! [`isqrt`] / [`icbrt`] and never name a float method directly.
//!
//! ## `W ↔ &[u64]` bridge
//!
//! The seed leaf is `&[u64]`-based; the consumers here are generic over
//! `W: BigInt`, whose `Limbs` associated type is opaque (`Copy` only, no
//! slice view). The two are bridged through the kept u128 magnitude
//! surface ([`BigInt::mag_into_u128`] / [`BigInt::from_mag_sign_u128`]):
//! the operand's magnitude is unpacked into a `u64` work slice, fed to
//! the leaf, and the `u64` seed limbs are repacked into u128 limbs to
//! rebuild `W`. No `BigInt` method is added, and the leaf stays a pure
//! `&[u64]` leaf.

use crate::algo_x_support::seed::{cbrt_seed, sqrt_seed};
use crate::int::types::traits::BigInt;

/// Stack-buffer capacity (in `u64` limbs) for the `W ↔ &[u64]` seed
/// bridge — covers the widest work integer in the crate (Int<256> =
/// 256 u64 limbs) with seed-placement slack, matching the
/// `SCRATCH_LIMBS` budget the `isqrt`/`icbrt` Newton kernels use.
const BRIDGE_LIMBS: usize = 288;
/// u128-limb capacity for the magnitude round-trip (`⌈BRIDGE_LIMBS / 2⌉`).
const BRIDGE_U128_LIMBS: usize = BRIDGE_LIMBS / 2 + 1;

/// Unpacks the magnitude of `n` into the `u64` work slice `out_u64`
/// (little-endian), returning the populated length. Bridges the kept
/// u128 magnitude surface ([`BigInt::mag_into_u128`]) to the seed leaf's
/// `&[u64]` interface — pure primitive limb splitting, no `BigInt`
/// method beyond the existing magnitude bridge.
#[inline]
fn mag_to_u64<W: BigInt>(n: W, out_u64: &mut [u64]) -> usize {
    let u128_len = (W::LIMBS + 1) >> 1;
    let mut mag = [0u128; BRIDGE_U128_LIMBS];
    n.mag_into_u128(&mut mag[..u128_len]);
    let mut i = 0;
    while i < u128_len {
        let v = mag[i];
        out_u64[2 * i] = v as u64;
        out_u64[2 * i + 1] = (v >> 64) as u64;
        i += 1;
    }
    2 * u128_len
}

/// Rebuilds a non-negative `W` from the little-endian `u64` seed limbs
/// `seed_u64`. The inverse of [`mag_to_u64`]: repacks pairs of `u64`
/// limbs into u128 limbs and hands them to the kept
/// [`BigInt::from_mag_sign_u128`] bridge.
#[inline]
fn u64_to_w<W: BigInt>(seed_u64: &[u64]) -> W {
    let u64_len = seed_u64.len();
    let u128_len = (u64_len + 1) >> 1;
    let mut mag = [0u128; BRIDGE_U128_LIMBS];
    let mut i = 0;
    while i < u128_len {
        let lo = seed_u64[2 * i] as u128;
        let hi = if 2 * i + 1 < u64_len {
            (seed_u64[2 * i + 1] as u128) << 64
        } else {
            0
        };
        mag[i] = lo | hi;
        i += 1;
    }
    W::from_mag_sign_u128(&mag[..u128_len], false)
}

/// Newton seed for `⌊√n⌋` in the wide integer `W`, sourced from the
/// cross-algorithm seed leaf ([`crate::algo_x_support::seed::sqrt_seed`]).
///
/// Bridges the leaf's `&[u64]` interface to generic `W` (see the module
/// docs): unpacks `n`'s magnitude to a `u64` work slice, calls the leaf,
/// repacks the `u64` seed limbs into `W`. The leaf chooses the `std` f64
/// bootstrap or the `no_std` 1-bit fallback internally; both are safe
/// over-estimates. Always returns `≥ W::ONE`.
#[inline]
#[must_use]
pub(crate) fn sqrt_seed_w<W: BigInt>(n: W) -> W {
    let bits = n.bit_length();
    if bits <= 1 {
        // n == 1 → ⌊√1⌋ = 1; the leaf's preconditions assume bits ≥ 2.
        return W::ONE;
    }
    let mut n_u64 = [0u64; BRIDGE_LIMBS];
    let n_len = mag_to_u64(n, &mut n_u64);
    let mut seed_u64 = [0u64; BRIDGE_LIMBS];
    sqrt_seed(&n_u64[..n_len], bits, &mut seed_u64[..n_len]);
    u64_to_w::<W>(&seed_u64[..n_len])
}

/// Newton seed for `⌊∛n⌋` in the wide integer `W`, sourced from the
/// cross-algorithm seed leaf ([`crate::algo_x_support::seed::cbrt_seed`]).
/// Companion of [`sqrt_seed_w`]; same `W ↔ &[u64]` bridge.
#[inline]
#[must_use]
pub(crate) fn cbrt_seed_w<W: BigInt>(n: W) -> W {
    let bits = n.bit_length();
    if bits <= 1 {
        return W::ONE;
    }
    let mut n_u64 = [0u64; BRIDGE_LIMBS];
    let n_len = mag_to_u64(n, &mut n_u64);
    let mut seed_u64 = [0u64; BRIDGE_LIMBS];
    cbrt_seed(&n_u64[..n_len], bits, &mut seed_u64[..n_len]);
    u64_to_w::<W>(&seed_u64[..n_len])
}

/// `⌊√n⌋` for `n > 0`, computed in the wide integer `W`.
///
/// The Newton loop is seeded via [`sqrt_seed_w`] (the cross-algorithm
/// seed leaf — `std` f64 bootstrap or `no_std` 1-bit fallback). One
/// unconditional AM-GM pre-step (`(x + n/x)/2 ≥ ⌈√n⌉`) restores the
/// monotone-decrease precondition regardless of the seed direction. The
/// returned `⌊√n⌋` is identical on `std` and `no_std`.
#[inline]
#[must_use]
pub(crate) fn isqrt<W: BigInt>(n: W) -> W {
    let seed = sqrt_seed_w::<W>(n);
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

/// `⌊∛n⌋` for `n > 0`, computed in the wide integer `W`.
///
/// The Newton cube-root loop is seeded via [`cbrt_seed_w`] (the
/// cross-algorithm seed leaf — `std` f64 bootstrap or `no_std` 1-bit
/// fallback) with one unconditional AM-GM pre-step (`(x, x, n/x²)` ⇒
/// `≥ ⌈∛n⌉`). Both `std` and `no_std` return the same `⌊∛n⌋`.
///
/// Callers needing the *full generic-kernel* rounding (the
/// `generic_wide::cbrt` half-step logic) supply this as the loop core;
/// the bespoke `D57<20>` kernel uses it directly.
#[inline]
#[must_use]
pub(crate) fn icbrt<W: BigInt>(n: W) -> W {
    let three = W::ONE + W::ONE + W::ONE;
    let seed = cbrt_seed_w::<W>(n);
    let x0 = if seed <= W::ZERO { W::ONE } else { seed };

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
