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

/// Count the significant integer digits of a decimal value string (the part left
/// of the point, leading zeros and sign stripped). `"316.22"` → 3, `"0.004"` → 0,
/// `"-50.0"` → 2. Used to grade a fixed-significant-digit library to the fractional
/// depth it can actually represent at this magnitude: a float with `S` significant
/// digits holds only `S − int_digits` fractional places once the integer part grows,
/// so a flat fractional `max_precision` would over-claim (and score false
/// `MisRounded`) on large-magnitude results. The grader passes the golden OUTPUT
/// value to `limits`, so the depth can be sized per result.
fn int_digits(value: &str) -> u32 {
    value
        .trim_start_matches(['-', '+'])
        .split('.')
        .next()
        .unwrap_or("")
        .trim_start_matches('0')
        .len() as u32
}

/// [rust_decimal](https://docs.rs/rust_decimal): a 96-bit fixed-point decimal with
/// up to ~28 significant digits (scale capped at 28), rounding half-to-even
/// (banker's) — its kernels break exact-half ties on coefficient parity, and
/// `round_dp` defaults to nearest-even. Transcendentals come from its `maths` feature
/// (`MathematicalOps`); it has no cbrt / exp2 / log2 / inverse-trig / hyperbolics, so
/// those are simply not declared.
pub struct RustDecimal;

impl RustDecimal {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Exp, Function::Ln, Function::Log10, Function::Powf,
        Function::Sin, Function::Cos, Function::Tan,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Significant-digit reach (~28, with the scale also capped at 28).
    const SIG_DIGITS: u32 = 28;
}

impl DecimalSubject for RustDecimal {
    type Value = Decimal;

    fn name(&self) -> String {
        "rust_decimal".into()
    }

    fn capabilities(&self) -> Capabilities {
        let mut functions = BTreeMap::new();
        for &f in Self::FUNCS {
            // rust_decimal rounds half-to-even (banker's) uniformly — arithmetic and
            // the `maths` transcendentals alike; an out-of-range result has no
            // representation (the `checked_*` ops yield `None`) -> `Absent`.
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Absent },
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

    fn limits(&self, value: &str) -> Limits {
        // Fixed-significant (~28 digits, scale <= 28): the representable fractional
        // depth shrinks to 28 − int_digits as the integer part grows, so grade to that
        // depth (a flat 28 over-claimed for large-magnitude sqrt/exp/powf/div results).
        let max_precision = Self::SIG_DIGITS.saturating_sub(int_digits(value));
        Limits {
            min_value: Some(Decimal::MIN.to_string()),
            max_value: Some(Decimal::MAX.to_string()),
            max_precision,
            // ~28-figure coefficient: a wider input can't be ingested exactly -> skip.
            max_significant_digits: Some(Self::SIG_DIGITS),
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
                Function::Sin => from_opt(x.checked_sin()),
                Function::Cos => from_opt(x.checked_cos()),
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
    /// f64's reliable significant-digit budget (`f64::DIGITS` = 15 is the guaranteed
    /// floor; 16 round-trips for nearly all values — used as the grading reach).
    const SIG_DIGITS: u32 = 16;
    /// Cap on graded fractional places (a sub-1 value carries ~15 fractional digits).
    const FRAC_CAP: u32 = 15;
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

    fn limits(&self, value: &str) -> Limits {
        // f64 carries ~16 reliable significant digits, so its representable fractional
        // depth is 16 − int_digits, capped at 15. A flat 15 over-claimed the moment
        // |x| >= 10, manufacturing artifact MisRounded rather than honest
        // binary-vs-decimal misses; grading a constant ~16 significant digits is fair.
        let max_precision = Self::SIG_DIGITS.saturating_sub(int_digits(value)).min(Self::FRAC_CAP);
        Limits {
            min_value: Some(format!("{}", f64::MIN)),
            max_value: Some(format!("{}", f64::MAX)),
            max_precision,
            // ~16 reliable figures: a longer literal isn't f64-representable -> skip.
            max_significant_digits: Some(Self::SIG_DIGITS),
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

use std::num::NonZeroU64;

use bigdecimal::{BigDecimal, Context};

/// [bigdecimal](https://docs.rs/bigdecimal): an arbitrary-precision decimal with no
/// finite envelope. Being unbounded, computing `exp` (or any exp-family op) of a
/// large golden input would produce an astronomically large number and never
/// terminate, so `exp` is deliberately NOT declared — only the slow-growth `sqrt`
/// and `cbrt` plus the five arithmetic ops are (it has no `ln` / `powf` / trig).
/// add/sub/mul/rem are exact in bigdecimal; sqrt/cbrt/div round, and the crate's
/// DEFAULT context is only 100 SIGNIFICANT digits (half-to-even) — which leaves
/// FEWER than 30 fractional digits once a result's integer part is large (golden
/// sqrt/cbrt/div outputs reach hundreds, and div up to 1233, integer digits). So
/// those three are computed under an explicit high-precision context instead, giving
/// bigdecimal (genuinely arbitrary-precision) a fair >= 30 fractional digits before
/// the 30-place grade. Being unbounded it never overflows: the only non-value outcome
/// is `sqrt` of a negative (-> imaginary).
pub struct BigDecimalSubject;

impl BigDecimalSubject {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Cbrt,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Grading depth (fractional places).
    const PRECISION: u32 = 30;
    /// Significant-digit context for the rounding ops (sqrt/cbrt/div): covers the
    /// widest golden output (~1233 integer digits for div) plus the 30 graded
    /// fractional places, with headroom, so >= 30 correct fractional digits exist.
    const COMPUTE_PRECISION: u64 = 1320;
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
        // Arbitrary precision: no figure cap (it ingests any-length literal exactly).
        Limits { min_value: None, max_value: None, max_precision: Self::PRECISION, max_significant_digits: None }
    }

    fn execute(
        &self,
        func: Function,
        _mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[BigDecimal]) -> Computed<BigDecimal> {
        // High-precision context for the rounding ops; add/sub/mul/rem stay exact.
        let ctx = Context::new(
            NonZeroU64::new(Self::COMPUTE_PRECISION).unwrap(),
            bigdecimal::rounding::RoundingMode::HalfEven,
        );
        move |inputs| {
            let x = &inputs[0];
            match func {
                Function::Sqrt => match x.sqrt_with_context(&ctx) {
                    Some(v) => Computed::Value(v),
                    None => Computed::NonReal(NonReal::Imaginary),
                },
                Function::Cbrt => Computed::Value(x.cbrt_with_context(&ctx)),
                Function::Add => Computed::Value(x + &inputs[1]),
                Function::Sub => Computed::Value(x - &inputs[1]),
                Function::Mul => Computed::Value(x * &inputs[1]),
                // bigdecimal's `/` is pinned to the 100-digit default context; divide
                // at high precision via the context-aware reciprocal instead.
                Function::Div => {
                    let inv = inputs[1].inverse_with_context(&ctx);
                    Computed::Value(x * &inv)
                }
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
/// are lifted to a high (1280-significant-digit) working precision so `ln` and every
/// rounded arithmetic op compute well past the 30-fractional-place grade depth at the
/// golden set's largest magnitudes; results are graded to 30 fractional places.
/// Unbounded, so the envelope is open; `Display` is plain decimal (the
/// `expand_scientific` wrap is a harmless no-op here).
pub struct DashuFloat;

impl DashuFloat {
    const FUNCS: &'static [Function] = &[
        Function::Ln,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// Significant-digit precision every operation is lifted to before computing.
    /// dashu rounds each binary op to `Context::max(lhs, rhs)` significant digits, so
    /// this must cover the widest result: golden `div`/`add`/`sub` operands reach ~925
    /// integer digits, so 1280 sig digits leaves ample fractional headroom past the
    /// 30-place grade depth (a 34-digit working precision silently truncated every
    /// large-magnitude result to garbage — a false `MisRounded`, not a real one).
    const WORKING_PRECISION: usize = 1280;
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
        // Arbitrary precision: no figure cap (it ingests any-length literal exactly).
        Limits { min_value: None, max_value: None, max_precision: Self::PRECISION, max_significant_digits: None }
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
                // dashu has no -inf/NaN sentinel; ln of a non-positive would panic.
                // No golden case feeds one today — this guards for parity with the
                // rust_decimal / decimal-rs adapters' NonReal mapping.
                Function::Ln if x <= DBig::ZERO => Computed::NonReal(NonReal::NegativeInfinity),
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
    /// D512's significant-digit reach (512-bit coefficient ≈ 154 decimal digits).
    const SIG_DIGITS: u32 = 154;
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
            // fastnum's default context traps exceptional conditions, so overflow /
            // division-by-zero PANIC under a normal (debug) `cargo test` rather than
            // yielding ±infinity; `Panic` is the faithful policy. Dormant on the
            // golden set anyway (open envelope -> never out of range).
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Panic },
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

    fn limits(&self, value: &str) -> Limits {
        // D512 is fixed-significant (~154 digits): once the integer part grows, the
        // representable fractional depth shrinks to `154 − int_digits`. Grade to the
        // smaller of the 50-place cap and that depth, so large-magnitude results
        // (e.g. sqrt of a ~600-digit input) aren't scored against fractional digits
        // D512 cannot hold. Envelope stays open (D512's magnitude range dwarfs the set).
        let max_precision = Self::PRECISION.min(Self::SIG_DIGITS.saturating_sub(int_digits(value)));
        // ~154-figure D512 coefficient: a longer literal can't be ingested -> skip.
        Limits { min_value: None, max_value: None, max_precision, max_significant_digits: Some(Self::SIG_DIGITS) }
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
/// Its envelope is `(10^38 - 1)·10^126`. Because it is fixed-significant (38 digits)
/// with a floating point, the fractional depth it can represent is value-dependent
/// (`38 − int_digits`), so results grade to that depth rather than a flat value.
/// Default rounding is half-up (half-away-from-zero).
pub struct DecimalRsSubject;

impl DecimalRsSubject {
    const FUNCS: &'static [Function] = &[
        Function::Sqrt, Function::Ln, Function::Exp, Function::Powf,
        Function::Add, Function::Sub, Function::Mul, Function::Div, Function::Rem,
    ];
    /// decimal-rs's significant-digit reach (`MAX_PRECISION = 38`).
    const SIG_DIGITS: u32 = 38;
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

    fn limits(&self, value: &str) -> Limits {
        // Magnitude envelope: int_val has <= 38 digits and scale reaches -126, so the
        // largest magnitude is (10^38 - 1)·10^126 — 38 nines followed by 126 zeros.
        let max = format!("{}{}", "9".repeat(38), "0".repeat(126));
        // Fixed-significant + floating point: the representable fractional depth
        // shrinks as the integer part grows, so grade to 38 − int_digits (a flat 34
        // over-claimed for large-magnitude sqrt/exp/powf/div, scoring false MisRounded).
        let max_precision = Self::SIG_DIGITS.saturating_sub(int_digits(value));
        Limits {
            min_value: Some(format!("-{max}")),
            max_value: Some(max),
            max_precision,
            // 38-figure coefficient: a longer literal can't be ingested exactly -> skip.
            max_significant_digits: Some(Self::SIG_DIGITS),
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
            // The FASC canonical pipeline rounds half-away-from-zero on the decimal
            // compute path (and to-nearest on the 2^-128 grid for bare-integer
            // inputs), both accurate well past the 30-digit grade depth. The grader
            // re-rounds g_math's deeper output to that depth under this declared
            // mode, so half-away-from-zero is the faithful declaration. An
            // out-of-range result fails evaluation -> panic, caught as
            // `Computed::Panic` (the declared overflow signal).
            functions.insert(
                f,
                FnSupport { mode: RoundingMode::HalfAwayFromZero, overflow: Overflow::Panic },
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
            // Q128.128 magnitude bound already excludes the wide inputs; no figure cap.
            max_significant_digits: None,
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
