"""flint validator oracle.

NOTICE: this adapter is OUR code (MIT/Apache). It calls the user-installed LGPL
package `python-flint` (wrapping FLINT/Arb) at arm's length via its public API at
runtime -- a "work that uses the Library" (LGPL section 5), NOT a derivative.
`python-flint` is NOT bundled; install it yourself: `pip install python-flint`.
"""
from typing import List

from ..functions import FUNCTIONS
from ..oracle import GUARD, Oracle, OracleUnavailable, format_fetched, register

_PROOF = {"sqrt", "exp", "ln", "log2", "log10", "exp2", "expm1", "log1p",
          "sin", "cos", "tan",
          "atan", "asin", "acos", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh"}

# Functions this oracle may CHECK but must never GENERATE. See `can_generate`/`_rem`.
_VALIDATE_ONLY = {"rem"}


def _decimal_ratio(flint, s: str):
    """A plain decimal literal as an EXACT `(numerator, denominator)` pair of `fmpz`.

    No binary intermediary: the digits become an integer and the point becomes a power of
    ten, so nothing is approximated on the way in.
    """
    neg = s.startswith("-")
    ip, _, fp = s.lstrip("+-").partition(".")
    num = flint.fmpz(int((ip + fp) or "0"))
    return (-num if neg else num), flint.fmpz(10) ** len(fp)


def _rem_exact(flint, sa: str, sb: str):
    """Rust truncated remainder of two exact decimal literals, as an exact `fmpz` ratio.

    `fraction` GENERATES `rem`; this lets flint CHECK that answer instead of leaving it
    cross-checked only by oracles that cannot pin. It deliberately does NOT go through Arb:
    `rem` needs `trunc(a/b)`, and at an exact multiple the true quotient IS an integer, so
    every ball around it contains points below that integer and `floor` never resolves
    however far precision is pushed. Integers have no such boundary, so this answers every
    row rather than abstaining on the ones that matter most.

    `fmpz`'s `%` is FLOORED — `fmpz(-17) % fmpz(7)` is `4`. The crate's `%`, and
    `fraction_oracle`, are TRUNCATED, with the sign following the dividend, so `-17 % 7` is
    `-3`. Pulling one divisor back whenever the remainder's sign disagrees with the
    dividend's converts one to the other, over all four sign combinations.

    Note what this is and is not as a check. It is a second exact implementation on a
    different bignum backend, so it catches a scaling, parsing or sign-convention mistake
    on either side. It is not an independent METHOD — it agrees with `fraction` by
    construction when both are right.
    """
    na, da = _decimal_ratio(flint, sa)
    nb, db = _decimal_ratio(flint, sb)
    # Put both over the common denominator da*db; the remainder is then integer-only.
    dividend, divisor = na * db, nb * da
    r = dividend % divisor
    if r != 0 and (r < 0) != (dividend < 0):
        r -= divisor
    return r, da * db


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
        # Arb exposes no expm1/log1p, so both are composed. The ball stays
        # RIGOROUS through the composition — `exp(x)` has magnitude ~1 for a small
        # x, so subtracting the exact 1 leaves the ABSOLUTE radius unchanged, and
        # `value()` pins against the absolute scale 10^-(precision+GUARD). Relative
        # accuracy is what cancels, and the pin does not depend on it.
        "expm1": lambda: a.exp() - flint.arb(1),
        "log1p": lambda: (flint.arb(1) + a).log(),
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


class FlintOracle(Oracle):
    def name(self) -> str:
        return "flint"

    def supports(self, func: str) -> bool:
        # only what _eval_flint implements
        return func in FUNCTIONS and func in (
            _PROOF | {"cbrt", "hypot", "log", "atan2", "powf",
                      "add", "sub", "mul", "div", "rem"}
        )

    def can_generate(self, func: str) -> bool:
        # `rem` is validate-only here: it is discontinuous at exact multiples, where an
        # interval around the true quotient never resolves however far precision is
        # pushed, so flint can confirm `fraction`'s answer but must not be its source.
        # See `_rem`.
        return func not in _VALIDATE_ONLY and self.supports(func)

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        try:
            import flint  # lazy
        except ImportError as e:
            raise OracleUnavailable("flint: python-flint not installed") from e
        if func == "rem":
            # Exact integer path — no ball, nothing to escalate, and no boundary to
            # straddle. See `_rem_exact`.
            r, den = _rem_exact(flint, inputs[0], inputs[1])
            neg = r < 0
            mag = -r if neg else r
            return format_fetched(
                neg, int((mag * flint.fmpz(10) ** (precision + GUARD)) // den), precision)
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
                # `int(z)` IS the shared primitive: floor(|value| * 10^(precision+GUARD)),
                # pinned by Arb's rigorous interval rather than approximated.
                return format_fetched(r < 0, int(z), precision)
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
