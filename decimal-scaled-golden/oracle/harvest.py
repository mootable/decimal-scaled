"""Read the per-function oracle input files: `lead/<func>.pb` — the lead
the generator transmutes into gold.

A `.pb` file is the `.au` shape minus the output column and the generation
provenance: one case per line (`arity` space-separated decimal literals), split
purely by FUNCTION — no width/scale anywhere (inputs are width-agnostic; the gate
derives every (width, scale) cell from each input). A `//` comment line sets the
WHY for every following input until the next comment — functional intent only
("near-zero directed-rounding band", "regression: retired exp_underflow.rs pin"),
carried by the generator into the `.au` per-line provenance comment.

Inputs are deduped by value (first why wins) and filtered to the function's
domain; a line whose field count does not match the function's arity is skipped
with a warning."""
import sys
from pathlib import Path
from typing import List, Optional, Tuple

from .functions import FUNCTIONS

# The why attached to inputs that precede any comment line.
DEFAULT_WHY = "coverage"


def harvest(func: str, lead_dir: Path) -> List[Tuple[List[str], str]]:
    """`(inputs, why)` for every in-domain case in `<lead_dir>/<func>.pb`."""
    f = FUNCTIONS[func]
    path = Path(lead_dir) / f"{func}.pb"
    if not path.exists():
        return []
    seen = set()
    out: List[Tuple[List[str], str]] = []
    why: str = DEFAULT_WHY
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("//"):
            text = line[2:].strip()
            if text:
                why = text
            continue
        fields = line.split()
        if len(fields) != f.arity:
            print(f"[warn] {path.name}: skipping line with {len(fields)} fields "
                  f"(arity {f.arity}): {line[:60]}", file=sys.stderr)
            continue
        key = tuple(fields)
        if key in seen or not f.in_domain(fields):
            continue
        seen.add(key)
        out.append((fields, why))
    return out
