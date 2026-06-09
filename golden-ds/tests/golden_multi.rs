//! Multi-tier end-to-end: drive decimal-scaled through the decimal-scaled-golden
//! harness across all 88 band-edge `(width, scale)` cells. Each cell is ONE
//! single-cell subject (`DsSubject<D>` over the concrete decimal type); the harness
//! is generic over `DecimalSubject`, so each subject is run via `GoldenRunner::run`
//! (no erasure) and its `SubjectCollector` collected into a `RunCollector`.
//! Full-surface + slow, so `#[ignore]`d — run explicitly:
//!
//! ```text
//! cargo test -p golden-ds --release --test golden_multi -- --ignored --nocapture
//! ```
//! Honour `GOLDEN_THREADS` to cap parallelism (default = available cores).

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use decimal_scaled::{
    D115, D1232, D153, D18, D230, D307, D38, D462, D57, D616, D76, D924,
};
use decimal_scaled_golden::{
    CaseLoader, ExecutionResult, FileLoader, Function, GoldenCase, GoldenRunner, Limits, Outcome,
    OverflowValidator, ParallelRunner, RoundingValidator, RunCollector, RunOnce,
};
use golden_ds::{golden_dir, thread_count, DsSubject, FUNCS, GEN_PRECISION, GUARD};

/// Loads each function's golden file ONCE (via a `FileLoader`) and clones it per
/// request — the per-subject `run` would otherwise re-read+re-parse the (multi-MB)
/// files 88 times.
struct CachingLoader {
    inner: FileLoader,
    cache: Mutex<HashMap<Function, Arc<Vec<GoldenCase>>>>,
}

impl CachingLoader {
    fn new(dir: &str) -> CachingLoader {
        CachingLoader { inner: FileLoader::new(dir), cache: Mutex::new(HashMap::new()) }
    }
}

impl CaseLoader for CachingLoader {
    fn load(&self, function: Function) -> Cow<'_, [GoldenCase]> {
        let arc = self
            .cache
            .lock()
            .unwrap()
            .entry(function)
            .or_insert_with(|| Arc::new(self.inner.load(function).into_owned()))
            .clone();
        Cow::Owned((*arc).clone())
    }
    fn oracle_limits(&self) -> Limits {
        // Verifiable depth = generation precision minus the rounding-guard digits.
        Limits { min_value: None, max_value: None, max_precision: (GEN_PRECISION - GUARD) as u32, max_significant_digits: None }
    }
}

/// Run one `runner.run` per band-edge `(width, scale)` cell, collecting each
/// subject's `SubjectCollector` into one `RunCollector`.
macro_rules! run_subjects {
    ($runner:expr, $funcs:expr, $($D:ty => ($w:expr, $s:expr)),+ $(,)?) => {{
        let mut rc = RunCollector::new();
        $(
            rc.add($runner.run(&DsSubject::<$D>::new($w, $s), $funcs));
        )+
        rc
    }};
}

#[test]
#[ignore = "full-surface golden; run via: cargo test -p golden-ds --release --test golden_multi -- --ignored --nocapture"]
fn golden_multi_tier() {
    let funcs: Vec<Function> = FUNCS.to_vec();
    let runner = ParallelRunner {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(CachingLoader::new(golden_dir())),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
            Box::new(OverflowValidator),
        ],
    };

    let rc = run_subjects! { runner, &funcs,
        // D18 — Int<1>, 64-bit storage
        D18<0> => (18, 0), D18<3> => (18, 3), D18<4> => (18, 4),
        D18<9> => (18, 9), D18<13> => (18, 13), D18<17> => (18, 17),
        // D38 — Int<2>, 128-bit
        D38<0> => (38, 0), D38<2> => (38, 2), D38<6> => (38, 6),
        D38<9> => (38, 9), D38<10> => (38, 10), D38<12> => (38, 12),
        D38<17> => (38, 17), D38<18> => (38, 18), D38<19> => (38, 19),
        D38<28> => (38, 28), D38<37> => (38, 37),
        // D57 — Int<3>, 192-bit
        D57<0> => (57, 0), D57<14> => (57, 14), D57<20> => (57, 20),
        D57<28> => (57, 28), D57<30> => (57, 30), D57<42> => (57, 42),
        D57<56> => (57, 56),
        // D76 — Int<4>, 256-bit
        D76<0> => (76, 0), D76<18> => (76, 18), D76<19> => (76, 19),
        D76<38> => (76, 38), D76<40> => (76, 40), D76<57> => (76, 57),
        D76<75> => (76, 75),
        // D115 — Int<6>, 384-bit
        D115<0> => (115, 0), D115<28> => (115, 28), D115<50> => (115, 50),
        D115<57> => (115, 57), D115<86> => (115, 86), D115<114> => (115, 114),
        // D153 — Int<8>, 512-bit
        D153<0> => (153, 0), D153<38> => (153, 38), D153<76> => (153, 76),
        D153<114> => (153, 114), D153<152> => (153, 152),
        // D230 — Int<12>, 768-bit
        D230<0> => (230, 0), D230<57> => (230, 57), D230<115> => (230, 115),
        D230<172> => (230, 172), D230<229> => (230, 229),
        // D307 — Int<16>, 1024-bit
        D307<0> => (307, 0), D307<30> => (307, 30), D307<50> => (307, 50),
        D307<70> => (307, 70), D307<76> => (307, 76), D307<120> => (307, 120),
        D307<153> => (307, 153), D307<230> => (307, 230), D307<306> => (307, 306),
        // D462 — Int<24>, 1536-bit
        D462<0> => (462, 0), D462<30> => (462, 30), D462<100> => (462, 100),
        D462<115> => (462, 115), D462<180> => (462, 180), D462<231> => (462, 231),
        D462<346> => (462, 346), D462<461> => (462, 461),
        // D616 — Int<32>, 2048-bit
        D616<0> => (616, 0), D616<30> => (616, 30), D616<130> => (616, 130),
        D616<154> => (616, 154), D616<240> => (616, 240), D616<308> => (616, 308),
        D616<462> => (616, 462), D616<615> => (616, 615),
        // D924 — Int<48>, 3072-bit
        D924<0> => (924, 0), D924<30> => (924, 30), D924<180> => (924, 180),
        D924<231> => (924, 231), D924<350> => (924, 350), D924<462> => (924, 462),
        D924<693> => (924, 693), D924<923> => (924, 923),
        // D1232 — Int<64>, 4096-bit
        D1232<0> => (1232, 0), D1232<30> => (1232, 30), D1232<250> => (1232, 250),
        D1232<308> => (1232, 308), D1232<470> => (1232, 470), D1232<616> => (1232, 616),
        D1232<924> => (1232, 924), D1232<1231> => (1232, 1231),
    };

    let mut pass = 0usize;
    let mut skip = 0usize;
    let mut panic = 0usize;
    let mut bad = 0usize;
    for subject in &rc.subjects {
        let cfg = &subject.capabilities.config;
        let w = cfg.get("width").cloned().unwrap_or_default();
        let s = cfg.get("scale").cloned().unwrap_or_default();
        for fc in &subject.functions {
            for cell in &fc.cells {
                if matches!(cell.result(), Some(ExecutionResult::Skipped)) {
                    skip += 1;
                    continue;
                }
                for outcome in &cell.validations {
                    match outcome {
                        Outcome::Pass => pass += 1,
                        Outcome::Skipped => skip += 1,
                        Outcome::Precision { .. } => {}
                        Outcome::Panic => {
                            panic += 1;
                            eprintln!("  PANIC {} @({w},{s}) [{}.golden:{}] input={:?}", fc.function.name(), fc.function.name(), cell.line, cell.inputs);
                        }
                        other => {
                            bad += 1;
                            eprintln!("  BAD {} @({w},{s}) [{}.golden:{}]: {:?} on {:?}", fc.function.name(), fc.function.name(), cell.line, other, cell.inputs);
                        }
                    }
                }
            }
        }
    }
    eprintln!("TOTAL: {pass} pass / {skip} skip / {panic} panic / {bad} bad");
    assert_eq!(bad, 0, "mis-rounded / wrong-mode / error cells found");
    assert_eq!(panic, 0, "decimal-scaled panicked on a representable cell");
    assert!(pass > 0, "no Pass across any cell");
}
