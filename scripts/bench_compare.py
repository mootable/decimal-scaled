#!/usr/bin/env python3
"""Compare averaged criterion estimates across two sets of bench runs.

Usage:
    python scripts/bench_compare.py <baseline-glob> <new-glob>

Each argument is a glob pattern (shell-quoted) matching one or more
criterion output directories. Each matched directory should contain
estimates.json files in `<group>/<bench>/new/`.

Example:
    python scripts/bench_compare.py 'bench-results/baseline-*/D57_s20' \
                                    'bench-results/narrow-*/D57_s20'
"""
import glob
import json
import os
import statistics
import sys


def collect(root):
    """Returns {bench_id: mean_ns} for one criterion root."""
    out = {}
    for dirpath, _, filenames in os.walk(root):
        if "estimates.json" in filenames and dirpath.endswith(os.sep + "new"):
            with open(os.path.join(dirpath, "estimates.json")) as fh:
                est = json.load(fh)
            mean_ns = est["mean"]["point_estimate"]
            rel = os.path.relpath(os.path.dirname(dirpath), root)
            out[rel.replace("\\", "/")] = mean_ns
    return out


def collect_many(pattern):
    """Returns {bench_id: [mean_ns, mean_ns, ...]} averaged across all roots
    matching the glob pattern."""
    combined = {}
    roots = sorted(glob.glob(pattern, recursive=True))
    if not roots:
        print(f"WARN: no roots match {pattern!r}", file=sys.stderr)
    for root in roots:
        single = collect(root)
        for k, v in single.items():
            combined.setdefault(k, []).append(v)
    return combined, roots


def fmt(ns):
    if ns >= 1e9:
        return f"{ns / 1e9:8.3f}  s"
    if ns >= 1e6:
        return f"{ns / 1e6:8.3f} ms"
    if ns >= 1e3:
        return f"{ns / 1e3:8.3f} us"
    return f"{ns:8.3f} ns"


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        sys.exit(1)
    base_pat, new_pat = sys.argv[1], sys.argv[2]
    base, base_roots = collect_many(base_pat)
    new, new_roots = collect_many(new_pat)
    print(f"baseline: {len(base_roots)} root(s)")
    for r in base_roots:
        print(f"  {r}")
    print(f"new:      {len(new_roots)} root(s)")
    for r in new_roots:
        print(f"  {r}")
    print()
    keys = sorted(set(base) | set(new))
    print(f"{'bench':40s}  {'baseline (med)':>14s}  {'new (med)':>14s}  {'ratio':>7s}  {'verdict':>8s}")
    print("-" * 90)
    for k in keys:
        b_samples = base.get(k, [])
        n_samples = new.get(k, [])
        if not b_samples or not n_samples:
            continue
        b_med = statistics.median(b_samples)
        n_med = statistics.median(n_samples)
        ratio = n_med / b_med if b_med else float("nan")
        if ratio < 0.85:
            verdict = "  WIN"
        elif ratio < 0.95:
            verdict = "  win"
        elif ratio > 1.15:
            verdict = "  LOSS"
        elif ratio > 1.05:
            verdict = "  loss"
        else:
            verdict = "   ~ "
        print(f"{k:40s}  {fmt(b_med):>14s}  {fmt(n_med):>14s}  {ratio:7.3f}  {verdict:>8s}")


if __name__ == "__main__":
    main()
