"""Cross-check every committed constant golden against the flint/Arb oracle.

Reads the in-tree `src/consts/table.rs` — i.e. the actual bytes the crate
compiles — and independently recomputes every golden mantissa with Arb's
rigorous ball arithmetic.

This is a standing regrade, not a one-off. It grades each golden TWICE:

  PRIMARY  the identity the generator uses (imported from
           `gen_const_table`, so the committed bytes are checked against
           the generator that claims to produce them);

  SECOND   a DIFFERENT identity for the same constant, written here —
           `4*atan(1)` for pi, `1/exp(-1)` for e, `2*cos(pi/5)` for the
           golden ratio, `2*atanh(1/3)` for ln2, `ln2+ln5` for ln10,
           `1-log10(5)` for log10(2), and the pi-derived constants routed
           through `atan(1)` rather than Arb's own pi.

A precision that is too low is caught by the determination itself, which
refuses to emit an undetermined digit. A WRONG IDENTITY is not: it would
be determined, and confidently wrong. That is what SECOND is for.

Neither failure is repaired here; this script only grades.

Run from the repository root:

    python scripts/check_const_against_flint.py
"""

from __future__ import annotations

import re
import sys

from flint import arb, fmpq

from gen_const_table import CONST_CLASS, CONSTS, W_NARROW, WORKING_NARROW
from tang_flint_oracle import (
    IndeterminateSlot,
    decimal_prec_for,
    determine_decimal_floor,
)

GOLDEN_RE = re.compile(
    r"const (\w+)_GOLDEN: &\[u64\] = &\[(.*?)\n\];",
    re.DOTALL,
)
HEX_RE = re.compile(r"0x([0-9a-f]{16})")

# ── The second derivation: a different identity per constant ─────────────
#
# `arb(1).atan()` is pi/4, computed by Arb's inverse-tangent rather than by
# its pi constant, so every pi-derived entry below reaches pi along a
# different route than the generator does.
QUARTER_PI = lambda: arb(1).atan()                        # noqa: E731

SECOND = {
    "pi": ("4*atan(1)", lambda: QUARTER_PI() * 4),
    "tau": ("8*atan(1)", lambda: QUARTER_PI() * 8),
    "half_pi": ("2*atan(1)", lambda: QUARTER_PI() * 2),
    "quarter_pi": ("atan(1)", QUARTER_PI),
    "e": ("1/exp(-1)", lambda: 1 / arb(-1).exp()),
    "golden": ("2*cos(pi/5)", lambda: (QUARTER_PI() * 4 / 5).cos() * 2),
    "ln2": ("2*atanh(1/3)", lambda: arb(fmpq(1, 3)).atanh() * 2),
    "ln10": ("ln2+ln5", lambda: arb(2).log() + arb(5).log()),
    "log10_2": ("1-log10(5)", lambda: 1 - arb(5).log() / arb(10).log()),
    "deg_per_rad": ("45/atan(1)", lambda: arb(45) / QUARTER_PI()),
    "rad_per_deg": ("atan(1)/45", lambda: QUARTER_PI() / 45),
}


def parse_goldens(path: str) -> dict:
    """Return `{"<CONST>_<BAND>": golden_int}` from the generated table."""
    with open(path, encoding="utf-8") as f:
        src = f.read()
    out = {}
    for name, body in GOLDEN_RE.findall(src):
        limbs = [int(h, 16) for h in HEX_RE.findall(body)]
        # stored little-endian, narrowest fit
        n = 0
        for i, limb in enumerate(limbs):
            n |= limb << (64 * i)
        out[name] = n
    assert out, f"{path}: no golden mantissas parsed"
    return out


def bands_for(name: str):
    """The four `(band, gp)` pairs for a constant — mirrors the generator."""
    base_max, xw_max, xxw_max = CONST_CLASS[name]
    narrow_max = WORKING_NARROW if name in ("pi", "ln2", "ln10") else W_NARROW
    return [
        ("NARROW", narrow_max + 1),
        ("BASE", base_max + 1),
        ("XW", xw_max + 1),
        ("XXW", xxw_max + 1),
    ]


def main() -> int:
    goldens = parse_goldens("src/consts/table.rs")
    primary = dict(CONSTS)

    checked = determined = 0
    mismatch = []
    undetermined = []
    escalated = []
    worst = (float("inf"), "", 0)

    for name, _ in CONSTS:
        for band, gp in bands_for(name):
            key = f"{name.upper()}_{band}"
            want = goldens.get(key)
            if want is None:
                mismatch.append((key, None, None, "absent from table.rs"))
                continue
            second_label, second_fn = SECOND[name]
            for which, fn in (("primary", primary[name]),
                              ("second", second_fn)):
                checked += 1
                label = f"{name} {band} [{which}]"
                try:
                    got, spare, prec = determine_decimal_floor(fn, gp, label)
                except IndeterminateSlot as e:
                    undetermined.append((label, str(e)))
                    continue
                determined += 1
                if prec != decimal_prec_for(gp):
                    escalated.append((label, prec))
                if spare < worst[0]:
                    worst = (spare, label, gp)
                if got != want:
                    ident = (second_label if which == "second"
                             else "generator identity")
                    mismatch.append((
                        label, want, got,
                        f"{ident}, determined at prec={prec} bits",
                    ))

    print(f"goldens in table.rs : {len(goldens)}")
    print(f"derivations checked : {checked}"
          "   (each golden twice: generator identity + a second identity)")
    print(f"determined by arb   : {determined}")
    print(f"undetermined        : {len(undetermined)}")
    print(f"precision raised    : {len(escalated)}")
    print(f"MISMATCHES          : {len(mismatch)}")
    if worst[1]:
        print(f"tightest derivation : {worst[1]} ({worst[2]} digits) — its "
              f"enclosure could be 2^{worst[0]:.0f} times wider and still "
              "determine every retained digit")

    for label, prec in escalated:
        print(f"  RAISED PRECISION {label}: determined only at {prec} bits")
    for label, why in undetermined:
        print(f"  UNDETERMINED {label}: {why}")
    for label, want, got, note in mismatch:
        print()
        print(f"  MISMATCH {label}   ({note})")
        print(f"    in tree   : {want}")
        print(f"    flint/arb : {got}")

    return 1 if (mismatch or undetermined) else 0


if __name__ == "__main__":
    sys.exit(main())
