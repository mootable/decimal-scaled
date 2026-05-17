"""Parse a samply / Firefox Profiler gzipped JSON and print the
top N functions by self-sample count.

Self-time = sample where the function is the leaf frame.
Includes a second pass that walks each stack and tallies
inclusive (any-frame) counts too, so you can see both
"where the CPU is" and "what called what".

Usage: python scripts/samply_hot_frames.py trace/exp_samply.json.gz [--top 30]
"""

import sys
import json
import gzip
import argparse
from collections import Counter


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("profile")
    p.add_argument("--top", type=int, default=30)
    p.add_argument(
        "--filter",
        default="decimal_scaled",
        help="only print functions matching this substring",
    )
    args = p.parse_args()

    with gzip.open(args.profile, "rt", encoding="utf-8") as f:
        prof = json.load(f)

    total_self: Counter[str] = Counter()
    total_inclusive: Counter[str] = Counter()
    total_samples = 0

    for thread in prof["threads"]:
        strings = thread["stringArray"]
        func_table = thread["funcTable"]
        frame_table = thread["frameTable"]
        stack_table = thread["stackTable"]
        samples = thread["samples"]

        if samples["length"] == 0:
            continue

        func_name = lambda fi: strings[func_table["name"][fi]]
        frame_func = frame_table["func"]
        stack_frame = stack_table["frame"]
        stack_prefix = stack_table["prefix"]

        def stack_leaf(stack_idx):
            return func_name(frame_func[stack_frame[stack_idx]])

        def walk_stack(stack_idx):
            while stack_idx is not None:
                yield func_name(frame_func[stack_frame[stack_idx]])
                stack_idx = stack_prefix[stack_idx]

        for stack_idx in samples["stack"]:
            if stack_idx is None:
                continue
            total_samples += 1
            total_self[stack_leaf(stack_idx)] += 1
            for fn in set(walk_stack(stack_idx)):
                total_inclusive[fn] += 1

    print(f"Total samples: {total_samples}")
    print(f"\n=== Top {args.top} by SELF time (leaf frame) ===")
    for name, count in total_self.most_common():
        if args.filter and args.filter not in name:
            continue
        pct = 100.0 * count / total_samples
        print(f"  {pct:5.1f}%  ({count:5}) {name}")
        if args.top <= 0:
            break
        args.top -= 1
        if args.top == 0:
            break

    print(f"\n=== Top 30 by INCLUSIVE time (any frame on stack) ===")
    n = 30
    for name, count in total_inclusive.most_common():
        if args.filter and args.filter not in name:
            continue
        pct = 100.0 * count / total_samples
        print(f"  {pct:5.1f}%  ({count:5}) {name}")
        n -= 1
        if n == 0:
            break


if __name__ == "__main__":
    main()
