// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Focused samply probe for the decimal `ln` ALGORITHM (the `wide_transcendental`
//! / `ln_tang` / `ln_series` cores) across ALL widths × the 5 scale points,
//! driven by the bbc operand (`ln(2.0)`) + the adversarial golden inputs. One
//! `<width>_s<scale>` per run so samply attributes time cleanly to one cell.
//! Branch-only (the algorithm's OWN hot frames — the branch-vs-prod ratios come
//! from the bbc, not here). Profiles the post-rescale ln (the §9.20 baked-Newton
//! `÷10^w` is in this build), so the frames show ln's residual cost.
//!
//! Build: cargo build --release --example ln_probe --features "wide x-wide xx-wide"
//! Run:   ln_probe d1232_s616      # the worst ln regression cell

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Reconstruct a decimal-value string from a golden RAW storage integer at
/// `scale` (raw = value·10^scale → insert a point `scale` digits from the
/// right). Mirrors `bench-compare/examples/div_probe.rs`.
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
        format!("{sign}0.{}{}", "0".repeat(s - digits.len()), digits)
    }
}

/// Load `ln` input operands at `scale`: the bbc operand `2.0` FIRST (the cell
/// the bbc samples), then the golden table's input column (col 0) — the
/// adversarial near-1 / tie / max-magnitude inputs. Skips comments, `0`
/// (ln domain) and any negative input. Caps the set so the profile stays focused.
fn load_ops<T: std::str::FromStr>(path: &str, scale: u32) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut out = vec!["2.0".parse::<T>().expect("parse bbc 2.0")];
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let c0 = l.split('\t').next().unwrap_or("");
            if c0.is_empty() || c0 == "0" || c0.starts_with('-') {
                continue;
            }
            if let Ok(v) = raw_to_value_str(c0, scale).parse::<T>() {
                out.push(v);
            }
            if out.len() >= 24 {
                break;
            }
        }
    }
    out
}

macro_rules! golden {
    ($name:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/", $name)
    };
}

/// One `(width, scale)` cell: load its operands, then a time-budgeted `.ln()`
/// loop (≈4 s) so the ~3-orders cost spread (narrow ns → wide ms) all profiles
/// in bounded time.
macro_rules! cell {
    ($sel:expr, $tag:literal, $alias:ident, $scale:literal, $file:literal) => {
        if $sel == $tag {
            let ops = load_ops::<decimal_scaled::$alias<$scale>>(golden!($file), $scale);
            let budget = Duration::from_secs(4);
            let t0 = Instant::now();
            let mut acc = ops[0];
            let mut count = 0u64;
            while t0.elapsed() < budget {
                for &x in &ops {
                    acc = black_box(black_box(x).ln());
                }
                count += ops.len() as u64;
            }
            black_box(acc);
            eprintln!(
                "{}: ops={} calls={} ({:.1} ns/ln)",
                $sel,
                ops.len(),
                count,
                t0.elapsed().as_nanos() as f64 / count as f64,
            );
            return;
        }
    };
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();
    cell!(sel, "d18_s0", D18, 0, "ln_d18_s0.txt");
    cell!(sel, "d18_s4", D18, 4, "ln_d18_s4.txt");
    cell!(sel, "d18_s9", D18, 9, "ln_d18_s9.txt");
    cell!(sel, "d18_s13", D18, 13, "ln_d18_s13.txt");
    cell!(sel, "d18_s17", D18, 17, "ln_d18_s17.txt");
    cell!(sel, "d38_s0", D38, 0, "ln_d38_s0.txt");
    cell!(sel, "d38_s9", D38, 9, "ln_d38_s9.txt");
    cell!(sel, "d38_s19", D38, 19, "ln_d38_s19.txt");
    cell!(sel, "d38_s28", D38, 28, "ln_d38_s28.txt");
    cell!(sel, "d38_s37", D38, 37, "ln_d38_s37.txt");
    cell!(sel, "d57_s0", D57, 0, "ln_d57_s0.txt");
    cell!(sel, "d57_s14", D57, 14, "ln_d57_s14.txt");
    cell!(sel, "d57_s28", D57, 28, "ln_d57_s28.txt");
    cell!(sel, "d57_s42", D57, 42, "ln_d57_s42.txt");
    cell!(sel, "d57_s56", D57, 56, "ln_d57_s56.txt");
    cell!(sel, "d76_s0", D76, 0, "ln_d76_s0.txt");
    cell!(sel, "d76_s19", D76, 19, "ln_d76_s19.txt");
    cell!(sel, "d76_s38", D76, 38, "ln_d76_s38.txt");
    cell!(sel, "d76_s57", D76, 57, "ln_d76_s57.txt");
    cell!(sel, "d76_s75", D76, 75, "ln_d76_s75.txt");
    cell!(sel, "d115_s0", D115, 0, "ln_d115_s0.txt");
    cell!(sel, "d115_s28", D115, 28, "ln_d115_s28.txt");
    cell!(sel, "d115_s57", D115, 57, "ln_d115_s57.txt");
    cell!(sel, "d115_s86", D115, 86, "ln_d115_s86.txt");
    cell!(sel, "d115_s114", D115, 114, "ln_d115_s114.txt");
    cell!(sel, "d153_s0", D153, 0, "ln_d153_s0.txt");
    cell!(sel, "d153_s38", D153, 38, "ln_d153_s38.txt");
    cell!(sel, "d153_s76", D153, 76, "ln_d153_s76.txt");
    cell!(sel, "d153_s114", D153, 114, "ln_d153_s114.txt");
    cell!(sel, "d153_s152", D153, 152, "ln_d153_s152.txt");
    cell!(sel, "d230_s0", D230, 0, "ln_d230_s0.txt");
    cell!(sel, "d230_s57", D230, 57, "ln_d230_s57.txt");
    cell!(sel, "d230_s115", D230, 115, "ln_d230_s115.txt");
    cell!(sel, "d230_s172", D230, 172, "ln_d230_s172.txt");
    cell!(sel, "d230_s229", D230, 229, "ln_d230_s229.txt");
    cell!(sel, "d307_s0", D307, 0, "ln_d307_s0.txt");
    cell!(sel, "d307_s76", D307, 76, "ln_d307_s76.txt");
    cell!(sel, "d307_s153", D307, 153, "ln_d307_s153.txt");
    cell!(sel, "d307_s230", D307, 230, "ln_d307_s230.txt");
    cell!(sel, "d307_s306", D307, 306, "ln_d307_s306.txt");
    cell!(sel, "d462_s0", D462, 0, "ln_d462_s0.txt");
    cell!(sel, "d462_s115", D462, 115, "ln_d462_s115.txt");
    cell!(sel, "d462_s231", D462, 231, "ln_d462_s231.txt");
    cell!(sel, "d462_s346", D462, 346, "ln_d462_s346.txt");
    cell!(sel, "d462_s461", D462, 461, "ln_d462_s461.txt");
    cell!(sel, "d616_s0", D616, 0, "ln_d616_s0.txt");
    cell!(sel, "d616_s154", D616, 154, "ln_d616_s154.txt");
    cell!(sel, "d616_s308", D616, 308, "ln_d616_s308.txt");
    cell!(sel, "d616_s462", D616, 462, "ln_d616_s462.txt");
    cell!(sel, "d616_s615", D616, 615, "ln_d616_s615.txt");
    cell!(sel, "d924_s0", D924, 0, "ln_d924_s0.txt");
    cell!(sel, "d924_s231", D924, 231, "ln_d924_s231.txt");
    cell!(sel, "d924_s462", D924, 462, "ln_d924_s462.txt");
    cell!(sel, "d924_s693", D924, 693, "ln_d924_s693.txt");
    cell!(sel, "d924_s923", D924, 923, "ln_d924_s923.txt");
    cell!(sel, "d1232_s0", D1232, 0, "ln_d1232_s0.txt");
    cell!(sel, "d1232_s308", D1232, 308, "ln_d1232_s308.txt");
    cell!(sel, "d1232_s616", D1232, 616, "ln_d1232_s616.txt");
    cell!(sel, "d1232_s924", D1232, 924, "ln_d1232_s924.txt");
    cell!(sel, "d1232_s1231", D1232, 1231, "ln_d1232_s1231.txt");

    eprintln!("usage: ln_probe <width>_s<scale>  (got {sel:?})");
    std::process::exit(2);
}
