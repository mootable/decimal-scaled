// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Wide-tier `quantize` scale-down, differenced against an independent
//! implementation of the same operation.
//!
//! # Why this file exists, and why it lives here
//!
//! `quantize_with`'s scale-down routes its `/ 10^shift` through the crate's
//! rescale matcher, which picks between three kernels on `(shift,
//! significant width)`. Its baked-reciprocal arm engages only from
//! twenty-four u64 limbs upward with `shift` in `200..=1850` — so no cell
//! it serves was reachable from `quantize` in any existing test, the widest
//! of which stopped at D76's four limbs.
//!
//! The arm is compiled out unless `x-wide` or `xx-wide` is on, and the
//! width-sharded integration suite builds one tier at a time
//! (`--features d462,macros,dyn`), which does **not** turn those umbrellas
//! on. This file is therefore a root-crate test gated on `x-wide`: that is
//! the build the full-feature leg uses, so the three kernels are all
//! reachable here and all three are gated rather than merely covered. The
//! shift ladder below walks them — 30 is the single-chunk band, 100 the
//! chained band below the threshold, and 361 and up inside it.
//!
//! # The oracle
//!
//! Not a hand-computed expected value; at these widths that would be a
//! second implementation with no reviewer. Each result is instead
//! differenced against `convert_from_with` at the *same* width, which
//! reaches the same operation by a wholly separate route: a typed
//! `div_rem` against `10^shift` in the integer layer, never touching the
//! rescale matcher. The two share only the mode decider, so they disagree
//! if the quotient, the remainder comparison, the last decimal digit, or
//! the sign handling differs on either side.
//!
//! # Both liveness guards matter
//!
//! A differential that only ever hits the zero-remainder early return
//! agrees trivially and proves nothing, so each test asserts that the
//! rounding actually ran:
//!
//! - the dense case asserts `Trunc` and `AwayFromZero` disagree;
//! - the tie case asserts `HalfTowardZero` and `HalfAwayFromZero` disagree.
//!
//! Neither is a heuristic. Both are exact biconditionals, provable from the
//! mode definitions rather than observed to work, so neither can pass on an
//! input that fails to exercise what it names. `Trunc` keeps the truncated
//! quotient and `AwayFromZero` steps one away from zero on any non-zero
//! discard, so the two differ **if and only if** the discarded digits are
//! non-zero. `HalfTowardZero` and `HalfAwayFromZero` both bump above the
//! half and both decline below it, and split only at the half itself — so
//! they differ **if and only if** the remainder is exactly the half, which
//! is the tie the test set out to construct.
//!
//! Both inputs are built near the storage maximum on purpose: the matcher
//! keys on the significant limb length after leading-zero trimming, so a
//! small value at a wide tier routes as a narrow one and would silently
//! miss the band. Each test asserts its input stayed above `MAX >> 3` so
//! that property cannot rot.

#![cfg(feature = "x-wide")]

use decimal_scaled::{Int, RoundingMode};

const ALL_MODES: [RoundingMode; 8] = [
    RoundingMode::HalfToEven,
    RoundingMode::HalfAwayFromZero,
    RoundingMode::HalfTowardZero,
    RoundingMode::Trunc,
    RoundingMode::Floor,
    RoundingMode::Ceiling,
    RoundingMode::AwayFromZero,
    RoundingMode::ZeroFiveUp,
];

/// Emits the dense pair and the exact-tie pair for one
/// `(tier, limbs, source scale, target scale)` cell.
macro_rules! wide_quantize_differential {
    ($dense:ident, $tie:ident, $ty:ident, $n:literal, $src:literal, $tgt:literal) => {
        /// Generic (non-tie) remainders across all eight modes, both signs,
        /// at three near-maximal magnitudes.
        #[test]
        fn $dense() {
            type Src = decimal_scaled::$ty<$src>;
            type Dst = decimal_scaled::$ty<$tgt>;

            let one = Int::<$n>::ONE;
            let three = one + one + one;
            let seven = three + three + one;
            let width_floor = Int::<$n>::MAX >> 3u32;

            for mag in [Int::<$n>::MAX, Int::<$n>::MAX / three, Int::<$n>::MAX / seven] {
                assert!(
                    mag >= width_floor,
                    "{}: input dropped below the width floor, so it no longer \
                     routes as a full-width value",
                    stringify!($dense)
                );
                for raw in [mag, Int::<$n>::ZERO - mag] {
                    let x = Src::from_bits(raw);

                    let trunc = x.quantize_with::<$tgt>(RoundingMode::Trunc);
                    let away = x.quantize_with::<$tgt>(RoundingMode::AwayFromZero);
                    assert_ne!(
                        trunc.to_bits(),
                        away.to_bits(),
                        "{}: discarded digits were zero, so the rounding path \
                         never ran and this proves nothing",
                        stringify!($dense)
                    );

                    for &mode in &ALL_MODES {
                        let got = x.quantize_with::<$tgt>(mode);
                        let want = Dst::convert_from_with(x, mode)
                            .expect("a same-width scale-down cannot overflow");
                        assert_eq!(
                            got.to_bits(),
                            want.to_bits(),
                            "{}: quantize disagrees with convert_from_with under {:?}",
                            stringify!($dense),
                            mode
                        );
                    }
                }
            }
        }

        /// The exact half-way remainder — the one input that separates the
        /// three nearest modes — built by construction rather than searched
        /// for.
        #[test]
        fn $tie() {
            type Src = decimal_scaled::$ty<$src>;
            type Dst = decimal_scaled::$ty<$tgt>;

            let one = Int::<$n>::ONE;
            let pow = Int::<$n>::TEN.pow(($src - $tgt) as u32);
            // `10^k` is even for every `k >= 1`, so the half is exact.
            let half = pow >> 1u32;
            // The largest quotient leaving room for the half digit and for a
            // bump away from zero.
            let quotient = Int::<$n>::MAX / pow - one;
            let raw = quotient * pow + half;

            assert!(
                raw >= Int::<$n>::MAX >> 3u32,
                "{}: input dropped below the width floor, so it no longer \
                 routes as a full-width value",
                stringify!($tie)
            );

            for signed in [raw, Int::<$n>::ZERO - raw] {
                let x = Src::from_bits(signed);

                let toward = x.quantize_with::<$tgt>(RoundingMode::HalfTowardZero);
                let away = x.quantize_with::<$tgt>(RoundingMode::HalfAwayFromZero);
                assert_ne!(
                    toward.to_bits(),
                    away.to_bits(),
                    "{}: the remainder was not the exact half, so the tie-break \
                     branch never ran",
                    stringify!($tie)
                );

                for &mode in &ALL_MODES {
                    let got = x.quantize_with::<$tgt>(mode);
                    let want = Dst::convert_from_with(x, mode)
                        .expect("a same-width scale-down cannot overflow");
                    assert_eq!(
                        got.to_bits(),
                        want.to_bits(),
                        "{}: quantize disagrees with convert_from_with under {:?}",
                        stringify!($tie),
                        mode
                    );
                }
            }
        }
    };
}

// D462 — twenty-four u64 limbs, the matcher's lower width bound. These
// three cells walk the shift axis across all three kernels.
wide_quantize_differential!(d462_shift30_dense, d462_shift30_tie, D462, 24, 130, 100);
wide_quantize_differential!(d462_shift100_dense, d462_shift100_tie, D462, 24, 200, 100);
wide_quantize_differential!(d462_shift361_dense, d462_shift361_tie, D462, 24, 461, 100);

// D616 comes with `x-wide`; the two widest tiers need `xx-wide`.
wide_quantize_differential!(d616_shift500_dense, d616_shift500_tie, D616, 32, 600, 100);
#[cfg(feature = "d924")]
wide_quantize_differential!(d924_shift800_dense, d924_shift800_tie, D924, 48, 900, 100);
#[cfg(feature = "d1232")]
wide_quantize_differential!(
    d1232_shift1100_dense,
    d1232_shift1100_tie,
    D1232,
    64,
    1200,
    100
);
