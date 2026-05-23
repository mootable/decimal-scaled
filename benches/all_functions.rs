//! Per-function benchmark sweep over every public surface of `D38<12>`.
//!
//! Companion to `mul_div_candidates.rs`. Where that bench compares
//! several candidate algorithms against the production `Mul` / `Div`
//! at multiple operand magnitudes, this bench covers the *full* public
//! API of `decimal-scaled` -- one bench per function, one representative
//! input -- so absolute timings can be tracked over time.
//!
//! No comparisons, no alternate implementations. Each bench measures
//! exactly one production code path. Operand inputs are `black_box`-ed
//! to defeat constant folding; outputs are returned from the closure
//! so the optimiser cannot drop the call.
//!
//! # Layout
//!
//! Benches are grouped by source module so the criterion HTML report
//! mirrors the crate's file layout:
//!
//! - `core_type`   -- `from_bits` / `to_bits` / `multiplier` / `Default`
//! - `arithmetic`  -- operators + math methods + predicates
//! - `conversions` -- From / TryFrom / `from_f64` / `to_int` / `to_f64` / `to_f32`
//! - `consts`      -- `pi` / `tau` / `half_pi` / `quarter_pi` / `golden` / `e`
//! - `powers`      -- `pow` / `powi` / `powf` / `sqrt` / `cbrt` / `mul_add` / `hypot` + checked/wrapping/saturating/overflowing `pow`
//! - `log_exp`     -- `ln` / `log` / `log2` / `log10` / `exp` / `exp2`
//! - `trig`        -- `sin`..`atanh` / `to_degrees` / `to_radians`
//! - `bitwise`     -- bitwise operators + bit-counting helpers
//! - `overflow`    -- `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` for add/sub/mul/div/rem/neg
//! - `display`     -- `Display` / `Debug` / `FromStr`
//! - `compat`      -- `from_num` / `to_num` (i64f64 migration shim)
//!
//! Run with `cargo bench -p decimal-scaled --bench all_functions`.
//!
//! # Inputs
//!
//! Inputs are picked to exercise the typical CAD-magnitude path
//! (raw ~1.5e15 scaled = 1500 model units at SCALE=12) -- not the
//! near-overflow boundary. The boundary is the job of
//! `mul_div_candidates.rs`.
use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::{D38, D38s12, DecimalConstants, Int};
use std::hint::black_box;

type D = D38<12>;

/// `D38`'s storage (`Int<2>`) from a raw two's-complement `i128` bit
/// pattern, usable in `const` context. The low/high limbs are the
/// little-endian halves of the i128.
const fn raw(v: i128) -> Int<2> {
    Int::<2>::from_limbs([v as u64, (v >> 64) as u64])
}

// Representative inputs. RAW_A and RAW_B are mid-magnitude (well within
// i128 range, above i64 boundary) so the production fast paths stay
// representative of typical CAD use.
const RAW_A: i128 = 1_500_000_000_000; // == 1.5
const RAW_B: i128 = 2_300_000_000_000; // == 2.3
const A: D = D::from_bits(raw(RAW_A));
const B: D = D::from_bits(raw(RAW_B));
// Small positive value safe for log/sqrt/etc.
const POS: D = D::from_bits(raw(2_000_000_000_000)); // == 2.0
// Small angle in radians, ~0.5 (~28.6 degrees).
const SMALL_ANGLE: D = D::from_bits(raw(500_000_000_000)); // == 0.5
// Small unit value safe for asin/acos/atanh (<1).
const UNIT_FRAC: D = D::from_bits(raw(300_000_000_000)); // == 0.3

// =============================================================================
// core_type
// =============================================================================
fn bench_core_type(c: &mut Criterion) {
    let mut g = c.benchmark_group("core_type");

    g.bench_function("from_bits", |bn| bn.iter(|| D::from_bits(raw(black_box(RAW_A)))));
    g.bench_function("to_bits", |bn| bn.iter(|| black_box(A).to_bits()));
    g.bench_function("multiplier", |bn| bn.iter(D::multiplier));
    g.bench_function("Default::default", |bn| bn.iter(D::default));

    g.finish();
}

// =============================================================================
// arithmetic
// =============================================================================
fn bench_arithmetic(c: &mut Criterion) {
    let mut g = c.benchmark_group("arithmetic");

    // Operators
    g.bench_function("Add", |bn| bn.iter(|| black_box(A) + black_box(B)));
    g.bench_function("Sub", |bn| bn.iter(|| black_box(A) - black_box(B)));
    g.bench_function("Mul", |bn| bn.iter(|| black_box(A) * black_box(B)));
    g.bench_function("Div", |bn| bn.iter(|| black_box(A) / black_box(B)));
    g.bench_function("Rem", |bn| bn.iter(|| black_box(A) % black_box(B)));
    g.bench_function("Neg", |bn| bn.iter(|| -black_box(A)));
    g.bench_function("AddAssign", |bn| {
        bn.iter(|| {
            let mut a = black_box(A);
            a += black_box(B);
            a
        })
    });
    g.bench_function("SubAssign", |bn| {
        bn.iter(|| {
            let mut a = black_box(A);
            a -= black_box(B);
            a
        })
    });
    g.bench_function("MulAssign", |bn| {
        bn.iter(|| {
            let mut a = black_box(A);
            a *= black_box(B);
            a
        })
    });
    g.bench_function("DivAssign", |bn| {
        bn.iter(|| {
            let mut a = black_box(A);
            a /= black_box(B);
            a
        })
    });
    g.bench_function("RemAssign", |bn| {
        bn.iter(|| {
            let mut a = black_box(A);
            a %= black_box(B);
            a
        })
    });

    // Math methods
    g.bench_function("abs", |bn| bn.iter(|| black_box(A).abs()));
    g.bench_function("signum", |bn| bn.iter(|| black_box(A).signum()));
    g.bench_function("floor", |bn| bn.iter(|| black_box(A).floor()));
    g.bench_function("ceil", |bn| bn.iter(|| black_box(A).ceil()));
    g.bench_function("round", |bn| bn.iter(|| black_box(A).round()));
    g.bench_function("trunc", |bn| bn.iter(|| black_box(A).trunc()));
    g.bench_function("fract", |bn| bn.iter(|| black_box(A).fract()));
    g.bench_function("min", |bn| bn.iter(|| black_box(A).min(black_box(B))));
    g.bench_function("max", |bn| bn.iter(|| black_box(A).max(black_box(B))));
    g.bench_function("clamp", |bn| {
        let lo = D::from_bits(raw(0));
        let hi = D::from_bits(raw(5_000_000_000_000));
        bn.iter(|| black_box(A).clamp(black_box(lo), black_box(hi)))
    });
    g.bench_function("recip", |bn| bn.iter(|| black_box(A).recip()));
    g.bench_function("copysign", |bn| {
        bn.iter(|| black_box(A).copysign(black_box(B)))
    });
    g.bench_function("div_euclid", |bn| {
        bn.iter(|| black_box(A).div_euclid(black_box(B)))
    });
    g.bench_function("rem_euclid", |bn| {
        bn.iter(|| black_box(A).rem_euclid(black_box(B)))
    });
    g.bench_function("div_floor", |bn| {
        bn.iter(|| black_box(A).div_floor(black_box(B)))
    });
    g.bench_function("div_ceil", |bn| {
        bn.iter(|| black_box(A).div_ceil(black_box(B)))
    });
    g.bench_function("abs_diff", |bn| {
        bn.iter(|| black_box(A).abs_diff(black_box(B)))
    });
    g.bench_function("midpoint", |bn| {
        bn.iter(|| black_box(A).midpoint(black_box(B)))
    });

    // Predicates -- these should be ~free, included for completeness.
    g.bench_function("is_nan", |bn| bn.iter(|| black_box(A).is_nan()));
    g.bench_function("is_infinite", |bn| bn.iter(|| black_box(A).is_infinite()));
    g.bench_function("is_finite", |bn| bn.iter(|| black_box(A).is_finite()));
    g.bench_function("is_normal", |bn| bn.iter(|| black_box(A).is_normal()));
    g.bench_function("is_zero", |bn| bn.iter(|| black_box(A).is_zero()));
    g.bench_function("is_positive", |bn| bn.iter(|| black_box(A).is_positive()));
    g.bench_function("is_negative", |bn| bn.iter(|| black_box(A).is_negative()));

    g.finish();
}

// =============================================================================
// conversions
// =============================================================================
fn bench_conversions(c: &mut Criterion) {
    let mut g = c.benchmark_group("conversions");

    // From<integer>
    g.bench_function("From_i8", |bn| bn.iter(|| D::from(black_box(42_i8))));
    g.bench_function("From_i16", |bn| bn.iter(|| D::from(black_box(4242_i16))));
    g.bench_function("From_i32", |bn| bn.iter(|| D::from(black_box(424_242_i32))));
    g.bench_function("From_i64", |bn| bn.iter(|| D::from(black_box(424_242_i64))));
    g.bench_function("From_u8", |bn| bn.iter(|| D::from(black_box(42_u8))));
    g.bench_function("From_u16", |bn| bn.iter(|| D::from(black_box(4242_u16))));
    g.bench_function("From_u32", |bn| bn.iter(|| D::from(black_box(424_242_u32))));
    g.bench_function("From_u64", |bn| bn.iter(|| D::from(black_box(424_242_u64))));

    // TryFrom<wide integer / float>
    g.bench_function("TryFrom_i128", |bn| {
        bn.iter(|| D::try_from(black_box(424_242_i128)))
    });
    g.bench_function("TryFrom_u128", |bn| {
        bn.iter(|| D::try_from(black_box(424_242_u128)))
    });
    g.bench_function("TryFrom_f32", |bn| {
        bn.iter(|| D::try_from(black_box(1.5_f32)))
    });
    g.bench_function("TryFrom_f64", |bn| {
        bn.iter(|| D::try_from(black_box(1.5_f64)))
    });

    // Saturating float bridge (no `TryFrom` equivalent: saturates on
    // NaN/inf/overflow instead of erroring).
    g.bench_function("from_f64", |bn| bn.iter(|| D::from_f64(black_box(1.5_f64))));
    g.bench_function("to_int", |bn| bn.iter(|| black_box(A).to_int()));
    g.bench_function("to_f64", |bn| bn.iter(|| black_box(A).to_f64()));
    g.bench_function("to_f32", |bn| bn.iter(|| black_box(A).to_f32()));

    g.finish();
}

// =============================================================================
// consts (DecimalConstants trait)
// =============================================================================
fn bench_consts(c: &mut Criterion) {
    let mut g = c.benchmark_group("consts");

    // Each call rescales a 35-digit i128 reference constant down to
    // SCALE=12 with half-away-from-zero rounding.
    g.bench_function("pi", |bn| bn.iter(D::pi));
    g.bench_function("tau", |bn| bn.iter(D::tau));
    g.bench_function("half_pi", |bn| bn.iter(D::half_pi));
    g.bench_function("quarter_pi", |bn| bn.iter(D::quarter_pi));
    g.bench_function("golden", |bn| bn.iter(D::golden));
    g.bench_function("e", |bn| bn.iter(D::e));

    g.finish();
}

// =============================================================================
// powers
// =============================================================================
fn bench_powers(c: &mut Criterion) {
    let mut g = c.benchmark_group("powers");

    g.bench_function("pow_8", |bn| {
        bn.iter(|| black_box(POS).pow(black_box(8_u32)))
    });
    g.bench_function("powi_neg3", |bn| {
        bn.iter(|| black_box(POS).powi(black_box(-3_i32)))
    });
    g.bench_function("powf", |bn| {
        let exp = D::from_bits(raw(2_500_000_000_000)); // == 2.5
        bn.iter(|| black_box(POS).powf(black_box(exp)))
    });
    g.bench_function("sqrt", |bn| bn.iter(|| black_box(POS).sqrt()));
    g.bench_function("cbrt", |bn| bn.iter(|| black_box(POS).cbrt()));
    g.bench_function("mul_add", |bn| {
        bn.iter(|| black_box(A).mul_add(black_box(B), black_box(POS)))
    });
    g.bench_function("hypot", |bn| bn.iter(|| black_box(A).hypot(black_box(B))));

    // Overflow-variant `pow` family.
    g.bench_function("checked_pow_8", |bn| {
        bn.iter(|| black_box(POS).checked_pow(black_box(8_u32)))
    });
    g.bench_function("wrapping_pow_8", |bn| {
        bn.iter(|| black_box(POS).wrapping_pow(black_box(8_u32)))
    });
    g.bench_function("saturating_pow_8", |bn| {
        bn.iter(|| black_box(POS).saturating_pow(black_box(8_u32)))
    });
    g.bench_function("overflowing_pow_8", |bn| {
        bn.iter(|| black_box(POS).overflowing_pow(black_box(8_u32)))
    });

    g.finish();
}

// =============================================================================
// log_exp (std-only)
// =============================================================================
fn bench_log_exp(c: &mut Criterion) {
    let mut g = c.benchmark_group("log_exp");

    g.bench_function("ln", |bn| bn.iter(|| black_box(POS).ln()));
    g.bench_function("log", |bn| {
        let base = D::from_bits(raw(10_000_000_000_000)); // == 10
        bn.iter(|| black_box(POS).log(black_box(base)))
    });
    g.bench_function("log2", |bn| bn.iter(|| black_box(POS).log2()));
    g.bench_function("log10", |bn| bn.iter(|| black_box(POS).log10()));
    g.bench_function("exp", |bn| bn.iter(|| black_box(SMALL_ANGLE).exp()));
    g.bench_function("exp2", |bn| bn.iter(|| black_box(SMALL_ANGLE).exp2()));

    g.finish();
}

// =============================================================================
// trig (std-only)
// =============================================================================
fn bench_trig(c: &mut Criterion) {
    let mut g = c.benchmark_group("trig");

    g.bench_function("sin", |bn| bn.iter(|| black_box(SMALL_ANGLE).sin()));
    g.bench_function("cos", |bn| bn.iter(|| black_box(SMALL_ANGLE).cos()));
    g.bench_function("tan", |bn| bn.iter(|| black_box(SMALL_ANGLE).tan()));
    g.bench_function("asin", |bn| bn.iter(|| black_box(UNIT_FRAC).asin()));
    g.bench_function("acos", |bn| bn.iter(|| black_box(UNIT_FRAC).acos()));
    g.bench_function("atan", |bn| bn.iter(|| black_box(SMALL_ANGLE).atan()));
    g.bench_function("atan2", |bn| {
        bn.iter(|| black_box(SMALL_ANGLE).atan2(black_box(POS)))
    });
    g.bench_function("sinh", |bn| bn.iter(|| black_box(SMALL_ANGLE).sinh()));
    g.bench_function("cosh", |bn| bn.iter(|| black_box(SMALL_ANGLE).cosh()));
    g.bench_function("tanh", |bn| bn.iter(|| black_box(SMALL_ANGLE).tanh()));
    g.bench_function("asinh", |bn| bn.iter(|| black_box(SMALL_ANGLE).asinh()));
    g.bench_function("acosh", |bn| bn.iter(|| black_box(POS).acosh()));
    g.bench_function("atanh", |bn| bn.iter(|| black_box(UNIT_FRAC).atanh()));
    g.bench_function("to_degrees", |bn| {
        bn.iter(|| black_box(SMALL_ANGLE).to_degrees())
    });
    g.bench_function("to_radians", |bn| {
        let deg = D::from_bits(raw(45_000_000_000_000)); // == 45 degrees
        bn.iter(|| black_box(deg).to_radians())
    });

    g.finish();
}

// =============================================================================
// bitwise
// =============================================================================
fn bench_bitwise(c: &mut Criterion) {
    let mut g = c.benchmark_group("bitwise");

    // Operators
    g.bench_function("BitAnd", |bn| bn.iter(|| black_box(A) & black_box(B)));
    g.bench_function("BitOr", |bn| bn.iter(|| black_box(A) | black_box(B)));
    g.bench_function("BitXor", |bn| bn.iter(|| black_box(A) ^ black_box(B)));
    g.bench_function("Shl", |bn| bn.iter(|| black_box(A) << black_box(3_u32)));
    g.bench_function("Shr", |bn| bn.iter(|| black_box(A) >> black_box(3_u32)));
    g.bench_function("Not", |bn| bn.iter(|| !black_box(A)));

    // Helper methods
    g.bench_function("unsigned_shr", |bn| {
        bn.iter(|| black_box(A).unsigned_shr(black_box(3_u32)))
    });
    g.bench_function("rotate_left", |bn| {
        bn.iter(|| black_box(A).rotate_left(black_box(7_u32)))
    });
    g.bench_function("rotate_right", |bn| {
        bn.iter(|| black_box(A).rotate_right(black_box(7_u32)))
    });
    g.bench_function("leading_zeros", |bn| {
        bn.iter(|| black_box(A).leading_zeros())
    });
    g.bench_function("trailing_zeros", |bn| {
        bn.iter(|| black_box(A).trailing_zeros())
    });
    g.bench_function("count_ones", |bn| bn.iter(|| black_box(A).count_ones()));
    g.bench_function("count_zeros", |bn| bn.iter(|| black_box(A).count_zeros()));
    g.bench_function("is_power_of_two", |bn| {
        bn.iter(|| black_box(A).is_power_of_two())
    });
    g.bench_function("next_power_of_two", |bn| {
        bn.iter(|| black_box(A).next_power_of_two())
    });

    g.finish();
}

// =============================================================================
// overflow_variants
// =============================================================================
fn bench_overflow_variants(c: &mut Criterion) {
    let mut g = c.benchmark_group("overflow");

    // Add family
    g.bench_function("checked_add", |bn| {
        bn.iter(|| black_box(A).checked_add(black_box(B)))
    });
    g.bench_function("wrapping_add", |bn| {
        bn.iter(|| black_box(A).wrapping_add(black_box(B)))
    });
    g.bench_function("saturating_add", |bn| {
        bn.iter(|| black_box(A).saturating_add(black_box(B)))
    });
    g.bench_function("overflowing_add", |bn| {
        bn.iter(|| black_box(A).overflowing_add(black_box(B)))
    });

    // Sub family
    g.bench_function("checked_sub", |bn| {
        bn.iter(|| black_box(A).checked_sub(black_box(B)))
    });
    g.bench_function("wrapping_sub", |bn| {
        bn.iter(|| black_box(A).wrapping_sub(black_box(B)))
    });
    g.bench_function("saturating_sub", |bn| {
        bn.iter(|| black_box(A).saturating_sub(black_box(B)))
    });
    g.bench_function("overflowing_sub", |bn| {
        bn.iter(|| black_box(A).overflowing_sub(black_box(B)))
    });

    // Neg family
    g.bench_function("checked_neg", |bn| bn.iter(|| black_box(A).checked_neg()));
    g.bench_function("wrapping_neg", |bn| bn.iter(|| black_box(A).wrapping_neg()));
    g.bench_function("saturating_neg", |bn| {
        bn.iter(|| black_box(A).saturating_neg())
    });
    g.bench_function("overflowing_neg", |bn| {
        bn.iter(|| black_box(A).overflowing_neg())
    });

    // Mul family
    g.bench_function("checked_mul", |bn| {
        bn.iter(|| black_box(A).checked_mul(black_box(B)))
    });
    g.bench_function("wrapping_mul", |bn| {
        bn.iter(|| black_box(A).wrapping_mul(black_box(B)))
    });
    g.bench_function("saturating_mul", |bn| {
        bn.iter(|| black_box(A).saturating_mul(black_box(B)))
    });
    g.bench_function("overflowing_mul", |bn| {
        bn.iter(|| black_box(A).overflowing_mul(black_box(B)))
    });

    // Div family
    g.bench_function("checked_div", |bn| {
        bn.iter(|| black_box(A).checked_div(black_box(B)))
    });
    g.bench_function("wrapping_div", |bn| {
        bn.iter(|| black_box(A).wrapping_div(black_box(B)))
    });
    g.bench_function("saturating_div", |bn| {
        bn.iter(|| black_box(A).saturating_div(black_box(B)))
    });
    g.bench_function("overflowing_div", |bn| {
        bn.iter(|| black_box(A).overflowing_div(black_box(B)))
    });

    // Rem family
    g.bench_function("checked_rem", |bn| {
        bn.iter(|| black_box(A).checked_rem(black_box(B)))
    });
    g.bench_function("wrapping_rem", |bn| {
        bn.iter(|| black_box(A).wrapping_rem(black_box(B)))
    });
    g.bench_function("overflowing_rem", |bn| {
        bn.iter(|| black_box(A).overflowing_rem(black_box(B)))
    });

    g.finish();
}

// =============================================================================
// display / parse
// =============================================================================
fn bench_display(c: &mut Criterion) {
    let mut g = c.benchmark_group("display");

    g.bench_function("Display", |bn| bn.iter(|| format!("{}", black_box(A))));
    g.bench_function("Debug", |bn| bn.iter(|| format!("{:?}", black_box(A))));
    g.bench_function("LowerHex", |bn| bn.iter(|| format!("{:x}", black_box(A))));
    g.bench_function("Binary", |bn| bn.iter(|| format!("{:b}", black_box(A))));
    g.bench_function("FromStr_parse", |bn| {
        let s = "1.5";
        bn.iter(|| black_box(s).parse::<D38s12>())
    });

    g.finish();
}

// =============================================================================
// fixed_compat shim
// =============================================================================
fn bench_compat(c: &mut Criterion) {
    let mut g = c.benchmark_group("compat");

    g.bench_function("from_num_i32", |bn| {
        bn.iter(|| D::from_num(black_box(424_242_i32)))
    });
    g.bench_function("from_num_i64", |bn| {
        bn.iter(|| D::from_num(black_box(424_242_i64)))
    });
    g.bench_function("from_num_f64", |bn| {
        bn.iter(|| D::from_num(black_box(1.5_f64)))
    });
    g.bench_function("to_num_i32", |bn| bn.iter(|| black_box(A).to_num::<i32>()));
    g.bench_function("to_num_i64", |bn| bn.iter(|| black_box(A).to_num::<i64>()));
    g.bench_function("to_num_f64", |bn| bn.iter(|| black_box(A).to_num::<f64>()));

    g.finish();
}

criterion_group!(
    benches,
    bench_core_type,
    bench_arithmetic,
    bench_conversions,
    bench_consts,
    bench_powers,
    bench_log_exp,
    bench_trig,
    bench_bitwise,
    bench_overflow_variants,
    bench_display,
    bench_compat,
);
criterion_main!(benches);
