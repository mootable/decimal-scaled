#!/usr/bin/env python3
"""Summarise criterion estimates.json files under a benchmark directory.

Usage:
    python scripts/bench_summary.py <bench-dir> [<compare-dir>]
"""
import json
import os
import sys


def collect(root):
    out = {}
    for dirpath, _, filenames in os.walk(root):
        if "estimates.json" in filenames and dirpath.endswith(os.sep + "new"):
            with open(os.path.join(dirpath, "estimates.json")) as fh:
                est = json.load(fh)
            mean_ns = est["mean"]["point_estimate"]
            # bench id is the relative path stripped of /new
            rel = os.path.relpath(os.path.dirname(dirpath), root)
            out[rel.replace("\\", "/")] = mean_ns
    return out


def fmt(ns):
    if ns >= 1e9:
        return f"{ns / 1e9:8.3f}  s"
    if ns >= 1e6:
        return f"{ns / 1e6:8.3f} ms"
    if ns >= 1e3:
        return f"{ns / 1e3:8.3f} us"
    return f"{ns:8.3f} ns"


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    base = collect(sys.argv[1])
    if len(sys.argv) == 2:
        for name in sorted(base):
            print(f"{name:40s}  {fmt(base[name])}")
        return
    cmp = collect(sys.argv[2])
    keys = sorted(set(base) | set(cmp))
    print(f"{'bench':40s}  {'baseline':>12s}  {'new':>12s}  {'ratio':>7s}")
    for k in keys:
        b = base.get(k)
        n = cmp.get(k)
        if b is None or n is None:
            continue
        ratio = n / b if b else float("nan")
        arrow = "  " if 0.95 <= ratio <= 1.05 else ("v " if ratio < 1.0 else "^ ")
        print(f"{k:40s}  {fmt(b)}  {fmt(n)}  {arrow}{ratio:5.2f}")


if __name__ == "__main__":
    main()
