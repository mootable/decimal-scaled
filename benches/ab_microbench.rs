// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Reusable A/B microbench support - the fast pre-check that decides which
//! algorithm a `policy::<fn>::dispatch` `Select` arm should pick.
//!
//! # What this is for
//!
//! Each policy file (`src/int/policy/<fn>.rs`, `src/policy/<fn>.rs`) picks
//! one `Algorithm` arm per storage width via a `const fn select::<N>()`.
//! Choosing the right arm is an empirical question: at width N, is kernel A
//! or kernel B faster? This module is the focused, sub-60-second answer to
//! exactly that question for ONE dispatch - run it BEFORE committing hours
//! to the `library_comparison` / `full_matrix` sweeps.
//!
//! # The model
//!
//! You bring:
//!
//! 1. TWO candidates - closures (or fn pointers) with an identical
//!    signature `Fn(In) -> Out`. These are the two algorithms under test
//!    (two kernel fns, two `Algorithm` arms, two hand-rolled forms). Each
//!    carries a short label.
//! 2. A set of input VALUES - any `IntoIterator` of `In`. You pick whatever
//!    inputs matter for the decision (magnitude buckets, edge operands, a
//!    representative spread).
//! 3. The TWO TYPES - supplied at the call site via turbofish on
//!    `ab_compare` or, more ergonomically, via the `ab_sweep!` macro which
//!    stamps the same candidate/input recipe across a list of types.
//!
//! # Const-fold defeat (critical for this crate)
//!
//! Dispatch in this crate is designed to const-fold: `const { select() }`
//! plus dead-arm elimination collapses each monomorphisation to a direct
//! call. A naive bench of a pure function over a fixed input would let the
//! optimiser hoist the whole computation to a constant and measure nothing.
//! `ab_compare` therefore wraps BOTH the input (before the call) and the
//! output (after) in `black_box`, so the candidate cannot be folded away or
//! dead-code-eliminated. Inputs are cloned per iteration from a
//! `black_box`-ed source so a `Copy` input is re-fed opaquely each time.
//!
//! # Budget
//!
//! `micro_criterion` pins a small sample/measurement window so a full
//! two-candidate x handful-of-inputs comparison finishes in well under a
//! minute on a working machine. This is a pre-check, not a publishable
//! sweep.
//!
//! # Reporting
//!
//! Each candidate is timed per input under its own `BenchmarkId`. Criterion
//! prints the per-candidate point estimate; after the group finishes,
//! `ab_compare` re-times each candidate once over the whole input set with a
//! coarse wall-clock loop and prints a single `A/B verdict` line naming the
//! winner and the ratio, so the decision is legible without parsing the
//! criterion tables.
//!
//! This file is `#![allow(dead_code)]`: it is a support module included by
//! every A/B bench, and any individual bench uses only the pieces it needs.

#![allow(dead_code)]

use criterion::{BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::{Duration, Instant};

/// One labelled algorithm candidate: a name plus the callable under test.
///
/// `In` is the per-iteration input (must be `Clone` so the harness can
/// re-feed an opaque copy each iteration); `Out` is whatever the candidate
/// produces (it is `black_box`-ed, never inspected).
pub struct Candidate<In, Out, F: Fn(In) -> Out> {
    pub label: &'static str,
    pub run: F,
    _io: ::core::marker::PhantomData<fn(In) -> Out>,
}

impl<In, Out, F: Fn(In) -> Out> Candidate<In, Out, F> {
    /// Build a candidate from a label and a callable.
    pub fn new(label: &'static str, run: F) -> Self {
        Self { label, run, _io: ::core::marker::PhantomData }
    }
}

/// A focused criterion config: short warm-up / sample / measurement windows
/// so an A/B comparison stays a sub-minute pre-check rather than a sweep.
///
/// Mirrors the `micro()` helper in `int_ops_micro.rs`.
pub fn micro_criterion() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_millis(150))
        .measurement_time(Duration::from_millis(400))
}

/// Run BOTH candidates over the same input set under criterion, then print
/// an A/B verdict line naming the winner.
///
/// - `group_name` names the criterion benchmark group (e.g. `isqrt/Int512`).
/// - `label_of` turns each input into a short stable string for its
///   `BenchmarkId` (e.g. magnitude bucket).
/// - `inputs` is materialised once into a `Vec`; each input is fed to both
///   candidates.
/// - `a` / `b` are the two `Candidate`s.
///
/// The TWO TYPES under test are encoded in `In` / `Out` and are normally
/// pinned at the call site by turbofish or by the `ab_sweep!` macro.
///
/// Both the input (per iteration, via a `black_box`-ed clone) and the output
/// are opaqued to defeat const-fold / DCE.
pub fn ab_compare<In, Out, FA, FB, L>(
    c: &mut Criterion,
    group_name: &str,
    label_of: L,
    inputs: impl IntoIterator<Item = In>,
    a: Candidate<In, Out, FA>,
    b: Candidate<In, Out, FB>,
) where
    In: Clone,
    FA: Fn(In) -> Out,
    FB: Fn(In) -> Out,
    L: Fn(&In) -> String,
{
    let inputs: Vec<In> = inputs.into_iter().collect();
    assert!(!inputs.is_empty(), "ab_compare: empty input set");

    let mut group = c.benchmark_group(group_name);
    for input in &inputs {
        let id = label_of(input);
        group.bench_with_input(BenchmarkId::new(a.label, &id), input, |bn, input| {
            // black_box the input going IN and the output coming OUT.
            bn.iter(|| black_box((a.run)(black_box(input.clone()))));
        });
        group.bench_with_input(BenchmarkId::new(b.label, &id), input, |bn, input| {
            bn.iter(|| black_box((b.run)(black_box(input.clone()))));
        });
    }
    group.finish();

    // Coarse wall-clock re-time over the whole input set -> a single,
    // human-legible verdict line. Intentionally rough (the precise numbers
    // are criterions above); it exists so the winner is obvious without
    // reading the tables.
    let ta = coarse_time(&inputs, &a.run);
    let tb = coarse_time(&inputs, &b.run);
    let (win, lose, tw, tl) = if ta <= tb {
        (a.label, b.label, ta, tb)
    } else {
        (b.label, a.label, tb, ta)
    };
    let ratio = tl.as_secs_f64() / tw.as_secs_f64().max(f64::MIN_POSITIVE);
    println!(
        "A/B verdict [{group_name}]: {win} beats {lose} by {ratio:.2}x \
         ({win}={tw:?} vs {lose}={tl:?} over {} inputs)",
        inputs.len()
    );
}

/// Coarse repeated-pass timing of one candidate over the whole input set.
/// Used only for the verdict line; criterion owns the precise estimates.
fn coarse_time<In: Clone, Out, F: Fn(In) -> Out>(inputs: &[In], run: &F) -> Duration {
    // A fixed pass count keeps the verdict cheap and bounded; scaled so even
    // ~ns ops accumulate a measurable span.
    const PASSES: u32 = 2_000;
    let start = Instant::now();
    for _ in 0..PASSES {
        for input in inputs {
            black_box(run(black_box(input.clone())));
        }
    }
    start.elapsed()
}

/// Stamp the SAME candidate/input recipe across a list of concrete types -
/// the ergonomic "two (or more) types" entry point.
///
/// Each entry names a type and a closure `|c| { .. }` that receives the
/// `&mut Criterion` and runs one `ab_compare` for that type. This keeps the
/// two-type call site to a couple of lines while the per-type candidate
/// construction (which usually needs the concrete type) stays explicit.
///
/// ```ignore
/// ab_sweep!(c =>
///     Int<16> => |c| ab_compare(c, "mul/Int1024", lbl, inputs(16),
///                                Candidate::new("school", school::<16>),
///                                Candidate::new("kara",   kara::<16>)),
///     Int<32> => |c| ab_compare(c, "mul/Int2048", lbl, inputs(32),
///                                Candidate::new("school", school::<32>),
///                                Candidate::new("kara",   kara::<32>)),
/// );
/// ```
#[macro_export]
macro_rules! ab_sweep {
    ($c:expr => $( $ty:ty => $body:expr ),+ $(,)?) => {{
        $(
            {
                // `$ty` is documentary at the call site; the closure body
                // pins it concretely. Naming it here keeps the per-type
                // intent visible in the source.
                let _type_marker: ::core::marker::PhantomData<$ty> =
                    ::core::marker::PhantomData;
                let f = $body;
                f($c);
            }
        )+
    }};
}
