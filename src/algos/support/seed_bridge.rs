// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Typed-`W` Newton-root *seed* bridge over the cross-algorithm seed leaf.
//!
//! [`sqrt_seed_w`] hands a generic wide integer `W: BigInt` to the
//! `&[u64]`-based seed leaf ([`crate::algo_x_support::seed::sqrt_seed`])
//! and rebuilds the resulting seed back into `W`. It is the seed-only
//! companion of the floor-root surface `W::isqrt` (which produces the
//! *final* root): the wide fixed-point `sqrt` kernel in
//! [`decl_wide_transcendental!`](crate::macros::wide_transcendental) needs
//! the over-estimate *seed* to start its own scaled Newton loop, not the
//! integer floor root, so it cannot use `W::isqrt` directly.
//!
//! The leaf encapsulates the only `std`/`no_std` divergence: under `std`
//! it bootstraps from `f64::sqrt` of the top 64 significant bits of `n`
//! (~53 correct bits in one shot); under `no_std` it falls back to the
//! classical pure-integer 1-bit seed (`2^⌈bits/2⌉`) — pure-integer math,
//! never `libm` / `num_traits::Float`. Either way the seed is a safe
//! over-estimate, so the caller's monotone-downward Newton loop converges
//! to the same floor root.
//!
//! ## `W ↔ &[u64]` bridge
//!
//! The leaf is `&[u64]`-based; `W: BigInt` has an opaque `Limbs` type, so
//! the two are bridged through the kept u128 magnitude surface
//! ([`BigInt::mag_into_u128`] / [`BigInt::from_mag_sign_u128`]): the
//! operand's magnitude is unpacked into a `u64` work slice, fed to the
//! leaf, and the `u64` seed limbs are repacked into u128 limbs to rebuild
//! `W`. No `BigInt` method is added, and the leaf stays a pure `&[u64]`
//! leaf.

use crate::algo_x_support::seed::sqrt_seed;
use crate::int::types::traits::BigInt;

/// Stack-buffer capacity (in `u64` limbs) for the `W ↔ &[u64]` seed
/// bridge — covers the widest work integer in the crate (Int<256> =
/// 256 u64 limbs) with seed-placement slack.
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

/// Newton *seed* for `⌊√n⌋` in the wide integer `W`, sourced from the
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
