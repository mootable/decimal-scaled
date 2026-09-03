// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `quantize` surface sweep — width x scale-pair, both directions.
//!
//! # Why this bench exists
//!
//! The scale-change family (`quantize` / `requantize` / the deprecated
//! `rescale` aliases) had NO performance coverage of any kind: the
//! `bench-branch-compare` sweep measures the arithmetic and
//! transcendental surface and never calls any of them. This file is the
//! first measurement of the family, not a tuning of an existing one.
//!
//! The governing standard is the monotonicity premise: *the same or
//! better performance for a scale as the width goes down, and better
//! performance for a width as the scale goes down* — less work must not
//! cost more. A cell that does less work but takes longer is an
//! INVERSION, i.e. a defect, and this bench exists to make inversions in
//! this family visible for the first time.
//!
//! # What drives the cost (and therefore the sampling strategy)
//!
//! `TARGET_SCALE` is a const generic, so every `(SCALE, TARGET_SCALE)`
//! pair is a separate monomorphisation — the full `O(S^2)` grid cannot
//! be swept without exploding compile time and binary size. It does not
//! need to be. `quantize_with` does its work as a function of the scale
//! DELTA `k = |TARGET_SCALE - SCALE|`:
//!
//! - `k == 0`  — bit-identity, returns `from_bits` immediately;
//! - `k > 0` up   — build `10^k`, ONE `checked_mul`; exact, no rounding;
//! - `k > 0` down — build `10^k`, one `/`, one `%`, then rounding.
//!
//! So the cost axis is `k` (through the limb count of `10^k`), not the
//! source scale on its own. The sweep therefore samples `k` at the
//! structurally meaningful points of a u64-limb representation rather
//! than sampling the (src, tgt) product grid: `k = 1` (minimal move),
//! `k ~ M/2` (mid), and `k = M` (the tier's full span), plus the
//! identity. That reduces the grid from `O(S^2)` to a handful of cells
//! per width while still bracketing the whole cost curve.
//!
//! # The two input classes (why one input would have lied)
//!
//! Scaling DOWN divides and then rounds — but `quantize_with` returns
//! early when the remainder is zero, skipping the entire rounding block.
//! A round-number operand therefore measures a strictly different (and
//! much shorter) path than a value with a non-zero residue. Both are
//! benched:
//!
//! - `exact` — `2`, whose low-order digits are all zero, so
//!   `remainder == zero` and the early return fires;
//! - `dense` — `1.123456789...`, whose fractional digits cycle `1..=9`
//!   and so never contain a zero run, guaranteeing a non-zero remainder
//!   for every `k >= 1` and forcing the full divide-and-round path.
//!
//! The gap between them is the cost of the rounding block itself.
//!
//! # Layout
//!
//! - `quantize_common/...` — the SAME scale pair at every width, so the
//!   rows of one group are directly comparable and a WIDTH inversion
//!   (a narrower tier costing more than a wider one at identical work)
//!   is visible by reading down the group.
//! - `quantize_span/<W>` — that tier's own `M`-relative cells, so the
//!   cost curve against the delta `k` is visible within a width, and a
//!   SCALE inversion (a smaller move costing more than a larger one) is
//!   visible by reading down the group.
//!
//! Operands are parsed ONCE, outside `bn.iter`; the timed closure holds
//! only the `quantize` call, `black_box`-guarded on the way in and out
//! so the const-generic dispatch cannot be folded away.
//!
//! Run with:
//! `cargo bench --features wide --bench quantize_surface`

use criterion::Criterion;
use std::hint::black_box;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::micro_criterion;

#[path = "../support/scale_operands.rs"]
mod scale_operands;
use scale_operands::{dense_str, exact_str};

// ── section A: identical scale pairs at every width ───────────────────

/// One row of a common-cell group: the named width, benched at the
/// group's shared `(src -> tgt)` pair. The row label is the WIDTH, so
/// the group reads as a width comparison at identical work.
macro_rules! common_row {
    ($g:expr, $w:literal, $ty:ident, $src:literal, $tgt:literal, $mk:ident) => {{
        let v: ::decimal_scaled::$ty<$src> =
            $mk($src).parse().expect("bench operand parses");
        $g.bench_function($w, |bn| {
            bn.iter(|| black_box(black_box(v).quantize::<$tgt>()))
        });
    }};
}

/// Stamp ONE `(src -> tgt)` pair across every width under a single
/// criterion group. Every width does identical work here, so any spread
/// between the rows is a width effect and a narrower tier costing more
/// than a wider one is an inversion.
///
/// The pairs chosen below are legal at every benched width because each
/// tier's `MAX_SCALE` (`digits - 1`) is at least 17.
macro_rules! common_cell {
    ($c:expr, $group:literal, $src:literal, $tgt:literal, $mk:ident) => {{
        let mut g = $c.benchmark_group(concat!("quantize_common/", $group));
        common_row!(g, "D18", D18, $src, $tgt, $mk);
        common_row!(g, "D38", D38, $src, $tgt, $mk);
        common_row!(g, "D57", D57, $src, $tgt, $mk);
        common_row!(g, "D76", D76, $src, $tgt, $mk);
        common_row!(g, "D153", D153, $src, $tgt, $mk);
        common_row!(g, "D307", D307, $src, $tgt, $mk);
        g.finish();
    }};
}

// ── section B: per-width, M-relative span cells ───────────────────────

/// Stamp a width's own cells into one group, labelled by the cell. The
/// rows share a width, so reading down the group gives the cost curve
/// against the scale delta `k` and exposes a SCALE inversion (a smaller
/// move costing more than a larger one at the same width).
macro_rules! span_cells {
    ($c:expr, $w:literal, $ty:ident,
     [$( ($label:literal, $mk:ident, $src:literal, $tgt:literal) ),+ $(,)?]) => {{
        let mut g = $c.benchmark_group(concat!("quantize_span/", $w));
        $(
            {
                let v: ::decimal_scaled::$ty<$src> =
                    $mk($src).parse().expect("bench operand parses");
                g.bench_function(
                    concat!($label, "_s", $src, "_to_s", $tgt),
                    |bn| bn.iter(|| black_box(black_box(v).quantize::<$tgt>())),
                );
            }
        )+
        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // ── A. identical work at every width ──────────────────────────────
    //
    // `s17 -> s16` (k = 1), `s17 -> s0` (k = 17) and `s0 -> s17`
    // (k = 17 up) are legal at D18 (MAX_SCALE 17) and therefore at every
    // wider tier too, so all six widths run the SAME pair. Both input
    // classes are stamped for the DOWN cells, where the early return
    // makes them different paths; the UP direction never rounds, so it
    // is benched dense only.
    common_cell!(c, "dn1_dense_s17_to_s16", 17, 16, dense_str);
    common_cell!(c, "dn1_exact_s17_to_s16", 17, 16, exact_str);
    common_cell!(c, "dnmax_dense_s17_to_s0", 17, 0, dense_str);
    common_cell!(c, "dnmax_exact_s17_to_s0", 17, 0, exact_str);
    common_cell!(c, "upmax_dense_s0_to_s17", 0, 17, dense_str);

    // ── B. each tier's own span ───────────────────────────────────────
    //
    // Cells are `M`-relative (`M = MAX_SCALE = digits - 1`): the
    // identity, a 1-place move, a half-span move and a full-span move,
    // in both directions. `exact` is stamped alongside `dense` on the
    // two down-cells where the rounding block dominates.
    //
    // D18,  M = 17
    span_cells!(c, "D18", D18, [
        ("id",       dense_str,  8,  8),
        ("dn1",      dense_str, 17, 16),
        ("dnhalf",   dense_str, 17,  8),
        ("dnhalf_x", exact_str, 17,  8),
        ("dnmax",    dense_str, 17,  0),
        ("dnmax_x",  exact_str, 17,  0),
        ("up1",      dense_str,  8,  9),
        ("uphalf",   dense_str,  0,  8),
        ("upmax",    dense_str,  0, 17),
    ]);
    // D38,  M = 37
    span_cells!(c, "D38", D38, [
        ("id",       dense_str, 18, 18),
        ("dn1",      dense_str, 37, 36),
        ("dnhalf",   dense_str, 37, 18),
        ("dnhalf_x", exact_str, 37, 18),
        ("dnmax",    dense_str, 37,  0),
        ("dnmax_x",  exact_str, 37,  0),
        ("up1",      dense_str, 18, 19),
        ("uphalf",   dense_str,  0, 18),
        ("upmax",    dense_str,  0, 37),
    ]);
    // D57,  M = 56
    span_cells!(c, "D57", D57, [
        ("id",       dense_str, 28, 28),
        ("dn1",      dense_str, 56, 55),
        ("dnhalf",   dense_str, 56, 28),
        ("dnhalf_x", exact_str, 56, 28),
        ("dnmax",    dense_str, 56,  0),
        ("dnmax_x",  exact_str, 56,  0),
        ("up1",      dense_str, 28, 29),
        ("uphalf",   dense_str,  0, 28),
        ("upmax",    dense_str,  0, 56),
    ]);
    // D76,  M = 75
    span_cells!(c, "D76", D76, [
        ("id",       dense_str, 37, 37),
        ("dn1",      dense_str, 75, 74),
        ("dnhalf",   dense_str, 75, 37),
        ("dnhalf_x", exact_str, 75, 37),
        ("dnmax",    dense_str, 75,  0),
        ("dnmax_x",  exact_str, 75,  0),
        ("up1",      dense_str, 37, 38),
        ("uphalf",   dense_str,  0, 37),
        ("upmax",    dense_str,  0, 75),
    ]);
    // D153, M = 152
    span_cells!(c, "D153", D153, [
        ("id",       dense_str,  76,  76),
        ("dn1",      dense_str, 152, 151),
        ("dnhalf",   dense_str, 152,  76),
        ("dnhalf_x", exact_str, 152,  76),
        ("dnmax",    dense_str, 152,   0),
        ("dnmax_x",  exact_str, 152,   0),
        ("up1",      dense_str,  76,  77),
        ("uphalf",   dense_str,   0,  76),
        ("upmax",    dense_str,   0, 152),
    ]);
    // D307, M = 306
    span_cells!(c, "D307", D307, [
        ("id",       dense_str, 153, 153),
        ("dn1",      dense_str, 306, 305),
        ("dnhalf",   dense_str, 306, 153),
        ("dnhalf_x", exact_str, 306, 153),
        ("dnmax",    dense_str, 306,   0),
        ("dnmax_x",  exact_str, 306,   0),
        ("up1",      dense_str, 153, 154),
        ("uphalf",   dense_str,   0, 153),
        ("upmax",    dense_str,   0, 306),
    ]);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
