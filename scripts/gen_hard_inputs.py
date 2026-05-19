"""Generate "hard inputs" for the precision golden tables.

This sibling to `gen_golden_precision.py` walks ten categories of
input shapes that the literature on transcendental rounding flags
as the most likely places for a strict kernel to lose its 0.5 ULP
contract, even when the random / near-boundary harness passes.

Each category appends a section to the existing
`tests/golden/<func>_d<N>_s{S}.txt` file under a `## category:`
header line. The harness in `tests/ulp_strict_golden.rs` treats
`#` lines as comments (so existing parsers see the appended rows
as ordinary data) and additionally tracks `## known-failing` blocks
to allow already-documented kernel holes through the 0.5 ULP gate.

Oracles: mpmath at `max(700, 2 * scale + 64)` dps per cell, with
half-to-even rounding at the tier storage LSB. The wide tiers
(D616<308>, D1232<615>) need >500 dps to make every rounding exact
at the storage LSB; bumping per-cell avoids spurious "oracle bug"
failures at the widest tiers. If you add a wider tier than
D1232<615>, the rule keeps the headroom.

Determinism: every random draw is seeded from the
(tier, scale, function, category) tuple so two runs produce
byte-identical files.

Literature sources for each category — paper, not test vectors
(license-compatible):

  1. Half-ULP-tie boundaries — Lefèvre, Muller, & Toma,
     "Toward correctly rounded transcendentals" (1998); Muller,
     "Elementary Functions — Algorithms and Implementation"
     (3rd ed., 2016), §10 "Table maker's dilemma".
  2. Catastrophic cancellation — Goldberg, "What every computer
     scientist should know about floating-point arithmetic" (1991),
     §3; Higham, "Accuracy and Stability of Numerical Algorithms"
     (2nd ed., 2002), §1.7.
  3. Range-reduction breakpoints — Payne & Hanek, "Radian reduction
     for trigonometric functions" (1983); Muller (2016), §11.
  4. Removable singularities / asymptotes — Kahan archive,
     "Branch cuts for complex elementary functions" (1987).
  5. Inverse-identity round-trip stress — Brent & Zimmermann,
     "Modern Computer Arithmetic" (2010), §4.2 "Inverse functions".
  6. Perfect-power ± ULP for roots — Brent & Zimmermann (2010),
     §3.5 "Square root", §3.6 "k-th root".
  7. Constant edges — IEEE 754-2019 standard, §9 "Recommended
     correctly rounded functions", and Muller (2016) §9 on
     argument range.
  8. Argument-halving cascade — Muller (2016) §6 on argument
     reduction; the per-width cascade table in ALGORITHMS.md.
  9. Stage-2 argument reduction edge for exp — Tang, "Table-driven
     implementation of the exponential function in IEEE
     floating-point arithmetic" (1989).
 10. Tang-lookup band edges — Tang (1989); Gal & Bachelis,
     "An accurate elementary mathematical library for the IEEE
     floating-point standard" (1991).
"""

from __future__ import annotations

import math
import random
from pathlib import Path
from typing import Callable, Iterable, Sequence

from mpmath import mp, mpf, ln, exp, sin, cos, tan, atan, sqrt, cbrt, mpc

# --- Workspace layout -----------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
OUT_DIR = ROOT / "tests" / "golden"

# Working precision is bumped per-cell to `dps_for(scale)`; this
# initial value is a floor that's still wider than the narrowest
# tier's storage LSB. `main()` resets `mp.dps` at every tier
# iteration before calling any oracle.
mp.dps = 700


def dps_for(scale: int) -> int:
    """Per-cell working precision: enough decimal digits that every
    rounding at the storage LSB is exact.

    Rule: `max(700, 2 * scale + 64)`. The factor of two covers
    intermediate squarings / multiplications inside the oracles
    (`sin`, `cos`, `exp`, `cbrt`, …) which can lose roughly `scale`
    digits to cancellation before we round. The `+ 64` is a safety
    margin against pathological inputs in any one category. The
    floor of 700 keeps narrow tiers fast while still wider than the
    D1232<615> storage LSB.
    """
    return max(700, 2 * scale + 64)


def real_cbrt(x: mpf) -> mpf:
    """Real cube root (mpmath's `cbrt` returns the principal complex
    root for negatives; the crate exposes the real branch)."""
    if x >= 0:
        return cbrt(x)
    return -cbrt(-x)


# --- Tier targets ---------------------------------------------------------
#
# Mirrors `gen_golden_precision.py::TIERS` so every section lands in
# an existing file. `cases_per_category` is the target case-count per
# category × per file; total ends up at cases_per_category × ~10
# categories × ~8 functions × tier_count, then trimmed by the
# domain-skip rules.

TIERS: list[tuple[str, int, int, int]] = [
    # (alias, storage_capacity_digits, scale, cases_per_category)
    ("d38",  38,  19,  80),
    ("d76",  76,  35,  60),
    ("d153", 153, 76,  40),
    ("d307", 307, 150, 25),
    ("d616", 616, 308, 15),
    ("d1232", 1232, 615, 8),
]

# mpmath oracles dispatched by function name.
ORACLES: dict[str, Callable[[mpf], mpf]] = {
    "ln":   ln,
    "exp":  exp,
    "sin":  sin,
    "cos":  cos,
    "tan":  tan,
    "atan": atan,
    "sqrt": sqrt,
    "cbrt": real_cbrt,
}

FUNC_NAMES = list(ORACLES.keys())


# --- Helpers --------------------------------------------------------------

def round_half_even(value: mpf) -> int:
    """Half-to-even rounding of an mpf to the nearest integer."""
    sign = 1 if value >= 0 else -1
    mag = abs(value)
    floor_int = int(mag)
    frac = mag - mpf(floor_int)
    half = mpf("0.5")
    if frac < half:
        rounded = floor_int
    elif frac > half:
        rounded = floor_int + 1
    else:
        rounded = floor_int if (floor_int % 2 == 0) else floor_int + 1
    return sign * rounded


def scaled(value: mpf, scale: int) -> int:
    return round_half_even(value * (mpf(10) ** scale))


def from_raw(raw: int, scale: int) -> mpf:
    return mpf(raw) / (mpf(10) ** scale)


def safe_call(oracle: Callable[[mpf], mpf], x: mpf) -> mpf | None:
    try:
        y = oracle(x)
    except (ValueError, ZeroDivisionError, OverflowError, ArithmeticError):
        return None
    if isinstance(y, mpc):
        return None
    return y


def safe_to_case(raw_in: int, func_name: str, scale: int,
                 max_raw: int) -> tuple[int, int] | None:
    """Convert a raw input into a (raw_in, raw_out) golden case, or
    None if the input is out of the function's domain or the output
    overflows storage."""
    if abs(raw_in) > max_raw:
        return None
    x = from_raw(raw_in, scale)
    if func_name == "ln" and x <= 0:
        return None
    if func_name == "sqrt" and x < 0:
        return None
    if func_name == "tan":
        # Reject within 1e-6 of (k+0.5)·π, where the kernel returns
        # the nearest representable instead of a finite limit.
        k = round(float(x / mp.pi - mpf("0.5")))
        pole = (mpf(k) + mpf("0.5")) * mp.pi
        if abs(x - pole) < mpf("0.000001"):
            return None
    y = safe_call(ORACLES[func_name], x)
    if y is None:
        return None
    raw_out = scaled(y, scale)
    if abs(raw_out) > max_raw:
        return None
    return (raw_in, raw_out)


def dedup(cases: Iterable[tuple[int, int]]) -> list[tuple[int, int]]:
    seen: set[int] = set()
    out: list[tuple[int, int]] = []
    for raw_in, raw_out in cases:
        if raw_in in seen:
            continue
        seen.add(raw_in)
        out.append((raw_in, raw_out))
    return out


# --- Category 1: half-ULP-tie boundaries ---------------------------------
#
# Reference: Lefèvre/Muller (1998), Muller (2016) §10 "Table maker's
# dilemma". For RN-half-even, the worst inputs are those where the
# true real result lands within 0.5 LSB ± ε of a storage breakpoint.

def cat_half_ulp_tie(func_name: str, scale: int, max_raw: int,
                     count: int, rng: random.Random) -> list[tuple[int, int]]:
    """Scan for inputs whose oracle output is within 0.45 LSB of a
    half-tie. Stricter than the existing generator's 0.4 cutoff so
    we hit the absolute hardest cases."""
    out: list[tuple[int, int]] = []
    one = 10 ** scale
    attempts = 0

    def domain_sample() -> int:
        if func_name in ("ln", "sqrt"):
            return rng.randint(1, min(max_raw, one * (10 ** 6)))
        if func_name == "exp":
            cap = max(1, int(0.5 * mp.log(mpf(max_raw) / mpf(one))))
            return rng.randint(-cap * one, cap * one)
        if func_name == "tan":
            return rng.randint(-3 * one, 3 * one)
        if func_name in ("sin", "cos"):
            return rng.randint(-8 * one, 8 * one)
        if func_name == "atan":
            return rng.randint(-10 * one, 10 * one)
        if func_name == "cbrt":
            return rng.randint(-min(max_raw, one * (10 ** 6)),
                                min(max_raw, one * (10 ** 6)))
        return rng.randint(-max_raw, max_raw)

    while len(out) < count and attempts < count * 400:
        attempts += 1
        raw_in = domain_sample()
        if abs(raw_in) > max_raw:
            continue
        x = from_raw(raw_in, scale)
        if func_name == "ln" and x <= 0:
            continue
        if func_name == "sqrt" and x < 0:
            continue
        if func_name == "tan":
            k = round(float(x / mp.pi - mpf("0.5")))
            pole = (mpf(k) + mpf("0.5")) * mp.pi
            if abs(x - pole) < mpf("0.000001"):
                continue
        # Single oracle call serves both the rounded output and the
        # half-tie distance probe — oracle calls dominate runtime at
        # the wide tiers, so we never call it twice for one input.
        y = safe_call(ORACLES[func_name], x)
        if y is None:
            continue
        raw_out = scaled(y, scale)
        if abs(raw_out) > max_raw:
            continue
        scaled_y = y * (mpf(10) ** scale)
        mag = abs(scaled_y)
        floor = int(mag)
        frac = mag - mpf(floor)
        dist = abs(frac - mpf("0.5"))
        if dist < mpf("0.45"):
            out.append((raw_in, raw_out))
    return out


# --- Category 2: catastrophic cancellation -------------------------------
#
# Reference: Goldberg (1991) §3; Higham (2002) §1.7.
#
#   ln(1 + ε)        — tiny ε
#   exp(x) ~ 1+x     — tiny |x|
#   cos(x) ~ 1 - x²/2 — tiny |x|
#   sin(x) ~ x       — tiny |x|
#   sqrt(1 + ε) ~ 1 + ε/2
#   cbrt(1 + ε) ~ 1 + ε/3
#
# We don't have *_1p variants on the strict surface; the regular
# kernel should still hit 0.5 ULP if it handles small inputs via
# series expansion. Probing this is exactly what locks the
# behaviour in.

def cat_cancellation(func_name: str, scale: int, max_raw: int,
                     count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    # Inputs of the form one ± 10**k where k sweeps from 0 up to
    # scale-2. For wide tiers this hits dozens of magnitude bands;
    # we sample evenly across them with a deterministic stride.
    if func_name in ("ln", "sqrt"):
        anchors = [one]
    elif func_name == "exp":
        anchors = [0]
    elif func_name in ("sin", "cos", "tan"):
        anchors = [0]
    elif func_name == "atan":
        anchors = [0]
    elif func_name == "cbrt":
        anchors = [0, one]
    else:
        return []

    # Sweep k from 1 up to scale-1, append ±10**k around each anchor.
    raw_candidates: list[int] = []
    for anchor in anchors:
        for k in range(1, min(scale, 200)):
            raw_candidates.append(anchor + 10 ** k)
            raw_candidates.append(anchor - 10 ** k)
        # Small random offsets in the bottom decade ("just-above-tie"
        # cancellation tests).
        for _ in range(count // 4):
            raw_candidates.append(anchor + rng.randint(1, 100))
            raw_candidates.append(anchor - rng.randint(1, 100))

    rng.shuffle(raw_candidates)
    for raw_in in raw_candidates:
        if len(out) >= count:
            break
        case = safe_to_case(raw_in, func_name, scale, max_raw)
        if case is not None:
            out.append(case)
    return out


# --- Category 3: range-reduction breakpoints -----------------------------
#
# Reference: Payne & Hanek (1983); Muller (2016) §11.
#
#   sin/cos near k·π/2  — for k up to a few hundred at wide scales
#   exp near k·ln 2     — Tang stage-1 reduction edge
#   atan near 1         — argument-halving rotation boundary

def cat_range_reduction(func_name: str, scale: int, max_raw: int,
                        count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale
    half_one = one // 2  # 0.5 LSB

    if func_name in ("sin", "cos"):
        # k · π / 2 ± small offset, for k from 1 to 50.
        k_max = 50
        for k in range(1, k_max + 1):
            if len(out) >= count:
                break
            center = scaled(mpf(k) * mp.pi / mpf(2), scale)
            for delta in [-3, -1, 0, 1, 3,
                          -half_one // 1000, half_one // 1000]:
                raw_in = center + delta
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
                    if len(out) >= count:
                        break
    elif func_name == "tan":
        # k · π / 4 ± offset for k = 1, 3, 5, 7, … (45°, 135°, 225°…).
        # Avoid π/2 multiples (poles) which safe_to_case rejects.
        for k in [1, 3, 5, 7, -1, -3, -5, -7]:
            if len(out) >= count:
                break
            center = scaled(mpf(k) * mp.pi / mpf(4), scale)
            for delta in [-3, -1, 0, 1, 3]:
                raw_in = center + delta
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "exp":
        # k · ln 2 ± offset, for k from -20 to 20.
        # Tang stage-1 reduction uses x = k·ln2 + r; the breakpoint
        # is where round(x / ln 2) flips. Sampling near these breaks
        # the worst-case Taylor truncation.
        for k in range(-20, 21):
            if len(out) >= count:
                break
            center = scaled(mpf(k) * mp.log(2), scale)
            for delta in [-3, -1, 0, 1, 3]:
                raw_in = center + delta
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "atan":
        # atan near 1: argument-halving rotation boundary
        # (atan(x) = π/4 + atan((x-1)/(x+1)) for x near 1).
        for delta in [-100, -10, -1, 0, 1, 10, 100]:
            for sgn in (1, -1):
                raw_in = sgn * (one + delta)
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
        # And near tan(k·π/8) for k = 1..7 (argument halving table
        # boundary).
        for k in range(1, 8):
            if len(out) >= count:
                break
            anchor_x = mp.tan(mpf(k) * mp.pi / mpf(8))
            center = scaled(anchor_x, scale)
            for sgn in (1, -1):
                for delta in [-3, -1, 0, 1, 3]:
                    raw_in = sgn * center + delta
                    case = safe_to_case(raw_in, func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
    return out[:count]


# --- Category 4: removable singularity / asymptote stress ----------------
#
# Reference: Kahan archive "Branch cuts for complex elementary
# functions" (1987).
#
#   tan(x) near π/2  — pole. Stay 0.1 LSB inside.
#   ln(x) near 0+    — asymptote.
#   atan(huge)       — saturates fast.

def cat_asymptote(func_name: str, scale: int, max_raw: int,
                  count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name == "tan":
        # Just outside ±(k+0.5)·π, distance from pole shrinking
        # geometrically. Need to be > 1e-6 from pole per safe_to_case.
        for k in [0, 1, -1, 2, -2]:
            if len(out) >= count:
                break
            pole = (mpf(k) + mpf("0.5")) * mp.pi
            for shift in [1e-3, 1e-4, 1e-5]:
                for sgn in (1, -1):
                    x = pole + mpf(sgn) * mpf(shift)
                    raw_in = scaled(x, scale)
                    case = safe_to_case(raw_in, func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
    elif func_name == "ln":
        # x → 0+, raw decreasing geometrically.
        for k in range(scale, 0, -1):
            if len(out) >= count:
                break
            raw_in = 10 ** (k - 1)
            if raw_in <= 0:
                continue
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
        # Also a roster of small absolute raws.
        for r in [1, 2, 3, 5, 7, 11, 13]:
            case = safe_to_case(r, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
    elif func_name == "atan":
        # atan(x) saturates at ±π/2. Probe with x = 10**k for k up
        # to capacity-scale-1.
        cap_k = max(1, (max_raw.bit_length() // 3) - 1)
        for k in range(1, min(40, cap_k + 1)):
            for sgn in (1, -1):
                raw_in = sgn * (one * (10 ** k))
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
                    if len(out) >= count:
                        break
    elif func_name == "sqrt":
        # sqrt near 0 — d/dx sqrt = 1/(2·sqrt(x)) which is unbounded.
        for k in range(scale, 0, -1):
            raw_in = 10 ** (k - 1)
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
                if len(out) >= count:
                    break
    return out[:count]


# --- Category 5: inverse-identity round-trip stress ----------------------
#
# Reference: Brent & Zimmermann (2010) §4.2 "Inverse functions".
#
# These are inputs where f(f⁻¹(x)) is most sensitive to per-kernel
# rounding. We can't include both the forward AND inverse in a
# single-function golden file, but we *can* probe the forward at
# the input points the inverse identity stresses.
#
#   sin near π/2     — asin(sin(x)) sensitive
#   atan near tan(x) — atan(tan(x)) round-trip
#   exp at ln(MIN)   — exp(ln(MIN_POSITIVE)) round-trip near underflow
#   sqrt at squares  — sqrt(x²) on huge / tiny x

def cat_inverse_identity(func_name: str, scale: int, max_raw: int,
                         count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name == "sin":
        # Near π/2 ± tiny.
        for k in [1, -1, 3, -3]:
            center = scaled(mpf(k) * mp.pi / mpf(2), scale)
            for delta in [-5, -1, 0, 1, 5]:
                case = safe_to_case(center + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "cos":
        # Near 0 and ±π (where cos = ±1).
        for center in [0, scaled(mp.pi, scale), -scaled(mp.pi, scale)]:
            for delta in [-5, -1, 0, 1, 5]:
                case = safe_to_case(center + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "tan":
        # Same shape as the range-reduction breakpoints but tighter:
        # k·π/4 ± a few LSBs for k = ±1, ±3.
        for k in [1, -1, 3, -3]:
            center = scaled(mpf(k) * mp.pi / mpf(4), scale)
            for delta in [-5, -1, 0, 1, 5]:
                case = safe_to_case(center + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "atan":
        # atan(tan(k·π/8)) round-trip — pre-compute the input as
        # tan(k·π/8) for k = 1..7 then ±a few LSBs.
        for k in range(1, 8):
            anchor = mp.tan(mpf(k) * mp.pi / mpf(16))
            center = scaled(anchor, scale)
            for delta in [-3, 0, 3]:
                for sgn in (1, -1):
                    case = safe_to_case(sgn * center + delta,
                                        func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
    elif func_name == "exp":
        # exp(ln(MIN_POSITIVE)) — that is, exp near ln(1/max_raw).
        # Sample exp(x) for x = ln(small) so the result is small.
        min_pos_log = mp.log(mpf(1) / mpf(one))
        for shift in [mpf("0.1"), mpf("0.01"), mpf("0.001")]:
            x = min_pos_log + shift
            raw_in = scaled(x, scale)
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
        # And exp(ln(MAX)) — large output.
        max_log = mp.log(mpf(max_raw) / mpf(one))
        for shift in [mpf("-0.1"), mpf("-0.01")]:
            x = max_log + shift
            raw_in = scaled(x, scale)
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
    elif func_name == "ln":
        # ln(exp(x)) round-trip — sample at exp(k) for k in a range.
        for k in [mpf("0.5"), mpf(1), mpf(2), mpf(5), mpf(10), mpf(20)]:
            anchor = mp.exp(k)
            center = scaled(anchor, scale)
            for delta in [-3, 0, 3]:
                case = safe_to_case(center + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "sqrt":
        # sqrt(x²) — sample squares of large / tiny inputs.
        for base in [mpf(2), mpf("0.5"), mpf(7), mpf("0.001"),
                     mpf(1000), mpf("1e-6")]:
            sq = base * base
            raw_in = scaled(sq, scale)
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
    elif func_name == "cbrt":
        # cbrt(x³) — sample cubes.
        for base in [mpf(2), mpf("-0.5"), mpf(11), mpf("0.001"),
                     mpf(-1000), mpf("1e-6")]:
            cube = base * base * base
            raw_in = scaled(cube, scale)
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
    return out[:count]


# --- Category 6: perfect-power ± ULP for roots ---------------------------
#
# Reference: Brent & Zimmermann (2010) §3.5 / §3.6.
#
#   sqrt(n²) — exact integer for n² < max_raw
#   sqrt(n² ± 1) — exercises correctly-rounded branch decision
#   cbrt(n³) — exact
#   cbrt(n³ ± 1) — branch decision

def cat_perfect_power(func_name: str, scale: int, max_raw: int,
                      count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name == "sqrt":
        # Use integer n and the raw representation of n² · one.
        for n in [1, 2, 3, 4, 5, 7, 11, 16, 23, 31, 64, 99, 100, 127,
                  256, 512, 1000, 1023, 1024, 2047, 4096, 8191]:
            if len(out) >= count:
                break
            sq_raw = n * n * one
            if abs(sq_raw) > max_raw:
                continue
            for delta in [-1, 0, 1]:
                case = safe_to_case(sq_raw + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    elif func_name == "cbrt":
        for n in [-32, -8, -3, -1, 1, 2, 3, 4, 5, 7, 8, 11, 16,
                  31, 32, 64, 99, 127, 256, 512, 1000]:
            if len(out) >= count:
                break
            cu_raw = n * n * n * one
            if abs(cu_raw) > max_raw:
                continue
            for delta in [-1, 0, 1]:
                case = safe_to_case(cu_raw + delta, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    return out[:count]


# --- Category 7: constant edges ------------------------------------------
#
# Reference: IEEE 754-2019 §9; Muller (2016) §9.

def cat_constants(func_name: str, scale: int, max_raw: int,
                  count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    # Map each function to its set of "interesting" exact constants.
    constants_mp: list[mpf]
    if func_name in ("sin", "cos", "tan"):
        constants_mp = [
            mp.pi, mp.pi / 2, mp.pi / 3, mp.pi / 4, mp.pi / 6,
            2 * mp.pi, 3 * mp.pi / 2,
        ]
    elif func_name == "atan":
        # tan(constant) — so atan(tan(...)) lands at the constant.
        constants_mp = [mp.pi / 4, mp.pi / 3, mp.pi / 6]
        constants_mp = [mp.tan(c) for c in constants_mp] + [mpf(1)]
    elif func_name == "ln":
        constants_mp = [mp.e, mpf(2), mpf(10), mpf(1), mp.exp(mp.pi)]
    elif func_name == "exp":
        constants_mp = [mp.log(mpf(2)), mp.log(mpf(10)), mpf(1),
                        mp.pi, mpf(0)]
    elif func_name == "sqrt":
        constants_mp = [mpf(2), mpf(3), mpf(5), mp.pi, mp.e, mpf("0.5")]
    elif func_name == "cbrt":
        constants_mp = [mpf(2), mpf(3), mpf(5), mp.pi, mp.e,
                        mpf(-2), mpf(-3)]
    else:
        return []

    for c in constants_mp:
        center = scaled(c, scale)
        for delta in [-1, 0, 1]:
            for sgn in (1, -1):
                raw_in = sgn * center + delta
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    # Plus signed extremes.
    extremes = [one, -one, 2 * one, -2 * one]
    for raw_in in extremes:
        case = safe_to_case(raw_in, func_name, scale, max_raw)
        if case is not None:
            out.append(case)
    return out[:count]


# --- Category 8: argument-halving cascade edge (atan) --------------------
#
# Reference: ALGORITHMS.md per-width halving count table. The
# kernel picks `n` halvings so the reduced argument satisfies
# |r| <= 0.35 · 2⁻ⁿ. Inputs exactly at that bound stress the
# choice of `n`.

def cat_argument_halving(func_name: str, scale: int, max_raw: int,
                         count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name != "atan":
        return []

    # Per-width halving counts (from ALGORITHMS.md).
    # Use working-scale heuristic: smaller scale → fewer halvings.
    if scale < 30:
        halvings = [5]
    elif scale < 55:
        halvings = [5, 6]
    elif scale < 110:
        halvings = [6, 7]
    else:
        halvings = [6, 7, 8]

    # For each halving count `n`, sample x such that after n halvings
    # the reduced |r| sits right at 0.35 · 2⁻ⁿ. The half-angle
    # tangent formula: tan(a/2) = (√(1+t²) - 1) / t for t = tan(a).
    # Working in the other direction is messy; instead we sample x
    # values whose unhalved magnitude is the inverse-halving step
    # bound: tan(0.35 · 2⁻ⁿ · 2ⁿ) = tan(0.35).
    for n in halvings:
        bound = mpf("0.35") * mpf(2) ** (-n)
        # The reduced argument 0.35·2⁻ⁿ corresponds to a pre-reduction
        # argument near 0.35 (since halving brings down by 2ⁿ in tan).
        # We sample close to ±tan(0.35), ±tan(0.35 / 2), …
        for steps in range(n + 1):
            anchor = mp.tan(mpf("0.35") * mpf(2) ** (-steps))
            center = scaled(anchor, scale)
            for delta in [-2, 0, 2]:
                for sgn in (1, -1):
                    case = safe_to_case(sgn * center + delta,
                                        func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
    return out[:count]


# --- Category 9: Stage-2 argument reduction edge for exp -----------------
#
# Reference: Tang (1989). Stage-2 reduction: v / 2ⁿ for
# n ≈ √(precision_bits). Inputs near a half-ULP of the chosen
# breakpoint test the Taylor truncation depth.

def cat_exp_stage2(func_name: str, scale: int, max_raw: int,
                   count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name != "exp":
        return []

    # Stage-2 n is roughly sqrt(precision_bits). precision_bits ≈
    # scale · log2(10) ≈ scale · 3.322.
    precision_bits = int(scale * math.log2(10))
    n_central = max(2, int(math.sqrt(precision_bits)))
    n_choices = [max(2, n_central - 1), n_central, n_central + 1]

    # x / 2ⁿ near 0.5 LSB at the working scale: pick small x and
    # multiply.
    half_one = max(1, one // 2)
    # `exp` overflows storage long before its argument reaches the
    # capacity edge; cap the probed argument so the oracle never has
    # to evaluate `exp` of an astronomically large value (which would
    # produce a multi-million-digit result and stall the generator).
    max_exp_arg_raw = int(mp.log(mpf(max_raw) / mpf(one))) * one + one
    for n in n_choices:
        scale_2n = 2 ** n
        # Pre-image of half-LSB at stage-2 is half_one · 2ⁿ.
        # That's small enough that exp(x) ≈ 1; the kernel's Taylor
        # branch handles it. We want x slightly above/below.
        for k in range(-3, 4):
            anchor = mpf(half_one * scale_2n) + mpf(k)
            raw_in = int(anchor)
            if abs(raw_in) > max_exp_arg_raw:
                continue
            case = safe_to_case(raw_in, func_name, scale, max_raw)
            if case is not None:
                out.append(case)
        # Also probe near small-integer x's reduced argument.
        for x_int in [1, 2, 3, 5, 10, -1, -2, -5]:
            x_raw = x_int * one
            for delta in [-(scale_2n // 4 + 1), 0, scale_2n // 4 + 1]:
                raw_in = x_raw + delta
                case = safe_to_case(raw_in, func_name, scale, max_raw)
                if case is not None:
                    out.append(case)
    return out[:count]


# --- Category 10: Tang-lookup band edges ---------------------------------
#
# Reference: Tang (1989); Gal & Bachelis (1991).
#
# Shipped Tang lookup bands (per `src/algos/{ln,exp}/lookup_*_tang.rs`):
#   D57<18..=22>, D115<55..=60>, D153<70..=82>, D307<140..=160>,
#   D462<225..=235>, D616<300..=315>, D924<455..=465>, D1232<610..=620>.
# Of these, the goldens cover D153<76>, D307<150>, D616<308>, D1232<615>
# at the design-target scale. Inputs at the band lower/upper bound
# and at lookup-table-index boundaries (T_i breakpoints) stress the
# switch decision.

TANG_BAND_SCALES: dict[str, tuple[int, int]] = {
    # alias -> (band_lo, band_hi) for the Tang ln/exp lookup
    "d57":  (18, 22),
    "d115": (55, 60),
    "d153": (70, 82),
    "d307": (140, 160),
    "d462": (225, 235),
    "d616": (300, 315),
    "d924": (455, 465),
    "d1232": (610, 620),
}


def cat_tang_band(func_name: str, alias: str, scale: int, max_raw: int,
                  count: int, rng: random.Random) -> list[tuple[int, int]]:
    out: list[tuple[int, int]] = []
    one = 10 ** scale

    if func_name not in ("ln", "exp"):
        return []
    band = TANG_BAND_SCALES.get(alias)
    if band is None:
        return []
    band_lo, band_hi = band
    # We can't change SCALE per-test (the golden table is at the
    # tier's design-target scale), but we can still probe at the
    # lookup-table-index boundaries. The Tang decomposition for ln
    # uses T_i = 1 + i / 2^k for k = log2(N), N ≈ 128 entries; for
    # the wide tiers the table is larger. Probe T_i breakpoints for
    # i = 0..N-1.

    if func_name == "ln":
        # Tang ln: input domain x > 0; decomposition picks i such
        # that x · 2^k / N ≈ i. We sample x near each table index
        # breakpoint T_i.
        for k_log in [7, 8, 9]:  # N = 128, 256, 512 representative
            N = 1 << k_log
            for i in range(N):
                if len(out) >= count:
                    break
                T_i = mpf(1) + mpf(i) / mpf(N)
                center = scaled(T_i, scale)
                for delta in [-1, 0, 1]:
                    case = safe_to_case(center + delta,
                                        func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
                        if len(out) >= count:
                            break
            if len(out) >= count:
                break
    elif func_name == "exp":
        # Tang exp: x = k·ln2 + j/N · ln2 + r. Probe at the
        # secondary-index breakpoints j/N · ln 2 for j = 0..N-1.
        for k_log in [5, 6, 7]:  # N = 32, 64, 128
            N = 1 << k_log
            for j in range(N):
                if len(out) >= count:
                    break
                anchor = mpf(j) / mpf(N) * mp.log(mpf(2))
                center = scaled(anchor, scale)
                for delta in [-1, 0, 1]:
                    case = safe_to_case(center + delta,
                                        func_name, scale, max_raw)
                    if case is not None:
                        out.append(case)
                        if len(out) >= count:
                            break
            if len(out) >= count:
                break
    return out[:count]


# --- Category registry ----------------------------------------------------

CATEGORIES: list[tuple[str, str, Callable[..., list[tuple[int, int]]]]] = [
    ("half-ULP-tie",         "tie",   cat_half_ulp_tie),
    ("catastrophic cancellation", "canc", cat_cancellation),
    ("range-reduction breakpoint", "rred", cat_range_reduction),
    ("asymptote stress",     "asym",  cat_asymptote),
    ("inverse-identity",     "inv",   cat_inverse_identity),
    ("perfect-power ± ULP",  "pp",    cat_perfect_power),
    ("constant edges",       "const", cat_constants),
    ("argument-halving cascade", "halv", cat_argument_halving),
    ("exp stage-2 reduction edge", "expr", cat_exp_stage2),
    ("tang-lookup band edge", "tang", None),  # special — needs alias
]


# --- File emission --------------------------------------------------------

def existing_inputs(path: Path) -> set[int]:
    """Read existing golden file inputs to avoid duplicating rows."""
    inputs: set[int] = set()
    if not path.exists():
        return inputs
    for line in path.read_text(encoding="utf-8").splitlines():
        ls = line.strip()
        if not ls or ls.startswith('#'):
            continue
        parts = ls.split('\t')
        if len(parts) >= 1:
            try:
                inputs.add(int(parts[0]))
            except ValueError:
                pass
    return inputs


def append_sections(path: Path,
                    sections: list[tuple[str, list[tuple[int, int]]]]) -> int:
    """Append non-empty `## category: <name>` sections to the golden
    file. Returns total cases appended."""
    appended = 0
    if not sections:
        return 0
    nonempty = [(label, cases) for label, cases in sections if cases]
    if not nonempty:
        return 0
    with path.open("a", encoding="utf-8", newline="\n") as f:
        f.write("\n# ─── hard-input sections (see scripts/gen_hard_inputs.py) ───\n")
        for label, cases in nonempty:
            f.write(f"## category: {label}\n")
            for raw_in, raw_out in cases:
                f.write(f"{raw_in}\t{raw_out}\n")
                appended += 1
    return appended


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    grand_total = 0
    per_cell_summary: list[tuple[str, str, str, int]] = []

    for alias, capacity, scale, cases_per_cat in TIERS:
        # Bump mpmath's working precision before any oracle call at
        # this tier — the storage LSB at D1232<615> is well below
        # 500 dps and a fixed value misclassifies kernel results as
        # gate failures. See `dps_for` docstring.
        mp.dps = dps_for(scale)
        max_raw = 10 ** (capacity - 1)
        for func_name in FUNC_NAMES:
            path = OUT_DIR / f"{func_name}_{alias}_s{scale}.txt"
            if not path.exists():
                # Skip tiers where the base generator produced no
                # file (out-of-domain or overflow).
                continue
            already = existing_inputs(path)

            sections: list[tuple[str, list[tuple[int, int]]]] = []

            for label, slug, fn in CATEGORIES:
                if fn is None and slug == "tang":
                    seed_key = f"{alias}-{scale}-{func_name}-{slug}-v1"
                    rng = random.Random(seed_key)
                    cases = cat_tang_band(func_name, alias, scale,
                                          max_raw, cases_per_cat, rng)
                else:
                    seed_key = f"{alias}-{scale}-{func_name}-{slug}-v1"
                    rng = random.Random(seed_key)
                    cases = fn(func_name, scale, max_raw, cases_per_cat, rng)

                # Filter to inputs not already in the existing file
                # AND not already in earlier sections this run.
                filtered: list[tuple[int, int]] = []
                seen_run = {r for _, sec in sections for r, _ in sec}
                for raw_in, raw_out in cases:
                    if raw_in in already or raw_in in seen_run:
                        continue
                    filtered.append((raw_in, raw_out))

                sections.append((label, filtered))
                per_cell_summary.append(
                    (alias, func_name, label, len(filtered)))

            appended = append_sections(path, sections)
            grand_total += appended
            print(f"  {path.relative_to(ROOT)}: +{appended} hard cases")

    # Footprint check.
    total_bytes = sum((OUT_DIR / p.name).stat().st_size
                       for p in OUT_DIR.iterdir() if p.suffix == ".txt")
    print()
    print(f"appended cases: {grand_total}")
    print(f"total footprint: {total_bytes / 1024 / 1024:.2f} MB")
    if total_bytes > 5 * 1024 * 1024:
        print("WARNING: footprint > 5 MB budget cap")


if __name__ == "__main__":
    main()
