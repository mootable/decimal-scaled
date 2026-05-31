//! samply probe: decimal `div` across ALL widths × the 5-point scale set,
//! driven by GOLDEN operand pairs (the adversarial edge cases), branch vs prod.
//!
//! Two cost components are visible by sweeping scale: at scale 0 a decimal
//! `a / b` is purely the INT divide (no rescale); at higher scales the
//! `÷10^w` rescale (a second divide that grows with scale) dominates. Sweeping
//! WIDTH shows the int-divide's width-scaling. The operands are the
//! `tests/golden/div_d<N>_s<S>.txt` pairs (full-width, near-tie, carry, max
//! magnitude), so the profile sees the value-dependent edge cases.
//!
//! Run ONE `<width>_s<scale>_<branch|prod>` per invocation so symbolized frames
//! don't collide (branch + prod share the internal crate name `decimal_scaled`):
//!   div_probe d57_s28_branch        # D57 at scale 28, this crate
//!   div_probe d462_s461_prod        # D462 at scale 461, 0.4.4
//! Widths × 5-point scales below; `_branch` = this crate, `_prod` = 0.4.4.

use std::hint::black_box;

/// Reconstruct a decimal-value string from a golden RAW storage integer at
/// `scale`: the raw is `value · 10^scale`, so insert a decimal point `scale`
/// digits from the right (the public `from_str` then yields a `D<…, scale>`
/// whose stored raw equals the golden raw). `from_str` on the raw alone would
/// be wrong for `scale > 0` (it would re-scale by another `10^scale`).
fn raw_to_value_str(raw: &str, scale: u32) -> String {
    let (sign, digits) = match raw.strip_prefix('-') {
        Some(d) => ("-", d),
        None => ("", raw),
    };
    if scale == 0 {
        return format!("{sign}{digits}");
    }
    let s = scale as usize;
    if digits.len() > s {
        let (int, frac) = digits.split_at(digits.len() - s);
        format!("{sign}{int}.{frac}")
    } else {
        let zeros = "0".repeat(s - digits.len());
        format!("{sign}0.{zeros}{digits}")
    }
}

/// Load `(a, b)` operand pairs from a 4-column golden table
/// (`a_raw \t b_raw \t floor \t cls`), reconstructing each operand at `scale`.
/// Skips comments and `b == 0`; caps the set so the profile stays focused.
fn load_ops<T: std::str::FromStr>(path: &str, scale: u32) -> Vec<(T, T)>
where
    T::Err: std::fmt::Debug,
{
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let mut out = Vec::new();
    // The bbc-benched div operand (`2 / 3`) FIRST: the small-quotient SETUP path
    // the bbc samples (quotient 0 at scale 0; a real rescaled fraction at higher
    // scales). The golden corpus skews to larger quotients, so without this the
    // profile would miss the very cell the bbc flags. `2 / 3` at this cell's scale
    // = `from_str("2") / from_str("3")`.
    out.push((
        "2".parse::<T>().expect("parse bbc a"),
        "3".parse::<T>().expect("parse bbc b"),
    ));
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = l.split('\t').collect();
        if cols.len() < 2 || cols[1] == "0" {
            continue;
        }
        let a = raw_to_value_str(cols[0], scale).parse::<T>().expect("parse a");
        let b = raw_to_value_str(cols[1], scale).parse::<T>().expect("parse b");
        out.push((a, b));
        if out.len() >= 24 {
            break;
        }
    }
    assert!(!out.is_empty(), "no operands loaded from {path}");
    out
}

/// Tight `a / b` loop over the golden operand set, for samply.
fn run<T>(path: &str, scale: u32, iters: u64)
where
    T: Copy + std::ops::Div<Output = T> + std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let ops = load_ops::<T>(path, scale);
    let t0 = std::time::Instant::now();
    let mut acc = ops[0].0;
    for _ in 0..iters {
        for &(a, b) in &ops {
            acc = black_box(black_box(a) / black_box(b));
        }
    }
    black_box(acc);
    let el = t0.elapsed();
    let total = (ops.len() as u64) * iters;
    eprintln!(
        "ops={} iters={} total_div={} loop={:.3}s ({:.1} ns/div)",
        ops.len(),
        iters,
        total,
        el.as_secs_f64(),
        el.as_nanos() as f64 / (total as f64),
    );
}

macro_rules! golden {
    ($name:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/golden/", $name)
    };
}

/// One `(width, scale)` cell → a branch arm and a prod arm.
macro_rules! cell {
    ($sel:expr, $tag:literal, $alias:ident, $scale:literal, $file:literal, $iters:expr) => {
        if $sel == concat!($tag, "_branch") {
            return run::<decimal_scaled::$alias<$scale>>(golden!($file), $scale, $iters);
        }
        if $sel == concat!($tag, "_prod") {
            return run::<prod::$alias<$scale>>(golden!($file), $scale, $iters);
        }
    };
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();

    // D18 (S=17)
    cell!(sel, "d18_s0", D18, 0, "div_d18_s0.txt", 2_000_000);
    cell!(sel, "d18_s4", D18, 4, "div_d18_s4.txt", 2_000_000);
    cell!(sel, "d18_s9", D18, 9, "div_d18_s9.txt", 2_000_000);
    cell!(sel, "d18_s13", D18, 13, "div_d18_s13.txt", 2_000_000);
    cell!(sel, "d18_s17", D18, 17, "div_d18_s17.txt", 2_000_000);
    // D38 (S=38)
    cell!(sel, "d38_s0", D38, 0, "div_d38_s0.txt", 1_500_000);
    cell!(sel, "d38_s9", D38, 9, "div_d38_s9.txt", 1_500_000);
    cell!(sel, "d38_s19", D38, 19, "div_d38_s19.txt", 1_500_000);
    cell!(sel, "d38_s28", D38, 28, "div_d38_s28.txt", 1_500_000);
    cell!(sel, "d38_s37", D38, 37, "div_d38_s37.txt", 1_500_000);
    // D57 (S=56)
    cell!(sel, "d57_s0", D57, 0, "div_d57_s0.txt", 1_200_000);
    cell!(sel, "d57_s14", D57, 14, "div_d57_s14.txt", 1_200_000);
    cell!(sel, "d57_s28", D57, 28, "div_d57_s28.txt", 1_000_000);
    cell!(sel, "d57_s42", D57, 42, "div_d57_s42.txt", 900_000);
    cell!(sel, "d57_s56", D57, 56, "div_d57_s56.txt", 800_000);
    // D76 (S=75)
    cell!(sel, "d76_s0", D76, 0, "div_d76_s0.txt", 1_000_000);
    cell!(sel, "d76_s19", D76, 19, "div_d76_s19.txt", 900_000);
    cell!(sel, "d76_s38", D76, 38, "div_d76_s38.txt", 800_000);
    cell!(sel, "d76_s57", D76, 57, "div_d76_s57.txt", 700_000);
    cell!(sel, "d76_s75", D76, 75, "div_d76_s75.txt", 600_000);
    // D115 (S=114)
    cell!(sel, "d115_s0", D115, 0, "div_d115_s0.txt", 700_000);
    cell!(sel, "d115_s28", D115, 28, "div_d115_s28.txt", 600_000);
    cell!(sel, "d115_s57", D115, 57, "div_d115_s57.txt", 500_000);
    cell!(sel, "d115_s86", D115, 86, "div_d115_s86.txt", 450_000);
    cell!(sel, "d115_s114", D115, 114, "div_d115_s114.txt", 400_000);
    // D153 (S=152)
    cell!(sel, "d153_s0", D153, 0, "div_d153_s0.txt", 500_000);
    cell!(sel, "d153_s38", D153, 38, "div_d153_s38.txt", 400_000);
    cell!(sel, "d153_s76", D153, 76, "div_d153_s76.txt", 350_000);
    cell!(sel, "d153_s114", D153, 114, "div_d153_s114.txt", 300_000);
    cell!(sel, "d153_s152", D153, 152, "div_d153_s152.txt", 250_000);
    // D230 (S=229)
    cell!(sel, "d230_s0", D230, 0, "div_d230_s0.txt", 300_000);
    cell!(sel, "d230_s57", D230, 57, "div_d230_s57.txt", 250_000);
    cell!(sel, "d230_s115", D230, 115, "div_d230_s115.txt", 200_000);
    cell!(sel, "d230_s172", D230, 172, "div_d230_s172.txt", 170_000);
    cell!(sel, "d230_s229", D230, 229, "div_d230_s229.txt", 150_000);
    // D307 (S=306)
    cell!(sel, "d307_s0", D307, 0, "div_d307_s0.txt", 200_000);
    cell!(sel, "d307_s76", D307, 76, "div_d307_s76.txt", 150_000);
    cell!(sel, "d307_s153", D307, 153, "div_d307_s153.txt", 120_000);
    cell!(sel, "d307_s230", D307, 230, "div_d307_s230.txt", 100_000);
    cell!(sel, "d307_s306", D307, 306, "div_d307_s306.txt", 80_000);
    // D462 (S=461)
    cell!(sel, "d462_s0", D462, 0, "div_d462_s0.txt", 100_000);
    cell!(sel, "d462_s115", D462, 115, "div_d462_s115.txt", 80_000);
    cell!(sel, "d462_s231", D462, 231, "div_d462_s231.txt", 60_000);
    cell!(sel, "d462_s346", D462, 346, "div_d462_s346.txt", 50_000);
    cell!(sel, "d462_s461", D462, 461, "div_d462_s461.txt", 40_000);
    // D616 (S=615)
    cell!(sel, "d616_s0", D616, 0, "div_d616_s0.txt", 60_000);
    cell!(sel, "d616_s154", D616, 154, "div_d616_s154.txt", 45_000);
    cell!(sel, "d616_s308", D616, 308, "div_d616_s308.txt", 35_000);
    cell!(sel, "d616_s462", D616, 462, "div_d616_s462.txt", 28_000);
    cell!(sel, "d616_s615", D616, 615, "div_d616_s615.txt", 22_000);
    // D924 (S=923)
    cell!(sel, "d924_s0", D924, 0, "div_d924_s0.txt", 30_000);
    cell!(sel, "d924_s231", D924, 231, "div_d924_s231.txt", 20_000);
    cell!(sel, "d924_s462", D924, 462, "div_d924_s462.txt", 14_000);
    cell!(sel, "d924_s693", D924, 693, "div_d924_s693.txt", 10_000);
    cell!(sel, "d924_s923", D924, 923, "div_d924_s923.txt", 8_000);
    // D1232 (S=1231)
    cell!(sel, "d1232_s0", D1232, 0, "div_d1232_s0.txt", 16_000);
    cell!(sel, "d1232_s308", D1232, 308, "div_d1232_s308.txt", 9_000);
    cell!(sel, "d1232_s616", D1232, 616, "div_d1232_s616.txt", 6_000);
    cell!(sel, "d1232_s924", D1232, 924, "div_d1232_s924.txt", 4_500);
    cell!(sel, "d1232_s1231", D1232, 1231, "div_d1232_s1231.txt", 3_500);

    eprintln!("usage: div_probe <width>_s<scale>_<branch|prod> (got {sel:?})");
    std::process::exit(2);
}
