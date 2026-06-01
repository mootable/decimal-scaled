// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `rescale` policy — `mag / 10^scale` (the decimal `÷10^SCALE`), rounded.
//!
//! The single matcher for the post-multiply / post-Taylor rescale. One
//! operation, three kept kernels:
//!
//! * [`Algorithm::MgSingle`] — the single-chunk Möller–Granlund magic-number
//!   divide ([`mg_divide::div_pow10_mag_u128`]), for `scale ≤ 38`.
//! * [`Algorithm::MgChain`] — the chained MG divide
//!   ([`mg_divide::div_pow10_chain_mag_u128`]), for the narrow / out-of-baked-
//!   range cells where Newton does not win.
//! * [`Algorithm::Newton`] — the **baked-reciprocal** Newton rescale
//!   ([`newton_reciprocal::newton_rescale_arm`]), SELECTED for the wide /
//!   high-scale band (see [`select`]). The §9.20 baked `⌊2^k/10^scale⌋` const
//!   table ([`crate::consts::newton_recip`]) makes its `precompute` a table
//!   lookup, not a per-call Knuth divide, so the one-pass O(width) apply beats
//!   the `⌈scale/38⌉`-pass `MgChain` 1.5–13× above the crossover.
//!
//! [`select`] is a `const fn` keyed on `(scale, width_bits)`, so the two
//! entry doors share ONE classifier (the const/slice dual-door of
//! `docs/ARCHITECTURE.md`):
//!
//! * [`dispatch_mag`] — the **slice door**: rescales a `&mut [u128]`
//!   magnitude in place. Called by the `Int<N>` decimal `mul`
//!   ([`crate::algos::mul::mul_widen_divide`]) with a **const `SCALE`** and a
//!   **const width**, so `select` const-folds the verdict to a single kernel
//!   call per monomorphisation. (Its magnitude is the full `2N` product, so it
//!   is already sized to the real width — no trim needed.)
//! * [`dispatch_wide_pow10`] — the **typed door**: rescales a work integer
//!   `W`. Called by the wide-transcendental rounding (the `wide_transcendental`
//!   cores, `lib.rs::round_div_pow10_fast`) with a **runtime `scale`** from
//!   the Taylor loops, so `select` is one runtime branch. It is
//!   **magnitude-length-aware** (task 9.24): the per-term magnitude is far
//!   shorter than the work buffer, so it trims the leading-zero high limbs and
//!   routes + sizes `select`/Newton on the SIGNIFICANT length, not `W::BITS`
//!   (see the fn doc).

use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// The three rescale kernels. `Newton` (baked-reciprocal Newton — the §9.20
/// const table makes `precompute` a lookup, not a per-call Knuth divide) is
/// now SELECTED for the wide / high-scale rescale (see [`select`]); `MgSingle`
/// for `scale <= 38`, `MgChain` for the rest.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm {
    MgSingle,
    MgChain,
    Newton,
}

/// Pick the rescale kernel for `mag / 10^scale`. `const fn`, so a const
/// `scale` (the `mul` door) folds the verdict to one arm and a runtime
/// `scale` (the transcendental door) is a single branch. Keyed on
/// `(scale, width_bits)`.
///
/// Routing (policy-map `newton_vs_mg`, with the §9.20 baked reciprocal table
/// making `precompute` a lookup): `scale <= 38` → single-pass `MgSingle`. For
/// the wide / high-scale rescale where the baked table applies — work width
/// `24..=132` u64 limbs AND `scale` in `200..=1850` — baked-reciprocal
/// **Newton** wins 1.5–13× over the `⌈scale/38⌉`-pass `MgChain` (the win grows
/// with scale; 200 is a conservative continuous threshold safely past every
/// per-width crossover). Everything else → `MgChain`: the narrow / `< 24`-limb
/// widths (Newton doesn't win), `scale > 1850` (beyond the baked range — there
/// `precompute` would fall back to a per-call Knuth divide, the 9.18.2 loss),
/// and the marginal `39..200` band.
#[inline]
const fn select(scale: u32, width_bits: u32) -> Algorithm {
    if scale <= 38 {
        return Algorithm::MgSingle;
    }
    let width_limbs = width_bits / 64;
    // 1850 = `consts::newton_recip::NEWTON_RECIP_MAX_SCALE` (the baked cap);
    // literal here so this always-compiled `select` doesn't depend on the
    // wide-gated const.
    if scale >= 200 && scale <= 1850 && width_limbs >= 24 && width_limbs <= 132 {
        return Algorithm::Newton;
    }
    Algorithm::MgChain
}

/// `mag /= 10^scale` in place, rounded under `mode` — the **slice door**.
/// `neg` is the result sign (rounding tie-break); `width_bits` is the work
/// width in bits (the Newton key). `scale == 0` is a no-op.
#[inline]
pub(crate) fn dispatch_mag(
    mag: &mut [u128],
    scale: u32,
    neg: bool,
    mode: RoundingMode,
    width_bits: u32,
) {
    if scale == 0 {
        return;
    }
    match select(scale, width_bits) {
        Algorithm::MgSingle => {
            crate::algos::support::mg_divide::div_pow10_mag_u128(mag, scale, neg, mode)
        }
        Algorithm::MgChain => {
            crate::algos::support::mg_divide::div_pow10_chain_mag_u128(mag, scale, neg, mode)
        }
        Algorithm::Newton => crate::algos::support::newton_reciprocal::newton_rescale_arm(
            mag, scale, neg, mode, width_bits,
        ),
    }
}

/// `n / 10^scale` for a work integer `W`, rounded under `mode` — the
/// **typed door**. Forms the u128 magnitude from `W`'s scratch carrier
/// (`W::Scratch = Limbs<N>`; size lives in the impl, no const work-width
/// parameter) and forwards to [`dispatch_mag`]. Used at the wide-
/// transcendental rounding call sites where `scale` is a runtime value.
///
/// **Magnitude-length-aware (task 9.24).** The wide-transcendental per-term
/// magnitude is far SHORTER than its work buffer: the `÷10^w` rescale of a
/// Taylor/Tang term divides a `~2·w`-digit numerator (e.g. ~52 u64 limbs at
/// working scale `w≈492`) held inside a wide work integer (D616's strict-Tang
/// work is `Int<128>` = 128 u64 limbs). Every rescale kernel's cost scales
/// with the SIGNIFICANT length, not the buffer width, so the routing key and
/// the baked-Newton apply size are taken from the trimmed significant length —
/// **not** `W::BITS`. Without this, `select` saw the full 128-limb width and
/// sized the baked-Newton reciprocal at 157 limbs (forming a full-width
/// multiply-by-reciprocal + a 128-limb `quot·10^scale` product PER term) where
/// `MgChain` scales with the real ~52-limb length — the regression `0b3df16c`
/// introduced by widening the Newton cap 96→132 and pulling `Int<128>` in. With
/// the trim, a short transcendental magnitude keeps the **Newton** kernel (the
/// owner's decision) but at its REAL width, where it genuinely wins and the
/// per-term products shrink ~2×. The baked reciprocal accessor slices the HIGH
/// limbs of the width-132 table, so any shrunk width ≤ 132 stays a lookup, never
/// the slow per-call Knuth precompute. The const-folding `mul` slice door
/// ([`dispatch_mag`] with a const width) is UNAFFECTED — only this runtime
/// typed door narrows. Bit-identical: every kernel computes the exact
/// correctly-rounded floor division regardless of the routed width.
#[inline]
pub(crate) fn dispatch_wide_pow10<W>(n: W, scale: u32, mode: RoundingMode) -> W
where
    W: BigInt,
    W::Scratch: ComputeLimbs,
{
    let mut buf = <W::Scratch as ComputeLimbs>::single_u128();
    let mag = &mut buf.as_mut()[..W::U128_LIMBS];
    let neg = n.mag_into_u128(mag);

    // Significant u128-limb length (strip the leading-zero high limbs). The
    // numerator's high limbs above the value are zero (`mag_into_u128` wrote the
    // full magnitude), and the quotient `floor(mag / 10^scale) ≤ mag` so its
    // significant length never exceeds `sig` — the high limbs stay zero and are
    // read back unchanged by `from_mag_sign_u128`.
    let mut sig = mag.len();
    while sig > 1 && mag[sig - 1] == 0 {
        sig -= 1;
    }
    // u128 limbs → bits (128 b/limb), capped at the work width so we never claim
    // more than the buffer's real bit width (odd-`N` storage rounds `U128_LIMBS`
    // up, leaving the top u128 limb only half-populated).
    let sig_bits = (sig as u32).saturating_mul(128).min(<W as BigInt>::BITS);

    dispatch_mag(&mut mag[..sig], scale, neg, mode, sig_bits);
    W::from_mag_sign_u128(mag, neg)
}
