//! `bnum`-backed 256-bit decimal — a benchmark-only baseline.
//!
//! The crate's wide decimal tiers run on the in-tree hand-rolled
//! integer backend. This minimal `bnum`-backed decimal exists solely
//! so the head-to-head benchmark can measure the hand-rolled backend
//! against an established big-integer library. It is **not** part of
//! the crate's public surface and is compiled only for benchmarks —
//! `bnum` is a dev-dependency, not a normal dependency.
//!
//! Only the operations the benchmark exercises are implemented:
//! `from_int` and the four arithmetic operators.

use bnum::cast::As;
use bnum::types::{I256, I512};

/// A 256-bit fixed-point decimal whose storage is `value * 10^SCALE`,
/// backed by `bnum`'s `I256`.
#[derive(Clone, Copy)]
pub struct BnumD256<const SCALE: u32>(pub I256);

impl<const SCALE: u32> BnumD256<SCALE> {
    /// `10^SCALE` as the storage type.
    #[inline]
    fn multiplier() -> I256 {
        I256::from_str_radix("10", 10)
            .expect("bnum baseline: invalid base-10 literal")
            .pow(SCALE)
    }

    /// Constructs from an integer by scaling to `value * 10^SCALE`.
    #[inline]
    pub fn from_int(value: i128) -> Self {
        let widened: I256 = value.as_();
        BnumD256(widened * Self::multiplier())
    }
}

impl<const SCALE: u32> core::ops::Add for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        BnumD256(self.0 + rhs.0)
    }
}

impl<const SCALE: u32> core::ops::Sub for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        BnumD256(self.0 - rhs.0)
    }
}

impl<const SCALE: u32> core::ops::Mul for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let a: I512 = self.0.as_();
        let b: I512 = rhs.0.as_();
        let m: I512 = I512::from_str_radix("10", 10)
            .expect("bnum baseline: invalid base-10 literal")
            .pow(SCALE);
        BnumD256((a * b / m).as_())
    }
}

impl<const SCALE: u32> core::ops::Div for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let a: I512 = self.0.as_();
        let b: I512 = rhs.0.as_();
        let m: I512 = I512::from_str_radix("10", 10)
            .expect("bnum baseline: invalid base-10 literal")
            .pow(SCALE);
        BnumD256((a * m / b).as_())
    }
}

impl<const SCALE: u32> core::ops::Rem for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        BnumD256(self.0 % rhs.0)
    }
}

impl<const SCALE: u32> core::ops::Neg for BnumD256<SCALE> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        BnumD256(-self.0)
    }
}
