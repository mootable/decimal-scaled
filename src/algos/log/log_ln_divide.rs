// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `log_ln_divide` — arbitrary-base logarithm by the ratio of natural logs.
//!
//! `log(self, base) = ln(self) / ln(base)`. Two realisations of the one
//! composition, and the [`crate::policy::log`] matcher delegates *down* to
//! both:
//!
//! - The FIXED-guard shells (`Algorithm::LnDivide`), routed for every
//!   ordinary base: on the narrow tiers [`log_ln_divide_d18`] /
//!   [`log_ln_divide_d38`] (the 256-bit `Fixed` shell in
//!   `ln::ln_series_2limb`; D18 widens to Int<2> and narrows back), on the
//!   wide tiers the per-tier `log_strict_with_kernel` free functions emitted
//!   by `decl_wide_transcendental!` (in `crate::types::widths`). Every one
//!   runs the ratio at `w = SCALE + 30` and hands it to the Ziv walker.
//! - The CONDITIONED shell (`Algorithm::LnDivideConditioned`),
//!   [`log_ln_divide_conditioned`]: ONE generic kernel over the storage
//!   width `N` and a work integer `Wk`, whose guard is sized a priori from
//!   the base's conditioning number [`near_one_digits`]
//!   (`k = ceil(-log10 |b - 1|)`): `w = SCALE + 30 + k`, and which forms
//!   the ratio from the EXACT offsets `b_raw − 10^SCALE` (and
//!   `x_raw − 10^SCALE` when `x` is near 1 too) rather than from `ln b`
//!   ([`conditioned_ratio`]). Routed iff `k > 0`, i.e. the base lies
//!   within 0.1 of 1.
//!
//! # Why a base near 1 needs its own guard
//!
//! `ln b` for `b = 1 + 10^-k` has VALUE `~10^-k` but is computed to an
//! ABSOLUTE precision of `c` units of `10^-w` (`c` is the natural-log
//! kernel's accumulated truncation — ~1 to ~30 depending on how many series
//! terms are live at `w`), so its RELATIVE error is `c · 10^(k - w)`. The
//! quotient inherits that relative error, multiplied by its own magnitude
//! `ln x / ln b ≈ ln x · 10^k`. In storage ULP the `10^SCALE` cancels:
//!
//! ```text
//! err_ulp ≈ ln x · c · 10^(2k - GUARD)
//! ```
//!
//! independent of scale and of width. A fixed `GUARD = 30` is therefore
//! exact only up to `k ≈ 14`; `k = 15` sits on the boundary and `k = 18`
//! is a million past it. Golden 33885976351 on the shipped code:
//! `log(2, 1 + 10^-18)` came back 924,196 ULP out at D57 s28 — and at the
//! SAME delta on every wider width at equal scale (D230, D307, D462, D616,
//! D924 s30 all 924,196), because `c` is a function of `w` alone; it climbs
//! to 18,768,244 (`c ≈ 27`) at D1232 s924. The Ziv walker cannot catch it:
//! it escalates only inside the near-tie band, and a gross error leaves a
//! pseudo-random residual that is "clear" of every boundary. So the guard
//! is sized BEFORE the first probe, from `k` — the `log` analogue of
//! `exp`'s result-magnitude lift (`exp_result_int_digits`).
//!
//! # Why the work width follows `k`
//!
//! `w = SCALE + 30 + k` outruns a tier's fixed composition width in its
//! worst band (`k` runs up to `SCALE`, the base's own representability).
//! The policy therefore chooses the work integer from `k`
//! ([`fits_budget`] / [`fits_capacity`]) — the matcher's width axis, keyed
//! on the value — and the kernel takes its natural-log core from the
//! caller ([`series_core`] / [`tang_core`]) so `policy::ln`'s Tang/Series
//! verdict is honoured, with Series where the lifted scale exceeds the
//! baked Tang table's reach ([`tang_table_reaches`]).

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core::{
    round_to_storage_clear_of_tie_g, round_to_storage_directed_g, round_to_storage_with_g,
    to_work_scaled_g,
};
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::int::types::Int;
use crate::support::rounding::{RoundingMode, DEFAULT_ROUNDING_MODE};

/// D18 strict `log(self, base)`: widen to the D38 work width, run D38's
/// strict log, narrow back. Panics (with a scale-tagged overflow message)
/// if the result does not fit D18.
#[inline]
pub(crate) fn log_ln_divide_d18<const SCALE: u32>(
    raw: Int<1>,
    base_raw: Int<1>,
    mode: RoundingMode,
) -> Int<1> {
    let wide_value: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(raw).into();
    let wide_base: crate::D<Int<2>, SCALE> = crate::D::<Int<1>, SCALE>(base_raw).into();
    let log_value: crate::D<Int<1>, SCALE> =
        ::core::convert::TryInto::try_into(wide_value.log_with(wide_base, mode))
            .unwrap_or_else(|_| {
                crate::support::diagnostics::overflow_panic_with_scale("D18::log", SCALE)
            });
    log_value.0
}

/// D38 strict `log(self, base)` via the `ln::ln_series_2limb` 256-bit log
/// kernel, on raw storage. `None` = result out of storage range.
#[inline]
pub(crate) fn log_ln_divide_d38<const SCALE: u32>(
    raw: Int<2>,
    base_raw: Int<2>,
    mode: RoundingMode,
) -> Option<Int<2>> {
    crate::algos::ln::ln_series_2limb::log::<SCALE>(raw, base_raw, mode)
}

/// The narrow tiers' Tang `log(x, base)` for an ORDINARY base (`k == 0`) —
/// the Tang sibling of [`log_ln_divide_d38`], which runs the same
/// composition on the Series core and stays the kept alternative.
///
/// `log(x, b) = ln x / ln b` at the fixed [`COMPOSITION_GUARD`], both logs
/// through the width-generic Tang core ([`tang_core`]) at the caller's
/// narrow-safe work width `Wk`, then the shared [`log_ratio_finish`] — the
/// same finish (exact integer-power pin, clear-of-tie single shot, exact
/// rational-power pin, Ziv-escalated directed narrowing) the wide
/// fixed-guard shells and the conditioned arm already run, so every path
/// decides the same input the same way.
///
/// ── WHY TANG HERE ──
///
/// The wide fixed-guard shells take their natural-log core from
/// `ln_fixed_routed_agm`, which is Tang at every wide tier and every scale
/// (`policy::ln::is_tang`), while the Series sibling pays
/// `exp_generic::ln_fixed`'s Brent reduction and its `O(w)` artanh series
/// TWICE per call — once for `x`, once for the base — at
/// `w = SCALE + 30`. That is the same-scale inversion `policy::ln` closed
/// for `ln` itself by routing the narrow tiers to Tang; `log` composes two
/// natural logs, so it carried the inversion twice over and was left on
/// Series when `ln` moved. Tang's reduction is a table read and its artanh
/// runs at `|t| < 1/256` (~`0.21·w` terms against Series' ~`1.05·w`).
///
/// ── THE WORK WIDTH ──
///
/// `Wk` is the caller's; the policy binds `Int<12>`, the same narrow-safe
/// work integer `policy::ln`'s narrow Tang arm runs at — 6 u128 limbs
/// against the narrow build's `MAX_U128_LIMB = 8`, so it is a legal
/// `resize_to` receiver both ways where `Int<24>` is not. Its budget
/// carries this composition at EVERY narrow cell, not just the benched
/// ones: the walker sizes by `8 · limbs = 96` digits against
/// `needed_digits(SCALE, 0) = SCALE + 42`, i.e. `80` at the widest narrow
/// cell `D38<37>`, and the arithmetic capacity `2w + 40 = 176` sits inside
/// `19 · limbs = 228`. The baked Tang table reaches `w <= 134` in a narrow
/// build (`LN_TANG_B = 512` bits against `w · log2(10) + 64`), above the
/// `96` the walker can ever ask for — so this path needs no
/// [`tang_table_reaches`] gate, and the table's own
/// `p_full <= LN_TANG_LIMBS` assertion stands behind it.
///
/// `None` = result out of storage range: the narrow tiers' `checked_`
/// contract. `log` genuinely leaves narrow storage for an ordinary base
/// near the band edge (`log_1.1(x)` at `D38<37>` reaches ~`926`, past
/// `Int<2>::MAX` at that scale), so the range is decided on the probe
/// BEFORE the finish runs — the finish panics past storage — exactly as
/// the conditioned narrow shell decides it.
#[inline]
#[must_use]
pub(crate) fn log_ln_divide_tang_narrow<
    const N: usize,
    Wk: BigInt,
    const SCALE: u32,
    const CAP: u128,
>(
    raw: Int<N>,
    base_raw: Int<N>,
    mode: RoundingMode,
) -> Option<Int<N>>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    assert!(raw > Int::<N>::ZERO, "log: argument must be positive");
    assert!(base_raw > Int::<N>::ZERO, "log: base must be positive");
    // `b == 1` is the domain wall (`ln 1 = 0`, and the quotient would
    // divide by zero). Decided EXACTLY on the storage integer — one
    // comparison, no `ln b` — rather than on a rounded core result.
    assert!(
        base_raw != eg::pow10::<Int<N>>(SCALE),
        "log: base must not equal 1 (ln(1) is zero)"
    );
    let ln_at = tang_core::<Wk, SCALE, CAP>();
    let ratio_at = |guard_digits: u32| {
        conditioned_ratio_quotient::<N, Wk, SCALE>(raw, base_raw, guard_digits, &ln_at)
    };
    let probe = ratio_at(COMPOSITION_GUARD);
    let single_shot =
        narrow_single_shot::<N, Wk>(probe, SCALE + COMPOSITION_GUARD, SCALE, mode)?;
    // Within one ULP of the storage extreme the walker's ±1 could leave
    // range and its own check would panic; the single shot stands there (a
    // tie that deep at the extreme is Table-Maker's-Dilemma residue either
    // way) — the conditioned narrow shell's rule, unchanged.
    if single_shot.abs() >= Int::<N>::MAX - Int::<N>::ONE {
        return Some(single_shot);
    }
    Some(log_ratio_finish::<N, Wk, SCALE>(
        raw,
        base_raw,
        mode,
        COMPOSITION_GUARD,
        probe,
        ratio_at,
    ))
}

// ── The conditioned composition ─────────────────────────────────────────

/// Guard digits of the `ln(x)/ln(b)` composition for an ordinary base — the
/// value every fixed-guard shell runs at (`decl_wide_transcendental!`'s
/// `GUARD`, `ln_series_2limb::STRICT_GUARD`). The conditioned shell adds
/// `k` on top ([`lifted_guard`]).
pub(crate) const COMPOSITION_GUARD: u32 = 30;

/// The base's conditioning number: how many significant digits `ln b`
/// loses to `b`'s proximity to 1 — `ceil(-log10 |b - 1|)` — or `0` when
/// `|b - 1| >= 0.1`.
///
/// `base_raw` is the storage integer `b · 10^scale`. The `0.1` threshold
/// keeps every ordinary base (`2`, `10`, `e`, `0.5`, `1.5`, `7`, …) at
/// `k = 0`, so the fixed-guard shells stay routed for them, unchanged: a
/// loss under one digit is absorbed many times over by the 30-digit guard
/// (`err_ulp ≈ ln x · c · 10^(2 - 30)`). Below it `k` is the exact
/// ceiling: with `10^j <= |b_raw - 10^scale| < 10^(j+1)` the loss is
/// `-log10 |b - 1| ∈ (scale - j - 1, scale - j]`, so `k = scale - j`.
///
/// One comparison settles the ordinary band; only a base within 0.1 of 1
/// pays the `log2(scale)`-step `pow10` bisection that pins `j`.
pub(crate) fn near_one_digits<S: BigInt>(base_raw: S, scale: u32) -> u32 {
    if scale == 0 {
        return 0; // an integer base other than 1 is at least 1 away
    }
    let one = eg::pow10::<S>(scale);
    let distance = (base_raw - one).abs();
    // |b - 1| >= 0.1: the ordinary band.
    if distance >= eg::pow10::<S>(scale - 1) {
        return 0;
    }
    // b == 1 is the callers' domain wall (it never reaches a kernel); there
    // is no finite lift for it, and it must not underflow the bracket below.
    if distance == S::ZERO {
        return 0;
    }
    // Largest j with 10^j <= distance. The bracket is [0, scale - 2]: the
    // distance is in [1, 10^(scale - 1)), so 10^0 <= distance holds at the
    // bottom and 10^(scale - 1) > distance at the top.
    let (mut lo, mut hi) = (0u32, scale - 2);
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if eg::pow10::<S>(mid) <= distance {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    scale - lo
}

/// Working-scale guard of the conditioned composition at conditioning
/// `k`: `30 + k` — the result's own integer digits, and nothing for the
/// divisor. The composition never forms `ln b` (see [`conditioned_probe`]),
/// so the only precision a base near 1 costs is the `k` digits its result
/// `ln x · 10^k` carries above an ordinary one — the same rule `exp` applies
/// through `exp_result_int_digits`, and the floor no formulation can go
/// under: a result with `SCALE + k + 1` significant digits needs that many
/// working digits in whatever determines it. At `k = 0` it is the fixed
/// guard exactly. (The naive quotient [`conditioned_probe_quotient`] needs
/// `30 + 2k`; the module doc derives both.)
pub(crate) const fn lifted_guard(near_one_digits: u32) -> u32 {
    COMPOSITION_GUARD + near_one_digits
}

/// Decimal digits the conditioned composition asks of its work integer at
/// `(scale, k)`, on the `8 · limbs` scale the rung ladder and the Ziv
/// walker size by (`work_rung`; `cap_digits = BITS/8 - int_digits - 8` in
/// `round_to_storage_directed_tagged_impl_g`): the working scale
/// `scale + 30 + k`, the result's integer digits (`≤ k + 4` — `ln x` is
/// below `10^4` at every width), and the walker's own `8` — i.e.
/// `scale + 2k + 42`. A width that meets it leaves the walker its ordinary
/// escalation headroom above the lifted base guard.
pub(crate) const fn needed_digits(scale: u32, near_one_digits: u32) -> u32 {
    scale + lifted_guard(near_one_digits) + near_one_digits + 4 + 8
}

/// `true` iff a work integer of `limbs` u64 limbs carries the conditioned
/// composition at `(scale, k)` WITH the walker's escalation headroom — the
/// policy's width choice.
pub(crate) const fn fits_budget(scale: u32, near_one_digits: u32, limbs: usize) -> bool {
    needed_digits(scale, near_one_digits) <= 8 * (limbs as u32)
}

/// `true` iff a work integer of `limbs` u64 limbs can HOLD the conditioned
/// composition's intermediates at `(scale, k)` at all: the base probe's
/// `ln(x) · 10^w · 10^w` divide numerator and the Tang table reconstruction
/// `slot_hi · 10^w` both span about `2w + 40` decimal digits, against the
/// `19.27 · limbs` a u64-limb integer holds (`19` here, a safe floor). The
/// fail-closed wall: past it the wrapping limb arithmetic would return a
/// plausible wrong value, so the policy asserts it on the widest width it
/// can reach rather than compute.
pub(crate) const fn fits_capacity(scale: u32, near_one_digits: u32, limbs: usize) -> bool {
    2 * (scale + lifted_guard(near_one_digits)) + 40 <= 19 * (limbs as u32)
}

/// `true` iff the baked Tang `ln` table reaches working scale `w` — the
/// mirror of the `p_full <= LN_TANG_LIMBS` assertion inside
/// `ln_tang_table::ln_table_entry_baked` (`w · log2(10) + 64` bits of slot
/// against the `B = 7168` stored). The conditioned lift can exceed it at
/// the two widest tiers above scale ~702 (`w = 3·SCALE + 30` at `k = SCALE`);
/// the policy takes the Series core there.
#[cfg(feature = "_wide-support")]
pub(crate) const fn tang_table_reaches(working_scale: u32) -> bool {
    let need_bits = (working_scale as u64) * 3322 / 1000 + 64;
    need_bits <= 64 * (crate::algos::support::ln_tang_table::LN_TANG_LIMBS as u64)
}

/// `ln 2` at `working_scale` in `Wk`, const-folded at the fixed-guard
/// working scale (the fixed shells' hot path — `ln2_cf_agm`'s shape); the
/// conditioned shell runs above it and takes the runtime table lookup, the
/// same value.
#[inline]
fn ln2_at<Wk: BigInt, const SCALE: u32>(working_scale: u32) -> Wk {
    if working_scale == SCALE + COMPOSITION_GUARD {
        crate::consts::ln2_by_scale::<Wk>(SCALE + COMPOSITION_GUARD, DEFAULT_ROUNDING_MODE)
    } else {
        crate::consts::ln2_by_working_scale::<Wk>(working_scale, DEFAULT_ROUNDING_MODE)
    }
}

/// The Series natural-log core at work width `Wk` for
/// [`log_ln_divide_conditioned`]: `exp_generic::ln_fixed` (Brent sqrt
/// reduction + artanh series) — the kernel every narrow-tier Ziv probe and
/// the wide tiers' `Algorithm::Series` already run.
#[inline]
pub(crate) fn series_core<Wk: BigInt, const SCALE: u32>() -> impl Fn(Wk, u32) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    |working_value, working_scale| {
        eg::ln_fixed::<Wk>(working_value, working_scale, ln2_at::<Wk, SCALE>(working_scale))
    }
}

/// The Tang natural-log core at work width `Wk` — `ln_tang::tang_ln_fixed_g`
/// with the tier's artanh cap `CAP` and `INTERNAL_EXTRA = false`, exactly as
/// the wide fixed shells' `ln_fixed_routed_agm` runs it. The caller gates it
/// on [`tang_table_reaches`], or proves the reach from its own width budget
/// (the narrow shell [`log_ln_divide_tang_narrow`] does the latter).
///
/// NOT `_wide-support`-gated: the Tang table is baked in BOTH builds — the
/// narrow one stores the same `M + 1` slots at `B = 512` bits (8 u64 limbs)
/// against the wide build's 7168 — and `policy::ln`'s narrow Tang arm
/// already reads it through the same accessor. The core is width-generic,
/// so the narrow `log` composition binds it at its own narrow-safe work
/// width exactly as the wide shells bind it at `Wagm`.
#[inline]
pub(crate) fn tang_core<Wk: BigInt, const SCALE: u32, const CAP: u128>() -> impl Fn(Wk, u32) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    |working_value, working_scale| {
        crate::algos::ln::ln_tang::tang_ln_fixed_g::<Wk, CAP, false>(
            working_value,
            working_scale,
            ln2_at::<Wk, SCALE>,
        )
    }
}

/// `log(x, base)` by `ln(x)/ln(base)` at the conditioning-lifted guard, on
/// storage `Int<N>` in the work integer `Wk` — [`conditioned_probe`] then
/// [`conditioned_finish`].
///
/// The shell is the wide fixed-guard `log_strict_with_kernel` — the domain
/// walls, the exact integer-power pin, the clear-of-tie single shot, the
/// Ziv-escalated directed narrowing — with `guard` (from [`lifted_guard`])
/// where that shell has the const `GUARD`, the ratio formed by
/// [`conditioned_ratio`] instead of the naive quotient, the natural-log
/// core `ln_at` (`working value at w, w -> ln at w`) supplied by the caller
/// so the matcher's `ln` routing is honoured, and the exact rational-power
/// pin the narrow shell carries.
///
/// The two pins are not fast paths; they are the only way the rational
/// half of the domain can be decided. For decimal `x` and `b` the value
/// `log_b(x)` is either rational or transcendental — a corollary of the
/// Gelfond–Schneider theorem (an algebraic irrational `r` would make
/// `b^r = x` transcendental) — and on an exactly rational result the Ziv
/// escalation cannot converge in principle: the residual at every depth is
/// the kernel's own noise around a boundary the value sits exactly on, so
/// the walker's cap would hand back a 1-ULP directed miss. The pins settle
/// those inputs by exact integer arithmetic; the walker keeps the
/// transcendental remainder, where escalation is the sound method.
///
/// Panics on a non-positive argument or base, on `base == 1`, and — like
/// every wide shell — when the result does not fit `Int<N>`; a caller whose
/// contract is `None` past storage range-gates the probe first
/// (`policy::log`'s narrow arm).
pub(crate) fn log_ln_divide_conditioned<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    mode: RoundingMode,
    guard: u32,
    ln_at: impl Fn(Wk, u32) -> Wk,
) -> Int<N>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let probe_ratio = conditioned_probe::<N, Wk, SCALE>(raw, base_raw, guard, &ln_at);
    conditioned_finish::<N, Wk, SCALE>(raw, base_raw, mode, guard, &ln_at, probe_ratio)
}

/// The natural log of storage `value` lifted by `guard_digits`, at working
/// scale `SCALE + guard_digits`, through the caller's core.
#[inline]
fn ln_of<const N: usize, Wk: BigInt, const SCALE: u32>(
    ln_at: &impl Fn(Wk, u32) -> Wk,
    value: Int<N>,
    guard_digits: u32,
) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    ln_at(to_work_scaled_g::<Int<N>, Wk>(value, guard_digits), SCALE + guard_digits)
}

/// The base probe of the conditioned composition: the domain walls, then
/// [`conditioned_ratio`] at working scale `SCALE + guard` in `Wk`. Split
/// from [`conditioned_finish`] so a caller whose storage contract is `None`
/// past range (the narrow arm) can gate on the probe BEFORE any narrowing
/// runs — every narrowing step in the finish range-checks against `Int<N>`
/// and panics past it, the wide contract.
pub(crate) fn conditioned_probe<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    guard: u32,
    ln_at: &impl Fn(Wk, u32) -> Wk,
) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    if raw <= Int::<N>::ZERO {
        panic!("log: argument must be positive");
    }
    if base_raw <= Int::<N>::ZERO {
        panic!("log: base must be positive");
    }
    if base_raw == eg::pow10::<Int<N>>(SCALE) {
        panic!("log: base must not equal 1");
    }
    conditioned_ratio::<N, Wk, SCALE>(raw, base_raw, guard, ln_at)
}

/// `log_b(x)` at working scale `SCALE + guard_digits`, without ever forming
/// `ln b` — nor `ln x` when `x` is itself within 0.1 of 1:
///
/// ```text
/// ε = d_b / 10^SCALE,  d_b = b_raw − 10^SCALE         (an EXACT integer)
/// ln b = ε · g(ε),     g(ε) = ln(1+ε)/ε = Θ(1)
/// R = ln x / (ε · g(ε)) = ( ln x / g(ε) ) · 10^SCALE / d_b
///
/// a = d_x / 10^SCALE,  d_x = x_raw − 10^SCALE,  |a| < 0.1:
/// ln x = a · g(a)
/// R = ( g(a) / g(ε) ) · d_x / d_b
/// ```
///
/// `g` is evaluated by its own series from the offset ([`g_of_epsilon`]) —
/// never by computing `ln(1+ε)` and dividing — so no quantity of size
/// `10^-k` is ever held at absolute resolution; the only division by a
/// small number is by the exact integer `d_b`, which preserves relative
/// precision. The probe's relative error is then `~(c + ln x)·10^-w`, so
/// the guard need only cover the result's own magnitude (`lifted_guard`:
/// `30 + k`), where the naive quotient ([`conditioned_ratio_quotient`])
/// needs `30 + 2k`.
///
/// The numerator gets the same treatment when `x` is near 1 because `ln x`
/// held at absolute resolution `10^-w` carries only `w − k_x` significant
/// digits, and a DIRECTED rounding can need more of them than the base
/// guard leaves. In the power-of-ten family `x = 1 + 10^-p`, `b = 1 + 10^-q`
/// the visible terms of `R` land exactly on a grid line and the deciding
/// term (`(a/ε)·a²/3`-sized) sits `3p − q` digits down, so `ln x` at
/// absolute resolution needs `w > 3p` to see it — past the `Int<256>`
/// walker cap at D924 s900, where golden 33918031518 saw the unresolved
/// endgame hand the grid value to Ceiling / AwayFromZero / ZeroFiveUp —
/// while `d_x · g(a)` needs `w > 2p`, inside the base probe (the
/// `doubly_near_one_residue_is_decided_from_both_exact_offsets` test at
/// unit size). An ordinary `x` keeps the natural-log core, bit for bit:
/// `g`'s series is only fast inside the 0.1 band.
///
/// Used for the base probe and for every escalated probe of the walker, so
/// the escalation sequence is the same formulation at every depth.
fn conditioned_ratio<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    guard_digits: u32,
    ln_at: &impl Fn(Wk, u32) -> Wk,
) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + guard_digits;
    let one_scaled = eg::pow10::<Wk>(SCALE);
    let d_b = BigInt::resize_to::<Wk>(base_raw) - one_scaled;
    let g_b = g_of_epsilon::<Wk>(d_b, one_scaled, working_scale);
    if near_one_digits::<Int<N>>(raw, SCALE) > 0 {
        let d_x = BigInt::resize_to::<Wk>(raw) - one_scaled;
        let g_x = g_of_epsilon::<Wk>(d_x, one_scaled, working_scale);
        // `g(a) / g(ε)` at scale `w`, then `· d_x / d_b` — two exact
        // integers — rounded half-even. The product spans `w + SCALE`
        // digits, inside the `2w + 40` the capacity wall budgets.
        let g_ratio = eg::div::<Wk>(g_x, g_b, working_scale);
        return eg::round_div::<Wk>(g_ratio * d_x, d_b);
    }
    let ln_x_over_g = eg::div::<Wk>(
        ln_of::<N, Wk, SCALE>(ln_at, raw, guard_digits),
        g_b,
        working_scale,
    );
    // `(ln x / g)` is at scale `w`; dividing by `ε = d_b / 10^SCALE` is
    // `· 10^SCALE / d_b`, rounded half-even — the exact-integer divide.
    eg::div::<Wk>(ln_x_over_g, d_b, SCALE)
}

/// The base-offset-only form of [`conditioned_ratio`] — `ln x` from the
/// natural-log core for EVERY `x`, the base alone from its exact offset —
/// KEPT as an unrouted reference. It is what shipped in the reformulation
/// and it is correctly rounded everywhere the deciding digit lies within
/// the walker's reach; the live form differs from it only where `x` is
/// within 0.1 of 1 too, and there only on the deep directed residues the
/// module doc describes (`w > 3p` against `w > 2p`). The bit-identity test
/// runs the live form against this and against
/// [`conditioned_ratio_quotient`].
#[allow(dead_code)]
fn conditioned_ratio_offset_base<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    guard_digits: u32,
    ln_at: &impl Fn(Wk, u32) -> Wk,
) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let working_scale = SCALE + guard_digits;
    let one_scaled = eg::pow10::<Wk>(SCALE);
    let d_b = BigInt::resize_to::<Wk>(base_raw) - one_scaled;
    let g_b = g_of_epsilon::<Wk>(d_b, one_scaled, working_scale);
    let ln_x_over_g = eg::div::<Wk>(
        ln_of::<N, Wk, SCALE>(ln_at, raw, guard_digits),
        g_b,
        working_scale,
    );
    eg::div::<Wk>(ln_x_over_g, d_b, SCALE)
}

/// `g(ε) = ln(1+ε)/ε` at working scale `w`, for `ε = d / 10^SCALE` given as
/// the exact integer `d` and `10^SCALE` (`one_scaled`), `|ε| < 0.1`:
///
/// ```text
/// t = ε / (2 + ε),   |t| < 0.048
/// g(ε) = (2 / (2 + ε)) · (1 + t²/3 + t⁴/5 + …)     [ artanh(t)/t ]
/// ```
///
/// Both factors are `Θ(1)` and every term is a small correction, so a
/// fixed-scale carrier holds `g` at full relative precision (`~j` units of
/// `10^-w` after `j` terms — under `w/2.6` terms, each gaining at least
/// `−log10 t² ≥ 2.6` digits). `t` and `2/(2+ε)` are single rounded divides
/// by the exact `(2 + ε)·10^SCALE`.
fn g_of_epsilon<Wk: BigInt>(d: Wk, one_scaled: Wk, working_scale: u32) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let two_plus_eps = one_scaled + one_scaled + d;
    let t = eg::div::<Wk>(d, two_plus_eps, working_scale);
    let pre = eg::div::<Wk>(one_scaled + one_scaled, two_plus_eps, working_scale);
    let one_w = eg::one::<Wk>(working_scale);
    let t_sq = eg::mul::<Wk>(t, t, working_scale);
    let mut sum = one_w;
    let mut term = one_w;
    let mut term_index: u128 = 1;
    loop {
        term = eg::mul::<Wk>(term, t_sq, working_scale);
        let contribution = term / eg::lit::<Wk>((2 * term_index + 1) as i128);
        if contribution == Wk::ZERO {
            break;
        }
        sum = sum + contribution;
        term_index += 1;
        // Each term gains at least 2.6 digits, so the loop ends well inside
        // `w` iterations; the cap is a belt, never the exit.
        if term_index > working_scale as u128 {
            break;
        }
    }
    eg::mul::<Wk>(pre, sum, working_scale)
}

/// The plain quotient `ln(x)/ln(base)` at working scale
/// `SCALE + guard_digits` — the composition every fixed-guard shell runs.
///
/// ROUTED for the ordinary-base narrow path
/// ([`log_ln_divide_tang_narrow`], `k == 0`, guard `30`), and kept as the
/// reference formulation against which [`conditioned_ratio`] is checked
/// for a base near 1. It is correctly rounded only at guard `30 + 2k` (the
/// module doc's law: `ln b ~ 10^-k` held at absolute resolution loses `k`
/// digits of relative precision and the quotient multiplies that by its
/// own `10^k`), which is why the conditioned arm reformulates instead of
/// calling it; at `k == 0` the two guards coincide at `30` and the plain
/// quotient IS the correct composition. The bit-identity test runs it at
/// the `30 + 2k` lift against the reformulation at `30 + k`.
fn conditioned_ratio_quotient<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    guard_digits: u32,
    ln_at: &impl Fn(Wk, u32) -> Wk,
) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    eg::div::<Wk>(
        ln_of::<N, Wk, SCALE>(ln_at, raw, guard_digits),
        ln_of::<N, Wk, SCALE>(ln_at, base_raw, guard_digits),
        SCALE + guard_digits,
    )
}

/// Everything after the probe of the CONDITIONED composition:
/// [`log_ratio_finish`] with [`conditioned_ratio`] as the walker's
/// recompute. `probe_ratio` is [`conditioned_probe`]'s value for the same
/// arguments.
pub(crate) fn conditioned_finish<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    mode: RoundingMode,
    guard: u32,
    ln_at: &impl Fn(Wk, u32) -> Wk,
    probe_ratio: Wk,
) -> Int<N>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    log_ratio_finish::<N, Wk, SCALE>(raw, base_raw, mode, guard, probe_ratio, |guard_digits| {
        conditioned_ratio::<N, Wk, SCALE>(raw, base_raw, guard_digits, ln_at)
    })
}

/// Everything after the probe of EVERY `ln(x)/ln(b)` composition, whatever
/// formed the ratio: the exact integer-power pin, the clear-of-tie single
/// shot, the exact rational-power pin, and the Ziv-escalated directed
/// narrowing into `Int<N>` (which panics past storage). `probe_ratio` is
/// the ratio at `guard`; `ratio_at(g)` is the same composition at guard
/// `g`, called only for an ESCALATED probe (`g > guard`) — the walker's
/// first probe is `probe_ratio` itself, so an ordinary input never
/// recomputes.
///
/// The wide tiers' fixed-guard shell (`log_strict_with_kernel`, emitted by
/// `decl_wide_transcendental!`) runs this finish with its own two-core
/// quotient as `ratio_at`; the conditioned arm runs it through
/// [`conditioned_finish`]. One finish, so the exact rational-power pin —
/// which the wide shell lacked: golden `log_1.21(1.1) = 1/2`,
/// `log_1.44(1.728) = 3/2` came back a ULP off under a directed mode
/// wherever the kernel noise fell on the wrong side of the grid line —
/// decides the same inputs the same way on every path. For decimal `x` and
/// `b`, `log_b(x)` is rational or transcendental (Gelfond–Schneider): on
/// the rational half no escalation converges, the residual at every depth
/// being noise around the grid line, so a pin by exact integer arithmetic
/// is the only sound decision; on the transcendental half the residual is
/// genuine and escalation is the sound method. On an input the pins do not
/// take, the sequence is the walker's own: the single shot IS its first
/// step, bit for bit, and the walker is entered exactly when it would have
/// escalated.
pub(crate) fn log_ratio_finish<const N: usize, Wk: BigInt, const SCALE: u32>(
    raw: Int<N>,
    base_raw: Int<N>,
    mode: RoundingMode,
    guard: u32,
    probe_ratio: Wk,
    ratio_at: impl Fn(u32) -> Wk,
) -> Int<N>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let base_working_scale = SCALE + guard;
    // Exact-power pin: `x == base^k` ⇒ the result is exactly the integer `k`.
    let exponent = eg::round_to_nearest_int::<Wk>(probe_ratio, base_working_scale);
    if log_is_exact_int::<Wk>(
        BigInt::resize_to::<Wk>(raw),
        BigInt::resize_to::<Wk>(base_raw),
        SCALE,
        exponent,
    ) {
        return exact_int_at_scale::<N, Wk>(exponent, SCALE, mode);
    }
    // The ordinary case: a residual clear of the mode's deciding boundary
    // narrows in one shot — the walker's own first probe, arm for arm
    // (`directed_narrow` / `nearest_narrow` at the base guard), so this is
    // bit-identical to entering the walker and returning from its first step.
    if let Some(narrowed) = round_to_storage_clear_of_tie_g::<Int<N>, Wk>(
        probe_ratio,
        base_working_scale,
        SCALE,
        mode,
        Int::<N>::MAX,
        Int::<N>::MIN,
    ) {
        return narrowed;
    }
    // Near a boundary. An EXACT rational result never resolves by escalation:
    // the probe's residual at every depth is the kernel's own noise around
    // the grid line, so the directed walker's unresolved endgame — the base
    // probe's directed narrowing — lands 1 ULP off under whichever directed
    // mode faces the noise's side (golden 33893684900: `log_1.0201(1.01)`,
    // `log_1.0201(1.030301)`, delta 1, Floor/Ceiling/Trunc only). Decide it by
    // exact integer arithmetic first, as the narrow shell does.
    if let Some(pinned) = log_rational_pow_pin::<N, Wk>(
        raw,
        base_raw,
        SCALE,
        probe_ratio,
        base_working_scale,
        mode,
    ) {
        return pinned;
    }
    // Directed narrowing through the shared Ziv escalation. The walker
    // always probes `guard` first, and that probe is `probe_ratio` (pure
    // functions of the same arguments); only an escalated probe recomputes.
    round_to_storage_directed_g::<Int<N>, Wk>(
        guard,
        SCALE,
        mode,
        Int::<N>::MAX,
        Int::<N>::MIN,
        |guard_digits| {
            if guard_digits == guard {
                return probe_ratio;
            }
            ratio_at(guard_digits)
        },
    )
}

/// Single-shot narrowing of a working value into `Int<N>` with the fit
/// reported as `Option` — the narrow tiers' `checked_` contract, where the
/// generic narrowings panic. Rounds `value` (at `working_scale`) to `target`
/// under `mode` exactly as `round_to_storage_with_g` does, then fits it:
/// `None` when it lies outside `Int<N>`. Every resize here has a NARROW
/// receiver or goes through `Wk`'s exact scratch: the width-erased
/// `BigInt::resize_to` blanket is sized to the build's `MAX_U128_LIMB`,
/// which in a narrow build cannot hold an `Int<24>` receiver (golden
/// 33893684900 / 33895788589: every conditioned narrow call panicked at
/// `resize_to` — `range end index 12 out of range for slice of length 8`).
pub(crate) fn narrow_single_shot<const N: usize, Wk: BigInt>(
    value: Wk,
    working_scale: u32,
    target: u32,
    mode: RoundingMode,
) -> Option<Int<N>>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let shift = working_scale - target;
    let rounded = if shift == 0 {
        value
    } else if shift <= 38 {
        crate::algos::support::mg_divide::div_wide_pow10::<Wk>(value, shift, mode)
    } else {
        crate::algos::support::rescale::dispatch_wide_pow10::<Wk>(value, shift, mode)
    };
    // Up-resize the NARROW bounds (their own width sizes the blanket buffer).
    let max_w = BigInt::resize_to::<Wk>(Int::<N>::MAX);
    let min_w = BigInt::resize_to::<Wk>(Int::<N>::MIN);
    if rounded > max_w || rounded < min_w {
        return None;
    }
    // Down-resize through `Wk`'s exact scratch, never the blanket.
    let is_negative = rounded < Wk::ZERO;
    let mag = if is_negative { -rounded } else { rounded };
    let mut buf = <Wk::Scratch as ComputeLimbs>::single_u64();
    eg::unpack_mag(mag, buf.as_mut());
    Some(Int::<N>::from_mag_sign_u64(buf.as_ref(), is_negative))
}

/// `value == base^exponent` exactly at storage scale `scale` (both raw
/// storage integers lifted into `S`) — the exact-integer-power pin of the
/// composition, the wide shells' `log_is_exact_int` made a free generic.
/// Reduces to the integer domain so the running power never carries the
/// `10^scale` factor; a base that is not an exact integer multiple of
/// `10^scale` (every near-1 base) can never be an exact power, so the
/// conditioned path only ever pins `exponent == 0` (`x == 1`). Overflow of
/// the running power short-circuits to `false`.
fn log_is_exact_int<S: BigInt>(value_raw: S, base_raw: S, scale: u32, exponent: i128) -> bool {
    let one_at_scale = eg::pow10::<S>(scale);
    if exponent == 0 {
        return value_raw == one_at_scale;
    }
    let (base_quotient, base_remainder) = base_raw.div_rem(one_at_scale);
    if base_remainder != S::ZERO {
        return false;
    }
    let base_int = base_quotient;
    let abs_exponent = exponent.unsigned_abs();
    let limit_bits = <S as BigInt>::BITS - 4;
    if exponent > 0 {
        // value == base^|k|: require `value` itself integral.
        let (value_quotient, value_remainder) = value_raw.div_rem(one_at_scale);
        if value_remainder != S::ZERO {
            return false;
        }
        let value_int = value_quotient;
        let mut running_power = S::ONE;
        let mut i: u128 = 0;
        while i < abs_exponent {
            if eg::bit_length::<S>(running_power) + eg::bit_length::<S>(base_int) >= limit_bits {
                return false;
            }
            running_power = running_power * base_int;
            i += 1;
        }
        running_power == value_int
    } else {
        // value == 1 / base^|k|: `value_raw · base_int^|k|` must equal the
        // storage `1` exactly.
        let mut running_product = value_raw;
        let mut i: u128 = 0;
        while i < abs_exponent {
            if eg::bit_length::<S>(running_product) + eg::bit_length::<S>(base_int) >= limit_bits
            {
                return false;
            }
            running_product = running_product * base_int;
            i += 1;
        }
        running_product == one_at_scale
    }
}

/// Exact rational-power pin for a near-boundary probe — the narrow shell's
/// `log_rational_pow_pin` (`ln_series_2limb`) made generic over the work
/// integer.
///
/// The boundary CANDIDATE is `2R` rounded to the nearest integer at `scale`
/// — the result in half-ULPs: even = a grid candidate (a directed near-grid
/// case, `log_1.0201(1.01) = 1/2` on every grid from scale 4 up), odd = a
/// half candidate (a nearest near-half tie, `log_4(32) = 5/2` at scale 0).
/// It verifies `log_(b_num/b_den)(x_num/x_den) == n/(2·10^scale)` EXACTLY
/// through the integer identity `x^q == b^p` (`p/q` the candidate in lowest
/// terms; a negative candidate inverts the base fraction), by bounded
/// integer powers — an over-budget candidate defers to the walker, as the
/// narrow shell's does. On a verified candidate the result is decided by
/// exact integer arithmetic: a grid candidate IS the result, a half candidate
/// applies the mode's tie rule. Only a probe inside the near-tie band pays
/// for this.
fn log_rational_pow_pin<const N: usize, Wk: BigInt>(
    raw: Int<N>,
    base_raw: Int<N>,
    scale: u32,
    probe_ratio: Wk,
    working_scale: u32,
    mode: RoundingMode,
) -> Option<Int<N>>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let lit = |n: i128| <Wk as BigInt>::from_i128(n);
    // `2R` at `scale`, rounded half-to-even from the working scale.
    let half_ulp_numerator =
        eg::round_div_pow10::<Wk>(probe_ratio + probe_ratio, working_scale - scale);
    if half_ulp_numerator == Wk::ZERO {
        return None; // R = 0 ⇔ x == 1, pinned upstream
    }
    let is_negative = half_ulp_numerator < Wk::ZERO;
    let abs_numerator = if is_negative { -half_ulp_numerator } else { half_ulp_numerator };
    let one_scaled = eg::pow10::<Wk>(scale);
    let (reduced_num, reduced_den) = reduce_fraction::<Wk>(abs_numerator, one_scaled + one_scaled);
    let (x_num, x_den) = reduce_fraction::<Wk>(BigInt::resize_to::<Wk>(raw), one_scaled);
    let (base_num, base_den) = reduce_fraction::<Wk>(BigInt::resize_to::<Wk>(base_raw), one_scaled);
    // log_b(x) = ±p/q ⇔ x^q == b^(±p): for the negative sign the base
    // fraction inverts.
    let (target_num, target_den) =
        if is_negative { (base_den, base_num) } else { (base_num, base_den) };
    let x_pow_num = pow_bounded::<Wk>(x_num, reduced_den)?;
    let x_pow_den = pow_bounded::<Wk>(x_den, reduced_den)?;
    let base_pow_num = pow_bounded::<Wk>(target_num, reduced_num)?;
    let base_pow_den = pow_bounded::<Wk>(target_den, reduced_num)?;
    if x_pow_num != base_pow_num || x_pow_den != base_pow_den {
        return None;
    }
    // Exact value ±n/(2·10^scale): fold the half-ULP form to storage.
    let (result_magnitude, half) = abs_numerator.div_rem(lit(2));
    let magnitude = if half == Wk::ZERO {
        result_magnitude
    } else {
        // Exactly on the half between `result_magnitude` and
        // `result_magnitude + 1` (magnitude side): the mode's tie rule, by
        // exact integer arithmetic.
        let bump = crate::support::rounding::should_bump(
            mode,
            core::cmp::Ordering::Equal,
            result_magnitude.div_rem(lit(10)).1.to_i128() as u8,
            !is_negative,
        );
        if bump { result_magnitude + Wk::ONE } else { result_magnitude }
    };
    let signed = if is_negative { -magnitude } else { magnitude };
    Some(round_to_storage_with_g::<Int<N>, Wk>(
        signed,
        scale,
        scale,
        mode,
        Int::<N>::MAX,
        Int::<N>::MIN,
    ))
}

/// `base^exponent` in `Wk`, `None` when the result could exceed the bit
/// budget `Wk::BITS − 8`. The bit-length pre-check (`bits(base) · exponent`
/// bounds the product's bits) bounds every square-and-multiply intermediate,
/// so no step can wrap; `1^anything` is exact at any exponent and `0` never
/// verifies (the inputs are domain-asserted positive).
fn pow_bounded<Wk: BigInt>(base: Wk, exponent: Wk) -> Option<Wk> {
    if base == Wk::ZERO {
        return None;
    }
    if base == Wk::ONE {
        return Some(Wk::ONE);
    }
    let base_bits = base.bit_length();
    let budget = <Wk as BigInt>::BITS - 8;
    // `exponent` can be as large as `2·10^scale` (an irreducible candidate
    // denominator), far past `i128` at the wide tiers — compare in `Wk`.
    if exponent > <Wk as BigInt>::from_i128((budget / base_bits) as i128) {
        return None;
    }
    let mut remaining = exponent.to_i128() as u128;
    let mut accumulator = Wk::ONE;
    let mut base_power = base;
    while remaining > 0 {
        if remaining & 1 == 1 {
            accumulator = accumulator * base_power;
        }
        remaining >>= 1;
        if remaining > 0 {
            base_power = base_power * base_power;
        }
    }
    Some(accumulator)
}

/// Greatest common divisor (Euclid) of two non-negative `Wk` values, on the
/// exact per-width divide.
fn gcd<Wk: BigInt>(mut lhs: Wk, mut rhs: Wk) -> Wk
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    while rhs != Wk::ZERO {
        let (_, remainder) = eg::div_rem_exact::<Wk>(lhs, rhs);
        lhs = rhs;
        rhs = remainder;
    }
    lhs
}

/// Reduces `numerator / denominator` (both non-negative) to lowest terms.
fn reduce_fraction<Wk: BigInt>(numerator: Wk, denominator: Wk) -> (Wk, Wk)
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    let common_divisor = gcd::<Wk>(numerator, denominator);
    (
        eg::div_rem_exact::<Wk>(numerator, common_divisor).0,
        eg::div_rem_exact::<Wk>(denominator, common_divisor).0,
    )
}

/// Storage representation of the exact `integer_value` at `scale`
/// (`integer_value · 10^scale`), range-checked into `Int<N>` — panics when
/// it does not fit, matching the walker's own narrowing.
fn exact_int_at_scale<const N: usize, Wk: BigInt>(
    integer_value: i128,
    scale: u32,
    mode: RoundingMode,
) -> Int<N>
where
    <Wk as BigInt>::Scratch: ComputeLimbs,
{
    round_to_storage_with_g::<Int<N>, Wk>(
        eg::scale_by_k::<Wk>(eg::one::<Wk>(scale), integer_value),
        scale,
        scale,
        mode,
        Int::<N>::MAX,
        Int::<N>::MIN,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reference conditioning number by plain decimal digit counting on
    /// `u128` — independent of the `pow10` bisection.
    fn reference_k(base_raw: u128, scale: u32) -> u32 {
        if scale == 0 {
            return 0;
        }
        let one = 10u128.pow(scale);
        let distance = base_raw.abs_diff(one);
        if distance == 0 || distance >= 10u128.pow(scale - 1) {
            return 0;
        }
        let mut digits = 0u32;
        let mut d = distance;
        while d > 0 {
            d /= 10;
            digits += 1;
        }
        scale - (digits - 1)
    }

    #[test]
    fn near_one_digits_matches_decimal_digit_count() {
        // Every scale the narrow tiers reach, a spread of distances on both
        // sides of 1 — powers of ten, one above, one below, and odd values.
        let mut checked = 0u32;
        for scale in [1u32, 2, 4, 6, 19, 30, 38] {
            let one = 10u128.pow(scale);
            let mut distances = vec![1u128, 2, 3, 7, 9, 11, 99, 101, 12_345];
            for j in 0..scale {
                let p = 10u128.pow(j);
                distances.extend([p, p + 1, p.saturating_sub(1).max(1), 3 * p, 9 * p + p / 2]);
            }
            for d in distances {
                if d >= one {
                    continue;
                }
                for raw in [one + d, one - d] {
                    let got = near_one_digits::<Int<2>>(Int::<2>::from_i128(raw as i128), scale);
                    assert_eq!(got, reference_k(raw, scale), "scale {scale} raw {raw}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 0);
    }

    #[test]
    fn ordinary_bases_are_unconditioned() {
        // The bases every fixed-guard shell keeps: k must be 0 so nothing off
        // the near-1 band moves.
        const S: u32 = 19;
        let one = 10i128.pow(S);
        for base in [2 * one, 10 * one, one / 2, 3 * one / 2, 7 * one, 9 * one / 10, 11 * one / 10] {
            assert_eq!(near_one_digits::<Int<2>>(Int::<2>::from_i128(base), S), 0, "base {base}");
        }
        // The two golden rows: k is the exact count the lift is sized from.
        assert_eq!(near_one_digits::<Int<2>>(Int::<2>::from_i128(one + 10), S), 18);
        assert_eq!(near_one_digits::<Int<2>>(Int::<2>::from_i128(one + 10_000), S), 15);
        assert_eq!(near_one_digits::<Int<2>>(Int::<2>::from_i128(one - 10_000), S), 15);
        // Scale 0: an integer base is never near 1.
        assert_eq!(near_one_digits::<Int<2>>(Int::<2>::from_i128(2), 0), 0);
    }

    /// The pin's inputs as the kernel forms them: `probe_ratio` at working
    /// scale `w = scale + guard`, perturbed by `noise` working units so the
    /// candidate is reached from BOTH sides of the boundary, as a kernel
    /// residual would land it.
    fn pin<const N: usize>(
        x_raw: i128,
        base_raw: i128,
        scale: u32,
        ratio_units_at_scale_w: i128,
        noise: i128,
        mode: RoundingMode,
    ) -> Option<i128> {
        const GUARD: u32 = 30;
        let w = scale + GUARD;
        let probe = <Int<24> as BigInt>::from_i128(ratio_units_at_scale_w + noise);
        log_rational_pow_pin::<N, Int<24>>(
            Int::<N>::from_i128(x_raw),
            Int::<N>::from_i128(base_raw),
            scale,
            probe,
            w,
            mode,
        )
        .map(|v| v.as_i128())
    }

    const ALL_MODES: [RoundingMode; 8] = [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
        RoundingMode::AwayFromZero,
        RoundingMode::ZeroFiveUp,
    ];

    #[test]
    fn rational_pin_decides_the_golden_grid_cases_under_every_mode() {
        // log_1.0201(1.01) = 1/2 and log_1.0201(1.030301) = 3/2 at scale 6:
        // on-grid rationals, so every mode must return exactly 0.500000 /
        // 1.500000 whether the probe sits a few working units below or
        // above the grid line — the golden 33893684900 failure was the
        // walker returning 1 ULP off under a directed mode on exactly this.
        const S: u32 = 6;
        let one = 10i128.pow(S);
        let w_units = 10i128.pow(S + 30);
        for noise in [-7, -1, 1, 7] {
            for mode in ALL_MODES {
                assert_eq!(
                    pin::<2>(1_010_000, 1_020_100, S, w_units / 2, noise, mode),
                    Some(one / 2),
                    "log_1.0201(1.01) noise {noise} mode {mode:?}"
                );
                assert_eq!(
                    pin::<2>(1_030_301, 1_020_100, S, 3 * w_units / 2, noise, mode),
                    Some(3 * one / 2),
                    "log_1.0201(1.030301) noise {noise} mode {mode:?}"
                );
                // Below 1: log_0.99(0.9801) = 2 — an integer, but reachable
                // here too when the integer pin is bypassed.
                assert_eq!(
                    pin::<2>(980_100, 990_000, S, 2 * w_units, noise, mode),
                    Some(2 * one),
                    "log_0.99(0.9801) noise {noise} mode {mode:?}"
                );
            }
        }
    }

    #[test]
    fn rational_pin_half_tie_takes_the_mode_rule() {
        // log_4(32) = 5/2 EXACTLY at scale 0: a genuine tie between 2 and 3
        // that only exact integer arithmetic can certify.
        let w_units = 10i128.pow(30);
        let ratio = 5 * w_units / 2;
        assert_eq!(pin::<2>(32, 4, 0, ratio, 3, RoundingMode::HalfToEven), Some(2));
        assert_eq!(pin::<2>(32, 4, 0, ratio, -3, RoundingMode::HalfAwayFromZero), Some(3));
        assert_eq!(pin::<2>(32, 4, 0, ratio, 3, RoundingMode::HalfTowardZero), Some(2));
        assert_eq!(pin::<2>(32, 4, 0, ratio, -3, RoundingMode::Ceiling), Some(3));
        assert_eq!(pin::<2>(32, 4, 0, ratio, 3, RoundingMode::Floor), Some(2));
        // log_16(8) = 3/4: a half tie at scale 1 (7.5 tenths).
        let w1 = 10i128.pow(31);
        assert_eq!(pin::<2>(80, 160, 1, 3 * w1 / 4, 2, RoundingMode::HalfToEven), Some(8));
        assert_eq!(pin::<2>(80, 160, 1, 3 * w1 / 4, -2, RoundingMode::HalfTowardZero), Some(7));
    }

    #[test]
    fn rational_pin_declines_what_it_cannot_certify() {
        const S: u32 = 6;
        let w_units = 10i128.pow(S + 30);
        // A candidate that is NOT an exact power: log_1.0201(1.0101) ≈ 0.505,
        // probe placed exactly on the 0.5 grid line — must defer, never pin.
        for mode in ALL_MODES {
            assert_eq!(pin::<2>(1_010_100, 1_020_100, S, w_units / 2, 0, mode), None);
        }
        // A candidate whose verification would exceed the work integer's
        // bit budget defers as well: `2R` rounding to `10^6 + 1` half-ULPs
        // is the irreducible `(10^6 + 1)/(2·10^6)`, and `101^(2·10^6)` is far
        // past Int<24>'s 1528 bits.
        assert_eq!(
            pin::<2>(1_010_000, 1_020_100, S, w_units / 2 + 5 * 10i128.pow(29), 0, RoundingMode::Floor),
            None
        );
        // pow_bounded itself.
        let two = <Int<24> as BigInt>::from_i128(2);
        assert_eq!(pow_bounded::<Int<24>>(two, <Int<24> as BigInt>::from_i128(10)).map(|v| v.as_i128()), Some(1024));
        assert_eq!(pow_bounded::<Int<24>>(<Int<24> as BigInt>::ONE, <Int<24> as BigInt>::from_i128(i128::MAX)).map(|v| v.as_i128()), Some(1));
        assert_eq!(pow_bounded::<Int<24>>(two, <Int<24> as BigInt>::from_i128(2_000)), None);
        assert_eq!(pow_bounded::<Int<24>>(<Int<24> as BigInt>::ZERO, two), None);
    }

    /// The narrow conditioned arm end to end, through the policy exactly as
    /// the golden gate reaches it, against the narrow `Fixed` shell on the
    /// same inputs. Both are correctly rounded, so they must agree bit for
    /// bit — and the conditioned arm must not panic where the fixed shell
    /// returns a value (golden 33895788589: every conditioned narrow row
    /// panicked at every scale and mode while the wide arms were clean).
    #[test]
    fn narrow_conditioned_arm_agrees_with_the_fixed_shell() {
        fn check<const SCALE: u32>(x: i128, base: i128) {
            let one = 10i128.pow(SCALE);
            let (raw, braw) = (Int::<2>::from_i128(x * one), Int::<2>::from_i128(base));
            for mode in ALL_MODES {
                let conditioned = crate::policy::log::checked_dispatch::<2, SCALE>(raw, braw, mode);
                let fixed = log_ln_divide_d38::<SCALE>(raw, braw, mode);
                assert_eq!(conditioned, fixed, "x={x} base_raw={base} scale={SCALE} mode={mode:?}");
                assert!(conditioned.is_some(), "x={x} base_raw={base} scale={SCALE}: in range");
            }
        }
        // bases within 0.1 of 1 (k >= 2), both sides, the rows the gate ran:
        // 1.01, 0.95, 1.05, 1.0001, 1.000001 (as raw storage at each scale).
        check::<2>(2, 101);
        check::<2>(2, 95);
        check::<2>(2, 105);
        check::<6>(2, 1_010_000);
        check::<6>(2, 950_000);
        check::<6>(2, 1_050_000);
        check::<6>(2, 1_000_100);
        check::<6>(2, 1_000_001);
        check::<6>(7, 1_001_000);
        check::<19>(2, 10_000_000_000_000_000_000 + 100_000_000_000_000_000);
        check::<19>(9_999_999, 10_000_000_000_000_000_000 + 100_000_000);
        // Past storage: log_1.01(2) = 69.66 does not fit D38<37> (6.97e38 >
        // i128::MAX). The narrow contract is `None`, never a panic, and the
        // fixed shell says the same.
        let one37 = 10i128.pow(37);
        let (raw, braw) = (Int::<2>::from_i128(2 * one37), Int::<2>::from_i128(101 * 10i128.pow(35)));
        for mode in ALL_MODES {
            assert_eq!(crate::policy::log::checked_dispatch::<2, 37>(raw, braw, mode), None, "D38<37> range {mode:?}");
            assert_eq!(log_ln_divide_d38::<37>(raw, braw, mode), None, "fixed shell agrees {mode:?}");
        }
        // D18: its own storage as the walker's target, against the fixed path
        // (widen to Int<2>, the D38 shell, narrow back).
        let one9 = 10i128.pow(9);
        for (x, base) in [(2i128, 101 * 10i128.pow(7)), (3, 95 * 10i128.pow(7)), (7, 1_001_000_000)] {
            let (raw1, braw1) = (Int::<1>::from_i128(x * one9), Int::<1>::from_i128(base));
            let (raw2, braw2) = (Int::<2>::from_i128(x * one9), Int::<2>::from_i128(base));
            for mode in ALL_MODES {
                let conditioned = crate::policy::log::checked_dispatch::<1, 9>(raw1, braw1, mode);
                let fixed = log_ln_divide_d38::<9>(raw2, braw2, mode).and_then(crate::policy::narrow_fit::<1>);
                assert_eq!(conditioned, fixed, "D18<9> x={x} base_raw={base} mode={mode:?}");
                assert!(conditioned.is_some());
            }
        }
    }

    #[test]
    fn lift_and_budget_arithmetic() {
        assert_eq!(lifted_guard(0), COMPOSITION_GUARD);
        assert_eq!(lifted_guard(18), 48);
        // D57 s28, k = 28: inside Wagm's (Int<16>) budget under the
        // reformulated lift (126 <= 128); D76 s38, k = 38 is past it and
        // inside Wexp's (Int<32>).
        assert!(fits_budget(28, 28, 16));
        assert!(!fits_budget(38, 38, 16));
        assert!(fits_budget(38, 38, 32));
        // Capacity at the corner the assert guards: D462 s461, k = 461 fits
        // Wexp Int<128> (1944 <= 2432); Wagm Int<64> could not hold it.
        assert!(fits_capacity(461, 461, 128));
        assert!(!fits_capacity(461, 461, 64));
    }

    /// `g(ε)·ε` must track `ln(1+ε)` to a few working units at every band
    /// position, both sides of 1 — `g` is computed by its own series from
    /// the exact `d`, never from `ln`, so this is the one place the two
    /// meet. The reference is the crate's own generic Series kernel run at
    /// DOUBLE the working scale and scaled down, so its own truncation
    /// (tens of units at `w` for an input just below 1 — two Brent sqrt
    /// levels and a ~w/2-term artanh) is far below a unit at `w`; what
    /// remains is `g`'s own rounding, a unit or two. A disagreement beyond
    /// that would be a wrong series, not a wrong oracle.
    #[test]
    fn g_of_epsilon_times_epsilon_tracks_ln1p() {
        type W = Int<24>;
        const S: u32 = 6;
        const WS: u32 = 40; // working scale
        const WS2: u32 = 80; // the reference's working scale
        let one_scaled = eg::pow10::<W>(S);
        let one_w2 = eg::one::<W>(WS2);
        for d in [100_000i128, 50_000, 10_000, 1, -1, -20_000, -99_999] {
            let d_w = <W as BigInt>::from_i128(d);
            let g = g_of_epsilon::<W>(d_w, one_scaled, WS);
            let eps_w = d_w * eg::pow10::<W>(WS - S); // ε at scale w, exact
            let lhs = eg::mul::<W>(g, eps_w, WS);
            let eps_w2 = d_w * eg::pow10::<W>(WS2 - S);
            let ln2 = crate::consts::ln2_by_working_scale::<W>(WS2, RoundingMode::HalfToEven);
            let rhs = eg::round_div_pow10::<W>(eg::ln_fixed::<W>(one_w2 + eps_w2, WS2, ln2), WS2 - WS);
            let diff = (lhs - rhs).abs().to_i128();
            assert!(diff <= 4, "d={d}: g·ε and ln(1+ε) differ by {diff} units at w={WS}");
        }
    }

    /// The live form at lift `k`, the base-offset-only form at lift `k` and
    /// the naive quotient at lift `2k` are all correctly rounded on these
    /// inputs (every deciding digit is within the walker's reach), so
    /// through the same finish they must agree bit for bit under every mode
    /// — the two kept alternatives as the references. Where the live form
    /// and the base-offset-only form part company is the deep doubly-near-1
    /// residue, covered by `doubly_near_one_residue_is_decided_from_both_exact_offsets`.
    #[test]
    fn reformulated_ratio_agrees_with_the_kept_references() {
        type W = Int<24>;
        fn check<const SCALE: u32>(x_raw: i128, base_raw: i128) {
            let (raw, braw) = (Int::<2>::from_i128(x_raw), Int::<2>::from_i128(base_raw));
            let k = near_one_digits::<Int<2>>(braw, SCALE);
            assert!(k > 0, "not a near-1 base");
            let ln_at = series_core::<W, SCALE>();
            let (g_guard, q_guard) = (COMPOSITION_GUARD + k, COMPOSITION_GUARD + 2 * k);
            let probe_g = conditioned_ratio::<2, W, SCALE>(raw, braw, g_guard, &ln_at);
            let probe_o = conditioned_ratio_offset_base::<2, W, SCALE>(raw, braw, g_guard, &ln_at);
            let probe_q = conditioned_ratio_quotient::<2, W, SCALE>(raw, braw, q_guard, &ln_at);
            for mode in ALL_MODES {
                let a = conditioned_finish::<2, W, SCALE>(raw, braw, mode, g_guard, &ln_at, probe_g);
                let o = conditioned_finish::<2, W, SCALE>(raw, braw, mode, g_guard, &ln_at, probe_o);
                let b = conditioned_finish::<2, W, SCALE>(raw, braw, mode, q_guard, &ln_at, probe_q);
                assert_eq!(a, o, "vs base-offset-only: x={x_raw} base={base_raw} scale={SCALE} mode={mode:?}");
                assert_eq!(a, b, "vs naive quotient: x={x_raw} base={base_raw} scale={SCALE} mode={mode:?}");
            }
        }
        let one6 = 10i128.pow(6);
        check::<6>(2 * one6, 1_010_000);
        check::<6>(2 * one6, 950_000);
        check::<6>(7 * one6, 1_001_000);
        check::<6>(2 * one6, 1_000_001);
        check::<6>(1_010_000, 1_020_100); // exact 1/2 — the pin, both ways
        let one19 = 10i128.pow(19);
        check::<19>(2 * one19, one19 + 100); // k = 17
        check::<19>(3 * one19, one19 - 100); // below 1
        check::<19>(one19 + 10_000_000, one19 + 10_000); // x near 1 too
        check::<19>(9_999_999 * one19, one19 + 100_000_000); // k = 11, ln x ~ 16
    }

    /// Both arguments within 0.1 of 1 — the deep directed hard case of the
    /// power-of-ten family `x = 1 + 10^-p`, `b = 1 + 10^-q`. The visible
    /// terms of `ln x / ln b` land exactly on a grid line and the deciding
    /// term is `(a/ε)·a²/3`-sized, `3p - q` digits down. Holding `ln x` at
    /// absolute resolution needs `w > 3p` to see it; from both exact offsets
    /// (`d_x · g(a) / (d_b · g(ε))`) the quotient's error is
    /// `10^(log10(d_x/d_b) - w)`, so `w > 2p` does — inside the base probe.
    /// Golden 33918031518 (D924 s900, `x = 1 + 10^-682`, `b = 1 + 10^-700`):
    /// the residue `+3.3·10^-447` ULP needed `w = 2047` the old way, past
    /// the `Int<256>` walker cap of 2022, so the unresolved endgame handed
    /// the grid value to Ceiling / AwayFromZero / ZeroFiveUp. The same shape
    /// at unit size: storage `Int<3>`, scale 45, work `Int<16>` (walker cap
    /// `128 - int_digits - 8`); every threshold below is from a fixed-point
    /// simulation of both formulations against flint.
    #[test]
    fn doubly_near_one_residue_is_decided_from_both_exact_offsets() {
        type St = Int<3>;
        type W = Int<16>;
        const S: u32 = 45;
        let p10 = |n: u32| eg::pow10::<St>(n);
        let lit = |v: i128| eg::lit::<St>(v);
        // (p, q, nearest grid line, value above it?, ZeroFiveUp's answer):
        // 1. p 43, q 42: R ≈ 0.1, the value 10^-40 ULP BELOW 10^44 + 45
        //    (`-aε/4 - ε²/12`): Floor / Trunc / ZeroFiveUp take the line
        //    below. Old way resolves from w = 126 > cap 120; new from 85.
        // 2. p 43, q 45: R ≈ 100, 10^-39 ULP ABOVE 10^47 - 4950 (`+a²/3`):
        //    Ceiling / AwayFromZero / ZeroFiveUp bump. 130 > cap 117; new 87.
        // 3. p 44, q 45: R ≈ 10, 10^-42 ULP above 10^46 - 45. 133 > 118; 89.
        let cases: [(u32, u32, St, bool, St); 3] = [
            (43, 42, p10(44) + lit(45), false, p10(44) + lit(44)),
            (43, 45, p10(47) - lit(4950), true, p10(47) - lit(4949)),
            (44, 45, p10(46) - lit(45), true, p10(46) - lit(44)),
        ];
        for (p, q, grid, above, zero_five_up) in cases {
            let (raw, braw) = (p10(S) + p10(S - p), p10(S) + p10(S - q));
            assert_eq!(near_one_digits::<St>(raw, S), p);
            let k = near_one_digits::<St>(braw, S);
            assert_eq!(k, q);
            let guard = lifted_guard(k);
            let ln_at = series_core::<W, S>();
            let probe = conditioned_probe::<3, W, S>(raw, braw, guard, &ln_at);
            let (below_line, above_line) = if above { (grid, grid + lit(1)) } else { (grid - lit(1), grid) };
            for mode in ALL_MODES {
                let expected = match mode {
                    RoundingMode::Floor | RoundingMode::Trunc => below_line,
                    RoundingMode::Ceiling | RoundingMode::AwayFromZero => above_line,
                    RoundingMode::ZeroFiveUp => zero_five_up,
                    _ => grid,
                };
                let got = conditioned_finish::<3, W, S>(raw, braw, mode, guard, &ln_at, probe);
                assert_eq!(got, expected, "p={p} q={q} mode={mode:?}");
            }
        }
    }

    /// Exact rationals through the FIXED shell — a base 0.1 or more from 1
    /// (`k = 0`), so `policy::log` routes `LnDivide`, the wide tiers' macro
    /// shell. `log_b(x)` for decimal `x` and `b` is rational or
    /// transcendental (Gelfond–Schneider); on a terminating rational the
    /// working residual at every depth is kernel noise around the grid
    /// line, so the directed walker never converges and its unresolved
    /// endgame returns the base probe's narrowing — 1 ULP off under a
    /// directed mode wherever that noise lands on the wrong side, cell by
    /// cell. Every cell, every mode, exact — the rational-power pin the
    /// narrow `Fixed` shell already carries, on the wide shell too.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[test]
    fn wide_fixed_shell_decides_exact_rationals_under_every_mode() {
        fn digits(m: i128) -> u32 {
            let (mut d, mut v) = (0u32, m.unsigned_abs());
            while v > 0 {
                v /= 10;
                d += 1;
            }
            d.max(1)
        }
        fn cell<const N: usize, const SCALE: u32>(
            misses: &mut Vec<String>,
            max_digits: u32,
            x: (i128, u32),
            base: (i128, u32),
            expected: (i128, u32),
        ) {
            // The harness's own rule: a value needing more significant
            // digits than the tier holds at this scale is not a cell.
            let fits = |(m, f): (i128, u32)| SCALE >= f && digits(m) + SCALE - f < max_digits;
            if !(fits(x) && fits(base) && fits(expected)) {
                return;
            }
            // (mantissa, fraction digits) → raw at SCALE.
            let at = |(m, f): (i128, u32)| Int::<N>::from_i128(m) * eg::pow10::<Int<N>>(SCALE - f);
            let (raw, braw, want) = (at(x), at(base), at(expected));
            for mode in ALL_MODES {
                let got = crate::policy::log::checked_dispatch::<N, SCALE>(raw, braw, mode);
                if got != Some(want) {
                    misses.push(format!(
                        "N={N} scale={SCALE} x={x:?} base={base:?} mode={mode:?}: got {got:?}, want {want:?}"
                    ));
                }
            }
        }
        fn scale<const N: usize, const SCALE: u32>(misses: &mut Vec<String>, max_digits: u32) {
            // x, base, log_base(x) — each as (mantissa, fraction digits).
            let d = max_digits;
            cell::<N, SCALE>(misses, d, (11, 1), (121, 2), (5, 1)); // 1/2
            cell::<N, SCALE>(misses, d, (1331, 3), (121, 2), (15, 1)); // 3/2
            cell::<N, SCALE>(misses, d, (1728, 3), (144, 2), (15, 1)); // 3/2
            cell::<N, SCALE>(misses, d, (12, 1), (144, 2), (5, 1)); // 1/2
            cell::<N, SCALE>(misses, d, (2, 0), (4, 0), (5, 1)); // 1/2
            cell::<N, SCALE>(misses, d, (8, 0), (16, 0), (75, 2)); // 3/4
            cell::<N, SCALE>(misses, d, (2, 0), (16, 0), (25, 2)); // 1/4
            cell::<N, SCALE>(misses, d, (8, 0), (256, 0), (375, 3)); // 3/8
            cell::<N, SCALE>(misses, d, (15, 1), (225, 2), (5, 1)); // 1/2
            cell::<N, SCALE>(misses, d, (5, 1), (25, 2), (5, 1)); // 1/2, base below 1
            cell::<N, SCALE>(misses, d, (2, 0), (25, 2), (-5, 1)); // -1/2
            cell::<N, SCALE>(misses, d, (1, 1), (100, 0), (-5, 1)); // -1/2, x below 1
            cell::<N, SCALE>(misses, d, (729, 3), (81, 2), (15, 1)); // 3/2, both below 1
            cell::<N, SCALE>(misses, d, (100000, 0), (100, 0), (25, 1)); // 5/2
        }
        let mut misses = Vec::new();
        scale::<3, 3>(&mut misses, 57);
        scale::<3, 14>(&mut misses, 57);
        scale::<3, 28>(&mut misses, 57);
        scale::<3, 42>(&mut misses, 57);
        scale::<3, 53>(&mut misses, 57);
        assert!(misses.is_empty(), "{} mis-rounded cells:\n{}", misses.len(), misses.join("\n"));
    }

    /// The wide fixed shell now finishes through `log_ratio_finish`. On an
    /// input no pin takes — the overwhelming majority of `log` calls — that
    /// must be BIT-IDENTICAL to the sequence it replaced, the exact-power
    /// pin then the walker, which this test rebuilds from the tier's own
    /// helpers: same probe, same recompute, same bounds. Ordinary bases on
    /// both sides of 1, arguments across the range, every mode, scales
    /// across the tier — including results that sit close to a grid line,
    /// where the finish's single shot declines, its rational pin declines,
    /// and the walker escalates exactly as before.
    #[cfg(any(feature = "d57", feature = "wide"))]
    #[test]
    fn wide_fixed_shell_finish_is_bit_identical_on_ordinary_inputs() {
        use crate::types::widths::wide_trig_d57 as core;
        fn check<const SCALE: u32>(x: (i128, u32), base: (i128, u32)) {
            let at = |(m, f): (i128, u32)| Int::<3>::from_i128(m) * eg::pow10::<Int<3>>(SCALE - f);
            let (raw, braw) = (at(x), at(base));
            let base_working_scale = SCALE + core::GUARD;
            let ratio_at = |guard_digits: u32| {
                let working_scale = SCALE + guard_digits;
                core::div_agm(
                    core::ln_fixed_routed_agm::<SCALE>(core::to_work_scaled_agm(raw, guard_digits), working_scale),
                    core::ln_fixed_routed_agm::<SCALE>(core::to_work_scaled_agm(braw, guard_digits), working_scale),
                    working_scale,
                )
            };
            let probe = ratio_at(core::GUARD);
            for mode in ALL_MODES {
                // The sequence the shell ran before this finish.
                let exponent = core::round_to_nearest_int_agm(probe, base_working_scale);
                let before = if core::log_is_exact_int::<core::Wagm>(
                    core::to_work_scaled_agm(raw, 0),
                    core::to_work_scaled_agm(braw, 0),
                    SCALE,
                    exponent,
                ) {
                    core::exact_int_at_scale(exponent, SCALE)
                } else {
                    core::round_to_storage_directed::<core::Wagm>(core::GUARD, SCALE, mode, |guard_digits| {
                        if guard_digits == core::GUARD {
                            probe
                        } else {
                            ratio_at(guard_digits)
                        }
                    })
                };
                let now = core::log_strict_with_kernel::<SCALE>(raw, braw, mode);
                assert_eq!(now, before, "x={x:?} base={base:?} scale={SCALE} mode={mode:?}");
            }
        }
        fn scale<const SCALE: u32>() {
            for base in [(2, 0), (10, 0), (5, 1), (3, 0), (725, 2), (13, 1), (1000, 0)] {
                for x in [(2, 0), (3, 0), (725, 2), (3, 1), (123456789, 3), (15, 1), (1, 2), (8, 0), (1024, 0)] {
                    check::<SCALE>(x, base);
                }
            }
        }
        scale::<5>();
        scale::<20>();
        scale::<40>();
    }
}

