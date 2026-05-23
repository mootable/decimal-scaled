//! One-shot ULP-accuracy report for the library-comparison
//! transcendentals at the 128-bit (D38<19>) tier. Each library is
//! asked to compute ln(2) / exp(1) / sin(1) / sqrt(2); the answer
//! is rendered as a decimal string at ≥ 19 fractional digits and
//! compared to a D76<19> baseline rounded to 19 digits.
//!
//! Run with:
//!     cargo run --release --example ulp_report --features wide,x-wide
//!
//! Output is a markdown table written to stdout that we paste into
//! docs/benchmarks.md §5.

use decimal_scaled::{D38, D76};

use bigdecimal::BigDecimal;
use dashu_float::DBig;
use decimal_rs::Decimal as DecimalRs;
use fastnum::dec128;
use g_math::canonical::{evaluate, gmath};
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

type Ref19 = D76<19>;

fn ref19_to_bits(v: Ref19) -> i128 {
    v.to_bits()
        .to_i128_checked()
        .expect("D76<19> baseline fits i128 at scale 19")
}

fn parse_decimal_string_to_d19_bits(s: &str) -> i128 {
    let s = s.trim();
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

fn ulp<A: Into<i128>, B: Into<i128>>(a: A, b: B) -> u128 {
    // Accepts D38's `Int<2>` to_bits (via `From<Int<2>> for i128`) and plain
    // `i128` reference values alike.
    let (a, b): (i128, i128) = (a.into(), b.into());
    a.wrapping_sub(b).unsigned_abs()
}

fn main() {
    println!("# Transcendental ULP errors at D38<19> (1 ULP = 10⁻¹⁹)");
    println!();
    println!(
        "Baseline: `D76<19>` integer-only `*_strict` (≥ 49 effective working digits, rounded back to 19)."
    );
    println!();
    println!(
        "| op    | decimal-scaled | fastnum | rust_decimal | dashu-float | decimal-rs | bigdecimal | g_math |"
    );
    println!(
        "|-------|----------------|---------|--------------|-------------|------------|------------|--------|"
    );

    // ── ln(2) ──────────────────────────────────────────────
    let base_ln = ref19_to_bits(Ref19::try_from(2).unwrap().ln_strict());
    let ds_ln = D38::<19>::try_from(2).unwrap().ln_strict().to_bits();
    let fn_ln = parse_decimal_string_to_d19_bits(&format!("{}", dec128!(2).ln()));
    let rd_ln = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        RustDecimal::from_i128_with_scale(20_000_000_000_000_000_000_i128, 19).ln()
    ));
    let db_ln = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        DBig::from_parts(2.into(), 0)
            .with_precision(19)
            .value()
            .ln()
    ));
    let dr_ln = parse_decimal_string_to_d19_bits(&format!(
        "{}",
        DecimalRs::from(2).ln().expect("decimal-rs ln(2)")
    ));
    let bd_ln_str = format!("{:.19}", "(no ln in bigdecimal 0.4)");
    let _ = bd_ln_str;
    let gm_ln = {
        let r = evaluate(&gmath("2.0").ln()).expect("g_math ln(2)");
        parse_decimal_string_to_d19_bits(&r.to_decimal_string(25))
    };

    println!(
        "| ln(2) | **{}** | {} | {} | {} | {} | — | {} |",
        ulp(ds_ln, base_ln),
        ulp(fn_ln, base_ln),
        ulp(rd_ln, base_ln),
        ulp(db_ln, base_ln),
        ulp(dr_ln, base_ln),
        ulp(gm_ln, base_ln),
    );

    // ── exp(1) — i.e. e ─────────────────────────────────────
    let base_exp = ref19_to_bits(Ref19::try_from(1).unwrap().exp_strict());
    let ds_exp = D38::<19>::try_from(1).unwrap().exp_strict().to_bits();
    let fn_exp = parse_decimal_string_to_d19_bits(&format!("{}", dec128!(1).exp()));
    let rd_exp = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        RustDecimal::from_i128_with_scale(10_000_000_000_000_000_000_i128, 19).exp()
    ));
    let db_exp = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        DBig::from_parts(1.into(), 0)
            .with_precision(19)
            .value()
            .exp()
    ));
    let dr_exp = parse_decimal_string_to_d19_bits(&format!(
        "{}",
        DecimalRs::from(1).exp().expect("decimal-rs exp(1)")
    ));
    let gm_exp = {
        let r = evaluate(&gmath("1.0").exp()).expect("g_math exp(1)");
        parse_decimal_string_to_d19_bits(&r.to_decimal_string(25))
    };

    println!(
        "| exp(1) | **{}** | {} | {} | {} | {} | — | {} |",
        ulp(ds_exp, base_exp),
        ulp(fn_exp, base_exp),
        ulp(rd_exp, base_exp),
        ulp(db_exp, base_exp),
        ulp(dr_exp, base_exp),
        ulp(gm_exp, base_exp),
    );

    // ── sin(1) ─────────────────────────────────────────────
    let base_sin = ref19_to_bits(Ref19::try_from(1).unwrap().sin_strict());
    let ds_sin = D38::<19>::try_from(1).unwrap().sin_strict().to_bits();
    let fn_sin = parse_decimal_string_to_d19_bits(&format!("{}", dec128!(1).sin()));
    let rd_sin = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        RustDecimal::from_i128_with_scale(10_000_000_000_000_000_000_i128, 19).sin()
    ));
    let gm_sin = {
        let r = evaluate(&gmath("1.0").sin()).expect("g_math sin(1)");
        parse_decimal_string_to_d19_bits(&r.to_decimal_string(25))
    };

    println!(
        "| sin(1) | **{}** | {} | {} | — | — | — | {} |",
        ulp(ds_sin, base_sin),
        ulp(fn_sin, base_sin),
        ulp(rd_sin, base_sin),
        ulp(gm_sin, base_sin),
    );

    // ── sqrt(2) ────────────────────────────────────────────
    let base_sqrt = ref19_to_bits(Ref19::try_from(2).unwrap().sqrt_strict());
    let ds_sqrt = D38::<19>::try_from(2).unwrap().sqrt_strict().to_bits();
    let fn_sqrt = parse_decimal_string_to_d19_bits(&format!("{}", dec128!(2).sqrt()));
    let rd_sqrt = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        RustDecimal::from_i128_with_scale(20_000_000_000_000_000_000_i128, 19)
            .sqrt()
            .expect("rust_decimal sqrt(2)")
    ));
    let dr_sqrt = parse_decimal_string_to_d19_bits(&format!(
        "{}",
        DecimalRs::from(2).sqrt().expect("decimal-rs sqrt(2)")
    ));
    let bd_sqrt = parse_decimal_string_to_d19_bits(&format!(
        "{:.19}",
        BigDecimal::from(2).sqrt().expect("bigdecimal sqrt(2)")
    ));
    let gm_sqrt = {
        let r = evaluate(&gmath("2.0").sqrt()).expect("g_math sqrt(2)");
        parse_decimal_string_to_d19_bits(&r.to_decimal_string(25))
    };

    println!(
        "| sqrt(2) | **{}** | {} | {} | — | {} | {} | {} |",
        ulp(ds_sqrt, base_sqrt),
        ulp(fn_sqrt, base_sqrt),
        ulp(rd_sqrt, base_sqrt),
        ulp(dr_sqrt, base_sqrt),
        ulp(bd_sqrt, base_sqrt),
        ulp(gm_sqrt, base_sqrt),
    );
}
