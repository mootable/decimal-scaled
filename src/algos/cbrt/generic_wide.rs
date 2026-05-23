//! Generic wide-integer cube-root kernel — Newton iteration on
//! `mag · 10^(2·SCALE)` over a wide work integer wide enough to cover
//! it.
//!
//! See `crate::algos::sqrt::sqrt_newton` for the parallel sqrt
//! kernel — same parameterisation shape. The kernel is parameterised
//! over `(Storage, CbrtWide)` via the [`crate::int::types::traits::BigInt`]
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
//! See the D57 shim below for the original investigation; the same
//! arithmetic applies to every tier listed.

use crate::support::rounding::RoundingMode;
use crate::int::types::traits::{BigInt, wide_cast};

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
    S: BigInt,
    W: BigInt,
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

    // `n` is built from `mag` (the non-negative magnitude, line above) ×
    // a positive power of ten, so it is always >= 0. `leading_zeros`
    // therefore follows its two's-complement contract on a non-negative
    // value — equivalently `BITS - sig_bits` — and never the negative /
    // MIN branch.
    let sig_bits = <W as BigInt>::BITS - n.leading_zeros();
    // Seed Newton with an f64-cbrt bootstrap. The classical
    // `1 << ceil(sig_bits/3)` seed has 1 bit of accuracy; an
    // f64 `(top_64 as f64).cbrt()` lands within ~10⁻¹⁵ relative
    // error, ~53 bits of accuracy. Newton's cubic-root iteration
    // doubles correct bits per step, so the seed cuts ~5 iterations
    // off the loop and each saved iteration eliminates one
    // multi-limb `n / (x * x)` Knuth division — the dominant cost.
    //
    // Construct the seed by computing `f64::cbrt(top)` where `top`
    // is the most-significant 64 bits of `n`, then shifting back
    // to the correct magnitude. The shift must account for `sig_bits
    // mod 3` (the trailing bit-width that the cbrt operation
    // doesn't see in the truncated `top`).
    let mut x = if crate::policy::float_seed::FLOAT_SEED_AVAILABLE && sig_bits >= 8 {
        // Extract top 64 bits of `n` as an f64-feedable value.
        let top_shift = sig_bits - 64.min(sig_bits);
        let mag_for_top: W = n >> top_shift;
        // `mag_for_top` fits 64 bits by construction.
        let top_u128: u128 = {
            // Read the low 128 bits of mag_for_top — convert via
            // `BigInt::mag_into_u128` (truncates to dst length).
            let mut buf = [0u128; 1];
            mag_for_top.mag_into_u128(&mut buf);
            buf[0]
        };
        let top_f = top_u128 as f64;
        let seed_f64 = crate::policy::float_seed::cbrt_f64(top_f);
        // `n ≈ top * 2^top_shift`. cbrt(n) ≈ seed_f64 * 2^(top_shift/3).
        // top_shift may not be a multiple of 3 — handle the residue
        // by multiplying seed_f64 by `2^(residue / 3)`.
        let third = top_shift / 3;
        let residue = top_shift % 3; // 0, 1, or 2
        let factor: f64 = match residue {
            1 => 1.2599210498948732, // 2^(1/3)
            2 => 1.5874010519681994, // 2^(2/3)
            _ => 1.0,
        };
        let scaled_f64 = seed_f64 * factor;
        // Place an over-estimate seed at bit position `third` in W.
        let truncated = scaled_f64 as u128;
        let frac_nonzero = (truncated as f64) != scaled_f64;
        let seed_int: u128 = truncated
            .saturating_add(if frac_nonzero { 1 } else { 0 })
            .saturating_add(1);
        let mut seed_w: W = wide_cast::<u128, W>(seed_int);
        if third > 0 {
            seed_w = seed_w << third;
        }
        if seed_w == zero { one } else { seed_w }
    } else {
        one << sig_bits.div_ceil(3)
    };
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
    let eight_q_cubed = if q == zero {
        zero
    } else {
        two_q * two_q * two_q
    };
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

use crate::int::types::Int;

// D57: `MAX_SCALE = 57`, kernel needs `mag * 10^(2*SCALE)` ≈ `10^171`
// at the high end which does not fit `Int384`'s ~`10^115` capacity.
// Use `Int768` so the work integer covers the full SCALE range.
#[cfg(any(feature = "d57", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d57, Int<3>, Int<12>);

// D76: `MAX_SCALE = 76`, work peaks near `10^228`; Int512 (~10^154)
// overflows, bump to Int1024 (~10^308).
#[cfg(any(feature = "d76", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d76, Int<4>, Int<16>);

// D115: `MAX_SCALE = 115`, work peaks near `10^345`; Int768 (~10^231)
// overflows, bump to Int1536 (~10^462).
#[cfg(any(feature = "d115", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d115, Int<6>, Int<24>);

// D153: `MAX_SCALE = 153`, work peaks near `10^459`; Int1024 (~10^308)
// overflows, bump to Int2048 (~10^616).
#[cfg(any(feature = "d153", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d153, Int<8>, Int<32>);

// D230: `MAX_SCALE = 230`, work peaks near `10^690`; Int1536 (~10^462)
// overflows, bump to Int3072 (~10^924).
#[cfg(any(feature = "d230", feature = "wide"))]
decl_cbrt_kernel_shim!(cbrt_d230, Int<12>, Int<48>);

// D307: `MAX_SCALE = 307`, work peaks near `10^921`; Int2048 (~10^616)
// overflows, bump to Int4096 (~10^1233).
#[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
decl_cbrt_kernel_shim!(
    cbrt_d307,
    Int<16>,
    Int<64>
);

// D462: `MAX_SCALE = 461`, work peaks near `10^1383`; Int3072 (~10^924)
// overflows, bump to Int6144 (~10^1849).
#[cfg(any(feature = "d462", feature = "x-wide"))]
decl_cbrt_kernel_shim!(
    cbrt_d462,
    Int<24>,
    Int<96>
);

// D616: `MAX_SCALE = 615`, work peaks near `10^1845`; Int4096 (~10^1233)
// overflows, bump to Int8192 (~10^2466).
#[cfg(any(feature = "d616", feature = "x-wide"))]
decl_cbrt_kernel_shim!(
    cbrt_d616,
    Int<32>,
    Int<128>
);

// D924: `MAX_SCALE = 923`, work peaks near `10^2769`; Int6144 (~10^1849)
// overflows, bump to Int12288 (~10^3699).
#[cfg(any(feature = "d924", feature = "xx-wide"))]
decl_cbrt_kernel_shim!(
    cbrt_d924,
    Int<48>,
    Int<192>
);

// D1232: `MAX_SCALE = 1231`, work peaks near `10^3693`; Int8192 (~10^2466)
// overflows, bump to Int16384 (~10^4932).
#[cfg(any(feature = "d1232", feature = "xx-wide"))]
decl_cbrt_kernel_shim!(
    cbrt_d1232,
    Int<64>,
    Int<256>
);
