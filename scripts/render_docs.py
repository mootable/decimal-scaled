#!/usr/bin/env python3
"""Single-source-of-truth renderer for the repeated facts in the docs.

Repeated facts — the crate version, the storage-width tier family, the
`MAX_SCALE = N - 1` rule, and the precision (LSBε) shootout tables —
live in exactly ONE place each:

  * the crate version    -> `Cargo.toml` `[package] version`
  * the tier family      -> `docs/_data/tiers.json`
  * the precision tables -> `results/precision/*.tsv` (rendered by
    `scripts/render_precision_table.py`, the same source the bench
    self-renderer uses)

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
   `Cargo.toml`, like the version; or, for a precision table, point at
   the relevant `results/precision/*.tsv`).
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
PRECISION_DIR = ROOT / "results" / "precision"
GOLDEN_DIR = ROOT / "decimal-scaled-golden" / "golden"
GOLDEN_RESULTS = ROOT / "results" / "golden" / "summary.tsv"
CELLS_SRC = ROOT / "decimal-scaled-cells" / "src" / "lib.rs"
ROUNDING_SRC = ROOT / "decimal-scaled-golden" / "src" / "support" / "rounding.rs"
TIMING_RESULTS = ROOT / "results" / "timing" / "bbc_medians.tsv"

# The precision-table renderer lives alongside this script. Import it as
# a module so the doc tables come from exactly the same code path (and
# the same TSV result files) as its standalone CLI output.
sys.path.insert(0, str(Path(__file__).resolve().parent))
import render_precision_table as precision  # noqa: E402

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
    """The Precision-page headline stats line — derived entirely from the test
    DEFINITION (the golden files, the `cells!` surface, the `RoundingMode`
    enum), so it is complete without a CI run. `total = inputs x (width,scale)
    x modes`."""
    cases, funcs = golden_counts()
    cells = golden_surface_cells()
    modes = rounding_mode_count()
    total = cases * cells * modes
    return (
        f"We execute {cases:,} specialised inputs across all {funcs} functions, on "
        f"{cells} widths and scales, under all {modes} rounding modes, resulting "
        f"in {total:,} separate checks."
    )


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


# Precision-table builders. Each returns the markdown table body for one
# width (and, where the doc shows a subset, one method ordering), read
# straight from the committed `results/precision/*.tsv` files via
# `render_precision_table.render_table`. Every cell traces back to one
# measured TSV row, so the findings prose can never drift from the data.

def _precision_table(width: str, methods: str | None) -> str:
    return precision.render_table(str(PRECISION_DIR), width, only=methods)


def render_precision_d38_readme() -> str:
    # README representative slice: nine methods at D38<19>.
    return _precision_table("D38", "sqrt,cbrt,exp,ln,sin,cos,tan,atan,asinh")


def render_precision_d38() -> str:
    # benchmarks.md: the full 22-function surface at D38<19>.
    return _precision_table("D38", None)


def render_precision_d76() -> str:
    # benchmarks.md: the eight-method wide-tier subset at D76<35>.
    return _precision_table("D76", "sqrt,cbrt,exp,ln,sin,cos,tan,atan")


def render_precision_d307() -> str:
    # benchmarks.md: the eight-method deep-scale subset at D307<150>.
    return _precision_table("D307", "sqrt,cbrt,exp,ln,sin,cos,tan,atan")


# `key -> (target file relative to ROOT, builder)`.
REGIONS: dict[str, tuple[str, "callable"]] = {
    "widths:table": ("docs/widths.md", render_widths_table),
    "widths:count": ("docs/widths.md", render_width_count_word),
    "install:dependency": ("README.md", render_install_dependency),
    "precision:D38:readme": ("README.md", render_precision_d38_readme),
    "golden:counts": ("docs/golden.md", render_golden_counts),
    "precision:stats": ("docs/precision.md", render_precision_stats),
    "precision:surface": ("docs/precision.md", render_precision_surface),
    "precision:D38": ("docs/comparisons.md", render_precision_d38),
    "precision:D76": ("docs/comparisons.md", render_precision_d76),
    "precision:D307": ("docs/comparisons.md", render_precision_d307),
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
