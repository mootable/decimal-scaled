"""mpfr validator oracle.

NOTICE: this adapter is OUR code (MIT/Apache). It calls the user-installed LGPL
package `gmpy2` (wrapping MPFR/GMP) at arm's length via its public API at runtime
-- a "work that uses the Library" (LGPL section 5), NOT a derivative. `gmpy2` is
NOT bundled; install it yourself: `pip install gmpy2`.
"""
from typing import List

from ..functions import FUNCTIONS
from ..oracle import Oracle, OracleUnavailable, register


def _format(neg: bool, scaled: int, precision: int) -> str:
    sign = "-" if neg and scaled != 0 else ""
    s = str(scaled)
    if precision == 0:
        return f"{sign}{s}"
    s = s.rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


class MpfrOracle(Oracle):
    def name(self) -> str:
        return "mpfr"

    def supports(self, func: str) -> bool:
        return func in FUNCTIONS and func not in ("rem",)

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        try:
            import gmpy2  # lazy
        except ImportError as e:
            raise OracleUnavailable("mpfr: gmpy2 not installed") from e
        ctx = gmpy2.get_context()
        ctx.precision = int((precision + 40) * 3.3219281) + 64
        x = [gmpy2.mpfr(s) for s in inputs]
        a = x[0]
        table = {
            "sqrt": lambda: gmpy2.sqrt(a), "cbrt": lambda: gmpy2.cbrt(a),
            "exp": lambda: gmpy2.exp(a), "ln": lambda: gmpy2.log(a),
            "log2": lambda: gmpy2.log2(a), "log10": lambda: gmpy2.log10(a),
            "exp2": lambda: gmpy2.exp2(a),
            "sin": lambda: gmpy2.sin(a), "cos": lambda: gmpy2.cos(a), "tan": lambda: gmpy2.tan(a),
            "atan": lambda: gmpy2.atan(a), "asin": lambda: gmpy2.asin(a), "acos": lambda: gmpy2.acos(a),
            "sinh": lambda: gmpy2.sinh(a), "cosh": lambda: gmpy2.cosh(a), "tanh": lambda: gmpy2.tanh(a),
            "asinh": lambda: gmpy2.asinh(a), "acosh": lambda: gmpy2.acosh(a), "atanh": lambda: gmpy2.atanh(a),
            "log": lambda: gmpy2.log(x[0]) / gmpy2.log(x[1]), "atan2": lambda: gmpy2.atan2(x[0], x[1]),
            "powf": lambda: x[0] ** x[1], "hypot": lambda: gmpy2.hypot(x[0], x[1]),
            "add": lambda: x[0] + x[1], "sub": lambda: x[0] - x[1], "mul": lambda: x[0] * x[1],
            "div": lambda: x[0] / x[1],
        }
        if func not in table:
            raise NotImplementedError(f"mpfr adapter does not implement {func}")
        r = table[func]()
        neg = r < 0
        scaled = int(gmpy2.floor(abs(r) * gmpy2.mpfr(10) ** precision))
        return _format(neg, scaled, precision)


register("mpfr", MpfrOracle)
