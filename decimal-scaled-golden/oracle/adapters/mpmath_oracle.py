"""mpmath generator oracle (BSD). Computes each function to arbitrary precision
and returns a plain signed `digits.digits` string. A value that TERMINATES within
`precision` fractional digits is written stripped of its trailing zeros (so it has
fewer than `precision` frac digits, marking it exact); a non-terminating value is
truncated TOWARD ZERO to exactly `precision` frac digits. Preserving the
terminated-vs-truncated distinction is essential — the consumer's tie detection
(HalfToEven, Ceiling/Floor) depends on whether a residual exists below the stored
digits."""
from typing import List

import mpmath

from ..functions import FUNCTIONS
from ..oracle import Oracle, register

# Extra fractional digits computed beyond `precision` to decide termination: if
# they are all zero, the value terminated within `precision` and is stripped;
# otherwise it is a genuine truncation. A coincidental run of this many zeros in a
# non-terminating expansion is ~10^-GUARD — negligible.
GUARD = 40


def _format(r, precision: int) -> str:
    sign = "-" if r < 0 else ""
    scaled_guard = int(mpmath.floor(abs(r) * (mpmath.mpf(10) ** (precision + GUARD))))
    if scaled_guard % (10 ** GUARD) == 0:
        # Terminated within `precision` digits: strip trailing zeros.
        exact = scaled_guard // (10 ** GUARD)  # value * 10^precision, exact
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
    # Non-terminating: truncate toward zero to exactly `precision` frac digits.
    scaled = scaled_guard // (10 ** GUARD)
    if precision == 0:
        return f"{sign}{scaled}"
    s = str(scaled).rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


_UNARY = {
    "sqrt": mpmath.sqrt,
    # REAL cube root (mpmath.cbrt takes the complex principal branch for x<0).
    "cbrt": lambda x: (-mpmath.cbrt(-x) if x < 0 else mpmath.cbrt(x)),
    "exp": mpmath.exp, "ln": mpmath.ln,
    "log2": lambda x: mpmath.log(x, 2), "log10": lambda x: mpmath.log(x, 10),
    "exp2": lambda x: mpmath.power(2, x),
    "sin": mpmath.sin, "cos": mpmath.cos, "tan": mpmath.tan,
    "atan": mpmath.atan, "asin": mpmath.asin, "acos": mpmath.acos,
    "sinh": mpmath.sinh, "cosh": mpmath.cosh, "tanh": mpmath.tanh,
    "asinh": mpmath.asinh, "acosh": mpmath.acosh, "atanh": mpmath.atanh,
}


def _eval(func: str, x):
    if func in _UNARY:
        return _UNARY[func](x[0])
    if func == "log":   return mpmath.log(x[0], x[1])    # log base x[1] of x[0]
    if func == "atan2": return mpmath.atan2(x[0], x[1])  # atan2(y, x)
    if func == "powf":  return mpmath.power(x[0], x[1])
    if func == "hypot": return mpmath.hypot(x[0], x[1])
    if func == "add":   return x[0] + x[1]
    if func == "sub":   return x[0] - x[1]
    if func == "mul":   return x[0] * x[1]
    if func == "div":   return x[0] / x[1]
    if func == "rem":
        # Rust truncated remainder (sign follows the dividend), NOT mpmath.fmod,
        # which floors (result sign follows the divisor). Matches decimal-scaled's `%`.
        q = x[0] / x[1]
        tq = mpmath.floor(q) if q >= 0 else mpmath.ceil(q)
        return x[0] - x[1] * tq
    raise ValueError(f"unknown function {func}")


class MpmathOracle(Oracle):
    def name(self) -> str:
        return "mpmath"

    def supports(self, func: str) -> bool:
        return func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        mpmath.mp.dps = precision + GUARD + 30
        x = [mpmath.mpf(s) for s in inputs]
        r = _eval(func, x)
        # ensure working precision covers the RESULT's integer part + the guard
        if r != 0:
            int_digits = max(0, int(mpmath.floor(mpmath.log10(abs(r)))) + 1)
        else:
            int_digits = 1
        need = precision + int_digits + GUARD + 25
        if mpmath.mp.dps < need:
            mpmath.mp.dps = need
            x = [mpmath.mpf(s) for s in inputs]
            r = _eval(func, x)
        return _format(r, precision)


register("mpmath", MpmathOracle)
