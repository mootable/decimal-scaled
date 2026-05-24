//! FAST agent-validation subset of the correctly-rounded golden oracle.
//!
//! This is a CURATED, seconds-to-run slice of the full
//! `ulp_strict_golden` gate, meant for quick correctness validation
//! while iterating on a kernel. It is NOT the authoritative gate: the
//! full 264-table, every-width, every-function matrix lives in
//! `tests/ulp_strict_golden.rs` and is the coordinator/CI gate. Always
//! run that before claiming correctness.
//!
//! What this subset covers:
//!   * 6 representative functions  `sqrt`, `cbrt`, `exp`, `ln`, `sin`,
//!     `atan`;
//!   * 3 widths spanning the tiers  `D18` (narrow primitive), `D57`
//!     (mid wide), `D307` (deep wide);
//!   * only the FIRST `ROW_CAP` data rows of each oracle table;
//!   * ALL SIX `RoundingMode`s.
//!
//! It reuses the EXACT same precision harness, oracle fold, and
//! zero-tolerance assertion as the full golden: every scored cell must
//! be bit-exact correctly rounded (`lsbe == 0`). There is no parallel
//! comparison path.

#![cfg(not(feature = "fast"))]

#[path = "support/precision_harness.rs"]
mod harness;

#[path = "support/precision_subject_ds.rs"]
mod subject_ds;

use decimal_scaled::RoundingMode;
use harness::{
    GoldenCase, Harness, Input, Method, PrecisionResult, PrecisionSubject, Width,
    parse_golden_line,
};
use subject_ds::DecimalScaledSubject;

/// The six rounding modes. Every curated case is checked under all six.
const MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

/// Cap on the number of DATA rows scored per oracle table. Keeps the
/// whole subset to a few seconds while still spanning each table's
/// leading (typically near-1 / small-argument) cases.
const ROW_CAP: usize = 20;

/// Map a curated function name to the harness [`Method`].
fn method_of(func: &str) -> Method {
    match func {
        "ln" => Method::Ln,
        "exp" => Method::Exp,
        "sin" => Method::Sin,
        "atan" => Method::Atan,
        "sqrt" => Method::Sqrt,
        "cbrt" => Method::Cbrt,
        other => panic!("unknown function: {other}"),
    }
}

/// Drive the reference subject (`decimal-scaled`) through the harness for
/// the first `ROW_CAP` data rows of one `(func, width)` table, across all
/// six modes, asserting every scored cell is correctly rounded
/// (`lsbe == 0`)  identical to the full golden's verdict, just on a
/// capped row set.
fn check(func: &str, width: Width, table: &str) {
    let subject = DecimalScaledSubject;
    let method = method_of(func);
    let scale = width.canonical_scale();
    let mut failures = 0usize;
    let mut scored = 0usize;

    for line in table.lines() {
        if scored >= ROW_CAP {
            break;
        }
        let Some(GoldenCase {
            input,
            input2,
            floor,
            cls,
        }) = parse_golden_line(line)
        else {
            continue;
        };
        scored += 1;
        let case = GoldenCase {
            input: input.clone(),
            input2: input2.clone(),
            floor: floor.clone(),
            cls,
        };
        let inp = Input {
            raw: input.clone(),
            input2: input2.clone(),
            width,
            scale,
        };
        for &mode in MODES.iter() {
            let out = subject.eval(method, width, scale, &inp, mode);
            match Harness::score(&out, &case, scale) {
                PrecisionResult::NotApplicable => {
                    eprintln!(
                        "FAIL: {func} {} mode={mode:?} input={input} \
                         input2={input2:?}: subject returned NotApplicable",
                        width.name(),
                    );
                    failures += 1;
                }
                PrecisionResult::Executed {
                    lsbe, ulp, value, ..
                } => {
                    if lsbe != 0 {
                        eprintln!(
                            "FAIL: {func} {} mode={mode:?} input={input} \
                             input2={input2:?} floor={floor} cls={cls:?} \
                             value={value} lsbe={lsbe} ulp={ulp}",
                            width.name(),
                        );
                        failures += 1;
                    }
                }
            }
        }
    }
    assert!(
        failures == 0,
        "{}: {func}: {failures} (case, mode) pairs not correctly rounded (lsbe != 0)",
        width.name(),
    );
}

macro_rules! micro {
    ($modname:ident, $width:expr, { $( $fn:ident = $file:literal ; )+ }) => {
        mod $modname {
            use super::{Width, check};
            const WIDTH: Width = $width;
            $(
                #[test]
                fn $fn() {
                    check(stringify!($fn), WIDTH, include_str!($file));
                }
            )+
        }
    };
}

// Narrow primitive tier.
micro!(d18, Width::D18, {
    sqrt = "golden/sqrt_d18_s9.txt";
    cbrt = "golden/cbrt_d18_s9.txt";
    exp  = "golden/exp_d18_s9.txt";
    ln   = "golden/ln_d18_s9.txt";
    sin  = "golden/sin_d18_s9.txt";
    atan = "golden/atan_d18_s9.txt";
});

// Mid wide tier.
micro!(d57, Width::D57, {
    sqrt = "golden/sqrt_d57_s28.txt";
    cbrt = "golden/cbrt_d57_s28.txt";
    exp  = "golden/exp_d57_s28.txt";
    ln   = "golden/ln_d57_s28.txt";
    sin  = "golden/sin_d57_s28.txt";
    atan = "golden/atan_d57_s28.txt";
});

// Deep wide tier.
micro!(d307, Width::D307, {
    sqrt = "golden/sqrt_d307_s150.txt";
    cbrt = "golden/cbrt_d307_s150.txt";
    exp  = "golden/exp_d307_s150.txt";
    ln   = "golden/ln_d307_s150.txt";
    sin  = "golden/sin_d307_s150.txt";
    atan = "golden/atan_d307_s150.txt";
});
