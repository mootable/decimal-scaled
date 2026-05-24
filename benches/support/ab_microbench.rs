// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Reusable N-way microbench support - the fast pre-check that decides which
//! algorithm a `policy::<fn>::dispatch` `Select` arm should pick.
//!
//! # What this is for
//!
//! Each policy file (`src/int/policy/<fn>.rs`, `src/policy/<fn>.rs`) picks
//! one `Algorithm` arm per storage width via a `const fn select::<N>()`.
//! Choosing the right arm is an empirical question: at width N, 
//! which kernel is fastest? This module is the focused, sub-60-second answer to
//! exactly that question for ONE dispatch - run it BEFORE committing hours
//! to the `library_comparison` / `full_matrix` sweeps.
//!
//! # The model
//!
//! You bring:
//!
//! 1. N >= 2 candidates - closures (or fn pointers) with an identical
//!    signature `Fn(In) -> Out`. These are the algorithms under test
//!    (kernel fns, `Algorithm` arms, hand-rolled forms). Each carries a
//!    short label.
//! 2. A set of input VALUES - any `IntoIterator` of `In`. You pick whatever
//!    inputs matter for the decision (magnitude buckets, edge operands, a
//!    representative spread).
//! 3. The concrete TYPES - supplied at the call site via turbofish on
//!    `compare_all` or, more ergonomically, via the `ab_sweep!` macro which
//!    stamps the same candidate/input recipe across a list of types.
//!
//! # Const-fold defeat (critical for this crate)
//!
//! Dispatch in this crate is designed to const-fold: `const { select() }`
//! plus dead-arm elimination collapses each monomorphisation to a direct
//! call. A naive bench of a pure function over a fixed input would let the
//! optimiser hoist the whole computation to a constant and measure nothing.
//! `compare_all` therefore wraps BOTH the input (before the call) and the
//! output (after) in `black_box`, so the candidate cannot be folded away or
//! dead-code-eliminated. Inputs are cloned per iteration from a
//! `black_box`-ed source so a `Copy` input is re-fed opaquely each time.
//!
//! # Budget
//!
//! `micro_criterion` pins a small sample/measurement window so a full
//! N-candidate x handful-of-inputs comparison finishes in well under a
//! minute on a working machine. For N candidates and the default preset
//! (sample 20, warm 150 ms, measure 400 ms) the total criterion time grows
//! linearly in N * |inputs|; a 4-candidate x 3-input sweep runs in roughly
//! the same wall time as a 2-candidate x 6-input sweep - both finish in
//! seconds. Sub-nanosecond `change:` deltas printed by criterion are noise
//! at this resolution; treat gaps smaller than ~1 ns as effectively tied.
//!
//! # Reporting
//!
//! Each candidate is timed per input under its own `BenchmarkId`. Criterion
//! prints the per-candidate point estimate; after the group finishes,
//! `compare_all` re-times each candidate once over the whole input set with
//! a coarse wall-clock loop, sorts them fastest->slowest, and prints:
//!
//! 1. A ranking table (rank, label, coarse time, ratio vs winner).
//! 2. A single verdict line naming the winner and its margin over the
//!    runner-up (identical format to the old A/B line, so tooling that
//!    greps for it still works).
//!
//! This file is `#![allow(dead_code)]`: it is a support module included by
//! every bench, and any individual bench uses only the pieces it needs.

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
/// so an N-way comparison stays a sub-minute pre-check rather than a sweep.
///
/// Mirrors the `micro()` helper in `int_ops_micro.rs`.
pub fn micro_criterion() -> Criterion {
    Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_millis(150))
        .measurement_time(Duration::from_millis(400))
}

/// Internal: a type-erased candidate used by `compare_all_dyn`.
struct ErasedCandidate<'a, In, Out> {
    label: &'static str,
    run: Box<dyn Fn(In) -> Out + 'a>,
}

/// Core N-way implementation shared by `compare_all` and `ab_compare`.
///
/// Accepts type-erased candidates so the public wrappers can accept
/// heterogeneous concrete function types while sharing a single timed loop.
///
/// # Resolution caveat
///
/// Sub-nanosecond `change:` deltas from criterion are measurement noise at
/// this preset. Treat margins smaller than ~1 % as effectively tied; the
/// ranking table flags them with a "~tie" note.
fn compare_all_dyn<In, Out>(
    c: &mut Criterion,
    group_name: &str,
    label_of: impl Fn(&In) -> String,
    inputs: impl IntoIterator<Item = In>,
    candidates: Vec<ErasedCandidate<'_, In, Out>>,
) where
    In: Clone,
{
    assert!(candidates.len() >= 2, "compare_all: need at least 2 candidates");
    let inputs: Vec<In> = inputs.into_iter().collect();
    assert!(!inputs.is_empty(), "compare_all: empty input set");

    // Criterion group: one BenchmarkId per (candidate x input).
    let mut group = c.benchmark_group(group_name);
    for ec in &candidates {
        for input in &inputs {
            let id = label_of(input);
            group.bench_with_input(BenchmarkId::new(ec.label, &id), input, |bn, input| {
                // black_box the input going IN and the output coming OUT.
                bn.iter(|| black_box((ec.run)(black_box(input.clone()))));
            });
        }
    }
    group.finish();

    // Coarse wall-clock re-time: one entry per candidate.
    let mut timings: Vec<(&'static str, Duration)> = candidates
        .iter()
        .map(|ec| (ec.label, coarse_time_dyn(&inputs, &*ec.run)))
        .collect();

    // Sort fastest -> slowest.
    timings.sort_by_key(|&(_, d)| d);

    let (winner_label, winner_time) = timings[0];
    let (runner_up_label, runner_up_time) = timings[1];
    let margin =
        runner_up_time.as_secs_f64() / winner_time.as_secs_f64().max(f64::MIN_POSITIVE);

    // Ranking table.
    println!(
        "Ranking [{group_name}] ({} candidates, {} inputs):",
        timings.len(),
        inputs.len()
    );
    for (rank, (label, dur)) in timings.iter().enumerate() {
        let ratio = dur.as_secs_f64() / winner_time.as_secs_f64().max(f64::MIN_POSITIVE);
        let note = if rank == 0 {
            " <- winner".to_string()
        } else if ratio < 1.01 {
            " (~tie with winner)".to_string()
        } else {
            format!(" ({ratio:.2}x slower)")
        };
        println!("  #{}: {:20} {:?}{}", rank + 1, label, dur, note);
    }

    // Verdict line - same format as the original A/B line so any tooling
    // that greps for "A/B verdict" continues to work.
    println!(
        "A/B verdict [{group_name}]: {winner_label} beats {runner_up_label} by {margin:.2}x          ({winner_label}={winner_time:?} vs {runner_up_label}={runner_up_time:?} over {} inputs)",
        inputs.len()
    );
}

/// Coarse repeated-pass timing for the ranking/verdict line.
/// Criterion owns the precise per-candidate estimates.
fn coarse_time_dyn<In: Clone, Out>(inputs: &[In], run: &dyn Fn(In) -> Out) -> Duration {
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

/// Run N >= 2 candidates over the same input set under criterion.
///
/// Pass a `Vec` of `(label, fn)` pairs, one per algorithm arm. A third (or
/// fourth) candidate slots in as another entry in the vec:
///
/// ```ignore
/// compare_all(c, "mul/Int1024", lbl, inputs(16), vec![
///     ("school", school_run),
///     ("kara",   kara_run),
///     ("toom3",  toom3_run),   // third candidate - add more the same way
/// ]);
/// ```
///
/// All entries must share the same `In` / `Out` types. The harness
/// `black_box`-guards both inputs and outputs to defeat const-fold/DCE,
/// runs each under `micro_criterion` preset (sample 20, warm 150 ms,
/// measure 400 ms), and prints a ranking table plus a verdict line.
///
/// Sub-nanosecond `change:` deltas are noise; treat gaps below ~1 % as tied.
pub fn compare_all<In, Out, F>(
    c: &mut Criterion,
    group_name: &str,
    label_of: impl Fn(&In) -> String,
    inputs: impl IntoIterator<Item = In>,
    candidates: Vec<(&'static str, F)>,
) where
    In: Clone,
    F: Fn(In) -> Out,
{
    let erased: Vec<ErasedCandidate<'_, In, Out>> = candidates
        .into_iter()
        .map(|(label, run)| ErasedCandidate { label, run: Box::new(run) })
        .collect();
    compare_all_dyn(c, group_name, label_of, inputs, erased);
}

/// Run TWO candidates over the same input set under criterion.
///
/// Thin wrapper around `compare_all_dyn` that preserves the original
/// pairwise [`Candidate`]-based API so existing call sites compile unchanged.
/// Prefer [`compare_all`] with a two-entry vec for new code.
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
    let erased: Vec<ErasedCandidate<'_, In, Out>> = vec![
        ErasedCandidate { label: a.label, run: Box::new(a.run) },
        ErasedCandidate { label: b.label, run: Box::new(b.run) },
    ];
    compare_all_dyn(c, group_name, label_of, inputs, erased);
}

/// Stamp the SAME candidate/input recipe across a list of concrete types.
///
/// Each entry names a type and a closure `|c| { .. }` that receives the
/// `&mut Criterion` and runs one `compare_all` (or `ab_compare`) for that
/// type. This keeps the multi-type call site to a couple of lines while
/// the per-type candidate construction (which usually needs the concrete
/// type) stays explicit.
///
/// ```ignore
/// ab_sweep!(c =>
///     Int<16> => |c| compare_all(c, "mul/Int1024", lbl, inputs(16), vec![
///                        ("school", school::<16>),
///                        ("kara",   kara::<16>)]),
///     Int<32> => |c| compare_all(c, "mul/Int2048", lbl, inputs(32), vec![
///                        ("school", school::<32>),
///                        ("kara",   kara::<32>)]),
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
