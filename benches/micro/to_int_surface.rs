// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `to_int` dispatch-seam A/B — width x scale x rounding mode, both
//! input classes.
//!
//! # Why this bench exists
//!
//! `to_int_with` had NO performance coverage of any kind: it appears in
//! none of the repo's bench files, and `to_int` has a single data point
//! (`benches/support/all_functions.rs`, one width, one scale, default
//! mode). The 0.5.1 change that lifted its rounding out of
//! `decl_decimal_int_conversion_methods!` — where it was a second copy of
//! the `should_bump` decision, emitted twelve times — therefore altered a
//! rounding implementation on a surface nothing measured. This file is
//! the first measurement of it.
//!
//! # What it compares
//!
//! The two `policy::to_int` `Algorithm` arms, over the same operands:
//!
//! - `qr` — `to_int_quotient_remainder`, two typed full-width divides on
//!   the storage integer. The shape that shipped in 0.5.x, so this arm is
//!   also the BEFORE side of the change.
//! - `rescale` — `to_int_rescale`, one pass through the `÷10^scale`
//!   matcher, which produces the quotient and applies `mode` together.
//!
//! `select` names `qr` at every cell today. Moving a cell to `rescale` is
//! this bench's verdict to give, and it must be given for a contiguous
//! width/scale region rather than for one cell.
//!
//! # The four axes, and why each is here
//!
//! **Width x scale.** One group per `(width, scale)` cell. Each width is
//! benched at a low scale and at its own near-`MAX_SCALE` span, because
//! the two arms scale differently: `qr`'s cost follows the storage width
//! whatever the operand holds, while `rescale` sizes off the operand's
//! significant limb length.
//!
//! **All eight rounding modes.** The mode is the INPUT axis of every
//! group, so each arm is timed once per mode. That is the axis a
//! `should_bump` substitution can regress, and it is also what makes the
//! identity wall below cover every branch of the decider.
//!
//! **Exact vs dense operands.** `to_int` returns early when the remainder
//! is zero, skipping the whole rounding block. An operand set that only
//! hit that path would measure none of the work this change is about and
//! would report green regardless. So both classes are benched, and the
//! `assert_reaches_rounding` wall below PROVES which path each operand
//! takes rather than assuming it (see *The reach proof*).
//!
//! **Before/after on the constant hoisting.** The 0.5.x body built its
//! `zero` and `one` by parsing a string literal (`from_str_radix("0",
//! 10).expect(..)`) on every call; the kernel uses the compile-time
//! `Int::<N>::ZERO` / `ONE`. That hoisting is inside the `qr` arm, so the
//! `qr` timings here are the AFTER side of it and the 0.5.1 release
//! figures are the before.
//!
//! # The reach proof
//!
//! A bench whose inputs never reach the rounding block is worse than no
//! bench. Two assertions establish which path each operand takes, and
//! both run before anything is timed:
//!
//! - an `exact` operand must give the SAME result under all eight modes —
//!   only the `remainder == 0` early return can do that, so agreement is
//!   proof the rounding block was SKIPPED;
//! - a `dense` operand must give at least two DIFFERENT results across
//!   the eight modes — only the rounding block can produce a disagreement,
//!   so a spread is proof it RAN.
//!
//! The counts are printed per cell, so the CI log carries the evidence
//! rather than the claim.
//!
//! # The identity wall
//!
//! Before either arm is timed, every operand is evaluated through BOTH
//! arms under all eight modes and the results are asserted equal. The two
//! arms are meant to be bit-identical — they take the same verdict from
//! the same decider — so any divergence is a defect, and the bench fails
//! rather than reporting a ranking between two different answers.
//!
//! Run with:
//! `cargo bench --features wide,bench-alt --bench to_int_surface`
//!
//! `bench-alt` is what exposes the two arms individually
//! (`__bench_internals`); without it cargo skips this bench and still
//! exits 0, which the CI micro job turns into a hard error rather than a
//! green tick on a run that graded nothing.

use criterion::Criterion;
use decimal_scaled::RoundingMode;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{ab_compare, micro_criterion, Candidate};

#[path = "../support/scale_operands.rs"]
mod scale_operands;
use scale_operands::{dense_str, exact_str};

/// Every rounding mode, in one place. A hardcoded subset still compiles
/// after a variant is added — it just covers less, silently — so the
/// mode axis is defined once and every cell uses it.
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

/// Short stable label for a mode, used as the criterion input id.
fn mode_label(mode: RoundingMode) -> String {
    match mode {
        RoundingMode::HalfToEven => "half_even",
        RoundingMode::HalfAwayFromZero => "half_away",
        RoundingMode::HalfTowardZero => "half_toward",
        RoundingMode::Trunc => "trunc",
        RoundingMode::Floor => "floor",
        RoundingMode::Ceiling => "ceiling",
        RoundingMode::AwayFromZero => "away",
        RoundingMode::ZeroFiveUp => "zero_five_up",
    }
    .to_string()
}

/// One `(width, scale, class)` cell: prove the operand's path, prove the
/// two arms agree, then time them against each other over the mode axis.
///
/// `$n` is the tier's u64-limb count — `to_int_rescale` takes `SCALE` as
/// a const generic, which no operand can infer, so both consts are named
/// here.
macro_rules! to_int_cell {
    ($c:expr, $w:literal, $ty:ident, $n:literal, $scale:literal,
     $class:literal, $mk:ident, $reaches:literal) => {{
        let text = $mk($scale);
        let v: ::decimal_scaled::$ty<$scale> =
            text.parse().expect("bench operand parses");
        let raw = v.0;
        let divisor = ::decimal_scaled::$ty::<$scale>::multiplier();
        let cell = concat!($w, "/", $class, "_s", stringify!($scale));

        let qr = |mode: RoundingMode| {
            ::decimal_scaled::__bench_internals::to_int_quotient_remainder::<$n>(
                raw, divisor, mode)
        };
        let rs = |mode: RoundingMode| {
            ::decimal_scaled::__bench_internals::to_int_rescale::<$n, $scale>(raw, mode)
        };

        // ── the identity wall ─────────────────────────────────────────
        // Both arms, all eight modes, before anything is timed.
        for mode in ALL_MODES {
            let a = qr(mode);
            let b = rs(mode);
            assert_eq!(
                a, b,
                "to_int arms disagree at {} mode {}: qr={} rescale={}",
                cell, mode_label(mode), a, b
            );
        }

        // ── the reach proof ───────────────────────────────────────────
        // How many DISTINCT results the eight modes produce. An exact
        // operand takes the `remainder == 0` early return, so every mode
        // must agree (1 distinct). A dense operand runs the rounding
        // block, which is the only thing that can make modes disagree, so
        // it must produce more than one.
        let mut results: Vec<i64> = ALL_MODES.iter().map(|&m| qr(m)).collect();
        results.sort_unstable();
        results.dedup();
        let distinct = results.len();
        println!(
            "reach [{}]: {} distinct results across 8 modes (rounding block {})",
            cell,
            distinct,
            if $reaches { "REACHED" } else { "skipped" }
        );
        if $reaches {
            assert!(
                distinct > 1,
                "{}: dense operand produced ONE result across all eight modes — \
                 the rounding block was NOT reached, so this cell measures nothing",
                cell
            );
        } else {
            assert_eq!(
                distinct, 1,
                "{}: exact operand produced {} results across eight modes — \
                 it did not take the early return, so it is not an exact-class input",
                cell, distinct
            );
        }

        ab_compare(
            $c,
            concat!("to_int/", $w, "/", $class, "_s", stringify!($scale)),
            |m: &RoundingMode| mode_label(*m),
            ALL_MODES,
            Candidate::new("qr", qr),
            Candidate::new("rescale", rs),
        );
    }};
}

/// Stamp both input classes at one `(width, scale)` cell.
macro_rules! to_int_scale {
    ($c:expr, $w:literal, $ty:ident, $n:literal, $scale:literal) => {{
        to_int_cell!($c, $w, $ty, $n, $scale, "dense", dense_str, true);
        to_int_cell!($c, $w, $ty, $n, $scale, "exact", exact_str, false);
    }};
}

fn bench(c: &mut Criterion) {
    // Each width at a LOW scale (2 — the money scale, where the divisor
    // is one limb whatever the storage width) and at its own near-span
    // scale, so the arms' different growth in the storage width and in
    // the operand's significant length are both visible.
    //
    // The tier's `MAX_SCALE` is `digits - 1`; the span cell sits at it.
    to_int_scale!(c, "D18", D18, 1, 2);
    to_int_scale!(c, "D18", D18, 1, 17);

    to_int_scale!(c, "D38", D38, 2, 2);
    to_int_scale!(c, "D38", D38, 2, 37);

    to_int_scale!(c, "D57", D57, 3, 2);
    to_int_scale!(c, "D57", D57, 3, 56);

    to_int_scale!(c, "D76", D76, 4, 2);
    to_int_scale!(c, "D76", D76, 4, 75);

    to_int_scale!(c, "D153", D153, 8, 2);
    to_int_scale!(c, "D153", D153, 8, 152);

    to_int_scale!(c, "D307", D307, 16, 2);
    to_int_scale!(c, "D307", D307, 16, 306);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
