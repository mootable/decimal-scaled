"""Generate mpmath-oracle golden precision tables for the crate's
strict transcendentals.

For each (width, scale, function) tier we emit a `.txt` file under
`tests/golden/` with one `<input_raw>\t<floor_raw>\t<cls>` per line:

  * `input_raw` — the storage integer the kernel receives, i.e. the
    value `x` represented as `round(x * 10**scale)` is NOT used; the
    column is the literal storage integer and the mathematical input
    is `input_raw / 10**scale`.
  * `floor_raw` — `floor(f(x) * 10**scale)`, i.e. the true value of
    the function at the tier scale rounded toward negative infinity.
    This is mode-independent: the correctly-rounded result under any
    `RoundingMode` is either `floor_raw` or `floor_raw + 1`, and the
    `cls` column says which.
  * `cls` — classification of the fractional part `frac = f(x) *
    10**scale - floor_raw`, which lies in `[0, 1)`:
      - `Z`  exact: `frac == 0` (the value is exactly representable
             at the tier scale; every mode returns `floor_raw`).
      - `L`  low:   `0 < frac < 0.5` (nearest is `floor_raw`).
      - `E`  tie:   `frac == 0.5` exactly (half-way; nearest modes
             break the tie, directed modes ignore it).
      - `G`  high:  `0.5 < frac < 1` (nearest is `floor_raw + 1`).

This three-column "floor + tie-class" encoding lets the Rust harness
(`tests/ulp_strict_golden.rs`) compute the correctly-rounded result
for EVERY `RoundingMode` from a single table — no per-mode tables.
The harness asserts the kernel's `*_strict_with(mode)` output equals
that mode's correctly-rounded integer EXACTLY (delta == 0 storage
LSB). That is the crate's "0.5 ULP, correctly rounded" guarantee:
the result is the true real value rounded to the last representable
place under the active rounding rule, with zero tolerance.

The fractional part is computed by mpmath at a per-tier working
precision of `max(700, 2*SCALE + 64)` decimal digits — wide enough
that the digits past the tier scale are themselves resolved to many
extra places, so the `L` / `E` / `G` classification is unambiguous
for the transcendental kernels (whose outputs are irrational and so
never land exactly on a half-tie except where the input forces it).

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
All thirteen decimal widths are covered; case counts taper hard for
the wider tiers (where each line is hundreds of digits) so the
budget holds.

Usage:
    pip install mpmath
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

# Working precision wider than every shipped tier. The widest tier
# is D1232<615>, whose storage LSB is `10^-615`. Internal oracle
# computations (squarings, intermediate range reductions) double the
# digit count, so we need `>= 2 * SCALE + small_const`. The dps is
# raised per tier (see the per-tier loop below); the global default
# is set to the tightest tier and lifted before each tier is processed.
mp.dps = 700

# --- Tier targets ---------------------------------------------------------
#
# (width_alias, storage_digit_capacity, scale, base_case_count)
#
# `storage_digit_capacity` is the documented "decimal digits" the
# storage type holds; `max_abs_raw` clamps random draws so the integer
# input always fits the type. We use a conservative cap of
# `10 ** (capacity - 1)` so signed arithmetic in the kernel cannot
# trip near the type's true MAX.
#
# Coverage choice — EVERY one of the crate's thirteen decimal widths
# is represented at its design-target SCALE (~ capacity / 2, matching
# the bespoke-kernel slots and neighbour tiers). Case counts taper
# hard at the wider tiers to stay inside the 5 MB committed budget;
# every width is present even where its per-cell count is small.

# Signed storage maxima for the tiers whose true range is narrower
# than `10 ** (capacity - 1)`. Only the primitive-backed tiers need
# an entry — the wide-int tiers hold far more than `10 ** (capacity
# - 1)`, so their conservative decimal cap is the binding one.
STORAGE_MAX = {
    "d9": 2 ** 31 - 1,   # i32::MAX
    "d18": 2 ** 63 - 1,  # i64::MAX
    "d38": 2 ** 127 - 1, # i128::MAX
}

TIERS = [
    # (alias, storage_capacity_digits, scale, base_case_count)
    ("d9",    9,    4,   180),
    ("d18",   18,   9,   180),
    ("d38",   38,   19,  160),
    ("d57",   57,   28,  140),
    ("d76",   76,   35,  120),
    ("d115",  115,  57,  100),
    ("d153",  153,  76,  90),
    ("d230",  230,  115, 70),
    ("d307",  307,  150, 60),
    ("d462",  462,  230, 44),
    ("d616",  616,  308, 36),
    ("d924",  924,  460, 24),
    ("d1232", 1232, 615, 20),
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


def floor_and_class(value: mpf, scale: int) -> tuple[int, str]:
    """Return `(floor_raw, cls)` for `value` at the tier scale.

    `floor_raw = floor(value * 10**scale)` (toward negative infinity).
    `cls` classifies the fractional remainder `frac` in `[0, 1)`:

      * `Z` — `frac == 0` (exactly representable),
      * `L` — `0 < frac < 0.5`,
      * `E` — `frac == 0.5` exactly,
      * `G` — `0.5 < frac < 1`.

    The classification is mode-independent: the harness derives the
    correctly-rounded integer for any `RoundingMode` from
    `(floor_raw, cls)` and the sign of `floor_raw`.
    """
    scaled = value * (mpf(10) ** scale)
    # mpmath's floor on an mpf returns an mpf; int() of that truncates
    # toward zero, which for a non-negative mpf equals floor. We do the
    # floor explicitly via mp.floor to get round-toward-negative-infinity
    # for negative values too.
    floor_int = int(mp.floor(scaled))
    frac = scaled - mpf(floor_int)
    half = mpf("0.5")
    if frac == 0:
        cls = "Z"
    elif frac < half:
        cls = "L"
    elif frac == half:
        cls = "E"
    else:
        cls = "G"
    return floor_int, cls


def round_half_even_from(floor_int: int, cls: str) -> int:
    """Half-to-even rounded integer from a `(floor_raw, cls)` pair.

    Used only for the in-budget cap check (whether the rounded result
    fits the storage type) — the harness re-derives every mode itself.
    """
    if cls in ("Z", "L"):
        return floor_int
    if cls == "G":
        return floor_int + 1
    # Exact tie — bank to even.
    return floor_int if (floor_int % 2 == 0) else floor_int + 1


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
    "# each line: <input_raw>\\t<floor_raw>\\t<cls>",
    "# input_raw  storage integer of x at the tier scale (x = input_raw / 10**scale).",
    "# floor_raw  floor(f(x) * 10**scale), rounded toward negative infinity.",
    "# cls        fractional class of f(x)*10**scale - floor_raw, in [0,1):",
    "#              Z = exact (frac == 0), L = 0<frac<0.5, E = frac==0.5, G = 0.5<frac<1.",
    "# The correctly-rounded result under any RoundingMode is floor_raw or",
    "# floor_raw+1, derived from (floor_raw, cls, sign) by the harness.",
    "# Computed by mpmath at max(700, 2*SCALE + 64)-digit working precision.",
]


def emit_file(path: Path, cases: list[tuple[int, int, str]]) -> int:
    """Write the golden table. Returns the file byte count."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as f:
        for line in HEADER_LINES:
            f.write(line + "\n")
        for raw_in, floor_raw, cls in cases:
            f.write(f"{raw_in}\t{floor_raw}\t{cls}\n")
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
        # `max_raw` clamps both inputs and rounded outputs to what the
        # storage type can actually hold. The documented decimal
        # capacity (`10 ** (capacity - 1)`) is the headroom-conservative
        # ceiling for the wide tiers, but the *primitive* tiers (D9 =
        # i32, D18 = i64) saturate well below `10 ** (capacity - 1)`:
        # i32::MAX ~ 2.1e9, i64::MAX ~ 9.2e18. Cap to the true signed
        # maximum there so no input or output overflows the storage on
        # the Rust side.
        max_raw = 10 ** (capacity - 1)
        if alias in STORAGE_MAX:
            max_raw = min(max_raw, STORAGE_MAX[alias])
        counts = category_counts(base_count)
        # Lift mpmath working precision so the oracle's intermediate
        # squarings stay safely above the tier's storage LSB. The
        # `2*SCALE + 64` floor covers the worst case where the oracle
        # squares an LSB-scale residual; the global lower bound of 700
        # keeps the narrow tiers from running unnecessarily slow on
        # small `2*SCALE` values.
        mp.dps = max(700, 2 * scale + 64)

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
            cases: list[tuple[int, int, str]] = []
            for raw_in in deduped:
                # Drop inputs the storage type can't hold (edge rosters
                # build a few magnitudes from `one * 10**k` that exceed
                # the narrow-tier signed maximum).
                if abs(raw_in) > max_raw:
                    continue
                x = from_raw(raw_in, scale)
                y = safe_call(oracle, x)
                if y is None:
                    continue
                floor_raw, cls = floor_and_class(y, scale)
                # Both neighbours (floor and floor+1) must fit the
                # storage type — any RoundingMode may select either.
                if abs(floor_raw) > max_raw or abs(floor_raw) + 1 > max_raw:
                    continue
                cases.append((raw_in, floor_raw, cls))

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
