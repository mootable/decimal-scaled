"""Harvest the per-function input set from the existing golden set
(tests/golden/<func>_d*_s*.txt). Those files store inputs as RAW scaled integers at
the scale S encoded in the filename; convert each back to a canonical decimal
string (value = raw / 10^S), dedupe by value, keep only in-domain inputs."""
import re
from decimal import Decimal
from pathlib import Path
from typing import List

from .functions import FUNCTIONS

_SCALE_RE = re.compile(r"_s(\d+)")


def _canon(d: Decimal) -> str:
    s = format(d, "f")
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return s if s not in ("", "-", "-0") else "0"


def harvest(func: str, golden_dir: Path) -> List[List[str]]:
    f = FUNCTIONS[func]
    seen = set()
    out: List[List[str]] = []
    for path in sorted(golden_dir.glob(f"{func}_d*_s*.txt")):
        m = _SCALE_RE.search(path.name)
        if not m:
            continue
        denom = Decimal(10) ** int(m.group(1))
        for line in path.read_text().splitlines():
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            fields = line.split("\t") if "\t" in line else line.split()
            if len(fields) < f.arity:
                continue
            try:
                inputs = [_canon(Decimal(fields[i]) / denom) for i in range(f.arity)]
            except Exception:
                continue
            key = tuple(inputs)
            if key in seen or not f.in_domain(inputs):
                continue
            seen.add(key)
            out.append(inputs)
    return out
