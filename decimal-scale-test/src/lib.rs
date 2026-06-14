// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! decimal-scaled subjects for the `decimal-scaled-golden` harness.
//!
//! One *erased* [`DsSubject`] carries a `(width, scale, mode)` triple and presents
//! `Value = String`, so the harness pipeline (runner → collectors → validators →
//! reporter) monomorphises ONCE rather than once per concrete `D<SCALE>` cell. The
//! per-cell concrete work — parse, compute, format, and the storage envelope — is
//! the `decimal-scaled-cells` crate's `(width, scale)` fan-out (`cells!`), which
//! this subject reaches through its two concrete, non-generic shim entry points
//! (`dispatch_compute` / `dispatch_limits`). That placement is deliberate: the
//! fan-out's heavy monomorphisations compile ONCE in the cells leaf rlib instead
//! of once per consuming target (this lib's rlib AND its unit-test harness used
//! to pay the full bill each). The subject lives here (not in a test) so the
//! full-surface gate, the single-cell proof, and `golden-competitors` all share
//! it. [`Filter`] reads the `GOLDEN_*` env vars so a `cargo test` run can target
//! just the cells / modes / functions under investigation.

// The cells fan-out is per-tier feature-gated (this crate forwards each tier
// feature to decimal-scaled-cells), so the crate compiles at ANY feature set:
// a default (narrow) build carries the D18/D38 cells and the golden gate
// sweeps just those; wider builds add their tiers' cells. `Filter::cells()`
// drops cells whose tier is not compiled, so CELLS itself stays the full grid
// as data.

// Historical-release subjects for the version-history gates (tests/history.rs);
// relocated to decimal-scaled-cells (the same compile-once placement), each
// pinned release compiling only behind its `history-*` feature.
#[cfg(any(feature = "history-044", feature = "history-033"))]
pub use decimal_scaled_cells::history;

use std::collections::BTreeMap;

use decimal_scaled::RoundingMode as DsMode;
use decimal_scaled_golden::{
    Capabilities, Computed, DecimalSubject, FnSupport, Function, Limits, Overflow, Radix,
    RoundingMode,
};

// The cell-shim surface this crate's subjects and consumers were built on:
// the full band-edge grid (`CELLS` + `tier_compiled`), the function list, and
// the typed op bridge (`compute` + `DsOps`) — re-exported so existing
// consumers (golden-competitors, the history gates) keep their import paths.
pub use decimal_scaled_cells::{
    compute, tier_compiled, DsOps, CELLS, COMPARE_SCALES, FUNCS, GOLDEN_CELLS,
};

/// Generation precision / rounding guard of the golden set (the file `#` header
/// carries the authoritative values; these mirror them for the validators).
pub const GEN_PRECISION: usize = 1233;
pub const GUARD: usize = 2;

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
        decimal_scaled_cells::dispatch_limits(self.width, self.scale)
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
        // The per-cell concrete work is the cells crate's compile-once shim.
        move |inputs| decimal_scaled_cells::dispatch_compute(width, scale, func, inputs, m)
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

    /// The golden band-edge `(width, scale)` cells passing the width/scale filters,
    /// restricted to the tiers this build actually compiles (a narrow build sweeps
    /// D18/D38). The golden gate and the version-history pins walk these — NOT the
    /// lib-compare-only scales (see [`Filter::compare_cells`]).
    pub fn cells(&self) -> Vec<(u32, u32)> {
        self.select(GOLDEN_CELLS)
    }

    /// The lib-compare cells: the [`COMPARE_SCALES`] subset of the full COMPILED grid
    /// passing the same filters — per width, those of {17, 28, 37, 152} the tier can
    /// hold. Decoupled from [`Filter::cells`] so the comparison's precision choices
    /// never enlarge the golden grid; the two gates share only the compile-once cells.
    pub fn compare_cells(&self) -> Vec<(u32, u32)> {
        self.select(CELLS)
            .into_iter()
            .filter(|(_, s)| COMPARE_SCALES.contains(s))
            .collect()
    }

    /// The width/scale/tier-compiled filter shared by [`Filter::cells`] and
    /// [`Filter::compare_cells`].
    fn select(&self, cells: &[(u32, u32)]) -> Vec<(u32, u32)> {
        cells
            .iter()
            .copied()
            .filter(|(w, _)| tier_compiled(*w))
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
        // GOLDEN_CELLS is the band-edge correctness grid: the 88 band-edge cells
        // across the 12 widths, plus the four ln-lookup band cells (D307<290>,
        // D616<590>, D924<900>, D1232<1200>) and the D230<30> cell. CELLS is its
        // union with the 34 lib-compare-only scales (the `; compare` tails the
        // lib-perf bench times at; golden/history never walk them) — spot-check both
        // counts, the golden edges, and that the union contains the golden grid.
        assert_eq!(GOLDEN_CELLS.len(), 93);
        assert_eq!(GOLDEN_CELLS.first(), Some(&(18, 0)));
        assert_eq!(GOLDEN_CELLS.last(), Some(&(1232, 1231)));
        assert_eq!(CELLS.len(), 127);
        assert!(GOLDEN_CELLS.iter().all(|c| CELLS.contains(c)));
    }

    #[test]
    fn dispatch_round_trips_a_known_cell() {
        // sqrt(2) at D38<19> under half-to-even, via the erased dispatch (the
        // cells crate's compile-once shim): a 19-dp value on the right prefix
        // (not pinned digit-for-digit, to stay robust).
        let out = decimal_scaled_cells::dispatch_compute(
            38, 19, Function::Sqrt, &["2".to_string()], DsMode::HalfToEven,
        );
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
        let lim = decimal_scaled_cells::dispatch_limits(38, 19);
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
