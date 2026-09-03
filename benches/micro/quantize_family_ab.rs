// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Scale-change family A/B arms: delegation parity, rounding-mode cost,
//! and whether `requantize` composes or duplicates work.
//!
//! Companion to `quantize_surface`, which sweeps the plain `quantize`
//! width x scale surface. This file asks the three comparative questions
//! that surface cannot answer, each as an N-way `compare_all` (ranking
//! table + verdict line) or a labelled criterion group.
//!
//! # 1. Delegation parity — `rescale` vs `quantize`
//!
//! `rescale` / `rescale_with` are deprecated aliases that delegate to
//! `quantize` / `quantize_with` with nothing else in the body. They
//! should therefore be INDISTINGUISHABLE. They are benched precisely to
//! prove that: a measurable gap in a pure one-line delegation is a
//! finding (a missed inline), not a curiosity. `compare_all` prints the
//! margin, and anything inside its "~tie" band is the expected result.
//!
//! # 2. Rounding-mode cost
//!
//! `quantize_with` takes the mode as a RUNTIME argument, so the eight
//! modes share one monomorphisation and differ only in the work after
//! the divide. They are not equal work:
//!
//! - `Trunc` returns the quotient with no further work;
//! - `Floor` / `Ceiling` / `AwayFromZero` add at most one increment;
//! - the `Half*` modes compare the remainder against `divisor/2`
//!   (`HalfToEven` additionally tests the quotient's low bit);
//! - `ZeroFiveUp` needs the last DECIMAL digit of the quotient, i.e. an
//!   extra full-width `quotient % ten` — the source itself notes this is
//!   "a wide `%`, so O(limbs)".
//!
//! That makes `ZeroFiveUp` the mode whose cost is expected to scale with
//! the width, so the mode arm is run at a narrow AND a wide tier: if the
//! gap widens with the width, the extra modulo is the cause.
//!
//! Only the DOWN direction is benched here — scaling up multiplies and
//! never rounds, so the mode is dead weight in that direction.
//!
//! # 3. `requantize` — composition or duplication
//!
//! `requantize` moves width and scale together, branching on direction:
//! growing widens first then rescales at the wider (more expensive)
//! width; shrinking rescales at the source width first then narrows. The
//! interesting question is whether a combined move costs about the same
//! as the scale move alone (composition) or noticeably more
//! (duplicated work) — and, for the same-width case, whether
//! `requantize::<same_N, T>` is as cheap as the plain `quantize::<T>`
//! it ought to reduce to. The `requantize_compose` group answers that
//! last one head-to-head, both candidates returning the identical type.
//!
//! Operands are parsed ONCE outside `bn.iter`; every timed closure holds
//! only the call, `black_box`-guarded in and out so the const-generic
//! dispatch cannot fold away.
//!
//! Run with:
//! `cargo bench --features wide --bench quantize_family_ab`

use criterion::Criterion;
use decimal_scaled::{D38, D153, RoundingMode};
use std::hint::black_box;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

#[path = "../support/scale_operands.rs"]
mod scale_operands;
use scale_operands::dense_str;

// ── 1. delegation parity ──────────────────────────────────────────────

/// `quantize` vs the deprecated `rescale` alias at one cell. The alias
/// body is a single delegating call, so the two should tie; a margin
/// outside the noise band is the finding.
macro_rules! parity_pair {
    ($c:expr, $w:literal, $ty:ident, $src:literal, $tgt:literal) => {{
        let v: ::decimal_scaled::$ty<$src> =
            dense_str($src).parse().expect("bench operand parses");
        compare_all(
            $c,
            concat!("quantize_parity/", $w, "_s", $src, "_to_s", $tgt),
            |_: &::decimal_scaled::$ty<$src>| {
                concat!($w, "_s", $src, "_to_s", $tgt).to_string()
            },
            vec![v],
            vec![
                (
                    "quantize",
                    (|x: ::decimal_scaled::$ty<$src>| x.quantize::<$tgt>())
                        as fn(
                            ::decimal_scaled::$ty<$src>,
                        )
                            -> ::decimal_scaled::$ty<$tgt>,
                ),
                ("rescale", |x: ::decimal_scaled::$ty<$src>| {
                    x.rescale::<$tgt>()
                }),
            ],
        );
    }};
}

/// `quantize_with` vs the deprecated `rescale_with` alias, same contract
/// as [`parity_pair`] but on the explicit-mode entry point.
macro_rules! parity_pair_with {
    ($c:expr, $w:literal, $ty:ident, $src:literal, $tgt:literal) => {{
        let v: ::decimal_scaled::$ty<$src> =
            dense_str($src).parse().expect("bench operand parses");
        compare_all(
            $c,
            concat!("quantize_with_parity/", $w, "_s", $src, "_to_s", $tgt),
            |_: &::decimal_scaled::$ty<$src>| {
                concat!($w, "_s", $src, "_to_s", $tgt).to_string()
            },
            vec![v],
            vec![
                (
                    "quantize_with",
                    (|x: ::decimal_scaled::$ty<$src>| {
                        x.quantize_with::<$tgt>(RoundingMode::HalfToEven)
                    })
                        as fn(
                            ::decimal_scaled::$ty<$src>,
                        )
                            -> ::decimal_scaled::$ty<$tgt>,
                ),
                ("rescale_with", |x: ::decimal_scaled::$ty<$src>| {
                    x.rescale_with::<$tgt>(RoundingMode::HalfToEven)
                }),
            ],
        );
    }};
}

// The deprecated aliases are called DELIBERATELY here — proving they
// still cost exactly what they delegate to is the point of the arm.
#[allow(deprecated)]
fn bench_parity(c: &mut Criterion) {
    // A narrow tier and a wide one, each on a full-span scale-down (the
    // direction that does the most work, so any delegation overhead has
    // the best chance of showing against it).
    parity_pair!(c, "D18", D18, 17, 0);
    parity_pair!(c, "D38", D38, 37, 18);
    parity_pair!(c, "D153", D153, 152, 76);
    parity_pair!(c, "D307", D307, 306, 153);

    parity_pair_with!(c, "D38", D38, 37, 18);
    parity_pair_with!(c, "D307", D307, 306, 153);
}

// ── 2. rounding-mode cost ─────────────────────────────────────────────

/// All eight rounding modes over one scale-DOWN cell. The mode is a
/// runtime argument, so these share a monomorphisation and differ only
/// in the post-divide work.
macro_rules! mode_group {
    ($c:expr, $w:literal, $ty:ident, $src:literal, $tgt:literal) => {{
        let v: ::decimal_scaled::$ty<$src> =
            dense_str($src).parse().expect("bench operand parses");
        compare_all(
            $c,
            concat!("quantize_modes/", $w, "_s", $src, "_to_s", $tgt),
            |_: &::decimal_scaled::$ty<$src>| {
                concat!($w, "_s", $src, "_to_s", $tgt).to_string()
            },
            vec![v],
            vec![
                (
                    "Trunc",
                    (|x: ::decimal_scaled::$ty<$src>| {
                        x.quantize_with::<$tgt>(RoundingMode::Trunc)
                    })
                        as fn(
                            ::decimal_scaled::$ty<$src>,
                        )
                            -> ::decimal_scaled::$ty<$tgt>,
                ),
                ("Floor", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::Floor)
                }),
                ("Ceiling", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::Ceiling)
                }),
                ("AwayFromZero", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::AwayFromZero)
                }),
                ("HalfToEven", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::HalfToEven)
                }),
                ("HalfAwayFromZero", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::HalfAwayFromZero)
                }),
                ("HalfTowardZero", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::HalfTowardZero)
                }),
                ("ZeroFiveUp", |x: ::decimal_scaled::$ty<$src>| {
                    x.quantize_with::<$tgt>(RoundingMode::ZeroFiveUp)
                }),
            ],
        );
    }};
}

fn bench_modes(c: &mut Criterion) {
    // Narrow and wide, so a mode whose cost scales with the limb count
    // (`ZeroFiveUp`, which needs an extra full-width `% 10`) separates
    // from the constant-work modes as the width grows.
    mode_group!(c, "D18", D18, 17, 0);
    mode_group!(c, "D38", D38, 37, 18);
    mode_group!(c, "D153", D153, 152, 76);
    mode_group!(c, "D307", D307, 306, 153);
}

// ── 3. requantize: width x scale together ─────────────────────────────

fn bench_requantize(c: &mut Criterion) {
    // Source D38<18> (dense). Every target below is in range: the value
    // is ~1.1234, so widening never overflows, and the narrowing cells
    // pair the width drop with a scale drop that brings the magnitude
    // inside the narrower tier (a narrow-at-equal-scale from a high
    // source scale cannot fit by construction, so it is not benched).
    let v38: D38<18> = dense_str(18).parse().expect("bench operand parses");
    {
        let mut g = c.benchmark_group("requantize/from_D38_s18");
        // Baseline: the scale move alone, no width change.
        g.bench_function("quantize_only_s9", |bn| {
            bn.iter(|| black_box(black_box(v38).quantize::<9>()))
        });
        // Same width, both axes unchanged — the cheapest possible call.
        g.bench_function("same_width_s18_identity", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<2, 18>()))
        });
        // Same width, scale down — should reduce to `quantize_only_s9`.
        g.bench_function("same_width_s9", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<2, 9>()))
        });
        // Width only, scale held.
        g.bench_function("widen_s18_width_only", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<4, 18>()))
        });
        // Width + scale, both directions of the scale move.
        g.bench_function("widen_s9_down", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<4, 9>()))
        });
        g.bench_function("widen_s27_up", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<4, 27>()))
        });
        g.bench_function("narrow_s9_down", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<1, 9>()))
        });
        g.bench_function("narrow_s17_down", |bn| {
            bn.iter(|| black_box(black_box(v38).requantize::<1, 17>()))
        });
        g.finish();
    }

    // The same matrix at a wide tier, where the "widen first, then
    // rescale at the wider width" branch has a much larger penalty to
    // pay if the order genuinely costs anything.
    let v153: D153<76> = dense_str(76).parse().expect("bench operand parses");
    {
        let mut g = c.benchmark_group("requantize/from_D153_s76");
        g.bench_function("quantize_only_s38", |bn| {
            bn.iter(|| black_box(black_box(v153).quantize::<38>()))
        });
        g.bench_function("same_width_s76_identity", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<8, 76>()))
        });
        g.bench_function("same_width_s38", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<8, 38>()))
        });
        g.bench_function("widen_s76_width_only", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<16, 76>()))
        });
        g.bench_function("widen_s38_down", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<16, 38>()))
        });
        g.bench_function("widen_s114_up", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<16, 114>()))
        });
        g.bench_function("narrow_s38_down", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<4, 38>()))
        });
        g.bench_function("narrow_s28_down", |bn| {
            bn.iter(|| black_box(black_box(v153).requantize::<3, 28>()))
        });
        g.finish();
    }

    // Head-to-head: does `requantize` at an UNCHANGED width reduce to
    // the plain `quantize`? Both candidates return the identical type,
    // so this is a true A/B and the verdict line is the answer.
    compare_all(
        c,
        "requantize_compose/D38_s18_to_s9",
        |_: &D38<18>| "D38_s18_to_s9".to_string(),
        vec![v38],
        vec![
            ("quantize", (|x: D38<18>| x.quantize::<9>()) as fn(D38<18>) -> D38<9>),
            ("requantize_same_width", |x: D38<18>| x.requantize::<2, 9>()),
        ],
    );
    compare_all(
        c,
        "requantize_compose/D153_s76_to_s38",
        |_: &D153<76>| "D153_s76_to_s38".to_string(),
        vec![v153],
        vec![
            (
                "quantize",
                (|x: D153<76>| x.quantize::<38>()) as fn(D153<76>) -> D153<38>,
            ),
            ("requantize_same_width", |x: D153<76>| x.requantize::<8, 38>()),
        ],
    );
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench_parity(&mut c);
    bench_modes(&mut c);
    bench_requantize(&mut c);
    c.final_summary();
}
