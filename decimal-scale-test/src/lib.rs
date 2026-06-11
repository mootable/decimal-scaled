//! decimal-scaled subjects for the `decimal-scaled-golden` harness.
//!
//! One *erased* [`DsSubject`] carries a `(width, scale, mode)` triple and presents
//! `Value = String`, so the harness pipeline (runner → collectors → validators →
//! reporter) monomorphises ONCE rather than once per concrete `D<SCALE>` cell. The
//! per-cell concrete work — parse, compute, format, and the storage envelope — is
//! reached through a `(width, scale)` match (`cells!`) that fans out to the concrete
//! decimal type only at the two leaf functions ([`compute`], `limits`). The subject
//! lives here (not in a test) so the full-surface gate, the single-cell proof, and
//! `golden-competitors` all share it. [`Filter`] reads the `GOLDEN_*` env vars so a
//! `cargo test` run can target just the cells / modes / functions under investigation.

// The subjects enumerate every decimal tier (D18..D1232), so the crate body
// needs the full wide feature ladder; a narrower build (e.g. the
// default-feature leg of the consolidated test suite) compiles this crate
// empty. The golden gate target pins the same requirement via
// `required-features` in Cargo.toml.
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

// Historical-release subjects for the version-history gates (tests/history.rs);
// each pinned release compiles only behind its `history-*` feature.
#[cfg(any(feature = "history-044", feature = "history-033"))]
pub mod history;

use std::collections::BTreeMap;

use decimal_scaled::{
    DecimalArithmetic, DecimalTranscendental, D115, D1232, D153, D18, D230, D307, D38, D462, D57,
    D616, D76, D924, RoundingMode as DsMode,
};
use decimal_scaled_golden::{
    Capabilities, Computed, DecimalSubject, FnSupport, Function, Limits, Overflow, Radix,
    RoundingMode,
};

/// Generation precision / rounding guard of the golden set (the file `#` header
/// carries the authoritative values; these mirror them for the validators).
pub const GEN_PRECISION: usize = 1233;
pub const GUARD: usize = 2;

/// Every function the golden set covers (a missing file just contributes no cases).
pub const FUNCS: &[Function] = &[
    Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2, Function::Log10,
    Function::Exp2, Function::Sin, Function::Cos, Function::Tan, Function::Atan, Function::Asin,
    Function::Acos, Function::Sinh, Function::Cosh, Function::Tanh, Function::Asinh, Function::Acosh,
    Function::Atanh, Function::Log, Function::Atan2, Function::Powf, Function::Hypot, Function::Add,
    Function::Sub, Function::Mul, Function::Div, Function::Rem,
];

/// Every rounding mode, in report order — directed rounding (Ceiling/Floor/Trunc) is
/// swept alongside the three nearest modes, since a fixed-width decimal rounds its
/// last digit differently per mode and a directed-rounding regression shows only there.
pub const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Ceiling,
    RoundingMode::Floor,
    RoundingMode::Trunc,
];

/// Absolute path to the harness's committed golden set. `env!` is baked at THIS
/// crate's compile time, so the path stays correct no matter which crate calls it.
pub fn golden_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../decimal-scaled-golden/golden")
}

/// Worker-thread cap, honouring `GOLDEN_THREADS` (default = available cores).
pub fn thread_count() -> usize {
    std::env::var("GOLDEN_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1))
}

/// Map the harness rounding mode onto decimal-scaled's.
pub fn ds_mode(m: RoundingMode) -> DsMode {
    match m {
        RoundingMode::HalfToEven => DsMode::HalfToEven,
        RoundingMode::HalfAwayFromZero => DsMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero => DsMode::HalfTowardZero,
        RoundingMode::Ceiling => DsMode::Ceiling,
        RoundingMode::Floor => DsMode::Floor,
        RoundingMode::Trunc => DsMode::Trunc,
    }
}

/// Inherent rounded mul/div aren't on a width-generic trait, so bridge them
/// locally — one delegating impl per width, scale-generic.
pub trait DsOps: Sized {
    fn ds_mul_with(self, o: Self, m: DsMode) -> Self;
    fn ds_div_with(self, o: Self, m: DsMode) -> Self;
}
macro_rules! impl_ds_ops {
    ($($D:ident),+ $(,)?) => { $(
        impl<const S: u32> DsOps for $D<S> {
            fn ds_mul_with(self, o: Self, m: DsMode) -> Self { self.mul_with(o, m) }
            fn ds_div_with(self, o: Self, m: DsMode) -> Self { self.div_with(o, m) }
        }
    )+ };
}
impl_ds_ops!(D18, D38, D57, D76, D115, D153, D230, D307, D462, D616, D924, D1232);

/// The op only (after parse, before format). `d2` is the second operand for binary
/// functions; a missing one is a golden-data fault and panics (the harness records it).
pub fn compute<D>(func: Function, x: D, d2: Option<D>, m: DsMode) -> D
where
    D: DecimalArithmetic + DecimalTranscendental + DsOps + Copy,
{
    let bin = || d2.expect("binary function needs two operands");
    match func {
        Function::Sqrt => x.sqrt_strict_with(m),
        Function::Cbrt => x.cbrt_strict_with(m),
        Function::Exp => x.exp_strict_with(m),
        Function::Ln => x.ln_strict_with(m),
        Function::Log2 => x.log2_strict_with(m),
        Function::Log10 => x.log10_strict_with(m),
        Function::Exp2 => x.exp2_strict_with(m),
        Function::Sin => x.sin_strict_with(m),
        Function::Cos => x.cos_strict_with(m),
        Function::Tan => x.tan_strict_with(m),
        Function::Atan => x.atan_strict_with(m),
        Function::Asin => x.asin_strict_with(m),
        Function::Acos => x.acos_strict_with(m),
        Function::Sinh => x.sinh_strict_with(m),
        Function::Cosh => x.cosh_strict_with(m),
        Function::Tanh => x.tanh_strict_with(m),
        Function::Asinh => x.asinh_strict_with(m),
        Function::Acosh => x.acosh_strict_with(m),
        Function::Atanh => x.atanh_strict_with(m),
        Function::Log => x.log_strict_with(bin(), m),
        Function::Atan2 => x.atan2_strict_with(bin(), m),
        Function::Powf => x.powf_strict_with(bin(), m),
        Function::Hypot => x.hypot_strict_with(bin(), m),
        Function::Add => x + bin(),
        Function::Sub => x - bin(),
        Function::Mul => x.ds_mul_with(bin(), m),
        Function::Div => x.ds_div_with(bin(), m),
        Function::Rem => x % bin(),
    }
}

/// Parse → compute → format at one concrete decimal type `D`. The strict op panics on
/// an out-of-range result; the harness catches that as `Computed::Panic` and judges it
/// against the cell's range. Parse of a harness-vetted (representable) input cannot
/// fail; a failure is a golden-data fault and panics with the offending literal.
fn compute_typed<D>(func: Function, inputs: &[String], m: DsMode) -> Computed<String>
where
    D: DecimalArithmetic
        + DecimalTranscendental
        + DsOps
        + core::str::FromStr
        + core::fmt::Display
        + Copy,
{
    let parse =
        |s: &str| s.parse::<D>().unwrap_or_else(|_| panic!("could not parse representable input {s:?}"));
    let x = parse(&inputs[0]);
    let d2 = inputs.get(1).map(|s| parse(s));
    Computed::Value(compute(func, x, d2, m).to_string())
}

/// The exact storage envelope of one concrete decimal type, in decimal — decimal-scaled's
/// own MIN/MAX constants and its fixed fractional depth. No bit-width math leaks into the
/// harness, and the magnitude envelope + fractional depth bound exactly what it can hold,
/// so no separate significant-figure cap is needed.
fn limits_typed<D>(scale: u32) -> Limits
where
    D: DecimalArithmetic + core::fmt::Display,
{
    Limits {
        min_value: Some(<D as DecimalArithmetic>::MIN.to_string()),
        max_value: Some(<D as DecimalArithmetic>::MAX.to_string()),
        max_precision: scale,
        max_significant_digits: None,
    }
}

/// Enumerate the band-edge `(width, scale)` cells and fan the two leaf operations out
/// to the concrete decimal type for each. `CELLS` is the public cell list; the two
/// dispatch fns are the erased subject's only bridge to a concrete `D<SCALE>`.
macro_rules! cells {
    ($($D:ident < $s:literal > => $w:literal),+ $(,)?) => {
        /// Every band-edge `(width, scale)` cell the full-surface gate enumerates.
        pub const CELLS: &[(u32, u32)] = &[ $( ($w, $s) ),+ ];

        fn dispatch_compute(
            width: u32, scale: u32, func: Function, inputs: &[String], m: DsMode,
        ) -> Computed<String> {
            match (width, scale) {
                $( ($w, $s) => compute_typed::<$D<$s>>(func, inputs, m), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }

        fn dispatch_limits(width: u32, scale: u32) -> Limits {
            match (width, scale) {
                $( ($w, $s) => limits_typed::<$D<$s>>($s), )+
                _ => panic!("no decimal-scaled cell for (width={width}, scale={scale})"),
            }
        }
    };
}

cells! {
    // D18 — Int<1>, 64-bit storage
    D18<0> => 18, D18<3> => 18, D18<4> => 18, D18<9> => 18, D18<13> => 18, D18<17> => 18,
    // D38 — Int<2>, 128-bit
    D38<0> => 38, D38<2> => 38, D38<6> => 38, D38<9> => 38, D38<10> => 38, D38<12> => 38,
    D38<17> => 38, D38<18> => 38, D38<19> => 38, D38<28> => 38, D38<37> => 38,
    // D57 — Int<3>, 192-bit
    D57<0> => 57, D57<14> => 57, D57<20> => 57, D57<28> => 57, D57<30> => 57, D57<42> => 57,
    D57<56> => 57,
    // D76 — Int<4>, 256-bit
    D76<0> => 76, D76<18> => 76, D76<19> => 76, D76<38> => 76, D76<40> => 76, D76<57> => 76,
    D76<75> => 76,
    // D115 — Int<6>, 384-bit
    D115<0> => 115, D115<28> => 115, D115<50> => 115, D115<57> => 115, D115<86> => 115,
    D115<114> => 115,
    // D153 — Int<8>, 512-bit
    D153<0> => 153, D153<38> => 153, D153<76> => 153, D153<114> => 153, D153<152> => 153,
    // D230 — Int<12>, 768-bit
    D230<0> => 230, D230<57> => 230, D230<115> => 230, D230<172> => 230, D230<229> => 230,
    // D307 — Int<16>, 1024-bit (s290: the ln lookup band s285-295)
    D307<0> => 307, D307<30> => 307, D307<50> => 307, D307<70> => 307, D307<76> => 307,
    D307<120> => 307, D307<153> => 307, D307<230> => 307, D307<290> => 307, D307<306> => 307,
    // D462 — Int<24>, 1536-bit
    D462<0> => 462, D462<30> => 462, D462<100> => 462, D462<115> => 462, D462<180> => 462,
    D462<231> => 462, D462<346> => 462, D462<461> => 462,
    // D616 — Int<32>, 2048-bit (s590: the ln lookup band s585-595)
    D616<0> => 616, D616<30> => 616, D616<130> => 616, D616<154> => 616, D616<240> => 616,
    D616<308> => 616, D616<462> => 616, D616<590> => 616, D616<615> => 616,
    // D924 — Int<48>, 3072-bit (s900: the ln lookup band s895-905)
    D924<0> => 924, D924<30> => 924, D924<180> => 924, D924<231> => 924, D924<350> => 924,
    D924<462> => 924, D924<693> => 924, D924<900> => 924, D924<923> => 924,
    // D1232 — Int<64>, 4096-bit (s1200: the ln lookup band s1195-1205)
    D1232<0> => 1232, D1232<30> => 1232, D1232<250> => 1232, D1232<308> => 1232,
    D1232<470> => 1232, D1232<616> => 1232, D1232<924> => 1232, D1232<1200> => 1232,
    D1232<1231> => 1232,
}

/// One erased decimal-scaled subject: a `(width, scale)` cell tested under one rounding
/// `mode`. `Value = String`, so the whole harness pipeline monomorphises once; the
/// concrete decimal type is reached only via the `(width, scale)` dispatch. The triple
/// also rides in `Capabilities::config` as report metadata.
#[derive(Clone, Copy)]
pub struct DsSubject {
    pub width: u32,
    pub scale: u32,
    pub mode: RoundingMode,
}

impl DsSubject {
    /// A subject tested under the default half-to-even rounding.
    pub fn new(width: u32, scale: u32) -> DsSubject {
        DsSubject::with_mode(width, scale, RoundingMode::HalfToEven)
    }

    /// A subject tested under a specific rounding `mode` — the full set is swept by the
    /// `golden_all_modes` gate so directed rounding (Ceiling/Floor/Trunc) is covered,
    /// not just the default.
    pub fn with_mode(width: u32, scale: u32, mode: RoundingMode) -> DsSubject {
        DsSubject { width, scale, mode }
    }
}

impl DecimalSubject for DsSubject {
    type Value = String;

    fn name(&self) -> String {
        format!("decimal-scaled D{}<{}> {:?}", self.width, self.scale, self.mode)
    }

    fn capabilities(&self) -> Capabilities {
        // decimal-scaled's overflow contract is to PANIC on an out-of-range result,
        // in BOTH debug and release, for every default op — transcendental AND
        // arithmetic (a fixed-width decimal has no infinity/NaN). The opt-out
        // `wrapping_`/`checked_`/`saturating_` variants are not the path tested here.
        let mut functions = BTreeMap::new();
        for &f in FUNCS {
            functions.insert(f, FnSupport { mode: self.mode, overflow: Overflow::Panic });
        }
        let mut config = BTreeMap::new();
        config.insert("width".into(), self.width.to_string());
        config.insert("scale".into(), self.scale.to_string());
        config.insert("mode".into(), format!("{:?}", self.mode));
        Capabilities { name: "decimal-scaled".into(), radix: Radix::Decimal, config, functions }
    }

    fn string_to_value(&self, s: &str) -> String {
        // Erased: the value carried between parse and compute is the literal itself;
        // the concrete parse happens inside `execute` at the dispatched type.
        s.to_string()
    }

    fn value_to_string(&self, v: &String) -> String {
        v.clone()
    }

    fn limits(&self, _value: &str) -> Limits {
        dispatch_limits(self.width, self.scale)
    }

    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[String]) -> Computed<String> {
        let (width, scale, m) = (self.width, self.scale, ds_mode(mode));
        // The strict op panics on overflow; the harness catches that as
        // `Computed::Panic` (a test failure judged against the cell's range).
        move |inputs| dispatch_compute(width, scale, func, inputs, m)
    }
}

/// Command-line filter over the full surface, read from the `GOLDEN_*` env vars so a
/// `cargo test` run can target just the cells / modes / functions under investigation:
///
/// ```text
/// GOLDEN_WIDTHS=38,1232  GOLDEN_SCALES=0,19   # subset the (width, scale) cells
/// GOLDEN_MODES=ceiling,floor                  # subset the rounding modes
/// GOLDEN_FUNCS=exp,cosh                       # subset the functions
/// GOLDEN_SAMPLE=50                            # keep ~1-in-50 golden rows (+ the edges)
/// GOLDEN_STRIPE=3/20                          # keep rows where line % 20 == 3 (a CI shard)
/// ```
///
/// An unset (or empty) list means "all"; `GOLDEN_SAMPLE` defaults to 1 (keep every row).
/// `GOLDEN_STRIPE=k/n` partitions the rows round-robin across `n` parallel jobs: the
/// stripes are disjoint and their union is exactly the full set (unlike `GOLDEN_SAMPLE`,
/// which keeps a subset), so a striped fleet still checks every row. The modulus `n` is
/// free — CI passes it from one workflow-level variable so the shard count is tunable.
pub struct Filter {
    widths: Option<Vec<u32>>,
    scales: Option<Vec<u32>>,
    modes: Option<Vec<RoundingMode>>,
    funcs: Vec<Function>,
    sample: usize,
    stripe: Option<(usize, usize)>,
}

impl Filter {
    /// Read the `GOLDEN_*` env vars into a filter. An unrecognised list token is
    /// SKIPPED with a stderr warning (never a silent drop — a typo'd
    /// `GOLDEN_FUNCS=copsh` would otherwise quietly widen or empty the run).
    pub fn from_env() -> Filter {
        Filter {
            widths: env_list("GOLDEN_WIDTHS", |t| t.parse::<u32>().ok()),
            scales: env_list("GOLDEN_SCALES", |t| t.parse::<u32>().ok()),
            modes: env_list("GOLDEN_MODES", parse_mode),
            funcs: env_list("GOLDEN_FUNCS", parse_func).unwrap_or_else(|| FUNCS.to_vec()),
            sample: env_nonempty("GOLDEN_SAMPLE")
                .and_then(|s| s.trim().parse().ok())
                .filter(|&n| n >= 1)
                .unwrap_or(1),
            stripe: env_nonempty("GOLDEN_STRIPE").and_then(|s| parse_stripe(&s)),
        }
    }

    /// The `(width, scale)` cells passing the width/scale filters.
    pub fn cells(&self) -> Vec<(u32, u32)> {
        CELLS
            .iter()
            .copied()
            .filter(|(w, _)| self.widths.as_ref().is_none_or(|ws| ws.contains(w)))
            .filter(|(_, s)| self.scales.as_ref().is_none_or(|ss| ss.contains(s)))
            .collect()
    }

    /// The modes to sweep, falling back to `default` when `GOLDEN_MODES` is unset.
    pub fn modes(&self, default: &[RoundingMode]) -> Vec<RoundingMode> {
        self.modes.clone().unwrap_or_else(|| default.to_vec())
    }

    /// The functions to run (all, unless `GOLDEN_FUNCS` subsets them).
    pub fn funcs(&self) -> &[Function] {
        &self.funcs
    }

    /// The 1-in-`n` golden-row sampling factor (1 = keep every row).
    pub fn sample(&self) -> usize {
        self.sample
    }

    /// The `(k, n)` row stripe (`GOLDEN_STRIPE=k/n`), or `None` for every row.
    pub fn stripe(&self) -> Option<(usize, usize)> {
        self.stripe
    }
}

/// Parse `"k/n"` with `n >= 1` and `k < n`; anything else is rejected with a stderr
/// warning (an out-of-range stripe would silently run zero rows).
fn parse_stripe(s: &str) -> Option<(usize, usize)> {
    let (k, n) = s.trim().split_once('/')?;
    let parsed = match (k.trim().parse::<usize>(), n.trim().parse::<usize>()) {
        (Ok(k), Ok(n)) if n >= 1 && k < n => Some((k, n)),
        _ => None,
    };
    if parsed.is_none() {
        eprintln!("GOLDEN_STRIPE: ignoring {s:?} (expected k/n with k < n)");
    }
    parsed
}

/// Parse a rounding-mode name (case-/separator-insensitive, with the common aliases).
pub fn parse_mode(s: &str) -> Option<RoundingMode> {
    use RoundingMode::*;
    match s.trim().to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
        "half_to_even" | "halftoeven" | "even" | "bankers" | "nearest" => Some(HalfToEven),
        "half_away_from_zero" | "halfawayfromzero" | "half_away" | "away" | "half_up" => {
            Some(HalfAwayFromZero)
        }
        "half_toward_zero" | "halftowardzero" | "half_toward" | "toward" | "half_down" => {
            Some(HalfTowardZero)
        }
        "ceiling" | "ceil" | "up" => Some(Ceiling),
        "floor" | "down" => Some(Floor),
        "trunc" | "truncate" | "zero" => Some(Trunc),
        _ => None,
    }
}

/// Parse a function name against the canonical golden-file names (e.g. `"exp"`, `"atan2"`).
pub fn parse_func(s: &str) -> Option<Function> {
    let s = s.trim().to_ascii_lowercase();
    FUNCS.iter().copied().find(|f| f.name() == s)
}

/// A non-empty, non-blank env var value, or `None`.
fn env_nonempty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.trim().is_empty())
}

/// A comma-separated list from `key` parsed item-wise, or `None` when the var is
/// unset/blank. An unparseable token is skipped WITH a stderr warning naming the
/// variable and the token.
fn env_list<T>(key: &str, parse: impl Fn(&str) -> Option<T>) -> Option<Vec<T>> {
    env_nonempty(key).map(|s| {
        s.split(',')
            .filter_map(|t| {
                let token = t.trim();
                let v = parse(token);
                if v.is_none() {
                    eprintln!("{key}: ignoring unrecognised token {token:?}");
                }
                v
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_enumerates_every_band_edge() {
        // The 88 band-edge cells across the 12 widths, plus the four ln-lookup
        // band cells (D307<290>, D616<590>, D924<900>, D1232<1200>); spot-check
        // the count and edges.
        assert_eq!(CELLS.len(), 92);
        assert_eq!(CELLS.first(), Some(&(18, 0)));
        assert_eq!(CELLS.last(), Some(&(1232, 1231)));
    }

    #[test]
    fn dispatch_round_trips_a_known_cell() {
        // sqrt(2) at D38<19> under half-to-even, via the erased dispatch: a 19-dp
        // value on the right prefix (not pinned digit-for-digit, to stay robust).
        let out = dispatch_compute(38, 19, Function::Sqrt, &["2".to_string()], DsMode::HalfToEven);
        match out {
            Computed::Value(v) => {
                assert!(v.starts_with("1.41421356237309"), "got {v}");
                assert_eq!(v.split_once('.').unwrap().1.len(), 19, "19 fractional digits");
            }
            other => panic!("expected Value, got {other:?}"),
        }
    }

    #[test]
    fn limits_report_the_concrete_envelope() {
        let lim = dispatch_limits(38, 19);
        assert_eq!(lim.max_precision, 19);
        assert!(lim.min_value.as_deref().unwrap().starts_with('-'));
        assert!(lim.max_value.is_some());
    }

    #[test]
    fn mode_aliases_parse() {
        assert_eq!(parse_mode("Ceiling"), Some(RoundingMode::Ceiling));
        assert_eq!(parse_mode("half-away"), Some(RoundingMode::HalfAwayFromZero));
        assert_eq!(parse_mode("trunc"), Some(RoundingMode::Trunc));
        assert_eq!(parse_mode("nonsense"), None);
    }

    #[test]
    fn func_names_parse() {
        assert_eq!(parse_func("exp"), Some(Function::Exp));
        assert_eq!(parse_func("ATAN2"), Some(Function::Atan2));
        assert_eq!(parse_func("nope"), None);
    }
}
