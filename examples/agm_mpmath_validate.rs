//! AGM precision-lift validation harness.
//!
//! Emits, for each (tier, SCALE) target, a deterministic sequence of
//! `ln_strict_agm(x)` and `exp_strict_agm(y)` results as
//! tab-separated lines:
//!
//! ```text
//! <tier>\t<scale>\t<fn>\t<input_decimal_string>\t<output_decimal_string>
//! ```
//!
//! A Python driver (`scripts/agm_mpmath_validate.py`) parses each
//! line, computes the truth at 500-digit mpmath precision and reports
//! the absolute ULP delta at storage scale.
//!
//! The inputs are a 16-point fan covering the AGM-relevant range:
//! arguments near 1 (where the AGM seed `4/s` is large), then orders
//! of magnitude away from 1 (where the seed shrinks and the
//! `sqrt(a · b)` truncation error gets amplified).
//!
//! Run with:
//!
//! ```text
//! cargo run --release --example agm_mpmath_validate --features "wide x-wide xx-wide"
//! ```

#![allow(unused_imports)]

#[cfg(feature = "wide")]
use decimal_scaled::D307;
#[cfg(feature = "x-wide")]
use decimal_scaled::{D462, D616};
#[cfg(feature = "xx-wide")]
use decimal_scaled::{D924, D1232};

/// 16-point input fan for ln. Covers the dynamic range that
/// stresses the AGM seed `4/s` — from `s ≈ 1` (large seed) to
/// `s ≈ 1e6` (tiny seed, the regime the precision lift targets).
const LN_INPUTS: &[&str] = &[
    "1.000001",
    "1.5",
    "2.0",
    "2.71828182845904523536",
    "3.14159265358979323846",
    "10",
    "100",
    "1000",
    "1000000",
    "0.5",
    "0.1",
    "0.01",
    "0.000001",
    "7.389056098930650227230427460",
    "0.367879441171442321595523770",
    "1.4142135623730950488016887242",
];

/// 16-point input fan for exp. Covers the Newton-on-AGM range:
/// near zero (small Taylor seed), moderate, and beyond the Newton
/// convergence radius (so the range reduction `v = k·ln 2 + s`
/// runs).
const EXP_INPUTS: &[&str] = &[
    "0.000001",
    "0.1",
    "0.5",
    "1.0",
    "1.5",
    "2.302585092994045684017991454684",
    "3.0",
    "5.0",
    "10.0",
    "-0.5",
    "-1.0",
    "-5.0",
    "0.693147180559945309417232121458",
    "0.7071067811865475244008443621",
    "1.4142135623730950488016887242",
    "0.25",
];

macro_rules! emit_tier {
    ($T:ty, $tier:literal, $scale:literal) => {{
        use core::str::FromStr;
        for s in LN_INPUTS {
            let x = <$T>::from_str(s).expect("input parse");
            let r = x.ln_strict_agm();
            println!("{}\t{}\tln_agm\t{}\t{}", $tier, $scale, s, r);
        }
        for s in EXP_INPUTS {
            let x = <$T>::from_str(s).expect("input parse");
            let r = x.exp_strict_agm();
            println!("{}\t{}\texp_agm\t{}\t{}", $tier, $scale, s, r);
        }
    }};
}

fn main() {
    #[cfg(feature = "wide")]
    {
        emit_tier!(D307<150>, "D307", 150);
    }
    #[cfg(feature = "x-wide")]
    {
        emit_tier!(D462<230>, "D462", 230);
        emit_tier!(D616<308>, "D616", 308);
    }
    #[cfg(feature = "xx-wide")]
    {
        emit_tier!(D924<460>, "D924", 460);
        emit_tier!(D1232<615>, "D1232", 615);
    }
}
