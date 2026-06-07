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

// One strict-transcendental dispatch shared by every cell variant. The result is
// the same type `$x`, re-wrapped into the same DsValue variant via `$wrap`.
macro_rules! exec_one {
    ($func:expr, $x:expr, $m:expr, $wrap:path) => {{
        let y = match $func {
            Function::Sqrt => $x.sqrt_strict_with($m),
            Function::Cbrt => $x.cbrt_strict_with($m),
            Function::Exp => $x.exp_strict_with($m),
            Function::Ln => $x.ln_strict_with($m),
            Function::Log2 => $x.log2_strict_with($m),
            Function::Log10 => $x.log10_strict_with($m),
            Function::Exp2 => $x.exp2_strict_with($m),
            Function::Sin => $x.sin_strict_with($m),
            Function::Cos => $x.cos_strict_with($m),
            Function::Tan => $x.tan_strict_with($m),
            Function::Atan => $x.atan_strict_with($m),
            Function::Asin => $x.asin_strict_with($m),
            Function::Acos => $x.acos_strict_with($m),
            Function::Sinh => $x.sinh_strict_with($m),
            Function::Cosh => $x.cosh_strict_with($m),
            Function::Tanh => $x.tanh_strict_with($m),
            Function::Asinh => $x.asinh_strict_with($m),
            Function::Acosh => $x.acosh_strict_with($m),
            Function::Atanh => $x.atanh_strict_with($m),
            _ => return Computed::Skip,
        };
        Computed::Value($wrap(y))
    }};
}

macro_rules! ds_cells {
    ($($variant:ident => ($D:ty, $w:expr, $s:expr)),+ $(,)?) => {
        enum DsValue { $($variant($D),)+ }

        struct DecimalScaledAdapter;

        impl DecimalSubject for DecimalScaledAdapter {
            type Value = DsValue;

            fn capabilities(&self, func: Function) -> Capabilities {
                let supported = matches!(
                    func,
                    Function::Sqrt | Function::Cbrt | Function::Exp | Function::Ln
                        | Function::Log2 | Function::Log10 | Function::Exp2
                        | Function::Sin | Function::Cos | Function::Tan | Function::Atan
                        | Function::Asin | Function::Acos | Function::Sinh | Function::Cosh
                        | Function::Tanh | Function::Asinh | Function::Acosh | Function::Atanh
                );
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
                    $(DsValue::$variant(x) => exec_one!(func, *x, m, DsValue::$variant),)+
                }
            }
        }

        const CELLS: &[(u32, u32)] = &[$(($w, $s)),+];
    };
}

ds_cells! {
    D18S9 => (D18<9>, 18, 9),
    D38S19 => (D38<19>, 38, 19),
    D57S28 => (D57<28>, 57, 28),
    D76S38 => (D76<38>, 76, 38),
    D115S57 => (D115<57>, 115, 57),
    D153S76 => (D153<76>, 153, 76),
    D230S115 => (D230<115>, 230, 115),
    D307S153 => (D307<153>, 307, 153),
    D462S231 => (D462<231>, 462, 231),
    D616S308 => (D616<308>, 616, 308),
    D924S462 => (D924<462>, 924, 462),
    D1232S616 => (D1232<616>, 1232, 616),
}

const FUNCS: &[(&str, Function)] = &[
    ("sqrt", Function::Sqrt), ("exp", Function::Exp), ("ln", Function::Ln),
    ("log2", Function::Log2), ("log10", Function::Log10),
    ("sin", Function::Sin), ("cos", Function::Cos), ("tan", Function::Tan),
    ("atan", Function::Atan), ("asin", Function::Asin), ("acos", Function::Acos),
    ("sinh", Function::Sinh), ("cosh", Function::Cosh), ("tanh", Function::Tanh),
    ("asinh", Function::Asinh), ("acosh", Function::Acosh), ("atanh", Function::Atanh),
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
