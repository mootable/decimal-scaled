//! Micro-bench: wide-tier `exp` / `ln` per tier and
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
    println!("== exp (narrow-tier sanity) ==");
    {
        let a = D18::<5>::ONE / D18::<5>::try_from(2_i64).unwrap();
        time("D9<5>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }
    {
        let a = D18::<9>::ONE / D18::<9>::try_from(2_i64).unwrap();
        time("D18<9>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }

    println!();
    println!("== exp ==");
    {
        let a = D38::<19>::ONE / D38::<19>::try_from(2_i64).unwrap(); // 0.5
        time("D38<19>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }
    {
        let a = D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }
    {
        let a = D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }
    {
        let a = D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::exp(0.5)", || {
            black_box(black_box(a).exp());
        });
    }

    println!();
    println!("== ln ==");
    {
        let a = D38::<19>::try_from(2_i64).unwrap() - D38::<19>::ONE / D38::<19>::try_from(2_i64).unwrap(); // 1.5
        time("D38<19>::ln(1.5)", || {
            black_box(black_box(a).ln());
        });
    }
    {
        let a = D76::<35>::try_from(2_i64).unwrap() - D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::ln(1.5)", || {
            black_box(black_box(a).ln());
        });
    }
    {
        let a = D153::<75>::try_from(2_i64).unwrap() - D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::ln(1.5)", || {
            black_box(black_box(a).ln());
        });
    }
    {
        let a = D307::<150>::try_from(2_i64).unwrap() - D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::ln(1.5)", || {
            black_box(black_box(a).ln());
        });
    }

    println!();
    println!("== sin (input = 1) ==");
    {
        let a = D38::<19>::ONE;
        time("D38<19>::sin(1)", || {
            black_box(black_box(a).sin());
        });
    }
    {
        let a = D76::<35>::ONE;
        time("D76<35>::sin(1)", || {
            black_box(black_box(a).sin());
        });
    }
    {
        let a = D153::<75>::ONE;
        time("D153<75>::sin(1)", || {
            black_box(black_box(a).sin());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::sin(1)", || {
            black_box(black_box(a).sin());
        });
    }

    println!();
    println!("== sin (input ≈ 1.5, near π/2) ==");
    {
        let a = D76::<35>::ONE + D76::<35>::ONE / D76::<35>::try_from(2_i64).unwrap();
        time("D76<35>::sin(1.5)", || {
            black_box(black_box(a).sin());
        });
    }
    {
        let a = D153::<75>::ONE + D153::<75>::ONE / D153::<75>::try_from(2_i64).unwrap();
        time("D153<75>::sin(1.5)", || {
            black_box(black_box(a).sin());
        });
    }
    {
        let a = D307::<150>::ONE + D307::<150>::ONE / D307::<150>::try_from(2_i64).unwrap();
        time("D307<150>::sin(1.5)", || {
            black_box(black_box(a).sin());
        });
    }

    println!();
    println!("== sin_cos vs (sin + cos) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::(sin, cos)", || {
            black_box((black_box(a).sin(), black_box(a).cos()));
        });
        time("D76<35>::sin_cos", || {
            black_box(black_box(a).sin_cos());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::(sin, cos)", || {
            black_box((black_box(a).sin(), black_box(a).cos()));
        });
        time("D307<150>::sin_cos", || {
            black_box(black_box(a).sin_cos());
        });
    }

    println!();
    println!("== cos / tan (post-Pythagorean) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::cos(1)", || {
            black_box(black_box(a).cos());
        });
        time("D76<35>::tan(1)", || {
            black_box(black_box(a).tan());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::cos(1)", || {
            black_box(black_box(a).cos());
        });
        time("D307<150>::tan(1)", || {
            black_box(black_box(a).tan());
        });
    }

    println!();
    println!("== atan (input = 1) ==");
    {
        let a = D76::<35>::ONE;
        time("D76<35>::atan(1)", || {
            black_box(black_box(a).atan());
        });
    }
    {
        let a = D153::<75>::ONE;
        time("D153<75>::atan(1)", || {
            black_box(black_box(a).atan());
        });
    }
    {
        let a = D307::<150>::ONE;
        time("D307<150>::atan(1)", || {
            black_box(black_box(a).atan());
        });
    }
}
