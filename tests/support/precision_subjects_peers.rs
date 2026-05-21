//! Peer-library [`PrecisionSubject`] adapters: dashu-float, rust_decimal,
//! fastnum, decimal-rs, bigdecimal.
//!
//! Refactored from the per-library drivers in
//! `benches/lib_cmp_precision.rs` into the harness trait. Each subject
//! parses the canonical decimal value string, runs the method at its
//! native precision, and reports its result as a decimal string plus the
//! rounding mode it actually applies. Where a library lacks a method or
//! cannot represent the (width, scale), it returns
//! [`SubjectOutput::NotApplicable`].
//!
//! These pull in the comparator crates, so this file is included ONLY by
//! the bench runner (`benches/lib_cmp_precision_harness.rs`), never by
//! `tests/`. It references the harness surface via `super::harness::*`.

use super::harness::{Input, Method, PrecisionSubject, SubjectOutput, Width};
use decimal_scaled::RoundingMode;

// The widest scale a fixed-precision peer can carry before we declare it
// unable to represent the (width, scale). Beyond this it is `n/a`.
fn peer_max_scale(name: &str) -> u32 {
    match name {
        "rust_decimal" => 28,
        "fastnum" => 34,
        "decimal-rs" => 38,
        "bigdecimal" => u32::MAX, // arbitrary precision (but no transcendentals)
        "dashu-float" => u32::MAX, // arbitrary precision
        // g_math is binary fixed-point. Its canonical evaluator infers a
        // Q-format tier from the input literal's decimal places and, for
        // the small magnitudes the roster uses, materialises results at
        // roughly Q64.64 — about 19 significant fractional digits — and
        // its `to_decimal_string` emits no more than that no matter how
        // many digits are requested. So 19 is the deepest scale at which
        // it produces a value of the target precision; past it the value
        // simply cannot be represented and it is `n/a` (matching the old
        // deep-150 table, where every fixed-precision peer scored "—").
        "g_math" => 19,
        _ => 0,
    }
}

// ── dashu-float ─────────────────────────────────────────────────────

/// dashu-float `DBig` — arbitrary-precision base-10, HalfAway rounding.
/// Exposes exp / ln / sqrt only.
pub struct DashuSubject;

impl PrecisionSubject for DashuSubject {
    fn name(&self) -> &str {
        "dashu-float"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfAwayFromZero
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        use dashu_float::DBig;
        use dashu_float::ops::SquareRoot;

        let Ok(x) = input.value_string().parse::<DBig>() else {
            return SubjectOutput::NotApplicable;
        };
        // Carry the full tier scale plus a guard so the result is not
        // precision-clamped; we want dashu's own accuracy.
        let prec = (scale as usize) + 40;
        let x = x.with_precision(prec).value();
        let y = match method {
            Method::Exp => x.exp(),
            Method::Ln => {
                if x <= DBig::ZERO {
                    return SubjectOutput::NotApplicable;
                }
                x.ln()
            }
            Method::Sqrt => {
                if x < DBig::ZERO {
                    return SubjectOutput::NotApplicable;
                }
                x.sqrt()
            }
            _ => return SubjectOutput::NotApplicable,
        };
        SubjectOutput::Computed {
            value: y.to_string(),
            rounding: RoundingMode::HalfAwayFromZero,
        }
    }
}

// ── rust_decimal ────────────────────────────────────────────────────

/// rust_decimal — 96-bit, ≤ 28 significant digits. ln/exp/sqrt/sin/cos/
/// tan via `MathematicalOps`; no atan, no cbrt. Banker's (HalfEven).
pub struct RustDecimalSubject;

impl PrecisionSubject for RustDecimalSubject {
    fn name(&self) -> &str {
        "rust_decimal"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfToEven
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        use rust_decimal::Decimal as RustDecimal;
        use rust_decimal::MathematicalOps;

        if scale > peer_max_scale("rust_decimal") {
            return SubjectOutput::NotApplicable;
        }
        let Ok(x) = input.value_string().parse::<RustDecimal>() else {
            return SubjectOutput::NotApplicable;
        };
        let y = match method {
            Method::Exp => x.checked_exp(),
            Method::Ln => x.checked_ln(),
            Method::Sin => Some(x.sin()),
            Method::Cos => Some(x.cos()),
            Method::Tan => Some(x.tan()),
            Method::Sqrt => x.sqrt(),
            _ => return SubjectOutput::NotApplicable,
        };
        match y {
            Some(y) => SubjectOutput::Computed {
                value: y.to_string(),
                rounding: RoundingMode::HalfToEven,
            },
            None => SubjectOutput::NotApplicable,
        }
    }
}

// ── fastnum ─────────────────────────────────────────────────────────

/// fastnum `D128` — 128-bit decimal, ~34 significant digits. Default
/// context rounding is HalfUp (== HalfAwayFromZero for the non-negative
/// magnitudes here). Covers the full transcendental surface.
pub struct FastnumSubject;

impl PrecisionSubject for FastnumSubject {
    fn name(&self) -> &str {
        "fastnum"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfAwayFromZero
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        use fastnum::{D128, decimal::Context};

        if scale > peer_max_scale("fastnum") {
            return SubjectOutput::NotApplicable;
        }
        let parse = |s: &str| D128::from_str(s, Context::default()).ok();
        let Some(x) = parse(&input.value_string()) else {
            return SubjectOutput::NotApplicable;
        };
        let y = match method {
            Method::Sqrt => x.sqrt(),
            Method::Cbrt => x.cbrt(),
            Method::Exp => x.exp(),
            Method::Ln => x.ln(),
            Method::Log2 => x.log2(),
            Method::Log10 => x.log10(),
            Method::Sin => x.sin(),
            Method::Cos => x.cos(),
            Method::Tan => x.tan(),
            Method::Atan => x.atan(),
            Method::Asin => x.asin(),
            Method::Acos => x.acos(),
            Method::Sinh => x.sinh(),
            Method::Cosh => x.cosh(),
            Method::Tanh => x.tanh(),
            Method::Asinh => x.asinh(),
            Method::Acosh => x.acosh(),
            Method::Atanh => x.atanh(),
            Method::Pow => {
                let Some(e) = input.value2_string().and_then(|s| parse(&s)) else {
                    return SubjectOutput::NotApplicable;
                };
                x.pow(e)
            }
            _ => return SubjectOutput::NotApplicable,
        };
        if y.is_nan() || y.is_infinite() {
            return SubjectOutput::NotApplicable;
        }
        SubjectOutput::Computed {
            value: y.to_string(),
            rounding: RoundingMode::HalfAwayFromZero,
        }
    }
}

// ── decimal-rs ──────────────────────────────────────────────────────

/// decimal-rs — 128-bit decimal. Exposes ln/exp/sqrt only.
pub struct DecimalRsSubject;

impl PrecisionSubject for DecimalRsSubject {
    fn name(&self) -> &str {
        "decimal-rs"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfToEven
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        use decimal_rs::Decimal as DecimalRs;

        if scale > peer_max_scale("decimal-rs") {
            return SubjectOutput::NotApplicable;
        }
        let Ok(x) = input.value_string().parse::<DecimalRs>() else {
            return SubjectOutput::NotApplicable;
        };
        let y = match method {
            Method::Exp => x.exp(),
            Method::Ln => x.ln(),
            Method::Sqrt => x.sqrt(),
            _ => return SubjectOutput::NotApplicable,
        };
        match y {
            Some(y) => SubjectOutput::Computed {
                value: y.to_string(),
                rounding: RoundingMode::HalfToEven,
            },
            None => SubjectOutput::NotApplicable,
        }
    }
}

// ── bigdecimal ──────────────────────────────────────────────────────

/// bigdecimal — arbitrary-precision base-10, but exposes no
/// transcendentals on its stable surface. Always `NotApplicable` for the
/// transcendental roster; carried so the table shows it honestly as a
/// peer that cannot be scored.
pub struct BigDecimalSubject;

impl PrecisionSubject for BigDecimalSubject {
    fn name(&self) -> &str {
        "bigdecimal"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfToEven
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        // bigdecimal has sqrt/cbrt (with a context) but no exp/ln/trig.
        use bigdecimal::BigDecimal;
        let _ = scale;
        let Ok(x) = input.value_string().parse::<BigDecimal>() else {
            return SubjectOutput::NotApplicable;
        };
        let y = match method {
            Method::Sqrt => x.sqrt(),
            Method::Cbrt => Some(x.cbrt()),
            _ => return SubjectOutput::NotApplicable,
        };
        match y {
            Some(y) => SubjectOutput::Computed {
                value: y.to_string(),
                rounding: RoundingMode::HalfToEven,
            },
            None => SubjectOutput::NotApplicable,
        }
    }
}

// ── g_math ──────────────────────────────────────────────────────────

/// g_math — a binary fixed-point expression DSL that advertises "0 ULP
/// transcendentals". It is NOT a decimal library: the canonical evaluator
/// infers a Q-format tier (Q64.64 … Q256.256) from the input literal and
/// computes in binary fixed-point, then renders a decimal string. For the
/// small magnitudes the roster uses it materialises at roughly Q64.64
/// (~19 significant fractional digits) and `to_decimal_string` emits no
/// more than that regardless of the requested digit count.
///
/// Modelled faithfully: at D38<19> it is within range and we score its
/// emitted value rounded to the storage grid (it lands many LSB off on
/// most transcendentals — the empirical refutation of the "0 ULP"
/// claim). At scales beyond 19 (D76<35>, D307<150>) it genuinely cannot
/// represent a value of the target precision, so it returns
/// [`SubjectOutput::NotApplicable`] rather than being scored against a
/// scale it never reaches.
///
/// Its native binary rounding is nearest-even, reported as
/// [`RoundingMode::HalfToEven`].
pub struct GMathSubject;

impl PrecisionSubject for GMathSubject {
    fn name(&self) -> &str {
        "g_math"
    }
    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfToEven
    }
    fn eval(
        &self,
        method: Method,
        _width: Width,
        scale: u32,
        input: &Input,
        _mode: RoundingMode,
    ) -> SubjectOutput {
        use g_math::canonical::{evaluate, gmath_parse};

        if scale > peer_max_scale("g_math") {
            return SubjectOutput::NotApplicable;
        }

        let Ok(x) = gmath_parse(&input.value_string()) else {
            return SubjectOutput::NotApplicable;
        };
        // Build the method expression. g_math exposes the full single- and
        // two-argument transcendental surface EXCEPT base-2/base-10 logs,
        // an arbitrary-base log, and cbrt — those are `n/a`.
        let expr = match method {
            Method::Sqrt => x.sqrt(),
            Method::Exp => x.exp(),
            Method::Ln => x.ln(),
            Method::Sin => x.sin(),
            Method::Cos => x.cos(),
            Method::Tan => x.tan(),
            Method::Atan => x.atan(),
            Method::Asin => x.asin(),
            Method::Acos => x.acos(),
            Method::Sinh => x.sinh(),
            Method::Cosh => x.cosh(),
            Method::Tanh => x.tanh(),
            Method::Asinh => x.asinh(),
            Method::Acosh => x.acosh(),
            Method::Atanh => x.atanh(),
            Method::Pow => {
                let Some(v2) = input.value2_string() else {
                    return SubjectOutput::NotApplicable;
                };
                let Ok(e) = gmath_parse(&v2) else {
                    return SubjectOutput::NotApplicable;
                };
                x.pow(e)
            }
            Method::Atan2 => {
                let Some(v2) = input.value2_string() else {
                    return SubjectOutput::NotApplicable;
                };
                let Ok(x2) = gmath_parse(&v2) else {
                    return SubjectOutput::NotApplicable;
                };
                x.atan2(x2)
            }
            // cbrt, log2, log10, arbitrary-base log: not exposed.
            _ => return SubjectOutput::NotApplicable,
        };

        let Ok(value) = evaluate(&expr) else {
            return SubjectOutput::NotApplicable;
        };
        // Ask for a generous digit count (well past the storage scale);
        // g_math emits only what its working tier resolved, and the
        // harness rounds that to the storage grid.
        let s = value.to_decimal_string((scale as usize) + 12);
        let low = s.to_ascii_lowercase();
        if low.contains("nan") || low.contains("inf") || low.contains("error") {
            return SubjectOutput::NotApplicable;
        }
        SubjectOutput::Computed {
            value: s,
            rounding: RoundingMode::HalfToEven,
        }
    }
}
