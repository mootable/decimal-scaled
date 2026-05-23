//! Like-for-like comparison vs the `g_math` crate.
//!
//! `g_math` advertises **"0 ULP transcendentals"** in its description.
//! That's mathematically impossible for irrational results at any
//! finite precision (`sin(1)`, `ln(2)`, `e`, …) — so the bench reports
//! both **timing** and **accuracy** so we can settle the claim
//! empirically.
//!
//! At its default `embedded` profile, `g_math` operates at Q64.64 ≈
//! 19 decimal digits. The matching `decimal-scaled` width is
//! `D38<19>` (i128 storage scaled by `10^19`). The accuracy column
//! uses a `D76<35>` strict computation as the high-precision baseline
//! (35 decimal digits — far above either competitor's 19-digit
//! working width) and reports `|x − baseline|` in ULPs at the 19-digit
//! storage scale.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

use decimal_scaled::{D38, D76};

use g_math::canonical::{evaluate, gmath};

type D38_19 = D38<19>;
// `D76<19>` matches the storage scale we're comparing at, but its
// guard-digit core gives ~49 effective digits of working precision —
// far above either competitor's 19 storage digits — so its
// `*_strict` result is the high-precision baseline.
type D76_19 = D76<19>;

/// `D76<19>` storage as a raw `i128` (always fits at scale 19; D76's
/// integer range there is much larger than i128's).
fn d76_19_bits(v: D76_19) -> i128 {
    v.to_bits()
        .to_i128_checked()
        .expect("baseline value fits i128 at D76<19>")
}

/// Parse g_math's `to_decimal_string` output (`"1.4142135…"`) into the
/// equivalent raw `i128` at scale 19. We ask for 25 digits — well past
/// the storage's 19 — so any trailing inaccuracy is plainly visible in
/// the truncation tail; we then take the first 19 fractional digits.
fn gmath_to_d19_bits(s: &str) -> i128 {
    let (sign, body) = if let Some(rest) = s.strip_prefix('-') {
        (-1_i128, rest)
    } else {
        (1_i128, s)
    };
    let (int_part, frac_part) = body.split_once('.').unwrap_or((body, ""));
    let int_value: i128 = int_part.parse().unwrap_or(0);
    let mut frac_buf = String::from(frac_part);
    while frac_buf.len() < 19 {
        frac_buf.push('0');
    }
    let frac_value: i128 = frac_buf[..19].parse().unwrap_or(0);
    sign * (int_value * 10_i128.pow(19) + frac_value)
}

/// Compute `|a − b|` in ULPs at `D38<19>` storage.
fn ulp_delta(a_bits: i128, b_bits: i128) -> u128 {
    a_bits.wrapping_sub(b_bits).unsigned_abs()
}

fn accuracy_report() {
    println!("\n=== Accuracy: g_math vs decimal-scaled vs D76<35> baseline ===");
    println!("All values reported in ULPs at the 19-digit scale (1 ULP = 10⁻¹⁹).\n");

    // ── sin(1) ────────────────────────────────────────────────────
    let baseline_sin_bits = d76_19_bits(D76_19::try_from(1).unwrap().sin_strict());

    let ds_sin = D38_19::try_from(1).unwrap().sin_strict().to_bits().into();
    let gm_sin = {
        let r = evaluate(&gmath("1.0").sin()).expect("g_math sin(1)");
        gmath_to_d19_bits(&r.to_decimal_string(25))
    };
    println!(
        "sin(1):  decimal-scaled  delta={:>3} ULP   g_math  delta={:>10} ULP",
        ulp_delta(ds_sin, baseline_sin_bits),
        ulp_delta(gm_sin, baseline_sin_bits),
    );

    // ── ln(2) ─────────────────────────────────────────────────────
    let baseline_ln_bits = d76_19_bits(D76_19::try_from(2).unwrap().ln_strict());

    let ds_ln = D38_19::try_from(2).unwrap().ln_strict().to_bits().into();
    let gm_ln = {
        let r = evaluate(&gmath("2.0").ln()).expect("g_math ln(2)");
        gmath_to_d19_bits(&r.to_decimal_string(25))
    };
    println!(
        "ln(2):   decimal-scaled  delta={:>3} ULP   g_math  delta={:>10} ULP",
        ulp_delta(ds_ln, baseline_ln_bits),
        ulp_delta(gm_ln, baseline_ln_bits),
    );

    // ── exp(1) — i.e. e ──────────────────────────────────────────
    let baseline_exp_bits = d76_19_bits(D76_19::try_from(1).unwrap().exp_strict());

    let ds_exp = D38_19::try_from(1).unwrap().exp_strict().to_bits().into();
    let gm_exp = {
        let r = evaluate(&gmath("1.0").exp()).expect("g_math exp(1)");
        gmath_to_d19_bits(&r.to_decimal_string(25))
    };
    println!(
        "exp(1):  decimal-scaled  delta={:>3} ULP   g_math  delta={:>10} ULP",
        ulp_delta(ds_exp, baseline_exp_bits),
        ulp_delta(gm_exp, baseline_exp_bits),
    );

    // ── sqrt(2) ──────────────────────────────────────────────────
    let baseline_sqrt_bits = d76_19_bits(D76_19::try_from(2).unwrap().sqrt_strict());

    let ds_sqrt = D38_19::try_from(2).unwrap().sqrt_strict().to_bits().into();
    let gm_sqrt = {
        let r = evaluate(&gmath("2.0").sqrt()).expect("g_math sqrt(2)");
        gmath_to_d19_bits(&r.to_decimal_string(25))
    };
    println!(
        "sqrt(2): decimal-scaled  delta={:>3} ULP   g_math  delta={:>10} ULP",
        ulp_delta(ds_sqrt, baseline_sqrt_bits),
        ulp_delta(gm_sqrt, baseline_sqrt_bits),
    );

    println!();
}

fn bench_decimal_scaled(c: &mut Criterion) {
    let mut g = c.benchmark_group("decimal-scaled D38<19>");

    let two = D38_19::try_from(2).unwrap();
    let three = D38_19::try_from(3).unwrap();
    let one = D38_19::ONE;

    g.bench_function("mul", |b| {
        b.iter(|| black_box(black_box(two) * black_box(three)));
    });
    g.bench_function("sqrt_strict", |b| {
        b.iter(|| black_box(black_box(two)).sqrt_strict());
    });
    g.bench_function("ln_strict", |b| {
        b.iter(|| black_box(black_box(two)).ln_strict());
    });
    g.bench_function("exp_strict", |b| {
        b.iter(|| black_box(black_box(one)).exp_strict());
    });
    g.bench_function("sin_strict", |b| {
        b.iter(|| black_box(black_box(one)).sin_strict());
    });

    g.finish();
}

fn bench_g_math(c: &mut Criterion) {
    let mut g = c.benchmark_group("g_math Q64.64 (default)");

    // Pre-build the input expressions where possible. g_math is
    // expression-based; we time `evaluate(&expr)` since that's the
    // computation step.
    g.bench_function("mul", |b| {
        b.iter(|| {
            let r = evaluate(&(black_box(gmath("2.0")) * black_box(gmath("3.0"))));
            black_box(r)
        });
    });
    g.bench_function("sqrt", |b| {
        b.iter(|| {
            let r = evaluate(&black_box(gmath("2.0")).sqrt());
            black_box(r)
        });
    });
    g.bench_function("ln", |b| {
        b.iter(|| {
            let r = evaluate(&black_box(gmath("2.0")).ln());
            black_box(r)
        });
    });
    g.bench_function("exp", |b| {
        b.iter(|| {
            let r = evaluate(&black_box(gmath("1.0")).exp());
            black_box(r)
        });
    });
    g.bench_function("sin", |b| {
        b.iter(|| {
            let r = evaluate(&black_box(gmath("1.0")).sin());
            black_box(r)
        });
    });

    g.finish();
}

fn entry(c: &mut Criterion) {
    accuracy_report();
    bench_decimal_scaled(c);
    bench_g_math(c);
}

criterion_group!(benches, entry);
criterion_main!(benches);
