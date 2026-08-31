"""Read the per-function oracle input files: `lead/<func>.pb` — the lead
the generator transmutes into gold.

A `.pb` file is the `.au` shape minus the output column and the generation
provenance: one case per line (`arity` space-separated decimal literals), split
purely by FUNCTION — no width/scale anywhere (inputs are width-agnostic; the gate
derives every (width, scale) cell from each input). A `//` comment line sets the
WHY for every following input until the next comment — functional intent only
("near-zero directed-rounding band", "regression: retired exp_underflow.rs pin"),
carried by the generator into the `.au` per-line provenance comment.

A `#precision=<digits>` line sets the GENERATION PRECISION for every following
input until the next such line, scoped exactly like the `//` why above it;
`#precision=default` restores the command line's `--precision`. Other `#` lines
stay comments. The override exists because a few adversarial inputs are only
GRADABLE when generated deeper than the set's default: where the true value sits
just under a storage grid line, its digits run 9 from the storage LSB down to the
deciding term, and any generation precision landing INSIDE that run rounds up and
carries back onto the grid line, destroying the very evidence the row was built to
carry (see the never_exact block in `lead/exp.pb`). Deeper is always safe for the
harness — it tests `len(frac) >= gen_precision` — so the file header stays at the
default and only the marked rows go deeper.

Inputs are deduped by value (first why wins) and filtered to the function's
domain; a line whose field count does not match the function's arity is skipped
with a warning."""
import sys
from pathlib import Path
from typing import List, Optional, Tuple

from .functions import FUNCTIONS

# The why attached to inputs that precede any comment line.
DEFAULT_WHY = "coverage"

# Block-scoped generation-precision override; `default` clears it.
PRECISION_DIRECTIVE = "#precision="


def harvest(func: str, lead_dir: Path) -> List[Tuple[List[str], str, Optional[int]]]:
    """`(inputs, why, precision)` for every in-domain case in `<lead_dir>/<func>.pb`.

    `precision` is the block's `#precision=` override, or `None` to use the
    caller's default.
    """
    f = FUNCTIONS[func]
    path = Path(lead_dir) / f"{func}.pb"
    if not path.exists():
        return []
    seen = set()
    out: List[Tuple[List[str], str, Optional[int]]] = []
    why: str = DEFAULT_WHY
    precision: Optional[int] = None
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line:
            continue
        if line.startswith("#"):
            if line.startswith(PRECISION_DIRECTIVE):
                arg = line[len(PRECISION_DIRECTIVE):].strip()
                precision = None if arg == "default" else int(arg)
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
        out.append((fields, why, precision))
    return out
