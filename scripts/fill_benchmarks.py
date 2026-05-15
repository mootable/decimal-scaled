"""Parse criterion stdout from the full_matrix bench runs and
substitute the timings (with per-row winner emboldened) into the
docs/benchmarks.md.draft template.

Run from the crate root:
    python scripts/fill_benchmarks.py
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOGS = [
    ROOT / ".." / ".." / ".." / "tmp" / "full_matrix_arith.log",
    ROOT / ".." / ".." / ".." / "tmp" / "full_matrix_transc.log",
]
# Fallback paths since /tmp on Windows resolves differently.
LOG_CANDIDATES = [
    r"C:\Users\jacko\AppData\Local\Temp\full_matrix_arith.log",
    r"C:\Users\jacko\AppData\Local\Temp\full_matrix_transc2.log",
]
DRAFT = ROOT / "docs" / "benchmarks.md.draft"
OUT = ROOT / "docs" / "benchmarks.md"

# Unit suffix → multiplier into nanoseconds.
UNIT_NS = {
    "ps": 1e-3,
    "ns": 1.0,
    "µs": 1e3,
    "us": 1e3,
    "ms": 1e6,
    "s": 1e9,
}

LINE_RE = re.compile(
    r"^([\w/]+)\s+time:\s+\["
    r"(\S+)\s+(\w+)\s+"
    r"(\S+)\s+(\w+)\s+"
    r"(\S+)\s+(\w+)\]"
)
# Two-line form: `<name>\n  time:  [...]`. Criterion wraps when
# the bench name is wider than the column.
NAME_ONLY_RE = re.compile(r"^([\w/]+)$")
WRAPPED_TIME_RE = re.compile(
    r"^\s*time:\s+\["
    r"(\S+)\s+(\w+)\s+"
    r"(\S+)\s+(\w+)\s+"
    r"(\S+)\s+(\w+)\]"
)


def parse_logs(paths: list[Path]) -> dict[str, float]:
    """Returns bench_name -> median_ns."""
    out: dict[str, float] = {}
    for path in paths:
        if not Path(path).exists():
            continue
        last_name: str | None = None
        for line in Path(path).read_text(encoding="utf-8", errors="replace").splitlines():
            stripped = line.rstrip()
            # Single-line form.
            m = LINE_RE.match(stripped)
            if m:
                name = m.group(1)
                value = float(m.group(4))
                unit = m.group(5)
                if unit in UNIT_NS:
                    out[name] = value * UNIT_NS[unit]
                last_name = None
                continue
            # Wrapped form: name on its own line, then `  time:  [...]`
            name_m = NAME_ONLY_RE.match(stripped)
            if name_m and "/" in name_m.group(1):
                last_name = name_m.group(1)
                continue
            wrap_m = WRAPPED_TIME_RE.match(line)
            if wrap_m and last_name is not None:
                value = float(wrap_m.group(3))
                unit = wrap_m.group(4)
                if unit in UNIT_NS:
                    out[last_name] = value * UNIT_NS[unit]
                last_name = None
    return out


def pick_unit(ns: float) -> str:
    """The most readable unit for a single value."""
    if ns < 1.0:
        return "ps"
    if ns < 1e3:
        return "ns"
    if ns < 1e6:
        return "µs"
    if ns < 1e9:
        return "ms"
    return "s"


_UNIT_DIV = {"ps": 1e-3, "ns": 1.0, "µs": 1e3, "ms": 1e6, "s": 1e9}


def fmt(ns: float, unit: str | None = None) -> str:
    """Format `ns` in the given unit (defaulting to the most readable
    one for that value alone).

    - Scientific notation `m×10ⁿ` for values < 0.01 of the row unit
      (the row's natural unit is much bigger than this cell — keeps
      the order-of-magnitude legible).
    - 1–2 fractional decimals in the comfortable 0.1–100 range.
    - Comma-grouped integer when a row's chosen unit lands much
      smaller than this cell and the value crosses 1 000."""
    if unit is None:
        unit = pick_unit(ns)
    val = ns / _UNIT_DIV[unit]
    if val < 0.01:
        # Scientific notation: e.g. 1.26×10⁻⁵
        mantissa = val
        exp = 0
        while mantissa < 1.0 and exp > -12:
            mantissa *= 10.0
            exp -= 1
        sup_digits = "⁻" + "".join("⁰¹²³⁴⁵⁶⁷⁸⁹"[int(d)] for d in str(-exp))
        s = f"{mantissa:.2f}×10{sup_digits}"
    elif val < 0.1:
        s = f"{val:.3f}"
    elif val < 10:
        s = f"{val:.2f}"
    elif val < 100:
        s = f"{val:.1f}"
    elif val < 1000:
        s = f"{val:.0f}"
    else:
        # Comma-separated integer for readability when a row spans
        # many orders of magnitude and the median unit lands small.
        s = f"{val:,.0f}"
    return f"{s} {unit}"


# Mapping from markdown placeholder → bench name in the criterion log.
# We declare the placeholders that share a row so we can bold the
# row's winner.


def build_placeholder_map() -> dict[str, str]:
    """All placeholders → bench-name in the criterion log."""
    m: dict[str, str] = {}

    # Arithmetic. Per-type table; six ops per type×scale.
    arith_specs = [
        ("D32",   "ARITH_D32",   [(0, "S0"), (5, "S5"), (9, "S9")]),
        ("D64",   "ARITH_D64",   [(0, "S0"), (9, "S9"), (18, "S18")]),
        ("D128",  "ARITH_D128",  [(0, "S0"), (19, "S19"), (38, "S38")]),
        ("D256",  "ARITH_D256",  [(0, "S0"), (35, "S35"), (76, "S76")]),
        ("D512",  "ARITH_D512",  [(0, "S0"), (75, "S75"), (153, "S153")]),
        ("D1024", "ARITH_D1024", [(0, "S0"), (150, "S150"), (307, "S307")]),
    ]
    ops = ["ADD", "SUB", "MUL", "DIV", "REM", "NEG"]
    for type_name, prefix, scales in arith_specs:
        for sc_num, sc_tag in scales:
            for op in ops:
                ph = f"{prefix}_{sc_tag}_{op}"
                bench = f"arith/{type_name}_s{sc_num}/{op.lower()}"
                m[ph] = bench

    # Baselines for arith.
    bd_scales = "s35"
    for op in ops:
        m[f"ARITH_BD_{op}"] = f"arith/bnum_d256_{bd_scales}/{op.lower()}"
        m[f"ARITH_RD_{op}"] = f"arith/rust_decimal/{op.lower()}"
        m[f"ARITH_FX_{op}"] = f"arith/fixed_i64f64/{op.lower()}"

    # Lossy.
    lossy_specs = [
        ("D32",  "D32_s9"),
        ("D64",  "D64_s18"),
        ("D128", "D128_s38"),
    ]
    for type_name, bench_tag in lossy_specs:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"LOSSY_{type_name}_{fn}"] = f"lossy/{bench_tag}/{fn.lower()}"
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        m[f"LOSSY_RD_{fn}"] = f"lossy/rust_decimal/{fn.lower()}"

    # Strict (narrow).
    narrow_strict_specs = [
        ("D32",  "D32_s9",   "D32"),
        ("D64",  "D64_s18",  "D64"),
        ("D128_S0",  "D128_s0",  "D128"),
        ("D128_S19", "D128_s19", "D128"),
        ("D128_S38", "D128_s38", "D128"),
    ]
    for ph_tag, bench_tag, _typ in narrow_strict_specs:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"STRICT_{ph_tag}_{fn}"] = f"strict/{bench_tag}/{fn.lower()}"

    # Strict (wide).
    wide_strict_specs = [
        ("D256_S0",   "D256_s0"),
        ("D256_S35",  "D256_s35"),
        ("D256_S76",  "D256_s76"),
        ("D512_S0",   "D512_s0"),
        ("D512_S75",  "D512_s75"),
        ("D512_S153", "D512_s153"),
        ("D1024_S0",   "D1024_s0"),
        ("D1024_S150", "D1024_s150"),
        ("D1024_S307", "D1024_s307"),
    ]
    for ph_tag, bench_tag in wide_strict_specs:
        for fn in ["LN", "EXP", "SIN", "SQRT"]:
            m[f"STRICT_{ph_tag}_{fn}"] = f"strict_wide/{bench_tag}/{fn.lower()}"

    return m


# Rows declared by their placeholder names (the markdown rows). Used
# to compute the winner across each row.
ROWS: list[list[str]] = []

# Arithmetic rows: one per (type, op). Columns = scales (+ baselines
# for the D128 / D256 tables).
def add_arith_rows() -> None:
    ops = ["ADD", "SUB", "MUL", "DIV", "REM", "NEG"]

    # D32: three scales.
    for op in ops:
        ROWS.append([f"ARITH_D32_S0_{op}", f"ARITH_D32_S5_{op}", f"ARITH_D32_S9_{op}"])
    # D64.
    for op in ops:
        ROWS.append([f"ARITH_D64_S0_{op}", f"ARITH_D64_S9_{op}", f"ARITH_D64_S18_{op}"])
    # D128 + rust_decimal + fixed.
    for op in ops:
        ROWS.append([
            f"ARITH_D128_S0_{op}",
            f"ARITH_D128_S19_{op}",
            f"ARITH_D128_S38_{op}",
            f"ARITH_RD_{op}",
            f"ARITH_FX_{op}",
        ])
    # D256 + bnum_d256.
    for op in ops:
        ROWS.append([
            f"ARITH_D256_S0_{op}",
            f"ARITH_D256_S35_{op}",
            f"ARITH_D256_S76_{op}",
            f"ARITH_BD_{op}",
        ])
    # D512.
    for op in ops:
        ROWS.append([f"ARITH_D512_S0_{op}", f"ARITH_D512_S75_{op}", f"ARITH_D512_S153_{op}"])
    # D1024.
    for op in ops:
        ROWS.append([f"ARITH_D1024_S0_{op}", f"ARITH_D1024_S150_{op}", f"ARITH_D1024_S307_{op}"])


def add_lossy_rows() -> None:
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        ROWS.append([f"LOSSY_D32_{fn}", f"LOSSY_D64_{fn}", f"LOSSY_D128_{fn}", f"LOSSY_RD_{fn}"])


def add_strict_rows() -> None:
    # Narrow strict.
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        ROWS.append([
            f"STRICT_D32_{fn}",
            f"STRICT_D64_{fn}",
            f"STRICT_D128_S0_{fn}",
            f"STRICT_D128_S19_{fn}",
            f"STRICT_D128_S38_{fn}",
        ])
    # Wide strict.
    for fn in ["LN", "EXP", "SIN", "SQRT"]:
        ROWS.append([
            f"STRICT_D256_S0_{fn}",
            f"STRICT_D256_S35_{fn}",
            f"STRICT_D256_S76_{fn}",
            f"STRICT_D512_S0_{fn}",
            f"STRICT_D512_S75_{fn}",
            f"STRICT_D512_S153_{fn}",
            f"STRICT_D1024_S0_{fn}",
            f"STRICT_D1024_S150_{fn}",
            f"STRICT_D1024_S307_{fn}",
        ])


def main() -> None:
    add_arith_rows()
    add_lossy_rows()
    add_strict_rows()

    timings = parse_logs([Path(p) for p in LOG_CANDIDATES])
    print(f"loaded {len(timings)} bench measurements")

    placeholder_map = build_placeholder_map()

    # Per-placeholder: which unit to render in (the row's MEDIAN
    # natural unit — strikes a balance between the winner's tiny
    # numbers and the slowest cell's giant ones) and whether to bold
    # (the row's winner — the minimum time, regardless of unit).
    _UNIT_ORDER = ["ps", "ns", "µs", "ms", "s"]
    unit_for: dict[str, str] = {}
    bold_set: set[str] = set()
    for row in ROWS:
        cells = []
        for ph in row:
            bench = placeholder_map.get(ph)
            if bench is None:
                continue
            t = timings.get(bench)
            if t is None:
                continue
            cells.append((ph, t))
        if not cells:
            continue
        winner_ph, _ = min(cells, key=lambda c: c[1])
        bold_set.add(winner_ph)
        # Median of each cell's natural unit, using the ordering
        # ps < ns < µs < ms < s. Even-count rows pick the lower
        # midpoint (favouring readability over precision in the
        # slowest cell).
        unit_indices = sorted(_UNIT_ORDER.index(pick_unit(t)) for _, t in cells)
        median_idx = unit_indices[(len(unit_indices) - 1) // 2]
        row_unit = _UNIT_ORDER[median_idx]
        for ph, _ in cells:
            unit_for[ph] = row_unit

    # Format every placeholder.
    text = DRAFT.read_text(encoding="utf-8")

    missing: list[str] = []
    for ph, bench in placeholder_map.items():
        t = timings.get(bench)
        if t is None:
            missing.append(f"{ph} -> {bench}")
            substitute = "—"
        else:
            unit = unit_for.get(ph)
            s = fmt(t, unit)
            substitute = f"**{s}**" if ph in bold_set else s
        text = text.replace(f"`__{ph}__`", substitute)

    if missing:
        print("MISSING:")
        for m in missing:
            print(f"  {m}")

    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT}")


if __name__ == "__main__":
    main()
