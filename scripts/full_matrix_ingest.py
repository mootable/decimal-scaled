#!/usr/bin/env python3
"""Walk a tree of Criterion ``full_matrix`` artifacts (per-width
``criterion-full_matrix-D{N}`` JSON output) and emit / fill the §1
arithmetic, §2 fast-transcendental and §3 strict-transcendental
timing tables of ``docs/benchmarks.md``.

The ``bench-full`` workflow uploads, per storage width, the contents
of ``target/criterion/`` as the artifact ``criterion-full_matrix-D<N>``.
After downloading every per-width artifact into one directory the
tree has the shape::

    <artifacts-root>/
      criterion-full_matrix-D<N>/
        <group>/                 (arith | lossy | strict | strict_wide)
          <tag>/                 (D<N>_s<scale>  |  rust_decimal_s19  | ...)
            <op>/                (add | sub | ... | ln | exp | ...)
              new/
                benchmark.json   <- canonical id "<group>/<tag>/<op>"
                estimates.json   <- median.point_estimate is in ns

Criterion turns the ``/`` in a bench function id into nested
directories, so every measured leaf is ``<group>/<tag>/<op>/new``.
The collected median map is keyed by that canonical id
(``arith/D76_s35/mul``) and merged across every per-width artifact —
the cross-crate baselines live in whichever width's binary declares
them (``fixed_i64f64`` in D18, ``rust_decimal_s19`` in D38,
``bnum_d76_s35`` in D76), so the merge gathers them all.

The ``lib_cmp_*`` figure ids belong to the §5 figures pipeline
(``scripts/lib_cmp_ingest.py``) and are simply never referenced by
the placeholder map below, so they are ignored on fill.

Rendering matches the conventions documented at the top of
``benchmarks.md``:

- Per row, every cell renders in one unit — the row's *median*
  natural unit under the ordering ps < ns < µs < ms < s — so the
  cells compare directly.
- The row's winner (minimum median, regardless of unit) is bold.
- Cells whose natural unit is far below the row unit render in
  scientific notation (``1.26×10⁻⁵ ms``).

It never fabricates a value — a missing leaf renders as ``—`` and is
reported on stderr, never guessed.

One-command release refresh (replaces the old stdout-log parse):

    python scripts/full_matrix_ingest.py --artifacts <root> --fill

That reads ``docs/benchmarks.md.draft`` and writes ``docs/benchmarks.md``
§1–§3 in place. Read-only inspection modes are still available:

    python scripts/full_matrix_ingest.py --artifacts <root> --dump
    python scripts/full_matrix_ingest.py --artifacts <root> --tables
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DRAFT = ROOT / "docs" / "benchmarks.md.draft"
OUT = ROOT / "docs" / "benchmarks.md"

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
    """Return ``{canonical_id: median_ns}`` for every measured leaf.

    The canonical id is the Criterion bench-function id
    (``<group>/<tag>/<op>``), reconstructed from the on-disk nested
    directory layout. Medians are merged across every per-width
    ``criterion-full_matrix-D<N>`` artifact found under ``root``.
    """
    if not os.path.isdir(root):
        sys.exit(f"artifacts root not found: {root}")
    out: dict[str, float] = {}
    n_artifacts = 0
    for entry in sorted(os.listdir(root)):
        if not re.match(r"^criterion-full_matrix-D\d+$", entry):
            continue
        n_artifacts += 1
        crit_root = os.path.join(root, entry)
        for group in sorted(os.listdir(crit_root)):
            gpath = os.path.join(crit_root, group)
            if not os.path.isdir(gpath):
                continue
            # Criterion nests <group>/<tag>/<op>/new. Walk to every
            # directory that owns a `new/estimates.json` leaf.
            for tag in sorted(os.listdir(gpath)):
                tpath = os.path.join(gpath, tag)
                if not os.path.isdir(tpath):
                    continue
                for op in sorted(os.listdir(tpath)):
                    leaf_new = os.path.join(tpath, op, "new")
                    med = median_ns(leaf_new)
                    if med is None:
                        continue
                    out[f"{group}/{tag}/{op}"] = med
    if n_artifacts == 0:
        sys.exit(
            f"no criterion-full_matrix-D* artifact dirs under {root}")
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


def fmt(ns: float, unit: str | None = None) -> str:
    """Format ``ns`` in ``unit`` (defaulting to its own natural unit).

    - Scientific notation ``m×10ⁿ`` for values < 0.01 of the row unit
      (keeps the order-of-magnitude legible when a fast cell sits in
      a slow row).
    - 1–2 fractional decimals in the comfortable 0.1–100 range.
    - Comma-grouped integer when the row unit lands far below this
      cell and the value crosses 1 000.
    """
    if unit is None:
        unit = pick_unit(ns)
    val = ns / _UNIT_DIV[unit]
    if val < 0.01:
        mantissa, exp = val, 0
        while mantissa < 1.0 and exp > -12:
            mantissa *= 10.0
            exp -= 1
        sup = "⁻" + "".join("⁰¹²³⁴⁵⁶⁷⁸⁹"[int(d)] for d in str(-exp))
        return f"{mantissa:.2f}×10{sup} {unit}"
    if val < 0.1:
        s = f"{val:.3f}"
    elif val < 10:
        s = f"{val:.2f}"
    elif val < 100:
        s = f"{val:.1f}"
    elif val < 1000:
        s = f"{val:.0f}"
    else:
        s = f"{val:,.0f}"
    return f"{s} {unit}"


def row_unit(values: list[float]) -> str:
    """The row's median natural unit (lower midpoint on even counts)."""
    idx = sorted(_UNIT_ORDER.index(pick_unit(v)) for v in values)
    return _UNIT_ORDER[idx[(len(idx) - 1) // 2]]


# ---------------------------------------------------------------------------
# §1–§3 placeholder map. Keys are the ``__NAME__`` placeholders that
# appear in docs/benchmarks.md.draft; values are the canonical Criterion
# ids produced by collect().
# ---------------------------------------------------------------------------

def build_placeholder_map() -> dict[str, str]:
    m: dict[str, str] = {}

    # --- §1 Arithmetic: per-width tables, six ops × three scales. ---
    # (placeholder family, criterion type tag, [(scale, scale-tag)])
    arith_specs = [
        ("D9",   "D9",   [(0, "S0"), (5, "S5"), (9, "S9")]),
        ("D18",  "D18",  [(0, "S0"), (9, "S9"), (18, "S18")]),
        ("D38",  "D38",  [(0, "S0"), (19, "S19"), (38, "S38")]),
        ("D76",  "D76",  [(0, "S0"), (35, "S35"), (76, "S76")]),
        ("D153", "D153", [(0, "S0"), (75, "S75"), (153, "S153")]),
        ("D307", "D307", [(0, "S0"), (150, "S150"), (307, "S307")]),
    ]
    ops = ["ADD", "SUB", "MUL", "DIV", "REM", "NEG"]
    for fam, dtype, scales in arith_specs:
        for sc_num, sc_tag in scales:
            for op in ops:
                ph = f"ARITH_{fam}_{sc_tag}_{op}"
                m[ph] = f"arith/{dtype}_s{sc_num}/{op.lower()}"

    # §1 cross-crate arithmetic baselines (each lives in one width's
    # artifact: fixed in D18, rust_decimal in D38, bnum in D76).
    for op in ops:
        m[f"ARITH_BD_{op}"] = f"arith/bnum_d76_s35/{op.lower()}"
        m[f"ARITH_RD_{op}"] = f"arith/rust_decimal_s19/{op.lower()}"
        m[f"ARITH_FX_{op}"] = f"arith/fixed_i64f64/{op.lower()}"

    # --- §2 Fast transcendentals (f64-bridge), largest-scale column. ---
    lossy_specs = [("D9", "D9_s9"), ("D18", "D18_s18"), ("D38", "D38_s38")]
    for fam, tag in lossy_specs:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"LOSSY_{fam}_{fn}"] = f"lossy/{tag}/{fn.lower()}"
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        m[f"LOSSY_RD_{fn}"] = f"lossy/rust_decimal/{fn.lower()}"

    # --- §3 Strict transcendentals — narrow tiers. ---
    # D9 / D18 columns are the max-scale series cost; D38 spans 3 scales.
    narrow_strict = [
        ("D9",      "D9_s9"),
        ("D18",     "D18_s18"),
        ("D38_S0",  "D38_s0"),
        ("D38_S19", "D38_s19"),
        ("D38_S38", "D38_s38"),
    ]
    for ph_tag, tag in narrow_strict:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"STRICT_{ph_tag}_{fn}"] = f"strict/{tag}/{fn.lower()}"

    # --- §3 Strict transcendentals — wide tiers. ---
    wide_strict = [
        ("D76_S0",    "D76_s0"),
        ("D76_S35",   "D76_s35"),
        ("D76_S76",   "D76_s76"),
        ("D153_S0",   "D153_s0"),
        ("D153_S75",  "D153_s75"),
        ("D153_S153", "D153_s153"),
        ("D307_S0",   "D307_s0"),
        ("D307_S150", "D307_s150"),
        ("D307_S307", "D307_s307"),
    ]
    for ph_tag, tag in wide_strict:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"STRICT_{ph_tag}_{fn}"] = f"strict_wide/{tag}/{fn.lower()}"

    return m


def build_rows() -> list[list[str]]:
    """Placeholder rows used to pick the per-row unit and winner —
    one inner list per markdown row, in the §1–§3 reading order."""
    rows: list[list[str]] = []
    ops = ["ADD", "SUB", "MUL", "DIV", "REM", "NEG"]

    # §1 arith rows.
    for op in ops:
        rows.append([f"ARITH_D9_S0_{op}", f"ARITH_D9_S5_{op}", f"ARITH_D9_S9_{op}"])
    for op in ops:
        rows.append([f"ARITH_D18_S0_{op}", f"ARITH_D18_S9_{op}", f"ARITH_D18_S18_{op}"])
    for op in ops:
        rows.append([
            f"ARITH_D38_S0_{op}", f"ARITH_D38_S19_{op}", f"ARITH_D38_S38_{op}",
            f"ARITH_RD_{op}", f"ARITH_FX_{op}",
        ])
    for op in ops:
        rows.append([
            f"ARITH_D76_S0_{op}", f"ARITH_D76_S35_{op}", f"ARITH_D76_S76_{op}",
            f"ARITH_BD_{op}",
        ])
    for op in ops:
        rows.append([f"ARITH_D153_S0_{op}", f"ARITH_D153_S75_{op}", f"ARITH_D153_S153_{op}"])
    for op in ops:
        rows.append([f"ARITH_D307_S0_{op}", f"ARITH_D307_S150_{op}", f"ARITH_D307_S307_{op}"])

    # §2 fast-transcendental rows.
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        rows.append([f"LOSSY_D9_{fn}", f"LOSSY_D18_{fn}", f"LOSSY_D38_{fn}", f"LOSSY_RD_{fn}"])

    # §3 strict rows: narrow.
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        rows.append([
            f"STRICT_D9_{fn}", f"STRICT_D18_{fn}",
            f"STRICT_D38_S0_{fn}", f"STRICT_D38_S19_{fn}", f"STRICT_D38_S38_{fn}",
        ])
    # §3 strict rows: wide.
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        rows.append([
            f"STRICT_D76_S0_{fn}", f"STRICT_D76_S35_{fn}", f"STRICT_D76_S76_{fn}",
            f"STRICT_D153_S0_{fn}", f"STRICT_D153_S75_{fn}", f"STRICT_D153_S153_{fn}",
            f"STRICT_D307_S0_{fn}", f"STRICT_D307_S150_{fn}", f"STRICT_D307_S307_{fn}",
        ])
    return rows


def fill_draft(med: dict[str, float]) -> tuple[str, list[str]]:
    """Fill the §1–§3 placeholders in the draft. Returns the filled
    markdown text and the list of placeholders with no measurement."""
    ph_map = build_placeholder_map()
    rows = build_rows()

    # Per row: choose the row's median natural unit and bold the winner.
    unit_for: dict[str, str] = {}
    bold: set[str] = set()
    for row in rows:
        cells = [(ph, med[ph_map[ph]]) for ph in row
                 if ph in ph_map and ph_map[ph] in med]
        if not cells:
            continue
        winner_ph = min(cells, key=lambda c: c[1])[0]
        bold.add(winner_ph)
        unit = row_unit([t for _, t in cells])
        for ph, _ in cells:
            unit_for[ph] = unit

    text = DRAFT.read_text(encoding="utf-8")
    missing: list[str] = []
    for ph, bench in ph_map.items():
        t = med.get(bench)
        if t is None:
            missing.append(f"{ph} -> {bench}")
            sub = "—"
        else:
            s = fmt(t, unit_for.get(ph))
            sub = f"**{s}**" if ph in bold else s
        text = text.replace(f"`__{ph}__`", sub)
    return text, missing


def emit_tables(med: dict[str, float]) -> tuple[str, list, list[str]]:
    """Render compact §1 arith + §3 strict (mid column) blocks for the
    ``--tables`` inspection mode. Not used by ``--fill``."""
    missing: list[str] = []
    arith_blocks: list[str] = []
    for w in WIDTH_ORDER:
        scales = ARITH_SCALES[w]
        lines = [f"### {w} - {WIDTH_BITS[w]} bits", ""]
        lines.append("| op | " + " | ".join(f"s = {s}" for s in scales) + " |")
        lines.append("|---|" + "---|" * len(scales))
        for op in OPS:
            cells = []
            for s in scales:
                key = f"arith/{w}_s{s}/{op}"
                v = med.get(key)
                if v is None:
                    missing.append(key)
                cells.append(v)
            rendered = [fmt(v, pick_unit(v)) if v is not None else "—"
                        for v in cells]
            lines.append(f"| {op} | " + " | ".join(rendered) + " |")
        arith_blocks.append("\n".join(lines))

    strict_blocks: list = []
    for w in WIDTH_ORDER:
        s = STRICT_MID[w]
        group = "strict" if w in ("D9", "D18", "D38") else "strict_wide"
        cells = {}
        for fn in STRICT_FNS:
            key = f"{group}/{w}_s{s}/{fn}"
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
                    help="print every canonical id and median_ns")
    ap.add_argument("--tables", action="store_true",
                    help="print compact §1/§3 markdown for inspection")
    ap.add_argument("--fill", action="store_true",
                    help="fill docs/benchmarks.md.draft -> docs/benchmarks.md (§1–§3)")
    args = ap.parse_args()

    med = collect(args.artifacts)
    print(f"collected {len(med)} medians", file=sys.stderr)

    if args.dump:
        for k in sorted(med):
            print(f"{k}\t{med[k]:.6f}")

    if args.tables:
        arith_md, strict_blocks, missing = emit_tables(med)
        print(arith_md)
        print()
        print("=== STRICT (mid column) ===")
        for w, s, cells in strict_blocks:
            print(f"{w} (s={s})")
            for fn in STRICT_FNS:
                v = cells[fn]
                print(f"  {fn}: " + (fmt(v, pick_unit(v)) if v is not None else "—"))
        if missing:
            print(f"MISSING {len(missing)} ids:", file=sys.stderr)
            for k in missing:
                print(f"  {k}", file=sys.stderr)

    if args.fill:
        text, missing = fill_draft(med)
        OUT.write_text(text, encoding="utf-8")
        print(f"wrote {OUT}", file=sys.stderr)
        if missing:
            print(f"MISSING {len(missing)} placeholders (rendered as —):",
                  file=sys.stderr)
            for k in missing:
                print(f"  {k}", file=sys.stderr)


if __name__ == "__main__":
    main()
