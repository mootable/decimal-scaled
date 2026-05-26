// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam N-way A/B for the integer sum-of-squares policy
//! (`src/int/policy/sum_sq.rs`).
//!
//! Decision being mapped: per storage width `N` and operand magnitude
//! (significant limb length), which kernel forms `a² + b²` fastest while
//! remaining bit-identical to the reference:
//!
//! - `schoolbook` -> the WIRED kernel: each square via the general
//!   `mul_schoolbook(x, x, ..)` product (≈ L² limb-mults per square).
//! - `comba`      -> a KEPT alternative: each square via a symmetric
//!   product-scanning pass exploiting `xᵢ·xⱼ == xⱼ·xᵢ` (≈ L²/2 limb-mults).
//!   Bit-identical to schoolbook by construction.
//!
//! The continuous axis is operand significant-limb length (int has no scale
//! axis). The squaring cost is ~quadratic in `sig_len`, so comba's halved
//! partial-product count should help more as operands grow. Per width we
//! sweep:
//!
//! - `full`    — both operands span all `N` limbs (top bit cleared). `a²+b²`
//!   then exceeds the `Int<N>` signed range, so BOTH kernels form the full
//!   radicand and return `None` identically — the full-width squaring cost is
//!   what is measured (the worst case, where comba's edge is largest).
//! - `half`    — both operands occupy the low ⌈N/2⌉ limbs.
//! - `quarter` — both operands occupy the low ⌈N/4⌉ limbs.
//! - `single`  — both operands a single limb.
//! - `mixed`   — one full-width, one single-limb (unequal `sig_len`).
//!
//! Every candidate is asserted bit-identical to `schoolbook` (the reference)
//! across the full magnitude spread at every width BEFORE timing — the
//! validity wall. The harness `black_box`-guards inputs/outputs.
//!
//! Run: `cargo bench --features "wide x-wide xx-wide bench-alt" --bench sum_sq_ab`

use criterion::Criterion;
use decimal_scaled::Int;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// An operand pair `(a, b)` at a labelled magnitude class.
#[derive(Clone)]
struct PairIn<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

/// Build an `Int<N>` whose low `sig` limbs are random (top bit of the
/// significant top limb cleared so the value stays a positive magnitude).
fn synth<const N: usize>(s: &mut u64, sig: usize) -> Int<N> {
    let sig = sig.clamp(1, N);
    let mut limbs = [0u64; N];
    for limb in limbs.iter_mut().take(sig) {
        *limb = mix(s);
    }
    if limbs[sig - 1] == 0 {
        limbs[sig - 1] = 0x8000_0000_0000_0001;
    }
    // Clear the top bit of the SIGNIFICANT top limb so Int reads it positive.
    limbs[sig - 1] &= i64::MAX as u64;
    if limbs[sig - 1] == 0 {
        limbs[sig - 1] = 1;
    }
    Int::<N>::from_limbs(limbs)
}

fn inputs<const N: usize>() -> Vec<PairIn<N>> {
    let mut s = 0xC0FF_EE00_5151_5151_u64 ^ ((N as u64) << 16);
    let full = N;
    let half = N.div_ceil(2);
    let quarter = N.div_ceil(4);
    vec![
        PairIn { label: "full", a: synth::<N>(&mut s, full), b: synth::<N>(&mut s, full) },
        PairIn { label: "half", a: synth::<N>(&mut s, half), b: synth::<N>(&mut s, half) },
        PairIn { label: "quarter", a: synth::<N>(&mut s, quarter), b: synth::<N>(&mut s, quarter) },
        PairIn { label: "single", a: synth::<N>(&mut s, 1), b: synth::<N>(&mut s, 1) },
        PairIn { label: "mixed", a: synth::<N>(&mut s, full), b: synth::<N>(&mut s, 1) },
    ]
}

/// One width cell: schoolbook (the wired kernel) vs comba (the kept
/// alternative). `$n` is the storage limb count; `$sb`/`$cb` the concrete-`N`
/// `__bench_internals` wrappers for that width.
macro_rules! sum_sq_cell {
    ($c:expr, $label:literal, $n:literal, $sb:path, $cb:path) => {{
        let sb = |i: PairIn<$n>| -> Option<Int<$n>> { $sb(i.a, i.b) };
        let cb = |i: PairIn<$n>| -> Option<Int<$n>> { $cb(i.a, i.b) };
        for i in inputs::<$n>() {
            assert_eq!(
                sb(i.clone()),
                cb(i.clone()),
                "sum_sq schoolbook vs comba {} {}",
                $label,
                i.label
            );
        }
        compare_all(
            $c,
            &format!("sum_sq/{}", $label),
            |i: &PairIn<$n>| i.label.to_string(),
            inputs::<$n>(),
            vec![
                ("schoolbook", Box::new(sb) as Box<dyn Fn(PairIn<$n>) -> Option<Int<$n>>>),
                ("comba", Box::new(cb)),
            ],
        );
    }};
}

fn bench(c: &mut Criterion) {
    use decimal_scaled::__bench_internals as bi;
    sum_sq_cell!(c, "n2_D38", 2, bi::sum_sq_sb_n2, bi::sum_sq_cb_n2);
    sum_sq_cell!(c, "n3_D57", 3, bi::sum_sq_sb_n3, bi::sum_sq_cb_n3);
    sum_sq_cell!(c, "n4_D76", 4, bi::sum_sq_sb_n4, bi::sum_sq_cb_n4);
    sum_sq_cell!(c, "n6_D115", 6, bi::sum_sq_sb_n6, bi::sum_sq_cb_n6);
    sum_sq_cell!(c, "n8_D153", 8, bi::sum_sq_sb_n8, bi::sum_sq_cb_n8);
    sum_sq_cell!(c, "n12_D230", 12, bi::sum_sq_sb_n12, bi::sum_sq_cb_n12);
    sum_sq_cell!(c, "n16_D307", 16, bi::sum_sq_sb_n16, bi::sum_sq_cb_n16);
    sum_sq_cell!(c, "n24_D462", 24, bi::sum_sq_sb_n24, bi::sum_sq_cb_n24);
    sum_sq_cell!(c, "n32_D616", 32, bi::sum_sq_sb_n32, bi::sum_sq_cb_n32);
    sum_sq_cell!(c, "n48_D924", 48, bi::sum_sq_sb_n48, bi::sum_sq_cb_n48);
    sum_sq_cell!(c, "n64_D1232", 64, bi::sum_sq_sb_n64, bi::sum_sq_cb_n64);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
