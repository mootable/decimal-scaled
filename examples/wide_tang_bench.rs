//! Micro-bench: the WIDE-tier (D307..D1232) strict transcendentals in the
//! hot regression band, plus the pure-arithmetic non-Tang cells
//! (to_degrees/to_radians) whose cost is purely the wide work-int
//! arithmetic. Used to attribute the wide-band cost and to A/B the
//! u128-packed work-multiply (see `benches/micro/mul_low_u128_ab.rs` for
//! the isolated packed-vs-unpacked low-mul comparison).
//!
//! Run with:
//!     cargo run --release --example wide_tang_bench --features wide,x-wide,xx-wide

use std::hint::black_box;
use std::time::Instant;

use decimal_scaled::{D307, D462, D616, D924, D1232};

const ITERS: u32 = 1_000;

fn time<F: FnMut()>(label: &str, mut f: F) {
    for _ in 0..50 {
        f();
    }
    let start = Instant::now();
    for _ in 0..ITERS {
        f();
    }
    let elapsed = start.elapsed();
    let per = elapsed.as_nanos() as f64 / ITERS as f64;
    println!("  {:<34} {:>12.0} ns/op", label, per);
}

macro_rules! suite {
    ($ty:ident, $scale:literal) => {{
        let a = $ty::<$scale>::ONE / $ty::<$scale>::try_from(2_i64).unwrap();
        time(concat!(stringify!($ty), "<", stringify!($scale), ">::exp(0.5)"), || {
            black_box(black_box(a).exp_strict());
        });
        let o = $ty::<$scale>::ONE;
        time(concat!(stringify!($ty), "<", stringify!($scale), ">::sin(1)"), || {
            black_box(black_box(o).sin_strict());
        });
        // Pure-arithmetic, NON-Tang cells: x*180/pi and x*pi/180. No table
        // component, so their cost is purely the wide work-int arithmetic.
        time(concat!(stringify!($ty), "<", stringify!($scale), ">::to_degrees"), || {
            black_box(black_box(o).to_degrees_strict());
        });
        time(concat!(stringify!($ty), "<", stringify!($scale), ">::to_radians"), || {
            black_box(black_box(o).to_radians_strict());
        });
    }};
}

fn main() {
    println!("== wide-tier transcendentals + pure-arith non-Tang (mid scale) ==");
    suite!(D307, 150);
    suite!(D462, 230);
    suite!(D616, 300);
    suite!(D924, 460);
    suite!(D1232, 615);
}
