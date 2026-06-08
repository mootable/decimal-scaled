//! End-to-end proof: drive decimal-scaled through the decimal-scaled-golden
//! harness at D38<19> against the proof corpus. One subject; the full 88-cell
//! enumeration is `golden_multi`.
use std::collections::BTreeMap;

use decimal_scaled::{D38, RoundingMode as DsMode};
use decimal_scaled_golden::{
    Capabilities, FileCaseLoader, FnSupport, Function, GoldenValue, Outcome, Overflow, RoundingMode,
    RoundingValidator, RunOnce, SeriesTester, Subject, Tester,
};

const GP: usize = 1233;

fn ds_mode(m: RoundingMode) -> DsMode {
    match m {
        RoundingMode::HalfToEven => DsMode::HalfToEven,
        RoundingMode::HalfAwayFromZero => DsMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero => DsMode::HalfTowardZero,
        RoundingMode::Ceiling => DsMode::Ceiling,
        RoundingMode::Floor => DsMode::Floor,
        RoundingMode::Trunc => DsMode::Trunc,
    }
}

fn compute(func: Function, x: D38<19>, m: DsMode) -> D38<19> {
    match func {
        Function::Sqrt => x.sqrt_strict_with(m),
        Function::Exp => x.exp_strict_with(m),
        Function::Ln => x.ln_strict_with(m),
        Function::Sin => x.sin_strict_with(m),
        other => unreachable!("subject only supports sqrt/exp/ln/sin, got {other:?}"),
    }
}

struct DecimalScaledD38S19;

impl Subject for DecimalScaledD38S19 {
    type Value = D38<19>;

    fn capabilities(&self) -> Capabilities {
        // decimal-scaled panics on overflow in BOTH debug and release (the
        // strict contract; the `wrapping_`/`checked_` variants aren't tested here).
        let overflow = Overflow::Panic;
        let mut functions = BTreeMap::new();
        for f in [Function::Sqrt, Function::Exp, Function::Ln, Function::Sin] {
            functions.insert(f, FnSupport { mode: RoundingMode::HalfToEven, overflow });
        }
        Capabilities {
            name: "decimal-scaled".to_string(),
            width: 38,
            scale: 19,
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> D38<19> {
        // The tester pre-filters unrepresentable inputs, so a parse failure here
        // is a genuine fault — let it panic (the harness records it).
        s.parse::<D38<19>>().expect("representable input")
    }

    fn value_to_string(&self, v: &D38<19>) -> String {
        v.to_string()
    }

    fn representable(&self, value: &GoldenValue) -> bool {
        value.to_decimal_string().parse::<D38<19>>().is_ok()
    }

    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[D38<19>]) -> D38<19> {
        let m = ds_mode(mode);
        move |inputs| compute(func, inputs[0], m)
    }
}

fn proof_dir() -> String {
    format!("{}/decimal-scaled-golden/golden", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn golden_proof_d38_s19() {
    let tester = SeriesTester {
        strategy: RunOnce,
        loader: Box::new(FileCaseLoader::new(proof_dir())),
        validators: vec![Box::new(RoundingValidator { gen_precision: GP })],
    };
    let funcs = [Function::Sqrt, Function::Ln, Function::Sin];
    let subject = tester.run(&DecimalScaledD38S19, &funcs);

    let mut pass = 0usize;
    let mut bad = 0usize;
    for fc in &subject.functions {
        let mut fpass = 0usize;
        for cell in &fc.cells {
            for outcome in &cell.validations {
                match outcome {
                    Outcome::Pass => {
                        pass += 1;
                        fpass += 1;
                    }
                    Outcome::Skipped => {}
                    other => {
                        bad += 1;
                        eprintln!("{}: {:?} on {:?}", fc.function.name(), other, cell.inputs);
                    }
                }
            }
        }
        eprintln!("{}: {fpass} pass / {} cells", fc.function.name(), fc.cells.len());
    }
    assert_eq!(bad, 0, "harness found mis-rounded / wrong-mode / panic cells");
    assert!(pass > 0, "harness produced no Pass cells across any function");
}
