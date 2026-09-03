// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `powf` as `exp(y · ln x)` with the exact-power pins in front of it.
//!
//! The production wide-tier `powf` kernel. It is `ExpWithLn` with three
//! things the naive composition does not have, and each of them exists
//! because the composition alone rounds wrong somewhere:
//!
//! 1. **Exact integer-power pins** — an integer exponent is answered by
//!    integer arithmetic (`algos::pow::powi_exact`), never by
//!    `exp(y·ln x)`, so `3^-2` and a sub-resolution `base^-k` round under
//!    the requested directed mode instead of inheriting the
//!    composition's error.
//! 2. **An algebraic `x^0.5 ≡ √x` pin** — the exp/ln chain loses a
//!    sub-ULP at a perfect-square base.
//! 3. **A result-sized working lift** (`k_lift`), gated by an analytic
//!    storage-overflow check taken BEFORE the lift is sized.
//!
//! That is what distinguishes it from [`super::pow_schoolbook`], the
//! naive `exp(y·ln x)` reference: they are two algorithms, not two
//! spellings of one.
//!
//! ## Layering
//!
//! An **algorithm function** (`docs/ARCHITECTURE.md` → "Layering
//! direction"): it computes through the [`WideTrigCore`] trait surface
//! and the width-generic `exp_generic` / `powi_exact` /
//! `powf_overflow_gate` leaves, and never calls a method on a decimal
//! type. The `x^0.5` pin is taken as a function argument rather than
//! named here — the same parameterisation
//! [`crate::algos::trig::sincos_joint`] uses for its escapes — so which
//! `sqrt` engine answers it stays a ROUTING decision owned by
//! `policy::pow`.

use crate::algos::exp::exp_generic as eg;
use crate::algos::pow::{powf_overflow_gate, powi_exact};
use crate::algos::support::wide_trig_core::{
    round_to_storage_directed_g, round_to_storage_with_g, WideTrigCore,
};
use crate::int::types::compute_limbs::ComputeLimbs;
use crate::int::types::traits::BigInt;
use crate::support::rounding::RoundingMode;

/// `base^exponent`, correctly rounded to storage under `mode`.
///
/// A non-positive base returns zero. A domain-valid result that does not
/// fit storage panics uniformly through the composition below.
///
/// `sqrt_escape` answers the exact-half exponent: `policy::pow` supplies
/// the cell's own `sqrt` verdict, so the pin lands on whichever engine
/// that cell routes `sqrt` to and this kernel never pins one.
///
/// Two-core: the composition runs on the wide `C::Wagm` work int, because
/// the exp argument `y·ln x` can exceed a narrowed `C::W`.
#[inline]
#[must_use]
pub(crate) fn powf_pinned_exp_with_ln<C: WideTrigCore, const SCALE: u32>(
    raw: C::Storage,
    exponent_raw: C::Storage,
    mode: RoundingMode,
    sqrt_escape: fn(C::Storage, RoundingMode) -> C::Storage,
) -> C::Storage
where
    <C::Wagm as BigInt>::Scratch: ComputeLimbs,
{
    let storage_zero = C::storage_zero();
    if raw <= storage_zero {
        return storage_zero;
    }
    // Exact integer-power pin. Uses the caller's `mode`: the reciprocal
    // of a non-terminating power (e.g. `3^-2`) and a sub-resolution
    // `base^-k` must round under the requested directed mode (Ceiling of
    // a sub-resolution `base^-k` rounds up to 1, not down to 0). The pin
    // divides the INTEGER `base^|n|`, so a terminating reciprocal is
    // exact even when the scaled `base^|n|·10^SCALE` overflows storage —
    // the case a `checked_pow` fast path would defer to the to-nearest
    // composition, mis-rounding Floor / Trunc by 1 LSB. `None`
    // (fractional base/exponent, or a positive power out of range) defers
    // to the composition below, which panics uniformly on a genuinely
    // out-of-range result.
    if let Some(pinned) = powi_exact::powi_exact_pin::<C::Storage, SCALE>(
        raw,
        exponent_raw,
        C::storage_max(),
        mode,
    ) {
        return pinned;
    }
    // Fractional-base integer-exponent fast path, under the caller's
    // `mode`.
    if let Some(integer_exponent) =
        powi_exact::exp_as_small_int_raw::<C::Storage, SCALE>(exponent_raw)
    {
        if integer_exponent == 0 {
            return C::storage_one(SCALE);
        }
        if let Some(pinned) = powi_exact::powi_terminating_pin::<C::Storage, SCALE>(
            raw,
            integer_exponent,
            C::storage_max(),
            mode,
        ) {
            return pinned;
        }
    }
    // x^0.5 ≡ √x. The exp(0.5·ln x) chain loses a sub-ULP at a
    // perfect-square base (e.g. 4^0.5), rounding 1 LSB short under the
    // directed modes; the sqrt kernel pins the exact algebraic root and
    // is correctly rounded for every input, so route the exact-half
    // exponent through it.
    {
        let two = <C::Storage as BigInt>::from_i128(2);
        let multiplier = C::storage_one(SCALE);
        if exponent_raw == multiplier / two {
            return sqrt_escape(raw, mode);
        }
    }
    // Large-result lift. `x^y = exp(y·ln x)` carries `~|y·ln x|·log10(e)`
    // integer digits; size the working lift from a base-guard probe of
    // the exp argument so the `exp_fixed` relative error stays
    // sub-storage-ULP after narrowing (same budget sinh/cosh use).
    let k_lift = {
        let base_working_scale = SCALE + C::GUARD;
        let probe_ln_x = C::ln_fixed_routed_agm::<SCALE>(
            C::to_work_scaled_agm(raw, C::GUARD),
            base_working_scale,
        );
        let probe_exp_arg = eg::mul::<C::Wagm>(
            C::to_work_scaled_agm(exponent_raw, C::GUARD),
            probe_ln_x,
            base_working_scale,
        );
        // Analytic storage-overflow gate, BEFORE the result-sized lift
        // below: a deep-overflow argument (`e^arg` provably past storage)
        // would size `k_lift` in the hundreds and push the working scale
        // past the work integer's safe ceiling, where the lifted `ln`'s
        // table product silently WRAPS to a near-zero garbage `ln x` that
        // defuses every downstream overflow check (the `1.5^1000.5` D76
        // deep-band defect). Panic contractually here instead — the gate
        // is a provable SUFFICIENT bound, so no representable cell fires
        // it (see `algos::pow::powf_overflow_gate`).
        if powf_overflow_gate::powf_overflow_gate_g::<C::Wagm>(
            probe_exp_arg,
            base_working_scale,
            <C::Storage as BigInt>::BITS,
            SCALE,
        ) {
            crate::support::diagnostics::overflow_panic_with_scale("powf_strict", SCALE);
        }
        // `probe_exp_arg` is the exp argument at the base working scale;
        // narrow it to scale `SCALE` to feed the `e^|·|` digit sizer
        // (squaring-safe capped).
        let arg_at_scale = round_to_storage_with_g::<C::Storage, C::Wagm>(
            probe_exp_arg,
            base_working_scale,
            SCALE,
            RoundingMode::Trunc,
            C::storage_max(),
            C::storage_min(),
        );
        // `e^arg` grows integer digits ONLY for a POSITIVE argument; for
        // a negative argument `e^arg ∈ (0, 1)` has zero integer digits
        // and needs NO lift. Sizing a lift there would inflate the
        // working scale `w = SCALE + GUARD + k_lift` until the
        // non-widening low-product `mul(y, ln_x, w)` overflows the `Wagm`
        // work integer (its `y·ln_x` exceeds `Wagm::BITS`) and WRAPS the
        // exp argument to garbage — the deep-underflow misround
        // (`powf("2","-200")` at D57/D76 mid-scales returned `e^-0.21 ≈
        // 0.808` instead of the sub-resolution 0). The sign gate mirrors
        // `exp2_result_int_digits`'s negative-argument early return and
        // the Tang/Series `extra = 0 for k ≤ 0` reassembly asymmetry.
        if arg_at_scale < storage_zero {
            0
        } else {
            C::exp_result_int_digits_agm(C::to_work_scaled_agm(arg_at_scale, 0), SCALE)
        }
    };
    let base_guard_digits = C::GUARD + k_lift;
    round_to_storage_directed_g::<C::Storage, C::Wagm>(
        base_guard_digits,
        SCALE,
        mode,
        C::storage_max(),
        C::storage_min(),
        |guard_digits| {
            let working_scale = SCALE + guard_digits;
            let ln_x = C::ln_fixed_routed_agm::<SCALE>(
                C::to_work_scaled_agm(raw, guard_digits),
                working_scale,
            );
            let exponent_w = C::to_work_scaled_agm(exponent_raw, guard_digits);
            C::exp_fixed_routed_agm::<SCALE>(
                eg::mul::<C::Wagm>(exponent_w, ln_x, working_scale),
                working_scale,
            )
        },
    )
}
