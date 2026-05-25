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

/// The widths swept. The transcendental golden tables embedded in the
/// harness cover D38 (full surface), a D76 subset, and D307<150> (the
/// deep-scale tier, full surface); the table renders the cells that have
/// an oracle.
const WIDTHS: [Width; 3] = [Width::D38, Width::D76, Width::D307];

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

    // ── 1. Sweep + persist per-library result files ────────────────
    for subject in &subjects {
        let mut rows: Vec<Row> = Vec::new();
        // Deterministic key order: width (narrow→wide) then method
        // (canonical order).
        for &width in &WIDTHS {
            for &method in &Method::TRANSCENDENTAL {
                let cell = score_cell(subject.as_ref(), method, width, DRIVE_MODE, SAMPLE_CAP);
                match cell {
                    None => continue, // no oracle table for this cell
                    Some(c) if c.scored == 0 => rows.push(Row {
                        method: method.name(),
                        width: width.name(),
                        scale: width.canonical_scale(),
                        rounding: format!("{:?}", subject.native_mode()),
                        kind: "na",
                        max_lsbe: -1,
                        max_ulp: "na".to_string(),
                        scored: 0,
                        correctly_rounded: 0,
                    }),
                    Some(c) => rows.push(Row {
                        method: method.name(),
                        width: width.name(),
                        scale: width.canonical_scale(),
                        rounding: format!("{:?}", subject.native_mode()),
                        kind: "executed",
                        max_lsbe: c.max_lsbe as i64,
                        max_ulp: fmt_ulp(c.max_ulp),
                        scored: c.scored,
                        correctly_rounded: c.correctly_rounded,
                    }),
                }
            }
        }
        write_tsv(&out_dir, subject.name(), &rows);
    }

    // ── 2. Render the shootout table FROM the committed files ───────
    println!("# Precision shootout — generated from results/precision/*.tsv\n");
    for &width in &WIDTHS {
        let table = render_from_files(&out_dir, width);
        println!("{table}");
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
         # result columns: kind(executed|na)\tmax_lsbe\tmax_ulp\tscored\tcorrectly_rounded\n\
         # source: benches/lib_cmp_precision.rs (regenerate with \
         `cargo bench --bench lib_cmp_precision --features wide,x-wide,xx-wide,macros`)"
    )
    .unwrap();
    writeln!(
        f,
        "method\twidth\tscale\trounding\tkind\tmax_lsbe\tmax_ulp\tscored\tcorrectly_rounded"
    )
    .unwrap();
    for r in rows {
        writeln!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.method,
            r.width,
            r.scale,
            r.rounding,
            r.kind,
            r.max_lsbe,
            r.max_ulp,
            r.scored,
            r.correctly_rounded
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

    let libs = [
        "decimal-scaled",
        "fastnum",
        "rust_decimal",
        "dashu-float",
        "decimal-rs",
        "bigdecimal",
        "g_math",
    ];
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
