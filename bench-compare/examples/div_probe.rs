//! samply probe: decimal `div` at the regressed s0 cells, BOTH versions.
//!
//! bbc shows narrow div@s0 regressed (div_D57_s0 1.61×, div_D115_s0 1.36×).
//! At scale 0 a decimal `a / b` is the INT divide (no rescale), so this probes
//! the int-divide path the owner wants analysed first. Replays the bbc operand
//! (`2 / 3` at scale 0) in a tight `black_box` loop, branch (`decimal_scaled`)
//! vs prod (`prod` = 0.4.4) — one version per run so symbolized frames don't
//! collide (both crates share the internal name `decimal_scaled`).
//!
//! Run: div_probe <d57_branch|d57_prod|d115_branch|d115_prod>

use std::hint::black_box;

fn loop_branch_d57() {
    let x: decimal_scaled::D57<0> = "2".parse().unwrap();
    let b: decimal_scaled::D57<0> = "3".parse().unwrap();
    let mut acc = x;
    for _ in 0..6_000_000u64 {
        acc = black_box(black_box(x) / black_box(b));
    }
    black_box(acc);
}

fn loop_prod_d57() {
    let x: prod::D57<0> = "2".parse().unwrap();
    let b: prod::D57<0> = "3".parse().unwrap();
    let mut acc = x;
    for _ in 0..6_000_000u64 {
        acc = black_box(black_box(x) / black_box(b));
    }
    black_box(acc);
}

fn loop_branch_d115() {
    let x: decimal_scaled::D115<0> = "2".parse().unwrap();
    let b: decimal_scaled::D115<0> = "3".parse().unwrap();
    let mut acc = x;
    for _ in 0..5_000_000u64 {
        acc = black_box(black_box(x) / black_box(b));
    }
    black_box(acc);
}

fn loop_prod_d115() {
    let x: prod::D115<0> = "2".parse().unwrap();
    let b: prod::D115<0> = "3".parse().unwrap();
    let mut acc = x;
    for _ in 0..5_000_000u64 {
        acc = black_box(black_box(x) / black_box(b));
    }
    black_box(acc);
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();
    match sel.as_str() {
        "d57_branch" => loop_branch_d57(),
        "d57_prod" => loop_prod_d57(),
        "d115_branch" => loop_branch_d115(),
        "d115_prod" => loop_prod_d115(),
        other => {
            eprintln!("usage: div_probe <d57_branch|d57_prod|d115_branch|d115_prod> (got {other:?})");
            std::process::exit(2);
        }
    }
}
