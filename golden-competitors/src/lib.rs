//! Competitor decimal-library subjects for the `decimal-scaled-golden` harness.
//!
//! Each wraps a third-party decimal library as a `DecimalSubject`, declaring only
//! the functions that library actually provides (the harness skips the rest) and
//! its representability envelope. Comparison runs pair these with the decimal-scaled
//! subjects (via `golden-ds`) on the same golden set, so every competitor is graded
//! side-by-side against decimal-scaled. Competitor crates are dependencies of THIS
//! crate only — never the core library or the agnostic harness.

use std::collections::BTreeMap;
use std::str::FromStr;

use decimal_scaled_golden::{
    Capabilities, Computed, DecimalSubject, FnSupport, Function, Limits, NonReal, Overflow, Radix,
    RoundingMode,
};

use rust_decimal::{Decimal, MathematicalOps};

/// [rust_decimal](https://docs.rs/rust_decimal): a 96-bit fixed-point decimal with
/// up to ~28 significant digits, rounding half-away-from-zero. Transcendentals come
/// from its `maths` feature (`MathematicalOps`); it has no cbrt / exp2 / log2 /
/// inverse-trig / hyperbolics, so those are simply not declared.
pub struct RustDecimal;

impl RustDecimal {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Exp, Function::Ln, Function::Log10, Function::Powf,
        Function::Sin, Function::Cos, Function::Tan,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
}

impl DecimalSubject for RustDecimal {
    type Value = Decimal;

    fn name(&self) -> String {
        "rust_decimal".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            // rust_decimal rounds half-away-from-zero; an out-of-range result has
            // no representation (the `checked_*` ops yield `None`) -> `Absent`.
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Absent },
            );
        }
        Capabilities {
            name: "rust_decimal".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> Decimal {
        Decimal::from_str(s)
            .or_else(|_| Decimal::from_str_exact(s))
            .unwrap_or_else(|e| panic!("rust_decimal could not parse {s:?}: {e}"))
    }

    fn value_to_string(&self, v: &Decimal) -> String {
        v.normalize().to_string()
    }

    fn limits(&self, _value: &str) -> Limits {
        Limits {
            min_value: Some(Decimal::MIN.to_string()),
            max_value: Some(Decimal::MAX.to_string()),
            max_precision: 28,
        }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[Decimal]) -> Computed<Decimal> {
        move |inputs| {
            let x = inputs[0];
            let from_opt = |o: Option<Decimal>| match o {
                Some(v) => Computed::Value(v),
                None => Computed::Absent,
            };
            match func {
                Function::Sqrt => match x.sqrt() {
                    Some(v) => Computed::Value(v),
                    None => Computed::NonReal(NonReal::Imaginary),
                },
                Function::Exp => from_opt(x.checked_exp()),
                Function::Ln => {
                    if x <= Decimal::ZERO {
                        Computed::NonReal(NonReal::NegativeInfinity)
                    } else {
                        from_opt(x.checked_ln())
                    }
                }
                Function::Log10 => {
                    if x <= Decimal::ZERO {
                        Computed::NonReal(NonReal::NegativeInfinity)
                    } else {
                        from_opt(x.checked_log10())
                    }
                }
                Function::Powf => from_opt(x.checked_powd(inputs[1])),
                Function::Sin => Computed::Value(x.sin()),
                Function::Cos => Computed::Value(x.cos()),
                Function::Tan => from_opt(x.checked_tan()),
                Function::Add => from_opt(x.checked_add(inputs[1])),
                Function::Sub => from_opt(x.checked_sub(inputs[1])),
                Function::Mul => from_opt(x.checked_mul(inputs[1])),
                Function::Div => from_opt(x.checked_div(inputs[1])),
                Function::Rem => from_opt(x.checked_rem(inputs[1])),
                other => Computed::Error(format!("rust_decimal: unsupported {}", other.name())),
            }
        }
    }
}
