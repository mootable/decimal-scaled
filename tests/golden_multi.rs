//! Multi-tier end-to-end: drive decimal-scaled through the decimal-scaled-golden
//! harness across all 88 band-edge `(width, scale)` cells. Each cell is ONE
//! single-cell subject (`DsSubject<D>` over the concrete decimal type); the
//! runner walks the subject vec and reads each subject's cell + modes/overflow
//! from its `capabilities()`. Requires the wide-tier features (compiles to
//! nothing without them). Full-surface + slow, so `#[ignore]`d — run explicitly:
//!
//! ```text
//! cargo test --release --features wide,x-wide,xx-wide --test golden_multi -- --ignored --nocapture
//! ```
//! Honour `GOLDEN_THREADS` to cap parallelism (default = available cores).
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use std::marker::PhantomData;

use decimal_scaled::{
    DecimalArithmetic, DecimalTranscendental, D115, D1232, D153, D18, D230, D307, D38, D462, D57,
    D616, D76, D924, RoundingMode as DsMode,
};
use decimal_scaled_golden::parser::GoldenCase;
use decimal_scaled_golden::{
    parser, run_parallel, Capabilities, Computed, FnSupport, Function, Outcome, Overflow,
    RoundingMode, RunOnce, Subject, Validator,
};
use std::collections::BTreeMap;

const GP: usize = 1233;

/// (corpus file stem, function) for every function the corpus covers.
const FUNCS: &[(&str, Function)] = &[
    ("sqrt", Function::Sqrt), ("cbrt", Function::Cbrt), ("exp", Function::Exp),
    ("ln", Function::Ln), ("log2", Function::Log2), ("log10", Function::Log10),
    ("exp2", Function::Exp2), ("sin", Function::Sin), ("cos", Function::Cos),
    ("tan", Function::Tan), ("atan", Function::Atan), ("asin", Function::Asin),
    ("acos", Function::Acos), ("sinh", Function::Sinh), ("cosh", Function::Cosh),
    ("tanh", Function::Tanh), ("asinh", Function::Asinh), ("acosh", Function::Acosh),
    ("atanh", Function::Atanh), ("log", Function::Log), ("atan2", Function::Atan2),
    ("powf", Function::Powf), ("hypot", Function::Hypot), ("add", Function::Add),
    ("sub", Function::Sub), ("mul", Function::Mul), ("div", Function::Div),
    ("rem", Function::Rem),
];

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

/// Inherent rounded mul/div aren't on a width-generic trait, so bridge them
/// locally — one delegating impl per width, scale-generic.
trait DsOps: Sized {
    fn ds_mul_with(self, o: Self, m: DsMode) -> Self;
    fn ds_div_with(self, o: Self, m: DsMode) -> Self;
}
macro_rules! impl_ds_ops {
    ($($D:ident),+ $(,)?) => { $(
        impl<const S: u32> DsOps for $D<S> {
            fn ds_mul_with(self, o: Self, m: DsMode) -> Self { self.mul_with(o, m) }
            fn ds_div_with(self, o: Self, m: DsMode) -> Self { self.div_with(o, m) }
        }
    )+ };
}
impl_ds_ops!(D18, D38, D57, D76, D115, D153, D230, D307, D462, D616, D924, D1232);

/// One single-cell decimal-scaled subject over the concrete decimal type `D`.
struct DsSubject<D> {
    width: u32,
    scale: u32,
    storage_bits: u32,
    _p: PhantomData<D>,
}

impl<D> DsSubject<D>
where
    D: DecimalArithmetic + DecimalTranscendental + DsOps,
{
    /// The op only — shared by `eval` (after parse, before format) and the
    /// `compute_thunk` timed closure. `None` if a binary op is missing operand 2.
    fn compute(&self, func: Function, x: D, d2: Option<D>, m: DsMode) -> Option<D> {
        Some(match func {
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
            Function::Log => x.log_strict_with(d2?, m),
            Function::Atan2 => x.atan2_strict_with(d2?, m),
            Function::Powf => x.powf_strict_with(d2?, m),
            Function::Hypot => x.hypot_strict_with(d2?, m),
            Function::Add => x + d2?,
            Function::Sub => x - d2?,
            Function::Mul => x.ds_mul_with(d2?, m),
            Function::Div => x.ds_div_with(d2?, m),
            Function::Rem => x % d2?,
        })
    }

    /// Parse `inputs` to typed values, skipping any not exactly representable.
    fn parse_inputs(&self, inputs: &[&str]) -> Option<(D, Option<D>)>
    where
        D: core::str::FromStr,
    {
        let mut vals: Vec<D> = Vec::with_capacity(inputs.len());
        for s in inputs {
            if frac_len(s) > self.scale as usize {
                return None;
            }
            match s.parse::<D>() {
                Ok(v) => vals.push(v),
                Err(_) => return None,
            }
        }
        let x = *vals.first()?;
        Some((x, vals.get(1).copied()))
    }
}

impl<D> Subject for DsSubject<D>
where
    D: DecimalArithmetic + DecimalTranscendental + DsOps + core::str::FromStr,
{
    fn capabilities(&self) -> Capabilities {
        // decimal-scaled follows Rust's overflow contract: debug panics, release
        // wraps (2's-complement on the integer storage).
        let overflow = if cfg!(debug_assertions) { Overflow::Panic } else { Overflow::Wrap };
        let modes = RoundingMode::ALL.to_vec(); // full rounding-mode coverage
        let mut functions = BTreeMap::new();
        for (_, f) in FUNCS {
            functions.insert(*f, FnSupport { modes: modes.clone(), overflow });
        }
        Capabilities {
            name: "decimal-scaled".to_string(),
            width: self.width,
            scale: self.scale,
            storage_bits: self.storage_bits,
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
        let (x, d2) = match self.parse_inputs(inputs) {
            Some(v) => v,
            None => return Computed::Skip,
        };
        match self.compute(func, x, d2, ds_mode(mode)) {
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
        // Parse the whole batch UP FRONT (untimed).
        let mut batch: Vec<(D, Option<D>)> = Vec::with_capacity(cases.len());
        for case in cases {
            let refs: Vec<&str> = case.inputs.iter().map(|s| s.as_str()).collect();
            if let Some(pair) = self.parse_inputs(&refs) {
                batch.push(pair);
            }
        }
        if batch.is_empty() {
            return None;
        }
        Some(Box::new(move || {
            for (x, d2) in &batch {
                std::hint::black_box(self.compute(func, *x, *d2, m));
            }
        }))
    }
}

/// Build the subject vec: one `DsSubject<D>` per band-edge `(width, scale)` cell.
macro_rules! ds_subjects {
    ($($D:ty => ($w:expr, $s:expr, $bits:expr)),+ $(,)?) => {
        fn ds_subjects() -> Vec<Box<dyn Subject + Sync>> {
            vec![ $(
                Box::new(DsSubject::<$D> { width: $w, scale: $s, storage_bits: $bits, _p: PhantomData })
                    as Box<dyn Subject + Sync>
            ),+ ]
        }
    };
}

ds_subjects! {
    // D18 — Int<1>, 64-bit storage
    D18<0> => (18, 0, 64), D18<3> => (18, 3, 64), D18<4> => (18, 4, 64),
    D18<9> => (18, 9, 64), D18<13> => (18, 13, 64), D18<17> => (18, 17, 64),
    // D38 — Int<2>, 128-bit
    D38<0> => (38, 0, 128), D38<2> => (38, 2, 128), D38<6> => (38, 6, 128),
    D38<9> => (38, 9, 128), D38<10> => (38, 10, 128), D38<12> => (38, 12, 128),
    D38<17> => (38, 17, 128), D38<18> => (38, 18, 128), D38<19> => (38, 19, 128),
    D38<28> => (38, 28, 128), D38<37> => (38, 37, 128),
    // D57 — Int<3>, 192-bit
    D57<0> => (57, 0, 192), D57<14> => (57, 14, 192), D57<20> => (57, 20, 192),
    D57<28> => (57, 28, 192), D57<30> => (57, 30, 192), D57<42> => (57, 42, 192),
    D57<56> => (57, 56, 192),
    // D76 — Int<4>, 256-bit
    D76<0> => (76, 0, 256), D76<18> => (76, 18, 256), D76<19> => (76, 19, 256),
    D76<38> => (76, 38, 256), D76<40> => (76, 40, 256), D76<57> => (76, 57, 256),
    D76<75> => (76, 75, 256),
    // D115 — Int<6>, 384-bit
    D115<0> => (115, 0, 384), D115<28> => (115, 28, 384), D115<50> => (115, 50, 384),
    D115<57> => (115, 57, 384), D115<86> => (115, 86, 384), D115<114> => (115, 114, 384),
    // D153 — Int<8>, 512-bit
    D153<0> => (153, 0, 512), D153<38> => (153, 38, 512), D153<76> => (153, 76, 512),
    D153<114> => (153, 114, 512), D153<152> => (153, 152, 512),
    // D230 — Int<12>, 768-bit
    D230<0> => (230, 0, 768), D230<57> => (230, 57, 768), D230<115> => (230, 115, 768),
    D230<172> => (230, 172, 768), D230<229> => (230, 229, 768),
    // D307 — Int<16>, 1024-bit
    D307<0> => (307, 0, 1024), D307<30> => (307, 30, 1024), D307<50> => (307, 50, 1024),
    D307<70> => (307, 70, 1024), D307<76> => (307, 76, 1024), D307<120> => (307, 120, 1024),
    D307<153> => (307, 153, 1024), D307<230> => (307, 230, 1024), D307<306> => (307, 306, 1024),
    // D462 — Int<24>, 1536-bit
    D462<0> => (462, 0, 1536), D462<30> => (462, 30, 1536), D462<100> => (462, 100, 1536),
    D462<115> => (462, 115, 1536), D462<180> => (462, 180, 1536), D462<231> => (462, 231, 1536),
    D462<346> => (462, 346, 1536), D462<461> => (462, 461, 1536),
    // D616 — Int<32>, 2048-bit
    D616<0> => (616, 0, 2048), D616<30> => (616, 30, 2048), D616<130> => (616, 130, 2048),
    D616<154> => (616, 154, 2048), D616<240> => (616, 240, 2048), D616<308> => (616, 308, 2048),
    D616<462> => (616, 462, 2048), D616<615> => (616, 615, 2048),
    // D924 — Int<48>, 3072-bit
    D924<0> => (924, 0, 3072), D924<30> => (924, 30, 3072), D924<180> => (924, 180, 3072),
    D924<231> => (924, 231, 3072), D924<350> => (924, 350, 3072), D924<462> => (924, 462, 3072),
    D924<693> => (924, 693, 3072), D924<923> => (924, 923, 3072),
    // D1232 — Int<64>, 4096-bit
    D1232<0> => (1232, 0, 4096), D1232<30> => (1232, 30, 4096), D1232<250> => (1232, 250, 4096),
    D1232<308> => (1232, 308, 4096), D1232<470> => (1232, 470, 4096), D1232<616> => (1232, 616, 4096),
    D1232<924> => (1232, 924, 4096), D1232<1231> => (1232, 1231, 4096),
}

fn corpus_path(name: &str) -> String {
    format!("{}/decimal-scaled-golden/golden/{}.txt", env!("CARGO_MANIFEST_DIR"), name)
}

fn thread_count() -> usize {
    std::env::var("GOLDEN_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1))
}

#[test]
#[ignore = "full-surface golden; run via: cargo test --release --features wide,x-wide,xx-wide --test golden_multi -- --ignored --nocapture"]
fn golden_multi_tier() {
    let subjects = ds_subjects();
    let tester = Validator::validation_tester(GP);
    let strategy = RunOnce;
    let threads = thread_count();
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
        let recs = run_parallel(&tester, &strategy, &subjects, *f, &cases, threads);
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
    assert_eq!(total_bad, 0, "mis-rounded / wrong-mode / error cells found");
    assert_eq!(total_panic, 0, "decimal-scaled panicked on a representable cell");
    assert!(total_pass > 0, "no Pass across any cell");
}
