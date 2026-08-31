"""Check the mpmath oracle's working-precision budget against flint.

`python -m oracle.conditioning_check`

The mpmath adapter predicts how much precision a value needs (it is a point float,
so unlike flint's Arb ball it cannot notice for itself when precision ran out). That
prediction rests on a per-function condition number, `|x . grad f(x)|`, tabulated in
`adapters/mpmath_oracle.py`. A wrong or missing entry does not fail loudly: it
returns a number with silently corrupted tail digits, which then shows up only as a
cross-oracle disagreement that DROPS golden lines.

So this drives each function DOWN its ill-conditioning axis -- toward the domain edge
where its derivative diverges, or out along the argument where range reduction bites
-- and asserts mpmath still agrees with flint on terms the generator would accept.
Depths run well past anything in `lead/`, because the point is to catch the budget
breaking before an input that provokes it is ever committed.

Run it after changing the budget, after adding a `_COND` entry, or after adding a
function. flint is optional; without it there is nothing to compare against and the
check reports itself skipped.
"""
import sys

from .generate import ACCEPT_ULPS, GEN_PRECISION, _approx_mag, _diff_scaled, _within_bound
from .oracle import OracleUnavailable, get_oracle
# import adapters so they register themselves:
from .adapters import flint_oracle, mpmath_oracle  # noqa: F401


def _near_one(k):
    """1 - 10^-k, as an exact decimal literal: k nines."""
    return "0." + "9" * k


def _above_one(k):
    """1 + 10^-k."""
    return "1." + "0" * (k - 1) + "1"


def _large(k):
    """A k-integer-digit argument that is deliberately NOT a round power of ten, so
    it is not exactly representable in binary and its representation error is real."""
    return "1234567" + "8" * (k - 7) + ".7654321"


# Each probe names the axis it walks and why the budget is under stress there. The
# depths are the amplification in decimal digits, which is what the budget must add.
_PROBES = [
    # Endpoint asymptotes: the derivative diverges, so the input's representation
    # error is amplified by 10^depth. These are the sites from issue #66.
    ("log1p -> -1+   (f' = 1/(1+t))", "log1p", lambda k: [f"-{_near_one(k)}"], (70, 120, 400, 1200)),
    ("atanh -> +1    (f' = 1/(1-x^2))", "atanh", lambda k: [_near_one(k)], (70, 120, 400, 1200)),
    ("atanh -> -1", "atanh", lambda k: [f"-{_near_one(k)}"], (70, 120, 400)),
    # acosh/asin/acos carry a SQUARE-ROOT singularity, so the amplification is half
    # the approach depth -- the check that the budget follows the derivative rather
    # than merely counting the input's digits.
    ("acosh -> 1+    (f' = 1/sqrt(x^2-1))", "acosh", lambda k: [_above_one(2 * k)], (70, 120, 400)),
    ("asin -> 1      (f' = 1/sqrt(1-x^2))", "asin", lambda k: [_near_one(2 * k)], (70, 400)),
    ("acos -> 1", "acos", lambda k: [_near_one(2 * k)], (70, 400)),
    # No endpoint here at all: for sin/cos/tan the same term is the cost of range
    # reduction, A = log10|x|. Nothing in lead/ is this large, which is exactly why
    # it needs a guard -- category 3 of the hard-input classes invites such inputs.
    ("sin large argument", "sin", lambda k: [_large(k)], (80, 200, 600)),
    ("cos large argument", "cos", lambda k: [_large(k)], (80, 200, 600)),
    ("tan large argument", "tan", lambda k: [_large(k)], (80, 200, 600)),
    # Growth axis rather than a singularity: the result's own magnitude already
    # covered these, so they guard against the budget REGRESSING.
    ("exp large argument", "exp", lambda k: [str(k)], (100, 900)),
    ("sinh large argument", "sinh", lambda k: [str(k)], (100, 900)),
    ("cancelling sum", "add", lambda k: [_above_one(k), "-1"], (80, 400)),
    ("cancelling difference", "sub", lambda k: [_above_one(k), "1"], (80, 400)),
]

# Inputs sitting exactly ON a derivative pole. No finite condition number exists, so
# the budget can never settle by prediction; the adapter must fall back to accepting
# a value that has stopped moving as precision widens.
_POLES = [
    ("acosh", ["1"]), ("asin", ["1"]), ("asin", ["-1"]),
    ("acos", ["1"]), ("acos", ["-1"]), ("atanh", ["0"]),
    ("sqrt", ["0"]), ("cbrt", ["0"]), ("cosh", ["0"]),
]


def main(argv=None):
    precision = int(argv[0]) if argv else GEN_PRECISION
    try:
        flint = get_oracle("flint")
    except OracleUnavailable as e:
        print(f"skipped: no independent oracle to compare against ({e})")
        return 0
    mpmath_o = get_oracle("mpmath")

    failures = 0
    print(f"conditioning check at {precision} digits, tolerance {ACCEPT_ULPS} ulp\n")

    for func, inputs in _POLES:
        try:
            mpmath_o.value(func, inputs, precision)
            print(f"  ok   pole {func}({inputs[0]})")
        except Exception as e:
            failures += 1
            print(f"  FAIL pole {func}({inputs[0]}): {type(e).__name__}: {e}")

    for label, func, build, depths in _PROBES:
        for depth in depths:
            inputs = build(depth)
            try:
                expected = flint.value(func, inputs, precision)
            except Exception as e:
                # flint could not pin this one; it cannot arbitrate, so neither can we.
                print(f"  --   {label} @ {depth}: flint abstained ({type(e).__name__})")
                continue
            try:
                got = mpmath_o.value(func, inputs, precision)
            except Exception as e:
                failures += 1
                print(f"  FAIL {label} @ {depth}: mpmath {type(e).__name__}: {e}")
                continue
            diff, scale = _diff_scaled(expected, got)
            if _within_bound(diff, scale, precision):
                print(f"  ok   {label} @ {depth}")
            else:
                failures += 1
                print(f"  FAIL {label} @ {depth}: delta~{_approx_mag(diff, scale)} "
                      f"> {ACCEPT_ULPS} ulp")

    print(f"\n{failures} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
