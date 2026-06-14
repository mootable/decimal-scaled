// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Dispatch-seam A/B for the integer hypot policy (`src/int/policy/hypot.rs`).
//!
//! Decision being modelled: per storage width `N`, should `hypot` route to the
//! generic Pythagoras path (form the radicand `a² + b²` in scratch, floor root
//! via the Newton slice `isqrt` with a per-iteration multi-precision `div_rem`,
//! round) or to the native-`u128` fast path (`hypot_u128_fast`: when both
//! operands fit a single `u64` limb the radicand fits a `u128`, so it floors
//! the root in u128 with an f64 seed + exact remainder round and no
//! multi-precision divide; otherwise it falls through to `hypot_pythagoras`,
//! paying only the cheap `fit_one` guard)?
//!
//! Because the fast path is VALUE-gated (`fit_one(a) && fit_one(b)`), the
//! decision has TWO regimes per width — swept here as the magnitude/shape axis:
//!
//! - `single` — both operands fit one `u64` limb (the u128 fast branch
//!   engages). This is where `u128_fast` can win; its edge is the avoided
//!   multi-precision `isqrt` divide loop, so it should hold at every width.
//! - `two`    — both operands fit two `u64` limbs (`< 2^128`), high limb
//!   non-zero (the NEW u256 scalar fast branch engages). This is the decimal
//!   `s >= 19` slow band the old `fit_one`-only gate cliffed into Pythagoras;
//!   `u128_fast` should win here at every width >= 2.
//! - `two_low` — both operands JUST over one limb (`[2^64, 2^68)`, high limb
//!   1..16): the u256 arm's LOW band, the decimal `s >= 19` regime at small
//!   real values (the bbc `hypot D38<19>` cell's `3.0`/`4.0` operands live
//!   here). Fewer Newton iterations than `two`'s ~127-bit operands.
//! - `multi`  — operands span the full width (the fast branch falls through to
//!   Pythagoras). Here `u128_fast` adds only the `fit_one` guard, so it should
//!   be statistically TIED with `pythagoras` and never a meaningful loss.
//! - `skew`   — one operand single-limb, the other full width (fast branch
//!   falls through: the radicand of a wide × narrow pair does not fit u128).
//!
//! Two registered arms (both bit-identical across all six RoundingModes —
//! asserted before timing):
//! - `pythagoras` -> `hypot_pythagoras` (the production kernel).
//! - `u128_fast`  -> `hypot_u128_fast` (the candidate).
//!
//! Run: `cargo bench --features "wide bench-alt" --bench hypot_ab`

use criterion::Criterion;
use decimal_scaled::__bench_internals::{hypot_pythagoras, hypot_u128_fast};
use decimal_scaled::{Int, RoundingMode};

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

const ALL_MODES: [RoundingMode; 6] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
];

const MODE: RoundingMode = RoundingMode::HalfToEven;

#[derive(Clone)]
struct HIn<const N: usize> {
    label: &'static str,
    a: Int<N>,
    b: Int<N>,
}

/// Build a magnitude/shape spread for width `N`:
/// - `single`: both operands fit one u64 limb (fast-path engages).
/// - `multi`:  both operands span the full width (fast-path falls through).
/// - `skew`:   one single-limb, one full-width (fast-path falls through).
///
/// Magnitudes are deterministic, derived from a small splitmix so the radicand
/// is non-trivial (not a perfect square) and the inputs stay representative
/// across widths.
fn inputs<const N: usize>() -> Vec<HIn<N>> {
    fn mix(s: &mut u64) -> u64 {
        *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *s;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    let mut s = 0xC0FF_EE00_1234_5678_u64 ^ (N as u64);

    // A magnitude that fits a single u64 limb (clear top bit so a² + b²
    // can't sign-overflow at N == 1, keeps the value positive everywhere).
    let single = |s: &mut u64| {
        let mut out = [0u64; N];
        out[0] = mix(s) & (i64::MAX as u64);
        Int::<N>::from_limbs(out)
    };
    // A magnitude that fits TWO u64 limbs (< 2^128) with the high limb
    // non-zero -- exercises the NEW u256 scalar fast arm (the decimal `s >= 19`
    // slow band that the old `fit_one`-only gate cliffed into Pythagoras).
    // Top bit of limb 1 cleared so the radicand sum stays < 2^256.
    let two = |s: &mut u64| {
        let mut out = [0u64; N];
        out[0] = mix(s);
        if N >= 2 {
            out[1] = (mix(s) & (i64::MAX as u64)) | 1; // < 2^127, high limb non-zero
        }
        Int::<N>::from_limbs(out)
    };
    // A magnitude that JUST exceeds one u64 limb (high limb in 1..16, value in
    // [2^64, 2^68)) -- the LOW band of the u256 scalar fast arm. This is the
    // decimal `s >= 19` regime at small real values (e.g. the bbc `hypot
    // D38<19>` cell's `3.0`/`4.0` operands, raw 3e19/4e19 with high limb 1):
    // the root has ~65-68 bits, so the Newton iteration count differs from the
    // `two` shape's ~127-bit operands. A continuous region, not a point pin.
    let two_low = |s: &mut u64| {
        let mut out = [0u64; N];
        out[0] = mix(s);
        if N >= 2 {
            out[1] = (mix(s) & 0xF) | 1; // high limb 1..=15
        }
        Int::<N>::from_limbs(out)
    };
    // A magnitude that spans the FULL storage width (top limb non-zero, top
    // sign bit cleared so the operand itself is a valid positive Int<N>).
    let full = |s: &mut u64| {
        let mut out = [0u64; N];
        for k in 0..N {
            out[k] = mix(s);
        }
        out[N - 1] &= i64::MAX as u64;
        // Guarantee the top limb is non-zero so this is a genuine N-limb value.
        if out[N - 1] == 0 {
            out[N - 1] = 1;
        }
        Int::<N>::from_limbs(out)
    };

    vec![
        HIn { label: "single", a: single(&mut s), b: single(&mut s) },
        HIn { label: "two", a: two(&mut s), b: two(&mut s) },
        HIn { label: "two_low", a: two_low(&mut s), b: two_low(&mut s) },
        HIn { label: "multi", a: full(&mut s), b: full(&mut s) },
        HIn { label: "skew", a: single(&mut s), b: full(&mut s) },
    ]
}

macro_rules! hypot_cell {
    ($c:expr, $n:literal, $label:literal) => {{
        // Correctness cross-check: both arms must agree, on EVERY mode and
        // EVERY shape, before timing (the validity wall — `u128_fast` is only
        // eligible where bit-identical to `pythagoras`).
        for o in inputs::<$n>() {
            for mode in ALL_MODES {
                assert_eq!(
                    hypot_pythagoras::<$n>(o.a, o.b, mode),
                    hypot_u128_fast::<$n>(o.a, o.b, mode),
                    "hypot arms disagree at N={} ({}, {:?})",
                    $n,
                    o.label,
                    mode
                );
            }
        }
        compare_all(
            $c,
            concat!("hypot/", $label),
            |o: &HIn<$n>| o.label.to_string(),
            inputs::<$n>(),
            vec![
                (
                    "pythagoras",
                    (|o: HIn<$n>| hypot_pythagoras::<$n>(o.a, o.b, MODE))
                        as fn(HIn<$n>) -> Option<Int<$n>>,
                ),
                ("u128_fast", |o: HIn<$n>| hypot_u128_fast::<$n>(o.a, o.b, MODE)),
            ],
        );
    }};
}

fn bench_hypot(c: &mut Criterion) {
    // Width axis: N in {2,3,4,6,8,12,16,24,32,48,64}. (N == 1 / D18 is covered
    // by the narrow tier; the policy crossover of interest is N >= 2.)
    hypot_cell!(c, 2, "N2_D38");
    hypot_cell!(c, 3, "N3_D57");
    hypot_cell!(c, 4, "N4_D76");
    hypot_cell!(c, 6, "N6_D115");
    hypot_cell!(c, 8, "N8_D153");
    hypot_cell!(c, 12, "N12_D230");
    hypot_cell!(c, 16, "N16_D307");
    // The N >= 24 cells root a 2N-limb radicand through isqrt_newton, whose
    // build-max scratch is sized by the widest ENABLED tier — at the declared
    // minimum features (`wide`, MAX_WORK_N = 16, 40 limbs) a 48-limb radicand
    // overruns it (range-end panic). Gate each band to the feature set whose
    // scratch actually carries it.
    #[cfg(feature = "x-wide")]
    {
        hypot_cell!(c, 24, "N24_D462");
        hypot_cell!(c, 32, "N32_D616");
    }
    #[cfg(feature = "xx-wide")]
    {
        hypot_cell!(c, 48, "N48_D924");
        hypot_cell!(c, 64, "N64_D1232");
    }
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_hypot(&mut c);
    c.final_summary();
}
