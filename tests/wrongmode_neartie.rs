// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Near-tie WrongMode rounding (Table-Maker's-Dilemma) regression.
//!
//! At a near-half-ULP tie the deciding series term sits below the base
//! `SCALE + GUARD` working scale, so the kernel saw an exact half and
//! round-to-nearest (HalfToEven) landed on the wrong side. The shared
//! near-tie escalation (`round_to_storage_directed_impl_g`'s nearest branch)
//! now confirms the round against a wider guard, so HalfToEven agrees with
//! the directed mode the true value falls toward:
//!
//! - `exp(1e-14)` @ D57<28> = 1 + 1e-14 + ½·10⁻²⁸ + (1/6)·10⁻⁴² : the cubic
//!   tail tips it just ABOVE the half-ULP boundary → rounds UP (== Ceiling).
//! - `cosh(1e-28)` @ D57<56> = 1 + ½·10⁻⁵⁶ + (1/24)·10⁻¹¹² : the quartic tail
//!   tips it just ABOVE the half → rounds UP (== Ceiling).
//! - `ln(0.999)` @ D38<6> = −0.0010005003… : magnitude just ABOVE 0.0010005
//!   (the `ln1p_band` even-S edge bug used to return the linear term and
//!   misround) → rounds AWAY from zero (== Floor).

#![cfg(all(feature = "wide", not(feature = "fast")))]

use decimal_scaled::{RoundingMode, D38, D57};

#[test]
fn neartie_wrongmode_rounds_to_the_correct_side() {
    let (he, fl, ce) = (RoundingMode::HalfToEven, RoundingMode::Floor, RoundingMode::Ceiling);

    // exp(1e-14): just above a half-ULP tie ⇒ HalfToEven rounds up (== Ceiling).
    let x: D57<28> = "0.00000000000001".parse().unwrap();
    assert_eq!(x.exp_strict_with(he), x.exp_strict_with(ce), "exp(1e-14) HToE must round up");
    assert_ne!(x.exp_strict_with(he), x.exp_strict_with(fl), "exp(1e-14) HToE must not be Floor");

    // cosh(1e-28): just above a half-ULP tie ⇒ HalfToEven rounds up (== Ceiling).
    let x: D57<56> = "0.0000000000000000000000000001".parse().unwrap();
    assert_eq!(x.cosh_strict_with(he), x.cosh_strict_with(ce), "cosh(1e-28) HToE must round up");
    assert_ne!(x.cosh_strict_with(he), x.cosh_strict_with(fl), "cosh(1e-28) HToE must not be Floor");

    // ln(0.999): magnitude just above the half ⇒ HalfToEven rounds away from
    // zero (more negative, == Floor), NOT the linear-term shortcut (== Ceiling).
    let x: D38<6> = "0.999".parse().unwrap();
    assert_eq!(x.ln_strict_with(he), x.ln_strict_with(fl), "ln(0.999) HToE must round away from zero");
    assert_ne!(x.ln_strict_with(he), x.ln_strict_with(ce), "ln(0.999) HToE must not be Ceiling");
    assert_eq!(x.ln_strict_with(he).to_string(), "-0.001001");
}
