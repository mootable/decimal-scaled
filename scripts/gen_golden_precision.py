"""Generate mpmath-oracle golden precision tables for the crate's
strict transcendentals.

For each (width, scale, function) tier we emit a `.txt` file under
`tests/golden/` with one `<input_raw_int>\t<expected_raw_int>` per
line. `input_raw_int` is the storage value the kernel receives;
`expected_raw_int` is the half-to-even rounding of the true real
value f(input_raw_int / 10**scale), computed by mpmath at 500
decimal digits of precision.

The harness `tests/ulp_strict_golden.rs` reads each file, calls the
corresponding kernel, and asserts `|actual - expected| <= 1` storage
LSB (the 0.5 ULP contract).

Categories of cases per (tier, function) file:

  * `near_boundary` — small inputs around the function's natural
    boundary (ln near 1, exp near 0, trig near 0/quarter-pi/half-pi,
    roots near perfect squares/cubes).
  * `half_ulp_tie` — inputs whose true output is bracketed within
    `(k - 0.4, k + 0.4)` storage LSBs around the half-tie point.
    Hardest tie-breaking edge.
  * `random_uniform` — deterministic-seeded uniform samples across
    the natural input domain.
  * `edge_values` — a fixed roster of small / large magnitudes
    (e.g. MAX, MAX/2, MIN, ULP, -ULP, …) appropriate to the function
    domain.

Reproducibility: the script is deterministic — every random draw is
seeded from `random.Random(<seeded_key>)`. Two runs produce
byte-identical golden files.

Footprint budget: aim for <= 5 MB committed under `tests/golden/`.
Case counts taper for wider tiers (where each line is hundreds of
digits) so the budget holds.

Usage:
    python scripts/gen_golden_precision.py
"""

from __future__ import annotations

import random
from pathlib import Path
from typing import Callable

from mpmath import mp, mpf, ln, exp, sin, cos, tan, atan, sqrt, cbrt, mpc


def real_cbrt(x: mpf) -> mpf:
    """Real cube root. mpmath's cbrt returns the principal complex
    root for negative inputs; the crate's `cbrt_strict` is the real
    cube root, so we mirror that convention."""
    if x >= 0:
        return cbrt(x)
    return -cbrt(-x)

# --- Workspace layout -----------------------------------------------------

ROOT = Path(__file__).resolve().parent.parent
OUT_DIR = ROOT / "tests" / "golden"

# Working precision wider than the widest tier (D1232<615>) so every
# rounding is exact at the tier's storage LSB.
mp.dps = 500

# --- Tier targets ---------------------------------------------------------
#
# (width_alias, storage_digit_capacity, scale, max_abs_raw, label_prefix)
#
# `storage_digit_capacity` is the documented "decimal digits" the
# storage type holds; `max_abs_raw` clamps random draws so the integer
# input always fits the type. We use a conservative cap of
# `10 ** (capacity - 1)` so signed arithmetic in the kernel cannot
# trip near the type's true MAX.
#
# Coverage choice — we cover the design-target SCALE for every wide
# tier, but trim case counts at the wider tiers to stay inside the
# 5 MB committed budget. D115<57>, D462<230>, D924<460> are deferred
# (called out in docs/precision-testing.md).

TIERS = [
    # (alias, storage_capacity_digits, scale, base_case_count)
    ("d38",  38,  19,  240),
    ("d76",  76,  35,  200),
    ("d153", 153, 76,  140),
    ("d307", 307, 150, 100),
    ("d616", 616, 308, 60),
    ("d1232", 1232, 615, 24),
]

# --- mpmath function oracles ----------------------------------------------
#
# Each entry: (name, oracle, domain_name)
#
# Oracles receive an mpf and return an mpf. The harness uses the
# function name to dispatch to the corresponding strict kernel.

FUNCS: list[tuple[str, Callable[[mpf], mpf], str]] = [
    ("ln",   ln,                       "positive"),
    ("exp",  exp,                      "moderate_real"),
    ("sin",  sin,                      "real"),
    ("cos",  cos,                      "real"),
    ("tan",  tan,                      "tan_safe"),
    ("atan", atan,                     "real"),
    ("sqrt", sqrt,                     "nonneg"),
    ("cbrt", real_cbrt,                "real"),
]

# --- Helpers --------------------------------------------------------------

def round_half_even(value: mpf) -> int:
    """Half-to-even rounding of an mpf to the nearest integer.

    `int(...)` truncates toward zero; we need banker's rounding to
    match the crate-default `HalfToEven` mode at the storage LSB.
    """
    # Use mpmath's nstr at high precision and parse, then bank-round
    # by inspecting the fractional half. We work in two stages:
    #  1. floor of |value|
    #  2. fractional remainder; if exactly 0.5, bank to even.
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
        # Exact tie — bank to even.
        rounded = floor_int if (floor_int % 2 == 0) else floor_int + 1
    return sign * rounded


def scaled(value: mpf, scale: int) -> int:
    """Round `value * 10**scale` half-to-even to the storage LSB."""
    return round_half_even(value * (mpf(10) ** scale))


def from_raw(raw: int, scale: int) -> mpf:
    """The mathematical value of a storage integer at the tier scale."""
    return mpf(raw) / (mpf(10) ** scale)


def safe_call(oracle: Callable[[mpf], mpf], x: mpf) -> mpf | None:
    try:
        y = oracle(x)
    except (ValueError, ZeroDivisionError, OverflowError, ArithmeticError):
        return None
    # Some mpmath functions (cbrt, sqrt) return mpc for negative
    # inputs. The crate's strict surface either rejects (sqrt) or
    # returns the real branch (cbrt — wrapped above). If we still get
    # a complex result here, drop the input.
    if isinstance(y, mpc):
        return None
    return y


# --- Per-function input strategies ----------------------------------------

def sample_inputs(func_name: str, scale: int, max_raw: int, count: int,
                  rng: random.Random) -> list[int]:
    """Generate `count` storage-int inputs appropriate to the function.

    Returns raw storage values (not the mathematical x). Excludes
    domain-illegal inputs (negatives for ln/sqrt, ±k·π/2 for tan, …).
    """
    out: list[int] = []
    one = 10 ** scale

    if func_name == "ln":
        # x > 0; cover (0, 1) and (1, large). Avoid extreme magnitudes
        # so the working scale doesn't run out of guard digits.
        cap = min(max_raw, one * (10 ** 8))
        while len(out) < count:
            mag = rng.randint(1, cap)
            out.append(mag)

    elif func_name == "exp":
        # exp(x) overflows fast — clamp |x| <= ln(max representable).
        # For the storage cap, ln(10^(width)) ~ width * ln(10). We use
        # ln(max_raw / 10^scale) as the hard ceiling and stay 10% inside.
        max_x_int = max(1, int(0.9 * mp.log(mpf(max_raw) / mpf(one))))
        max_x = max(1, max_x_int)
        while len(out) < count:
            x_int = rng.randint(-max_x, max_x)
            # Encode x as a raw storage int by sampling in [-max_x, max_x]
            # and then a random fraction below the LSB.
            frac = rng.randint(0, one - 1)
            raw = x_int * one + frac
            if abs(raw) <= max_raw:
                out.append(raw)

    elif func_name in ("sin", "cos", "tan"):
        # Real-valued; we sample |x| modestly so range reduction has
        # finite cost. Cover (-4π, 4π) but with a few large magnitudes.
        twelve_pi_scaled = int(mpf(12) * mp.pi * one)
        cap = min(max_raw, twelve_pi_scaled)
        # Tan: also avoid raw values within 0.1 LSB of (k+0.5)*π/scale.
        while len(out) < count:
            raw = rng.randint(-cap, cap)
            if func_name == "tan":
                x = from_raw(raw, scale)
                # Distance from nearest (k+0.5)*π.
                k = round(float(x / mp.pi - mpf("0.5")))
                pole = (mpf(k) + mpf("0.5")) * mp.pi
                if abs(x - pole) < mpf("0.0001"):
                    continue
            out.append(raw)

    elif func_name == "atan":
        # All reals safe. Cover small and large magnitudes (atan
        # saturates fast so we want a few large draws for coverage).
        while len(out) < count:
            scale_factor = rng.choice([1, 10, 1000, 10**6])
            raw = rng.randint(-max_raw // scale_factor, max_raw // scale_factor)
            out.append(raw)

    elif func_name == "sqrt":
        # x >= 0.
        while len(out) < count:
            out.append(rng.randint(0, max_raw))

    elif func_name == "cbrt":
        # All reals safe.
        while len(out) < count:
            out.append(rng.randint(-max_raw, max_raw))

    return out


# --- Half-ULP-tie hunter --------------------------------------------------

def find_half_ulp_ties(func_name: str, oracle: Callable[[mpf], mpf],
                       scale: int, max_raw: int, want: int,
                       rng: random.Random) -> list[int]:
    """Search for inputs whose true output lands within 0.4 LSB of
    a half-tie point `(k + 0.5)` at the storage scale.

    Strategy: random scan, accept if |frac(y * 10^scale) - 0.5| < 0.4.
    Slower than bisection but unbiased; the half-tie surface is dense
    enough across the natural domain that random-scan converges fast.

    Returns at most `want` raw storage inputs.
    """
    out: list[int] = []
    one = 10 ** scale
    attempts = 0
    cap = min(max_raw, one * (10 ** 6))

    while len(out) < want and attempts < want * 200:
        attempts += 1
        # Sample a candidate raw input across the function's domain.
        if func_name in ("ln", "sqrt"):
            raw = rng.randint(1, cap)
        elif func_name in ("exp",):
            # Keep |x| <= ~20 so exp doesn't overflow at narrow scales.
            raw = rng.randint(-20 * one, 20 * one)
            if abs(raw) > max_raw:
                continue
        elif func_name == "tan":
            raw = rng.randint(-3 * one, 3 * one)
            x = from_raw(raw, scale)
            k = round(float(x / mp.pi - mpf("0.5")))
            pole = (mpf(k) + mpf("0.5")) * mp.pi
            if abs(x - pole) < mpf("0.01"):
                continue
        elif func_name in ("sin", "cos"):
            raw = rng.randint(-4 * one, 4 * one)
        elif func_name == "atan":
            raw = rng.randint(-10 * one, 10 * one)
        elif func_name == "cbrt":
            raw = rng.randint(-cap, cap)
        else:
            raw = rng.randint(-cap, cap)

        x = from_raw(raw, scale)
        y = safe_call(oracle, x)
        if y is None:
            continue
        # The kernel's expected output, in LSB units, is round(y * 10^scale).
        # The "fractional half-distance" is |frac(y * 10^scale) - 0.5|.
        scaled_y = y * (mpf(10) ** scale)
        sign = 1 if scaled_y >= 0 else -1
        mag = abs(scaled_y)
        floor = int(mag)
        frac = mag - mpf(floor)
        dist = abs(frac - mpf("0.5"))
        if dist < mpf("0.4"):
            # Output fits storage if the rounded value is within range.
            rounded = sign * (floor if frac < mpf("0.5") else floor + 1)
            if abs(rounded) <= max_raw:
                out.append(raw)
    return out


# --- Edge values ---------------------------------------------------------

def edge_inputs(func_name: str, scale: int, max_raw: int) -> list[int]:
    """Function-appropriate edge inputs as raw storage values."""
    one = 10 ** scale
    half_max = max_raw // 2

    if func_name == "ln":
        # ln(1) = 0, ln near 1, ln of small and large.
        return [
            1,                       # smallest positive raw
            one,                     # x = 1 exactly
            one - 1, one + 1,        # 1 ± ULP
            2 * one,                 # x = 2 (ln 2)
            10 * one,                # x = 10 (ln 10)
            one // 2,                # x = 0.5
            one * 100,
            one * 1000,
        ]
    if func_name == "exp":
        # exp(0), exp(±1), small around zero.
        cap = max(1, int(0.5 * mp.log(mpf(max_raw) / mpf(one))))
        return [
            0,
            1, -1,
            one, -one,
            cap * one, -(cap * one),
            one // 2, -(one // 2),
        ]
    if func_name in ("sin", "cos"):
        pi_raw = int(mp.pi * one)
        half_pi_raw = int(mp.pi * one / 2)
        quarter_pi_raw = int(mp.pi * one / 4)
        return [
            0, one, -one,
            pi_raw, -pi_raw,
            half_pi_raw, -half_pi_raw,
            quarter_pi_raw, -quarter_pi_raw,
            2 * pi_raw, -2 * pi_raw,
        ]
    if func_name == "tan":
        quarter_pi_raw = int(mp.pi * one / 4)
        return [
            0, one, -one,
            quarter_pi_raw, -quarter_pi_raw,
            int(mp.pi * one / 6), -int(mp.pi * one / 6),
            int(mp.pi * one / 3), -int(mp.pi * one / 3),
        ]
    if func_name == "atan":
        return [
            0, one, -one,
            half_max, -half_max,
            10 * one, -10 * one,
            one // 2, -(one // 2),
        ]
    if func_name == "sqrt":
        return [
            0, 1,
            one,
            4 * one, 9 * one, 16 * one, 100 * one,
            one * 10**6,
            half_max,
        ]
    if func_name == "cbrt":
        return [
            0, 1, -1,
            one, -one,
            8 * one, -8 * one,
            27 * one, -27 * one,
            half_max, -half_max,
        ]
    return []


# --- File emission --------------------------------------------------------

HEADER_LINES = [
    "# golden precision oracle table",
    "# generated by scripts/gen_golden_precision.py",
    "# each line: <input_raw>\\t<expected_raw>",
    "# input_raw is the storage integer of x at the tier scale.",
    "# expected_raw is the half-to-even rounding of f(x) at the tier scale,",
    "# computed by mpmath at 500-digit precision.",
]


def emit_file(path: Path, cases: list[tuple[int, int]]) -> int:
    """Write the golden table. Returns the file byte count."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as f:
        for line in HEADER_LINES:
            f.write(line + "\n")
        for raw_in, raw_out in cases:
            f.write(f"{raw_in}\t{raw_out}\n")
    return path.stat().st_size


# --- Driver ---------------------------------------------------------------

# Skip patterns: some (func, tier) combinations are impractical
# (e.g. exp at D1232<615> overflows almost immediately) — handled
# via the per-function clamps; nothing is hardcoded here.

# Per-tier scaling of category counts: each total is the sum of the
# four categories. The constants are chosen to land near `base_count`
# from TIERS and keep the wider tiers within the byte budget.

def category_counts(base_count: int) -> dict[str, int]:
    edge_count_estimate = 11  # roster size, function-dependent
    rest = max(8, base_count - edge_count_estimate)
    # Random gets the lion's share; near-boundary and half-tie split.
    return {
        "near_boundary": rest // 4,
        "half_ulp_tie": rest // 4,
        "random_uniform": rest // 2,
    }


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    total_bytes = 0
    total_cases = 0

    for alias, capacity, scale, base_count in TIERS:
        max_raw = 10 ** (capacity - 1)
        counts = category_counts(base_count)

        for func_name, oracle, _domain in FUNCS:
            seed_key = f"{alias}-{scale}-{func_name}-v1"
            rng = random.Random(seed_key)

            inputs: list[int] = []

            # Near-boundary — domain-specific small values.
            inputs.extend(near_boundary_inputs(func_name, scale, max_raw,
                                               counts["near_boundary"], rng))

            # Random uniform.
            inputs.extend(sample_inputs(func_name, scale, max_raw,
                                        counts["random_uniform"], rng))

            # Half-ULP-tie.
            inputs.extend(find_half_ulp_ties(func_name, oracle, scale,
                                             max_raw, counts["half_ulp_tie"],
                                             rng))

            # Edge values.
            inputs.extend(edge_inputs(func_name, scale, max_raw))

            # Dedupe while preserving order.
            seen: set[int] = set()
            deduped: list[int] = []
            for raw in inputs:
                if raw not in seen:
                    seen.add(raw)
                    deduped.append(raw)

            # Evaluate the oracle for each input.
            cases: list[tuple[int, int]] = []
            for raw_in in deduped:
                x = from_raw(raw_in, scale)
                y = safe_call(oracle, x)
                if y is None:
                    continue
                raw_out = scaled(y, scale)
                if abs(raw_out) > max_raw:
                    continue
                cases.append((raw_in, raw_out))

            if not cases:
                continue

            out_path = OUT_DIR / f"{func_name}_{alias}_s{scale}.txt"
            file_bytes = emit_file(out_path, cases)
            total_bytes += file_bytes
            total_cases += len(cases)
            print(f"  {out_path.relative_to(ROOT)}: "
                  f"{len(cases)} cases, {file_bytes} bytes")

    print()
    print(f"total cases: {total_cases}")
    print(f"total bytes: {total_bytes} ({total_bytes / 1024 / 1024:.2f} MB)")


def near_boundary_inputs(func_name: str, scale: int, max_raw: int,
                         count: int, rng: random.Random) -> list[int]:
    """Domain-specific cluster of small / boundary inputs."""
    one = 10 ** scale
    if count <= 0:
        return []

    if func_name == "ln":
        # Cluster around x = 1 (where ln crosses zero) and around the
        # `_strict` range-reduction boundary (close to e).
        out = []
        for _ in range(count // 2):
            delta = rng.randint(-one // 100, one // 100)
            out.append(max(1, one + delta))
        e_raw = int(mp.e * one)
        for _ in range(count - count // 2):
            delta = rng.randint(-one // 100, one // 100)
            out.append(max(1, e_raw + delta))
        return out

    if func_name == "exp":
        # Cluster around 0 and around ±1.
        out = []
        for _ in range(count // 3):
            out.append(rng.randint(-one // 100, one // 100))
        for _ in range(count // 3):
            delta = rng.randint(-one // 100, one // 100)
            out.append(one + delta)
        for _ in range(count - 2 * (count // 3)):
            delta = rng.randint(-one // 100, one // 100)
            out.append(-one + delta)
        return out

    if func_name in ("sin", "cos"):
        # Cluster around 0, π/4, π/2, π.
        pi_raw = int(mp.pi * one)
        half_pi = pi_raw // 2
        quarter_pi = pi_raw // 4
        anchors = [0, quarter_pi, half_pi, pi_raw, -quarter_pi, -half_pi, -pi_raw]
        out = []
        for _ in range(count):
            a = rng.choice(anchors)
            delta = rng.randint(-one // 1000, one // 1000)
            out.append(a + delta)
        return out

    if func_name == "tan":
        # Cluster around 0 and ±π/4. Stay safely away from ±π/2.
        quarter_pi = int(mp.pi * one / 4)
        anchors = [0, quarter_pi, -quarter_pi]
        out = []
        for _ in range(count):
            a = rng.choice(anchors)
            delta = rng.randint(-one // 1000, one // 1000)
            out.append(a + delta)
        return out

    if func_name == "atan":
        # Cluster around 0 and ±1 (where the function inflects).
        out = []
        anchors = [0, one, -one]
        for _ in range(count):
            a = rng.choice(anchors)
            delta = rng.randint(-one // 100, one // 100)
            out.append(a + delta)
        return out

    if func_name == "sqrt":
        # Cluster around perfect squares.
        out = []
        for _ in range(count):
            n = rng.randint(1, 1000)
            base = n * n * one
            delta = rng.randint(-one // 10, one // 10)
            out.append(max(0, base + delta))
        return out

    if func_name == "cbrt":
        # Cluster around perfect cubes.
        out = []
        for _ in range(count):
            n = rng.randint(-100, 100)
            base = n * n * n * one
            delta = rng.randint(-one // 10, one // 10)
            out.append(base + delta)
        return out

    return []


if __name__ == "__main__":
    main()
