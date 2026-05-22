//! Bespoke `exp_strict` kernel slot for `D57<SCALE>` with `SCALE ∈ 45..=56`.
//!
//! At deep storage scales the wide-tier `exp_fixed` runs the Taylor
//! series and `~√p` post-squarings against an `Int192` × `Int192 →
//! Int384` `mul` whose Knuth divide grows with the working scale
//! `w = SCALE + GUARD = 75..=87`. Two-stage range reduction collapses
//! the post-squaring loop into a single table multiply:
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! with `M = 512` (see the `M` const for the tuning rationale) so
//! `|δ| ≤ ln 2 / 1024 ≈ 6.8·10⁻⁴`. Taylor on δ then converges in
//! ~15 terms at `w = SCALE + 30 ≤ 87`, vs the ~16 post-Taylor
//! squarings the generic `exp_fixed` path runs.
//!
//! The slot is exposed through [`crate::policy::exp::ExpPolicy`] only
//! for `SCALE ∈ 45..=56`; lower scales keep using the generic
//! [`super::wide_kernel::exp_strict_d57`] which is already cheap there.
//!
//! ## Correctness
//!
//! Error budget at working scale `w` (in LSB-of-`w`):
//!
//! - 2× `(div, mul)` for range + sub-range reduction: ≤ 2 LSB
//! - One table multiply (`mul_cached`): ≤ 0.5 LSB
//! - Taylor on `δ`: ~25 rounded muls → ≤ 12.5 LSB
//!
//! Total ≤ ~15 LSB-of-`w` = ~15·10⁻³⁰ at storage scale. The strict
//! contract requires ≤ 0.5 LSB-of-storage = 0.5·10⁻ᴿᴱ — a margin of
//! 28+ orders of magnitude even at `SCALE = 57`.

#![cfg(any(feature = "d57", feature = "wide"))]

use crate::support::rounding::RoundingMode;
use crate::types::widths::wide_trig_d57 as core;
use crate::wide_int::Int192;

/// Table size — number of `exp(j · ln(2) / M)` entries per working
/// scale. Power of two so the index quantisation step `ln(2) / M`
/// keeps the cheap integer-division path. Larger M shrinks the
/// post-table Taylor remainder `|δ| ≤ ln(2) / (2M)` and so shaves
/// Taylor iterations.
///
/// Tuned empirically against `examples/d57_exp_scale_range_bench`:
/// the curve is monotonic (bigger table → faster hot-path) but with
/// diminishing returns past ~M = 512, and the one-off per-thread
/// table seed costs `M · exp_fixed(w)` calls — roughly `M · 17 µs`
/// at `SCALE = 57`. `M = 512` strikes the balance: 4-6 µs faster
/// per call on the hot path against ~9 ms one-time cold-start cost,
/// vs. M = 4096's ~8 µs / call wins at ~70 ms cold-start.
///
/// Per-thread memory cost: `M · sizeof(W) = M · 128 B` (Int1024 for
/// the D57 wide-tier transcendental core), so 64 KB at M = 512.
const M: u32 = 512;

crate::policy::table_cache::decl_table_cache!(entry = core::W, compute = compute_table);

/// Build the `exp(j · ln 2 / M)` table at working scale `w` using the
/// canonical `exp_fixed` kernel (one call per slot, paid once per
/// thread per `w`).
fn compute_table(w: u32) -> alloc::vec::Vec<core::W> {
    let mut out = alloc::vec::Vec::with_capacity(M as usize);
    let l2 = core::ln2(w);
    out.push(core::one(w)); // j = 0: exp(0) = 1.
    for j in 1..M {
        // c_j = j · ln(2) / M, computed at working scale.
        // ln(2) is at scale w; divide by M (small) to get c_j at scale w.
        let cj_w = (l2 * core::lit(j as u128)) / core::lit(M as u128);
        out.push(core::exp_fixed(cj_w, w));
    }
    out
}

/// `e^x` strict kernel for `D57<SCALE>` with `SCALE ∈ 45..=56`.
///
/// Two-stage range reduction:
/// 1. `k = round(x / ln 2)`, `s = x − k·ln 2`, `|s| ≤ ln 2 / 2`.
/// 2. `j = round(s · M / ln 2)`, `δ = s − j · ln 2 / M`,
///    `|δ| ≤ ln 2 / (2M)`.
///
/// Then `e^x = 2^k · table[j] · e^δ`, with `e^δ` evaluated by Taylor
/// on the very small remainder (~15 terms at `w = SCALE + 30` for
/// `M = 512`).
#[inline]
#[must_use]
pub(crate) fn exp_strict<const SCALE: u32>(raw: Int192, mode: RoundingMode) -> Int192 {
    // exp(0) = 1 short-circuit (matches generic wide_kernel).
    if raw == Int192::ZERO {
        // D57::<SCALE>::ONE raw is 10^SCALE in storage units.
        let ten: Int192 = crate::wide_int::wide_cast::<u128, Int192>(10);
        return ten.pow(SCALE);
    }

    let w = SCALE + core::GUARD;
    let v_w = core::to_work(raw);
    let one_w = core::one(w);
    let pow10_w = one_w;
    let l2 = core::ln2(w);

    // Stage 1: v = k·ln 2 + s, |s| ≤ ln 2 / 2.
    let k = core::round_to_nearest_int(core::div_cached(v_w, l2, pow10_w), w);
    // `core::scale_by_k` is module-private; inline the same logic:
    // signed multiply `ln2 · k` (k is i128).
    let k_l2 = if k >= 0 {
        l2 * core::lit(k as u128)
    } else {
        -(l2 * core::lit((-k) as u128))
    };
    let s = v_w - k_l2;

    // Stage 2: s = j_signed · (ln 2 / M) + δ, |δ| ≤ ln 2 / (2M).
    // j_signed ∈ [-M/2, M/2]. The lookup table is keyed on a non-
    // negative index, so when j_signed < 0 we use the identity
    // `exp(c_j_signed) = exp(c_{j_signed + M}) · 2^(-1)` — i.e. take
    // `j_idx = j_signed + M`, fold the factor-of-½ into the final
    // `2^k` shift as `k_adj = -1`. δ is always computed against the
    // *signed* offset `c_{j_signed}` so it stays in the tight band.
    let j_signed =
        core::round_to_nearest_int(core::div_cached(s * core::lit(M as u128), l2, pow10_w), w);
    let cj_signed_w = if j_signed >= 0 {
        (l2 * core::lit(j_signed as u128)) / core::lit(M as u128)
    } else {
        -((l2 * core::lit((-j_signed) as u128)) / core::lit(M as u128))
    };
    let delta = s - cj_signed_w;
    let (j_idx, k_adj) = if j_signed >= 0 {
        (j_signed as u32, 0i128)
    } else {
        ((j_signed + M as i128) as u32, -1i128)
    };
    debug_assert!(
        j_idx < M,
        "exp_strict d57 s45..=56: table index out of range"
    );

    // Taylor: exp(δ) = 1 + δ + δ²/2! + … on |δ| ≤ ln(2)/(2M) ≈ 6.8·10⁻⁴
    // for M = 512. Term n shrinks as δⁿ / n!: at n ≈ 15 the contribution
    // falls below 10⁻⁹⁰, comfortably below the 1 LSB-of-w cutoff for
    // w ≤ 87, so the loop exits on the zero-term break in ~15 iterations.
    let mut sum = one_w + delta;
    let mut term = delta;
    let mut n: u128 = 2;
    loop {
        term = core::mul_cached(term, delta, pow10_w) / core::lit(n);
        if term == core::zero() {
            break;
        }
        sum = sum + term;
        n += 1;
        if n > 200 {
            break;
        }
    }

    // exp(s) = table[j_idx] · sum at working scale.
    let exp_cj = table_entry(w, j_idx as usize);
    let exp_s = core::mul_cached(exp_cj, sum, pow10_w);

    // Reassemble: exp(v) = 2^(k + k_adj) · exp(s).
    let k_total = k + k_adj;
    let result = if k_total >= 0 {
        let shift = k_total as u32;
        // Bit-length guard; matches wide_kernel::exp_fixed.
        debug_assert!(
            core::bit_length(exp_s) + shift < core::W::BITS,
            "exp_strict d57 s45..=56: result overflows the representable range",
        );
        exp_s << shift
    } else {
        let neg_k = (-k_total) as u32;
        if neg_k as u128 >= core::bit_length(exp_s) as u128 {
            core::zero()
        } else {
            exp_s >> neg_k
        }
    };

    core::round_to_storage_with(result, w, SCALE, mode)
}
