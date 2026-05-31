//! samply probe: wide `ln` at the regressed cells, BOTH versions.
//!
//! bbc (97a10531) shows ln regressed at the wide tiers: ln_D924_s462 1.66×,
//! ln_D1232_s616 1.75×. Replays the bbc operand (`ln(2.0)` at the cell scale —
//! see bench-compare/benches/compare_common.rs `x = "2.0"`) in a tight
//! `black_box` loop, branch (`decimal_scaled`) vs prod (`prod` = 0.4.4) — one
//! version per run so symbolized frames don't collide.
//!
//! Run: ln_probe <d924_branch|d924_prod|d1232_branch|d1232_prod>

use std::hint::black_box;

fn loop_branch_d924() {
    let x: decimal_scaled::D924<462> = "2.0".parse().unwrap();
    let mut acc = x;
    for _ in 0..50_000u64 {
        acc = black_box(black_box(x).ln());
    }
    black_box(acc);
}

fn loop_prod_d924() {
    let x: prod::D924<462> = "2.0".parse().unwrap();
    let mut acc = x;
    for _ in 0..50_000u64 {
        acc = black_box(black_box(x).ln());
    }
    black_box(acc);
}

fn loop_branch_d1232() {
    let x: decimal_scaled::D1232<616> = "2.0".parse().unwrap();
    let mut acc = x;
    for _ in 0..30_000u64 {
        acc = black_box(black_box(x).ln());
    }
    black_box(acc);
}

fn loop_prod_d1232() {
    let x: prod::D1232<616> = "2.0".parse().unwrap();
    let mut acc = x;
    for _ in 0..30_000u64 {
        acc = black_box(black_box(x).ln());
    }
    black_box(acc);
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();
    match sel.as_str() {
        "d924_branch" => loop_branch_d924(),
        "d924_prod" => loop_prod_d924(),
        "d1232_branch" => loop_branch_d1232(),
        "d1232_prod" => loop_prod_d1232(),
        other => {
            eprintln!("usage: ln_probe <d924_branch|d924_prod|d1232_branch|d1232_prod> (got {other:?})");
            std::process::exit(2);
        }
    }
}
