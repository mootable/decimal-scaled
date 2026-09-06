#!/usr/bin/env python3
"""Collate the `bbc_probe` readings taken at both ends of every bench job.

The probe (`probe/bbc_probe.rs`) runs at the START and at the END of each bench
job and writes one JSON file per invocation into `target/criterion/_bbc_probe/`.
The aggregate job merges every cell's criterion subtree into one place, so those
files arrive here together and this script turns them into the two things the
readings exist to establish:

  * **RESOLUTION** — at a given per-operation cost, how far apart do two
    measurements of IDENTICAL work land? That is the real floor under the
    regression filter, and it is a property of the runner, not of our code.

  * **DRIFT** — did the runner change while the sweep ran? Each job's start and
    end readings measure the same synthetic ladder, so their ratio is drift and
    nothing else.

WHY THIS IS A SEPARATE SCRIPT. `summarise.py` owns the criterion tree, the
regression table and `bbc_medians.tsv`, and none of those had to change to carry
the probe: the readings ride in a `_bbc_probe/` subdirectory that summarise.py's
glob (`**/new/benchmark.json`) cannot match, so its schema is untouched and the
two outputs stay independent. Nothing here reads or writes a benchmark median.

IT NEVER FAILS THE JOB. A missing, partial or unparseable set of readings is
reported and exits 0. The probe is an annotation on the measurement; it is not a
gate on it, and a runner that drifted still produced a perfectly good
branch÷prod ratio because both sides were measured in the same job.

Outputs:
  * stdout — the rendered markdown (the workflow appends it to the run summary).
  * `--tsv PATH` — one row per (job, position, rung), the full readings.
  * `--md PATH` — the same markdown to a file, for the aggregate artifact.
"""

import argparse
import glob
import json
import os

PROBE_DIR = os.path.join("target", "criterion", "_bbc_probe")

# Jobs whose start->end ladder moved by more than this are listed as having
# drifted. PROVISIONAL, and deliberately not enforced: it annotates, it never
# fails a job. It exists so the summary can point at the worst jobs rather than
# make the reader scan sixty rows, and it should be reset from measured data
# once a few full sweeps have banked readings — setting it from a guess today
# would repeat the exact mistake the probe was built to correct.
DRIFT_FLAG = 0.10


def load(probe_dir):
    """Every probe reading, newest schema only, keyed (job, position)."""
    out = {}
    bad = []
    for path in sorted(glob.glob(os.path.join(probe_dir, "*.json"))):
        try:
            with open(path, encoding="utf-8") as f:
                doc = json.load(f)
        except (OSError, json.JSONDecodeError) as e:
            bad.append((os.path.basename(path), str(e)))
            continue
        if doc.get("schema") != "bbc-probe/1":
            bad.append((os.path.basename(path), f"unknown schema {doc.get('schema')!r}"))
            continue
        out[(doc.get("job", "?"), doc.get("position", "?"))] = doc
    return out, bad


def quantile(values, q):
    """Linear-interpolated quantile over an unsorted list; None when empty."""
    s = sorted(v for v in values if v is not None)
    if not s:
        return None
    pos = q * (len(s) - 1)
    lo, hi = int(pos), min(int(pos) + 1, len(s) - 1)
    return s[lo] if lo == hi else s[lo] + (s[hi] - s[lo]) * (pos - lo)


def fmt_ns(v):
    if v is None:
        return "—"
    if v < 1e3:
        return f"{v:.2f} ns"
    if v < 1e6:
        return f"{v / 1e3:.2f} µs"
    return f"{v / 1e6:.2f} ms"


def fmt_pct(v):
    return "—" if v is None else f"{v * 100:.2f} %"


def fmt_x(v):
    return "—" if v is None else f"{v:.3f}×"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--probe-dir", default=PROBE_DIR)
    ap.add_argument("--tsv", help="write the full per-(job, position, rung) readings here")
    ap.add_argument("--md", help="also write the rendered markdown here")
    args = ap.parse_args()

    docs, bad = load(args.probe_dir)
    lines = ["## Timing probe — what this run could actually resolve", ""]

    if not docs:
        lines += [
            "_No probe readings in this run._ Either the bench jobs predate the probe, "
            "or `bbc_probe` did not run. The regression table above is unaffected — "
            "the probe annotates the measurement, it does not produce it.",
            "",
        ]
        render(lines, args)
        return

    jobs = sorted({j for (j, _) in docs})
    positions = [p for p in ("start", "end") if any(pp == p for (_, pp) in docs)]

    # ── The clock, pooled over every reading ────────────────────────────
    #
    # The tick and the read cost are different numbers and are reported as
    # such. Neither is the answer to "what floor should the filter use" — an
    # amortised per-iteration figure legitimately sits far below both, because
    # it divides one clock interval across hundreds of thousands of operations.
    # They are here to say what the raw instrument does, so that the ladder
    # below is read as the separate quantity it is.
    ticks = [d["clock"]["min_nonzero_ns"] for d in docs.values() if d.get("clock")]
    reads = [d["clock"]["median_ns"] for d in docs.values() if d.get("clock")]
    zeros = sum(d["clock"]["zero_reads"] for d in docs.values() if d.get("clock"))
    total_reads = sum(d["clock"]["reads"] for d in docs.values() if d.get("clock"))
    cpus = sorted({d.get("cpu_model", "?") for d in docs.values()})

    lines += [
        f"_{len(docs)} readings from {len(jobs)} job(s), positions "
        f"`{'` + `'.join(positions)}`._",
        "",
        "### The clock itself",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| smallest non-zero interval (the tick) | {fmt_ns(quantile(ticks, 0.5))} |",
        f"| cost of one `Instant::now()` (median) | {fmt_ns(quantile(reads, 0.5))} |",
        f"| back-to-back reads returning zero | {zeros} of {total_reads} |",
        "",
        "_The tick is the granularity in the literal sense; the read cost is what "
        "asking costs. Neither is the floor under the regression filter: a benched "
        "figure is an amortised per-iteration cost spread across a whole loop between "
        "two reads, so it legitimately sits far below both. The floor is the ladder "
        "below._",
        "",
    ]

    # ── The ladder: the actual resolution answer ────────────────────────
    #
    # Two spreads, and the distinction is the entire point:
    #
    #   WITHIN-JOB — the same measurement repeated back to back in one process
    #   on one machine. This is the null distribution for branch÷prod, because
    #   that is exactly the shape of the comparison: both sides run in the same
    #   job, seconds apart. It is the number the regression filter needs.
    #
    #   CROSS-JOB — the same rung's median across every job in the sweep, i.e.
    #   across separate runner VMs. This is the null for any ratio taken BETWEEN
    #   cells (width or scale monotonicity), which the workflow already warns is
    #   invalid under `group: cell`. Measured here on a synthetic reference, so
    #   it separates "the machines differ" from "our code differs".
    rung_reps = []
    for d in docs.values():
        for r in d.get("ladder", []):
            if r["reps"] not in rung_reps:
                rung_reps.append(r["reps"])
    rung_reps.sort()

    lines += [
        "### Cost ladder — repeatability of an identical synthetic workload",
        "",
        "| nominal cost | within-job spread p50 | within-job spread p90 | worst pair | cross-job p90÷p10 | cross-job max÷min |",
        "|---:|---:|---:|---:|---:|---:|",
    ]

    ladder_rows = []
    for reps in rung_reps:
        medians, spreads, ranges = [], [], []
        for d in docs.values():
            for r in d.get("ladder", []):
                if r["reps"] == reps:
                    medians.append(r["median_ns"])
                    spreads.append(r["spread_rel"])
                    ranges.append(r["range_rel"])
        cost = quantile(medians, 0.5)
        p10, p90 = quantile(medians, 0.10), quantile(medians, 0.90)
        lo, hi = (min(medians) if medians else None), (max(medians) if medians else None)
        row = {
            "reps": reps,
            "cost_ns": cost,
            "within_p50": quantile(spreads, 0.5),
            "within_p90": quantile(spreads, 0.9),
            "worst_pair": quantile(ranges, 1.0),
            "cross_p90_p10": (p90 / p10) if (p10 and p90) else None,
            "cross_max_min": (hi / lo) if (lo and hi) else None,
        }
        ladder_rows.append(row)
        lines.append(
            f"| {fmt_ns(row['cost_ns'])} | {fmt_pct(row['within_p50'])} "
            f"| {fmt_pct(row['within_p90'])} | {fmt_pct(row['worst_pair'])} "
            f"| {fmt_x(row['cross_p90_p10'])} | {fmt_x(row['cross_max_min'])} |"
        )

    # The headline: the worst within-job p90 over the whole ladder. A relative
    # threshold at or above this cannot be tripped by measurement noise on a
    # same-job comparison, at ANY cost on the ladder.
    worst_within = max(
        (r["within_p90"] for r in ladder_rows if r["within_p90"] is not None),
        default=None,
    )
    lines += [
        "",
        "_**Within-job spread** is the same measurement repeated back to back in one "
        "process — the null distribution for branch÷prod, which is measured exactly "
        "that way. **Cross-job** is the same rung across separate runner VMs — the null "
        "for any ratio read BETWEEN cells, on a workload that is identical by "
        "construction._",
        "",
    ]
    if worst_within is not None:
        lines += [
            f"**Worst within-job spread anywhere on the ladder: {fmt_pct(worst_within)}.** "
            "A purely relative regression threshold at or above this figure cannot be "
            "tripped by measurement noise at any cost on the ladder — including the "
            "~ns rungs the current fixed 10 ns floor renders unassessable. This is a "
            "reading, not a change: the filter in `summarise.py` is untouched.",
            "",
        ]

    # ── Drift: the start/end pair, which is why there are two ───────────
    drift_rows = []
    for job in jobs:
        a, b = docs.get((job, "start")), docs.get((job, "end"))
        if not a or not b:
            continue
        per_rung = []
        end_by_reps = {r["reps"]: r["median_ns"] for r in b.get("ladder", [])}
        for r in a.get("ladder", []):
            s, e = r["median_ns"], end_by_reps.get(r["reps"])
            if s and e:
                per_rung.append(e / s)
        if per_rung:
            drift_rows.append({"job": job, "drift": quantile(per_rung, 0.5), "rungs": len(per_rung)})

    lines += ["### Within-job drift (end ÷ start, identical workload)", ""]
    if not drift_rows:
        lines += [
            "_No job produced both a start and an end reading, so drift could not be "
            "computed. The pair is the only thing that reveals it — a single reading "
            "cannot tell a slow machine from a machine that became slow._",
            "",
        ]
    else:
        d = [r["drift"] for r in drift_rows]
        flagged = sorted(
            (r for r in drift_rows if abs(r["drift"] - 1.0) > DRIFT_FLAG),
            key=lambda r: abs(r["drift"] - 1.0),
            reverse=True,
        )
        lines += [
            "| quantity | value |",
            "|---|---:|",
            f"| jobs with both readings | {len(drift_rows)} of {len(jobs)} |",
            f"| drift p50 | {fmt_x(quantile(d, 0.5))} |",
            f"| drift p90 | {fmt_x(quantile(d, 0.9))} |",
            f"| drift range | {fmt_x(min(d))} … {fmt_x(max(d))} |",
            f"| jobs past ±{DRIFT_FLAG:.0%} | {len(flagged)} |",
            "",
            "_Above 1.00× the runner got SLOWER while the sweep ran, so that job's cells "
            "are inflated relative to its own start. The branch÷prod ratio largely "
            "survives it — both sides run seconds apart inside one criterion group — but "
            "a heavily drifted job's absolute medians should be read with that in mind._",
            "",
        ]
        if flagged:
            lines += [
                f"<details><summary>{len(flagged)} job(s) past ±{DRIFT_FLAG:.0%}</summary>",
                "",
                "| job | end ÷ start |",
                "|---|---:|",
            ]
            lines += [f"| `{r['job']}` | {fmt_x(r['drift'])} |" for r in flagged]
            lines += ["", "</details>", ""]

    if len(cpus) > 1:
        lines += [
            f"_Runner CPU models seen this run: {len(cpus)} distinct — "
            + ", ".join(f"`{c}`" for c in cpus[:4])
            + ("…" if len(cpus) > 4 else "")
            + ". A pool this mixed is itself a source of the cross-job spread above._",
            "",
        ]

    if bad:
        lines += [
            f"_Unreadable probe files: {len(bad)} — "
            + ", ".join(f"`{n}` ({e})" for n, e in bad[:5])
            + ("…" if len(bad) > 5 else "")
            + "._",
            "",
        ]

    if args.tsv:
        with open(args.tsv, "w", encoding="utf-8") as f:
            f.write(
                "job\tposition\tcpu_model\treps\titers\tmedian_ns\tmin_ns\tmax_ns"
                "\tp10_ns\tp90_ns\tspread_rel\trange_rel\twithin_spread_rel"
                "\tclock_tick_ns\tclock_read_ns\n"
            )
            for (job, pos), doc in sorted(docs.items()):
                c = doc.get("clock", {})
                for r in doc.get("ladder", []):
                    f.write(
                        f"{job}\t{pos}\t{doc.get('cpu_model', '?')}\t{r['reps']}\t{r['iters']}\t"
                        f"{r['median_ns']}\t{r['min_ns']}\t{r['max_ns']}\t{r['p10_ns']}\t"
                        f"{r['p90_ns']}\t{r['spread_rel']}\t{r['range_rel']}\t"
                        f"{r['within_spread_rel']}\t{c.get('min_nonzero_ns')}\t"
                        f"{c.get('median_ns')}\n"
                    )

    render(lines, args)


def render(lines, args):
    md = "\n".join(lines)
    print(md)
    if args.md:
        with open(args.md, "w", encoding="utf-8") as f:
            f.write(md + "\n")


if __name__ == "__main__":
    main()
