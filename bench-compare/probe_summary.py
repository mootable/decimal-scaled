#!/usr/bin/env python3
"""Collate the two `bbc_probe` series taken at both ends of every bench job.

TWO SERIES, REPORTED SEPARATELY. They answer different questions and each is
useless for the other's:

  * **synthetic** (`probe/bbc_probe.rs`) — what interval can this runner
    actually resolve? No crate operation appears in it, so it stays comparable
    across every future run regardless of what the library does. This is the
    calibration proper, and it is what sets the floor under the regression
    filter.

  * **pinned** (`examples/bbc_pinned_probe.rs`) — did OUR OWN baseline move? It
    times one deliberately fixed library operation. Nothing else in the sweep
    can see a slow uniform creep, because every bbc cell is a ratio against the
    previous release and a uniform drift reads as no change at all.

They are NEVER blended. When a merged figure moves you cannot tell a slower
runner from slower code, and removing that ambiguity is what the synthetic
series is for. The one derived quantity that legitimately joins them —
pinned ÷ synthetic, which normalises the machine out and leaves our code — is
computed explicitly and labelled as derived.

Each probe runs at the START and END of every bench job and writes one JSON file
per invocation into `target/criterion/_bbc_probe/`. The aggregate job merges
every cell's criterion subtree, so they arrive here together.

WHY THIS IS A SEPARATE SCRIPT. `summarise.py` owns the criterion tree, the
regression table and `bbc_medians.tsv`, and none of those had to change to carry
the probes: the readings ride in a `_bbc_probe/` subdirectory that its glob
(`**/new/benchmark.json`) cannot match. Nothing here reads or writes a benchmark
median.

IT NEVER FAILS THE JOB. Missing, partial or unparseable readings are reported
and it exits 0. The probes annotate the measurement; they do not gate it, and a
runner that drifted still produced a perfectly good branch÷prod ratio because
both sides were measured in the same job.

Outputs:
  * stdout — the rendered markdown (the workflow appends it to the run summary).
  * `--tsv PATH` — one row per (job, series, position, entry), the full readings.
  * `--md PATH` — the same markdown to a file, for the aggregate artifact.
"""

import argparse
import glob
import json
import os

PROBE_DIR = os.path.join("target", "criterion", "_bbc_probe")

# Jobs whose start->end readings moved by more than this are LISTED as having
# drifted. PROVISIONAL, and deliberately not enforced: it annotates, it never
# fails a job. It exists so the summary can point at the worst jobs rather than
# make the reader scan sixty rows, and it should be reset from measured data
# once a few full sweeps have banked readings — setting it from a guess today
# would repeat the exact mistake the probes were built to correct.
DRIFT_FLAG = 0.10

SYNTHETIC = "synthetic"
PINNED = "pinned"


def load(probe_dir):
    """Every probe reading, keyed `(job, series, position)`."""
    out, bad = {}, []
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
        key = (doc.get("job", "?"), doc.get("series", SYNTHETIC), doc.get("position", "?"))
        out[key] = doc
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


def entries_of(docs, series):
    """`label -> {median_ns: [...], spread_rel: [...], range_rel: [...]}` pooled
    over every reading of one series."""
    pooled = {}
    for (_job, s, _pos), doc in docs.items():
        if s != series:
            continue
        for e in doc.get("entries", []):
            slot = pooled.setdefault(e["label"], {"median_ns": [], "spread_rel": [], "range_rel": []})
            for k in slot:
                slot[k].append(e.get(k))
    return pooled


def ladder_rows(pooled):
    """One summary row per label, ordered by cost — so it reads as a ladder
    whatever the labels happen to be."""
    rows = []
    for label, v in pooled.items():
        med = quantile(v["median_ns"], 0.5)
        p10, p90 = quantile(v["median_ns"], 0.10), quantile(v["median_ns"], 0.90)
        lo = min((x for x in v["median_ns"] if x is not None), default=None)
        hi = max((x for x in v["median_ns"] if x is not None), default=None)
        rows.append({
            "label": label,
            "cost_ns": med,
            "within_p50": quantile(v["spread_rel"], 0.5),
            "within_p90": quantile(v["spread_rel"], 0.9),
            "worst_pair": quantile(v["range_rel"], 1.0),
            "cross_p90_p10": (p90 / p10) if (p10 and p90) else None,
            "cross_max_min": (hi / lo) if (lo and hi) else None,
        })
    rows.sort(key=lambda r: (r["cost_ns"] is None, r["cost_ns"]))
    return rows


def drift_rows(docs, series):
    """Per job, `end ÷ start` on identical work, medianed over that series'
    entries."""
    jobs = sorted({j for (j, s, _p) in docs if s == series})
    out = []
    for job in jobs:
        a, b = docs.get((job, series, "start")), docs.get((job, series, "end"))
        if not a or not b:
            continue
        end_by_label = {e["label"]: e["median_ns"] for e in b.get("entries", [])}
        ratios = [
            end_by_label[e["label"]] / e["median_ns"]
            for e in a.get("entries", [])
            if e.get("median_ns") and end_by_label.get(e["label"])
        ]
        if ratios:
            out.append({"job": job, "drift": quantile(ratios, 0.5)})
    return out, jobs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--probe-dir", default=PROBE_DIR)
    ap.add_argument("--tsv", help="write the full per-(job, series, position, entry) readings here")
    ap.add_argument("--md", help="also write the rendered markdown here")
    args = ap.parse_args()

    docs, bad = load(args.probe_dir)
    lines = ["## Timing probes — what this run could resolve, and whether our baseline moved", ""]

    if not docs:
        lines += [
            "_No probe readings in this run._ Either the bench jobs predate the probes, "
            "or they did not run. The regression table above is unaffected — the probes "
            "annotate the measurement, they do not produce it.",
            "",
        ]
        render(lines, args)
        return

    jobs = sorted({j for (j, _s, _p) in docs})
    lines += [
        f"_{len(docs)} readings from {len(jobs)} job(s). The two series are reported "
        "SEPARATELY and are never blended: the synthetic one says what the MACHINE can "
        "measure, the pinned one whether OUR OWN code moved. A merged figure would leave "
        "a slower runner and slower code indistinguishable, which is the ambiguity the "
        "synthetic series exists to remove._",
        "",
    ]

    # ── The clock, pooled over every reading ────────────────────────────
    ticks = [d["clock"]["min_nonzero_ns"] for d in docs.values() if d.get("clock")]
    reads = [d["clock"]["median_ns"] for d in docs.values() if d.get("clock")]
    zeros = sum(d["clock"]["zero_reads"] for d in docs.values() if d.get("clock"))
    total = sum(d["clock"]["reads"] for d in docs.values() if d.get("clock"))
    lines += [
        "### The clock itself",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| smallest non-zero interval (the tick) | {fmt_ns(quantile(ticks, 0.5))} |",
        f"| cost of one `Instant::now()` (median) | {fmt_ns(quantile(reads, 0.5))} |",
        f"| back-to-back reads returning zero | {zeros} of {total} |",
        "",
        "_The tick is granularity in the literal sense; the read cost is what asking "
        "costs. Neither is the floor under the regression filter: a benched figure is an "
        "amortised per-iteration cost spread across a whole loop between two reads, so it "
        "legitimately sits far below both. The floor is the synthetic ladder below._",
        "",
    ]

    # ── SERIES 1: synthetic — the resolution answer ─────────────────────
    syn = ladder_rows(entries_of(docs, SYNTHETIC))
    lines += ["### Series 1 — synthetic reference (what the machine can resolve)", ""]
    if not syn:
        lines += ["_No synthetic readings in this run._", ""]
        worst_within = None
    else:
        lines += [
            "| nominal cost | within-job spread p50 | within-job spread p90 | worst pair | cross-job p90÷p10 | cross-job max÷min |",
            "|---:|---:|---:|---:|---:|---:|",
        ]
        for r in syn:
            lines.append(
                f"| {fmt_ns(r['cost_ns'])} | {fmt_pct(r['within_p50'])} "
                f"| {fmt_pct(r['within_p90'])} | {fmt_pct(r['worst_pair'])} "
                f"| {fmt_x(r['cross_p90_p10'])} | {fmt_x(r['cross_max_min'])} |"
            )
        worst_within = max(
            (r["within_p90"] for r in syn if r["within_p90"] is not None), default=None
        )
        lines += [
            "",
            "_**Within-job spread** is the same measurement repeated back to back in one "
            "process — the null distribution for branch÷prod, which is measured exactly "
            "that way. **Cross-job** is the same rung across separate runner VMs — the "
            "null for any ratio read BETWEEN cells, on a workload identical by "
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

    # ── SERIES 2: pinned — did our own baseline move? ───────────────────
    pin = ladder_rows(entries_of(docs, PINNED))
    subjects = sorted({d.get("subject", "?") for (_j, s, _p), d in docs.items() if s == PINNED})
    lines += ["### Series 2 — pinned library operation (whether our baseline moved)", ""]
    if not pin:
        lines += ["_No pinned readings in this run._", ""]
    else:
        lines += [
            f"Subject: **`{'`, `'.join(subjects)}`**",
            "",
            "| operation | cost | within-job spread p50 | within-job spread p90 | worst pair | cross-job max÷min |",
            "|---|---:|---:|---:|---:|---:|",
        ]
        for r in pin:
            lines.append(
                f"| `{r['label']}` | {fmt_ns(r['cost_ns'])} | {fmt_pct(r['within_p50'])} "
                f"| {fmt_pct(r['within_p90'])} | {fmt_pct(r['worst_pair'])} "
                f"| {fmt_x(r['cross_max_min'])} |"
            )
        lines += [
            "",
            "_**This figure is only meaningful across runs.** Within one run it says "
            "nothing — it is the movement of this number over time that would reveal a "
            "slow creep in the cheapest operation in the library, which no bbc cell can "
            "show, because every cell is a ratio against the previous release and a "
            "uniform drift looks like no change at all._",
            "",
            "_**Changing what this probe measures invalidates the entire historical "
            "series.** The subject travels inside every reading so such a change is "
            "visible in the data rather than inferred from a commit._",
            "",
        ]

        # The one legitimate join of the two series, computed explicitly: the
        # pinned cost divided by the synthetic rung nearest it in cost. The
        # machine cancels, so what remains is our code. Reported as derived —
        # the raw series above stay unmerged.
        if syn:
            p = pin[0]
            if p["cost_ns"]:
                near = min(
                    (r for r in syn if r["cost_ns"]),
                    key=lambda r: abs(r["cost_ns"] - p["cost_ns"]),
                )
                lines += [
                    f"**Derived — pinned ÷ synthetic: "
                    f"{fmt_x(p['cost_ns'] / near['cost_ns'])}** "
                    f"(`{p['label']}` at {fmt_ns(p['cost_ns'])} against the nearest "
                    f"synthetic rung, {fmt_ns(near['cost_ns'])}). This ratio has the "
                    "machine divided out, so it is the number to track across runs: it "
                    "moves when OUR code moves, not when the runner does. It is derived "
                    "here, not blended — both series remain reported on their own above.",
                    "",
                ]

    # ── Drift: why each probe runs twice ────────────────────────────────
    lines += ["### Within-job drift (end ÷ start, identical work)", ""]
    syn_drift, syn_jobs = drift_rows(docs, SYNTHETIC)
    pin_drift, _ = drift_rows(docs, PINNED)
    if not syn_drift and not pin_drift:
        lines += [
            "_No job produced both a start and an end reading, so drift could not be "
            "computed. The pair is the only thing that reveals it — a single reading "
            "cannot tell a slow machine from a machine that became slow._",
            "",
        ]
    else:
        lines += ["| series | jobs paired | p50 | p90 | range |", "|---|---:|---:|---:|---:|"]
        for name, rows in ((SYNTHETIC, syn_drift), (PINNED, pin_drift)):
            if not rows:
                continue
            d = [r["drift"] for r in rows]
            lines.append(
                f"| {name} | {len(rows)} of {len(syn_jobs) or len(rows)} "
                f"| {fmt_x(quantile(d, 0.5))} | {fmt_x(quantile(d, 0.9))} "
                f"| {fmt_x(min(d))} … {fmt_x(max(d))} |"
            )
        lines += [
            "",
            "_Above 1.00× the runner got SLOWER while the sweep ran, so that job's cells "
            "are inflated relative to its own start. Both series measure the same thing "
            "here — within one job our code cannot change — so they are a cross-check on "
            "each other. The branch÷prod ratio largely survives drift (both sides run "
            "seconds apart inside one criterion group), but a heavily drifted job's "
            "absolute medians should be read with it in mind._",
            "",
        ]
        flagged = sorted(
            (r for r in syn_drift if abs(r["drift"] - 1.0) > DRIFT_FLAG),
            key=lambda r: abs(r["drift"] - 1.0),
            reverse=True,
        )
        if flagged:
            lines += [
                f"<details><summary>{len(flagged)} job(s) past ±{DRIFT_FLAG:.0%} "
                "(synthetic series)</summary>",
                "",
                "| job | end ÷ start |",
                "|---|---:|",
            ]
            lines += [f"| `{r['job']}` | {fmt_x(r['drift'])} |" for r in flagged]
            lines += ["", "</details>", ""]

    cpus = sorted({d.get("cpu_model", "?") for d in docs.values()})
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
                "job\tseries\tposition\tsubject\tcpu_model\tlabel\titers\tmedian_ns"
                "\tmin_ns\tmax_ns\tp10_ns\tp90_ns\tspread_rel\trange_rel"
                "\twithin_spread_rel\tclock_tick_ns\tclock_read_ns\n"
            )
            for (job, series, pos), doc in sorted(docs.items()):
                c = doc.get("clock", {})
                for e in doc.get("entries", []):
                    f.write(
                        f"{job}\t{series}\t{pos}\t{doc.get('subject', '')}\t"
                        f"{doc.get('cpu_model', '?')}\t{e['label']}\t{e['iters']}\t"
                        f"{e['median_ns']}\t{e['min_ns']}\t{e['max_ns']}\t{e['p10_ns']}\t"
                        f"{e['p90_ns']}\t{e['spread_rel']}\t{e['range_rel']}\t"
                        f"{e['within_spread_rel']}\t{c.get('min_nonzero_ns')}\t"
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
