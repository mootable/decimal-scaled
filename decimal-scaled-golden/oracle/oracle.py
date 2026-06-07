"""Oracle interface + registry.

Available oracles (each usable as GENERATOR or VALIDATOR -- same interface; the
role is set by config in oracles.json, not by the oracle):
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

    @abstractmethod
    def supports(self, func: str) -> bool: ...

    @abstractmethod
    def value(self, func: str, inputs: List[str], precision: int) -> str:
        """The function value as a signed `digits.digits` string, truncated toward
        zero to `precision` fractional digits."""
        ...


_REGISTRY: Dict[str, Callable[[], Oracle]] = {}


def register(name: str, factory: Callable[[], Oracle]) -> None:
    _REGISTRY[name] = factory


def get_oracle(name: str) -> Oracle:
    if name not in _REGISTRY:
        raise OracleUnavailable(f"unknown oracle '{name}' (known: {sorted(_REGISTRY)})")
    return _REGISTRY[name]()


def available() -> List[str]:
    return sorted(_REGISTRY)
