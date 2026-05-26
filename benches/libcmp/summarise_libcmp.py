#!/usr/bin/env python3
"""Collate the per-(width, scale) library-comparison cells into a peer-timing
table across the full (op × width × scale) surface.

The workflow merges every per-(width,scale) bench cell into one criterion
tree. The harness names each group `lib_cmp/<bit>bit_s<scale>` and each row
`<library>/<op>` (e.g. group `lib_cmp/512bit_s76`, function `dashu-float/ln`),
so the full benched id is `lib_cmp/512bit_s76/dashu-float/ln`.

For every (bit-width, scale, op) we collect each library's median and render
ONE table sorted worst-first by the decimal-scaled-vs-best-peer ratio (so the
ops where decimal-scaled trails the fastest peer surface first), at the scale
they occur — the single-scale blindness the fan-out removes.

Outputs:
  * stdout + `$GITHUB_STEP_SUMMARY` (when redirected): the markdown table.
  * `--tsv PATH`: every cell as TSV (bit, scale, op, library, median_ns).
  * `--md PATH`: the rendered markdown written to a file (downloadable artifact).

Criterion 0.8 LOWERCASES the on-disk group dir, so read the canonical-case
`group_id`/`function_id` from each `benchmark.json` rather than the path.
"""
import argparse
import glob
import json
import os
import re

ROOT = "target/criterion"

# group_id `lib_cmp/<bit>bit_s<scale>`, function_id `<library>/<op>`.
_GROUP = re.compile(r"^lib_cmp/(?P<bit>\d+)bit_s(?P<scale>\d+)$")
_FUNC = re.compile(r"^(?P<lib>.+)/(?P<op>[^/]+)$")

OURS = "decimal-scaled"

# (bit:int, scale:int, op) -> {library: median_ns}
data: dict[tuple, dict[str, float]] = {}

for bj in glob.glob(os.path.join(ROOT, "**", "new", "benchmark.json"), recursive=True):
    try:
        with open(bj) as f:
            meta = json.load(f)
    except (OSError, json.JSONDecodeError):
        continue
    gid = meta.get("group_id")
    fid = meta.get("function_id")
    if gid is None or fid is None:
        continue
    mg = _GROUP.match(gid)
    mf = _FUNC.match(fid)
    if mg is None or mf is None:
        continue
    est = os.path.join(os.path.dirname(bj), "estimates.json")
    if not os.path.exists(est):
        continue
    try:
        with open(est) as f:
            med = json.load(f)["median"]["point_estimate"]
    except (OSError, json.JSONDecodeError, KeyError):
        continue
    key = (int(mg.group("bit")), int(mg.group("scale")), mf.group("op"))
    data.setdefault(key, {})[mf.group("lib")] = med


def fmt(ns):
    if ns is None:
        return "—"
    if ns < 1e3:
        return f"{ns:.1f} ns"
    if ns < 1e6:
        return f"{ns / 1e3:.2f} µs"
    return f"{ns / 1e6:.2f} ms"


# Build rows: every (bit, scale, op) where decimal-scaled ran. The "ratio" is
# decimal-scaled / fastest peer (>1 = a peer beats us; <1 = we win).
rows = []
all_libs: set[str] = set()
for (bit, scale, op), libs in data.items():
    all_libs.update(libs)
    ours = libs.get(OURS)
    peers = {k: v for k, v in libs.items() if k != OURS}
    best_peer = min(peers.values()) if peers else None
    ratio = (ours / best_peer) if (ours and best_peer) else None
    rows.append(
        {
            "bit": bit,
            "scale": scale,
            "op": op,
            "ours": ours,
            "best_peer": best_peer,
            "ratio": ratio,
            "libs": libs,
        }
    )

# Worst-first: largest decimal-scaled/best-peer ratio first (None ratios last).
rows.sort(key=lambda r: (r["ratio"] is None, -(r["ratio"] or 0.0)))

lines = [
    "## library-comparison: decimal-scaled vs peers (op × width × scale)",
    "",
    "_Worst-first by `decimal-scaled / fastest-peer` median ratio (× > 1 = a "
    "peer is faster than decimal-scaled at that cell). Surfaced across the full "
    "(op × width × scale) surface so a scale-dependent gap shows where it occurs._",
    "",
    "| op | width | scale | decimal-scaled | fastest peer | × (ours/peer) |",
    "|---|---:|---:|---:|---:|---:|",
]
for r in rows:
    ratio = f"{r['ratio']:.2f}×" if r["ratio"] is not None else "—"
    lines.append(
        f"| {r['op']} | {r['bit']}bit | {r['scale']} "
        f"| {fmt(r['ours'])} | {fmt(r['best_peer'])} | {ratio} |"
    )

if not rows:
    lines.append("| _(no cells collated)_ ||||||")

lines += [
    "",
    f"_Total cells: {len(rows)}. Libraries seen: {', '.join(sorted(all_libs))}._",
]

table_md = "\n".join(lines)


def write_tsv(path: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write("bit\tscale\top\tlibrary\tmedian_ns\n")
        for (bit, scale, op), libs in sorted(data.items()):
            for lib, ns in sorted(libs.items()):
                f.write(f"{bit}\t{scale}\t{op}\t{lib}\t{ns:.3f}\n")


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
