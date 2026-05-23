#!/usr/bin/env python3
"""Render the branch-vs-prod Criterion results as a markdown table.

Walks `target/criterion`, pairs each `branch/<fn>` benchmark with its
`prod/<fn>` counterpart by median time, and prints a GitHub-flavoured
markdown table (branch | prod | Δ%) to stdout — the workflow appends it to
`$GITHUB_STEP_SUMMARY` so the comparison is visible directly in the run.
"""
import glob
import json
import os

ROOT = "target/criterion"

# key (op/width, side-token removed) -> {"branch": ns, "prod": ns}
data: dict[str, dict[str, float]] = {}
for est in glob.glob(os.path.join(ROOT, "**", "new", "estimates.json"), recursive=True):
    rel = os.path.relpath(est, ROOT).replace(os.sep, "/").split("/")[:-2]  # drop new/estimates.json
    side = next((s for s in ("branch", "prod") if s in rel), None)
    if side is None:
        continue
    key = "/".join(p for p in rel if p != side)
    with open(est) as f:
        data.setdefault(key, {})[side] = json.load(f)["median"]["point_estimate"]


def fmt(ns):
    if ns is None:
        return "—"
    if ns < 1e3:
        return f"{ns:.1f} ns"
    if ns < 1e6:
        return f"{ns / 1e3:.2f} µs"
    return f"{ns / 1e6:.2f} ms"


ref = os.environ.get("BENCH_REF", "branch")
prod = os.environ.get("PROD_VERSION", "?")

lines = [
    f"### bench-branch-compare: `{ref}` (branch) vs prod `{prod}` (latest published tag)",
    "",
    "| op / width | branch | prod | Δ (branch vs prod) |",
    "|---|---:|---:|---:|",
]
for key in sorted(data):
    b = data[key].get("branch")
    p = data[key].get("prod")
    delta = f"{(b / p - 1) * 100:+.1f}%" if (b and p) else "—"
    lines.append(f"| {key} | {fmt(b)} | {fmt(p)} | {delta} |")
lines += ["", "_Negative Δ = the branch is faster than prod._"]
print("\n".join(lines))
