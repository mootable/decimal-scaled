//! Per-tier library_comparison bench for D18 (64-bit storage).
//! Run: cargo bench --bench lib_cmp_d18
//!      cargo bench --bench lib_cmp_d18 -- _s9/   (just scale 9)
//!
//! Scale set dedup{0, 30, S/2, S-1} with S=18: {0, 9, 17} (30 > S-1, dropped).
//! Each scale's groups are `lib_cmp/64bit_s<scale>`; a single scale is
//! selectable by the criterion name-filter `-- _s<scale>/` (the trailing `/`
//! anchors the scale).

#[macro_use]
mod lib_cmp_common;

use criterion::{Criterion, criterion_group, criterion_main};
use decimal_scaled::D18;
use fastnum::dec64;
use fixed::types::I32F32;
use rust_decimal::Decimal as RustDecimal;

// One full peer-comparison block at one const-generic SCALE.
macro_rules! tier_scale {
    ($c:expr, $s:literal) => {{
        let mut g = $c.benchmark_group(concat!("lib_cmp/64bit_s", $s));

        let a = D18::<$s>::from(2);
        let b = D18::<$s>::from(1);
        arith_copy!(g, "decimal-scaled", a, b);

        {
            let mant = 2_i128.checked_mul(10_i128.pow($s as u32)).unwrap();
            let mantb = 1_i128.checked_mul(10_i128.pow($s as u32)).unwrap();
            let a = RustDecimal::from_i128_with_scale(mant, $s as u32);
            let b = RustDecimal::from_i128_with_scale(mantb, $s as u32);
            arith_copy!(g, "rust_decimal", a, b);
        }
        {
            let a = I32F32::from_num(2);
            let b = I32F32::from_num(1);
            arith_copy!(g, "fixed_i32f32", a, b);
        }
        {
            let a = dec64!(2);
            let b = dec64!(1);
            arith_copy!(g, "fastnum", a, b);
        }

        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    tier_scale!(c, 0);
    tier_scale!(c, 9);
    tier_scale!(c, 17);
}

criterion_group!(benches, bench);
criterion_main!(benches);
