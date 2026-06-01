// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Focused samply probe for the decimal `atan` ALGORITHM (the Tang
//! `atan_tang_3limb_s44_56` kernel + the generic `atan_fixed` / `atan_series`
//! cores) across ALL widths × the 5 scale points, driven by the bbc operand
//! (`atan(0.5)`) + the adversarial golden inputs. One `<width>_s<scale>` per
//! run so samply attributes time cleanly to one cell. Branch-only (the
//! algorithm's OWN hot frames — the branch-vs-prod ratios come from the bbc,
//! not here).
//!
//! Build: cargo build --release --example atan_probe --features "wide x-wide xx-wide"
//! Run:   atan_probe d57_s56      # the worst atan regression cell

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Reconstruct a decimal-value string from a golden RAW storage integer at
/// `scale` (raw = value·10^scale → insert a point `scale` digits from the
/// right). Mirrors `examples/ln_probe.rs`.
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

/// Load `atan` input operands at `scale`: the bbc operand `0.5` FIRST (an
/// interior value that engages the table reduction), then the golden table's
/// input column (col 0) — the adversarial near-0 / near-1 / tie / max-magnitude
/// inputs. atan's domain is all reals, so no input is filtered. Caps the set so
/// the profile stays focused.
fn load_ops<T: std::str::FromStr>(path: &str, scale: u32) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let mut out = vec!["0.5".parse::<T>().expect("parse bbc 0.5")];
    if let Ok(text) = std::fs::read_to_string(path) {
        for line in text.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let c0 = l.split('\t').next().unwrap_or("");
            if c0.is_empty() {
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

/// One `(width, scale)` cell: load its operands, then a time-budgeted `.atan()`
/// loop (≈4 s) so the cost spread (narrow ns → wide µs) all profiles in
/// bounded time.
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
                    acc = black_box(black_box(x).atan());
                }
                count += ops.len() as u64;
            }
            black_box(acc);
            eprintln!(
                "{}: ops={} calls={} ({:.1} ns/atan)",
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
    cell!(sel, "d18_s0", D18, 0, "atan_d18_s0.txt");
    cell!(sel, "d18_s4", D18, 4, "atan_d18_s4.txt");
    cell!(sel, "d18_s9", D18, 9, "atan_d18_s9.txt");
    cell!(sel, "d18_s13", D18, 13, "atan_d18_s13.txt");
    cell!(sel, "d18_s17", D18, 17, "atan_d18_s17.txt");
    cell!(sel, "d38_s0", D38, 0, "atan_d38_s0.txt");
    cell!(sel, "d38_s9", D38, 9, "atan_d38_s9.txt");
    cell!(sel, "d38_s19", D38, 19, "atan_d38_s19.txt");
    cell!(sel, "d38_s28", D38, 28, "atan_d38_s28.txt");
    cell!(sel, "d38_s37", D38, 37, "atan_d38_s37.txt");
    cell!(sel, "d57_s0", D57, 0, "atan_d57_s0.txt");
    cell!(sel, "d57_s14", D57, 14, "atan_d57_s14.txt");
    cell!(sel, "d57_s28", D57, 28, "atan_d57_s28.txt");
    cell!(sel, "d57_s42", D57, 42, "atan_d57_s42.txt");
    cell!(sel, "d57_s56", D57, 56, "atan_d57_s56.txt");
    cell!(sel, "d76_s0", D76, 0, "atan_d76_s0.txt");
    cell!(sel, "d76_s19", D76, 19, "atan_d76_s19.txt");
    cell!(sel, "d76_s38", D76, 38, "atan_d76_s38.txt");
    cell!(sel, "d76_s57", D76, 57, "atan_d76_s57.txt");
    cell!(sel, "d76_s75", D76, 75, "atan_d76_s75.txt");
    cell!(sel, "d115_s0", D115, 0, "atan_d115_s0.txt");
    cell!(sel, "d115_s28", D115, 28, "atan_d115_s28.txt");
    cell!(sel, "d115_s57", D115, 57, "atan_d115_s57.txt");
    cell!(sel, "d115_s86", D115, 86, "atan_d115_s86.txt");
    cell!(sel, "d115_s114", D115, 114, "atan_d115_s114.txt");
    cell!(sel, "d153_s0", D153, 0, "atan_d153_s0.txt");
    cell!(sel, "d153_s38", D153, 38, "atan_d153_s38.txt");
    cell!(sel, "d153_s76", D153, 76, "atan_d153_s76.txt");
    cell!(sel, "d153_s114", D153, 114, "atan_d153_s114.txt");
    cell!(sel, "d153_s152", D153, 152, "atan_d153_s152.txt");
    cell!(sel, "d230_s0", D230, 0, "atan_d230_s0.txt");
    cell!(sel, "d230_s57", D230, 57, "atan_d230_s57.txt");
    cell!(sel, "d230_s115", D230, 115, "atan_d230_s115.txt");
    cell!(sel, "d230_s172", D230, 172, "atan_d230_s172.txt");
    cell!(sel, "d230_s229", D230, 229, "atan_d230_s229.txt");
    cell!(sel, "d307_s0", D307, 0, "atan_d307_s0.txt");
    cell!(sel, "d307_s76", D307, 76, "atan_d307_s76.txt");
    cell!(sel, "d307_s153", D307, 153, "atan_d307_s153.txt");
    cell!(sel, "d307_s230", D307, 230, "atan_d307_s230.txt");
    cell!(sel, "d307_s306", D307, 306, "atan_d307_s306.txt");
    cell!(sel, "d462_s0", D462, 0, "atan_d462_s0.txt");
    cell!(sel, "d462_s115", D462, 115, "atan_d462_s115.txt");
    cell!(sel, "d462_s231", D462, 231, "atan_d462_s231.txt");
    cell!(sel, "d462_s346", D462, 346, "atan_d462_s346.txt");
    cell!(sel, "d462_s461", D462, 461, "atan_d462_s461.txt");

    eprintln!("usage: atan_probe <width>_s<scale>  (got {sel:?})");
    std::process::exit(2);
}
