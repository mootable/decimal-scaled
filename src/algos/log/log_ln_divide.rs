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
//!   (`k = ceil(-log10 |b - 1|)`): `w = SCALE + 30 + 2k`. Routed iff
//!   `k > 0`, i.e. the base lies within 0.1 of 1.
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
//! `w = SCALE + 30 + 2k` outruns a tier's fixed composition width in its
//! worst band (`k` runs up to `SCALE`, the base's own representability).
//! The policy therefore chooses the work integer from `k`
//! ([`fits_budget`] / [`fits_capacity`]) — the matcher's width axis, keyed
//! on the value — and the kernel takes its natural-log core from the
//! caller ([`series_core`] / [`tang_core`]) so `policy::ln`'s Tang/Series
//! verdict is honoured, with Series where the lifted scale exceeds the
//! baked Tang table's reach ([`tang_table_reaches`]).

use crate::algos::exp::exp_generic as eg;
use crate::algos::support::wide_trig_core::{
    round_to_storage_directed_g, round_to_storage_with_g, to_work_scaled_g,
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

// ── The conditioned composition ─────────────────────────────────────────

/// Guard digits of the `ln(x)/ln(b)` composition for an ordinary base — the
/// value every fixed-guard shell runs at (`decl_wide_transcendental!`'s
/// `GUARD`, `ln_series_2limb::STRICT_GUARD`). The conditioned shell adds
/// `2k` on top ([`lifted_guard`]).
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
/// `k`: `30 + 2k`. One `k` restores the relative precision `ln b` lost,
/// the other covers the result's own magnitude `ln x · 10^k` — see the
/// module doc. At `k = 0` it is the fixed guard exactly.
pub(crate) const fn lifted_guard(near_one_digits: u32) -> u32 {
    COMPOSITION_GUARD + 2 * near_one_digits
}

/// Decimal digits the conditioned composition asks of its work integer at
/// `(scale, k)`, on the `8 · limbs` scale the rung ladder and the Ziv
/// walker size by (`work_rung`; `cap_digits = BITS/8 - int_digits - 8` in
/// `round_to_storage_directed_tagged_impl_g`): the working scale
/// `scale + 30 + 2k`, the result's integer digits (`≤ k + 4` — `ln x` is
/// below `10^4` at every width), and the walker's own `8`. A width that
/// meets it leaves the walker its ordinary escalation headroom above the
/// lifted base guard.
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
/// on [`tang_table_reaches`].
#[cfg(feature = "_wide-support")]
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
/// storage `Int<N>` in the work integer `Wk`.
///
/// The shell is the wide fixed-guard `log_strict_with_kernel`, verbatim —
/// the domain walls, the exact integer-power pin, the Ziv-escalated
/// directed narrowing — with `guard` (from [`lifted_guard`]) where that
/// shell has the const `GUARD`, and the natural-log core `ln_at`
/// (`working value at w, w -> ln at w`) supplied by the caller so the
/// matcher's `ln` routing is honoured. Panics on a non-positive argument
/// or base, on `base == 1`, and — like every wide shell — when the result
/// does not fit `Int<N>`; a caller that needs `None` instead computes on a
/// wider `N` and fits the result down (`policy::log`'s narrow arm).
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
    if raw <= Int::<N>::ZERO {
        panic!("log: argument must be positive");
    }
    if base_raw <= Int::<N>::ZERO {
        panic!("log: base must be positive");
    }
    if base_raw == eg::pow10::<Int<N>>(SCALE) {
        panic!("log: base must not equal 1");
    }
    let base_working_scale = SCALE + guard;
    let ln_of = |value: Int<N>, guard_digits: u32| -> Wk {
        ln_at(to_work_scaled_g::<Int<N>, Wk>(value, guard_digits), SCALE + guard_digits)
    };
    let probe_ratio =
        eg::div::<Wk>(ln_of(raw, guard), ln_of(base_raw, guard), base_working_scale);
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
            let working_scale = SCALE + guard_digits;
            eg::div::<Wk>(ln_of(raw, guard_digits), ln_of(base_raw, guard_digits), working_scale)
        },
    )
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

    #[test]
    fn lift_and_budget_arithmetic() {
        assert_eq!(lifted_guard(0), COMPOSITION_GUARD);
        assert_eq!(lifted_guard(18), 66);
        // D57 s28, k = 28: past Wagm's (Int<16>) budget, inside Wexp's (Int<32>).
        assert!(!fits_budget(28, 28, 16));
        assert!(fits_budget(28, 28, 32));
        // Capacity at the absurd corner the assert guards: D462 s461, k = 461
        // fits Int<176> but not Int<128>.
        assert!(!fits_capacity(461, 461, 128));
        assert!(fits_capacity(461, 461, 176));
    }
}

