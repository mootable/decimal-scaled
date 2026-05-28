// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the INT-tier `neg` policy — the wide-tier
//! `neg_D462`/`D616`/`D924`/`D1232` regression recovery.
//!
//! Candidates (all bit-identical across the input spread, asserted
//! before timing):
//!
//! - `fused_split` — the ROUTED kernel: limb 0 is special-cased
//!                   (`!a[0] + 1`) and limbs 1..N only carry-propagate
//!                   when `a[0] == MAX`; the common path reduces to
//!                   independent `!a[i]` writes (vectorisable, no
//!                   cross-limb dependency chain).
//! - `two_pass`    — the previous routed shape: NOT loop into `out[N]`,
//!                   then a full-width `add_assign_fixed(out, [1, 0, …,
//!                   0])` over a second stack array. Reference baseline.
//! - `fused_open`  — single-pass open-coded NOT + carry-prop `+1`
//!                   (dependent chain through every limb). Candidate
//!                   that loses to `fused_split` at wide widths because
//!                   the dependent chain blocks vectorisation on the
//!                   common path.
//!
//! A/B confirms `fused_split` wins at every bbc-target wide tier
//! (D462/D616/D924/D1232) by 1.40x-1.83x — generic, one kernel for
//! every `N`.
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench neg_kernel_ab`

use criterion::Criterion;
use decimal_scaled::Int;
use decimal_scaled::__bench_internals::{neg_fused_open, neg_fused_split, neg_two_pass};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use crate::ab_sweep as decimal_scaled_ab_sweep;
use ab_microbench::{compare_all, micro_criterion};

/// Deterministic limb fill for an `Int<N>`. `seed` rotates per-limb so the
/// top limb is also non-zero (so the value is wide).
fn synth<const N: usize>(seed: u64) -> Int<N> {
    let mut mag = [0u64; N];
    for i in 0..N {
        mag[i] = seed
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .wrapping_add(i as u64 * 0x1357_9BDF)
            ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
    }
    Int::<N>::from_limbs(mag)
}

#[derive(Clone)]
struct NegIn<const N: usize> {
    label: &'static str,
    x: Int<N>,
}

fn neg_inputs<const N: usize>() -> Vec<NegIn<N>> {
    vec![
        NegIn { label: "low", x: synth::<N>(7) },
        NegIn { label: "mid", x: synth::<N>(1009) },
        NegIn { label: "high", x: synth::<N>(7919) },
        // limb-0 = u64::MAX in storage: carry must propagate from limb 0
        // — exercises the worst case for any early-exit shape.
        NegIn { label: "carry_chain", x: {
            let mut mag = [u64::MAX; N];
            // perturb the top limb so it is not literally MIN
            if N > 0 { mag[N - 1] = 0x7FFF_FFFF_FFFF_FFFF; }
            Int::<N>::from_limbs(mag)
        } },
        // tiny: only limb 0 non-zero — minimal magnitude.
        NegIn { label: "tiny", x: {
            let mut mag = [0u64; N];
            if N > 0 { mag[0] = 2; }
            Int::<N>::from_limbs(mag)
        } },
        // half_wide: lower N/2 limbs nonzero, upper half zero. This is
        // close to what `D::try_from(2)` produces at a wide scale (e.g.
        // 2 * 10^(SCALE/2)) — the bbc `full_matrix` /neg cell shape.
        NegIn { label: "half_wide", x: {
            let mut mag = [0u64; N];
            let half = N / 2;
            for i in 0..half {
                mag[i] = 0x9E37_79B9_7F4A_7C15u64
                    .wrapping_mul((i as u64).wrapping_add(1));
            }
            Int::<N>::from_limbs(mag)
        } },
    ]
}

fn bench_neg<const N: usize>(c: &mut Criterion, label: &str) {
    // Correctness gate: every candidate bit-identical across the spread.
    for i in neg_inputs::<N>() {
        let t = neg_two_pass::<N>(i.x);
        let fo = neg_fused_open::<N>(i.x);
        let fs = neg_fused_split::<N>(i.x);
        assert_eq!(
            fo.as_limbs(),
            t.as_limbs(),
            "neg fused_open vs two_pass {label} {}",
            i.label
        );
        assert_eq!(
            fs.as_limbs(),
            t.as_limbs(),
            "neg fused_split vs two_pass {label} {}",
            i.label
        );
    }
    compare_all(
        c,
        &format!("neg/{label}"),
        |i: &NegIn<N>| i.label.to_string(),
        neg_inputs::<N>(),
        vec![
            (
                "fused_split",
                (|i: NegIn<N>| neg_fused_split::<N>(i.x)) as fn(NegIn<N>) -> Int<N>,
            ),
            ("two_pass", |i: NegIn<N>| neg_two_pass::<N>(i.x)),
            ("fused_open", |i: NegIn<N>| neg_fused_open::<N>(i.x)),
        ],
    );
}

fn bench(c: &mut Criterion) {
    decimal_scaled_ab_sweep!(c =>
        Int<1>  => |c: &mut Criterion| bench_neg::<1>(c,  "Int64_D18"),
        Int<2>  => |c: &mut Criterion| bench_neg::<2>(c,  "Int128_D38"),
        Int<3>  => |c: &mut Criterion| bench_neg::<3>(c,  "Int192_D57"),
        Int<4>  => |c: &mut Criterion| bench_neg::<4>(c,  "Int256_D76"),
        Int<6>  => |c: &mut Criterion| bench_neg::<6>(c,  "Int384_D115"),
        Int<8>  => |c: &mut Criterion| bench_neg::<8>(c,  "Int512_D153"),
        Int<12> => |c: &mut Criterion| bench_neg::<12>(c, "Int768_D230"),
        Int<16> => |c: &mut Criterion| bench_neg::<16>(c, "Int1024_D307"),
        Int<24> => |c: &mut Criterion| bench_neg::<24>(c, "Int1536_D462"),
        Int<32> => |c: &mut Criterion| bench_neg::<32>(c, "Int2048_D616"),
        Int<48> => |c: &mut Criterion| bench_neg::<48>(c, "Int3072_D924"),
        Int<64> => |c: &mut Criterion| bench_neg::<64>(c, "Int4096_D1232"),
    );
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
