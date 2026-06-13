//! Comparative precision runner on the `decimal-scaled-golden` harness.
//!
//! Sweeps (function × width × scale) over decimal-scaled plus the six peer
//! libraries, scores each cell against the multi-oracle golden set under
//! `decimal-scaled-golden/golden/`, and:
//!
//!   1. PERSISTS deterministic, committable per-library result files
//!      under `results/precision/<library>.tsv` — the single source of
//!      truth for the README / benchmarks precision tables;
//!   2. RENDERS the LSBε / ULP shootout table FROM those files and
//!      prints it to stdout (so the docs table is generated from the
//!      committed data, not recomputed ad-hoc);
//!   3. RENDERS the fidelity grade report (rubric published inline).
//!
//! This is the port of `benches/libcmp/lib_cmp_precision.rs` onto the
//! agnostic harness: the measurement intent and the persisted TSV schema
//! are unchanged (same columns, same row keys, same renderers downstream),
//! but the engine is now the harness's `ParallelRunner` + `PrecisionValidator`
//! over the `DecimalSubject` adapters — `DsSubject` per `(width, scale)` cell
//! from `decimal-scale-test`, and the competitor adapters from this crate.
//!
//! The width/scale surface is `decimal_scale_test::CELLS` (the band-edge
//! cells the golden gate enumerates). For each cell the golden cases are
//! filtered to inputs exactly representable at that cell, and every library
//! is graded with the oracle clamped to the CELL scale — so a peer's grade
//! depth is `min(its own reach at the value, the cell scale)`, the same
//! grade-at-own-last-digit rule as before, with the peer's reach now
//! value-aware (a fixed-significant peer's fractional depth shrinks as the
//! integer part grows) rather than a flat cap.
//!
//! Run:
//!   cargo run --release -p golden-competitors --bin lib_cmp_precision
//!
//! The `GOLDEN_WIDTHS` / `GOLDEN_SCALES` / `GOLDEN_FUNCS` env vars
//! (see `decimal_scale_test::Filter`) subset the sweep for a quick slice.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use decimal_scale_test::{golden_dir, thread_count, DsSubject, Filter, GEN_PRECISION};
use decimal_scaled_golden::string_decimal::{cmp_mag, strip_lead, within};
use decimal_scaled_golden::{
    CaseLoader, Computed, DecimalSubject, ExecutionResult, FileLoader, Function,
    GoldenCase, GoldenRunner, Limits, Outcome, ParallelRunner, PrecisionValidator, RoundingMode,
    RunOnce,
};
use golden_competitors::{
    BigDecimalSubject, DashuFloat, DecimalRsSubject, FastNum, GMath, RustDecimal, F32,
};

/// Cap on golden inputs scored per cell. Accuracy, not timing — a couple
/// hundred inputs saturates the claim while keeping wide-tier kernels fast.
/// Applied as a deterministic STRIDE over the cell's whole representable
/// population (see `stride_sample`), not its head, so a cell's sample spans
/// the golden file's full input range.
const SAMPLE_CAP: usize = 200;

/// The widths surfaced as per-width shootout tables in the rendered report.
/// The scoring sweep itself is NOT gated by this — it covers every
/// `(function, width, scale)` cell of the band-edge surface; these are only
/// the widths additionally rendered as side-by-side LSBε/ULP tables (the
/// canonical-scale cell per width). The grade summary covers the full
/// scanned surface regardless.
const TABLE_WIDTHS: [&str; 3] = ["D38", "D76", "D307"];

/// The library roster, in render order. Single source for the per-library
/// TSV files, the per-width tables, and the grade report. `f32` is a binary
/// subject (graded against the golden `2` value at its ~7-digit reach, owner
/// Q-C) — appended after the other binary-backed peer (g_math).
const LIBS: [&str; 8] = [
    "decimal-scaled",
    "fastnum",
    "rust_decimal",
    "dashu-float",
    "decimal-rs",
    "bigdecimal",
    "g_math",
    "f32",
];

/// The twelve decimal tiers: `(name, digit capacity, canonical scale)`. The
/// canonical scale is the one the per-width shootout tables render at.
const WIDTHS: [(&str, u32, u32); 12] = [
    ("D18", 18, 9),
    ("D38", 38, 19),
    ("D57", 57, 28),
    ("D76", 76, 38),
    ("D115", 115, 57),
    ("D153", 153, 76),
    ("D230", 230, 115),
    ("D307", 307, 153),
    ("D462", 462, 231),
    ("D616", 616, 308),
    ("D924", 924, 462),
    ("D1232", 1232, 616),
];

/// The transcendental surface, in canonical table order (the per-width
/// shootout tables' columns and the per-function grade-table row order).
const TRANSCENDENTAL: [&str; 22] = [
    "sqrt", "cbrt", "exp", "ln", "log2", "log10", "exp2", "sin", "cos", "tan", "atan", "asin",
    "acos", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "log", "atan2", "powf",
];

fn canonical_scale(width: &str) -> Option<u32> {
    WIDTHS.iter().find(|(n, ..)| *n == width).map(|&(_, _, s)| s)
}

/// Sanitise a library name into a stable filename stem.
fn file_stem(name: &str) -> String {
    name.replace('-', "_")
}

// ════════════════════════════════════════════════════════════════════
// Cell roster + scoring on the harness
// ════════════════════════════════════════════════════════════════════

/// A pre-filtered, pre-capped slice of one function's golden cases, with the
/// oracle clamped to the cell scale — so every subject's grade depth is
/// `min(its own reach, the cell scale)`.
struct CellSlice {
    cases: Arc<Vec<GoldenCase>>,
    oracle: Limits,
}

impl CaseLoader for CellSlice {
    fn load(&self, _func: Function) -> Cow<'_, [GoldenCase]> {
        Cow::Borrowed(&self.cases)
    }
    fn oracle_limits(&self) -> Limits {
        self.oracle.clone()
    }
}

/// Count of fraction digits up to the last non-zero one (trailing zeros
/// ignored): the depth at which the value is exactly representable.
fn significant_frac_digits(s: &str) -> usize {
    s.split_once('.').map(|(_, f)| f.trim_end_matches('0').len()).unwrap_or(0)
}

/// True if `input` is exactly representable at the cell whose envelope is
/// `lim` (the `DsSubject` cell limits: fixed fractional depth + MIN/MAX).
fn cell_representable(input: &str, lim: &Limits) -> bool {
    significant_frac_digits(input) <= lim.max_precision as usize
        && within(input, lim.min_value.as_deref(), lim.max_value.as_deref())
}

/// At most `cap` cases spread EVENLY across `kept` (deterministic stride):
/// sample index `i` maps to population index `i * len / cap`. Taking the
/// head instead would bias a cell toward the golden file's leading region.
fn stride_sample(kept: Vec<&GoldenCase>, cap: usize) -> Vec<GoldenCase> {
    let len = kept.len();
    if len <= cap {
        return kept.into_iter().cloned().collect();
    }
    (0..cap).map(|i| kept[i * len / cap].clone()).collect()
}

/// One scored cell: the aggregate over the roster.
#[derive(Clone, Default)]
struct CellScore {
    scored: usize,
    correctly_rounded: usize,
    panicked: usize,
    /// Worst |subject − correctly-rounded-oracle| over the roster, as a
    /// decimal-magnitude string in last-place units at the grade depth.
    worst: String,
    /// The deepest grade depth observed over the scored roster — the
    /// library's actual reach at this cell (equals the cell scale for
    /// decimal-scaled and the arbitrary-precision peers).
    reach_scale: u32,
}

/// Run one subject over one cell roster and aggregate the harness's
/// per-case `Precision { ulps }` outcomes into a `CellScore`.
fn score_cell<S: DecimalSubject + Sync>(
    subject: &S,
    func: Function,
    roster: &Arc<Vec<GoldenCase>>,
    oracle: &Limits,
    threads: usize,
) -> CellScore {
    let runner = ParallelRunner {
        threads,
        strategy: RunOnce,
        loader: Box::new(CellSlice { cases: Arc::clone(roster), oracle: oracle.clone() }),
        validators: vec![Box::new(PrecisionValidator { gen_precision: GEN_PRECISION })],
    };
    let sc = runner.run(subject, &[func]);

    let mut out = CellScore { worst: "0".to_string(), ..CellScore::default() };
    let Some(fc) = sc.functions.first() else {
        return out;
    };
    for cell in &fc.cells {
        let ulps = cell.validations.iter().find_map(|o| match o {
            Outcome::Precision { ulps } => Some(ulps),
            _ => None,
        });
        match (cell.result(), ulps) {
            (Some(ExecutionResult::Computed(Computed::Value(_))), Some(u)) => {
                out.scored += 1;
                if strip_lead(u) == "0" {
                    out.correctly_rounded += 1;
                }
                if cmp_mag(u, &out.worst) == core::cmp::Ordering::Greater {
                    out.worst = u.clone();
                }
                // The depth this case was graded at: the library's own reach
                // at the golden value, clamped to the cell's oracle depth —
                // the same derivation the validator's grade uses.
                let reach =
                    subject.limits(&cell.expected).max_precision.min(oracle.max_precision);
                out.reach_scale = out.reach_scale.max(reach);
            }
            (Some(ExecutionResult::Computed(Computed::Panic(_))), _) => {
                out.panicked += 1;
            }
            _ => {} // skipped / non-real / absent / error / out-of-range: n/a
        }
    }
    if out.scored == 0 {
        // Nothing scored: report the nominal reach so downstream readers
        // still see the depth this library WOULD have been graded at.
        out.reach_scale = subject.limits("1").max_precision.min(oracle.max_precision);
    }
    out
}

/// The subject's declared rounding mode for `func` (its native mode — every
/// adapter declares one uniform mode), as the TSV's `rounding` column.
fn declared_mode<S: DecimalSubject>(subject: &S, func: Function) -> RoundingMode {
    let caps = subject.capabilities();
    caps.function(func)
        .map(|s| s.mode)
        .or_else(|| caps.functions.values().next().map(|s| s.mode))
        .unwrap_or(RoundingMode::HalfToEven)
}

// ════════════════════════════════════════════════════════════════════
// Persisted rows (the unchanged TSV schema)
// ════════════════════════════════════════════════════════════════════

/// Decimal-digit length of an unsigned magnitude string. `0` for zero —
/// the exact count of contaminated trailing decimal digits in the error.
fn dec_digit_len(mag: &str) -> u32 {
    let mag = strip_lead(mag);
    if mag == "0" { 0 } else { mag.len() as u32 }
}

/// Bit length of an unsigned decimal-magnitude string. `0` for zero.
/// Exact via u128 where it fits; a conservative `digits·log2(10)` ceiling
/// for wider magnitudes (already a gross miss — bucketing suffices).
fn dec_bit_len(mag: &str) -> u32 {
    let mag = strip_lead(mag);
    if mag == "0" {
        return 0;
    }
    if mag.len() <= 38 {
        if let Ok(v) = mag.parse::<u128>() {
            return 128 - v.leading_zeros();
        }
    }
    (mag.len() as f64 * core::f64::consts::LOG2_10).ceil() as u32
}

/// Convert an unsigned decimal-magnitude string to f64 (saturating to
/// `INFINITY` past f64 range) for the continuous ULP distance.
fn dec_to_f64(mag: &str) -> f64 {
    let mag = strip_lead(mag);
    if mag == "0" {
        return 0.0;
    }
    mag.parse::<f64>().unwrap_or(f64::INFINITY)
}

/// One persisted row, fully keyed and self-describing.
struct Row {
    method: &'static str,
    width: String,
    scale: u32,
    rounding: String,
    /// "executed" or "na".
    kind: &'static str,
    max_lsbe: i64,
    max_ulp: String,
    scored: usize,
    correctly_rounded: usize,
    /// Worst DECIMAL error-digit length over the roster — what the
    /// fidelity grade scores. `-1` for an n/a cell.
    max_digits: i64,
    /// The scale this subject was actually graded at — its own last
    /// representable digit, clamped to the cell scale. For a fixed-precision
    /// peer running on a cell deeper than its reach this is below `scale`,
    /// and the LSBε/ULP/`max_digits` in this row measure correctness at
    /// THAT depth. For arbitrary-precision peers and decimal-scaled this
    /// equals `scale`.
    reach_scale: u32,
}

impl Row {
    fn from_score(
        method: &'static str,
        width: String,
        scale: u32,
        rounding: RoundingMode,
        c: &CellScore,
    ) -> Row {
        if c.scored == 0 {
            Row {
                method,
                width,
                scale,
                rounding: format!("{rounding:?}"),
                kind: "na",
                max_lsbe: -1,
                max_ulp: "na".to_string(),
                scored: 0,
                correctly_rounded: 0,
                max_digits: -1,
                reach_scale: c.reach_scale,
            }
        } else {
            Row {
                method,
                width,
                scale,
                rounding: format!("{rounding:?}"),
                kind: "executed",
                max_lsbe: dec_bit_len(&c.worst) as i64,
                max_ulp: fmt_ulp(dec_to_f64(&c.worst)),
                scored: c.scored,
                correctly_rounded: c.correctly_rounded,
                max_digits: dec_digit_len(&c.worst) as i64,
                reach_scale: c.reach_scale,
            }
        }
    }
}

/// Format a ULP value deterministically (fixed-form where small,
/// scientific where large) so file diffs are stable across runs.
fn fmt_ulp(u: f64) -> String {
    if u == 0.0 {
        "0".to_string()
    } else if u < 0.001 {
        format!("{u:.3e}")
    } else if u < 1e6 {
        format!("{u:.6}")
    } else {
        format!("{u:.3e}")
    }
}

/// `<repo>/results/precision`. `CARGO_MANIFEST_DIR` is this crate's root,
/// one level under the repository root.
fn results_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("results");
    p.push("precision");
    p
}

/// Write one library's rows to `results/precision/<stem>.tsv`. Stable
/// header + stable row order ⇒ meaningful diffs across runs.
fn write_tsv(dir: &Path, name: &str, rows: &[Row]) {
    let path = dir.join(format!("{}.tsv", file_stem(name)));
    let mut f = fs::File::create(&path).expect("create result tsv");
    writeln!(
        f,
        "# library: {name}\n# key: method\twidth\tscale\trounding\tresult\n\
         # result columns: kind(executed|na)\tmax_lsbe\tmax_ulp\tscored\tcorrectly_rounded\tmax_digits\treach_scale\n\
         # max_digits = exact decimal-digit length of the worst error magnitude \
         (what the fidelity grade scores); max_lsbe is its bit-width.\n\
         # reach_scale = the depth (fractional digits) this library was actually \
         graded at — its own last representable digit; equals `scale` for \
         arbitrary-precision peers and decimal-scaled, falls below `scale` for a \
         fixed-precision peer running on a deeper cell. The lsbe/ulp/digits \
         in this row measure correctness at THAT depth (grade-at-own-last-digit), \
         and the gap `scale - reach_scale` is the shallower-reach metric.\n\
         # source: golden-competitors/src/bin/lib_cmp_precision.rs (regenerate with \
         `cargo run --release -p golden-competitors --bin lib_cmp_precision`)"
    )
    .unwrap();
    writeln!(
        f,
        "method\twidth\tscale\trounding\tkind\tmax_lsbe\tmax_ulp\tscored\tcorrectly_rounded\tmax_digits\treach_scale"
    )
    .unwrap();
    for r in rows {
        writeln!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.method,
            r.width,
            r.scale,
            r.rounding,
            r.kind,
            r.max_lsbe,
            r.max_ulp,
            r.scored,
            r.correctly_rounded,
            r.max_digits,
            r.reach_scale,
        )
        .unwrap();
    }
    eprintln!("wrote {} ({} rows)", path.display(), rows.len());
}

// ════════════════════════════════════════════════════════════════════
// main — sweep, persist, render
// ════════════════════════════════════════════════════════════════════

fn main() {
    let filter = Filter::from_env();
    let cells = filter.cells();
    let funcs: Vec<Function> = filter.funcs().to_vec();
    let out_dir = results_dir();
    fs::create_dir_all(&out_dir).expect("create results/precision dir");

    let loader = FileLoader::new(golden_dir());
    let oracle_base = loader.oracle_limits();
    let threads = thread_count();
    eprintln!(
        "golden surface: {} (width, scale) cells x {} functions from {}",
        cells.len(),
        funcs.len(),
        golden_dir()
    );

    // Quiet the default panic hook during the sweep: subjects panic on
    // overflow / domain edges by contract (the harness catches each as
    // `Computed::Panic`); those are surfaced as an explicit summary below
    // rather than a wall of backtraces interleaved with the tables.
    std::panic::set_hook(Box::new(|_| {}));
    // (library, function, width, scale) cells where a subject panicked.
    let mut panic_cells: Vec<(String, &'static str, String, u32)> = Vec::new();

    // ── 1. Sweep + persist per-library result files ────────────────
    let mut rows: BTreeMap<&'static str, Vec<Row>> = LIBS.iter().map(|l| (*l, Vec::new())).collect();
    let mut total_cells = 0usize;

    for &func in &funcs {
        let cases: Vec<GoldenCase> = loader.load(func).into_owned();
        if cases.is_empty() {
            continue;
        }
        for &(width, scale) in &cells {
            let ds = DsSubject::new(width, scale);
            // The cell's envelope (fixed fractional depth + MIN/MAX) defines
            // which golden inputs belong to this cell.
            let cell_limits = ds.limits("");
            let kept: Vec<&GoldenCase> = cases
                .iter()
                .filter(|c| c.inputs.iter().all(|i| cell_representable(i, &cell_limits)))
                .collect();
            let roster: Arc<Vec<GoldenCase>> = Arc::new(stride_sample(kept, SAMPLE_CAP));
            if roster.is_empty() {
                continue;
            }
            total_cells += 1;
            // Clamp the oracle to the CELL scale: every library's grade
            // depth becomes min(its own reach, the cell scale).
            let oracle = Limits {
                min_value: None,
                max_value: None,
                max_precision: oracle_base.max_precision.min(scale),
                max_significant_digits: None,
            };
            let width_name = format!("D{width}");

            // One scoring pass per library. Each arm is the same generic
            // call at a different concrete subject type.
            macro_rules! score {
                ($lib:expr, $subject:expr) => {{
                    let subject = $subject;
                    let c = score_cell(&subject, func, &roster, &oracle, threads);
                    if c.panicked > 0 {
                        panic_cells.push((
                            $lib.to_string(),
                            func.name(),
                            width_name.clone(),
                            scale,
                        ));
                    }
                    rows.get_mut($lib).expect("known library").push(Row::from_score(
                        func.name(),
                        width_name.clone(),
                        scale,
                        declared_mode(&subject, func),
                        &c,
                    ));
                }};
            }
            score!("decimal-scaled", ds);
            score!("fastnum", FastNum);
            score!("rust_decimal", RustDecimal);
            score!("dashu-float", DashuFloat);
            score!("decimal-rs", DecimalRsSubject);
            score!("bigdecimal", BigDecimalSubject);
            score!("g_math", GMath);
            score!("f32", F32);
        }
    }

    for lib in LIBS {
        write_tsv(&out_dir, lib, &rows[lib]);
    }

    // Restore the default panic hook for the rest of the run.
    let _ = std::panic::take_hook();

    // Surface any cells a subject panicked on — a kernel/library crash on
    // an edge input is a robustness signal, not silently swallowed.
    if !panic_cells.is_empty() {
        eprintln!(
            "NOTE: {} (library, function, width, scale) cell(s) had panicking \
             inputs (each counted as n/a for scoring):",
            panic_cells.len()
        );
        for (lib, func, width, scale) in &panic_cells {
            eprintln!("  PANIC: {lib} {func} {width} s{scale}");
        }
    }

    // ── 2. Render the shootout tables FROM the committed files ──────
    println!("# Precision shootout — generated from results/precision/*.tsv\n");
    for width in TABLE_WIDTHS {
        let table = render_from_files(&out_dir, width);
        println!("{table}");
    }

    // ── 3. Fidelity grading — per-function + overall + coverage ─────
    let report = render_fidelity_report(&out_dir, total_cells);
    println!("{report}");
    // Surface the grade report to the CI step summary when present.
    if let Ok(path) = std::env::var("GITHUB_STEP_SUMMARY") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{report}");
        }
    }
}

// ════════════════════════════════════════════════════════════════════
// Rendering — reads the committed TSVs back (proving the docs tables
// are generated from the single committed source, not recomputed)
// ════════════════════════════════════════════════════════════════════

/// A parsed result cell from a committed TSV.
#[derive(Clone)]
struct FileCell {
    kind: String,
    max_lsbe: i64,
    max_ulp: String,
}

/// Render the LSBε (ULP) shootout for one width by READING the committed
/// per-library TSV files.
fn render_from_files(dir: &Path, width: &str) -> String {
    // (library, method) -> cell, for this width at its canonical scale.
    let mut data: BTreeMap<String, BTreeMap<String, FileCell>> = BTreeMap::new();
    let mut modes: BTreeMap<String, String> = BTreeMap::new();
    let want_scale = canonical_scale(width);

    for lib in LIBS {
        let path = dir.join(format!("{}.tsv", file_stem(lib)));
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            if line.starts_with('#') || line.starts_with("method\t") {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 9 {
                continue;
            }
            if cols[1] != width {
                continue;
            }
            // Per-width LSBε table renders the canonical-scale cell (the
            // full multi-scale surface is in the TSVs + the grade summary).
            if cols[2].parse::<u32>().ok() != want_scale {
                continue;
            }
            modes.entry(lib.to_string()).or_insert(cols[3].to_string());
            data.entry(lib.to_string()).or_default().insert(
                cols[0].to_string(),
                FileCell {
                    kind: cols[4].to_string(),
                    max_lsbe: cols[5].parse().unwrap_or(-1),
                    max_ulp: cols[6].to_string(),
                },
            );
        }
    }

    let mut out = String::new();
    out.push_str(&format!(
        "## {} (scale {}) — LSBε (ULP)\n\n",
        width,
        want_scale.unwrap_or(0)
    ));
    out.push_str(
        "Cell = LSBε (max |ULP distance to true|). 0 (0) = correctly \
         rounded (bit-exact under the subject's reported mode). `n/a` = \
         method not exposed or width/scale not representable.\n\n",
    );
    out.push_str("| library | mode |");
    for m in TRANSCENDENTAL {
        out.push_str(&format!(" {m} |"));
    }
    out.push('\n');
    out.push_str("|---|---|");
    for _ in TRANSCENDENTAL {
        out.push_str("---|");
    }
    out.push('\n');

    for lib in LIBS {
        let row = data.get(lib);
        let mode = modes.get(lib).cloned().unwrap_or_else(|| "-".to_string());
        out.push_str(&format!("| {lib} | {mode} |"));
        for m in TRANSCENDENTAL {
            let cell = row.and_then(|r| r.get(m));
            let s = match cell {
                None => "n/a".to_string(),
                Some(c) if c.kind == "na" => "n/a".to_string(),
                Some(c) => {
                    let ulp = c.max_ulp.parse::<f64>().unwrap_or(0.0);
                    let ulp_str = if ulp == 0.0 {
                        "0".to_string()
                    } else if ulp < 10.0 {
                        format!("{ulp:.2}")
                    } else {
                        format!("{ulp:.1e}")
                    };
                    format!("{} ({ulp_str})", c.max_lsbe)
                }
            };
            out.push_str(&format!(" {s} |"));
        }
        out.push('\n');
    }
    out.push('\n');
    out
}

/// One library's parsed result rows read back from its committed TSV:
/// (method, width, scale, kind, max_digits, reach_scale) per row.
struct LibRows {
    rows: Vec<(String, String, u32, String, i64, u32)>,
}

fn read_lib_rows(dir: &Path, lib: &str) -> LibRows {
    let mut rows = Vec::new();
    let path = dir.join(format!("{}.tsv", file_stem(lib)));
    if let Ok(text) = fs::read_to_string(&path) {
        for line in text.lines() {
            if line.starts_with('#') || line.starts_with("method\t") {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 10 {
                continue;
            }
            let scale = cols[2].parse::<u32>().unwrap_or(0);
            let max_digits = cols[9].parse::<i64>().unwrap_or(-1);
            let reach_scale = cols.get(10).and_then(|s| s.parse::<u32>().ok()).unwrap_or(scale);
            rows.push((
                cols[0].to_string(),
                cols[1].to_string(),
                scale,
                cols[4].to_string(),
                max_digits,
                reach_scale,
            ));
        }
    }
    LibRows { rows }
}

/// Parse a tier-width string (`"D307"`) into its decimal digit capacity
/// (`307`). Falls back to `0` for an unrecognised label (treated as a
/// minimal width by `cell_demerit`, which clamps to >= 1).
fn tier_width_of(width: &str) -> u32 {
    width.trim_start_matches(['D', 'd']).parse::<u32>().unwrap_or(0)
}

// ════════════════════════════════════════════════════════════════════
// Fidelity grading — per-function + overall two-letter grade / 0–100 score
// ════════════════════════════════════════════════════════════════════
//
// The locked rubric (published alongside the tables):
//
//   * `incorrect_digits` = the EXACT decimal-digit length of the cell's
//     worst error magnitude — the count of contaminated trailing decimal
//     digits in the last-place error. `0` ⇒ correctly rounded.
//   * per-cell demerit, RELATIVE to the tier's precision:
//       `d = 0` if correctly rounded, else
//       `d = min(10, ceil(incorrect_digits / tier_width * 10))`.
//   * `mean_demerit = Σ d / executed_cells`.
//   * `score (closeness) = 100 * (1 − mean_demerit/10)` ∈ [0,100].
//   * `%CR (reliability) = 100 * correct_cells / executed_cells`.
//   * grade bands (both): A == 100 · B [95,100) · C [85,95) · D [70,85) ·
//     E [50,70) · F < 50.
//   * headline per library = TWO letters: `grade(score)` then `grade(%CR)`.

/// Per-cell demerit `d`, RELATIVE to the tier's precision.
fn cell_demerit(max_digits: u32, tier_width: u32) -> f64 {
    if max_digits == 0 {
        0.0
    } else {
        let tw = tier_width.max(1) as f64;
        let raw = (max_digits as f64 / tw * 10.0).ceil();
        raw.min(10.0)
    }
}

/// The A–F letter grade for a 0–100 metric (closeness score and %CR alike).
fn grade_for_score(s: f64) -> char {
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

/// Running fidelity accumulator over a set of cells.
#[derive(Clone, Copy, Default)]
struct Fidelity {
    demerits: f64,
    cells: usize,
    correct: usize,
}

impl Fidelity {
    fn record(&mut self, max_digits: u32, tier_width: u32) {
        self.demerits += cell_demerit(max_digits, tier_width);
        self.cells += 1;
        if max_digits == 0 {
            self.correct += 1;
        }
    }
    fn mean_demerit(&self) -> f64 {
        if self.cells == 0 { 0.0 } else { self.demerits / self.cells as f64 }
    }
    fn score(&self) -> f64 {
        (100.0 * (1.0 - self.mean_demerit() / 10.0)).clamp(0.0, 100.0)
    }
    fn cr_pct(&self) -> f64 {
        if self.cells == 0 { 0.0 } else { 100.0 * self.correct as f64 / self.cells as f64 }
    }
    fn grade(&self) -> char {
        grade_for_score(self.score())
    }
    fn cr_grade(&self) -> char {
        grade_for_score(self.cr_pct())
    }
    fn two_letter(&self) -> String {
        format!("{}{}", self.grade(), self.cr_grade())
    }
}

/// Reach (precision/depth) statistics over the executed cells of one
/// library — independent of rounding correctness, so a shallower peer is
/// credited for accuracy at its own last digit AND honestly shown as not
/// reaching deeper.
#[derive(Clone, Copy, Default)]
struct ReachStats {
    sum_reach: u64,
    sum_cell: u64,
    full_reach_cells: usize,
    cells: usize,
}

impl ReachStats {
    fn record(&mut self, reach: u32, cell_scale: u32) {
        self.sum_reach += reach as u64;
        self.sum_cell += cell_scale as u64;
        if reach >= cell_scale {
            self.full_reach_cells += 1;
        }
        self.cells += 1;
    }
    fn mean_reach(&self) -> f64 {
        if self.cells == 0 { 0.0 } else { self.sum_reach as f64 / self.cells as f64 }
    }
    fn mean_cell(&self) -> f64 {
        if self.cells == 0 { 0.0 } else { self.sum_cell as f64 / self.cells as f64 }
    }
    fn full_reach_pct(&self) -> f64 {
        if self.cells == 0 {
            0.0
        } else {
            100.0 * self.full_reach_cells as f64 / self.cells as f64
        }
    }
}

/// The published rubric text, emitted alongside every grade table so the
/// grading is auditable from the report itself.
fn fidelity_rubric() -> &'static str {
    "### Fidelity grading rubric\n\
     \n\
     Each scored cell is one `(method, width, scale)` slice of the golden set. \
     Every library is graded **at its OWN last representable digit** \
     (`reach_scale`): the golden value is rounded down to that depth under the \
     library's reported mode, and the library's emission is rounded to the same \
     depth before diffing. So a fixed-precision peer that cannot reach \
     decimal-scaled's deep cell scale is graded at its reach — correctly-rounded-\
     there is `0` demerits, off-by-N-there is `N` — and its shallower reach is \
     reported independently as a separate metric, NOT a rounding failure. `n/a` \
     cells (a method the library does not expose at all, or inputs it rejects) \
     are excluded from BOTH the cell count and the demerit sum.\n\
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

/// Render the fidelity grade report by READING the committed per-library
/// TSVs, with the rubric published inline. `total_cells` = the number of
/// `(function, width, scale)` cells the sweep scanned (the coverage base).
fn render_fidelity_report(dir: &Path, total_cells: usize) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    out.push_str("# Fidelity grades — generated from results/precision/*.tsv\n\n");
    out.push_str(fidelity_rubric());
    out.push('\n');

    // Collect per-library per-function fidelity + a pooled per-library
    // accumulator + a runnable counter + reach stats, in one pass. The
    // demerit is precision-relative, so each cell folds in its OWN tier
    // width.
    let mut graded: BTreeMap<&str, (BTreeMap<String, Fidelity>, Fidelity, usize, ReachStats)> =
        BTreeMap::new();
    for lib in LIBS {
        let data = read_lib_rows(dir, lib);
        let mut per_fn: BTreeMap<String, Fidelity> = BTreeMap::new();
        let mut pooled = Fidelity::default();
        let mut runnable = 0usize;
        let mut reach = ReachStats::default();
        for (method, width, scale, kind, max_digits, reach_scale) in &data.rows {
            if kind != "executed" || *max_digits < 0 {
                continue; // n/a cell — method not exposed / input rejected.
            }
            runnable += 1;
            let tw = tier_width_of(width);
            per_fn.entry(method.clone()).or_default().record(*max_digits as u32, tw);
            pooled.record(*max_digits as u32, tw);
            reach.record(*reach_scale, *scale);
        }
        graded.insert(lib, (per_fn, pooled, runnable, reach));
    }

    // ── 1. OVERALL table — one row per library, best score first ──
    out.push_str("## Overall — per-library headline grade\n\n");
    out.push_str(
        "Every library is graded **at its OWN last representable digit** — its \
         `reach_scale` — not at decimal-scaled's deeper cell scale. So a \
         fixed-precision peer that cannot reach our scale is NOT a rounding \
         failure: it is scored as correctly-rounded-at-its-reach (or off-by-N \
         there) and its shallower reach is reported separately in the `reach` \
         column (`mean reach / mean cell scale`). Sorted by closeness score \
         (best first). The grade is two letters: 1st = closeness `grade(score)`, \
         2nd = reliability `grade(%CR)`.\n\n",
    );
    out.push_str("| library | grade | score | %CR | runnable | reach (mean / cell, full %) |\n");
    out.push_str("|---|---|---|---|---|---|\n");
    let mut overall: Vec<(&str, &Fidelity, usize, &ReachStats)> = graded
        .iter()
        .map(|(lib, (_pf, pooled, runnable, reach))| (*lib, pooled, *runnable, reach))
        .collect();
    // Best closeness first; runnable cells break ties (more coverage wins).
    overall.sort_by(|a, b| {
        b.1.score()
            .partial_cmp(&a.1.score())
            .unwrap_or(core::cmp::Ordering::Equal)
            .then(b.2.cmp(&a.2))
    });
    for (lib, pooled, runnable, reach) in &overall {
        if *runnable == 0 {
            let _ = writeln!(out, "| {lib} | — | n/a | n/a | 0 | n/a |");
        } else {
            let _ = writeln!(
                out,
                "| {lib} | **{}** | {:.1} | {:.1} | {runnable} | {:.1} / {:.1} ({:.0}%) |",
                pooled.two_letter(),
                pooled.score(),
                pooled.cr_pct(),
                reach.mean_reach(),
                reach.mean_cell(),
                reach.full_reach_pct(),
            );
        }
    }
    out.push('\n');

    // ── 2. Per-library per-function grade tables ──
    for lib in LIBS {
        let (per_fn, pooled, runnable, reach) = match graded.get(lib) {
            Some(g) => g,
            None => continue,
        };
        let _ = writeln!(out, "## {lib} — fidelity\n");
        if *runnable == 0 {
            out.push_str("_No runnable cells (library exposes none of the scanned methods)._\n\n");
            continue;
        }

        // Per-function rows, in canonical method order.
        out.push_str("| function | cells | mean_demerit | score | %CR | grade |\n");
        out.push_str("|---|---|---|---|---|---|\n");
        for name in TRANSCENDENTAL {
            let Some(f) = per_fn.get(name) else { continue };
            let _ = writeln!(
                out,
                "| {name} | {} | {:.3} | {:.1} | {:.1} | {}{} |",
                f.cells,
                f.mean_demerit(),
                f.score(),
                f.cr_pct(),
                f.grade(),
                f.cr_grade(),
            );
        }

        let _ = writeln!(
            out,
            "\n**Overall {lib}: {}** — score **{:.1}** (closeness) / **{:.1}%** CR \
             (reliability); mean_demerit = {:.3} over {} runnable cells. \
             **Reach** mean **{:.1}** / cell mean **{:.1}** (full-reach on **{:.0}%** of cells) \
             — graded at this library's OWN last representable digit, not at the \
             cell scale.\n",
            pooled.two_letter(),
            pooled.score(),
            pooled.cr_pct(),
            pooled.mean_demerit(),
            pooled.cells,
            reach.mean_reach(),
            reach.mean_cell(),
            reach.full_reach_pct(),
        );
    }

    // ── 3. Coverage (runnable cells / total scanned) ──
    out.push_str("## Coverage (runnable cells / total scanned)\n\n");
    out.push_str(
        "Coverage = cells where the library exposed the method and parsed the \
         input. Under grade-at-own-last-digit, a fixed-precision peer running \
         on a cell deeper than its reach is RUNNABLE (graded at its own last \
         digit, shallower reach reported in the overall table) — runnable does \
         not narrow just because our scale is deeper.\n\n",
    );
    out.push_str("| library | runnable | total | coverage |\n");
    out.push_str("|---|---|---|---|\n");
    for lib in LIBS {
        let runnable = graded.get(lib).map(|(_, _, r, _)| *r).unwrap_or(0);
        let cov = if total_cells == 0 {
            0.0
        } else {
            100.0 * runnable as f64 / total_cells as f64
        };
        let _ = writeln!(out, "| {lib} | {runnable} | {total_cells} | {cov:.0}% |");
    }
    out.push('\n');

    out
}
