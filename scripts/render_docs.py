#!/usr/bin/env python3
"""Single-source-of-truth renderer for the repeated facts in the docs.

Repeated facts — the crate version, the storage-width tier family, the
`MAX_SCALE = N - 1` rule, and the bench result surfaces (precision,
performance, history, comparisons) — live in exactly ONE place each:

  * the crate version    -> `Cargo.toml` `[package] version`
  * the tier family      -> `docs/_data/tiers.json`
  * the bench surfaces   -> the `results/**/*.tsv` files each bench job
    self-commits (golden, timing, history, lib_cmp)

This script is the ONE entry point: a single invocation fills every
generated region in `README.md` and the files under `docs/` from those
sources. Each region is bounded by a pair of HTML comments::

    <!-- BEGIN GENERATED:<key> -->
    ...generated body (do not edit by hand)...
    <!-- END GENERATED:<key> -->

The text between the markers is replaced wholesale by the renderer for
`<key>`; the markers themselves are preserved. Editing inside a region
by hand is pointless — the next render (and the `docs-drift` CI gate)
overwrites it.

Usage::

    python scripts/render_docs.py            # rewrite the docs in place
    python scripts/render_docs.py --check     # exit 1 if any doc is stale

`--check` renders into memory and compares against what is committed; it
does not touch the working tree, so it is safe to run in CI. The
`docs-drift` workflow runs the plain (in-place) form and then
`git diff --exit-code`, which is equivalent and also surfaces the diff.

ADDING A NEW SINGLE-SOURCED FACT
--------------------------------
1. Put the source datum in `docs/_data/*.json` (or read it from
   `Cargo.toml`, like the version, or a committed `results/**/*.tsv`).
2. Add a `render_<key>()` builder below that returns the region body
   (no trailing newline, no marker lines).
3. Register it in `REGIONS` with the file it lives in.
4. Wrap the target text in the docs with the matching
   `<!-- BEGIN GENERATED:<key> -->` / `<!-- END GENERATED:<key> -->`
   markers.
5. Run `python scripts/render_docs.py`; the drift gate then guards it.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA_DIR = ROOT / "docs" / "_data"
GOLDEN_DIR = ROOT / "decimal-scaled-golden" / "golden"
GOLDEN_RESULTS = ROOT / "results" / "golden" / "summary.tsv"
CELLS_SRC = ROOT / "decimal-scaled-cells" / "src" / "lib.rs"
ROUNDING_SRC = ROOT / "decimal-scaled-golden" / "src" / "support" / "rounding.rs"
TIMING_RESULTS = ROOT / "results" / "timing" / "bbc_medians.tsv"
HISTORY_RESULTS = ROOT / "results" / "history" / "history.tsv"
LIBCMP_RESULTS = ROOT / "results" / "lib_cmp" / "medians.tsv"

BEGIN = "<!-- BEGIN GENERATED:{key} -->"
END = "<!-- END GENERATED:{key} -->"


# --- Data sources ---------------------------------------------------------


def crate_version() -> str:
    """The `[package] version` from Cargo.toml (e.g. `0.5.0`)."""
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    # Match the first `version = "..."` after the [package] header so a
    # dependency's version key can never be picked up by mistake.
    pkg = re.search(r"\[package\](.*?)(?:\n\[|\Z)", text, re.S)
    section = pkg.group(1) if pkg else text
    m = re.search(r'(?m)^\s*version\s*=\s*"([^"]+)"', section)
    if not m:
        raise SystemExit("render_docs: could not find [package] version in Cargo.toml")
    return m.group(1)


def version_minor(version: str) -> str:
    """The `major.minor` install pin (e.g. `0.5.0` -> `0.5`)."""
    parts = version.split(".")
    return ".".join(parts[:2]) if len(parts) >= 2 else version


def load_tiers() -> list[dict]:
    data = json.loads((DATA_DIR / "tiers.json").read_text(encoding="utf-8"))
    return data["tiers"]


# Word forms for the small counts we cite in prose.
_NUMBER_WORDS = {
    1: "one", 2: "two", 3: "three", 4: "four", 5: "five", 6: "six",
    7: "seven", 8: "eight", 9: "nine", 10: "ten", 11: "eleven",
    12: "twelve", 13: "thirteen", 14: "fourteen", 15: "fifteen",
}


def number_word(n: int) -> str:
    return _NUMBER_WORDS.get(n, str(n))


# --- Region builders ------------------------------------------------------
#
# Each returns the region BODY only (the lines between the markers), with
# no leading/trailing blank line and no marker lines.


def render_widths_table() -> str:
    """The full tier table (one row per tiers.json entry) for docs/widths.md.

    `MAX_SCALE` is derived as `digits - 1` (the `MAX_SCALE = N - 1` rule)
    rather than stored, so the column can never drift from the digit
    count in the type name.
    """
    rows = [
        "| Type | Constructor macro | Underlying signed integer | `MAX_SCALE` | Max value at SCALE 0 | Required feature |",
        "|---|---|---|---|---|---|",
    ]
    for t in load_tiers():
        max_scale = t["digits"] - 1
        rows.append(
            f"| `{t['name']}<S>` "
            f"| `{t['name'].lower()}!` "
            f"| `{t['int']}` ({t['bits']} bits) "
            f"| {max_scale} "
            f"| {t['max_at_s0']} "
            f"| {t['feature']} |"
        )
    return "\n".join(rows)


def render_install_dependency() -> str:
    """The README install snippet, version pinned to the current minor."""
    pin = version_minor(crate_version())
    return (
        "```toml\n"
        "[dependencies]\n"
        f'decimal-scaled = {{ version = "{pin}", features = ["macros"] }}\n'
        "```"
    )


def render_width_count_word() -> str:
    """The spelled-out tier count, e.g. `thirteen storage widths`,
    inlined into the docs/widths.md lede so it tracks tiers.json."""
    tiers = load_tiers()
    return f"{number_word(len(tiers))} storage widths"


def golden_counts() -> tuple[int, int]:
    """`(total golden values, number of functions)` read straight from the
    committed `decimal-scaled-golden/golden/*.au` files: one data line per
    value, one file per function. A data line starts with a digit or a `-`;
    `#` metadata, `//` provenance, and blank lines are skipped — the same
    lines the harness loader treats as cases."""
    files = sorted(GOLDEN_DIR.glob("*.au"))
    total = 0
    for f in files:
        for line in f.read_text(encoding="utf-8").splitlines():
            s = line.lstrip()
            if s and (s[0].isdigit() or s[0] == "-"):
                total += 1
    return total, len(files)


def render_golden_counts() -> str:
    """Inline count for docs/golden.md, e.g. `101,809 answers across 28
    functions`, counted from the golden files so it tracks regeneration."""
    total, funcs = golden_counts()
    return f"{total:,} answers across {funcs} functions"


# --- Precision page (docs/precision.md) — generated from results/golden/ -----
#
# The golden-comprehensive CI run self-commits results/golden/summary.tsv: the
# per-input surface AGGREGATED to one row per (function, width, scale, mode):
#   function  width  scale  mode  passed  failed  na
# where each per-input outcome falls in exactly one bucket — `passed` (correctly
# rounded), `failed` (a real correctness failure: mis-rounded / wrong-mode /
# error / timeout / panic), or `na` (not a check: an out-of-tier "skipped" or an
# out-of-domain "non-real" input). The raw per-input rows (~56M, ~4.7 GB) are the
# run's uploaded artifact, never git — this aggregate drives every table.

_PENDING = "_Pending the first golden-comprehensive CI run — this renders from `results/golden/summary.tsv`._"
_GOLDEN_HEADER = ["function", "width", "scale", "mode", "passed", "failed", "na"]


def _golden_rows() -> list[tuple[str, int, int, str, int, int, int]] | None:
    """`(function, width, scale, mode, passed, failed, na)` per cell, or None if
    results/golden/summary.tsv is absent or not yet on the current schema (an
    older/sample file renders the surface as pending, never as garbage)."""
    if not GOLDEN_RESULTS.exists():
        return None
    lines = GOLDEN_RESULTS.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].split("\t")[:7] != _GOLDEN_HEADER:
        return None
    rows = []
    for line in lines[1:]:
        c = line.split("\t")
        if len(c) >= 7 and c[1].isdigit() and c[2].isdigit():
            rows.append((c[0], int(c[1]), int(c[2]), c[3], int(c[4]), int(c[5]), int(c[6])))
    return rows


def golden_surface_cells() -> int:
    """The number of `(width, scale)` combinations — counted from the `cells!`
    macro invocation, the single source of the band-edge surface."""
    text = CELLS_SRC.read_text(encoding="utf-8")
    m = re.search(r"cells!\s*\{(.*?)\n\}", text, re.S)
    body = m.group(1) if m else ""
    return sum(
        len([x for x in braces.split(",") if x.strip()])
        for braces in re.findall(r"=>\s*\d+\s*\{([^}]*)\}", body)
    )


def rounding_mode_count() -> int:
    """The number of rounding modes — the `RoundingMode` enum's variant count."""
    text = ROUNDING_SRC.read_text(encoding="utf-8")
    m = re.search(r"enum RoundingMode\s*\{(.*?)\}", text, re.S)
    body = m.group(1) if m else ""
    return len(re.findall(r"^\s*[A-Z]\w+\s*,", body, re.M))


def render_precision_stats() -> str:
    """The Precision-page headline stats line.

    The input / function / cell / mode counts come from the test DEFINITION (the
    golden files, the `cells!` surface, the `RoundingMode` enum). The TOTAL comes
    from the committed run, because the product of those four is not what the
    gate does: an input carrying more significant digits than a tier can hold is
    filtered by that subject's `limits()` and never executes at that cell.
    Multiplying them out claimed 96,159,960 against 51,510,016 actually graded.
    State the four inputs to the surface, then the measured result."""
    cases, funcs = golden_counts()
    cells = golden_surface_cells()
    modes = rounding_mode_count()
    rows = _golden_rows()
    if not rows:
        return _PENDING
    total = sum(r[4] + r[5] for r in rows)
    return (
        f"We execute {cases:,} specialised inputs across all {funcs} functions, on "
        f"{cells} widths and scales, under all {modes} rounding modes. Not every "
        f"input is representable at every width and scale — one needing more "
        f"significant digits than a tier holds is filtered before it runs — so "
        f"the graded total is {total:,} separate checks."
    )


def render_home_tested() -> str:
    """The number of value tests actually GRADED, summed from the committed
    `results/golden/summary.tsv` — the run, not a multiplication.

    This used to return `inputs × cells × modes`. That product was never
    reconciled against a run and overstated the gate by ~1.86x in every release
    measured: 96,159,960 claimed against 51,510,016 graded, and 71,469,918
    against 38,405,970 the release before.

    The gap is not a defect in the gate — it is work the harness deliberately
    does not do. A golden input carrying more significant digits than a tier can
    hold is filtered by that subject's `limits()` and never executes at that
    cell, so the product counts combinations that by construction cannot be
    tested. `sqrt` and `cbrt` lose ~40% of their grid that way, because their
    adversarial inputs are the longest in the set.

    Passed + failed is what "value tests" can honestly mean: a verdict was
    produced. Anything else in the summary either never ran or produced no
    rounding verdict to grade.
    """
    rows = _golden_rows()
    if not rows:
        return _PENDING
    return f"{sum(r[4] + r[5] for r in rows):,}"


def render_precision_surface() -> str:
    """The correctly-rounded surface: one ROW per function, one COLUMN per
    storage width. Each cell collapses every scale and rounding mode for that
    `(function, width)` to a single verdict with its count beneath: `✓` over the
    number of checks verified correctly-rounded when nothing failed, else `✗`
    over the number of failing checks; `·` where the surface carries no data for
    that pair. (Out-of-domain / out-of-tier inputs are `na`, counted in neither.)"""
    rows = _golden_rows()
    if not rows:
        return _PENDING
    agg: dict[tuple[str, int], list[int]] = {}  # (fn,w) -> [passed, failed]
    for fn, w, _s, _m, p, fl, _na in rows:
        a = agg.setdefault((fn, w), [0, 0])
        a[0] += p
        a[1] += fl
    funcs = sorted({fn for fn, _w in agg})
    widths = sorted({w for _fn, w in agg})
    head = "| Function | " + " | ".join(f"D{w}" for w in widths) + " |"
    # Function column left-aligned; the per-width verdict columns centred so the
    # ✓ / ✗ marks (and the count stacked beneath via <br>) sit under their headers.
    rule = "| :-- | " + " | ".join([":-:"] * len(widths)) + " |"
    out = [head, rule]
    for fn in funcs:
        cells = []
        for w in widths:
            if (fn, w) not in agg:
                cells.append("·")
            else:
                passed, failed = agg[(fn, w)]
                if failed == 0:
                    cells.append(f"✓<br>{passed:,}")
                else:
                    cells.append(f"✗<br>{failed:,}")
        out.append(f"| `{fn}` | " + " | ".join(cells) + " |")
    return "\n".join(out)


# --- Performance page (docs/performance.md) — generated from results/timing/ --
#
# bench-branch-compare self-commits results/timing/bbc_medians.tsv:
#   op  width  scale  prod_ns  branch_ns  delta_ns  delta_pct  ratio
# `branch_ns` is THIS build's median for that (op, width, scale). The page is a
# SECTION: an index (header + units + width map) plus three category sub-pages, each
# one section per op — a width x scale table (each cell in its own natural time unit —
# values span up to ~6 decades, so a single per-table unit is unreadable) beside a
# log-time-vs-width graph (solid lines for scale 0 and the max scale, dashed for the
# intermediate band-edge scales, a light fill between the two solid lines). bbc has
# no per-call distribution, so the Performance graphs carry no min–max band.

_PENDING_PERF = "_Pending the first bench-branch-compare CI run — this renders from `results/timing/bbc_medians.tsv`._"

# Named time units as powers of ten nanoseconds (the page's helper legend).
_TIME_UNITS = [("ns", 0), ("µs", 3), ("ms", 6), ("s", 9)]
_SUP = str.maketrans("0123456789", "⁰¹²³⁴⁵⁶⁷⁸⁹")
_FRACTIONS = {(1, 2): "½", (1, 3): "⅓", (2, 3): "⅔", (1, 4): "¼", (3, 4): "¾"}


_TIMING_HEADER = ["op", "width", "scale", "prod_ns", "branch_ns", "delta_ns", "delta_pct", "ratio"]


def _timing_rows() -> list[tuple[str, int, int, float]] | None:
    """`(op, width, scale, ns)` from results/timing/bbc_medians.tsv (`branch_ns`
    = this build's median), or None if the file isn't committed yet / carries a
    foreign or superseded schema (the header guard mirrors `_golden_rows`).

    PUBLISHABLE rows only — diagnostic rows (`is_diagnostic_op`) are dropped
    here, at the one place the medians enter this file, so every caller is
    covered and a future one cannot reintroduce them by accident."""
    if not TIMING_RESULTS.exists():
        return None
    lines = TIMING_RESULTS.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].split("\t")[:8] != _TIMING_HEADER:
        return None
    rows = []
    for line in lines[1:]:
        c = line.split("\t")  # op width scale prod_ns branch_ns ...
        if len(c) >= 5 and not is_diagnostic_op(c[0]):
            w = c[1].lstrip("D")
            if w.isdigit() and c[2].lstrip("-").isdigit():
                rows.append((c[0], int(w), int(c[2]), float(c[4])))
    return rows


def _unit_of(ns: float) -> tuple[str, int]:
    """The largest named unit whose magnitude is <= `ns` (so the value reads in
    [1, 1000))."""
    for label, power in reversed(_TIME_UNITS):
        if ns >= 10 ** power:
            return label, power
    return _TIME_UNITS[0]


def _fmt_ns(ns: float) -> str:
    """One timing in its own natural unit, three significant figures (e.g.
    `1.87 ns`, `2.84 ms`). A narrow no-break space keeps value+unit together."""
    label, power = _unit_of(ns)
    return f"{ns / 10 ** power:.3g} {label}"


def _ns_decade(d: int) -> str:
    """A power-of-ten-nanoseconds y-axis tick label, e.g. `d=3` -> `1 µs`."""
    for label, power in reversed(_TIME_UNITS):
        if d >= power:
            return f"{10 ** (d - power):g} {label}"
    return f"{10 ** d:g} ns"


def _units_legend(ns_values) -> str:
    """The page's helper table mapping each time unit that actually appears to its
    size in nanoseconds (e.g. `us | 10^3 ns`)."""
    unit_by_power = {power: label for label, power in _TIME_UNITS}
    used = sorted({_unit_of(v)[1] for v in ns_values})
    rows = ["| Unit | In nanoseconds |", "| :-- | --: |"]
    rows += [f"| {unit_by_power[p]} | 10{str(p).translate(_SUP)} ns |" for p in used]
    return "\n".join(rows)


def _width_int_table(widths) -> str:
    """The decimal-tier reference map (from tiers.json), for the widths present:
    `Width | Decimals | Integer | Bits` (e.g. `D18 | 18 | Int<1> | 64`)."""
    tiers = {t["digits"]: t for t in load_tiers()}
    rows = ["| Width | Decimals | Integer | Bits |", "| :-- | --: | :-- | --: |"]
    for w in sorted(widths):
        t = tiers.get(w)
        if t:
            rows.append(f"| {t['name']} | {t['digits']} | `{t['int']}` | {t['bits']} |")
    return "\n".join(rows)


def render_bench_widths() -> str:
    """The decimal-tier -> integer-width reference table for the right column of
    the Performance/History page headers (every tier, from tiers.json)."""
    return _width_int_table([t["digits"] for t in load_tiers()])


def render_slowest_cell() -> str:
    """The slowest measured cell on the whole surface, for the widths-page prose.

    Generated rather than written down because a hand-typed performance figure
    becomes a lie the moment the code gets faster, and nothing catches it. This
    page previously claimed the widest transcendentals "approach a second per
    call"; by the time anyone checked, the true figure was ~5.5 ms — wrong by
    more than two orders of magnitude, in the direction that scares users off a
    tier that works fine. Single-sourcing it from the same medians the
    Performance page renders means the docs-drift gate now fails the PR instead.
    """
    rows = _timing_rows()
    if not rows:
        return _PENDING_PERF
    ns, op, width, scale = max((ns, op, w, s) for op, w, s, ns in rows)
    return (f"at most ~{_fmt_ns(ns)} per call (the slowest cell measured "
            f"anywhere on the surface: `{op}` at `D{width}<{scale}>`)")


def _pos_labels(p: int) -> list[str]:
    """Band-edge column labels for `p` sampled scales: `0`, the fractions, `max`."""
    if p <= 1:
        return ["0"]
    from math import gcd
    out = []
    for i in range(p):
        if i == 0:
            out.append("0")
        elif i == p - 1:
            out.append("max")
        else:
            g = gcd(i, p - 1)
            out.append(_FRACTIONS.get((i // g, (p - 1) // g), f"{i}/{p - 1}"))
    return out


def _perf_series(op_rows) -> tuple[list[int], int, dict[int, list]]:
    """`(widths, P, series)` for one op: widths sorted; `P` = the sampled-scale
    count (data-driven — every scale present is included); `series[width]` = the
    timings by ascending scale, padded with `None` to length `P`."""
    by_w: dict[int, dict[int, float]] = {}
    for _op, w, s, ns in op_rows:
        by_w.setdefault(w, {})[s] = ns
    widths = sorted(by_w)
    P = max((len(v) for v in by_w.values()), default=0)
    series = {}
    for w in widths:
        vals = [by_w[w][s] for s in sorted(by_w[w])]
        series[w] = vals + [None] * (P - len(vals))
    return widths, P, series


# --- Operand families on the chart ------------------------------------------
#
# A family chart carries THREE dimensions, so each gets its own channel and they
# never compete:
#   * width    -> x
#   * scale    -> line STYLE (solid at scale 0 and max, dashed between). Unchanged.
#   * family   -> COLOUR, plus a marker shape on the max-scale line.
#
# The colours are the first three slots of a categorical palette validated for
# BOTH surfaces rather than eyeballed:
#   light (surface #fcfcfb): lightness band PASS, chroma PASS, CVD separation
#     PASS (worst adjacent pair dE 9.2 deutan, above the 8 target), normal-vision
#     PASS (dE 27.6), contrast WARN on the aqua slot at 2.74:1;
#   dark  (surface #1a1a19): all five checks PASS.
# The light-mode contrast WARN obligates relief -- visible labels or a table
# view -- and BOTH are already present: every op section renders the full
# width x scale table beside its chart, and the legend labels each family in
# text. Identity is therefore never colour-alone, which is also why the marker
# shapes are worth their ink in greyscale and print.
#
# Three slots is the maximum any op needs (`log` = log / @hard / @near1 and
# `powf` = powf / @int / @hard are the widest families). Slots are assigned in
# FIXED ORDER by family name, never cycled, so a family keeps its colour when a
# sibling is toggled off -- colour follows the entity, not its rank.
_FAM_SLOTS = 3
# The average is a DERIVED line, not a family, so it deliberately does not take a
# categorical slot: it renders in the page's own ink, so it reads as an annotation
# over the data rather than as another series competing with it.
_FAM_MARKS = ("circle", "square", "triangle")

# Defined ONCE here and consumed by both surfaces -- the docs pages emit it and
# `perf_compare.py` folds the same constant into its standalone <style>, so the
# two cannot drift. mkdocs-material flags dark mode with a data attribute on
# <html>; the standalone page has no such attribute and follows the OS. Both
# selectors are listed, and each is inert on the surface that does not use it.
#
# The toggles are pure CSS -- `:checked` plus a sibling combinator -- so the
# published site needs NO JavaScript and gains no `extra_javascript` entry. The
# rules key on the checkbox's POSITION, not its id, so this block is a fixed
# handful of rules rather than generated per-op CSS; the ids exist only to bind
# each <label> to its input.
PERF_FAMILY_CSS = """
:root{--fam-1:#2a78d6;--fam-2:#eb6834;--fam-3:#1baf7a}
[data-md-color-scheme="slate"]{--fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}
@media (prefers-color-scheme:dark){:root:not([data-md-color-scheme="default"]){
  --fam-1:#3987e5;--fam-2:#d95926;--fam-3:#199e70}}
.perf-chart{position:relative}
.perf-chart>input.fam-toggle{position:absolute;width:1px;height:1px;opacity:0;
  pointer-events:none;margin:0}
.perf-chart>input.fam-toggle:nth-of-type(1):not(:checked)~figure .fam-1,
.perf-chart>input.fam-toggle:nth-of-type(2):not(:checked)~figure .fam-2,
.perf-chart>input.fam-toggle:nth-of-type(3):not(:checked)~figure .fam-3,
.perf-chart>input.fam-toggle:nth-of-type(4):not(:checked)~figure .fam-avg{
  display:none}
.perf-chart .fam-legend{display:flex;flex-wrap:wrap;gap:.35rem .9rem;
  justify-content:center;margin-top:.25rem}
.perf-chart .fam-key{cursor:pointer;user-select:none;font-size:.7rem;
  display:inline-flex;align-items:center;gap:.3rem;opacity:.45;
  border-bottom:1px dotted currentColor}
.perf-chart .fam-key::before{content:"";width:.75rem;height:0;
  border-top:2px solid currentColor}
.perf-chart .fam-key.k1{color:var(--fam-1)}
.perf-chart .fam-key.k2{color:var(--fam-2)}
.perf-chart .fam-key.k3{color:var(--fam-3)}
.perf-chart>input.fam-toggle:nth-of-type(1):checked~figure .fam-key.k1,
.perf-chart>input.fam-toggle:nth-of-type(2):checked~figure .fam-key.k2,
.perf-chart>input.fam-toggle:nth-of-type(3):checked~figure .fam-key.k3,
.perf-chart>input.fam-toggle:nth-of-type(4):checked~figure .fam-key.kavg{
  opacity:1;border-bottom-style:solid}
.perf-chart .fam-key.kavg::before{border-top-style:dashed}
"""


def _perf_families(op_rows) -> tuple[list[str], list[int], int, dict]:
    """`(families, widths, P, series)` for one base op's whole family.

    `families` is the row names in FIXED order — the base op first, then its
    `@variant` siblings alphabetically — so a family's colour never depends on
    which siblings happen to be present in the data. `widths` is the union
    across families; `series[(family, width)]` is that family's timings by
    ascending scale, padded to `P`."""
    by_fam: dict[str, dict[int, dict[int, float]]] = {}
    for op, w, s, ns in op_rows:
        by_fam.setdefault(op, {}).setdefault(w, {})[s] = ns
    families = sorted(by_fam, key=lambda o: (_FAMILY_SEP in o, o))
    widths = sorted({w for f in by_fam.values() for w in f})
    P = max((len(sc) for f in by_fam.values() for sc in f.values()), default=0)
    series: dict[tuple[str, int], list] = {}
    for fam, by_w in by_fam.items():
        for w, sc in by_w.items():
            vals = [sc[s] for s in sorted(sc)]
            series[(fam, w)] = vals + [None] * (P - len(vals))
    return families, widths, P, series


def _perf_svg(widths: list[int], P: int, series: dict[int, list]) -> str:
    """Single-family chart — the original `series[width] -> timings` shape, kept
    so any caller holding one op's rows still works. Delegates to the family
    renderer with one family, which is exactly the old output plus a group
    wrapper."""
    return _perf_svg_families(["_"], widths, P,
                              {("_", w): v for w, v in series.items()})


def _perf_svg_families(families: list[str], widths: list[int], P: int,
                       series: dict, average: bool = False) -> str:
    """A log-time(y) vs width(x) line graph with every FAMILY overlaid on one set
    of axes: per family, one polyline per sampled scale — solid for scale 0 and
    the max scale, dashed for the intermediate ones — coloured by family, with a
    marker shape on the max-scale line as a second, non-colour cue. The
    scale-band fill is drawn for the BASE family only; one fill per family would
    overlap into mud and the band is a property of the op, not of the operand.

    Each family is wrapped in `<g class="fam-N">` and the optional average in
    `<g class="fam-avg">`, which is what the CSS toggles switch. Inline SVG, so
    it tracks the light/dark palette through the custom properties above."""
    import math
    flat = [v for vs in series.values() for v in vs if v is not None]
    if not flat or len(widths) < 2:
        return ""
    lo, hi = math.floor(math.log10(min(flat))), math.ceil(math.log10(max(flat)))
    if hi <= lo:
        hi = lo + 1
    W, H, L, Rm, Tm, Bm = 460, 240, 52, 10, 10, 30
    pw, ph, n = W - L - Rm, H - Tm - Bm, len(widths)

    def xp(i):
        return L + pw * i / (n - 1)

    def yp(ns):
        return Tm + ph * (hi - math.log10(ns)) / (hi - lo)

    def col(k):
        # Slot 0 keeps the brand tone so a single-family chart is unchanged.
        return "var(--md-primary-fg-color)" if k == 0 else f"var(--fam-{k + 1})"

    p = [
        f'<svg viewBox="0 0 {W} {H}" width="100%" style="height:auto;'
        f'color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg">'
    ]
    for d in range(lo, hi + 1):  # y gridlines + decade labels
        y = yp(10 ** d)
        p.append(f'<line x1="{L}" y1="{y:.1f}" x2="{L + pw}" y2="{y:.1f}" '
                 f'stroke="currentColor" stroke-opacity="0.15"/>')
        p.append(f'<text x="{L - 6}" y="{y + 3:.1f}" text-anchor="end" font-size="9" '
                 f'fill="currentColor">{_ns_decade(d)}</text>')
    for i, w in enumerate(widths):  # x (width) labels
        p.append(f'<text x="{xp(i):.1f}" y="{Tm + ph + 12}" text-anchor="middle" '
                 f'font-size="8" fill="currentColor">{w}</text>')

    def marker(shape, x, y, c):
        if shape == "square":
            return (f'<rect x="{x - 2.2:.1f}" y="{y - 2.2:.1f}" width="4.4" '
                    f'height="4.4" fill="{c}"/>')
        if shape == "triangle":
            return (f'<polygon points="{x:.1f},{y - 2.8:.1f} {x + 2.6:.1f},'
                    f'{y + 2.2:.1f} {x - 2.6:.1f},{y + 2.2:.1f}" fill="{c}"/>')
        return f'<circle cx="{x:.1f}" cy="{y:.1f}" r="2.3" fill="{c}"/>'

    for k, fam in enumerate(families):
        c = col(k)
        p.append(f'<g class="fam-{k + 1}">')
        if k == 0:  # scale band, base family only
            s0 = [(xp(i), series[(fam, w)][0]) for i, w in enumerate(widths)
                  if (fam, w) in series and series[(fam, w)][0] is not None]
            sm = [(xp(i), series[(fam, w)][P - 1]) for i, w in enumerate(widths)
                  if (fam, w) in series and series[(fam, w)][P - 1] is not None]
            if len(s0) >= 2 and len(sm) >= 2:
                pts = " ".join(f"{x:.1f},{yp(v):.1f}" for x, v in s0)
                pts += " " + " ".join(f"{x:.1f},{yp(v):.1f}" for x, v in reversed(sm))
                p.append(f'<polygon points="{pts}" fill="{c}" fill-opacity="0.10"/>')
        for j in range(P):
            line = [(xp(i), series[(fam, w)][j]) for i, w in enumerate(widths)
                    if (fam, w) in series and series[(fam, w)][j] is not None]
            if len(line) < 2:
                continue
            pts = " ".join(f"{x:.1f},{yp(v):.1f}" for x, v in line)
            solid = j == 0 or j == P - 1
            dash = "" if solid else ' stroke-dasharray="3 3"'
            p.append(f'<polyline points="{pts}" fill="none" stroke="{c}" '
                     f'stroke-width="{1.6 if solid else 1.0}"{dash}/>')
            if j == P - 1 and len(families) > 1:  # second cue, max-scale line only
                shape = _FAM_MARKS[k % len(_FAM_MARKS)]
                for x, v in line:
                    p.append(marker(shape, x, yp(v), c))
        p.append("</g>")

    if average and len(families) > 1:
        p.append('<g class="fam-avg">')
        for j in range(P):
            line = []
            for i, w in enumerate(widths):
                vs = [series[(f, w)][j] for f in families
                      if (f, w) in series and series[(f, w)][j] is not None]
                if vs:
                    line.append((xp(i), sum(vs) / len(vs)))
            if len(line) >= 2:
                pts = " ".join(f"{x:.1f},{yp(v):.1f}" for x, v in line)
                p.append(f'<polyline points="{pts}" fill="none" stroke="currentColor" '
                         f'stroke-width="1.3" stroke-dasharray="6 3" '
                         f'stroke-opacity="0.85"/>')
        p.append("</g>")

    p.append(f'<line x1="{L}" y1="{Tm}" x2="{L}" y2="{Tm + ph}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append(f'<line x1="{L}" y1="{Tm + ph}" x2="{L + pw}" y2="{Tm + ph}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append("</svg>")
    return "".join(p)


def _slug(text: str) -> str:
    """An id-safe token — `log@near1` -> `log-near1`."""
    return "".join(c if c.isalnum() else "-" for c in text)


def perf_chart_block(base: str, op_rows, uid: str = "") -> str:
    """One op's complete chart: the overlaid family chart plus its toggle
    legend, as a self-contained HTML block.

    BOTH surfaces render through this — the published Performance pages and
    `perf_compare.py`'s standalone page — so the chart cannot diverge between
    them. `uid` distinguishes the two copies `perf_compare` draws per op
    (published beside branch) whose checkbox ids would otherwise collide.

    Every family is drawn and every family toggle starts ON: the raw data is the
    default view. The average is one MORE toggle, off by default, so it sits
    alongside the families rather than standing in for them."""
    families, widths, P, series = _perf_families(op_rows)
    svg = _perf_svg_families(families, widths, P, series, average=len(families) > 1)
    if not svg:
        return ""
    if len(families) == 1:  # no family axis — no legend to draw
        return (f'<figure>{svg}<figcaption>Median time vs width (log scale). '
                f'Solid: scale 0 and max; dashed: the intermediate band-edge '
                f'scales.</figcaption></figure>')
    pre = f"f-{_slug(uid + base)}"
    keys = [f'<input type="checkbox" class="fam-toggle" id="{pre}-{k}" checked>'
            for k in range(len(families))]
    keys.append(f'<input type="checkbox" class="fam-toggle" id="{pre}-avg">')
    labels = [f'<label class="fam-key k{k + 1}" for="{pre}-{k}">{f}</label>'
              for k, f in enumerate(families)]
    labels.append(f'<label class="fam-key kavg" for="{pre}-avg">average</label>')
    return (
        '<div class="perf-chart">'
        + "".join(keys)
        + "<figure>"
        + svg
        + '<figcaption>Median time vs width (log scale). Colour = operand family '
          '(click a legend entry to show or hide it); solid = scale 0 and max, '
          'dashed = the intermediate band-edge scales. The shaded band is the '
          'base row\'s scale spread.</figcaption>'
        + '<div class="fam-legend">' + "".join(labels) + "</div>"
        + "</figure></div>"
    )


def render_performance_units() -> str:
    """The time-unit legend for the Performance page header (left column)."""
    rows = _timing_rows()
    return _units_legend([r[3] for r in rows]) if rows else _PENDING_PERF


# --- Op classification -----------------------------------------------------
#
# Each bench surface (Performance / History / Comparisons) is a SECTION whose
# three category sub-pages each render only their own ops. `op_category` routes a
# function name to its category; the labels name the pages and the section
# headings; the order fixes the page order. The member sets are by function name
# as it appears in the bench data.
_CATEGORY_ORDER = ("arithmetic", "roots-and-exponents", "trigonometry")
_CATEGORY_LABELS = {
    "arithmetic": "Arithmetic",
    "roots-and-exponents": "Roots and Exponents",
    "trigonometry": "Trigonometry",
}
_CATEGORY_OPS = {
    "arithmetic": ("add", "sub", "mul", "div", "rem", "neg"),
    "roots-and-exponents": (
        "sqrt", "cbrt", "exp", "exp2", "expm1", "ln", "log", "log1p", "log2", "log10",
        "powf", "hypot",
    ),
    "trigonometry": (
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
        "to_degrees", "to_radians",
    ),
}
_OP_CATEGORY = {op: cat for cat, ops in _CATEGORY_OPS.items() for op in ops}
_warned_ops: set[str] = set()

# --- Operand-family rows (`op@variant`) -------------------------------------
#
# The bench sweep measures one input per cell, so an op whose kernel branches
# on the value carries a small FAMILY of rows: the base `op` plus `op@<variant>`
# (`ln@hard`, `log@near1`, `powf@int`, ...), each pinned to a different path
# through the same public function. See `bench-compare/benches/compare_common.rs`
# -> "Operand families".
#
# A variant belongs to its BASE op's category — `log@near1` is a logarithm
# however it is spelled — so the separator is stripped before the category
# lookup. Without this every family row would land in `trigonometry`, which is
# where `op_category`'s fallback puts anything it does not recognise, and the
# only signal would be a printed warning nobody reads in CI.
#
# `@` is the separator because it cannot occur in a Rust method name, so it can
# never collide with a real function: `to_degrees` contains `_`, which is why
# `_` is not used here and why `summarise.py` anchors its group regex on the
# `_D<n>_s<n>` suffix rather than the first underscore. Criterion does not
# rewrite `@` in a report directory name (`report::make_filename_safe` replaces
# only `? " / \ * < > : | ^`), and the collator reads the canonical-case
# `group_id` out of `benchmark.json` anyway.
_FAMILY_SEP = "@"


def base_op(op: str) -> str:
    """The public function a bench row measures: `ln@hard` -> `ln`, `ln` -> `ln`.

    Used for categorisation only — the row keeps its own full name everywhere
    it is displayed, because the variant is the whole point of the row."""
    return op.split(_FAMILY_SEP, 1)[0]

# --- Diagnostic (measured, never published) bench rows ---------------------
#
# The sweep carries rows that exist to MEASURE a kernel, not to document a
# public function. `ln_nd` is the first: it runs `ln` at a non-degenerate
# argument, because the published `ln` row's operand (2.0) is an exact power of
# two whose range reduction collapses to mantissa 1 — so that row times a
# short-circuit, not the log series. Such a row must stay in the bbc artifacts
# (it IS the measurement) and must never reach the site, because `ln_nd` names
# no function anyone can call.
#
# Excluded by a NAMING CONVENTION, not a list of op names: name a diagnostic row
# `<op>_nd` and it is measured and never published, with nothing to remember at
# render time. A marker for a different KIND of diagnostic is added here — one
# entry covering every row that uses it — rather than one entry per row.
#
# The public direction is structural too: an op the category table lists is
# public by definition and ALWAYS publishes, so no marker added here can ever
# silently swallow a real function. `log2`/`log10` are listed, so they publish
# whatever else changes.
_DIAGNOSTIC_SUFFIXES = ("_nd",)


def is_diagnostic_op(op: str) -> bool:
    """True for a bench row that measures a kernel but names no public function,
    so it must not be published. An op listed in `_CATEGORY_OPS` is public by
    definition and is never diagnostic, whatever its spelling.

    An `op@variant` family row is judged on its BASE op, so a family of a
    published function publishes and a family of a diagnostic one would not.
    The two conventions stay independent: `_nd` marks "never publish", `@`
    marks "same function, different input"."""
    if base_op(op) in _OP_CATEGORY:
        return False
    return op.endswith(_DIAGNOSTIC_SUFFIXES)


def op_category(op: str) -> str:
    """The category an op renders under: `arithmetic`, `roots-and-exponents`, or
    `trigonometry`. An `op@variant` row takes its base op's category. An op in
    no listed set falls back to trigonometry (so a new function still renders
    somewhere), logged once per name."""
    cat = _OP_CATEGORY.get(base_op(op))
    if cat is None:
        if op not in _warned_ops:
            _warned_ops.add(op)
            print(f"render_docs: op '{op}' is in no category — rendering under trigonometry")
        return "trigonometry"
    return cat


def _ops_in_category(ops, category: str) -> list[str]:
    """The ops from `ops` belonging to `category`, alphabetical."""
    return sorted(o for o in ops if op_category(o) == category)


def _perf_op_section(op: str, op_rows) -> list[str]:
    """The `### op` block for one operation: the width x scale timing table beside
    the overlaid family chart, each cell in its own natural unit.

    `op` is the BASE op and `op_rows` carries its whole family, so the section
    holds ONE chart with the families laid over each other rather than one chart
    per row. The table lists every family so the numbers behind the overlay are
    readable directly — which is also the relief the palette's light-mode
    contrast check requires."""
    families, widths, P, series = _perf_families(op_rows)
    # "Width" while an op has no family axis — the column then holds nothing but
    # widths, and renaming it would churn every single-row op's page for nothing.
    head = ("| Width | " if len(families) == 1 else "| Row | ") \
        + " | ".join(_pos_labels(P)) + " |"
    rule = "| :-- | " + " | ".join(["--:"] * P) + " |"
    trows = [head, rule]
    for fam in families:
        for w in widths:
            vals = series.get((fam, w))
            if vals is None:
                continue
            cells = [(_fmt_ns(v) if v is not None else "·") for v in vals]
            label = f"D{w}" if len(families) == 1 else f"`{fam}` D{w}"
            trows.append(f"| {label} | " + " | ".join(cells) + " |")
    return [
        f"### `{op}`",
        "",
        '<div class="grid perf-grid" markdown>',
        "",
        "\n".join(trows),
        "",
        perf_chart_block(op, op_rows),
        "",
        "</div>",
        "",
    ]


def _render_perf_category(category: str) -> str:
    """The body of one Performance category sub-page: just the `### op` sections for
    that category (the page IS the category, so no `## category` header). All from
    results/timing/bbc_medians.tsv.

    Rows are grouped by BASE op, so `ln` and `ln@hard` land in one section under
    one chart instead of two sections that a reader would have to compare by
    flicking between them."""
    rows = _timing_rows()
    if not rows:
        return _PENDING_PERF
    by_op: dict[str, list] = {}
    for r in rows:
        by_op.setdefault(base_op(r[0]), []).append(r)
    ops = _ops_in_category(by_op, category)
    if not ops:
        return f"_No {_CATEGORY_LABELS[category].lower()} functions in this dataset._"
    # The family CSS travels WITH the block that needs it, emitted once per page,
    # rather than living in `docs/stylesheets/extra.css`. That keeps one
    # definition for both surfaces (`perf_compare.py` folds the same constant
    # into its own <style>) instead of a copy in each that can drift apart.
    out: list[str] = [f"<style>{PERF_FAMILY_CSS}</style>", ""]
    for op in ops:
        out += _perf_op_section(op, by_op[op])
    return "\n".join(out).rstrip()


def render_performance_arithmetic() -> str:
    return _render_perf_category("arithmetic")


def render_performance_roots() -> str:
    return _render_perf_category("roots-and-exponents")


def render_performance_trig() -> str:
    return _render_perf_category("trigonometry")


# --- History page (docs/history.md) — generated from results/history/ ---------
#
# The history gates (history.yml / tests/history.rs) time the live crate beside
# the pinned releases (0.4.4, 0.3.3) over ONE representative cell per width (the
# middle-band scale, single mode), reported never asserted. The aggregate job
# self-commits results/history/history.tsv as per-(function, width, version)
# nanoseconds:  function  width  version  nanos  min  max  (`nanos` is the cell
# median; `min`/`max` bound its samples for the graph band — absent on the older
# 4-column TSV, which then renders bandless). `version` is the subject's capability
# name — `decimal-scaled` (live) or `decimal-scaled@X.Y.Z`. The page is a SECTION:
# an index (header + units + width map) plus three category sub-pages, each a width
# x version table (median time + the slowdown vs the latest release) beside a
# log-time-vs-width graph with one banded line per version.

_PENDING_HIST = "_Pending the first history-gates CI run — this renders from `results/history/history.tsv`._"
_HIST_HEADER = ["function", "width", "version", "nanos"]
# Distinct line colours per version; newest gets the primary brand tone.
_VER_COLORS = ["var(--md-primary-fg-color)", "var(--md-accent-fg-color)",
               "var(--dusk-purple,#7A6A8E)", "#367594", "#787A79"]


def _hist_version_label(name: str) -> str:
    """A subject capability name -> a clean version: the live `decimal-scaled` ->
    the current crate version; `decimal-scaled@X.Y.Z` -> `X.Y.Z`."""
    if name.startswith("decimal-scaled@"):
        return name.split("@", 1)[1]
    if name == "decimal-scaled":
        return crate_version()
    return name


def _semver_key(v: str) -> tuple:
    return tuple(int(x) if x.isdigit() else 0 for x in v.split("."))


def _history_rows() -> list[tuple[str, int, str, float, float, float]] | None:
    """`(function, width, version_label, median_ns, lo_ns, hi_ns)` per cell, or None
    if the summary is absent / not on the current schema (renders pending rather than
    garbage). `lo`/`hi` are the min/max columns when present (the band); on the older
    4-column TSV both equal the median, so the page renders with no band. The header
    guard checks only the first four columns, so old and new TSVs both pass.

    Diagnostic rows are dropped here on the same rule as `_timing_rows`. The
    History bench carries none today; applying it at both readers is what makes
    "no published page renders a diagnostic row" a property of this file rather
    than a fact about one data source."""
    if not HISTORY_RESULTS.exists():
        return None
    lines = HISTORY_RESULTS.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].split("\t")[:4] != _HIST_HEADER:
        return None
    rows = []
    for line in lines[1:]:
        c = line.split("\t")
        if len(c) >= 4 and c[1].isdigit() and not is_diagnostic_op(c[0]):
            med = float(c[3])
            lo = float(c[4]) if len(c) >= 6 and c[4] else med
            hi = float(c[5]) if len(c) >= 6 and c[5] else med
            rows.append((c[0], int(c[1]), _hist_version_label(c[2]), med, lo, hi))
    return rows


def _history_svg(widths: list[int], versions: list[str], latest: str,
                 cells: dict[tuple[int, str], tuple[float, float, float]]) -> str:
    """Log-time(y) vs width(x), one polyline per version (distinct colours, the
    latest release boldest) with a small in-graph legend. Each version also draws a
    translucent min–max band BEHIND the median lines; a version whose band is
    degenerate (lo==hi at every point, e.g. the older single-sample TSV) draws none.
    `cells` maps `(width, version) -> (median, lo, hi)`."""
    import math
    flat = [v for w in widths for ver in versions if (w, ver) in cells
            for v in cells[(w, ver)]]
    if len(flat) < 2 or len(widths) < 2:
        return ""
    lo, hi = math.floor(math.log10(min(flat))), math.ceil(math.log10(max(flat)))
    if hi <= lo:
        hi = lo + 1
    W, H, L, Rm, Tm, Bm = 460, 262, 52, 10, 30, 30  # extra top margin for the legend
    pw, ph, n = W - L - Rm, H - Tm - Bm, len(widths)
    xp = lambda i: L + pw * i / (n - 1)
    yp = lambda ns: Tm + ph * (hi - math.log10(ns)) / (hi - lo)
    colour = {v: _VER_COLORS[k % len(_VER_COLORS)]
              for k, v in enumerate(sorted(versions, key=_semver_key, reverse=True))}
    p = [f'<svg viewBox="0 0 {W} {H}" width="100%" style="height:auto;'
         f'color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg">']
    for d in range(lo, hi + 1):
        y = yp(10 ** d)
        p.append(f'<line x1="{L}" y1="{y:.1f}" x2="{L + pw}" y2="{y:.1f}" '
                 f'stroke="currentColor" stroke-opacity="0.15"/>')
        p.append(f'<text x="{L - 6}" y="{y + 3:.1f}" text-anchor="end" font-size="9" '
                 f'fill="currentColor">{_ns_decade(d)}</text>')
    for i, w in enumerate(widths):
        p.append(f'<text x="{xp(i):.1f}" y="{Tm + ph + 12}" text-anchor="middle" '
                 f'font-size="8" fill="currentColor">{w}</text>')
    lx = L  # legend across the top margin
    for v in versions:
        p.append(f'<line x1="{lx}" y1="20" x2="{lx + 14}" y2="20" stroke="{colour[v]}" stroke-width="2"/>')
        p.append(f'<text x="{lx + 17}" y="23" font-size="9" fill="currentColor">{v}</text>')
        lx += 24 + 7 * len(v)
    for v in versions:  # min–max bands first, behind the median lines
        seg = [(xp(i), *cells[(w, v)]) for i, w in enumerate(widths) if (w, v) in cells]
        if len(seg) < 2 or all(m == lo_ == hi_ for _x, m, lo_, hi_ in seg):
            continue
        top = " ".join(f"{x:.1f},{yp(hi_):.1f}" for x, _m, _lo, hi_ in seg)
        bot = " ".join(f"{x:.1f},{yp(lo_):.1f}" for x, _m, lo_, _hi in reversed(seg))
        p.append(f'<polygon points="{top} {bot}" fill="{colour[v]}" fill-opacity="0.12"/>')
    for v in versions:  # median lines on top
        line = [(xp(i), cells[(w, v)][0]) for i, w in enumerate(widths) if (w, v) in cells]
        if len(line) < 2:
            continue
        pts = " ".join(f"{x:.1f},{yp(val):.1f}" for x, val in line)
        p.append(f'<polyline points="{pts}" fill="none" stroke="{colour[v]}" '
                 f'stroke-width="{2.0 if v == latest else 1.3}"/>')
    p.append(f'<line x1="{L}" y1="{Tm}" x2="{L}" y2="{Tm + ph}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append(f'<line x1="{L}" y1="{Tm + ph}" x2="{L + pw}" y2="{Tm + ph}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append("</svg>")
    return "".join(p)


def render_history_units() -> str:
    """The time-unit legend for the History page header (left column)."""
    rows = _history_rows()
    return _units_legend([r[3] for r in rows]) if rows else _PENDING_HIST


def _history_op_section(op: str, cells, versions: list[str], latest: str) -> list[str]:
    """The `### op` block for one operation: the width x version table (median time +
    slowdown vs the latest release) beside its per-version log-time graph. `cells`
    maps `(width, version) -> (median, lo, hi)` for this op only."""
    widths = sorted({w for (w, _v) in cells})
    head = "| Width | " + " | ".join(versions) + " |"
    rule = "| :-- | " + " | ".join(["--:"] * len(versions)) + " |"
    trows = [head, rule]
    for w in widths:
        ref = cells.get((w, latest))
        ref_med = ref[0] if ref else None
        row = []
        for v in versions:
            cell = cells.get((w, v))
            if cell is None:
                row.append("·")
            elif v == latest or not ref_med:
                row.append(_fmt_ns(cell[0]))
            else:
                row.append(f"{_fmt_ns(cell[0])} ({cell[0] / ref_med:.2g}×)")
        trows.append(f"| D{w} | " + " | ".join(row) + " |")
    return [
        f"### `{op}`",
        "",
        '<div class="grid perf-grid" markdown>',
        "",
        "\n".join(trows),
        "",
        "<figure>",
        _history_svg(widths, versions, latest, cells),
        "<figcaption>Median time vs width (log scale), one line per release with a "
        "shaded min–max band; the multiplier is the slowdown relative to the latest."
        "</figcaption>",
        "</figure>",
        "",
        "</div>",
        "",
    ]


def _render_history_category(category: str) -> str:
    """The body of one History category sub-page: just the `### op` sections for that
    category (the page IS the category). From results/history/history.tsv."""
    rows = _history_rows()
    if not rows:
        return _PENDING_HIST
    versions = sorted({v for _fn, _w, v, *_ in rows}, key=_semver_key)
    latest = versions[-1]
    by_op: dict[str, dict[tuple[int, str], tuple[float, float, float]]] = {}
    for fn, w, v, med, lo, hi in rows:
        by_op.setdefault(fn, {})[(w, v)] = (med, lo, hi)
    ops = _ops_in_category(by_op, category)
    if not ops:
        return f"_No {_CATEGORY_LABELS[category].lower()} functions in this dataset._"
    out: list[str] = []
    for op in ops:
        out += _history_op_section(op, by_op[op], versions, latest)
    return "\n".join(out).rstrip()


def render_history_arithmetic() -> str:
    return _render_history_category("arithmetic")


def render_history_roots() -> str:
    return _render_history_category("roots-and-exponents")


def render_history_trig() -> str:
    return _render_history_category("trigonometry")


# --- Comparisons page (docs/comparisons.md) — speed vs peer crates ------------
#
# The library-perf bench (lib-perf.yml / golden-competitors/tests/lib_perf.rs) times
# decimal-scaled beside the fixed-precision peer crates over the golden set. To compare
# SPEED fairly, decimal-scaled is timed at several COMPARISON SCALES per width — one
# per peer-precision level (17 narrow anchor; 28 = rust_decimal; 37 = D38 ceiling =
# decimal-rs / g_math's 38 significant digits; 152 = D153 ceiling ≈ fastnum's 154) —
# so each peer is read beside the decimal-scaled line at its own precision. The
# aggregate self-commits results/lib_cmp/medians.tsv:
#   function  width  library  scale  nanos  min  max
# (`nanos` is the per-(cell, scale) median; `min`/`max` bound its samples for the band
# / whisker). The page is a SECTION: an index (header + units + the per-library
# precision model + the input-distribution table) plus three category sub-pages, each a
# decimal-scaled scale x width table and a peers table, beside ONE graph: decimal-scaled
# as a line per comparison scale on a LOG width axis, and each fixed-precision peer as a
# single marker at its significant-digit capacity (bigdecimal/dashu are parked — their
# working width is input-driven, deferred to a later release).

_PENDING_CMP = "_Pending the first lib-perf CI run — this renders from `results/lib_cmp/medians.tsv`._"
_CMP_HEADER = ["function", "width", "library", "scale", "nanos", "min", "max"]
_OURS = "decimal-scaled"
# The decimal-scaled comparison scales, ascending (one line each).
_CMP_SCALES = [17, 28, 37, 152]
# Each fixed-precision peer: (significant-digit capacity for its x marker, the matching
# decimal-scaled scale for the slowdown ratio).
_PEERS = {
    "rust_decimal": (28, 28),
    "decimal-rs": (38, 37),
    "g_math": (38, 37),
    "fastnum": (154, 152),
}
# Peer render order (widest capacity first), used for the table + legend.
_PEER_ORDER = ["fastnum", "decimal-rs", "g_math", "rust_decimal"]
# decimal-scaled: a blue ramp light->dark with rising precision. Peers: a muted palette.
_DS_SCALE_COLOR = {17: "#7aa7f5", 28: "#3b82f6", 37: "#2563eb", 152: "#1e3a8a"}
_OURS_FALLBACK = "#2563eb"
_PEER_COLOR = {
    "rust_decimal": "#C68A2E", "decimal-rs": "#7A6A8E",
    "g_math": "#5E8C3A", "fastnum": "#B5663C",
}
# Our width tiers, the x-axis gridline ticks.
_CMP_WIDTH_TICKS = [18, 38, 57, 76, 115, 153, 230, 307, 462, 616, 924, 1232]


def _libcmp_rows():
    """`(function, width, library, scale, median_ns, lo_ns, hi_ns)` per (cell, scale),
    or None if the summary is absent / not on the current schema. decimal-scaled emits
    several scales per width (one comparison line each); each fixed peer repeats across
    the cells it was driven on. `lo`/`hi` are the min/max columns (the band/whisker).
    A pre-scale-column (6-field) TSV fails the header guard and renders as pending."""
    if not LIBCMP_RESULTS.exists():
        return None
    lines = LIBCMP_RESULTS.read_text(encoding="utf-8").splitlines()
    if not lines or lines[0].split("\t") != _CMP_HEADER:
        return None
    rows = []
    for line in lines[1:]:
        c = line.split("\t")  # function width library scale nanos min max
        if len(c) >= 7 and c[1].isdigit():
            rows.append((
                c[0], int(c[1]), c[2], int(c[3]) if c[3].isdigit() else 0,
                float(c[4]), float(c[5]), float(c[6]),
            ))
    return rows


def _comparisons_svg(ds_series, peer_points) -> str:
    """decimal-scaled as one line per comparison scale (a faint min–max band behind
    each) on a LOG significant-digit-width x-axis, plus each fixed-precision peer as a
    single diamond marker at its capacity width with a min–max whisker. `ds_series`
    maps scale -> sorted `[(width, med, lo, hi)]`; `peer_points` is
    `[(library, capacity, med, lo, hi)]` in render order."""
    import math
    flat = [v for s in ds_series.values() for (_w, m, lo, hi) in s for v in (m, lo, hi)]
    flat += [v for (_l, _c, m, lo, hi) in peer_points for v in (m, lo, hi)]
    if not flat:
        return ""
    ylo, yhi = math.floor(math.log10(min(flat))), math.ceil(math.log10(max(flat)))
    if yhi <= ylo:
        yhi = ylo + 1
    ticks = _CMP_WIDTH_TICKS
    xlo, xhi = math.log10(ticks[0] * 0.9), math.log10(ticks[-1] * 1.07)
    W, H, L, Rm, Tm, Bm = 560, 320, 54, 12, 50, 34  # top margin holds the legend
    pw, ph = W - L - Rm, H - Tm - Bm
    base = Tm + ph

    def xp(width):
        return L + pw * (math.log10(width) - xlo) / (xhi - xlo)

    def yp(ns):
        return Tm + ph * (yhi - math.log10(ns)) / (yhi - ylo)

    p = [f'<svg viewBox="0 0 {W} {H}" width="100%" style="height:auto;'
         f'color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg">']
    for d in range(ylo, yhi + 1):     # y gridlines + decade labels
        y = yp(10 ** d)
        p.append(f'<line x1="{L}" y1="{y:.1f}" x2="{L + pw}" y2="{y:.1f}" '
                 f'stroke="currentColor" stroke-opacity="0.15"/>')
        p.append(f'<text x="{L - 6}" y="{y + 3:.1f}" text-anchor="end" font-size="9" '
                 f'fill="currentColor">{_ns_decade(d)}</text>')
    for w in ticks:                   # x (width) gridlines + labels
        x = xp(w)
        p.append(f'<line x1="{x:.1f}" y1="{Tm}" x2="{x:.1f}" y2="{base}" '
                 f'stroke="currentColor" stroke-opacity="0.06"/>')
        p.append(f'<text x="{x:.1f}" y="{base + 12}" text-anchor="middle" '
                 f'font-size="8" fill="currentColor">{w}</text>')
    p.append(f'<text x="{L + pw / 2:.1f}" y="{base + 25}" text-anchor="middle" '
             f'font-size="8.5" fill="currentColor" fill-opacity="0.75">'
             f'significant-digit width (log)</text>')
    for scale in sorted(ds_series):   # min–max bands first, behind the lines
        series = ds_series[scale]
        if len(series) < 2 or all(m == lo == hi for (_w, m, lo, hi) in series):
            continue
        col = _DS_SCALE_COLOR.get(scale, _OURS_FALLBACK)
        top = " ".join(f"{xp(w):.1f},{yp(hi):.1f}" for (w, _m, _lo, hi) in series)
        bot = " ".join(f"{xp(w):.1f},{yp(lo):.1f}" for (w, _m, lo, _hi) in reversed(series))
        p.append(f'<polygon points="{top} {bot}" fill="{col}" fill-opacity="0.10"/>')
    for scale in sorted(ds_series):   # decimal-scaled lines (or an isolated dot)
        series = ds_series[scale]
        col = _DS_SCALE_COLOR.get(scale, _OURS_FALLBACK)
        if len(series) == 1:
            (w, m, _lo, _hi) = series[0]
            p.append(f'<circle cx="{xp(w):.1f}" cy="{yp(m):.1f}" r="1.8" fill="{col}"/>')
        else:
            pl = " ".join(f"{xp(w):.1f},{yp(m):.1f}" for (w, m, _lo, _hi) in series)
            p.append(f'<polyline points="{pl}" fill="none" stroke="{col}" stroke-width="1.7"/>')
    for (lib, cap, m, lo, hi) in peer_points:   # peer markers + min–max whisker
        x, col = xp(cap), _PEER_COLOR.get(lib, "#9aa0a6")
        if hi > lo:
            p.append(f'<line x1="{x:.1f}" y1="{yp(lo):.1f}" x2="{x:.1f}" y2="{yp(hi):.1f}" '
                     f'stroke="{col}" stroke-width="1.3"/>')
            for yy in (lo, hi):
                p.append(f'<line x1="{x - 3:.1f}" y1="{yp(yy):.1f}" x2="{x + 3:.1f}" '
                         f'y2="{yp(yy):.1f}" stroke="{col}" stroke-width="1.3"/>')
        r = 3.0
        p.append(f'<polygon points="{x:.1f},{yp(m) - r:.1f} {x + r:.1f},{yp(m):.1f} '
                 f'{x:.1f},{yp(m) + r:.1f} {x - r:.1f},{yp(m):.1f}" fill="{col}" '
                 f'stroke="var(--md-default-bg-color)" stroke-width="0.6"/>')
    lx, ly = L, 14                    # legend, wrapping to a second row if needed
    legend = [(f"decimal-scaled @{s} prec", _DS_SCALE_COLOR.get(s, _OURS_FALLBACK))
              for s in sorted(ds_series)]
    legend += [(lib, _PEER_COLOR.get(lib, "#9aa0a6")) for (lib, *_r) in peer_points]
    for (text, col) in legend:
        wl = 16 + 5.6 * len(text)
        if lx + wl > L + pw:
            lx, ly = L, ly + 13
        p.append(f'<rect x="{lx:.1f}" y="{ly}" width="9" height="9" fill="{col}"/>')
        p.append(f'<text x="{lx + 12:.1f}" y="{ly + 8}" font-size="8" fill="currentColor">{text}</text>')
        lx += wl
    p.append(f'<line x1="{L}" y1="{Tm}" x2="{L}" y2="{base}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append(f'<line x1="{L}" y1="{base}" x2="{L + pw}" y2="{base}" stroke="currentColor" stroke-opacity="0.4"/>')
    p.append("</svg>")
    return "".join(p)


def render_comparisons_units() -> str:
    """The time-unit legend for the Comparisons page header (left column)."""
    rows = _libcmp_rows()
    return _units_legend([r[4] for r in rows]) if rows else _PENDING_CMP


def _comparisons_op_section(op: str, op_rows) -> list[str]:
    """The `### op` block: a decimal-scaled scale x width table and a peers table, above
    ONE graph (decimal-scaled lines per comparison scale + peer markers). `op_rows` is
    `[(width, library, scale, median, lo, hi)]` for this op only."""
    import statistics
    ds_series: dict[int, list] = {}             # scale -> [(width, med, lo, hi)]
    ds_at: dict[tuple[int, int], float] = {}    # (width, scale) -> med
    peer_acc: dict[str, tuple] = {}             # library -> ([med], [lo], [hi])
    for (w, lib, sc, med, lo, hi) in op_rows:
        if lib == _OURS:
            ds_series.setdefault(sc, []).append((w, med, lo, hi))
            ds_at[(w, sc)] = med
        else:
            a = peer_acc.setdefault(lib, ([], [], []))
            a[0].append(med)
            a[1].append(lo)
            a[2].append(hi)
    for sc in ds_series:
        ds_series[sc].sort()
    # Each fixed peer collapses to one point: median of its per-cell medians, with the
    # min/max spread for the whisker; placed at its significant-digit capacity.
    peer_points = []
    for lib in _PEER_ORDER:
        acc = peer_acc.get(lib)
        if acc and acc[0] and lib in _PEERS:
            cap, _match = _PEERS[lib]
            peer_points.append((lib, cap, statistics.median(acc[0]), min(acc[1]), max(acc[2])))

    out = [f"### `{op}`", "", '<div class="grid perf-grid" markdown>', ""]

    # decimal-scaled table: comparison-scale columns x width rows.
    scales = [s for s in _CMP_SCALES if s in ds_series]
    ds_widths = sorted({w for s in ds_series.values() for (w, *_r) in s})
    if scales and ds_widths:
        head = "| Width | " + " | ".join(f"@{s} prec" for s in scales) + " |"
        rule = "| :-- | " + " | ".join(["--:"] * len(scales)) + " |"
        trows = [head, rule]
        for w in ds_widths:
            cells = " | ".join(
                _fmt_ns(ds_at[(w, s)]) if (w, s) in ds_at else "·" for s in scales
            )
            trows.append(f"| D{w} | {cells} |")
        out += [
            "**decimal-scaled** — median time per call at each comparison scale "
            "(· = the tier cannot hold that precision):",
            "",
            "\n".join(trows),
            "",
        ]

    # peers table: capacity + median + slowdown vs decimal-scaled at the matching scale.
    if peer_points:
        prows = [
            "| Peer | Precision | Median | vs decimal-scaled |",
            "| :-- | --: | --: | --: |",
        ]
        for (lib, cap, med, _lo, _hi) in peer_points:
            _c, match = _PEERS[lib]
            ref_series = ds_series.get(match)
            ref = ref_series[0][1] if ref_series else None  # smallest holding width
            ratio = f"{med / ref:.2g}× @{match}" if ref else "·"
            prows.append(f"| {lib} | {cap} | {_fmt_ns(med)} | {ratio} |")
        out += [
            "**peers** (fixed precision; each timed beside the matching "
            "decimal-scaled line):",
            "",
            "\n".join(prows),
            "",
        ]

    out += [
        "<figure>",
        _comparisons_svg(ds_series, peer_points),
        "<figcaption>decimal-scaled timed at each comparison scale its tier can hold "
        "(17, 28, 37, 152) across its widths — one line each, with a shaded min–max "
        "band; every fixed-precision peer is one diamond at its significant-digit "
        "capacity with a min–max whisker. Arithmetic is width-bound, so its scale-lines "
        "nearly overlap; transcendentals spread by precision.</figcaption>",
        "</figure>",
        "",
        "</div>",
        "",
    ]
    return out


def _render_comparisons_category(category: str) -> str:
    """The body of one Comparisons category sub-page: just the `### op` sections for
    that category (the page IS the category). From results/lib_cmp/medians.tsv —
    decimal-scaled timed at the comparison scales (17/28/37/152), peers at their fixed
    precision."""
    rows = _libcmp_rows()
    if not rows:
        return _PENDING_CMP
    by_op: dict[str, list] = {}
    for (op, w, lib, sc, med, lo, hi) in rows:
        by_op.setdefault(op, []).append((w, lib, sc, med, lo, hi))
    ops = _ops_in_category(by_op, category)
    if not ops:
        return f"_No {_CATEGORY_LABELS[category].lower()} functions in this dataset._"
    out: list[str] = []
    for op in ops:
        out += _comparisons_op_section(op, by_op[op])
    return "\n".join(out).rstrip()


def render_comparisons_arithmetic() -> str:
    return _render_comparisons_category("arithmetic")


def render_comparisons_roots() -> str:
    return _render_comparisons_category("roots-and-exponents")


def render_comparisons_trig() -> str:
    return _render_comparisons_category("trigonometry")


# --- Comparisons input distribution (docs/comparisons.md) ---------------------
#
# Characterises the golden inputs the comparison runs over: per function, from its
# committed `decimal-scaled-golden/golden/<fn>.au` file, the count of input fields
# and the spread of their fractional- / significant-digit counts plus the integer
# share. Inputs are every data line's first `arity` fields (the last is the expected
# output), matching the harness loader; `arity` is 2 for the binary functions.
_BINARY_FUNCS = frozenset(
    {"log", "atan2", "powf", "hypot", "add", "sub", "mul", "div", "rem"}
)
_CMP_INPUTS_PENDING = (
    "_Pending the golden set — generated from the committed files under "
    "`decimal-scaled-golden/golden/`._"
)


def _parse_decimal(token: str):
    """`(fractional_digits, significant_digits, is_integer)` for one decimal token
    (`123.0045`, `-7`, `1e-38`), or None if it is not a decimal number. Fractional
    digits fold in any `eN` exponent (so `1e-38` is 38, `1.5e2` is 0); significant
    digits count from the first non-zero digit, and a pure integer's trailing zeros
    are not significant. An integer is scale 0 — zero fractional digits."""
    s = token.strip()
    if not s:
        return None
    if s[0] in "+-":
        s = s[1:]
    exp = 0
    for ec in ("e", "E"):
        if ec in s:
            mant, _, e = s.partition(ec)
            try:
                exp = int(e)
            except ValueError:
                return None
            s = mant
            break
    int_part, _dot, frac_part = s.partition(".")
    digits = int_part + frac_part
    if not digits or not digits.isdigit():
        return None
    frac_digits = max(0, len(frac_part) - exp)
    stripped = digits.lstrip("0")
    if not stripped:
        sig = 0
    elif not frac_part:
        sig = len(stripped.rstrip("0")) or 1
    else:
        sig = len(stripped)
    return frac_digits, sig, frac_digits == 0


def _golden_input_stats(path: Path, arity: int):
    """Read one golden `.au` file once; return `(count, fracs, sigs, n_int)` over its
    input fields — every data line's first `arity` fields — or None if it has none.
    `#` metadata, `//` comments and blank lines are skipped and a line whose field
    count != arity + 1 is ignored, matching the harness loader."""
    fracs: list[int] = []
    sigs: list[int] = []
    n_int = 0
    for raw in path.read_text(encoding="utf-8").splitlines():
        text = raw.strip()
        if not text or text.startswith("#") or text.startswith("//"):
            continue
        fields = [f for f in text.replace("\t", " ").split(" ") if f]
        if len(fields) != arity + 1:
            continue
        for tok in fields[:arity]:
            parsed = _parse_decimal(tok)
            if parsed is None:
                continue
            frac, sig, is_int = parsed
            fracs.append(frac)
            sigs.append(sig)
            if is_int:
                n_int += 1
    if not fracs:
        return None
    return len(fracs), fracs, sigs, n_int


def _fmt_mmm(vals) -> str:
    """`min / mean / max` for a list of integer digit counts (mean to one decimal)."""
    return f"{min(vals)} / {sum(vals) / len(vals):.1f} / {max(vals)}"


def render_comparisons_inputs() -> str:
    """The input-distribution table for the Comparisons index: per function, the
    count of input fields and the min/mean/max of their fractional- and
    significant-digit counts, plus the integer share. Functions in category order."""
    if not GOLDEN_DIR.exists():
        return _CMP_INPUTS_PENDING
    files = {p.stem: p for p in GOLDEN_DIR.glob("*.au")}
    if not files:
        return _CMP_INPUTS_PENDING
    rows = [
        "| Function | Inputs | Fractional digits (min/mean/max) "
        "| Significant digits (min/mean/max) | % integer |",
        "| :-- | --: | :-- | :-- | --: |",
    ]
    any_row = False
    for category in _CATEGORY_ORDER:
        for fn in sorted(f for f in files if op_category(f) == category):
            stats = _golden_input_stats(files[fn], 2 if fn in _BINARY_FUNCS else 1)
            if stats is None:
                continue
            n, fracs, sigs, n_int = stats
            rows.append(
                f"| `{fn}` | {n:,} | {_fmt_mmm(fracs)} | {_fmt_mmm(sigs)} "
                f"| {100 * n_int / n:.0f}% |"
            )
            any_row = True
    return "\n".join(rows) if any_row else _CMP_INPUTS_PENDING


# `key -> (target file relative to ROOT, builder)`.
REGIONS: dict[str, tuple[str, "callable"]] = {
    "widths:table": ("docs/widths.md", render_widths_table),
    "widths:count": ("docs/widths.md", render_width_count_word),
    "widths:slowest": ("docs/widths.md", render_slowest_cell),
    "install:dependency": ("README.md", render_install_dependency),
    "home:tested": ("docs/index.md", render_home_tested),
    "readme:tested": ("README.md", render_home_tested),
    "golden:counts": ("docs/golden.md", render_golden_counts),
    "precision:stats": ("docs/precision.md", render_precision_stats),
    "precision:surface": ("docs/precision.md", render_precision_surface),
    "performance:units": ("docs/performance.md", render_performance_units),
    "performance:widths": ("docs/performance.md", render_bench_widths),
    "performance:body:arithmetic": ("docs/performance/arithmetic.md", render_performance_arithmetic),
    "performance:body:roots": ("docs/performance/roots-and-exponents.md", render_performance_roots),
    "performance:body:trig": ("docs/performance/trigonometry.md", render_performance_trig),
    "history:units": ("docs/history.md", render_history_units),
    "history:widths": ("docs/history.md", render_bench_widths),
    "history:body:arithmetic": ("docs/history/arithmetic.md", render_history_arithmetic),
    "history:body:roots": ("docs/history/roots-and-exponents.md", render_history_roots),
    "history:body:trig": ("docs/history/trigonometry.md", render_history_trig),
    "comparisons:units": ("docs/comparisons.md", render_comparisons_units),
    "comparisons:inputs": ("docs/comparisons.md", render_comparisons_inputs),
    "comparisons:body:arithmetic": ("docs/comparisons/arithmetic.md", render_comparisons_arithmetic),
    "comparisons:body:roots": ("docs/comparisons/roots-and-exponents.md", render_comparisons_roots),
    "comparisons:body:trig": ("docs/comparisons/trigonometry.md", render_comparisons_trig),
}


# --- Region splice --------------------------------------------------------


def splice(text: str, key: str, body: str) -> str:
    """Replace the body between the `key` markers in `text`. The markers
    are preserved; everything between them becomes `body`."""
    begin = BEGIN.format(key=key)
    end = END.format(key=key)
    # Region body sits between the marker lines. Capture leading/trailing
    # newlines so single-line (inline) regions and block regions both work.
    pattern = re.compile(
        re.escape(begin) + r"(.*?)" + re.escape(end), re.S
    )
    if not pattern.search(text):
        raise SystemExit(
            f"render_docs: markers for '{key}' not found in the target file"
        )

    # Inline region (markers on the same line) -> no surrounding newlines.
    # Block region (markers on their own lines) -> body on its own lines.
    def repl(m: re.Match) -> str:
        between = m.group(1)
        if "\n" in between:
            return f"{begin}\n{body}\n{end}"
        return f"{begin}{body}{end}"

    return pattern.sub(repl, text, count=1)


def render_file(rel_path: str, keys: list[str]) -> tuple[str, str]:
    """Return `(original, rendered)` text for one target file."""
    path = ROOT / rel_path
    original = path.read_text(encoding="utf-8")
    rendered = original
    for key in keys:
        _file, builder = REGIONS[key]
        rendered = splice(rendered, key, builder())
    return original, rendered


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--check",
        action="store_true",
        help="report stale docs and exit 1 without writing",
    )
    args = ap.parse_args()

    # Group regions by their target file.
    by_file: dict[str, list[str]] = {}
    for key, (rel_path, _builder) in REGIONS.items():
        by_file.setdefault(rel_path, []).append(key)

    stale: list[str] = []
    for rel_path, keys in by_file.items():
        original, rendered = render_file(rel_path, keys)
        if rendered == original:
            continue
        if args.check:
            stale.append(rel_path)
        else:
            (ROOT / rel_path).write_text(rendered, encoding="utf-8", newline="\n")
            print(f"render_docs: updated {rel_path}")

    if args.check:
        if stale:
            print("render_docs: STALE generated regions in:")
            for p in stale:
                print(f"  - {p}")
            print("Run `python scripts/render_docs.py` and commit the result.")
            return 1
        print("render_docs: all generated regions are up to date.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
