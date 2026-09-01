"""Shared rigorous `flint`/Arb oracle for the baked Tang tables.

The four Tang generators — `gen_ln_tang_table.py`, `gen_exp_tang_table.py`,
`gen_atan_tang_table.py`, `gen_sincos_tang_table.py` — each bake the
correctly-rounded binary fixed-point

    slot = round(value · 2^B)

of a value in `[0, 1)`. They share ONE determination routine, `determine`
below, so they cannot drift apart in how a retained bit is decided.

## Why Arb rather than a binary float library

Arb is an interval (ball) arithmetic library: every value carries a proven
enclosure `midpoint ± radius`, and every operation propagates that enclosure
rigorously. So the retained digits are not merely *believed* correct at some
working precision — the library itself certifies that no value in the
enclosure would round differently. `round_scaled` refuses to emit a slot
whose digits the enclosure does not pin down; it never widens the tolerance,
retries at a lower bar, or falls back to another library.

## Working precision

The Tang tables retain `B = 7168` bits. `ORACLE_PREC_BITS` is set far beyond
that so the enclosure of `value · 2^B` is tighter than one unit by thousands
of bits, leaving the rounding decidable for every entry. That starting
precision is only a HINT: the determination is what proves the bits, and it
escalates until it can prove them or gives up loudly.
"""

from __future__ import annotations

import math
import sys

from flint import arb, ctx

# ── Working precision ────────────────────────────────────────────────────
#
# `B = 7168` bits are retained. At 16384 bits of working precision the
# enclosure of `value · 2^B` is narrower than one unit in the last retained
# place by roughly `16384 − 7168 = 9216` bits, so `floor(value · 2^B + 1/2)`
# is determined with an enormous margin. (For comparison the superseded
# mpmath generators ran at `mp.dps = 2600`, about 8637 bits, and carried no
# error bound at all.)
ORACLE_PREC_BITS = 16384

# Ceiling for precision escalation. If an entry's bounds still straddle an
# integer boundary here, the entry is UNDETERMINED and that is reported as a
# finding — never rounded past.
MAX_PREC_BITS = 262144


class IndeterminateSlot(RuntimeError):
    """An Arb enclosure was too wide to determine a slot's retained digits.

    Raised rather than handled: an undetermined slot is a finding about the
    precision policy, not something to paper over.
    """


def set_precision() -> None:
    """Set the global Arb working precision. Call once at generator start."""
    ctx.prec = ORACLE_PREC_BITS


def scaled_ball(val: arb, b: int) -> arb:
    """Return the enclosure of `val · 2^B + 1/2` whose floor is the slot.

    `2^B` is a power of two, so scaling by it only shifts the exponent of an
    Arb ball — it is exact, and adding the exactly-representable `1/2` is
    exact too at this working precision. The returned ball is therefore no
    wider than the enclosure of `val` itself, scaled.
    """
    return val * arb(2) ** b + arb(0.5)


def determine(make_val, b: int, label: str):
    """Determine `floor(val · 2^B + 1/2)` and PROVE it from Arb's bounds.

    Returns `(n, spare_bits, prec_used)`.

    `make_val` is a CALLABLE returning the value as an `arb` at the ambient
    working precision — not a precomputed ball, because escalating the
    precision requires recomputing the value, not merely re-examining an
    enclosure that was already fixed at the old precision.

    The determination criterion is the explicit one: the enclosure's LOWER
    and UPPER bounds must floor to the same integer, i.e. the interval lies
    strictly inside `[n, n+1)` and so agrees at the full retained precision.
    `spare_bits` is `log2` of the distance from the interval to the nearest
    integer boundary — the margin by which the entry is decided.

    If the bounds disagree we ESCALATE the working precision (announcing it
    on stderr — never silently) and recompute. Only if the entry is still
    undetermined at `MAX_PREC_BITS` do we raise `IndeterminateSlot`.
    """
    prec = ORACLE_PREC_BITS
    while True:
        ctx.prec = prec
        ball = scaled_ball(make_val(), b)
        lo_n = ball.lower().floor().unique_fmpz()
        hi_n = ball.upper().floor().unique_fmpz()
        if lo_n is not None and hi_n is not None and int(lo_n) == int(hi_n):
            n = int(lo_n)
            if not 0 <= n < (1 << b):
                raise IndeterminateSlot(f"{label}: slot {n} out of {b}-bit range")
            # Safety factor, in bits: how much WIDER the enclosure could be
            # before this entry stopped being decidable. Subtracting the
            # exact integer keeps the residuals small enough to inspect.
            #
            #   margin = distance from the interval to the nearest integer
            #            boundary, in units of the last retained bit
            #   radius = half the enclosure's width, same units
            #
            # The entry is determined because radius < margin; the ratio is
            # the headroom. Reporting the margin alone would be misleading:
            # it is always below 1/2 by construction.
            rel = ball - arb(n)
            lo_gap = rel.lower()
            hi_gap = arb(1) - rel.upper()
            margin = lo_gap if lo_gap < hi_gap else hi_gap
            return n, _safety_bits(margin, rel.rad()), prec
        if prec >= MAX_PREC_BITS:
            raise IndeterminateSlot(
                f"{label}: Arb bounds still disagree at prec={prec} bits "
                f"(B={b}); the enclosure spans an integer boundary, so the "
                f"retained digits are NOT determined. Bounds "
                f"[{ball.lower()}, {ball.upper()}]"
            )
        prec *= 2
        print(
            f"  NOTE {label}: bounds disagreed at the retained precision; "
            f"escalating working precision to {prec} bits",
            file=sys.stderr,
        )


def _log2(x: arb):
    """Exact `log2` of a non-negative EXACT `arb`, or `None` if it is zero.

    Deliberately NOT via `float()`: these quantities run to around `2^-9216`,
    thousands of orders of magnitude below the smallest double, so converting
    would flush them to zero and silently report infinite headroom. `man_exp`
    gives the mantissa and exponent as exact integers instead.
    """
    man, exp = x.man_exp()
    man, exp = int(man), int(exp)
    if man <= 0:
        return None
    return exp + math.log2(man)


def _safety_bits(margin: arb, radius: arb) -> float:
    """Bits of headroom: `log2(margin / radius)`.

    `+inf` when the enclosure is exact (zero radius). A value of `N` means
    the enclosure could be `2^N` times wider and the entry would still be
    determined; `<= 0` would mean it was decided only barely.
    """
    log_radius = _log2(radius)
    if log_radius is None:
        return float("inf")
    log_margin = _log2(margin)
    if log_margin is None:
        return float("-inf")
    return log_margin - log_radius


def round_scaled(make_val, b: int, label: str) -> int:
    """Return `floor(val · 2^B + 1/2)`, PROVEN correct by Arb's bounds.

    Round-half-up matches the historical generators; the values are
    irrational, so no exact tie can occur and the result equals
    round-to-nearest.
    """
    n, _spare, _prec = determine(make_val, b, label)
    return n


def limbs_msb_first(n: int, b_limbs: int, label: str) -> list:
    """Split `n` into `b_limbs` u64 limbs, MOST-SIGNIFICANT limb first.

    The stored layout is little-endian magnitude but emitted MS-limb first so
    a narrower tier can read a contiguous HIGH-limb PREFIX as a free slice.
    """
    le = []
    x = n
    for _ in range(b_limbs):
        le.append(x & 0xFFFFFFFFFFFFFFFF)
        x >>= 64
    if x != 0:
        raise IndeterminateSlot(f"{label}: value exceeds {b_limbs} limbs")
    return list(reversed(le))


def slot_limbs(make_val, b: int, b_limbs: int, label: str) -> list:
    """`round(val · 2^B)` as a `[u64; b_limbs]` MS-limb-first magnitude.

    `make_val` is a callable returning the value at the ambient precision, so
    the oracle can recompute it if it has to escalate. See `determine`.
    """
    return limbs_msb_first(round_scaled(make_val, b, label), b_limbs, label)
