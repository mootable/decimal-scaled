// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Peer-library performance bench — decimal-scaled timed BESIDE the competitor
//! decimal libraries over the same golden set. The library analog of the
//! version-history `history_all` gate (`decimal-scale-test/tests/history.rs`):
//! same shape, but the subjects are the competitor libraries (plus decimal-scaled
//! itself) instead of pinned past versions.
//!
//! It lives in `golden-competitors`, NOT `decimal-scale-test`, because the peer
//! libraries are dependencies of THIS crate only (never the core library or the
//! agnostic harness). The decimal-scaled subject [`DsSubject`] comes from
//! `decimal-scale-test`, which `golden-competitors` already depends on, so this is
//! the one crate that can pair both sides on the same harness.
//!
//! Like the history gate this is a FILTERED slice of golden — one representative
//! scale-30 cell per width, capped rows per function, a single mode — so
//! the timing run is cheap; the full correctness surface is the golden gate's job.
//! Each subject runs over the cell's golden rows with the `Timed` strategy as a
//! free ride-along (exactly ONE timed call per golden row), aggregated to per-cell
//! medians downstream. Timing is always REPORTED, never asserted (this is a perf
//! bench, not a correctness gate). Each competitor declares only the functions its
//! library exposes, so a missing `(function, library)` cell simply does not emit
//! (the docs render the gap).
//!
//! decimal-scaled is timed at scale 30 (the nearest compiled scale to 30 per width —
//! the peers' effective precision); every competitor runs over the
//! same golden rows and the harness skips the inputs it cannot represent, so a
//! fixed-precision peer drops out on the wide tiers (an honest gap), and a peer
//! that has no equivalent for a function never emits a row for it.
//!
//! Honours the `GOLDEN_*` env filters (`GOLDEN_WIDTHS` / `GOLDEN_SCALES` /
//! `GOLDEN_MODES` / `GOLDEN_FUNCS`), so a focused slice runs in seconds. Set
//! `LIBPERF_PARALLEL=<N>` (N ≥ 2) for N-thread execution on the wide tiers (CI
//! uses this — see `runner()`). When `LIBPERF_REPORT_DIR` is set the full per-line
//! surface is written there as one TSV per shard (named by `GOLDEN_WIDTHS`); the
//! CI aggregate collapses it to per-(library, function, width) medians.
//!
//! ```text
//! GOLDEN_WIDTHS=18 GOLDEN_FUNCS=sqrt,add \
//!   cargo test -p golden-competitors --release \
//!     --test lib_perf lib_perf_all -- --ignored --nocapture
//! ```

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use decimal_scaled_golden::{
    CaseLoader, DecimalSubject, ExecutionResult, FileLoader, Function, GoldenCase, GoldenRunner,
    Limits, Outcome, OverflowValidator, ParallelRunner, RoundingMode, RoundingValidator,
    SequentialRunner, SubjectCollector, Timed,
};

use decimal_scale_test::{golden_dir, DsSubject, Filter, GEN_PRECISION, GUARD};
use golden_competitors::{
    BigDecimalSubject, DashuFloat, DecimalRsSubject, FastNum, GMath, RustDecimal,
};

/// Serialises the gate's run: it swaps the process-global panic hook (competitor
/// overflow / domain edges panic by contract, caught by the harness) and the
/// `Timed` medians shouldn't interleave with another test's.
static HOOK_GUARD: Mutex<()> = Mutex::new(());

/// Timed-strategy executions per golden row: exactly ONE, so the wall-clock signal
/// rides along the run for free — one extra call per row, never a timing loop over
/// the row set. Per-row noise washes out in the per-cell MEDIANS computed downstream
/// (the timing is advisory: reported, never asserted).
const TIMED_EXECUTIONS: u32 = 1;

/// Cap on golden rows timed per function (per cell). Up to a thousand single-call
/// samples spread across the file's magnitude range give a stable median (and a
/// representative min/max spread) while keeping the wide-tier kernels — and the
/// seven-subject fan-out — affordable.
const ROW_CAP: usize = 1000;

/// The library roster, decimal-scaled first, in render order — mirrors the
/// precision shootout's `LIBS` so the Comparisons page roster matches the Precision
/// page's. (`golden-competitors` also carries an `F64` reference subject; it is a
/// binary-radix baseline, not a peer decimal crate, so it is excluded here exactly
/// as the precision shootout excludes it.)
const LIBS: [&str; 7] = [
    "decimal-scaled",
    "fastnum",
    "rust_decimal",
    "dashu-float",
    "decimal-rs",
    "bigdecimal",
    "g_math",
];

/// Dispatch enum so the per-subject run stays singly generic (`GoldenRunner::run`
/// is generic over `S`, which makes the trait not object-safe).
enum LibRunner {
    Sequential(SequentialRunner<Timed>),
    Parallel(ParallelRunner<Timed>),
}

impl GoldenRunner for LibRunner {
    fn run<S: DecimalSubject + Sync>(
        &self,
        subject: &S,
        functions: &[Function],
    ) -> SubjectCollector {
        match self {
            LibRunner::Sequential(r) => r.run(subject, functions),
            LibRunner::Parallel(r) => r.run(subject, functions),
        }
    }
}

/// A read-once, capped golden loader: each function's file is parsed ONCE, strided
/// down to at most [`ROW_CAP`] rows (spanning the file's magnitude range, not just
/// its head — a head cap would bias toward the leading region), then that small
/// capped set is cached and cloned per request. The seven-subject × per-cell fan-out
/// would otherwise re-read + re-parse the multi-MB golden files thousands of times.
struct CappedLoader {
    inner: FileLoader,
    cache: Mutex<HashMap<Function, Arc<Vec<GoldenCase>>>>,
}

impl CappedLoader {
    /// Capped loader over the committed golden set. `GOLDEN_DIR` overrides the
    /// compile-time path (a staged CI exe runs on a different checkout than the one
    /// that baked `CARGO_MANIFEST_DIR`).
    fn golden() -> CappedLoader {
        let dir = match std::env::var("GOLDEN_DIR") {
            Ok(d) if !d.trim().is_empty() => d,
            _ => golden_dir().to_string(),
        };
        CappedLoader { inner: FileLoader::new(&dir), cache: Mutex::new(HashMap::new()) }
    }
}

/// At most `cap` cases spread EVENLY across `cases` (deterministic stride): sample
/// index `i` maps to population index `i * len / cap`. The head would bias the cell
/// toward the golden file's leading magnitudes.
fn stride_cap(cases: Vec<GoldenCase>, cap: usize) -> Vec<GoldenCase> {
    let len = cases.len();
    if len <= cap {
        return cases;
    }
    (0..cap).map(|i| cases[i * len / cap].clone()).collect()
}

impl CaseLoader for CappedLoader {
    fn load(&self, function: Function) -> Cow<'_, [GoldenCase]> {
        let arc = self
            .cache
            .lock()
            .unwrap()
            .entry(function)
            .or_insert_with(|| Arc::new(stride_cap(self.inner.load(function).into_owned(), ROW_CAP)))
            .clone();
        Cow::Owned((*arc).clone())
    }

    fn oracle_limits(&self) -> Limits {
        // Verifiable depth = generation precision minus the rounding-guard digits
        // (matches the golden gate's CachingLoader).
        Limits {
            min_value: None,
            max_value: None,
            max_precision: (GEN_PRECISION - GUARD) as u32,
            max_significant_digits: None,
        }
    }
}

/// One representative cell per width, pinned to **scale 30** — the precision the
/// peer libraries effectively compute at (~28-38 digits; see [`compare_scale`]). A
/// like-for-like speed comparison needs comparable work per call, so decimal-scaled
/// is timed at ~30 fractional digits rather than its full per-width reach (hundreds
/// of digits at the wide tiers the peers never match). Each width takes the compiled
/// scale nearest 30; a width too narrow for 30 (only D18, MAX_SCALE 17) falls back
/// to half its max scale. Respects `GOLDEN_WIDTHS`/`GOLDEN_SCALES` and the
/// compiled-tier set (delegated to `filter.cells()`).
fn lib_cells(filter: &Filter) -> Vec<(u32, u32)> {
    let mut by_width: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (w, s) in filter.cells() {
        by_width.entry(w).or_default().push(s);
    }
    by_width
        .into_iter()
        .map(|(w, mut scales)| {
            scales.sort_unstable();
            (w, compare_scale(&scales))
        })
        .collect()
}

/// The comparison scale for one width's sorted compiled-scale list: **30** (the
/// peers' effective precision) where the width can hold it, else the nearest
/// compiled scale to 30. A width too narrow for 30 (max scale < 30 — only D18)
/// targets half its max scale instead, so it still does a representative fraction
/// of work rather than its full reach.
fn compare_scale(scales: &[u32]) -> u32 {
    let max_scale = *scales.last().expect("each width has at least one compiled cell");
    let target = if max_scale >= 30 { 30 } else { max_scale / 2 };
    *scales
        .iter()
        .min_by_key(|&&s| (s as i64 - target as i64).abs())
        .expect("each width has at least one compiled cell")
}

/// Build the runner, honouring `LIBPERF_PARALLEL`:
///
/// - **Absent / empty → `SequentialRunner`**: contention-free `Timed` medians for
///   the narrow / mid tiers where a sequential sweep is affordable.
/// - **`LIBPERF_PARALLEL=N` (N ≥ 2) → `ParallelRunner { threads: N }`**: the wide
///   tiers, where a sequential sweep over decimal-scaled exceeds the CI budget. The
///   ride-along medians are noisier (cross-core jitter) but the timing is advisory
///   anyway — the perf source of truth is the bbc over the full surface.
fn runner() -> LibRunner {
    let threads: usize = std::env::var("LIBPERF_PARALLEL")
        .ok()
        .filter(|v| !v.is_empty())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    if threads >= 2 {
        LibRunner::Parallel(ParallelRunner {
            threads,
            strategy: Timed { number_of_executions: TIMED_EXECUTIONS },
            loader: Box::new(CappedLoader::golden()),
            validators: vec![
                Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
                Box::new(OverflowValidator),
            ],
        })
    } else {
        LibRunner::Sequential(SequentialRunner {
            strategy: Timed { number_of_executions: TIMED_EXECUTIONS },
            loader: Box::new(CappedLoader::golden()),
            validators: vec![
                Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
                Box::new(OverflowValidator),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// Cell flattening — one comparable record per golden cell per library.
// ---------------------------------------------------------------------------

/// How one cell went, judged against the subject's OWN declared contract.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Grade {
    /// Not run (unrepresentable input / unsupported function).
    Skip,
    /// Every verdict is Pass (or the informational Precision).
    Pass,
    /// At least one failing verdict.
    Fail,
}

/// One cell across the comparison: `(width, scale, mode, function, golden line)`.
type Key = (u32, u32, String, &'static str, usize);

struct Cell {
    grade: Grade,
    timing: Option<u64>,
}

/// A failing verdict — everything except Pass, the informational Precision, and the
/// runner-level Skipped (mirrors the console reporter's notion).
fn is_failure(o: &Outcome) -> bool {
    !matches!(o, Outcome::Pass | Outcome::Precision { .. } | Outcome::Skipped)
}

/// Flatten one library's collector into keyed cells. The `(width, scale, mode)`
/// come from the CALLER's loop, not the subject's `Capabilities::config` — the
/// competitor subjects carry no config (only `DsSubject` does), and the cell a
/// competitor is being COMPARED at is the decimal-scaled cell we drove it on.
fn flatten_into(sc: &SubjectCollector, w: u32, s: u32, mode: RoundingMode, into: &mut BTreeMap<Key, Cell>) {
    let modestr = format!("{mode:?}");
    for fc in &sc.functions {
        for cell in &fc.cells {
            let grade = match cell.result() {
                Some(ExecutionResult::Skipped) | None => Grade::Skip,
                Some(ExecutionResult::HarnessError(_)) => Grade::Fail,
                Some(ExecutionResult::Computed(_)) => {
                    if cell.validations.iter().any(is_failure) {
                        Grade::Fail
                    } else {
                        Grade::Pass
                    }
                }
            };
            into.insert(
                (w, s, modestr.clone(), fc.function.name(), cell.line),
                Cell { grade, timing: cell.timing },
            );
        }
    }
}

/// Run one library subject over one `(width, scale, mode)` cell and flatten it into
/// the per-library map. Generic over `S` so each competitor's concrete subject type
/// reuses the one runner instance.
fn collect<S: DecimalSubject + Sync>(
    runner: &LibRunner,
    subject: &S,
    funcs: &[Function],
    library: &'static str,
    w: u32,
    s: u32,
    mode: RoundingMode,
    per_lib: &mut BTreeMap<&'static str, BTreeMap<Key, Cell>>,
) {
    let sc = runner.run(subject, funcs);
    flatten_into(&sc, w, s, mode, per_lib.entry(library).or_default());
}

/// Median of a sample (`None` when empty).
fn median(mut xs: Vec<u64>) -> Option<u64> {
    if xs.is_empty() {
        return None;
    }
    xs.sort_unstable();
    Some(xs[xs.len() >> 1])
}

/// Per-function timing medians for one flattened library.
fn timing_medians(cells: &BTreeMap<Key, Cell>) -> BTreeMap<&'static str, u64> {
    let mut per_func: BTreeMap<&'static str, Vec<u64>> = BTreeMap::new();
    for (&(_, _, _, func, _), cell) in cells {
        if let Some(ns) = cell.timing {
            per_func.entry(func).or_default().push(ns);
        }
    }
    per_func.into_iter().filter_map(|(f, xs)| median(xs).map(|m| (f, m))).collect()
}

// ---------------------------------------------------------------------------
// The gate — decimal-scaled vs every peer, one timed run.
// ---------------------------------------------------------------------------

/// decimal-scaled timed beside every competitor library over one scale-30 cell per
/// width, capped rows, single mode. Per-line timing is written to
/// `LIBPERF_REPORT_DIR` (one TSV per shard) and a per-library median summary is
/// printed — all REPORTED, never asserted. Heavy; dispatch/on-demand only.
///
/// SPECIALIST gate: a deliberate opt-in cost switch with a CI venue (lib-perf.yml
/// runs it with `-- --ignored`), NOT a parked ignore.
#[test]
#[ignore = "peer-library perf bench; heavy, run on demand via --ignored --nocapture"]
fn lib_perf_all() {
    let _hook_guard = HOOK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
    let filter = Filter::from_env();
    let modes = filter.modes(&[RoundingMode::HalfToEven]);
    let cells = lib_cells(&filter);
    let funcs = filter.funcs();
    let runner = runner();

    // Competitors panic on overflow / domain edges by contract (the harness catches
    // each as `Computed::Panic`); silence the default hook so the sweep isn't drowned
    // in backtraces.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));

    let mut per_lib: BTreeMap<&'static str, BTreeMap<Key, Cell>> = BTreeMap::new();
    for &(w, s) in &cells {
        for &mode in &modes {
            // decimal-scaled at this tier's scale-30 cell, then every peer over the same
            // golden rows (the harness skips the inputs each peer cannot represent).
            collect(&runner, &DsSubject::with_mode(w, s, mode), funcs, "decimal-scaled", w, s, mode, &mut per_lib);
            collect(&runner, &FastNum, funcs, "fastnum", w, s, mode, &mut per_lib);
            collect(&runner, &RustDecimal, funcs, "rust_decimal", w, s, mode, &mut per_lib);
            collect(&runner, &DashuFloat, funcs, "dashu-float", w, s, mode, &mut per_lib);
            collect(&runner, &DecimalRsSubject, funcs, "decimal-rs", w, s, mode, &mut per_lib);
            collect(&runner, &BigDecimalSubject, funcs, "bigdecimal", w, s, mode, &mut per_lib);
            collect(&runner, &GMath, funcs, "g_math", w, s, mode, &mut per_lib);
        }
    }

    std::panic::set_hook(prev_hook);

    // Publish the full per-line surface as a committable TSV when CI sets
    // LIBPERF_REPORT_DIR — keyed by LIBRARY (the history emit keyed by version). One
    // file per shard (named by GOLDEN_WIDTHS) so the aggregate splices without
    // collision; the aggregate collapses the line dimension to per-cell medians.
    if let Some(dir) = std::env::var_os("LIBPERF_REPORT_DIR") {
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).expect("create LIBPERF_REPORT_DIR");
        let mut out = String::from("library\tfunction\twidth\tscale\tmode\tline\tgrade\tnanos\n");
        for (library, lib_cells) in &per_lib {
            for ((w, s, mode, func, line), cell) in lib_cells {
                let grade = match cell.grade {
                    Grade::Pass => "pass",
                    Grade::Fail => "fail",
                    Grade::Skip => "skip",
                };
                let ns = cell.timing.map(|n| n.to_string()).unwrap_or_default();
                out.push_str(&format!(
                    "{library}\t{func}\t{w}\t{s}\t{mode}\t{line}\t{grade}\t{ns}\n"
                ));
            }
        }
        let shard = std::env::var("GOLDEN_WIDTHS").unwrap_or_else(|_| "all".into()).replace(',', "_");
        std::fs::write(dir.join(format!("lib_perf-{shard}.tsv")), out).expect("write lib_perf tsv");
    }

    // Per-library, per-function median summary (advisory; reported, never asserted).
    eprintln!("== lib_perf_all: decimal-scaled vs peers (median ns/call across rows; reported, never asserted) ==");
    let mut total = 0usize;
    for library in LIBS {
        let Some(lib_cells) = per_lib.get(library) else { continue };
        total += lib_cells.len();
        let medians = timing_medians(lib_cells);
        eprintln!("-- {library} --");
        eprintln!("{:<8} {:>6} {:>6} {:>6} {:>14}", "func", "pass", "fail", "skip", "median-ns");
        let mut per_func: BTreeMap<&'static str, (usize, usize, usize)> = BTreeMap::new();
        for (&(_, _, _, func, _), cell) in lib_cells {
            let e = per_func.entry(func).or_default();
            match cell.grade {
                Grade::Pass => e.0 += 1,
                Grade::Fail => e.1 += 1,
                Grade::Skip => e.2 += 1,
            }
        }
        for (func, (pass, fail, skip)) in &per_func {
            let ns = medians.get(func).map(|n| n.to_string()).unwrap_or_else(|| "-".into());
            eprintln!("{func:<8} {pass:>6} {fail:>6} {skip:>6} {ns:>14}");
        }
    }

    // Sanity (not a timing assertion): the run actually exercised cells. Guards a
    // mis-filtered / mis-featured invocation that would otherwise be a silent no-op.
    assert!(total > 0, "lib_perf collected no cells (check GOLDEN_WIDTHS vs the compiled tier features)");
}
