#!/usr/bin/env python3
"""Render the precision LSBe (ULP) shootout markdown table FROM the
committed `results/precision/*.tsv` files — the single source of truth
produced by `benches/lib_cmp_precision.rs`.

This is the Python sibling of the `render_from_files` routine in
`benches/lib_cmp_precision.rs`: same column order, same cell format, so
the README / docs tables can be regenerated without re-running the
multi-minute oracle sweep. Every cell traces back to exactly one TSV
row; nothing is hand-typed or recomputed.

Cell format mirrors the bench renderer:
    `<lsbe> (<ulp>)`   e.g. `0 (0)`, `67 (1.1e20)`
    `n/a`              method not exposed / width-scale not representable

Usage:
    python scripts/render_precision_table.py --dir results/precision \\
        --width D38
    python scripts/render_precision_table.py            # all widths
"""
import argparse
import os
import sys

# Library row order — matches `libs` in benches/lib_cmp_precision.rs.
LIBS = [
    "decimal-scaled",
    "fastnum",
    "rust_decimal",
    "dashu-float",
    "decimal-rs",
    "bigdecimal",
]

# D-type -> canonical scale, matches Width::canonical_scale() and the
# scale column in the TSVs.
SCALE = {"D38": 19, "D76": 35}


def file_stem(name):
    return name.replace("-", "_")


def fmt_ulp(max_ulp):
    """Mirror the bench renderer's ULP formatting: 0 -> "0",
    < 10 -> 2dp, otherwise 1-sig-fig scientific."""
    try:
        u = float(max_ulp)
    except ValueError:
        return "0"
    if u == 0.0:
        return "0"
    if u < 10.0:
        return f"{u:.2f}"
    # Rust's {:.1e} prints `1.1e20` (no `+`, no zero-padded exponent);
    # Python's default is `1.1e+20`, so normalise to match the bench.
    mant, exp = f"{u:.1e}".split("e")
    return f"{mant}e{int(exp)}"


def read_tsv(path, width):
    """(method -> (kind, max_lsbe, max_ulp)) for the given width."""
    cells = {}
    mode = None
    if not os.path.isfile(path):
        return cells, mode
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or line.startswith("#") or line.startswith("method\t"):
                continue
            cols = line.split("\t")
            if len(cols) < 9 or cols[1] != width:
                continue
            if mode is None:
                mode = cols[3]
            # method -> (kind, max_lsbe, max_ulp); preserve first seen.
            cells.setdefault(cols[0], (cols[4], cols[5], cols[6]))
    return cells, mode


def methods_in_order(dir_, width):
    """Method column order = the order rows appear in the canonical
    decimal-scaled TSV (which is Method::TRANSCENDENTAL order)."""
    order = []
    path = os.path.join(dir_, "decimal_scaled.tsv")
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if not line or line.startswith("#") or line.startswith("method\t"):
                continue
            cols = line.split("\t")
            if len(cols) < 9 or cols[1] != width:
                continue
            if cols[0] not in order:
                order.append(cols[0])
    return order


def render_width(dir_, width, only=None):
    methods = methods_in_order(dir_, width)
    if only:
        wanted = [m.strip() for m in only.split(",")]
        methods = [m for m in wanted if m in methods]
    data = {}
    modes = {}
    for lib in LIBS:
        cells, mode = read_tsv(os.path.join(dir_, file_stem(lib) + ".tsv"), width)
        data[lib] = cells
        modes[lib] = mode or "-"

    out = []
    out.append(f"## {width} (scale {SCALE.get(width, '?')}) — LSBε (ULP)\n")
    out.append(
        "Cell = LSBε (max |ULP distance to true|). 0 (0) = correctly "
        "rounded (bit-exact under the subject's reported mode). `n/a` = "
        "method not exposed or width/scale not representable.\n"
    )
    header = "| library | mode |" + "".join(f" {m} |" for m in methods)
    sep = "|---|---|" + "---|" * len(methods)
    out.append(header)
    out.append(sep)
    for lib in LIBS:
        cells = data[lib]
        row = f"| {lib} | {modes[lib]} |"
        for m in methods:
            cell = cells.get(m)
            if cell is None or cell[0] == "na":
                s = "n/a"
            else:
                s = f"{cell[1]} ({fmt_ulp(cell[2])})"
            row += f" {s} |"
        out.append(row)
    out.append("")
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dir", default="results/precision",
                    help="directory holding the per-library TSV files")
    ap.add_argument("--width", default=None,
                    help="render a single width (e.g. D38); default all")
    ap.add_argument("--methods", default=None,
                    help="comma-separated method subset + order "
                         "(e.g. sqrt,cbrt,exp,ln); default = all in the TSV")
    args = ap.parse_args()

    if not os.path.isdir(args.dir):
        sys.exit(f"results dir not found: {args.dir}")

    widths = [args.width] if args.width else ["D38", "D76"]
    out = sys.stdout
    out.reconfigure(encoding="utf-8")
    for w in widths:
        out.write(render_width(args.dir, w, only=args.methods))
        out.write("\n")


if __name__ == "__main__":
    main()
