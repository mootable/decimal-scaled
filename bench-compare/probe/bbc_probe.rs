//! `bbc_probe` — a synthetic timing probe for `bench-branch-compare`.
//!
//! Two questions, asked of the runner and of nothing else: what is the smallest
//! interval this machine can resolve, and how far apart do two measurements of
//! IDENTICAL work land?
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
//! Nobody has ever measured that floor. `10 ns` is a guess. This probe replaces
//! the guess with a number.
//!
//! # Why synthetic, and why nothing from the crate appears here
//!
//! A probe built on one of our own operations measures the runner AND our code
//! at once, so when it moves you cannot tell which of the two moved. A
//! synthetic reference stays comparable across every future run whatever the
//! library does, which is the whole point of having one.
//!
//! That is enforced by the BUILD, not by this comment. The file is compiled
//! with `rustc` directly (see `bench-branch-compare.yml`, the "Compile the
//! timing probe" step) with no manifest, no dependency graph and no `extern
//! crate` — so it CANNOT link `decimal-scaled` or `prod` even by accident, and
//! it cannot be perturbed by the run's tier feature set. It is also why the
//! probe is not a `[[bench]]`/`[[bin]]` target: adding one would put it in
//! `bench-compare`'s dependency graph, which is exactly the coupling being
//! avoided.
//!
//! # What is actually measured
//!
//! **The clock.** `Instant::now()` read back to back, many times. The smallest
//! NON-ZERO delta is the tick — the granularity in the literal sense — and the
//! median delta is what one reading costs. These are different numbers and both
//! matter: a tick of 1 ns with a 25 ns call cost behaves very differently from
//! a 25 ns tick.
//!
//! **The cost ladder.** Eight rungs of the same synthetic kernel at
//! geometrically increasing work per operation, spanning roughly 2 ns to 30 µs
//! — the range the sweep's own cells occupy. Each rung is measured the way
//! criterion measures a cell (a loop of `iters` operations between two clock
//! reads, per-operation = elapsed ÷ iters, median over samples), and that whole
//! measurement is then REPEATED as independent replicates.
//!
//! The spread across those replicates is the quantity the filter needs. It is
//! not the same thing as the clock's resolution, and conflating the two is the
//! trap this probe is built to avoid: at ~1 ns/op the reported figure is an
//! amortised per-iteration cost derived from hundreds of thousands of
//! iterations between two timestamps, so it legitimately sits far BELOW the
//! timer's own tick. The reported value therefore cannot tell you the
//! resolution. Only repeating the identical measurement can, which is what the
//! replicates are.
//!
//! # Where it runs
//!
//! At the START and at the END of every bench job. The pair is the only thing
//! that reveals within-job drift — whether the runner degraded while the sweep
//! ran. A single reading cannot distinguish a slow machine from a machine that
//! became slow.
//!
//! # Invariants that keep readings comparable across runs
//!
//! * The kernel is a SERIAL DEPENDENCY CHAIN — each step needs the previous
//!   step's result — so its cost is set by instruction latency rather than by
//!   how the scheduler arranges independent work. That is deliberately the part
//!   of a CPU's behaviour that is most stable across microarchitectures and
//!   least sensitive to how a future compiler decides to lay the loop out.
//! * The multiply is followed by an xor-shift. Without it a run of constant
//!   multiplies collapses: `x·K` applied `n` times is `x·Kⁿ`, one multiply, and
//!   the ladder's rungs would all cost the same. The xor-shift is not
//!   decoration.
//! * `kernel` is `#[inline(never)]`, so the outer loop's shape cannot change
//!   with the optimiser's mood. It costs a call per operation, which is why the
//!   finest rung lands near 2 ns rather than 1 ns — and near 2 ns is exactly
//!   where the cells this probe exists to rescue already sit.
//! * `rustc` is invoked with explicit, pinned flags and no `-C target-cpu`, so
//!   codegen is the generic x86-64 baseline and does not shift when GitHub
//!   changes runner hardware. The build string is recorded in the output, so a
//!   toolchain change that DOES move the ladder is attributable rather than
//!   mysterious.
//!
//! # Contract
//!
//! It NEVER fails the job. Every error path prints and exits 0. A probe that
//! can red a bench cell would trade a quality annotation for a data outage —
//! and the cell's branch÷prod ratio is still perfectly good even on a runner
//! that drifted, because both sides were measured in the same job seconds
//! apart.
//!
//! # Usage
//!
//! ```text
//! bbc_probe <position> <job-name> <out-dir>
//! ```
//!
//! `position` is `start` or `end`; `job-name` is the matrix entry's name
//! (`D18-s0`, `D57-all`, …); `out-dir` receives `<job>-<position>.json`.

use std::env;
use std::fs;
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

// ── Configuration ────────────────────────────────────────────────────────
//
// Sized so the whole probe runs in well under a second. At two invocations per
// job across a 60-job sweep that is ~1 minute of billed time for the entire
// run, against bench cells budgeted at 20 minutes each.

/// Back-to-back `Instant::now()` reads for the clock characterisation.
const CLOCK_SAMPLES: usize = 20_000;

/// The ladder: inner repetitions per operation. Geometric in steps of 4, so
/// eight rungs cover ~2 ns to ~30 µs — the span of the sweep's own cells, from
/// `add` at D18 to the wide transcendentals.
const RUNG_REPS: [u32; 8] = [1, 4, 16, 64, 256, 1_024, 4_096, 16_384];

/// Independent repeats of the WHOLE measurement per rung. Their spread is the
/// deliverable: it is what "measure this cell twice and compare" actually does.
const REPLICATES: usize = 9;

/// Samples inside one replicate. Their spread is criterion's own within-run
/// variability, reported alongside for contrast — it is systematically smaller
/// than the replicate spread, which is precisely why reading a single run's
/// confidence interval understates how reproducible a cell is.
const SAMPLES_PER_REPLICATE: usize = 11;

/// Wall time one sample aims for. Long enough that the clock's own call cost is
/// amortised into invisibility (at the finest rung, ~25 ns spread over hundreds
/// of thousands of iterations), short enough to keep the probe cheap.
const SAMPLE_TARGET_NS: u64 = 500_000;

/// Hard stop on the calibration ramp, so a pathologically slow clock cannot
/// spin here indefinitely.
const ITERS_CAP: u64 = 1 << 32;

/// Multiplier from PCG / `splitmix64`. Any odd constant with well-mixed bits
/// does; this one is used because it is a recognisable published value rather
/// than an invented one.
const MUL: u64 = 6_364_136_223_846_793_005;

/// The unit of synthetic work: one 64-bit multiply and one xor-shift, repeated
/// `reps` times through a single accumulator.
///
/// Serial by construction — every step consumes the previous step's output — so
/// the cost is `reps` × (multiply latency + shift + xor), not something the
/// scheduler can overlap away. `#[inline(never)]` fixes the call boundary so
/// the measured shape does not drift with the optimiser.
#[inline(never)]
fn kernel(mut acc: u64, reps: u32) -> u64 {
    for _ in 0..reps {
        acc = acc.wrapping_mul(MUL);
        acc ^= acc >> 29;
    }
    acc
}

/// One timed batch: `iters` operations of `reps` work each, between two clock
/// reads. Returns the elapsed nanoseconds for the whole batch.
///
/// `reps` is laundered through `black_box` OUTSIDE the timed region, so the
/// inner loop bound is opaque to the optimiser (no unrolling to a constant, no
/// collapsing the rungs into each other) without paying for the barrier on
/// every iteration. `acc` is laundered INSIDE it, which is what stops the outer
/// loop being hoisted or vectorised — the value genuinely has to make the round
/// trip each time.
fn measure(reps: u32, iters: u64) -> u64 {
    let reps = black_box(reps);
    let mut acc: u64 = black_box(0x9E37_79B9_7F4A_7C15);
    let t0 = Instant::now();
    for _ in 0..iters {
        acc = kernel(black_box(acc), reps);
    }
    let elapsed = t0.elapsed();
    black_box(acc);
    elapsed.as_nanos() as u64
}

/// Find the iteration count that makes one sample last `SAMPLE_TARGET_NS`, the
/// way criterion sizes its own samples. Doubles as the rung's warm-up: by the
/// time this returns, the code is in i-cache and the branch predictors are
/// trained, so the first replicate is not systematically slower than the rest.
fn calibrate(reps: u32) -> u64 {
    let mut iters: u64 = 1;
    loop {
        let ns = measure(reps, iters);
        if ns >= SAMPLE_TARGET_NS || iters >= ITERS_CAP {
            return iters;
        }
        // Growth is clamped: at least 2 so the ramp always advances even when
        // the reading is noise, at most 64 so a sub-tick reading (`ns == 0` at
        // the finest rung, where one operation is far below the clock's own
        // granularity) cannot leap straight to the cap.
        let grow = (SAMPLE_TARGET_NS / ns.max(1)).clamp(2, 64);
        iters = iters.saturating_mul(grow).min(ITERS_CAP);
    }
}

// ── Statistics ───────────────────────────────────────────────────────────

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v
}

/// Linear-interpolated quantile over an ALREADY-SORTED slice.
fn quantile(s: &[f64], q: f64) -> f64 {
    if s.is_empty() {
        return f64::NAN;
    }
    let pos = q * (s.len() - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    if lo == hi {
        s[lo]
    } else {
        let frac = pos - lo as f64;
        s[lo] * (1.0 - frac) + s[hi] * frac
    }
}

// ── The two measurements ─────────────────────────────────────────────────

struct Clock {
    zero_reads: usize,
    min_nonzero_ns: f64,
    median_ns: f64,
    p99_ns: f64,
}

/// Characterise the clock itself: read it back to back and look at the deltas.
///
/// The smallest non-zero delta is the TICK — the finest interval the clock can
/// represent. The median delta is what one read COSTS. A clock with a 1 ns tick
/// and a 25 ns read cost reports zero deltas essentially never and a median of
/// 25; a clock with a coarse tick reports a run of zeros and then a jump. The
/// count of zero reads is what distinguishes the two cases, so it is kept.
fn probe_clock() -> Clock {
    let mut deltas: Vec<f64> = Vec::with_capacity(CLOCK_SAMPLES);
    let mut prev = Instant::now();
    for _ in 0..CLOCK_SAMPLES {
        let now = Instant::now();
        deltas.push(now.duration_since(prev).as_nanos() as f64);
        prev = now;
    }
    let zero_reads = deltas.iter().filter(|d| **d == 0.0).count();
    let nonzero = sorted(deltas.iter().copied().filter(|d| *d > 0.0).collect());
    let all = sorted(deltas);
    Clock {
        zero_reads,
        min_nonzero_ns: nonzero.first().copied().unwrap_or(f64::NAN),
        median_ns: quantile(&all, 0.5),
        p99_ns: quantile(&all, 0.99),
    }
}

struct Rung {
    reps: u32,
    iters: u64,
    /// Per-replicate medians — the values a repeated measurement would report.
    median_ns: f64,
    min_ns: f64,
    max_ns: f64,
    p10_ns: f64,
    p90_ns: f64,
    /// `(p90 − p10) / median` across replicates. THE number the filter needs:
    /// the relative disagreement between two measurements of identical work.
    spread_rel: f64,
    /// `(max − min) / median` across replicates — the worst pair seen.
    range_rel: f64,
    /// Median over replicates of that replicate's OWN `(p90 − p10) / median`
    /// across its samples. This is criterion's within-run variability; it is
    /// reported for contrast because it is the number a single run's confidence
    /// interval reflects, and it understates `spread_rel`.
    within_spread_rel: f64,
}

fn probe_rung(reps: u32) -> Rung {
    let iters = calibrate(reps);
    let mut replicate_medians: Vec<f64> = Vec::with_capacity(REPLICATES);
    let mut within: Vec<f64> = Vec::with_capacity(REPLICATES);

    for _ in 0..REPLICATES {
        let mut samples: Vec<f64> = Vec::with_capacity(SAMPLES_PER_REPLICATE);
        for _ in 0..SAMPLES_PER_REPLICATE {
            samples.push(measure(reps, iters) as f64 / iters as f64);
        }
        let samples = sorted(samples);
        // The median per-operation time, which is exactly the statistic
        // `summarise.py` reads out of criterion (`median.point_estimate`). The
        // probe must report the same statistic or it is characterising the
        // spread of something the table does not use.
        let m = quantile(&samples, 0.5);
        replicate_medians.push(m);
        within.push(if m > 0.0 {
            (quantile(&samples, 0.9) - quantile(&samples, 0.1)) / m
        } else {
            f64::NAN
        });
    }

    let r = sorted(replicate_medians);
    let median_ns = quantile(&r, 0.5);
    let p10_ns = quantile(&r, 0.10);
    let p90_ns = quantile(&r, 0.90);
    let min_ns = r.first().copied().unwrap_or(f64::NAN);
    let max_ns = r.last().copied().unwrap_or(f64::NAN);
    let w = sorted(within);

    Rung {
        reps,
        iters,
        median_ns,
        min_ns,
        max_ns,
        p10_ns,
        p90_ns,
        spread_rel: if median_ns > 0.0 { (p90_ns - p10_ns) / median_ns } else { f64::NAN },
        range_rel: if median_ns > 0.0 { (max_ns - min_ns) / median_ns } else { f64::NAN },
        within_spread_rel: quantile(&w, 0.5),
    }
}

// ── Runner identity ──────────────────────────────────────────────────────

/// CPU model and core count, read from `/proc/cpuinfo`.
///
/// Worth carrying: GitHub's hosted pool is not homogeneous, and a rung median
/// that shifts between runs is explained very differently depending on whether
/// the CPU model shifted with it.
fn cpu_identity() -> (String, usize) {
    let Ok(text) = fs::read_to_string("/proc/cpuinfo") else {
        return ("unknown".to_string(), 0);
    };
    let model = text
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.split_once(':'))
        .map(|(_, v)| v.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let cores = text.lines().filter(|l| l.starts_with("processor")).count();
    (model, cores)
}

// ── JSON emission (hand-rolled: the probe has no dependencies) ────────────

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// `NaN`/`Infinity` are not JSON. Emit `null` rather than an unparseable
/// document — a consumer can skip a null, it cannot skip a syntax error.
fn json_num(v: f64) -> String {
    if v.is_finite() { format!("{v:.6}") } else { "null".to_string() }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: bbc_probe <start|end> <job-name> <out-dir>");
        // Exit 0 even here: the probe is never allowed to red a bench cell.
        return;
    }
    let position = args[1].clone();
    let job = args[2].clone();
    let out_dir = args[3].clone();

    // Filled by the workflow with `rustc -V` and the exact codegen flags, so a
    // ladder that shifts because the toolchain moved is attributable. Empty
    // when run by hand, which is honest rather than a fake value.
    let build = env::var("BBC_PROBE_BUILD").unwrap_or_default();

    let clock = probe_clock();
    let rungs: Vec<Rung> = RUNG_REPS.iter().map(|r| probe_rung(*r)).collect();
    let (cpu_model, cpu_cores) = cpu_identity();

    let mut j = String::with_capacity(4096);
    j.push_str("{\n");
    // Versioned so a later change to what is measured is detectable rather than
    // silently mixed into a history of the old shape.
    j.push_str("  \"schema\": \"bbc-probe/1\",\n");
    j.push_str(&format!("  \"position\": {},\n", json_string(&position)));
    j.push_str(&format!("  \"job\": {},\n", json_string(&job)));
    j.push_str(&format!("  \"build\": {},\n", json_string(&build)));
    j.push_str(&format!("  \"cpu_model\": {},\n", json_string(&cpu_model)));
    j.push_str(&format!("  \"cpu_cores\": {cpu_cores},\n"));
    j.push_str("  \"clock\": {\n");
    j.push_str(&format!("    \"reads\": {CLOCK_SAMPLES},\n"));
    j.push_str(&format!("    \"zero_reads\": {},\n", clock.zero_reads));
    j.push_str(&format!("    \"min_nonzero_ns\": {},\n", json_num(clock.min_nonzero_ns)));
    j.push_str(&format!("    \"median_ns\": {},\n", json_num(clock.median_ns)));
    j.push_str(&format!("    \"p99_ns\": {}\n", json_num(clock.p99_ns)));
    j.push_str("  },\n");
    j.push_str(&format!("  \"replicates\": {REPLICATES},\n"));
    j.push_str(&format!("  \"samples_per_replicate\": {SAMPLES_PER_REPLICATE},\n"));
    j.push_str("  \"ladder\": [\n");
    for (i, r) in rungs.iter().enumerate() {
        j.push_str("    {");
        j.push_str(&format!("\"reps\": {}, ", r.reps));
        j.push_str(&format!("\"iters\": {}, ", r.iters));
        j.push_str(&format!("\"median_ns\": {}, ", json_num(r.median_ns)));
        j.push_str(&format!("\"min_ns\": {}, ", json_num(r.min_ns)));
        j.push_str(&format!("\"max_ns\": {}, ", json_num(r.max_ns)));
        j.push_str(&format!("\"p10_ns\": {}, ", json_num(r.p10_ns)));
        j.push_str(&format!("\"p90_ns\": {}, ", json_num(r.p90_ns)));
        j.push_str(&format!("\"spread_rel\": {}, ", json_num(r.spread_rel)));
        j.push_str(&format!("\"range_rel\": {}, ", json_num(r.range_rel)));
        j.push_str(&format!("\"within_spread_rel\": {}", json_num(r.within_spread_rel)));
        j.push_str(if i + 1 == rungs.len() { "}\n" } else { "},\n" });
    }
    j.push_str("  ]\n}\n");

    // A compact human line in the job log, so a reading is visible without
    // downloading an artifact.
    println!("bbc_probe [{position}] job={job} cpu={cpu_model} cores={cpu_cores}");
    println!(
        "  clock: tick {} ns, read {} ns (median), {} zero reads of {}",
        json_num(clock.min_nonzero_ns),
        json_num(clock.median_ns),
        clock.zero_reads,
        CLOCK_SAMPLES
    );
    for r in &rungs {
        println!(
            "  rung reps={:>5}: {:>12} ns/op   replicate spread {:>7} %   (within-run {} %)",
            r.reps,
            json_num(r.median_ns),
            json_num(r.spread_rel * 100.0),
            json_num(r.within_spread_rel * 100.0),
        );
    }

    // Never fail the job — report the problem and leave the bench data alone.
    let dir = Path::new(&out_dir);
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("bbc_probe: could not create {out_dir}: {e} — readings not written");
        return;
    }
    let path = dir.join(format!("{job}-{position}.json"));
    match fs::write(&path, &j) {
        Ok(()) => println!("bbc_probe: wrote {}", path.display()),
        Err(e) => eprintln!("bbc_probe: could not write {}: {e}", path.display()),
    }
}
