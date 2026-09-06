//! `bbc_probe` — the SYNTHETIC probe series for `bench-branch-compare`.
//!
//! One question, asked of the runner and of nothing else: what is the smallest
//! difference this machine can actually resolve? Its companion series
//! (`examples/bbc_pinned_probe.rs`) asks the other half — whether our own
//! baseline moved — and the two are reported separately, never blended. When a
//! merged figure moves you cannot tell a slower runner from slower code, and
//! removing exactly that ambiguity is what this series exists for.
//!
//! # Why it exists
//!
//! `summarise.py` puts a cell in the regression focus when `Δ% > +1` AND
//! `Δ > 10 ns`. The second arm is a FIXED ABSOLUTE floor applied across cells
//! whose cost spans three orders of magnitude. On a cell reporting ~1 ns/op,
//! 10 ns is a 1000% change — so the cheapest operations are unassessable by
//! construction: a real 50% regression on `add` is filtered out and footnoted
//! as sitting below the measurement floor, which it does not.
//!
//! Nobody had ever measured that floor. `10 ns` was a guess. This probe
//! replaces the guess with a number.
//!
//! # Why nothing from the crate appears here
//!
//! A probe built on one of our own operations measures the runner AND our code
//! at once. A synthetic reference stays comparable across every future run
//! whatever the library does, which is the whole point of having one.
//!
//! That is enforced by the BUILD, not by this comment. The file is compiled
//! with `rustc` directly (see `bench-branch-compare.yml`, "Compile the
//! synthetic timing probe") with no manifest, no dependency graph and no
//! `extern crate` — so it CANNOT link `decimal-scaled` or `prod` even by
//! accident, and it cannot be perturbed by the run's tier feature set.
//!
//! # What is measured
//!
//! **The clock** — its tick and its read cost; see `probe_core::probe_clock`.
//!
//! **A cost ladder** — eight rungs of the same synthetic kernel at
//! geometrically increasing work per operation, spanning roughly 2 ns to 30 µs,
//! the range the sweep's own cells occupy. Each rung is measured the way
//! criterion measures a cell, and that whole measurement is then repeated as
//! independent replicates; their spread is the deliverable. See `probe_core`
//! for the methodology, which the pinned series shares so the two are directly
//! comparable.
//!
//! The reported per-operation figure cannot itself tell you the resolution: at
//! ~2 ns/op it is an amortised cost derived from hundreds of thousands of
//! iterations between two timestamps, so it legitimately sits far BELOW the
//! timer's own tick. Only repeating the identical measurement can, which is
//! what the replicates are, and conflating the two is the trap this is built to
//! avoid.
//!
//! # Invariants that keep the ladder comparable across runs
//!
//! * The kernel is a SERIAL DEPENDENCY CHAIN — each step consumes the previous
//!   step's result — so its cost is set by instruction latency rather than by
//!   how the scheduler arranges independent work. That is deliberately the part
//!   of a CPU's behaviour most stable across microarchitectures and least
//!   sensitive to how a future compiler lays the loop out.
//! * The multiply is followed by an xor-shift. Without it a run of constant
//!   multiplies collapses: `x·K` applied `n` times is `x·Kⁿ`, one multiply, and
//!   every rung would cost the same. The xor-shift is not decoration.
//! * `kernel` is `#[inline(never)]`, so the measured shape cannot drift with
//!   the optimiser's mood. It costs a call per operation, which is why the
//!   finest rung lands near 2 ns rather than 1 ns — and near 2 ns is exactly
//!   where the cells this probe exists to rescue already sit.
//! * `rustc` is invoked with explicit pinned flags and no `-C target-cpu`, so
//!   codegen is the generic x86-64 baseline and does not shift when the hosted
//!   runner pool changes hardware. The build string travels in the output, so a
//!   ladder that moves because the TOOLCHAIN moved is attributable.
//!
//! # Usage
//!
//! ```text
//! bbc_probe <start|end> <job-name> <out-dir>
//! ```

mod probe_core;

use std::env;
use std::hint::black_box;

/// The ladder: inner repetitions per operation. Geometric in steps of 4, so
/// eight rungs cover ~2 ns to ~30 µs — the span of the sweep's own cells, from
/// `add` at D18 to the wide transcendentals.
const RUNG_REPS: [u32; 8] = [1, 4, 16, 64, 256, 1_024, 4_096, 16_384];

/// Multiplier from PCG / `splitmix64`. Any odd constant with well-mixed bits
/// does; this one is used because it is a recognisable published value rather
/// than an invented one.
const MUL: u64 = 6_364_136_223_846_793_005;

/// The unit of synthetic work: one 64-bit multiply and one xor-shift, repeated
/// `reps` times through a single accumulator. Serial by construction, so the
/// cost is `reps` × (multiply latency + shift + xor) and not something the
/// scheduler can overlap away.
#[inline(never)]
fn kernel(mut acc: u64, reps: u32) -> u64 {
    for _ in 0..reps {
        acc = acc.wrapping_mul(MUL);
        acc ^= acc >> 29;
    }
    acc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: bbc_probe <start|end> <job-name> <out-dir>");
        // Exit 0 even here: the probe is never allowed to red a bench cell.
        return;
    }
    let (position, job, out_dir) = (&args[1], &args[2], &args[3]);

    // Filled by the workflow with `rustc -V` and the exact codegen flags, so a
    // ladder that shifts because the toolchain moved is attributable. Empty
    // when run by hand, which is honest rather than a fake value.
    let build = env::var("BBC_PROBE_BUILD").unwrap_or_default();

    let clock = probe_core::probe_clock();

    let entries: Vec<probe_core::Entry> = RUNG_REPS
        .iter()
        .map(|&reps| {
            // `reps` is laundered OUTSIDE the timed region, so the inner loop
            // bound is opaque to the optimiser — no unrolling to a constant, no
            // collapsing the rungs into each other — without paying for the
            // barrier on every iteration. `acc` is laundered INSIDE it, which
            // is what stops the outer loop being hoisted or vectorised: the
            // value genuinely has to make the round trip each time.
            let reps = black_box(reps);
            let mut acc: u64 = black_box(0x9E37_79B9_7F4A_7C15);
            let entry = probe_core::measure(&format!("r{reps}"), || {
                acc = kernel(black_box(acc), reps);
            });
            black_box(acc);
            entry
        })
        .collect();

    probe_core::emit(
        "synthetic",
        "mul+xorshift chain (no crate code)",
        position,
        job,
        out_dir,
        &build,
        &clock,
        &entries,
    );
}
