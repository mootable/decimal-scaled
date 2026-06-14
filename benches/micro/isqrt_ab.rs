// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam N-way A/B for the integer isqrt policy
//! (`src/int/policy/isqrt.rs`).
//!
//! Decision being mapped: per storage width `N` and radicand magnitude,
//! which of the registered algorithms is the FASTEST that is also VALID
//! (bit-identical to the reference floor square root):
//!
//! - `native`     -> `isqrt_native_fixed` (hardware `u64::isqrt` for `N == 1`,
//!   `u128::isqrt` for `N == 2`; for `N >= 3` the const wrapper falls through
//!   to Newton, so the `native` arm is only a *distinct* candidate at
//!   `N <= 2`).
//! - `newton`     -> `isqrt_newton_slice` (width-agnostic Newton iteration:
//!   shared `algo_x_support::seed::sqrt_seed` f64 over-estimate seed + Knuth
//!   `div_rem` per iteration). Generic over all `N`.
//! - `schoolbook` -> `isqrt_schoolbook_slice` (two-bits-at-a-time bitwise
//!   digit-by-digit; pure integer, no divide, no float seed). Generic over
//!   all `N`; the registered reference baseline.
//!
//! The continuous axis is radicand MAGNITUDE / shape (int has no scale axis).
//! Per width we sweep magnitude classes that exercise the seed + the
//! floor-correction logic:
//!
//! - `full`   — a non-perfect-square radicand spanning the full `N` limbs
//!   (the representative wide case).
//! - `psq`    — an EXACT perfect square `k²` (k spans the full half-width):
//!   the root is integral, the worst case for any "is it exactly k?" check.
//! - `psq_m1` — `k² - 1` (root `k-1`): the floor-correction worst case just
//!   below a perfect square.
//! - `psq_p1` — `k² + 1` (root `k`): just above a perfect square.
//! - `half`   — a radicand occupying only the low half of the storage (root
//!   ~quarter width) — exercises the seed at a smaller magnitude.
//!
//! Every candidate is asserted bit-identical to `schoolbook` (the reference)
//! across the full magnitude spread at every width BEFORE timing — the
//! validity wall: a candidate is eligible only where it produces the exact
//! floor root. The harness `black_box`-guards inputs/outputs.
//!
//! Run: `cargo bench --features "wide x-wide xx-wide bench-alt" --bench isqrt_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    isqrt_karatsuba_slice, isqrt_native_fixed, isqrt_newton_slice, isqrt_schoolbook_slice,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// A radicand `n` (little-endian u64 magnitude) at a labelled magnitude class.
#[derive(Clone)]
struct RIn {
    label: &'static str,
    n: Vec<u64>,
}

fn mix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Multiply little-endian `a` by little-endian `b` into a `2*len`-limb
/// product (schoolbook; used only to build exact `k²` test radicands here).
fn mul_le(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0u64; a.len() + b.len()];
    for (i, &ai) in a.iter().enumerate() {
        let mut carry: u128 = 0;
        for (j, &bj) in b.iter().enumerate() {
            let t = ai as u128 * bj as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + b.len()] = out[i + b.len()].wrapping_add(carry as u64);
    }
    out
}

/// Decrement a little-endian magnitude by 1 (assumes value >= 1).
fn dec1(v: &mut [u64]) {
    for limb in v.iter_mut() {
        let (d, b) = limb.overflowing_sub(1);
        *limb = d;
        if !b {
            break;
        }
    }
}

/// Increment a little-endian magnitude by 1 (no overflow into a new limb in
/// our use — k² always leaves headroom).
fn inc1(v: &mut [u64]) {
    for limb in v.iter_mut() {
        let (s, c) = limb.overflowing_add(1);
        *limb = s;
        if !c {
            break;
        }
    }
}

/// Build the magnitude spread for a radicand of `limbs` u64 limbs.
fn inputs(limbs: usize) -> Vec<RIn> {
    let mut s = 0xC0FF_EE00_1234_5678_u64 ^ (limbs as u64);

    // A non-perfect-square full-width radicand (top limb non-zero).
    let full = {
        let mut v = vec![0u64; limbs];
        for limb in v.iter_mut() {
            *limb = mix(&mut s);
        }
        if v[limbs - 1] == 0 {
            v[limbs - 1] = 0x8000_0000_0000_0001;
        }
        v
    };

    // A radicand using only the low half of the storage.
    let half = {
        let mut v = vec![0u64; limbs];
        let hi = limbs.div_ceil(2);
        for k in 0..hi {
            v[k] = mix(&mut s);
        }
        if v[hi - 1] == 0 {
            v[hi - 1] = 0x8000_0000_0000_0001;
        }
        v
    };

    // Build a root `k` of ~ceil(limbs/2) limbs, then k² (≈ `limbs` limbs) for
    // the perfect-square edge cases. The product is 2*klimbs; we keep it at
    // `limbs` width (klimbs = ceil(limbs/2) gives a 2*klimbs >= limbs product
    // — slice/pad to exactly `limbs`).
    let klimbs = limbs.div_ceil(2);
    let mut k = vec![0u64; klimbs];
    for limb in k.iter_mut() {
        *limb = mix(&mut s);
    }
    if k[klimbs - 1] == 0 {
        k[klimbs - 1] = 0x8000_0000_0000_0001;
    }
    // Clear k's top bit so k² fits within `limbs` limbs comfortably.
    k[klimbs - 1] &= (1u64 << 63) - 1;
    if k[klimbs - 1] == 0 {
        k[klimbs - 1] = 1;
    }
    let ksq_full = mul_le(&k, &k); // 2*klimbs limbs
    let to_n = |v: &[u64]| -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        let take = v.len().min(limbs);
        out[..take].copy_from_slice(&v[..take]);
        out
    };
    let psq = to_n(&ksq_full);
    let mut psq_m1 = psq.clone();
    dec1(&mut psq_m1);
    let mut psq_p1 = psq.clone();
    inc1(&mut psq_p1);

    vec![
        RIn { label: "full", n: full },
        RIn { label: "half", n: half },
        RIn { label: "psq", n: psq },
        RIn { label: "psq_m1", n: psq_m1 },
        RIn { label: "psq_p1", n: psq_p1 },
    ]
}

fn run_newton(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    isqrt_newton_slice(&i.n, &mut out);
    out
}
fn run_schoolbook(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    isqrt_schoolbook_slice(&i.n, &mut out);
    out
}
fn run_karatsuba(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    isqrt_karatsuba_slice(&i.n, &mut out);
    out
}

/// Generic (N >= 3) cell: newton vs karatsuba vs schoolbook (native is not a
/// distinct candidate — the const wrapper routes N >= 3 to Newton).
fn cell_generic(c: &mut Criterion, limbs: usize, label: &str) {
    for i in inputs(limbs) {
        assert_eq!(
            run_newton(i.clone()),
            run_schoolbook(i.clone()),
            "isqrt newton vs schoolbook (reference) {label} {}",
            i.label
        );
        assert_eq!(
            run_karatsuba(i.clone()),
            run_schoolbook(i.clone()),
            "isqrt karatsuba vs schoolbook (reference) {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("isqrt/{label}"),
        |i: &RIn| i.label.to_string(),
        inputs(limbs),
        vec![
            ("newton", Box::new(run_newton) as Box<dyn Fn(RIn) -> Vec<u64>>),
            ("karatsuba", Box::new(run_karatsuba)),
            ("schoolbook", Box::new(run_schoolbook)),
        ],
    );
}

/// Narrow cell (`N <= 2`): native (hardware) vs newton vs schoolbook.
fn cell_native<const N: usize>(c: &mut Criterion, label: &str) {
    let run_native = |i: RIn| -> Vec<u64> {
        let mut arr = [0u64; N];
        let mut out = [0u64; N];
        arr.copy_from_slice(&i.n[..N]);
        isqrt_native_fixed::<N>(&arr, &mut out);
        out.to_vec()
    };
    for i in inputs(N) {
        assert_eq!(run_native(i.clone()), run_schoolbook(i.clone()), "isqrt native vs ref {label} {}", i.label);
        assert_eq!(run_newton(i.clone()), run_schoolbook(i.clone()), "isqrt newton vs ref {label} {}", i.label);
    }
    compare_all(
        c,
        &format!("isqrt/{label}"),
        |i: &RIn| i.label.to_string(),
        inputs(N),
        vec![
            ("native", Box::new(run_native) as Box<dyn Fn(RIn) -> Vec<u64>>),
            ("newton", Box::new(run_newton)),
            ("schoolbook", Box::new(run_schoolbook)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    // Narrow tiers where `native` is a distinct hardware candidate.
    cell_native::<1>(c, "n1_D18");
    cell_native::<2>(c, "n2_D38");
    // Generic tiers: full width sweep N in {3,4,6,8,12,16,24,32,48,64}.
    cell_generic(c, 3, "n3_D57");
    cell_generic(c, 4, "n4_D76");
    cell_generic(c, 6, "n6_D115");
    cell_generic(c, 8, "n8_D153");
    cell_generic(c, 12, "n12_D230");
    cell_generic(c, 16, "n16_D307");
    cell_generic(c, 24, "n24_D462");
    cell_generic(c, 32, "n32_D616");
    cell_generic(c, 48, "n48_D924");
    cell_generic(c, 64, "n64_D1232");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
