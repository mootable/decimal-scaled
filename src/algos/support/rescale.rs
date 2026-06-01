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
//!   ([`mg_divide::div_pow10_chain_mag_u128`]), for `scale > 38`.
//! * [`Algorithm::Newton`] — the uncached Newton-reciprocal rescale
//!   ([`newton_reciprocal::newton_rescale_arm`]) — a **kept-alt** (see the
//!   calibration note below).
//!
//! [`select`] is a `const fn` keyed on `(scale, width_bits)`, so the two
//! entry doors share ONE classifier (the const/slice dual-door of
//! `docs/ARCHITECTURE.md`):
//!
//! * [`dispatch_mag`] — the **slice door**: rescales a `&mut [u128]`
//!   magnitude in place. Called by the `Int<N>` decimal `mul`
//!   ([`crate::algos::mul::mul_widen_divide`]) with a **const `SCALE`**, so
//!   `select` const-folds the verdict to a single kernel call per
//!   monomorphisation.
//! * [`dispatch_wide_pow10`] — the **typed door**: rescales a work integer
//!   `W`. Called by the wide-transcendental rounding (the `wide_transcendental`
//!   cores, `lib.rs::round_div_pow10_fast`) with a **runtime `scale`** from
//!   the Taylor loops, so `select` is one runtime branch.
//!
//! ## Calibration — why Newton is a kept-alt, not selected
//!
//! The §9.2 no-state rule deleted the reciprocal cache the Newton rescale
//! amortised against, so the uncached kernel now recomputes its
//! `⌊2^k / 10^scale⌋` reciprocal (a Knuth divide) on EVERY call. Measured
//! (9.18.2, decimal `mul` slow path): that per-call precompute makes Newton
//! ~2× slower than `MgChain` at the cells the old bespoke predicate routed to
//! it — D230<229> 2.15×, D924<462> 2.0× — and forcing `MgChain` recovers both
//! to parity with 0.4.4 (which used the MG chain). The asymptotics agree: a
//! one-shot `⌊2^k/10^scale⌋` is O(k·pow_len) by Knuth but the apply needs the
//! reciprocal anyway, so uncached Newton is `MgChain` + a precompute and
//! cannot win. So [`select`] routes `scale > 38` to `MgChain`. The Newton
//! kernel stays callable (the `Newton` arm) but is unselected; reviving it
//! needs a baked-`r` const table, whose binary-size cost is deferred
//! (the 9.20 shrink goal — a separate owner call).

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
#[inline]
pub(crate) fn dispatch_wide_pow10<W>(n: W, scale: u32, mode: RoundingMode) -> W
where
    W: BigInt,
    W::Scratch: ComputeLimbs,
{
    let bits = <W as BigInt>::BITS;
    let mut buf = <W::Scratch as ComputeLimbs>::single_u128();
    let mag = &mut buf.as_mut()[..W::U128_LIMBS];
    let neg = n.mag_into_u128(mag);
    dispatch_mag(mag, scale, neg, mode, bits);
    W::from_mag_sign_u128(mag, neg)
}
