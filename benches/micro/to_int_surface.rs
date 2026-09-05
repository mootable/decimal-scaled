// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `to_int` dispatch-seam A/B — width x scale x rounding mode, both
//! input classes, against a verbatim copy of the 0.5.x inline rounding.
//!
//! # Why this bench exists
//!
//! `to_int_with` had NO performance coverage of any kind: it appears in
//! none of the 79 files under `benches/`, and `to_int` has a single data
//! point (`benches/support/all_functions.rs:212` — `D38<12>`, one
//! constant operand, default mode). The 0.5.1 change that lifted its
//! rounding out of `decl_decimal_int_conversion_methods!` — where it was
//! a second copy of the `should_bump` decision, emitted twelve times —
//! therefore altered a rounding implementation on a surface nothing
//! measured. This file is the first measurement of it.
//!
//! # The three arms
//!
//! - `inline_051` — a VERBATIM transcription of the eight-mode match the
//!   macro carried in 0.5.x, written against the public `Int` / `Uint`
//!   surface. The BEFORE side, and the oracle the other two are checked
//!   against.
//! - `qr` — `to_int_quotient_remainder`, the same two typed full-width
//!   divides, with the inline match replaced by the shared `should_bump`
//!   decider. The AFTER side of that substitution.
//! - `rescale` — `to_int_rescale`, one pass through the `/10^scale`
//!   matcher, which produces the quotient and applies `mode` together.
//!
//! `policy::to_int::select` names `qr` at every cell today. Moving a cell
//! to `rescale` is this bench's verdict to give, and it must be given for
//! a contiguous width/scale region rather than for one cell.
//!
//! All three finish with the same `narrow_saturating_i64`, which moved
//! unchanged, so the comparison is about the ROUNDING — the part that
//! actually changed.
//!
//! # The four axes, and why each is here
//!
//! **Width x scale.** One group per `(width, scale)` cell. Each width is
//! benched at a low scale and at its own near-`MAX_SCALE` span, because
//! the arms scale differently: the typed divides follow the storage width
//! whatever the operand holds, while `rescale` sizes off the operand's
//! significant limb length.
//!
//! **All eight rounding modes.** The mode is the INPUT axis of every
//! group, so each arm is timed once per mode. That is the axis a
//! `should_bump` substitution can regress, and it is what makes the
//! identity wall below cover every branch of the decider — including
//! `AwayFromZero` and `ZeroFiveUp`, which had no test coverage at all
//! before 0.5.1.
//!
//! **Exact vs dense operands.** `to_int` returns early when the remainder
//! is zero, skipping the whole rounding block. An operand set that only
//! hit that path would measure none of the work this change is about and
//! would report green regardless. Both classes are benched, and the reach
//! proof below establishes which path each operand takes rather than
//! assuming it.
//!
//! **Before/after on the constant hoisting.** The 0.5.x body built its
//! `zero` and `one` by parsing a string literal
//! (`from_str_radix("0", 10).expect(..)`) on every call; the kernel uses
//! the compile-time `Int::<N>::ZERO` / `ONE`. `inline_051` keeps the
//! string parses, so `inline_051` vs `qr` IS that before/after pair —
//! the two arms are otherwise the same arithmetic.
//!
//! # The identity wall
//!
//! Before anything is timed, every operand is evaluated through all three
//! arms under all eight modes and the results are asserted equal. This is
//! what makes the `should_bump` substitution *proven* rather than
//! reasoned: `inline_051` is the pre-change code, so agreement is a
//! direct old-vs-new check at every mode, width, scale and input class —
//! 288 comparisons in total.
//!
//! # The reach proof
//!
//! A bench whose inputs never reach the rounding block is worse than no
//! bench, because it reports green. Two assertions establish which path
//! each operand takes:
//!
//! - an `exact` operand must give the SAME result under all eight modes —
//!   only the `remainder == 0` early return can do that, so agreement is
//!   proof the rounding block was SKIPPED;
//! - a `dense` operand must give at least two DIFFERENT results across
//!   the eight modes — only the rounding block can produce a
//!   disagreement, so a spread is proof it RAN.
//!
//! The counts are printed per cell, so the log carries the evidence
//! rather than the claim.
//!
//! Run with:
//! `cargo bench --features wide,bench-alt --bench to_int_surface`
//!
//! `bench-alt` is what exposes the arms individually
//! (`__bench_internals`); without it cargo skips this bench and still
//! exits 0, which the CI micro job turns into a hard error rather than a
//! green tick on a run that graded nothing.

use criterion::Criterion;
use decimal_scaled::RoundingMode;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::{compare_all, micro_criterion};

#[path = "../support/scale_operands.rs"]
mod scale_operands;
use scale_operands::{dense_str, exact_str};

/// Every rounding mode, in one place. A hardcoded subset still compiles
/// after a variant is added — it just covers less, silently — so the mode
/// axis is defined once and every cell uses it.
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

/// One `(width, scale, class)` cell.
///
/// Each arm is a nested `fn` item rather than a closure so all three
/// share one `fn` pointer type and can go into a single `compare_all`
/// ranking. The operands ride in the input tuple for the same reason —
/// and it guards them behind `black_box` on every iteration, which
/// matters here because the dispatch is DESIGNED to const-fold.
///
/// `$n` is the tier's u64-limb count: `to_int_rescale` takes `SCALE` as a
/// const generic, which no operand can infer, so both consts are named.
macro_rules! to_int_cell {
    ($c:expr, $w:literal, $ty:ident, $n:literal, $scale:literal,
     $class:literal, $mk:ident, $reaches:literal) => {{
        type Storage = ::decimal_scaled::Int<$n>;
        type In = (Storage, Storage, RoundingMode);

        /// Transcription of the eight-mode rounding match that
        /// `decl_decimal_int_conversion_methods!` emitted in 0.5.x,
        /// including its per-call `from_str_radix` constant parses. The
        /// pre-change behaviour, kept as the oracle for the identity wall
        /// and as the before-side of the constant hoisting.
        ///
        /// Verbatim except in two places, both forced by visibility and
        /// neither touching a rounding decision:
        ///
        /// - the trailing narrow is the crate's own
        ///   `narrow_saturating_i64` (the old body inlined the identical
        ///   three lines, and that part moved unchanged);
        /// - `ZeroFiveUp` compared `(quotient % ten).as_i128()
        ///   .unsigned_abs()` against `0` and `5`. `as_i128` is
        ///   crate-private, so the same magnitude is taken as
        ///   `Uint<N>` and compared against `Uint` 0 and 5. `|q % 10|`
        ///   is under 10 either way, so the two comparisons agree at
        ///   every operand.
        #[inline(never)]
        fn inline_051(input: In) -> i64 {
            let (raw, divisor, mode) = input;
            let zero = Storage::from_str_radix("0", 10)
                .expect("wide decimal: invalid base-10 literal");
            let one = Storage::from_str_radix("1", 10)
                .expect("wide decimal: invalid base-10 literal");
            let quotient = raw / divisor;
            let remainder = raw % divisor;
            let int_rounded: Storage = if remainder == zero {
                quotient
            } else {
                let abs_remainder = remainder.unsigned_abs();
                // `divisor` is `10^SCALE` and always positive, so
                // `unsigned_abs()` is the value itself; `>> 1` is the
                // half-LSB threshold.
                let half = divisor.unsigned_abs() >> 1;
                let is_non_negative = !raw.is_negative();
                match mode {
                    RoundingMode::HalfToEven => {
                        if abs_remainder < half {
                            quotient
                        } else if abs_remainder > half {
                            if is_non_negative { quotient + one } else { quotient - one }
                        } else if !quotient.bit(0) {
                            quotient
                        } else if is_non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    RoundingMode::HalfAwayFromZero => {
                        if abs_remainder < half {
                            quotient
                        } else if is_non_negative {
                            quotient + one
                        } else {
                            quotient - one
                        }
                    }
                    RoundingMode::HalfTowardZero => {
                        if abs_remainder > half {
                            if is_non_negative { quotient + one } else { quotient - one }
                        } else {
                            quotient
                        }
                    }
                    RoundingMode::Trunc => quotient,
                    RoundingMode::Floor => {
                        if is_non_negative { quotient } else { quotient - one }
                    }
                    RoundingMode::Ceiling => {
                        if is_non_negative { quotient + one } else { quotient }
                    }
                    // Any non-zero remainder lifts the magnitude; the
                    // `remainder == zero` case returned above.
                    RoundingMode::AwayFromZero => {
                        if is_non_negative { quotient + one } else { quotient - one }
                    }
                    // Pivot on the last decimal digit of |quotient| — a
                    // wide `%`, so O(limbs).
                    RoundingMode::ZeroFiveUp => {
                        let ten = Storage::from_str_radix("10", 10)
                            .expect("wide decimal: invalid base-10 literal");
                        let digit = (quotient % ten).unsigned_abs();
                        let five = Storage::from_str_radix("5", 10)
                            .expect("wide decimal: invalid base-10 literal")
                            .unsigned_abs();
                        if digit == zero.unsigned_abs() || digit == five {
                            if is_non_negative { quotient + one } else { quotient - one }
                        } else {
                            quotient
                        }
                    }
                }
            };
            ::decimal_scaled::__bench_internals::to_int_narrow_saturating::<$n>(int_rounded)
        }

        /// The routed arm: two typed divides, rounding via `should_bump`.
        #[inline(never)]
        fn qr(input: In) -> i64 {
            ::decimal_scaled::__bench_internals::to_int_quotient_remainder::<$n>(
                input.0, input.1, input.2)
        }

        /// The alternative arm: one pass through the `/10^scale` matcher.
        #[inline(never)]
        fn rescale(input: In) -> i64 {
            ::decimal_scaled::__bench_internals::to_int_rescale::<$n, $scale>(input.0, input.2)
        }

        let v: ::decimal_scaled::$ty<$scale> =
            $mk($scale).parse().expect("bench operand parses");
        let raw = v.0;
        let divisor = ::decimal_scaled::$ty::<$scale>::multiplier();
        let inputs: [In; 8] = [
            (raw, divisor, ALL_MODES[0]), (raw, divisor, ALL_MODES[1]),
            (raw, divisor, ALL_MODES[2]), (raw, divisor, ALL_MODES[3]),
            (raw, divisor, ALL_MODES[4]), (raw, divisor, ALL_MODES[5]),
            (raw, divisor, ALL_MODES[6]), (raw, divisor, ALL_MODES[7]),
        ];
        let cell = concat!($w, "/", $class, "_s", stringify!($scale));

        // ── the identity wall ─────────────────────────────────────────
        // All three arms, all eight modes, before anything is timed.
        // `inline_051` is the pre-change code, so this is a direct
        // old-vs-new check and not merely a self-consistency check.
        for input in inputs {
            let want = inline_051(input);
            let got_qr = qr(input);
            let got_rs = rescale(input);
            assert_eq!(
                want, got_qr,
                "{}: qr disagrees with the 0.5.x inline rounding at mode {}: {} vs {}",
                cell, mode_label(input.2), got_qr, want
            );
            assert_eq!(
                want, got_rs,
                "{}: rescale disagrees with the 0.5.x inline rounding at mode {}: {} vs {}",
                cell, mode_label(input.2), got_rs, want
            );
        }

        // ── the reach proof ───────────────────────────────────────────
        // How many DISTINCT results the eight modes produce. An exact
        // operand takes the `remainder == 0` early return, so every mode
        // must agree. A dense operand runs the rounding block, which is
        // the only thing that can make modes disagree.
        let mut results: Vec<i64> = inputs.iter().map(|&i| qr(i)).collect();
        results.sort_unstable();
        results.dedup();
        let distinct = results.len();
        println!(
            "reach [{}]: {} distinct results across 8 modes — rounding block {}",
            cell, distinct, if $reaches { "REACHED" } else { "skipped" }
        );
        if $reaches {
            assert!(
                distinct > 1,
                "{}: dense operand produced ONE result across all eight modes — the \
                 rounding block was NOT reached, so this cell measures nothing",
                cell
            );
        } else {
            assert_eq!(
                distinct, 1,
                "{}: exact operand produced {} results across eight modes — it did not \
                 take the early return, so it is not an exact-class input",
                cell, distinct
            );
        }

        compare_all(
            $c,
            concat!("to_int/", $w, "/", $class, "_s", stringify!($scale)),
            |i: &In| mode_label(i.2),
            inputs,
            vec![
                ("inline_051", inline_051 as fn(In) -> i64),
                ("qr", qr as fn(In) -> i64),
                ("rescale", rescale as fn(In) -> i64),
            ],
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
