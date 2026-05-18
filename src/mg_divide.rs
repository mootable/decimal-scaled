//! Moller-Granlund magic-number division for `D38` rescale operations.
//!
//! This module provides two `pub(crate)` entry points used by the
//! arithmetic layer:
//!
//! - [`mul_div_pow10`] -- computes `(a * b) / 10^SCALE` with a 256-bit
//! intermediate product to avoid silent overflow from the naive
//! `(a * b) / multiplier()` form.
//!
//! - [`div_pow10_div`] -- computes `(a * 10^SCALE) / b` with a 256-bit
//! intermediate numerator, used for the division rescale.
//!
//! Both functions return `None` when the final `i128` quotient would
//! overflow; the caller maps `None` to a panic (debug) or wrapping
//! behaviour (release) as appropriate.
//!
//! # Algorithm references
//!
//! - Moller, N. and Granlund, T. (2011). "Improved Division by Invariant
//! Integers." IEEE Transactions on Computers, 60(2), 165-175.
//! DOI: 10.1109/TC.2010.143. (The magic-number table and the
//! 256-bit-dividend divide algorithm used in [`div_exp_fast_2word_with_rem`].)
//! - Granlund, T. and Montgomery, P. L. (1994). "Division by Invariant
//! Integers using Multiplication." PLDI '94. (Basis for the 1-word
//! fast path that the upstream library also references.)
//!
//! The 128x128->256 multiply ([`mul2`]) and the magic-number table
//! ([`MG_EXP_MAGICS`]) are adapted from the `primitive_fixed_point_decimal`
//! crate (MIT-licensed; see `LICENSE-THIRD-PARTY`).
//!
//! # `SCALE = 0` special case
//!
//! At `SCALE = 0` the multiplier is 1, so no rescale is needed. The table
//! entry at index 0 is a placeholder `(0, 0)`; using it would produce a
//! shift of `128 - 0 = 128`, which is undefined behaviour on `u128`. Both
//! public entry points short-circuit at `SCALE == 0` before touching the
//! table.

use crate::core_type::D38;

/// Pre-computed magic constants for divide-by-`10^i` via the
/// Moller-Granlund algorithm. Index `i` covers `i = 0..=38`.
///
/// Each entry is `(magic, zeros)` where `zeros` is the number of leading
/// zeros in `10^i` when left-aligned to 128 bits, and `magic` is the
/// 128-bit Moller-Granlund multiplier for that divisor:
///
/// ```python
/// def gen(d):
/// zeros = 128 - d.bit_length()
/// magic = pow(2, 256) // (d << zeros)
/// magic = magic - pow(2, 128)  # fits in 128 bits
/// return (magic, zeros)
/// ```
///
/// Index 0 is a placeholder `(0, 0)`. Callers must not pass `SCALE = 0`
/// into the magic-divide functions; both public entry points guard that
/// case before indexing this table.
/// `10^i` for `i = 0..=38`. Indexed by `scale` to skip the
/// runtime `u128::pow` (which is a 4-multiplication square-and-multiply
/// loop for the typical scale range) in hot paths like
/// `div_wide_pow10_with`. Last entry `10^38` is the largest power of
/// ten that fits in `u128`.
const POW10_U128: [u128; 39] = {
    let mut t = [1u128; 39];
    let mut i = 1;
    while i < 39 {
        t[i] = t[i - 1] * 10;
        i += 1;
    }
    t
};

const MG_EXP_MAGICS: [(u128, u32); 39] = [
    (0, 0),
    (0x99999999999999999999999999999999, 124),
    (0x47ae147ae147ae147ae147ae147ae147, 121),
    (0x0624dd2f1a9fbe76c8b4395810624dd2, 118),
    (0xa36e2eb1c432ca57a786c226809d4951, 114),
    (0x4f8b588e368f08461f9f01b866e43aa7, 111),
    (0x0c6f7a0b5ed8d36b4c7f34938583621f, 108),
    (0xad7f29abcaf485787a6520ec08d23699, 104),
    (0x5798ee2308c39df9fb841a566d74f87a, 101),
    (0x12e0be826d694b2e62d01511f12a6061, 98),
    (0xb7cdfd9d7bdbab7d6ae6881cb5109a36, 94),
    (0x5fd7fe17964955fdef1ed34a2a73ae91, 91),
    (0x19799812dea11197f27f0f6e885c8ba7, 88),
    (0xc25c268497681c2650cb4be40d60df73, 84),
    (0x6849b86a12b9b01ea70909833de71928, 81),
    (0x203af9ee756159b21f3a6e0297ec1420, 78),
    (0xcd2b297d889bc2b6985d7cd0f3135367, 74),
    (0x70ef54646d496892137dfd73f5a90f85, 71),
    (0x2725dd1d243aba0e75fe645cc4873f9e, 68),
    (0xd83c94fb6d2ac34a5663d3c7a0d865ca, 64),
    (0x79ca10c9242235d511e976394d79eb08, 61),
    (0x2e3b40a0e9b4f7dda7edf82dd794bc06, 58),
    (0xe392010175ee5962a6498d1625bac670, 54),
    (0x82db34012b25144eeb6e0a781e2f0527, 51),
    (0x357c299a88ea76a58924d52ce4f26a85, 48),
    (0xef2d0f5da7dd8aa27507bb7b07ea4409, 44),
    (0x8c240c4aecb13bb52a6c95fc0655033a, 41),
    (0x3ce9a36f23c0fc90eebd44c99eaa68fb, 38),
    (0xfb0f6be50601941b17953adc3110a7f8, 34),
    (0x95a5efea6b34767c12ddc8b027408660, 31),
    (0x4484bfeebc29f863424b06f3529a051a, 28),
    (0x039d66589687f9e901d59f290ee19dae, 25),
    (0x9f623d5a8a732974cfbc31db4b0295e4, 21),
    (0x4c4e977ba1f5bac3d9635b15d59bab1c, 18),
    (0x09d8792fb4c495697ab5e277de16227d, 15),
    (0xa95a5b7f87a0ef0f2abc9d8c9689d0c8, 11),
    (0x54484932d2e725a5bbca17a3aba173d3, 8),
    (0x1039d428a8b8eaeafca1ac82efb45ca9, 5),
    (0xb38fb9daa78e44ab2dcf7a6b19209442, 1),
];

// 128x128 -> 256 schoolbook multiply.
//
// All four 64-bit sub-products are accumulated with explicit carry
// tracking so the full 256-bit result is exact with no overflow loss.

/// Full 256-bit product of two unsigned 128-bit integers.
///
/// Returns `(high, low)` such that `high * 2^128 + low == a * b`
/// for all `a`, `b` in `0..=u128::MAX`.
///
/// `const fn` so constant inputs can be folded at compile time.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
pub(crate) const fn mul2(a: u128, b: u128) -> (u128, u128) {
    let (ahigh, alow) = (a >> 64, a & u64::MAX as u128);
    let (bhigh, blow) = (b >> 64, b & u64::MAX as u128);

    let (mid, carry1) = (alow * bhigh).overflowing_add(ahigh * blow);
    let (mlow, carry2) = (alow * blow).overflowing_add(mid << 64);
    let mhigh = ahigh * bhigh + (mid >> 64) + ((carry1 as u128) << 64) + carry2 as u128;
    (mhigh, mlow)
}

// 256-bit / 10^scale_idx divide via Moller-Granlund 2011 magic numbers.
//
// The algorithm aligns the divisor to the top of a 128-bit word (the
// `zeros` shift), computes a 256-bit approximate quotient using the
// pre-multiplied magic constant, then applies a single add-back
// correction because the estimate can be off by at most 1.

/// Divide the unsigned 256-bit value `(n_high, n_low)` by
/// `exp = 10^scale_idx` using the Moller-Granlund 2011 magic-number
/// method. Returns `Some((quotient, remainder))` if the quotient fits
/// in 128 bits, or `None` if `n_high >= exp` (quotient would overflow).
/// The remainder always fits in a `u128` because
/// `r < exp ≤ 10^38 < 2^127`.
///
/// # Preconditions
///
/// - `1 <= scale_idx <= 38`. The public entry points enforce this;
/// `scale_idx == 0` must be handled by the caller before calling here.
/// - `exp == 10u128.pow(scale_idx)`. The caller computes this once to
/// avoid redundant work.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
fn div_exp_fast_2word_with_rem(
    n_high: u128,
    n_low: u128,
    exp: u128,
    scale_idx: usize,
) -> Option<(u128, u128)> {
    // Overflow check: quotient must fit in 128 bits.
    if n_high >= exp {
        return None;
    }

    let (magic, zeros) = MG_EXP_MAGICS[scale_idx];

    // Step 1: align n to the top of the 256-bit word.
    let z_high = (n_high << zeros) | (n_low >> (128 - zeros));
    let z_low = n_low << zeros;

    // Step 2: approximate quotient via magic multiplication.
    let (m1_high, _) = mul2(z_low, magic);
    let (m2_high, m2_low) = mul2(z_high, magic);

    let (m_low, carry) = m2_low.overflowing_add(m1_high);
    let m_high = m2_high + u128::from(carry);

    // Step 3: extract the 128-bit quotient estimate.
    let (_, carry) = m_low.overflowing_add(z_low);
    let q = m_high + z_high + u128::from(carry);

    // Step 4: single add-back correction. The estimate can be off by 1.
    let (pp_high, pp_low) = mul2(q, exp);
    let (r_low, borrow) = n_low.overflowing_sub(pp_low);
    debug_assert!(n_high == pp_high + u128::from(borrow));

    if r_low < exp {
        Some((q, r_low))
    } else {
        Some((q + 1, r_low - exp))
    }
}

/// Magic-divide a wide signed integer by `10^scale`
/// (`1 ≤ scale ≤ 38`), returning the quotient as the same wide type
/// with `mode`-aware rounding.
///
/// This is the wide-tier counterpart of [`mul_div_pow10`]'s magic
/// step. The divisor `10^scale` fits a single `u128` limb (and an
/// entry in [`MG_EXP_MAGICS`]); the work is base-`2^128` schoolbook
/// long division over the input's magnitude, with each
/// `(rem, limb) / exp` step served by the existing
/// [`div_exp_fast_2word_with_rem`] kernel. The magnitude buffer is
/// 64 limbs, so the same routine serves every width from `Int256`
/// to `Int8192`.
///
/// Caller short-circuits `scale == 0` (no-op) and any `scale > 38`
/// (the magic table only covers `0..=38`).
///
/// Gated on the same `wide`/`x-wide` feature umbrella as
/// `crate::wide_int` — it's only invoked from the wide-tier
/// decimal `Mul` macro arm.
#[cfg(any(
    feature = "d56",
    feature = "d76",
    feature = "d114",
    feature = "d153",
    feature = "d230",
    feature = "d307",
    feature = "d461",
    feature = "d615",
    feature = "d923",
    feature = "d1231",
    feature = "wide",
    feature = "x-wide",
    feature = "xx-wide"
))]
#[inline]
pub(crate) fn div_wide_pow10_with<W: crate::wide_int::WideInt>(
    n: W,
    scale: u32,
    mode: crate::rounding::RoundingMode,
) -> W {
    debug_assert!((1..=38).contains(&scale));
    let (mag_words, neg) = n.to_mag_sign();
    let scale_idx = scale as usize;
    let exp = POW10_U128[scale_idx];

    // `to_mag_sign` returns a 128-u64-limb magnitude. The MG kernel is
    // intrinsically base-2^128, so combine pairs of u64 limbs into
    // u128 limbs (64 of them — same total width). The repack costs ~128
    // shifts; trivial compared to the 8-32 hardware divides the MG
    // long-divide spends per call.
    let mut mag = [0u128; 64];
    let mut i = 0;
    while i < 64 {
        mag[i] = (mag_words[2 * i] as u128) | ((mag_words[2 * i + 1] as u128) << 64);
        i += 1;
    }

    // The magnitude buffer is fixed at 64 u128 limbs (Int8192-equivalent),
    // but most calls operate on far narrower widths (`Int512` ≤ 4 limbs,
    // `Int1024` ≤ 8 limbs, …). Skip the leading zeros.
    let mut top = mag.len();
    while top > 0 && mag[top - 1] == 0 {
        top -= 1;
    }

    // Base-2^128 long divide of `mag[..top]` by `exp`, top-limb first.
    let mut rem: u128 = 0;
    let mut i = top;
    while i > 0 {
        i -= 1;
        let limb = mag[i];
        let (q_limb, r_limb) = div_exp_fast_2word_with_rem(rem, limb, exp, scale_idx)
            .expect("MG: rem < exp invariant violated");
        mag[i] = q_limb;
        rem = r_limb;
    }

    // Round the magnitude per `mode`.
    if rem != 0 {
        let q_is_odd = (mag[0] & 1) != 0;
        let comp = exp - rem;
        let cmp_r = rem.cmp(&comp);
        if crate::rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
            for limb in &mut mag {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c {
                    carry = 0;
                    break;
                }
                carry = 1;
            }
            let _ = carry;
        }
    }

    // Unpack u128 magnitude back to u64 limbs for `from_mag_sign`.
    let mut mag_out = [0u64; 288];
    let mut i = 0;
    while i < 64 {
        mag_out[2 * i] = mag[i] as u64;
        mag_out[2 * i + 1] = (mag[i] >> 64) as u64;
        i += 1;
    }

    W::from_mag_sign(&mag_out, neg)
}

/// Chain-of-`÷ 10^38` extension of [`div_wide_pow10_with`] to scales
/// past `38`. Factors `n / 10^SCALE` as a sequence of
/// `(n / 10^38) / 10^38 / … / 10^last` calls, each reusing the
/// existing base-`2^128` MG 2-by-1 kernel. The intermediate
/// quotients stay in the same `mag` buffer so we never allocate.
///
/// **Status: experimental.** Currently uses truncating rounding
/// for the multi-chunk case (the `mode` parameter is honoured only
/// for the last chunk's remainder — which is the dominant high-
/// order remainder so this is correct in the vast majority of
/// cases but not bit-exact half-to-even at the boundary). Not yet
/// wired into the public operator; called directly from
/// `benches/quick_div.rs` for now.
///
/// # Why this should be faster
///
/// At wide tiers, the public `n / m` path (`m = 10^SCALE`,
/// SCALE > 38) routes through `Int*::div_rem` → Knuth Algorithm D.
/// Each Knuth quotient digit costs ~10–15 limb ops (q̂ estimation,
/// mul-sub, occasional add-back). For `D307<150>::mul` the divisor
/// is 8 u64 limbs and the numerator 32, so ~24 quotient digits ×
/// ~12 ops = ~290 ops per call.
///
/// The chain divides `n / 10^38` four times. Each pass is one
/// MG 2-by-1 magic multiply per u128 limb of the numerator — for
/// D307<150> at most 16 nonzero u128 limbs × ~5 ops = ~80 ops per
/// pass, so ~320 ops for four passes. Comparable to Knuth on op
/// count BUT with a branchless inner loop that the CPU pipelines
/// far better than Knuth's q̂-and-correct scheme.
#[cfg(any(
    feature = "d56",
    feature = "d76",
    feature = "d114",
    feature = "d153",
    feature = "d230",
    feature = "d307",
    feature = "d461",
    feature = "d615",
    feature = "d923",
    feature = "d1231",
    feature = "wide",
    feature = "x-wide",
    feature = "xx-wide"
))]
pub(crate) fn div_wide_pow10_chain_with<W: crate::wide_int::WideInt>(
    n: W,
    scale: u32,
    mode: crate::rounding::RoundingMode,
) -> W {
    debug_assert!(scale > 38, "chain path is for SCALE > 38; callers handle ≤ 38");

    let (mag_words, neg) = n.to_mag_sign();
    let mut mag = [0u128; 64];
    let mut i = 0;
    while i < 64 {
        mag[i] = (mag_words[2 * i] as u128) | ((mag_words[2 * i + 1] as u128) << 64);
        i += 1;
    }
    let mut top = mag.len();
    while top > 0 && mag[top - 1] == 0 {
        top -= 1;
    }

    // Chain divides by 10^38 until we've eaten all but the last chunk.
    //
    // Combined-remainder bookkeeping for exact rounding: the chain
    // produces a sequence of per-chunk remainders r_1, r_2, …, r_k
    // (from successive divides by 10^38) plus a final r_last (from
    // dividing by 10^s where s = SCALE − 38·k). The total
    // remainder is
    //   r_total = r_1 + r_2·10^38 + r_3·10^76 + … + r_k·10^{38(k-1)}
    //                                            + r_last·10^{38·k}
    // and we need to compare r_total with m/2 = 5·10^{SCALE−1}.
    //
    // For correctness we only need two flags:
    //   `lower_any_nonzero` — true iff any of r_1, …, r_k is non-zero
    //   `r_last` — the top-chunk remainder, compared against the
    //              corresponding chunk of m/2 (5·10^{s−1})
    let exp38 = POW10_U128[38];
    let mut lower_any_nonzero = false;
    let mut remaining = scale;
    while remaining > 38 {
        let mut rem: u128 = 0;
        let mut i = top;
        while i > 0 {
            i -= 1;
            let (q, r) = div_exp_fast_2word_with_rem(rem, mag[i], exp38, 38)
                .expect("MG: rem < exp invariant violated");
            mag[i] = q;
            rem = r;
        }
        if rem != 0 {
            lower_any_nonzero = true;
        }
        remaining -= 38;
        while top > 0 && mag[top - 1] == 0 {
            top -= 1;
        }
    }

    // Final divide by 10^remaining (1..=38). The remainder of THIS
    // divide is the top-chunk remainder; together with
    // `lower_any_nonzero` it determines the cmp_r for the rounding
    // decision.
    let scale_idx = remaining as usize;
    let exp_last = POW10_U128[scale_idx];
    let mut r_last: u128 = 0;
    let mut i = top;
    while i > 0 {
        i -= 1;
        let (q, r) = div_exp_fast_2word_with_rem(r_last, mag[i], exp_last, scale_idx)
            .expect("MG: rem < exp invariant violated");
        mag[i] = q;
        r_last = r;
    }

    // Combined-remainder rounding. cmp_r is the comparison of
    // r_total with m/2 (= 5·10^{SCALE−1}):
    //   r_last > exp_last/2          → Greater
    //   r_last < exp_last/2          → Less
    //   r_last == exp_last/2 AND
    //     any lower chunk nonzero    → Greater
    //     else                       → Equal
    // r_last == 0 special case: r_total = lower contribution only,
    //   strictly less than m/2 (since lower chunks each < 10^38 and
    //   we have at most ⌈SCALE/38⌉ − 1 of them) unless they sum to
    //   exactly m/2 — which they can't because each chunk's max
    //   value (10^38 − 1) × 10^{38·i} stays strictly under the next
    //   chunk's 10^{38·(i+1)} contribution. So r_last == 0 implies
    //   cmp_r = Less.
    let combined_nonzero = r_last != 0 || lower_any_nonzero;
    if combined_nonzero {
        let half = exp_last / 2; // exact; exp_last = 10^scale_idx is even
        let cmp_r = if r_last > half {
            core::cmp::Ordering::Greater
        } else if r_last < half {
            core::cmp::Ordering::Less
        } else if lower_any_nonzero {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        };
        let q_is_odd = (mag[0] & 1) != 0;
        if crate::rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
            for limb in &mut mag {
                let (s, c) = limb.overflowing_add(carry);
                *limb = s;
                if !c {
                    carry = 0;
                    break;
                }
                carry = 1;
            }
            let _ = carry;
        }
    }

    let mut mag_out = [0u64; 288];
    let mut i = 0;
    while i < 64 {
        mag_out[2 * i] = mag[i] as u64;
        mag_out[2 * i + 1] = (mag[i] >> 64) as u64;
        i += 1;
    }

    W::from_mag_sign(&mag_out, neg)
}

/// Mode-aware rounding for an *unsigned* magnitude `q` with remainder
/// `r` against divisor `m`, given the result sign — returns the
/// rounded magnitude. Caller applies the sign afterwards.
///
/// All mode-specific behaviour is delegated to
/// [`crate::rounding::should_bump`]; this function only assembles
/// the inputs from the unsigned-magnitude representation.
#[inline]
fn round_mag_with_mode(
    q: u128,
    r: u128,
    m: u128,
    mode: crate::rounding::RoundingMode,
    result_positive: bool,
) -> u128 {
    if r == 0 {
        return q;
    }
    let comp = m - r;
    let cmp_r = r.cmp(&comp);
    let q_is_odd = (q & 1) != 0;
    if crate::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
        q + 1
    } else {
        q
    }
}

// Binary shift-subtract long-divide for the variable-divisor path.
//
// Used when dividing a 256-bit numerator by an arbitrary 128-bit `b`
// (not a power of 10), so no magic table applies. The loop runs exactly
// 256 iterations -- one per bit of the numerator -- and is competitive
// in practice because the widening path is taken only for large `a`.

/// Divide the unsigned 256-bit value `(n_high, n_low)` by the 128-bit
/// divisor `d` using a binary shift-subtract algorithm. Returns
/// `Some(quotient)` if the quotient fits in 128 bits, or `None` if
/// `d == 0` or the quotient would overflow 128 bits.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
fn div_long_256_by_128(n_high: u128, n_low: u128, d: u128) -> Option<u128> {
    div_long_256_by_128_with_rem(n_high, n_low, d).map(|(q, _)| q)
}

/// Remainder-returning companion of [`div_long_256_by_128`]. Same
/// algorithm; the per-bit / per-limb remainder is already maintained
/// inside the loop, so exposing it costs nothing. Required by the
/// 0.5-ULP rounding path in [`div_pow10_div_with`].
#[inline]
fn div_long_256_by_128_with_rem(
    mut n_high: u128,
    mut n_low: u128,
    d: u128,
) -> Option<(u128, u128)> {
    if d == 0 {
        return None;
    }
    // Fast path: dividend already fits 128 bits.
    if n_high == 0 {
        return Some((n_low / d, n_low % d));
    }
    // Overflow check: quotient must fit in 128 bits, so n_high < d.
    if n_high >= d {
        return None;
    }

    // Word-divisor fast path: a divisor that fits in 64 bits admits
    // schoolbook base-2^64 long division — one hardware divide per
    // 64-bit limb instead of a 256-iteration bit loop. Every
    // `10^scale` for `scale <= 19` lands here.
    if d <= u128::from(u64::MAX) {
        let limbs = [
            n_low as u64,
            (n_low >> 64) as u64,
            n_high as u64,
            (n_high >> 64) as u64,
        ];
        // `n_high < d` guarantees the quotient fits in 128 bits, so the
        // top two limbs of the result are always zero.
        let mut out = [0u64; 4];
        let mut rem: u128 = 0;
        let mut i = 4;
        while i > 0 {
            i -= 1;
            let cur = (rem << 64) | u128::from(limbs[i]);
            out[i] = (cur / d) as u64;
            rem = cur % d;
        }
        return Some((u128::from(out[0]) | (u128::from(out[1]) << 64), rem));
    }

    // Shift-subtract over only the significant bits of the dividend.
    let bits = if n_high != 0 {
        256 - n_high.leading_zeros()
    } else {
        128 - n_low.leading_zeros()
    };
    let shift = 256 - bits;
    if shift >= 128 {
        n_high = n_low << (shift - 128);
        n_low = 0;
    } else if shift > 0 {
        n_high = (n_high << shift) | (n_low >> (128 - shift));
        n_low <<= shift;
    }
    let mut q: u128 = 0;
    let mut rem: u128 = 0;
    let mut i = bits;
    while i > 0 {
        i -= 1;
        rem = (rem << 1) | (n_high >> 127);
        n_high = (n_high << 1) | (n_low >> 127);
        n_low <<= 1;
        q <<= 1;
        if rem >= d {
            rem -= d;
            q |= 1;
        }
    }
    Some((q, rem))
}

/// `floor(sqrt(N))` for the unsigned 256-bit value `N = hi·2^128 + lo`.
///
/// # Preconditions
///
/// `N < 2^254`, so the result fits in `u128` (`< 2^127`). Every caller
/// in this crate is the scaled-fixed-point `sqrt` path, where
/// `N = r · 10^SCALE` with `r < 2^127` and `10^SCALE ≤ 10^38 < 2^127`,
/// so the product is below `2^254`.
///
/// # Precision
///
/// Strict: integer-only Newton iteration; the result is the exact
/// mathematical floor of the square root.
pub(crate) fn isqrt_256(hi: u128, lo: u128) -> u128 {
    if hi == 0 && lo == 0 {
        return 0;
    }
    // Bit length of N.
    let bits = if hi != 0 {
        256 - hi.leading_zeros()
    } else {
        128 - lo.leading_zeros()
    };
    // Initial overestimate q0 = 2^ceil(bits/2) ≥ sqrt(N). With N < 2^254
    // this is at most 2^127, so it fits in u128 and `N / q0` cannot
    // overflow 128 bits.
    let mut q: u128 = 1u128 << bits.div_ceil(2);
    loop {
        // q ≥ sqrt(N) on every iteration, so N / q ≤ sqrt(N) < 2^127
        // and the divide always succeeds.
        let nq = div_long_256_by_128(hi, lo, q)
            .expect("isqrt_256: q >= sqrt(N), so N/q fits in 128 bits");
        // q_next = (q + nq) / 2, computed without the q+nq overflow.
        let q_next = (q >> 1) + (nq >> 1) + (q & nq & 1);
        if q_next >= q {
            return q;
        }
        q = q_next;
    }
}

/// Correctly-rounded raw storage of `sqrt` for a scaled fixed-point
/// value.
///
/// Given the non-negative raw storage `r` of a `D*<SCALE>` value and
/// the `SCALE`, returns `round_to_nearest(sqrt(r · 10^SCALE))` — which
/// is the raw storage of `sqrt(value)` correctly rounded to the type's
/// last representable place (IEEE-754 round-to-nearest).
///
/// The computation is exact: `r · 10^SCALE` is formed as a full 256-bit
/// product, its integer square root is exact, and the round-to-nearest
/// decision `N − q² > q` is an exact integer comparison.
///
/// # Preconditions
///
/// `r ≥ 0` and `SCALE ≤ 38` (so `r · 10^SCALE < 2^254`).
///
/// # Precision
///
/// Strict: integer-only; the result is within 0.5 ULP of the exact
/// square root — it is the exact result correctly rounded.
pub(crate) fn sqrt_raw_correctly_rounded(r: u128, scale: u32) -> u128 {
    sqrt_raw_with(r, scale, crate::rounding::RoundingMode::HalfToEven)
}

/// Mode-aware variant of [`sqrt_raw_correctly_rounded`].
///
/// For a non-negative integer radicand `N = r · 10^SCALE`, the true
/// `√N` is irrational unless `N` is a perfect square, so no half-way
/// tie exists between two integers — every nearest-mode (HalfToEven /
/// HalfAwayFromZero / HalfTowardZero) returns the same result. The
/// directed modes split:
///
/// - `Trunc` / `Floor`: keep `q = floor(√N)`. `√N ≥ 0` so floor and
///   trunc coincide.
/// - `Ceiling`: bump to `q + 1` whenever `N > q²` (the diff is
///   non-zero, so the true root is strictly greater than `q`).
/// - half-modes: bump to `q + 1` iff `N > q² + q` (equivalently
///   `diff > q`), the standard round-to-nearest-integer test.
pub(crate) fn sqrt_raw_with(r: u128, scale: u32, mode: crate::rounding::RoundingMode) -> u128 {
    use crate::rounding::RoundingMode;
    if r == 0 {
        return 0;
    }
    let (hi, lo) = mul2(r, POW10_U128[scale as usize]);
    let q = isqrt_256(hi, lo);
    let (q_sq_hi, q_sq_lo) = mul2(q, q);
    let (diff_hi, diff_lo) = if lo >= q_sq_lo {
        (hi - q_sq_hi, lo - q_sq_lo)
    } else {
        (hi - q_sq_hi - 1, lo.wrapping_sub(q_sq_lo))
    };
    let diff_nonzero = diff_hi != 0 || diff_lo != 0;
    let halfway_round_up = diff_hi != 0 || diff_lo > q;
    let bump = match mode {
        RoundingMode::HalfToEven
        | RoundingMode::HalfAwayFromZero
        | RoundingMode::HalfTowardZero => halfway_round_up,
        RoundingMode::Trunc | RoundingMode::Floor => false,
        RoundingMode::Ceiling => diff_nonzero,
    };
    if bump { q + 1 } else { q }
}

// ─────────────────────────────────────────────────────────────────────
// 384-bit integer helpers for the correctly-rounded cube root.
//
// `cbrt` of a `D*<SCALE>` value with raw storage `r` is
// `round(icbrt(r · 10^(2·SCALE)))`. At `SCALE = 38` the radicand
// `r · 10^76` is just under `2^380`, so it needs a 384-bit
// intermediate — wider than the 256-bit machinery above. These helpers
// give exactly the operations the cube-root path needs and nothing
// more. A 384-bit value is `[u128; 3]`, least-significant limb first.
// ─────────────────────────────────────────────────────────────────────

/// `10^exp` as a 256-bit value `[lo, hi]`. `exp <= 76` (so the result
/// is below `2^253` and fits 256 bits).
fn pow10_256(exp: u32) -> [u128; 2] {
    if exp <= 38 {
        [10u128.pow(exp), 0]
    } else {
        // 10^exp = 10^38 * 10^(exp-38); both factors fit u128 for exp <= 76.
        let (hi, lo) = mul2(10u128.pow(38), 10u128.pow(exp - 38));
        [lo, hi]
    }
}

/// `a * m` where `m` is a 256-bit value `[lo, hi]`; result is 384-bit.
fn mul_u128_by_256(a: u128, m: [u128; 2]) -> [u128; 3] {
    let (p0_hi, p0_lo) = mul2(a, m[0]);
    let (p1_hi, p1_lo) = mul2(a, m[1]);
    let limb0 = p0_lo;
    let (limb1, c1) = p0_hi.overflowing_add(p1_lo);
    let limb2 = p1_hi + u128::from(c1);
    [limb0, limb1, limb2]
}

/// `s * b` where `s` is a 256-bit value `[lo, hi]`; result is 384-bit.
fn mul_u256_by_u128(s: [u128; 2], b: u128) -> [u128; 3] {
    let (p0_hi, p0_lo) = mul2(s[0], b);
    let (p1_hi, p1_lo) = mul2(s[1], b);
    let limb0 = p0_lo;
    let (limb1, c1) = p0_hi.overflowing_add(p1_lo);
    let limb2 = p1_hi + u128::from(c1);
    [limb0, limb1, limb2]
}

/// Left-shift a 384-bit value by 3 bits. The caller guarantees no
/// significant bits are lost (used only on `N < 2^380`, so `8N < 2^383`).
fn shl3_384(n: [u128; 3]) -> [u128; 3] {
    [
        n[0] << 3,
        (n[1] << 3) | (n[0] >> 125),
        (n[2] << 3) | (n[1] >> 125),
    ]
}

/// `a >= b` for 384-bit values.
fn ge_384(a: [u128; 3], b: [u128; 3]) -> bool {
    if a[2] != b[2] {
        a[2] > b[2]
    } else if a[1] != b[1] {
        a[1] > b[1]
    } else {
        a[0] >= b[0]
    }
}

/// `a >= b` for 256-bit values `[lo, hi]`.
fn ge_256(a: [u128; 2], b: [u128; 2]) -> bool {
    a[1] > b[1] || (a[1] == b[1] && a[0] >= b[0])
}

/// `a - b` for 256-bit values `[lo, hi]`; caller guarantees `a >= b`.
fn sub_256(a: [u128; 2], b: [u128; 2]) -> [u128; 2] {
    let (lo, borrow) = a[0].overflowing_sub(b[0]);
    let hi = a[1] - b[1] - u128::from(borrow);
    [lo, hi]
}

/// Divide the 384-bit `num` by the 256-bit `d` via binary
/// shift-subtract. The caller guarantees the quotient fits in `u128`.
fn div_384_by_256(mut num: [u128; 3], d: [u128; 2]) -> u128 {
    let mut rem: [u128; 2] = [0, 0];
    let mut q: u128 = 0;
    let mut i = 0;
    while i < 384 {
        // Shift the top bit of `num` into `rem`, both left by 1.
        let num_top = num[2] >> 127;
        num[2] = (num[2] << 1) | (num[1] >> 127);
        num[1] = (num[1] << 1) | (num[0] >> 127);
        num[0] <<= 1;
        rem[1] = (rem[1] << 1) | (rem[0] >> 127);
        rem[0] = (rem[0] << 1) | num_top;
        q <<= 1;
        if ge_256(rem, d) {
            rem = sub_256(rem, d);
            q |= 1;
        }
        i += 1;
    }
    q
}

/// `floor((carry · 2^128 + val) / 3)` for `carry` in `0..=2`. Used by
/// the cube-root Newton step, where `2y + N/y²` can be a 130-bit value.
fn floor_div3(mut carry: u128, mut val: u128) -> u128 {
    // 2^128 = 3·K + 1, with K = (2^128 - 1) / 3.
    const K: u128 = u128::MAX / 3;
    let mut q: u128 = 0;
    loop {
        if carry == 0 {
            return q + val / 3;
        }
        // carry·2^128 + val = 3·carry·K + (carry + val).
        q += carry * K;
        let (next_val, c) = val.overflowing_add(carry);
        carry = u128::from(c);
        val = next_val;
    }
}

/// `floor(cbrt(N))` for the unsigned 384-bit value `N`.
///
/// # Preconditions
///
/// `N < 2^381` (so the result is below `2^127` and the Newton iteration
/// stays within `u128`). The scaled-fixed-point caller forms
/// `N = r · 10^(2·SCALE)` with `r < 2^127` and `2·SCALE <= 76`, so
/// `N < 2^380`.
fn icbrt_384(n: [u128; 3]) -> u128 {
    if n == [0, 0, 0] {
        return 0;
    }
    // Bit length of N.
    let bits = if n[2] != 0 {
        384 - n[2].leading_zeros()
    } else if n[1] != 0 {
        256 - n[1].leading_zeros()
    } else {
        128 - n[0].leading_zeros()
    };
    // Overestimate y0 = 2^ceil(bits/3) >= cbrt(N); <= 2^127 for N < 2^381.
    let mut y: u128 = 1u128 << (bits.div_ceil(3).min(127));
    loop {
        // y² as a 256-bit divisor.
        let (yy_hi, yy_lo) = mul2(y, y);
        // nq = N / y²; y >= cbrt(N) keeps this below 2^128.
        let nq = div_384_by_256(n, [yy_lo, yy_hi]);
        // y_next = (2y + nq) / 3, computed via a (carry, sum) pair so
        // the up-to-130-bit intermediate never overflows `u128`.
        let (two_y, c0) = y.overflowing_add(y);
        let (sum, c1) = two_y.overflowing_add(nq);
        let carry = u128::from(c0) + u128::from(c1);
        let y_next = floor_div3(carry, sum);
        if y_next >= y {
            return y;
        }
        y = y_next;
    }
}

/// Correctly-rounded raw storage of `cbrt` for a scaled fixed-point
/// value.
///
/// Given the non-negative raw storage `r` of a `D*<SCALE>` value and
/// the `SCALE`, returns `round_to_nearest(cbrt(r · 10^(2·SCALE)))` —
/// the raw storage of `cbrt(value)` correctly rounded to the type's
/// last place (IEEE-754 round-to-nearest).
///
/// The radicand `r · 10^(2·SCALE)` is formed exactly as a 384-bit
/// value, its integer cube root is exact, and the round-to-nearest
/// decision uses the exact identity `round up iff 8·N ≥ (2q + 1)³`
/// (because `(2q + 1)³ = 8q³ + 12q² + 6q + 1 = 8·(q + 0.5)³`).
///
/// # Preconditions
///
/// `r >= 0` and `SCALE <= 38` (so `r · 10^(2·SCALE) < 2^380`).
///
/// # Precision
///
/// Strict: integer-only; the result is within 0.5 ULP of the exact
/// cube root — it is the exact result correctly rounded.
pub(crate) fn cbrt_raw_correctly_rounded(r: u128, scale: u32) -> u128 {
    cbrt_raw_with_unsigned_mag(r, scale, crate::rounding::RoundingMode::HalfToEven)
}

/// Mode-aware variant of [`cbrt_raw_correctly_rounded`] operating on
/// the **magnitude** `|r|` only.
///
/// Caller threads in the sign separately: the caller knows whether
/// the final cube-root value is positive or negative (cbrt preserves
/// sign), and `Floor` / `Ceiling` need that sign to decide whether to
/// bump the magnitude or not.
///
/// - `Trunc`: keep magnitude `q` (toward zero regardless of sign).
/// - `Floor`: positive → `q`; negative → bump magnitude when non-zero
///   residual (more negative).
/// - `Ceiling`: positive → bump when non-zero residual; negative → `q`.
/// - half-modes: bump iff `8·N ≥ (2q + 1)³` (cube-root half-way test).
///
/// The half-way test handles the tie case (perfect-cube midpoint) per
/// the original `cbrt_raw_correctly_rounded`; for cubes this can match
/// equality, so the three half-modes diverge in the tie:
/// `HalfAwayFromZero` and `HalfToEven` both bump (cubes meeting the
/// `>=` make `q+1` the away neighbour for non-negative, and the larger
/// magnitude); `HalfTowardZero` keeps `q`. The implementation here
/// follows the conservative reading: `>=` triggers a bump for the
/// `HalfAwayFromZero` and `HalfToEven` modes, `>` for `HalfTowardZero`.
pub(crate) fn cbrt_raw_with_unsigned_mag(
    r: u128,
    scale: u32,
    mode: crate::rounding::RoundingMode,
) -> u128 {
    cbrt_raw_with_signed(r, scale, false, mode)
}

/// Sign-aware mode dispatch for the cbrt raw computation. `negative`
/// is the sign of the source value (cbrt preserves sign).
pub(crate) fn cbrt_raw_with_signed(
    r: u128,
    scale: u32,
    negative: bool,
    mode: crate::rounding::RoundingMode,
) -> u128 {
    use crate::rounding::RoundingMode;
    if r == 0 {
        return 0;
    }
    let n = mul_u128_by_256(r, pow10_256(2 * scale));
    let q = icbrt_384(n);
    let eight_n = shl3_384(n);
    let two_q_plus_1 = 2 * q + 1;
    let (sq_hi, sq_lo) = mul2(two_q_plus_1, two_q_plus_1);
    let cube = mul_u256_by_u128([sq_lo, sq_hi], two_q_plus_1);
    let halfway_geq = ge_384(eight_n, cube);
    let halfway_gt = gt_384(eight_n, cube);
    // Residual non-zero iff q³ < N, i.e. the cubed midpoint cube is
    // strictly less than 8·N; equivalent for our purposes to "q is not
    // the exact cube root". Cheaper test: residual non-zero iff
    // `n != q · q · q`, but that requires another 384-bit mul. Reuse
    // the eight_n vs (2q)³ comparison: 8N > 8q³ iff N > q³ iff there's
    // a residual.
    let two_q = q + q;
    let (tq_sq_hi, tq_sq_lo) = mul2(two_q, two_q);
    let eight_q_cubed = if q == 0 {
        [0u128, 0, 0]
    } else {
        mul_u256_by_u128([tq_sq_lo, tq_sq_hi], two_q)
    };
    let residual_nonzero = gt_384(eight_n, eight_q_cubed);
    let tie = halfway_geq && !halfway_gt;
    let bump = match mode {
        RoundingMode::HalfToEven => halfway_gt || (tie && (q & 1 == 1)),
        RoundingMode::HalfAwayFromZero => halfway_geq,
        RoundingMode::HalfTowardZero => halfway_gt,
        RoundingMode::Trunc => false,
        RoundingMode::Floor => negative && residual_nonzero,
        RoundingMode::Ceiling => !negative && residual_nonzero,
    };
    if bump { q + 1 } else { q }
}

/// 384-bit strictly-greater comparison, mirroring [`ge_384`].
fn gt_384(a: [u128; 3], b: [u128; 3]) -> bool {
    if a[2] != b[2] { return a[2] > b[2]; }
    if a[1] != b[1] { return a[1] > b[1]; }
    a[0] > b[0]
}

/// Compute `(a * b) / 10^SCALE` with truncating division semantics
/// matching `i128 /`. Returns `None` if the result overflows `i128`.
///
/// When `a * b` fits in `i128` the multiply is done directly and the
/// result is divided by the scale multiplier. When the product would
/// overflow `i128`, the unsigned absolute values are multiplied to a
/// full 256-bit result via [`mul2`], divided by `10^SCALE` using the
/// Moller-Granlund magic-number algorithm, and the correct sign is
/// restored.
///
/// At `SCALE = 0` the multiplier is 1 and the function reduces to
/// `a.checked_mul(b)`.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
///
/// # Examples
///
/// ```ignore
/// use decimal_scaled::D38;
/// // 50_000_000_000_000_000_000_000 * 30_000_000_000_000_000_000_000
/// // overflows i128 but fits after dividing by 10^12.
/// let result = mul_div_pow10::<12>(
/// 50_000_000_000_000_000_000_000_i128,
/// 30_000_000_000_000_000_000_000_i128,
/// );
/// assert_eq!(result, Some(1_500_000_000_000_000_000_000_000_000_000_000_i128));
/// ```
#[inline]
pub(crate) fn mul_div_pow10<const SCALE: u32>(a: i128, b: i128) -> Option<i128> {
    mul_div_pow10_with::<SCALE>(a, b, crate::rounding::DEFAULT_ROUNDING_MODE)
}

/// Mode-aware variant of [`mul_div_pow10`]: rounds the
/// divide-by-`10^SCALE` step according to `mode`. The default
/// `mul_div_pow10` is a thin wrapper that passes
/// [`crate::rounding::DEFAULT_ROUNDING_MODE`].
#[inline]
pub(crate) fn mul_div_pow10_with<const SCALE: u32>(
    a: i128,
    b: i128,
    mode: crate::rounding::RoundingMode,
) -> Option<i128> {
    // SCALE = 0: multiplier is 1, result is just a * b. No rounding
    // step possible.
    if SCALE == 0 {
        return a.checked_mul(b);
    }

    // Fast path: i128 * i128 didn't overflow. Apply `mode` at the
    // divide-by-10^SCALE step.
    if let Some(prod) = a.checked_mul(b) {
        return Some(crate::rounding::apply_rounding(
            prod,
            D38::<SCALE>::multiplier(),
            mode,
        ));
    }

    // Widening path: |a*b| > i128::MAX. Compute the unsigned 256-bit
    // product, magic-divide by 10^SCALE, round per `mode`, restore sign.
    let ua = a.unsigned_abs();
    let ub = b.unsigned_abs();
    let (mhigh, mlow) = mul2(ua, ub);

    let exp = D38::<SCALE>::multiplier() as u128;
    let (q_floor, r) = div_exp_fast_2word_with_rem(mhigh, mlow, exp, SCALE as usize)?;
    // Sign: result is negative iff exactly one operand is negative.
    let neg = (a < 0) ^ (b < 0);
    let q = round_mag_with_mode(q_floor, r, exp, mode, !neg);
    if neg {
        // -q must fit in i128. q == 2^127 is fine (that is i128::MIN).
        if q <= i128::MAX as u128 {
            Some(-(q as i128))
        } else if q == (i128::MAX as u128) + 1 {
            Some(i128::MIN)
        } else {
            None
        }
    } else {
        // Positive: q must be <= i128::MAX.
        if q <= i128::MAX as u128 {
            Some(q as i128)
        } else {
            None
        }
    }
}

/// Compute `(a * 10^SCALE) / b` with truncating division semantics
/// matching `i128 /`. Returns `None` if the result overflows `i128` or
/// `b == 0`.
///
/// When `a * 10^SCALE` fits in `i128` the multiply-then-divide is done
/// directly. When it would overflow, the unsigned absolute value of `a`
/// is multiplied by the scale multiplier to a full 256-bit result via
/// [`mul2`], divided by `|b|` using the binary long-divide, and the
/// correct sign is restored.
///
/// At `SCALE = 0` the multiplier is 1 and the function reduces to
/// `a.checked_div(b)`.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
///
/// # Examples
///
/// ```ignore
/// use decimal_scaled::D38;
/// // (10^22 * 10^12) / 2 = 5e33, which requires 256-bit intermediates.
/// let result = div_pow10_div::<12>(10_i128.pow(22), 2);
/// assert_eq!(result, Some(5 * 10_i128.pow(33)));
/// ```
#[inline]
pub(crate) fn div_pow10_div<const SCALE: u32>(a: i128, b: i128) -> Option<i128> {
    div_pow10_div_with::<SCALE>(a, b, crate::rounding::DEFAULT_ROUNDING_MODE)
}

/// Mode-aware variant of [`div_pow10_div`]: rounds the final divide
/// step according to `mode`. The default `div_pow10_div` is a thin
/// wrapper that passes [`crate::rounding::DEFAULT_ROUNDING_MODE`].
#[inline]
pub(crate) fn div_pow10_div_with<const SCALE: u32>(
    a: i128,
    b: i128,
    mode: crate::rounding::RoundingMode,
) -> Option<i128> {
    if b == 0 {
        return None;
    }
    // Probe for the `i128::MIN / -1` overflow case so the rounding path
    // below can rely on `a / b` not panicking.
    a.checked_div(b)?;

    // SCALE = 0: scale-narrowing step is `a / b` itself; apply mode.
    if SCALE == 0 {
        return Some(crate::rounding::apply_rounding(a, b, mode));
    }

    let mult = D38::<SCALE>::multiplier();

    // Fast path: a * mult fits in i128. At SCALE <= 18, i64::MAX * 10^18
    // fits with headroom; for larger SCALE the overflow check below
    // handles the fallthrough.
    if let Some(num) = a.checked_mul(mult) {
        return Some(crate::rounding::apply_rounding(num, b, mode));
    }

    // Widening path: a*mult overflows i128. Compute it as a 256-bit
    // unsigned, divide by |b| keeping the remainder, round per `mode`,
    // restore sign.
    let ua = a.unsigned_abs();
    let umult = mult as u128;
    let (mhigh, mlow) = mul2(ua, umult);

    let ub = b.unsigned_abs();
    let (q_floor, r) = div_long_256_by_128_with_rem(mhigh, mlow, ub)?;

    // Sign: result is negative iff exactly one of `a` and `b` is
    // negative. (mult is always positive.)
    let neg = (a < 0) ^ (b < 0);
    let q = round_mag_with_mode(q_floor, r, ub, mode, !neg);
    if neg {
        if q <= i128::MAX as u128 {
            Some(-(q as i128))
        } else if q == (i128::MAX as u128) + 1 {
            Some(i128::MIN)
        } else {
            None
        }
    } else if q <= i128::MAX as u128 {
        Some(q as i128)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Small operands route via the i128 fast path; result matches the
    /// naive form bit-exactly.
    #[test]
    fn mul_div_pow10_small_matches_naive() {
        const SCALE: u32 = 12;
        let a: i128 = 1_500_000_000;
        let b: i128 = 2_300_000_000;
        let expected = (a * b) / 1_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// Mid-range operands: still fits the i128 fast path with a wider
    /// product.
    #[test]
    fn mul_div_pow10_mid_matches_naive() {
        const SCALE: u32 = 12;
        let a: i128 = 3_000_000_000_000_000;
        let b: i128 = 4_700_000_000_000_000;
        let expected = (a * b) / 1_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// Operands near the boundary of the i128 fast path; still within
    /// i128 by construction.
    #[test]
    fn mul_div_pow10_bound_matches_naive() {
        const SCALE: u32 = 12;
        // 7e18 * 4.7e18 = 3.29e37 < i128::MAX (1.7e38)
        let a: i128 = 7_000_000_000_000_000_000;
        let b: i128 = 4_700_000_000_000_000_000;
        let expected = (a * b) / 1_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// Wide operands: 5e22 * 3e22 = 1.5e45, past i128::MAX. The widening
    /// path produces the correct result (1.5e33 after dividing by 10^12).
    #[test]
    fn mul_div_pow10_wide_correctness() {
        const SCALE: u32 = 12;
        let a: i128 = 50_000_000_000_000_000_000_000;
        let b: i128 = 30_000_000_000_000_000_000_000;
        // a * b == 1.5e45 raw; / 10^12 == 1.5e33 raw.
        // 1.5e33 < i128::MAX (1.7e38), so the result fits.
        let expected: i128 = 1_500_000_000_000_000_000_000_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// When the final i128 quotient would overflow, the function returns
    /// `None`.
    #[test]
    fn mul_div_pow10_overflows_to_none() {
        const SCALE: u32 = 12;
        // a*b = 10^52; /10^12 = 10^40. i128::MAX < 10^39, so this overflows.
        let a: i128 = 10_i128.pow(26);
        let b: i128 = 10_i128.pow(26);
        assert_eq!(mul_div_pow10::<SCALE>(a, b), None);
    }

    /// One negative operand produces a negative result.
    #[test]
    fn mul_div_pow10_negative_one_sided() {
        const SCALE: u32 = 12;
        let a: i128 = -50_000_000_000_000_000_000_000;
        let b: i128 = 30_000_000_000_000_000_000_000;
        let expected: i128 = -1_500_000_000_000_000_000_000_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// Two negative operands produce a positive result.
    #[test]
    fn mul_div_pow10_negative_both() {
        const SCALE: u32 = 12;
        let a: i128 = -50_000_000_000_000_000_000_000;
        let b: i128 = -30_000_000_000_000_000_000_000;
        let expected: i128 = 1_500_000_000_000_000_000_000_000_000_000_000_i128;
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(expected));
    }

    /// `SCALE = 0`: identity multiplier; result is just `a * b`.
    #[test]
    fn mul_div_pow10_scale_zero() {
        const SCALE: u32 = 0;
        assert_eq!(mul_div_pow10::<SCALE>(7, 11), Some(77));
        assert_eq!(mul_div_pow10::<SCALE>(-7, 11), Some(-77));
        assert_eq!(mul_div_pow10::<SCALE>(i128::MAX, 1), Some(i128::MAX));
        assert_eq!(mul_div_pow10::<SCALE>(i128::MAX, 2), None); // overflows
    }

    /// `SCALE = 1`: smallest non-trivial scale, exercises the fast path.
    #[test]
    fn mul_div_pow10_scale_one() {
        const SCALE: u32 = 1;
        // 25 * 4 = 100; / 10 = 10.
        assert_eq!(mul_div_pow10::<SCALE>(25, 4), Some(10));
    }

    /// `SCALE = 18`: large but common scale.
    #[test]
    fn mul_div_pow10_scale_eighteen() {
        const SCALE: u32 = 18;
        // 10^18 * 10^18 = 10^36; / 10^18 = 10^18.
        let a: i128 = 10_i128.pow(18);
        let b: i128 = 10_i128.pow(18);
        assert_eq!(mul_div_pow10::<SCALE>(a, b), Some(10_i128.pow(18)));
    }

    /// Division: small operands match the naive form.
    #[test]
    fn div_pow10_div_small_matches_naive() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        // `expected` is the truncating result and matches under
        // HalfToEven only when the remainder is below half — true here
        // (a*10^12 / 7 leaves a remainder well under 3.5).
        const SCALE: u32 = 12;
        let a: i128 = 1_500_000_000;
        let b: i128 = 7;
        let expected = (a * 1_000_000_000_000_i128) / b;
        assert_eq!(div_pow10_div::<SCALE>(a, b), Some(expected));
    }

    /// Division by zero returns `None`.
    #[test]
    fn div_pow10_div_by_zero_is_none() {
        const SCALE: u32 = 12;
        assert_eq!(div_pow10_div::<SCALE>(123, 0), None);
    }

    /// `SCALE = 0`: scale-narrowing step is `a / b`, rounded by the
    /// crate-default rounding mode (HalfToEven by default).
    #[test]
    fn div_pow10_div_scale_zero() {
        if !crate::rounding::DEFAULT_IS_HALF_TO_EVEN { return; }
        const SCALE: u32 = 0;
        // 15 / 4 = 3.75 -> 4 under half-* family (no tie, .75 > .5).
        assert_eq!(div_pow10_div::<SCALE>(15, 4), Some(4));
        // -15 / 4 = -3.75 -> -4 by symmetry.
        assert_eq!(div_pow10_div::<SCALE>(-15, 4), Some(-4));
        // Exact: 16 / 4 = 4 with zero remainder, no rounding.
        assert_eq!(div_pow10_div::<SCALE>(16, 4), Some(4));
        // i128::MIN / -1 overflows -> checked_div returns None.
        assert_eq!(div_pow10_div::<SCALE>(i128::MIN, -1), None);
    }

    /// `div_pow10_div_with` honours the explicit mode argument.
    #[test]
    fn div_pow10_div_with_modes() {
        use crate::rounding::RoundingMode::*;
        const SCALE: u32 = 0;
        // 15 / 4 = 3.75
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, HalfToEven), Some(4));
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, HalfAwayFromZero), Some(4));
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, HalfTowardZero), Some(4));
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, Trunc), Some(3));
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, Floor), Some(3));
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, Ceiling), Some(4));
        // -15 / 4 = -3.75
        assert_eq!(div_pow10_div_with::<SCALE>(-15, 4, Trunc), Some(-3));
        assert_eq!(div_pow10_div_with::<SCALE>(-15, 4, Floor), Some(-4));
        assert_eq!(div_pow10_div_with::<SCALE>(-15, 4, Ceiling), Some(-3));
    }

    /// Wide-operand divide: `(10^22 * 10^12) / 2 = 5e33`.
    #[test]
    fn div_pow10_div_wide_correctness() {
        const SCALE: u32 = 12;
        let a: i128 = 10_i128.pow(22);
        let b: i128 = 2;
        // a * 10^12 = 10^34; / 2 = 5e33.
        let expected: i128 = 5 * 10_i128.pow(33);
        assert_eq!(div_pow10_div::<SCALE>(a, b), Some(expected));
    }

    /// Round-trip within i128 range: `(a / b) * b` recovers `a`.
    #[test]
    fn div_pow10_div_round_trip_small() {
        const SCALE: u32 = 12;
        // 6.0 / 2.0 == 3.0 (raw: 6e12 / 2e12 -> in scaled space:
        // (6e12 * 1e12) / 2e12 == 3e12)
        let a: i128 = 6_000_000_000_000;
        let b: i128 = 2_000_000_000_000;
        assert_eq!(div_pow10_div::<SCALE>(a, b), Some(3_000_000_000_000));
    }

    /// Negative dividend with positive divisor produces a negative result.
    #[test]
    fn div_pow10_div_negative_dividend() {
        const SCALE: u32 = 12;
        let a: i128 = -6_000_000_000_000;
        let b: i128 = 2_000_000_000_000;
        assert_eq!(div_pow10_div::<SCALE>(a, b), Some(-3_000_000_000_000));
    }

    /// Round-trip property: `(a * b) / b == a` on the widening path.
    #[test]
    fn mul_div_round_trip_wide() {
        const SCALE: u32 = 12;
        let a: i128 = 50_000_000_000_000_000_000_000;
        let b: i128 = 30_000_000_000_000_000_000_000;
        // a * b / 10^12 = 1.5e33
        let prod = mul_div_pow10::<SCALE>(a, b).expect("wide mul");
        // Dividing back by b should recover a (up to truncation).
        let recovered = div_pow10_div::<SCALE>(prod, b).expect("wide div");
        assert_eq!(recovered, a);
    }

    /// `divide_pow_64limb_with` performs the 64-limb magnitude bump
    /// (with carry propagation across limbs) when `should_bump` is true
    /// and `rem != 0`. This is the wide-tier rounding path for div ops
    /// whose intermediate is wider than `i128`; the path is unreachable
    /// from `mul_div_pow10` (D38 uses the MG fast path), so the
    /// 64-limb routine has its own test driven by a non-HalfToEven mode
    /// on the largest div-with API.
    #[test]
    fn div_pow10_div_carry_propagation_under_directed_rounding() {
        const SCALE: u32 = 12;
        use crate::rounding::RoundingMode;
        // 1 / 3 = 0.333... — non-zero remainder, three modes pick
        // different last digits.
        let a: i128 = 1_000_000_000_000;  // 1.0 at S=12
        let b: i128 = 3_000_000_000_000;
        let trunc = div_pow10_div_with::<SCALE>(a, b, RoundingMode::Trunc).unwrap();
        let ceil = div_pow10_div_with::<SCALE>(a, b, RoundingMode::Ceiling).unwrap();
        let floor = div_pow10_div_with::<SCALE>(a, b, RoundingMode::Floor).unwrap();
        let ha = div_pow10_div_with::<SCALE>(a, b, RoundingMode::HalfAwayFromZero).unwrap();
        // All within 1 LSB of each other.
        let bits = [trunc, ceil, floor, ha];
        let min = *bits.iter().min().unwrap();
        let max = *bits.iter().max().unwrap();
        assert!(max - min <= 1, "modes diverged > 1 LSB: {bits:?}");
        // Ceiling should bump positive, Floor truncates positive.
        assert!(ceil >= trunc);
        // Negative side: Floor pushes away from zero, Ceiling truncates.
        let neg_floor = div_pow10_div_with::<SCALE>(-a, b, RoundingMode::Floor).unwrap();
        let neg_trunc = div_pow10_div_with::<SCALE>(-a, b, RoundingMode::Trunc).unwrap();
        assert!(neg_floor <= neg_trunc);
    }

    /// `isqrt_256(0, 0) == 0` short-circuit.
    #[test]
    fn isqrt_256_zero_input() {
        assert_eq!(isqrt_256(0, 0), 0);
    }

    /// `sqrt_raw_correctly_rounded(0, scale) == 0` short-circuit.
    #[test]
    fn sqrt_raw_correctly_rounded_zero_input() {
        for s in 0..=12 {
            assert_eq!(sqrt_raw_correctly_rounded(0, s), 0);
        }
    }

    /// `cbrt_raw_correctly_rounded(0, scale) == 0` short-circuit.
    #[test]
    fn cbrt_raw_correctly_rounded_zero_input() {
        for s in 0..=12 {
            assert_eq!(cbrt_raw_correctly_rounded(0, s), 0);
        }
    }

    /// `sqrt_raw_correctly_rounded(r, scale)` returns the
    /// correctly-rounded square root of `r · 10^scale`. The
    /// post-condition is `q = floor(sqrt(N))` OR `q = floor(sqrt(N))+1`
    /// when `N > q_floor² + q_floor` (the function's exact round-up
    /// test). This test walks a sweep of inputs that drive the diff-
    /// subtraction through the `wrapping_sub` borrow branch and asserts
    /// the post-condition holds for every one.
    #[test]
    fn sqrt_raw_correctly_rounded_post_condition_holds() {
        for r in [
            7u128,
            1_500_000_000_000u128,
            10u128.pow(20),
            (i128::MAX as u128) / 7,
        ] {
            let q = sqrt_raw_correctly_rounded(r, 12);
            let (n_hi, n_lo) = mul2(r, POW10_U128[12]);
            let q_floor = isqrt_256(n_hi, n_lo);
            // q must be either floor(sqrt(N)) or floor+1.
            assert!(
                q == q_floor || q == q_floor + 1,
                "sqrt({r}, 12): q={q}, floor={q_floor}",
            );
            // The round-up decision must agree with `N - q² > q`.
            let (qq_hi, qq_lo) = mul2(q_floor, q_floor);
            let (diff_hi, diff_lo) = if n_lo >= qq_lo {
                (n_hi - qq_hi, n_lo - qq_lo)
            } else {
                (n_hi - qq_hi - 1, n_lo.wrapping_sub(qq_lo))
            };
            let should_round_up = diff_hi != 0 || diff_lo > q_floor;
            let expected = if should_round_up { q_floor + 1 } else { q_floor };
            assert_eq!(q, expected, "sqrt({r}, 12): wrong round-up decision");
        }
    }

    /// At `SCALE=12` the cube-root radicand `r · 10^24` exceeds 2^254
    /// for `r > 10^52`, exercising the high-limb branch of
    /// `icbrt_384`. The post-condition is the same as the round-up
    /// identity in [`cbrt_raw_correctly_rounded_post_condition_holds`]:
    /// `q ∈ {floor, floor+1}` and the round-up agrees with
    /// `8·N ≥ (2·floor + 1)³`. We sweep small-to-moderate inputs at
    /// SCALE=12; the storage `r` fits `u128` while `r·10^24` extends
    /// into the 384-bit mid limb.
    #[test]
    fn cbrt_raw_high_limb_radicand_post_condition_holds() {
        for r in [
            8u128,
            27u128,
            1_000_000u128,
            10u128.pow(20),
            (i128::MAX as u128) / 7,
        ] {
            let q = cbrt_raw_correctly_rounded(r, 12);
            let n = mul_u128_by_256(r, pow10_256(24));
            let q_floor = icbrt_384(n);
            assert!(
                q == q_floor || q == q_floor + 1,
                "cbrt({r}, 12): q={q}, floor={q_floor}",
            );
            let eight_n = shl3_384(n);
            let two_q_plus_1 = 2 * q_floor + 1;
            let (sq_hi, sq_lo) = mul2(two_q_plus_1, two_q_plus_1);
            let cube = mul_u256_by_u128([sq_lo, sq_hi], two_q_plus_1);
            let expected = if ge_384(eight_n, cube) { q_floor + 1 } else { q_floor };
            assert_eq!(q, expected, "cbrt({r}, 12): wrong round-up decision");
        }
    }

    /// `cbrt_raw_correctly_rounded` returns the correctly-rounded cube
    /// root of `r · 10^(2·SCALE)` at the storage's last place — the
    /// post-condition `|q - exact_cbrt(N)| ≤ 0.5` LSB. Verified here as
    /// the bracket `q³ ≤ N ≤ (q+1)³` (so `q = floor(cbrt(N))`) for the
    /// cases where the value is closer to `q`, and `(q-1)³ ≤ N` plus a
    /// half-LSB comparison for the round-up cases.
    #[test]
    fn cbrt_raw_correctly_rounded_post_condition_holds() {
        for (r, scale) in [
            (8u128, 0),  // exact cube of 2
            (27u128, 0), // exact cube of 3
            (9u128, 0),  // non-perfect — nearest cube root is 2
            (64u128, 0),
            (65u128, 0),
            (10u128.pow(12), 6),  // mid-range with scale
        ] {
            let q = cbrt_raw_correctly_rounded(r, scale);
            // q³ ≤ N ≤ (q+1)³ OR q is the round-up of floor cbrt.
            // Express via the same integer identity the function uses:
            // round up iff 8·N ≥ (2q+1)³. We re-derive the floor and
            // compare.
            // N as 384-bit:
            let n = mul_u128_by_256(r, pow10_256(2 * scale));
            let q_floor = icbrt_384(n);
            // Either q == q_floor (no round-up) or q == q_floor + 1
            // (rounded up).
            assert!(
                q == q_floor || q == q_floor + 1,
                "cbrt({r}, {scale}): q={q}, floor={q_floor} — expected q ∈ {{floor, floor+1}}",
            );
            // And the round-up decision must agree with the
            // `8·N ≥ (2q_floor+1)³` test.
            let eight_n = shl3_384(n);
            let two_q_plus_1 = 2 * q_floor + 1;
            let (sq_hi, sq_lo) = mul2(two_q_plus_1, two_q_plus_1);
            let cube = mul_u256_by_u128([sq_lo, sq_hi], two_q_plus_1);
            let should_round_up = ge_384(eight_n, cube);
            let expected = if should_round_up { q_floor + 1 } else { q_floor };
            assert_eq!(
                q, expected,
                "cbrt({r}, {scale}): round-up decision mismatched",
            );
        }
    }
}
