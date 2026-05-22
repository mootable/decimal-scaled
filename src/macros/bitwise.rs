//! Macro-generated bitwise operators and bit-manipulation methods for
//! the decimal widths.
//!
//! Every operator and method here works on the **raw storage bits**
//! (`value * 10^SCALE`), not the logical decimal value.
//!
//! The operator surface (`BitAnd` / `BitOr` / `BitXor` / `Not` / `Shl`
//! / `Shr` and the `*Assign` forms) is textually identical across the
//! `Int<N>` storage widths, so it lives in a shared `@common` arm. The
//! inherent bit-manipulation methods spell the "reinterpret as unsigned"
//! step with the `Int<N>` storage's `cast_unsigned` / `cast_signed`.

/// Emits the bitwise operator + method surface for a decimal type.
///
/// - `decl_decimal_bitwise!(wide D76, Int<4>)` — `Int<N>` storage.
macro_rules! decl_decimal_bitwise {
    // Wide storage.
    (wide $Type:ident, $Storage:ty) => {
        $crate::macros::bitwise::decl_decimal_bitwise!(@common $Type, $Storage);

        impl<const SCALE: u32> $Type<SCALE> {
            /// Logical (zero-fill) right shift of the raw storage by `n`
            /// bits. Unlike the arithmetic `Shr` operator, the vacated
            /// high bits are always zero regardless of sign.
            #[inline]
            #[must_use]
            pub fn unsigned_shr(self, n: u32) -> Self {
                Self((self.0.cast_unsigned() >> n).cast_signed())
            }

            /// Rotate the raw storage left by `n` bits.
            #[inline]
            #[must_use]
            pub fn rotate_left(self, n: u32) -> Self {
                Self(self.0.rotate_left(n))
            }

            /// Rotate the raw storage right by `n` bits.
            #[inline]
            #[must_use]
            pub fn rotate_right(self, n: u32) -> Self {
                Self(self.0.rotate_right(n))
            }

            /// Number of leading zero bits in the raw storage.
            #[inline]
            #[must_use]
            pub fn leading_zeros(self) -> u32 {
                self.0.leading_zeros()
            }

            /// Number of trailing zero bits in the raw storage.
            #[inline]
            #[must_use]
            pub fn trailing_zeros(self) -> u32 {
                self.0.trailing_zeros()
            }

            /// Population count of the raw storage.
            #[inline]
            #[must_use]
            pub fn count_ones(self) -> u32 {
                self.0.count_ones()
            }

            /// Number of zero bits in the raw storage.
            #[inline]
            #[must_use]
            pub fn count_zeros(self) -> u32 {
                self.0.count_zeros()
            }

            /// `true` if the raw storage, viewed as unsigned, is a power
            /// of two.
            #[inline]
            #[must_use]
            pub fn is_power_of_two(self) -> bool {
                self.0.cast_unsigned().is_power_of_two()
            }

            /// Smallest power of two >= the raw storage viewed as
            /// unsigned. Panics in debug builds on overflow.
            #[inline]
            #[must_use]
            pub fn next_power_of_two(self) -> Self {
                Self(self.0.cast_unsigned().next_power_of_two().cast_signed())
            }
        }
    };


    // Shared operator surface — identical for native and wide storage.
    (@common $Type:ident, $Storage:ty) => {
        impl<const SCALE: u32> ::core::ops::BitAnd for $Type<SCALE> {
            type Output = Self;
            /// Bitwise AND of the raw storage values.
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }
        impl<const SCALE: u32> ::core::ops::BitAndAssign for $Type<SCALE> {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 = self.0 & rhs.0;
            }
        }
        impl<const SCALE: u32> ::core::ops::BitOr for $Type<SCALE> {
            type Output = Self;
            /// Bitwise OR of the raw storage values.
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
        impl<const SCALE: u32> ::core::ops::BitOrAssign for $Type<SCALE> {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 = self.0 | rhs.0;
            }
        }
        impl<const SCALE: u32> ::core::ops::BitXor for $Type<SCALE> {
            type Output = Self;
            /// Bitwise XOR of the raw storage values.
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
        impl<const SCALE: u32> ::core::ops::BitXorAssign for $Type<SCALE> {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 = self.0 ^ rhs.0;
            }
        }
        impl<const SCALE: u32> ::core::ops::Shl<u32> for $Type<SCALE> {
            type Output = Self;
            /// Left-shift the raw storage by `n` bits. Debug-panics when
            /// `n` exceeds the storage width; wraps in release.
            #[inline]
            fn shl(self, n: u32) -> Self {
                Self(self.0 << n)
            }
        }
        impl<const SCALE: u32> ::core::ops::ShlAssign<u32> for $Type<SCALE> {
            #[inline]
            fn shl_assign(&mut self, n: u32) {
                self.0 = self.0 << n;
            }
        }
        impl<const SCALE: u32> ::core::ops::Shr<u32> for $Type<SCALE> {
            type Output = Self;
            /// Arithmetic (sign-extending) right-shift of the raw
            /// storage by `n` bits. Use [`Self::unsigned_shr`] for the
            /// logical (zero-fill) shift.
            #[inline]
            fn shr(self, n: u32) -> Self {
                Self(self.0 >> n)
            }
        }
        impl<const SCALE: u32> ::core::ops::ShrAssign<u32> for $Type<SCALE> {
            #[inline]
            fn shr_assign(&mut self, n: u32) {
                self.0 = self.0 >> n;
            }
        }
        impl<const SCALE: u32> ::core::ops::Not for $Type<SCALE> {
            type Output = Self;
            /// Bitwise complement of the raw storage (flip every bit).
            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }
    };
}

/// Maps a native signed storage type to its unsigned counterpart, used
/// by the `unsigned_shr` / `is_power_of_two` / `next_power_of_two`
/// "reinterpret as unsigned" step in the native arm of
/// `decl_decimal_bitwise!`.
pub(crate) trait Unsigned {
    type U;
}
impl Unsigned for i32 {
    type U = u32;
}
impl Unsigned for i64 {
    type U = u64;
}
impl Unsigned for i128 {
    type U = u128;
}

pub(crate) use decl_decimal_bitwise;
