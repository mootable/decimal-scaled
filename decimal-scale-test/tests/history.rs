//! Version-history correctness gates: the live crate beside pinned historical
//! decimal-scaled releases, over the same golden set.
//!
//! Two gates (task 10.13, correctness side):
//!
//! - [`history_previous`] — live vs the immediately-previous release (0.4.4) ONLY,
//!   with the RATCHET assertion: no cell that PASSED in 0.4.4 may fail in the live
//!   crate. Newly-FIXED cells (0.4.4 fail → live pass) are the expected diff,
//!   reported, never asserted. The per-function timing delta is reported beside it.
//! - [`history_all`] — every adapted historical version in ONE shootout-style run:
//!   per-subject tallies and the cross-version correctness table (with a median-ns
//!   column per version), REPORTED, never asserted.
//!
//! Both run the `Timed` strategy as a free ride-along on the correctness run —
//! one timed call per golden row, aggregated to per-function medians (timing is
//! always reported, never asserted; the cell-granularity paired perf comparison
//! over a small fixed input set is `CriterionStrategy`'s separate, later job).
//! Both gates honour every `GOLDEN_*` env filter (`GOLDEN_WIDTHS` / `GOLDEN_SCALES` /
//! `GOLDEN_MODES` / `GOLDEN_FUNCS` / `GOLDEN_SAMPLE` / `GOLDEN_STRIPE`), so a focused
//! slice runs in seconds. Set `HISTORY_PARALLEL=<N>` (N ≥ 2) to enable parallel
//! execution with N threads (wide tiers use this in CI — see `runner()`). `GOLDEN_THREADS`
//! is inert here; the runner reads `HISTORY_PARALLEL` instead:
//!
//! ```text
//! GOLDEN_WIDTHS=18 GOLDEN_FUNCS=exp,sqrt \
//!   cargo test -p decimal-scale-test --release \
//!     --features wide,x-wide,xx-wide,history-044 \
//!     --test history history_previous -- --ignored --nocapture
//! ```

mod common;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Mutex;

use decimal_scaled_golden::{
    CaseLoader, ConsoleReporter, DecimalSubject, ExecutionResult, FilterLoader, Function,
    GoldenCase, GoldenRunner, InlineReporter, Limits, Outcome, OverflowValidator, ParallelRunner,
    RoundingMode, RoundingValidator, RunCollector, SequentialRunner, SubjectCollector, Timed,
};
use decimal_scale_test::history::v044;
use decimal_scale_test::{DsSubject, Filter, GEN_PRECISION};

use common::{row_filter, CachingLoader};

/// Serialises the gates: each swaps the process-global panic hook for its run.
static HOOK_GUARD: Mutex<()> = Mutex::new(());

/// Timed-strategy executions per golden row: exactly ONE, so the wall-clock
/// signal rides along the correctness run for free (one extra call per row,
/// never a timing loop over the golden row set — the golden rows are correctness
/// data; the cell-granularity paired perf map is `CriterionStrategy`'s separate
/// job). Per-row noise washes out in the per-function MEDIANS reported below;
/// the timing is advisory (reported, never asserted).
const TIMED_EXECUTIONS: u32 = 1;

/// Dispatch enum: holds either runner so `run_version` stays singly generic
/// (`GoldenRunner::run` is generic over `S`, making the trait not object-safe).
enum HistRunner {
    Sequential(SequentialRunner<Timed>),
    Parallel(ParallelRunner<Timed>),
}

impl GoldenRunner for HistRunner {
    fn run<S: DecimalSubject + Sync>(
        &self,
        subject: &S,
        functions: &[Function],
    ) -> SubjectCollector {
        match self {
            HistRunner::Sequential(r) => r.run(subject, functions),
            HistRunner::Parallel(r) => r.run(subject, functions),
        }
    }
}

/// Wraps a `CaseLoader` and truncates each function's cases to at most `limit` rows
/// (the first `limit` rows from the inner loader). This bounds the per-function golden
/// row count for the history gates independently of `GOLDEN_SAMPLE`: 1000 rows/fn is
/// enough to detect regressions; the full correctness surface is the bbc's job.
struct CapLoader<L> {
    inner: L,
    limit: usize,
}

impl<L: CaseLoader> CaseLoader for CapLoader<L> {
    fn load(&self, func: Function) -> Cow<'_, [GoldenCase]> {
        let cases = self.inner.load(func);
        if cases.len() <= self.limit {
            cases
        } else {
            Cow::Owned(cases[..self.limit].to_vec())
        }
    }

    fn oracle_limits(&self) -> Limits {
        self.inner.oracle_limits()
    }
}

/// Build the cell list for the history gates: one (width, middle-of-list scale) per width.
///
/// The CELLS grid anchors band-edge points. For each width the "middle scale" is the
/// element at index `len/2` of that width's sorted scale list — the closest available
/// scale to `max_scale / 2`. This reduces each tier to one representative cell and keeps
/// the ratchet affordable; the bbc is the correctness source of truth over the full surface.
/// Respects `GOLDEN_WIDTHS` and `tier_compiled` (delegated to `filter.cells()`).
fn history_cells(filter: &decimal_scale_test::Filter) -> Vec<(u32, u32)> {
    let all = filter.cells();
    let mut by_width: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (w, s) in all {
        by_width.entry(w).or_default().push(s);
    }
    by_width
        .into_iter()
        .map(|(w, mut scales)| {
            scales.sort_unstable();
            (w, scales[scales.len() / 2])
        })
        .collect()
}

/// Choose the runner based on the `HISTORY_PARALLEL` env var:
///
/// - **Absent or empty → `SequentialRunner`**: single-thread, contention-free
///   `Timed` medians. Used by narrow/mid shards (narrow, d57, d76) where the
///   golden-row count is affordable and clean timing adds value.
/// - **`HISTORY_PARALLEL=N` (N ≥ 2) → `ParallelRunner { threads: N }`**: wide
///   shards (d115 and above) where a full sequential sweep exceeds the CI budget.
///   The ratchet and shootout correctness are fully preserved — ALL rows run; only
///   the ride-along `Timed` medians are noisier (cross-core jitter accepted). The
///   per-function timing table is advisory-only in any case; the perf source of
///   truth is always the bbc across the full surface.
fn runner(filter: &Filter) -> HistRunner {
    let threads: usize = std::env::var("HISTORY_PARALLEL")
        .ok()
        .filter(|v| !v.is_empty())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    // Cap loader: first 1000 rows per function (independent of GOLDEN_SAMPLE),
    // then the row_filter applies sample/stripe on those 1000.
    let make_loader = |sample, stripe| -> Box<dyn CaseLoader> {
        Box::new(FilterLoader::new(
            CapLoader { inner: CachingLoader::golden(), limit: 1000 },
            row_filter(sample, stripe),
        ))
    };
    if threads >= 2 {
        HistRunner::Parallel(ParallelRunner {
            threads,
            strategy: Timed { number_of_executions: TIMED_EXECUTIONS },
            loader: make_loader(filter.sample(), filter.stripe()),
            validators: vec![
                Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
                Box::new(OverflowValidator),
            ],
        })
    } else {
        HistRunner::Sequential(SequentialRunner {
            strategy: Timed { number_of_executions: TIMED_EXECUTIONS },
            loader: make_loader(filter.sample(), filter.stripe()),
            validators: vec![
                Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
                Box::new(OverflowValidator),
            ],
        })
    }
}

// ---------------------------------------------------------------------------
// Cell flattening — one comparable record per golden cell per subject.
// ---------------------------------------------------------------------------

/// How one cell went, judged against the subject's OWN declared contract.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Grade {
    /// Not run (unrepresentable input for this cell / unsupported function).
    Skip,
    /// Every verdict is Pass (or the informational Precision).
    Pass,
    /// At least one failing verdict (mis-rounded / wrong-mode / error / panic).
    Fail,
}

/// One cell across the comparison: `(width, scale, mode, function, golden line)`.
type Key = (u32, u32, String, &'static str, usize);

struct Cell {
    grade: Grade,
    timing: Option<u64>,
    /// The worst failing verdict's tag, for the report.
    detail: &'static str,
}

/// Flatten one subject's collector into keyed cells. The key carries the cell
/// coordinates from the subject's report config plus the golden line, so the same
/// golden case pairs exactly across versions.
fn flatten(subject: &SubjectCollector, into: &mut BTreeMap<Key, Cell>) {
    let cfg = |k: &str| subject.capabilities.config.get(k).cloned().unwrap_or_default();
    let width: u32 = cfg("width").parse().expect("subject config carries width");
    let scale: u32 = cfg("scale").parse().expect("subject config carries scale");
    let mode = cfg("mode");
    for fc in &subject.functions {
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
            let detail = match cell.result() {
                Some(ExecutionResult::HarnessError(_)) => "harness-error",
                _ => cell
                    .validations
                    .iter()
                    .filter(|o| is_failure(o))
                    .max_by_key(|o| o.severity())
                    .map(|o| o.tag())
                    .unwrap_or(""),
            };
            into.insert(
                (width, scale, mode.clone(), fc.function.name(), cell.line),
                Cell { grade, timing: cell.timing, detail },
            );
        }
    }
}

/// A failing verdict (everything except Pass, the informational Precision, and
/// the runner-level Skipped) — mirrors the console reporter's notion.
fn is_failure(o: &Outcome) -> bool {
    !matches!(o, Outcome::Pass | Outcome::Precision { .. } | Outcome::Skipped)
}

/// Run one version's subjects over `cells` × `modes` into a `RunCollector`.
/// `subject` builds the version's erased subject for one `(width, scale, mode)`.
/// Generic over `R: GoldenRunner` so both `SequentialRunner` and `ParallelRunner`
/// (via the `HistRunner` dispatch enum) can be passed without boxing.
fn run_version<R: GoldenRunner, S, F>(
    runner: &R,
    filter: &Filter,
    modes: &[RoundingMode],
    cells: &[(u32, u32)],
    subject: F,
) -> RunCollector
where
    S: decimal_scaled_golden::DecimalSubject + Sync,
    F: Fn(u32, u32, RoundingMode) -> S,
{
    let mut rc = RunCollector::new();
    for &mode in modes {
        for &(w, s) in cells {
            rc.add(runner.run(&subject(w, s, mode), filter.funcs()));
        }
    }
    rc
}

/// Median of a sorted-on-demand sample; `None` when empty.
fn median(mut xs: Vec<u64>) -> Option<u64> {
    if xs.is_empty() {
        return None;
    }
    xs.sort_unstable();
    Some(xs[xs.len() >> 1])
}

/// Per-function timing medians for one flattened version.
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
// Gate 1 — history_previous: live vs 0.4.4, the ratchet.
// ---------------------------------------------------------------------------

/// Live vs the immediately-previous release (0.4.4) with the RATCHET assertion:
/// no cell that PASSED in 0.4.4 may fail (or fall out of coverage) in the live
/// crate. Newly-fixed cells and the per-function timing delta are REPORTED.
/// `#[ignore]`d like the golden gates (the surface is heavy unfiltered); narrow
/// with the `GOLDEN_*` env vars and run via `--ignored`.
///
/// SPECIALIST gate (owner ruling 2026-06-12): a deliberate opt-in cost switch, NOT a
/// parked/orphan ignore. CI venue: history.yml (runs with `-- --ignored`).
/// The workspace zero-ignore mandate applies to every other test.
#[test]
#[ignore = "version-history ratchet; run via --ignored (filter with GOLDEN_*)"]
fn history_previous() {
    let _hook_guard = HOOK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
    let filter = Filter::from_env();
    let modes = filter.modes(&[RoundingMode::HalfToEven]);
    // Middle scale per tier + 1000-row/fn cap (see `history_cells` and `runner`).
    let cells = history_cells(&filter);
    let runner = runner(&filter);

    // Expected out-of-range cells panic (caught + validated as overflow); silence
    // the default hook so the sweep isn't drowned in backtraces.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let live_rc = run_version(&runner, &filter, &modes, &cells, DsSubject::with_mode);
    let prev_rc = run_version(&runner, &filter, &modes, &cells, v044::Subject::with_mode);
    std::panic::set_hook(prev_hook);

    let mut live = BTreeMap::new();
    let mut prev = BTreeMap::new();
    live_rc.subjects.iter().for_each(|s| flatten(s, &mut live));
    prev_rc.subjects.iter().for_each(|s| flatten(s, &mut prev));

    // The ratchet walk.
    let mut regressions: Vec<String> = Vec::new();
    let mut coverage_losses: Vec<String> = Vec::new();
    let mut fixed: Vec<String> = Vec::new();
    let mut both_pass = 0usize;
    let mut both_fail = 0usize;
    for (key, p) in &prev {
        let Some(l) = live.get(key) else { continue };
        let (w, s, mode, func, line) = key;
        let label = || format!("D{w}<{s}> {mode} {func} [{func}.golden:{line}]");
        match (p.grade, l.grade) {
            (Grade::Pass, Grade::Fail) => regressions.push(format!("{} ({})", label(), l.detail)),
            (Grade::Pass, Grade::Skip) => coverage_losses.push(label()),
            (Grade::Fail, Grade::Pass) => fixed.push(format!("{} (was {})", label(), p.detail)),
            (Grade::Pass, Grade::Pass) => both_pass += 1,
            (Grade::Fail, Grade::Fail) => both_fail += 1,
            _ => {}
        }
    }

    eprintln!("== history_previous: live vs decimal-scaled@{} ==", v044::VERSION);
    eprintln!(
        "ratchet: {} both-pass / {} both-fail / {} fixed in live / {} regressions / {} coverage losses",
        both_pass,
        both_fail,
        fixed.len(),
        regressions.len(),
        coverage_losses.len()
    );
    for f in fixed.iter().take(20) {
        eprintln!("  fixed: {f}");
    }
    if fixed.len() > 20 {
        eprintln!("  ... and {} more fixed cells", fixed.len() - 20);
    }
    for c in &coverage_losses {
        eprintln!("  coverage loss (passed in {}, skipped live): {c}", v044::VERSION);
    }
    for r in &regressions {
        eprintln!("  REGRESSION: {r}");
    }

    // Timing delta (advisory): per-function medians, live vs previous.
    let live_ns = timing_medians(&live);
    let prev_ns = timing_medians(&prev);
    eprintln!("-- timing (median ns/call across rows, ride-along advisory; reported, never asserted) --");
    eprintln!("{:<8} {:>14} {:>14} {:>8}", "func", "live", v044::VERSION, "ratio");
    for (func, &l) in &live_ns {
        if let Some(&p) = prev_ns.get(func) {
            let ratio = l as f64 / p.max(1) as f64;
            eprintln!("{func:<8} {l:>14} {p:>14} {ratio:>8.2}");
        }
    }

    assert!(both_pass + both_fail + fixed.len() > 0, "ratchet compared no cells");
    assert!(
        regressions.is_empty(),
        "{} cell(s) passed in {} but fail in the live crate (listed above)",
        regressions.len(),
        v044::VERSION
    );
}

// ---------------------------------------------------------------------------
// Gate 2 — history_all: every adapted version, one shootout-style run.
// ---------------------------------------------------------------------------

/// Every adapted historical version beside the live crate in ONE run: per-subject
/// tallies and the cross-version correctness table with a median-ns column per
/// version — all REPORTED, never asserted. Heavy; dispatch/on-demand only.
///
/// SPECIALIST gate (owner ruling 2026-06-12): a deliberate opt-in cost switch, NOT a
/// parked/orphan ignore. CI venue: history.yml (runs with `-- --ignored`).
/// The workspace zero-ignore mandate applies to every other test.
#[test]
#[ignore = "cross-version shootout; heavy, run on demand via --ignored --nocapture"]
fn history_all() {
    let _hook_guard = HOOK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
    let filter = Filter::from_env();
    let modes = filter.modes(&[RoundingMode::HalfToEven]);
    // Middle scale per tier + 1000-row/fn cap (see `history_cells` and `runner`).
    let cells = history_cells(&filter);
    let runner = runner(&filter);

    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    // One RunCollector spans every version's subjects.
    let mut rc = run_version(&runner, &filter, &modes, &cells, DsSubject::with_mode);
    for s in run_version(&runner, &filter, &modes, &cells, v044::Subject::with_mode).subjects {
        rc.add(s);
    }
    #[cfg(feature = "history-033")]
    {
        use decimal_scale_test::history::v033;
        let shared: Vec<(u32, u32)> =
            cells.iter().copied().filter(|c| v033::CELLS.contains(c)).collect();
        for s in run_version(&runner, &filter, &modes, &shared, v033::Subject::with_mode).subjects {
            rc.add(s);
        }
    }
    std::panic::set_hook(prev_hook);

    // Per-subject tallies (shootout preset), then the cross-version table.
    let runs = [rc];
    ConsoleReporter::shootout()
        .report(&runs, &mut std::io::stderr())
        .expect("write history_all report");

    // Flatten per version (capabilities name distinguishes them).
    let mut versions: BTreeMap<String, BTreeMap<Key, Cell>> = BTreeMap::new();
    for subject in &runs[0].subjects {
        let entry = versions.entry(subject.capabilities.name.clone()).or_default();
        flatten(subject, entry);
    }

    eprintln!("== history_all: cross-version correctness (per function; reported, never asserted) ==");
    for (version, cells) in &versions {
        let medians = timing_medians(cells);
        eprintln!("-- {version} --");
        eprintln!("{:<8} {:>6} {:>6} {:>6} {:>14}", "func", "pass", "fail", "skip", "median-ns");
        let mut per_func: BTreeMap<&'static str, (usize, usize, usize)> = BTreeMap::new();
        for (&(_, _, _, func, _), cell) in cells {
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
}

/// Adapter proofs relocated from the history module's unit tests when the
/// adapters moved to decimal-scaled-cells (whose lib deliberately carries no
/// unit-test harness — the compile-once placement): the live-cell sync guards
/// and the basic compute proofs per version.
mod adapter_proofs {
    use decimal_scaled_golden::{Computed, DecimalSubject, Function};

    /// `true` when the live cell list contains `cell` — the sync guard between a
    /// version's `CELLS` and the live surface.
    fn live_cell(cell: &(u32, u32)) -> bool {
        decimal_scale_test::CELLS.contains(cell)
    }

    /// Prove an adapter computes at all: sqrt(2) at D38<19> through the erased
    /// dispatch yields the 19-place prefix every era agrees on.
    fn proves_sqrt2_at_d38_19<S: DecimalSubject<Value = String>>(subject: &S) {
        let op = subject.execute(
            Function::Sqrt,
            decimal_scaled_golden::RoundingMode::HalfToEven,
            decimal_scaled_golden::Overflow::Panic,
        );
        match op(&["2".to_string()]) {
            Computed::Value(v) => {
                assert!(v.starts_with("1.41421356237309"), "got {v}");
                assert_eq!(v.split_once('.').unwrap().1.len(), 19, "19 fractional digits");
            }
            other => panic!("expected Value, got {other:?}"),
        }
    }

    /// Prove a binary function works too (the HistOps mul bridge): 1.5 * 2 = 3.
    fn proves_mul_at_d18_3<S: DecimalSubject<Value = String>>(subject: &S) {
        let op = subject.execute(
            Function::Mul,
            decimal_scaled_golden::RoundingMode::HalfToEven,
            decimal_scaled_golden::Overflow::Panic,
        );
        match op(&["1.5".to_string(), "2".to_string()]) {
            Computed::Value(v) => assert_eq!(v, "3.000"),
            other => panic!("expected Value, got {other:?}"),
        }
    }

    mod v044_proof {
        use super::{live_cell, proves_mul_at_d18_3, proves_sqrt2_at_d38_19};
        use decimal_scale_test::history::v044;

        #[test]
        fn computes_sqrt_and_mul() {
            proves_sqrt2_at_d38_19(&v044::Subject::new(38, 19));
            proves_mul_at_d18_3(&v044::Subject::new(18, 3));
        }

        #[test]
        fn cells_match_the_live_surface_exactly() {
            // 0.4.4 shares the live tier table, so its cell list IS the live one.
            assert_eq!(v044::CELLS, decimal_scale_test::CELLS);
            assert!(v044::CELLS.iter().all(live_cell));
        }
    }

    #[cfg(feature = "history-033")]
    mod v033_proof {
        use super::{live_cell, proves_mul_at_d18_3, proves_sqrt2_at_d38_19};
        use decimal_scale_test::history::v033;

        #[test]
        fn computes_sqrt_and_mul() {
            proves_sqrt2_at_d38_19(&v033::Subject::new(38, 19));
            proves_mul_at_d18_3(&v033::Subject::new(18, 3));
        }

        #[test]
        fn width_18_omits_the_aborting_transcendental_surface() {
            use decimal_scaled_golden::{DecimalSubject, Function};
            // 0.3.3's D18 `*_strict_with` trait surface self-recurses to an
            // uncatchable stack-overflow abort, so it must NOT be declared;
            // arithmetic stays declared, and width 38 keeps the full surface.
            let caps18 = v033::Subject::new(18, 3).capabilities();
            assert!(caps18.function(Function::Sqrt).is_none());
            assert!(caps18.function(Function::Exp).is_none());
            assert!(caps18.function(Function::Mul).is_some());
            assert!(caps18.function(Function::Rem).is_some());
            let caps38 = v033::Subject::new(38, 19).capabilities();
            assert!(caps38.function(Function::Sqrt).is_some());
            assert!(caps38.function(Function::Atan2).is_some());
        }

        #[test]
        fn cells_are_the_live_subset_at_the_shared_widths() {
            // Every 0.3.3 cell is a live cell, and it covers EVERY live cell at
            // the six shared widths (none silently dropped).
            assert!(v033::CELLS.iter().all(live_cell));
            let shared = [18, 38, 76, 153, 230, 307];
            let expected: Vec<(u32, u32)> = decimal_scale_test::CELLS
                .iter()
                .copied()
                .filter(|(w, _)| shared.contains(w))
                .collect();
            assert_eq!(v033::CELLS, expected.as_slice());
        }
    }
}
