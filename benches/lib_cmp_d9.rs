//! Per-tier library_comparison bench for D9 (32-bit storage).
//! Run: cargo bench --bench lib_cmp_d9

#[macro_use]
mod lib_cmp_common;

use criterion::{criterion_group, criterion_main, Criterion};
use decimal_scaled::D9;
use fixed::types::I16F16;
use rust_decimal::Decimal as RustDecimal;

fn bench(c: &mut Criterion) {
    for &scale in &[0_usize, 5, 9] {
        let group_name = format!("lib_cmp/32bit_s{scale}");
        let mut g = c.benchmark_group(&group_name);

        match scale {
            0 => {
                let a = D9::<0>::from_int(2); let b = D9::<0>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            5 => {
                let a = D9::<5>::from_int(2); let b = D9::<5>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            9 => {
                let a = D9::<9>::from_int(2); let b = D9::<9>::from_int(1);
                arith_copy!(g, "decimal-scaled", a, b);
            }
            _ => unreachable!(),
        }

        {
            let mant = 2_i64.checked_mul(10_i64.pow(scale as u32)).unwrap();
            let mantb = 1_i64.checked_mul(10_i64.pow(scale as u32)).unwrap();
            let a = RustDecimal::new(mant, scale as u32);
            let b = RustDecimal::new(mantb, scale as u32);
            arith_copy!(g, "rust_decimal", a, b);
        }

        {
            let a = I16F16::from_num(2); let b = I16F16::from_num(1);
            arith_copy!(g, "fixed_i16f16", a, b);
        }

        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
