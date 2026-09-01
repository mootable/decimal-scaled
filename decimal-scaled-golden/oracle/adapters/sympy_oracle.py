"""sympy validator oracle (BSD). NOTE: sympy's evalf uses mpmath, so it is NOT a
fully independent cross-check of an mpmath generator -- prefer flint/mpfr for
independence. Lazy-imports sympy."""
from typing import List

from ..functions import FUNCTIONS
from ..oracle import GUARD, Oracle, OracleUnavailable, format_fetched, register


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
            # sympy has no expm1/log1p; the inputs are exact Rationals, so the
            # composed expressions are exact and evalf resolves the cancellation.
            "expm1": lambda: sympy.exp(x[0]) - 1, "log1p": lambda: sympy.log(1 + x[0]),
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
        # Wide enough for `precision` + the shared termination GUARD, plus slack.
        mp.dps = precision + GUARD + 60
        r = mpmath.mpf(str(sympy.N(expr, precision + GUARD + 50)))
        # The shared primitive: floor toward zero at `precision` + GUARD. Previously
        # this floored at `precision` with no guard window, so this oracle had no
        # exactness path and rendered every value as a full-length truncation.
        scaled_guard = int(mpmath.floor(abs(r) * (mpmath.mpf(10) ** (precision + GUARD))))
        return format_fetched(r < 0, scaled_guard, precision)


register("sympy", SympyOracle)
