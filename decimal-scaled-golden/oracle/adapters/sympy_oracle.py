"""sympy validator oracle (BSD). NOTE: sympy's evalf uses mpmath, so it is NOT a
fully independent cross-check of an mpmath generator -- prefer flint/mpfr for
independence. Lazy-imports sympy."""
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


class SympyOracle(Oracle):
    def name(self) -> str:
        return "sympy"

    def supports(self, func: str) -> bool:
        return func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        try:
            import sympy  # lazy
        except ImportError as e:
            raise OracleUnavailable("sympy: not installed") from e
        import mpmath
        S = sympy.S
        x = [S(s) for s in inputs]
        table = {
            "sqrt": lambda: sympy.sqrt(x[0]), "cbrt": lambda: sympy.cbrt(x[0]),
            "exp": lambda: sympy.exp(x[0]), "ln": lambda: sympy.log(x[0]),
            "log2": lambda: sympy.log(x[0], 2), "log10": lambda: sympy.log(x[0], 10),
            "exp2": lambda: sympy.Integer(2) ** x[0],
            "sin": lambda: sympy.sin(x[0]), "cos": lambda: sympy.cos(x[0]), "tan": lambda: sympy.tan(x[0]),
            "atan": lambda: sympy.atan(x[0]), "asin": lambda: sympy.asin(x[0]), "acos": lambda: sympy.acos(x[0]),
            "sinh": lambda: sympy.sinh(x[0]), "cosh": lambda: sympy.cosh(x[0]), "tanh": lambda: sympy.tanh(x[0]),
            "asinh": lambda: sympy.asinh(x[0]), "acosh": lambda: sympy.acosh(x[0]), "atanh": lambda: sympy.atanh(x[0]),
            "log": lambda: sympy.log(x[0], x[1]), "atan2": lambda: sympy.atan2(x[0], x[1]),
            "powf": lambda: x[0] ** x[1], "hypot": lambda: sympy.sqrt(x[0] ** 2 + x[1] ** 2),
            "add": lambda: x[0] + x[1], "sub": lambda: x[0] - x[1], "mul": lambda: x[0] * x[1],
            "div": lambda: x[0] / x[1],
        }
        if func not in table:
            raise NotImplementedError(f"sympy adapter does not implement {func}")
        expr = table[func]()
        mp = mpmath.mp
        mp.dps = precision + 60
        r = mpmath.mpf(str(sympy.N(expr, precision + 50)))
        neg = r < 0
        scaled = int(mpmath.floor(abs(r) * (mpmath.mpf(10) ** precision)))
        return _format(neg, scaled, precision)


register("sympy", SympyOracle)
