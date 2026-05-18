//! Generic wide-integer cube-root kernel — Newton iteration on
//! `mag · 10^(2·SCALE)` over a wide work integer wide enough to cover
//! it.
//!
//! See [`crate::algos::sqrt::generic_wide`] for the parallel sqrt
//! kernel — same parameterisation shape. The kernel is parameterised
//! over `(Storage, CbrtWide)` via the [`crate::wide_int::WideStorage`]
//! trait; a small per-tier shim macro forwards each tier to the
//! generic [`cbrt`] function so call sites stay unchanged.
//!
//! # Why the work integer is wider than `Storage * 2`
//!
//! Each wide tier's cbrt kernel computes `n = mag * 10^(2*SCALE)` in
//! the `CbrtWide` work integer. At `MAX_SCALE` this can reach roughly
//! `10^(3*MAX_SCALE)` bits, which overflows the obvious next-up width
//! (`Storage * 2`). Each tier's work integer is therefore bumped one
//! further step so the kernel is correct across the full SCALE range.
//! See the D56 shim below for the original investigation; the same
//! arithmetic applies to every tier listed.

use crate::rounding::RoundingMode;
use crate::wide_int::WideStorage;

/// Generic cube-root kernel for the wide-integer family.
///
/// `S` is the storage type backing `Dxx<SCALE>`; `W` is the work
/// integer wide enough to hold `|raw| · 10^(2·scale)` without
/// overflow (one width step *beyond* the obvious next-up — see the
/// module docs). The Newton iteration runs in `W`, and the rounded
/// quotient is cast back to `S`.
#[inline]
#[must_use]
pub(crate) fn cbrt<S, W>(raw: S, scale: u32, mode: RoundingMode) -> S
where
    S: WideStorage,
    W: WideStorage,
{
    if raw == S::ZERO {
        return S::ZERO;
    }
    let zero = W::ZERO;
    let one = W::ONE;
    let three = W::ONE + W::ONE + W::ONE;
    let ten = W::TEN;

    let widened: W = raw.resize_to::<W>();
    let negative = widened < zero;
    let mag = if negative { -widened } else { widened };
    let n: W = mag * ten.pow(2 * scale);

    let sig_bits = W::BITS - n.leading_zeros();
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
    let q_is_odd = (q % (one + one)) != zero;
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
    signed.resize_to::<S>()
}

/// Emits a `cbrt_<tier>(raw, scale, mode) -> Storage` shim around
/// the generic [`cbrt`] kernel.
macro_rules! decl_cbrt_kernel_shim {
    ($name:ident, $Storage:ty, $CbrtWide:ty) => {
        /// Cube-root kernel shim for one wide tier. See module docs.
        #[inline]
        #[must_use]
        pub(crate) fn $name(raw: $Storage, scale: u32, mode: RoundingMode) -> $Storage {
            cbrt::<$Storage, $CbrtWide>(raw, scale, mode)
        }
    };
}

// D56: `MAX_SCALE = 57`, kernel needs `mag * 10^(2*SCALE)` ≈ `10^171`
// at the high end which does not fit `Int384`'s ~`10^115` capacity.
// Use `Int768` so the work integer covers the full SCALE range.
#[cfg(any(feature = "d56", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d56, crate::wide_int::Int192, crate::wide_int::Int768);

// D76: `MAX_SCALE = 76`, work peaks near `10^228`; Int512 (~10^154)
// overflows, bump to Int1024 (~10^308).
#[cfg(any(feature = "d76", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d76, crate::wide_int::Int256, crate::wide_int::Int1024);

// D114: `MAX_SCALE = 115`, work peaks near `10^345`; Int768 (~10^231)
// overflows, bump to Int1536 (~10^462).
#[cfg(any(feature = "d114", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d114, crate::wide_int::Int384, crate::wide_int::Int1536);

// D153: `MAX_SCALE = 153`, work peaks near `10^459`; Int1024 (~10^308)
// overflows, bump to Int2048 (~10^616).
#[cfg(any(feature = "d153", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d153, crate::wide_int::Int512, crate::wide_int::Int2048);

// D230: `MAX_SCALE = 230`, work peaks near `10^690`; Int1536 (~10^462)
// overflows, bump to Int3072 (~10^924).
#[cfg(any(feature = "d230", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d230, crate::wide_int::Int768, crate::wide_int::Int3072);

// D307: `MAX_SCALE = 307`, work peaks near `10^921`; Int2048 (~10^616)
// overflows, bump to Int4096 (~10^1233).
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_cbrt_kernel_shim!(cbrt_d307, crate::wide_int::Int1024, crate::wide_int::Int4096);

// D461: `MAX_SCALE = 461`, work peaks near `10^1383`; Int3072 (~10^924)
// overflows, bump to Int6144 (~10^1849).
#[cfg(any(feature = "d461", feature = "x-wide"))]
decl_cbrt_kernel_shim!(cbrt_d461, crate::wide_int::Int1536, crate::wide_int::Int6144);

// D615: `MAX_SCALE = 615`, work peaks near `10^1845`; Int4096 (~10^1233)
// overflows, bump to Int8192 (~10^2466).
#[cfg(any(feature = "d615", feature = "x-wide"))]
decl_cbrt_kernel_shim!(cbrt_d615, crate::wide_int::Int2048, crate::wide_int::Int8192);

// D923: `MAX_SCALE = 923`, work peaks near `10^2769`; Int6144 (~10^1849)
// overflows, bump to Int12288 (~10^3699).
#[cfg(any(feature = "d923", feature = "xx-wide"))]
decl_cbrt_kernel_shim!(cbrt_d923, crate::wide_int::Int3072, crate::wide_int::Int12288);

// D1231: `MAX_SCALE = 1231`, work peaks near `10^3693`; Int8192 (~10^2466)
// overflows, bump to Int16384 (~10^4932).
#[cfg(any(feature = "d1231", feature = "xx-wide"))]
decl_cbrt_kernel_shim!(cbrt_d1231, crate::wide_int::Int4096, crate::wide_int::Int16384);
