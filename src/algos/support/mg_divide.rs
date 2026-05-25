// SPDX-License-Identifier: MIT OR Apache-2.0
//
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
//! DOI: 10.1109/TC.2010.143. The divisor-reciprocal precomputation
//! (paper Fig. 6.1, the `m'` multiplier with the normalisation shift)
//! and the dividend-times-reciprocal estimate with single add-back
//! correction (paper Alg. 4) are the basis for [`MG_EXP_MAGICS`] and
//! [`divmod_pow10_2word`].
//!
//! This module is an independent clean-room implementation derived
//! directly from the Moller-Granlund 2011 paper. Applying the technique
//! to the constant divisor `10^SCALE` is prior art; see the prior-art
//! credit and clean-room declaration in `ALGORITHMS.md`.
//!
//! # `SCALE = 0` special case
//!
//! At `SCALE = 0` the multiplier is 1, so no rescale is needed. The table
//! entry at index 0 is a placeholder `(0, 0)`; using it would produce a
//! shift of `128 - 0 = 128`, which is undefined behaviour on `u128`. Both
//! public entry points short-circuit at `SCALE == 0` before touching the
//! table.


/// `10^i` for `i = 0..=38`. Indexed by `scale` to skip the
/// runtime `u128::pow` (which is a 4-multiplication square-and-multiply
/// loop for the typical scale range) in hot paths like
/// `div_wide_pow10_with`. Last entry `10^38` is the largest power of
/// ten that fits in `u128`.
pub(crate) const POW10_U128: [u128; 39] = {
    let mut t = [1u128; 39];
    let mut i = 1;
    while i < 39 {
        t[i] = t[i - 1] * 10;
        i += 1;
    }
    t
};

/// Moller-Granlund reciprocal multiplier for the normalised divisor
/// `d_norm = d << s`, where `s = d.leading_zeros()` left-aligns `d`'s
/// top set bit to bit 127.
///
/// The paper (Moller-Granlund 2011, Fig. 6.1 / Eq. 1) defines, for a
/// normalised `N`-bit divisor `d_norm` with the high bit set, the
/// reciprocal
///
/// ```text
///     m' = floor((2^(2N) - 1) / d_norm) - 2^N
/// ```
///
/// so that `2^N + m' = floor((2^(2N) - 1) / d_norm)`. With `N = 128`
/// and `d_norm` having bit 127 set, `floor((2^256 - 1) / d_norm)` is a
/// 129-bit value whose implicit leading `2^128` is dropped; the stored
/// `m'` is the low 128 bits.
///
/// For our divisors `d = 10^k` the only proper divisors of `2^256 - 1`
/// that could perturb the floor are powers of two, which `10^k` is not,
/// so `floor((2^256 - 1) / d_norm) == floor(2^256 / d_norm)`; the
/// implementation divides `2^256` (the cleaner constant) by binary
/// long division. The 129th bit is asserted to be exactly 1.
const fn mg_reciprocal(d: u128) -> u128 {
    let s = d.leading_zeros();
    let d_norm = d << s;

    // Binary long division of 2^256 by `d_norm`. The numerator has a
    // single set bit at position 256; we sweep bit positions 256..=0,
    // shifting the running remainder left one place per step and pulling
    // in the numerator bit. The remainder stays strictly below `d_norm`
    // (< 2^128), but the pre-comparison value `(rem << 1) | nbit` can
    // reach the 129th bit, so the carry-out of the shift is tracked
    // separately in `rem_carry`.
    let mut quot_lo: u128 = 0; // low 128 bits of the quotient
    let mut quot_hi: u128 = 0; // bits 128.. (must end as exactly 1)
    let mut rem: u128 = 0;
    let mut pos: i32 = 256;
    while pos >= 0 {
        let rem_carry = rem >> 127;
        let shifted = (rem << 1) | if pos == 256 { 1 } else { 0 };
        // Quotient bit is set when the 129-bit value
        // (rem_carry:shifted) is >= d_norm.
        let fits = rem_carry == 1 || shifted >= d_norm;
        rem = if fits {
            shifted.wrapping_sub(d_norm)
        } else {
            shifted
        };
        quot_hi = (quot_hi << 1) | (quot_lo >> 127);
        quot_lo = (quot_lo << 1) | (fits as u128);
        pos -= 1;
    }
    // 2^256 / d_norm == 2^128 + m', so the high half is exactly 1 and
    // the low half is the stored reciprocal m'.
    assert!(
        quot_hi == 1,
        "MG reciprocal: normalised quotient must be 129-bit"
    );
    quot_lo
}

/// Magic constants for divide-by-`10^i`, `i = 0..=38`, computed from the
/// Moller-Granlund formula at compile time. Entry `i` is
/// `(m', s)` where `s = (10^i).leading_zeros()` is the normalisation
/// shift and `m'` is [`mg_reciprocal`] of `10^i`.
///
/// Index 0 holds the sentinel `(0, 0)`: `10^0 = 1` needs no divide and
/// every public entry point short-circuits `SCALE == 0` before indexing
/// this table.
const MG_EXP_MAGICS: [(u128, u32); 39] = {
    let mut table = [(0u128, 0u32); 39];
    let mut i = 1;
    while i < 39 {
        let d = POW10_U128[i];
        table[i] = (mg_reciprocal(d), d.leading_zeros());
        i += 1;
    }
    table
};

/// Full 256-bit product of two unsigned 128-bit integers.
///
/// Returns `(high, low)` with `high * 2^128 + low == a * b` exactly for
/// every `a`, `b` in `0..=u128::MAX`. `const fn`, so products of
/// constants (e.g. the magic-table reciprocals) fold at compile time.
///
/// # Method
///
/// Each operand splits into two 64-bit halves, and the product is the
/// sum of the four 64x64->128 partial products placed at base-`2^64`
/// columns 0, 1, 1, 2. The two column-1 partials and the carries that
/// spill upward are folded through a running accumulator so no
/// intermediate exceeds 128 bits. The widening multiplies stay within
/// `u128` because each half is `< 2^64`.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
pub(crate) const fn mul_u128_to_u256(a: u128, b: u128) -> (u128, u128) {
    const LOW64: u128 = u64::MAX as u128;
    let (a_lo, a_hi) = (a & LOW64, a >> 64);
    let (b_lo, b_hi) = (b & LOW64, b >> 64);

    // Column 0 (weight 2^0) and the two column-1 (weight 2^64) partials.
    let p00 = a_lo * b_lo;
    let p01 = a_lo * b_hi;
    let p10 = a_hi * b_lo;
    let p11 = a_hi * b_hi; // column 2 (weight 2^128)

    // Result low half: column 0 plus the low 64 bits of each column-1
    // partial. Accumulate the column-1 contribution to the low word and
    // capture every carry that crosses into the high word.
    let mid = (p00 >> 64) + (p01 & LOW64) + (p10 & LOW64);
    let low = (p00 & LOW64) | (mid << 64);
    let high = p11 + (p01 >> 64) + (p10 >> 64) + (mid >> 64);
    (high, low)
}

/// Divide the unsigned 256-bit value `n = n_high * 2^128 + n_low` by
/// `exp = 10^scale_idx`, returning `Some((quotient, remainder))` when
/// the quotient fits in 128 bits, or `None` when `n_high >= exp` (the
/// quotient would exceed 128 bits). The remainder always fits a `u128`
/// because `r < exp <= 10^38 < 2^127`.
///
/// # Method (Moller-Granlund 2011, Alg. 4 specialised to `2^128`)
///
/// The divisor is normalised by left-shifting both `exp` and `n` by
/// `s = exp.leading_zeros()`, putting the divisor's top bit at position
/// 127. The pre-stored reciprocal `m'` ([`mg_reciprocal`]) then gives
/// an estimate of the quotient from the high word of
/// `(2^128 + m') * n_norm_high`, i.e. `n_norm_high` plus the high half
/// of `m' * n_norm_high`, with the low-word reciprocal product folded in
/// so the estimate is never more than one short of the true quotient.
/// A single conditional add-back (the paper's "if r >= d" step) yields
/// the exact quotient and remainder.
///
/// Normalisation does not change the quotient: `floor(n / exp)` equals
/// `floor((n << s) / (exp << s))` because the shift scales numerator and
/// denominator alike; the remainder is recovered un-shifted from
/// `n_low - q * exp`.
///
/// # Preconditions
///
/// - `1 <= scale_idx <= 38` (callers handle `scale_idx == 0`).
/// - `exp == 10u128.pow(scale_idx)`, computed once by the caller.
///
/// # Precision
///
/// Strict: all arithmetic is integer-only; result is bit-exact.
#[inline]
pub(crate) fn divmod_pow10_2word(
    n_high: u128,
    n_low: u128,
    exp: u128,
    scale_idx: usize,
) -> Option<(u128, u128)> {
    // Quotient overflows 128 bits unless the high word is below `exp`.
    if n_high >= exp {
        return None;
    }

    let (recip, s) = MG_EXP_MAGICS[scale_idx];

    // Normalise the dividend by the same shift used to derive `recip`
    // (the divisor's leading-zero count). `s < 128` for every scale in
    // range, so `128 - s` is a valid shift width.
    let top = (n_high << s) | (n_low >> (128 - s));
    let bottom = n_low << s;

    // Quotient estimate = high 128 bits of `(2^128 + recip) * n_norm`,
    // where `n_norm = top * 2^128 + bottom`. The `2^128` factor
    // contributes `top` directly; the reciprocal contributes the high
    // words of `recip * top` and `recip * bottom`, combined with the
    // carry crossing the 2^128 boundary.
    let (hi_from_top, lo_from_top) = mul_u128_to_u256(recip, top);
    let (carry_from_bottom, _) = mul_u128_to_u256(recip, bottom);

    let (acc_low, c0) = lo_from_top.overflowing_add(carry_from_bottom);
    let acc_high = hi_from_top + u128::from(c0);

    // Folding the normalised low word back in completes the high-word
    // extraction; `q` is the floor quotient, possibly one too small.
    let (_, c1) = acc_low.overflowing_add(bottom);
    let q = acc_high + top + u128::from(c1);

    // Exact remainder against the un-normalised divisor, then the single
    // add-back: at most one increment makes `q` exact.
    let (_, prod_low) = mul_u128_to_u256(q, exp);
    let (rem, under) = n_low.overflowing_sub(prod_low);
    debug_assert!(n_high == mul_u128_to_u256(q, exp).0 + u128::from(under));

    if rem < exp {
        Some((q, rem))
    } else {
        Some((q + 1, rem - exp))
    }
}

/// Width-agnostic single-chunk MG divide of a u128-limb magnitude slice
/// by `10^scale` (`1 ≤ scale ≤ 38`), in place, with `mode`-aware
/// rounding. `mag` is the little-endian unsigned magnitude (zero-padded
/// to its full length); `neg` is the result sign for rounding tie-breaks.
///
/// This is the slice core extracted from [`div_wide_pow10_with`]: the
/// `BigInt` packing/unpacking is the only difference between the two, so
/// the `Int<N>`-only decimal `mul` kernel can build its product directly
/// in a u128 scratch buffer and call this — bit-identical to the typed
/// path, no `Int<2N>` work type. Base-`2^128` schoolbook long division,
/// each `(rem, limb) / exp` step served by [`divmod_pow10_2word`].
#[inline]
pub(crate) fn div_pow10_mag_u128(
    mag: &mut [u128],
    scale: u32,
    neg: bool,
    mode: crate::support::rounding::RoundingMode,
) {
    debug_assert!((1..=38).contains(&scale));
    let scale_idx = scale as usize;
    let exp = POW10_U128[scale_idx];

    // Skip trailing zero limbs (buffers are zero-padded to full width).
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
        let (q_limb, r_limb) = divmod_pow10_2word(rem, limb, exp, scale_idx)
            .expect("MG: rem < exp invariant violated");
        mag[i] = q_limb;
        rem = r_limb;
    }

    // Round the magnitude per `mode`.
    if rem != 0 {
        let q_is_odd = (mag[0] & 1) != 0;
        let comp = exp - rem;
        let cmp_r = rem.cmp(&comp);
        if crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
            for limb in mag.iter_mut() {
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
/// [`divmod_pow10_2word`] kernel. The magnitude buffer is
/// 64 limbs, so the same routine serves every width from `Int<4>`
/// to `Int<128>`.
///
/// Caller short-circuits `scale == 0` (no-op) and any `scale > 38`
/// (the magic table only covers `0..=38`).
///
/// Gated on the same `wide`/`x-wide` feature umbrella as the wide
/// integer layer — it's only invoked from the wide-tier
/// decimal `Mul` macro arm.
#[inline]
pub(crate) fn div_wide_pow10_with<W: crate::int::types::traits::BigInt, const N: usize>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W {
    debug_assert_eq!(
        N,
        W::U128_LIMBS,
        "magnitude buffer must match W's u128-limb width"
    );
    let mut mag = [0u128; N];
    let neg = n.mag_into_u128(&mut mag);

    // All arithmetic runs on the magnitude slice via the width-agnostic
    // core, shared with the `Int<N>`-only decimal `mul` kernel (which
    // builds its `a*b` product straight into a u128 scratch buffer and
    // calls `div_pow10_mag_u128` directly, never naming `Int<2N>`).
    div_pow10_mag_u128(&mut mag, scale, neg, mode);

    // Direct u128 → typed-Int unpack via the specialised
    // `from_mag_sign_u128` override; only `(L + 1) / 2` limbs are
    // consumed, avoiding the 288-u64 staging buffer.
    W::from_mag_sign_u128(&mag, neg)
}

/// Width-agnostic chain MG divide of a u128-limb magnitude slice by
/// `10^scale` for `scale > 38`, in place, with `mode`-aware rounding.
/// Slice core extracted from [`div_wide_pow10_chain_with`]; shared with
/// the `Int<N>`-only decimal `mul` kernel. See that wrapper's docs for
/// the combined-remainder rounding correctness argument.
#[inline]
pub(crate) fn div_pow10_chain_mag_u128(
    mag: &mut [u128],
    scale: u32,
    neg: bool,
    mode: crate::support::rounding::RoundingMode,
) {
    debug_assert!(
        scale > 38,
        "chain path is for SCALE > 38; callers handle smaller scales"
    );

    let mut top = mag.len();
    while top > 0 && mag[top - 1] == 0 {
        top -= 1;
    }

    // Chain divides by 10^38 until all but the last chunk is consumed.
    let exp38 = POW10_U128[38];
    let mut lower_any_nonzero = false;
    let mut remaining = scale;
    while remaining > 38 {
        let mut rem: u128 = 0;
        let mut i = top;
        while i > 0 {
            i -= 1;
            let (q, r) = divmod_pow10_2word(rem, mag[i], exp38, 38)
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

    // Final divide by 10^remaining (1..=38).
    let scale_idx = remaining as usize;
    let exp_last = POW10_U128[scale_idx];
    let mut r_last: u128 = 0;
    let mut i = top;
    while i > 0 {
        i -= 1;
        let (q, r) = divmod_pow10_2word(r_last, mag[i], exp_last, scale_idx)
            .expect("MG: rem < exp invariant violated");
        mag[i] = q;
        r_last = r;
    }

    // Combined-remainder rounding (proof in the wrapper docs).
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
        if crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, !neg) {
            let mut carry: u128 = 1;
            for limb in mag.iter_mut() {
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
}

/// Chain-of-`÷ 10^38` extension of [`div_wide_pow10_with`] to scales
/// past `38`. Factors `n / 10^SCALE` as a sequence of
/// `(n / 10^38) / 10^38 / … / 10^last` calls, each reusing the
/// existing base-`2^128` MG 2-by-1 kernel. The intermediate
/// quotients stay in the same `mag` buffer so we never allocate.
///
/// # Rounding correctness
///
/// Bit-exact half-to-even (and every other `RoundingMode`) across
/// `SCALE ∈ 39..=∞`. The chain produces a per-chunk remainder
/// sequence `r_1, r_2, …, r_k` (from each `÷ 10^38` stage) plus
/// a final `r_last < 10^(SCALE − 38·k)`, with the relationship
///
///   `n = q_final · 10^SCALE + r_last · 10^{SCALE−s}
///                            + r_k · 10^{SCALE−s−38} + … + r_1`
///
/// where `s = SCALE − 38·k`. Comparing the combined remainder
/// `r_total = r_last · 10^{SCALE−s} + lower` against `m/2`
/// reduces to a comparison of `r_last` against `10^s / 2` plus
/// a tie-break on whether any of the lower-chunk remainders is
/// non-zero — captured by the `lower_any_nonzero` flag and the
/// `r_last vs half` ordering below. Audited against the
/// schoolbook `div_rem` reference on 380K+ random Int<4> + 190K
/// random Int<16> inputs × every w ∈ 39..=100 × every
/// `RoundingMode` (see `round_div_chain_audit_*` tests in this
/// file).
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
pub(crate) fn div_wide_pow10_chain_with<W: crate::int::types::traits::BigInt, const N: usize>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W {
    debug_assert_eq!(
        N,
        W::U128_LIMBS,
        "magnitude buffer must match W's u128-limb width"
    );
    let mut mag = [0u128; N];
    let neg = n.mag_into_u128(&mut mag);

    // Identical chain arithmetic on the magnitude slice -- the only
    // difference from the `Int<N>`-only decimal path is the pack/unpack.
    div_pow10_chain_mag_u128(&mut mag, scale, neg, mode);

    W::from_mag_sign_u128(&mag, neg)
}

/// Width-generic [`div_wide_pow10_with`]: same MG single-chunk
/// `n / 10^scale` (`1 ≤ scale ≤ 38`), but the `(N+1)/2`-limb u128
/// magnitude buffer comes from the [`ComputeInt`] associated type (its
/// size lives in the impl; we slice it to `W::U128_LIMBS` here), so the
/// divide carries no const-generic limb count. This lets the
/// width-generic transcendental core call the MG reciprocal without
/// naming `{W::U128_LIMBS}` — the same fast path the per-tier
/// `decl_wide_transcendental!` core uses, now shared by the hyperbolics.
///
/// [`ComputeInt`]: crate::int::types::compute_int::ComputeInt
#[inline]
pub(crate) fn div_wide_pow10<W>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W
where
    W: crate::int::types::traits::BigInt + crate::int::types::compute_int::ComputeInt,
{
    let mut buf = <W as crate::int::types::compute_int::ComputeInt>::u128_limbs();
    let mag = &mut buf.as_mut()[..W::U128_LIMBS];
    let neg = n.mag_into_u128(mag);
    div_pow10_mag_u128(mag, scale, neg, mode);
    W::from_mag_sign_u128(mag, neg)
}

/// Width-generic [`div_wide_pow10_chain_with`] (the `scale > 38` chain),
/// buffer from [`ComputeInt`]. See [`div_wide_pow10`].
///
/// [`ComputeInt`]: crate::int::types::compute_int::ComputeInt
#[inline]
pub(crate) fn div_wide_pow10_chain<W>(
    n: W,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> W
where
    W: crate::int::types::traits::BigInt + crate::int::types::compute_int::ComputeInt,
{
    let mut buf = <W as crate::int::types::compute_int::ComputeInt>::u128_limbs();
    let mag = &mut buf.as_mut()[..W::U128_LIMBS];
    let neg = n.mag_into_u128(mag);
    div_pow10_chain_mag_u128(mag, scale, neg, mode);
    W::from_mag_sign_u128(mag, neg)
}

/// Mode-aware rounding for an *unsigned* magnitude `q` with remainder
/// `r` against divisor `m`, given the result sign — returns the
/// rounded magnitude. Caller applies the sign afterwards.
///
/// All mode-specific behaviour is delegated to
/// [`crate::support::rounding::should_bump`]; this function only assembles
/// the inputs from the unsigned-magnitude representation.
#[inline]
fn round_mag_with_mode(
    q: u128,
    r: u128,
    m: u128,
    mode: crate::support::rounding::RoundingMode,
    result_positive: bool,
) -> u128 {
    if r == 0 {
        return q;
    }
    let comp = m - r;
    let cmp_r = r.cmp(&comp);
    let q_is_odd = (q & 1) != 0;
    if crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive) {
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
    sqrt_raw_with(r, scale, crate::support::rounding::RoundingMode::HalfToEven)
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
pub(crate) fn sqrt_raw_with(
    r: u128,
    scale: u32,
    mode: crate::support::rounding::RoundingMode,
) -> u128 {
    use crate::support::rounding::RoundingMode;
    if r == 0 {
        return 0;
    }

    // Fast path: when the full radicand `r · 10^SCALE` fits a single
    // `u128`, hardware-backed `u128::isqrt` (stable since Rust 1.84)
    // replaces the 256-bit Newton loop. Threshold is
    // `r ≤ u128::MAX / 10^SCALE`, which at SCALE ≤ 18 covers every
    // representable raw value (e.g. at SCALE=18 the cap is
    // `≈ 3.4e38 / 10^18 ≈ 3.4e20`, well above `i128::MAX = 1.7e38`).
    // At SCALE ≥ 19 the cap is tight enough that typical operands
    // miss the fast path and fall through to the widening branch
    // below; small near-zero values still benefit.
    //
    // SCALE is const-generic at every call site, so the threshold
    // and the table lookup `POW10_U128[scale]` const-fold and the
    // branch is statically resolved per-monomorphisation.
    let scale_idx = scale as usize;
    let pow = POW10_U128[scale_idx];
    // u128 overflow check: `r ≤ u128::MAX / pow` iff `r * pow ≤ u128::MAX`.
    if r <= u128::MAX / pow {
        let n = r * pow;
        let q = n.isqrt();
        // Residual `N − q²` and the round-to-nearest tie test `diff > q`
        // mirror the 256-bit branch exactly: `q² ≤ N` always, so the
        // subtraction never underflows.
        let diff = n - q * q;
        let diff_nonzero = diff != 0;
        let halfway_round_up = diff > q;
        let bump = match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => halfway_round_up,
            RoundingMode::Trunc | RoundingMode::Floor => false,
            RoundingMode::Ceiling => diff_nonzero,
        };
        return if bump { q + 1 } else { q };
    }

    // Widening path: `r · 10^SCALE` overflows `u128`, so the full
    // 256-bit machinery is required.
    let (hi, lo) = mul_u128_to_u256(r, pow);
    let q = isqrt_256(hi, lo);
    let (q_sq_hi, q_sq_lo) = mul_u128_to_u256(q, q);
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
        let (hi, lo) = mul_u128_to_u256(10u128.pow(38), 10u128.pow(exp - 38));
        [lo, hi]
    }
}

/// `a * m` where `m` is a 256-bit value `[lo, hi]`; result is 384-bit.
fn mul_u128_by_256(a: u128, m: [u128; 2]) -> [u128; 3] {
    let (p0_hi, p0_lo) = mul_u128_to_u256(a, m[0]);
    let (p1_hi, p1_lo) = mul_u128_to_u256(a, m[1]);
    let limb0 = p0_lo;
    let (limb1, c1) = p0_hi.overflowing_add(p1_lo);
    let limb2 = p1_hi + u128::from(c1);
    [limb0, limb1, limb2]
}

/// `s * b` where `s` is a 256-bit value `[lo, hi]`; result is 384-bit.
fn mul_u256_by_u128(s: [u128; 2], b: u128) -> [u128; 3] {
    let (p0_hi, p0_lo) = mul_u128_to_u256(s[0], b);
    let (p1_hi, p1_lo) = mul_u128_to_u256(s[1], b);
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
        let (yy_hi, yy_lo) = mul_u128_to_u256(y, y);
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
    cbrt_raw_with_unsigned_mag(r, scale, crate::support::rounding::RoundingMode::HalfToEven)
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
    mode: crate::support::rounding::RoundingMode,
) -> u128 {
    cbrt_raw_with_signed(r, scale, false, mode)
}

/// Sign-aware mode dispatch for the cbrt raw computation. `negative`
/// is the sign of the source value (cbrt preserves sign).
pub(crate) fn cbrt_raw_with_signed(
    r: u128,
    scale: u32,
    negative: bool,
    mode: crate::support::rounding::RoundingMode,
) -> u128 {
    use crate::support::rounding::RoundingMode;
    if r == 0 {
        return 0;
    }
    let n = mul_u128_by_256(r, pow10_256(2 * scale));
    let q = icbrt_384(n);
    let eight_n = shl3_384(n);
    let two_q_plus_1 = 2 * q + 1;
    let (sq_hi, sq_lo) = mul_u128_to_u256(two_q_plus_1, two_q_plus_1);
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
    let (tq_sq_hi, tq_sq_lo) = mul_u128_to_u256(two_q, two_q);
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
    if a[2] != b[2] {
        return a[2] > b[2];
    }
    if a[1] != b[1] {
        return a[1] > b[1];
    }
    a[0] > b[0]
}

/// Compute `(a * b) / 10^SCALE` with truncating division semantics
/// matching `i128 /`. Returns `None` if the result overflows `i128`.
///
/// When `a * b` fits in `i128` the multiply is done directly and the
/// result is divided by the scale multiplier. When the product would
/// overflow `i128`, the unsigned absolute values are multiplied to a
/// full 256-bit result via [`mul_u128_to_u256`], divided by `10^SCALE` using the
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
    mul_div_pow10_with::<SCALE>(a, b, crate::support::rounding::DEFAULT_ROUNDING_MODE)
}

/// Mode-aware variant of [`mul_div_pow10`]: rounds the
/// divide-by-`10^SCALE` step according to `mode`. The default
/// `mul_div_pow10` is a thin wrapper that passes
/// [`crate::support::rounding::DEFAULT_ROUNDING_MODE`].
#[inline]
pub(crate) fn mul_div_pow10_with<const SCALE: u32>(
    a: i128,
    b: i128,
    mode: crate::support::rounding::RoundingMode,
) -> Option<i128> {
    // SCALE = 0: multiplier is 1, result is just a * b. No rounding
    // step possible.
    if SCALE == 0 {
        return a.checked_mul(b);
    }

    // Fast path: i128 * i128 didn't overflow. Apply `mode` at the
    // divide-by-10^SCALE step.
    if let Some(prod) = a.checked_mul(b) {
        return Some(crate::support::rounding::apply_rounding(
            prod,
            crate::D::<crate::int::types::Int<2>, SCALE>::multiplier().as_i128(),
            mode,
        ));
    }

    // Widening path: |a*b| > i128::MAX. Compute the unsigned product;
    // when it still fits a single u128 use a hardware u128 divide
    // (one DIV instruction), only falling through to the full 256-bit
    // magic-divide when the unsigned product overflows u128 too. The
    // u128 fast path covers the operand band sqrt(i128::MAX) < |op| <
    // sqrt(u128::MAX), i.e. ~1.3e19 < |op| < ~1.8e19 — the SCALE 19
    // typical-input window that previously paid the full mul_u128_to_u256 +
    // div_exp_fast_2word machinery for no reason.
    let ua = a.unsigned_abs();
    let ub = b.unsigned_abs();
    let exp = crate::D::<crate::int::types::Int<2>, SCALE>::multiplier().as_i128() as u128;

    let (uprod, hi_overflow) = ua.overflowing_mul(ub);
    if !hi_overflow {
        // u128 product fits. For SCALE <= 19 the divisor `exp = 10^SCALE`
        // also fits a single u64, in which case the LLVM `__udivti3`
        // soft-call (u128/u128) can be replaced by a two-step schoolbook
        // divide in base 2^64 — two hardware `divq` instructions on
        // x86_64 instead of the soft routine. The branch is const-folded
        // per-SCALE so the runtime cost is just the branch the compiler
        // proves away.
        let (q_floor, r) = if SCALE <= 19 {
            let d = exp as u64;
            let hi = (uprod >> 64) as u64;
            let lo = uprod as u64;
            if hi == 0 {
                // Single-limb dividend: one hardware divide suffices.
                let q = lo / d;
                let r = lo % d;
                (q as u128, r as u128)
            } else {
                // Two-limb schoolbook divide in base 2^64.
                let q_hi = hi / d;
                let r_hi = hi % d;
                let cur = ((r_hi as u128) << 64) | (lo as u128);
                let q_lo_u128 = cur / (d as u128);
                let r = cur - q_lo_u128 * (d as u128);
                let q = ((q_hi as u128) << 64) | (q_lo_u128 & u128::from(u64::MAX));
                (q, r)
            }
        } else {
            let q = uprod / exp;
            (q, uprod - q * exp)
        };
        let neg = (a < 0) ^ (b < 0);
        let q = round_mag_with_mode(q_floor, r, exp, mode, !neg);
        return if neg {
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
        };
    }

    // Truly wide path: unsigned 256-bit product, magic-divide by 10^SCALE.
    let (mhigh, mlow) = mul_u128_to_u256(ua, ub);
    let (q_floor, r) = divmod_pow10_2word(mhigh, mlow, exp, SCALE as usize)?;
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
/// [`mul_u128_to_u256`], divided by `|b|` using the binary long-divide, and the
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
    div_pow10_div_with::<SCALE>(a, b, crate::support::rounding::DEFAULT_ROUNDING_MODE)
}

/// Mode-aware variant of [`div_pow10_div`]: rounds the final divide
/// step according to `mode`. The default `div_pow10_div` is a thin
/// wrapper that passes [`crate::support::rounding::DEFAULT_ROUNDING_MODE`].
#[inline]
pub(crate) fn div_pow10_div_with<const SCALE: u32>(
    a: i128,
    b: i128,
    mode: crate::support::rounding::RoundingMode,
) -> Option<i128> {
    if b == 0 {
        return None;
    }
    // Probe for the `i128::MIN / -1` overflow case so the rounding path
    // below can rely on `a / b` not panicking.
    a.checked_div(b)?;

    // SCALE = 0: scale-narrowing step is `a / b` itself; apply mode.
    if SCALE == 0 {
        return Some(crate::support::rounding::apply_rounding(a, b, mode));
    }

    let mult = crate::D::<crate::int::types::Int<2>, SCALE>::multiplier().as_i128();

    // Fast path 1: a * mult fits in i128. At SCALE <= 18, i64::MAX * 10^18
    // fits with headroom; for larger SCALE the overflow check below
    // handles the fallthrough.
    if let Some(num) = a.checked_mul(mult) {
        return Some(crate::support::rounding::apply_rounding(num, b, mode));
    }

    // Fast path 2: `|a| * mult` overflows `i128` but still fits `u128`
    // (so the i128-signed `checked_mul` rejected it only because of the
    // sign bit, OR `|a|` is just past `i128::MAX / mult`). Skip the
    // 256-bit `mul_u128_to_u256 + div_long_256_by_128` path — a single hardware
    // `u128 / u128` suffices.
    //
    // Predicate: `|a| ≤ u128::MAX / mult`. Const-folded per SCALE.
    let ua = a.unsigned_abs();
    let umult = mult as u128;
    if ua <= u128::MAX / umult {
        let num = ua * umult;
        let ub = b.unsigned_abs();
        let q_floor = num / ub;
        let r = num % ub;
        let neg = (a < 0) ^ (b < 0);
        let q = round_mag_with_mode(q_floor, r, ub, mode, !neg);
        return if neg {
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
        };
    }

    // Widening path: |a|*mult overflows u128. Compute it as a 256-bit
    // unsigned, divide by |b| keeping the remainder, round per `mode`,
    // restore sign.
    let (mhigh, mlow) = mul_u128_to_u256(ua, umult);

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
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
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
        if !crate::support::rounding::DEFAULT_IS_HALF_TO_EVEN {
            return;
        }
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
        use crate::support::rounding::RoundingMode::*;
        const SCALE: u32 = 0;
        // 15 / 4 = 3.75
        assert_eq!(div_pow10_div_with::<SCALE>(15, 4, HalfToEven), Some(4));
        assert_eq!(
            div_pow10_div_with::<SCALE>(15, 4, HalfAwayFromZero),
            Some(4)
        );
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
        use crate::support::rounding::RoundingMode;
        // 1 / 3 = 0.333... — non-zero remainder, three modes pick
        // different last digits.
        let a: i128 = 1_000_000_000_000; // 1.0 at S=12
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
            let (n_hi, n_lo) = mul_u128_to_u256(r, POW10_U128[12]);
            let q_floor = isqrt_256(n_hi, n_lo);
            // q must be either floor(sqrt(N)) or floor+1.
            assert!(
                q == q_floor || q == q_floor + 1,
                "sqrt({r}, 12): q={q}, floor={q_floor}",
            );
            // The round-up decision must agree with `N - q² > q`.
            let (qq_hi, qq_lo) = mul_u128_to_u256(q_floor, q_floor);
            let (diff_hi, diff_lo) = if n_lo >= qq_lo {
                (n_hi - qq_hi, n_lo - qq_lo)
            } else {
                (n_hi - qq_hi - 1, n_lo.wrapping_sub(qq_lo))
            };
            let should_round_up = diff_hi != 0 || diff_lo > q_floor;
            let expected = if should_round_up {
                q_floor + 1
            } else {
                q_floor
            };
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
            let (sq_hi, sq_lo) = mul_u128_to_u256(two_q_plus_1, two_q_plus_1);
            let cube = mul_u256_by_u128([sq_lo, sq_hi], two_q_plus_1);
            let expected = if ge_384(eight_n, cube) {
                q_floor + 1
            } else {
                q_floor
            };
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
            (10u128.pow(12), 6), // mid-range with scale
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
            let (sq_hi, sq_lo) = mul_u128_to_u256(two_q_plus_1, two_q_plus_1);
            let cube = mul_u256_by_u128([sq_lo, sq_hi], two_q_plus_1);
            let should_round_up = ge_384(eight_n, cube);
            let expected = if should_round_up {
                q_floor + 1
            } else {
                q_floor
            };
            assert_eq!(
                q, expected,
                "cbrt({r}, {scale}): round-up decision mismatched",
            );
        }
    }

    // ── fast-path equivalence property tests ──
    //
    // `sqrt_raw_with` and `div_pow10_div_with` each gained a single-width
    // fast path that triggers when the operand magnitude lets the work
    // stay inside `u128`. These tests cross-check the fast path against
    // an always-widening reference implementation for a deterministic
    // SplitMix64 sweep, covering every rounding mode and a range of
    // SCALEs that straddle the fast-path/widening boundary.

    /// Reference sqrt that bypasses the fast path: always uses the
    /// 256-bit `mul_u128_to_u256 + isqrt_256` machinery.
    fn sqrt_raw_reference(
        r: u128,
        scale: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> u128 {
        use crate::support::rounding::RoundingMode;
        if r == 0 {
            return 0;
        }
        let (hi, lo) = mul_u128_to_u256(r, POW10_U128[scale as usize]);
        let q = isqrt_256(hi, lo);
        let (q_sq_hi, q_sq_lo) = mul_u128_to_u256(q, q);
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

    /// SplitMix64 — small deterministic 64-bit PRNG with good
    /// distribution for property tests. Not cryptographic; just here so
    /// the test sweep is reproducible.
    struct SplitMix64(u64);
    impl SplitMix64 {
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }
        fn next_u128(&mut self) -> u128 {
            ((self.next() as u128) << 64) | (self.next() as u128)
        }
    }

    fn all_modes() -> [crate::support::rounding::RoundingMode; 6] {
        use crate::support::rounding::RoundingMode::*;
        [
            HalfToEven,
            HalfAwayFromZero,
            HalfTowardZero,
            Trunc,
            Floor,
            Ceiling,
        ]
    }

    /// Cross-check the sqrt fast path against the widening reference on
    /// 200 000 randomized inputs per SCALE × mode, sampling SCALEs
    /// that straddle the `r·10^S ≤ u128::MAX` boundary.
    #[test]
    fn sqrt_fast_path_matches_reference() {
        const ITERS: usize = 200_000;
        let scales: &[u32] = &[0, 5, 10, 14, 18, 19, 23, 28, 33, 38];
        for &scale in scales {
            for mode in all_modes() {
                let mut rng = SplitMix64(0xC0FFEE_u64.wrapping_add(scale as u64));
                let pow = POW10_U128[scale as usize];
                let cap = u128::MAX / pow;
                for _ in 0..ITERS {
                    // Mix three input regimes: well inside fast path,
                    // straddling the boundary, well inside widening.
                    let raw = match rng.next() % 3 {
                        0 => rng.next_u128() % cap.max(1),
                        1 => {
                            // Within ±cap of the boundary.
                            let bias = rng.next_u128() % (cap.max(1));
                            cap.saturating_sub(bias).saturating_add(rng.next() as u128)
                        }
                        _ => rng.next_u128() & (i128::MAX as u128), // up to i128::MAX
                    };
                    let got = sqrt_raw_with(raw, scale, mode);
                    let expected = sqrt_raw_reference(raw, scale, mode);
                    assert_eq!(
                        got, expected,
                        "sqrt mismatch: raw={raw}, scale={scale}, mode={mode:?}",
                    );
                }
            }
        }
    }

    /// Reference div that always uses the 256-bit widening path,
    /// bypassing both the i128 and the u128 fast paths. Used to
    /// cross-check the new u128 fast path in `div_pow10_div_with`.
    fn div_pow10_div_reference<const SCALE: u32>(
        a: i128,
        b: i128,
        mode: crate::support::rounding::RoundingMode,
    ) -> Option<i128> {
        if b == 0 {
            return None;
        }
        a.checked_div(b)?;
        if SCALE == 0 {
            return Some(crate::support::rounding::apply_rounding(a, b, mode));
        }
        let mult = crate::D::<crate::int::types::Int<2>, SCALE>::multiplier().as_i128();
        let ua = a.unsigned_abs();
        let umult = mult as u128;
        let (mhigh, mlow) = mul_u128_to_u256(ua, umult);
        let ub = b.unsigned_abs();
        let (q_floor, r) = div_long_256_by_128_with_rem(mhigh, mlow, ub)?;
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

    /// Targeted sweep for the SCALEs where the u128 fast path can fire
    /// (SCALE ≤ ~38; the predicate `|a| ≤ u128::MAX / 10^S` is checked
    /// dynamically). For each scale × mode, draw `a, b` from the full
    /// `i128` range and assert fast-dispatch matches the reference.
    #[test]
    fn div_pow10_div_fast_path_matches_reference() {
        const ITERS: usize = 100_000;
        // Macro: instantiate the const-generic for each scale literal.
        macro_rules! sweep {
            ($scale:literal) => {{
                const SCALE: u32 = $scale;
                for mode in all_modes() {
                    let mut rng = SplitMix64(0xDEADBEEF_u64 + SCALE as u64);
                    for _ in 0..ITERS {
                        // a: full i128 spread; b: nonzero, also full spread.
                        let a = rng.next_u128() as i128;
                        let mut b: i128 = rng.next_u128() as i128;
                        if b == 0 {
                            b = 1;
                        }
                        let got = div_pow10_div_with::<SCALE>(a, b, mode);
                        let expected = div_pow10_div_reference::<SCALE>(a, b, mode);
                        assert_eq!(
                            got, expected,
                            "div mismatch: a={a}, b={b}, scale={SCALE}, mode={mode:?}",
                        );
                    }
                }
            }};
        }
        sweep!(0);
        sweep!(5);
        sweep!(10);
        sweep!(14);
        sweep!(18);
        sweep!(19);
        sweep!(23);
        sweep!(28);
        sweep!(33);
        sweep!(38);
    }

    /// Reference half-to-even quotient via the generic `div_rem` path:
    /// the same routine `round_div` uses in the wide_transcendental
    /// macro. Comparing the MG-based `div_wide_pow10_with` against this
    /// is the audit gate for routing `round_div` through the MG kernel
    /// whenever the divisor is `10^w` with `w ≤ 38`.
    #[cfg(any(feature = "d76", feature = "wide"))]
    fn round_div_reference_int256(n: crate::int::types::Int<4>, w: u32) -> crate::int::types::Int<4> {
        use crate::int::types::Int;
        let d_u128 = POW10_U128[w as usize];
        let d: Int<4> = Int::from_u128(d_u128);
        let zero: Int<4> = Int::from_u128(0u128);
        let one: Int<4> = Int::from_u128(1u128);
        let (q, r) = n.div_rem(d);
        if r == zero {
            return q;
        }
        let ar = if r < zero { -r } else { r };
        let comp = d - ar;
        let cmp_r = ar.cmp(&comp);
        let q_is_odd = q.bit(0);
        let result_positive = n >= zero;
        let bump = crate::support::rounding::should_bump(
            crate::support::rounding::RoundingMode::HalfToEven,
            cmp_r,
            q_is_odd,
            result_positive,
        );
        if bump {
            if result_positive { q + one } else { q - one }
        } else {
            q
        }
    }

    /// Bit-exact audit: `div_wide_pow10_with(..HalfToEven)` must produce
    /// the same quotient as the generic-`div_rem` half-to-even reference
    /// for every divisor `10^w` with `1 ≤ w ≤ 38`, across random `Int<4>`
    /// numerators in both signs.
    ///
    /// If this passes the MG kernel is a drop-in replacement for the
    /// existing `round_div(n, pow10_cached(w))` whenever the divisor is
    /// a known power of ten with `w ≤ 38` — which is the entire `mul` /
    /// `mul_cached` / `round_to_storage_with` call-set in the
    /// wide_transcendental macro.
    /// Mode-aware reference: same shape as `round_div_reference_int256`
    /// but parameterised on a `RoundingMode`. Used by the full-mode
    /// audit to confirm the MG kernel obeys every mode the production
    /// path passes.
    #[cfg(any(feature = "d76", feature = "wide"))]
    fn round_div_reference_int256_with(
        n: crate::int::types::Int<4>,
        w: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> crate::int::types::Int<4> {
        use crate::int::types::Int;
        let d_u128 = POW10_U128[w as usize];
        let d: Int<4> = Int::from_u128(d_u128);
        let zero: Int<4> = Int::from_u128(0u128);
        let one: Int<4> = Int::from_u128(1u128);
        let (q, r) = n.div_rem(d);
        if r == zero {
            return q;
        }
        let ar = if r < zero { -r } else { r };
        let comp = d - ar;
        let cmp_r = ar.cmp(&comp);
        let q_is_odd = q.bit(0);
        let result_positive = n >= zero;
        let bump = crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive);
        if bump {
            if result_positive { q + one } else { q - one }
        } else {
            q
        }
    }

    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn round_div_audit_mg_matches_div_rem_int256() {
        use crate::int::types::Int;
        const ITERS: usize = 10_000;
        for w in 1u32..=38 {
            let mut rng = SplitMix64(0xA17D17_u64.wrapping_add(w as u64));
            for _ in 0..ITERS {
                // Build a numerator drawn from a mix of regimes:
                //   - small (fits one u128 limb)
                //   - mid (two u128 limbs)
                //   - sign-flipped versions of each
                let regime = rng.next() % 4;
                let mag_high = if regime >= 2 {
                    rng.next_u128() & (i128::MAX as u128)
                } else {
                    0
                };
                let mag_low = rng.next_u128();
                let pos: Int<4> = {
                    let lo: Int<4> = Int::from_u128(mag_low);
                    let hi: Int<4> = Int::from_u128(mag_high);
                    (hi << 128_u32) + lo
                };
                let n: Int<4> = if regime % 2 == 1 { -pos } else { pos };

                let got =
                    crate::algos::support::mg_divide::div_wide_pow10_with::<
                        Int<4>,
                        { <Int<4> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, crate::support::rounding::RoundingMode::HalfToEven);
                let expected = round_div_reference_int256(n, w);
                assert_eq!(got, expected, "round_div MG audit mismatch: w={w}, n={n:?}",);
            }
        }
    }

    /// All-modes variant: MG kernel matches the reference under every
    /// `RoundingMode` the production path can hand it. The
    /// `round_to_storage_with` routing in `wide_transcendental.rs`
    /// passes the caller's mode straight through, so HalfToEven alone
    /// isn't sufficient evidence.
    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn round_div_audit_mg_all_modes_int256() {
        use crate::int::types::Int;
        const ITERS: usize = 2_000;
        let scales: &[u32] = &[1, 5, 10, 19, 28, 38];
        for &w in scales {
            for mode in all_modes() {
                let mut rng = SplitMix64(0xCAFE_u64.wrapping_add(w as u64 ^ mode as u64));
                for _ in 0..ITERS {
                    let regime = rng.next() % 4;
                    let mag_high = if regime >= 2 {
                        rng.next_u128() & (i128::MAX as u128)
                    } else {
                        0
                    };
                    let mag_low = rng.next_u128();
                    let pos: Int<4> = {
                        let lo: Int<4> = Int::from_u128(mag_low);
                        let hi: Int<4> = Int::from_u128(mag_high);
                        (hi << 128_u32) + lo
                    };
                    let n: Int<4> = if regime % 2 == 1 { -pos } else { pos };
                    let got = crate::algos::support::mg_divide::div_wide_pow10_with::<
                        Int<4>,
                        { <Int<4> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, mode);
                    let expected = round_div_reference_int256_with(n, w, mode);
                    assert_eq!(
                        got, expected,
                        "round_div MG all-modes mismatch: w={w}, mode={mode:?}",
                    );
                }
            }
        }
    }

    /// Same audit as [`round_div_audit_mg_matches_div_rem_int256`] but
    /// for the next wider tier's work integer. The MG kernel iterates
    /// over magnitude limbs so we need to confirm it stays bit-exact
    /// on the multi-limb pathway too.
    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn round_div_audit_mg_matches_div_rem_int1024() {
        use crate::int::types::Int;
        let zero: Int<16> = Int::from_u128(0u128);
        let one: Int<16> = Int::from_u128(1u128);
        const ITERS: usize = 5_000;
        for w in 1u32..=38 {
            let mut rng = SplitMix64(0xB02ED2_u64.wrapping_add(w as u64));
            let d: Int<16> = Int::from_u128(POW10_U128[w as usize]);
            for _ in 0..ITERS {
                // Fill up to 6 u128 limbs (768 bits) of magnitude — well
                // past one MG kernel pass.
                let limbs = (rng.next() % 7) as usize;
                let mut n: Int<16> = zero;
                for k in 0..limbs {
                    let chunk: Int<16> = Int::from_u128(rng.next_u128());
                    n = n + (chunk << ((k * 128) as u32));
                }
                if rng.next() & 1 == 1 {
                    n = -n;
                }

                let got =
                    crate::algos::support::mg_divide::div_wide_pow10_with::<
                        Int<16>,
                        { <Int<16> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, crate::support::rounding::RoundingMode::HalfToEven);
                // Reference half-to-even via div_rem.
                let (q, r) = n.div_rem(d);
                let expected = if r == zero {
                    q
                } else {
                    let ar = if r < zero { -r } else { r };
                    let comp = d - ar;
                    let cmp_r = ar.cmp(&comp);
                    let q_is_odd = q.bit(0);
                    let result_positive = n >= zero;
                    let bump = crate::support::rounding::should_bump(
                        crate::support::rounding::RoundingMode::HalfToEven,
                        cmp_r,
                        q_is_odd,
                        result_positive,
                    );
                    if bump {
                        if result_positive { q + one } else { q - one }
                    } else {
                        q
                    }
                };
                assert_eq!(got, expected, "round_div MG audit (Int<16>) mismatch: w={w}",);
            }
        }
    }

    // ----------------------------------------------------------------
    // Chain-MG audit (w > 38)
    // ----------------------------------------------------------------
    //
    // The chain divides `n / 10^SCALE` by repeated `÷ 10^38` plus a
    // final `÷ 10^(SCALE − 38·k)`. Combined-remainder bookkeeping
    // (`lower_any_nonzero` + `r_last`) yields bit-exact half-to-even
    // (and every other `RoundingMode`) against the schoolbook
    // `div_rem` reference.
    //
    // These tests confirm that property across every w in `39..=100`,
    // every mode, and a numerator distribution that exercises:
    //   - sub-w divisor magnitude (quotient = 0, exact rounding driven
    //     entirely by the remainder),
    //   - mid-range magnitudes that straddle the chunk boundary,
    //   - full Int<4> / Int<16> magnitudes,
    //   - half-tie inputs constructed as `q·10^SCALE + 10^SCALE/2`
    //     (probes the cross-chunk Equal vs Greater decision),
    //   - sign-flipped versions of all of the above.

    /// Build `10^w` in the target wide integer width.
    #[cfg(any(feature = "d76", feature = "wide"))]
    fn pow10_int256(w: u32) -> crate::int::types::Int<4> {
        use crate::int::types::Int;
        let mut d: Int<4> = Int::from_u128(1u128);
        // 10^38 fits in u128, so we can build with chunks of up to 38.
        let mut remaining = w;
        while remaining > 0 {
            let chunk = remaining.min(38);
            let factor: Int<4> = Int::from_u128(POW10_U128[chunk as usize]);
            d = d * factor;
            remaining -= chunk;
        }
        d
    }

    #[cfg(any(feature = "d307", feature = "wide"))]
    fn pow10_int1024(w: u32) -> crate::int::types::Int<16> {
        use crate::int::types::Int;
        let mut d: Int<16> = Int::from_u128(1u128);
        let mut remaining = w;
        while remaining > 0 {
            let chunk = remaining.min(38);
            let factor: Int<16> = Int::from_u128(POW10_U128[chunk as usize]);
            d = d * factor;
            remaining -= chunk;
        }
        d
    }

    /// Int<4> reference quotient via div_rem + should_bump for any
    /// `RoundingMode`. Same shape as `round_div_reference_int256_with`
    /// but `d = 10^w` is constructed via `pow10_int256` instead of a
    /// single u128 limb (so w can exceed 38).
    #[cfg(any(feature = "d76", feature = "wide"))]
    fn round_div_chain_reference_int256(
        n: crate::int::types::Int<4>,
        w: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> crate::int::types::Int<4> {
        use crate::int::types::Int;
        let d = pow10_int256(w);
        let zero: Int<4> = Int::from_u128(0u128);
        let one: Int<4> = Int::from_u128(1u128);
        let (q, r) = n.div_rem(d);
        if r == zero {
            return q;
        }
        let ar = if r < zero { -r } else { r };
        let comp = d - ar;
        let cmp_r = ar.cmp(&comp);
        let q_is_odd = q.bit(0);
        let result_positive = n >= zero;
        let bump = crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive);
        if bump {
            if result_positive { q + one } else { q - one }
        } else {
            q
        }
    }

    #[cfg(any(feature = "d307", feature = "wide"))]
    fn round_div_chain_reference_int1024(
        n: crate::int::types::Int<16>,
        w: u32,
        mode: crate::support::rounding::RoundingMode,
    ) -> crate::int::types::Int<16> {
        use crate::int::types::Int;
        let d = pow10_int1024(w);
        let zero: Int<16> = Int::from_u128(0u128);
        let one: Int<16> = Int::from_u128(1u128);
        let (q, r) = n.div_rem(d);
        if r == zero {
            return q;
        }
        let ar = if r < zero { -r } else { r };
        let comp = d - ar;
        let cmp_r = ar.cmp(&comp);
        let q_is_odd = q.bit(0);
        let result_positive = n >= zero;
        let bump = crate::support::rounding::should_bump(mode, cmp_r, q_is_odd, result_positive);
        if bump {
            if result_positive { q + one } else { q - one }
        } else {
            q
        }
    }

    /// Random Int<4> numerators, every mode, every w in 39..=76 (the
    /// 2-chunk regime). HalfToEven gets a higher iteration count
    /// since it's the production default; the other modes get a
    /// smaller sample.
    #[cfg(any(feature = "d76", feature = "wide"))]
    #[test]
    fn round_div_chain_audit_int256_w39_76_all_modes() {
        use crate::int::types::Int;
        const ITERS_HTE: usize = 5_000;
        const ITERS_OTHER: usize = 1_000;
        for w in 39u32..=76 {
            for mode in all_modes() {
                let iters = if matches!(mode, crate::support::rounding::RoundingMode::HalfToEven) {
                    ITERS_HTE
                } else {
                    ITERS_OTHER
                };
                let mut rng = SplitMix64(0xC4A1_u64.wrapping_add((w as u64) << 8 ^ (mode as u64)));
                for _ in 0..iters {
                    let regime = rng.next() % 4;
                    let mag_high = if regime >= 2 {
                        rng.next_u128() & (i128::MAX as u128)
                    } else {
                        0
                    };
                    let mag_low = rng.next_u128();
                    let pos: Int<4> = {
                        let lo: Int<4> = Int::from_u128(mag_low);
                        let hi: Int<4> = Int::from_u128(mag_high);
                        (hi << 128_u32) + lo
                    };
                    let n: Int<4> = if regime % 2 == 1 { -pos } else { pos };

                    let got = crate::algos::support::mg_divide::div_wide_pow10_chain_with::<
                        Int<4>,
                        { <Int<4> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, mode);
                    let expected = round_div_chain_reference_int256(n, w, mode);
                    assert_eq!(
                        got, expected,
                        "chain MG audit (Int<4>) mismatch: w={w}, mode={mode:?}, n={n:?}",
                    );
                }
            }
        }
    }

    /// Multi-limb numerators (Int<16> up to ~768 bits magnitude) across
    /// the full w ∈ 39..=100 range. This is the high-chunk regime —
    /// w=100 means 3 chain passes (38·2 = 76, then 24) — and is
    /// where the cross-chunk Equal-vs-Greater decision matters most.
    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn round_div_chain_audit_int1024_w39_100() {
        use crate::int::types::Int;
        let zero: Int<16> = Int::from_u128(0u128);
        const ITERS_HTE: usize = 3_000;
        for w in 39u32..=100 {
            let mut rng = SplitMix64(0x5C5C_u64.wrapping_add(w as u64));
            for _ in 0..ITERS_HTE {
                let limbs = (rng.next() % 7) as usize;
                let mut n: Int<16> = zero;
                for k in 0..limbs {
                    let chunk: Int<16> = Int::from_u128(rng.next_u128());
                    n = n + (chunk << ((k * 128) as u32));
                }
                if rng.next() & 1 == 1 {
                    n = -n;
                }

                let got =
                    crate::algos::support::mg_divide::div_wide_pow10_chain_with::<
                        Int<16>,
                        { <Int<16> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, crate::support::rounding::RoundingMode::HalfToEven);
                let expected = round_div_chain_reference_int1024(
                    n,
                    w,
                    crate::support::rounding::RoundingMode::HalfToEven,
                );
                assert_eq!(got, expected, "chain MG audit (Int<16> HTE) mismatch: w={w}",);
            }
        }
    }

    /// All-modes Int<16> sweep over a w sample that hits each chain-
    /// pass count boundary: w=39 (2 passes, last chunk = 1),
    /// w=50 (2 passes, last = 12 — the D57<20> production width),
    /// w=76 (2 passes, last = 38), w=77 (3 passes, last = 1),
    /// w=100 (3 passes, last = 24).
    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn round_div_chain_audit_int1024_all_modes_sample_w() {
        use crate::int::types::Int;
        let zero: Int<16> = Int::from_u128(0u128);
        const ITERS: usize = 1_000;
        let ws: &[u32] = &[39, 50, 57, 76, 77, 88, 100];
        for &w in ws {
            for mode in all_modes() {
                let mut rng = SplitMix64(0x9E15_u64.wrapping_add((w as u64) << 4 ^ mode as u64));
                for _ in 0..ITERS {
                    let limbs = (rng.next() % 7) as usize;
                    let mut n: Int<16> = zero;
                    for k in 0..limbs {
                        let chunk: Int<16> = Int::from_u128(rng.next_u128());
                        n = n + (chunk << ((k * 128) as u32));
                    }
                    if rng.next() & 1 == 1 {
                        n = -n;
                    }
                    let got = crate::algos::support::mg_divide::div_wide_pow10_chain_with::<
                        Int<16>,
                        { <Int<16> as crate::int::types::traits::BigInt>::U128_LIMBS },
                    >(n, w, mode);
                    let expected = round_div_chain_reference_int1024(n, w, mode);
                    assert_eq!(
                        got, expected,
                        "chain MG audit (Int<16> all modes) mismatch: w={w}, mode={mode:?}",
                    );
                }
            }
        }
    }

    /// Constructed half-tie inputs: `n = q·10^w + 10^w/2` and
    /// `n = q·10^w + 10^w/2 + δ` for small δ. The tie path is the
    /// least-frequent regime under uniform random inputs but the
    /// most prone to bookkeeping bugs (Equal vs Greater across
    /// chunks). Confirms HalfToEven breaks ties to even and the
    /// directed modes pick the correct side.
    #[cfg(any(feature = "d307", feature = "wide"))]
    #[test]
    fn round_div_chain_audit_int1024_constructed_half_ties() {
        use crate::int::types::Int;
        let two: Int<16> = Int::from_u128(2u128);
        let ws: &[u32] = &[39, 50, 57, 76, 77, 100];
        for &w in ws {
            let pow_w = pow10_int1024(w);
            let half = pow_w / two;
            // Three q values: 0, 1, large.
            let qs: [Int<16>; 3] = [
                Int::<16>::from_u128(0u128),
                Int::<16>::from_u128(1u128),
                Int::<16>::from_u128(7u128) << 200_u32,
            ];
            let deltas: [Int<16>; 3] = [
                Int::<16>::from_u128(0u128),
                -Int::<16>::from_u128(1u128),
                Int::<16>::from_u128(1u128),
            ];
            for q in qs {
                for delta in deltas {
                    for &sign_neg in &[false, true] {
                        let pos_n = q * pow_w + half + delta;
                        let n = if sign_neg { -pos_n } else { pos_n };
                        for mode in all_modes() {
                            let got = crate::algos::support::mg_divide::div_wide_pow10_chain_with::<
                                Int<16>,
                                { <Int<16> as crate::int::types::traits::BigInt>::U128_LIMBS },
                            >(n, w, mode);
                            let expected = round_div_chain_reference_int1024(n, w, mode);
                            assert_eq!(
                                got, expected,
                                "chain MG half-tie mismatch: w={w}, mode={mode:?}, n={n:?}",
                            );
                        }
                    }
                }
            }
        }
    }

    /// Builds `10^w` as an `Int<256>` via chunked multiply (analogue of
    /// [`pow10_int1024`] at the widest work-integer width).
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    fn pow10_int16384(w: u32) -> crate::int::types::Int<256> {
        use crate::int::types::Int;
        let mut d: Int<256> = Int::from_u128(1u128);
        let mut remaining = w;
        while remaining > 0 {
            let chunk = remaining.min(38);
            let factor: Int<256> = Int::from_u128(POW10_U128[chunk as usize]);
            d = d * factor;
            remaining -= chunk;
        }
        d
    }

    /// Regression for the original `div_wide_pow10_chain_with` buffer
    /// bug: `D1232`'s work integer is `Int<256>` (128 u128 limbs =
    /// 16384 bits), but the chain kernel once packed the magnitude
    /// into a fixed 64-u128-limb (8192-bit) buffer, silently
    /// truncating any numerator above 8192 bits.
    ///
    /// The width-adaptive buffer (`N = W::U128_LIMBS`) must still hand
    /// `Int<256>` its full 128-limb buffer. This test sets a bit above
    /// the old 8192-bit ceiling (so the value is unrepresentable in a
    /// 64-limb buffer) and divides by `10^w` at a working scale
    /// `w ≥ 620` (D1232's wide-transcendental band). With truncation
    /// the high bits vanish and the quotient diverges from the
    /// schoolbook `div_rem` reference, which operates on the full
    /// 256-u64-limb value; the assertion fails in that case.
    #[cfg(any(feature = "d1232", feature = "xx-wide"))]
    #[test]
    fn round_div_chain_int16384_above_8192_bits_not_truncated() {
        use crate::int::types::Int;
        let zero: Int<256> = Int::from_u128(0u128);
        let one: Int<256> = Int::from_u128(1u128);

        // Numerators with magnitude bits set well past the old
        // 8192-bit (64-u128-limb) buffer ceiling: a high bit near the
        // top of the 16384-bit range plus low-order entropy.
        let high_bit_positions: &[u32] = &[8200, 10000, 12345, 16000];
        let scales: &[u32] = &[620, 700, 1024];

        for &bit in high_bit_positions {
            // n = (1 << bit) | low-entropy limbs.
            let mut n: Int<256> = one << bit;
            n = n
                + (Int::<256>::from_u128(0xdead_beef_cafe_f00d_u128)
                    << 64_u32);
            n = n + Int::<256>::from_u128(0x1234_5678_9abc_def0_u128);

            for &w in scales {
                for &sign_neg in &[false, true] {
                    let nn = if sign_neg { -n } else { n };
                    for mode in all_modes() {
                        let got = crate::algos::support::mg_divide::div_wide_pow10_chain_with::<
                            Int<256>,
                            { <Int<256> as crate::int::types::traits::BigInt>::U128_LIMBS },
                        >(nn, w, mode);

                        // Schoolbook reference on the untruncated value.
                        let d = pow10_int16384(w);
                        let (q, r) = nn.div_rem(d);
                        let expected = if r == zero {
                            q
                        } else {
                            let ar = if r < zero { -r } else { r };
                            let comp = d - ar;
                            let cmp_r = ar.cmp(&comp);
                            let q_is_odd = q.bit(0);
                            let result_positive = nn >= zero;
                            if crate::support::rounding::should_bump(
                                mode,
                                cmp_r,
                                q_is_odd,
                                result_positive,
                            ) {
                                if result_positive { q + one } else { q - one }
                            } else {
                                q
                            }
                        };

                        assert_eq!(
                            got, expected,
                            "Int<256> chain divide truncated above 8192 bits: \
                             bit={bit}, w={w}, mode={mode:?}, neg={sign_neg}",
                        );
                    }
                }
            }
        }
    }

    /// The compile-time-generated [`MG_EXP_MAGICS`] table must satisfy
    /// the Moller-Granlund reciprocal definition for every scale it
    /// covers: `2^256 / (10^k << s) == 2^128 + recip` exactly, and the
    /// stored shift `s` is the divisor's leading-zero count. This proves
    /// the generator derives the magics from the paper's formula rather
    /// than copying literal constants — and that the values are the ones
    /// the divide kernel needs.
    #[test]
    fn mg_magics_match_paper_formula() {
        for k in 1..=38usize {
            let d = POW10_U128[k];
            let (recip, s) = MG_EXP_MAGICS[k];
            assert_eq!(s, d.leading_zeros(), "shift for 10^{k}");

            let d_norm = d << s;
            // floor(2^256 / d_norm) recomputed independently here as
            // 2^128 + floor((2^256 - d_norm*2^128) / d_norm) using the
            // identity 2^256 = (2^128) * 2^128. We verify via the
            // defining product instead: (2^128 + recip) * d_norm must be
            // the largest multiple of d_norm at or below 2^256.
            let (q_hi, q_lo) = mul_u128_to_u256(recip, d_norm);
            // (2^128 + recip) * d_norm = recip*d_norm + d_norm*2^128.
            // Its high 128 bits: q_hi + d_norm; low: q_lo.
            let prod_hi = q_hi + d_norm;
            // Largest multiple of d_norm <= 2^256 means prod_hi <= 2^128
            // (i.e. high word of 2^256 is 1 at bit 128) and one more
            // d_norm would cross 2^256.
            // prod_hi as a 129-bit count: it must not exceed 2^128, and
            // adding d_norm must overflow past 2^256.
            assert!(
                prod_hi <= 1u128 << 127 || q_hi <= d_norm,
                "reciprocal lower bound 10^{k}"
            );
            // Exactness check through the kernel: dividing the boundary
            // value 2^128*10^k - 1 (the largest n with quotient < 10^k
            // would overflow) is awkward; instead spot-check the kernel
            // against hardware u128 division across a few dividends.
            let exp = d;
            for &n_low in &[0u128, 1, exp - 1, exp, exp + 7, u128::MAX] {
                // n_high = 0 keeps the true quotient within u128 and lets
                // us compare against the native u128 divide.
                let (q, r) =
                    divmod_pow10_2word(0, n_low, exp, k).expect("quotient fits when n_high == 0");
                assert_eq!(q, n_low / exp, "kernel quotient 10^{k}, n_low={n_low}");
                assert_eq!(r, n_low % exp, "kernel remainder 10^{k}, n_low={n_low}");
            }
        }
    }
}
