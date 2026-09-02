"""Cross-check every committed Tang table slot against the flint/Arb oracle.

Reads the in-tree `src/algos/support/*_tang_table.rs` tables — i.e. the
actual bytes the crate compiles — and independently recomputes every slot
with Arb's rigorous ball arithmetic.

This is a standing regrade, not a one-off: it re-derives the constants from
the mathematical definitions rather than from the generators, so it catches a
hand-edited table, a generator bug, and a stale regeneration alike.

For each slot it reports whether Arb's enclosure DETERMINES the retained
digits, and whether the determined value matches what is committed. A
mismatch is a defect in a baked constant; an undetermined slot is a finding
about the precision policy. Neither is repaired here: this script only
grades.

Run from the repository root:

    python scripts/check_tang_against_flint.py
"""

from __future__ import annotations

import re
import sys

from flint import arb, fmpq

from tang_flint_oracle import (
    ORACLE_PREC_BITS,
    IndeterminateSlot,
    determine,
    set_precision,
)

B_LIMBS = 112
B = B_LIMBS * 64

STATIC_RE = re.compile(
    r"pub\(crate\) static (\w+): \[\[u64; \d+\]; (\d+)\] = \[(.*?)\n\];",
    re.DOTALL,
)
HEX_RE = re.compile(r"0x([0-9a-f]{16})")


def parse_table(path: str) -> dict:
    """Return `{static_name: [slot_int, ...]}` from a generated table file."""
    with open(path, encoding="utf-8") as f:
        src = f.read()
    out = {}
    for name, count, body in STATIC_RE.findall(src):
        limbs = [int(h, 16) for h in HEX_RE.findall(body)]
        count = int(count)
        assert len(limbs) == count * B_LIMBS, (
            f"{path}:{name}: got {len(limbs)} limbs, expected {count * B_LIMBS}"
        )
        slots = []
        for s in range(count):
            chunk = limbs[s * B_LIMBS:(s + 1) * B_LIMBS]
            # emitted MOST-significant limb first
            n = 0
            for limb in chunk:
                n = (n << 64) | limb
            slots.append(n)
        out[name] = slots
    assert out, f"{path}: no slot arrays parsed"
    return out


# ── The four value definitions, mirroring each generator ──────────────────

# Each yields `(label, thunk)`; the thunk computes the value at whatever
# working precision the oracle is currently using, so it can be recomputed
# if the oracle has to escalate. `None` marks a structural zero slot.

def ln_values(count: int):
    """L_i = ln(1 + i/128); i = 0 is exactly 0."""
    m = 128
    for i in range(count):
        yield f"ln i={i}", (None if i == 0 else
                            (lambda i=i: arb(fmpq(m + i, m)).log()))


def exp_values(count: int):
    """F_j = exp(j·ln2/512) − 1; j = 0 is exactly 0."""
    m = 512
    for j in range(count):
        yield f"exp j={j}", (None if j == 0 else
                             (lambda j=j: (arb.const_log2() * j / m).exp()
                              - arb(1)))


def atan_values(count: int):
    """A_j = atan(j/512); j = 0 is exactly 0."""
    m = 512
    for j in range(count):
        yield f"atan j={j}", (None if j == 0 else
                              (lambda j=j: arb(fmpq(j, m)).atan()))


def sincos_values(count: int, which: str):
    """sin/cos of c_j = j·π/(4·512); j = 0 is stored all-zero."""
    m = 512

    def val(j, which):
        c_j = arb.pi() * j / (4 * m)
        return c_j.sin() if which == "sin" else c_j.cos()

    for j in range(count):
        yield f"{which} j={j}", (None if j == 0 else
                                 (lambda j=j: val(j, which)))


def grade(label_values, committed: list, results: dict) -> None:
    for (label, thunk), want in zip(label_values, committed):
        results["checked"] += 1
        if thunk is None:
            # Structural zero slot; nothing for the oracle to determine.
            if want != 0:
                results["mismatch"].append((label, want, 0, "structural zero"))
            continue
        try:
            got, spare, prec = determine(thunk, B, label)
        except IndeterminateSlot as e:
            results["undetermined"].append((label, str(e)))
            continue
        results["determined"] += 1
        if prec != ORACLE_PREC_BITS:
            results["escalated"].append((label, prec))
        # Track the TIGHTEST margin seen: the entry that came closest to
        # being undecidable is the one that bounds the whole table's claim.
        if spare < results["worst_spare"][0]:
            results["worst_spare"] = (spare, label)
        if got != want:
            results["mismatch"].append(
                (label, want, got,
                 f"determined at prec={prec} bits, headroom 2^{spare:.0f}")
            )


def main() -> int:
    set_precision()
    base = "src/algos/support/"
    results = {
        "checked": 0,
        "determined": 0,
        "mismatch": [],
        "undetermined": [],
        "escalated": [],
        "worst_spare": (float("inf"), ""),
    }

    ln = parse_table(base + "ln_tang_table.rs")["LN_TANG_SLOTS"]
    grade(ln_values(len(ln)), ln, results)
    print(f"ln      : {len(ln)} slots")

    ex = parse_table(base + "exp_tang_table.rs")["EXP_TANG_SLOTS"]
    grade(exp_values(len(ex)), ex, results)
    print(f"exp     : {len(ex)} slots")

    at = parse_table(base + "atan_tang_table.rs")["ATAN_TANG_SLOTS"]
    grade(atan_values(len(at)), at, results)
    print(f"atan    : {len(at)} slots")

    sc = parse_table(base + "sincos_tang_table.rs")
    sin_slots = sc["SINCOS_TANG_SIN"]
    cos_slots = sc["SINCOS_TANG_COS"]
    grade(sincos_values(len(sin_slots), "sin"), sin_slots, results)
    grade(sincos_values(len(cos_slots), "cos"), cos_slots, results)
    print(f"sincos  : {len(sin_slots)} sin + {len(cos_slots)} cos slots")

    spare, worst_label = results["worst_spare"]
    print()
    print(f"working precision : {ORACLE_PREC_BITS} bits (retained B = {B})")
    print(f"slots checked     : {results['checked']}")
    print(f"determined by arb : {results['determined']}"
          "   (lower and upper bounds floor to the same integer)")
    print(f"undetermined      : {len(results['undetermined'])}")
    print(f"precision raised  : {len(results['escalated'])}")
    print(f"MISMATCHES        : {len(results['mismatch'])}")
    if worst_label:
        print(f"tightest entry    : {worst_label} — its enclosure could be "
              f"2^{spare:.0f} times wider and still determine every retained "
              "bit (worst case; all others have more headroom)")

    for label, prec in results["escalated"]:
        print(f"  RAISED PRECISION {label}: determined only at {prec} bits")
    for label, why in results["undetermined"]:
        print(f"  UNDETERMINED {label}: {why}")
    for label, want, got, note in results["mismatch"]:
        print()
        print(f"  MISMATCH {label}")
        print(f"    in tree   : {want:#x}")
        print(f"    flint/arb : {got:#x}")
        print(f"    delta     : {got - want:+d}")
        print(f"    {note}")

    return 1 if (results["mismatch"] or results["undetermined"]) else 0


if __name__ == "__main__":
    sys.exit(main())
