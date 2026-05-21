//! Library-agnostic precision-test harness.
//!
//! This module is the shared engine behind two consumers:
//!
//!   * `tests/precision_harness_smoke.rs` — a green-on-`cargo test` gate
//!     that drives the reference subject (`decimal-scaled`) through the
//!     harness and asserts it lands at 0 LSBε / 0.0 ULP, the same
//!     correctly-rounded invariant `ulp_strict_golden` proves (this is
//!     ADDITIVE — the 286/286 golden gate is untouched);
//!   * `benches/lib_cmp_precision_harness.rs` — the comparative runner
//!     that sweeps every subject and emits the shootout table.
//!
//! It is included into both crate roots verbatim via
//! `#[path = "support/precision_harness.rs"]` (tests) and
//! `#[path = "../tests/support/precision_harness.rs"]` (benches). Both
//! `tests/` and `benches/` are independent crate roots, so a shared
//! `#[path]`-included file is the cleanest way to share code between
//! them without exporting harness internals from `src/` and the public
//! API.
//!
//! # The model (owner's spec)
//!
//! A [`PrecisionSubject`] is any library that can be asked to evaluate a
//! [`Method`] at a given [`Width`] / scale on an [`Input`], under a
//! [`RoundingMode`]. It reports back the value it computed as a decimal
//! string plus the rounding it actually applied, or [`SubjectOutput::NotApplicable`]
//! when it can neither represent that (width, scale) nor expose the
//! method.
//!
//! The [`Harness`] owns the oracle — the very same mpmath golden tables
//! under `tests/golden/` that `tests/ulp_strict_golden.rs` consumes — and
//! from a subject's value-string + reported rounding computes:
//!
//!   * **LSBε** — the bit-width of `|value − oracle_correctly_rounded|`
//!     measured in storage LSB at the tier scale (`0` ⇒ bit-exact);
//!   * **ULP** — the same magnitude as a continuous distance in units of
//!     the storage LSB (`1` LSB == `1` ULP at the tier scale).
//!
//! # The half-even-vs-truncation fairness rule
//!
//! Different libraries break ties differently. The golden table records
//! `(floor_raw, cls)`, from which the correctly-rounded integer for ANY
//! mode is derivable. The harness rounds the oracle under the subject's
//! **reported** mode before diffing, so a truncating library is scored
//! against the truncated oracle and a half-even library against the
//! half-even oracle — convention is never mistaken for inaccuracy.

#![allow(dead_code)]

use decimal_scaled::RoundingMode;

// ════════════════════════════════════════════════════════════════════
// Method — the strict transcendental + arithmetic surface
// ════════════════════════════════════════════════════════════════════

/// The function under test. Covers the full strict transcendental
/// surface plus the four arithmetic operations. Two-argument methods
/// (`Log`, `Atan2`, `Pow`, and the arithmetic ops) consume the second
/// operand from [`Input::input2`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    // ── transcendental, single-argument ──
    Sqrt,
    Cbrt,
    Exp,
    Ln,
    Log2,
    Log10,
    Exp2,
    Sin,
    Cos,
    Tan,
    Atan,
    Asin,
    Acos,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    // ── transcendental, two-argument ──
    Log,
    Atan2,
    Pow,
    // ── arithmetic ──
    Add,
    Sub,
    Mul,
    Div,
}

impl Method {
    /// Every method in canonical (table) order.
    pub const ALL: [Method; 26] = [
        Method::Sqrt,
        Method::Cbrt,
        Method::Exp,
        Method::Ln,
        Method::Log2,
        Method::Log10,
        Method::Exp2,
        Method::Sin,
        Method::Cos,
        Method::Tan,
        Method::Atan,
        Method::Asin,
        Method::Acos,
        Method::Sinh,
        Method::Cosh,
        Method::Tanh,
        Method::Asinh,
        Method::Acosh,
        Method::Atanh,
        Method::Log,
        Method::Atan2,
        Method::Pow,
        Method::Add,
        Method::Sub,
        Method::Mul,
        Method::Div,
    ];

    /// The transcendental surface only — what the golden oracle covers.
    pub const TRANSCENDENTAL: [Method; 22] = [
        Method::Sqrt,
        Method::Cbrt,
        Method::Exp,
        Method::Ln,
        Method::Log2,
        Method::Log10,
        Method::Exp2,
        Method::Sin,
        Method::Cos,
        Method::Tan,
        Method::Atan,
        Method::Asin,
        Method::Acos,
        Method::Sinh,
        Method::Cosh,
        Method::Tanh,
        Method::Asinh,
        Method::Acosh,
        Method::Atanh,
        Method::Log,
        Method::Atan2,
        Method::Pow,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Method::Sqrt => "sqrt",
            Method::Cbrt => "cbrt",
            Method::Exp => "exp",
            Method::Ln => "ln",
            Method::Log2 => "log2",
            Method::Log10 => "log10",
            Method::Exp2 => "exp2",
            Method::Sin => "sin",
            Method::Cos => "cos",
            Method::Tan => "tan",
            Method::Atan => "atan",
            Method::Asin => "asin",
            Method::Acos => "acos",
            Method::Sinh => "sinh",
            Method::Cosh => "cosh",
            Method::Tanh => "tanh",
            Method::Asinh => "asinh",
            Method::Acosh => "acosh",
            Method::Atanh => "atanh",
            Method::Log => "log",
            Method::Atan2 => "atan2",
            Method::Pow => "powf",
            Method::Add => "add",
            Method::Sub => "sub",
            Method::Mul => "mul",
            Method::Div => "div",
        }
    }

    /// `true` for the two-argument methods (need [`Input::input2`]).
    pub fn is_binary(self) -> bool {
        matches!(
            self,
            Method::Log
                | Method::Atan2
                | Method::Pow
                | Method::Add
                | Method::Sub
                | Method::Mul
                | Method::Div
        )
    }

    /// The golden table stem for this method, or `None` for the
    /// arithmetic ops (no oracle table — they are exact by construction
    /// and not part of the golden roster).
    pub fn golden_stem(self) -> Option<&'static str> {
        match self {
            Method::Add | Method::Sub | Method::Mul | Method::Div => None,
            other => Some(other.name()),
        }
    }
}

// ════════════════════════════════════════════════════════════════════
// Width — the thirteen decimal tiers D9 … D1232
// ════════════════════════════════════════════════════════════════════

/// One of the crate's thirteen decimal width tiers. Each tier pins a
/// storage bit-width and a canonical scale (the scale the golden tables
/// were generated at — `SCALE = floor((digits-1)/2)` for that tier).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Width {
    D9,
    D18,
    D38,
    D57,
    D76,
    D115,
    D153,
    D230,
    D307,
    D462,
    D616,
    D924,
    D1232,
}

impl Width {
    /// Every tier, narrow → wide.
    pub const ALL: [Width; 13] = [
        Width::D9,
        Width::D18,
        Width::D38,
        Width::D57,
        Width::D76,
        Width::D115,
        Width::D153,
        Width::D230,
        Width::D307,
        Width::D462,
        Width::D616,
        Width::D924,
        Width::D1232,
    ];

    /// Decimal digit capacity of the tier (the `N` in `DN`).
    pub fn digits(self) -> u32 {
        match self {
            Width::D9 => 9,
            Width::D18 => 18,
            Width::D38 => 38,
            Width::D57 => 57,
            Width::D76 => 76,
            Width::D115 => 115,
            Width::D153 => 153,
            Width::D230 => 230,
            Width::D307 => 307,
            Width::D462 => 462,
            Width::D616 => 616,
            Width::D924 => 924,
            Width::D1232 => 1232,
        }
    }

    /// The canonical scale the golden tables use for this tier.
    pub fn canonical_scale(self) -> u32 {
        match self {
            Width::D9 => 4,
            Width::D18 => 9,
            Width::D38 => 19,
            Width::D57 => 28,
            Width::D76 => 35,
            Width::D115 => 57,
            Width::D153 => 76,
            Width::D230 => 115,
            Width::D307 => 150,
            Width::D462 => 230,
            Width::D616 => 308,
            Width::D924 => 460,
            Width::D1232 => 615,
        }
    }

    /// The `dN_sS` golden-file infix for this tier at its canonical scale.
    pub fn golden_infix(self) -> &'static str {
        match self {
            Width::D9 => "d9_s4",
            Width::D18 => "d18_s9",
            Width::D38 => "d38_s19",
            Width::D57 => "d57_s28",
            Width::D76 => "d76_s35",
            Width::D115 => "d115_s57",
            Width::D153 => "d153_s76",
            Width::D230 => "d230_s115",
            Width::D307 => "d307_s150",
            Width::D462 => "d462_s230",
            Width::D616 => "d616_s308",
            Width::D924 => "d924_s460",
            Width::D1232 => "d1232_s615",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Width::D9 => "D9",
            Width::D18 => "D18",
            Width::D38 => "D38",
            Width::D57 => "D57",
            Width::D76 => "D76",
            Width::D115 => "D115",
            Width::D153 => "D153",
            Width::D230 => "D230",
            Width::D307 => "D307",
            Width::D462 => "D462",
            Width::D616 => "D616",
            Width::D924 => "D924",
            Width::D1232 => "D1232",
        }
    }
}

// ════════════════════════════════════════════════════════════════════
// Input — a canonical decimal at a tier/scale
// ════════════════════════════════════════════════════════════════════

/// One canonical input: the storage integer(s) of the operand(s) at the
/// tier scale, expressed as decimal strings so any subject can parse
/// them losslessly. `input` is `x`; `input2` is the second operand for
/// binary methods (`None` otherwise).
///
/// `value_string()` renders the operand as a plain decimal value string
/// (`raw / 10^scale`) for libraries that consume a decimal literal.
#[derive(Clone, Debug)]
pub struct Input {
    /// First operand storage integer at `scale`, as a decimal string.
    pub raw: String,
    /// Second operand storage integer at `scale`, for binary methods.
    pub input2: Option<String>,
    /// The tier this input belongs to.
    pub width: Width,
    /// The scale `raw` / `input2` are expressed at.
    pub scale: u32,
}

impl Input {
    /// `raw / 10^scale` as a plain decimal value string (no exponent).
    pub fn value_string(&self) -> String {
        raw_to_decimal_string(&self.raw, self.scale)
    }

    /// `input2 / 10^scale` as a plain decimal value string, if present.
    pub fn value2_string(&self) -> Option<String> {
        self.input2
            .as_ref()
            .map(|s| raw_to_decimal_string(s, self.scale))
    }
}

/// Render a (possibly negative) storage integer `raw / 10^scale` as a
/// plain decimal string. Pure decimal-string arithmetic so it is exact
/// at any width.
pub fn raw_to_decimal_string(raw: &str, scale: u32) -> String {
    let scale = scale as usize;
    let (neg, mag) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw.strip_prefix('+').unwrap_or(raw)),
    };
    let mag = mag.trim_start_matches('0');
    let mag = if mag.is_empty() { "0" } else { mag };
    let body = if scale == 0 {
        mag.to_string()
    } else if mag.len() > scale {
        let cut = mag.len() - scale;
        format!("{}.{}", &mag[..cut], &mag[cut..])
    } else {
        format!("0.{}{}", "0".repeat(scale - mag.len()), mag)
    };
    if neg && mag != "0" {
        format!("-{body}")
    } else {
        body
    }
}

// ════════════════════════════════════════════════════════════════════
// SubjectOutput / PrecisionResult
// ════════════════════════════════════════════════════════════════════

/// What a subject reports for one `(method, width, scale, input, mode)`.
#[derive(Clone, Debug)]
pub enum SubjectOutput {
    /// The subject computed a value. `value` is the result as a decimal
    /// string; `rounding` is the mode the subject actually applied (it
    /// may ignore the requested mode and report its own native one).
    Computed {
        value: String,
        rounding: RoundingMode,
    },
    /// The subject cannot represent this (width, scale), or does not
    /// expose this method, or rejected the input (domain / NaN / inf).
    NotApplicable,
}

/// The harness's scored verdict for one cell.
#[derive(Clone, Debug)]
pub enum PrecisionResult {
    /// The subject ran and was scored against the oracle.
    Executed {
        /// The subject's value, as a decimal string.
        value: String,
        /// The rounding the subject reported applying.
        rounding: RoundingMode,
        /// Bit-width of `|value − oracle_cr|` in storage LSB. `0` ⇒
        /// bit-exact (correctly rounded under the reported mode).
        lsbe: u32,
        /// `|value − oracle_cr|` as a continuous distance in storage LSB
        /// (`1` LSB == `1` ULP at the tier scale).
        ulp: f64,
    },
    /// The subject reported [`SubjectOutput::NotApplicable`], or no
    /// oracle exists for this `(method, width)`.
    NotApplicable,
}

impl PrecisionResult {
    /// `true` if executed and bit-exact (`lsbe == 0`).
    pub fn is_correctly_rounded(&self) -> bool {
        matches!(self, PrecisionResult::Executed { lsbe: 0, .. })
    }
}

// ════════════════════════════════════════════════════════════════════
// PrecisionSubject — the library adapter trait
// ════════════════════════════════════════════════════════════════════

/// A library that can be driven by the harness. The subject sets up its
/// type for `(width, scale)`, runs `method` on `input` under `mode`, and
/// returns its computed value-as-string + the rounding it actually
/// applied — or [`SubjectOutput::NotApplicable`].
pub trait PrecisionSubject {
    /// Display name for the shootout table.
    fn name(&self) -> &str;

    /// The rounding mode this subject natively applies (for the table
    /// note). The harness scores the oracle under whatever the subject
    /// REPORTS per call; this is just the documentation default.
    fn native_mode(&self) -> RoundingMode;

    /// Run one cell.
    fn eval(
        &self,
        method: Method,
        width: Width,
        scale: u32,
        input: &Input,
        mode: RoundingMode,
    ) -> SubjectOutput;
}

// ════════════════════════════════════════════════════════════════════
// The oracle — golden (floor, cls) → correctly-rounded integer per mode
// ════════════════════════════════════════════════════════════════════

/// Fractional class parsed from the golden table's class column.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cls {
    /// `frac == 0` — exactly representable at the scale.
    Exact,
    /// `0 < frac < 0.5`.
    Low,
    /// `frac == 0.5` — exact tie.
    Tie,
    /// `0.5 < frac < 1`.
    High,
}

impl Cls {
    fn parse(s: &str) -> Self {
        match s {
            "Z" => Cls::Exact,
            "L" => Cls::Low,
            "E" => Cls::Tie,
            "G" => Cls::High,
            other => panic!("unknown class column: {other:?}"),
        }
    }
}

/// One golden roster line: the input column(s) plus the floor/class
/// reference for the oracle.
#[derive(Clone, Debug)]
pub struct GoldenCase {
    pub input: String,
    pub input2: Option<String>,
    /// `floor(f(x) * 10^scale)` as a decimal string.
    pub floor: String,
    pub cls: Cls,
}

/// Parse one golden line; `None` for comments / blanks.
pub fn parse_golden_line(line: &str) -> Option<GoldenCase> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let parts: Vec<&str> = line.split('\t').collect();
    match parts.len() {
        3 => Some(GoldenCase {
            input: parts[0].to_string(),
            input2: None,
            floor: parts[1].to_string(),
            cls: Cls::parse(parts[2]),
        }),
        4 => Some(GoldenCase {
            input: parts[0].to_string(),
            input2: Some(parts[1].to_string()),
            floor: parts[2].to_string(),
            cls: Cls::parse(parts[3]),
        }),
        other => panic!("golden line has {other} columns, expected 3 or 4: {line:?}"),
    }
}

/// Whether the correctly-rounded value for `mode` is `floor + 1` (true)
/// or `floor` (false), given the class and the sign of the true value.
/// Mirrors the rule in `tests/ulp_strict_golden.rs` exactly so the
/// harness's oracle and the golden gate agree.
///
/// `HalfToEven` on an exact tie is parity-dependent and resolved by the
/// caller (it needs `floor`'s storage parity).
fn bump_to_ceil(mode: RoundingMode, cls: Cls, true_nonneg: bool) -> bool {
    match cls {
        Cls::Exact => false,
        Cls::Low => match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => false,
            RoundingMode::Trunc => !true_nonneg,
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
        Cls::High => match mode {
            RoundingMode::HalfToEven
            | RoundingMode::HalfAwayFromZero
            | RoundingMode::HalfTowardZero => true,
            RoundingMode::Trunc => !true_nonneg,
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
        Cls::Tie => match mode {
            RoundingMode::HalfToEven => true, // overridden by caller on parity
            RoundingMode::HalfAwayFromZero => true_nonneg,
            RoundingMode::HalfTowardZero => !true_nonneg,
            RoundingMode::Trunc => !true_nonneg,
            RoundingMode::Floor => false,
            RoundingMode::Ceiling => true,
        },
    }
}

/// The correctly-rounded oracle integer (as a decimal string at the
/// tier scale) for `(floor, cls)` under `mode`. This is the half-even-
/// vs-truncation fairness fold: the harness rounds the oracle under the
/// subject's REPORTED mode before diffing.
pub fn oracle_correctly_rounded(floor: &str, cls: Cls, mode: RoundingMode) -> String {
    let true_nonneg = !floor.starts_with('-');

    if cls == Cls::Tie && mode == RoundingMode::HalfToEven {
        // Pick the even neighbour: floor if its last digit is even.
        let last = floor
            .bytes()
            .rev()
            .find(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .unwrap_or(0);
        return if last % 2 == 0 {
            floor.to_string()
        } else {
            dec_add_one(floor)
        };
    }

    if bump_to_ceil(mode, cls, true_nonneg) {
        dec_add_one(floor)
    } else {
        floor.to_string()
    }
}

// ════════════════════════════════════════════════════════════════════
// The Harness
// ════════════════════════════════════════════════════════════════════

/// Owns the oracle and computes [`PrecisionResult`] from a subject's
/// reported value-string + rounding.
pub struct Harness;

impl Harness {
    /// Score a subject's output for one cell against the oracle case.
    ///
    /// `case.floor`/`case.cls` define the correctly-rounded oracle at the
    /// tier scale. The subject's value string is parsed into a storage
    /// integer at the same scale; the diff to the mode-folded oracle is
    /// the LSBε (bit width) and ULP (continuous distance).
    pub fn score(out: &SubjectOutput, case: &GoldenCase, scale: u32) -> PrecisionResult {
        let (value, rounding) = match out {
            SubjectOutput::NotApplicable => return PrecisionResult::NotApplicable,
            SubjectOutput::Computed { value, rounding } => (value, *rounding),
        };

        // Parse the subject's decimal value into a scaled integer string
        // at the tier scale, ROUNDING any guard digits past `scale` under
        // the subject's REPORTED mode. A high-precision peer emits more
        // digits than the tier holds; truncating those would cost it up
        // to a full LSB even when its value is within 0.5 ULP of true.
        // Rounding the subject's own emission to the storage grid under
        // the same mode the oracle is folded under is the symmetric, fair
        // comparison: it measures the value the peer would store, not a
        // parse artifact.
        let Some(subject_scaled) = decimal_to_scaled_rounded(value, scale, rounding) else {
            return PrecisionResult::NotApplicable;
        };

        // Fold the oracle under the subject's reported mode.
        let oracle = oracle_correctly_rounded(&case.floor, case.cls, rounding);

        let diff = dec_abs_diff(&subject_scaled, &oracle);
        let lsbe = dec_bit_len(&diff);
        let ulp = dec_to_f64(&diff);

        PrecisionResult::Executed {
            value: value.clone(),
            rounding,
            lsbe,
            ulp,
        }
    }
}

// ════════════════════════════════════════════════════════════════════
// Decimal-string arithmetic — width-agnostic, no big-int dependency
// ════════════════════════════════════════════════════════════════════
//
// The harness scores at every width up to D1232<615>, where a single
// storage integer is ~1200 decimal digits — far past i128. Rather than
// pull in a big-integer crate, the harness does its handful of integer
// operations directly on the decimal digit strings: parse a decimal
// value to a scaled integer, take an absolute difference, count decimal
// digits / bits, and convert a small magnitude to f64. All exact at any
// width.

/// Parse a plain decimal string `value` into the integer
/// `trunc(value * 10^scale)` rendered as a (signed) decimal string.
/// Truncates digits past `scale`. Returns `None` for non-finite or
/// exponent-bearing strings.
pub fn decimal_to_scaled_string(value: &str, scale: u32) -> Option<String> {
    let scale = scale as usize;
    let s = value.trim();
    let low = s.to_ascii_lowercase();
    if low.contains("nan") || low.contains("inf") || low.contains("overflow") || low.contains('e') {
        return None;
    }
    let (neg, body) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (int_part, frac_part) = body.split_once('.').unwrap_or((body, ""));
    if int_part.is_empty() && frac_part.is_empty() {
        return None;
    }
    if !int_part.bytes().all(|b| b.is_ascii_digit())
        || !frac_part.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let mut frac = String::from(frac_part);
    if frac.len() < scale {
        frac.push_str(&"0".repeat(scale - frac.len()));
    } else {
        frac.truncate(scale);
    }
    let digits = format!("{int_part}{frac}");
    let trimmed = digits.trim_start_matches('0');
    let trimmed = if trimmed.is_empty() { "0" } else { trimmed };
    if neg && trimmed != "0" {
        Some(format!("-{trimmed}"))
    } else {
        Some(trimmed.to_string())
    }
}

/// Parse a plain decimal string `value` into `round(value * 10^scale)`
/// under `mode`, rendered as a signed decimal string. Unlike
/// [`decimal_to_scaled_string`] (which truncates), this rounds the digits
/// past `scale` so a high-precision peer is scored on the value it would
/// store at the grid, not docked a full LSB by truncation. Returns `None`
/// for non-finite / exponent-bearing strings.
pub fn decimal_to_scaled_rounded(value: &str, scale: u32, mode: RoundingMode) -> Option<String> {
    let scale = scale as usize;
    let s = value.trim();
    let low = s.to_ascii_lowercase();
    if low.contains("nan") || low.contains("inf") || low.contains("overflow") || low.contains('e') {
        return None;
    }
    let (neg, body) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    let (int_part, frac_part) = body.split_once('.').unwrap_or((body, ""));
    if int_part.is_empty() && frac_part.is_empty() {
        return None;
    }
    if !int_part.bytes().all(|b| b.is_ascii_digit())
        || !frac_part.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    // The kept magnitude is int_part followed by the first `scale` frac
    // digits; the dropped tail decides the rounding increment.
    let mut frac = String::from(frac_part);
    let (kept_frac, tail): (String, String) = if frac.len() <= scale {
        frac.push_str(&"0".repeat(scale - frac.len()));
        (frac, String::new())
    } else {
        let t = frac.split_off(scale);
        (frac, t)
    };
    let kept_digits = format!("{int_part}{kept_frac}");
    let kept = {
        let t = kept_digits.trim_start_matches('0');
        if t.is_empty() { "0".to_string() } else { t.to_string() }
    };

    // Decide whether to round the magnitude up by one based on the tail.
    let round_up = if tail.is_empty() || tail.bytes().all(|b| b == b'0') {
        false
    } else {
        let first = tail.as_bytes()[0] - b'0';
        let rest_nonzero = tail.bytes().skip(1).any(|b| b != b'0');
        let half_or_more = first > 5 || (first == 5 && rest_nonzero);
        let exact_half = first == 5 && !rest_nonzero;
        match mode {
            RoundingMode::Trunc => false,
            RoundingMode::Floor => neg,    // toward -inf: negatives round away
            RoundingMode::Ceiling => !neg, // toward +inf: positives round away
            RoundingMode::HalfAwayFromZero => first >= 5,
            RoundingMode::HalfTowardZero => half_or_more,
            RoundingMode::HalfToEven => {
                if exact_half {
                    let last = kept.bytes().last().map(|b| b - b'0').unwrap_or(0);
                    last % 2 == 1
                } else {
                    first > 5
                }
            }
        }
    };

    let mag = if round_up { dec_add_mag(&kept, "1") } else { kept };
    let mag = {
        let t = mag.trim_start_matches('0');
        if t.is_empty() { "0".to_string() } else { t.to_string() }
    };
    if neg && mag != "0" {
        Some(format!("-{mag}"))
    } else {
        Some(mag)
    }
}

/// `a + 1` for a signed decimal-integer string.
fn dec_add_one(a: &str) -> String {
    if let Some(mag) = a.strip_prefix('-') {
        // -(|a| - 1)
        let dec = dec_sub_mag(mag, "1");
        if dec == "0" {
            "0".to_string()
        } else {
            format!("-{dec}")
        }
    } else {
        dec_add_mag(a, "1")
    }
}

/// `|a − b|` for two signed decimal-integer strings, as an unsigned
/// magnitude string.
pub fn dec_abs_diff(a: &str, b: &str) -> String {
    let (na, ma) = split_sign(a);
    let (nb, mb) = split_sign(b);
    if na == nb {
        // same sign: |a-b| = | |ma| - |mb| |
        dec_sub_mag(ma, mb)
    } else {
        // opposite sign: |a-b| = |ma| + |mb|
        dec_add_mag(ma, mb)
    }
}

fn split_sign(x: &str) -> (bool, &str) {
    match x.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, x.strip_prefix('+').unwrap_or(x)),
    }
}

/// Magnitude addition of two unsigned decimal-digit strings.
fn dec_add_mag(a: &str, b: &str) -> String {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut out = Vec::new();
    let mut carry = 0u8;
    let (mut i, mut j) = (a.len(), b.len());
    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 {
            i -= 1;
            a[i] - b'0'
        } else {
            0
        };
        let db = if j > 0 {
            j -= 1;
            b[j] - b'0'
        } else {
            0
        };
        let s = da + db + carry;
        out.push(b'0' + (s % 10));
        carry = s / 10;
    }
    if out.is_empty() {
        return "0".to_string();
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

/// Magnitude `| |a| − |b| |` of two unsigned decimal-digit strings.
fn dec_sub_mag(a: &str, b: &str) -> String {
    let (hi, lo) = match cmp_mag(a, b) {
        core::cmp::Ordering::Less => (b, a),
        _ => (a, b),
    };
    let hi = hi.trim_start_matches('0');
    let lo = lo.trim_start_matches('0');
    let (hb, lb) = (hi.as_bytes(), lo.as_bytes());
    let mut out = Vec::new();
    let mut borrow = 0i8;
    let (mut i, mut j) = (hb.len(), lb.len());
    while i > 0 {
        i -= 1;
        let dh = (hb[i] - b'0') as i8;
        let dl = if j > 0 {
            j -= 1;
            (lb[j] - b'0') as i8
        } else {
            0
        };
        let mut d = dh - dl - borrow;
        if d < 0 {
            d += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(b'0' + d as u8);
    }
    while out.len() > 1 && *out.last().unwrap() == b'0' {
        out.pop();
    }
    out.reverse();
    let s = String::from_utf8(out).unwrap();
    if s.is_empty() { "0".to_string() } else { s }
}

/// Compare two unsigned decimal-digit strings by magnitude.
fn cmp_mag(a: &str, b: &str) -> core::cmp::Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

/// Bit length of an unsigned decimal-magnitude string. `0` ⇒ exactly
/// zero (correctly rounded). Uses `f64::log2` of the digit count for the
/// rough scale, then refines exactly for small magnitudes that fit u128.
pub fn dec_bit_len(mag: &str) -> u32 {
    let mag = mag.trim_start_matches('0');
    if mag.is_empty() || mag == "0" {
        return 0;
    }
    // Small magnitudes: exact via u128.
    if mag.len() <= 38 {
        if let Ok(v) = mag.parse::<u128>() {
            return 128 - v.leading_zeros();
        }
    }
    // Large magnitudes: bit_len ≈ digits * log2(10), conservative ceil.
    // (Only used at the very wide tiers where a non-zero diff is already
    // a gross miss; the integer bit-width estimate is monotone and
    // sufficient to distinguish "off by k LSB" buckets in the table.)
    let digits = mag.len() as f64;
    (digits * core::f64::consts::LOG2_10).ceil() as u32
}

/// Convert an unsigned decimal-magnitude string to f64 (saturating to
/// `INFINITY` past f64 range). Used for the continuous ULP distance.
pub fn dec_to_f64(mag: &str) -> f64 {
    let mag = mag.trim_start_matches('0');
    if mag.is_empty() {
        return 0.0;
    }
    mag.parse::<f64>().unwrap_or(f64::INFINITY)
}

// ════════════════════════════════════════════════════════════════════
// Golden roster loading
// ════════════════════════════════════════════════════════════════════

/// The golden tables for the transcendental surface, embedded at compile
/// time so the harness is hermetic. Keyed by `"<method>_<infix>"`, e.g.
/// `"sqrt_d38_s19"`. Generated by the `golden_tables!` macro below.
///
/// Only the tiers/methods we need for the comparative sweep are listed.
/// `golden_table(method, width)` returns `None` for an unlisted cell.
pub fn golden_table(method: Method, width: Width) -> Option<&'static str> {
    let stem = method.golden_stem()?;
    let key = (stem, width);
    GOLDEN_TABLES
        .iter()
        .find(|(m, w, _)| *m == stem && *w == width.golden_infix())
        .map(|(_, _, body)| *body)
        .or_else(|| {
            let _ = key;
            None
        })
}

/// Load the golden roster for `(method, width)` into parsed cases.
pub fn golden_roster(method: Method, width: Width) -> Vec<GoldenCase> {
    match golden_table(method, width) {
        None => Vec::new(),
        Some(body) => body.lines().filter_map(parse_golden_line).collect(),
    }
}

macro_rules! golden_tables {
    ( $( $method:literal @ $infix:literal => $file:literal ),+ $(,)? ) => {
        const GOLDEN_TABLES: &[(&str, &str, &str)] = &[
            $( ($method, $infix, include_str!(concat!("../golden/", $file))) ),+
        ];
    };
}

// The transcendental surface across every tier. Arithmetic ops have no
// golden table (exact by construction) and are absent here.
golden_tables! {
    // D38<19> — the canonical comparison tier used by the table below.
    "sqrt"  @ "d38_s19" => "sqrt_d38_s19.txt",
    "cbrt"  @ "d38_s19" => "cbrt_d38_s19.txt",
    "exp"   @ "d38_s19" => "exp_d38_s19.txt",
    "ln"    @ "d38_s19" => "ln_d38_s19.txt",
    "log2"  @ "d38_s19" => "log2_d38_s19.txt",
    "log10" @ "d38_s19" => "log10_d38_s19.txt",
    "exp2"  @ "d38_s19" => "exp2_d38_s19.txt",
    "sin"   @ "d38_s19" => "sin_d38_s19.txt",
    "cos"   @ "d38_s19" => "cos_d38_s19.txt",
    "tan"   @ "d38_s19" => "tan_d38_s19.txt",
    "atan"  @ "d38_s19" => "atan_d38_s19.txt",
    "asin"  @ "d38_s19" => "asin_d38_s19.txt",
    "acos"  @ "d38_s19" => "acos_d38_s19.txt",
    "sinh"  @ "d38_s19" => "sinh_d38_s19.txt",
    "cosh"  @ "d38_s19" => "cosh_d38_s19.txt",
    "tanh"  @ "d38_s19" => "tanh_d38_s19.txt",
    "asinh" @ "d38_s19" => "asinh_d38_s19.txt",
    "acosh" @ "d38_s19" => "acosh_d38_s19.txt",
    "atanh" @ "d38_s19" => "atanh_d38_s19.txt",
    "log"   @ "d38_s19" => "log_d38_s19.txt",
    "atan2" @ "d38_s19" => "atan2_d38_s19.txt",
    "powf"  @ "d38_s19" => "powf_d38_s19.txt",
    // D76<35> — the wide reference tier used by the smoke gate.
    "sqrt"  @ "d76_s35" => "sqrt_d76_s35.txt",
    "cbrt"  @ "d76_s35" => "cbrt_d76_s35.txt",
    "exp"   @ "d76_s35" => "exp_d76_s35.txt",
    "ln"    @ "d76_s35" => "ln_d76_s35.txt",
    "sin"   @ "d76_s35" => "sin_d76_s35.txt",
    "cos"   @ "d76_s35" => "cos_d76_s35.txt",
    "tan"   @ "d76_s35" => "tan_d76_s35.txt",
    "atan"  @ "d76_s35" => "atan_d76_s35.txt",
    // D307<150> — the deep-scale reference tier. Full transcendental
    // surface, the same mpmath golden roster `tests/ulp_strict_golden.rs`
    // proves the kernels correctly-rounded against. Lets the comparative
    // shootout render a deep-scale row where every fixed-precision peer
    // is out of range.
    "sqrt"  @ "d307_s150" => "sqrt_d307_s150.txt",
    "cbrt"  @ "d307_s150" => "cbrt_d307_s150.txt",
    "exp"   @ "d307_s150" => "exp_d307_s150.txt",
    "ln"    @ "d307_s150" => "ln_d307_s150.txt",
    "log2"  @ "d307_s150" => "log2_d307_s150.txt",
    "log10" @ "d307_s150" => "log10_d307_s150.txt",
    "exp2"  @ "d307_s150" => "exp2_d307_s150.txt",
    "sin"   @ "d307_s150" => "sin_d307_s150.txt",
    "cos"   @ "d307_s150" => "cos_d307_s150.txt",
    "tan"   @ "d307_s150" => "tan_d307_s150.txt",
    "atan"  @ "d307_s150" => "atan_d307_s150.txt",
    "asin"  @ "d307_s150" => "asin_d307_s150.txt",
    "acos"  @ "d307_s150" => "acos_d307_s150.txt",
    "sinh"  @ "d307_s150" => "sinh_d307_s150.txt",
    "cosh"  @ "d307_s150" => "cosh_d307_s150.txt",
    "tanh"  @ "d307_s150" => "tanh_d307_s150.txt",
    "asinh" @ "d307_s150" => "asinh_d307_s150.txt",
    "acosh" @ "d307_s150" => "acosh_d307_s150.txt",
    "atanh" @ "d307_s150" => "atanh_d307_s150.txt",
    "log"   @ "d307_s150" => "log_d307_s150.txt",
    "atan2" @ "d307_s150" => "atan2_d307_s150.txt",
    "powf"  @ "d307_s150" => "powf_d307_s150.txt",
}

// ════════════════════════════════════════════════════════════════════
// Comparative runner — sweep (method × width × scale) over subjects
// ════════════════════════════════════════════════════════════════════

/// One scored cell of the shootout: the worst LSBε and worst ULP over the
/// roster, plus how many inputs were scored.
#[derive(Clone, Copy, Default)]
pub struct CellScore {
    pub scored: usize,
    pub na: usize,
    pub max_lsbe: u32,
    pub max_ulp: f64,
    pub correctly_rounded: usize,
}

impl CellScore {
    fn record(&mut self, r: &PrecisionResult) {
        match r {
            PrecisionResult::NotApplicable => self.na += 1,
            PrecisionResult::Executed { lsbe, ulp, .. } => {
                self.scored += 1;
                if *lsbe > self.max_lsbe {
                    self.max_lsbe = *lsbe;
                }
                if *ulp > self.max_ulp {
                    self.max_ulp = *ulp;
                }
                if *lsbe == 0 {
                    self.correctly_rounded += 1;
                }
            }
        }
    }

    /// `true` if every scored input was bit-exact.
    pub fn all_cr(&self) -> bool {
        self.scored > 0 && self.correctly_rounded == self.scored
    }
}

/// Score one subject on one `(method, width)` over the whole golden
/// roster at the tier's canonical scale, under the requested driving
/// `mode`. Returns `None` if no oracle table exists for the cell.
pub fn score_cell(
    subject: &dyn PrecisionSubject,
    method: Method,
    width: Width,
    mode: RoundingMode,
    sample_cap: usize,
) -> Option<CellScore> {
    let roster = golden_roster(method, width);
    if roster.is_empty() {
        return None;
    }
    let scale = width.canonical_scale();
    let mut cell = CellScore::default();
    for case in roster.iter().take(sample_cap) {
        let input = Input {
            raw: case.input.clone(),
            input2: case.input2.clone(),
            width,
            scale,
        };
        let out = subject.eval(method, width, scale, &input, mode);
        let r = Harness::score(&out, case, scale);
        cell.record(&r);
    }
    Some(cell)
}

/// Render a cell in the README/benchmarks precision-table format:
/// `LSBε (ULP)`. A correctly-rounded cell is `0 (0.00)`; `n/a` when the
/// subject never produced a scorable value.
pub fn fmt_cell(cell: Option<CellScore>) -> String {
    match cell {
        None => "n/a".to_string(),
        Some(c) if c.scored == 0 => "n/a".to_string(),
        Some(c) => {
            let ulp = c.max_ulp;
            let ulp_str = if ulp == 0.0 {
                "0.00".to_string()
            } else if ulp < 0.001 {
                format!("{ulp:.1e}")
            } else if ulp < 10.0 {
                format!("{ulp:.2}")
            } else {
                format!("{ulp:.1e}")
            };
            format!("{} ({ulp_str})", c.max_lsbe)
        }
    }
}

/// Emit the comparative shootout table to a `String`. LSBε first, ULP in
/// parens, one row per subject, one column per method, over `widths`.
/// Each cell is the worst (LSBε, ULP) over the roster for that
/// `(subject, method)` at the listed width (the first width that has an
/// oracle table for the method is used per column header).
pub fn render_shootout(
    subjects: &[&dyn PrecisionSubject],
    methods: &[Method],
    width: Width,
    mode: RoundingMode,
    sample_cap: usize,
) -> String {
    use core::fmt::Write;
    let mut out = String::new();

    let _ = writeln!(
        out,
        "## Precision shootout — {} (scale {})\n",
        width.name(),
        width.canonical_scale()
    );
    let _ = writeln!(
        out,
        "Each cell is **LSBε (max |ULP distance to true|)**: LSBε is the \
         bit-width of the gap to the correctly-rounded oracle at the \
         storage scale (0 ⇒ bit-exact, i.e. correctly rounded under the \
         subject's reported mode); ULP is the same gap as a continuous \
         distance in storage LSB. The oracle is rounded under each \
         subject's REPORTED mode before diffing, so a truncating library \
         is not penalised against a half-even one. `n/a` ⇒ the subject \
         cannot represent the (width, scale) or does not expose the \
         method.\n"
    );

    // Header.
    let _ = write!(out, "| subject | mode |");
    for m in methods {
        let _ = write!(out, " {} |", m.name());
    }
    let _ = writeln!(out);
    let _ = write!(out, "|---|---|");
    for _ in methods {
        let _ = write!(out, "---|");
    }
    let _ = writeln!(out);

    // Rows.
    for s in subjects {
        let _ = write!(out, "| {} | {:?} |", s.name(), s.native_mode());
        for &m in methods {
            let cell = score_cell(*s, m, width, mode, sample_cap);
            let _ = write!(out, " {} |", fmt_cell(cell));
        }
        let _ = writeln!(out);
    }
    out
}
