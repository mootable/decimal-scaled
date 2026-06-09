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

/// Expand a possibly-scientific decimal string (`"1.23E-10"`, `"4.5e+6"`) into a
/// plain `[-]int[.frac]` decimal — the only shape the harness's string-decimal
/// grader parses. A string with no exponent passes through unchanged, so it is
/// safe to apply to every library whose `Display` *might* emit scientific form
/// (fastnum and dashu-float do, for small/large magnitudes). Trailing/leading
/// zeros are irrelevant to the grader (it truncates at the graded depth).
fn expand_scientific(s: &str) -> String {
    let (neg, body) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (mant, exp) = match body.split_once(['e', 'E']) {
        Some((m, e)) => (m, e.trim_start_matches('+').parse::<i64>().unwrap_or(0)),
        None => (body, 0),
    };
    let (int_part, frac_part) = mant.split_once('.').unwrap_or((mant, ""));
    let digits = format!("{int_part}{frac_part}");
    // Where the decimal point lands within `digits` (count of integer digits).
    let point = int_part.len() as i64 + exp;
    let plain = if point <= 0 {
        format!("0.{}{digits}", "0".repeat((-point) as usize))
    } else if point as usize >= digits.len() {
        format!("{digits}{}", "0".repeat(point as usize - digits.len()))
    } else {
        let (i, f) = digits.split_at(point as usize);
        format!("{i}.{f}")
    };
    if neg { format!("-{plain}") } else { plain }
}

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

/// `f64` — the platform's native IEEE-754 binary double (~15–17 significant decimal
/// digits), via std's inherent float methods. It provides EVERY function in the set.
/// Radix is `Binary` (it rounds on the `2^-k` grid, not the decimal one — a
/// verdict-neutral annotation): graded for decimal compliance it mis-rounds often,
/// which is the point. NaN -> `NonReal::NaN`; ±inf (overflow) -> the signed infinities.
pub struct F64;

impl F64 {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2,
        Function::Log10, Function::Exp2, Function::Sin, Function::Cos, Function::Tan,
        Function::Atan, Function::Asin, Function::Acos, Function::Sinh, Function::Cosh,
        Function::Tanh, Function::Asinh, Function::Acosh, Function::Atanh, Function::Log,
        Function::Atan2, Function::Powf, Function::Hypot, Function::Add, Function::Sub,
        Function::Mul, Function::Div, Function::Rem,
    ];
}

fn classify_f64(v: f64) -> Computed<f64> {
    if v.is_nan() {
        Computed::NonReal(NonReal::NaN)
    } else if v.is_infinite() {
        Computed::NonReal(if v > 0.0 { NonReal::PositiveInfinity } else { NonReal::NegativeInfinity })
    } else {
        Computed::Value(v)
    }
}

impl DecimalSubject for F64 {
    type Value = f64;

    fn name(&self) -> String {
        "f64".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Infinity },
            );
        }
        Capabilities {
            name: "f64".into(),
            radix: Radix::Binary,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> f64 {
        s.parse::<f64>().unwrap_or_else(|e| panic!("f64 could not parse {s:?}: {e}"))
    }

    fn value_to_string(&self, v: &f64) -> String {
        // The shortest decimal that round-trips to this f64; graded to f64's depth.
        format!("{v}")
    }

    fn limits(&self, _value: &str) -> Limits {
        Limits {
            min_value: Some(format!("{}", f64::MIN)),
            max_value: Some(format!("{}", f64::MAX)),
            max_precision: 15,
        }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[f64]) -> Computed<f64> {
        move |inputs| {
            let x = inputs[0];
            classify_f64(match func {
                Function::Sqrt => x.sqrt(),
                Function::Cbrt => x.cbrt(),
                Function::Exp => x.exp(),
                Function::Ln => x.ln(),
                Function::Log2 => x.log2(),
                Function::Log10 => x.log10(),
                Function::Exp2 => x.exp2(),
                Function::Sin => x.sin(),
                Function::Cos => x.cos(),
                Function::Tan => x.tan(),
                Function::Atan => x.atan(),
                Function::Asin => x.asin(),
                Function::Acos => x.acos(),
                Function::Sinh => x.sinh(),
                Function::Cosh => x.cosh(),
                Function::Tanh => x.tanh(),
                Function::Asinh => x.asinh(),
                Function::Acosh => x.acosh(),
                Function::Atanh => x.atanh(),
                Function::Log => x.log(inputs[1]),
                Function::Atan2 => x.atan2(inputs[1]),
                Function::Powf => x.powf(inputs[1]),
                Function::Hypot => x.hypot(inputs[1]),
                Function::Add => x + inputs[1],
                Function::Sub => x - inputs[1],
                Function::Mul => x * inputs[1],
                Function::Div => x / inputs[1],
                Function::Rem => x % inputs[1],
            })
        }
    }
}

// ---------------------------------------------------------------------------
// bigdecimal — arbitrary-precision decimal.
// ---------------------------------------------------------------------------

use bigdecimal::BigDecimal;

/// [bigdecimal](https://docs.rs/bigdecimal): an arbitrary-precision decimal with no
/// finite envelope. Being unbounded, computing `exp` (or any exp-family op) of a
/// large golden input would produce an astronomically large number and never
/// terminate, so `exp` is deliberately NOT declared — only the slow-growth `sqrt`
/// and `cbrt` plus the five arithmetic ops are (it has no `ln` / `powf` / trig).
/// sqrt/cbrt run at the crate's default `Context` (~100 significant digits,
/// half-to-even), well above the 30 fractional places results are graded to. Being
/// unbounded it never overflows: the only non-value outcome is `sqrt` of a negative
/// (-> imaginary).
pub struct BigDecimalSubject;

impl BigDecimalSubject {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Cbrt,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Grading depth (fractional places) — well under the ~100-digit compute context.
    const PRECISION: u32 = 30;
}

impl DecimalSubject for BigDecimalSubject {
    type Value = BigDecimal;

    fn name(&self) -> String {
        "bigdecimal".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Absent },
            );
        }
        Capabilities {
            name: "bigdecimal".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> BigDecimal {
        BigDecimal::from_str(s).unwrap_or_else(|e| panic!("bigdecimal could not parse {s:?}: {e}"))
    }

    fn value_to_string(&self, v: &BigDecimal) -> String {
        v.normalized().to_plain_string()
    }

    fn limits(&self, _value: &str) -> Limits {
        Limits { min_value: None, max_value: None, max_precision: Self::PRECISION }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[BigDecimal]) -> Computed<BigDecimal> {
        move |inputs| {
            let x = &inputs[0];
            match func {
                Function::Sqrt => match x.sqrt() {
                    Some(v) => Computed::Value(v),
                    None => Computed::NonReal(NonReal::Imaginary),
                },
                Function::Cbrt => Computed::Value(x.cbrt()),
                Function::Add => Computed::Value(x + &inputs[1]),
                Function::Sub => Computed::Value(x - &inputs[1]),
                Function::Mul => Computed::Value(x * &inputs[1]),
                Function::Div => Computed::Value(x / &inputs[1]),
                Function::Rem => Computed::Value(x % &inputs[1]),
                other => Computed::Error(format!("bigdecimal: unsupported {}", other.name())),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// dashu-float — arbitrary-precision floating decimal (`FBig` / `DBig`).
// ---------------------------------------------------------------------------

use dashu_float::DBig;

/// [dashu-float](https://docs.rs/dashu-float): an arbitrary-precision floating
/// decimal (`DBig` = `FBig<HalfAway, 10>`, rounding half-away-from-zero). Being
/// unbounded, `exp` / `powf` of a large golden input would grow without bound and
/// never terminate, so those exp-family ops are deliberately NOT declared — only the
/// slow-growth `ln` plus arithmetic are (it has no `sqrt` / `cbrt` / trig). Inputs
/// are lifted to a 34-significant-digit working precision so `ln` computes to that
/// depth; results are graded to 30 fractional places. Unbounded, so the envelope is
/// open; `Display` can emit scientific form, expanded for the grader.
pub struct DashuFloat;

impl DashuFloat {
    const FUNCS: &'static [Function] = &[
        Function::Ln,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Significant-digit precision every operation is lifted to before computing.
    const WORKING_PRECISION: usize = 34;
    /// Grading depth (fractional places) — well under the working precision.
    const PRECISION: u32 = 30;
}

impl DecimalSubject for DashuFloat {
    type Value = DBig;

    fn name(&self) -> String {
        "dashu-float".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Absent },
            );
        }
        Capabilities {
            name: "dashu-float".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> DBig {
        let v: DBig =
            s.parse().unwrap_or_else(|e| panic!("dashu-float could not parse {s:?}: {e:?}"));
        v.with_precision(Self::WORKING_PRECISION).value()
    }

    fn value_to_string(&self, v: &DBig) -> String {
        expand_scientific(&v.to_string())
    }

    fn limits(&self, _value: &str) -> Limits {
        Limits { min_value: None, max_value: None, max_precision: Self::PRECISION }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[DBig]) -> Computed<DBig> {
        move |inputs| {
            let x = inputs[0].clone();
            match func {
                Function::Ln => Computed::Value(x.ln()),
                Function::Add => Computed::Value(x + inputs[1].clone()),
                Function::Sub => Computed::Value(x - inputs[1].clone()),
                Function::Mul => Computed::Value(x * inputs[1].clone()),
                Function::Div => Computed::Value(x / inputs[1].clone()),
                Function::Rem => Computed::Value(x % inputs[1].clone()),
                other => Computed::Error(format!("dashu-float: unsupported {}", other.name())),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// fastnum — fixed-size decimals (`D512`, ~154 significant digits).
// ---------------------------------------------------------------------------

use fastnum::D512;

/// [fastnum](https://docs.rs/fastnum): fixed-size decimals; `D512` is a 512-bit
/// coefficient (~154 significant digits) with NaN / ±infinity signalling and default
/// rounding half-up (half-away-from-zero). It is the most complete competitor for
/// real functions: sqrt, cbrt, ln, log2, log10, exp, exp2 and sin/cos/tan, plus
/// `pow` and the five arithmetic ops — no inverse/hyperbolic trig, no log/atan2/
/// hypot. Its magnitude range is so vast that it never overflows on golden data, so
/// the envelope is treated as open; an out-of-range op would yield ±infinity (the
/// declared signal). Results grade to 50 fractional places. `Display` uses scientific
/// form for small magnitudes, so output is expanded to plain decimal for the grader.
pub struct FastNum;

impl FastNum {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Cbrt, Function::Ln, Function::Log2, Function::Log10,
        Function::Exp, Function::Exp2, Function::Sin, Function::Cos, Function::Tan,
        Function::Powf, Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Grading depth (fractional places) — well under D512's ~154-digit reach.
    const PRECISION: u32 = 50;
}

/// Map a fastnum result to a `Computed`: NaN -> `NaN`, ±infinity -> the signed
/// infinities (overflow), otherwise a finite value.
fn classify_fastnum(v: D512) -> Computed<D512> {
    if v.is_nan() {
        Computed::NonReal(NonReal::NaN)
    } else if v.is_infinite() {
        Computed::NonReal(if v.is_sign_negative() {
            NonReal::NegativeInfinity
        } else {
            NonReal::PositiveInfinity
        })
    } else {
        Computed::Value(v)
    }
}

impl DecimalSubject for FastNum {
    type Value = D512;

    fn name(&self) -> String {
        "fastnum".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Infinity },
            );
        }
        Capabilities {
            name: "fastnum".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> D512 {
        s.parse::<D512>().unwrap_or_else(|e| panic!("fastnum could not parse {s:?}: {e}"))
    }

    fn value_to_string(&self, v: &D512) -> String {
        expand_scientific(&v.reduce().to_string())
    }

    fn limits(&self, _value: &str) -> Limits {
        Limits { min_value: None, max_value: None, max_precision: Self::PRECISION }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[D512]) -> Computed<D512> {
        move |inputs| {
            let x = inputs[0];
            classify_fastnum(match func {
                Function::Sqrt => x.sqrt(),
                Function::Cbrt => x.cbrt(),
                Function::Ln => x.ln(),
                Function::Log2 => x.log2(),
                Function::Log10 => x.log10(),
                Function::Exp => x.exp(),
                Function::Exp2 => x.exp2(),
                Function::Sin => x.sin(),
                Function::Cos => x.cos(),
                Function::Tan => x.tan(),
                Function::Powf => x.pow(inputs[1]),
                Function::Add => x + inputs[1],
                Function::Sub => x - inputs[1],
                Function::Mul => x * inputs[1],
                Function::Div => x / inputs[1],
                Function::Rem => x % inputs[1],
                other => return Computed::Error(format!("fastnum: unsupported {}", other.name())),
            })
        }
    }
}

// ---------------------------------------------------------------------------
// decimal-rs — fixed-precision decimal (up to 38 significant digits).
// ---------------------------------------------------------------------------

use decimal_rs::Decimal as DecimalRs;

/// [decimal-rs](https://docs.rs/decimal-rs): a fixed-precision decimal (up to 38
/// significant digits) that signals out-of-range by returning `None` from its
/// `checked_*` / `sqrt` / `ln` / `exp` methods (-> `Absent`). It provides sqrt, ln,
/// exp and `checked_pow` plus the five checked arithmetic ops — no cbrt, no trig.
/// Its envelope is `(10^38 - 1)·10^126`; results grade to 34 fractional places (just
/// under its 38-digit reach). Default rounding is half-up (half-away-from-zero).
pub struct DecimalRsSubject;

impl DecimalRsSubject {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Ln, Function::Exp, Function::Powf,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Grading depth (fractional places) — just under decimal-rs's 38-digit reach.
    const PRECISION: u32 = 34;
}

impl DecimalSubject for DecimalRsSubject {
    type Value = DecimalRs;

    fn name(&self) -> String {
        "decimal-rs".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Absent },
            );
        }
        Capabilities {
            name: "decimal-rs".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> DecimalRs {
        DecimalRs::from_str(s).unwrap_or_else(|e| panic!("decimal-rs could not parse {s:?}: {e}"))
    }

    fn value_to_string(&self, v: &DecimalRs) -> String {
        expand_scientific(&v.normalize().to_string())
    }

    fn limits(&self, _value: &str) -> Limits {
        // int_val has <= 38 digits and scale reaches -126, so the largest magnitude
        // is (10^38 - 1)·10^126 — 38 nines followed by 126 zeros.
        let max = format!("{}{}", "9".repeat(38), "0".repeat(126));
        Limits {
            min_value: Some(format!("-{max}")),
            max_value: Some(max),
            max_precision: Self::PRECISION,
        }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[DecimalRs]) -> Computed<DecimalRs> {
        move |inputs| {
            let x = &inputs[0];
            let from_opt = |o: Option<DecimalRs>| match o {
                Some(v) => Computed::Value(v),
                None => Computed::Absent,
            };
            match func {
                Function::Sqrt => match x.sqrt() {
                    Some(v) => Computed::Value(v),
                    None => Computed::NonReal(NonReal::Imaginary),
                },
                Function::Ln => {
                    if *x <= DecimalRs::ZERO {
                        Computed::NonReal(NonReal::NegativeInfinity)
                    } else {
                        from_opt(x.ln())
                    }
                }
                Function::Exp => from_opt(x.exp()),
                Function::Powf => from_opt(x.checked_pow(&inputs[1])),
                Function::Add => from_opt(x.checked_add(&inputs[1])),
                Function::Sub => from_opt(x.checked_sub(&inputs[1])),
                Function::Mul => from_opt(x.checked_mul(&inputs[1])),
                Function::Div => from_opt(x.checked_div(&inputs[1])),
                Function::Rem => from_opt(x.checked_rem(&inputs[1])),
                other => Computed::Error(format!("decimal-rs: unsupported {}", other.name())),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// g_math — deterministic zero-float fixed-point with a lazy expression engine.
// ---------------------------------------------------------------------------

use g_math::canonical::{evaluate, gmath_parse, LazyExpr};

/// [g_math](https://github.com/nierto/gMath) (`g_math` 0.4): a deterministic,
/// zero-float fixed-point library with a lazy expression engine. Built on the
/// `balanced` profile (Q128.128 — signed integer magnitude up to ~1.7e38, ~38
/// fractional digits). It is the richest competitor for *functions*: sqrt, ln, exp,
/// the full trig set (sin/cos/tan, asin/acos/atan, atan2) and hyperbolics
/// (sinh/cosh/tanh, asinh/acosh/atanh), plus `pow` and four arithmetic ops — but no
/// remainder, cbrt, log2/log10/exp2, log or hypot. The `Value` is a lazy `LazyExpr`:
/// the op is built lazily and only realised (evaluated to a decimal string) in
/// `value_to_string`, where an out-of-range result fails evaluation and surfaces as
/// a panic the harness catches (matching the declared `Panic` overflow policy).
/// Results grade to 30 fractional places, within the Q128.128 fractional reach.
pub struct GMath;

impl GMath {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Ln, Function::Exp,
        Function::Sin, Function::Cos, Function::Tan,
        Function::Atan, Function::Asin, Function::Acos,
        Function::Sinh, Function::Cosh, Function::Tanh,
        Function::Asinh, Function::Acosh, Function::Atanh,
        Function::Powf, Function::Atan2,
        Function::Add, Function::Sub, Function::Mul, Function::Div,
    ];
    /// Fractional grading depth, within the Q128.128 fractional reach (~38 digits).
    const PRECISION: u32 = 30;
    /// Decimal digits requested when realising a result to a string.
    const REALISE_DIGITS: usize = 60;
}

impl DecimalSubject for GMath {
    type Value = LazyExpr;

    fn name(&self) -> String {
        "g_math".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic },
            );
        }
        Capabilities {
            name: "g_math".into(),
            radix: Radix::Decimal,
            config: BTreeMap::new(),
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> LazyExpr {
        gmath_parse(s).unwrap_or_else(|e| panic!("g_math could not parse {s:?}: {e:?}"))
    }

    fn value_to_string(&self, v: &LazyExpr) -> String {
        // Realise the lazy expression. A failure (out-of-range / undefined) panics,
        // which the harness catches as `Computed::Panic` (the declared overflow signal).
        match evaluate(v) {
            Ok(sv) => sv.to_decimal_string(Self::REALISE_DIGITS),
            Err(e) => panic!("g_math evaluation failed: {e:?}"),
        }
    }

    fn limits(&self, _value: &str) -> Limits {
        // balanced profile = Q128.128: signed integer magnitude up to 2^127 - 1.
        let bound = "170141183460469231731687303715884105727";
        Limits {
            min_value: Some(format!("-{bound}")),
            max_value: Some(bound.into()),
            max_precision: Self::PRECISION,
        }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[LazyExpr]) -> Computed<LazyExpr> {
        move |inputs| {
            let x = inputs[0].clone();
            let expr = match func {
                Function::Sqrt => x.sqrt(),
                Function::Ln => x.ln(),
                Function::Exp => x.exp(),
                Function::Sin => x.sin(),
                Function::Cos => x.cos(),
                Function::Tan => x.tan(),
                Function::Atan => x.atan(),
                Function::Asin => x.asin(),
                Function::Acos => x.acos(),
                Function::Sinh => x.sinh(),
                Function::Cosh => x.cosh(),
                Function::Tanh => x.tanh(),
                Function::Asinh => x.asinh(),
                Function::Acosh => x.acosh(),
                Function::Atanh => x.atanh(),
                Function::Powf => x.pow(inputs[1].clone()),
                Function::Atan2 => x.atan2(inputs[1].clone()),
                Function::Add => x + inputs[1].clone(),
                Function::Sub => x - inputs[1].clone(),
                Function::Mul => x * inputs[1].clone(),
                Function::Div => x / inputs[1].clone(),
                other => return Computed::Error(format!("g_math: unsupported {}", other.name())),
            };
            Computed::Value(expr)
        }
    }
}
