//! Per-tier ULP measurement of the `*_fast` (f64-bridge) variants
//! against the integer-only `*_strict` reference. Reported as
//! decimal digits of the result that disagree at storage scale.
//!
//! Run with:
//! ```
//! cargo run --release --example fast_vs_strict_ulp --features "wide x-wide xx-wide fast"
//! ```

use decimal_scaled::{D18, D38};
#[cfg(feature = "wide")]
use decimal_scaled::{D57, D76, D115, D153, D230, D307};
#[cfg(feature = "x-wide")]
use decimal_scaled::{D462, D616};
#[cfg(feature = "xx-wide")]
use decimal_scaled::{D924, D1232};

/// Number of trailing decimal digits in the strict result that the
/// fast f64-bridge result fails to reproduce. Computed as
/// `floor(log10(|strict_bits − fast_bits|)) + 1`, capped at the
/// total digit length of the strict result. A return of 0 means
/// bit-identical to the strict reference; N means the last N
/// decimal places of the result are noise.
fn digits_disagree<T: ToString>(strict_bits: T, fast_bits: T) -> usize {
    let s = strict_bits.to_string();
    let f = fast_bits.to_string();
    // Strip leading minus signs and parse to a u128/BigInt — but we
    // don't have BigInt available without a dep. Compare lexically
    // by length-then-digit, computing |s - f| as another digit
    // string via subtract_abs.
    let a = strip_sign(&s);
    let b = strip_sign(&f);
    let diff_digits = subtract_abs(a, b);
    if diff_digits == "0" {
        return 0;
    }
    diff_digits.len().min(a.len().max(b.len()))
}

fn strip_sign(s: &str) -> &str {
    s.trim_start_matches('-')
}

/// Decimal-string absolute difference: returns the digit string of
/// `|a − b|` without leading zeros. Both inputs are digit strings
/// with no sign and no leading zeros (other than "0" itself).
fn subtract_abs(a: &str, b: &str) -> String {
    use std::cmp::Ordering;
    let (big, small) = match a.len().cmp(&b.len()).then_with(|| a.cmp(b)) {
        Ordering::Less => (b, a),
        _ => (a, b),
    };
    let big: Vec<u8> = big.bytes().rev().map(|c| c - b'0').collect();
    let small: Vec<u8> = small.bytes().rev().map(|c| c - b'0').collect();
    let mut out = Vec::with_capacity(big.len());
    let mut borrow: i16 = 0;
    for i in 0..big.len() {
        let s = small.get(i).copied().unwrap_or(0) as i16;
        let mut d = big[i] as i16 - s - borrow;
        if d < 0 {
            d += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(d as u8 + b'0');
    }
    while out.len() > 1 && *out.last().unwrap() == b'0' {
        out.pop();
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

macro_rules! row {
    ($T:ty, $scale_label:literal, $arg_setup:expr) => {{
        let x: $T = $arg_setup;
        // ln / sin / sqrt all take 1.5; exp uses x - 1 = 0.5 to
        // keep e^x in the storage range of every tier (D9<9>'s
        // max is ~2.1, so e^1 ≈ 2.71 already overflows).
        let half = x - <$T>::from_int(1);
        let s_ln = x.ln_strict().to_bits();
        let f_ln = x.ln_fast().to_bits();
        let s_exp = half.exp_strict().to_bits();
        let f_exp = half.exp_fast().to_bits();
        let s_sin = x.sin_strict().to_bits();
        let f_sin = x.sin_fast().to_bits();
        let s_sqrt = x.sqrt_strict().to_bits();
        let f_sqrt = x.sqrt_fast().to_bits();
        println!(
            "| {:10} | {:>8} | {:>9} | {:>9} | {:>10} |",
            $scale_label,
            digits_disagree(s_ln, f_ln),
            digits_disagree(s_exp, f_exp),
            digits_disagree(s_sin, f_sin),
            digits_disagree(s_sqrt, f_sqrt),
        );
    }};
}

fn main() {
    println!("Fast `*` (f64 bridge) vs strict `*_strict` — trailing");
    println!("decimal digits of the result that disagree at storage");
    println!("scale. Each row uses argument 1.5 for ln/sin/sqrt and 0.5");
    println!("for exp.");
    println!();
    println!("| type / s   | ln noise | exp noise | sin noise | sqrt noise |");
    println!("|------------|----------|-----------|-----------|------------|");
    row!(
        D18<9>,
        "D18<9>",
        D18::<9>::from_int(1) + D18::<9>::from_int(1) / D18::<9>::from_int(2)
    );
    row!(
        D18<18>,
        "D18<18>",
        D18::<18>::from_int(1) + D18::<18>::from_int(1) / D18::<18>::from_int(2)
    );
    row!(
        D38<19>,
        "D38<19>",
        D38::<19>::from_int(1) + D38::<19>::from_int(1) / D38::<19>::from_int(2)
    );
    row!(
        D38<38>,
        "D38<38>",
        D38::<38>::from_bits(decimal_scaled::Int::<2>::try_from(15_000_000_000_000_000_000_000_000_000_000_000_000_i128).unwrap())
    );
    #[cfg(feature = "wide")]
    {
        row!(
            D57<28>,
            "D57<28>",
            D57::<28>::from_int(1) + D57::<28>::from_int(1) / D57::<28>::from_int(2)
        );
        row!(
            D76<35>,
            "D76<35>",
            D76::<35>::from_int(1) + D76::<35>::from_int(1) / D76::<35>::from_int(2)
        );
        row!(
            D115<57>,
            "D115<57>",
            D115::<57>::from_int(1) + D115::<57>::from_int(1) / D115::<57>::from_int(2)
        );
        row!(
            D153<75>,
            "D153<75>",
            D153::<75>::from_int(1) + D153::<75>::from_int(1) / D153::<75>::from_int(2)
        );
        row!(
            D230<115>,
            "D230<115>",
            D230::<115>::from_int(1) + D230::<115>::from_int(1) / D230::<115>::from_int(2)
        );
        row!(
            D307<150>,
            "D307<150>",
            D307::<150>::from_int(1) + D307::<150>::from_int(1) / D307::<150>::from_int(2)
        );
    }
    #[cfg(feature = "x-wide")]
    {
        row!(
            D462<230>,
            "D462<230>",
            D462::<230>::from_int(1) + D462::<230>::from_int(1) / D462::<230>::from_int(2)
        );
        row!(
            D616<308>,
            "D616<308>",
            D616::<308>::from_int(1) + D616::<308>::from_int(1) / D616::<308>::from_int(2)
        );
    }
    #[cfg(feature = "xx-wide")]
    {
        row!(
            D924<461>,
            "D924<461>",
            D924::<461>::from_int(1) + D924::<461>::from_int(1) / D924::<461>::from_int(2)
        );
        row!(
            D1232<616>,
            "D1232<616>",
            D1232::<616>::from_int(1) + D1232::<616>::from_int(1) / D1232::<616>::from_int(2)
        );
    }
}
