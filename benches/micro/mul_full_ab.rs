// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! N-way A/B for the `int::policy::mul` FULL equal-length product.
//!
//! Ranks every candidate the production `widen_mul` dispatcher chooses
//! between, across width `N` and (for the schoolbook arm) the `LimbSize`
//! axis (`u64` vs packed `u128`):
//!
//! - `slice`     -> `mul_schoolbook` (base-2^64 slice — the historic kernel,
//!   the bit-identicality reference).
//! - `u64`       -> `mul_full_limb::<N, u64>` (fixed-width unrolled, the
//!   current Schoolbook arm for odd / u64-chosen N).
//! - `u128`      -> `mul_full_limb::<N, u128>` (u128-packed, even N only — the
//!   current even-N Schoolbook arm).
//! - `kara`      -> `mul_karatsuba_forced` (one forced recursion level at each
//!   width — the exact shape the dispatcher emits when the threshold == N).
//!
//! Two crossovers are localized from this map:
//!   (a) schoolbook -> Karatsuba (is `KARATSUBA_THRESHOLD = 64` right?), and
//!   (b) the u64 -> u128 `LimbSize` crossover per even N (does u128 win at
//!       EVERY even N, or do small even N prefer u64?).
//!
//! A candidate is eligible at a cell only where it is bit-identical to
//! `slice` (the full product is exact — every arm must agree; any mismatch
//! is flagged by the pre-timing `assert_eq!`).
//!
//! Run with:
//! `cargo bench --features "wide x-wide xx-wide bench-alt" --bench mul_full_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{
    mul_full_u128_12, mul_full_u128_16, mul_full_u128_2, mul_full_u128_24, mul_full_u128_32,
    mul_full_u128_4, mul_full_u128_48, mul_full_u128_6, mul_full_u128_64, mul_full_u128_8,
    mul_full_u64_12, mul_full_u64_16, mul_full_u64_2, mul_full_u64_24, mul_full_u64_3,
    mul_full_u64_32, mul_full_u64_4, mul_full_u64_48, mul_full_u64_6, mul_full_u64_64,
    mul_full_u64_8, mul_karatsuba_forced, mul_slice,
    mul_kara_u64_4, mul_kara_u128_4,
    mul_kara_u64_6, mul_kara_u128_6,
    mul_kara_u64_8, mul_kara_u128_8,
    mul_kara_u64_12, mul_kara_u128_12,
    mul_kara_u64_16, mul_kara_u128_16,
    mul_kara_u64_24, mul_kara_u128_24,
    mul_kara_u64_32, mul_kara_u128_32,
    mul_kara_u64_48, mul_kara_u128_48,
    mul_kara_u64_64, mul_kara_u128_64,
};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

/// A seeded equal-length operand pair plus a label for its `BenchmarkId`.
#[derive(Clone)]
struct Operands<const N: usize> {
    label: &'static str,
    a: [u64; N],
    b: [u64; N],
}

/// Deterministic limb fill (splitmix64 over a seeded counter).
fn fill<const N: usize>(seed: u64) -> [u64; N] {
    let mut out = [0u64; N];
    let mut state = seed;
    for x in out.iter_mut() {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        *x = z ^ (z >> 31);
    }
    out
}

/// Low / mid / high seeded operand pairs at width `N`.
fn operand_set<const N: usize>() -> Vec<Operands<N>> {
    vec![
        Operands { label: "low", a: fill::<N>(3), b: fill::<N>(5) },
        Operands { label: "mid", a: fill::<N>(7), b: fill::<N>(13) },
        Operands { label: "high", a: fill::<N>(1009), b: fill::<N>(7919) },
    ]
}

/// Slice schoolbook reference: allocate + zero a `2N` output, multiply.
fn slice_run<const N: usize>(ops: Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_slice(&ops.a, &ops.b, &mut out);
    out
}

/// Karatsuba with one forced recursion level (threshold == N), u64 slice base.
fn kara_run<const N: usize>(ops: Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_karatsuba_forced(&ops.a, &ops.b, &mut out, N);
    out
}


/// Common reference product (the bit-identicality oracle) for assertions.
fn reference<const N: usize>(ops: &Operands<N>) -> Vec<u64> {
    let mut out = vec![0u64; 2 * N];
    mul_slice(&ops.a, &ops.b, &mut out);
    out
}

/// Karatsuba is only a candidate at or above the policy's recursion floor
/// (`KARATSUBA_THRESHOLD >= 4`): the z1 sum-product only strictly shrinks
/// below `n` once `n >= 4`, and the fixed scratch carve is sized for that.
/// So N=2/3 omit the `kara` arm (it would never be wired there anyway).
const KARA_FLOOR: usize = 4;

/// N-way comparison at one ODD width (no u128 arm — packing needs even N).
fn compare_odd<const N: usize>(
    c: &mut Criterion,
    width_label: &str,
    u64_run: fn(Operands<N>) -> Vec<u64>,
) {
    let mut arms: Vec<(&str, fn(Operands<N>) -> Vec<u64>)> =
        vec![("slice", slice_run::<N>), ("u64", u64_run)];
    for ops in operand_set::<N>() {
        let r = reference::<N>(&ops);
        assert_eq!(u64_run(ops.clone()), r, "u64 != slice at N={N} ({})", ops.label);
        if N >= KARA_FLOOR {
            assert_eq!(kara_run::<N>(ops.clone()), r, "kara != slice at N={N} ({})", ops.label);
        }
    }
    if N >= KARA_FLOOR {
        arms.push(("kara", kara_run::<N>));
    }
    compare_all(
        c,
        &format!("mul_full/{width_label}"),
        |ops: &Operands<N>| ops.label.to_string(),
        operand_set::<N>(),
        arms,
    );
}

/// N-way comparison at one EVEN width — adds the `u128`-packed arm.
fn compare_even<const N: usize>(
    c: &mut Criterion,
    width_label: &str,
    u64_run: fn(Operands<N>) -> Vec<u64>,
    u128_run: fn(Operands<N>) -> Vec<u64>,
) {
    let mut arms: Vec<(&str, fn(Operands<N>) -> Vec<u64>)> =
        vec![("slice", slice_run::<N>), ("u64", u64_run), ("u128", u128_run)];
    for ops in operand_set::<N>() {
        let r = reference::<N>(&ops);
        assert_eq!(u64_run(ops.clone()), r, "u64 != slice at N={N} ({})", ops.label);
        assert_eq!(u128_run(ops.clone()), r, "u128 != slice at N={N} ({})", ops.label);
        if N >= KARA_FLOOR {
            assert_eq!(kara_run::<N>(ops.clone()), r, "kara != slice at N={N} ({})", ops.label);
        }
    }
    if N >= KARA_FLOOR {
        arms.push(("kara", kara_run::<N>));
    }
    compare_all(
        c,
        &format!("mul_full/{width_label}"),
        |ops: &Operands<N>| ops.label.to_string(),
        operand_set::<N>(),
        arms,
    );
}

/// Adapt a fixed-array wrapper `fn(&[u64;N],&[u64;N],&mut [u64])` into the
/// `Operands<N> -> Vec<u64>` shape the harness times, allocating + returning
/// the `2N` product so nothing is const-folded away.
macro_rules! adapt {
    ($wrap:path, $N:literal) => {{
        fn run(ops: Operands<$N>) -> Vec<u64> {
            let mut out = vec![0u64; 2 * $N];
            $wrap(&ops.a, &ops.b, &mut out);
            out
        }
        run as fn(Operands<$N>) -> Vec<u64>
    }};
}

/// Sweep the full width axis. Coarse `{2,4,8,16,32,64}` plus the
/// bisection points `{3,6,12,24,48}` that localize both crossovers (the
/// u64/u128 LimbSize edge and the schoolbook/Karatsuba edge).
fn bench_mul_full(c: &mut Criterion) {
    compare_even::<2>(c, "Int128", adapt!(mul_full_u64_2, 2), adapt!(mul_full_u128_2, 2));
    compare_odd::<3>(c, "Int192", adapt!(mul_full_u64_3, 3));
    compare_even::<4>(c, "Int256", adapt!(mul_full_u64_4, 4), adapt!(mul_full_u128_4, 4));
    compare_even::<6>(c, "Int384", adapt!(mul_full_u64_6, 6), adapt!(mul_full_u128_6, 6));
    compare_even::<8>(c, "Int512", adapt!(mul_full_u64_8, 8), adapt!(mul_full_u128_8, 8));
    compare_even::<12>(c, "Int768", adapt!(mul_full_u64_12, 12), adapt!(mul_full_u128_12, 12));
    compare_even::<16>(c, "Int1024", adapt!(mul_full_u64_16, 16), adapt!(mul_full_u128_16, 16));
    compare_even::<24>(c, "Int1536", adapt!(mul_full_u64_24, 24), adapt!(mul_full_u128_24, 24));
    compare_even::<32>(c, "Int2048", adapt!(mul_full_u64_32, 32), adapt!(mul_full_u128_32, 32));
    compare_even::<48>(c, "Int3072", adapt!(mul_full_u64_48, 48), adapt!(mul_full_u128_48, 48));
    compare_even::<64>(c, "Int4096", adapt!(mul_full_u64_64, 64), adapt!(mul_full_u128_64, 64));
}



/// N-way Karatsuba Limb-size A/B: kara_u64 vs kara_u128 at each even width.
/// Both arms bit-identical to the slice reference (asserted before timing).
/// Groups: kara_limb/<width>.
fn bench_kara_limb(c: &mut Criterion) {
    // N=4
    {
        for ops in operand_set::<4>() {
            let r = reference::<4>(&ops);
            assert_eq!(adapt!(mul_kara_u64_4, 4)(ops.clone()), r, "kara_u64 != slice N=4");
            assert_eq!(adapt!(mul_kara_u128_4, 4)(ops.clone()), r, "kara_u128 != slice N=4");
        }
        compare_all(
            c,
            "kara_limb/Int256",
            |ops: &Operands<4>| ops.label.to_string(),
            operand_set::<4>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_4, 4) as fn(Operands<4>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_4, 4) as fn(Operands<4>) -> Vec<u64>),
            ],
        );
    }
    // N=6
    {
        for ops in operand_set::<6>() {
            let r = reference::<6>(&ops);
            assert_eq!(adapt!(mul_kara_u64_6, 6)(ops.clone()), r, "kara_u64 != slice N=6");
            assert_eq!(adapt!(mul_kara_u128_6, 6)(ops.clone()), r, "kara_u128 != slice N=6");
        }
        compare_all(
            c,
            "kara_limb/Int384",
            |ops: &Operands<6>| ops.label.to_string(),
            operand_set::<6>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_6, 6) as fn(Operands<6>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_6, 6) as fn(Operands<6>) -> Vec<u64>),
            ],
        );
    }
    // N=8
    {
        for ops in operand_set::<8>() {
            let r = reference::<8>(&ops);
            assert_eq!(adapt!(mul_kara_u64_8, 8)(ops.clone()), r, "kara_u64 != slice N=8");
            assert_eq!(adapt!(mul_kara_u128_8, 8)(ops.clone()), r, "kara_u128 != slice N=8");
        }
        compare_all(
            c,
            "kara_limb/Int512",
            |ops: &Operands<8>| ops.label.to_string(),
            operand_set::<8>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_8, 8) as fn(Operands<8>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_8, 8) as fn(Operands<8>) -> Vec<u64>),
            ],
        );
    }
    // N=12
    {
        for ops in operand_set::<12>() {
            let r = reference::<12>(&ops);
            assert_eq!(adapt!(mul_kara_u64_12, 12)(ops.clone()), r, "kara_u64 != slice N=12");
            assert_eq!(adapt!(mul_kara_u128_12, 12)(ops.clone()), r, "kara_u128 != slice N=12");
        }
        compare_all(
            c,
            "kara_limb/Int768",
            |ops: &Operands<12>| ops.label.to_string(),
            operand_set::<12>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_12, 12) as fn(Operands<12>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_12, 12) as fn(Operands<12>) -> Vec<u64>),
            ],
        );
    }
    // N=16
    {
        for ops in operand_set::<16>() {
            let r = reference::<16>(&ops);
            assert_eq!(adapt!(mul_kara_u64_16, 16)(ops.clone()), r, "kara_u64 != slice N=16");
            assert_eq!(adapt!(mul_kara_u128_16, 16)(ops.clone()), r, "kara_u128 != slice N=16");
        }
        compare_all(
            c,
            "kara_limb/Int1024",
            |ops: &Operands<16>| ops.label.to_string(),
            operand_set::<16>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_16, 16) as fn(Operands<16>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_16, 16) as fn(Operands<16>) -> Vec<u64>),
            ],
        );
    }
    // N=24
    {
        for ops in operand_set::<24>() {
            let r = reference::<24>(&ops);
            assert_eq!(adapt!(mul_kara_u64_24, 24)(ops.clone()), r, "kara_u64 != slice N=24");
            assert_eq!(adapt!(mul_kara_u128_24, 24)(ops.clone()), r, "kara_u128 != slice N=24");
        }
        compare_all(
            c,
            "kara_limb/Int1536",
            |ops: &Operands<24>| ops.label.to_string(),
            operand_set::<24>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_24, 24) as fn(Operands<24>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_24, 24) as fn(Operands<24>) -> Vec<u64>),
            ],
        );
    }
    // N=32
    {
        for ops in operand_set::<32>() {
            let r = reference::<32>(&ops);
            assert_eq!(adapt!(mul_kara_u64_32, 32)(ops.clone()), r, "kara_u64 != slice N=32");
            assert_eq!(adapt!(mul_kara_u128_32, 32)(ops.clone()), r, "kara_u128 != slice N=32");
        }
        compare_all(
            c,
            "kara_limb/Int2048",
            |ops: &Operands<32>| ops.label.to_string(),
            operand_set::<32>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_32, 32) as fn(Operands<32>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_32, 32) as fn(Operands<32>) -> Vec<u64>),
            ],
        );
    }
    // N=48
    {
        for ops in operand_set::<48>() {
            let r = reference::<48>(&ops);
            assert_eq!(adapt!(mul_kara_u64_48, 48)(ops.clone()), r, "kara_u64 != slice N=48");
            assert_eq!(adapt!(mul_kara_u128_48, 48)(ops.clone()), r, "kara_u128 != slice N=48");
        }
        compare_all(
            c,
            "kara_limb/Int3072",
            |ops: &Operands<48>| ops.label.to_string(),
            operand_set::<48>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_48, 48) as fn(Operands<48>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_48, 48) as fn(Operands<48>) -> Vec<u64>),
            ],
        );
    }
    // N=64
    {
        for ops in operand_set::<64>() {
            let r = reference::<64>(&ops);
            assert_eq!(adapt!(mul_kara_u64_64, 64)(ops.clone()), r, "kara_u64 != slice N=64");
            assert_eq!(adapt!(mul_kara_u128_64, 64)(ops.clone()), r, "kara_u128 != slice N=64");
        }
        compare_all(
            c,
            "kara_limb/Int4096",
            |ops: &Operands<64>| ops.label.to_string(),
            operand_set::<64>(),
            vec![
                ("kara_u64",  adapt!(mul_kara_u64_64, 64) as fn(Operands<64>) -> Vec<u64>),
                ("kara_u128", adapt!(mul_kara_u128_64, 64) as fn(Operands<64>) -> Vec<u64>),
            ],
        );
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_mul_full(&mut c);
    bench_kara_limb(&mut c);
    c.final_summary();
}
