"""Exact base-10 oracle on Python's stdlib `fractions.Fraction` (PSF licence) for the
finite-result arithmetic ops only: add, sub, mul, div, rem.

These results are finite/rational decimals, so a BINARY oracle (mpmath/flint/mpfr) is
the wrong tool: an exact decimal like `-0.00000004555500747` has no finite binary form,
so rendering its binary approximation to 1233 digits yields a spurious tail (`...46999...`)
that the termination check then bakes in as a truncation. Fraction arithmetic is exact, so
every result is the true rational: add/sub/mul/rem results always have a finite decimal
expansion (stored stripped to mark them exact when it fits the generation precision; a
deeper-than-precision expansion — e.g. a product of two near-precision-deep operands — is
truncated toward zero like any other oracle value) and div may be genuinely non-terminating.

The transcendentals stay on the binary/base-10 oracles (irrational results; binary to many
guard digits, cross-validated, is correct there). This oracle covers only the five exact
ops and is their ground truth."""
from fractions import Fraction
from typing import List

from ..functions import FUNCTIONS
from ..oracle import GUARD, Oracle, format_fetched, register

_EXACT_OPS = {"add", "sub", "mul", "div", "rem"}


def _eval(func: str, a: Fraction, b: Fraction) -> Fraction:
    if func == "add":
        return a + b
    if func == "sub":
        return a - b
    if func == "mul":
        return a * b
    if func == "div":
        return a / b
    if func == "rem":
        # Rust truncated remainder: the sign follows the dividend (`int()` truncates
        # a Fraction toward zero), NOT a floored modulo. Matches decimal-scaled's `%`.
        tq = int(a / b)
        return a - b * tq
    raise ValueError(f"fraction oracle does not handle {func}")


def _scaled_guard(r: Fraction, precision: int) -> int:
    """`floor(|r| * 10^(precision+GUARD))` — this oracle's formatting primitive, and
    the only one of the six that is EXACT: integer division of exact rationals, with
    no approximation to floor and nothing rounded on the way.

    The shared guard rule agrees with this oracle's algebra: a rational terminates
    within `precision` iff its reduced denominator divides `10^precision`, and then
    every digit below `precision` — the guard digits included — is zero, so the
    all-zero-guard branch fires exactly when it should.
    """
    ar = -r if r < 0 else r
    return (ar.numerator * 10 ** (precision + GUARD)) // ar.denominator


class FractionOracle(Oracle):
    def name(self) -> str:
        return "fraction"

    def radix(self) -> str:
        return "decimal"

    def supports(self, func: str) -> bool:
        return func in _EXACT_OPS and func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        # Fraction parses a decimal string exactly (no binary intermediary).
        a = Fraction(inputs[0])
        b = Fraction(inputs[1])
        r = _eval(func, a, b)
        return format_fetched(r < 0, _scaled_guard(r, precision), precision)


register("fraction", FractionOracle)
