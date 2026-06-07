//! End-to-end proof: drive decimal-scaled through the decimal-scaled-golden
//! harness against the Phase-3 proof corpus at D38<19>. Single cell; the full
//! multi-cell enumeration comes later.
use decimal_scaled::{D38, DecimalTranscendental, RoundingMode as DsMode};
use decimal_scaled_golden::parser;
use decimal_scaled_golden::{
    run, Capabilities, Computed, CorrectnessTester, DecimalSubject, Function, Outcome, RoundingMode,
};

const GP: usize = 1233;

struct DecimalScaledAdapter;

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

impl DecimalSubject for DecimalScaledAdapter {
    type Value = D38<19>;

    fn capabilities(&self, func: Function) -> Capabilities {
        let supported = matches!(
            func,
            Function::Sqrt | Function::Exp | Function::Ln | Function::Sin
        );
        Capabilities {
            name: "decimal-scaled".to_string(),
            supported,
            max_width: 38,
            max_scale: 19,
            rounding_modes: RoundingMode::ALL.to_vec(),
        }
    }

    fn to_text(&self, v: &D38<19>) -> String {
        v.to_string()
    }

    fn from_text(&self, s: &str, width: u32, scale: u32) -> Computed<D38<19>> {
        if (width, scale) != (38, 19) {
            return Computed::Skip;
        }
        // An input with more fractional digits than the tier scale is not exactly
        // representable here -- skip it rather than silently round the input.
        if frac_len(s) > scale as usize {
            return Computed::Skip;
        }
        match s.parse::<D38<19>>() {
            Ok(v) => Computed::Value(v),
            Err(_) => Computed::Skip,
        }
    }

    fn execute(
        &self,
        func: Function,
        inputs: &[D38<19>],
        _width: u32,
        _scale: u32,
        mode: RoundingMode,
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
    format!(
        "{}/decimal-scaled-golden/golden/{}.txt",
        env!("CARGO_MANIFEST_DIR"),
        name
    )
}

#[test]
fn golden_proof_d38_s19() {
    let funcs = [
        ("sqrt", Function::Sqrt),
        ("ln", Function::Ln),
        ("sin", Function::Sin),
    ];
    let mut total_pass = 0usize;
    for (name, f) in funcs {
        let body = std::fs::read_to_string(corpus_path(name)).expect("read proof corpus");
        let cases = parser::parse(f, &body);
        assert!(!cases.is_empty(), "{name}: no cases parsed");
        let recs = run(
            &CorrectnessTester,
            &DecimalScaledAdapter,
            f,
            &cases,
            &[(38, 19)],
            &[RoundingMode::HalfToEven],
            GP,
        );
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
