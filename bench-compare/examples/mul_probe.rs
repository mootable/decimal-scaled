//! samply probe: decimal `mul` at the regressed bbc cells, BOTH versions.
//!
//! Replays the exact bbc operand (`2.0 * 3.5`, the `compare_common` harness
//! op) at `D230<229>` and `D924<462>`, in a tight `black_box`-guarded loop, for
//! the branch (`decimal_scaled`) AND prod (`prod` = 0.4.4) side by side — one
//! version per run so the symbolized frames don't collide (both crates share
//! the internal name `decimal_scaled`).
//!
//! Run (single argv token, matching `run_samply.sh`):
//!   mul_probe <d230_branch|d230_prod|d924_branch|d924_prod>
//! Iteration counts are tuned per cell to ~3-5 s of work (dense samples).

use std::hint::black_box;

fn loop_branch_d230() {
    let x: decimal_scaled::D230<229> = "2.0".parse().unwrap();
    let b: decimal_scaled::D230<229> = "3.5".parse().unwrap();
    let mut acc = x;
    for _ in 0..3_000_000u64 {
        acc = black_box(black_box(x) * black_box(b));
    }
    black_box(acc);
}

fn loop_prod_d230() {
    let x: prod::D230<229> = "2.0".parse().unwrap();
    let b: prod::D230<229> = "3.5".parse().unwrap();
    let mut acc = x;
    for _ in 0..3_000_000u64 {
        acc = black_box(black_box(x) * black_box(b));
    }
    black_box(acc);
}

fn loop_branch_d924() {
    let x: decimal_scaled::D924<462> = "2.0".parse().unwrap();
    let b: decimal_scaled::D924<462> = "3.5".parse().unwrap();
    let mut acc = x;
    for _ in 0..1_000_000u64 {
        acc = black_box(black_box(x) * black_box(b));
    }
    black_box(acc);
}

fn loop_prod_d924() {
    let x: prod::D924<462> = "2.0".parse().unwrap();
    let b: prod::D924<462> = "3.5".parse().unwrap();
    let mut acc = x;
    for _ in 0..1_000_000u64 {
        acc = black_box(black_box(x) * black_box(b));
    }
    black_box(acc);
}

fn main() {
    let sel = std::env::args().nth(1).unwrap_or_default();
    match sel.as_str() {
        "d230_branch" => loop_branch_d230(),
        "d230_prod" => loop_prod_d230(),
        "d924_branch" => loop_branch_d924(),
        "d924_prod" => loop_prod_d924(),
        other => {
            eprintln!("usage: mul_probe <d230_branch|d230_prod|d924_branch|d924_prod> (got {other:?})");
            std::process::exit(2);
        }
    }
}
