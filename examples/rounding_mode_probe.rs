//! Diagnostic: are the "1 ULP off" peers in §5's accuracy table
//! actually 1 ULP off, or just using a different rounding mode?
//!
//! For each test value, compute it to high precision via D76 strict
//! and render at SCALE=19 under HalfToEven (our default),
//! HalfAwayFromZero, HalfTowardZero, and truncate. Then print each
//! external library's published output, so we can see which
//! rounding-mode candidate the peer matches.

use decimal_scaled::{D38, D76, RoundingMode};
use fastnum::dec128;
use rust_decimal::Decimal as RustDecimal;
use rust_decimal::MathematicalOps;

type Hi = D76<25>;

fn print_candidates(name: &str, hi: Hi) {
    println!("\n## {} at SCALE=19", name);
    // Render at scale 25 then explicitly rescale to 19 under each mode.
    for mode in [
        RoundingMode::HalfToEven,
        RoundingMode::HalfAwayFromZero,
        RoundingMode::HalfTowardZero,
        RoundingMode::Trunc,
        RoundingMode::Floor,
        RoundingMode::Ceiling,
    ] {
        let down: D76<19> = hi.rescale_with::<19>(mode);
        println!("  {:?} -> {}", mode, down);
    }
}

fn main() {
    println!("# Rounding-mode probe at SCALE=19");
    println!();
    println!("Each candidate shows the SCALE=19 rendering of the");
    println!("D76<25> strict reference under the named rounding mode.");
    println!("Compare against each library's actual output below to");
    println!("identify which mode they used.");

    // e = 2.71828...
    let e_hi = Hi::from(1).exp_strict();
    print_candidates("exp(1) = e", e_hi);

    // sin(1)
    let sin1_hi = Hi::from(1).sin_strict();
    print_candidates("sin(1)", sin1_hi);

    // ln(2)
    let ln2_hi = Hi::from(2).ln_strict();
    print_candidates("ln(2)", ln2_hi);

    // sqrt(2)
    let sqrt2_hi = Hi::from(2).sqrt_strict();
    print_candidates("sqrt(2)", sqrt2_hi);

    println!("\n# External library outputs at SCALE=19");

    println!("\n## fastnum @ D128 (38 internal digits)");
    println!("  exp(1)   = {}", dec128!(1).exp());
    println!("  sin(1)   = {}", dec128!(1).sin());
    println!("  ln(2)    = {}", dec128!(2).ln());
    println!("  sqrt(2)  = {}", dec128!(2).sqrt());

    println!("\n## fastnum @ D256 (~75 internal digits)");
    println!("  exp(1)   = {}", fastnum::dec256!(1).exp());
    println!("  sin(1)   = {}", fastnum::dec256!(1).sin());
    println!("  ln(2)    = {}", fastnum::dec256!(2).ln());
    println!("  sqrt(2)  = {}", fastnum::dec256!(2).sqrt());

    println!("\n## fastnum @ D512 (~155 internal digits)");
    println!("  exp(1)   = {}", fastnum::dec512!(1).exp());
    println!("  sin(1)   = {}", fastnum::dec512!(1).sin());
    println!("  ln(2)    = {}", fastnum::dec512!(2).ln());
    println!("  sqrt(2)  = {}", fastnum::dec512!(2).sqrt());

    println!("\n## rust_decimal (s=19)");
    let two = RustDecimal::from_i128_with_scale(20_000_000_000_000_000_000_i128, 19);
    let one = RustDecimal::from_i128_with_scale(10_000_000_000_000_000_000_i128, 19);
    println!("  exp(1)   = {:.19}", one.exp());
    println!("  sin(1)   = {:.19}", one.sin());
    println!("  ln(2)    = {:.19}", two.ln());
    println!("  sqrt(2)  = {:.19}", two.sqrt().unwrap());

    println!("\n## decimal-scaled D38<19> strict (HalfToEven default)");
    println!("  exp(1)   = {}", D38::<19>::from(1).exp_strict());
    println!("  sin(1)   = {}", D38::<19>::from(1).sin_strict());
    println!("  ln(2)    = {}", D38::<19>::from(2).ln_strict());
    println!("  sqrt(2)  = {}", D38::<19>::from(2).sqrt_strict());
}
