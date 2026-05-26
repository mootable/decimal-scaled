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

    /// Reverse of [`Method::golden_stem`]: map a golden-table filename
    /// stem (e.g. `"exp"`, `"powf"`) back to its [`Method`]. `None` for
    /// an unrecognised stem (so the scanner skips files it can't map).
    pub fn from_stem(stem: &str) -> Option<Method> {
        Method::TRANSCENDENTAL
            .iter()
            .copied()
            .find(|m| m.golden_stem() == Some(stem))
    }
}

// ════════════════════════════════════════════════════════════════════
// Width — the twelve decimal tiers D9 … D1232
// ════════════════════════════════════════════════════════════════════

/// One of the crate's twelve decimal width tiers. Each tier pins a
/// storage bit-width and a canonical scale (the scale the golden tables
/// were generated at — `SCALE = floor((digits-1)/2)` for that tier).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Width {
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
    pub const ALL: [Width; 12] = [
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

    /// Reverse of [`Width::digits`]: map a decimal digit capacity (the
    /// `N` parsed from a `d<N>` golden infix) back to its tier. `None`
    /// for an `N` that is not one of the twelve tiers.
    pub fn from_digits(n: u32) -> Option<Width> {
        Width::ALL.iter().copied().find(|w| w.digits() == n)
    }

    /// The canonical scale the golden tables use for this tier.
    pub fn canonical_scale(self) -> u32 {
        match self {
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
        /// Significant DECIMAL digit-length of `|value − oracle_cr|`. `0`
        /// ⇒ bit-exact. This is the count of contaminated trailing decimal
        /// digits in the last-place error and is what the fidelity grade
        /// scores (an error of `100000` ⇒ `6`), distinct from `lsbe` (its
        /// bit-width).
        digits: u32,
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
        // Decimal-digit length of the error magnitude BEFORE bit-conversion
        // — what the fidelity grade scores (exact contaminated-digit count).
        let digits = dec_digit_len(&diff);
        let ulp = dec_to_f64(&diff);

        PrecisionResult::Executed {
            value: value.clone(),
            rounding,
            lsbe,
            digits,
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
        if t.is_empty() {
            "0".to_string()
        } else {
            t.to_string()
        }
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
            RoundingMode::Floor => neg, // toward -inf: negatives round away
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

    let mag = if round_up {
        dec_add_mag(&kept, "1")
    } else {
        kept
    };
    let mag = {
        let t = mag.trim_start_matches('0');
        if t.is_empty() {
            "0".to_string()
        } else {
            t.to_string()
        }
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

/// Significant DECIMAL digit length of a decimal-magnitude string. Sibling
/// of [`dec_bit_len`], but counts base-10 digits rather than bits: strip
/// any sign and leading zeros, then return the remaining digit count.
/// `0` ⇒ exactly zero (correctly rounded). This is the EXACT count of
/// contaminated trailing decimal digits in a last-place error — e.g. an
/// error magnitude of `100000` is `6` decimal digits.
pub fn dec_digit_len(mag: &str) -> u32 {
    let mag = mag.strip_prefix('-').unwrap_or(mag);
    let mag = mag.strip_prefix('+').unwrap_or(mag);
    let mag = mag.trim_start_matches('0');
    if mag.is_empty() {
        0
    } else {
        mag.len() as u32
    }
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

/// Load the golden roster for `(method, width)` into parsed cases, at the
/// width's canonical scale, from the compile-time embedded tables. Kept
/// for back-compat callers; the comparative sweep uses the runtime
/// scanner ([`golden_scan`] + [`golden_roster_at`]) so it picks up EVERY
/// `(method, width, scale)` file on disk, not just the embedded subset.
pub fn golden_roster(method: Method, width: Width) -> Vec<GoldenCase> {
    match golden_table(method, width) {
        None => Vec::new(),
        Some(body) => body.lines().filter_map(parse_golden_line).collect(),
    }
}

// ════════════════════════════════════════════════════════════════════
// Runtime golden scanner — pick up EVERY (method, width, scale) on disk
// ════════════════════════════════════════════════════════════════════
//
// The embedded `golden_tables!` roster above only lists the handful of
// cells the historic table needed (~3 widths at 1 scale). `tests/golden/`
// actually holds the full mpmath surface: 22 transcendental methods over
// 12 width tiers, several scales each (narrow tiers s0/s_mid/s_max, wide
// tiers s0/s30/.../s_max). The comparative sweep scores ALL of them by
// scanning the directory at runtime and reverse-mapping each filename
// `<stem>_d<N>_s<scale>.txt` to `(Method, Width, scale)`.

/// One scanned golden cell: a method/width/scale triple with the absolute
/// path to its table file.
#[derive(Clone, Debug)]
pub struct GoldenCell {
    pub method: Method,
    pub width: Width,
    pub scale: u32,
    pub path: std::path::PathBuf,
}

/// The `tests/golden` directory, resolved from `CARGO_MANIFEST_DIR` (the
/// crate root regardless of where `cargo bench`/`cargo test` is invoked).
pub fn golden_dir() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("golden");
    p
}

/// Parse a golden filename stem `"<method>_d<N>_s<scale>"` (the file's
/// name without the `.txt` extension) into `(Method, Width, scale)`.
/// `None` if any component is unrecognised — callers skip such files.
pub fn parse_golden_filename(stem: &str) -> Option<(Method, Width, u32)> {
    // Split off the trailing `_s<scale>`.
    let (head, scale_s) = stem.rsplit_once("_s")?;
    let scale: u32 = scale_s.parse().ok()?;
    // Split off the `_d<N>` infix.
    let (method_s, digits_s) = head.rsplit_once("_d")?;
    let digits: u32 = digits_s.parse().ok()?;
    let method = Method::from_stem(method_s)?;
    let width = Width::from_digits(digits)?;
    Some((method, width, scale))
}

/// Scan `tests/golden/*.txt` and return every recognised
/// `(method, width, scale)` cell, sorted deterministically by
/// (method canonical order, width narrow→wide, scale ascending). This is
/// the auto-pickup roster the comparative sweep iterates — it grows
/// automatically as golden files are added, with no hardcoded list.
pub fn golden_scan() -> Vec<GoldenCell> {
    let dir = golden_dir();
    let mut cells: Vec<GoldenCell> = Vec::new();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return cells;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if let Some((method, width, scale)) = parse_golden_filename(stem) {
            cells.push(GoldenCell {
                method,
                width,
                scale,
                path,
            });
        }
    }
    // Deterministic order: method (canonical), width (narrow→wide), scale.
    let method_rank = |m: Method| Method::ALL.iter().position(|x| *x == m).unwrap_or(usize::MAX);
    let width_rank = |w: Width| Width::ALL.iter().position(|x| *x == w).unwrap_or(usize::MAX);
    cells.sort_by(|a, b| {
        method_rank(a.method)
            .cmp(&method_rank(b.method))
            .then(width_rank(a.width).cmp(&width_rank(b.width)))
            .then(a.scale.cmp(&b.scale))
    });
    cells
}

/// Load and parse the golden roster from an explicit table file path (a
/// runtime-scanned cell). Empty vec if the file can't be read.
pub fn golden_roster_at_path(path: &std::path::Path) -> Vec<GoldenCase> {
    match std::fs::read_to_string(path) {
        Ok(body) => body.lines().filter_map(parse_golden_line).collect(),
        Err(_) => Vec::new(),
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
    /// Inputs on which the subject PANICKED (counted as not-applicable for
    /// scoring, but tracked separately so a kernel/library crash on an
    /// edge input is surfaced, not silently swallowed).
    pub panicked: usize,
    pub max_lsbe: u32,
    /// Worst DECIMAL digit-length of the error magnitude over the roster
    /// — what the fidelity grade scores (the exact contaminated-digit
    /// count, not the bit-width `max_lsbe`).
    pub max_digits: u32,
    pub max_ulp: f64,
    pub correctly_rounded: usize,
}

impl CellScore {
    fn record(&mut self, r: &PrecisionResult) {
        match r {
            PrecisionResult::NotApplicable => self.na += 1,
            PrecisionResult::Executed {
                lsbe, digits, ulp, ..
            } => {
                self.scored += 1;
                if *lsbe > self.max_lsbe {
                    self.max_lsbe = *lsbe;
                }
                if *digits > self.max_digits {
                    self.max_digits = *digits;
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

/// Score one subject on a pre-parsed golden `roster` at an EXPLICIT
/// `(method, width, scale)`, under the requested driving `mode`. The
/// scale is passed in rather than derived from the width so the same cell
/// can be scored at every scale a tier has a golden file for (s0, s30,
/// …, s_max), not only the canonical one. Returns `None` for an empty
/// roster.
pub fn score_roster(
    subject: &dyn PrecisionSubject,
    method: Method,
    width: Width,
    scale: u32,
    roster: &[GoldenCase],
    mode: RoundingMode,
    sample_cap: usize,
) -> Option<CellScore> {
    if roster.is_empty() {
        return None;
    }
    let mut cell = CellScore::default();
    for case in roster.iter().take(sample_cap) {
        let input = Input {
            raw: case.input.clone(),
            input2: case.input2.clone(),
            width,
            scale,
        };
        // A subject may PANIC on an edge input (e.g. a divide-by-zero in a
        // kernel, or a peer library's domain assert). Isolate each cell so
        // one crash does not abort the whole sweep; a panic is recorded as
        // not-applicable for scoring but counted in `panicked` so the
        // caller can surface it.
        let evaluated = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            subject.eval(method, width, scale, &input, mode)
        }));
        match evaluated {
            Ok(out) => {
                let r = Harness::score(&out, case, scale);
                cell.record(&r);
            }
            Err(_) => {
                cell.panicked += 1;
                cell.na += 1;
            }
        }
    }
    Some(cell)
}

/// Score one subject on one runtime-scanned [`GoldenCell`] (it carries
/// the method/width/scale and the table path). The auto-pickup entry
/// point for the comparative sweep. Returns `None` if the table is empty.
pub fn score_scanned_cell(
    subject: &dyn PrecisionSubject,
    cell: &GoldenCell,
    mode: RoundingMode,
    sample_cap: usize,
) -> Option<CellScore> {
    let roster = golden_roster_at_path(&cell.path);
    score_roster(
        subject,
        cell.method,
        cell.width,
        cell.scale,
        &roster,
        mode,
        sample_cap,
    )
}

/// Score one subject on one `(method, width)` over the embedded golden
/// roster at the tier's canonical scale, under the requested driving
/// `mode`. Back-compat wrapper over [`score_roster`]; the comparative
/// sweep now uses [`score_scanned_cell`] over [`golden_scan`]. Returns
/// `None` if no oracle table exists for the cell.
pub fn score_cell(
    subject: &dyn PrecisionSubject,
    method: Method,
    width: Width,
    mode: RoundingMode,
    sample_cap: usize,
) -> Option<CellScore> {
    let roster = golden_roster(method, width);
    let scale = width.canonical_scale();
    score_roster(subject, method, width, scale, &roster, mode, sample_cap)
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

// ════════════════════════════════════════════════════════════════════
// Fidelity grading — per-function + overall two-letter grade / 0–100 score
// ════════════════════════════════════════════════════════════════════
//
// The owner's locked rubric (published alongside the tables):
//
//   * `incorrect_digits` = the EXACT decimal-digit length of the cell's
//     worst error magnitude (`dec_digit_len` of the decimal `diff`, BEFORE
//     it is converted to bits) — the count of contaminated trailing
//     decimal digits in the last-place error. `0` ⇒ correctly rounded.
//     (e.g. an error magnitude of `100000` ⇒ `incorrect_digits = 6`.) This
//     is NOT `floor(log10(max_lsbe)) + 1`: `max_lsbe` is a BIT length, so
//     that formula would score the `100000` error as 2 rather than 6.
//   * per-cell demerit, RELATIVE to the tier's precision:
//       `d = 0` if correctly rounded (`incorrect_digits == 0`),
//       else `d = min(10, ceil(incorrect_digits / tier_width * 10))`,
//     where `tier_width` is the tier's decimal digit capacity (`D307` ⇒
//     307). An N-digit miss in a `tier_width`-digit number costs
//     `ceil(N/tier_width * 10)` — ceil so ANY miss is ≥ 1, capped at 10 so
//     a catastrophic / overflow error caps at 10. Each cell therefore needs
//     its OWN tier width (a function spans many widths).
//   * `mean_demerit = Σ d / executed_cells` — pooled over a function's
//     width×scale cells (per function) and over all cells (per library).
//     `n/a` cells (function or scale the library can't run) are excluded
//     from BOTH the count and the demerit sum.
//   * `score (closeness) = 100 * (1 − mean_demerit/10)` ∈ [0,100]; 100 only
//     when every cell is correctly rounded.
//   * `%CR (reliability) = 100 * correct_cells / executed_cells` — the
//     fraction of cells that are EXACTLY correctly rounded.
//   * grade bands (applied to BOTH score and %CR):
//       A  == 100 (exactly)
//       B  [95, 100)
//       C  [85, 95)
//       D  [70, 85)
//       E  [50, 70)
//       F  < 50
//   * the headline per library is a TWO-LETTER grade
//     `grade(score)` followed by `grade(%CR)` (1st = how close, 2nd = how
//     reliable), e.g. `AA`, `BC`, `CF`.

/// `incorrect_digits` for a cell: the EXACT decimal-digit length of its
/// worst error magnitude (the cell's `max_digits`, i.e. `dec_digit_len` of
/// the decimal error `diff`). `0` ⇒ correctly rounded. An identity pass —
/// the digit-length is computed at scoring time, not re-derived from the
/// bit-width — but kept as a named seam so the rubric maps to one place.
pub fn incorrect_digits(max_digits: u32) -> u32 {
    max_digits
}

/// Per-cell demerit `d`, RELATIVE to the tier's precision: `0` when
/// correctly rounded (`max_digits == 0`), else
/// `min(10, ceil(incorrect_digits / tier_width * 10))`, where `tier_width`
/// is the tier's decimal digit capacity (e.g. `D307` ⇒ `307`). Ceil so any
/// miss costs ≥ 1; capped at 10 so a catastrophic / overflow error = 10.
pub fn cell_demerit(max_digits: u32, tier_width: u32) -> f64 {
    if max_digits == 0 {
        0.0
    } else {
        let tw = tier_width.max(1) as f64;
        let raw = (incorrect_digits(max_digits) as f64 / tw * 10.0).ceil();
        raw.min(10.0)
    }
}

/// The 0–100 closeness score for a `mean_demerit`:
/// `100 * (1 − mean_demerit/10)`, clamped to `[0, 100]`.
pub fn score_for_mean_demerit(mean_demerit: f64) -> f64 {
    (100.0 * (1.0 - mean_demerit / 10.0)).clamp(0.0, 100.0)
}

/// The A–F letter grade for a 0–100 metric (applies to BOTH the closeness
/// score and the %CR), in the owner's bands:
/// A = 100 exactly · B = [95,100) · C = [85,95) · D = [70,85) ·
/// E = [50,70) · F = < 50.
pub fn grade_for_score(s: f64) -> char {
    if s >= 100.0 {
        'A'
    } else if s >= 95.0 {
        'B'
    } else if s >= 85.0 {
        'C'
    } else if s >= 70.0 {
        'D'
    } else if s >= 50.0 {
        'E'
    } else {
        'F'
    }
}

/// Running fidelity accumulator over a set of cells: total demerits, the
/// executed-cell count, and the correctly-rounded-cell count.
#[derive(Clone, Copy, Default)]
pub struct Fidelity {
    pub demerits: f64,
    pub cells: usize,
    pub correct: usize,
}

impl Fidelity {
    /// Fold one EXECUTED cell into the accumulator: its worst decimal
    /// error-digit length (`max_digits`) and the cell's tier width (needed
    /// for the precision-relative demerit). `n/a` cells must NOT be passed.
    pub fn record(&mut self, max_digits: u32, tier_width: u32) {
        self.demerits += cell_demerit(max_digits, tier_width);
        self.cells += 1;
        if max_digits == 0 {
            self.correct += 1;
        }
    }

    /// `mean_demerit = Σ d / executed_cells` (`0` when no cells ran).
    pub fn mean_demerit(&self) -> f64 {
        if self.cells == 0 {
            0.0
        } else {
            self.demerits / self.cells as f64
        }
    }

    /// Closeness score `100 * (1 − mean_demerit/10)` ∈ [0,100].
    pub fn score(&self) -> f64 {
        score_for_mean_demerit(self.mean_demerit())
    }

    /// Reliability `%CR = 100 * correct_cells / executed_cells`.
    pub fn cr_pct(&self) -> f64 {
        if self.cells == 0 {
            0.0
        } else {
            100.0 * self.correct as f64 / self.cells as f64
        }
    }

    /// Letter grade of the closeness score.
    pub fn grade(&self) -> char {
        grade_for_score(self.score())
    }

    /// Letter grade of the %CR.
    pub fn cr_grade(&self) -> char {
        grade_for_score(self.cr_pct())
    }

    /// The two-letter headline grade: `grade(score)` then `grade(%CR)`.
    pub fn two_letter(&self) -> String {
        format!("{}{}", self.grade(), self.cr_grade())
    }
}

/// The published rubric text, emitted alongside every grade table so the
/// grading is auditable from the report itself.
pub fn fidelity_rubric() -> &'static str {
    "### Fidelity grading rubric\n\
     \n\
     Each scored cell is one `(method, width, scale)` golden table; its \
     worst error is the largest gap to the correctly-rounded oracle over \
     the cell's inputs (`0` ⇒ bit-exact / correctly rounded under the \
     library's own reported rounding mode). `n/a` cells (a function or \
     scale a library cannot run) are excluded from BOTH the cell count and \
     the demerit sum.\n\
     \n\
     * **error_digits** = the EXACT decimal-digit length of that worst \
     error magnitude (the count of contaminated trailing decimal digits; \
     e.g. an error of `100000` ⇒ `6`), `0` when correctly rounded. \
     Measured on the decimal error itself, not its bit-width.\n\
     * **per-cell demerit** (precision-relative): `d = 0` if correctly \
     rounded, else `d = min(10, ceil(error_digits / tier_width × 10))`, \
     where `tier_width` is the tier's decimal digit capacity (`D307` ⇒ \
     307). Ceil ⇒ any miss costs ≥ 1; capped at 10 so a catastrophic / \
     overflow error = 10. So a 3-digit miss in a 307-digit number barely \
     registers, while a gross miss saturates.\n\
     * **mean_demerit** `= Σ d / executed_cells`, pooled over a function's \
     width×scale cells (per function) and over all cells (per library).\n\
     \n\
     **Two scores, because they measure different things.**\n\
     \n\
     * **score (closeness)** `= 100·(1 − mean_demerit/10)` ∈ [0,100], \
     severity-weighted: how close the results are on average, *relative to \
     the tier's precision*. A cell wrong by a few digits in a 307-digit \
     number barely dents it; a catastrophic cell (demerit capped at 10) \
     hits hard. 100 only when every cell is correctly rounded.\n\
     * **%CR (reliability)** `= 100·correct_cells / executed_cells` — the \
     fraction of cells that are *exactly* correctly rounded (bit-exact \
     under the library's own reported rounding mode). It counts how *often* \
     the library is exactly right, ignoring how small the misses are.\n\
     \n\
     They diverge for libraries that are *frequently but slightly* wrong: \
     a library bit-exact only ~29% of the time (low %CR) can still score \
     ~85 (high closeness) if its misses are tiny fractions of wide tiers.\n\
     \n\
     * **grade bands** (applied to BOTH score and %CR): A `= 100` · \
     B `[95,100)` · C `[85,95)` · D `[70,85)` · E `[50,70)` · F `< 50`.\n\
     * **headline** = a TWO-LETTER grade `grade(score)·grade(%CR)` (e.g. \
     `AA`, `BC`, `CF`): 1st letter = how close, 2nd = how reliable. Each \
     library is graded over its **runnable** cells only; coverage \
     (runnable / total) is published so broader libraries — structurally \
     more exposed to demerits — are auditable.\n"
}
