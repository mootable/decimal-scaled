//! Generic wide-integer cube-root kernel — Newton iteration on
//! `mag · 10^(2·SCALE)` over a wide work integer wide enough to cover
//! it.
//!
//! See [`crate::algos::sqrt::generic_wide`] for the parallel sqrt
//! kernel — same parameterisation shape, one free function per tier
//! emitted via a small macro for type-uniformity reasons.

use crate::rounding::RoundingMode;

/// Emits a `cbrt_<tier>(raw, scale, mode) -> Storage` free function.
///
/// Body is the same Newton-iteration + round-mode-aware bump path the
/// original `decl_wide_roots!` macro inlined into every `Dxx<S>` —
/// extracted verbatim so behaviour is byte-identical.
macro_rules! decl_cbrt_kernel {
    ($name:ident, $Storage:ty, $CbrtWide:ty) => {
        /// Cube-root kernel for one wide tier. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, scale: u32, mode: RoundingMode) -> $Storage {
            let storage_zero = <$Storage>::from_str_radix("0", 10)
                .expect("cbrt kernel: invalid base-10 literal");
            if raw == storage_zero {
                return storage_zero;
            }
            let zero = <$CbrtWide>::from_str_radix("0", 10)
                .expect("cbrt kernel: invalid base-10 literal");
            let one = <$CbrtWide>::from_str_radix("1", 10)
                .expect("cbrt kernel: invalid base-10 literal");
            let three = <$CbrtWide>::from_str_radix("3", 10)
                .expect("cbrt kernel: invalid base-10 literal");
            let ten = <$CbrtWide>::from_str_radix("10", 10)
                .expect("cbrt kernel: invalid base-10 literal");

            let widened = raw.resize::<$CbrtWide>();
            let negative = widened < zero;
            let mag = if negative { -widened } else { widened };
            let n: $CbrtWide = mag * ten.pow(2 * scale);

            let sig_bits = <$CbrtWide>::BITS - n.leading_zeros();
            let mut x = one << sig_bits.div_ceil(3);
            loop {
                let y = (x + x + n / (x * x)) / three;
                if y >= x {
                    break;
                }
                x = y;
            }
            let q = x;

            let eight_n = n << 3u32;
            let t = q + q + one;
            let cube = t * t * t;
            let halfway_geq = eight_n >= cube;
            let halfway_gt = eight_n > cube;
            let tie = halfway_geq && !halfway_gt;
            let two_q = q + q;
            let eight_q_cubed = if q == zero { zero } else { two_q * two_q * two_q };
            let residual_nonzero = eight_n > eight_q_cubed;
            let q_is_odd = (q.clone() % (one + one)) != zero;
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
            signed.resize::<$Storage>()
        }
    };
}

// D56: `MAX_SCALE = 57`, kernel needs `mag * 10^(2*SCALE)` ≈ `10^171`
// at the high end which does not fit `Int384`'s ~`10^115` capacity.
// Use `Int768` so the work integer covers the full SCALE range.
#[cfg(any(feature = "d56", feature = "wide"))]
decl_cbrt_kernel!(cbrt_d56, crate::wide_int::Int192, crate::wide_int::Int768);

#[cfg(any(feature = "d76", feature = "wide"))]
decl_cbrt_kernel!(cbrt_d76, crate::wide_int::Int256, crate::wide_int::Int512);

#[cfg(any(feature = "d114", feature = "wide"))]
decl_cbrt_kernel!(cbrt_d114, crate::wide_int::Int384, crate::wide_int::Int768);

#[cfg(any(feature = "d153", feature = "wide"))]
decl_cbrt_kernel!(cbrt_d153, crate::wide_int::Int512, crate::wide_int::Int1024);

#[cfg(any(feature = "d230", feature = "wide"))]
decl_cbrt_kernel!(cbrt_d230, crate::wide_int::Int768, crate::wide_int::Int1536);

#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_cbrt_kernel!(cbrt_d307, crate::wide_int::Int1024, crate::wide_int::Int2048);

#[cfg(any(feature = "d461", feature = "x-wide"))]
decl_cbrt_kernel!(cbrt_d461, crate::wide_int::Int1536, crate::wide_int::Int3072);

#[cfg(any(feature = "d615", feature = "x-wide"))]
decl_cbrt_kernel!(cbrt_d615, crate::wide_int::Int2048, crate::wide_int::Int4096);

#[cfg(any(feature = "d923", feature = "xx-wide"))]
decl_cbrt_kernel!(cbrt_d923, crate::wide_int::Int3072, crate::wide_int::Int6144);

#[cfg(any(feature = "d1231", feature = "xx-wide"))]
decl_cbrt_kernel!(cbrt_d1231, crate::wide_int::Int4096, crate::wide_int::Int8192);
