#!/usr/bin/env python3
"""Walk a tree of Criterion artifacts (per-width `lib_cmp_d{N}` bench
output) and emit the same `<source>\\t<id>\\t<median>\\t<unit>` TSV that
`examples/chart_gen.rs` consumes.

Designed for the per-width artifact layout uploaded by the
`bench-full` workflow with `bench_family = lib_cmp`:

    lib-cmp-results/
      criterion-lib_cmp-D<N>/
        lib_cmp_<bitwidth>_s<scale>/
          <lib>_<op>/
            new/
              benchmark.json   <- canonical id ("lib_cmp/<bw>_s<s>/<lib>/<op>")
              estimates.json   <- median.point_estimate is in ns

We merge with the existing `target/medians.tsv`: every row whose id
starts with `lib_cmp/<bitwidth>_s` for a width we have fresh data on
is dropped, then fresh rows replace them. Rows from widths we don't
have data on (notably 128bit / D38 while that job is still grinding)
stay untouched. Non-`lib_cmp/...` rows are kept as-is.

Usage:
    python scripts/lib_cmp_ingest.py \\
        --artifacts lib-cmp-results \\
        --existing target/medians.tsv \\
        --out target/medians.tsv
"""
import argparse
import json
import os
import re
import sys

# D-type -> storage bit width. The lib_cmp group label is
# `<bitwidth>bit_s<scale>`. Matches `benches/lib_cmp_d{N}.rs`.
WIDTH_MAP = {
    "D9":    "32",
    "D18":   "64",
    "D38":   "128",
    "D57":   "192",
    "D76":   "256",
    "D115":  "384",
    "D153":  "512",
    "D230":  "768",
    "D307":  "1024",
    "D462":  "1536",
    "D616":  "2048",
    "D924":  "3072",
    "D1232": "4096",
}


def collect_from_artifacts(root):
    """Walk every `criterion-lib_cmp-D*/` subtree under `root` and
    yield (source, id, value_str, unit) rows for each bench leaf.
    `value_str` is formatted to 5 significant figures to match the
    existing TSV's precision."""
    rows = []
    if not os.path.isdir(root):
        sys.exit(f"artifacts root not found: {root}")
    seen_widths = set()
    for entry in sorted(os.listdir(root)):
        m = re.match(r"^criterion-lib_cmp-(D\d+)$", entry)
        if not m:
            continue
        dtype = m.group(1)
        bw = WIDTH_MAP.get(dtype)
        if bw is None:
            print(f"skip unknown D-type: {dtype}", file=sys.stderr)
            continue
        seen_widths.add(bw)
        src = f"lib_cmp_d{dtype[1:].lower()}"  # D9 -> lib_cmp_d9
        dtype_root = os.path.join(root, entry)
        # Each group dir is `lib_cmp_<bw>bit_s<scale>`.
        for grp in sorted(os.listdir(dtype_root)):
            gpath = os.path.join(dtype_root, grp)
            if not grp.startswith(f"lib_cmp_{bw}bit_s"):
                continue
            if not os.path.isdir(gpath):
                continue
            for leaf in sorted(os.listdir(gpath)):
                leaf_new = os.path.join(gpath, leaf, "new")
                bj = os.path.join(leaf_new, "benchmark.json")
                ej = os.path.join(leaf_new, "estimates.json")
                if not (os.path.isfile(bj) and os.path.isfile(ej)):
                    continue
                with open(bj, encoding="utf-8") as f:
                    bm = json.load(f)
                with open(ej, encoding="utf-8") as f:
                    est = json.load(f)
                full_id = bm.get("full_id")
                med = est.get("median", {}).get("point_estimate")
                if full_id is None or med is None:
                    continue
                # Criterion's `point_estimate` is in nanoseconds.
                # Pick a unit that keeps the rendered number in [1, 1000)
                # so the TSV looks like the existing one.
                value, unit = pick_unit(med)
                rows.append((src, full_id, value, unit))
    return rows, seen_widths


def pick_unit(ns):
    if ns < 1.0:
        return f"{ns * 1000.0:.4g}", "ps"
    if ns < 1000.0:
        return f"{ns:.5g}", "ns"
    if ns < 1_000_000.0:
        return f"{ns / 1000.0:.5g}", "µs"
    if ns < 1_000_000_000.0:
        return f"{ns / 1_000_000.0:.5g}", "ms"
    return f"{ns / 1_000_000_000.0:.5g}", "s"


def load_existing(path):
    if not os.path.isfile(path):
        return []
    rows = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line:
                continue
            cols = line.split("\t")
            if len(cols) != 4:
                continue
            rows.append(tuple(cols))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--artifacts", required=True,
                    help="root containing criterion-lib_cmp-D*/ dirs")
    ap.add_argument("--existing", required=True,
                    help="existing TSV to merge with (kept for widths "
                         "we have no fresh data on)")
    ap.add_argument("--out", required=True, help="output TSV path")
    args = ap.parse_args()

    fresh, fresh_widths = collect_from_artifacts(args.artifacts)
    print(f"fresh rows: {len(fresh)} across widths: "
          f"{sorted(int(w) for w in fresh_widths)}", file=sys.stderr)

    # Build a prefix set of `lib_cmp/<bw>bit_s` for every width we
    # have fresh data for, so we can drop stale rows of those widths
    # from the existing TSV.
    drop_prefixes = tuple(f"lib_cmp/{bw}bit_s" for bw in fresh_widths)

    kept = []
    dropped = 0
    for src, id_, val, unit in load_existing(args.existing):
        if id_.startswith(drop_prefixes):
            dropped += 1
            continue
        kept.append((src, id_, val, unit))
    print(f"kept {len(kept)} existing rows, dropped {dropped} stale "
          f"rows for {len(fresh_widths)} widths", file=sys.stderr)

    # Concatenate; dedupe by (source, id) keeping the last (fresh) row.
    by_key = {}
    for row in kept + fresh:
        by_key[(row[0], row[1])] = row

    # Force UTF-8 + LF on stdout: chart_gen.rs reads the file
    # expecting the literal "µs" bytes, and Python's default Windows
    # code page would mangle them.
    out = open(args.out, "w", encoding="utf-8", newline="\n")
    for (src, id_), row in sorted(by_key.items()):
        out.write("\t".join(row) + "\n")
    out.close()
    print(f"wrote {args.out} ({len(by_key)} rows)", file=sys.stderr)


if __name__ == "__main__":
    main()
