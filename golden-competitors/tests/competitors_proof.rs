//! Proof that a competitor adapter runs end-to-end against the golden set through
//! the harness, graded down to the competitor's own precision. Unlike `golden_multi`
//! this is NOT a 0-bad gate — a competitor is *expected* to mis-round some values
//! (that is the comparison); we only assert it computes a meaningful share correctly,
//! and print the pass/skip/bad split so the numbers are visible.

use decimal_scaled_golden::{
    ExecutionResult, FileLoader, Function, GoldenRunner, Outcome, OverflowValidator, ParallelRunner,
    RoundingValidator, RunOnce,
};
use golden_competitors::RustDecimal;
use golden_ds::{golden_dir, thread_count, GEN_PRECISION};

#[test]
fn rust_decimal_validates_against_the_golden_set() {
    let runner = ParallelRunner {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(FileLoader::new(golden_dir())),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
            Box::new(OverflowValidator),
        ],
    };
    // The functions rust_decimal actually provides (the rest are unsupported and skip).
    let funcs = [Function::Sqrt, Function::Exp, Function::Ln, Function::Sin];
    let sc = runner.run(&RustDecimal, &funcs);

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
    eprintln!(
        "rust_decimal vs golden (graded to its 28 digits): {pass} pass / {skip} skip / {bad} bad"
    );
    assert!(pass > 0, "rust_decimal should correctly compute at least some golden values");
}
