"""Oracle interface + registry.

Available oracles (each usable as GENERATOR or VALIDATOR -- same interface; the
role is set by GENERATOR_POLICY / VALIDATOR_ORDER in generate.py, CLI-overridable,
not by the oracle):
  mpmath  - BSD                          - full coverage   - default generator
  sympy   - BSD                          - cross-check     - validator (wraps mpmath: weak independence)
  flint   - LGPL (python-flint / Arb)    - independent     - strong validator [optional: pip install python-flint]
  mpfr    - LGPL (gmpy2 / MPFR)          - independent     - strong validator [optional: pip install gmpy2]

The flint/mpfr adapters import their LGPL bindings LAZILY and are NOT bundled;
they are 'works that use the Library' (LGPL section 5), not derivatives.
"""
from abc import ABC, abstractmethod
from typing import Callable, Dict, List


class OracleUnavailable(Exception):
    """Raised when an oracle's backing package is not installed."""


class Oracle(ABC):
    @abstractmethod
    def name(self) -> str: ...

    def radix(self) -> str:
        """The oracle's working radix: `"binary"` (mpmath/flint/mpfr — arbitrary
        precision but base 2) or `"decimal"` (exact/decimal — base 10). Recorded in
        a line comment when a binary validator disagrees with a base-10 generator, so
        the radix-rounding difference is auditable."""
        return "binary"

    @abstractmethod
    def supports(self, func: str) -> bool: ...

    def can_generate(self, func: str) -> bool:
        """Whether this oracle may GENERATE `func`, as opposed to merely validating it.

        Defaults to `supports`, because computing a function and being trusted to produce
        the stored answer are usually the same claim. They come apart when an oracle can
        CHECK a value it cannot reliably PIN: flint computes `rem` well enough to confirm
        someone else's answer, but `rem` is discontinuous at exact multiples, where an
        interval around the true quotient never resolves — so it must not be the source of
        that answer. Overriding here keeps that judgement next to the adapter that knows
        why, instead of in a per-function table in the pipeline.
        """
        return self.supports(func)

    @abstractmethod
    def value(self, func: str, inputs: List[str], precision: int) -> str:
        """The function value as a signed `digits.digits` string, truncated toward
        zero to `precision` fractional digits.

        Every oracle implements this by computing `scaled_guard` in its own numeric
        type and handing it to the shared [`format_fetched`] — see the two rules
        documented there. An adapter must NOT format a value itself: six hand-rolled
        copies of one algorithm is what let a generator and its validator share a
        defect and read as agreement."""
        ...


# Digits fetched BEYOND `precision`. They are not stored; they decide termination —
# an all-zero guard means the value terminated within `precision`. Kept generous: the
# window only has to be wider than a run of zeros a non-terminating value can plausibly
# show, and `exactness.enforce_truncation_marker` re-decides every "exact" verdict
# against an algebraic proof, so a false positive here costs nothing but a proof call.
GUARD = 40


def format_fetched(neg: bool, scaled_guard: int, precision: int) -> str:
    """THE fetch contract, in one place, for every oracle.

    Two rules, and nothing else:

    1. **Rounded DOWN to `precision` + [`GUARD`].** `scaled_guard` is
       `floor(|value| * 10^(precision+GUARD))` — the value TRUNCATED toward zero, never
       rounded. Rounding at a working precision can carry across a run of nines and move
       digits the caller is going to keep; flooring cannot.
    2. **An exact value shorter than `precision` returns its own shorter length.** A
       fraction shorter than `precision` IS the exactness signal, which is why the
       guard digits are fetched but not stored.

    `neg` is the value's sign, kept separate so a negative value that truncates to zero
    never renders as `-0.000…0`.

    Each adapter supplies `scaled_guard` from its own numeric type — that primitive is
    the ONLY thing an oracle implements, and it must truncate toward zero rather than
    round. How rigorously an oracle can produce it differs (Arb pins it with a rigorous
    interval; a point float can only approximate it), and that difference is a property
    of the numeric type, not of this contract.
    """
    sign = "-" if neg else ""
    if scaled_guard % (10 ** GUARD) == 0:
        # Rule 2: terminated within `precision`. Strip trailing zeros so the SHORTER
        # length carries the claim. `enforce_truncation_marker` re-decides this against
        # an algebraic proof and pads the value back when it cannot be justified.
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
    # Rule 1: a genuine truncation to exactly `precision` fractional digits.
    scaled = scaled_guard // (10 ** GUARD)
    if scaled == 0:
        sign = ""  # never render a signed zero (-0.000…0)
    if precision == 0:
        return f"{sign}{scaled}"
    s = str(scaled).rjust(precision + 1, "0")
    return f"{sign}{s[:-precision]}.{s[-precision:]}"


_REGISTRY: Dict[str, Callable[[], Oracle]] = {}


def register(name: str, factory: Callable[[], Oracle]) -> None:
    _REGISTRY[name] = factory


def get_oracle(name: str) -> Oracle:
    if name not in _REGISTRY:
        raise OracleUnavailable(f"unknown oracle '{name}' (known: {sorted(_REGISTRY)})")
    return _REGISTRY[name]()


def available() -> List[str]:
    return sorted(_REGISTRY)
