// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam N-way A/B for the integer icbrt policy
//! (`src/int/policy/icbrt.rs`).
//!
//! Decision being mapped: per storage width `N` and radicand magnitude,
//! which of the registered algorithms is the FASTEST that is also VALID
//! (bit-identical to the reference floor cube root):
//!
//! - `newton`     -> `icbrt_newton_slice` (width-agnostic Brent–Zimmermann
//!   Newton iteration: shared `algo_x_support::seed::cbrt_seed` f64-`cbrt`
//!   over-estimate seed + the `s_new = (2·s + n/s²)/3` recurrence, one divide
//!   per step). Generic over all `N`. This is the wired kernel for EVERY `N`
//!   — the `Native` policy arm (`N ∈ {1,2}`) also delegates here, so newton
//!   is the single live candidate at every width.
//! - `schoolbook` -> `icbrt_schoolbook_slice` (bit-by-bit restoring cube
//!   root; pure integer, no divide, no float seed). Generic over all `N`;
//!   the registered reference baseline.
//!
//! The continuous axis is radicand MAGNITUDE / shape (int has no scale axis).
//! Per width we sweep magnitude classes that exercise the seed + the
//! floor-correction logic:
//!
//! - `full`   — a non-perfect-cube radicand spanning the full `N` limbs (the
//!   representative wide case).
//! - `pcube`    — an EXACT perfect cube `k³` (k spans ~ceil(limbs/3)): the
//!   root is integral, the worst case for the floor-correction.
//! - `pcube_m1` — `k³ - 1` (root `k-1`): the floor-correction worst case just
//!   below a perfect cube.
//! - `pcube_p1` — `k³ + 1` (root `k`): just above a perfect cube.
//! - `half`   — a radicand occupying only the low half of the storage —
//!   exercises the seed at a smaller magnitude.
//!
//! Every candidate is asserted bit-identical to `schoolbook` (the reference)
//! across the full magnitude spread at every width BEFORE timing — the
//! validity wall: a candidate is eligible only where it produces the exact
//! floor root. The harness `black_box`-guards inputs/outputs.
//!
//! Run: `cargo bench --features "wide x-wide xx-wide bench-alt" --bench icbrt_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    icbrt_newton_recip_slice, icbrt_newton_slice, icbrt_schoolbook_slice,
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

/// Multiply little-endian `a` by little-endian `b` into an `a.len()+b.len()`-limb
/// product (schoolbook; used here to build exact `k³` test radicands).
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

/// Increment a little-endian magnitude by 1 (the k³ radicands leave headroom).
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
    let mut s = 0xC0FF_EE00_1234_5678_u64 ^ ((limbs as u64) << 8);

    // A non-perfect-cube full-width radicand (top limb non-zero).
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

    // Build a root `k` of ~ceil(limbs/3) limbs, then k³ (≈ `limbs` limbs) for
    // the perfect-cube edge cases. k³ has at most 3*klimbs limbs; with
    // klimbs = ceil(limbs/3) the product covers `limbs` width — slice/pad to
    // exactly `limbs`. Clear k's top bits so k³ does not overflow `limbs`.
    let klimbs = limbs.div_ceil(3).max(1);
    let mut k = vec![0u64; klimbs];
    for limb in k.iter_mut() {
        *limb = mix(&mut s);
    }
    if k[klimbs - 1] == 0 {
        k[klimbs - 1] = 0x0000_0000_0000_0001;
    }
    // Keep k's top limb small so k³ stays within `limbs` limbs comfortably
    // (top limb < 2^21 ⇒ cube of the top limb < 2^63, no spill past 3·klimbs).
    k[klimbs - 1] &= (1u64 << 20) - 1;
    if k[klimbs - 1] == 0 {
        k[klimbs - 1] = 1;
    }
    let ksq = mul_le(&k, &k); // 2*klimbs limbs
    let kcube_full = mul_le(&ksq, &k); // 3*klimbs limbs
    let to_n = |v: &[u64]| -> Vec<u64> {
        let mut out = vec![0u64; limbs];
        let take = v.len().min(limbs);
        out[..take].copy_from_slice(&v[..take]);
        out
    };
    let pcube = to_n(&kcube_full);
    let mut pcube_m1 = pcube.clone();
    dec1(&mut pcube_m1);
    let mut pcube_p1 = pcube.clone();
    inc1(&mut pcube_p1);

    vec![
        RIn { label: "full", n: full },
        RIn { label: "half", n: half },
        RIn { label: "pcube", n: pcube },
        RIn { label: "pcube_m1", n: pcube_m1 },
        RIn { label: "pcube_p1", n: pcube_p1 },
    ]
}

fn run_newton(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    icbrt_newton_slice(&i.n, &mut out);
    out
}
fn run_schoolbook(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    icbrt_schoolbook_slice(&i.n, &mut out);
    out
}
fn run_newton_recip(i: RIn) -> Vec<u64> {
    let mut out = vec![0u64; i.n.len()];
    icbrt_newton_recip_slice(&i.n, &mut out);
    out
}

/// One width cell: newton (the wired kernel for every `N`, incl. the `N∈{1,2}`
/// `Native` policy arm which delegates to it) vs newton_recip (the
/// division-free candidate) vs schoolbook (the reference).
fn cell(c: &mut Criterion, limbs: usize, label: &str) {
    for i in inputs(limbs) {
        assert_eq!(
            run_newton(i.clone()),
            run_schoolbook(i.clone()),
            "icbrt newton vs schoolbook (reference) {label} {}",
            i.label
        );
        assert_eq!(
            run_newton_recip(i.clone()),
            run_schoolbook(i.clone()),
            "icbrt newton_recip vs schoolbook (reference) {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("icbrt/{label}"),
        |i: &RIn| i.label.to_string(),
        inputs(limbs),
        vec![
            ("newton", Box::new(run_newton) as Box<dyn Fn(RIn) -> Vec<u64>>),
            ("newton_recip", Box::new(run_newton_recip)),
            ("schoolbook", Box::new(run_schoolbook)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    // Full width sweep N in {1,2,3,4,6,8,12,16,24,32,48,64}.
    cell(c, 1, "n1_D18");
    cell(c, 2, "n2_D38");
    cell(c, 3, "n3_D57");
    cell(c, 4, "n4_D76");
    cell(c, 6, "n6_D115");
    cell(c, 8, "n8_D153");
    cell(c, 12, "n12_D230");
    cell(c, 16, "n16_D307");
    cell(c, 24, "n24_D462");
    cell(c, 32, "n32_D616");
    cell(c, 48, "n48_D924");
    cell(c, 64, "n64_D1232");
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
