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
use golden_competitors::{
    BigDecimalSubject, DashuFloat, DecimalRsSubject, FastNum, GMath, F64, RustDecimal,
};
use golden_ds::{golden_dir, thread_count, FUNCS, GEN_PRECISION};

use Function::*;

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

    // bigdecimal: arbitrary precision; sqrt/cbrt + the five arithmetic ops (exp
    // excluded — its unbounded growth would never terminate on large golden inputs).
    let (bd_pass, ..) = run_competitor(
        &BigDecimalSubject,
        &[Sqrt, Cbrt, Add, Sub, Mul, Div, Rem],
    );
    // dashu-float: arbitrary precision; ln + arithmetic (exp/powf excluded — they
    // would grow without bound and never terminate on large golden inputs).
    let (dashu_pass, ..) = run_competitor(
        &DashuFloat,
        &[Ln, Add, Sub, Mul, Div, Rem],
    );
    // fastnum (D512): the most complete real-function competitor.
    let (fast_pass, ..) = run_competitor(
        &FastNum,
        &[
            Sqrt, Cbrt, Ln, Log2, Log10, Exp, Exp2, Sin, Cos, Tan, Powf, Add, Sub, Mul, Div, Rem,
        ],
    );
    // decimal-rs: fixed 38-digit; sqrt/ln/exp/powf + checked arithmetic.
    let (drs_pass, ..) = run_competitor(
        &DecimalRsSubject,
        &[Sqrt, Ln, Exp, Powf, Add, Sub, Mul, Div, Rem],
    );
    // g_math: deterministic fixed-point; richest function set (trig + hyperbolics).
    let (gm_pass, ..) = run_competitor(
        &GMath,
        &[
            Sqrt, Ln, Exp, Sin, Cos, Tan, Atan, Asin, Acos, Sinh, Cosh, Tanh, Asinh, Acosh, Atanh,
            Powf, Atan2, Add, Sub, Mul, Div,
        ],
    );

    assert!(rd_pass > 0, "rust_decimal should correctly compute some golden values");
    assert!(f64_pass > 0, "f64 should correctly compute some golden values");
    assert!(bd_pass > 0, "bigdecimal should correctly compute some golden values");
    assert!(dashu_pass > 0, "dashu-float should correctly compute some golden values");
    assert!(fast_pass > 0, "fastnum should correctly compute some golden values");
    assert!(drs_pass > 0, "decimal-rs should correctly compute some golden values");
    assert!(gm_pass > 0, "g_math should correctly compute some golden values");
}
