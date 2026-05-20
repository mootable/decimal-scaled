//! Const-generic fixed-width integers.
//!
//! A single `Uint<const LIMBS: usize>` / `Int<const LIMBS: usize>` pair
//! parameterised by the number of 64-bit little-endian limbs, replacing
//! the family of named `IntXXXX` / `UintXXXX` types. Bit width is
//! derived (`BITS = LIMBS * 64`); every width this crate uses
//! (256 … 16384 bits) is a clean multiple of 64, so the limb count is
//! the natural single parameter — it sidesteps the
//! `LIMBS = ceil(BITS / 64)` derivation that a `BITS`-parameterised type
//! cannot express on stable Rust.
//!
//! Storage is `[u64; LIMBS]`, matching the limb representation the
//! arithmetic primitives already use. Methods delegate to the
//! width-matched limb algorithms; because `LIMBS` is a compile-time
//! constant, the limb loops unroll and any `match LIMBS` arms const-fold
//! per monomorphisation — no runtime dispatch.

use crate::wide_int::limbs_add_assign_u64_fixed;

/// Unsigned fixed-width integer of `N` little-endian 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Uint<const N: usize> {
    limbs: [u64; N],
}

/// Signed (two's-complement) fixed-width integer of `N` little-endian
/// 64-bit limbs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Int<const N: usize> {
    limbs: [u64; N],
}

impl<const N: usize> Uint<N> {
    /// Number of 64-bit limbs.
    pub const LIMBS: usize = N;
    /// Bit width (`LIMBS * 64`).
    pub const BITS: usize = N * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
        Self { limbs }
    };
    /// Largest representable value (all limbs set).
    pub const MAX: Self = Self { limbs: [u64::MAX; N] };

    /// Constructs from raw little-endian limbs.
    #[inline]
    pub const fn from_limbs(limbs: [u64; N]) -> Self {
        Self { limbs }
    }

    /// Borrows the raw little-endian limbs.
    #[inline]
    pub const fn as_limbs(&self) -> &[u64; N] {
        &self.limbs
    }

    /// Wrapping addition (modulo `2^BITS`).
    #[inline]
    pub fn wrapping_add(mut self, rhs: Self) -> Self {
        limbs_add_assign_u64_fixed(&mut self.limbs, &rhs.limbs);
        self
    }
}

impl<const N: usize> Int<N> {
    /// Number of 64-bit limbs.
    pub const LIMBS: usize = N;
    /// Bit width (`LIMBS * 64`).
    pub const BITS: usize = N * 64;

    /// Additive identity.
    pub const ZERO: Self = Self { limbs: [0; N] };
    /// Multiplicative identity.
    pub const ONE: Self = {
        let mut limbs = [0u64; N];
        limbs[0] = 1;
        Self { limbs }
    };

    /// Constructs from raw little-endian two's-complement limbs.
    #[inline]
    pub const fn from_limbs(limbs: [u64; N]) -> Self {
        Self { limbs }
    }

    /// Borrows the raw little-endian limbs.
    #[inline]
    pub const fn as_limbs(&self) -> &[u64; N] {
        &self.limbs
    }
}

// ── Named aliases ──────────────────────────────────────────────────
// Preserve the existing surface so the const-generic types can be
// introduced without renaming every call site at once. Limb counts
// match `decl_wide_int!`: bit_width / 64.

pub type Uint192 = Uint<3>;
pub type Uint256 = Uint<4>;
pub type Uint384 = Uint<6>;
pub type Uint512 = Uint<8>;
pub type Uint768 = Uint<12>;
pub type Uint1024 = Uint<16>;
pub type Uint2048 = Uint<32>;
pub type Uint4096 = Uint<64>;

pub type Int192 = Int<3>;
pub type Int256 = Int<4>;
pub type Int384 = Int<6>;
pub type Int512 = Int<8>;
pub type Int768 = Int<12>;
pub type Int1024 = Int<16>;
pub type Int2048 = Int<32>;
pub type Int4096 = Int<64>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consts_and_round_trip() {
        assert_eq!(Uint::<4>::LIMBS, 4);
        assert_eq!(Uint::<4>::BITS, 256);
        assert_eq!(*Uint::<4>::ZERO.as_limbs(), [0, 0, 0, 0]);
        assert_eq!(*Uint::<4>::ONE.as_limbs(), [1, 0, 0, 0]);
        assert_eq!(*Uint::<4>::MAX.as_limbs(), [u64::MAX; 4]);

        let v = Uint::<4>::from_limbs([7, 8, 9, 10]);
        assert_eq!(*v.as_limbs(), [7, 8, 9, 10]);
    }

    #[test]
    fn aliases_have_expected_widths() {
        assert_eq!(Int256::BITS, 256);
        assert_eq!(Int4096::BITS, 4096);
        assert_eq!(Uint1024::LIMBS, 16);
    }

    #[test]
    fn wrapping_add_carries_across_limbs() {
        // (2^64 - 1) + 1 carries into the next limb.
        let a = Uint::<4>::from_limbs([u64::MAX, 0, 0, 0]);
        let sum = a.wrapping_add(Uint::<4>::ONE);
        assert_eq!(*sum.as_limbs(), [0, 1, 0, 0]);

        // All-ones + 1 wraps to zero (modulo 2^256).
        let wrap = Uint::<4>::MAX.wrapping_add(Uint::<4>::ONE);
        assert_eq!(*wrap.as_limbs(), [0, 0, 0, 0]);
    }
}
