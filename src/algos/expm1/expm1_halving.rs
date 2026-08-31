// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

// candidate: binary halving + the E*(E+2) doubling recurrence, no ln2, not wired

//! `expm1` by pure binary halving and the cancellation-free doubling
//! recurrence.
//!
//! Reduce by an exact binary right shift, `u = v / 2^n`, evaluate the
//! leading-term-dropped Taylor series on the tiny `u`, then climb back with
//!
//! ```text
//! expm1(2u) = e^{2u} - 1 = (e^u - 1)(e^u + 1) = E * (E + 2)
//! ```
//!
//! applied `n` times. No `ln 2` constant, no range-reduction division, and no
//! `x - k*ln2` subtraction — so the reduction carries NO cancellation at all
//! (the one genuine cancellation site of the classic `k*ln 2` route).
//!
//! # Cancellation
//!
//! `E in (-1, inf)` so `E + 2 in (1, inf)`: the recurrence is a product of a
//! value and a quantity bounded away from zero, and `E + 2` is a sum of a
//! bounded-magnitude term with `2`. Neither step can cancel.
//!
//! # Error propagation
//!
//! With `P = 10^w` and `d_j` the absolute error in working units at level `j`,
//!
//! ```text
//! d_{j+1} <= d_j * (2 + 2*|E_j|) + 1/2          (E_j in value units)
//! ```
//!
//! so over `n` levels the amplification is
//!
//! ```text
//! prod_j (2 + 2*E_j) = 2^n * prod_j (1 + E_j) = 2^n * prod_j e^{v/2^{n-j}} ~ 2^n * e^v
//! ```
//!
//! — relative amplification `2^n` (identical to `exp`'s squaring chain),
//! absolute amplification `2^n * e^v`. The guard lift below provisions exactly
//! those two factors: `ceil(n*log10 2)` digits for the chain and
//! `ceil(|v|*log10 e)` digits for the growth.
//!
//! **For `v <= 0` the recurrence CONTRACTS.** Writing `E_j = -1 + eps_j` (so
//! `eps_j = e^{v/2^{n-j}}`), `E_j*(E_j + 2) = -1 + eps_j^2`: `eps` squares each
//! level and the amplification factor `2 + 2*E_j = 2*eps_j -> 0`. The whole
//! negative half-domain therefore needs no growth lift at all, which the
//! `v > 0` gate on `growth` below reflects.
//!
//! # Cost of the missing `k*ln 2`
//!
//! The classic reduction keeps every squaring on `sum ~ P` (because
//! `|s| <= ln2/2`) and applies the `2^k` growth ONCE at the end. This kernel
//! carries the growth through the recurrence, so its last product is
//! `~ e^v * P^2` rather than `~ P^2`: it needs up to `w_ext*log2(10)` MORE bits
//! of work integer for large positive `v`. That is the trade — no `ln 2`, no
//! divide, no reduction cancellation, in exchange for a taller peak on the
//! positive side. `expm1_reduced` is the other side of it.
//!
//! # Validity
//!
//! * `v <= 0`: `2*w_ext*log2(10) + 1 + 64 < BITS`.
//! * `v > 0`: `2*w_ext*log2(10) + ceil(|v|*log2 e) + 64 < BITS`.
//! * The halving depth is bounded by the caller's guard: the shift injects one
//!   working unit which the chain amplifies by `2^n`, so correct rounding needs
//!   `2^n < 1/2 * 10^guard`. The `n_digits` lift below covers it internally, so
//!   the wall is on the work integer, not the guard.

#![allow(dead_code)]

use super::expm1_support as sup;
use crate::algos::exp::exp_generic as eg;
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;

/// The halving + doubling core: `expm1(s)` at working scale `w`, reducing by
/// `n` binary halvings and climbing back with `n` applications of
/// `E <- E*(E + 2)`.
///
/// Shared with [`super::expm1_reduced`] (which supplies an already
/// `k*ln 2`-reduced `s` and its own `n`), so the recurrence has ONE body.
///
/// `n` is capped at `bit_length(s) - 1` so the shift can never take the
/// argument to zero — a zero `u` would return 0 for a genuinely non-zero `s`.
/// The cap binds only for sub-resolution arguments, which the strict wrapper's
/// near-min pin owns anyway. The shift semantics are `exp_fixed`'s `s >> n`
/// (arithmetic for negative values); its one working unit of truncation is
/// what the callers' guard lift provisions against the `2^n` amplification.
pub(crate) fn expm1_doubling_core<S: BigInt>(s: S, w: u32, n: u32) -> S
where
    S::Scratch: ComputeLimbs,
{
    let n = n.min(eg::bit_length::<S>(s).saturating_sub(1));
    let u = s >> n;
    let mut e = eg::expm1_fixed::<S>(u, w);
    let two_p = eg::one::<S>(w) + eg::one::<S>(w);
    let mut i = 0;
    while i < n {
        e = eg::mul::<S>(e, e + two_p, w);
        i += 1;
    }
    e
}

/// `expm1(v)` for a working-scale value `v_w` at scale `w`, by halving +
/// doubling. `None` = cannot be produced in `S` at this `w` (the
/// `try_exp_fixed` contract: detect once, the policy wrapper applies the
/// overflow policy).
pub(crate) fn expm1_halving_fixed<S: BigInt>(v_w: S, w: u32) -> Option<S>
where
    S::Scratch: ComputeLimbs,
{
    if v_w == S::ZERO {
        return Some(S::ZERO);
    }
    match sup::regime::<S>(v_w, w) {
        sup::Regime::Overflow => return None,
        sup::Regime::MinusOne => return Some(sup::just_above_minus_one::<S>(w)),
        sup::Regime::Fits => {}
    }

    let c = sup::ceil_abs_int::<S>(v_w, w);
    let positive = v_w > S::ZERO;

    // Guard lift. Two independent factors, each converted to decimal digits:
    //   * the chain's `2^n` amplification of the shift truncation and of every
    //     level's half-unit rounding  -> ceil(n*log10 2) digits;
    //   * the result's own growth `e^v` (positive arguments only — the
    //     recurrence contracts for v <= 0) -> ceil(|v|*log10 e) digits.
    // `n` is estimated at `w` and used at `w_ext`; `halving_levels` grows like
    // sqrt(w), so the drift over the lift is under one level, covered by the
    // flat slack.
    let n_est = sup::band_levels(c) + sup::halving_levels(w);
    let n_digits = ((n_est as u64) * 30_103).div_ceil(100_000) as u32;
    let growth = if positive { sup::result_int_digits(c) } else { 0 };
    let extra = n_digits + growth + 4;

    let w_ext = w.checked_add(extra)?;
    // Peak: the last doubling forms `E_{n-1} * (E_{n-1} + 2P)` with
    // `|E_{n-1}| ~ e^{v/2} * P`, i.e. a product of `~ e^v * P^2`.
    let peak = 2 * sup::scale_bits(w_ext) + if positive { sup::result_bits(c) } else { 1 };
    if !sup::peak_fits::<S>(peak) {
        return None;
    }

    let v_ext = if extra == 0 {
        v_w
    } else {
        v_w * eg::pow10::<S>(extra)
    };
    let n = sup::band_levels(c) + sup::halving_levels(w_ext);
    let e = expm1_doubling_core::<S>(v_ext, w_ext, n);

    let r = if extra == 0 {
        e
    } else {
        eg::round_div_pow10::<S>(e, extra)
    };
    Some(r)
}
