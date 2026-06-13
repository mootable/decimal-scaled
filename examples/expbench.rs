//! Micro-bench: wide-tier `exp_strict` / `ln_strict` per tier and
//! midpoint scale.
//!
//! Run with:
//!     cargo run --release --example expbench --features wide,x-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D18, D38, D76, D153, D307};

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
    println!("== exp_strict (narrow-tier sanity) ==");
    {
        let a = D18::<5>::ONE / D18::<5>::try_from(2_i64).unwrap();
        time("D9<5>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D18::<9>::ONE / D18::<9>::try_from(2_i64).unwrap();
        time("D18<9>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }

    println!();
    println!("== exp_strict ==");
    {
        let a = D38::<19>::ONE / D38::<19>::try_from(2_i64).unwrap(); // 0.5
        time("D38<19>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }
    {
        let a = D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::exp_strict(0.5)", || {
            black_box(black_box(a).exp_strict());
        });
    }

    println!();
    println!("== ln_strict ==");
    {
        let a = D38::<19>::try_from(2_i64).unwrap() - D38::<19>::ONE / D38::<19>::try_from(2_i64).unwrap(); // 1.5
        time("D38<19>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D76::<35>::try_from(2_i64).unwrap() - D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D153::<75>::try_from(2_i64).unwrap() - D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }
    {
        let a = D307::<150>::try_from(2_i64).unwrap() - D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::ln_strict(1.5)", || {
            black_box(black_box(a).ln_strict());
        });
    }

    println!();
    println!("== sin_strict (input = 1) ==");
    {
        let a = D38::<19>::ONE;
        time("D38<19>::sin_strict(1)", || {
            black_box(black_box(a).sin_strict());
        });
    }
    {
        let a = D76::<35>::ONE;
        time("D76<35>::sin_strict(1)", || {
            black_box(black_box(a).sin_strict());
        });
    }
    {
        let a = D153::<75>::ONE;
        time("D153<75>::sin_strict(1)", || {
            black_box(black_box(a).sin_strict());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::sin_strict(1)", || {
            black_box(black_box(a).sin_strict());
        });
    }

    println!();
    println!("== sin_strict (input ≈ 1.5, near π/2) ==");
    {
        let a = D76::<35>::ONE + D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::sin_strict(1.5)", || {
            black_box(black_box(a).sin_strict());
        });
    }
    {
        let a = D153::<75>::ONE + D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::sin_strict(1.5)", || {
            black_box(black_box(a).sin_strict());
        });
    }
    {
        let a = D307::<150>::ONE + D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::sin_strict(1.5)", || {
            black_box(black_box(a).sin_strict());
        });
    }

    println!();
    println!("== sin_cos_strict vs (sin + cos) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::(sin, cos)", || {
            black_box((black_box(a).sin_strict(), black_box(a).cos_strict()));
        });
        time("D76<35>::sin_cos_strict", || {
            black_box(black_box(a).sin_cos_strict());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::(sin, cos)", || {
            black_box((black_box(a).sin_strict(), black_box(a).cos_strict()));
        });
        time("D307<150>::sin_cos_strict", || {
            black_box(black_box(a).sin_cos_strict());
        });
    }

    println!();
    println!("== cos_strict / tan_strict (post-Pythagorean) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::cos_strict(1)", || {
            black_box(black_box(a).cos_strict());
        });
        time("D76<35>::tan_strict(1)", || {
            black_box(black_box(a).tan_strict());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::cos_strict(1)", || {
            black_box(black_box(a).cos_strict());
        });
        time("D307<150>::tan_strict(1)", || {
            black_box(black_box(a).tan_strict());
        });
    }

    println!();
    println!("== atan_strict (input = 1) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::atan_strict(1)", || {
            black_box(black_box(a).atan_strict());
        });
    }
    {
        let a = D153::<75>::ONE;
        time("D153<75>::atan_strict(1)", || {
            black_box(black_box(a).atan_strict());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::atan_strict(1)", || {
            black_box(black_box(a).atan_strict());
        });
    }
}
