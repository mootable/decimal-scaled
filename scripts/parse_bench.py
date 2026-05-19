#!/usr/bin/env python3
"""Quick bench-trial artifact reader.

Usage: parse_bench.py <artifact-dir> [filter-substr]
"""
import json
import os
import sys


def main():
    root = sys.argv[1] if len(sys.argv) > 1 else "."
    needle = sys.argv[2] if len(sys.argv) > 2 else ""
    results = []
    for dirpath, dirnames, filenames in os.walk(root):
        if "estimates.json" in filenames and dirpath.endswith("/new"):
            with open(os.path.join(dirpath, "estimates.json")) as fh:
                data = json.load(fh)
            mean = data["mean"]["point_estimate"]
            label = dirpath[len(root):].rstrip("/new").lstrip("./")
            if needle in label:
                results.append((label, mean))
    results.sort()
    for label, mean in results:
        if mean < 1e3:
            print(f"{mean:8.2f} ns  {label}")
        elif mean < 1e6:
            print(f"{mean/1e3:8.2f} us  {label}")
        else:
            print(f"{mean/1e6:8.2f} ms  {label}")


if __name__ == "__main__":
    main()
