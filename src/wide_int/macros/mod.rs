//! The `decl_wide_int!` macro — emits a concrete fixed-width signed /
//! unsigned integer type pair, delegating its arithmetic to the slice
//! primitives in the parent module.

// ─────────────────────────────────────────────────────────────────────
// decl_wide_int! — concrete fixed-width signed/unsigned integer newtypes.
//
// Each invocation emits an unsigned `$U` ([u128; L]) and a
// two's-complement signed `$S` ([u128; L]), both delegating their
// arithmetic to the slice primitives above. The arithmetic surface is
// `const fn` so the decimal types built on it can be `const`.
// ─────────────────────────────────────────────────────────────────────

/// Emits the `$U` / `$S` integer pair for a fixed limb count.
///
/// - `$U` / `$S` — the unsigned and signed type names.
/// - `$L` — limb count (`[u128; $L]`); the bit width is `$L * 128`.
/// - `$D` — `2 * $L`, the buffer width for widening multiply/divide
/// intermediates.
macro_rules! decl_wide_int {
    ($U:ident, $S:ident, $L:tt, $D:tt) => {
        // ── Unsigned ──────────────────────────────────────────────────
        /// Hand-rolled fixed-width unsigned integer, little-endian limbs.
        // `Default` is derived manually rather than via `#[derive]` so it
        // works for any limb count — the standard library only emits
        // `Default` for arrays up to `N = 32`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $U(pub(crate) [u128; $L]);

        impl ::core::default::Default for $U {
            #[inline]
            fn default() -> Self { Self::ZERO }
        }

        impl $U {
            pub(crate) const ZERO: $U = $U([0; $L]);
            pub(crate) const MAX: $U = $U([u128::MAX; $L]);
            pub(crate) const BITS: u32 = $L * 128;

            #[inline]
            pub(crate) const fn is_zero(self) -> bool {
                $crate::wide_int::limbs_is_zero(&self.0)
            }
            #[inline]
            pub(crate) const fn leading_zeros(self) -> u32 {
                Self::BITS - $crate::wide_int::limbs_bit_len(&self.0)
            }
            #[inline]
            pub(crate) const fn count_ones(self) -> u32 {
                let mut total = 0;
                let mut i = 0;
                while i < $L {
                    total += self.0[i].count_ones();
                    i += 1;
                }
                total
            }
            #[inline]
            pub(crate) const fn is_power_of_two(self) -> bool {
                self.count_ones() == 1
            }
            pub(crate) const fn next_power_of_two(self) -> $U {
                if self.is_zero() {
                    let mut o = [0u128; $L];
                    o[0] = 1;
                    return $U(o);
                }
                if self.is_power_of_two() {
                    return self;
                }
                let bits = $crate::wide_int::limbs_bit_len(&self.0);
                let mut out = [0u128; $L];
                if (bits as usize) < $L * 128 {
                    out[(bits / 128) as usize] = 1u128 << (bits % 128);
                }
                $U(out)
            }
            /// Reinterprets the bit pattern as the signed sibling.
            #[inline]
            pub(crate) const fn cast_signed(self) -> $S {
                $S(self.0)
            }
            /// Parses an unsigned decimal string. Only base 10 is
            /// supported.
            pub(crate) const fn from_str_radix(
                s: &str,
                radix: u32,
            ) -> ::core::result::Result<$U, ()> {
                if radix != 10 {
                    return ::core::result::Result::Err(());
                }
                let bytes = s.as_bytes();
                let mut acc = [0u128; $L];
                let ten = [10u128];
                let mut k = 0;
                while k < bytes.len() {
                    let ch = bytes[k];
                    if ch < b'0' || ch > b'9' {
                        return ::core::result::Result::Err(());
                    }
                    let d = (ch - b'0') as u128;
                    let mut scaled = [0u128; $D];
                    $crate::wide_int::limbs_mul(&acc, &ten, &mut scaled);
                    let mut next = [0u128; $L];
                    let mut c = 0;
                    while c < $L {
                        next[c] = scaled[c];
                        c += 1;
                    }
                    $crate::wide_int::limbs_add_assign(&mut next, &[d]);
                    acc = next;
                    k += 1;
                }
                ::core::result::Result::Ok($U(acc))
            }
            pub(crate) const fn pow(self, mut exp: u32) -> $U {
                let mut acc = {
                    let mut o = [0u128; $L];
                    o[0] = 1;
                    $U(o)
                };
                let mut base = self;
                while exp > 0 {
                    if exp & 1 == 1 {
                        acc = acc.wrapping_mul(base);
                    }
                    exp >>= 1;
                    if exp > 0 {
                        base = base.wrapping_mul(base);
                    }
                }
                acc
            }
            #[inline]
            pub(crate) const fn wrapping_mul(self, rhs: $U) -> $U {
                let mut prod = [0u128; $D];
                $crate::wide_int::limbs_mul(&self.0, &rhs.0, &mut prod);
                let mut out = [0u128; $L];
                let mut i = 0;
                while i < $L {
                    out[i] = prod[i];
                    i += 1;
                }
                $U(out)
            }
            #[inline]
            pub(crate) fn isqrt(self) -> $U {
                let mut out = [0u128; $L];
                $crate::wide_int::limbs_isqrt(&self.0, &mut out);
                $U(out)
            }
        }

        impl ::core::cmp::Ord for $U {
            #[inline]
            fn cmp(&self, other: &$U) -> ::core::cmp::Ordering {
                $crate::wide_int::limbs_cmp(&self.0, &other.0).cmp(&0)
            }
        }
        impl ::core::cmp::PartialOrd for $U {
            #[inline]
            fn partial_cmp(&self, other: &$U) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(self.cmp(other))
            }
        }
        impl ::core::ops::Shr<u32> for $U {
            type Output = $U;
            #[inline]
            fn shr(self, n: u32) -> $U {
                let mut out = [0u128; $L];
                $crate::wide_int::limbs_shr(&self.0, n, &mut out);
                $U(out)
            }
        }
        impl ::core::ops::Shl<u32> for $U {
            type Output = $U;
            #[inline]
            fn shl(self, n: u32) -> $U {
                let mut out = [0u128; $L];
                $crate::wide_int::limbs_shl(&self.0, n, &mut out);
                $U(out)
            }
        }
        impl ::core::ops::Sub for $U {
            type Output = $U;
            #[inline]
            fn sub(mut self, rhs: $U) -> $U {
                $crate::wide_int::limbs_sub_assign(&mut self.0, &rhs.0);
                self
            }
        }
        impl ::core::ops::Div for $U {
            type Output = $U;
            #[inline]
            fn div(self, rhs: $U) -> $U {
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                // Runtime dispatcher: Knuth for multi-limb divisors
                // and BZ for very-wide ones. The const-fn
                // `limbs_divmod` is kept for compile-time callers.
                $crate::wide_int::limbs_divmod_dispatch(&self.0, &rhs.0, &mut q, &mut r);
                $U(q)
            }
        }
        impl ::core::ops::Rem for $U {
            type Output = $U;
            #[inline]
            fn rem(self, rhs: $U) -> $U {
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                $crate::wide_int::limbs_divmod_dispatch(&self.0, &rhs.0, &mut q, &mut r);
                $U(r)
            }
        }
        impl ::core::ops::BitAnd for $U {
            type Output = $U;
            #[inline]
            fn bitand(self, rhs: $U) -> $U {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] & rhs.0[i];
                }
                $U(o)
            }
        }
        impl ::core::ops::BitOr for $U {
            type Output = $U;
            #[inline]
            fn bitor(self, rhs: $U) -> $U {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] | rhs.0[i];
                }
                $U(o)
            }
        }
        impl ::core::ops::BitXor for $U {
            type Output = $U;
            #[inline]
            fn bitxor(self, rhs: $U) -> $U {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] ^ rhs.0[i];
                }
                $U(o)
            }
        }

        // ── Signed ────────────────────────────────────────────────────
        /// Hand-rolled fixed-width two's-complement signed integer.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $S(pub(crate) [u128; $L]);

        impl $S {
            pub(crate) const ZERO: $S = $S([0; $L]);
            pub(crate) const ONE: $S = {
                let mut a = [0u128; $L];
                a[0] = 1;
                $S(a)
            };
            pub(crate) const BITS: u32 = $L * 128;
            pub(crate) const MAX: $S = {
                let mut a = [u128::MAX; $L];
                a[$L - 1] = i128::MAX as u128;
                $S(a)
            };
            pub(crate) const MIN: $S = {
                let mut a = [0u128; $L];
                a[$L - 1] = 1u128 << 127;
                $S(a)
            };

            #[inline]
            pub(crate) const fn is_negative(self) -> bool {
                self.0[$L - 1] >> 127 == 1
            }
            #[inline]
            pub(crate) const fn is_zero(self) -> bool {
                $crate::wide_int::limbs_is_zero(&self.0)
            }
            #[inline]
            pub(crate) const fn is_positive(self) -> bool {
                !self.is_negative() && !self.is_zero()
            }
            /// Two's-complement negation (wrapping; `MIN.negate() == MIN`).
            #[inline]
            pub(crate) const fn negate(self) -> $S {
                let mut out = [0u128; $L];
                let mut i = 0;
                while i < $L {
                    out[i] = !self.0[i];
                    i += 1;
                }
                $crate::wide_int::limbs_add_assign(&mut out, &[1]);
                $S(out)
            }
            /// `|self|` as the unsigned sibling. `MIN` maps to `2^(BITS-1)`.
            #[inline]
            pub(crate) const fn unsigned_abs(self) -> $U {
                if self.is_negative() {
                    $U(self.negate().0)
                } else {
                    $U(self.0)
                }
            }
            /// Reinterprets the bit pattern as the unsigned sibling.
            #[inline]
            pub(crate) const fn cast_unsigned(self) -> $U {
                $U(self.0)
            }
            /// Builds a signed value from a non-negative magnitude limb
            /// slice and a sign, truncating the magnitude into `$L`
            /// limbs.
            pub(crate) const fn from_mag_limbs(mag: &[u128], negative: bool) -> $S {
                let mut out = [0u128; $L];
                let n = if mag.len() < $L { mag.len() } else { $L };
                let mut i = 0;
                while i < n {
                    out[i] = mag[i];
                    i += 1;
                }
                let v = $S(out);
                if negative && !v.is_zero() {
                    v.negate()
                } else {
                    v
                }
            }
            #[inline]
            pub(crate) const fn leading_zeros(self) -> u32 {
                Self::BITS - $crate::wide_int::limbs_bit_len(&self.unsigned_abs().0)
            }
            #[inline]
            pub(crate) const fn count_ones(self) -> u32 {
                let mut total = 0;
                let mut i = 0;
                while i < $L {
                    total += self.0[i].count_ones();
                    i += 1;
                }
                total
            }
            #[inline]
            pub(crate) const fn count_zeros(self) -> u32 {
                Self::BITS - self.count_ones()
            }
            /// `true` if bit `idx` of the two's-complement representation
            /// is set.
            #[inline]
            pub(crate) const fn bit(self, idx: u32) -> bool {
                let limb = (idx / 128) as usize;
                if limb >= $L {
                    return self.is_negative();
                }
                (self.0[limb] >> (idx % 128)) & 1 == 1
            }
            #[inline]
            pub(crate) const fn trailing_zeros(self) -> u32 {
                let mut i = 0;
                while i < $L {
                    if self.0[i] != 0 {
                        return i as u32 * 128 + self.0[i].trailing_zeros();
                    }
                    i += 1;
                }
                Self::BITS
            }
            /// Parse a signed decimal magnitude from `s`. Mirrors
            /// `i128::from_str_radix` in shape: accepts an optional
            /// leading `-`, then ASCII digits. Currently only `radix
            /// == 10` is supported; any other value returns `Err(())`.
            /// `const fn` so the consts module can parse the build-
            /// time generated string literals.
            pub const fn from_str_radix(
                s: &str,
                radix: u32,
            ) -> ::core::result::Result<$S, ()> {
                let bytes = s.as_bytes();
                let (negative, digits): (bool, &[u8]) =
                    if !bytes.is_empty() && bytes[0] == b'-' {
                        (true, bytes.split_at(1).1)
                    } else {
                        (false, bytes)
                    };
                // Reparse the digit bytes as a decimal magnitude.
                if radix != 10 {
                    return ::core::result::Result::Err(());
                }
                let mut acc = [0u128; $L];
                let ten = [10u128];
                let mut k = 0;
                while k < digits.len() {
                    let ch = digits[k];
                    if ch < b'0' || ch > b'9' {
                        return ::core::result::Result::Err(());
                    }
                    let d = (ch - b'0') as u128;
                    let mut scaled = [0u128; $D];
                    $crate::wide_int::limbs_mul(&acc, &ten, &mut scaled);
                    let mut next = [0u128; $L];
                    let mut c = 0;
                    while c < $L {
                        next[c] = scaled[c];
                        c += 1;
                    }
                    $crate::wide_int::limbs_add_assign(&mut next, &[d]);
                    acc = next;
                    k += 1;
                }
                ::core::result::Result::Ok($S::from_mag_limbs(&acc, negative))
            }
            /// Integer power: `self^exp` via right-to-left binary
            /// exponentiation. Wraps on overflow (same semantics as
            /// `i128::wrapping_pow`). `const fn`; runs at compile
            /// time when both inputs are const.
            pub const fn pow(self, mut exp: u32) -> $S {
                let mut acc = $S::ONE;
                let mut base = self;
                while exp > 0 {
                    if exp & 1 == 1 {
                        acc = acc.wrapping_mul(base);
                    }
                    exp >>= 1;
                    if exp > 0 {
                        base = base.wrapping_mul(base);
                    }
                }
                acc
            }
            #[inline]
            pub(crate) const fn wrapping_add(self, rhs: $S) -> $S {
                let mut out = self.0;
                $crate::wide_int::limbs_add_assign(&mut out, &rhs.0);
                $S(out)
            }
            #[inline]
            pub(crate) const fn wrapping_sub(self, rhs: $S) -> $S {
                self.wrapping_add(rhs.negate())
            }
            #[inline]
            pub(crate) const fn wrapping_neg(self) -> $S {
                self.negate()
            }
            pub(crate) const fn wrapping_mul(self, rhs: $S) -> $S {
                let negative = self.is_negative() ^ rhs.is_negative();
                let prod = self.unsigned_abs().wrapping_mul(rhs.unsigned_abs());
                $S::from_mag_limbs(&prod.0, negative)
            }
            /// Full `self · rhs` product widened into a `W: WideInt`,
            /// without going through the `WideInt::to_mag_sign` /
            /// `from_mag_sign` 64-limb buffer twice (once for `self`,
            /// once for `rhs`). Used by the wide-tier `Mul` operator
            /// to compute a `$Storage * $Storage → $Wider` step using
            /// the dedicated `$L × $L → 2$L` schoolbook in
            /// [`crate::wide_int::limbs_mul`] (which has a 2×2 fast
            /// path) instead of widening both operands then wrapping a
            /// `2$L × 2$L` multiply.
            #[inline]
            pub(crate) fn widen_mul<W: $crate::wide_int::WideInt>(self, rhs: $S) -> W {
                let negative = self.is_negative() ^ rhs.is_negative();
                let a = self.unsigned_abs();
                let b = rhs.unsigned_abs();
                let mut prod = [0u128; $D];
                // limbs_mul_fast dispatches to Karatsuba above n=16
                // (Int2048 and wider); schoolbook below. Karatsuba's
                // half-sum scratch needs alloc; under !alloc the fast
                // function falls back to limbs_mul anyway.
                $crate::wide_int::limbs_mul_fast(&a.0, &b.0, &mut prod);
                W::from_mag_sign(&prod, negative)
            }
            /// `self / rhs` truncating toward zero. `rhs` must be nonzero.
            pub(crate) const fn wrapping_div(self, rhs: $S) -> $S {
                if rhs.is_zero() {
                    panic!("wide integer: division by zero");
                }
                let negative = self.is_negative() ^ rhs.is_negative();
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                $crate::wide_int::limbs_divmod(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut q,
                    &mut r,
                );
                $S::from_mag_limbs(&q, negative)
            }
            /// `self % rhs`, result carries the sign of `self`.
            pub(crate) const fn wrapping_rem(self, rhs: $S) -> $S {
                if rhs.is_zero() {
                    panic!("wide integer: remainder by zero");
                }
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                $crate::wide_int::limbs_divmod(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut q,
                    &mut r,
                );
                $S::from_mag_limbs(&r, self.is_negative())
            }
            #[inline]
            const fn add_overflows(self, rhs: $S, result: $S) -> bool {
                self.is_negative() == rhs.is_negative()
                    && result.is_negative() != self.is_negative()
            }
            #[inline]
            pub(crate) const fn checked_add(self, rhs: $S) -> ::core::option::Option<$S> {
                let r = self.wrapping_add(rhs);
                if self.add_overflows(rhs, r) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(r)
                }
            }
            #[inline]
            pub(crate) const fn checked_sub(self, rhs: $S) -> ::core::option::Option<$S> {
                let r = self.wrapping_sub(rhs);
                if self.is_negative() != rhs.is_negative()
                    && r.is_negative() != self.is_negative()
                {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(r)
                }
            }
            #[inline]
            pub(crate) const fn checked_neg(self) -> ::core::option::Option<$S> {
                if $crate::wide_int::limbs_eq(&self.0, &$S::MIN.0) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(self.negate())
                }
            }
            pub(crate) const fn checked_mul(self, rhs: $S) -> ::core::option::Option<$S> {
                let negative = self.is_negative() ^ rhs.is_negative();
                let mut prod = [0u128; $D];
                $crate::wide_int::limbs_mul(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut prod,
                );
                let (_lo, hi) = prod.split_at($L);
                if !$crate::wide_int::limbs_is_zero(hi) {
                    return ::core::option::Option::None;
                }
                let r = $S::from_mag_limbs(prod.split_at($L).0, negative);
                if r.is_zero() || r.is_negative() == negative {
                    ::core::option::Option::Some(r)
                } else {
                    ::core::option::Option::None
                }
            }
            #[inline]
            pub(crate) const fn checked_div(self, rhs: $S) -> ::core::option::Option<$S> {
                if rhs.is_zero() {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(self.wrapping_div(rhs))
                }
            }
            #[inline]
            pub(crate) const fn checked_rem(self, rhs: $S) -> ::core::option::Option<$S> {
                if rhs.is_zero() {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(self.wrapping_rem(rhs))
                }
            }
            #[inline]
            pub(crate) fn isqrt(self) -> $S {
                $S(self.unsigned_abs().isqrt().0)
            }
            #[inline]
            pub(crate) const fn abs(self) -> $S {
                if self.is_negative() {
                    self.negate()
                } else {
                    self
                }
            }
            #[inline]
            pub(crate) const fn signum(self) -> $S {
                if self.is_zero() {
                    $S::ZERO
                } else if self.is_negative() {
                    $S::ONE.negate()
                } else {
                    $S::ONE
                }
            }
            #[inline]
            pub(crate) const fn overflowing_add(self, rhs: $S) -> ($S, bool) {
                let r = self.wrapping_add(rhs);
                (r, self.add_overflows(rhs, r))
            }
            #[inline]
            pub(crate) const fn overflowing_sub(self, rhs: $S) -> ($S, bool) {
                let r = self.wrapping_sub(rhs);
                let ov = self.is_negative() != rhs.is_negative()
                    && r.is_negative() != self.is_negative();
                (r, ov)
            }
            #[inline]
            pub(crate) const fn overflowing_neg(self) -> ($S, bool) {
                (self.negate(), $crate::wide_int::limbs_eq(&self.0, &$S::MIN.0))
            }
            #[inline]
            pub(crate) const fn overflowing_rem(self, rhs: $S) -> ($S, bool) {
                (self.wrapping_rem(rhs), false)
            }
            #[inline]
            pub(crate) const fn saturating_add(self, rhs: $S) -> $S {
                match self.checked_add(rhs) {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => {
                        if self.is_negative() { $S::MIN } else { $S::MAX }
                    }
                }
            }
            #[inline]
            pub(crate) const fn saturating_sub(self, rhs: $S) -> $S {
                match self.checked_sub(rhs) {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => {
                        if self.is_negative() { $S::MIN } else { $S::MAX }
                    }
                }
            }
            #[inline]
            pub(crate) const fn saturating_neg(self) -> $S {
                match self.checked_neg() {
                    ::core::option::Option::Some(v) => v,
                    ::core::option::Option::None => $S::MAX,
                }
            }
            pub(crate) fn rotate_left(self, n: u32) -> $S {
                let n = n % Self::BITS;
                if n == 0 {
                    return self;
                }
                let u = self.cast_unsigned();
                $S(((u << n) | (u >> (Self::BITS - n))).0)
            }
            #[inline]
            pub(crate) fn rotate_right(self, n: u32) -> $S {
                self.rotate_left(Self::BITS - (n % Self::BITS))
            }
            #[inline]
            pub(crate) const fn max(self, rhs: $S) -> $S {
                if $crate::wide_int::scmp(self.is_negative(), &self.0, rhs.is_negative(), &rhs.0) >= 0 {
                    self
                } else {
                    rhs
                }
            }
            #[inline]
            pub(crate) const fn min(self, rhs: $S) -> $S {
                if $crate::wide_int::scmp(self.is_negative(), &self.0, rhs.is_negative(), &rhs.0) <= 0 {
                    self
                } else {
                    rhs
                }
            }
            /// Euclidean division: the quotient that leaves a
            /// non-negative remainder.
            pub(crate) const fn div_euclid(self, rhs: $S) -> $S {
                let q = self.wrapping_div(rhs);
                let r = self.wrapping_rem(rhs);
                if r.is_negative() {
                    if rhs.is_negative() {
                        q.wrapping_add($S::ONE)
                    } else {
                        q.wrapping_sub($S::ONE)
                    }
                } else {
                    q
                }
            }
            /// Euclidean remainder — always non-negative.
            pub(crate) const fn rem_euclid(self, rhs: $S) -> $S {
                let r = self.wrapping_rem(rhs);
                if r.is_negative() {
                    r.wrapping_add(rhs.abs())
                } else {
                    r
                }
            }
            /// Exact `i128` value, or `None` if it does not fit.
            pub const fn to_i128_checked(self) -> ::core::option::Option<i128> {
                let negative = self.is_negative();
                let mag = self.unsigned_abs().0;
                let (lo, hi) = mag.split_at(1);
                if !$crate::wide_int::limbs_is_zero(hi) {
                    return ::core::option::Option::None;
                }
                if negative {
                    if lo[0] <= (i128::MAX as u128) + 1 {
                        ::core::option::Option::Some((lo[0] as i128).wrapping_neg())
                    } else {
                        ::core::option::Option::None
                    }
                } else if lo[0] <= i128::MAX as u128 {
                    ::core::option::Option::Some(lo[0] as i128)
                } else {
                    ::core::option::Option::None
                }
            }
            /// Exact `u128` value, or `None` if negative / too large.
            pub const fn to_u128_checked(self) -> ::core::option::Option<u128> {
                let (lo, hi) = self.0.split_at(1);
                if self.is_negative() || !$crate::wide_int::limbs_is_zero(hi) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(lo[0])
                }
            }
            /// Builds from a signed 128-bit value.
            #[inline]
            pub const fn from_i128(v: i128) -> $S {
                $S::from_mag_limbs(&[v.unsigned_abs()], v < 0)
            }
            /// Builds directly from the limb array. The limbs are
            /// interpreted as a little-endian two's-complement
            /// signed integer (i.e. the same in-memory shape
            /// `decl_wide_int!` uses). Useful for the serde binary
            /// path, which transports the raw limbs unchanged.
            #[inline]
            pub const fn from_limbs_le(limbs: [u128; $L]) -> $S {
                $S(limbs)
            }
            /// Read-only access to the underlying limbs. The
            /// little-endian two's-complement signed shape;
            /// symmetric with [`Self::from_limbs_le`].
            #[inline]
            pub const fn limbs_le(self) -> [u128; $L] {
                self.0
            }
            /// Builds from an unsigned 128-bit value.
            #[inline]
            pub const fn from_u128(v: u128) -> $S {
                $S::from_mag_limbs(&[v], false)
            }
            /// Truncating cast to `i128` (low 128 bits, sign-applied).
            #[inline]
            pub(crate) const fn as_i128(self) -> i128 {
                let mag = self.unsigned_abs().0[0];
                if self.is_negative() {
                    (mag as i128).wrapping_neg()
                } else {
                    mag as i128
                }
            }
            /// Widening / narrowing cast to any other `WideInt`.
            #[inline]
            pub(crate) fn resize<T: $crate::wide_int::WideInt>(self) -> T {
                $crate::wide_int::wide_cast(self)
            }
            /// Approximate `f64` value.
            pub(crate) fn as_f64(self) -> f64 {
                let mag = self.unsigned_abs().0;
                let mut acc = 0.0f64;
                for i in (0..$L).rev() {
                    acc = acc * 340_282_366_920_938_463_463_374_607_431_768_211_456.0
                        + mag[i] as f64;
                }
                if self.is_negative() {
                    -acc
                } else {
                    acc
                }
            }
            /// Builds from an `f64`, truncating toward zero. Saturates
            /// to `MIN` / `MAX` on out-of-range / non-finite input.
            ///
            /// `core`-only: the limb extraction relies on the `f64 →
            /// u128` cast for truncation, so it needs neither
            /// `f64::trunc` nor `f64::abs`.
            pub(crate) fn from_f64(v: f64) -> $S {
                if !v.is_finite() {
                    return $S::ZERO;
                }
                let negative = v < 0.0;
                let mut m = if negative { -v } else { v };
                let radix = 340_282_366_920_938_463_463_374_607_431_768_211_456.0;
                let mut limbs = [0u128; $L];
                let mut i = 0;
                while m >= 1.0 && i < $L {
                    // `rem` is the value modulo 2^128; the `as u128` cast
                    // truncates its fractional part. Subtracting `rem`
                    // leaves a clean multiple of the radix to shift down.
                    let rem = m % radix;
                    limbs[i] = rem as u128;
                    m = (m - rem) / radix;
                    i += 1;
                }
                if m >= 1.0 {
                    return if negative { $S::MIN } else { $S::MAX };
                }
                $S::from_mag_limbs(&limbs, negative)
            }
        }

        impl $crate::wide_int::WideInt for $S {
            #[inline]
            fn to_mag_sign(self) -> ([u128; 64], bool) {
                let mut out = [0u128; 64];
                let mag = self.unsigned_abs().0;
                out[..$L].copy_from_slice(&mag);
                (out, self.is_negative())
            }
            #[inline]
            fn from_mag_sign(mag: &[u128], negative: bool) -> $S {
                $S::from_mag_limbs(mag, negative)
            }
        }

        impl ::core::cmp::Ord for $S {
            #[inline]
            fn cmp(&self, other: &$S) -> ::core::cmp::Ordering {
                $crate::wide_int::scmp(
                    self.is_negative(),
                    &self.0,
                    other.is_negative(),
                    &other.0,
                )
                .cmp(&0)
            }
        }
        impl ::core::cmp::PartialOrd for $S {
            #[inline]
            fn partial_cmp(&self, other: &$S) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(self.cmp(other))
            }
        }
        impl ::core::ops::Add for $S {
            type Output = $S;
            #[inline]
            fn add(self, rhs: $S) -> $S {
                self.checked_add(rhs)
                    .expect(concat!(stringify!($S), ": add overflow"))
            }
        }
        impl ::core::ops::Sub for $S {
            type Output = $S;
            #[inline]
            fn sub(self, rhs: $S) -> $S {
                self.checked_sub(rhs)
                    .expect(concat!(stringify!($S), ": sub overflow"))
            }
        }
        impl ::core::ops::Mul for $S {
            type Output = $S;
            #[inline]
            fn mul(self, rhs: $S) -> $S {
                self.checked_mul(rhs)
                    .expect(concat!(stringify!($S), ": mul overflow"))
            }
        }
        impl ::core::ops::Div for $S {
            type Output = $S;
            #[inline]
            fn div(self, rhs: $S) -> $S {
                if rhs.is_zero() {
                    panic!(concat!(stringify!($S), ": division by zero"));
                }
                let negative = self.is_negative() ^ rhs.is_negative();
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                // Runtime dispatcher: Knuth / BZ for multi-limb (or
                // single-limb-too-wide-for-u64) divisors. The
                // `wrapping_div` sibling stays const for compile-time
                // evaluation; it uses the slower const path.
                $crate::wide_int::limbs_divmod_dispatch(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut q,
                    &mut r,
                );
                $S::from_mag_limbs(&q, negative)
            }
        }
        impl ::core::ops::Rem for $S {
            type Output = $S;
            #[inline]
            fn rem(self, rhs: $S) -> $S {
                if rhs.is_zero() {
                    panic!(concat!(stringify!($S), ": remainder by zero"));
                }
                let mut q = [0u128; $L];
                let mut r = [0u128; $L];
                $crate::wide_int::limbs_divmod_dispatch(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut q,
                    &mut r,
                );
                $S::from_mag_limbs(&r, self.is_negative())
            }
        }
        impl ::core::ops::Neg for $S {
            type Output = $S;
            #[inline]
            fn neg(self) -> $S {
                self.checked_neg()
                    .expect(concat!(stringify!($S), ": neg overflow"))
            }
        }
        impl ::core::ops::Shl<u32> for $S {
            type Output = $S;
            #[inline]
            fn shl(self, n: u32) -> $S {
                let mut out = [0u128; $L];
                $crate::wide_int::limbs_shl(&self.0, n, &mut out);
                $S(out)
            }
        }
        impl ::core::ops::Shr<u32> for $S {
            type Output = $S;
            /// Arithmetic right shift (sign-preserving).
            #[inline]
            fn shr(self, n: u32) -> $S {
                if self.is_negative() {
                    let mag = self.unsigned_abs();
                    let shifted = mag >> n;
                    $S::from_mag_limbs(&shifted.0, true)
                } else {
                    let mut out = [0u128; $L];
                    $crate::wide_int::limbs_shr(&self.0, n, &mut out);
                    $S(out)
                }
            }
        }
        impl ::core::ops::BitAnd for $S {
            type Output = $S;
            #[inline]
            fn bitand(self, rhs: $S) -> $S {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] & rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::BitOr for $S {
            type Output = $S;
            #[inline]
            fn bitor(self, rhs: $S) -> $S {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] | rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::BitXor for $S {
            type Output = $S;
            #[inline]
            fn bitxor(self, rhs: $S) -> $S {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = self.0[i] ^ rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::Not for $S {
            type Output = $S;
            #[inline]
            fn not(self) -> $S {
                let mut o = [0u128; $L];
                for i in 0..$L {
                    o[i] = !self.0[i];
                }
                $S(o)
            }
        }

        // ── Formatting ────────────────────────────────────────────────
        impl ::core::fmt::Display for $U {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 128];
                let s = $crate::wide_int::limbs_fmt_into(&self.0, 10, true, &mut buf);
                f.pad_integral(true, "", s)
            }
        }
        impl ::core::fmt::Display for $S {
            /// Decimal string of the signed value with a leading `-`
            /// for negatives. Needed by the serde wire-format path
            /// for wide-tier decimals, and useful in its own right
            /// for printing raw storage values.
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mag = self.unsigned_abs();
                if self.is_negative() && !mag.is_zero() {
                    let mut buf = [0u8; $L * 128];
                    let s = $crate::wide_int::limbs_fmt_into(&mag.0, 10, true, &mut buf);
                    f.pad_integral(false, "", s)
                } else {
                    ::core::fmt::Display::fmt(&mag, f)
                }
            }
        }
        impl ::core::fmt::LowerHex for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 128];
                let s = $crate::wide_int::limbs_fmt_into(&self.0, 16, true, &mut buf);
                f.pad_integral(true, "0x", s)
            }
        }
        impl ::core::fmt::UpperHex for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 128];
                let s = $crate::wide_int::limbs_fmt_into(&self.0, 16, false, &mut buf);
                f.pad_integral(true, "0x", s)
            }
        }
        impl ::core::fmt::Octal for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 128];
                let s = $crate::wide_int::limbs_fmt_into(&self.0, 8, true, &mut buf);
                f.pad_integral(true, "0o", s)
            }
        }
        impl ::core::fmt::Binary for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 128];
                let s = $crate::wide_int::limbs_fmt_into(&self.0, 2, true, &mut buf);
                f.pad_integral(true, "0b", s)
            }
        }
    };
}

pub(crate) use decl_wide_int;
