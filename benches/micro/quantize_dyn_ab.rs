// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `DynDecimal::quantize_to` vs the typed `quantize` — what the runtime
//! -scale facade actually costs.
//!
//! # Why this is a separate arm
//!
//! Every other entry point in the scale-change family takes
//! `TARGET_SCALE` as a CONST generic, so the scale move is fixed at
//! compile time and the whole dispatch const-folds. `DynDecimal::
//! quantize_to` takes `target_scale: u32` as a RUNTIME value and returns
//! `Option<Box<dyn DynDecimal>>`. It therefore cannot const-fold, and it
//! pays two costs the typed path never does:
//!
//! 1. a runtime `match target_scale { .. }` over the tier's scale arms,
//!    to recover the const scale the typed call had for free;
//! 2. a heap allocation per call for the boxed result (plus the matching
//!    deallocation, which the timed loop also pays when the value is
//!    dropped).
//!
//! This is the one shape in the family that SHOULD legitimately cost
//! more, so it is benched apart from the typed surface rather than mixed
//! into it. The point of the arm is to quantify the gap: the `typed` and
//! `dyn_quantize_to` rows of each group do the same logical operation on
//! the same value, so their ratio is the price of the facade.
//!
//! The identity cell (`s -> s`) is included deliberately: it does no
//! arithmetic at all, so its `dyn` row is close to a pure measurement of
//! the match-plus-box overhead with the scale move stripped out.
//!
//! `rescale_to` — the deprecated dyn alias — is benched alongside for
//! the same delegation-parity reason as the typed `rescale`: the body is
//! a single forwarding call, so a measurable gap would be a finding.
//!
//! Each cell asserts its `quantize_to` returns `Some` BEFORE timing: a
//! `None` (out-of-range target, or a scale-up overflow) returns early and
//! would silently measure a trivial path instead of the real one.
//!
//! # Coverage limit — D18 and D38 only, by construction
//!
//! The `DynDecimal` impl is emitted per width by
//! `decl_decimal_dyn_impl!`, and `src/types/traits/dyn_decimal.rs`
//! invokes it exactly TWICE: for `D18` (scales 0..=18) and `D38`
//! (scales 0..=38). No wide tier implements the trait, so `D57` and
//! above cannot be benched here — the erased value cannot even be
//! constructed. This file therefore covers the two widths the facade
//! actually supports; the absence of wide-tier cells is a property of
//! the crate, not a gap in the bench.
//!
//! Because both benched widths are ungated, this target needs only the
//! `dyn` feature — not `wide`.
//!
//! Run with:
//! `cargo bench --features dyn --bench quantize_dyn_ab`

use criterion::Criterion;
use decimal_scaled::DynDecimal;
use std::hint::black_box;

#[path = "../support/ab_microbench.rs"]
mod ab_microbench;
use ab_microbench::micro_criterion;

#[path = "../support/scale_operands.rs"]
mod scale_operands;
use scale_operands::dense_str;

/// One typed-vs-dyn cell: the same value and the same logical scale
/// move, with the const-generic typed call and the runtime-scale dyn
/// call as the two rows of a single group.
///
/// The deprecated `rescale_to` alias is called deliberately, to prove it
/// still costs exactly what it delegates to.
macro_rules! dyn_cell {
    ($c:expr, $w:literal, $ty:ident, $src:literal, $tgt:literal) => {{
        let typed: ::decimal_scaled::$ty<$src> =
            dense_str($src).parse().expect("bench operand parses");
        let erased: Box<dyn DynDecimal> = Box::new(typed);

        // Correctness gate BEFORE timing: a `None` would mean the timed
        // closure measures an early return, not the real path.
        assert!(
            erased.quantize_to($tgt).is_some(),
            concat!(
                "dyn quantize_to returned None for ",
                $w,
                " s",
                $src,
                " -> s",
                $tgt,
                "; the cell would time an early return"
            )
        );

        let mut g = $c.benchmark_group(concat!(
            "quantize_dyn/",
            $w,
            "_s",
            $src,
            "_to_s",
            $tgt
        ));
        g.bench_function("typed", |bn| {
            bn.iter(|| black_box(black_box(typed).quantize::<$tgt>()))
        });
        g.bench_function("dyn_quantize_to", |bn| {
            // `target_scale` is black_box-ed so it stays a genuine
            // runtime value — const-folding it would erase the very
            // difference this arm exists to measure.
            bn.iter(|| black_box(erased.quantize_to(black_box($tgt))))
        });
        g.bench_function("dyn_rescale_to", |bn| {
            #[allow(deprecated)]
            bn.iter(|| black_box(erased.rescale_to(black_box($tgt))))
        });
        g.finish();
    }};
}

fn bench(c: &mut Criterion) {
    // D18 and D38 ONLY — see the module docs: `DynDecimal` is emitted
    // for those two widths alone, so there is no wide-tier dyn cell to
    // bench.

    // Identity: no arithmetic, so the dyn row isolates the runtime
    // match + box/unbox overhead from the scale move itself.
    dyn_cell!(c, "D18", D18, 9, 9);
    dyn_cell!(c, "D38", D38, 18, 18);

    // Scale DOWN (divide + round) — the working direction.
    dyn_cell!(c, "D18", D18, 17, 0);
    dyn_cell!(c, "D38", D38, 37, 18);

    // Scale UP (multiply, exact) — the cheap direction, so the facade
    // overhead is the largest FRACTION of the total here.
    dyn_cell!(c, "D18", D18, 0, 17);
    dyn_cell!(c, "D38", D38, 0, 37);
}

fn main() {
    let mut c = micro_criterion().configure_from_args();
    bench(&mut c);
    c.final_summary();
}
