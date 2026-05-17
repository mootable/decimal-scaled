//! Micro-bench: wide-tier `exp_strict` / `ln_strict` per tier and
//! midpoint scale.
//!
//! Run with:
//!     cargo run --release --example expbench --features wide,x-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D153, D307, D38, D76};

const ITERS: u32 = 5_000;

fn time<F: FnMut()>(label: &str, mut f: F) {
    for _ in 0..100 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    let elapsed = start.elapsed();
    let per = elapsed.as_nanos() as f64 / ITERS as f64;
    println!("  {:<40} {:>10.0} ns/op", label, per);
}

fn main() {
    println!("== exp_strict ==");
    {
        let a = D38::<19>::ONE / D38::<19>::from_int(2); // 0.5
        time("D38<19>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D76::<35>::ONE / D76::<35>::from_int(2);
        time("D76<35>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D153::<75>::ONE / D153::<75>::from_int(2);
        time("D153<75>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D307::<150>::ONE / D307::<150>::from_int(2);
        time("D307<150>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }

    println!();
    println!("== ln_strict ==");
    {
        let a = D38::<19>::from_int(2) - D38::<19>::ONE / D38::<19>::from_int(2); // 1.5
        time("D38<19>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D76::<35>::from_int(2) - D76::<35>::ONE / D76::<35>::from_int(2);
        time("D76<35>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D153::<75>::from_int(2) - D153::<75>::ONE / D153::<75>::from_int(2);
        time("D153<75>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D307::<150>::from_int(2) - D307::<150>::ONE / D307::<150>::from_int(2);
        time("D307<150>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
}
