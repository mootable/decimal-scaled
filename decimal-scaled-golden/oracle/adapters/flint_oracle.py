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
        "sqrt": lambda: a.sqrt(),
        # REAL cube root (sign-preserving; Arb's root is for the non-negative branch).
        "cbrt": lambda: (-((-a).root(3)) if a < flint.arb(0) else a.root(3)),
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
        "atan2": lambda: flint.arb.atan2(x[0], x[1]), "powf": lambda: x[0] ** x[1],
    }
    if func not in table:
        raise NotImplementedError(f"flint adapter does not implement {func}")
    return table[func]()


# Digits beyond `precision` used to decide termination: an all-zero guard means the
# value terminated within `precision` and is stripped (marking it exact); otherwise it
# is a genuine truncation. Matches the mpmath/decimal oracles' contract.
GUARD = 40


def _format(neg: bool, scaled_guard: int, precision: int) -> str:
    """Signed `digits.digits` string from `scaled_guard = floor(|value| *
    10^(precision+GUARD))` — pinned by Arb's rigorous interval, so an exact result is
    pinned to the true integer (no point-float floor-one-below artifact). A value
    terminating within `precision` digits is stripped to mark it exact; otherwise it is
    truncated toward zero to exactly `precision` digits."""
    sign = "-" if neg else ""
    if scaled_guard % (10 ** GUARD) == 0:
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
    scaled = scaled_guard // (10 ** GUARD)
    if scaled == 0:
        sign = ""  # never render a signed zero (-0.000…0)
    if precision == 0:
        return f"{sign}{scaled}"
    s = str(scaled).rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


class FlintOracle(Oracle):
    def name(self) -> str:
        return "flint"

    def supports(self, func: str) -> bool:
        # only what _eval_flint implements
        return func in FUNCTIONS and func in (
            _PROOF | {"cbrt", "hypot", "log", "atan2", "powf", "add", "sub", "mul", "div"}
        )

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        try:
            import flint  # lazy
        except ImportError as e:
            raise OracleUnavailable("flint: python-flint not installed") from e
        # Budget the working precision by the RESULT's magnitude, not the input's:
        # exp2/exp of a large argument produce a many-integer-digit result that
        # needs that many EXTRA significant digits to pin `precision` fractional
        # digits (e.g. exp2(299) ≈ 10^90). Gauge the magnitude with a cheap first
        # pass, then size the precision and retry (doubling) until the Arb ball
        # pins a unique integer.
        # Pin to precision+GUARD digits; the GUARD digits decide termination.
        scale = precision + GUARD
        flint.ctx.prec = int((scale + 80) * 3.3219281) + 128
        r = _eval_flint(flint, func, [flint.arb(s) for s in inputs])
        result_digits = 1
        ar = abs(r)
        if ar > flint.arb(0):
            result_digits = max(1, int(float(ar.log() / flint.arb(10).log())) + 2)
        bits_base = int((scale + result_digits + 60) * 3.3219281) + 128
        last = None
        for attempt in range(10):
            flint.ctx.prec = bits_base << attempt
            r = _eval_flint(flint, func, [flint.arb(s) for s in inputs])
            mag = abs(r) * (flint.arb(10) ** scale)
            z = mag.floor().unique_fmpz()
            if z is None:
                # The floor straddles an integer boundary: either the value IS that
                # integer exactly (exp2 of an integer = 2^n), or it sits a
                # sub-resolution distance to one side. Accepting the ball's pinned
                # integer is correct ONLY in the exact case — for a sub-resolution
                # DEFICIT (tanh(1e-k) = 1e-k - v^3/3 with the residual below any
                # working window) it would round the truncation UP a digit, masking
                # the true 9-run. So the pinned integer is accepted only when the
                # candidate is PROVABLY exact; otherwise keep escalating precision
                # until the floor itself resolves on the true side.
                z = mag.unique_fmpz()
                # Numeric proof first (owner 2026-06-11): a zero-radius zero
                # residual ball proves the value IS the pinned integer; the
                # theorem check stays as the backstop for exact values whose
                # ball stayed inexact.
                if z is not None and not (
                    (mag - flint.arb(int(z))).is_zero()
                    or _provably_exact_candidate(func, inputs, r < 0, int(z), scale)
                ):
                    last = "floor straddles a boundary on a provably-inexact value"
                    z = None
            if z is not None:
                return _format(r < 0, int(z), precision)
            last = last or "ball not tight enough"
        raise RuntimeError(f"flint: could not pin {func}{inputs} to {precision} digits ({last})")


def _provably_exact_candidate(func, inputs, neg, scaled, scale):
    """True when the boundary-straddling candidate `(-1)^neg * scaled / 10^scale`
    (`scale` = the pinning scale, i.e. precision + GUARD) is provably the exact
    result (theorem / rational inverse-check)."""
    from ..exactness import _provably_exact

    sign = "-" if neg and scaled != 0 else ""
    s = str(scaled)
    if scale == 0:
        value = f"{sign}{s}"
    else:
        s = s.rjust(scale + 1, "0")
        value = f"{sign}{s[:-scale]}.{s[-scale:]}"
    try:
        return _provably_exact(func, inputs, value)
    except (ValueError, ZeroDivisionError, OverflowError):
        return False


register("flint", FlintOracle)
