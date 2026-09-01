// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Width-generic guard-digit `exp` core (always compiled).
//!
//! The per-tier `$core` modules (emitted by
//! [`decl_wide_transcendental!`](crate::macros::wide_transcendental))
//! bind an `exp_fixed` to one work integer `W`. Near the storage-overflow
//! edge — a large `e^|x|` whose integer-digit growth, the internal
//! `2^k`-reassembly lift, AND the repeated-squaring peak together exceed
//! `W`'s decimal capacity — the value can no longer be held at the
//! precision needed to round correctly.
//!
//! This module lifts the `exp_fixed` body out to a free function generic
//! over any [`BigInt`] integer `S` (whose scratch carrier impls [`ComputeLimbs`]), so a large-result
//! cell can run it in a WIDER work integer (the wide tiers' `Wexp`, or the
//! narrow D18/D38 tier's wider work integer) where the full lift + squaring
//! peak fit, then narrow correctly-rounded back to storage. The module is
//! always compiled (NOT gated behind `_wide-support`) precisely so the
//! narrow default build reaches it for the integer-regime D38 cells whose
//! 256-bit `Fixed` intermediate cannot host the lift.

use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

    /// Hard cap on series iterations — a safety net; every series
    /// terminates far sooner by reaching a zero term. Shared with the
    /// width-generic trig kernels (`algos::trig::trig_generic`), which
    /// mirror the per-tier cores' identical cap.
    pub(crate) const SERIES_CAP: u128 = 20_000;

    #[inline]
    pub(crate) fn lit<S: BigInt>(value: i128) -> S {
        S::from_i128(value)
    }
    #[inline]
    pub(crate) fn zero<S: BigInt>() -> S {
        S::ZERO
    }
    #[inline]
    pub(crate) fn pow10<S: BigInt>(exponent: u32) -> S {
        crate::consts::pow10::dispatch::<S>(exponent)
    }
    #[inline]
    pub(crate) fn one<S: BigInt>(working_scale: u32) -> S {
        pow10::<S>(working_scale)
    }
    /// Bit length of `|v|` (0 for zero).
    pub(crate) fn bit_length<S: BigInt>(value: S) -> u32 {
        <S as BigInt>::BITS - value.abs().leading_zeros()
    }
    /// Unpacks a non-negative `S` magnitude into a little-endian u64 limb
    /// buffer through the trait's u128 magnitude exit (`mag_into_u128`).
    /// `dst` must be freshly zeroed and at least `S`'s width.
    pub(crate) fn unpack_mag<S: BigInt>(value: S, dst: &mut [u64])
    where
        S::Scratch: ComputeLimbs,
    {
        let mut limbs = <S::Scratch as ComputeLimbs>::single_u128();
        value.mag_into_u128(limbs.as_mut());
        let mut i = 0;
        for &chunk in limbs.as_ref() {
            if i < dst.len() {
                dst[i] = chunk as u64;
                i += 1;
            }
            if i < dst.len() {
                dst[i] = (chunk >> 64) as u64;
                i += 1;
            }
        }
    }

    /// `div_rem` with EXACT per-width Knuth scratch — the value-generic
    /// divide the guard-digit kernels and the Ziv walkers route through.
    ///
    /// The blanket `Int<N>::div_rem` operator sizes its Knuth
    /// normalisation scratch from the build-max `MAX_WORK_N` blanket,
    /// which the narrow default build keeps at the STORAGE width
    /// (2 limbs, `MAX_SINGLE_LIMBS = 10`) — far below the `Int<24>` work
    /// integer the narrow near-tie Ziv escalation probes in (numerators
    /// up to the full 24 limbs). Here the scratch comes from `S`'s own
    /// carrier — `single_buffered_u64()` is exactly Knuth's
    /// `num.len() + 2` normalised-dividend requirement — so the divide is
    /// exact-per-width at EVERY build instead of leaning on the blanket
    /// (the exact-scratch migration the `compute_limbs` blanket docs call
    /// for). Engine choice follows the divide matcher's own
    /// `select_for_limbs` verdict; the u128-limb refinement (divisors of
    /// ≥ 24 limbs, which no narrow probe and no in-range work value here
    /// produces) falls to the value-identical base-2⁶⁴ Knuth. Truncated
    /// semantics, identical to `Int::div_rem`.
    pub(crate) fn div_rem_exact<S: BigInt>(numerator: S, divisor: S) -> (S, S)
    where
        S::Scratch: ComputeLimbs,
    {
        use crate::int::policy::div_rem::{select_for_limbs, Algorithm};
        let numerator_is_negative = numerator < S::ZERO;
        let divisor_is_negative = divisor < S::ZERO;
        let mut numerator_limbs = <S::Scratch as ComputeLimbs>::single_u64();
        let mut divisor_limbs = <S::Scratch as ComputeLimbs>::single_u64();
        unpack_mag(numerator.abs(), numerator_limbs.as_mut());
        unpack_mag(divisor.abs(), divisor_limbs.as_mut());
        let mut quotient_limbs = <S::Scratch as ComputeLimbs>::single_u64();
        let mut remainder_limbs = <S::Scratch as ComputeLimbs>::single_u64();
        match select_for_limbs(numerator_limbs.as_ref(), divisor_limbs.as_ref()) {
            // Single-limb divisor: the hardware remainder engine, no
            // normalisation scratch involved.
            Algorithm::Rem => crate::int::algos::div::div_rem::div_rem(
                numerator_limbs.as_ref(),
                divisor_limbs.as_ref(),
                quotient_limbs.as_mut(),
                remainder_limbs.as_mut(),
            ),
            // Knuth — with exact caller-sized scratch (see above).
            _ => {
                let mut dividend_scratch = <S::Scratch as ComputeLimbs>::single_buffered_u64();
                let mut divisor_scratch = <S::Scratch as ComputeLimbs>::single_buffered_u64();
                crate::int::algos::div::div_knuth::div_knuth_into(
                    numerator_limbs.as_ref(),
                    divisor_limbs.as_ref(),
                    quotient_limbs.as_mut(),
                    remainder_limbs.as_mut(),
                    dividend_scratch.as_mut(),
                    divisor_scratch.as_mut(),
                );
            }
        }
        let quotient = S::from_mag_sign_u64(
            quotient_limbs.as_ref(), numerator_is_negative != divisor_is_negative);
        let remainder = S::from_mag_sign_u64(remainder_limbs.as_ref(), numerator_is_negative);
        (quotient, remainder)
    }

    /// Half-to-even round of `numerator / divisor` for `S`.
    #[inline]
    pub(crate) fn round_div<S: BigInt>(numerator: S, divisor: S) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        round_div_sided(numerator, divisor).0
    }

    /// [`round_div`] plus which side of the returned quotient the EXACT
    /// quotient lies on — `Above` when the rounding went DOWN (the true
    /// quotient is larger than the value handed back), `Below` when it went
    /// UP, `None` when the division came out exact and there is no side.
    ///
    /// The quotient is bit-identical to [`round_div`]'s: that function is
    /// this one with the side dropped, so there is a single rounding body.
    /// The side costs two boolean tests and no extra division — the rounding
    /// decision it reads has already been made.
    ///
    /// # The polarity is READ, never assumed
    ///
    /// [`div_rem_exact`] truncates toward zero and its remainder carries the
    /// numerator's sign, so the exact quotient always sits
    /// `|remainder/divisor| < 1` on the AWAY-from-zero side of `quotient`.
    /// Keeping `quotient` therefore leaves the truth
    /// away from zero of the answer; bumping steps a full unit — necessarily
    /// past it — and leaves the truth on the toward-zero side. Both
    /// polarities occur at both signs, because this rounds HALF-TO-EVEN and
    /// not toward zero, so a fixed direction would be wrong wherever the
    /// residual passes the half.
    #[inline]
    fn round_div_sided<S: BigInt>(numerator: S, divisor: S) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        let (quotient, remainder) = div_rem_exact(numerator, divisor);
        if remainder == S::ZERO {
            return (quotient, None);
        }
        let abs_remainder = remainder.abs();
        let complement = divisor.abs() - abs_remainder;
        let remainder_cmp = if abs_remainder < complement {
            ::core::cmp::Ordering::Less
        } else if abs_remainder > complement {
            ::core::cmp::Ordering::Greater
        } else {
            ::core::cmp::Ordering::Equal
        };
        let quotient_is_odd = quotient.bit(0);
        let result_is_positive = (numerator < S::ZERO) == (divisor < S::ZERO);
        let bump = crate::support::rounding::should_bump(
            RoundingMode::HalfToEven,
            remainder_cmp,
            quotient_is_odd,
            result_is_positive,
        );
        // Away-from-zero for a positive result is UP, for a negative one DOWN
        // — so the truth is above the answer exactly when those two disagree.
        let side = if result_is_positive != bump {
            TailSign::Above
        } else {
            TailSign::Below
        };
        let rounded = if bump {
            if result_is_positive {
                quotient + S::ONE
            } else {
                quotient - S::ONE
            }
        } else {
            quotient
        };
        (rounded, Some(side))
    }

    /// Whether a `÷10^w` rounding left `quotient` no further from zero than the
    /// exact quotient of `product` — the ordering counterpart of the equality
    /// test a caller would otherwise write as `quotient · divisor == product`.
    ///
    /// `divisor` is the `10^w` the rounding divided by.
    ///
    /// # What the ordering buys over the equality
    ///
    /// A series term rounded TOWARD zero pulls its partial sum toward zero, so
    /// it leaves the true value on the AWAY-from-zero side — which is the side
    /// the dropped tail is already on whenever every term carries the argument's
    /// sign. Such a rounding therefore cannot break an agreement argument; only
    /// one that pushes the term AWAY from zero can. Demanding exactness rejects
    /// both, which costs every argument whose divisions merely came out on the
    /// harmless side. See [`log1p_fixed_inner`], whose tag rests on this.
    ///
    /// # Fail-closed
    ///
    /// `product` reaches this as a truncated low product, so a term wide enough
    /// to wrap it would make any ordering read off it meaningless.
    /// Reconstructing the product from `quotient` must land within one divisor
    /// of `product` — that gap IS the discarded remainder — so a wider gap means
    /// the read is not trustworthy and the answer is `false`, exactly as an
    /// away-from-zero rounding is.
    #[inline]
    fn rounded_toward_zero<S: BigInt>(quotient: S, product: S, divisor: S) -> bool {
        let reconstructed = quotient.wrapping_mul_low_u128(divisor);
        (reconstructed - product).abs() < divisor && reconstructed.abs() <= product.abs()
    }

    /// Half-to-even quotient `numerator / 10^exponent`, via the MG (magic-multiply)
    /// reciprocal — the same fast divide the per-tier
    /// `decl_wide_transcendental!` core uses, here for the width-generic
    /// path the hyperbolics run through. For `1 ≤ w ≤ 38` the
    /// single-chunk base-`2^128` kernel; for `w > 38` the chain of
    /// `÷ 10^38` stages. Bit-identical to the generic `round_div(n,
    /// 10^w)` (audited in `mg_divide::tests`), but replaces the
    /// per-Taylor-term 256-limb Knuth division that dominated the wide
    /// hyperbolic/exp cost. The buffer comes from `S`'s scratch carrier ([`ComputeLimbs`]), so no
    /// const-generic limb count appears here.
    #[inline]
    pub(crate) fn round_div_pow10<S: BigInt>(numerator: S, exponent: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        if exponent == 0 {
            return numerator;
        }
        if exponent <= 38 {
            return crate::algos::support::mg_divide::div_wide_pow10::<S>(
                numerator,
                exponent,
                RoundingMode::HalfToEven,
            );
        }
        // `exponent > 38` rescale: route through the rescale MATCHER (not
        // `div_wide_pow10_chain` directly) so the wide / high-scale band gets
        // the baked-reciprocal Newton arm + the 9.24 magnitude-trim, exactly
        // as the per-tier `wide_transcendental` cores do. The matcher only
        // deviates from `MgChain` where its pick is faster, and every selected
        // kernel is bit-identical (a fixed-mode `÷10^w` has one correct
        // answer — the rescale validity wall), so this is value-neutral and
        // never slower. Single source for the wide rescale across exp/ln/the
        // generic Tang kernel.
        crate::algos::support::rescale::dispatch_wide_pow10::<S>(
            numerator,
            exponent,
            RoundingMode::HalfToEven,
        )
    }
    /// `(lhs · rhs) / 10^working_scale`, rounded half-to-even.
    #[inline]
    pub(crate) fn mul<S: BigInt>(lhs: S, rhs: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        // u128-packed wide multiply: bit-identical to `a * b` (it IS the low
        // product) for even-limb work widths, ~1/4 the partial products;
        // falls back to the base-2^64 schoolbook for odd N. This is the hot
        // Taylor-term / squaring multiply, run at `Wexp` (up to Int<256>) for
        // exp + the hyperbolics.
        round_div_pow10(lhs.wrapping_mul_low_u128(rhs), working_scale)
    }
    /// Loop-friendly `mul` with a precomputed `10^w` divisor.
    #[inline]
    fn mul_cached<S: BigInt>(lhs: S, rhs: S, cached_pow10: S) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        round_div(lhs.wrapping_mul_low_u128(rhs), cached_pow10)
    }
    /// `(numerator · 10^w) / divisor`, rounded half-to-even (precomputed
    /// numerator factor).
    #[inline]
    pub(crate) fn div_cached<S: BigInt>(numerator: S, divisor: S, cached_pow10: S) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        div_cached_sided(numerator, divisor, cached_pow10).0
    }

    /// [`div_cached`] plus the side its rounding left the exact quotient on
    /// — see [`round_div_sided`]. Same single rounding body, same value.
    #[inline]
    fn div_cached_sided<S: BigInt>(
        numerator: S, divisor: S, cached_pow10: S) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        round_div_sided(numerator.wrapping_mul_low_u128(cached_pow10), divisor)
    }
    /// `value · multiplier` for a small unsigned multiplier.
    #[inline]
    fn mul_u<S: BigInt>(value: S, multiplier: u128) -> S {
        if multiplier <= u64::MAX as u128 {
            value.mul_u64(multiplier as u64)
        } else {
            value * S::from_i128(multiplier as i128)
        }
    }
    /// `k · c` where `k` is a signed range-reduction count. An n-by-1-word
    /// product (`mul_u64`) — O(limbs), not the full schoolbook —
    /// since `|k|` always fits one word on the range-reduction paths.
    #[inline]
    pub(crate) fn scale_by_k<S: BigInt>(constant: S, k: i128) -> S {
        if k >= 0 {
            mul_u(constant, k as u128)
        } else {
            -mul_u(constant, k.unsigned_abs())
        }
    }
    /// Rounds a working-scale value to the nearest integer (ties away
    /// from zero); used for the range-reduction quotient.
    pub(crate) fn round_to_nearest_int<S: BigInt>(working_value: S, working_scale: u32) -> i128
    where
        S::Scratch: ComputeLimbs,
    {
        let divisor = pow10::<S>(working_scale);
        let (quotient, remainder) = div_rem_exact(working_value, divisor);
        let half = divisor >> 1;
        let rounded_quotient = if remainder.abs() >= half {
            if working_value < S::ZERO { quotient - S::ONE } else { quotient + S::ONE }
        } else {
            quotient
        };
        crate::int::types::traits::BigInt::to_i128(rounded_quotient)
    }

    /// `ln 2` at `working_scale`, sourced from the unified constant
    /// table (`consts::ln2_by_working_scale`) — a static lookup +
    /// zero-extend, NOT a recompute. Replaces the former `2·artanh(1/3)`
    /// series (~`w` terms), which dominated the wide-tier exp/hyperbolic
    /// cost; the table's `ln2` band is sized (gen_const_table.py
    /// `LN2_MAXES`) to the peak `w_ext` this path can request. Mode is
    /// half-to-even, matching the per-tier core's `ln2_cf`.
    fn ln2<S: BigInt>(working_scale: u32) -> S {
        crate::consts::ln2_by_working_scale::<S>(working_scale, RoundingMode::HalfToEven)
    }

    /// `√v` at `working_scale`: `√(|v| · 10^w)`, truncating. Width-generic
    /// twin of the per-tier `$core::sqrt_fixed` (the multi-level argument
    /// reduction `ln_fixed` runs); bit-identical (same seed-library bootstrap
    /// + monotone-downward Newton). `|v| · 10^w` must fit `S`.
    pub(crate) fn sqrt_fixed<S: BigInt>(value: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let abs_value = value.abs();
        let radicand = abs_value * pow10::<S>(working_scale);
        if radicand <= zero::<S>() {
            return zero::<S>();
        }
        // Seed from the shared cross-algorithm seed leaf (std f64 bootstrap /
        // no_std 1-bit), both guaranteed over-estimates, so the AM-GM pre-step
        // + monotone-downward loop converge to the identical floor either way.
        let seed = crate::algos::support::seed_bridge::sqrt_seed_w::<S>(radicand);
        let initial_estimate = if seed <= zero::<S>() { lit::<S>(1) } else { seed };
        // `div_rem_exact` (not the `/` operator): the Newton divides run at
        // the full work width, past the narrow build's blanket divide
        // scratch — see [`div_rem_exact`].
        let mut estimate =
            (initial_estimate + div_rem_exact(radicand, initial_estimate).0) >> 1;
        loop {
            let next_estimate = (estimate + div_rem_exact(radicand, estimate).0) >> 1;
            if next_estimate >= estimate {
                return estimate;
            }
            estimate = next_estimate;
        }
    }

    /// Natural logarithm of a positive working-scale value, generic over the
    /// work integer `S`. Width-generic twin of the per-tier
    /// `$core::ln_fixed`: range-reduces `v = 2^k·m` with `m ∈ [1, 2)`, applies
    /// `sqrt_levels` levels of sqrt argument reduction (Brent 1976), evaluates
    /// `ln(m) = 2^(l+1)·artanh((m−1)/(m+1))`, returns `k·ln2 + ln(m)`.
    /// `ln2_at_working_scale` is `ln 2` at `working_scale`, supplied by the
    /// caller (the primitive wrapper passes the const-folded
    /// `ln2_cf::<SCALE>`; a composition passes its wide-work `ln2`), so this
    /// stays free of the `SCALE` const. Bit-identical to the per-tier core for
    /// the same `(v, w, ln2_w)`.
    pub(crate) fn ln_fixed<S: BigInt>(
        working_value: S, working_scale: u32, ln2_at_working_scale: S) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let one_at_working_scale = one::<S>(working_scale);
        let two_at_working_scale = one_at_working_scale + one_at_working_scale;
        let pow10_at_working_scale = one_at_working_scale;
        let mut k: i32 = bit_length::<S>(working_value) as i32
            - bit_length::<S>(one_at_working_scale) as i32;
        let mut mantissa_w = loop {
            let candidate_mantissa = if k >= 0 {
                working_value >> (k as u32)
            } else {
                working_value << ((-k) as u32)
            };
            if candidate_mantissa >= two_at_working_scale {
                k += 1;
            } else if candidate_mantissa < one_at_working_scale {
                k -= 1;
            } else {
                break candidate_mantissa;
            }
        };

        // Exact power-of-two boundary `m = 1` short-circuits: ln(m) = 0,
        // so ln(v) = k·ln2 exactly. Bit-identical to falling through —
        // each `sqrt_fixed(10^w, w)` returns `10^w` exactly (isqrt of the
        // perfect square `10^2w`), `t = (m−1)/(m+1) = 0`, and the artanh
        // series' first term is already zero — but skips the multi-level
        // sqrt reduction those steps would burn. Mirrors the Tang kernel's
        // `mantissa_w == one_at_working_scale` arm.
        if mantissa_w == one_at_working_scale {
            return scale_by_k::<S>(ln2_at_working_scale, k as i128);
        }

        // Multi-level sqrt argument reduction: `l ≈ √level_bound / 4`.
        let level_bound = working_scale.saturating_mul(3).saturating_add(1);
        let sqrt_levels: u32 = {
            let mut levels: u32 = 0;
            while (levels + 1) * (levels + 1) <= level_bound {
                levels += 1;
            }
            levels / 4
        };
        let mut i = 0;
        while i < sqrt_levels {
            mantissa_w = sqrt_fixed::<S>(mantissa_w, working_scale);
            i += 1;
        }

        let atanh_arg = div_cached::<S>(
            mantissa_w - one_at_working_scale,
            mantissa_w + one_at_working_scale,
            pow10_at_working_scale);
        let atanh_arg_sq = mul::<S>(atanh_arg, atanh_arg, working_scale);
        let mut sum = atanh_arg;
        let mut term = atanh_arg;
        let mut term_index: u128 = 1;
        loop {
            term = mul::<S>(term, atanh_arg_sq, working_scale);
            let contribution = term / lit::<S>((2 * term_index + 1) as i128);
            if contribution == zero::<S>() {
                break;
            }
            sum = sum + contribution;
            term_index += 1;
            if term_index > SERIES_CAP {
                break;
            }
        }
        // ln(m) = 2^(l+1)·artanh(t) = sum << (sqrt_levels + 1).
        let ln_mantissa = sum << (sqrt_levels + 1);
        scale_by_k::<S>(ln2_at_working_scale, k as i128) + ln_mantissa
    }

    /// `log1p(t) = ln(1 + t)` at `working_scale`, evaluated without
    /// ever forming `1 + t` — generic over the work integer `S` (the
    /// single source; the per-tier `decl_wide_transcendental!`
    /// `log1p_fixed` forwards here).
    ///
    /// Uses the Goldberg/Higham reformulation
    /// `log1p(t) = 2·artanh(t / (2 + t))`: `2 + t` is benign (no
    /// near-equal subtraction for `t > -1`) and the divide is
    /// well-conditioned, so `u ~ t/2` carries every significant digit of
    /// `t`, removing the catastrophic cancellation of the naive
    /// `ln(1 + t)` at the source. Domain: `t > -1` (the caller guards).
    ///
    /// Reference: N. J. Higham, *Accuracy and Stability of Numerical
    /// Algorithms* 2nd ed. (2002), 1.14.1 and Problem 1.4; J.-M. Muller,
    /// *Elementary Functions* 3rd ed. (2016), 4.4.
    pub(crate) fn log1p_fixed<S: BigInt>(argument: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        log1p_fixed_inner::<S>(argument, working_scale, None).0
    }

    /// [`log1p_fixed`] plus the side of the returned value the TRUE value
    /// lies on — see [`TailSign`] for what that buys and why nothing else can
    /// supply it.
    ///
    /// The value is bit-identical to [`log1p_fixed`]'s at every argument; the
    /// tag is the only difference, and it is `None` whenever it cannot be
    /// PROVED (see [`log1p_fixed_inner`]).
    ///
    /// `guard_digits` is the caller's sub-storage granularity — the
    /// `10^guard_digits` it will divide this value by to reach the storage
    /// grid. It buys nothing in the answer and everything in the cost: a tag is
    /// only ever CONSULTED where the caller's own residual cannot decide, so
    /// knowing that granularity lets the proof be skipped wherever it would be
    /// discarded. See [`side_by_deeper_probe`].
    pub(crate) fn log1p_fixed_tagged<S: BigInt>(
        argument: S,
        working_scale: u32,
        guard_digits: u32,
    ) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        log1p_fixed_inner::<S>(argument, working_scale, Some(guard_digits))
    }

    /// The one series loop behind [`log1p_fixed`] and
    /// [`log1p_fixed_tagged`].
    ///
    /// `tag_at_guard` is `Some(guard_digits)` to ask for the tag at the caller's
    /// sub-storage granularity, `None` for the bare value. `None` keeps the untagged path's
    /// arithmetic *exactly* what it was, for the reason [`expm1_fixed_inner`]
    /// documents: reading a rounding's direction costs a multiply-back, and the
    /// untagged path must not pay it.
    ///
    /// # Why this kernel's rule is NOT [`expm1_fixed_inner`]'s
    ///
    /// `expm1` tags only when every included term divided exactly, because
    /// otherwise an integer rounding error `|e| >= 1` dominates its sub-unit
    /// tail. It can demand that because its series is seeded with the input
    /// itself (`let mut sum = s`) — no leading divide. This kernel has one
    /// and it is unavoidable:
    ///
    /// ```text
    /// u = round_div(argument·10^w, 2·10^w + argument)
    /// ```
    ///
    /// and that divide is **provably never exact for the tiny-`t` family**:
    /// the quotient is `10^w / (2·10^m ± 1)`, whose divisor ends in 1 or 9 —
    /// odd, coprime to 10 — so it cannot divide `10^w = 2^w·5^w` unless it is
    /// 1. Mirroring `expm1`'s rule verbatim would leave the tag `None` on
    /// iteration one, for every width, scale, guard and sign.
    ///
    /// So no rounding here is treated as a disqualifying error. Each is an
    /// error term whose SIDE is known exactly, and the tag is proved by making
    /// every one of them agree:
    ///
    /// * write the returned value as `R = 2·S_k(u)` and the true one as
    ///   `V = 2·artanh(û)`, `û` the exact quotient. Then
    ///   `V − R = 2[artanh(û) − artanh(u/10^w)] + 2·T_k(u) + 2[S_k^exact(u) − S_k(u)]`.
    /// * `artanh` is strictly increasing, so the first bracket carries the
    ///   sign of `û − u/10^w` — precisely the side [`round_div_sided`]
    ///   reports;
    /// * `T_k`, the terms the loop dropped, is a sum of odd powers that ALL
    ///   carry `u`'s sign, so its side is `sign(u)` — at any depth and for
    ///   whatever reason the loop stopped. (`expm1` needed its vanish index's
    ///   parity here because ITS series alternates; `artanh`'s does not, so
    ///   this side is unconditional and no `vanished_at` is tracked.)
    /// * the third bracket is the rounding the INCLUDED terms carry. Every
    ///   term is an odd power of `u` and so carries `u`'s sign, which makes
    ///   "toward zero" and "toward the tail's side" the same statement: a term
    ///   rounded toward zero shrinks `|S_k|` and leaves the truth away from
    ///   zero, exactly where the tail already is. That holds for the shared
    ///   `u²` as well, whose own rounding scales every later term the same way,
    ///   so the errors compound in ONE direction rather than cancelling.
    ///   [`rounded_toward_zero`] reads each one; the per-term division needs no
    ///   read at all, since truncation is toward zero by construction.
    ///
    /// When every side AGREES the sum's side follows with no magnitude
    /// comparison at all, and that is the only case this rule tags.
    ///
    /// # Unanimity is the PRECONDITION of that proof, not an observation
    ///
    /// The argument above holds *because* nothing can cancel: every error term
    /// displaces the sum the same way, so their total displaces it that way
    /// too, whatever their sizes. The moment one term opposes, sizes are the
    /// only thing that decides — a large opposing term can cancel the rest or
    /// flip the total outright — and NO rule over signs can see that. So a
    /// mixed reading is not a weaker version of this proof, it is outside it.
    ///
    /// This is the same trap per-term exactness fell into on `expm1`: what
    /// decides is the ACCUMULATED effect, and a rule that reasons term by term
    /// while terms pull in both directions is asserting, not proving. Anything
    /// short of unanimity therefore fails this rule closed — it goes to
    /// [`side_by_deeper_probe`], which MEASURES the accumulation instead of
    /// arguing about it, and to `None` if even that cannot resolve it. A tag
    /// must never be produced from mixed directions by any other route.
    ///
    /// # Why the ORDER, and not exactness
    ///
    /// Demanding that each rounding be EXACT — `expm1`'s rule — is far stronger
    /// than the argument needs, and here it is fatal rather than merely strict.
    /// `u` is a rounded quotient, so `u²` almost never divides `10^w` exactly;
    /// since `u²` feeds every term, one surviving series term used to be enough
    /// to force the tag `None` for good. The tag could then fire only where the
    /// series contributed NOTHING, which left every directed round blind on the
    /// arguments whose sub-LSB imprecision is decided by a term the series does
    /// keep. Reading the ROUNDING'S DIRECTION instead admits all of those whose
    /// error terms still line up, at no cost: the multiply-back the exactness
    /// test already computed answers the ordering question too.
    ///
    /// # When the tag is `None`
    ///
    /// It fails CLOSED — the walker treats `None` as "make no adjustment":
    ///
    /// * no tag was asked for (`tag_at_guard` is `None`), or `u == 0` (no tail to
    ///   carry a sign);
    /// * the caller could not use one anyway — its residual already decides the
    ///   narrowing (see [`side_by_deeper_probe`]);
    /// * `|u| >= 10^w`, where the dropped tail neither converges nor need
    ///   carry its terms' common sign;
    /// * the sides genuinely oppose — an INCLUDED term's multiply rounded AWAY
    ///   from zero, or the seed divide's side is not the tail's — AND
    ///   [`side_by_deeper_probe`] could not settle it either, because the work
    ///   integer had no room to probe in or because the gap it measured was
    ///   inside its own error. ("Included" is load-bearing exactly as in
    ///   [`expm1_fixed_inner`]: the term that ENDS the loop is never added and
    ///   so contributes no error, and counting it would reject arguments whose
    ///   sum it never touched.) Opposition alone is NOT enough to refuse: a
    ///   dropped tail is not always negligible beside a half-ULP divide
    ///   residual, so asserting either side outright would be a guess — but
    ///   measuring the two against each other is not, which is what the probe
    ///   does.
    fn log1p_fixed_inner<S: BigInt>(
        argument: S,
        working_scale: u32,
        tag_at_guard: Option<u32>,
    ) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        let want_tag = tag_at_guard.is_some();
        let one_at_working_scale = one::<S>(working_scale);
        let two_at_working_scale = one_at_working_scale + one_at_working_scale;
        let pow10_at_working_scale = one_at_working_scale;
        // The seed divide, with the side its rounding left the true quotient
        // on. `div_cached` IS this with the side dropped.
        let (u, div_side) = div_cached_sided::<S>(
            argument, two_at_working_scale + argument, pow10_at_working_scale);
        // The side EVERY error term has to agree with: the dropped tail's,
        // which is `u`'s sign unconditionally (artanh's series does not
        // alternate). Read here so the loop can test each rounding against it;
        // it only means anything once `u != zero` is established below.
        let tail_side = if u > zero::<S>() {
            TailSign::Above
        } else {
            TailSign::Below
        };
        // `u²` feeds every term, so which way ITS rounding went gates all of
        // them.
        let (u2, u2_toward_zero) = if want_tag {
            let product = u.wrapping_mul_low_u128(u);
            let scaled = round_div_pow10::<S>(product, working_scale);
            (scaled, rounded_toward_zero::<S>(scaled, product, one_at_working_scale))
        } else {
            (mul::<S>(u, u, working_scale), true)
        };
        let mut sum = u;
        let mut term = u;
        let mut term_index: u128 = 1;
        // Every rounding that has actually reached `sum` moved it TOWARD zero,
        // so the truth still lies on the away-from-zero side the dropped tail
        // is on. The seed divide is the first such term, and the only one whose
        // side is read rather than derived.
        let mut agree = match div_side {
            None => true,
            Some(s) => s == tail_side,
        };
        loop {
            // `term_index` is bounded by SERIES_CAP (20_000), so the cast to
            // the generic `lit`'s i128 argument is lossless.
            let divisor = lit::<S>((2 * term_index + 1) as i128);
            // `step_toward_zero`: this term's rounding did not push the partial
            // sum away from zero, so it cannot oppose the tail.
            let (contribution, step_toward_zero) = if want_tag {
                // The same value `mul::<S>(term, u2, working_scale)` produces,
                // with the direction of its rounding recorded on the way through.
                let product = term.wrapping_mul_low_u128(u2);
                let scaled = round_div_pow10::<S>(product, working_scale);
                let mul_toward_zero =
                    rounded_toward_zero::<S>(scaled, product, one_at_working_scale);
                term = scaled;
                // The division truncates, so it can only ever pull the term
                // toward zero — the tail's own side. It is the one step that
                // never needs testing, which is why its remainder is dropped.
                let (quotient, _remainder) = div_rem_exact::<S>(term, divisor);
                (quotient, mul_toward_zero && u2_toward_zero)
            } else {
                term = mul::<S>(term, u2, working_scale);
                (term / divisor, true)
            };
            if contribution == zero::<S>() {
                break;
            }
            // Only a term that is actually ADDED can carry error into `sum`.
            agree = agree && step_toward_zero;
            sum = sum + contribution;
            term_index += 1;
            if term_index > SERIES_CAP {
                break;
            }
        }
        // Doubling is a positive scaling, so it preserves the side.
        let value = sum + sum;
        let tag = match tag_at_guard {
            // No tag asked for; no tail to carry a sign; or an argument whose
            // dropped terms need not share one.
            None => None,
            Some(_) if u == zero::<S>() || u.abs() >= one_at_working_scale => None,
            // Every error term reaching `sum` pushes the same way, so their sum
            // does too — no magnitude comparison anywhere.
            Some(_) if agree => Some(tail_side),
            // They genuinely oppose, so the total turns on their SIZES — an
            // opposing term can cancel the rest or flip it — and no reading of
            // signs can see that. Measure it rather than assert it.
            Some(guard_digits) => side_by_deeper_probe::<S>(
                argument, working_scale, value, guard_digits),
        };
        (value, tag)
    }

    /// Bits of the work integer [`side_by_deeper_probe`] leaves unused, so its
    /// widest product cannot reach the sign bit. Two limbs.
    const TAG_PROBE_SLACK_BITS: u64 = 128;

    /// The side the TRUE value lies on relative to `shallow_value`, established by
    /// RE-EVALUATING the series at a deeper working scale.
    ///
    /// [`log1p_fixed_inner`]'s sign rule settles the tag whenever the error
    /// terms line up. When they genuinely oppose, nothing about their signs can
    /// decide it and only their magnitudes can — so rather than bound them term
    /// by term, this recomputes the whole value at a deeper working scale,
    /// where each of those errors is orders smaller, and reads the side
    /// straight off the gap.
    ///
    /// # How deep, and why that is not a tuning choice
    ///
    /// As deep as the work integer allows, every time. The depth is a property
    /// of the WIDTH, never of the argument: probing shallower would refuse
    /// arguments the width could have settled, and there is no other claim on
    /// the capacity — it is the same free headroom the walker cannot use only
    /// because ITS depth is pinned to the constant tables. Reading the maximum
    /// off `BITS` also keeps the depth from being fitted to any particular
    /// argument or cell, which a literal digit count would invite.
    ///
    /// # Why this is not the walker's escalation
    ///
    /// It is the same idea the Ziv walker applies, at the one place the walker
    /// cannot reach. The walker stops at a const-table-safe cap because
    /// `pi`/`ln2`/`sincos` are provisioned only to about `BITS/8` digits, and
    /// probing past that would request an entry the generated table does not
    /// hold. This kernel reads none of them: its only constant is `10^w`, and
    /// `consts::pow10` falls back to `TEN.pow` outside its baked range. So the
    /// probe can compute deeper WITHOUT raising that cap or touching any
    /// table's provisioning.
    ///
    /// # Why the answer is sound
    ///
    /// `shallow_value` is lifted by an exact power of ten and so carries no error at
    /// all; the gap is therefore the probe's own reading of `V − R`, wrong by
    /// at most the probe's error. That error damps rather than accumulates:
    /// each term is formed from the one before it and scaled by `u² ≤ 1/9` over
    /// the whole region the matcher routes here, so a term inherits at most a
    /// ninth of its predecessor's error before adding its own rounding — under
    /// two units, and under three once the truncating division is counted. With
    /// the seed divide under one unit and the dropped tail under two, four
    /// units per term bounds the lot, and the loop takes at most `SERIES_CAP`
    /// of them. A gap CLEARING that bound has the sign of `V − R`; one that
    /// does not is `None`, fail-closed, exactly as before.
    ///
    /// # Termination
    ///
    /// The re-entry passes `tag_at_guard = None`, which is precisely the branch that
    /// never calls this function, so the recursion is one level deep.
    fn side_by_deeper_probe<S: BigInt>(
        argument: S,
        working_scale: u32,
        shallow_value: S,
        guard_digits: u32,
    ) -> Option<TailSign>
    where
        S::Scratch: ComputeLimbs,
    {
        // Only pay for a proof the caller can actually use. A tail sign changes
        // a narrowing in exactly two places: a DIRECTED round whose sub-storage
        // residual is exactly zero, and a NEAREST one whose residual is exactly
        // half. Anywhere else the residual itself decides and the sign is
        // discarded unread, so proving it there is pure waste — and this proof
        // is the expensive kind. Skipping it cannot change any result: it turns
        // a tag that would have been thrown away into `None`, which is thrown
        // away identically.
        // `div_rem_exact`, not the `%` operator: the blanket operator's scratch
        // is build-max sized, so in a narrow `exact-scratch` build it is cut for
        // that build's 2-limb storage while this kernel probes in a far wider
        // work integer — the hazard `round_to_storage_*` already avoids the same
        // way.
        let divisor = pow10::<S>(guard_digits);
        let (_quotient, remainder) = div_rem_exact::<S>(shallow_value.abs(), divisor);
        if remainder != zero::<S>() && remainder + remainder != divisor {
            return None;
        }
        // The probe's widest intermediates are products of two values under
        // `10^deep_scale` — the squared quotient, and the seed divide's numerator —
        // so twice that width plus the slack has to fit the work integer.
        // Inverting `bits(10^d) = d·log2(10) < (d·10 + 2)/3` for the largest
        // `d` that leaves room: `d ≤ 3·(BITS − slack)/20`.
        let work_integer_bits = u64::from(<S as BigInt>::BITS);
        // Fail closed on the (unreachable) overflow rather than fall back to a
        // depth the width cannot hold.
        let deep_scale =
            u32::try_from(3 * work_integer_bits.saturating_sub(TAG_PROBE_SLACK_BITS) / 20).ok()?;
        // No room to probe any deeper than the value was already computed at.
        let extra_digits =
            deep_scale.checked_sub(working_scale).filter(|digits| *digits > 0)?;
        let lift = pow10::<S>(extra_digits);
        let deep_value = log1p_fixed_inner::<S>(argument * lift, deep_scale, None).0;
        let gap = deep_value - shallow_value * lift;
        let bound = lit::<S>(4 * SERIES_CAP as i128 + 16);
        if gap.abs() > bound {
            Some(if gap > zero::<S>() {
                TailSign::Above
            } else {
                TailSign::Below
            })
        } else {
            None
        }
    }

    /// Which side of a working-scale sum the TRUE value lies on — the sign
    /// of the series terms the kernel dropped.
    ///
    /// # Why the Ziv walker cannot derive this for itself
    ///
    /// The walker decides a directed round from the sub-storage residual and
    /// a nearest round from that residual's distance to the half-ULP
    /// boundary. Both readings are blind in exactly one situation: when the
    /// kernel's working value lands EXACTLY on the boundary — a storage grid
    /// line for the directed modes, a midpoint for the nearest ones. The
    /// residual then reads as a clean zero or a clean tie, and the walker
    /// concludes the value is exactly representable, or exactly a tie, when
    /// it is neither: `e^x - 1` is transcendental at every algebraic
    /// `x != 0` (Lindemann-Weierstrass), so it never lands on a grid line.
    ///
    /// Escalating does not rescue it, and NOT for want of depth: at the top
    /// scale of D462 / D616 / D924 the work integer's `BITS/8` is
    /// 512 / 768 / 1024 against a `SCALE` of 461 / 615 / 923, so `max_guard`
    /// works out at 43 / 145 / 93 and a second probe always runs. What
    /// defeats it is that the landing is scale-INVARIANT: the terms'
    /// individual sub-LSB imprecisions cancel against one another, and the
    /// exact sum they leave is a whole multiple of the guard power — both
    /// facts turning on the argument's own factors of two, three and ten
    /// rather than on the working scale. So the deeper probe re-derives the
    /// same clean zero and draws the same conclusion; depth cannot see past a
    /// coincidence that does not depend on depth. (One shape differs in
    /// detail — a deeper probe can turn up a tiny NON-zero remainder rather
    /// than a clean zero — but it falls below the walker's absolute noise
    /// floor and is discarded, so the outcome is the same.)
    ///
    /// What decides the round there is the tail the kernel could not
    /// represent, and only the kernel that summed the series knows it.
    ///
    /// # Why a single fixed polarity cannot stand in
    ///
    /// The near-min `never_exact` flag asserts one direction for a whole
    /// function. That cannot work here: the correct direction varies with the
    /// ARGUMENT, not the function, and both directions occur within one
    /// `(width, scale)` cell. At `D1232<1231>`, `x = -3e-240` needs one and
    /// `x = -1e-306` the other, because the tail's first surviving term is
    /// the 6th in one case and the 5th in the other.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub(crate) enum TailSign {
        /// The true value is strictly ABOVE the returned sum.
        Above,
        /// The true value is strictly BELOW the returned sum.
        Below,
    }

    /// `expm1(s) = exp(s) - 1` at `working_scale`, evaluated as the
    /// Taylor series with the leading `1` term dropped so the
    /// `exp(s) - 1` subtraction of two values both `~ 1` never occurs:
    /// `expm1(s) = s + s^2/2! + s^3/3! + ...`. For tiny `s` the result
    /// keeps every digit of `s` (`kappa = |s/expm1(s)| -> 1`).
    ///
    /// This kernel is the accuracy-critical small-argument case
    /// `|s| <~ ln2/2`; the caller reduces a general argument to this band
    /// and reassembles via the exact `2^k` shift. No range reduction is
    /// performed here.
    ///
    /// The companion of [`log1p_fixed`]: both exist to remove the
    /// catastrophic cancellation of the naive form at the source.
    ///
    /// Reference: J.-M. Muller, *Elementary Functions* 3rd ed. (2016),
    /// 4.4; N. J. Higham, *Accuracy and Stability of Numerical
    /// Algorithms* 2nd ed. (2002), 1.14.1.
    pub(crate) fn expm1_fixed<S: BigInt>(reduced_arg: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        expm1_fixed_inner::<S>(reduced_arg, working_scale, false).0
    }

    /// [`expm1_fixed`] plus the sign of the terms it DROPPED — see
    /// [`TailSign`] for what that buys and why nothing else can supply it.
    ///
    /// The sum is bit-identical to [`expm1_fixed`]'s at every argument; the
    /// tag is the only difference, and it is `None` whenever it cannot be
    /// justified (see [`expm1_fixed_inner`]).
    pub(crate) fn expm1_fixed_tagged<S: BigInt>(
        reduced_arg: S, working_scale: u32) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        expm1_fixed_inner::<S>(reduced_arg, working_scale, true)
    }

    /// One step of the accumulated-error recurrence: the exact rational
    /// `eps_j` that this term contributes, as `(numerator, denominator)`.
    ///
    /// Writing `a_j` for the term the loop actually computed and `A_j` for
    /// the exact Taylor term, the error `eps_j = a_j - A_j` satisfies
    ///
    /// ```text
    ///   eps_j = (eps_{j-1} * s - rho_j) / (10^w * j)  -  r_j / j
    /// ```
    ///
    /// where `rho_j = prod - scaled * 10^w` is the `/10^w` rounding remainder
    /// and `r_j` the truncating `/j` remainder — both already in the loop's
    /// hand. Substituting `eps_{j-1} = numerator/denominator` turns that into
    /// `(Z / 10^w - den * r_j) / (den * j)` with `Z = num * s - den * rho_j`,
    /// so the error stays a SMALL-denominator rational exactly while `10^w`
    /// divides `Z` — which is the whole reason it can be tracked at all in a
    /// fixed-width integer.
    ///
    /// Returns `None` the moment the error stops being exactly
    /// representable: the division is not exact, an intermediate outgrows
    /// `S`, or the denominator outgrows `i128`. Every such verdict FAILS
    /// CLOSED — the caller stops tracking and the tag becomes `None`, which
    /// is exactly what the previous per-term rule returned for those inputs,
    /// so nothing loses coverage.
    fn term_error<S: BigInt>(
        eps: (S, i128),
        rho: S,
        rem_j: S,
        term_index: i128,
        reduced_arg: S,
        working_scale: u32,
    ) -> Option<(S, i128)>
    where
        S::Scratch: ComputeLimbs,
    {
        let (numerator, denominator) = eps;
        let z = numerator
            .checked_mul(reduced_arg)?
            .checked_sub(lit::<S>(denominator).checked_mul(rho)?)?;
        let (scaled_z, z_remainder) = div_rem_exact::<S>(z, one::<S>(working_scale));
        if z_remainder != zero::<S>() {
            return None;
        }
        Some((
            scaled_z.checked_sub(lit::<S>(denominator).checked_mul(rem_j)?)?,
            denominator.checked_mul(term_index)?,
        ))
    }

    /// Sum of two exact rationals, over their product denominator.
    ///
    /// Deliberately NOT reduced to lowest terms: the tag is only ever
    /// consulted for arguments small enough that the series vanishes within a
    /// handful of terms, and the only thing ever read back out is whether the
    /// NUMERATOR is zero — which no common factor can change. `None` on any
    /// overflow, failing closed exactly as [`term_error`] does.
    fn add_error<S: BigInt>(lhs: (S, i128), rhs: (S, i128)) -> Option<(S, i128)> {
        let (lhs_num, lhs_den) = lhs;
        let (rhs_num, rhs_den) = rhs;
        Some((
            lhs_num
                .checked_mul(lit::<S>(rhs_den))?
                .checked_add(rhs_num.checked_mul(lit::<S>(lhs_den))?)?,
            lhs_den.checked_mul(rhs_den)?,
        ))
    }

    /// The one series loop behind [`expm1_fixed`] and
    /// [`expm1_fixed_tagged`].
    ///
    /// `want_tag` exists to keep the untagged path's arithmetic *exactly*
    /// what it was: proving the tag needs each term's two rounding remainders,
    /// and recovering the `÷10^w` one costs a multiply-back
    /// (`round_div_pow10` rounds and does not report its remainder — on the
    /// `w > 38` path it routes through the shared rescale matcher, whose
    /// contract this must not disturb). That cost belongs only to the caller
    /// that needs the tag, so it is a runtime flag on ONE loop rather than a
    /// second loop or a const knob.
    ///
    /// # What the tag has to prove
    ///
    /// The tag names the side of `sum` the true value lies on, which is the
    /// tail's side only when `sum` IS the exact partial sum — i.e. when the
    /// accumulated rounding error is exactly zero. So that is what gets
    /// tracked, as an exact rational, via [`term_error`] and [`add_error`].
    ///
    /// Per-term exactness would be the easier test, and it is SUFFICIENT, but
    /// it is not NECESSARY and the difference is not academic: the terms'
    /// errors can cancel. At `x = -10^-m` the `÷3!` of the third term leaves
    /// exactly `+2/3` and the fourth term's two roundings leave exactly
    /// `-2/3`, so the sum is exact although neither term is — and because the
    /// value then lands ON a storage grid line, that is precisely where the
    /// walker needs the tag most. A per-term rule is `None` there and the
    /// directed round stands still on a value that is not actually on the
    /// grid.
    ///
    /// # When the tag is `None`
    ///
    /// It fails CLOSED — every caller treats `None` as "make no adjustment":
    ///
    /// * `want_tag` was not asked for, or `reduced_arg == 0` (`expm1(0) = 0`,
    ///   no tail);
    /// * `|s| > 1`, where `|s^j / j!|` is not yet monotonically decreasing,
    ///   so the tail need not carry its first term's sign;
    /// * the loop stopped at [`SERIES_CAP`] rather than because a term
    ///   vanished, so the tail is NOT below the working resolution;
    /// * the accumulated error is non-zero, or stopped being exactly
    ///   representable (see [`term_error`]). A non-zero error is of the same
    ///   order as the sub-unit tail, so it can outweigh it and put the true
    ///   value on the other side; proving the error ZERO is what makes the
    ///   tag a PROOF of the side rather than an overwhelming likelihood.
    ///
    /// Only the terms actually ADDED are tracked. The term that ENDS the loop
    /// reached zero by being rounded away, so its divisions are inexact
    /// whenever it was not already zero — but it never enters the sum and so
    /// cannot contribute error. Counting it would leave the error non-zero on
    /// essentially every input and the tag permanently `None`.
    fn expm1_fixed_inner<S: BigInt>(
        reduced_arg: S, working_scale: u32, want_tag: bool) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        let mut sum = reduced_arg;
        let mut term = reduced_arg;
        let mut term_index: u128 = 2;
        // The accumulated rounding error of the INCLUDED terms, as an exact
        // rational: `err` is the running total and `eps` the previous term's
        // own contribution (the recurrence needs it). `sum` is still the exact
        // partial sum exactly while `err`'s numerator is zero. `err_lost` =
        // the error stopped being exactly representable, so nothing can be
        // proven and the tag must fail closed.
        let mut eps: (S, i128) = (zero::<S>(), 1);
        let mut err: (S, i128) = (zero::<S>(), 1);
        let mut err_lost = false;
        // The index of the term that VANISHED at the working scale, i.e. the
        // first term of the neglected tail. `None` = the loop hit the cap.
        let mut vanished_at: ::core::option::Option<u128> = ::core::option::Option::None;
        loop {
            // `term_index` is bounded by SERIES_CAP (20_000), so the cast to
            // the generic `lit`'s i128 argument is lossless.
            let divisor = lit::<S>(term_index as i128);
            // `rho` / `rem_j`: this term's two rounding remainders — the one the
            // `÷10^w` shed and the one the truncating `÷j` shed. They are what
            // the error recurrence consumes; the untagged path never reads
            // them.
            let (next_term, rho, rem_j) = if want_tag {
                // The same value `mul::<S>(term, reduced_arg, working_scale)
                // / divisor` produces, with the two remainders recorded on the
                // way through.
                let product = term.wrapping_mul_low_u128(reduced_arg);
                let scaled = round_div_pow10::<S>(product, working_scale);
                let reconstructed = scaled.wrapping_mul_low_u128(one::<S>(working_scale));
                let (quotient, remainder) = div_rem_exact::<S>(scaled, divisor);
                (quotient, product - reconstructed, remainder)
            } else {
                (mul::<S>(term, reduced_arg, working_scale) / divisor,
                    zero::<S>(), zero::<S>())
            };
            term = next_term;
            if term == zero::<S>() {
                vanished_at = ::core::option::Option::Some(term_index);
                break;
            }
            // Only a term that is actually ADDED can carry error into `sum`,
            // so the accumulator advances HERE, past the vanish break.
            //
            // Nothing moves while the running error is still exactly zero AND
            // this term's own two divisions came out exact: `eps_j` is then
            // zero and both rationals stand. That is the overwhelmingly common
            // case, and short-circuiting it keeps the exact-input path clear of
            // the recurrence's extra divide — the cost lands only on inputs the
            // previous rule wrote off as `None` anyway.
            let quiet = eps.0 == zero::<S>() && rho == zero::<S>() && rem_j == zero::<S>();
            if want_tag && !err_lost && !quiet {
                match term_error::<S>(
                    eps, rho, rem_j, term_index as i128, reduced_arg, working_scale)
                    .and_then(|step| add_error::<S>(err, step).map(|total| (step, total)))
                {
                    ::core::option::Option::Some((step, total)) => {
                        eps = step;
                        err = total;
                    }
                    ::core::option::Option::None => err_lost = true,
                }
            }
            sum = sum + term;
            term_index += 1;
            if term_index > SERIES_CAP {
                break;
            }
        }
        let tag = match vanished_at {
            ::core::option::Option::Some(vanish_index)
                if want_tag
                    && !err_lost
                    && err.0 == zero::<S>()
                    && reduced_arg != zero::<S>()
                    && reduced_arg.abs() <= one::<S>(working_scale) =>
            {
                // The tail is `s^n/n! + s^(n+1)/(n+1)! + ...`, alternating and
                // strictly decreasing in magnitude for `|s| <= 1`, so it
                // carries its first term's sign: positive throughout for
                // `s > 0`, else `(-1)^n`.
                ::core::option::Option::Some(
                    if reduced_arg > zero::<S>() || vanish_index % 2 == 0 {
                        TailSign::Above
                    } else {
                        TailSign::Below
                    })
            }
            _ => ::core::option::Option::None,
        };
        (sum, tag)
    }

    /// Argument-magnitude regime of `e^v` for a `working_value`
    /// at `working_scale` in the work integer `S`, decided BEFORE the
    /// `k = round(v / ln 2)` range-reduction division runs.
    ///
    /// [`exp_fixed`] / [`exp_internal_peak_bits`] first compute `k` with a
    /// full work-integer divide whose dividend is `v_w · 10^w`. For an
    /// argument deep past the representable range that division is itself
    /// unsafe: the quotient `k` can exceed `i128` (so `round_to_nearest_int`
    /// TRUNCATES it — a wrapped, even wrong-signed `k` that silently routes
    /// an overflow down the underflow path, or vice versa), and the peak
    /// model's `|k| · 30103` product can exceed `u128`. This classifier
    /// bounds the argument analytically from its BIT LENGTH alone — no
    /// division — so the deep bands never reach that arithmetic.
    ///
    /// Derivation (both bounds are SUFFICIENT conditions, never fired by a
    /// representable cell):
    ///
    /// * **Overflow** (`v > 0`): the result `e^v` at scale `w` needs
    ///   `e^v · 10^w < 2^BITS`, i.e. `v < BITS·ln 2 − w·ln 10`. With
    ///   `R = ⌊BITS·6932/10000⌋ + 1 − ⌊w·23025/10000⌋ ≥ BITS·ln 2 − w·ln 10`
    ///   (0.6932 over-approximates ln 2, 2.3025 under-approximates ln 10),
    ///   and `|v| ≥ 2^(bl−1)/10^w` for `bit_len = bit_length(working_value)`,
    ///   the result provably overflows `S` once
    ///   `bit_len ≥ ⌈w·33220/10000⌉ + bits(R) + 2`
    ///   (because `2^(bl−1) ≥ 2^⌈w·3.3220⌉ · 2^bits(R) · 2 ≥ 10^w · R`,
    ///   with 3.3220 over-approximating log2 10 and `2^bits(R) ≥ R`).
    /// * **Underflow** (`v < 0`): `e^v < 10^−(w+1)` — strictly below the
    ///   working resolution — once `|v| ≥ (w+1)·ln 10`. With
    ///   `U = ⌊(w+1)·23026/10000⌋ + 1 ≥ (w+1)·ln 10` (2.3026 over-
    ///   approximates ln 10) the same bit-length argument gives the
    ///   threshold `bit_len ≥ ⌈w·33220/10000⌉ + bits(U) + 2`.
    ///
    /// A cell that does NOT fire has `|v|` within a small constant factor of
    /// the fired bound, so `|k| = |v|/ln 2` stays of order `BITS` — every
    /// downstream `i128` / `u128` / `u32` use of `k` is then in range, and
    /// the `k`-division dividend stays inside the divide scratch every
    /// build provisions for its in-range work.
    enum ArgRegime {
        /// Argument small enough for the body's range reduction.
        Fits,
        /// `v > 0` and `e^v · 10^w` provably exceeds `S`'s capacity.
        Overflow,
        /// `v < 0` and `e^v` is provably below the working resolution.
        Underflow,
    }

    /// Classifies `working_value` per [`ArgRegime`]'s analytic bounds. See the
    /// enum docs for the derivation.
    fn arg_regime<S: BigInt>(working_value: S, working_scale: u32) -> ArgRegime {
        if working_value == S::ZERO {
            return ArgRegime::Fits;
        }
        let bit_len = bit_length::<S>(working_value) as u64;
        // ⌈w · log2(10)⌉, over-approximated (33220/10000 ≥ log2 10).
        let working_scale_bits = ((working_scale as u64) * 33220).div_ceil(10000);
        // bits(x) = floor(log2 x) + 1, so 2^bits(x) ≥ x.
        let bits_of = |x: u64| 64 - x.leading_zeros() as u64;
        if working_value > S::ZERO {
            let bits_ln2 = (<S as BigInt>::BITS as u64) * 6932 / 10000 + 1;
            let scale_ln10 = (working_scale as u64) * 23025 / 10000;
            // R ≥ BITS·ln2 − w·ln10; clamp at 1 (a degenerate `w` no caller
            // forms — 10^w would not even fit S — but keep the math total).
            let overflow_arg_bound = bits_ln2.saturating_sub(scale_ln10).max(1);
            if bit_len >= working_scale_bits + bits_of(overflow_arg_bound) + 2 {
                return ArgRegime::Overflow;
            }
        } else {
            let underflow_arg_bound = ((working_scale as u64) + 1) * 23026 / 10000 + 1;
            if bit_len >= working_scale_bits + bits_of(underflow_arg_bound) + 2 {
                return ArgRegime::Underflow;
            }
        }
        ArgRegime::Fits
    }

    /// True worst-case bit-width the [`exp_fixed`] body reaches internally
    /// for a `working_value` at `working_scale`, in a work integer `S`
    /// of capacity `S::BITS` bits.
    ///
    /// Mirrors [`exp_fixed`]'s own `k` / `extra_digits` /
    /// `extended_working_scale` arithmetic EXACTLY (range-reduce
    /// `v = k·ln2 + s`, lift the working scale by
    /// `extra_digits`, run the Taylor squarings at the extended scale, then reassemble
    /// `2^k · exp(s)`), so the fit gate models the real squaring-reassembly
    /// PEAK — twice the extended scale in decimal digits for the symmetric `sum²` plus the
    /// `sum << k` shift — NOT just the final result magnitude. The body's
    /// `wrapping_sqr_low_u128` / `wrapping_mul_low_u128` return the low bits,
    /// so an internal peak that exceeds `S::BITS` silently TRUNCATES (an
    /// overflowed square collapses to 0) and the post-narrowing fit check —
    /// which only sees the wrapped, small result — never fires. This model
    /// lets [`exp_fixed`] reject such an argument UP FRONT instead.
    ///
    /// This is the width-generic single source for the peak estimate; the
    /// per-tier `decl_wide_transcendental!` `exp_internal_peak_bits` /
    /// `exp_fits_w` / `hyper_fits_w` gates delegate to it.
    pub(crate) fn exp_internal_peak_bits<S: BigInt>(working_value: S, working_scale: u32) -> u64 {
        // Argument-magnitude pre-gate (see [`ArgRegime`]): a deep argument
        // must not reach the `k` division below — its quotient can exceed
        // `i128` and its dividend the divide scratch. BOTH non-`Fits`
        // verdicts report an unbounded peak. For Overflow no `S` fits the
        // result. For Underflow the VALUE is tiny, but this function models
        // the peak of the UNGATED per-tier body its `exp_fits_w` callers
        // would run — and that body's range reduction provisions
        // `extra_digits ≈ |k|·0.30103` even for a deep NEGATIVE `k`,
        // pushing the extended scale and the `k·ln2` term past the tier work integer
        // (an `Int: mul overflow`). Reporting "does not fit" keeps such a
        // cell on the wider-lift route the deep band always took, where
        // [`exp_fixed`]'s own pre-gate / `k < -1` short-circuit returns the
        // canonical smallest-positive value without forming any of that
        // arithmetic.
        //
        // UNBOUNDED (no `S::Scratch` clause): the per-tier gate shells the
        // `decl_wide_transcendental!` macro emits (`exp_fits_w` /
        // `hyper_fits_w`) call this through a bare `<S: BigInt>` signature,
        // so the `k` estimate keeps the blanket `div_rem` route — the
        // status-quo path, whose per-tier operands the build blanket has
        // always covered. The exact-scratch path ([`try_exp_fixed`]) feeds
        // its own `k` to [`exp_peak_bits_model`] instead.
        if !matches!(arg_regime::<S>(working_value, working_scale), ArgRegime::Fits) {
            return u64::MAX;
        }
        let one_at_working_scale = one::<S>(working_scale);
        let ln2_at_working_scale = ln2::<S>(working_scale);
        let k = round_to_nearest_int_blanket(
            round_div_blanket(
                working_value.wrapping_mul_low_u128(one_at_working_scale),
                ln2_at_working_scale),
            working_scale,
        );
        exp_peak_bits_model::<S>(working_scale, k)
    }

    /// Blanket-scratch sibling of [`round_div`] (the `Int` operator's own
    /// `div_rem`), kept ONLY for [`exp_internal_peak_bits`]'s macro-facing
    /// unbounded signature — see there.
    fn round_div_blanket<S: BigInt>(numerator: S, divisor: S) -> S {
        let (quotient, remainder) = numerator.div_rem(divisor);
        if remainder == S::ZERO {
            return quotient;
        }
        let abs_remainder = remainder.abs();
        let complement = divisor.abs() - abs_remainder;
        let remainder_cmp = if abs_remainder < complement {
            ::core::cmp::Ordering::Less
        } else if abs_remainder > complement {
            ::core::cmp::Ordering::Greater
        } else {
            ::core::cmp::Ordering::Equal
        };
        let quotient_is_odd = quotient.bit(0);
        let result_is_positive = (numerator < S::ZERO) == (divisor < S::ZERO);
        if crate::support::rounding::should_bump(
            RoundingMode::HalfToEven,
            remainder_cmp,
            quotient_is_odd,
            result_is_positive,
        ) {
            if result_is_positive { quotient + S::ONE } else { quotient - S::ONE }
        } else {
            quotient
        }
    }

    /// Blanket-scratch sibling of [`round_to_nearest_int`] — see
    /// [`round_div_blanket`].
    fn round_to_nearest_int_blanket<S: BigInt>(working_value: S, working_scale: u32) -> i128 {
        let divisor = pow10::<S>(working_scale);
        let (quotient, remainder) = working_value.div_rem(divisor);
        let half = divisor >> 1;
        let rounded_quotient = if remainder.abs() >= half {
            if working_value < S::ZERO { quotient - S::ONE } else { quotient + S::ONE }
        } else {
            quotient
        };
        crate::int::types::traits::BigInt::to_i128(rounded_quotient)
    }

    /// Number of repeated-squaring levels the [`try_exp_fixed`] Taylor core
    /// runs at `extended_working_scale`: the largest `n ≥ 1` with
    /// `(n+1)² ≤ level_bound` for `level_bound = 3·ext + 1` (so
    /// `n ≈ √(3·ext)`).
    /// Shared by the body and the `k < 0` internal-peak clamp, which must
    /// evaluate the chain depth at the CLAMPED width.
    fn squaring_levels(extended_working_scale: u32) -> u32 {
        let level_bound = extended_working_scale.saturating_mul(3).saturating_add(1);
        let mut levels: u32 = 1;
        while (levels + 1) * (levels + 1) <= level_bound {
            levels += 1;
        }
        levels
    }

    /// The pure peak model for an ALREADY-computed range-reduction `k` —
    /// the divide-free tail of [`exp_internal_peak_bits`], shared with
    /// [`try_exp_fixed`] (which holds `k` from its own exact-scratch
    /// divide and must not re-derive it through the blanket).
    fn exp_peak_bits_model<S: BigInt>(working_scale: u32, k: i128) -> u64 {
        let abs_k_u128 = if k < 0 { -k } else { k } as u128;
        let extra_digits: u32 = if abs_k_u128 == 0 {
            0
        } else {
            // Saturating: `Fits` bounds `|k|` to order `BITS`, far inside
            // `u128`, but saturation keeps the model an UPPER bound (more
            // digits → a larger modelled peak → the gate fires) even if a
            // caller ever feeds an unclassified extreme.
            let digits = abs_k_u128.saturating_mul(30103).div_ceil(100_000);
            let capped = digits.min((<S as BigInt>::BITS / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };
        let extended_working_scale = (working_scale + extra_digits) as u64;
        // digits → bits: `log2(10) ≈ 3.3220 ≈ 3322/1000`.
        // Squaring peak: the symmetric `sum²` before the round-divide spans
        // twice the extended scale in decimal digits.
        let squaring_bits = 2 * extended_working_scale * 3322 / 1000;
        // Reassembly peak: `sum << k` lifts the extended-scale Taylor sum by
        // `|k|` bits. Saturating narrowing, same upper-bound rationale as
        // the `digits` product above.
        let reassembly_bits = (extended_working_scale * 3322 / 1000)
            .saturating_add(u64::try_from(abs_k_u128).unwrap_or(u64::MAX));
        let peak =
            if squaring_bits > reassembly_bits { squaring_bits } else { reassembly_bits };
        // Small safety slack on top of the modelled peak. The model can
        // under-count the TRUE internal peak by only a few bits: `sum` can
        // reach `√2·10^ext` (e^(ln2/2)), so the symmetric `sum²` reaches
        // `2·10^(2·ext)` — `2·ext` digits PLUS the leading factor `2`
        // (≈ +2 bits the `2·ext·3322/1000` digit count omits) — plus the
        // half-LSB residue of the rounded `÷10^ext`. ~4 bits suffices to
        // keep `peak` an UPPER bound (so the gate never lets a genuine wrap
        // through); one u64 limb (64) is a generous, clean pad.
        //
        // The slack MUST stay small. It is a flat additive bit count, so on a
        // tier whose work integer `S` is NARROW it eats a large fraction of
        // the budget — and D76 is the sole tier whose `Wexp == W == Int<16>`
        // (1024 bits), so a value that overflows `W` cannot lift to anything
        // wider: the gate IS the last line, and an over-large slack
        // false-panics in-range band-edge cosh/sinh/exp (sqr_bits ≈ 910 at the
        // D76<0> edge x≈175). 64 clears that edge with room while every other
        // tier's wider `Wexp` absorbs it. A result that genuinely overflows
        // STORAGE but still fits `S` is NOT this gate's concern — it is caught
        // downstream by the narrowing fit check (`round_to_storage_with_g`,
        // which panics "result out of range"); this gate guards only the work
        // integer `S` itself wrapping.
        peak + 64
    }

    /// Whether [`exp_fixed`]'s internal squaring-reassembly peak for
    /// `(working_value, working_scale)` fits the work integer `S` without
    /// wrapping. Used by the per-tier `exp_fits_w` / `hyper_fits_w`
    /// regime-routing gates.
    #[inline]
    pub(crate) fn exp_peak_fits<S: BigInt>(working_value: S, working_scale: u32) -> bool {
        exp_internal_peak_bits::<S>(working_value, working_scale) < <S as BigInt>::BITS as u64
    }

    /// `e^v` for a `working_value`, generic over the work
    /// integer `S`. Mirrors the per-tier `$core::exp_fixed` exactly
    /// (range-reduce `v = k·ln2 + s`, extend the working scale by
    /// `extra_digits` to absorb the `2^k` amplification, run the
    /// repeated-squaring Taylor core, reassemble `2^k · exp(s)`), but
    /// stays width-generic so the caller can run it in a wider integer
    /// for the large-result regime.
    ///
    /// # Panics
    ///
    /// Panics with the stable `"result out of range"` substring when the
    /// argument is so large that the internal squaring / `2^k`-reassembly
    /// peak would exceed the work integer `S`'s capacity. The body reduces
    /// modulo `2^BITS` (`wrapping_sqr_low_u128`), so an unchecked overflow
    /// here would silently TRUNCATE — collapsing a far-out-of-range result
    /// to a small (often zero) value that then slips through the caller's
    /// post-narrowing fit check. Failing loudly at the work integer it can
    /// no longer represent keeps the strict-transcendental overflow contract
    /// uniform: a result out of range PANICS at every tier and scale (in
    /// both debug and release), never returns a wrapped value. The caller
    /// runs this in the WIDEST work integer it can (`Wexp` / `WNarrow`); the
    /// panic fires only when even that cannot hold the peak — a genuinely
    /// unrepresentable result.
    pub(crate) fn exp_fixed<S: BigInt>(working_value: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        try_exp_fixed::<S>(working_value, working_scale)
            .unwrap_or_else(|| panic!("exp_generic::exp_fixed: result out of range"))
    }

    /// Whether the DIRECT `expm1` series reaches working scale `w` in no more
    /// terms than [`try_exp_fixed`]'s Smith chain spends on its squarings
    /// alone — i.e. the direct path is never the more expensive of the two.
    ///
    /// The chain runs `n = squaring_levels(w)` squarings whatever the
    /// argument (`n` is keyed on `w`, not on `|s|`), so `n` terms is the
    /// honest budget. The direct series' term `j` is `s^j/j!`, which vanishes
    /// at scale `w` once `j·d >= w` for `d = -log10|s|`; dropping the `j!`
    /// growth makes this a SUFFICIENT condition, erring toward the existing
    /// path. With `D` the decimal digit count of the working magnitude,
    /// `d = w - D + 1`.
    ///
    /// This is a cost gate, not a validity wall — both paths are correct
    /// kernels at every argument it can see, so a mis-estimate costs speed and
    /// never accuracy. The BUDGET it compares against is keyed on `w` alone;
    /// the comparison itself also reads the argument's magnitude, through `D`.
    /// No cell — no `N`, no `SCALE` — appears in either.
    fn direct_series_pays<S: BigInt>(working_value: S, working_scale: u32) -> bool {
        let magnitude = working_value.abs();
        if magnitude == zero::<S>() {
            return false;
        }
        // `digits <= floor(bl·log10 2) + 1`, at most one high — an over-estimate
        // shrinks the exponent and so only ever withholds the direct path.
        let digit_count = ((bit_length::<S>(magnitude) as u64 * 30_103) / 100_000) as u32 + 1;
        let decimal_exponent = (working_scale + 1).saturating_sub(digit_count);
        let squarings = squaring_levels(working_scale) as u64;
        squarings.saturating_mul(decimal_exponent as u64) >= working_scale as u64
    }

    /// `e^v` at working scale `w`, together with the side its neglected tail
    /// puts the TRUE value on where that is provable — the [`TailSign`] the
    /// Ziv walkers need when a zero residual leaves them blind.
    ///
    /// `try_exp_fixed` cannot supply one. It range-reduces and then runs
    /// `squaring_levels(w)` rounded divides (61 at `D1232<1231>`), each up to
    /// half a working unit with its direction untracked, while the neglected
    /// Taylor tail it would be reporting on is SUB-unit. The noise swamps the
    /// signal, so any tag from that path would be an assertion, not a proof.
    ///
    /// Where the direct series pays for itself ([`direct_series_pays`]) this
    /// evaluates `e^v = 1 + expm1(v)` instead. The `1` is `10^w` exactly, so
    /// the addition is exact and the side transfers unchanged from
    /// [`expm1_fixed_tagged`], whose own rule already fails closed (it tags
    /// only when the accumulated error is provably zero and the series
    /// vanished rather than hit the cap). Elsewhere the value is
    /// `try_exp_fixed`'s, bit-identical to before, and the tag is `None`.
    pub(crate) fn exp_fixed_tagged<S: BigInt>(
        working_value: S,
        working_scale: u32,
    ) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        exp_fixed_tagged_with::<S>(working_value, working_scale, || {
            exp_fixed::<S>(working_value, working_scale)
        })
    }

    /// [`exp_fixed_tagged`] over a caller-supplied untagged kernel — the tier
    /// cores route their own `exp_fixed` through a peak-fit gate that lifts to
    /// a wider work integer, so the fallback must stay THEIRS rather than
    /// being re-derived here. Only the gated direct-series branch is shared.
    pub(crate) fn exp_fixed_tagged_with<S: BigInt>(
        working_value: S,
        working_scale: u32,
        fallback: impl FnOnce() -> S,
    ) -> (S, Option<TailSign>)
    where
        S::Scratch: ComputeLimbs,
    {
        if direct_series_pays::<S>(working_value, working_scale) {
            let (expm1_value, tail_sign) = expm1_fixed_tagged::<S>(working_value, working_scale);
            (one::<S>(working_scale) + expm1_value, tail_sign)
        } else {
            (fallback(), None)
        }
    }

    /// Option-returning core of [`exp_fixed`] — the `checked_` seam's
    /// primitive. `None` means the internal squaring / `2^k`-reassembly
    /// peak provably exceeds the work integer `S`'s capacity: for `k ≥ 0`
    /// the result itself is out of range for any storage `S` serves at
    /// scale `w`; for `k < 0` (where the result is small but the
    /// working-precision lift peaks just as high) it means even the
    /// capacity-clamped working precision cannot deliver the digits the
    /// caller needs — either way the value cannot be computed in `S`
    /// without wrapping, and `None` is the explicit signal in place of a
    /// silently wrapped value. The seamed narrow kernels propagate it
    /// (their policy dispatch wrapper applies the default form's
    /// contractual panic), while [`exp_fixed`] panics directly for the
    /// unseamed callers — one detection, each wrapper applies its policy.
    pub(crate) fn try_exp_fixed<S: BigInt>(working_value: S, working_scale: u32) -> Option<S>
    where
        S::Scratch: ComputeLimbs,
    {
        // Argument-magnitude pre-gate (see [`ArgRegime`]). The very first
        // step below — `k = round(v / ln 2)` — is a full work-integer divide
        // on the `v_w · 10^w` dividend; for a deep argument that division is
        // the FIRST thing to break (an `i128`-truncated `k` silently flips
        // an overflow into the underflow path, and the oversized dividend
        // outruns the divide scratch a narrow build provisions), so the
        // out-of-range verdict must come BEFORE it. A provable overflow is
        // the uniform out-of-range signal; a provable underflow returns
        // the smallest positive working value exactly as the in-body
        // short-circuits below do (the caller's rounding turns it into 0,
        // or 1 ULP under Ceiling).
        match arg_regime::<S>(working_value, working_scale) {
            ArgRegime::Overflow => return None,
            ArgRegime::Underflow => return Some(lit::<S>(1)),
            ArgRegime::Fits => {}
        }
        let one_at_working_scale = one::<S>(working_scale);
        let ln2_at_working_scale = ln2::<S>(working_scale);
        let pow10_at_working_scale = one_at_working_scale;
        let k = round_to_nearest_int(
            div_cached(working_value, ln2_at_working_scale, pow10_at_working_scale),
            working_scale);
        // Deep underflow: e^v < 10^-w, so its working value is sub-resolution. For
        // a very negative k the extra-guard range reduction below provisions
        // `extra_digits ≈ |k|·0.3`, pushing the extended scale and the `k·ln2`
        // term past the
        // work integer S's capacity (an `Int: mul overflow`). Short-circuit to the
        // smallest positive working value, preserving the positive sub-resolution
        // so the caller rounds correctly (0 under nearest, the smallest positive
        // under Ceiling). Sufficient condition: e^v < 2^(k+1) <= 10^-w, i.e.
        // -(k+1)·log10(2) >= w  (log10(2) ≈ 30103/100000).
        if k < -1 {
            let underflow_depth = (-(k + 1)) as u128;
            if underflow_depth.saturating_mul(30103)
                >= (working_scale as u128).saturating_mul(100_000)
            {
                return Some(lit::<S>(1));
            }
        }
        // Overflow guard (positive results only). For `k >= 0`, `e^v >= 1` and
        // grows without bound; once the internal squaring / `2^k`-reassembly
        // peak exceeds `S::BITS` the body's `wrapping_*` arithmetic would
        // silently TRUNCATE the result (an overflowed square collapses to 0),
        // and the caller's post-narrowing fit check — seeing only the wrapped,
        // small value — would never fire, letting a far-out-of-range result
        // escape as a wrong (often zero) value. A fixed-width decimal has no
        // ∞/NaN, so there is nothing to return: signal out-of-range, uniform
        // across every tier and scale, in both debug and release (the
        // [`exp_fixed`] wrapper panics; the seamed callers propagate `None`).
        // The caller runs this in the WIDEST work integer it can (`Wexp` /
        // `WNarrow`); the verdict fires only when even that cannot hold the
        // peak — a genuinely unrepresentable result. (`k < 0` is the
        // underflow direction, handled by the short-circuits above and
        // below — never out of range.)
        if k >= 0 && exp_peak_bits_model::<S>(working_scale, k) >= <S as BigInt>::BITS as u64 {
            return None;
        }
        let abs_k_u128 = if k < 0 { -k } else { k } as u128;
        let extra_digits: u32 = if abs_k_u128 == 0 {
            0
        } else {
            // Saturating for the same upper-bound reason as the peak model;
            // the pre-gate already bounds `|k|` to order `BITS` here.
            let digits = abs_k_u128.saturating_mul(30103).div_ceil(100_000);
            let capped = digits.min((<S as BigInt>::BITS / 4) as u128) as u32;
            capped + 12 + (capped >> 2)
        };

        // `k < 0` internal-peak clamp. The `k >= 0` gate above does not cover
        // the negative-`k` band, yet the squaring chain's peak grows with
        // `extended_working_scale = working_scale + extra_digits` REGARDLESS
        // of `k`'s sign: every squaring forms
        // the full symmetric product `sum²` (`wrapping_sqr_low_u128`) BEFORE
        // its `÷10^ext`, and `sum` reaches up to `√2·10^ext`
        // (`e^(ln2/2)`, `s` at the range-reduction band edge), so the peak
        // intermediate reaches `2·10^(2·ext)`. For a deep-negative `k` the
        // un-clamped `extra_digits ≈ 1.25·|k|·log10(2) + 12` pushes that peak past
        // `S`'s capacity and the low-bits square WRAPS — `S`'s sign bit sets
        // and a NEGATIVE "e^x" is handed back (the exp(-62.175)·10^184
        // Int<24> instance: `k = -90`, `extra_digits = 47`, `ext = 231`,
        // `e^s·10^462 = 1.0219·2^1535`). Bound the peak and clamp the extra so
        // it provably fits; the clamp only ENGAGES where the un-clamped path
        // is past the provable-fit line, so every cell that fits today keeps
        // its bit-identical path.
        //
        // Capacity bound (sufficient no-wrap condition): the chain's largest
        // intermediate is `sum² < 2·10^(2·ext)·(1+ε)` with
        // `ε ≤ 2^(n+1)·(T+2)·10^-ext ≪ 2^-30` (the chain's accumulated
        // relative error, see the precision floor below), and the signed `S`
        // holds magnitudes below `2^(BITS-1)`. So it suffices that
        //   bits(2.0…·10^(2·ext)) ≤ 2·ext·log2(10) + 2  ≤  BITS − 2.
        // With the rational over-approximation log2(10) < 3322/1000 this is
        // implied by the integer condition
        //   ext · 6644 ≤ (BITS − 4) · 1000,
        // i.e. `ext ≤ W_EXT_CAP = (BITS − 4)·1000 / 6644` (floor). For
        // Int<24> (BITS = 1536): W_EXT_CAP = 230 — worst-case peak
        // `2·10^460 = 0.0166·2^1535` (fits), while the defect instance's
        // `ext = 231` reaches `1.0219·2^1535` (wraps). Every other
        // intermediate is strictly smaller:
        // `|extended_working_value| ≤ (|k|+1)·ln2·10^ext`
        // (bits ≈ log2|k| + ext·3.33 ≪ 2·ext·3.32), the `k·ln2` term is
        // the same size, and each Taylor `term·halved_arg` product is bounded
        // by `sum²`'s width.
        //
        // Precision floor (the clamp must not degrade correctness): with the
        // clamped `clamped_extra_digits` the kernel's absolute error at the
        // caller's `working_scale`, in units of `10^-w`, is bounded by
        //   err ≤ [√2·(2^n·(T+2) + |k|/2) · 2^-|k| + 1] · 10^-extra_c + 0.5
        // where `n = squaring_levels(w + extra_c)` (each squaring doubles the
        // chain's relative error and adds a half-unit rounding), `T ≤ 1.2·n+4`
        // is the Taylor term count (so `√2·(2^n·(T+2) + |k|/2) ≤ 2^(n+10)`,
        // using `|k| < 2^15` from the `Fits` pre-gate + `k < -1`
        // short-circuit), and the `2^-|k|` attenuation is the `sum >> |k|`
        // down-shift every `k < 0` reassembly applies — a deep-negative `k`
        // shrinks the chain noise by exactly the factor the result shrinks.
        // The `+1` is the shift-truncation unit and `0.5` the final rounded
        // `÷10^extra_c`. So `err ≤ 1` once
        //   extra_c ≥ ceil( max(0, n + 10 − |k|) · log10(2) ) + 1
        // (`10^(0.30103·b) ≥ 2^b`). A clamped cell that cannot meet this
        // floor genuinely needs more precision than `S` can hold at this `w`
        // — return the explicit `None` (the try_* contract's insufficient /
        // out-of-range signal), NEVER a silently wrapped value. At the
        // established instance (w = 184, |k| = 90): extra_c = 230 − 184 = 46,
        // n = squaring_levels(230) = 26, deficit = max(0, 36 − 90) = 0,
        // floor = 1 ≤ 46 — the clamp delivers with margin.
        let extra_digits: u32 = if k >= 0 {
            extra_digits
        } else {
            let w_ext_cap = ((<S as BigInt>::BITS as u64 - 4) * 1000 / 6644) as u32;
            if (working_scale as u64) + (extra_digits as u64) <= w_ext_cap as u64 {
                // Peak provably fits — the unchanged, bit-identical path.
                extra_digits
            } else {
                let clamped_extra_digits = w_ext_cap.saturating_sub(working_scale);
                let clamped_levels =
                    squaring_levels(working_scale + clamped_extra_digits) as u64;
                // `|k|` is far below u64 here (`Fits` bounds it to order
                // BITS); the `min` only keeps the cast total.
                let abs_k_u64 = abs_k_u128.min(u64::MAX as u128) as u64;
                let deficit_bits = (clamped_levels + 10).saturating_sub(abs_k_u64);
                let min_extra_digits = (deficit_bits * 30103).div_ceil(100_000) as u32 + 1;
                if clamped_extra_digits < min_extra_digits {
                    return None;
                }
                clamped_extra_digits
            }
        };

        let extended_working_scale = working_scale + extra_digits;
        let extended_working_value = if extra_digits == 0 {
            working_value
        } else {
            working_value * pow10::<S>(extra_digits)
        };
        let one_at_extended_scale = one::<S>(extended_working_scale);
        let ln2_at_extended_scale = ln2::<S>(extended_working_scale);
        let reduced_arg = extended_working_value - scale_by_k(ln2_at_extended_scale, k);

        let levels = squaring_levels(extended_working_scale);

        let halved_arg = reduced_arg >> levels;
        let mut sum = one_at_extended_scale + halved_arg;
        let mut term = halved_arg;
        let mut term_index: u128 = 2;
        loop {
            term = mul(term, halved_arg, extended_working_scale)
                / lit::<S>(term_index as i128);
            if term == S::ZERO {
                break;
            }
            sum = sum + term;
            term_index += 1;
            if term_index > SERIES_CAP {
                break;
            }
        }

        let mut squared = sum;
        let mut i = 0;
        while i < levels {
            // Dedicated low-half symmetric SQUARE through the limb-width
            // matcher (`wrapping_sqr_low_u128` → `int::policy::sqr_low`): the
            // u128-packed `sqr_low_limb` on even work widths (half the limbs),
            // bit-identical to the low-`BITS` of `x²`. The squaring sibling of
            // the Taylor `mul`'s `wrapping_mul_low_u128`; feeds the same divide.
            squared = round_div_pow10(squared.wrapping_sqr_low_u128(), extended_working_scale);
            i += 1;
        }
        let sum = squared;

        let exp_at_extended_scale = if k >= 0 {
            let shift = k as u32;
            if bit_length(sum) + shift >= <S as BigInt>::BITS {
                return None;
            }
            sum << shift
        } else {
            let right_shift_bits = -k as u128;
            if right_shift_bits >= bit_length(sum) as u128 {
                // Deep underflow: e^x (x < 0 here, since k < 0) is strictly
                // positive but below the working resolution. Return the
                // smallest positive working value (1 = 10^-w), NOT zero, so the
                // directed narrowing preserves the sign — Ceiling rounds up to
                // 1 ULP while Floor / Trunc / nearest still give 0. Returning a
                // bare zero loses positivity and rounds Ceiling to 0 (a
                // correctly-rounded defect). Reached only by direct e^(negative)
                // — the hyperbolics call `exp_fixed` on |x| >= 0.
                return Some(lit::<S>(1));
            }
            sum >> (right_shift_bits as u32)
        };
        let exp_value = if extra_digits == 0 {
            exp_at_extended_scale
        } else {
            round_div_pow10(exp_at_extended_scale, extra_digits)
        };
        // e^v > 0 for every finite v: a zero result here is genuine underflow
        // of `e^(negative)` below the working resolution, not a true zero.
        // Return the smallest positive value so the directed narrowing rounds
        // Ceiling up to 1 ULP (a bare zero would round Ceiling to 0 — a
        // correctly-rounded defect). Restricted to `k < 0`: for `k >= 0`,
        // `e^v >= 1`, so a 0 result would mean the working width overflowed,
        // and masking it as 1 would hide the defect rather than fix it.
        if k < 0 && exp_value == zero::<S>() {
            Some(lit::<S>(1))
        } else {
            Some(exp_value)
        }
    }

    /// Narrows a `Wexp`-computed `value` back down to the tier's
    /// own work integer `Dst`, panicking UNIFORMLY when it does not fit.
    ///
    /// The wide `exp` / hyperbolic compositions evaluate in the wider `Wexp`
    /// (their squaring peak needs the extra width — that is why the per-tier
    /// `exp_fits_w` / `hyper_fits_w` gate lifted them there), then narrow the
    /// result back to the tier work integer `Dst`. A genuinely out-of-range
    /// result — `e^|x|` larger than the tier can represent — is correctly
    /// computed at `Wexp` but EXCEEDS `Dst` at this step. A bare
    /// [`BigInt::resize_to`] would silently TRUNCATE it to `Dst`'s low bits,
    /// yielding a small wrapped value that then slips through the downstream
    /// storage-narrowing fit check — the exact silent-overflow the strict
    /// transcendental contract forbids. Detect it here and PANIC instead
    /// ("result out of range", identical at every tier and scale, in both
    /// debug and release).
    ///
    /// This is the structural twin of [`exp_fixed`]'s own peak gate: the gate
    /// guards the *work integer wrapping* during the squaring; this guards the
    /// *narrow back to the tier width* once the (correct) result is in hand.
    /// In-range results provably fit `Dst` — the tier work integer holds any
    /// value whose storage representation is in range at the lifted working
    /// scale — so this never fires for a representable cell. When
    /// `Dst == Wexp` (the widest tier, where no narrowing happens) it is a
    /// cheap bit-length compare that always passes. The hyperbolic results are
    /// non-negative (`e^|x|`, `cosh`, `sinh(|x|)`), so the magnitude test is
    /// exact: a value needing `≥ Dst::BITS` significant bits cannot fit the
    /// signed `Dst`.
    #[inline]
    pub(crate) fn resize_or_panic<Src: BigInt, Dst: BigInt>(value: Src) -> Dst {
        if bit_length::<Src>(value.abs()) >= <Dst as BigInt>::BITS {
            panic!("exp_generic: result out of range");
        }
        <Src as BigInt>::resize_to::<Dst>(value)
    }

    /// `(numerator · 10^working_scale) / divisor`, rounded half-to-even (the
    /// generic sibling of the per-tier `$core::div`).
    #[inline]
    pub(crate) fn div<S: BigInt>(numerator: S, divisor: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        // `(a·10^w)/b`, half-to-even. `10^w` comes from the `pow10` POLICY
        // (`pow10::dispatch`, via `pow10::<S>`), NOT a per-tier baked static;
        // the numerator product is the u128-packed truncated-low mul (the
        // macro `div`'s kernel) so routing through the policy costs no
        // multiply speed.
        round_div(numerator.wrapping_mul_low_u128(pow10::<S>(working_scale)), divisor)
    }

    /// `sinh(|x|)` at `working_scale` for a non-negative
    /// `abs_working_value` (= `|x|·10^w`), computed entirely in `S`:
    /// `(e^|x| − e^-|x|)/2`. The dominant `e^|x|` term is evaluated
    /// directly (`exp_fixed`) and the small `e^-|x|` via reciprocal, so
    /// the small term's relative error stays a small *absolute* error.
    pub(crate) fn sinh_pos<S: BigInt>(abs_working_value: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let exp_x = exp_fixed::<S>(abs_working_value, working_scale);
        let exp_neg_x = div(one::<S>(working_scale), exp_x, working_scale);
        (exp_x - exp_neg_x) >> 1
    }

    /// `cosh(|x|) = (e^|x| + e^-|x|)/2` at `working_scale`. See
    /// [`sinh_pos`].
    pub(crate) fn cosh_pos<S: BigInt>(abs_working_value: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let exp_x = exp_fixed::<S>(abs_working_value, working_scale);
        let exp_neg_x = div(one::<S>(working_scale), exp_x, working_scale);
        (exp_x + exp_neg_x) >> 1
    }

    /// `tanh(|x|) = (e^|x| − e^-|x|)/(e^|x| + e^-|x|)` at
    /// `working_scale`. See [`sinh_pos`].
    pub(crate) fn tanh_pos<S: BigInt>(abs_working_value: S, working_scale: u32) -> S
    where
        S::Scratch: ComputeLimbs,
    {
        let one_at_working_scale = one::<S>(working_scale);
        // Past the all-nines saturation onset |x| ≳ ln(10)/2·w ≈ 1.1513·w,
        // tanh(|x|) rounds to 1 − 10^−w; return that directly.
        let saturation_bound = (working_scale as i128) * 1152 / 1000 + 2;
        let saturated = one_at_working_scale - lit::<S>(1);
        // `div_rem_exact` (not the `/` operator) — the narrow build's
        // blanket divide scratch is below this work width.
        if div_rem_exact(abs_working_value, one_at_working_scale).0
            > lit::<S>(saturation_bound)
        {
            return saturated;
        }
        // Below `saturation_bound` use the negative-exponent identity tanh(|x|) =
        // (1 − m)/(1 + m), m = e^(−2|x|). Forming the dominant e^(+|x|) directly
        // overflows the work integer `S` once |x| ≳ (S::BITS·ln2 − w·ln10)/ln10,
        // which at high scale on a deep tier (w ≳ 0.67·S digits, e.g. D1232<924>)
        // is BELOW `saturation_bound` — a panic GAP. `exp_neg_2x` is tiny and is
        // formed by `exp_fixed`
        // on the NEGATIVE argument −2|x| (its 2^k reassembly shifts DOWN, never
        // the overflowing up-shift), so e^(+|x|) is never formed; the identity is
        // the exact tanh. Mirrors `trig_series_2limb::tanh_with_raw` (the narrow
        // path). `exp_neg_2x == 0` (defensive: unreachable since `exp_fixed` on a
        // negative argument returns >= 1 via the ArgRegime::Underflow
        // short-circuit in `try_exp_fixed` -- retained as a belt-and-suspenders
        // guard).
        let exp_neg_2x =
            exp_fixed::<S>(-(abs_working_value + abs_working_value), working_scale);
        if exp_neg_2x == lit::<S>(0) {
            return saturated;
        }
        div(
            one_at_working_scale - exp_neg_2x,
            one_at_working_scale + exp_neg_2x,
            working_scale)
    }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::int::types::Int;

    /// Sub-storage granularity handed to [`log1p_fixed_tagged`] by the kernel
    /// tests below. Immaterial to every one of them: it gates only the deeper
    /// probe, and each of these arguments is settled by the sign rule before
    /// the probe is reached. Zero states "the value IS the grid", so it cannot
    /// mask a tag even if one of them ever did reach it.
    const GRANULARITY: u32 = 0;

    /// [`round_div_sided`]'s polarity at all four sign/bump combinations plus
    /// the exact case. The `log1p` tail-sign channel rests entirely on this
    /// mapping, and it is what a fixed polarity would get wrong at half its
    /// arguments: the SAME truncation leaves the truth `Above` a positive
    /// quotient and `Below` a negative one.
    #[test]
    fn round_div_sided_reports_the_side_the_rounding_left_the_truth_on() {
        type I = Int<2>;
        let sided = |numerator: i128, divisor: i128|
            round_div_sided::<I>(lit::<I>(numerator), lit::<I>(divisor));

        // 7/3 = 2.333… — keeps q = 2, so the truth is above what came back.
        assert_eq!(sided(7, 3), (lit::<I>(2), Some(TailSign::Above)));
        // 8/3 = 2.667… — bumps to 3, a full unit past the truth.
        assert_eq!(sided(8, 3), (lit::<I>(3), Some(TailSign::Below)));
        // -7/3 — the same truncation as +7/3, opposite side.
        assert_eq!(sided(-7, 3), (lit::<I>(-2), Some(TailSign::Below)));
        // -8/3 — bumps to -3, past the truth, which is above it.
        assert_eq!(sided(-8, 3), (lit::<I>(-3), Some(TailSign::Above)));
        // Exact: no side to report.
        assert_eq!(sided(6, 3), (lit::<I>(2), None));
    }

    /// The tail-sign channel FIRES for the family that needs it, and reports
    /// OPPOSITE sides at the two signs.
    ///
    /// `t = ±10^-4` at working scale 12 is the small-scale twin of the
    /// `±10^-(S/2)` arguments that mis-round at `D462<346>` / `D616<590>`:
    /// the artanh series vanishes entirely at the working scale, so the seed
    /// divide's rounding and the dropped tail are the only error terms, and
    /// they agree. From `Q = ±p/(2 ± x)` with `p = 10^8`, `x = 10^-4`:
    ///
    /// ```text
    ///  Q(+) =  (p/2)(1 − x/2 + x²/4 − …) =  49_997_500.12499375…
    ///  Q(−) = −(p/2)(1 + x/2 + x²/4 + …) = −50_002_500.12500625…
    /// ```
    ///
    /// Both fractions are well under a half, so half-to-even keeps the
    /// truncation at BOTH signs — and truncation is toward zero, so the exact
    /// quotient is left on the away-from-zero side: `Above` for `u > 0`,
    /// `Below` for `u < 0`.
    ///
    /// A tag that were always `None` would be inert and would look exactly
    /// like "no regressions", so this pins that it is `Some` — and pins the
    /// side, which is the half a fixed polarity would get wrong.
    #[test]
    fn log1p_fixed_tagged_reports_opposite_sides_at_the_two_signs() {
        type I = Int<2>;
        let working_scale: u32 = 12;
        let argument = pow10::<I>(8);

        let (pos, pos_tag) = log1p_fixed_tagged::<I>(argument, working_scale, GRANULARITY);
        assert_eq!(pos, lit::<I>(2 * 49_997_500), "log1p(+1e-4) at w=12 is 2u");
        assert_eq!(
            pos_tag,
            Some(TailSign::Above),
            "a truncated seed divide and a positive tail both put the truth ABOVE"
        );

        let (neg, neg_tag) = log1p_fixed_tagged::<I>(-argument, working_scale, GRANULARITY);
        assert_eq!(neg, lit::<I>(-2 * 50_002_500), "log1p(-1e-4) at w=12 is 2u");
        assert_eq!(
            neg_tag,
            Some(TailSign::Below),
            "the same truncation at a negative argument puts the truth BELOW"
        );

        // The untagged wrapper is the tagged kernel with the tag dropped —
        // the value must not move.
        assert_eq!(log1p_fixed::<I>(argument, working_scale), pos);
        assert_eq!(log1p_fixed::<I>(-argument, working_scale), neg);
    }

    /// The `k < 0` internal-peak wrap: a
    /// deep-negative argument at a deep (cap-clamped Ziv probe) working
    /// scale — `exp(-62.175)` at `w = 184` in `Int<24>` (the narrow tiers'
    /// `WZiv`, 1536 bits) — range-reduces to `k = -90`, provisioning
    /// `extra = 47`, `w_ext = 231`; the final squaring then forms
    /// `e^s·10^462 = 1.0219·2^1535`, past the sign bit, and the un-guarded
    /// kernel handed back a WRAPPED, NEGATIVE "e^x" (≈ -9.5e156 for a true
    /// value of +9.948e156). The `k < 0` clamp caps `w_ext` at the
    /// provable-fit line (230 for Int<24>) instead. Asserts the guard
    /// delivers the true value — positive AND correct to well past the
    /// margin a wrapped or precision-starved value could fake:
    /// e^-62.175 · 10^184 = 9.94811020348122892…e156 (mpmath, 250 dps).
    #[test]
    fn exp_fixed_k_negative_internal_peak_clamped_int24() {
        // v = -62.175 · 10^184 = -62175 · 10^181
        let working_scale: u32 = 184;
        let working_value = lit::<Int<24>>(-62175) * pow10::<Int<24>>(181);
        let exp_value = try_exp_fixed::<Int<24>>(working_value, working_scale)
            .expect("in-range e^-62.175 at w=184 must not signal out-of-range");
        assert!(
            exp_value > zero::<Int<24>>(),
            "e^-62.175 must be strictly positive (a negative value is the wrap)"
        );
        // Tight oracle window: 9948110203481228920 · 10^138 < r·10^-184·10^184
        // < 9948110203481228921 · 10^138 (19 leading digits of the mpmath
        // value) — far beyond what the clamped precision could miss.
        let lo = lit::<Int<24>>(9_948_110_203_481_228_920) * pow10::<Int<24>>(138);
        let hi = lit::<Int<24>>(9_948_110_203_481_228_921) * pow10::<Int<24>>(138);
        assert!(
            exp_value > lo && exp_value < hi,
            "e^-62.175 · 10^184 outside its 19-digit oracle window"
        );
    }

    /// A deep-negative `exp_fixed` at a
    /// working scale `w ≥ 200` in D115's `Wexp = Int<64>` must not panic on
    /// baked-table-less builds — `w_ext = w + extra(|k|)` pushes the
    /// per-Taylor-term `÷10^w_ext` into the rescale Newton arm, whose
    /// per-call Knuth fallback dividend (`even(width + w_ext/19 + 3) + 1`
    /// u64 limbs) can outrun the build-max divide blanket (66 limbs at
    /// `MAX_WORK_N = 16`). The result itself is comfortably in range:
    /// `e^-357` at scale 200 is `10^(200 − 357·log10(e)) ≈ 8.8e44`.
    /// Pins the kernel at the exact shape: `v = -357·10^200`,
    /// `w = 200`, `S = Int<64>` (D115's production `Wexp`).
    #[cfg(any(feature = "d115", feature = "wide"))]
    #[test]
    fn exp_fixed_deep_negative_large_working_scale_int64() {
        let working_scale: u32 = 200;
        let working_value = lit::<Int<64>>(-357) * pow10::<Int<64>>(working_scale);
        let exp_value = exp_fixed::<Int<64>>(working_value, working_scale);
        // 357·log10(e) ≈ 155.057, so 10^44 < e^-357 · 10^200 < 10^45.
        assert!(exp_value > zero::<Int<64>>(), "e^-357 must stay strictly positive");
        assert!(
            exp_value > pow10::<Int<64>>(44) && exp_value < pow10::<Int<64>>(45),
            "e^-357 at working scale 200 out of its analytic bounds"
        );
    }

    /// The tag fires on an argument whose INCLUDED term was ROUNDED — the case
    /// an exactness gate refuses and the ordering rule admits.
    ///
    /// Both arguments below add a real series term AND leave `u²` inexact, so
    /// the exactness form of the rule returned `None` for them however their
    /// error terms lined up. That is the whole defect: `u` is a rounded
    /// quotient, so `u²` almost never divides `10^w`, and since `u²` feeds every
    /// term one surviving term used to kill the tag outright — leaving the
    /// directed modes blind wherever the sub-LSB imprecision is decided by a
    /// term the series keeps. The `u2_exact` assertion here is what makes this a
    /// DIFFERENTIAL: it pins that the old gate would have refused, so the test
    /// fails if the rule ever reverts to demanding exactness.
    ///
    /// Sides come from the series, not from this crate. At working scale 12,
    /// `log1p(t) = t − t²/2 + t³/3 − t⁴/4 …` gives
    ///
    /// ```text
    ///  t = +3·10⁻⁴:  3·10⁸ − 45 000 + 9 − 0.002…  = 299 955 008.998…
    ///  t = −4·10⁻⁴: −4·10⁸ − 80 000 − 21.333 − …  = −400 080 021.339…
    /// ```
    ///
    /// so the truth sits just ABOVE the returned `299 955 008` at the positive
    /// argument and just BELOW the returned `−400 080 020` at the negative one —
    /// opposite sides, which is exactly what a fixed polarity would get wrong.
    #[test]
    fn log1p_fixed_tagged_fires_when_an_included_term_was_rounded() {
        type I = Int<2>;
        let working_scale: u32 = 12;
        let one_at_working_scale = one::<I>(working_scale);

        // `u²` inexact + at least one term added: the two conditions that
        // together made the exactness gate refuse these arguments.
        let refused_by_the_exactness_gate = |argument: I| -> bool {
            let (u, _side) = div_cached_sided::<I>(
                argument,
                one_at_working_scale + one_at_working_scale + argument,
                one_at_working_scale);
            let product = u.wrapping_mul_low_u128(u);
            let u2 = round_div_pow10::<I>(product, working_scale);
            let u2_inexact = u2.wrapping_mul_low_u128(one_at_working_scale) != product;
            let term = round_div_pow10::<I>(u.wrapping_mul_low_u128(u2), working_scale);
            let (first_contribution, _remainder) = div_rem_exact::<I>(term, lit::<I>(3));
            u2_inexact && first_contribution != zero::<I>()
        };

        let pos = lit::<I>(3) * pow10::<I>(8); // t = +3·10⁻⁴
        assert!(
            refused_by_the_exactness_gate(pos),
            "the positive argument must be one the exactness gate refused"
        );
        let (pos_v, pos_tag) = log1p_fixed_tagged::<I>(pos, working_scale, GRANULARITY);
        assert_eq!(pos_v, lit::<I>(299_955_008), "log1p(3e-4) at w=12");
        assert_eq!(
            pos_tag,
            Some(TailSign::Above),
            "299 955 008.998… lies ABOVE the returned 299 955 008"
        );

        let neg = lit::<I>(-4) * pow10::<I>(8); // t = −4·10⁻⁴
        assert!(
            refused_by_the_exactness_gate(neg),
            "the negative argument must be one the exactness gate refused"
        );
        let (neg_v, neg_tag) = log1p_fixed_tagged::<I>(neg, working_scale, GRANULARITY);
        assert_eq!(neg_v, lit::<I>(-400_080_020), "log1p(-4e-4) at w=12");
        assert_eq!(
            neg_tag,
            Some(TailSign::Below),
            "-400 080 021.339… lies BELOW the returned -400 080 020"
        );

        // The untagged wrapper is the tagged kernel with the tag dropped — the
        // value must not move on either argument.
        assert_eq!(log1p_fixed::<I>(pos, working_scale), pos_v);
        assert_eq!(log1p_fixed::<I>(neg, working_scale), neg_v);
    }
}
