//! decimal-scaled subjects for the `decimal-scaled-golden` harness.
//!
//! `DsSubject<D>` is one single-cell subject over a concrete decimal type `D`
//! (e.g. `D38<19>`); the harness is generic over `DecimalSubject`, so each subject
//! runs via `GoldenRunner::run` with no erasure. The adapter lives here (not in a
//! test) so both the `golden_multi` full-surface gate and the `golden_proof`
//! single-cell proof reuse it — and so `golden-competitors` can pull decimal-scaled
//! into a comparison run alongside competitor subjects.

use std::collections::BTreeMap;
use std::marker::PhantomData;

use decimal_scaled::{
    DecimalArithmetic, DecimalTranscendental, D115, D1232, D153, D18, D230, D307, D38, D462, D57,
    D616, D76, D924, RoundingMode as DsMode,
};
use decimal_scaled_golden::{
    Capabilities, Computed, DecimalSubject, FnSupport, Function, Limits, Overflow, Radix,
    RoundingMode,
};

/// Generation precision / rounding guard of the golden set (the file `#` header
/// carries the authoritative values; these mirror them for the validators).
pub const GEN_PRECISION: usize = 1233;
pub const GUARD: usize = 2;

/// Every function the golden set covers (a missing file just contributes no cases).
pub const FUNCS: &[Function] = &[
    Function::Sqrt, Function::Cbrt, Function::Exp, Function::Ln, Function::Log2, Function::Log10,
    Function::Exp2, Function::Sin, Function::Cos, Function::Tan, Function::Atan, Function::Asin,
    Function::Acos, Function::Sinh, Function::Cosh, Function::Tanh, Function::Asinh, Function::Acosh,
    Function::Atanh, Function::Log, Function::Atan2, Function::Powf, Function::Hypot, Function::Add,
    Function::Sub, Function::Mul, Function::Div, Function::Rem,
];

/// Absolute path to the harness's committed golden set. `env!` is baked at THIS
/// crate's compile time, so the path stays correct no matter which crate calls it.
pub fn golden_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../decimal-scaled-golden/golden")
}

/// Worker-thread cap, honouring `GOLDEN_THREADS` (default = available cores).
pub fn thread_count() -> usize {
    std::env::var("GOLDEN_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1))
}

/// Map the harness rounding mode onto decimal-scaled's.
pub fn ds_mode(m: RoundingMode) -> DsMode {
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
pub trait DsOps: Sized {
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

/// The op only (after parse, before format). `d2` is the second operand for binary
/// functions; a missing one is a golden-data fault and panics (the harness records it).
pub fn compute<D>(func: Function, x: D, d2: Option<D>, m: DsMode) -> D
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

/// One single-cell decimal-scaled subject over the concrete decimal type `D`. The
/// `width`/`scale` ride in `Capabilities::config` as report metadata only.
pub struct DsSubject<D> {
    pub width: u32,
    pub scale: u32,
    pub _p: PhantomData<D>,
}

impl<D> DsSubject<D> {
    pub fn new(width: u32, scale: u32) -> DsSubject<D> {
        DsSubject { width, scale, _p: PhantomData }
    }
}

impl<D> DecimalSubject for DsSubject<D>
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
        // decimal-scaled's overflow contract is to PANIC on an out-of-range result,
        // in BOTH debug and release, for every default op — transcendental AND
        // arithmetic (a fixed-width decimal has no infinity/NaN). The opt-out
        // `wrapping_`/`checked_`/`saturating_` variants are not the path tested here.
        let mut functions = BTreeMap::new();
        for &f in FUNCS {
            functions.insert(f, FnSupport { mode: RoundingMode::HalfToEven, overflow: Overflow::Panic });
        }
        let mut config = BTreeMap::new();
        config.insert("width".into(), self.width.to_string());
        config.insert("scale".into(), self.scale.to_string());
        Capabilities { name: "decimal-scaled".into(), radix: Radix::Decimal, config, functions }
    }

    fn string_to_value(&self, s: &str) -> D {
        s.parse::<D>().unwrap_or_else(|_| panic!("could not parse representable input {s:?}"))
    }

    fn value_to_string(&self, v: &D) -> String {
        v.to_string()
    }

    fn limits(&self, _value: &str) -> Limits {
        // The exact storage envelope, in decimal — decimal-scaled's own MIN/MAX
        // constants. No bit-width math leaks into the harness.
        Limits {
            min_value: Some(<D as DecimalArithmetic>::MIN.to_string()),
            max_value: Some(<D as DecimalArithmetic>::MAX.to_string()),
            max_precision: self.scale,
        }
    }

    fn execute(
        &self,
        func: Function,
        mode: RoundingMode,
        _overflow: Overflow,
    ) -> impl Fn(&[D]) -> Computed<D> {
        let m = ds_mode(mode);
        // The strict op panics on overflow; the harness catches that as
        // `Computed::Panic` (a test failure judged against the cell's range).
        move |inputs| Computed::Value(compute(func, inputs[0], inputs.get(1).copied(), m))
    }
}
