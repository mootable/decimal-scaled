//! End-to-end proof: drive decimal-scaled through the decimal-scaled-golden
//! harness at D38<19> against the proof corpus. One single-cell subject; the
//! full 88-cell enumeration is `golden_multi`.
use std::collections::BTreeMap;

use decimal_scaled::{D38, RoundingMode as DsMode};
use decimal_scaled_golden::parser::GoldenCase;
use decimal_scaled_golden::{
    parser, run, Capabilities, Computed, FnSupport, Function, Outcome, Overflow, RoundingMode,
    RunOnce, Subject, Validator,
};

const GP: usize = 1233;

struct DecimalScaledD38S19;

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

fn frac_len(s: &str) -> usize {
    match s.split_once('.') {
        Some((_, f)) => f.len(),
        None => 0,
    }
}

impl DecimalScaledD38S19 {
    fn compute(&self, func: Function, x: D38<19>, m: DsMode) -> Option<D38<19>> {
        Some(match func {
            Function::Sqrt => x.sqrt_strict_with(m),
            Function::Exp => x.exp_strict_with(m),
            Function::Ln => x.ln_strict_with(m),
            Function::Sin => x.sin_strict_with(m),
            _ => return None,
        })
    }
}

impl Subject for DecimalScaledD38S19 {
    fn capabilities(&self) -> Capabilities {
        let overflow = if cfg!(debug_assertions) { Overflow::Panic } else { Overflow::Wrap };
        let mut functions = BTreeMap::new();
        for f in [Function::Sqrt, Function::Exp, Function::Ln, Function::Sin] {
            functions.insert(f, FnSupport { modes: vec![RoundingMode::HalfToEven], overflow });
        }
        Capabilities {
            name: "decimal-scaled".to_string(),
            width: 38,
            scale: 19,
            storage_bits: 128,
            functions,
        }
    }

    fn eval(
        &self,
        func: Function,
        inputs: &[&str],
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> Computed<String> {
        if frac_len(inputs[0]) > 19 {
            return Computed::Skip;
        }
        let x = match inputs[0].parse::<D38<19>>() {
            Ok(v) => v,
            Err(_) => return Computed::Skip,
        };
        match self.compute(func, x, ds_mode(mode)) {
            Some(y) => Computed::Value(y.to_string()),
            None => Computed::Skip,
        }
    }

    fn compute_thunk<'a>(
        &'a self,
        cases: &'a [GoldenCase],
        func: Function,
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> Option<Box<dyn FnMut() + 'a>> {
        let m = ds_mode(mode);
        let batch: Vec<D38<19>> = cases
            .iter()
            .filter_map(|c| {
                let s = c.inputs.first()?;
                if frac_len(s) > 19 {
                    return None;
                }
                s.parse::<D38<19>>().ok()
            })
            .collect();
        if batch.is_empty() {
            return None;
        }
        Some(Box::new(move || {
            for x in &batch {
                std::hint::black_box(self.compute(func, *x, m));
            }
        }))
    }
}

fn corpus_path(name: &str) -> String {
    format!("{}/decimal-scaled-golden/golden/{}.txt", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn golden_proof_d38_s19() {
    let funcs = [
        ("sqrt", Function::Sqrt),
        ("ln", Function::Ln),
        ("sin", Function::Sin),
    ];
    let tester = Validator::validation_tester(GP);
    let strategy = RunOnce;
    let subjects: Vec<Box<dyn Subject>> = vec![Box::new(DecimalScaledD38S19)];
    let mut total_pass = 0usize;
    for (name, f) in funcs {
        let body = std::fs::read_to_string(corpus_path(name)).expect("read proof corpus");
        let cases = parser::parse(f, &body);
        assert!(!cases.is_empty(), "{name}: no cases parsed");
        let recs = run(&tester, &strategy, &subjects, f, &cases);
        let mut pass = 0usize;
        let mut skip = 0usize;
        for r in &recs {
            match &r.outcome {
                Outcome::Pass => pass += 1,
                Outcome::Skipped => skip += 1,
                other => panic!("{name}: unexpected {:?} on input {:?}", other, r.detail),
            }
        }
        eprintln!("{name}: {pass} pass, {skip} skip ({} cases)", recs.len());
        total_pass += pass;
    }
    assert!(total_pass > 0, "harness produced no Pass cells across any function");
}
