//! End-to-end proof: drive decimal-scaled through the decimal-scaled-golden
//! harness at D38<19> against the golden proof set. One erased subject
//! (`DsSubject::new(38, 19)`, the same adapter the full surface enumerates); the
//! full 88-cell run is the `golden` gate.

// The erased subject's cell dispatch names every tier up to D1232, so the proof
// (like the gate) needs the full-width build; narrower feature sets skip it.
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use decimal_scaled_golden::{
    ExecutionResult, FileLoader, Function, GoldenRunner, Outcome, OverflowValidator, ParallelRunner,
    RoundingValidator, RunOnce,
};
use decimal_scale_test::{golden_dir, thread_count, DsSubject, GEN_PRECISION};

#[test]
fn golden_proof_d38_s19() {
    let runner = ParallelRunner {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(FileLoader::new(golden_dir())),
        // OverflowValidator too: the proof set is all in-range, so it must stay
        // silent — which it does only if `limits` correctly reports these values as
        // in range (a broken envelope would flag every cell).
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GEN_PRECISION }),
            Box::new(OverflowValidator),
        ],
    };
    let funcs = [Function::Sqrt, Function::Ln, Function::Sin];
    let subject = runner.run(&DsSubject::new(38, 19), &funcs);

    let mut pass = 0usize;
    let mut bad = 0usize;
    for fc in &subject.functions {
        let mut fpass = 0usize;
        for cell in &fc.cells {
            if matches!(cell.result(), Some(ExecutionResult::Skipped)) {
                continue;
            }
            for outcome in &cell.validations {
                match outcome {
                    Outcome::Pass => {
                        pass += 1;
                        fpass += 1;
                    }
                    Outcome::Precision { .. } => {}
                    other => {
                        bad += 1;
                        eprintln!("{} [{}.golden:{}]: {:?} on {:?}", fc.function.name(), fc.function.name(), cell.line, other, cell.inputs);
                    }
                }
            }
        }
        eprintln!("{}: {fpass} pass / {} cells", fc.function.name(), fc.cells.len());
    }
    assert_eq!(bad, 0, "harness found mis-rounded / wrong-mode / panic cells");
    assert!(pass > 0, "harness produced no Pass cells across any function");
}
