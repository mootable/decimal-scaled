//! Proof that each competitor adapter runs end-to-end against the golden set through
//! the harness, graded down to the competitor's own precision. Unlike `golden_multi`
//! this is NOT a 0-bad gate — a competitor is *expected* to mis-round many values
//! (that is the comparison); we only assert each computes a meaningful share
//! correctly, and print the pass/skip/bad split so the contrast with decimal-scaled
//! (0 bad across the whole surface) is visible.

use decimal_scaled_golden::{
    DecimalSubject, ExecutionResult, FileLoader, Function, GoldenRunner, Outcome,
    OverflowValidator, ParallelRunner, RoundingValidator, RunOnce,
};
use golden_competitors::{F64, RustDecimal};
use golden_ds::{golden_dir, thread_count, FUNCS, GEN_PRECISION};

/// Run one competitor over `funcs` against the golden set; print and return its
/// pass / skip / bad counts (bad = mis-rounded / wrong-mode / error vs the golden).
fn run_competitor<S: DecimalSubject + Sync>(subject: &S, funcs: &[Function]) -> (usize, usize, usize) {
    let runner = ParallelRunner {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(FileLoader::new(golden_dir())),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
            Box::new(OverflowValidator),
        ],
    };
    let sc = runner.run(subject, funcs);
    let (mut pass, mut bad, mut skip) = (0usize, 0usize, 0usize);
    for fc in &sc.functions {
        for cell in &fc.cells {
            if matches!(cell.result(), Some(ExecutionResult::Skipped)) {
                skip += 1;
                continue;
            }
            for o in &cell.validations {
                match o {
                    Outcome::Pass => pass += 1,
                    Outcome::Precision { .. } => {}
                    _ => bad += 1,
                }
            }
        }
    }
    eprintln!("{}: {pass} pass / {skip} skip / {bad} bad", subject.name());
    (pass, skip, bad)
}

#[test]
fn competitors_validate_against_the_golden_set() {
    eprintln!("-- competitor vs golden (graded to each lib's own precision; decimal-scaled = 0 bad) --");
    // rust_decimal: only the functions its `MathematicalOps` provides.
    let (rd_pass, ..) = run_competitor(
        &RustDecimal,
        &[Function::Sqrt, Function::Exp, Function::Ln, Function::Sin],
    );
    // f64: every function (binary radix — decimal compliance is the verdict).
    let (f64_pass, ..) = run_competitor(&F64, FUNCS);

    assert!(rd_pass > 0, "rust_decimal should correctly compute some golden values");
    assert!(f64_pass > 0, "f64 should correctly compute some golden values");
}
