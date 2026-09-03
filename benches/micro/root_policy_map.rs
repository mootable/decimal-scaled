// SPDX-FileCopyrightText: 2026 John Moxley
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Full-surface policy sweep for `policy::sqrt` and `policy::cbrt` — the
//! `policy-mapper` deliverable: for every `(width, scale)` sample point, the
//! measured speed of EVERY registered `Algorithm` candidate, with the winner
//! marked and every non-bit-identical candidate recorded as ineligible.
//!
//! # Why this is not `root_kernel_ab`
//!
//! `root_kernel_ab` pins `SCALE` as a CONST generic in every seam, so each
//! `(N, W, SCALE)` cell is a separate monomorphisation — a full-surface map
//! (12 widths x 5 grid points x 2 functions, plus every bisection) would mean
//! hundreds of instantiations of a kernel that runs up to 192 limbs wide, and
//! a recompile for each bisected scale. It also has no way to run a SUBSET:
//! `micro_criterion()` never calls `.configure_from_args()`, and this bench
//! family does not call it at the call site either, so a criterion name filter
//! is silently ignored and the whole suite runs; and `compare_all`'s verdict
//! comes from `coarse_time_dyn`, a fixed 2000-pass wall-clock loop that runs
//! outside criterion entirely and so cannot be filtered at all.
//!
//! This bench drives the SAME generic kernels through the runtime-scale seams
//! (`__bench_internals::{sqrt,cbrt}_*_rt`), so one monomorphisation per
//! `(N, W)` sweeps every scale and a bisection point costs no rebuild. The
//! sweep is selected entirely by environment variable.
//!
//! # Measurement
//!
//! The verdict line and ranking table that `compare_all` prints already come
//! from a hand-rolled wall-clock loop, not from criterion. This keeps that
//! contract (identical `Ranking [...]` / `A/B verdict [...]` output shape) and
//! makes the loop time-BUDGETED and interleaved instead of fixed-pass:
//!
//! * each `(candidate, input)` is measured for a fixed time budget, so a
//!   fast narrow cell and a slow 192-limb cell both cost the same wall time;
//! * candidates are measured round-robin across several rounds and the
//!   per-candidate MEDIAN is taken, so clock/thermal drift cannot bias one
//!   candidate that happened to be measured first;
//! * inputs and outputs are `black_box`-guarded, because the dispatch this
//!   measures is designed to const-fold.
//!
//! # The validity wall
//!
//! A candidate is eligible at a cell only where it is bit-identical to the
//! generic slice reference across the whole input spread and all eight
//! rounding modes. Candidates are probed under `catch_unwind`, so an arm that
//! panics at a width it was never valid for (the `Int<2>` / `Int<3>` bridges
//! narrow) is recorded as a failure rather than killing the sweep. Ineligible
//! candidates are still MEASURED and reported — they just cannot win a cell.
//!
//! # Selection (all optional)
//!
//! ```text
//! ROOTMAP_FN       sqrt | cbrt | both            (default both)
//! ROOTMAP_WIDTHS   comma list of N               (default all 12)
//! ROOTMAP_SCALES   comma list, or `grid`         (default grid = {0,S/4,S/2,3S/4,S-1})
//! ROOTMAP_BUDGET_MS   per (candidate,input) measurement budget  (default 20)
//! ROOTMAP_ROUNDS      interleaved rounds, median taken          (default 3)
//! ROOTMAP_TIMEOUT_S   per-cell watchdog                         (default 300)
//! ROOTMAP_MODES       1 = wall all 8 modes, 0 = HalfToEven only (default 1)
//! ```

use std::hint::black_box;
use std::io::{self, Write};
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use decimal_scaled::__bench_internals as bi;
use decimal_scaled::{Int, RoundingMode};

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
const MODE: RoundingMode = RoundingMode::HalfToEven;

// ── configuration ────────────────────────────────────────────────────────

struct Cfg {
    do_sqrt: bool,
    do_cbrt: bool,
    widths: Vec<usize>,
    scales: Option<Vec<u32>>, // None = the 5-point grid for the tier
    budget: Duration,
    rounds: usize,
    all_modes: bool,
}

fn env_str(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.is_empty())
}

fn cfg() -> Cfg {
    let f = env_str("ROOTMAP_FN").unwrap_or_else(|| "both".to_string());
    let widths = match env_str("ROOTMAP_WIDTHS") {
        Some(s) => s.split(',').filter_map(|t| t.trim().parse().ok()).collect(),
        None => vec![1, 2, 3, 4, 6, 8, 12, 16, 24, 32, 48, 64],
    };
    let scales = match env_str("ROOTMAP_SCALES") {
        Some(ref s) if s == "grid" => None,
        Some(s) => Some(s.split(',').filter_map(|t| t.trim().parse().ok()).collect()),
        None => None,
    };
    Cfg {
        do_sqrt: f == "both" || f == "sqrt",
        do_cbrt: f == "both" || f == "cbrt",
        widths,
        scales,
        budget: Duration::from_millis(
            env_str("ROOTMAP_BUDGET_MS").and_then(|v| v.parse().ok()).unwrap_or(20),
        ),
        rounds: env_str("ROOTMAP_ROUNDS").and_then(|v| v.parse().ok()).unwrap_or(3),
        all_modes: env_str("ROOTMAP_MODES").map(|v| v != "0").unwrap_or(true),
    }
}

/// Decimal digit capacity of the tier backed by `N` limbs — the type's NAME
/// (D115 -> 115), whose scale cap is `MAX_SCALE = name - 1`.
fn tier_digits(n: usize) -> u32 {
    match n {
        1 => 18,
        2 => 38,
        3 => 57,
        4 => 76,
        6 => 115,
        8 => 153,
        12 => 230,
        16 => 307,
        24 => 462,
        32 => 616,
        48 => 924,
        64 => 1232,
        _ => unreachable!("unsupported storage width N={n}"),
    }
}

/// The coarse scale grid for a tier: `{0, S/4, S/2, 3S/4, S-1}` (S = the
/// tier's digit capacity, so `S-1` is its MAX_SCALE). The STARTING POINT for
/// the sweep — bisection points are supplied via `ROOTMAP_SCALES`.
fn scale_grid(n: usize) -> Vec<u32> {
    let s = tier_digits(n);
    let mut v = vec![0, s / 4, s / 2, 3 * s / 4, s - 1];
    v.dedup();
    v
}

// ── per-cell watchdog ────────────────────────────────────────────────────
//
// A candidate that never terminates would otherwise hang the sweep with no
// record of which cell did it. The watchdog prints the full cell parameters
// and exits, so the cell is reproducible.

fn cell_slot() -> &'static Mutex<Option<(String, Instant)>> {
    static SLOT: OnceLock<Mutex<Option<(String, Instant)>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

fn start_watchdog(timeout: Duration) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        let cur = cell_slot().lock().unwrap().clone();
        if let Some((desc, started)) = cur {
            if started.elapsed() > timeout {
                println!(
                    "TIMEOUT\t{desc}\telapsed_s={:.1}\tlimit_s={:.0}",
                    started.elapsed().as_secs_f64(),
                    timeout.as_secs_f64()
                );
                println!(
                    "TIMEOUT: no result for this cell. Reproduce with ROOTMAP_FN / \
                     ROOTMAP_WIDTHS / ROOTMAP_SCALES set to the values in the cell tag above."
                );
                io::stdout().flush().ok();
                std::process::exit(3);
            }
        }
    });
}

fn enter_cell(desc: &str) {
    *cell_slot().lock().unwrap() = Some((desc.to_string(), Instant::now()));
}
fn leave_cell() {
    *cell_slot().lock().unwrap() = None;
}

// ── measurement ──────────────────────────────────────────────────────────

/// Nanoseconds per call for `run` over `input`, measured for `budget`.
/// Inputs and outputs are `black_box`-guarded so the const-folding dispatch
/// under test cannot be hoisted away.
fn measure_one<const N: usize>(
    run: &dyn Fn(Int<N>, RoundingMode) -> Int<N>,
    input: Int<N>,
    budget: Duration,
) -> f64 {
    // Warm-up: two calls, discarded (first-call page/branch-predictor effects).
    black_box(run(black_box(input), MODE));
    black_box(run(black_box(input), MODE));

    let mut calls: u64 = 0;
    let start = Instant::now();
    loop {
        black_box(run(black_box(input), MODE));
        calls += 1;
        if start.elapsed() >= budget {
            break;
        }
    }
    start.elapsed().as_secs_f64() * 1e9 / calls as f64
}

/// The per-round MINIMUM, not the mean or median.
///
/// This machine runs other agents' builds concurrently, and interference is
/// strictly additive: a round can only ever be slowed by a neighbouring
/// compile, never sped up. The minimum across interleaved rounds is therefore
/// the most robust estimator of the kernel's own cost, and it is far steadier
/// than the median when the interference is sporadic. The `slice` /
/// `schoolbook` pair — the SAME kernel behind two labels — is carried through
/// every cell as a live control on how much noise survives this.
fn robust(v: Vec<f64>) -> f64 {
    v.into_iter().fold(f64::INFINITY, f64::min)
}

// ── the cell runner ──────────────────────────────────────────────────────

struct Cand<'a, const N: usize> {
    label: &'static str,
    /// Takes the rounding mode so the validity wall can sweep all eight —
    /// a candidate that agrees only at `HalfToEven` is not bit-identical.
    run: Box<dyn Fn(Int<N>, RoundingMode) -> Int<N> + 'a>,
}

/// Race every candidate at one `(function, width, scale)` cell.
///
/// `reference` indexes the candidate every other candidate must reproduce
/// bit-for-bit to be eligible (the generic slice arm). Generic over `N` only,
/// so there is one instantiation per storage width, not per scale.
fn run_cell<const N: usize>(
    fname: &str,
    scale: u32,
    inputs: &[(&'static str, Int<N>)],
    cands: &[Cand<'_, N>],
    reference: usize,
    cfg: &Cfg,
) {
    let tier = tier_digits(N);
    let group = format!("{fname}_d{tier}_s{scale}");
    let tag = format!("fn={fname} N={N} tier=D{tier} scale={scale}");
    enter_cell(&tag);

    // ── validity wall: bit-identical to the reference, every input, every
    // mode. A candidate that panics (a narrowing bridge at a width it was
    // never valid for) is captured, not propagated.
    let modes: &[RoundingMode] = if cfg.all_modes { &ALL_MODES } else { &[MODE] };
    let mut status: Vec<String> = vec![String::from("ok"); cands.len()];

    // Reference results computed ONCE per (input, mode). At the widest tiers a
    // single root is milliseconds, so re-deriving the reference inside every
    // candidate's loop would make the wall cost more than the whole timing run.
    let mut expect: Vec<Vec<Int<N>>> = Vec::with_capacity(inputs.len());
    for (_, input) in inputs {
        let mut row = Vec::with_capacity(modes.len());
        for &m in modes {
            match panic::catch_unwind(AssertUnwindSafe(|| (cands[reference].run)(*input, m))) {
                Ok(v) => row.push(v),
                Err(_) => {
                    status[reference] = "PANIC(reference)".to_string();
                    println!("ROW\t{fname}\t{N}\tD{tier}\t{scale}\treference\tinf\tINELIGIBLE\tPANIC");
                    leave_cell();
                    return;
                }
            }
        }
        expect.push(row);
    }

    for (ci, cand) in cands.iter().enumerate() {
        if ci == reference {
            status[ci] = "reference".to_string();
            continue;
        }
        'cand: for (ii, (ilabel, input)) in inputs.iter().enumerate() {
            for (mi, &m) in modes.iter().enumerate() {
                match panic::catch_unwind(AssertUnwindSafe(|| (cand.run)(*input, m))) {
                    Err(_) => {
                        status[ci] = format!("FAIL:panic@{ilabel}/{m:?}");
                        break 'cand;
                    }
                    Ok(v) => {
                        if v != expect[ii][mi] {
                            status[ci] = format!("FAIL:mismatch@{ilabel}/{m:?}");
                            break 'cand;
                        }
                    }
                }
            }
        }
    }

    // ── timing: interleaved rounds, per-candidate median, per input.
    let mut times: Vec<Vec<f64>> = vec![vec![0.0; inputs.len()]; cands.len()];
    for (ii, (_, input)) in inputs.iter().enumerate() {
        let mut samples: Vec<Vec<f64>> = vec![Vec::new(); cands.len()];
        for _ in 0..cfg.rounds {
            for (ci, cand) in cands.iter().enumerate() {
                if status[ci].starts_with("FAIL") {
                    samples[ci].push(f64::INFINITY);
                    continue;
                }
                samples[ci].push(measure_one(&*cand.run, *input, cfg.budget));
            }
        }
        for (ci, s) in samples.into_iter().enumerate() {
            times[ci][ii] = robust(s);
        }
    }

    // Aggregate = mean ns/call across the input spread.
    let agg: Vec<f64> = times
        .iter()
        .map(|row| row.iter().sum::<f64>() / row.len() as f64)
        .collect();

    // ── report ──────────────────────────────────────────────────────────
    // Machine-readable rows first (one per candidate), then the ranking
    // table and verdict line in the `compare_all` output shape.
    for (ci, cand) in cands.iter().enumerate() {
        let eligible = !status[ci].starts_with("FAIL");
        println!(
            "ROW\t{fname}\t{N}\tD{tier}\t{scale}\t{}\t{:.1}\t{}\t{}",
            cand.label,
            agg[ci],
            if eligible { "eligible" } else { "INELIGIBLE" },
            status[ci]
        );
    }

    // Rank only the ELIGIBLE candidates — speed never overrides correctness.
    let mut rank: Vec<(usize, f64)> = (0..cands.len())
        .filter(|&ci| !status[ci].starts_with("FAIL"))
        .map(|ci| (ci, agg[ci]))
        .collect();
    rank.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!(
        "Ranking [{group}] ({} eligible of {} candidates, {} inputs):",
        rank.len(),
        cands.len(),
        inputs.len()
    );
    if rank.is_empty() {
        println!("  (no eligible candidate — see the INELIGIBLE rows above)");
    } else {
        let win = rank[0].1;
        for (r, (ci, t)) in rank.iter().enumerate() {
            let ratio = t / win;
            let note = if r == 0 {
                " <- winner".to_string()
            } else if ratio < 1.01 {
                " (~tie with winner)".to_string()
            } else {
                format!(" ({ratio:.2}x slower)")
            };
            println!("  #{}: {:16} {:>10.1} ns{}", r + 1, cands[*ci].label, t, note);
        }
    }
    if rank.len() >= 2 {
        let (wi, wt) = rank[0];
        let (ri, rt) = rank[1];
        println!(
            "A/B verdict [{group}]: {} beats {} by {:.2}x          ({}={:.1}ns vs {}={:.1}ns over {} inputs)",
            cands[wi].label, cands[ri].label, rt / wt,
            cands[wi].label, wt, cands[ri].label, rt, inputs.len()
        );
    } else if rank.len() == 1 {
        println!(
            "A/B verdict [{group}]: {} is the only eligible candidate",
            cands[rank[0].0].label
        );
    }

    // Noise control: `slice` and `schoolbook` are the SAME kernel behind two
    // labels, so their measured gap is pure measurement error. Any margin at
    // this cell smaller than this figure is NOT a real difference.
    let find = |l: &str| cands.iter().position(|c| c.label == l);
    if let (Some(a), Some(b)) = (find("slice"), find("schoolbook")) {
        let (lo, hi) = if agg[a] <= agg[b] { (agg[a], agg[b]) } else { (agg[b], agg[a]) };
        println!("noise [{group}]: same-kernel slice/schoolbook spread {:.3}x", hi / lo);
    }

    // Per-input winners — reported whenever the input value changes the
    // verdict, which is exactly the value-dependence the policy would need a
    // `ByValue` arm to express.
    let mut per_input: Vec<String> = Vec::new();
    let mut disagree = false;
    for (ii, (ilabel, _)) in inputs.iter().enumerate() {
        let mut best = usize::MAX;
        let mut best_t = f64::INFINITY;
        for ci in 0..cands.len() {
            if status[ci].starts_with("FAIL") {
                continue;
            }
            if times[ci][ii] < best_t {
                best_t = times[ci][ii];
                best = ci;
            }
        }
        if best == usize::MAX {
            continue;
        }
        if !rank.is_empty() && best != rank[0].0 {
            disagree = true;
        }
        // runner-up at this input, for the margin
        let mut second = f64::INFINITY;
        for ci in 0..cands.len() {
            if status[ci].starts_with("FAIL") || ci == best {
                continue;
            }
            if times[ci][ii] < second {
                second = times[ci][ii];
            }
        }
        let margin = if second.is_finite() { second / best_t } else { 1.0 };
        per_input.push(format!("{ilabel}:{}@{:.2}x", cands[best].label, margin));
    }
    println!(
        "per-input [{group}]{}: {}",
        if disagree { " VALUE-SPLIT" } else { "" },
        per_input.join(" | ")
    );
    io::stdout().flush().ok();
    leave_cell();
}

// ── operand spread ───────────────────────────────────────────────────────

/// The input spread for a cell. Two bench-branch-compare-shaped operands
/// (`1.0` / `2.0` at this scale), the tier's near-maximum magnitude, and the
/// minimum non-zero raw — so a value-dependent winner is visible.
fn inputs_for<const N: usize>(scale: u32) -> Vec<(&'static str, Int<N>)> {
    let mut lsb = [0u64; N];
    lsb[0] = 1;
    vec![
        ("v1.0", bi::operand_k_at_scale::<N>(1, scale)),
        ("v2.0", bi::operand_k_at_scale::<N>(2, scale)),
        ("v_top", bi::operand_k_at_scale::<N>(9, tier_digits(N) - 1)),
        ("v_lsb", bi::int_from_mag_limbs::<N>(&lsb)),
    ]
}

// ── per-width sweeps ─────────────────────────────────────────────────────
//
// One arm per storage width. `W` is the policy's own full-range work width
// (sqrt `W = 2N`, cbrt `W = 3N`) — the width the live decimal op pays. Every
// candidate is a thin seam over the SAME generic kernel the policy dispatches
// to; nothing here is a per-tier algorithm copy.

macro_rules! sweep_width {
    ($fname:ident, $n:literal, $wsqrt:literal, $wcbrt:literal) => {
        fn $fname(cfg: &Cfg) {
            let scales = cfg.scales.clone().unwrap_or_else(|| scale_grid($n));
            let max_scale = tier_digits($n) - 1;

            for &s in &scales {
                if s > max_scale {
                    println!(
                        "SKIP\tN={} tier=D{} scale={} — past MAX_SCALE={} for this tier",
                        $n, tier_digits($n), s, max_scale
                    );
                    continue;
                }
                let inputs = inputs_for::<$n>(s);

                if cfg.do_sqrt {
                    // Hoisted out of the timed region: production folds this
                    // at compile time via `const { Int::<W>::TEN.pow(SCALE) }`.
                    let p10 = bi::pow10_w::<$wsqrt>(s);
                    let cands: Vec<Cand<$n>> = vec![
                        Cand {
                            label: "slice",
                            run: Box::new(move |r, m| bi::sqrt_newton_rt::<$n>(r, s, m)),
                        },
                        Cand {
                            label: "native",
                            run: Box::new(move |r, m| bi::sqrt_native_rt::<$n, $wsqrt>(r, p10, m)),
                        },
                        Cand {
                            label: "mg_divide",
                            run: Box::new(move |r, m| bi::sqrt_mg_rt::<$n>(r, s, m)),
                        },
                        Cand {
                            label: "table_seed",
                            run: Box::new(move |r, m| bi::sqrt_table_seed_rt::<$n>(r, m)),
                        },
                        // `Schoolbook` dispatches to the SAME kernel as
                        // `Newton` — raced anyway (no arm is pre-dropped by
                        // reasoning); it is expected to read as a tie.
                        Cand {
                            label: "schoolbook",
                            run: Box::new(move |r, m| bi::sqrt_newton_rt::<$n>(r, s, m)),
                        },
                    ];
                    run_cell::<$n>("sqrt", s, &inputs, &cands, 0, cfg);
                }

                if cfg.do_cbrt {
                    let p10 = bi::pow10_w::<$wcbrt>(2 * s);
                    let cands: Vec<Cand<$n>> = vec![
                        Cand {
                            label: "slice",
                            run: Box::new(move |r, m| bi::cbrt_newton_rt::<$n>(r, s, m)),
                        },
                        Cand {
                            label: "fast_a",
                            run: Box::new(move |r, m| bi::cbrt_fast_a_rt::<$n, $wcbrt>(r, p10, m)),
                        },
                        Cand {
                            label: "fast_b",
                            run: Box::new(move |r, m| bi::cbrt_fast_b_rt::<$n, $wcbrt>(r, p10, m)),
                        },
                        Cand {
                            label: "native",
                            run: Box::new(move |r, m| bi::cbrt_native_rt::<$n, $wcbrt>(r, p10, m)),
                        },
                        Cand {
                            label: "mg_divide",
                            run: Box::new(move |r, m| bi::cbrt_mg_rt::<$n>(r, s, m)),
                        },
                        Cand {
                            label: "table_seed",
                            run: Box::new(move |r, m| bi::cbrt_table_seed_rt::<$n>(r, m)),
                        },
                        Cand {
                            label: "schoolbook",
                            run: Box::new(move |r, m| bi::cbrt_newton_rt::<$n>(r, s, m)),
                        },
                    ];
                    run_cell::<$n>("cbrt", s, &inputs, &cands, 0, cfg);
                }
            }
        }
    };
}

sweep_width!(sweep_n1, 1, 2, 3);
sweep_width!(sweep_n2, 2, 4, 6);
sweep_width!(sweep_n3, 3, 6, 9);
sweep_width!(sweep_n4, 4, 8, 12);
sweep_width!(sweep_n6, 6, 12, 18);
sweep_width!(sweep_n8, 8, 16, 24);
sweep_width!(sweep_n12, 12, 24, 36);
sweep_width!(sweep_n16, 16, 32, 48);
sweep_width!(sweep_n24, 24, 48, 72);
sweep_width!(sweep_n32, 32, 64, 96);
sweep_width!(sweep_n48, 48, 96, 144);
sweep_width!(sweep_n64, 64, 128, 192);

fn main() {
    let cfg = cfg();
    let timeout = Duration::from_secs(
        env_str("ROOTMAP_TIMEOUT_S").and_then(|v| v.parse().ok()).unwrap_or(300),
    );
    start_watchdog(timeout);

    // Panics are captured by the validity wall and reported as failures, so
    // the default hook's backtrace spam would only obscure the map.
    panic::set_hook(Box::new(|_| {}));

    println!(
        "root_policy_map: fn={} widths={:?} scales={} budget={}ms rounds={} modes={} timeout={}s",
        if cfg.do_sqrt && cfg.do_cbrt {
            "both"
        } else if cfg.do_sqrt {
            "sqrt"
        } else {
            "cbrt"
        },
        cfg.widths,
        cfg.scales.as_ref().map_or("grid".to_string(), |v| format!("{v:?}")),
        cfg.budget.as_millis(),
        cfg.rounds,
        if cfg.all_modes { 8 } else { 1 },
        timeout.as_secs()
    );
    println!("ROW\tfn\tN\ttier\tscale\tcandidate\tns_per_call\teligibility\tstatus");

    let started = Instant::now();
    for &n in &cfg.widths {
        match n {
            1 => sweep_n1(&cfg),
            2 => sweep_n2(&cfg),
            3 => sweep_n3(&cfg),
            4 => sweep_n4(&cfg),
            6 => sweep_n6(&cfg),
            8 => sweep_n8(&cfg),
            12 => sweep_n12(&cfg),
            16 => sweep_n16(&cfg),
            24 => sweep_n24(&cfg),
            32 => sweep_n32(&cfg),
            48 => sweep_n48(&cfg),
            64 => sweep_n64(&cfg),
            other => println!("SKIP\tunsupported width N={other}"),
        }
    }
    println!("root_policy_map: done in {:.1}s", started.elapsed().as_secs_f64());
}
