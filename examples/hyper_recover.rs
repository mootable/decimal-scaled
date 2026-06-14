//! Micro-bench: wide-tier hyperbolic recovery check.
//!
//! The wide hyperbolics (`sinh`/`cosh`/`tanh`) run their `exp` through the
//! width-generic `exp_generic` core at `Wexp` (up to `Int<256>`). Their only
//! arithmetic beyond `exp` is one reciprocal divide plus an add, so each
//! should sit within a small factor of `exp_strict` at the same tier. A large
//! cosh/exp ratio means the `÷ 10^w` in the generic core is the bottleneck.
//!
//! Run with:
//!     cargo run --release --example hyper_recover --features wide,x-wide,xx-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D307, D616, D924, D1232};

const ITERS: u32 = 2_000;

fn time<F: FnMut()>(label: &str, mut f: F) -> f64 {
    for _ in 0..50 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    let per = start.elapsed().as_nanos() as f64 / ITERS as f64;
    println!("  {:<32} {:>12.0} ns/op", label, per);
    per
}

fn main() {
    macro_rules! tier {
        ($T:ident, $S:literal, $name:literal) => {{
            // 1.5 — a normal-regime argument exercising the exp Taylor core.
            let a = $T::<$S>::try_from(3).unwrap() / $T::<$S>::try_from(2).unwrap();
            println!("== {} ==", $name);
            let e = time(concat!($name, "::exp"), || {
                black_box(black_box(a).exp_strict());
            });
            let c = time(concat!($name, "::cosh"), || {
                black_box(black_box(a).cosh_strict());
            });
            let s = time(concat!($name, "::sinh"), || {
                black_box(black_box(a).sinh_strict());
            });
            let t = time(concat!($name, "::tanh"), || {
                black_box(black_box(a).tanh_strict());
            });
            println!(
                "  -> cosh/exp {:.2}x  sinh/exp {:.2}x  tanh/exp {:.2}x\n",
                c / e,
                s / e,
                t / e
            );
        }};
    }
    tier!(D307, 150, "D307<150>");
    tier!(D616, 308, "D616<308>");
    tier!(D924, 461, "D924<461>");
    tier!(D1232, 615, "D1232<615>");
}
