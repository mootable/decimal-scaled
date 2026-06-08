//! Multi-tier end-to-end: drive decimal-scaled through the decimal-scaled-golden
//! harness across all 88 band-edge `(width, scale)` cells. Each cell is ONE
//! single-cell subject (`DsSubject<D>` over the concrete decimal type); the new
//! harness is generic over `Subject`, so each subject is run via `Tester::run`
//! (no erasure) and its `SubjectCollector` collected into a `TestCollector`.
//! Requires the wide-tier features (compiles to nothing without them).
//! Full-surface + slow, so `#[ignore]`d — run explicitly:
//!
//! ```text
//! cargo test --release --features wide,x-wide,xx-wide --test golden_multi -- --ignored --nocapture
//! ```
//! Honour `GOLDEN_THREADS` to cap parallelism (default = available cores).
#![cfg(all(feature = "wide", feature = "x-wide", feature = "xx-wide"))]

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use decimal_scaled::{
    DecimalArithmetic, DecimalTranscendental, D115, D1232, D153, D18, D230, D307, D38, D462, D57,
    D616, D76, D924, RoundingMode as DsMode,
};
use decimal_scaled_golden::parser::{self, GoldenCase};
use decimal_scaled_golden::{
    Capabilities, CaseLoader, FnSupport, Function, GoldenValue, Outcome, OverflowValidator,
    Overflow, ParallelTester, RoundingMode, RoundingValidator, RunOnce, Subject, TestCollector,
    Tester,
};
use std::collections::BTreeMap;

const GP: usize = 1233;

/// Every function the corpus covers (missing files just contribute no cases).
const FUNCS: &[Function] = &[
    Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2, Function::Log10,
    Function::Exp2, Function::Sin, Function::Cos, Function::Tan, Function::Atan, Function::Asin,
    Function::Acos, Function::Sinh, Function::Cosh, Function::Tanh, Function::Asinh, Function::Acosh,
    Function::Atanh, Function::Log, Function::Atan2, Function::Powf, Function::Hypot, Function::Add,
    Function::Sub, Function::Mul, Function::Div, Function::Rem,
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

/// The op only (after parse, before format). `d2` is the second operand for
/// binary functions; a missing one is a corpus fault and panics (the harness
/// records it).
fn compute<D>(func: Function, x: D, d2: Option<D>, m: DsMode) -> D
where
    D: DecimalArithmetic + DecimalTranscendental + DsOps + Copy,
{
    let bin = || d2.expect("binary function needs two operands");
    match func {
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
        Function::Log => x.log_strict_with(bin(), m),
        Function::Atan2 => x.atan2_strict_with(bin(), m),
        Function::Powf => x.powf_strict_with(bin(), m),
        Function::Hypot => x.hypot_strict_with(bin(), m),
        Function::Add => x + bin(),
        Function::Sub => x - bin(),
        Function::Mul => x.ds_mul_with(bin(), m),
        Function::Div => x.ds_div_with(bin(), m),
        Function::Rem => x % bin(),
    }
}

/// One single-cell decimal-scaled subject over the concrete decimal type `D`.
struct DsSubject<D> {
    width: u32,
    scale: u32,
    _p: PhantomData<D>,
}

impl<D> Subject for DsSubject<D>
where
    D: DecimalArithmetic
        + DecimalTranscendental
        + DsOps
        + core::str::FromStr
        + core::fmt::Display
        + Copy,
{
    type Value = D;

    fn capabilities(&self) -> Capabilities {
        // decimal-scaled's overflow contract is to PANIC on an out-of-range
        // result, in BOTH debug and release, for every default op — transcendental
        // AND arithmetic (a fixed-width decimal has no infinity/NaN, so a wrapped
        // or saturated value would be a silent wrong number). The opt-out
        // `wrapping_`/`checked_`/`saturating_` variants exist but are not the path
        // tested here.
        let overflow = Overflow::Panic;
        let mut functions = BTreeMap::new();
        for &f in FUNCS {
            functions.insert(f, FnSupport { mode: RoundingMode::HalfToEven, overflow });
        }
        Capabilities {
            name: "decimal-scaled".to_string(),
            width: self.width,
            scale: self.scale,
            functions,
        }
    }

    fn string_to_value(&self, s: &str) -> D {
        s.parse::<D>().unwrap_or_else(|_| panic!("could not parse representable input {s:?}"))
    }

    fn value_to_string(&self, v: &D) -> String {
        v.to_string()
    }

    fn representable(&self, value: &GoldenValue) -> bool {
        // External: decimal-scaled represents the true value iff its own
        // `FromStr` accepts it — `parse` rounds to the cell's scale and returns
        // `ParseError::OutOfRange` when the integer part exceeds the storage. No
        // bit-width math leaks into the tester; this is decimal-scaled's call.
        value.to_decimal_string().parse::<D>().is_ok()
    }

    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[D]) -> D {
        let m = ds_mode(mode);
        move |inputs| compute(func, inputs[0], inputs.get(1).copied(), m)
    }
}

/// Loads each function's corpus ONCE and clones it per request — the per-subject
/// `run` would otherwise re-read+re-parse the (multi-MB) files 88 times.
struct CachingLoader {
    dir: String,
    cache: Mutex<HashMap<Function, Arc<Vec<GoldenCase>>>>,
}

impl CachingLoader {
    fn new(dir: String) -> CachingLoader {
        CachingLoader { dir, cache: Mutex::new(HashMap::new()) }
    }
}

impl CaseLoader for CachingLoader {
    fn load(&self, function: Function) -> Vec<GoldenCase> {
        let mut cache = self.cache.lock().unwrap();
        let arc = cache
            .entry(function)
            .or_insert_with(|| {
                let path = format!("{}/{}.txt", self.dir, function.name());
                let body = std::fs::read_to_string(&path).unwrap_or_default();
                Arc::new(parser::parse(function, &body))
            })
            .clone();
        drop(cache);
        (*arc).clone()
    }
}

/// Run one `tester.run` per band-edge `(width, scale)` cell, collecting each
/// subject's `SubjectCollector` into one `TestCollector`.
macro_rules! run_subjects {
    ($tester:expr, $funcs:expr, $($D:ty => ($w:expr, $s:expr)),+ $(,)?) => {{
        let mut tc = TestCollector::new();
        $(
            tc.add($tester.run(
                &DsSubject::<$D> { width: $w, scale: $s, _p: PhantomData },
                $funcs,
            ));
        )+
        tc
    }};
}

fn proof_dir() -> String {
    format!("{}/decimal-scaled-golden/golden", env!("CARGO_MANIFEST_DIR"))
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
    let funcs: Vec<Function> = FUNCS.to_vec();
    let tester = ParallelTester {
        threads: thread_count(),
        strategy: RunOnce,
        loader: Box::new(CachingLoader::new(proof_dir())),
        validators: vec![
            Box::new(RoundingValidator { gen_precision: GP }),
            Box::new(OverflowValidator),
        ],
    };

    let tc = run_subjects! { tester, &funcs,
        // D18 — Int<1>, 64-bit storage
        D18<0> => (18, 0), D18<3> => (18, 3), D18<4> => (18, 4),
        D18<9> => (18, 9), D18<13> => (18, 13), D18<17> => (18, 17),
        // D38 — Int<2>, 128-bit
        D38<0> => (38, 0), D38<2> => (38, 2), D38<6> => (38, 6),
        D38<9> => (38, 9), D38<10> => (38, 10), D38<12> => (38, 12),
        D38<17> => (38, 17), D38<18> => (38, 18), D38<19> => (38, 19),
        D38<28> => (38, 28), D38<37> => (38, 37),
        // D57 — Int<3>, 192-bit
        D57<0> => (57, 0), D57<14> => (57, 14), D57<20> => (57, 20),
        D57<28> => (57, 28), D57<30> => (57, 30), D57<42> => (57, 42),
        D57<56> => (57, 56),
        // D76 — Int<4>, 256-bit
        D76<0> => (76, 0), D76<18> => (76, 18), D76<19> => (76, 19),
        D76<38> => (76, 38), D76<40> => (76, 40), D76<57> => (76, 57),
        D76<75> => (76, 75),
        // D115 — Int<6>, 384-bit
        D115<0> => (115, 0), D115<28> => (115, 28), D115<50> => (115, 50),
        D115<57> => (115, 57), D115<86> => (115, 86), D115<114> => (115, 114),
        // D153 — Int<8>, 512-bit
        D153<0> => (153, 0), D153<38> => (153, 38), D153<76> => (153, 76),
        D153<114> => (153, 114), D153<152> => (153, 152),
        // D230 — Int<12>, 768-bit
        D230<0> => (230, 0), D230<57> => (230, 57), D230<115> => (230, 115),
        D230<172> => (230, 172), D230<229> => (230, 229),
        // D307 — Int<16>, 1024-bit
        D307<0> => (307, 0), D307<30> => (307, 30), D307<50> => (307, 50),
        D307<70> => (307, 70), D307<76> => (307, 76), D307<120> => (307, 120),
        D307<153> => (307, 153), D307<230> => (307, 230), D307<306> => (307, 306),
        // D462 — Int<24>, 1536-bit
        D462<0> => (462, 0), D462<30> => (462, 30), D462<100> => (462, 100),
        D462<115> => (462, 115), D462<180> => (462, 180), D462<231> => (462, 231),
        D462<346> => (462, 346), D462<461> => (462, 461),
        // D616 — Int<32>, 2048-bit
        D616<0> => (616, 0), D616<30> => (616, 30), D616<130> => (616, 130),
        D616<154> => (616, 154), D616<240> => (616, 240), D616<308> => (616, 308),
        D616<462> => (616, 462), D616<615> => (616, 615),
        // D924 — Int<48>, 3072-bit
        D924<0> => (924, 0), D924<30> => (924, 30), D924<180> => (924, 180),
        D924<231> => (924, 231), D924<350> => (924, 350), D924<462> => (924, 462),
        D924<693> => (924, 693), D924<923> => (924, 923),
        // D1232 — Int<64>, 4096-bit
        D1232<0> => (1232, 0), D1232<30> => (1232, 30), D1232<250> => (1232, 250),
        D1232<308> => (1232, 308), D1232<470> => (1232, 470), D1232<616> => (1232, 616),
        D1232<924> => (1232, 924), D1232<1231> => (1232, 1231),
    };

    let mut pass = 0usize;
    let mut skip = 0usize;
    let mut panic = 0usize;
    let mut bad = 0usize;
    for subject in &tc.subjects {
        let (w, s) = (subject.capabilities.width, subject.capabilities.scale);
        for fc in &subject.functions {
            for cell in &fc.cells {
                for outcome in &cell.validations {
                    match outcome {
                        Outcome::Pass => pass += 1,
                        Outcome::Skipped => skip += 1,
                        Outcome::Precision { .. } => {}
                        Outcome::Panic => {
                            panic += 1;
                            eprintln!("  PANIC {} @({w},{s}) input={:?}", fc.function.name(), cell.inputs);
                        }
                        other => {
                            bad += 1;
                            eprintln!("  BAD {} @({w},{s}): {:?} on {:?}", fc.function.name(), other, cell.inputs);
                        }
                    }
                }
            }
        }
    }
    eprintln!("TOTAL: {pass} pass / {skip} skip / {panic} panic / {bad} bad");
    assert_eq!(bad, 0, "mis-rounded / wrong-mode / error cells found");
    assert_eq!(panic, 0, "decimal-scaled panicked on a representable cell");
    assert!(pass > 0, "no Pass across any cell");
}
