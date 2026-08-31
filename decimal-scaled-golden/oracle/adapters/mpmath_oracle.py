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

# Working digits held beyond what the budget below demands, absorbing mpmath's own
# few-ulp per-function error.
SLACK = 25

# Budget-escalation rounds. A readable condition number lands the budget in one
# step; the spare rounds are for a point whose conditioning the current window
# cannot yet read (see `_condition_digits`).
MAX_ESCALATIONS = 6


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
    if scaled == 0:
        sign = ""  # never render a signed zero (-0.000…0)
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
    # Native, not composed: mpmath.expm1/log1p keep full relative accuracy where
    # `exp(x) - 1` / `ln(1 + x)` would cancel to nothing for tiny arguments.
    "expm1": mpmath.expm1, "log1p": mpmath.log1p,
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


# --- conditioning: how many working digits the function itself destroys --------------
#
# mpmath is a POINT float. Unlike flint's Arb ball — which carries its own error
# bound, notices when precision ran out and escalates — mpmath returns a number with
# no indication of how much of it is meaningful. Its working precision must therefore
# be PREDICTED, and a budget sized only from the result's magnitude predicts it wrong
# wherever the function is ill-conditioned.
#
# A value held to `dps` significant digits carries relative error 10^-dps, i.e.
# absolute error |x|·10^-dps. That is true of the parsed input (a decimal literal is
# not generally a binary float) and equally of every intermediate. The function maps
# it to an absolute output error
#
#     |x · f'(x)| · 10^-dps          multivariate: max_i |x_i · ∂f/∂x_i|
#
# so `A = log10 |x · ∇f(x)|` digits are destroyed between input and output. `A` is a
# property of the DERIVATIVE, which is exactly what diverges at a domain edge — and
# the result's own magnitude says nothing about it. log1p at 1 + t = 1e-70 returns
# ln(1e-70) ≈ -161: three integer digits, while 1/(1 + t) = 1e70 eats seventy. The
# same shortfall hits atanh at both endpoints (1/(1 - x²)) and acosh at 1
# (1/√(x² - 1)) — and, away from any endpoint, sin/cos of a large argument, where
# `A = log10|x|` is precisely the cost of range reduction.
#
# The budget takes the WORSE of two independent lower bounds on dps (never their sum
# — they are alternative requirements, and adding them would double-count sinh of a
# large argument, where the result's magnitude and the derivative's are the same size):
#
#     result representation:  dps ≥ precision + GUARD + int_digits(r)
#     error propagation:      dps ≥ precision + GUARD + A
#
# `A ≤ int_digits(r)` at a well-conditioned point, so for the overwhelming majority of
# values this reproduces the previous budget exactly and costs nothing.
#
# Each entry below is |x · ∇f(x)| for that function, written in terms of the inputs
# and the already-computed result `r` so it needs no second transcendental evaluation.
# Where a closed form would cost one, a strict UPPER bound is used instead: over-
# estimating `A` only buys unneeded digits, under-estimating loses correct ones.
_COND = {
    # roots: x·f' = r/n
    "sqrt": lambda x, r: abs(r) / 2,
    "cbrt": lambda x, r: abs(r) / 3,
    # exponentials: f' = f (times a log factor ≤ 1 for exp2), so x·f' = x·r
    "exp": lambda x, r: abs(x[0] * r),
    "exp2": lambda x, r: abs(x[0] * r),
    "expm1": lambda x, r: abs(x[0] * (r + 1)),
    # logarithms: x·f' = 1/ln(base) — a constant, so a log is perfectly conditioned…
    "ln": lambda x, r: mpmath.mpf(1),
    "log2": lambda x, r: 1 / mpmath.ln(2),
    "log10": lambda x, r: 1 / mpmath.ln(10),
    # …except log1p, whose argument 1 + t cancels as t → -1. Sterbenz makes that sum
    # exact for t ∈ [-1, -0.5], so the estimate stays sharp exactly where it matters.
    "log1p": lambda x, r: abs(x[0] / (1 + x[0])),
    # circular: cos = ±√(1 - sin²) and vice versa, so both come free from r. For a
    # large argument this returns ~|x| — the range-reduction cost, correctly charged.
    "sin": lambda x, r: abs(x[0]) * mpmath.sqrt(abs(1 - r * r)),
    "cos": lambda x, r: abs(x[0]) * mpmath.sqrt(abs(1 - r * r)),
    "tan": lambda x, r: abs(x[0] * (1 + r * r)),
    # inverse circular: atan is flat, asin/acos diverge at ±1. Factoring 1 - x² as
    # (1 - x)(1 + x) keeps the near-endpoint subtraction exact (Sterbenz again).
    "atan": lambda x, r: abs(x[0] / (1 + x[0] * x[0])),
    "asin": lambda x, r: abs(x[0]) / mpmath.sqrt((1 - x[0]) * (1 + x[0])),
    "acos": lambda x, r: abs(x[0]) / mpmath.sqrt((1 - x[0]) * (1 + x[0])),
    # hyperbolic: cosh = √(1 + sinh²), sinh = √(cosh² - 1) — again free from r.
    "sinh": lambda x, r: abs(x[0]) * mpmath.sqrt(1 + r * r),
    "cosh": lambda x, r: abs(x[0]) * mpmath.sqrt(abs(r * r - 1)),
    "tanh": lambda x, r: abs(x[0] * (1 - r * r)),
    # inverse hyperbolic: asinh is flat; acosh diverges at 1 and atanh at ±1 — the
    # two endpoints that motivated this budget.
    "asinh": lambda x, r: abs(x[0]) / mpmath.sqrt(1 + x[0] * x[0]),
    "acosh": lambda x, r: abs(x[0]) / mpmath.sqrt((x[0] - 1) * (x[0] + 1)),
    "atanh": lambda x, r: abs(x[0]) / abs((1 - x[0]) * (1 + x[0])),
    # binary. add/sub/rem: ∂/∂a = 1, so the larger OPERAND sets the cost — which is
    # how a cancelling sum charges for the digits it cancels away.
    "add": lambda x, r: max(abs(x[0]), abs(x[1])),
    "sub": lambda x, r: max(abs(x[0]), abs(x[1])),
    "rem": lambda x, r: max(abs(x[0]), abs(x[1])),
    # mul/div/hypot: both partials come back to the result's own magnitude.
    "mul": lambda x, r: abs(r),
    "div": lambda x, r: abs(r),
    "hypot": lambda x, r: abs(r),
    # atan2: |x·y|/(x² + y²) ≤ 1/2 either way — always well conditioned.
    "atan2": lambda x, r: mpmath.mpf(1),
    # powf(a,b) = a^b: a·∂/∂a = b·r, b·∂/∂b = b·r·ln a.
    "powf": lambda x, r: abs(x[1] * r) * (max(1, abs(mpmath.ln(x[0]))) if x[0] > 0 else 1),
    # log(x, base): x·∂/∂x = 1/ln(base), base·∂/∂base = -r/ln(base).
    "log": lambda x, r: max(1, abs(r)) / abs(mpmath.ln(x[1])),
}


def _measure_cond(func, x, r):
    """|x · ∇f(x)| MEASURED, for a function with no `_COND` entry: one relative
    perturbation per input, over a step far above the rounding floor and far below
    the point itself. A first difference reads the derivative without knowing it, so
    a newly added function is conditioned-for automatically rather than silently
    inheriting the old magnitude-only budget."""
    h = mpmath.mpf(10) ** (-(mpmath.mp.dps // 2))
    worst = mpmath.mpf(0)
    for i in range(len(x)):
        if x[i] == 0:
            continue  # an exact zero carries no relative error to amplify
        y = list(x)
        y[i] = x[i] * (1 + h)
        worst = max(worst, abs((_eval(func, y) - r) / h))
    return worst


# log10(2), for turning mpmath's binary magnitude into decimal digits.
_LOG10_2 = 0.30102999566398120


def _decimal_digits(v):
    """The decimal digits left of `v`'s point (0 for |v| < 1), from the mpf's BINARY
    EXPONENT rather than a logarithm.

    `mag` returns an m with |v| <= 2^m, so m*log10(2) overshoots by at most two
    digits; stepping back down against exact powers of ten recovers the same count
    `floor(log10|v|) + 1` would give, for two cheap comparisons instead of a full
    working-precision transcendental evaluation. That matters more than it looks:
    the budget is computed for EVERY value, and on the cheap operations (add, mul,
    div) one 1233-digit `log10` costs several times the operation it is sizing.

    Exactness here is not just thrift. The budget picks the working precision, and
    mpmath is a point float whose rendering of an exactly-representable decimal can
    flip either side of the true value as that precision moves (the "floor one
    below" artifact flint's rigorous ball is immune to). A digit of slop in this
    count would perturb dps, and with it the terminate-vs-truncate marker, on values
    that were never ill-conditioned at all."""
    if v == 0:
        return 0
    digits = max(0, int(mpmath.mag(v) * _LOG10_2) + 1)
    while digits > 0 and abs(v) < mpmath.mpf(10) ** (digits - 1):
        digits -= 1
    return digits


def _int_digits(r):
    """Integer digits of `r` — the significant digits the result's own magnitude
    consumes before the fractional ones the caller asked for can begin."""
    return 1 if r == 0 else _decimal_digits(r)


def _condition_digits(func, x, r):
    """`ceil(log10 |x · ∇f(x)|)` — the working digits this function destroys at this
    point — clamped at 0, since a well-conditioned point costs nothing.

    `None` means the conditioning could not be read AT THIS WORKING PRECISION: the
    derivative expression overflowed, or its own cancellation bottomed out. That is
    itself evidence of severe ill-conditioning, so the caller escalates and re-reads
    rather than trusting a number it could not compute."""
    try:
        amplification = _COND[func](x, r) if func in _COND else _measure_cond(func, x, r)
        amplification = abs(mpmath.mpf(amplification))
    except (ZeroDivisionError, ValueError, OverflowError, NotImplementedError):
        return None
    if not mpmath.isfinite(amplification):
        return None
    return _decimal_digits(amplification)


class MpmathOracle(Oracle):
    def name(self) -> str:
        return "mpmath"

    def supports(self, func: str) -> bool:
        return func in FUNCTIONS

    def value(self, func: str, inputs: List[str], precision: int) -> str:
        # Size the working precision to the WORSE of the result's own magnitude and
        # the function's conditioning here, then confirm at that width — the second
        # reading is taken with the derivative resolved, so a point whose steepness
        # was invisible in the first window is caught before the value is returned.
        dps = precision + GUARD + 30
        previous = None
        for _ in range(MAX_ESCALATIONS):
            mpmath.mp.dps = dps
            x = [mpmath.mpf(s) for s in inputs]
            r = _eval(func, x)
            if not mpmath.isfinite(r):
                # Too narrow to hold the value at all: an argument cancelled to zero
                # (log1p of a t whose 1 + t falls below the window). Widen and retry.
                dps, previous = dps * 2, None
                continue
            amplification = _condition_digits(func, x, r)
            if amplification is None:
                # The conditioning cannot be read at this width, which is itself a
                # sign of a steep point — so widen rather than trust it. But accept a
                # value that stops moving across a widening: that is what an exact
                # answer at an infinite-derivative point does (acosh(1) = 0,
                # asin(±1) = ±π/2), where no finite budget would ever settle.
                current = _format(r, precision)
                if current == previous:
                    return current
                dps, previous = dps * 2, current
                continue
            need = precision + GUARD + SLACK + max(_int_digits(r), amplification)
            if dps >= need:
                return _format(r, precision)
            dps, previous = need, None
        raise RuntimeError(
            f"mpmath: could not settle a working precision for {func}{inputs} "
            f"(reached dps={dps})"
        )


register("mpmath", MpmathOracle)
