//! Multi-tier end-to-end: drive decimal-scaled through the decimal-scaled-golden
//! harness across all 12 widths at their canonical scale, against the broadened
//! proof corpus. Requires the wide-tier features (compiles to nothing without
//! them, so plain `cargo test` skips it).
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use decimal_scaled::{
    D115, D1232, D153, D18, D230, D307, D38, D462, D57, D616, D76, D924,
    RoundingMode as DsMode,
};
use decimal_scaled_golden::parser;
use decimal_scaled_golden::{
    run, Capabilities, Computed, CorrectnessTester, DecimalSubject, Function, Outcome, RoundingMode,
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

fn frac_len(s: &str) -> usize {
    match s.split_once('.') {
        Some((_, f)) => f.len(),
        None => 0,
    }
}

macro_rules! ds_cells {
    ($($variant:ident => ($D:ty, $w:expr, $s:expr)),+ $(,)?) => {
        enum DsValue { $($variant($D),)+ }

        struct DecimalScaledAdapter;

        impl DecimalSubject for DecimalScaledAdapter {
            type Value = DsValue;

            fn capabilities(&self, func: Function) -> Capabilities {
                let supported = true;
                Capabilities {
                    name: "decimal-scaled".to_string(),
                    supported,
                    max_width: 1232,
                    max_scale: 1231,
                    rounding_modes: RoundingMode::ALL.to_vec(),
                }
            }

            fn to_text(&self, v: &DsValue) -> String {
                match v {
                    $(DsValue::$variant(x) => x.to_string(),)+
                }
            }

            fn from_text(&self, s: &str, width: u32, scale: u32) -> Computed<DsValue> {
                if frac_len(s) > scale as usize {
                    return Computed::Skip;
                }
                match (width, scale) {
                    $(($w, $s) => match s.parse::<$D>() {
                        Ok(v) => Computed::Value(DsValue::$variant(v)),
                        Err(_) => Computed::Skip,
                    },)+
                    _ => Computed::Skip,
                }
            }

            fn execute(
                &self,
                func: Function,
                inputs: &[DsValue],
                _width: u32,
                _scale: u32,
                mode: RoundingMode,
            ) -> Computed<DsValue> {
                let m = ds_mode(mode);
                match &inputs[0] {
                    $(
                        DsValue::$variant(x0) => {
                            let x = *x0;
                            // binary ops take a second operand from the SAME cell variant
                            let d2 = match inputs.get(1) {
                                Some(DsValue::$variant(d)) => Some(*d),
                                _ => None,
                            };
                            let y = match func {
                                Function::Sqrt => x.sqrt_strict_with(m),
                                Function::Cbrt => x.cbrt_strict_with(m),
                                Function::Exp => x.exp_strict_with(m),
                                Function::Ln => x.ln_strict_with(m),
                                Function::Log2 => x.log2_strict_with(m),
                                Function::Log10 => x.log10_strict_with(m),
                                Function::Exp2 => x.exp2_strict_with(m),
                                Function::Sin => x.sin_strict_with(m),
                                Function::Cos => x.cos_strict_with(m),
                                Function::Tan => x.tan_strict_with(m),
                                Function::Atan => x.atan_strict_with(m),
                                Function::Asin => x.asin_strict_with(m),
                                Function::Acos => x.acos_strict_with(m),
                                Function::Sinh => x.sinh_strict_with(m),
                                Function::Cosh => x.cosh_strict_with(m),
                                Function::Tanh => x.tanh_strict_with(m),
                                Function::Asinh => x.asinh_strict_with(m),
                                Function::Acosh => x.acosh_strict_with(m),
                                Function::Atanh => x.atanh_strict_with(m),
                                Function::Log => match d2 { Some(d) => x.log_strict_with(d, m), None => return Computed::Skip },
                                Function::Atan2 => match d2 { Some(d) => x.atan2_strict_with(d, m), None => return Computed::Skip },
                                Function::Powf => match d2 { Some(d) => x.powf_strict_with(d, m), None => return Computed::Skip },
                                Function::Hypot => match d2 { Some(d) => x.hypot_strict_with(d, m), None => return Computed::Skip },
                                Function::Add => match d2 { Some(d) => x + d, None => return Computed::Skip },
                                Function::Sub => match d2 { Some(d) => x - d, None => return Computed::Skip },
                                Function::Mul => match d2 { Some(d) => x.mul_with(d, m), None => return Computed::Skip },
                                Function::Div => match d2 { Some(d) => x.div_with(d, m), None => return Computed::Skip },
                                Function::Rem => match d2 { Some(d) => x % d, None => return Computed::Skip },
                            };
                            Computed::Value(DsValue::$variant(y))
                        }
                    )+
                }
            }
        }

        const CELLS: &[(u32, u32)] = &[$(($w, $s)),+];
    };
}

ds_cells! {
    // D18 — Int<1>, canonical scale 9
    D18S0   => (D18<0>,   18,   0),
    D18S3   => (D18<3>,   18,   3),
    D18S4   => (D18<4>,   18,   4),
    D18S9   => (D18<9>,   18,   9),
    D18S13  => (D18<13>,  18,  13),
    D18S17  => (D18<17>,  18,  17),
    // D38 — Int<2>, canonical scale 19
    D38S0   => (D38<0>,   38,   0),
    D38S2   => (D38<2>,   38,   2),
    D38S6   => (D38<6>,   38,   6),
    D38S9   => (D38<9>,   38,   9),
    D38S10  => (D38<10>,  38,  10),
    D38S12  => (D38<12>,  38,  12),
    D38S17  => (D38<17>,  38,  17),
    D38S18  => (D38<18>,  38,  18),
    D38S19  => (D38<19>,  38,  19),
    D38S28  => (D38<28>,  38,  28),
    D38S37  => (D38<37>,  38,  37),
    // D57 — Int<3>, canonical scale 28
    D57S0   => (D57<0>,   57,   0),
    D57S14  => (D57<14>,  57,  14),
    D57S20  => (D57<20>,  57,  20),
    D57S28  => (D57<28>,  57,  28),
    D57S30  => (D57<30>,  57,  30),
    D57S42  => (D57<42>,  57,  42),
    D57S56  => (D57<56>,  57,  56),
    // D76 — Int<4>, canonical scale 38
    D76S0   => (D76<0>,   76,   0),
    D76S18  => (D76<18>,  76,  18),
    D76S19  => (D76<19>,  76,  19),
    D76S38  => (D76<38>,  76,  38),
    D76S40  => (D76<40>,  76,  40),
    D76S57  => (D76<57>,  76,  57),
    D76S75  => (D76<75>,  76,  75),
    // D115 — Int<6>, canonical scale 57
    D115S0   => (D115<0>,   115,   0),
    D115S28  => (D115<28>,  115,  28),
    D115S50  => (D115<50>,  115,  50),
    D115S57  => (D115<57>,  115,  57),
    D115S86  => (D115<86>,  115,  86),
    D115S114 => (D115<114>, 115, 114),
    // D153 — Int<8>, canonical scale 76
    D153S0   => (D153<0>,   153,   0),
    D153S38  => (D153<38>,  153,  38),
    D153S76  => (D153<76>,  153,  76),
    D153S114 => (D153<114>, 153, 114),
    D153S152 => (D153<152>, 153, 152),
    // D230 — Int<12>, canonical scale 115
    D230S0   => (D230<0>,   230,   0),
    D230S57  => (D230<57>,  230,  57),
    D230S115 => (D230<115>, 230, 115),
    D230S172 => (D230<172>, 230, 172),
    D230S229 => (D230<229>, 230, 229),
    // D307 — Int<16>, canonical scale 153
    D307S0   => (D307<0>,   307,   0),
    D307S30  => (D307<30>,  307,  30),
    D307S50  => (D307<50>,  307,  50),
    D307S70  => (D307<70>,  307,  70),
    D307S76  => (D307<76>,  307,  76),
    D307S120 => (D307<120>, 307, 120),
    D307S153 => (D307<153>, 307, 153),
    D307S230 => (D307<230>, 307, 230),
    D307S306 => (D307<306>, 307, 306),
    // D462 — Int<24>, canonical scale 231
    D462S0   => (D462<0>,   462,   0),
    D462S30  => (D462<30>,  462,  30),
    D462S100 => (D462<100>, 462, 100),
    D462S115 => (D462<115>, 462, 115),
    D462S180 => (D462<180>, 462, 180),
    D462S231 => (D462<231>, 462, 231),
    D462S346 => (D462<346>, 462, 346),
    D462S461 => (D462<461>, 462, 461),
    // D616 — Int<32>, canonical scale 308
    D616S0   => (D616<0>,   616,   0),
    D616S30  => (D616<30>,  616,  30),
    D616S130 => (D616<130>, 616, 130),
    D616S154 => (D616<154>, 616, 154),
    D616S240 => (D616<240>, 616, 240),
    D616S308 => (D616<308>, 616, 308),
    D616S462 => (D616<462>, 616, 462),
    D616S615 => (D616<615>, 616, 615),
    // D924 — Int<48>, canonical scale 462
    D924S0   => (D924<0>,   924,   0),
    D924S30  => (D924<30>,  924,  30),
    D924S180 => (D924<180>, 924, 180),
    D924S231 => (D924<231>, 924, 231),
    D924S350 => (D924<350>, 924, 350),
    D924S462 => (D924<462>, 924, 462),
    D924S693 => (D924<693>, 924, 693),
    D924S923 => (D924<923>, 924, 923),
    // D1232 — Int<64>, canonical scale 616
    D1232S0    => (D1232<0>,    1232,    0),
    D1232S30   => (D1232<30>,   1232,   30),
    D1232S250  => (D1232<250>,  1232,  250),
    D1232S308  => (D1232<308>,  1232,  308),
    D1232S470  => (D1232<470>,  1232,  470),
    D1232S616  => (D1232<616>,  1232,  616),
    D1232S924  => (D1232<924>,  1232,  924),
    D1232S1231 => (D1232<1231>, 1232, 1231),
}

const FUNCS: &[(&str, Function)] = &[
    ("sqrt", Function::Sqrt), ("cbrt", Function::Cbrt), ("exp", Function::Exp),
    ("exp2", Function::Exp2), ("ln", Function::Ln),
    ("log2", Function::Log2), ("log10", Function::Log10),
    ("sin", Function::Sin), ("cos", Function::Cos), ("tan", Function::Tan),
    ("atan", Function::Atan), ("asin", Function::Asin), ("acos", Function::Acos),
    ("sinh", Function::Sinh), ("cosh", Function::Cosh), ("tanh", Function::Tanh),
    ("asinh", Function::Asinh), ("acosh", Function::Acosh), ("atanh", Function::Atanh),
    ("log", Function::Log), ("atan2", Function::Atan2), ("powf", Function::Powf),
    ("hypot", Function::Hypot), ("add", Function::Add), ("sub", Function::Sub),
    ("mul", Function::Mul), ("div", Function::Div), ("rem", Function::Rem),
];

fn corpus_path(name: &str) -> String {
    format!(
        "{}/decimal-scaled-golden/golden/{}.txt",
        env!("CARGO_MANIFEST_DIR"),
        name
    )
}

#[test]
fn golden_multi_tier() {
    let mut total_pass = 0usize;
    let mut total_panic = 0usize;
    let mut total_bad = 0usize;
    for (name, f) in FUNCS {
        let body = match std::fs::read_to_string(corpus_path(name)) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let cases = parser::parse(*f, &body);
        if cases.is_empty() {
            continue;
        }
        let recs = run(
            &CorrectnessTester,
            &DecimalScaledAdapter,
            *f,
            &cases,
            CELLS,
            &[RoundingMode::HalfToEven],
            GP,
        );
        let mut pass = 0usize;
        let mut skip = 0usize;
        for r in &recs {
            match &r.outcome {
                Outcome::Pass => pass += 1,
                Outcome::Skipped => skip += 1,
                Outcome::Panic => {
                    total_panic += 1;
                    eprintln!("  PANIC {name} @({},{}) input={:?}", r.width, r.scale, r.detail);
                }
                other => {
                    total_bad += 1;
                    eprintln!("  BAD {name} @({},{}): {:?} on {:?}", r.width, r.scale, other, r.detail);
                }
            }
        }
        eprintln!("{name}: {pass} pass / {skip} skip across {} cells", recs.len());
        total_pass += pass;
    }
    eprintln!("TOTAL: {total_pass} pass / {total_panic} panic / {total_bad} bad");
    // Correctness is the hard gate: nowhere may decimal-scaled mis-round, use the
    // wrong mode, error, or panic where the result is representable at the tier.
    assert_eq!(total_bad, 0, "mis-rounded / wrong-mode / error cells found");
    assert_eq!(total_panic, 0, "decimal-scaled panicked on a representable cell");
    assert!(total_pass > 0, "no Pass across any cell");
}
