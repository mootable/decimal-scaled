"""Function registry mirroring the Rust `Function` enum (names from
`Function::name()`). Each entry: arity + an in-domain predicate over the input
strings (decimal.Decimal, never float).

Binary input order (provisional for now; the unary functions used by the proof
corpus are unambiguous): log=[x, base]; atan2=[y, x]; powf=[base, exp];
hypot=[a, b]; add/sub/mul/div/rem=[a, b].
"""
from dataclasses import dataclass
from decimal import Decimal
from typing import Callable, List


@dataclass(frozen=True)
class Func:
    name: str
    arity: int
    in_domain: Callable[[List[str]], bool]


def _d(s: str) -> Decimal:
    return Decimal(s)


def _all(_i: List[str]) -> bool: return True
def _nonneg(i: List[str]) -> bool: return _d(i[0]) >= 0
def _pos(i: List[str]) -> bool: return _d(i[0]) > 0
def _unit_closed(i: List[str]) -> bool: return Decimal(-1) <= _d(i[0]) <= Decimal(1)
def _ge1(i: List[str]) -> bool: return _d(i[0]) >= 1
def _unit_open(i: List[str]) -> bool: return Decimal(-1) < _d(i[0]) < Decimal(1)
def _y_nonzero(i: List[str]) -> bool: return _d(i[1]) != 0
def _log_dom(i: List[str]) -> bool: return _d(i[0]) > 0 and _d(i[1]) > 0
def _powf_dom(i: List[str]) -> bool: return _d(i[0]) >= 0


FUNCTIONS = {f.name: f for f in [
    Func("sqrt", 1, _nonneg), Func("cbrt", 1, _all), Func("exp", 1, _all),
    Func("ln", 1, _pos), Func("log2", 1, _pos), Func("log10", 1, _pos),
    Func("exp2", 1, _all),
    Func("sin", 1, _all), Func("cos", 1, _all), Func("tan", 1, _all),
    Func("atan", 1, _all), Func("asin", 1, _unit_closed), Func("acos", 1, _unit_closed),
    Func("sinh", 1, _all), Func("cosh", 1, _all), Func("tanh", 1, _all),
    Func("asinh", 1, _all), Func("acosh", 1, _ge1), Func("atanh", 1, _unit_open),
    Func("log", 2, _log_dom), Func("atan2", 2, _all), Func("powf", 2, _powf_dom),
    Func("hypot", 2, _all), Func("add", 2, _all), Func("sub", 2, _all),
    Func("mul", 2, _all), Func("div", 2, _y_nonzero), Func("rem", 2, _y_nonzero),
]}
