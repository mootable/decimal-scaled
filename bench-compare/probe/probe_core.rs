//! Shared measurement methodology for the two bbc probe series.
//!
//! There are TWO probes and they answer different questions:
//!
//!   * **synthetic** (`probe/bbc_probe.rs`) — what interval can this runner
//!     actually resolve? No crate operation appears in it, so it stays
//!     comparable across every future run regardless of what the library does.
//!
//!   * **pinned** (`examples/bbc_pinned_probe.rs`) — did OUR OWN baseline move?
//!     It times one library operation chosen because its cost has no reason to
//!     change.
//!
//! They must never be blended. When a merged figure moves you cannot tell a
//! slower runner from slower code, and removing exactly that ambiguity is what
//! the synthetic series is for. They share this file so the two are measured
//! IDENTICALLY — same calibration, same replicate structure, same statistic —
//! because a difference in method would otherwise masquerade as a difference in
//! result.
//!
//! What they cannot share is a binary. The synthetic is compiled by `rustc`
//! directly with no manifest and no dependency graph, which is what makes "no
//! crate operations" a property of the build rather than a promise in a
//! comment; the pinned probe must link the crate. So: one methodology, two
//! executables.
//!
//! # The statistic, and why it is this one
//!
//! `summarise.py` reads criterion's `median.point_estimate`. So a replicate
//! here is the MEDIAN per-operation time over a set of samples — the same
//! statistic the regression table consumes. Measuring the spread of a different
//! statistic would characterise something the table does not use.
//!
//! The deliverable is the spread ACROSS REPLICATES: independent repeats of the
//! whole measurement. That is the null distribution for branch÷prod, because
//! branch and prod are measured exactly that way — same job, same process,
//! seconds apart. The spread across samples WITHIN one replicate is reported
//! alongside for contrast; it is what a single run's confidence interval
//! reflects, and it is systematically smaller, which is why reading it alone
//! overstates how reproducible a cell is.

#![allow(dead_code)] // Each binary uses a subset; the module serves both.

use std::fs;
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

// ── Configuration, shared so both series are directly comparable ─────────

/// Back-to-back `Instant::now()` reads for the clock characterisation.
pub const CLOCK_SAMPLES: usize = 20_000;

/// Independent repeats of the WHOLE measurement. Their spread is the
/// deliverable.
pub const REPLICATES: usize = 9;

/// Samples inside one replicate, from which that replicate's median is taken.
pub const SAMPLES_PER_REPLICATE: usize = 11;

/// Wall time one sample aims for. Long enough that the clock's own read cost
/// amortises into invisibility — at ~2 ns/op that is one ~25 ns read spread
/// over a quarter of a million operations — and short enough to keep the whole
/// probe under a second.
pub const SAMPLE_TARGET_NS: u64 = 500_000;

/// Hard stop on the calibration ramp, so a pathological clock cannot spin here.
pub const ITERS_CAP: u64 = 1 << 32;

// ── One measured workload ────────────────────────────────────────────────

/// One workload's readings: a rung of the synthetic ladder, or the pinned
/// operation.
pub struct Entry {
    pub label: String,
    pub iters: u64,
    /// Median of the per-replicate medians — the value a repeated measurement
    /// would report.
    pub median_ns: f64,
    pub min_ns: f64,
    pub max_ns: f64,
    pub p10_ns: f64,
    pub p90_ns: f64,
    /// `(p90 − p10) / median` ACROSS replicates. The number the regression
    /// filter needs.
    pub spread_rel: f64,
    /// `(max − min) / median` across replicates — the worst pair observed.
    pub range_rel: f64,
    /// Median over replicates of that replicate's own `(p90 − p10) / median`
    /// across its samples — criterion's within-run variability, for contrast.
    pub within_spread_rel: f64,
}

fn timed(iters: u64, op: &mut impl FnMut()) -> u64 {
    let t0 = Instant::now();
    for _ in 0..iters {
        op();
    }
    t0.elapsed().as_nanos() as u64
}

/// Find the iteration count that makes one sample last [`SAMPLE_TARGET_NS`],
/// the way criterion sizes its own samples. Doubles as warm-up: by the time it
/// returns, the code is in i-cache and the predictors are trained, so the first
/// replicate is not systematically slower than the rest.
fn calibrate(op: &mut impl FnMut()) -> u64 {
    let mut iters: u64 = 1;
    loop {
        let ns = timed(iters, op);
        if ns >= SAMPLE_TARGET_NS || iters >= ITERS_CAP {
            return iters;
        }
        // Clamped growth: at least 2 so the ramp always advances even when the
        // reading is noise, at most 64 so a sub-tick reading (`ns == 0` where
        // one operation is far below the clock's granularity) cannot leap
        // straight to the cap.
        let grow = (SAMPLE_TARGET_NS / ns.max(1)).clamp(2, 64);
        iters = iters.saturating_mul(grow).min(ITERS_CAP);
    }
}

/// Measure one workload: calibrate, then [`REPLICATES`] independent repeats of
/// [`SAMPLES_PER_REPLICATE`] samples each.
pub fn measure<F: FnMut()>(label: &str, mut op: F) -> Entry {
    let iters = calibrate(&mut op);
    let mut replicate_medians: Vec<f64> = Vec::with_capacity(REPLICATES);
    let mut within: Vec<f64> = Vec::with_capacity(REPLICATES);

    for _ in 0..REPLICATES {
        let mut samples: Vec<f64> = Vec::with_capacity(SAMPLES_PER_REPLICATE);
        for _ in 0..SAMPLES_PER_REPLICATE {
            samples.push(timed(iters, &mut op) as f64 / iters as f64);
        }
        let samples = sorted(samples);
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
    let w = sorted(within);

    Entry {
        label: label.to_string(),
        iters,
        median_ns,
        min_ns: r.first().copied().unwrap_or(f64::NAN),
        max_ns: r.last().copied().unwrap_or(f64::NAN),
        p10_ns,
        p90_ns,
        spread_rel: if median_ns > 0.0 { (p90_ns - p10_ns) / median_ns } else { f64::NAN },
        range_rel: if median_ns > 0.0 {
            (r.last().copied().unwrap_or(f64::NAN) - r.first().copied().unwrap_or(f64::NAN))
                / median_ns
        } else {
            f64::NAN
        },
        within_spread_rel: quantile(&w, 0.5),
    }
}

// ── Statistics ───────────────────────────────────────────────────────────

pub fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v
}

/// Linear-interpolated quantile over an ALREADY-SORTED slice.
pub fn quantile(s: &[f64], q: f64) -> f64 {
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

// ── The clock ────────────────────────────────────────────────────────────

pub struct Clock {
    pub zero_reads: usize,
    pub min_nonzero_ns: f64,
    pub median_ns: f64,
    pub p99_ns: f64,
}

/// Characterise the clock: read it back to back and look at the deltas.
///
/// The smallest non-zero delta is the TICK — the finest interval the clock can
/// represent. The median delta is what one read COSTS. These are different
/// numbers and both matter, and NEITHER is the floor under the regression
/// filter: a benched figure is an amortised per-iteration cost derived from a
/// whole loop between two reads, so it legitimately sits far below both. The
/// zero-read count is what separates a fine tick from a coarse one.
pub fn probe_clock() -> Clock {
    let mut deltas: Vec<f64> = Vec::with_capacity(CLOCK_SAMPLES);
    let mut prev = Instant::now();
    for _ in 0..CLOCK_SAMPLES {
        let now = Instant::now();
        deltas.push(now.duration_since(prev).as_nanos() as f64);
        prev = now;
    }
    black_box(&deltas);
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

// ── Runner identity ──────────────────────────────────────────────────────

/// CPU model and core count from `/proc/cpuinfo`. Worth carrying: the hosted
/// pool is not homogeneous, and a reading that shifts between runs is explained
/// very differently depending on whether the CPU model shifted with it.
pub fn cpu_identity() -> (String, usize) {
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
pub fn json_num(v: f64) -> String {
    if v.is_finite() { format!("{v:.6}") } else { "null".to_string() }
}

/// Write one series' readings and echo a compact summary to the job log.
///
/// `series` is `synthetic` or `pinned`; `subject` names WHAT was timed and is
/// the pinned series' whole protection — it travels with every reading, so a
/// change of subject is visible in the data rather than inferred from a commit.
///
/// NEVER FAILS. Every error path reports and returns. A probe that can red a
/// bench cell trades a quality annotation for a data outage, and a drifted
/// job's branch÷prod ratio is still sound because both sides ran seconds apart
/// inside the same criterion group.
pub fn emit(
    series: &str,
    subject: &str,
    position: &str,
    job: &str,
    out_dir: &str,
    build: &str,
    clock: &Clock,
    entries: &[Entry],
) {
    let (cpu_model, cpu_cores) = cpu_identity();

    let mut j = String::with_capacity(4096);
    j.push_str("{\n");
    // Versioned, so a later change to WHAT is measured is detectable rather
    // than silently mixed into a history of the old shape.
    j.push_str("  \"schema\": \"bbc-probe/1\",\n");
    j.push_str(&format!("  \"series\": {},\n", json_string(series)));
    j.push_str(&format!("  \"subject\": {},\n", json_string(subject)));
    j.push_str(&format!("  \"position\": {},\n", json_string(position)));
    j.push_str(&format!("  \"job\": {},\n", json_string(job)));
    j.push_str(&format!("  \"build\": {},\n", json_string(build)));
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
    j.push_str("  \"entries\": [\n");
    for (i, e) in entries.iter().enumerate() {
        j.push_str("    {");
        j.push_str(&format!("\"label\": {}, ", json_string(&e.label)));
        j.push_str(&format!("\"iters\": {}, ", e.iters));
        j.push_str(&format!("\"median_ns\": {}, ", json_num(e.median_ns)));
        j.push_str(&format!("\"min_ns\": {}, ", json_num(e.min_ns)));
        j.push_str(&format!("\"max_ns\": {}, ", json_num(e.max_ns)));
        j.push_str(&format!("\"p10_ns\": {}, ", json_num(e.p10_ns)));
        j.push_str(&format!("\"p90_ns\": {}, ", json_num(e.p90_ns)));
        j.push_str(&format!("\"spread_rel\": {}, ", json_num(e.spread_rel)));
        j.push_str(&format!("\"range_rel\": {}, ", json_num(e.range_rel)));
        j.push_str(&format!("\"within_spread_rel\": {}", json_num(e.within_spread_rel)));
        j.push_str(if i + 1 == entries.len() { "}\n" } else { "},\n" });
    }
    j.push_str("  ]\n}\n");

    println!("bbc_probe [{series}/{position}] job={job} subject={subject}");
    println!("  cpu={cpu_model} cores={cpu_cores}");
    println!(
        "  clock: tick {} ns, read {} ns (median), {} zero reads of {}",
        json_num(clock.min_nonzero_ns),
        json_num(clock.median_ns),
        clock.zero_reads,
        CLOCK_SAMPLES
    );
    for e in entries {
        println!(
            "  {:>10}: {:>12} ns/op   replicate spread {:>7} %   (within-run {} %)",
            e.label,
            json_num(e.median_ns),
            json_num(e.spread_rel * 100.0),
            json_num(e.within_spread_rel * 100.0),
        );
    }

    let dir = Path::new(out_dir);
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("bbc_probe: could not create {out_dir}: {e} — readings not written");
        return;
    }
    // `<job>-<series>-<position>.json` — unique per job per series per end, so
    // every cell's readings merge into one directory without collision.
    let path = dir.join(format!("{job}-{series}-{position}.json"));
    match fs::write(&path, &j) {
        Ok(()) => println!("bbc_probe: wrote {}", path.display()),
        Err(e) => eprintln!("bbc_probe: could not write {}: {e}", path.display()),
    }
}
