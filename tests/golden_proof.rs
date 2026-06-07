//! End-to-end proof: drive decimal-scaled through the decimal-scaled-golden
//! harness at D38<19> against the proof corpus. One single-cell subject; the
//! full 88-cell enumeration is `golden_multi`.
use std::collections::BTreeMap;

use decimal_scaled::{D38, RoundingMode as DsMode};
use decimal_scaled_golden::{
    parser, run, Capabilities, Computed, CorrectnessTester, DecimalSubject, ErasedSubject,
    FnSupport, Function, Outcome, Overflow, RoundingMode,
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

impl DecimalSubject for DecimalScaledD38S19 {
    type Value = D38<19>;

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

    fn from_text(&self, s: &str) -> Computed<D38<19>> {
        // An input with more fractional digits than scale 19 is not exactly
        // representable here -- skip it rather than silently round.
        if frac_len(s) > 19 {
            return Computed::Skip;
        }
        match s.parse::<D38<19>>() {
            Ok(v) => Computed::Value(v),
            Err(_) => Computed::Skip,
        }
    }

    fn to_text(&self, v: &D38<19>) -> String {
        v.to_string()
    }

    fn execute(
        &self,
        func: Function,
        inputs: &[D38<19>],
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> Computed<D38<19>> {
        let x = inputs[0];
        let m = ds_mode(mode);
        let y = match func {
            Function::Sqrt => x.sqrt_strict_with(m),
            Function::Exp => x.exp_strict_with(m),
            Function::Ln => x.ln_strict_with(m),
            Function::Sin => x.sin_strict_with(m),
            _ => return Computed::Skip,
        };
        Computed::Value(y)
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
    let subjects: Vec<Box<dyn ErasedSubject>> = vec![Box::new(DecimalScaledD38S19)];
    let mut total_pass = 0usize;
    for (name, f) in funcs {
        let body = std::fs::read_to_string(corpus_path(name)).expect("read proof corpus");
        let cases = parser::parse(f, &body);
        assert!(!cases.is_empty(), "{name}: no cases parsed");
        let recs = run(&CorrectnessTester, &subjects, f, &cases, GP);
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
