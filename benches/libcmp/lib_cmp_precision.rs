//! Comparative precision runner for the library-agnostic harness.
//!
//! Sweeps (method × width × scale) over every [`PrecisionSubject`]
//! (decimal-scaled + the five peers), scores each cell against the
//! mpmath golden oracle, and:
//!
//!   1. PERSISTS deterministic, committable per-library result files
//!      under `results/precision/<library>.tsv` — the single source of
//!      truth for the README / benchmarks precision tables;
//!   2. RENDERS the LSBε / ULP shootout table FROM those files and
//!      prints it to stdout (so the docs table is generated from the
//!      committed data, not recomputed ad-hoc).
//!
//! This is the crate's ONE precision-comparison path: the bespoke
//! per-library drivers that used to live here are gone, replaced by the
//! [`PrecisionSubject`] adapters in `tests/support/`. The same harness
//! also backs the correctness gate (`tests/ulp_strict_golden.rs`), so
//! there is a single precision library, not two.
//!
//! `harness = false` with a plain `fn main` — this is an accuracy
//! harness, not a timing bench, so there is no criterion machinery.
//!
//! Run:
//!   cargo bench --bench lib_cmp_precision \
//!       --features wide,x-wide,xx-wide,macros

#[path = "../../tests/support/precision_harness.rs"]
mod harness;
use harness::*;

#[path = "../../tests/support/precision_subject_ds.rs"]
mod subject_ds;

#[path = "../../tests/support/precision_subjects_peers.rs"]
mod subjects_peers;

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use decimal_scaled::RoundingMode;
use subject_ds::DecimalScaledSubject;
use subjects_peers::{
    BigDecimalSubject, DashuSubject, DecimalRsSubject, FastnumSubject, GMathSubject,
    RustDecimalSubject,
};

/// Cap on golden inputs scored per cell. Accuracy, not timing — a couple
/// hundred inputs saturates the claim while keeping wide-tier kernels
/// fast.
const SAMPLE_CAP: usize = 200;

/// The driving mode for the comparative sweep. The oracle is folded
/// under each subject's REPORTED mode, so this only affects the modes
/// decimal-scaled is asked to apply; it reports back the same mode.
const DRIVE_MODE: RoundingMode = RoundingMode::HalfToEven;

/// The widths surfaced as per-width shootout tables in the rendered
/// report. The scoring sweep itself is NOT gated by this — it auto-picks
/// up EVERY `(method, width, scale)` golden file via `golden_scan()`;
/// these are only the widths we additionally render as side-by-side
/// LSBε/ULP tables (the canonical-scale cell per width). The grade
/// summary covers the full scanned surface regardless.
const TABLE_WIDTHS: [Width; 3] = [Width::D38, Width::D76, Width::D307];

fn subjects() -> Vec<Box<dyn PrecisionSubject>> {
    vec![
        Box::new(DecimalScaledSubject),
        Box::new(FastnumSubject),
        Box::new(RustDecimalSubject),
        Box::new(DashuSubject),
        Box::new(DecimalRsSubject),
        Box::new(BigDecimalSubject),
        Box::new(GMathSubject),
    ]
}

/// Sanitise a library name into a stable filename stem.
fn file_stem(name: &str) -> String {
    name.replace('-', "_")
}

/// One persisted row, fully keyed and self-describing.
struct Row {
    method: &'static str,
    width: &'static str,
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
    /// representable digit. For a fixed-precision peer running on a cell
    /// deeper than its cap, this is the cap (e.g. rust_decimal on a
    /// `D76<38>` cell ⇒ `reach_scale = 28`), and the LSBε/ULP/`max_digits`
    /// in this row measure correctness at THAT depth. For arbitrary-
    /// precision peers and decimal-scaled this equals `scale`.
    reach_scale: u32,
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

fn main() {
    let subjects = subjects();
    let out_dir = results_dir();
    fs::create_dir_all(&out_dir).expect("create results/precision dir");

    // Auto-pickup: every (method, width, scale) golden file on disk. NO
    // hardcoded width/scale list — the surface grows as goldens are added.
    let cells = golden_scan();
    eprintln!(
        "golden scan: {} (method, width, scale) cells from {}",
        cells.len(),
        golden_dir().display()
    );

    // Quiet the default panic hook during the sweep: a subject may panic
    // on an edge input (the harness isolates each cell with catch_unwind);
    // we surface those as an explicit summary below rather than a wall of
    // backtraces interleaved with the table.
    std::panic::set_hook(Box::new(|_| {}));
    // (subject, method, width, scale) cells where a subject panicked.
    let mut panic_cells: Vec<(String, String, String, u32)> = Vec::new();

    // ── 1. Sweep + persist per-library result files ────────────────
    for subject in &subjects {
        let mut rows: Vec<Row> = Vec::new();
        for cell in &cells {
            let scored = score_scanned_cell(subject.as_ref(), cell, DRIVE_MODE, SAMPLE_CAP);
            if let Some(c) = &scored {
                if c.panicked > 0 {
                    panic_cells.push((
                        subject.name().to_string(),
                        cell.method.name().to_string(),
                        cell.width.name().to_string(),
                        cell.scale,
                    ));
                }
            }
            match scored {
                None => continue, // empty / unreadable table
                Some(c) if c.scored == 0 => rows.push(Row {
                    method: cell.method.name(),
                    width: cell.width.name(),
                    scale: cell.scale,
                    rounding: format!("{:?}", subject.native_mode()),
                    kind: "na",
                    max_lsbe: -1,
                    max_ulp: "na".to_string(),
                    scored: 0,
                    correctly_rounded: 0,
                    max_digits: -1,
                    reach_scale: c.reach_scale,
                }),
                Some(c) => rows.push(Row {
                    method: cell.method.name(),
                    width: cell.width.name(),
                    scale: cell.scale,
                    rounding: format!("{:?}", subject.native_mode()),
                    kind: "executed",
                    max_lsbe: c.max_lsbe as i64,
                    max_ulp: fmt_ulp(c.max_ulp),
                    scored: c.scored,
                    correctly_rounded: c.correctly_rounded,
                    max_digits: c.max_digits as i64,
                    reach_scale: c.reach_scale,
                }),
            }
        }
        write_tsv(&out_dir, subject.name(), &rows);
    }

    // Restore the default panic hook for the rest of the run.
    let _ = std::panic::take_hook();

    // Surface any cells a subject panicked on — a kernel/library crash on
    // an edge input is a robustness signal, not silently swallowed.
    if !panic_cells.is_empty() {
        eprintln!(
            "WARNING: {} (subject, method, width, scale) cell(s) PANICKED during \
             evaluation (scored as n/a):",
            panic_cells.len()
        );
        for (subj, method, width, scale) in &panic_cells {
            eprintln!("  PANIC: {subj} {method} {width} s{scale}");
        }
    }

    // ── 2. Render the shootout tables FROM the committed files ──────
    println!("# Precision shootout — generated from results/precision/*.tsv\n");
    for &width in &TABLE_WIDTHS {
        let table = render_from_files(&out_dir, width);
        println!("{table}");
    }

    // ── 3. Fidelity grading — per-function + overall + coverage ─────
    let report = render_fidelity_report(&out_dir);
    println!("{report}");
    // Surface the grade report to the CI step summary when present.
    if let Ok(path) = std::env::var("GITHUB_STEP_SUMMARY") {
        if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{report}");
        }
    }
}

/// `<workspace>/results/precision`. `CARGO_MANIFEST_DIR` is the crate
/// root regardless of where `cargo bench` is invoked from.
fn results_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
         arbitrary-precision peers and decimal-scaled, equals the library's cap \
         for a fixed-precision peer running on a deeper cell. The lsbe/ulp/digits \
         in this row measure correctness at THAT depth (grade-at-own-last-digit), \
         and the gap `scale - reach_scale` is the shallower-reach metric.\n\
         # source: benches/lib_cmp_precision.rs (regenerate with \
         `cargo bench --bench lib_cmp_precision --features wide,x-wide,xx-wide,macros`)"
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

/// A parsed result cell from a committed TSV.
#[derive(Clone)]
struct FileCell {
    kind: String,
    max_lsbe: i64,
    max_ulp: String,
}

/// Render the LSBε (ULP) shootout for one width by READING the committed
/// per-library TSV files — proving the docs table is generated from the
/// single committed source, not recomputed.
fn render_from_files(dir: &Path, width: Width) -> String {
    use std::collections::BTreeMap;
    // (library, method) -> cell, for this width.
    let mut data: BTreeMap<String, BTreeMap<String, FileCell>> = BTreeMap::new();
    let mut modes: BTreeMap<String, String> = BTreeMap::new();

    let libs = LIBS;
    for lib in libs {
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
            if cols[1] != width.name() {
                continue;
            }
            // Per-width LSBε table renders the canonical-scale cell (the
            // full multi-scale surface is in the TSVs + the grade summary).
            if cols[2].parse::<u32>().ok() != Some(width.canonical_scale()) {
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

    let methods: Vec<&str> = Method::TRANSCENDENTAL.iter().map(|m| m.name()).collect();

    let mut out = String::new();
    out.push_str(&format!(
        "## {} (scale {}) — LSBε (ULP)\n\n",
        width.name(),
        width.canonical_scale()
    ));
    out.push_str(
        "Cell = LSBε (max |ULP distance to true|). 0 (0) = correctly \
         rounded (bit-exact under the subject's reported mode). `n/a` = \
         method not exposed or width/scale not representable.\n\n",
    );
    out.push_str("| library | mode |");
    for m in &methods {
        out.push_str(&format!(" {m} |"));
    }
    out.push('\n');
    out.push_str("|---|---|");
    for _ in &methods {
        out.push_str("---|");
    }
    out.push('\n');

    for lib in libs {
        let row = data.get(lib);
        let mode = modes.get(lib).cloned().unwrap_or_else(|| "-".to_string());
        out.push_str(&format!("| {lib} | {mode} |"));
        for m in &methods {
            let cell = row.and_then(|r| r.get(*m));
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

/// The library roster, in render order. Single source for both the
/// per-width tables and the grade report.
const LIBS: [&str; 7] = [
    "decimal-scaled",
    "fastnum",
    "rust_decimal",
    "dashu-float",
    "decimal-rs",
    "bigdecimal",
    "g_math",
];

/// One library's parsed result rows read back from its committed TSV.
struct LibRows {
    /// (method, width, scale, kind, max_digits, reach_scale) for every
    /// persisted row. `max_digits` is the exact decimal error-digit length
    /// the fidelity grade scores (`-1` for an n/a cell); `reach_scale` is
    /// the depth the row was graded at (the subject's own last digit),
    /// equal to `scale` for arbitrary-precision peers and decimal-scaled.
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
            // Need the max_digits column (index 9). `reach_scale` (index 10)
            // is a newer column; older TSVs default it to `scale` for
            // back-compat, but those rows pre-date grade-at-own-last-digit
            // and should be regenerated.
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
/// minimal width by `cell_demerit`, which clamps to ≥ 1).
fn tier_width_of(width: &str) -> u32 {
    width
        .trim_start_matches(['D', 'd'])
        .parse::<u32>()
        .unwrap_or(0)
}

/// Reach (precision/depth) statistics, accumulated over the executed cells
/// of one library. Independent of rounding correctness: a peer that grades
/// 100% correctly-rounded at every cell but only at depth 28 (rust_decimal
/// on D76 cells, scale 38) shows up as `mean_reach = 28`, while a peer
/// that reaches every cell shows `mean_reach == mean_cell_scale`. The two
/// metrics — closeness/%CR and reach — are reported side by side so a
/// shallower peer is credited for being accurate at its own last digit
/// AND honestly shown as not reaching deeper.
#[derive(Clone, Copy, Default)]
struct ReachStats {
    /// Sum of `reach_scale` over executed cells.
    sum_reach: u64,
    /// Sum of `cell_scale` over executed cells.
    sum_cell: u64,
    /// Cells where the subject reached the full cell scale (`reach == cell_scale`).
    full_reach_cells: usize,
    /// Total executed cells.
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
        if self.cells == 0 {
            0.0
        } else {
            self.sum_reach as f64 / self.cells as f64
        }
    }
    fn mean_cell(&self) -> f64 {
        if self.cells == 0 {
            0.0
        } else {
            self.sum_cell as f64 / self.cells as f64
        }
    }
    fn full_reach_pct(&self) -> f64 {
        if self.cells == 0 {
            0.0
        } else {
            100.0 * self.full_reach_cells as f64 / self.cells as f64
        }
    }
}

/// Render the fidelity grade report by READING the committed per-library
/// TSVs, with the rubric published inline. Generated from the single
/// committed source like every other table here.
///
/// Layout, top-down: (1) the OVERALL table (one row per library: closeness
/// score, %CR, the two-letter grade, runnable cells — best score first);
/// (2) the per-function tables (each function's score / %CR / grade per
/// library); (3) coverage; with the rubric/explanation block up front.
fn render_fidelity_report(dir: &Path) -> String {
    use std::collections::BTreeMap;
    use std::fmt::Write as _;

    let total_cells = golden_scan().len();

    let mut out = String::new();
    out.push_str("# Fidelity grades — generated from results/precision/*.tsv\n\n");
    out.push_str(fidelity_rubric());
    out.push('\n');

    // Collect per-library per-function fidelity + a pooled per-library
    // accumulator + a runnable counter + reach stats, in one pass. The
    // demerit is precision-relative, so each cell folds in its OWN tier
    // width. `reach` is the depth the cell was graded at (subject's own
    // last digit) — independent of rounding correctness, surfaces shallower
    // peers without penalising them as rounding failures.
    let mut graded: BTreeMap<
        &str,
        (BTreeMap<String, Fidelity>, Fidelity, usize, ReachStats),
    > = BTreeMap::new();
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
            per_fn
                .entry(method.clone())
                .or_default()
                .record(*max_digits as u32, tw);
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
         failure: it is scored as correctly-rounded-at-its-cap (or off-by-N \
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
            out.push_str(
                "_No runnable cells (library exposes none of the scanned methods)._\n\n",
            );
            continue;
        }

        // Per-function rows, in canonical method order.
        out.push_str("| function | cells | mean_demerit | score | %CR | grade |\n");
        out.push_str("|---|---|---|---|---|---|\n");
        for m in Method::TRANSCENDENTAL {
            let name = m.name();
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
         on a cell deeper than its cap is RUNNABLE (graded at its own last \
         digit, shallower reach reported in the overall table) — runnable no \
         longer narrows just because our scale is deeper.\n\n",
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
