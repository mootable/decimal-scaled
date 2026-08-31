#!/usr/bin/env python3
"""Collate the cross-version bench-history cells into a (version × scale)
trend table per (op × width).

The workflow downloads one criterion subtree per version into
`hist-cells/bench-history-<tag>/...`. Each cell's harness names its groups
`<op>_<W>_s<scale>` (e.g. `mul_D38_s30`) with a single `t` function row;
Criterion 0.8 LOWERCASES the on-disk group dir, so we read the canonical-case
`group_id` from each `benchmark.json` rather than parsing the path.

For every (op, width, scale) we collect each version's median and render, per
(op × width), a markdown table with one row per scale and one column per
version — so a regression that only appears at a particular scale is visible
in the trend, not hidden behind a single canonical scale.

Outputs:
  * stdout + `$GITHUB_STEP_SUMMARY` (when redirected): the markdown tables.
  * `--tsv PATH`: every cell as TSV (op, width, scale, version, median_ns).
  * `--md PATH`: the rendered markdown written to a file (downloadable artifact).
"""
import argparse
import glob
import json
import os
import re

ROOT = "hist-cells"

# `hist-cells/bench-history-<tag>/.../<group>/new/benchmark.json`
_CELL = re.compile(r"bench-history-(?P<tag>[^/\\]+)")
_GROUP = re.compile(r"^(?P<op>.+)_D(?P<width>\d+)_s(?P<scale>\d+)$")

# Version display order (oldest -> newest). Tags not listed here are appended
# after, sorted, so a new version cell still renders without a code change.
VERSION_ORDER = [
    "v0.2.5", "v0.3.2", "v0.3.3", "v0.4.0", "v0.4.2", "v0.4.3", "v0.4.4", "HEAD",
]
VERSION_LABEL = {"HEAD": "main"}

# (op, width:int, scale:int, version) -> median_ns
data: dict[tuple, float] = {}
versions_seen: set[str] = set()

for bj in glob.glob(os.path.join(ROOT, "**", "new", "benchmark.json"), recursive=True):
    norm = bj.replace("\\", "/")
    mcell = _CELL.search(norm)
    if mcell is None:
        continue
    tag = mcell.group("tag")
    try:
        with open(bj) as f:
            meta = json.load(f)
    except (OSError, json.JSONDecodeError):
        continue
    group_id = meta.get("group_id")
    if group_id is None:
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
    key = (m.group("op"), int(m.group("width")), int(m.group("scale")), tag)
    data[key] = med
    versions_seen.add(tag)


def fmt(ns):
    if ns is None:
        return "—"
    if ns < 1e3:
        return f"{ns:.1f} ns"
    if ns < 1e6:
        return f"{ns / 1e3:.2f} µs"
    return f"{ns / 1e6:.2f} ms"


def ordered_versions() -> list[str]:
    known = [v for v in VERSION_ORDER if v in versions_seen]
    extra = sorted(v for v in versions_seen if v not in VERSION_ORDER)
    return known + extra


versions = ordered_versions()

# (op, width) -> scale -> version -> ns
by_chart: dict[tuple, dict[int, dict[str, float]]] = {}
for (op, width, scale, tag), ns in data.items():
    by_chart.setdefault((op, width), {}).setdefault(scale, {})[tag] = ns

lines = [
    "## bench-history: cross-version (version × scale) trend",
    "",
    "_Median time per `(op × width)`, one row per scale, one column per "
    "published version (oldest → newest). A blank cell = that version lacked "
    "the function (no fabricated point)._",
    "",
]

for (op, width) in sorted(by_chart):
    lines.append(f"### {op} — D{width}")
    lines.append("")
    header = "| scale | " + " | ".join(VERSION_LABEL.get(v, v) for v in versions) + " |"
    sep = "|---:|" + "|".join("---:" for _ in versions) + "|"
    lines.append(header)
    lines.append(sep)
    for scale in sorted(by_chart[(op, width)]):
        per_v = by_chart[(op, width)][scale]
        cells = " | ".join(fmt(per_v.get(v)) for v in versions)
        lines.append(f"| {scale} | {cells} |")
    lines.append("")

if not by_chart:
    lines.append("_(no cells collated)_")

table_md = "\n".join(lines)


def write_tsv(path: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write("op\twidth\tscale\tversion\tmedian_ns\n")
        for (op, width, scale, tag), ns in sorted(data.items()):
            f.write(f"{op}\tD{width}\t{scale}\t{tag}\t{ns:.3f}\n")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--tsv", help="write every cell to this TSV path")
    ap.add_argument("--md", help="also write the rendered markdown to this path")
    args = ap.parse_args()

    print(table_md)
    if args.md:
        with open(args.md, "w", encoding="utf-8") as f:
            f.write(table_md + "\n")
    if args.tsv:
        write_tsv(args.tsv)


if __name__ == "__main__":
    main()
