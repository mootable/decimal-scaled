"""flint validator oracle.

NOTICE: this adapter is OUR code (MIT/Apache). It calls the user-installed LGPL
package `python-flint` (wrapping FLINT/Arb) at arm's length via its public API at
runtime -- a "work that uses the Library" (LGPL section 5), NOT a derivative.
`python-flint` is NOT bundled; install it yourself: `pip install python-flint`.
"""
from typing import List

from ..functions import FUNCTIONS
from ..oracle import Oracle, OracleUnavailable, register

_PROOF = {"sqrt", "exp", "ln", "log2", "log10", "exp2", "sin", "cos", "tan",
          "atan", "asin", "acos", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh"}


def _eval_flint(flint, func: str, x):
    a = x[0]
    table = {
        "sqrt": lambda: a.sqrt(), "cbrt": lambda: a.root(3),
        "exp": lambda: a.exp(), "ln": lambda: a.log(),
        "log2": lambda: a.log() / flint.arb(2).log(),
        "log10": lambda: a.log() / flint.arb(10).log(),
        "exp2": lambda: (a * flint.arb(2).log()).exp(),
        "sin": lambda: a.sin(), "cos": lambda: a.cos(), "tan": lambda: a.tan(),
        "atan": lambda: a.atan(), "asin": lambda: a.asin(), "acos": lambda: a.acos(),
        "sinh": lambda: a.sinh(), "cosh": lambda: a.cosh(), "tanh": lambda: a.tanh(),
        "asinh": lambda: a.asinh(), "acosh": lambda: a.acosh(), "atanh": lambda: a.atanh(),
        "hypot": lambda: (x[0] * x[0] + x[1] * x[1]).sqrt(),
        "log": lambda: x[0].log() / x[1].log(),
        "add": lambda: x[0] + x[1], "sub": lambda: x[0] - x[1],
        "mul": lambda: x[0] * x[1], "div": lambda: x[0] / x[1],
    }
    if func not in table:
        raise NotImplementedError(f"flint adapter does not implement {func}")
    return table[func]()


def _format(neg: bool, scaled: int, precision: int) -> str:
    sign = "-" if neg and scaled != 0 else ""
    s = str(scaled)
    if precision == 0:
        return f"{sign}{s}"
    s = s.rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


class FlintOracle(Oracle):
    def name(self) -> str:
        return "flint"

    def supports(self, func: str) -> bool:
        # only what _eval_flint implements
        return func in FUNCTIONS and func in (_PROOF | {"hypot", "log", "add", "sub", "mul", "div"})

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        try:
            import flint  # lazy
        except ImportError as e:
            raise OracleUnavailable("flint: python-flint not installed") from e
        bits_base = int((precision + 40) * 3.3219281) + 64
        last = None
        for attempt in range(5):
            flint.ctx.prec = bits_base + attempt * (bits_base // 2)
            x = [flint.arb(s) for s in inputs]
            r = _eval_flint(flint, func, x)
            neg = r < 0
            mag = abs(r) * (flint.arb(10) ** precision)
            fl = mag.floor()
            z = fl.unique_fmpz()        # the unique integer in the ball, or None if not pinned
            if z is None:
                last = "ball not tight enough"
                continue
            return _format(neg, int(z), precision)
        raise RuntimeError(f"flint: could not pin {func}{inputs} to {precision} digits ({last})")


register("flint", FlintOracle)
