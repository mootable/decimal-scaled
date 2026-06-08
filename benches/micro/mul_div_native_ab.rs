// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the narrow decimal divide policy
//! (`src/policy/div.rs`).
//!
//! Decision being modelled: for narrow storage (`N <= 2`, D18 / D38), should
//! the policy route to the hardware-`i128` `div_native` arm or the generic
//! `div_widen_scale` arm (which forms a `2N`-limb scratch numerator and
//! divides via the int-layer slice divide)?
//!
//! - `native` -> `div_native` (hardware `i128`, valid N <= 2).
//! - `widen`  -> `div_widen_scale` (generic over N).
//!
//! Result (this machine): native loses ~1.08x at D18 (`N == 1`) but wins
//! 1.26-1.46x at D38 (`N == 2`, s6 / s18), so the policy routes `N == 2`
//! only. (Decimal multiply was benched the same way and lost at every
//! narrow band, so it stays on `mul_widen_divide` and has no native arm.)
//!
//! Benched at a low scale (s6) and a higher scale (D38: s18) so both the
//! plain `i128` band and the 256-bit-intermediate band are exercised.
//! Inputs and outputs are `black_box`-guarded by the harness so no kernel
//! const-folds away. The bench also asserts native == widen on every input
//! before timing.
//!
//! Run with:
//! `cargo bench --features "wide bench-alt" --bench mul_div_native_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::RoundingMode;
use decimal_scaled::__bench_internals::{
    dec_div_native, dec_div_widen_scale_n1, dec_div_widen_scale_n2, dec_mul_native,
    dec_mul_widen_divide_n1, dec_mul_widen_divide_n2, int_from_mag_limbs,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const MODE: RoundingMode = RoundingMode::HalfToEven;

#[derive(Clone)]
struct Pair<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

/// Build an `Int<N>` from a non-negative magnitude (low two u64 limbs).
fn fromu<const N: usize>(v: u128) -> Int<N> {
    let mut mag = [0u64; N];
    mag[0] = v as u64;
    if N > 1 {
        mag[1] = (v >> 64) as u64;
    }
    int_from_mag_limbs::<N>(&mag)
}

/// In-range operand set so the result of `(a*10^S)/b` fits the storage,
/// exercising the hot (non-overflow) path both kernels share. `bits` is the
/// operand magnitude budget (~half the storage width). Three buckets.
fn operand_set<const N: usize>(bits: u32) -> Vec<Pair<N>> {
    let hi: u128 = (1u128 << bits) - 1;
    let mid: u128 = 1u128 << (bits / 2);
    vec![
        Pair { label: "low", a: fromu::<N>(123_457), b: fromu::<N>(98_765) },
        Pair { label: "mid", a: fromu::<N>(mid | 0xBEEF), b: fromu::<N>((mid >> 1) | 0x1357) },
        Pair { label: "high", a: fromu::<N>(hi), b: fromu::<N>(hi - 0xABCD_1234) },
    ]
}

// ---- D18 (Int<1>) ----

fn div1<const S: u32>(c: &mut Criterion, label: &str) {
    let m = Int::<1>::TEN.pow(S);
    let nat = move |p: Pair<1>| dec_div_native::<1, S>(p.a, p.b, MODE);
    let wid = move |p: Pair<1>| dec_div_widen_scale_n1(p.a, p.b, m, MODE);
    for p in operand_set::<1>(28) {
        assert_eq!(nat(p.clone()), wid(p.clone()), "div1 {label} {}", p.label);
    }
    compare_all(c, &format!("div_native/{label}"), |p: &Pair<1>| p.label.to_string(),
        operand_set::<1>(28),
        vec![("native", Box::new(nat) as Box<dyn Fn(Pair<1>) -> Int<1>>), ("widen", Box::new(wid))]);
}

// ---- D38 (Int<2>) ----

fn div2<const S: u32>(c: &mut Criterion, label: &str) {
    let m = Int::<2>::TEN.pow(S);
    let nat = move |p: Pair<2>| dec_div_native::<2, S>(p.a, p.b, MODE);
    let wid = move |p: Pair<2>| dec_div_widen_scale_n2(p.a, p.b, m, MODE);
    for p in operand_set::<2>(56) {
        assert_eq!(nat(p.clone()), wid(p.clone()), "div2 {label} {}", p.label);
    }
    compare_all(c, &format!("div_native/{label}"), |p: &Pair<2>| p.label.to_string(),
        operand_set::<2>(56),
        vec![("native", Box::new(nat) as Box<dyn Fn(Pair<2>) -> Int<2>>), ("widen", Box::new(wid))]);
}

// ---- decimal multiply: native vs widen-divide ----

fn mul1<const S: u32>(c: &mut Criterion, label: &str) {
    let m = Int::<1>::TEN.pow(S);
    let nat = move |p: Pair<1>| dec_mul_native::<1, S>(p.a, p.b, m, MODE);
    let wid = move |p: Pair<1>| dec_mul_widen_divide_n1::<S>(p.a, p.b, MODE);
    for p in operand_set::<1>(28) {
        assert_eq!(nat(p.clone()), wid(p.clone()), "mul1 {label} {}", p.label);
    }
    compare_all(c, &format!("mul_native/{label}"), |p: &Pair<1>| p.label.to_string(),
        operand_set::<1>(28),
        vec![("native", Box::new(nat) as Box<dyn Fn(Pair<1>) -> Int<1>>), ("widen", Box::new(wid))]);
}

fn mul2<const S: u32>(c: &mut Criterion, label: &str) {
    let m = Int::<2>::TEN.pow(S);
    let nat = move |p: Pair<2>| dec_mul_native::<2, S>(p.a, p.b, m, MODE);
    let wid = move |p: Pair<2>| dec_mul_widen_divide_n2::<S>(p.a, p.b, MODE);
    for p in operand_set::<2>(56) {
        assert_eq!(nat(p.clone()), wid(p.clone()), "mul2 {label} {}", p.label);
    }
    compare_all(c, &format!("mul_native/{label}"), |p: &Pair<2>| p.label.to_string(),
        operand_set::<2>(56),
        vec![("native", Box::new(nat) as Box<dyn Fn(Pair<2>) -> Int<2>>), ("widen", Box::new(wid))]);
}

fn bench(c: &mut Criterion) {
    // div: full SCALE sweep across the narrow-tier band so the policy
    // verdict (native at N==1|2) is validated at every scale the bbc
    // grid samples, not just the {6,18} corners. The bbc div regression
    // cluster sits at D18_s0/4/9/13/17 and D38_s0/9 -- those concrete
    // cells anchor the table.
    div1::<0>(c, "D18_s0");
    div1::<4>(c, "D18_s4");
    div1::<9>(c, "D18_s9");
    div1::<13>(c, "D18_s13");
    div1::<17>(c, "D18_s17");
    div1::<18>(c, "D18_s18");
    div2::<0>(c, "D38_s0");
    div2::<9>(c, "D38_s9");
    div2::<18>(c, "D38_s18");
    div2::<28>(c, "D38_s28");
    div2::<37>(c, "D38_s37");
    mul1::<6>(c, "D18_s6");
    mul1::<18>(c, "D18_s18");
    mul2::<6>(c, "D38_s6");
    mul2::<18>(c, "D38_s18");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
