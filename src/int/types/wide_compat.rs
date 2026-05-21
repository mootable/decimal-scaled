//! Kernel-facing trait compatibility for the const-generic integers.
//!
//! Implements the `pub(crate)` `WideInt` (this module) and `WideStorage`
//! (added in a sibling step) traits from [`crate::int::limbs`] for
//! [`Int<N>`] and [`Uint<N>`], so the wide-tier algorithm kernels — which
//! compose generically over `WideStorage: WideInt` — can run on the
//! const-generic family without any change to the kernel source.
//!
//! These are the const-generic ports of the bodies the `decl_wide_int!`
//! macro emits for the named `IntXXXX` / `UintXXXX` structs; keeping them
//! in their own file leaves `traits.rs` focused on the public `FixedInt`
//! surface.
//!
//! The shared magnitude staging buffer is 288 u64 limbs (18432 bits),
//! matching the macro impls and covering the widest work integer in the
//! crate (Int16384 = 256 limbs) with isqrt scratch slack.

use super::{Int, Uint};
use crate::int::limbs::WideInt;

/// 288 u64 limbs = 18432 bits — the shared `to_mag_sign` staging width.
const MAG_LIMBS: usize = 288;

impl<const N: usize> WideInt for Int<N> {
    // u128 limbs needed to hold this type's magnitude (ceiling division
    // for odd-N widths like Int<3>). Hot-path divide kernels size their
    // stack buffer to this exact width.
    const U128_LIMBS: usize = (N + 1) / 2;

    #[inline]
    fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool) {
        let mut out = [0u64; MAG_LIMBS];
        let mag = *self.unsigned_abs().as_limbs();
        // N never exceeds MAG_LIMBS for any width the crate instantiates.
        out[..N].copy_from_slice(&mag);
        (out, self.is_negative())
    }

    #[inline]
    fn from_mag_sign(mag: &[u64], negative: bool) -> Self {
        Self::from_mag_limbs(mag, negative)
    }

    /// Direct u64 → u128 limb pack into the caller's `dst` buffer. Only
    /// this type's own `N` u64 limbs are read (= `(N + 1) / 2` u128
    /// limbs); the rest of `dst` is zero-filled. Bypasses the 288-element
    /// default buffer.
    #[inline]
    fn mag_into_u128(self, dst: &mut [u128]) -> bool {
        let mag = *self.unsigned_abs().as_limbs();
        let n_full_pairs = N / 2;
        let dst_len = dst.len();
        let mut i = 0;
        let m = n_full_pairs.min(dst_len);
        while i < m {
            dst[i] = (mag[2 * i] as u128) | ((mag[2 * i + 1] as u128) << 64);
            i += 1;
        }
        // Odd-N tail: one u64 promoted with zero high half.
        if (N & 1) == 1 && i < dst_len && i < Self::U128_LIMBS {
            dst[i] = mag[2 * i] as u128;
            i += 1;
        }
        while i < dst_len {
            dst[i] = 0;
            i += 1;
        }
        self.is_negative()
    }

    /// Direct u128 → u64 limb unpack into this type's magnitude. Only
    /// `(N + 1) / 2` u128 limbs are read; excess is silently dropped
    /// (matching the truncating semantics of the default
    /// `from_mag_sign`). Bypasses the 288-element u64 staging buffer.
    #[inline]
    fn from_mag_sign_u128(mag: &[u128], negative: bool) -> Self {
        let u128_limbs = (N + 1) / 2;
        let mut out = [0u64; N];
        let m = u128_limbs.min(mag.len());
        let n_full_pairs = N / 2;
        let copy_pairs = n_full_pairs.min(m);
        let mut i = 0;
        while i < copy_pairs {
            let v = mag[i];
            out[2 * i] = v as u64;
            out[2 * i + 1] = (v >> 64) as u64;
            i += 1;
        }
        // Odd-N tail: only the low u64 of mag[i] survives.
        if (N & 1) == 1 && i < m {
            out[2 * i] = mag[i] as u64;
        }
        Self::from_mag_limbs(&out, negative)
    }
}

impl<const N: usize> WideInt for Uint<N> {
    const U128_LIMBS: usize = (N + 1) / 2;

    #[inline]
    fn to_mag_sign(self) -> ([u64; MAG_LIMBS], bool) {
        let mut out = [0u64; MAG_LIMBS];
        out[..N].copy_from_slice(self.as_limbs());
        // Unsigned values are never negative.
        (out, false)
    }

    #[inline]
    fn from_mag_sign(mag: &[u64], _negative: bool) -> Self {
        // Magnitude truncated into N limbs; the sign is irrelevant for an
        // unsigned type.
        let mut out = [0u64; N];
        let n = if mag.len() < N { mag.len() } else { N };
        out[..n].copy_from_slice(&mag[..n]);
        Uint::from_limbs(out)
    }

    #[inline]
    fn mag_into_u128(self, dst: &mut [u128]) -> bool {
        let mag = *self.as_limbs();
        let n_full_pairs = N / 2;
        let dst_len = dst.len();
        let mut i = 0;
        let m = n_full_pairs.min(dst_len);
        while i < m {
            dst[i] = (mag[2 * i] as u128) | ((mag[2 * i + 1] as u128) << 64);
            i += 1;
        }
        if (N & 1) == 1 && i < dst_len && i < Self::U128_LIMBS {
            dst[i] = mag[2 * i] as u128;
            i += 1;
        }
        while i < dst_len {
            dst[i] = 0;
            i += 1;
        }
        false
    }

    #[inline]
    fn from_mag_sign_u128(mag: &[u128], _negative: bool) -> Self {
        let u128_limbs = (N + 1) / 2;
        let mut out = [0u64; N];
        let m = u128_limbs.min(mag.len());
        let n_full_pairs = N / 2;
        let copy_pairs = n_full_pairs.min(m);
        let mut i = 0;
        while i < copy_pairs {
            let v = mag[i];
            out[2 * i] = v as u64;
            out[2 * i + 1] = (v >> 64) as u64;
            i += 1;
        }
        if (N & 1) == 1 && i < m {
            out[2 * i] = mag[i] as u64;
        }
        Uint::from_limbs(out)
    }
}
