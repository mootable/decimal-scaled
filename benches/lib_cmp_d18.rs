//! Per-tier library_comparison bench for D18 (64-bit storage).
//! Run: cargo bench --bench lib_cmp_d18

#[macro_use]
mod lib_cmp_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D18;
use fastnum::dec64;
use fixed::types::I32F32;
use rust_decimal::Decimal as RustDecimal;

fn bench(c: &mut Criterion) {
    for &scale in &[0_usize, 9, 18] {
        let group_name = format!("lib_cmp/64bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D18::<0>::from_int(2); let b = D18::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            9 => {
                let a = D18::<9>::from_int(2); let b = D18::<9>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            18 => {
                let a = D18::<18>::from_int(2); let b = D18::<18>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        if scale <= 18 {
            let mant = 2_i128.checked_mul(10_i128.pow(scale as u32)).unwrap();
            let mantb = 1_i128.checked_mul(10_i128.pow(scale as u32)).unwrap();
            let a = RustDecimal::from_i128_with_scale(mant, scale as u32);
            let b = RustDecimal::from_i128_with_scale(mantb, scale as u32);
            arith_copy!(g, "rust_decimal", a, b);
        }

        {
            let a = I32F32::from_num(2); let b = I32F32::from_num(1);
            arith_copy!(g, "fixed_i32f32", a, b);
        }

        {
            let a = dec64!(2); let b = dec64!(1);
            arith_copy!(g, "fastnum", a, b);
        }

        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
