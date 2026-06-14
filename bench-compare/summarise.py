#!/usr/bin/env python3
"""Collate the branch-vs-prod Criterion results into a worst-first FOCUS table
across the full (op x width x scale) surface.

The table is a FOCUS FILTER, not a bucketing scheme: it surfaces, worst-first,
every cell that is a real, measurable regression vs prod (Δ% > +1 AND Δ > 10 ns)
— those are the cells to work. There are no MUST-FIX / defer / exempt states.
Cells outside the filter are simply not in the focus (at-or-near prod, or below
the comparator's ~10 ns resolution); they are still listed as data, unlabelled.

Walks `target/criterion` (which the aggregate job populates by merging every
per-(width,scale) bench cell into one tree), pairs each `branch` measurement
with its `prod` counterpart by median time, and renders ONE combined table
sorted by Δ% descending (worst regression first) — so a scale-dependent
regression that a single-scale bench used to hide is surfaced immediately.

The harness names each Criterion group `<op>_<W>_s<scale>` (e.g.
`exp_D307_s153`) with `<side>` (branch|prod) as the function id. Criterion
0.8 LOWERCASES the on-disk group directory (report.rs `.to_lowercase()`), so
the directory is `exp_d307_s153` — NOT the original-case id. We therefore read
the canonical-case `group_id` from each `benchmark.json` (which preserves
`exp_D307_s153`) rather than parsing the lowercased path segment.

Outputs:
  * stdout + `$GITHUB_STEP_SUMMARY` (when the workflow redirects): the markdown
    regression table.
  * `--tsv PATH`: the full per-cell medians as a TSV (op, width, scale, prod_ns,
    branch_ns, delta_ns, delta_pct, ratio).
  * `--md PATH`: the rendered markdown table written to a file too (for the
    downloadable aggregate artifact).
"""
import argparse
import glob
import json
import os
import re

ROOT = "target/criterion"

# Canonical group id `<op>_D<width>_s<scale>` (read from benchmark.json, which
# preserves case). `to_degrees`/`to_radians` contain `_`, so anchor on the
# `_D<n>_s<n>` suffix, not the first `_`.
_GROUP = re.compile(r"^(?P<op>.+)_D(?P<width>\d+)_s(?P<scale>\d+)$")

# (op, width:int, scale:int) -> {"branch": ns, "prod": ns}
data: dict[tuple, dict[str, float]] = {}

# Only the `new/` snapshot (the just-run measurement); `base/` is the prior
# baseline criterion keeps for its own delta and must be ignored. `estimates.json`
# is a SIBLING of `benchmark.json` in that same `new/` dir.
for bj in glob.glob(os.path.join(ROOT, "**", "new", "benchmark.json"), recursive=True):
    try:
        with open(bj) as f:
            meta = json.load(f)
    except (OSError, json.JSONDecodeError):
        continue
    group_id = meta.get("group_id")
    side = meta.get("function_id")
    if group_id is None or side not in ("branch", "prod"):
        continue
    m = _GROUP.match(group_id)
    if m is None:
        continue
    est = os.path.join(os.path.dirname(bj), "estimates.json")
    if not os.path.exists(est):
        continue
    try:
        with open(est) as f:
            med = json.load(f)["median"]["point_estimate"]
    except (OSError, json.JSONDecodeError, KeyError):
        continue
    key = (m.group("op"), int(m.group("width")), int(m.group("scale")))
    data.setdefault(key, {})[side] = med


def fmt(ns):
    if ns is None:
        return "—"
    if ns < 1e3:
        return f"{ns:.1f} ns"
    if ns < 1e6:
        return f"{ns / 1e3:.2f} µs"
    return f"{ns / 1e6:.2f} ms"


# Build rows: every (op,width,scale) cell that has BOTH a branch and prod median.
rows = []
skipped = []  # (op,width,scale) with a missing side
for (op, width, scale), sides in data.items():
    b = sides.get("branch")
    p = sides.get("prod")
    if b is None or p is None:
        skipped.append((op, width, scale))
        continue
    delta_ns = b - p
    delta_pct = (b / p - 1.0) * 100.0 if p else 0.0
    ratio = (b / p) if p else float("inf")
    rows.append(
        {
            "op": op,
            "width": width,
            "scale": scale,
            "prod": p,
            "branch": b,
            "delta_ns": delta_ns,
            "delta_pct": delta_pct,
            "ratio": ratio,
        }
    )

# Worst-first: Δ% descending.
rows.sort(key=lambda r: r["delta_pct"], reverse=True)

ref = os.environ.get("BENCH_REF", "branch")
prod = os.environ.get("PROD_VERSION", "?")

# FOCUS filter: the cells to work, worst-first. A cell is in focus when it is a
# real, measurable regression vs prod — Δ% > +1 AND Δ > 10 ns. The Δ > 10 ns
# floor is a MEASUREMENT limit: a sub-10 ns delta is below the comparator's own
# noise (unmeasurable), NOT an exemption. Cells outside the filter are simply not
# in the focus — they are listed plainly below as data, with no judgment label.
focus = [r for r in rows if r["delta_pct"] > 1.0 and r["delta_ns"] > 10.0]
rest = [r for r in rows if r not in focus]

lines = [
    f"## bench-branch-compare: `{ref}` (branch) vs prod `{prod}` (latest published tag)",
    "",
    "_Focus list across the full (op × width × scale) surface, worst-first. "
    "Positive Δ% = the branch is SLOWER than prod. "
    "In focus: Δ% > +1 AND Δ > 10 ns (the 10 ns floor is the bench-resolution "
    "limit — a smaller delta is unmeasurable, not exempt). Every other cell is "
    "listed below as data, unlabelled._",
    "",
    "| op | width | scale | prod | branch | Δ | Δ% | × |",
    "|---|---|---:|---:|---:|---:|---:|---:|",
]
for r in focus:
    lines.append(
        f"| {r['op']} | D{r['width']} | {r['scale']} "
        f"| {fmt(r['prod'])} | {fmt(r['branch'])} | {fmt(r['delta_ns'])} "
        f"| {r['delta_pct']:+.1f}% | {r['ratio']:.2f}× |"
    )

if not focus:
    lines.append("| _(no cell in focus: none past Δ% > +1 AND Δ > 10 ns)_ ||||||||")

lines += [
    "",
    f"_Total paired cells: {len(rows)}. In focus: {len(focus)}. "
    f"Not in focus (below the +1% / 10 ns focus filter — at-or-near prod, "
    f"or below bench resolution): {len(rest)}._",
]

# The not-in-focus cells, plainly, worst-first — data, no judgment label. These
# are filtered out of the focus queue, not "exempt" or "deferred".
if rest:
    lines += [
        "",
        "<details><summary>Cells not in focus (data only)</summary>",
        "",
        "| op | width | scale | prod | branch | Δ | Δ% | × |",
        "|---|---|---:|---:|---:|---:|---:|---:|",
    ]
    for r in rest:
        lines.append(
            f"| {r['op']} | D{r['width']} | {r['scale']} "
            f"| {fmt(r['prod'])} | {fmt(r['branch'])} | {fmt(r['delta_ns'])} "
            f"| {r['delta_pct']:+.1f}% | {r['ratio']:.2f}× |"
        )
    lines += ["", "</details>"]
if skipped:
    lines.append(
        f"_Incomplete cells (only one side measured): {len(skipped)} — "
        + ", ".join(f"{op}_D{w}_s{s}" for op, w, s in sorted(skipped)[:20])
        + ("…" if len(skipped) > 20 else "")
        + "._"
    )

table_md = "\n".join(lines)


def write_tsv(path: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write("op\twidth\tscale\tprod_ns\tbranch_ns\tdelta_ns\tdelta_pct\tratio\n")
        for r in rows:
            f.write(
                f"{r['op']}\tD{r['width']}\t{r['scale']}\t{r['prod']:.3f}\t{r['branch']:.3f}\t"
                f"{r['delta_ns']:.3f}\t{r['delta_pct']:.3f}\t{r['ratio']:.4f}\n"
            )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--tsv", help="write the full per-cell medians to this TSV path")
    ap.add_argument("--md", help="also write the rendered markdown table to this path")
    args = ap.parse_args()

    print(table_md)
    if args.md:
        with open(args.md, "w", encoding="utf-8") as f:
            f.write(table_md + "\n")
    if args.tsv:
        write_tsv(args.tsv)


if __name__ == "__main__":
    main()
