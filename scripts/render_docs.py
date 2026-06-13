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
