//! The full-surface golden gate over decimal-scaled. One erased
//! [`DsSubject`](decimal_scale_test::DsSubject) (`Value = String`) is run over every
//! band-edge `(width, scale)` cell, so the harness pipeline monomorphises once and the
//! build stays light. Filter any axis from the command line via the `GOLDEN_*` env vars
//! (see [`Filter`](decimal_scale_test::Filter)):
//!
//! ```text
//! # default gate (half-to-even, every cell):
//! cargo test -p decimal-scale-test --release --test golden
//! # all six rounding modes, full surface:
//! cargo test -p decimal-scale-test --release --test golden -- --ignored --nocapture
//! # just the cells under investigation, one mode, 1-in-50 rows:
//! GOLDEN_WIDTHS=924,1232 GOLDEN_MODES=ceiling GOLDEN_FUNCS=exp,cosh GOLDEN_SAMPLE=50 \
//!   cargo test -p decimal-scale-test --release --test golden golden_default -- --nocapture
//! ```
//! Honour `GOLDEN_THREADS` to cap parallelism (default = available cores).

mod common;

use std::sync::Mutex;

use decimal_scaled_golden::{
    ConsoleReporter, FilterLoader, GoldenRunner, InlineReporter, OverflowValidator, ParallelRunner,
    Reporter, RoundingMode, RoundingValidator, RunCollector, RunOnce, RunSummary, TsvReporter,
};
use decimal_scale_test::{thread_count, DsSubject, Filter, ALL_MODES, GEN_PRECISION};

use common::{row_filter, CachingLoader};

/// Serialises the gates: each swaps the process-global panic hook for its run, so two
/// gates running on parallel test threads would race the take/set/restore sequence.
static HOOK_GUARD: Mutex<()> = Mutex::new(());

/// Run the (env-filtered) surface under the given default `modes` and return the tally.
/// One `RunCollector` accumulates every selected `(mode, width, scale)` subject.
fn run(default_modes: &[RoundingMode]) -> RunSummary {
    // One gate at a time — the panic-hook swap below is process-global.
    let _hook_guard = HOOK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
    let filter = Filter::from_env();
    let modes = filter.modes(default_modes);
    let cells = filter.cells();
    let funcs = filter.funcs();

    let runner = ParallelRunner {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(FilterLoader::new(
            CachingLoader::golden(),
            row_filter(filter.sample(), filter.stripe()),
        )),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
            Box::new(OverflowValidator),
        ],
    };

    // The expected out-of-range cells panic (caught + validated as overflow); silence
    // the default hook so a sweep isn't drowned in backtraces (which also dominated the
    // wall time). Restored before the report below.
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let mut rc = RunCollector::new();
    // Per-subject progress to stderr (visible live under --nocapture): which
    // (mode, width, scale) the sweep is on, where it is in the run, and how
    // long each subject took — so a slow or stuck cell is identifiable from
    // the log WHILE the job runs, not after.
    let started = std::time::Instant::now();
    let total = modes.len() * cells.len();
    let mut done = 0usize;
    for &mode in &modes {
        for &(w, s) in &cells {
            done += 1;
            eprintln!(
                "[{:>8.1}s] {done}/{total} {mode:?} D{w}<{s}> ({} fns)...",
                started.elapsed().as_secs_f64(),
                funcs.len(),
            );
            let cell_started = std::time::Instant::now();
            rc.add(runner.run(&DsSubject::with_mode(w, s, mode), funcs));
            eprintln!(
                "[{:>8.1}s] {done}/{total} {mode:?} D{w}<{s}> done in {:.1}s",
                started.elapsed().as_secs_f64(),
                cell_started.elapsed().as_secs_f64(),
            );
        }
    }
    std::panic::set_hook(prev_hook);

    let runs = [rc];
    let summary = ConsoleReporter::gate()
        .report(&runs, &mut std::io::stderr())
        .expect("write golden report");

    // CI stripe support: when `GOLDEN_REPORT_DIR` is set, also drop the full per-cell
    // TSV plus a one-line summary into that directory. Each striped job uploads its
    // directory as an artifact the moment it finishes (a mini report readable while
    // other stripes still run); the aggregate job downloads every stripe and splices
    // the TSVs into the combined surface report.
    if let Some(dir) = std::env::var_os("GOLDEN_REPORT_DIR") {
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).expect("create GOLDEN_REPORT_DIR");
        for output in TsvReporter.report(&runs).outputs {
            std::fs::write(dir.join(&output.name), output.content).expect("write golden tsv");
        }
        let stripe = std::env::var("GOLDEN_STRIPE").unwrap_or_else(|_| "-".into());
        std::fs::write(
            dir.join("summary.txt"),
            format!("stripe {stripe}: {summary}
"),
        )
        .expect("write golden summary");
    }
    summary
}

fn check(s: RunSummary) {
    eprintln!("golden: {s}");
    assert_eq!(s.bad, 0, "mis-rounded / wrong-mode / error cells found");
    assert_eq!(s.panic, 0, "decimal-scaled panicked on a representable cell");
    assert!(s.pass > 0, "no Pass across any cell");
}

/// Default-mode gate: half-to-even across the full surface. `#[ignore]`d so a plain
/// `cargo test` never trips the heavy full-surface run — the CI golden job opts in with
/// `--ignored` (and the `GOLDEN_*` env vars can narrow it). Run:
/// `cargo test -p decimal-scale-test --release --test golden golden_default -- --ignored --nocapture`
///
/// SPECIALIST gate (owner ruling 2026-06-12): a deliberate opt-in cost switch, NOT a
/// parked/orphan ignore. CI venue: ci.yml golden quick shards (runs with `-- --ignored`).
/// The workspace zero-ignore mandate applies to every other test.
#[test]
#[ignore = "full-surface golden; run via --ignored"]
fn golden_default() {
    check(run(&[RoundingMode::HalfToEven]));
}

/// All six rounding modes across the full surface — heavy, so `#[ignore]`d. Directed
/// rounding (Ceiling/Floor/Trunc) shows regressions the default mode hides.
///
/// SPECIALIST gate (owner ruling 2026-06-12): a deliberate opt-in cost switch, NOT a
/// parked/orphan ignore. CI venue: golden-comprehensive.yml (runs with `-- --ignored`).
/// The workspace zero-ignore mandate applies to every other test.
#[test]
#[ignore = "full six-mode surface; run via --ignored --nocapture"]
fn golden_all_modes() {
    check(run(&ALL_MODES));
}
