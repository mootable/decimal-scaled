#!/usr/bin/env python3
"""Walk a tree of Criterion ``full_matrix`` artifacts (per-width
``criterion-full_matrix-D{N}`` JSON output) and emit the §1 arithmetic
and §3 strict-transcendental timing tables for ``docs/benchmarks.md``.

The ``bench-full`` workflow uploads, per storage width, a tree of the
shape::

    criterion-full_matrix-D<N>/
      arith/
        D<N>_s<scale>_<op>/
          new/
            benchmark.json   <- canonical id ("arith/D<N>_s<s>/<op>")
            estimates.json    <- median.point_estimate is in ns
      strict/  (narrow tiers: D9 / D18 / D38)
        D<N>_s<scale>_<fn>/...
      strict_wide/  (wide tiers: D57 .. D1232)
        D<N>_s<scale>_<fn>/...

Only the crate's own ``D<N>_...`` ids are consumed; baseline ids
(``rust_decimal_*``, ``bnum_*``, ``fixed_*``) and the ``lib_cmp_*``
figure ids are ignored — those belong to the §5 figures pipeline
(``scripts/lib_cmp_ingest.py``).

Rendering matches the conventions documented at the top of
``benchmarks.md``:

- Per row, every cell renders in one unit — the row's *median*
  natural unit under the ordering ps < ns < µs < ms < s — so the
  cells compare directly.
- Picosecond cells render with the same precision the existing tables
  use; ns/µs/ms cells carry 4–5 significant figures.

This is a read-only reporter: it prints the medians (``--dump``) or the
rendered markdown tables (``--tables``) to stdout. It never fabricates
a value — a missing leaf is reported, not guessed.

Usage::

    python scripts/full_matrix_ingest.py --artifacts <root> --tables
    python scripts/full_matrix_ingest.py --artifacts <root> --dump
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys

# Storage width order and the bit-width label shown in §1 headings.
WIDTH_BITS = {
    "D9": 32, "D18": 64, "D38": 128, "D57": 192, "D76": 256,
    "D115": 384, "D153": 512, "D230": 768, "D307": 1024,
    "D462": 1536, "D616": 2048, "D924": 3072, "D1232": 4096,
}
WIDTH_ORDER = list(WIDTH_BITS.keys())

# The three scale columns per width in §1: smallest / midpoint / largest.
ARITH_SCALES = {
    "D9": [0, 5, 9], "D18": [0, 9, 18], "D38": [0, 19, 38],
    "D57": [0, 28, 56], "D76": [0, 35, 76], "D115": [0, 57, 114],
    "D153": [0, 75, 153], "D230": [0, 115, 230], "D307": [0, 150, 307],
    "D462": [0, 230, 461], "D616": [0, 308, 615], "D924": [0, 461, 923],
    "D1232": [0, 616, 1231],
}
OPS = ["add", "sub", "mul", "div", "rem", "neg"]

# §3 strict mid-scale column per width (the "s = mid" honest series cost).
STRICT_MID = {
    "D9": 5, "D18": 9, "D38": 19, "D57": 28, "D76": 35, "D115": 57,
    "D153": 75, "D230": 115, "D307": 150, "D462": 230, "D616": 308,
    "D924": 461, "D1232": 616,
}
STRICT_FNS = ["ln", "exp", "sin", "sqrt"]

_UNIT_DIV = {"ps": 1e-3, "ns": 1.0, "µs": 1e3, "ms": 1e6, "s": 1e9}
_UNIT_ORDER = ["ps", "ns", "µs", "ms", "s"]


def median_ns(leaf_new: str) -> float | None:
    bj = os.path.join(leaf_new, "benchmark.json")
    ej = os.path.join(leaf_new, "estimates.json")
    if not (os.path.isfile(bj) and os.path.isfile(ej)):
        return None
    with open(ej, encoding="utf-8") as f:
        est = json.load(f)
    med = est.get("median", {}).get("point_estimate")
    return float(med) if med is not None else None


def collect(root: str) -> dict[str, float]:
    """Return ``{full_id: median_ns}`` for every crate-own leaf."""
    if not os.path.isdir(root):
        sys.exit(f"artifacts root not found: {root}")
    out: dict[str, float] = {}
    for entry in sorted(os.listdir(root)):
        m = re.match(r"^criterion-full_matrix-(D\d+)$", entry)
        if not m:
            continue
        dtype = m.group(1)
        dtype_root = os.path.join(root, entry)
        for group in sorted(os.listdir(dtype_root)):
            gpath = os.path.join(dtype_root, group)
            if not os.path.isdir(gpath):
                continue
            if group not in ("arith", "strict", "strict_wide"):
                continue  # lib_cmp_* figure dirs are not ours to fill
            for leaf in sorted(os.listdir(gpath)):
                # Only crate-own ids: D<N>_s<scale>_<op|fn>.
                if not re.match(rf"^{dtype}_s\d+_\w+$", leaf):
                    continue
                med = median_ns(os.path.join(gpath, leaf, "new"))
                if med is None:
                    continue
                out[f"{group}/{leaf}"] = med
    return out


def pick_unit(ns: float) -> str:
    if ns < 1.0:
        return "ps"
    if ns < 1e3:
        return "ns"
    if ns < 1e6:
        return "µs"
    if ns < 1e9:
        return "ms"
    return "s"


def fmt(ns: float, unit: str) -> str:
    val = ns / _UNIT_DIV[unit]
    if val < 0.01:
        mantissa, exp = val, 0
        while mantissa < 1.0 and exp > -12:
            mantissa *= 10.0
            exp -= 1
        sup = "⁻" + "".join("⁰¹²³⁴⁵⁶⁷⁸⁹"[int(d)] for d in str(-exp))
        return f"{mantissa:.2f}×10{sup} {unit}"
    # 5 significant figures, trailing-zero trimmed, like the prior tables.
    s = f"{val:.5g}"
    return f"{s} {unit}"


def row_unit(values: list[float]) -> str:
    idx = sorted(_UNIT_ORDER.index(pick_unit(v)) for v in values)
    return _UNIT_ORDER[idx[(len(idx) - 1) // 2]]


def emit_tables(med: dict[str, float]) -> tuple[str, str, list[str]]:
    """Render §1 arith + §3 strict markdown blocks. Returns
    (arith_md, strict_md, missing_ids)."""
    missing: list[str] = []

    # --- §1 Arithmetic ---
    arith_blocks: list[str] = []
    for w in WIDTH_ORDER:
        scales = ARITH_SCALES[w]
        lines = [f"### {w} - {WIDTH_BITS[w]} bits", ""]
        lines.append("| op | " + " | ".join(f"s = {s}" for s in scales) + " |")
        lines.append("|---|" + "---|" * len(scales))
        for op in OPS:
            cells_ns = []
            for s in scales:
                key = f"arith/{w}_s{s}_{op}"
                v = med.get(key)
                if v is None:
                    missing.append(key)
                cells_ns.append(v)
            # §1 tables render each cell in its own natural unit.
            rendered = [fmt(v, pick_unit(v)) if v is not None else "—"
                        for v in cells_ns]
            lines.append(f"| {op} | " + " | ".join(rendered) + " |")
        arith_blocks.append("\n".join(lines))

    # --- §3 Strict transcendentals (mid column per width) ---
    strict_blocks: list[str] = []
    for w in WIDTH_ORDER:
        s = STRICT_MID[w]
        group = "strict" if w in ("D9", "D18", "D38") else "strict_wide"
        cells = {}
        for fn in STRICT_FNS:
            key = f"{group}/{w}_s{s}_{fn}"
            v = med.get(key)
            if v is None:
                missing.append(key)
            cells[fn] = v
        strict_blocks.append((w, s, cells))

    return "\n\n".join(arith_blocks), strict_blocks, missing


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--artifacts", required=True,
                    help="root containing criterion-full_matrix-D*/ dirs")
    ap.add_argument("--dump", action="store_true",
                    help="print every id and median_ns")
    ap.add_argument("--tables", action="store_true",
                    help="print rendered §1 markdown tables")
    args = ap.parse_args()

    med = collect(args.artifacts)
    print(f"collected {len(med)} medians", file=sys.stderr)

    if args.dump:
        for k in sorted(med):
            print(f"{k}\t{med[k]:.6f}")

    arith_md, strict_blocks, missing = emit_tables(med)

    if args.tables:
        print(arith_md)
        print()
        print("=== STRICT (mid column) ===")
        for w, s, cells in strict_blocks:
            print(f"{w} (s={s})")
            for fn in STRICT_FNS:
                v = cells[fn]
                if v is None:
                    print(f"  {fn}: —")
                else:
                    print(f"  {fn}: {fmt(v, pick_unit(v))}")

    if missing:
        print(f"MISSING {len(missing)} ids:", file=sys.stderr)
        for k in missing:
            print(f"  {k}", file=sys.stderr)


if __name__ == "__main__":
    main()
