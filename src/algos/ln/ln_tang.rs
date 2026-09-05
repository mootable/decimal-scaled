// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tier-generic Tang-style table-driven `ln` kernel.
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
//! `ln` method delegates *down* through the policy.
//!
//! This collapses the thirteen per-tier Tang `ln`
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
///
/// # `M` sets the artanh term count — and hence the `CAP` a tier needs
///
/// `M` is not only a table size: it bounds the residual `t`, which fixes how
/// many artanh terms the series in [`tang_ln_fixed_g`] must run, which is
/// what a tier's `CAP` has to clear. The chain, and the arithmetic to redo
/// when adding a tier or raising a max scale:
///
/// 1. Reduction gives `m ∈ [f_i, f_{i+1})` with `f_{i+1} − f_i = 1/M`, so
///    `|m − f_i| < 1/M` and `m + f_i ≥ 2`. Hence
///    **`|t| < 1/(2M) = 1/256`** at `M = 128`. (The module header quotes the
///    tighter `1/(2M + 1) = 1/257`, from `m + f_i > 2 + 1/M`. Both hold; this
///    bound deliberately uses the LOOSER `1/256`, because a larger `|t|` means
///    MORE terms, which is the conservative side for a cap.)
/// 2. Term `j` of `2·(t + t³/3 + …)` has magnitude `≈ |t|^(2j+1)`. At working
///    scale `w` the loop exits when that underflows the work integer's unit,
///    i.e. when `|t|^(2j+1) < 10^-w`.
/// 3. Taking logs with the worst case `|t| = 1/256`
///    (`log10 256 ≈ 2.408`): the loop needs
///    **`2j + 1 > w / 2.408`**, i.e. about `w / 4.8` iterations.
/// 4. So a cap of `C` iterations covers working scales up to
///
///    ```text
///    w ≤ (2C + 1) · 2.408
///    ```
///
/// `w` must be taken at its LARGEST: the tier's max scale plus `GUARD`, plus
/// whatever the directed-Ziv escalation adds, which is itself bounded by the
/// rung's `BITS/8`. Per-tier check (2026-09-03):
///
/// | tier | `CAP` | covers `w ≤` | max `w` | `j` needed | verdict |
/// |---|---:|---:|---:|---:|---|
/// | D57 | 100 | 482 | 64 (Ziv ≤ 120) | 13 | ample |
/// | D115 | 200 | 966 | 122 (Ziv ≤ 184) | 25 | ample |
/// | D153 | 200 | 966 | 162 (Ziv ≤ 248) | 34 | ample |
/// | D76, D230…D1232 | 400 | 1929 | ≤ 1241 (Ziv ≤ 1400) | ≤ 258 | ample |
///
/// The widest cell is `D1232<1231>`: `w = 1241`, `j ≈ 258`, against `CAP = 400`.
/// Every shipped tier clears its bound with room.
///
/// **A `CAP` that bites produces WRONG DIGITS, not slow ones** — the series is
/// simply truncated. That is a validity wall, not a safety net, and it is
/// live: `benches/micro/ln_wide_series_tang_ab.rs` races a `CAP = 200`
/// candidate at every tier, and at `D1232<1231>` (`w = 1239`, past 966) the
/// bench's validity wall catches it —
/// `tang_g8_c200 != series (x_lo, HalfToEven)`. That candidate is wired
/// nowhere; the wall rejected it exactly as intended.
///
/// Note this only became detectable once that map moved to non-degenerate
/// operands: under the old `{0.5, 2.0, 7.5}` spread the artanh loop ran zero
/// iterations at every cell, so no `CAP` could ever bite. Lowering `M` raises
/// `|t|` and therefore raises the `CAP` every tier needs — re-run step 3.
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

/// Tang-style `ln(v)` for a `working_value` (`= x · 10^w`),
/// returned at the same `working_scale`. Generic over the tier `C`,
/// the artanh-series iteration cap `CAP` (a safety net; the loop
/// terminates on a zero term far sooner), and the `INTERNAL_EXTRA`
/// directed-mode boundary-precision flag.
///
/// This is the working-scale Tang `ln` shared surface — the analogue of
/// [`crate::algos::exp::exp_tang::tang_exp_fixed`]. The narrow-ln-strict
/// kernel ([`ln_tang`]) wraps it with the Ziv-escalated storage narrowing;
/// `powf` composes it with `tang_exp_fixed` directly at working
/// scale (skipping a double round-to-storage).
///
/// ## Accuracy — the artanh truncation bias
///
/// The artanh series is truncated when `contribution = term / (2j+1)`
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
/// `INTERNAL_EXTRA = false` runs natively at `w` (the
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
    working_value: C::W,
    working_scale: u32,
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
    //
    // Currently UNWIRED: its last caller was the wide `ln_fixed_routed`
    // shim, removed with the `_approx` surface. Kept as the tier-bound
    // binding over the live generic kernel — it holds no computation of
    // its own, so nothing here is unroutable.
    tang_ln_fixed_g::<C::W, CAP, INTERNAL_EXTRA>(
        working_value, working_scale, |at_scale| C::ln2::<SCALE>(at_scale))
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
    working_value: S,
    working_scale: u32,
    ln2: impl Fn(u32) -> S,
) -> S
where
    S::Scratch: ComputeLimbs,
{
    // Stage 0 (INTERNAL_EXTRA only): widen the internal working scale
    // by `extra_digits = EXTERNAL_EXTRA_DIGITS` so the artanh-series
    // truncation bias (one-sided, ≈ 1 working-ULP) sits 12 decimal
    // digits below the caller's working ULP. The input is re-lifted
    // from `working_scale` to `extended_working_scale` by multiplying
    // by `10^extra_digits`.
    let (extended_working_scale, extended_working_value, extra_digits): (u32, S, u32) =
        if INTERNAL_EXTRA {
            let extra_digits = EXTERNAL_EXTRA_DIGITS;
            let extended_working_value = working_value * eg::pow10::<S>(extra_digits);
            (working_scale + extra_digits, extended_working_value, extra_digits)
        } else {
            (working_scale, working_value, 0)
        };

    let one_at_extended_scale = eg::one::<S>(extended_working_scale);
    let pow10_at_extended_scale = one_at_extended_scale;
    let two_at_extended_scale = one_at_extended_scale + one_at_extended_scale;

    // Stage 1: v = 2^k · m, m ∈ [1, 2). k from bit-shifts.
    let mut k: i32 = eg::bit_length::<S>(extended_working_value) as i32
        - eg::bit_length::<S>(one_at_extended_scale) as i32;
    let mantissa_w = loop {
        let candidate_mantissa = if k >= 0 {
            extended_working_value >> (k as u32)
        } else {
            extended_working_value << ((-k) as u32)
        };
        if candidate_mantissa >= two_at_extended_scale {
            k += 1;
        } else if candidate_mantissa < one_at_extended_scale {
            k -= 1;
        } else {
            break candidate_mantissa;
        }
    };

    // Stage 2: pick i. Boundary `m = 1` short-circuits: ln(m) = 0, so
    // ln(v) = k · ln(2).
    //
    // ── BENCHMARK HAZARD — read before choosing an `ln` operand ──
    //
    // This arm is a deliberate, bit-identical early-out (see below), and the
    // Series kernel `exp_generic::ln_fixed` has the SAME arm on the same
    // condition. So an exact power of two — `0.5`, `1`, `2.0`, `4`, … — runs
    // NEITHER kernel: no artanh series here, no Brent sqrt reduction there.
    // At every width and every scale. A benchmark on such an operand compares
    // two one-word `scale_by_k` products and says nothing about `ln`.
    //
    // There is a SECOND, Tang-only trap just below: `t = (m − f_i)/(m + f_i)`
    // is EXACTLY zero whenever `m` is an exact multiple of `1/M`, so the
    // artanh loop breaks on its first iteration while Series still pays its
    // full reduction. With `M = 128` that catches every value whose binary
    // mantissa terminates within 7 fraction bits — `7.0` (`m = 1.75`) and
    // `7.5` (`m = 1.875`) among them.
    //
    // In terms of the stored `raw = x·10^SCALE`, `raw` ODD and `raw % 5 != 0`
    // defeats both traps at every `SCALE >= 1`; at `SCALE == 0` the rule is
    // `raw` odd and `raw >= 257`. `benches/micro/ln_wide_series_tang_ab.rs`
    // states the derivation and asserts it on every operand it measures —
    // the first version of that map used `{0.5, 2.0, 7.5}`, all three
    // degenerate, which voided both its timings and its validity wall.
    let ln_at_extended_scale = if mantissa_w == one_at_extended_scale {
        // k·ln2 as an n-by-1-word product (`scale_by_k`, O(limbs)) — the
        // same value the previous full-width `ln2 * lit(k)` schoolbook
        // multiply produced (|k| < BITS, and k·ln2 at scale w_ext fits the
        // rung by construction, so the product never wraps), at a fraction
        // of the cost. Matches `exp_generic::ln_fixed`'s shape.
        eg::scale_by_k::<S>(ln2(extended_working_scale), k as i128)
    } else {
        // i ∈ [0, M); when m = 2 exactly (rare boundary post-rounding),
        // clamp to M-1 so the table lookup stays in range, then the
        // residual t handles the remaining tiny piece.
        let table_index_raw = ((mantissa_w - one_at_extended_scale)
            * eg::lit::<S>(M as i128))
            / one_at_extended_scale;
        let table_index_i128 = BigInt::to_i128(table_index_raw);
        let table_index = if table_index_i128 >= M as i128 {
            (M - 1) as usize
        } else {
            table_index_i128 as usize
        };

        let table_boundary = one_at_extended_scale
            + (one_at_extended_scale * eg::lit::<S>(table_index as i128))
                / eg::lit::<S>(M as i128);

        // Stage 3: t = (m - f_i) / (m + f_i). |t| < 1/(2M + 1).
        let atanh_arg = eg::div_cached::<S>(
            mantissa_w - table_boundary,
            mantissa_w + table_boundary,
            pow10_at_extended_scale);

        // Artanh series: 2 · (t + t³/3 + t⁵/5 + ...).
        //
        // ── The truncation below is LOAD-BEARING for directed rounding ──
        //
        // `term / lit(2j+1)` is integer division, so every term is TRUNCATED
        // toward zero and `sum` comes out SHORT. That is not merely tolerable
        // here, it is what makes the directed modes correct just BELOW `x = 1`,
        // and the property is easy to destroy by making this line "better".
        //
        // Below 1 the reduction gives `k = -1`, so the result is
        // `-ln2 + ln(m)`. A short `ln(m)` makes that MORE negative — larger in
        // magnitude, away from zero. Every truncation pushes that one way; none
        // can push back. The only terms that push toward zero are `ln2`'s
        // rounding and the table entry's, at most half a unit each.
        //
        // The net is measured: at `x = 1 − d·ULP` the returned magnitude sits at
        // least one working unit ABOVE `d·10^guard`, while the true sub-storage
        // residual `d²·10^(guard−S)/2` is far under one unit. So the walker sees
        // a NON-ZERO residual and its ordinary directed rule decides — `Floor`
        // reaches `adjust_log_near_zero` already stepped, and the tangent
        // bracket never fires for it. `ln2`'s rounding direction is NOT what
        // carries this: it is a near coin flip across scales, and cells that
        // read it at a down-rounding scale are correct anyway.
        //
        // So: correctness here rides on an ERROR being reliably one-sided and
        // at least one unit — not on the value being accurate. Making this
        // series MORE accurate (rounding these divisions instead of truncating,
        // or turning `INTERNAL_EXTRA` on at more tiers) shrinks that excess
        // toward zero. That is still correct, because a zero residual lands on
        // the linear term and `adjust_log_near_zero`'s tangent bracket then
        // supplies the same three answers — but it MOVES THE LOAD ONTO THAT
        // BRACKET, so it must stay intact. Only a NEGATIVE excess is wrong, and
        // that needs the two roundings to beat the accumulated truncation.
        //
        // Note `INTERNAL_EXTRA` is `true` at D462 ONLY (`policy::ln`); the other
        // nine wide tiers run natively at `working_scale` with no outward bump, so
        // they depend on exactly the argument above. See
        // `below_one_directed_modes_straddle` at the foot of this file.
        let atanh_arg_sq = eg::mul::<S>(atanh_arg, atanh_arg, extended_working_scale);
        let mut sum = atanh_arg;
        let mut term = atanh_arg;
        let mut term_index: u128 = 1;
        loop {
            term = eg::mul::<S>(term, atanh_arg_sq, extended_working_scale);
            let contribution = term / eg::lit::<S>((2 * term_index + 1) as i128);
            if contribution == eg::zero::<S>() {
                break;
            }
            sum = sum + contribution;
            term_index += 1;
            if term_index > CAP {
                break;
            }
        }
        let ln_mantissa = sum
            + sum
            + ln_table_entry_baked::<S>(
                extended_working_scale, table_index, pow10_at_extended_scale);

        // Final: ln(v) = k · ln(2) + ln(m). k·ln2 via the one-word
        // `scale_by_k` product (see the `mantissa_w == one_at_extended_scale`
        // arm above).
        eg::scale_by_k::<S>(ln2(extended_working_scale), k as i128) + ln_mantissa
    };

    if !INTERNAL_EXTRA || extra_digits == 0 {
        ln_at_extended_scale
    } else {
        // Truncate toward zero from `extended_working_scale` to
        // `working_scale`, then
        // bump the magnitude by 1 LSB-at-`working_scale` IF any digits
        // were discarded (`r_mag != 0`). The bump signals to the outer
        // directed rounder "sub-working-scale residual present, same sign
        // as the value" — preserving the residual sign at `working_scale`
        // even when truncation alone would round the residual to exactly
        // zero. The `+1` is at most one ULP-at-`working_scale`, i.e. `10^-guard`
        // storage ULPs (well below half a storage ULP), so nearest
        // stays correctly rounded.
        //
        // Sign-preservation argument: the discarded digits
        // `ln_at_extended_scale mod extra_pow10` share the sign of
        // `ln_at_extended_scale` (Rust integer truncation toward zero),
        // so the bumped magnitude `truncated + 1` straddles the true
        // value on the "outside" (in magnitude), which is exactly what a
        // directed rounder needs to decide whether to bump under each mode.
        let extra_pow10 = eg::pow10::<S>(extra_digits);
        let (truncated, has_residue) = if ln_at_extended_scale >= eg::zero::<S>() {
            let quotient = ln_at_extended_scale / extra_pow10;
            let has_discarded =
                ln_at_extended_scale - quotient * extra_pow10 != eg::zero::<S>();
            (quotient, has_discarded)
        } else {
            let abs_value = -ln_at_extended_scale;
            let quotient = abs_value / extra_pow10;
            let has_discarded = abs_value - quotient * extra_pow10 != eg::zero::<S>();
            (-quotient, has_discarded)
        };
        if has_residue {
            // Bump magnitude by 1 LSB-at-`working_scale` so the outer
            // rounder sees a nonzero residual with the value's sign.
            if truncated >= eg::zero::<S>() {
                truncated + eg::lit::<S>(1)
            } else {
                truncated - eg::lit::<S>(1)
            }
        } else {
            truncated
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
    St: BigInt,
    Wk: BigInt,
    Wtier: BigInt,
    const SCALE: u32,
    const GUARD: u32,
    const CAP: u128,
    const DIRECTED: bool,
    const INTERNAL_EXTRA: bool,
>(
    raw: St,
    storage_max: St,
    storage_min: St,
    mode: RoundingMode,
) -> St
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
    <Wtier as BigInt>::Scratch: ComputeLimbs,
{
    use crate::algos::support::wide_trig_core::{
        round_to_storage_directed_widening_g, round_to_storage_with_g, to_work_scaled_g,
    };
    use crate::support::rounding::DEFAULT_ROUNDING_MODE;

    if raw <= St::ZERO {
        panic!("tier ln: argument must be positive");
    }
    // `ln 2` at the RUNG work integer `Wk`, const-folded at the base working
    // scale `SCALE + GUARD` (the hot path) — the rung sibling of the per-tier
    // `C::ln2::<SCALE>` (value-identical; only the const-fold seam differs).
    let ln2 = |at_scale: u32| -> Wk {
        if at_scale == SCALE + GUARD {
            crate::consts::ln2_by_scale::<Wk>(SCALE + GUARD, DEFAULT_ROUNDING_MODE)
        } else {
            crate::consts::ln2_by_working_scale::<Wk>(at_scale, DEFAULT_ROUNDING_MODE)
        }
    };
    // The fall-up-width `ln 2` for the widening recompute (identical closure
    // shape at `Wtier` - the tier's full work integer, or `Wk` again when the
    // narrow path runs a single width).
    let ln2_tier_width = |at_scale: u32| -> Wtier {
        if at_scale == SCALE + GUARD {
            crate::consts::ln2_by_scale::<Wtier>(SCALE + GUARD, DEFAULT_ROUNDING_MODE)
        } else {
            crate::consts::ln2_by_working_scale::<Wtier>(at_scale, DEFAULT_ROUNDING_MODE)
        }
    };
    if DIRECTED {
        // Directed modes decide which side of a storage grid line the true
        // value falls; near a grid line the working-scale approximation can
        // land on the wrong side. Route through the shared Ziv escalation.
        //
        // `INTERNAL_EXTRA` buries the ~1-working-ULP artanh truncation bias
        // below the storage ULP (it runs the body at
        // `working_scale + EXTERNAL_EXTRA_DIGITS`,
        // ~5× the cost). That bias only flips a directed result NEAR x = 1,
        // where ln(x) ≈ x−1 is tiny and the deciding residual sits at the
        // precision boundary (the loss region is ε ~ 10^(−SCALE/2)); for x away
        // from 1 the result carries full significant digits and the bias is far
        // below half a storage ULP. So we VALUE-GATE the wide path: pay it ONLY
        // for near-1 inputs (`|x − 1| < 10^(−SCALE/4)`, comfortably covering the
        // loss region with margin) — every other input takes the fast
        // native-width path. `adjust_ln_near_one` (below) is itself value-gated
        // (`rounded == δ`), so the truly-unreachable ε case is handled regardless.
        let use_extra_digits = INTERNAL_EXTRA && {
            let one = eg::pow10::<St>(SCALE);
            let distance_from_one = if raw >= one { raw - one } else { one - raw };
            // near 1 iff |x − 1| < ~10^(−SCALE/4), tested on the bit-length so
            // the threshold is a compile-time const and the check is O(limbs)
            // (no per-call `pow`): `bit_length(10^k) ≈ k·log2(10)`, and ×3 is a
            // conservative `log2(10)`. Covers the ε ~ 10^(−SCALE/2) loss region
            // with wide margin; excludes ordinary operands (e.g. x = 1.5).
            distance_from_one.bit_length() < (SCALE - SCALE / 4) * 3
        };
        // Two-width fall-up: an unresolved-at-rung-cap near-tie reruns the
        // walker at the tier work width `C::W` (the `ln_tang` alias's own
        // realisation, verbatim) - see
        // `wide_trig_core::round_to_storage_directed_widening_g`.
        let rounded = if use_extra_digits {
            round_to_storage_directed_widening_g::<St, Wk, Wtier>(
                GUARD, SCALE, mode, storage_max, storage_min,
                |guard_digits| {
                    tang_ln_fixed_g::<Wk, CAP, true>(
                        to_work_scaled_g::<St, Wk>(raw, guard_digits),
                        SCALE + guard_digits, ln2,
                    )
                },
                |guard_digits| {
                    tang_ln_fixed_g::<Wtier, CAP, true>(
                        to_work_scaled_g::<St, Wtier>(raw, guard_digits),
                        SCALE + guard_digits, ln2_tier_width,
                    )
                },
            )
        } else {
            round_to_storage_directed_widening_g::<St, Wk, Wtier>(
                GUARD, SCALE, mode, storage_max, storage_min,
                |guard_digits| {
                    tang_ln_fixed_g::<Wk, CAP, false>(
                        to_work_scaled_g::<St, Wk>(raw, guard_digits),
                        SCALE + guard_digits, ln2,
                    )
                },
                |guard_digits| {
                    tang_ln_fixed_g::<Wtier, CAP, false>(
                        to_work_scaled_g::<St, Wtier>(raw, guard_digits),
                        SCALE + guard_digits, ln2_tier_width,
                    )
                },
            )
        };
        crate::algos::support::wide_trig_core::adjust_ln_near_one::<St, Wtier, SCALE>(rounded, raw, mode)
    } else {
        let working_scale = SCALE + GUARD;
        let working_value = tang_ln_fixed_g::<Wk, CAP, INTERNAL_EXTRA>(
            to_work_scaled_g::<St, Wk>(raw, GUARD), working_scale, ln2,
        );
        round_to_storage_with_g::<St, Wk>(
            working_value, working_scale, SCALE, mode, storage_max, storage_min,
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
    ln_tang_g::<C::Storage, C::W, C::W, SCALE, GUARD, CAP, DIRECTED, INTERNAL_EXTRA>(
        raw, C::storage_max(), C::storage_min(), mode,
    )
}

#[cfg(test)]
mod tests {
    /// Just BELOW `x = 1` the directed modes must STRADDLE — the contract the
    /// truncation note in [`tang_ln_fixed_g`] argues for, stated without an
    /// oracle.
    ///
    /// `ln(x)` is transcendental at every algebraic `x != 1`
    /// (Lindemann-Weierstrass), so it never lands on a storage grid line.
    /// `Ceiling` is therefore always exactly one ULP above `Floor`, and `Trunc`
    /// is whichever of the two faces zero — `Ceiling`, since `ln(x) < 0` below
    /// 1. Needing no oracle, this cannot rot when the oracle changes.
    ///
    /// It fails in exactly the way the note warns about. If the kernel's
    /// magnitude ever fell BELOW `d·10^guard`, the walker would read `q = d−1`;
    /// `adjust_log_near_zero` rescues `Floor` there via its tangent bracket but
    /// deliberately not `Trunc` or `Ceiling` (both arms are gated on `up`, and
    /// `delta < 0` here), so those two would come back one ULP short — visible
    /// here as a two-ULP span.
    ///
    /// Gated on the union of the tiers it checks so it cannot exist as an
    /// assertion-free pass in a build where every cell is compiled out, and it
    /// counts what it actually checked for the same reason.
    #[test]
    #[cfg(any(
        feature = "d57",
        feature = "d115",
        feature = "d307",
        feature = "d462",
        feature = "wide",
        feature = "x-wide"
    ))]
    fn below_one_directed_modes_straddle() {
        use crate::int::types::traits::BigInt;
        use crate::support::rounding::RoundingMode;

        let mut checked = 0u32;

        macro_rules! cell {
            ($limbs:literal, $scale:literal, $depth:literal) => {{
                type St = crate::int::types::Int<$limbs>;
                const S: u32 = $scale;
                let one = crate::consts::pow10::dispatch::<St>(S);
                for ulp_offset in 1i128..=$depth {
                    let raw = one - <St as BigInt>::from_i128(ulp_offset);
                    let ln_with_mode =
                        |mode| crate::D::<St, S>(raw).ln_with(mode).to_bits();
                    let floor = ln_with_mode(RoundingMode::Floor);
                    let ceiling = ln_with_mode(RoundingMode::Ceiling);
                    let trunc = ln_with_mode(RoundingMode::Trunc);
                    assert_eq!(
                        ceiling - floor,
                        <St as BigInt>::ONE,
                        "ln(1 - {ulp_offset}ulp) at Int<{}><{}>: Ceiling and Floor must straddle \
                         a value that cannot lie on the grid",
                        $limbs,
                        S
                    );
                    assert_eq!(
                        trunc, ceiling,
                        "ln(1 - {ulp_offset}ulp) at Int<{}><{}>: Trunc must be the neighbour \
                         facing zero",
                        $limbs, S
                    );
                    checked += 1;
                }
            }};
        }

        // A spread of the Tang tiers, at each one's top scale — where the
        // deciding term `d²·10^(guard−S)/2` is furthest below the working
        // resolution and the walker is blindest.
        #[cfg(any(feature = "d57", feature = "wide"))]
        cell!(3, 56, 3);
        #[cfg(any(feature = "d115", feature = "wide"))]
        cell!(6, 114, 3);
        #[cfg(any(feature = "d307", feature = "wide", feature = "x-wide"))]
        cell!(16, 306, 2);
        // D462 is the one tier with `INTERNAL_EXTRA`, so it exercises the
        // bumped narrow-back rather than the bare truncation.
        #[cfg(any(feature = "d462", feature = "x-wide"))]
        cell!(24, 461, 1);

        assert!(
            checked > 0,
            "no cell compiled in — this test must never pass without asserting"
        );
    }

    /// The bbc-benched D462<231> ln(2.0) cell: the exact-power-of-two
    /// operand takes the `mantissa_w == one_at_extended_scale`
    /// short-circuit (`ln(v) = k·ln2`) and
    /// must produce the correctly-rounded 231-digit value — pins the
    /// `scale_by_k` k·ln2 product and the direct-injection constant fold
    /// against the prior full-multiply shapes.
    #[test]
    #[cfg(feature = "d462")]
    fn ln_d462_s231_power_of_two_short_circuit() {
        let value: crate::D462<231> = "2.0".parse().unwrap();
        let ln_value = value.ln();
        let expect: crate::D462<231> = "0.693147180559945309417232121458176568075500134360255254120680009493393621969694715605863326996418687542001481020570685733685520235758130557032670751635075961930727570828371435190307038623891673471123350115364497955239120475172681575".parse().unwrap();
        assert_eq!(ln_value, expect);
    }

    /// The bbc-benched D115<0> ln(2) cell: SCALE = 0 now routes the Tang
    /// arm (the historic s0 Series gap); ln(2) = 0.693… rounds to 1 at
    /// scale 0 under the default nearest mode.
    #[test]
    #[cfg(feature = "d115")]
    fn ln_d115_s0_routes_and_rounds() {
        let value: crate::D115<0> = "2".parse().unwrap();
        let one: crate::D115<0> = "1".parse().unwrap();
        assert_eq!(value.ln(), one);
    }
}
