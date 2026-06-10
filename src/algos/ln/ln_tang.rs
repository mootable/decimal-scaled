// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `ln_strict` kernel.
//!
//! Tang 1990, "Table-driven implementation of the logarithm function in
//! IEEE floating-point arithmetic" (ACM TOMS 16(4) 378-400).
//!
//! ## Algorithm
//!
//! ```text
//! v = 2^k · m,                m ∈ [1, 2)
//! i = floor((m - 1) · M),     i ∈ [0, M)
//! f_i = 1 + i / M             (table-indexed boundary)
//! L_i = ln(f_i)               (table entry)
//! t = (m - f_i) / (m + f_i)   (|t| < 1 / (2M + 1))
//! ln(m) = L_i + 2 · artanh(t) (= L_i + ln((1 + t) / (1 - t)) the
//!                              identity reformulated as a series)
//! ln(v) = k · ln(2) + ln(m)
//! ```
//!
//! With `M = 128` the residual `|t| < 1/257 ≈ 3.9·10⁻³`, so
//! `|t²| < 1.5·10⁻⁵`; the artanh series `2·(t + t³/3 + t⁵/5 + ...)`
//! converges fast. The table `ln(1 + i/M)` is baked as binary
//! fixed-point consts (`ln_tang_table`); each lookup converts the one
//! indexed slot to the working scale (`round(slot · 10^w / 2^B)`) —
//! no runtime table build.
//!
//! ## Layering
//!
//! This is an **algorithm function** (`docs/ARCHITECTURE.md` →
//! "Layering direction"): it computes only through the
//! [`WideTrigCore`] trait surface (which forwards *down* into the
//! per-tier guard-digit kernels) and `BigInt` arithmetic on the work
//! integer. It never calls a method on a decimal type. The thirteen
//! `policy::ln` Tang arms call *down* to [`ln_tang`]; the type's
//! `ln_strict` method delegates *down* through the policy.
//!
//! This collapses the thirteen per-tier Tang `ln_strict`
//! kernels — structurally identical bar the `core` module
//! (`wide_trig_d*`), the storage `Int<N>`, the narrow guard
//! (`GUARD = 8` or `10`) and the artanh-series iteration cap — into one
//! generic over `C: WideTrigCore`.

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::ln_tang_table::ln_table_entry_baked;
use crate::algos::support::wide_trig_core::WideTrigCore;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// Table size — number of `ln(1 + i/M)` entries per working scale. The
/// `i = 0` slot is `ln(1) = 0`, the `i = M` slot is `ln(2)`. Every
/// shipped tier uses `M = 128`.
const M: u32 = 128;

/// Working-scale lift folded into [`ln_tang`]'s `GUARD` when
/// `EXTERNAL_EXTRA` is `true`. Buries the artanh-series truncation
/// bias (≈ one working-ULP, one-sided — see [`tang_ln_fixed`]) below
/// the storage ULP so directed-mode rounding lands on the correct side
/// of any near-grid-line input (e.g. `ln(1+√2) = asinh(±1)` at MAX
/// storage scale, where the outer Ziv loop's cap collapses to the base
/// guard and the kernel must come in pre-widened).
///
/// Sized at `12` matching `tang_exp_fixed`'s `INTERNAL_EXTRA` safety
/// margin — the bias is one working-ULP regardless of working scale,
/// so a fixed lift suffices.
pub(crate) const EXTERNAL_EXTRA_DIGITS: u32 = 12;

/// Tang-style `ln(v)` for a working-scale value `v_w` (`= x · 10^w`),
/// returned at the same working scale `w`. Generic over the tier `C`,
/// the artanh-series iteration cap `CAP` (a safety net; the loop
/// terminates on a zero term far sooner), and the `INTERNAL_EXTRA`
/// directed-mode boundary-precision flag.
///
/// This is the working-scale Tang `ln` shared surface — the analogue of
/// [`crate::algos::exp::exp_tang::tang_exp_fixed`]. The narrow-ln-strict
/// kernel ([`ln_tang`]) wraps it with the Ziv-escalated storage narrowing;
/// `powf_strict` composes it with `tang_exp_fixed` directly at working
/// scale (skipping a double round-to-storage).
///
/// ## Accuracy — the artanh truncation bias
///
/// The artanh series is truncated when `contrib = term / (2j+1)`
/// underflows to zero in the work integer. The omitted tail
/// `T = sum_{k>=J} t^(2k+3)/(2k+3)` carries the **sign of `t`** and a
/// magnitude bounded by ~1 working-ULP (the largest still-representable
/// term). Two doublings (`sum + sum` and `2 · artanh`) plus
/// kernel-internal `div_cached` / `mul` rounding push that into a
/// one-sided residual error of order one working-ULP at scale `w`. On
/// near-grid-line inputs (e.g. `ln(1+√2)` and `ln(1 − 10^-S)` at MAX
/// storage scale, where the outer Ziv loop's recompute cap collapses
/// to the base guard) the bias can flip the directed-rounding residual
/// sign and land the storage result one LSB off under Trunc/Floor/
/// Ceiling — though nearest is correctly rounded.
///
/// `INTERNAL_EXTRA = true` runs the whole body at an extended working
/// scale `w + EXTERNAL_EXTRA_DIGITS` and **truncates toward zero**
/// back to `w`, **with a magnitude bump of 1 LSB-at-w if any digits
/// were discarded**. Truncation preserves the residual SIGN at scale
/// `w` (the discarded digits share the sign of the value), and the
/// bump signals "sub-w-scale residual present" to the outer directed
/// rounder (the `+1` is at most one ULP-at-`w`, i.e. `10^-guard`
/// ULP-at-storage — well below half a storage ULP, so nearest stays
/// correctly rounded). This is the residual-preserving cousin of
/// `tang_exp_fixed`'s half-up narrow-back, the difference being that
/// ln has a two-sided residual whereas exp's caller pins the sign via
/// `round_to_storage_directed_never_exact`.
///
/// `INTERNAL_EXTRA = false` runs natively at `w` (legacy path; the
/// outer Ziv-escalation loop in [`ln_tang`] handles the bias by
/// growing `guard` when the cap leaves room). Set `false` only for
/// callers that already widen their `w` externally.
#[inline]
pub(crate) fn tang_ln_fixed<
    C: WideTrigCore,
    const CAP: u128,
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
    // [`tang_ln_fixed_g`]: binds the work integer to `C::W` and supplies
    // `ln 2` from the tier's `C::ln2::<SCALE>` (which carries the crate's
    // feature-flagged default rounding mode AND the per-scale const-fold).
    // One Tang `ln` kernel — the wide compositions call `tang_ln_fixed_g`
    // directly at their `Wagm` work width.
    tang_ln_fixed_g::<C::W, CAP, INTERNAL_EXTRA>(v_w, w, |ww| C::ln2::<SCALE>(ww))
}

/// Width-generic core of [`tang_ln_fixed`] — the Tang `ln` body over any
/// [`BigInt`] work integer `S`, reusing the unified `exp_generic` fixed-point
/// arithmetic leaves so there is no per-tier copy of the kernel.
///
/// `ln 2` is supplied by an accessor `ln2(working_scale)` rather than computed
/// here, so the caller owns the rounding mode (the crate's feature-flagged
/// default — never a hardcoded one) and any const-fold. The Tang `ln` table is
/// the already-width-generic [`ln_table_entry_baked`] (a binary,
/// scale-independent lookup). `tang_ln_fixed::<C>` is the thin tier-bound
/// wrapper; the wide compositions (`log`/`log2`/`log10`/`powf`/…) call this
/// directly at their `Wagm` work width.
#[inline]
pub(crate) fn tang_ln_fixed_g<S: BigInt, const CAP: u128, const INTERNAL_EXTRA: bool>(
    v_w: S,
    w: u32,
    ln2: impl Fn(u32) -> S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    // Stage 0 (INTERNAL_EXTRA only): widen the internal working scale
    // by `extra = EXTERNAL_EXTRA_DIGITS` so the artanh-series
    // truncation bias (one-sided, ≈ 1 working-ULP) sits 12 decimal
    // digits below the caller's working ULP. The input is re-lifted
    // from `w` to `w_ext` by multiplying by `10^extra`.
    let (w_ext, v_ext, extra): (u32, S, u32) = if INTERNAL_EXTRA {
        let extra = EXTERNAL_EXTRA_DIGITS;
        let v_ext = v_w * eg::pow10::<S>(extra);
        (w + extra, v_ext, extra)
    } else {
        (w, v_w, 0)
    };

    let one_w = eg::one::<S>(w_ext);
    let pow10_w = one_w;
    let two_w = one_w + one_w;

    // Stage 1: v = 2^k · m, m ∈ [1, 2). k from bit-shifts.
    let mut k: i32 = eg::bit_length::<S>(v_ext) as i32 - eg::bit_length::<S>(one_w) as i32;
    let m_w = loop {
        let m = if k >= 0 {
            v_ext >> (k as u32)
        } else {
            v_ext << ((-k) as u32)
        };
        if m >= two_w {
            k += 1;
        } else if m < one_w {
            k -= 1;
        } else {
            break m;
        }
    };

    // Stage 2: pick i. Boundary `m = 1` short-circuits: ln(m) = 0, so
    // ln(v) = k · ln(2).
    let result_at_w_ext = if m_w == one_w {
        if k >= 0 {
            ln2(w_ext) * eg::lit::<S>(k as i128)
        } else if k < 0 {
            -(ln2(w_ext) * eg::lit::<S>((-k) as i128))
        } else {
            eg::zero::<S>()
        }
    } else {
        // i ∈ [0, M); when m = 2 exactly (rare boundary post-rounding),
        // clamp to M-1 so the table lookup stays in range, then the
        // residual t handles the remaining tiny piece.
        let i_raw = ((m_w - one_w) * eg::lit::<S>(M as i128)) / one_w;
        let i_i128 = BigInt::to_i128(i_raw);
        let i_idx = if i_i128 >= M as i128 {
            (M - 1) as usize
        } else {
            i_i128 as usize
        };

        let f_i = one_w + (one_w * eg::lit::<S>(i_idx as i128)) / eg::lit::<S>(M as i128);

        // Stage 3: t = (m - f_i) / (m + f_i). |t| < 1/(2M + 1).
        let t = eg::div_cached::<S>(m_w - f_i, m_w + f_i, pow10_w);

        // Artanh series: 2 · (t + t³/3 + t⁵/5 + ...).
        let t2 = eg::mul::<S>(t, t, w_ext);
        let mut sum = t;
        let mut term = t;
        let mut j: u128 = 1;
        loop {
            term = eg::mul::<S>(term, t2, w_ext);
            let contrib = term / eg::lit::<S>((2 * j + 1) as i128);
            if contrib == eg::zero::<S>() {
                break;
            }
            sum = sum + contrib;
            j += 1;
            if j > CAP {
                break;
            }
        }
        let ln_m = sum + sum + ln_table_entry_baked::<S>(w_ext, i_idx, pow10_w);

        // Final: ln(v) = k · ln(2) + ln(m).
        let k_ln2 = if k >= 0 {
            ln2(w_ext) * eg::lit::<S>(k as i128)
        } else {
            -(ln2(w_ext) * eg::lit::<S>((-k) as i128))
        };
        k_ln2 + ln_m
    };

    if !INTERNAL_EXTRA || extra == 0 {
        result_at_w_ext
    } else {
        // Truncate toward zero from scale `w_ext` to scale `w`, then
        // bump the magnitude by 1 LSB-at-`w` IF any digits were
        // discarded (`r_mag != 0`). The bump signals to the outer
        // directed rounder "sub-w-scale residual present, same sign as
        // the value" — preserving the residual sign at scale `w` even
        // when truncation alone would round the residual to exactly
        // zero. The `+1` is at most one ULP-at-`w`, i.e. `10^-guard`
        // storage ULPs (well below half a storage ULP), so nearest
        // stays correctly rounded.
        //
        // Sign-preservation argument: the discarded digits
        // `result_at_w_ext mod p` share the sign of `result_at_w_ext`
        // (Rust integer truncation toward zero), so the bumped
        // magnitude `q + 1` straddles the true value on the "outside"
        // (in magnitude), which is exactly what a directed rounder
        // needs to decide whether to bump under each mode.
        let p = eg::pow10::<S>(extra);
        let (q_signed, has_residue) = if result_at_w_ext >= eg::zero::<S>() {
            let q = result_at_w_ext / p;
            let has = result_at_w_ext - q * p != eg::zero::<S>();
            (q, has)
        } else {
            let abs_v = -result_at_w_ext;
            let q = abs_v / p;
            let has = abs_v - q * p != eg::zero::<S>();
            (-q, has)
        };
        if has_residue {
            // Bump magnitude by 1 LSB-at-`w` so the outer rounder sees
            // a nonzero residual with the value's sign.
            if q_signed >= eg::zero::<S>() {
                q_signed + eg::lit::<S>(1)
            } else {
                q_signed - eg::lit::<S>(1)
            }
        } else {
            q_signed
        }
    }
}

/// Tier-generic Tang-style `ln(x)` strict kernel. Panics if `raw <= 0`.
///
/// - `C` — the per-tier [`WideTrigCore`] marker (`wide_trig_d*::Core`).
/// - `SCALE` — the decimal storage scale.
/// - `GUARD` — the narrow guard for this band (`8` or `10`).
/// - `CAP` — the artanh-series iteration safety cap.
/// - `DIRECTED` — `true` routes the final narrowing through the shared
///   directed-rounding Ziv escalation (the dominant shape, used by every
///   band where a near-grid-line directed decision can flip a storage
///   LSB); `false` narrows once with `round_to_storage_with` (the
///   D57<18..=22> band, whose original kernel rounded the working
///   approximation directly).
/// - `INTERNAL_EXTRA` — forwarded to [`tang_ln_fixed`]. When `true` the
///   kernel runs internally at scale `w + EXTERNAL_EXTRA_DIGITS` and
///   narrows back via truncation-with-residual-bump, hiding the
///   artanh truncation bias below the caller's working ULP. Required
///   for directed-mode correctness at MAX storage scale (the outer
///   Ziv loop's cap collapses to the base guard there, so the kernel
///   cannot rely on escalation) and on every near-grid-line directed
///   input (e.g. `ln(1+√2) = asinh(±1)`, `ln(1 − 10^-S)`). Mirrors
///   the analogous flag on `exp_tang`.
#[inline]
#[must_use]
pub(crate) fn ln_tang_g<
    C: WideTrigCore,
    Wk: BigInt,
    const SCALE: u32,
    const GUARD: u32,
    const CAP: u128,
    const DIRECTED: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_g, round_to_storage_with_g, to_work_scaled_g,
    };
    use crate::support::rounding::DEFAULT_ROUNDING_MODE;

    if raw <= C::storage_zero() {
        panic!("wide-tier ln: argument must be positive");
    }
    // `ln 2` at the RUNG work integer `Wk`, const-folded at the base working
    // scale `SCALE + GUARD` (the hot path) — the rung sibling of the per-tier
    // `C::ln2::<SCALE>` (value-identical; only the const-fold seam differs).
    let ln2 = |ww: u32| -> Wk {
        if ww == SCALE + GUARD {
            crate::consts::ln2_by_scale::<Wk>(SCALE + GUARD, DEFAULT_ROUNDING_MODE)
        } else {
            crate::consts::ln2_by_working_scale::<Wk>(ww, DEFAULT_ROUNDING_MODE)
        }
    };
    if DIRECTED {
        // Directed modes decide which side of a storage grid line the true
        // value falls; near a grid line the working-scale approximation can
        // land on the wrong side. Route through the shared Ziv escalation.
        //
        // `INTERNAL_EXTRA` buries the ~1-working-ULP artanh truncation bias
        // below the storage ULP (it runs the body at `w + EXTERNAL_EXTRA_DIGITS`,
        // ~5× the cost). That bias only flips a directed result NEAR x = 1,
        // where ln(x) ≈ x−1 is tiny and the deciding residual sits at the
        // precision boundary (the loss region is ε ~ 10^(−SCALE/2)); for x away
        // from 1 the result carries full significant digits and the bias is far
        // below half a storage ULP. So we VALUE-GATE the wide path: pay it ONLY
        // for near-1 inputs (`|x − 1| < 10^(−SCALE/4)`, comfortably covering the
        // loss region with margin) — every other input takes the fast
        // native-width path. `adjust_ln_near_one` (below) is itself value-gated
        // (`result == δ`), so the truly-unreachable ε case is handled regardless.
        let use_extra = INTERNAL_EXTRA && {
            let one = C::storage_one(SCALE);
            let diff = if raw >= one { raw - one } else { one - raw };
            // near 1 iff |x − 1| < ~10^(−SCALE/4), tested on the bit-length so
            // the threshold is a compile-time const and the check is O(limbs)
            // (no per-call `pow`): `bit_length(10^k) ≈ k·log2(10)`, and ×3 is a
            // conservative `log2(10)`. Covers the ε ~ 10^(−SCALE/2) loss region
            // with wide margin; excludes ordinary operands (e.g. x = 1.5).
            diff.bit_length() < (SCALE - SCALE / 4) * 3
        };
        let r = if use_extra {
            round_to_storage_directed_g::<C::Storage, Wk>(
                GUARD, SCALE, mode, C::storage_max(), C::storage_min(),
                |guard| {
                    tang_ln_fixed_g::<Wk, CAP, true>(
                        to_work_scaled_g::<C::Storage, Wk>(raw, guard), SCALE + guard, ln2,
                    )
                },
            )
        } else {
            round_to_storage_directed_g::<C::Storage, Wk>(
                GUARD, SCALE, mode, C::storage_max(), C::storage_min(),
                |guard| {
                    tang_ln_fixed_g::<Wk, CAP, false>(
                        to_work_scaled_g::<C::Storage, Wk>(raw, guard), SCALE + guard, ln2,
                    )
                },
            )
        };
        crate::algos::support::wide_trig_core::adjust_ln_near_one::<C, SCALE>(r, raw, mode)
    } else {
        let w = SCALE + GUARD;
        let r = tang_ln_fixed_g::<Wk, CAP, INTERNAL_EXTRA>(
            to_work_scaled_g::<C::Storage, Wk>(raw, GUARD), w, ln2,
        );
        round_to_storage_with_g::<C::Storage, Wk>(
            r, w, SCALE, mode, C::storage_max(), C::storage_min(),
        )
    }
}

/// `ln_tang` = the `Wk = C::W` instantiation of [`ln_tang_g`] — the work-rung
/// kernel at the tier's full primitive work width. This thin alias keeps every
/// existing `policy::ln` call site unchanged; the work-width campaign routes
/// narrower rungs through [`ln_tang_g`] directly (`policy::ln::tang_with_rung`).
#[inline]
#[must_use]
pub(crate) fn ln_tang<
    C: WideTrigCore,
    const SCALE: u32,
    const GUARD: u32,
    const CAP: u128,
    const DIRECTED: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: C::Storage,
    mode: RoundingMode,
) -> C::Storage
where
    <C::W as BigInt>::Scratch: ComputeLimbs,
{
    ln_tang_g::<C, C::W, SCALE, GUARD, CAP, DIRECTED, INTERNAL_EXTRA>(raw, mode)
}
