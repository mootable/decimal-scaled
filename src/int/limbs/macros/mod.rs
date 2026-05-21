//! The `decl_wide_int!` macro — emits a concrete fixed-width signed /
//! unsigned integer type pair, delegating its arithmetic to the slice
//! primitives in the parent module.

// ─────────────────────────────────────────────────────────────────────
// decl_wide_int! — concrete fixed-width signed/unsigned integer newtypes.
//
// Each invocation emits an unsigned `$U` and a two's-complement signed
// `$S`, both stored as `[u64; $L]` little-endian limb arrays so the
// inner arithmetic can use native `u64 × u64 → u128` and `u128 / u64`
// hardware instructions. The public `from_limbs_le` / `limbs_le`
// surface keeps its `[u128; $L]` signature (semver-stable boundary
// conversion). The arithmetic surface is `const fn` so the decimal
// types built on it can be `const`.
// ─────────────────────────────────────────────────────────────────────

/// Emits the `$U` / `$S` integer pair for a fixed limb count.
///
/// - `$U` / `$S` — the unsigned and signed type names.
/// - `$L` — u64 limb count; the bit width is `$L * 64`. Storage is
///   `[u64; $L]` little-endian limbs.
/// - `$D` — doubled u64 limb count (`2 * $L`); sizes the widening
///   multiply / divide intermediate buffer to `[u64; $D]`.
macro_rules! decl_wide_int {
    ($U:ident, $S:ident, $L:tt, $D:tt, $LP1:tt) => {
        // ── Unsigned ──────────────────────────────────────────────────
        /// Hand-rolled fixed-width unsigned integer, little-endian u64
        /// limbs (`$L` of them).
        // `Default` is derived manually rather than via `#[derive]` so it
        // works for any limb count — the standard library only emits
        // `Default` for arrays up to `N = 32`.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $U(pub(crate) [u64; $L]);

        impl ::core::default::Default for $U {
            #[inline]
            fn default() -> Self { Self::ZERO }
        }

        impl $U {
            pub(crate) const ZERO: $U = $U([0u64; $L]);
            pub(crate) const MAX: $U = $U([u64::MAX; $L]);
            pub(crate) const BITS: u32 = $L * 64;

            #[inline]
            pub(crate) const fn is_zero(self) -> bool {
                $crate::wide_int::limbs_is_zero_u64(&self.0)
            }
            #[inline]
            pub(crate) const fn leading_zeros(self) -> u32 {
                Self::BITS - $crate::wide_int::limbs_bit_len_u64(&self.0)
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
                    let mut o = [0u64; $L];
                    o[0] = 1;
                    return $U(o);
                }
                if self.is_power_of_two() {
                    return self;
                }
                let bits = $crate::wide_int::limbs_bit_len_u64(&self.0);
                let mut out = [0u64; $L];
                if (bits as usize) < $L * 64 {
                    out[(bits / 64) as usize] = 1u64 << (bits % 64);
                }
                $U(out)
            }
            /// Reinterprets the bit pattern as the signed sibling.
            #[inline]
            pub(crate) const fn cast_signed(self) -> $S {
                $S(self.0)
            }
            /// Builds from an unsigned 128-bit value (zero-extends
            /// the upper limbs).
            #[inline]
            pub const fn from_u128(v: u128) -> $U {
                let mut out = [0u64; $L];
                out[0] = v as u64;
                if $L > 1 {
                    out[1] = (v >> 64) as u64;
                }
                $U(out)
            }
            /// Approximate `f64` value (positive; truncated toward
            /// zero on overflow).
            pub(crate) fn as_f64(self) -> f64 {
                let mut acc = 0.0f64;
                let radix: f64 = 18_446_744_073_709_551_616.0;
                let mut i = $L;
                while i > 0 {
                    i -= 1;
                    acc = acc * radix + self.0[i] as f64;
                }
                acc
            }
            /// Builds from a non-negative `f64`. Negative inputs
            /// saturate to `ZERO`; non-finite saturates to `MAX`.
            /// Matches the `$S::from_f64` shape but unsigned.
            pub(crate) fn from_f64(v: f64) -> $U {
                if !v.is_finite() {
                    return if v.is_sign_negative() { $U::ZERO } else { $U::MAX };
                }
                if v <= 0.0 {
                    return $U::ZERO;
                }
                let mut m = v;
                let mut limbs = [0u64; $L];
                let mut i = 0;
                let radix: f64 = 18_446_744_073_709_551_616.0;
                while i < $L && m >= 1.0 {
                    // `m >= 1.0` and `radix > 0`, so the Euclidean remainder
                    // is just `m % radix` (result already in `[0, radix)`).
                    // Using the core `%` operator keeps this `no_std`-clean
                    // (`f64::rem_euclid` is std-only) and is bit-identical to
                    // the previous `m.rem_euclid(radix)` for these inputs.
                    let limb = (m % radix) as u64;
                    limbs[i] = limb;
                    m = (m / radix).floor();
                    i += 1;
                }
                if m >= 1.0 {
                    return $U::MAX;
                }
                $U(limbs)
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
                let mut acc = [0u64; $L];
                let ten = [10u64];
                let mut k = 0;
                while k < bytes.len() {
                    let ch = bytes[k];
                    if ch < b'0' || ch > b'9' {
                        return ::core::result::Result::Err(());
                    }
                    let d = (ch - b'0') as u64;
                    let mut scaled = [0u64; $D];
                    $crate::wide_int::limbs_mul_u64(&acc, &ten, &mut scaled);
                    let mut next = [0u64; $L];
                    let mut c = 0;
                    while c < $L {
                        next[c] = scaled[c];
                        c += 1;
                    }
                    $crate::wide_int::limbs_add_assign_u64(&mut next, &[d]);
                    acc = next;
                    k += 1;
                }
                ::core::result::Result::Ok($U(acc))
            }
            pub(crate) const fn pow(self, mut exp: u32) -> $U {
                let mut acc = {
                    let mut o = [0u64; $L];
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
                let mut prod = [0u64; $D];
                $crate::wide_int::limbs_mul_u64_fixed::<$L, $D>(&self.0, &rhs.0, &mut prod);
                let mut out = [0u64; $L];
                let mut i = 0;
                while i < $L {
                    out[i] = prod[i];
                    i += 1;
                }
                $U(out)
            }
            #[inline]
            pub(crate) fn isqrt(self) -> $U {
                let mut out = [0u64; $L];
                $crate::wide_int::limbs_isqrt_u64(&self.0, &mut out);
                $U(out)
            }
        }

        impl ::core::cmp::Ord for $U {
            #[inline]
            fn cmp(&self, other: &$U) -> ::core::cmp::Ordering {
                $crate::wide_int::limbs_cmp_u64(&self.0, &other.0).cmp(&0)
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
                let mut out = [0u64; $L];
                $crate::wide_int::limbs_shr_u64(&self.0, n, &mut out);
                $U(out)
            }
        }
        impl ::core::ops::Shl<u32> for $U {
            type Output = $U;
            #[inline]
            fn shl(self, n: u32) -> $U {
                let mut out = [0u64; $L];
                $crate::wide_int::limbs_shl_u64(&self.0, n, &mut out);
                $U(out)
            }
        }
        impl ::core::ops::Sub for $U {
            type Output = $U;
            #[inline]
            fn sub(mut self, rhs: $U) -> $U {
                $crate::wide_int::limbs_sub_assign_u64(&mut self.0, &rhs.0);
                self
            }
        }
        impl ::core::ops::Div for $U {
            type Output = $U;
            #[inline]
            fn div(self, rhs: $U) -> $U {
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_dispatch_u64(&self.0, &rhs.0, &mut q, &mut r);
                $U(q)
            }
        }
        impl ::core::ops::Rem for $U {
            type Output = $U;
            #[inline]
            fn rem(self, rhs: $U) -> $U {
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_dispatch_u64(&self.0, &rhs.0, &mut q, &mut r);
                $U(r)
            }
        }
        impl ::core::ops::BitAnd for $U {
            type Output = $U;
            #[inline]
            fn bitand(self, rhs: $U) -> $U {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] & rhs.0[i];
                }
                $U(o)
            }
        }
        impl ::core::ops::BitOr for $U {
            type Output = $U;
            #[inline]
            fn bitor(self, rhs: $U) -> $U {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] | rhs.0[i];
                }
                $U(o)
            }
        }
        impl ::core::ops::BitXor for $U {
            type Output = $U;
            #[inline]
            fn bitxor(self, rhs: $U) -> $U {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] ^ rhs.0[i];
                }
                $U(o)
            }
        }

        // ── Signed ────────────────────────────────────────────────────
        /// Hand-rolled fixed-width two's-complement signed integer.
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $S(pub(crate) [u64; $L]);

        impl $S {
            pub(crate) const ZERO: $S = $S([0u64; $L]);
            pub(crate) const ONE: $S = {
                let mut a = [0u64; $L];
                a[0] = 1;
                $S(a)
            };
            pub(crate) const BITS: u32 = $L * 64;
            pub(crate) const MAX: $S = {
                let mut a = [u64::MAX; $L];
                a[$L - 1] = i64::MAX as u64;
                $S(a)
            };
            pub(crate) const MIN: $S = {
                let mut a = [0u64; $L];
                a[$L - 1] = 1u64 << 63;
                $S(a)
            };

            #[inline]
            pub(crate) const fn is_negative(self) -> bool {
                self.0[$L - 1] >> 63 == 1
            }
            #[inline]
            pub(crate) const fn is_zero(self) -> bool {
                $crate::wide_int::limbs_is_zero_u64(&self.0)
            }
            #[inline]
            pub(crate) const fn is_positive(self) -> bool {
                !self.is_negative() && !self.is_zero()
            }
            /// Two's-complement negation (wrapping; `MIN.negate() == MIN`).
            #[inline]
            pub(crate) const fn negate(self) -> $S {
                let mut out = [0u64; $L];
                let mut i = 0;
                while i < $L {
                    out[i] = !self.0[i];
                    i += 1;
                }
                $crate::wide_int::limbs_add_assign_u64(&mut out, &[1]);
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
            /// Builds a signed value from a non-negative magnitude
            /// u64-limb slice and a sign, truncating the magnitude into
            /// `$L` limbs.
            pub(crate) const fn from_mag_limbs(mag: &[u64], negative: bool) -> $S {
                let mut out = [0u64; $L];
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
                Self::BITS - $crate::wide_int::limbs_bit_len_u64(&self.unsigned_abs().0)
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
                let limb = (idx / 64) as usize;
                if limb >= $L {
                    return self.is_negative();
                }
                (self.0[limb] >> (idx % 64)) & 1 == 1
            }
            #[inline]
            pub(crate) const fn trailing_zeros(self) -> u32 {
                let mut i = 0;
                while i < $L {
                    if self.0[i] != 0 {
                        return i as u32 * 64 + self.0[i].trailing_zeros();
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
                if radix != 10 {
                    return ::core::result::Result::Err(());
                }
                let mut acc = [0u64; $L];
                let ten = [10u64];
                let mut k = 0;
                while k < digits.len() {
                    let ch = digits[k];
                    if ch < b'0' || ch > b'9' {
                        return ::core::result::Result::Err(());
                    }
                    let d = (ch - b'0') as u64;
                    let mut scaled = [0u64; $D];
                    $crate::wide_int::limbs_mul_u64(&acc, &ten, &mut scaled);
                    let mut next = [0u64; $L];
                    let mut c = 0;
                    while c < $L {
                        next[c] = scaled[c];
                        c += 1;
                    }
                    $crate::wide_int::limbs_add_assign_u64(&mut next, &[d]);
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
                $crate::wide_int::limbs_add_assign_u64(&mut out, &rhs.0);
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
            /// `self · (n as $S)` with the sign of `self`, panicking
            /// on overflow. Routes through the n-by-1-word
            /// multi-precision primitive [`limbs_mul_u64_into`], which
            /// collapses the `L²` outer-product loop of the generic
            /// `wrapping_mul` down to `L` widening muls + `L`
            /// accumulator-and-carry folds when one operand fits a
            /// single u64. Used by the wide-tier strict transcendentals'
            /// `mul_u(a, n)` helper for `n ≤ u64::MAX`.
            ///
            /// Matches the panic-on-overflow semantics of the
            /// `Mul`-operator path it replaces (which goes through
            /// `checked_mul`): the top carry limb must be zero, and
            /// the magnitude must fit a signed `$S` (i.e. the high
            /// bit of the topmost magnitude limb is not set unless
            /// the result is `$S::MIN`).
            #[inline]
            pub(crate) const fn checked_mul_u64(self, n: u64) -> $S {
                let mag = self.unsigned_abs().0;
                let mut out = [0u64; $LP1];
                $crate::wide_int::limbs_mul_u64_into::<$L, $LP1>(&mag, n, &mut out);
                if out[$L] != 0 {
                    panic!(concat!(stringify!($S), ": mul overflow"));
                }
                let mut prod = [0u64; $L];
                let mut i = 0;
                while i < $L {
                    prod[i] = out[i];
                    i += 1;
                }
                let negative = self.is_negative();
                let r = $S::from_mag_limbs(&prod, negative);
                // `from_mag_limbs` only mishandles the `mag == 2^(BITS-1)`
                // edge: that's `$S::MIN` for `negative = true` (legal)
                // but overflows for `negative = false`.
                if r.is_zero() == false && r.is_negative() != negative {
                    panic!(concat!(stringify!($S), ": mul overflow"));
                }
                r
            }
            /// Full `self · rhs` product widened into a `W: WideInt`,
            /// without going through the `WideInt::to_mag_sign` /
            /// `from_mag_sign` magnitude buffer twice. Used by the
            /// wide-tier `Mul` operator to compute
            /// `$Storage * $Storage → $Wider` in one step.
            #[inline]
            pub(crate) fn widen_mul<W: $crate::wide_int::WideInt>(self, rhs: $S) -> W {
                let negative = self.is_negative() ^ rhs.is_negative();
                let a = self.unsigned_abs();
                let b = rhs.unsigned_abs();
                let mut prod = [0u64; $D];
                $crate::wide_int::limbs_mul_u64_fixed::<$L, $D>(&a.0, &b.0, &mut prod);
                W::from_mag_sign(&prod, negative)
            }
            /// `self / rhs` truncating toward zero. `rhs` must be nonzero.
            pub(crate) const fn wrapping_div(self, rhs: $S) -> $S {
                if rhs.is_zero() {
                    panic!("wide integer: division by zero");
                }
                let negative = self.is_negative() ^ rhs.is_negative();
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_u64(
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
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_u64(
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
                if $crate::wide_int::limbs_eq_u64(&self.0, &$S::MIN.0) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(self.negate())
                }
            }
            pub(crate) const fn checked_mul(self, rhs: $S) -> ::core::option::Option<$S> {
                let negative = self.is_negative() ^ rhs.is_negative();
                let mut prod = [0u64; $D];
                $crate::wide_int::limbs_mul_u64_fixed::<$L, $D>(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut prod,
                );
                let (_lo, hi) = prod.split_at($L);
                if !$crate::wide_int::limbs_is_zero_u64(hi) {
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
                (self.negate(), $crate::wide_int::limbs_eq_u64(&self.0, &$S::MIN.0))
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
                if $crate::wide_int::scmp_u64(self.is_negative(), &self.0, rhs.is_negative(), &rhs.0) >= 0 {
                    self
                } else {
                    rhs
                }
            }
            #[inline]
            pub(crate) const fn min(self, rhs: $S) -> $S {
                if $crate::wide_int::scmp_u64(self.is_negative(), &self.0, rhs.is_negative(), &rhs.0) <= 0 {
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
                // First two u64 limbs make up the low u128; everything
                // else must be zero.
                let (lo, hi) = mag.split_at(2);
                if !$crate::wide_int::limbs_is_zero_u64(hi) {
                    return ::core::option::Option::None;
                }
                let lo_u128 = (lo[0] as u128) | ((lo[1] as u128) << 64);
                if negative {
                    if lo_u128 <= (i128::MAX as u128) + 1 {
                        ::core::option::Option::Some((lo_u128 as i128).wrapping_neg())
                    } else {
                        ::core::option::Option::None
                    }
                } else if lo_u128 <= i128::MAX as u128 {
                    ::core::option::Option::Some(lo_u128 as i128)
                } else {
                    ::core::option::Option::None
                }
            }
            /// Exact `u128` value, or `None` if negative / too large.
            pub const fn to_u128_checked(self) -> ::core::option::Option<u128> {
                let (lo, hi) = self.0.split_at(2);
                if self.is_negative() || !$crate::wide_int::limbs_is_zero_u64(hi) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some((lo[0] as u128) | ((lo[1] as u128) << 64))
                }
            }
            /// Builds from a signed 128-bit value.
            #[inline]
            pub const fn from_i128(v: i128) -> $S {
                let mag = v.unsigned_abs();
                $S::from_mag_limbs(&[mag as u64, (mag >> 64) as u64], v < 0)
            }
            /// Builds directly from the u64 limb array
            /// (`[u64; $L]`). Limbs are interpreted as a
            /// little-endian two's-complement signed integer. Wire-
            /// format compatible with the historic `[u128; N]` shape
            /// on little-endian targets — same byte sequence.
            #[inline]
            pub const fn from_limbs_le(limbs: [u64; $L]) -> $S {
                $S(limbs)
            }
            /// Read-only access to the underlying u64 limbs.
            /// Symmetric with [`Self::from_limbs_le`].
            #[inline]
            pub const fn limbs_le(self) -> [u64; $L] {
                self.0
            }
            /// Builds from an unsigned 128-bit value.
            #[inline]
            pub const fn from_u128(v: u128) -> $S {
                $S::from_mag_limbs(&[v as u64, (v >> 64) as u64], false)
            }
            /// Truncating cast to `i128` (low 128 bits, sign-applied).
            #[inline]
            pub(crate) const fn as_i128(self) -> i128 {
                let mag = self.unsigned_abs().0;
                let lo = (mag[0] as u128) | ((mag[1] as u128) << 64);
                if self.is_negative() {
                    (lo as i128).wrapping_neg()
                } else {
                    lo as i128
                }
            }
            /// Widening / narrowing cast to any other `WideInt`.
            ///
            /// Skips the 288-u64 `wide_cast` staging buffer by
            /// passing this type's own `$L`-limb magnitude slice
            /// directly to `T::from_mag_sign`. The destination
            /// truncates / zero-extends as needed.
            #[inline]
            pub(crate) fn resize<T: $crate::wide_int::WideInt>(self) -> T {
                let negative = self.is_negative();
                let mag = self.unsigned_abs().0;
                T::from_mag_sign(&mag, negative)
            }
            /// Approximate `f64` value.
            pub(crate) fn as_f64(self) -> f64 {
                let mag = self.unsigned_abs().0;
                let mut acc = 0.0f64;
                // 18_446_744_073_709_551_616.0 = 2^64
                let radix: f64 = 18_446_744_073_709_551_616.0;
                for i in (0..($L)).rev() {
                    acc = acc * radix + mag[i] as f64;
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
            /// u64` cast for truncation, so it needs neither
            /// `f64::trunc` nor `f64::abs`.
            pub(crate) fn from_f64(v: f64) -> $S {
                if !v.is_finite() {
                    return $S::ZERO;
                }
                let negative = v < 0.0;
                let mut m = if negative { -v } else { v };
                let radix: f64 = 18_446_744_073_709_551_616.0; // 2^64
                let mut limbs = [0u64; $L];
                let mut i = 0;
                while m >= 1.0 && i < $L {
                    let rem = m % radix;
                    limbs[i] = rem as u64;
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
            // u128 limbs needed to hold this type's magnitude (ceiling
            // division for odd-L types like Int192). Hot-path divide
            // kernels size their stack buffer to this exact width.
            const U128_LIMBS: usize = ($L + 1) / 2;

            #[inline]
            fn to_mag_sign(self) -> ([u64; 288], bool) {
                let mut out = [0u64; 288];
                let mag = self.unsigned_abs().0;
                out[..($L)].copy_from_slice(&mag);
                (out, self.is_negative())
            }
            #[inline]
            fn from_mag_sign(mag: &[u64], negative: bool) -> $S {
                $S::from_mag_limbs(mag, negative)
            }

            /// Direct u64 → u128 limb pack into the caller's `dst`
            /// buffer. Only the type's own `$L` u64 limbs are read
            /// (= `($L + 1) / 2` u128 limbs); the rest of `dst` is
            /// zero-filled. Bypasses the 288-element default buffer.
            #[inline]
            fn mag_into_u128(self, dst: &mut [u128]) -> bool {
                let mag = self.unsigned_abs().0;
                // u128 limbs needed to hold this type's magnitude
                // (ceiling division for odd-L types like Int192).
                const U128_LIMBS: usize = <$S as $crate::wide_int::WideInt>::U128_LIMBS;
                let n_full_pairs = $L / 2;
                let dst_len = dst.len();
                let mut i = 0;
                let m = n_full_pairs.min(dst_len);
                while i < m {
                    dst[i] = (mag[2 * i] as u128) | ((mag[2 * i + 1] as u128) << 64);
                    i += 1;
                }
                // Odd-L tail: one u64 promoted with zero high half.
                if ($L & 1) == 1 && i < dst_len && i < U128_LIMBS {
                    dst[i] = mag[2 * i] as u128;
                    i += 1;
                }
                while i < dst_len {
                    dst[i] = 0;
                    i += 1;
                }
                self.is_negative()
            }

            /// Direct u128 → u64 limb unpack into this type's
            /// magnitude. Only `($L + 1) / 2` u128 limbs are read;
            /// excess is silently dropped (matches the truncating
            /// semantics of the default `from_mag_sign`). Bypasses
            /// the 288-element u64 staging buffer.
            #[inline]
            fn from_mag_sign_u128(mag: &[u128], negative: bool) -> $S {
                const U128_LIMBS: usize = ($L + 1) / 2;
                let mut out = [0u64; $L];
                let m = U128_LIMBS.min(mag.len());
                let mut i = 0;
                let n_full_pairs = $L / 2;
                let copy_pairs = n_full_pairs.min(m);
                while i < copy_pairs {
                    let v = mag[i];
                    out[2 * i] = v as u64;
                    out[2 * i + 1] = (v >> 64) as u64;
                    i += 1;
                }
                // Odd-L tail: only the low u64 of mag[i] survives.
                if ($L & 1) == 1 && i < m {
                    out[2 * i] = mag[i] as u64;
                }
                $S::from_mag_limbs(&out, negative)
            }
        }

        impl ::core::cmp::Ord for $S {
            #[inline]
            fn cmp(&self, other: &$S) -> ::core::cmp::Ordering {
                $crate::wide_int::scmp_u64(
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
        impl $S {
            /// Quotient + remainder in a single divmod call.
            #[inline]
            pub(crate) fn div_rem(self, rhs: $S) -> ($S, $S) {
                if rhs.is_zero() {
                    panic!(concat!(stringify!($S), ": division by zero"));
                }
                let neg_q = self.is_negative() ^ rhs.is_negative();
                let neg_r = self.is_negative();
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_dispatch_u64(
                    &self.unsigned_abs().0,
                    &rhs.unsigned_abs().0,
                    &mut q,
                    &mut r,
                );
                (
                    $S::from_mag_limbs(&q, neg_q),
                    $S::from_mag_limbs(&r, neg_r),
                )
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
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_dispatch_u64(
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
                let mut q = [0u64; $L];
                let mut r = [0u64; $L];
                $crate::wide_int::limbs_divmod_dispatch_u64(
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
                let mut out = [0u64; $L];
                $crate::wide_int::limbs_shl_u64(&self.0, n, &mut out);
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
                    let mut out = [0u64; $L];
                    $crate::wide_int::limbs_shr_u64(&self.0, n, &mut out);
                    $S(out)
                }
            }
        }
        impl ::core::ops::BitAnd for $S {
            type Output = $S;
            #[inline]
            fn bitand(self, rhs: $S) -> $S {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] & rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::BitOr for $S {
            type Output = $S;
            #[inline]
            fn bitor(self, rhs: $S) -> $S {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] | rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::BitXor for $S {
            type Output = $S;
            #[inline]
            fn bitxor(self, rhs: $S) -> $S {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = self.0[i] ^ rhs.0[i];
                }
                $S(o)
            }
        }
        impl ::core::ops::Not for $S {
            type Output = $S;
            #[inline]
            fn not(self) -> $S {
                let mut o = [0u64; $L];
                for i in 0..($L) {
                    o[i] = !self.0[i];
                }
                $S(o)
            }
        }

        // ── Formatting ────────────────────────────────────────────────
        impl ::core::fmt::Display for $U {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 64];
                let s = $crate::wide_int::limbs_fmt_into_u64(&self.0, 10, true, &mut buf);
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
                    let mut buf = [0u8; $L * 64];
                    let s = $crate::wide_int::limbs_fmt_into_u64(&mag.0, 10, true, &mut buf);
                    f.pad_integral(false, "", s)
                } else {
                    ::core::fmt::Display::fmt(&mag, f)
                }
            }
        }
        impl ::core::fmt::LowerHex for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 64];
                let s = $crate::wide_int::limbs_fmt_into_u64(&self.0, 16, true, &mut buf);
                f.pad_integral(true, "0x", s)
            }
        }
        impl ::core::fmt::UpperHex for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 64];
                let s = $crate::wide_int::limbs_fmt_into_u64(&self.0, 16, false, &mut buf);
                f.pad_integral(true, "0x", s)
            }
        }
        impl ::core::fmt::Octal for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 64];
                let s = $crate::wide_int::limbs_fmt_into_u64(&self.0, 8, true, &mut buf);
                f.pad_integral(true, "0o", s)
            }
        }
        impl ::core::fmt::Binary for $S {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut buf = [0u8; $L * 64];
                let s = $crate::wide_int::limbs_fmt_into_u64(&self.0, 2, true, &mut buf);
                f.pad_integral(true, "0b", s)
            }
        }

        // ── From<primitive> conversions ────────────────────────────
        //
        // Lossless widening from every Rust primitive integer to the
        // wide signed and unsigned tiers. Closes the gap that forced
        // downstream callers to write `Int192::from_u128(v as u128)`
        // when `v` was a `u64` (and similar contortions for the
        // smaller primitives). With these impls, `Int192::from(1_u64)`
        // and `From`-based `.into()` work like they do for the
        // built-in integer types.

        impl ::core::convert::From<u8>   for $U { #[inline] fn from(v: u8)   -> $U { <$U>::from_u128(v as u128) } }
        impl ::core::convert::From<u16>  for $U { #[inline] fn from(v: u16)  -> $U { <$U>::from_u128(v as u128) } }
        impl ::core::convert::From<u32>  for $U { #[inline] fn from(v: u32)  -> $U { <$U>::from_u128(v as u128) } }
        impl ::core::convert::From<u64>  for $U { #[inline] fn from(v: u64)  -> $U { <$U>::from_u128(v as u128) } }
        impl ::core::convert::From<u128> for $U { #[inline] fn from(v: u128) -> $U { <$U>::from_u128(v)         } }

        impl ::core::convert::From<u8>   for $S { #[inline] fn from(v: u8)   -> $S { <$S>::from_u128(v as u128) } }
        impl ::core::convert::From<u16>  for $S { #[inline] fn from(v: u16)  -> $S { <$S>::from_u128(v as u128) } }
        impl ::core::convert::From<u32>  for $S { #[inline] fn from(v: u32)  -> $S { <$S>::from_u128(v as u128) } }
        impl ::core::convert::From<u64>  for $S { #[inline] fn from(v: u64)  -> $S { <$S>::from_u128(v as u128) } }
        impl ::core::convert::From<u128> for $S { #[inline] fn from(v: u128) -> $S { <$S>::from_u128(v)         } }
        impl ::core::convert::From<i8>   for $S { #[inline] fn from(v: i8)   -> $S { <$S>::from_i128(v as i128) } }
        impl ::core::convert::From<i16>  for $S { #[inline] fn from(v: i16)  -> $S { <$S>::from_i128(v as i128) } }
        impl ::core::convert::From<i32>  for $S { #[inline] fn from(v: i32)  -> $S { <$S>::from_i128(v as i128) } }
        impl ::core::convert::From<i64>  for $S { #[inline] fn from(v: i64)  -> $S { <$S>::from_i128(v as i128) } }
        impl ::core::convert::From<i128> for $S { #[inline] fn from(v: i128) -> $S { <$S>::from_i128(v)         } }

        // ── Float conversions ──────────────────────────────────────
        //
        // `From<f32>` / `From<f64>` saturate on out-of-range / NaN
        // the same way the underlying `from_f64` does (covers both
        // tiers; the f32 path widens to f64 first). Negative floats
        // saturate to `ZERO` for the unsigned tier and produce the
        // signed magnitude for the signed tier.

        impl ::core::convert::From<f32> for $U { #[inline] fn from(v: f32) -> $U { <$U>::from_f64(v as f64) } }
        impl ::core::convert::From<f64> for $U { #[inline] fn from(v: f64) -> $U { <$U>::from_f64(v)        } }
        impl ::core::convert::From<f32> for $S { #[inline] fn from(v: f32) -> $S { <$S>::from_f64(v as f64) } }
        impl ::core::convert::From<f64> for $S { #[inline] fn from(v: f64) -> $S { <$S>::from_f64(v)        } }

        // Experimental floats (nightly + `experimental-floats`):
        // f16 widens to f64 losslessly; f128 narrows to f64 with
        // up-to-1-ulp-at-f64 quantisation. Both route through the
        // same saturating `from_f64`.
        #[cfg(all(feature = "experimental-floats"))]
        impl ::core::convert::From<f16> for $U { #[inline] fn from(v: f16) -> $U { <$U>::from_f64(v as f64) } }
        #[cfg(all(feature = "experimental-floats"))]
        impl ::core::convert::From<f128> for $U { #[inline] fn from(v: f128) -> $U { <$U>::from_f64(v as f64) } }
        #[cfg(all(feature = "experimental-floats"))]
        impl ::core::convert::From<f16> for $S { #[inline] fn from(v: f16) -> $S { <$S>::from_f64(v as f64) } }
        #[cfg(all(feature = "experimental-floats"))]
        impl ::core::convert::From<f128> for $S { #[inline] fn from(v: f128) -> $S { <$S>::from_f64(v as f64) } }
    };
}

pub(crate) use decl_wide_int;
