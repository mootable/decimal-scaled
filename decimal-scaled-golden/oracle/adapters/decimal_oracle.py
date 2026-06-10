"""Exact / base-10 oracle on Python's stdlib `decimal.Decimal` (PSF licence).

`Decimal` is a base-10 arbitrary-precision engine, so it is an INDEPENDENT radix
cross-check on the binary oracles (mpmath/flint) — and the generator for the four
functions it computes correctly-rounded natively (`sqrt`, `exp`, `ln`, `log10`).

Coverage:
  * native (correctly-rounded): sqrt, exp, ln, log10, and the arithmetic ops.
  * derived (composed from exp/ln/sqrt at high guard precision): cbrt, log2, exp2,
    log, sinh, cosh, tanh, asinh, acosh, atanh, hypot, powf. The composition is NOT
    correctly-rounded to the last digit, so these are used as a VALIDATOR only (the
    radix-bound tolerance absorbs the sub-ULP composition error), never a generator.
  * unsupported: the circular trig sin/cos/tan/atan/asin/acos/atan2 — `Decimal` has
    no trig and we do not hand-roll a series; those stay binary-only.

Output is the same signed `digits.digits` string contract as the other oracles: a
value terminating within `precision` fractional digits is stripped (marking it
exact); otherwise it is truncated toward zero to exactly `precision` digits."""
from decimal import Decimal, localcontext, ROUND_HALF_EVEN
from typing import List

from ..functions import FUNCTIONS
from ..oracle import Oracle, register

# Digits computed beyond `precision` to decide termination (all-zero guard ⇒ exact),
# plus the slack the derived compositions need so their accumulated rounding stays far
# below the stored depth (a handful of ULP from ~3-5 chained ops — 60 digits is ample).
TERM_GUARD = 40
WORK_GUARD = 60

_SUPPORTED = {
    "sqrt", "exp", "ln", "log10",                       # native, correctly-rounded
    "cbrt", "log2", "exp2", "log",                       # derived
    "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",   # derived
    "hypot", "powf",                                     # derived
    "add", "sub", "mul", "div", "rem",                   # native arithmetic
}


def _eval(func: str, xs: List[Decimal]) -> Decimal:
    a = xs[0]
    b = xs[1] if len(xs) > 1 else None
    two = Decimal(2)
    if func == "add":
        return a + b
    if func == "sub":
        return a - b
    if func == "mul":
        return a * b
    if func == "div":
        return a / b
    if func == "rem":
        # Rust truncated remainder: sign follows the dividend (int() truncates toward
        # zero), matching decimal-scaled's `%`.
        return a - b * Decimal(int(a / b))
    if func == "sqrt":
        return a.sqrt()
    if func == "exp":
        return a.exp()
    if func == "ln":
        return a.ln()
    if func == "log10":
        return a.log10()
    if func == "log2":
        return a.ln() / two.ln()
    if func == "exp2":
        return (a * two.ln()).exp()
    if func == "cbrt":
        if a == 0:
            return Decimal(0)
        mag = (abs(a).ln() / 3).exp()
        return -mag if a < 0 else mag
    if func == "log":  # log base b of a
        return a.ln() / b.ln()
    if func == "sinh":
        return (a.exp() - (-a).exp()) / two
    if func == "cosh":
        return (a.exp() + (-a).exp()) / two
    if func == "tanh":
        e2 = (a * two).exp()
        return (e2 - 1) / (e2 + 1)
    if func == "asinh":
        # asinh is odd; compute on |x| so the two terms are both positive (x + sqrt
        # would catastrophically cancel for large negative x, losing ~2*log10|x| digits).
        ax = -a if a < 0 else a
        mag = (ax + (ax * ax + 1).sqrt()).ln()
        return -mag if a < 0 else mag
    if func == "acosh":
        return (a + (a * a - 1).sqrt()).ln()
    if func == "atanh":
        return ((1 + a) / (1 - a)).ln() / two
    if func == "hypot":
        return (a * a + b * b).sqrt()
    if func == "powf":  # base a (>= 0), exponent b
        if a == 0:
            return Decimal(1) if b == 0 else Decimal(0)
        return (b * a.ln()).exp()
    raise ValueError(f"decimal oracle does not handle {func}")


def _format(r: Decimal, precision: int) -> str:
    sign = "-" if r < 0 else ""
    # scaled_guard = floor(|r| * 10^(precision+TERM_GUARD)), exact (×10^k shifts the
    # exponent; int() truncates toward zero).
    scaled_guard = int(abs(r).scaleb(precision + TERM_GUARD))
    if scaled_guard % (10 ** TERM_GUARD) == 0:
        exact = scaled_guard // (10 ** TERM_GUARD)  # value * 10^precision, exact
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
    scaled = scaled_guard // (10 ** TERM_GUARD)
    if precision == 0:
        return f"{sign}{scaled}"
    s = str(scaled).rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


class DecimalOracle(Oracle):
    def name(self) -> str:
        return "decimal"

    def radix(self) -> str:
        return "decimal"

    def supports(self, func: str) -> bool:
        return func in _SUPPORTED and func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        xs = [Decimal(s) for s in inputs]
        # Two-pass precision: a generous first pass to read the result's magnitude,
        # then size the working precision so `precision` fractional digits — plus the
        # termination guard and the composition slack — are all valid even for a
        # many-integer-digit result (exp/exp2/cosh/powf of a large argument).
        base = precision + TERM_GUARD + WORK_GUARD
        with localcontext() as ctx:
            ctx.prec = base
            ctx.rounding = ROUND_HALF_EVEN
            r = _eval(func, xs)
            int_digits = r.adjusted() + 1 if r != 0 else 1
            need = precision + max(0, int_digits) + TERM_GUARD + WORK_GUARD
            if ctx.prec < need:
                ctx.prec = need
                r = _eval(func, xs)
            return _format(r, precision)


register("decimal", DecimalOracle)
