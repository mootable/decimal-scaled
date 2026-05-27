//! The reference [`PrecisionSubject`]: `decimal-scaled` itself, driven
//! through its `*_strict_with(mode)` surface.
//!
//! This subject depends only on the crate under test, so it is usable
//! from BOTH `tests/` (the smoke gate) and `benches/` (the comparative
//! runner). The peer subjects (which pull in the comparator crates) live
//! in `precision_subjects_peers.rs`, included only by the bench.
//!
//! Included via `#[path]` alongside `precision_harness.rs`. The includer
//! exposes the harness as a sibling `mod harness`; this file pulls the
//! harness surface in via `super::harness::*`, so the includer only needs
//! to declare `mod harness` before `mod subject_ds`.

use super::harness::{
    Input, Method, PrecisionSubject, SubjectOutput, Width, raw_to_decimal_string,
};
use decimal_scaled::RoundingMode;
use decimal_scaled::{D18, D38, D57, D76, D115, D153, D230, D307, D462, D616, D924, D1232};
#[allow(unused_imports)]
use decimal_scaled::{DecimalConvert, DecimalTranscendental};
use decimal_scaled::Int;

/// `decimal-scaled` strict kernels — the reference subject. Correctly
/// rounded to 0 storage LSB under every mode (proven by the golden gate),
/// so every cell it produces is `0 (0.00)`.
pub struct DecimalScaledSubject;

/// Evaluate one strict method on a typed decimal `D` whose storage is
/// `Int`, given the raw operand strings at the tier scale. Returns the
/// result's storage integer as a decimal string (the subject's
/// value-as-scaled-integer), which the harness reads back at the scale.
macro_rules! eval_typed {
    ($D:ty, $Int:ty, $method:expr, $input:expr, $mode:expr) => {{
        type D = $D;
        let parse = |s: &str| <$Int>::from_str_radix(s, 10).ok();
        let raw = match parse(&$input.raw) {
            Some(v) => v,
            None => return SubjectOutput::NotApplicable,
        };
        let x = <D>::from_bits(raw);
        let y = match $method {
            Method::Sqrt => x.sqrt_strict_with($mode),
            Method::Cbrt => x.cbrt_strict_with($mode),
            Method::Exp => x.exp_strict_with($mode),
            Method::Ln => x.ln_strict_with($mode),
            Method::Log2 => x.log2_strict_with($mode),
            Method::Log10 => x.log10_strict_with($mode),
            Method::Exp2 => x.exp2_strict_with($mode),
            Method::Sin => x.sin_strict_with($mode),
            Method::Cos => x.cos_strict_with($mode),
            Method::Tan => x.tan_strict_with($mode),
            Method::Atan => x.atan_strict_with($mode),
            Method::Asin => x.asin_strict_with($mode),
            Method::Acos => x.acos_strict_with($mode),
            Method::Sinh => x.sinh_strict_with($mode),
            Method::Cosh => x.cosh_strict_with($mode),
            Method::Tanh => x.tanh_strict_with($mode),
            Method::Asinh => x.asinh_strict_with($mode),
            Method::Acosh => x.acosh_strict_with($mode),
            Method::Atanh => x.atanh_strict_with($mode),
            // Two-argument transcendentals + arithmetic + hypot. Every
            // one consumes the second operand from `input2`.
            Method::Log
            | Method::Atan2
            | Method::Pow
            | Method::Add
            | Method::Sub
            | Method::Mul
            | Method::Div
            | Method::Rem
            | Method::Hypot => {
                let raw2 = match $input.input2.as_deref().and_then(parse) {
                    Some(v) => v,
                    None => return SubjectOutput::NotApplicable,
                };
                let d2 = <D>::from_bits(raw2);
                match $method {
                    Method::Log => x.log_strict_with(d2, $mode),
                    Method::Atan2 => x.atan2_strict_with(d2, $mode),
                    Method::Pow => x.powf_strict_with(d2, $mode),
                    // add / sub / rem are exact at a shared scale (no
                    // rounding step), so they ignore `mode`; mul / div
                    // round the scale-narrowing step under `mode`; hypot
                    // rounds the sqrt under `mode`.
                    Method::Add => x + d2,
                    Method::Sub => x - d2,
                    Method::Rem => x % d2,
                    Method::Mul => x.mul_with(d2, $mode),
                    Method::Div => x.div_with(d2, $mode),
                    Method::Hypot => x.hypot_strict_with(d2, $mode),
                    _ => unreachable!(),
                }
            }
        };
        // The subject's value is its result storage integer rendered as a
        // plain integer string — which the harness reads as the scaled
        // value (no decimal point needed; scale is implied by the tier).
        SubjectOutput::Computed {
            value: raw_to_decimal_string(&y.to_bits().to_string(), $input.scale),
            rounding: $mode,
        }
    }};
}

impl PrecisionSubject for DecimalScaledSubject {
    fn name(&self) -> &str {
        "decimal-scaled"
    }

    fn native_mode(&self) -> RoundingMode {
        RoundingMode::HalfToEven
    }

    fn eval(
        &self,
        method: Method,
        width: Width,
        scale: u32,
        input: &Input,
        mode: RoundingMode,
    ) -> SubjectOutput {
        // The wide tiers carry a second golden cell at SCALE 30 (the low-scale
        // Tang rectangle in `policy::exp`), in addition to their canonical
        // design scale. Pick the const-generic type by the requested scale; the
        // canonical scale is the default. Narrow tiers have a single cell, so
        // their canonical scale is implicit.
        match width {
            // Narrow + mid tiers: the canonical (S/2) scale plus the two band
            // edges {0, capacity-1 = MAX_SCALE}. Each band-edge cell picks the
            // matching const-generic `D###<SCALE>` type (same storage `Int<N>`).
            Width::D18 if scale == 0 => eval_typed!(D18<0>, decimal_scaled::Int<1>, method, input, mode),
            Width::D18 if scale == 3 => eval_typed!(D18<3>, decimal_scaled::Int<1>, method, input, mode),
            Width::D18 if scale == 17 => eval_typed!(D18<17>, decimal_scaled::Int<1>, method, input, mode),
            Width::D18 => eval_typed!(D18<9>, decimal_scaled::Int<1>, method, input, mode),
            Width::D38 if scale == 0 => eval_typed!(D38<0>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 2 => eval_typed!(D38<2>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 6 => eval_typed!(D38<6>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 9 => eval_typed!(D38<9>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 10 => eval_typed!(D38<10>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 12 => eval_typed!(D38<12>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 17 => eval_typed!(D38<17>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 18 => eval_typed!(D38<18>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 if scale == 37 => eval_typed!(D38<37>, decimal_scaled::Int<2>, method, input, mode),
            Width::D38 => eval_typed!(D38<19>, decimal_scaled::Int<2>, method, input, mode),
            Width::D57 if scale == 0 => eval_typed!(D57<0>, Int<3>, method, input, mode),
            Width::D57 if scale == 20 => eval_typed!(D57<20>, Int<3>, method, input, mode),
            Width::D57 if scale == 30 => eval_typed!(D57<30>, Int<3>, method, input, mode),
            Width::D57 if scale == 56 => eval_typed!(D57<56>, Int<3>, method, input, mode),
            Width::D57 => eval_typed!(D57<28>, Int<3>, method, input, mode),
            Width::D76 if scale == 0 => eval_typed!(D76<0>, Int<4>, method, input, mode),
            Width::D76 if scale == 18 => eval_typed!(D76<18>, Int<4>, method, input, mode),
            Width::D76 if scale == 40 => eval_typed!(D76<40>, Int<4>, method, input, mode),
            Width::D76 if scale == 75 => eval_typed!(D76<75>, Int<4>, method, input, mode),
            Width::D76 => eval_typed!(D76<35>, Int<4>, method, input, mode),
            Width::D115 if scale == 0 => eval_typed!(D115<0>, Int<6>, method, input, mode),
            Width::D115 if scale == 50 => eval_typed!(D115<50>, Int<6>, method, input, mode),
            Width::D115 if scale == 114 => eval_typed!(D115<114>, Int<6>, method, input, mode),
            Width::D115 => eval_typed!(D115<57>, Int<6>, method, input, mode),
            Width::D153 if scale == 0 => eval_typed!(D153<0>, Int<8>, method, input, mode),
            Width::D153 if scale == 152 => eval_typed!(D153<152>, Int<8>, method, input, mode),
            Width::D153 => eval_typed!(D153<76>, Int<8>, method, input, mode),
            Width::D230 if scale == 0 => eval_typed!(D230<0>, Int<12>, method, input, mode),
            Width::D230 if scale == 229 => eval_typed!(D230<229>, Int<12>, method, input, mode),
            Width::D230 => eval_typed!(D230<115>, Int<12>, method, input, mode),
            Width::D307 if scale == 0 => eval_typed!(D307<0>, Int<16>, method, input, mode),
            Width::D307 if scale == 30 => eval_typed!(D307<30>, Int<16>, method, input, mode),
            Width::D307 if scale == 50 => eval_typed!(D307<50>, Int<16>, method, input, mode),
            Width::D307 if scale == 70 => eval_typed!(D307<70>, Int<16>, method, input, mode),
            Width::D307 if scale == 120 => eval_typed!(D307<120>, Int<16>, method, input, mode),
            Width::D307 if scale == 306 => eval_typed!(D307<306>, Int<16>, method, input, mode),
            Width::D307 => eval_typed!(D307<150>, Int<16>, method, input, mode),
            Width::D462 if scale == 0 => eval_typed!(D462<0>, Int<24>, method, input, mode),
            Width::D462 if scale == 30 => eval_typed!(D462<30>, Int<24>, method, input, mode),
            Width::D462 if scale == 100 => eval_typed!(D462<100>, Int<24>, method, input, mode),
            Width::D462 if scale == 180 => eval_typed!(D462<180>, Int<24>, method, input, mode),
            Width::D462 if scale == 461 => eval_typed!(D462<461>, Int<24>, method, input, mode),
            Width::D462 => eval_typed!(D462<230>, Int<24>, method, input, mode),
            Width::D616 if scale == 0 => eval_typed!(D616<0>, Int<32>, method, input, mode),
            Width::D616 if scale == 30 => eval_typed!(D616<30>, Int<32>, method, input, mode),
            Width::D616 if scale == 130 => eval_typed!(D616<130>, Int<32>, method, input, mode),
            Width::D616 if scale == 240 => eval_typed!(D616<240>, Int<32>, method, input, mode),
            Width::D616 if scale == 615 => eval_typed!(D616<615>, Int<32>, method, input, mode),
            Width::D616 => eval_typed!(D616<308>, Int<32>, method, input, mode),
            Width::D924 if scale == 0 => eval_typed!(D924<0>, Int<48>, method, input, mode),
            Width::D924 if scale == 30 => eval_typed!(D924<30>, Int<48>, method, input, mode),
            Width::D924 if scale == 180 => eval_typed!(D924<180>, Int<48>, method, input, mode),
            Width::D924 if scale == 350 => eval_typed!(D924<350>, Int<48>, method, input, mode),
            Width::D924 if scale == 923 => eval_typed!(D924<923>, Int<48>, method, input, mode),
            Width::D924 => eval_typed!(D924<460>, Int<48>, method, input, mode),
            Width::D1232 if scale == 0 => eval_typed!(D1232<0>, Int<64>, method, input, mode),
            Width::D1232 if scale == 30 => eval_typed!(D1232<30>, Int<64>, method, input, mode),
            Width::D1232 if scale == 250 => eval_typed!(D1232<250>, Int<64>, method, input, mode),
            Width::D1232 if scale == 470 => eval_typed!(D1232<470>, Int<64>, method, input, mode),
            Width::D1232 if scale == 1231 => eval_typed!(D1232<1231>, Int<64>, method, input, mode),
            Width::D1232 => eval_typed!(D1232<615>, Int<64>, method, input, mode),
        }
    }
}
