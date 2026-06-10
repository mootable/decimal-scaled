// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `exp_strict` kernel.
//!
//! Tang 1989, "Table-driven implementation of the exponential function
//! in IEEE floating-point arithmetic" (ACM TOMS 16(4)):
//!
//! ```text
//! e^v = 2^k · e^s,            s = v − k·ln 2,           |s| ≤ ln 2 / 2
//!     = 2^k · e^(c_j) · e^δ,  c_j = j · ln 2 / M,       j ∈ [0, M)
//!                              δ  = s − c_j,            |δ| ≤ ln 2 / (2M)
//! ```
//!
//! A two-stage range reduction collapses the post-stage-1 Taylor into a
//! table multiply (`exp(c_j)` read from the baked `M`-entry
//! `exp_tang_table` consts, the indexed slot converted to the working
//! scale per lookup) plus a short Taylor on the tiny remainder `δ`. The
//! result is reassembled as `2^(k+k_adj) · table[j] · e^δ`.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): it computes only through the
//! [`WideTrigCore`] trait surface and `BigInt` arithmetic on the work
//! integer; it never calls a method on a decimal type. `policy::exp`
//! calls [`exp_tang`] *down*; the type's `exp_strict` method delegates
//! *down* through the policy. The trig hyperbolic kernels reuse
//! [`tang_exp_fixed`] directly for their shared `(e^v, e^-v)` pair.
//!
//! Collapses the four per-tier D57 (18..=22 / 45..=56), D115
//! and D153 Tang exp kernels
//! kernels into one generic over `C: WideTrigCore`, the table size `M`,
//! and the per-band reduction/narrowing flags.

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::exp_tang_table::exp_table_entry_baked;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Tang-style `e^v_w` on an already-lifted working value `v_w` (`= x ·
/// 10^w`), returned at working scale `w`. Generic over the tier `C` and
/// the table size `M`.
///
/// `INTERNAL_EXTRA` selects the large-`k` mitigation. For `k > 0` the final
/// `2^k` reassembly (a LEFT shift) amplifies the reduction residual by
/// `2^k ≈ 10^(k·log10 2)` decimal digits, so a fixed narrow guard cannot
/// cover an unbounded `k`. When `true` the whole reduction runs at an
/// extended working scale `w + extra` (`extra = ceil(k·log10 2) + 12`, sized
/// for `k > 0` only) and the result is narrowed back to `w`
/// round-to-nearest; when `false` the body runs at the caller-supplied `w`
/// (the caller absorbs the `extra` lift in its own guard, or the band's `k`
/// is small enough not to need it). For `k ≤ 0` the reassembly is an
/// error-shrinking RIGHT shift, so no lift is taken (`extra = 0`) — see the
/// body comment. This is the shared surface the trig hyperbolic kernels reuse.
#[must_use]
pub(crate) fn tang_exp_fixed<
    C: WideTrigCore,
    const M: u32,
    const INTERNAL_EXTRA: bool,
    const SCALE: u32,
>(
    v_w: C::W,
    w: u32,
) -> C::W
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    // Thin `WideTrigCore`-bound wrapper over the width-generic
    // [`tang_exp_fixed_g`]: binds the work integer to `C::W` and supplies
    // `ln 2` from `C::ln2::<SCALE>` (the crate's feature-flagged default
    // rounding mode + the per-scale const-fold). One Tang `exp` kernel — the
    // wide compositions call `tang_exp_fixed_g` directly at their `Wagm` work
    // width.
    tang_exp_fixed_g::<C::W, M, INTERNAL_EXTRA>(v_w, w, |ww| C::ln2::<SCALE>(ww))
}

/// Width-generic core of [`tang_exp_fixed`] — the Tang `exp` body over any
/// [`BigInt`] work integer `S`, reusing the unified `exp_generic` fixed-point
/// arithmetic leaves (the sibling of [`crate::algos::ln::ln_tang::tang_ln_fixed_g`]).
///
/// `ln 2` is supplied by an accessor `ln2(working_scale)` so the caller owns
/// the rounding mode (the crate's feature-flagged default — never a hardcoded
/// one); the Tang `exp` table is the already-width-generic
/// [`exp_table_entry_baked`] (binary, scale-independent). `tang_exp_fixed::<C>`
/// is the thin tier-bound wrapper; the wide compositions (`powf`/`exp2`/the
/// hyperbolics) call this directly at their `Wagm` work width.
#[must_use]
pub(crate) fn tang_exp_fixed_g<S: BigInt, const M: u32, const INTERNAL_EXTRA: bool>(
    v_w: S,
    w: u32,
    ln2: impl Fn(u32) -> S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    // Stage 0 (INTERNAL_EXTRA only): size an extended working scale
    // `w_ext = w + extra` from `|k|` so the `2^k` reassembly does not
    // amplify the reduction residual past the storage LSB. Matches the
    // dynamic-margin reduction the generic `exp_fixed` uses (Muller,
    // *Elementary Functions* 3rd ed., §11.1). `k` is scale-invariant, so
    // reuse the value computed at `w` below.
    let k = {
        let one_w = eg::one::<S>(w);
        eg::round_to_nearest_int::<S>(eg::div_cached::<S>(v_w, ln2(w), one_w), w)
    };

    let (w_ext, v_ext, extra) = if INTERNAL_EXTRA {
        // Size the extended scale from `k` only for `k > 0`: the `2^k`
        // reassembly amplifies the residual only on the LEFT shift. For `k < 0`
        // (underflow) the reassembly is an error-shrinking RIGHT shift, so the
        // base scale already suffices — and inflating `w_ext` there would drive
        // the table-entry product `slot_hi · 10^w_ext` (≈ `2·w_ext·log2(10)`
        // bits) past the work integer `S`, silently wrapping `exp(c_j)`. (Same
        // asymmetry the `EXTERNAL_EXTRA` wrapper applies.)
        let extra: u32 = if k <= 0 {
            0
        } else {
            let digits = (k as u128 * 30103).div_ceil(100_000) as u32;
            digits + 12
        };
        let v_ext = if extra == 0 {
            v_w
        } else {
            v_w * eg::pow10::<S>(extra)
        };
        (w + extra, v_ext, extra)
    } else {
        (w, v_w, 0)
    };

    // Overflow gate (up front, before any `w_ext`-scale work). The body runs at
    // the extended scale `w_ext` — `one_w = 10^w_ext` and the `2^k` reassembly
    // `exp_s << k` — so a result too large to represent needs `w_ext` digits
    // (`≈ w_ext·log2(10)` bits) PLUS the `k`-bit shift to exceed `S`. Without
    // this gate the `10^w_ext` literal alone silently WRAPS once it passes `S`'s
    // width (e.g. exp2(1005) = e^696.7: `w_ext ≈ 372` ⇒ ~1236 bits > Wagm's
    // 1024 ⇒ garbage), and the result came back as 0 instead of panicking. A
    // fixed-width decimal has no infinity: PANIC, uniform across debug AND
    // release (the strict-transcendental overflow contract). In-range results
    // fit `S` (wider than storage) with room, so this never fires for a
    // representable cell. digits→bits: `log2(10) ≈ 3322/1000`.
    {
        let peak_bits =
            (w_ext as u64) * 3322 / 1000 + if k >= 0 { k as u64 } else { 0 };
        if peak_bits >= <S as BigInt>::BITS as u64 {
            panic!("tang_exp_fixed: result out of range");
        }
    }

    let one_w = eg::one::<S>(w_ext);
    let pow10_w = one_w;
    let l2 = ln2(w_ext);

    // Stage 1: v = k·ln 2 + s, |s| ≤ ln 2 / 2.
    let k_l2 = if k >= 0 {
        l2 * eg::lit::<S>(k)
    } else {
        -(l2 * eg::lit::<S>(-k))
    };
    let s = v_ext - k_l2;

    // Stage 2: s = j_signed · (ln 2 / M) + δ, |δ| ≤ ln 2 / (2M).
    let j_signed =
        eg::round_to_nearest_int::<S>(eg::div_cached::<S>(s * eg::lit::<S>(M as i128), l2, pow10_w), w_ext);
    let cj_signed_w = if j_signed >= 0 {
        (l2 * eg::lit::<S>(j_signed)) / eg::lit::<S>(M as i128)
    } else {
        -((l2 * eg::lit::<S>(-j_signed)) / eg::lit::<S>(M as i128))
    };
    let delta = s - cj_signed_w;
    let (j_idx, k_adj) = if j_signed >= 0 {
        (j_signed as u32, 0i128)
    } else {
        ((j_signed + M as i128) as u32, -1i128)
    };
    debug_assert!(j_idx < M, "tang_exp_fixed: table index out of range");

    // Taylor on δ.
    let mut sum = one_w + delta;
    let mut term = delta;
    let mut n: u128 = 2;
    loop {
        term = eg::mul::<S>(term, delta, w_ext) / eg::lit::<S>(n as i128);
        if term == eg::zero::<S>() {
            break;
        }
        sum = sum + term;
        n += 1;
        if n > 200 {
            break;
        }
    }

    let exp_cj = exp_table_entry_baked::<S>(w_ext, j_idx as usize, M, pow10_w);
    let exp_s = eg::mul::<S>(exp_cj, sum, w_ext);

    let k_total = k + k_adj;
    let scaled_at_w_ext = if k_total >= 0 {
        let shift = k_total as u32;
        // The `2^k` reassembly `exp_s << shift` wraps past `S`'s width once the
        // result is too large to represent — a genuinely out-of-range exp. A
        // fixed-width decimal has no infinity, so PANIC, uniform across debug
        // AND release (the strict-transcendental overflow contract). This was a
        // `debug_assert!`, so a RELEASE build silently WRAPPED to garbage — e.g.
        // `exp2(1005)` (= e^696.7, far beyond every tier) returned 0 instead of
        // panicking, while `exp2(200)` (overflow that still fits `S`, panicking
        // later at the storage narrow) was correct: a tier/scale-INVARIANT
        // violation the full-surface golden surfaced.
        if eg::bit_length::<S>(exp_s) + shift >= <S as BigInt>::BITS {
            panic!("tang_exp_fixed: result out of range");
        }
        exp_s << shift
    } else {
        let neg_k = (-k_total) as u32;
        if neg_k as u128 >= eg::bit_length::<S>(exp_s) as u128 {
            eg::zero::<S>()
        } else {
            exp_s >> neg_k
        }
    };

    if !INTERNAL_EXTRA || extra == 0 {
        scaled_at_w_ext
    } else {
        // Narrow the extended-scale result back to `w` round-to-nearest
        // (ties up via the `+ half` bias). `extra` is bounded so
        // `10^extra` stays well inside the working width.
        let p = eg::pow10::<S>(extra);
        let half = p / eg::lit::<S>(2);
        if scaled_at_w_ext >= eg::zero::<S>() {
            (scaled_at_w_ext + half) / p
        } else {
            -((-scaled_at_w_ext + half) / p)
        }
    }
}

/// Tier-generic Tang-style `e^x` strict kernel.
///
/// - `C` — the per-tier [`WideTrigCore`] marker (`wide_trig_d*::Core`).
/// - `SCALE` — the decimal storage scale.
/// - `M` — the Tang table size (`128` or `512`).
/// - `GUARD` — the narrow guard for this band (`8`, `10`, or the tier's
///   canonical `30`).
/// - `DIRECTED` — route the final narrowing through the directed-rounding
///   Ziv escalation (`true`), else narrow once with
///   `round_to_storage_with` (`false`).
/// - `EXTERNAL_EXTRA` — compute the large-`|k|` working-scale lift `extra`
///   in this wrapper and fold it into the directed base guard (the D115
///   shape; requires `DIRECTED`).
/// - `INTERNAL_EXTRA` — let [`tang_exp_fixed`] do the `extra` lift +
///   narrow-back internally (the D153 shape).
#[inline]
#[must_use]
pub(crate) fn exp_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const M: u32,
    const GUARD: u32,
    const DIRECTED: bool,
    const EXTERNAL_EXTRA: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    if raw == C::storage_zero() {
        return C::storage_one(SCALE);
    }

    if !DIRECTED && crate::support::rounding::is_nearest_mode(mode) {
        // Single-shot narrowing (D57 18..=22 and 45..=56) — NEAREST modes
        // only. Reduction runs at the const-folded `w = SCALE + GUARD`; the
        // band guard keeps the working error well under half a storage ULP,
        // so a single narrow is correctly rounded to nearest. Directed modes
        // (which must decide which SIDE of a grid line the true value lies,
        // and can sit a sub-resolution residual below the work-int's
        // resolution — `exp(-10^-S)` just under `1.0` at MAX scale) fall
        // through to the never-exact Ziv path below.
        let w = SCALE + GUARD;
        let v_w = C::to_work_scaled(raw, GUARD);
        let result = tang_exp_fixed::<C, M, INTERNAL_EXTRA, SCALE>(v_w, w);
        return C::round_to_storage_with(result, w, SCALE, mode);
    }

    let base_guard = if EXTERNAL_EXTRA {
        // The final reassembly is `exp_s << k` for `k ≥ 0` and `exp_s >> |k|`
        // for `k < 0`. Only the LEFT shift (`k ≥ 0`) amplifies the
        // working-scale rounding error by `2^k` (≈ `|k|·log10 2` digits), so
        // only there must the base guard widen by `extra` to keep the
        // post-shift residual inside the guard. For `k < 0` (the underflow
        // direction — `e^(large negative)`) the reassembly is a RIGHT shift
        // that shrinks the value and its absolute error, so the base `GUARD`
        // already covers it with vast margin; inflating the guard there is not
        // only needless but HARMFUL — it drives the working scale `w = SCALE +
        // base_guard` high enough that the Tang table-entry product
        // (`exp_table_entry_baked`'s `slot_hi · 10^w`, ≈ `2·w·log2(10)` bits)
        // overflows the work integer `S`, silently wrapping the `exp(c_j)`
        // factor (the deep-underflow misround at the wide tiers' max scale).
        // So size `extra` from `k` only when `k > 0`.
        let w = SCALE + GUARD;
        let one_w = C::one(w);
        let v_w_probe = C::to_work_scaled(raw, GUARD);
        let k = C::round_to_nearest_int(C::div_cached(v_w_probe, C::ln2::<SCALE>(w), one_w), w);
        let extra: u32 = if k <= 0 {
            0
        } else {
            let abs_k = k as u128;
            let digits = (abs_k * 30103).div_ceil(100_000);
            let capped = digits.min((C::w_bits() / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };
        GUARD + extra
    } else {
        GUARD
    };

    // Directed modes decide which side of a storage grid line the true
    // result falls; near a grid line the working-scale approximation can
    // land on the wrong side, so route through the shared Ziv escalation.
    // Nearest modes narrow once. `exp(x)` for `x != 0` is transcendental
    // (never exactly on a grid line — `raw == 0` is pinned above), so use the
    // never-exact narrowing: a zero working residual is a sub-resolution
    // artifact, and Ceiling must still round up (Floor / Trunc keep the floor)
    // on inputs whose deciding residual is below the work-int resolution
    // (`exp(-10^-S)` just under `1.0`).
    C::round_to_storage_directed_never_exact(base_guard, SCALE, mode, &mut |guard| {
        tang_exp_fixed::<C, M, INTERNAL_EXTRA, SCALE>(C::to_work_scaled(raw, guard), SCALE + guard)
    })
}

#[cfg(all(test, feature = "wide"))]
mod tests {
    //! Deep-underflow correctness for the Tang `exp` path.
    //!
    //! Regression for the wide-tier max-scale misround: at D76<75> the Tang
    //! `wide_tang_gate` admits large negative arguments (`e^(−34..−58)`, all
    //! representable), but the old `EXTERNAL_EXTRA` guard inflated the working
    //! scale `w` by `≈ |k|·log10 2` digits — even though the `k < 0`
    //! reassembly is an error-shrinking RIGHT shift that needs no such guard.
    //! That pushed the table-entry product `slot_hi · 10^w` past the tier work
    //! integer `Int<16>` (1024 bits), silently wrapping the `exp(c_j)` factor
    //! (~25 % error). The wider D307<75> tier runs the Series path and is the
    //! oracle: `exp` rounded to scale 75 is the same value at every storage
    //! width, so the two decimal renderings must be identical.

    use crate::types::widths::{D307, D76};
    use crate::RoundingMode;

    const MODES: [RoundingMode; 6] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ];

    #[test]
    fn tang_deep_underflow_matches_wide_series_oracle_d76_s75() {
        // Negative args spanning the underflow regime the Tang gate routes —
        // from below the overflow boundary (~−33) up to the storage edge
        // (max |x| ≈ 57.9 at D76<75>).
        let args = ["-20.0", "-33.5", "-34.37", "-40.0", "-45.123", "-50.25", "-55.0", "-57.5"];
        for s in args {
            let a76: D76<75> = s.parse().unwrap();
            let a307: D307<75> = s.parse().unwrap();
            for m in MODES {
                let got = a76.exp_strict_with(m).to_string();
                let want = a307.exp_strict_with(m).to_string();
                assert_eq!(got, want, "exp({s}) D76<75> vs D307<75> oracle, mode {m:?}");
            }
        }
    }
}
