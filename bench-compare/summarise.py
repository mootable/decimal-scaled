#!/usr/bin/env python3
"""Render the branch-vs-prod Criterion results as markdown tables.

Walks `target/criterion` (which the collator job populates by merging every
per-width bench artifact into one tree), pairs each `branch/<fn>` benchmark
with its `prod/<fn>` counterpart by median time, and prints ONE GitHub-
flavoured markdown table PER WIDTH (branch | prod | Δ%) to stdout — the
workflow appends them to `$GITHUB_STEP_SUMMARY` so the comparison is visible
directly in the run.

Tables are ordered by NUMERIC width ascending (D18, D38, … D1232), and rows
within a table by numeric-aware function key, so D1232 never sorts ahead of
the narrower widths the way a plain lexical sort would.
"""
import glob
import json
import os
import re

ROOT = "target/criterion"

# width-label (e.g. "D18") -> { op (e.g. "add") -> {"branch": ns, "prod": ns} }
#
# The harness names each Criterion group `<op>/<W>` (e.g. `add/D18`) with the
# `<side>` (branch|prod) as the function. Criterion SANITISES the `/` in a
# group id to `_` on disk, so the actual report dir is `<op>_<W>` (e.g.
# `add_D18`, `to_degrees_D115`) — NOT a nested `<op>/<W>`. Hence the path
# segments below `target/criterion` are [<op>_<W>, side]. Parse the group
# segment by stripping its trailing `_D<n>` width suffix (ops like
# `to_degrees`/`to_radians` themselves contain `_`, so match the suffix, not
# the first `_`).
_GROUP = re.compile(r"^(?P<op>.+)_(?P<width>D\d+)$")
data: dict[str, dict[str, dict[str, float]]] = {}
for est in glob.glob(os.path.join(ROOT, "**", "new", "estimates.json"), recursive=True):
    rel = os.path.relpath(est, ROOT).replace(os.sep, "/").split("/")[:-2]  # drop new/estimates.json
    side = next((s for s in ("branch", "prod") if s in rel), None)
    if side is None:
        continue
    group = "/".join(p for p in rel if p != side)  # the `<op>_<W>` dir
    m = _GROUP.match(group)
    if m is None:
        continue
    op, width = m.group("op"), m.group("width")
    with open(est) as f:
        med = json.load(f)["median"]["point_estimate"]
    data.setdefault(width, {}).setdefault(op, {})[side] = med


def fmt(ns):
    if ns is None:
        return "—"
    if ns < 1e3:
        return f"{ns:.1f} ns"
    if ns < 1e6:
        return f"{ns / 1e3:.2f} µs"
    return f"{ns / 1e6:.2f} ms"


def width_num(label: str) -> int:
    """Integer parsed from a `D<n>` label so widths sort numerically
    (D1232 last), never lexically (which would float D1232 ahead of D153)."""
    return int(label.lstrip("D").split("_")[0])


ref = os.environ.get("BENCH_REF", "branch")
prod = os.environ.get("PROD_VERSION", "?")

lines = [
    f"## bench-branch-compare: `{ref}` (branch) vs prod `{prod}` (latest published tag)",
    "",
    "_One table per width, narrowest first. Negative Δ = the branch is faster than prod._",
]

for width in sorted(data, key=width_num):
    ops = data[width]
    lines += [
        "",
        f"### {width}",
        "",
        "| op | branch | prod | Δ (branch vs prod) |",
        "|---|---:|---:|---:|",
    ]
    for op in sorted(ops):
        b = ops[op].get("branch")
        p = ops[op].get("prod")
        delta = f"{(b / p - 1) * 100:+.1f}%" if (b and p) else "—"
        lines.append(f"| {op} | {fmt(b)} | {fmt(p)} | {delta} |")

print("\n".join(lines))
