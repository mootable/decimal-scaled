//! samply probe: decimal `div` across ALL widths × the 5-point scale set,
//! driven by GOLDEN operand pairs (the adversarial edge cases), branch vs prod.
//!
//! Two cost components are visible by sweeping scale: at scale 0 a decimal
//! `a / b` is purely the INT divide (no rescale); at higher scales the
//! `÷10^w` rescale (a second divide that grows with scale) dominates. Sweeping
//! WIDTH shows the int-divide's width-scaling. The operands are the
//! `decimal-scaled-golden/golden/div.au` pairs (full-width, near-tie, carry, max
//! magnitude), so the profile sees the value-dependent edge cases.
//!
//! Run ONE `<width>_s<scale>_<branch|prod>` per invocation so symbolized frames
//! don't collide (branch + prod share the internal crate name `decimal_scaled`):
//!   div_probe d57_s28_branch        # D57 at scale 28, this crate
//!   div_probe d462_s461_prod        # D462 at scale 461, 0.4.4
//! Widths × 5-point scales below; `_branch` = this crate, `_prod` = 0.4.4.

use std::hint::black_box;

/// Load `(a, b)` operand pairs from the golden set (`a b value…` rows with
/// `#` header and `//` provenance lines; operands are width-agnostic decimal
/// literals, so every cell parses them directly). Skips zero divisors and
/// operands that don't fit this cell's storage; caps the set so the profile
/// stays focused.
fn load_ops<T: std::str::FromStr>(path: &str) -> Vec<(T, T)>
where
    T::Err: std::fmt::Debug,
{
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let mut out = Vec::new();
    // The bbc-benched div operand (`2 / 3`) FIRST: the small-quotient SETUP path
    // the bbc samples (quotient 0 at scale 0; a real rescaled fraction at higher
    // scales). The golden set skews to larger quotients, so without this the
    // profile would miss the very cell the bbc flags. `2 / 3` at this cell's scale
    // = `from_str("2") / from_str("3")`.
    out.push((
        "2".parse::<T>().expect("parse bbc a"),
        "3".parse::<T>().expect("parse bbc b"),
    ));
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with("//") {
            continue;
        }
        let mut cols = l.split_whitespace();
        let (Some(a), Some(b)) = (cols.next(), cols.next()) else { continue };
        if b.trim_start_matches('-').chars().all(|c| c == '0' || c == '.') {
            continue;
        }
        let (Ok(a), Ok(b)) = (a.parse::<T>(), b.parse::<T>()) else { continue };
        out.push((a, b));
        if out.len() >= 24 {
            break;
        }
    }
    assert!(!out.is_empty(), "no operands loaded from {path}");
    out
}

/// Tight `a / b` loop over the golden operand set, for samply.
fn run<T>(path: &str, iters: u64)
where
    T: Copy + std::ops::Div<Output = T> + std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let ops = load_ops::<T>(path);
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

/// The one golden div table — operands are width-agnostic decimal literals,
/// so every `(width, scale)` cell reads the same file and parses at its own
/// type (which carries the scale).
const GOLDEN_DIV: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../decimal-scaled-golden/golden/div.au");

/// One `(width, scale)` cell → a branch arm and a prod arm.
macro_rules! cell {
    ($sel:expr, $tag:literal, $alias:ident, $scale:literal, $iters:expr) => {
        if $sel == concat!($tag, "_branch") {
            return run::<decimal_scaled::$alias<$scale>>(GOLDEN_DIV, $iters);
        }
        if $sel == concat!($tag, "_prod") {
            return run::<prod::$alias<$scale>>(GOLDEN_DIV, $iters);
        }
    };
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();

    // D18 (S=17)
    cell!(sel, "d18_s0", D18, 0, 2_000_000);
    cell!(sel, "d18_s4", D18, 4, 2_000_000);
    cell!(sel, "d18_s9", D18, 9, 2_000_000);
    cell!(sel, "d18_s13", D18, 13, 2_000_000);
    cell!(sel, "d18_s17", D18, 17, 2_000_000);
    // D38 (S=38)
    cell!(sel, "d38_s0", D38, 0, 1_500_000);
    cell!(sel, "d38_s9", D38, 9, 1_500_000);
    cell!(sel, "d38_s19", D38, 19, 1_500_000);
    cell!(sel, "d38_s28", D38, 28, 1_500_000);
    cell!(sel, "d38_s37", D38, 37, 1_500_000);
    // D57 (S=56)
    cell!(sel, "d57_s0", D57, 0, 1_200_000);
    cell!(sel, "d57_s14", D57, 14, 1_200_000);
    cell!(sel, "d57_s28", D57, 28, 1_000_000);
    cell!(sel, "d57_s42", D57, 42, 900_000);
    cell!(sel, "d57_s56", D57, 56, 800_000);
    // D76 (S=75)
    cell!(sel, "d76_s0", D76, 0, 1_000_000);
    cell!(sel, "d76_s19", D76, 19, 900_000);
    cell!(sel, "d76_s38", D76, 38, 800_000);
    cell!(sel, "d76_s57", D76, 57, 700_000);
    cell!(sel, "d76_s75", D76, 75, 600_000);
    // D115 (S=114)
    cell!(sel, "d115_s0", D115, 0, 700_000);
    cell!(sel, "d115_s28", D115, 28, 600_000);
    cell!(sel, "d115_s57", D115, 57, 500_000);
    cell!(sel, "d115_s86", D115, 86, 450_000);
    cell!(sel, "d115_s114", D115, 114, 400_000);
    // D153 (S=152)
    cell!(sel, "d153_s0", D153, 0, 500_000);
    cell!(sel, "d153_s38", D153, 38, 400_000);
    cell!(sel, "d153_s76", D153, 76, 350_000);
    cell!(sel, "d153_s114", D153, 114, 300_000);
    cell!(sel, "d153_s152", D153, 152, 250_000);
    // D230 (S=229)
    cell!(sel, "d230_s0", D230, 0, 300_000);
    cell!(sel, "d230_s57", D230, 57, 250_000);
    cell!(sel, "d230_s115", D230, 115, 200_000);
    cell!(sel, "d230_s172", D230, 172, 170_000);
    cell!(sel, "d230_s229", D230, 229, 150_000);
    // D307 (S=306)
    cell!(sel, "d307_s0", D307, 0, 200_000);
    cell!(sel, "d307_s76", D307, 76, 150_000);
    cell!(sel, "d307_s153", D307, 153, 120_000);
    cell!(sel, "d307_s230", D307, 230, 100_000);
    cell!(sel, "d307_s306", D307, 306, 80_000);
    // D462 (S=461)
    cell!(sel, "d462_s0", D462, 0, 100_000);
    cell!(sel, "d462_s115", D462, 115, 80_000);
    cell!(sel, "d462_s231", D462, 231, 60_000);
    cell!(sel, "d462_s346", D462, 346, 50_000);
    cell!(sel, "d462_s461", D462, 461, 40_000);
    // D616 (S=615)
    cell!(sel, "d616_s0", D616, 0, 60_000);
    cell!(sel, "d616_s154", D616, 154, 45_000);
    cell!(sel, "d616_s308", D616, 308, 35_000);
    cell!(sel, "d616_s462", D616, 462, 28_000);
    cell!(sel, "d616_s615", D616, 615, 22_000);
    // D924 (S=923)
    cell!(sel, "d924_s0", D924, 0, 30_000);
    cell!(sel, "d924_s231", D924, 231, 20_000);
    cell!(sel, "d924_s462", D924, 462, 14_000);
    cell!(sel, "d924_s693", D924, 693, 10_000);
    cell!(sel, "d924_s923", D924, 923, 8_000);
    // D1232 (S=1231)
    cell!(sel, "d1232_s0", D1232, 0, 16_000);
    cell!(sel, "d1232_s308", D1232, 308, 9_000);
    cell!(sel, "d1232_s616", D1232, 616, 6_000);
    cell!(sel, "d1232_s924", D1232, 924, 4_500);
    cell!(sel, "d1232_s1231", D1232, 1231, 3_500);

    eprintln!("usage: div_probe <width>_s<scale>_<branch|prod> (got {sel:?})");
    std::process::exit(2);
}
