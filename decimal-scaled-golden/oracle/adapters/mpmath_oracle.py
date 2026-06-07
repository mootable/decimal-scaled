"""mpmath generator oracle (BSD). Computes each function to arbitrary precision
and returns the value truncated TOWARD ZERO to `precision` fractional digits, as a
plain signed `digits.digits` string."""
from typing import List

import mpmath

from ..functions import FUNCTIONS
from ..oracle import Oracle, register


def _truncated_digits(r, precision: int) -> str:
    sign = "-" if r < 0 else ""
    scaled = int(mpmath.floor(abs(r) * (mpmath.mpf(10) ** precision)))
    s = str(scaled)
    if precision == 0:
        return f"{sign}{s}"
    s = s.rjust(precision + 1, "0")
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
    if func == "rem":   return mpmath.fmod(x[0], x[1])
    raise ValueError(f"unknown function {func}")


class MpmathOracle(Oracle):
    def name(self) -> str:
        return "mpmath"

    def supports(self, func: str) -> bool:
        return func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        mpmath.mp.dps = precision + 60
        x = [mpmath.mpf(s) for s in inputs]
        r = _eval(func, x)
        # ensure working precision covers the RESULT's integer part too
        if r != 0:
            int_digits = max(0, int(mpmath.floor(mpmath.log10(abs(r)))) + 1)
        else:
            int_digits = 1
        need = precision + int_digits + 25
        if mpmath.mp.dps < need:
            mpmath.mp.dps = need
            x = [mpmath.mpf(s) for s in inputs]
            r = _eval(func, x)
        return _truncated_digits(r, precision)


register("mpmath", MpmathOracle)
