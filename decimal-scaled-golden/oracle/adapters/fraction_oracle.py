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
from ..oracle import Oracle, register

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


def _format(r: Fraction, precision: int) -> str:
    """Signed `digits.digits` string with the same terminated-vs-truncated contract as
    the mpmath oracle, but decided EXACTLY: a value terminates within `precision`
    fractional digits iff its reduced denominator divides 10^precision (it is then
    stored stripped of trailing zeros); otherwise it is truncated toward zero to
    exactly `precision` digits."""
    sign = "-" if r < 0 else ""
    ar = -r if r < 0 else r
    num, den = ar.numerator, ar.denominator
    pow10 = 10 ** precision
    if pow10 % den == 0:
        # Terminates within `precision`: exact = ar * 10^precision (an integer).
        exact = num * (pow10 // den)
        if exact == 0:
            return "0"
        z = 0
        while z < precision and exact % 10 == 0:
            exact //= 10
            z += 1
        frac_len = precision - z
        if frac_len == 0:
            return f"{sign}{exact}"
        s = str(exact).rjust(frac_len + 1, "0")
        return f"{sign}{s[:-frac_len]}.{s[-frac_len:]}"
    # Non-terminating (or terminating deeper than `precision`): truncate toward zero.
    scaled = (num * pow10) // den
    if scaled == 0:
        sign = ""  # never render a signed zero (-0.000…0)
    if precision == 0:
        return f"{sign}{scaled}"
    s = str(scaled).rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


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
        return _format(_eval(func, a, b), precision)


register("fraction", FractionOracle)
