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

Function surface: the twenty-two strict transcendental + root
functions — single-argument `ln exp exp2 log2 log10 sin cos tan atan
asin acos sinh cosh tanh asinh acosh atanh sqrt cbrt` (three-column
tables) and two-argument `log atan2 powf` (four-column tables, the
second column carrying the base / x / exponent operand).

Categories of cases per (tier, function) file:

  * `near_boundary` — small inputs around the function's natural
    boundary (ln near 1, exp near 0, trig near 0/quarter-pi/half-pi,
    roots near perfect squares/cubes, asin/acos/atanh near ±1).
  * `half_ulp_tie` — inputs whose true output is bracketed within
    `(k - 0.4, k + 0.4)` storage LSBs around the half-tie point.
    Hardest tie-breaking edge.
  * `random_uniform` — deterministic-seeded uniform samples across
    the natural input domain.
  * `edge_values` — a fixed roster of small / large magnitudes plus
    the function's edge classes: endpoint branch-points
    (asin/acos/atanh `±(one - 10^k)`, acosh `one + 10^k`), exact
    algebraic rosters (log2 of `2^k`, log10 of `10^k`, exp2 of `k`,
    sqrt of `n^2`, cbrt of `n^3`), the MAX / MAX-ULP boundary pair.
  * `overflow_edge` — the largest exp/exp2/sinh/cosh input whose
    result still fits storage (the just-fits boundary, asserted as a
    golden rather than silently dropped).
  * `large_trig` — very-large trig arguments `K·π` (full Payne-Hanek
    range reduction, beyond the 8–12π cap of the random samplers).
  * `saturation` — tanh grid-line inputs crowding the ±1 output line.
  * `monotonicity` — an `x+1` neighbour beside a sample of inputs so
    adjacent rounded outputs can be cross-checked for ordering.

Exact algebraic points are classified symbolically by integer
arithmetic, bypassing the finite-precision oracle's sub-LSB residual so
the exact value is pinned to the `Z` (no-bump) tie-class under every
rounding mode:

  * perfect squares for sqrt, perfect cubes for cbrt
    (`exact_algebraic_root`);
  * `log_b(b^k) = k` where the argument is an exact power of the base
    (`exact_log_base`) — the `log(v)/log(b)` oracle lands a hair below
    the integer (e.g. `log_2(32) = 5 − 5.9·10**-682` at 700 dps);
  * `base**(p/q)` where a perfect-power base meets a small rational
    exponent (`exact_powf`, e.g. `4**0.5 = 2`).

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
import sys
from pathlib import Path
from typing import Callable

from mpmath import (
    mp, mpf, ln, exp, sin, cos, tan, atan, sqrt, cbrt, mpc,
    asin, acos, atan2, sinh, cosh, tanh, asinh, acosh, atanh, power, log,
)


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
    # Low-scale (SCALE 30) cells for the wide tiers — the bench-branch-compare
    # exp regime, where the exp policy routes a Tang rectangle. The default
    # per-tier entries above sit at each tier's design SCALE (the Tang
    # rectangle's TOP edge / the Series wash zone); these s30 cells validate
    # the rectangle's LOW edge, where Tang's table reduction wins. Same width
    # capacity, scale pinned to 30.
    ("d307",  307,  30,  60),
    ("d462",  462,  30,  44),
    ("d616",  616,  30,  36),
    ("d924",  924,  30,  24),
    ("d1232", 1232, 30,  20),
    # Interior scale samples across each wide tier's exp Tang rectangle (the
    # rectangle spans 0..top; s30 is the low edge, the canonical entry above is
    # the top edge). Two interior scales per tier exercise Tang (small |x|) and
    # Series (large |x|) at mid-rectangle, not just the edges.
    ("d307",  307,  70,  60),
    ("d307",  307,  120, 60),
    ("d462",  462,  100, 44),
    ("d462",  462,  180, 44),
    ("d616",  616,  130, 36),
    ("d616",  616,  240, 36),
    ("d924",  924,  180, 24),
    ("d924",  924,  350, 24),
    ("d1232", 1232, 250, 20),
    ("d1232", 1232, 470, 20),
    # The two SCALE extremes per wide tier: 0 (integer exp) and MAX_SCALE
    # (= capacity - 1, the near-overflow / deep-underflow edge). With the
    # canonical entry (~S/2) and s30 these complete the {0, S/2, S-1} minimum
    # coverage every width should carry; the extremes are where rounding edges
    # (overflow guard, underflow Ceiling) are most likely to surface.
    ("d307",  307,  0,   60),
    ("d307",  307,  306, 60),
    ("d462",  462,  0,   44),
    ("d462",  462,  461, 44),
    ("d616",  616,  0,   36),
    ("d616",  616,  615, 36),
    ("d924",  924,  0,   24),
    ("d924",  924,  923, 24),
    ("d1232", 1232, 0,   20),
    ("d1232", 1232, 1231, 20),
]

# --- mpmath function oracles ----------------------------------------------
#
# Each entry: (name, oracle, domain_name)
#
# Oracles receive an mpf and return an mpf. The harness uses the
# function name to dispatch to the corresponding strict kernel.

FUNCS: list[tuple[str, Callable[[mpf], mpf], str]] = [
    ("ln",    ln,                       "positive"),
    ("exp",   exp,                      "moderate_real"),
    ("exp2",  lambda x: power(2, x),    "moderate_real"),
    ("log2",  lambda x: log(x, 2),      "positive"),
    ("log10", lambda x: log(x, 10),     "positive"),
    ("sin",   sin,                      "real"),
    ("cos",   cos,                      "real"),
    ("tan",   tan,                      "tan_safe"),
    ("atan",  atan,                     "real"),
    ("asin",  asin,                     "unit_interval"),
    ("acos",  acos,                     "unit_interval"),
    ("sinh",  sinh,                     "moderate_real"),
    ("cosh",  cosh,                     "moderate_real"),
    ("tanh",  tanh,                     "real"),
    ("asinh", asinh,                    "real"),
    ("acosh", acosh,                    "ge_one"),
    ("atanh", atanh,                    "open_unit"),
    ("sqrt",  sqrt,                     "nonneg"),
    ("cbrt",  real_cbrt,                "real"),
]

# Two-argument oracles. The harness emits a 4-column table
# (`input_raw \t input2_raw \t floor_raw \t cls`) for these; the second
# column is the second storage integer (`base` for log, `x` for atan2,
# `exp` for powf). Domain guards in `two_arg_inputs` keep every emitted
# pair inside the kernel's legal domain so the harness never panics.

TWO_ARG_FUNCS: list[tuple[str, Callable[[mpf, mpf], mpf], str]] = [
    # log(value, base) — the crate's `value.log_strict_with(base)`.
    ("log",   lambda v, b: log(v) / log(b),  "log_base"),
    # atan2(y, x) — the crate's `y.atan2_strict_with(x)`.
    ("atan2", lambda y, x: atan2(y, x),      "atan2"),
    # powf(base, exp) — the crate's `base.powf_strict_with(exp)`.
    ("powf",  lambda b, e: power(b, e),      "powf"),
]

# --- Helpers --------------------------------------------------------------


def _exact_integer_root(n: int, power: int) -> int | None:
    """Return `r` if `n == r**power` exactly (integer `r >= 0`), else
    `None`. Uses Newton's integer iteration so no float rounding can
    misjudge the perfect-power test, even for the 1230-digit magnitudes
    that arise at the widest tier.

    `n` must be non-negative; callers handle the sign for odd `power`.
    """
    if n < 0:
        return None
    if n == 0:
        return 0
    # Integer power-root by Newton iteration on `r**power == n`. Seed
    # from the bit length (no float — `n` can be 1000+ digits and would
    # overflow `float`). `r0 = 2**ceil(bitlen / power)` is a safe upper
    # seed; Newton descends monotonically to the floor root.
    bit_len = n.bit_length()
    r = 1 << ((bit_len + power - 1) // power)
    while True:
        # Newton step for floor(n**(1/power)).
        t = ((power - 1) * r + n // (r ** (power - 1))) // power
        if t >= r:
            break
        r = t
    # `r` is now floor(n**(1/power)); confirm it is an exact root.
    while r ** power > n:
        r -= 1
    while (r + 1) ** power <= n:
        r += 1
    return r if r ** power == n else None


def exact_algebraic_root(func_name: str, raw: int, scale: int) -> int | None:
    """Detect an EXACT algebraic root for `sqrt`/`cbrt` at the tier scale.

    For `x = raw / 10**scale`, the kernel's storage output integer is
    `round(f(x) * 10**scale)`. That is an exact integer with a zero
    residual precisely when an integer power-root exists:

      * `sqrt(x) * 10**scale = sqrt(raw * 10**scale)` — exact iff
        `raw * 10**scale` is a perfect square.
      * `cbrt(x) * 10**scale = cbrt(raw * 10**(2*scale))` — exact iff
        `raw * 10**(2*scale)` is a perfect cube (sign-symmetric).

    A finite-precision mpmath oracle reports a sub-LSB residual at these
    points (e.g. `cbrt(10**-615) = 10**-205` at D1232<615>), which would
    misclassify the tie and demand a directed bump the exact value does
    not warrant. Returning the exact integer here pins the result to the
    `Z` (no-bump) class for every rounding mode.

    Returns the exact `floor_raw` (= the exact result integer) or `None`
    when the input is not an exact algebraic point.
    """
    if func_name == "sqrt":
        if raw < 0:
            return None
        return _exact_integer_root(raw * (10 ** scale), 2)
    if func_name == "cbrt":
        n = raw * (10 ** (2 * scale))
        if n >= 0:
            return _exact_integer_root(n, 3)
        r = _exact_integer_root(-n, 3)
        return None if r is None else -r
    return None


def exact_log_base(value_raw: int, base_raw: int, scale: int) -> int | None:
    """Detect an EXACT `log_base(value)` at the tier scale.

    `log_b(v) = k` exactly when `v == b^k` for a non-negative integer
    `k`, with `v = value_raw / 10**scale` and `b = base_raw / 10**scale`.
    The exact storage result is then `k * 10**scale`.

    The finite-precision oracle computes `log(v) / log(b)` and lands a
    sub-LSB residual short of (or past) the integer `k` — e.g.
    `log(32)/log(2)` evaluates to `5 − 5.9·10**-682` at 700 dps, whose
    floor is `5·10**scale − 1` and whose fractional class is `G`,
    demanding a directed bump the true value (exactly 5) does not
    warrant. Pinning the exact integer here classifies the point as
    `Z` (no bump) under every rounding mode — the same treatment
    `exact_algebraic_root` gives the perfect square / cube roots.

    Detection is pure integer arithmetic: clearing the common `10**scale`
    denominators, `v == b^k` is `value_raw · 10**(scale·(k−1)) ==
    base_raw**k`. We iterate `k` upward while `base_raw**k` has not
    overshot `value_raw · 10**(scale·(k−1))`; the base magnitude grows
    geometrically so the loop is short.

    Returns the exact `floor_raw` (`= k · 10**scale`) or `None` when the
    pair is not an exact power.
    """
    one = 10 ** scale
    if value_raw <= 0 or base_raw <= 0 or base_raw == one:
        return None
    # k = 0 -> v == 1 -> log = 0 (any base): value_raw == 10**scale.
    if value_raw == one:
        return 0
    if value_raw < one and base_raw > one:
        # 0 < v < 1 with b > 1 -> log_b(v) < 0, never an exact
        # non-negative power; the oracle path classifies it.
        return None
    # Scan k = 1, 2, 3, … testing `value_raw · 10**(scale·(k−1)) ==
    # base_raw**k`. Both sides are exact integers.
    #
    # Termination: for a true power `v = b**k`, multiplying out the
    # `10**scale` denominators gives `base_raw**k == value_raw ·
    # 10**(scale·(k−1))`, so `(base_raw / one)**k == value_raw / one`.
    # The right side is the fixed ratio `v <= max representable`, so any
    # genuine `k` satisfies `(base_raw/one)**k == v`, bounded by
    # `k <= log(v) / log(base_raw/one)`. We cap the scan at the number of
    # base-`b` steps needed to reach `value_raw` (computed without floats
    # by counting how many times `base_raw` multiplies into `one` before
    # the running power's *integer part* exceeds `value_raw // one + 1`),
    # which both terminates the near-1 bases and admits every real power.
    target_int = value_raw // one + 1   # ceil(v) upper bound
    # Hard iteration cap. A genuine exact power `v = b**k` in the emitted
    # rosters has small `k` (the generators cap their `log_b(b**k)`
    # sweeps at `k <= 30`); a result `k · one` beyond this cap is far
    # larger than any value the tables hold. The cap also bounds the scan
    # for bases arbitrarily close to 1 (the ill-conditioning probe
    # `b = 1 + 10**-kk`), where `b**k` climbs toward `ceil(v)` only over
    # `~log(v)/log(b)` steps.
    k_cap = 256
    base_pow = 1                        # base_raw ** k accumulator
    k = 0
    while k < k_cap:
        k += 1
        base_pow *= base_raw
        lhs = value_raw * (10 ** (scale * (k - 1)))
        if base_pow == lhs:
            return k * one
        if base_pow > lhs:
            # `base_raw**k` has overshot the (also-growing) target — no
            # integer power matches.
            return None
        # Integer part of `b**k = base_pow / 10**(scale·k)`. Once it
        # exceeds `ceil(v)` the running power can no longer equal `v`
        # (b**k is monotone increasing for b > 1), so stop early.
        bk_int = base_pow // (10 ** (scale * k))
        if bk_int > target_int:
            return None
    return None


def exact_powf(base_raw: int, exp_raw: int, scale: int) -> int | None:
    """Detect an EXACT `base ** exp` at the tier scale.

    `base ** exp` is exactly the integer storage value `r · 10**scale`
    when `base = base_raw / 10**scale`, `exp = exp_raw / 10**scale`, and
    the real power `(base_raw/10**scale) ** (exp_raw/10**scale)` lands on
    a representable grid point with a zero residual. The headline case is
    a perfect-power base with a unit-fraction exponent — e.g.
    `4 ** 0.5 = 2`, `8 ** (1/3) = 2`, `9 ** 0.5 = 3` — where the
    `exp(y·ln x)` evaluation carries a sub-LSB error and rounds 1 LSB
    short under the directed modes even though the true value is exact.

    Reduce `exp = exp_raw / 10**scale` to lowest terms `p / q`. Then
    `base**(p/q)` is an exact integer iff `base**p` is a perfect `q`-th
    power. We work in cleared-denominator integer form:
    `base = base_raw / 10**scale`, so
    `base**p = base_raw**p / 10**(scale·p)`, and the `q`-th root is exact
    iff both `base_raw**p` and `10**(scale·p)` are perfect `q`-th powers
    AND the integer roots divide to a whole `r` with `r` representable.
    The result storage value is `r · 10**scale`.

    Returns the exact `floor_raw` (`= r · 10**scale`) or `None` when the
    pair is not an exact power. Negative / zero bases and exponents that
    do not reduce to an exact integer result return `None` (the kernel
    rejects negative bases and the oracle path handles the rest).
    """
    import math

    one = 10 ** scale
    if base_raw <= 0 or exp_raw == 0:
        return None
    # Reduce the exponent fraction exp_raw / 10**scale to lowest terms.
    sign = 1 if exp_raw > 0 else -1
    num = abs(exp_raw)
    den = one
    g = math.gcd(num, den)
    p = num // g          # exponent numerator (>= 1)
    q = den // g          # exponent denominator (>= 1)

    # Only small-fraction exponents can land on an exact representable
    # integer for a representable base (e.g. p/q ∈ {1/2, 1/3, 2/3, 3/2,
    # …}); a fraction that does not reduce to small p,q is irrational in
    # the exponent and the result is irrational — the finite-precision
    # oracle classifies it. Bounding p,q also keeps `base_raw**p` and the
    # `q`-th-root radicand from exploding to astronomically large
    # integers for the random-exponent samples.
    P_Q_CAP = 8
    if p > P_Q_CAP or q > P_Q_CAP:
        return None

    # base = base_raw / 10**scale, so the real result r satisfies
    #   r**q = base**p = base_raw**p / 10**(scale·p).
    # Clear the denominator by scaling r up by 10**(scale·p):
    #   (r · 10**(scale·p))**q = base_raw**p · 10**(scale·p·(q−1)).
    # The right side is an exact integer; its exact integer q-th root,
    # when it exists, is `r · 10**(scale·p)`, and `r` is an integer iff
    # that root is divisible by 10**(scale·p).
    radicand = (base_raw ** p) * (10 ** (scale * p * (q - 1)))
    scaled_root = _exact_integer_root(radicand, q)
    if scaled_root is None:
        return None
    denom = 10 ** (scale * p)
    if scaled_root % denom != 0:
        # base**(p/q) is irrational or a non-integer rational at this
        # scale — let the finite-precision oracle classify it.
        return None
    magnitude = scaled_root // denom      # the exact real result value
    if sign < 0:
        # base**(−p/q) = 1 / magnitude; integer only when magnitude == 1.
        if magnitude != 1:
            return None
        magnitude = 1
    return magnitude * one


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

    elif func_name in ("exp", "exp2", "sinh", "cosh"):
        # These grow exponentially — clamp |x| <= the natural ceiling so
        # the result still fits storage. exp/sinh/cosh share e^x growth;
        # exp2 grows like 2^x (ceiling is log2 of the cap).
        if func_name == "exp2":
            max_x_int = max(1, int(0.9 * mp.log(mpf(max_raw) / mpf(one), 2)))
        else:
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

    elif func_name in ("log2", "log10"):
        # Same domain as ln: x > 0.
        cap = min(max_raw, one * (10 ** 8))
        while len(out) < count:
            out.append(rng.randint(1, cap))

    elif func_name in ("asin", "acos"):
        # Domain [-1, 1]: sample raw in [-one, one].
        while len(out) < count:
            out.append(rng.randint(-one, one))

    elif func_name == "tanh":
        # Range (-1, 1); domain all reals but saturates fast — sample a
        # modest band plus a few larger draws.
        band = min(max_raw, 20 * one)
        while len(out) < count:
            out.append(rng.randint(-band, band))

    elif func_name == "asinh":
        # All reals; asinh grows like ln(2x) so large inputs are cheap.
        while len(out) < count:
            scale_factor = rng.choice([1, 10, 1000])
            out.append(rng.randint(-max_raw // scale_factor, max_raw // scale_factor))

    elif func_name == "acosh":
        # Domain x >= 1: sample raw in [one, large].
        cap = min(max_raw, one * (10 ** 6))
        while len(out) < count:
            out.append(rng.randint(one, cap))

    elif func_name == "atanh":
        # Domain (-1, 1): sample strictly inside (avoid |raw| == one).
        while len(out) < count:
            r = rng.randint(-(one - 1), one - 1)
            out.append(r)

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
        if func_name in ("ln", "sqrt", "log2", "log10"):
            raw = rng.randint(1, cap)
        elif func_name in ("acosh",):
            raw = rng.randint(one, min(cap, 10 ** 6 * one))
        elif func_name in ("asin", "acos"):
            raw = rng.randint(-one, one)
        elif func_name in ("atanh",):
            raw = rng.randint(-(one - 1), one - 1)
        elif func_name in ("tanh", "asinh"):
            raw = rng.randint(-10 * one, 10 * one)
        elif func_name in ("exp", "exp2", "sinh", "cosh"):
            # Keep |x| <= ~20 so the result doesn't overflow at narrow
            # scales while the tie hunter scans.
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

def sweep_exponents(scale: int, want: int = 12) -> list[int]:
    """A bounded, geometrically-spaced set of exponents `k` in
    `[0, scale)` for the endpoint branch-point sweeps `±(one - 10^k)`.

    Sweeping every `k` up to `scale` produces ~`scale` huge lines per
    function, which blows the committed budget at the wide tiers (e.g.
    615 lines × ~900 chars). A geometric sample walks the
    square-root-cancellation band — both the digits just inside ±1 and
    the far interior — at a fixed small cost independent of `scale`.
    """
    if scale <= 1:
        return list(range(scale))
    if scale <= want:
        return list(range(scale))
    # Geometric spread of exponents from 0 to scale-1, deduped.
    out = set()
    for i in range(want):
        k = round((scale - 1) * (i / (want - 1)))
        out.add(k)
    return sorted(out)


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
        # Exact squares (n^2) with ±1-LSB round-decision neighbours, plus
        # the MAX / MAX-ULP boundary pair.
        out = [
            0, 1,
            one,
            100 * one,
            one * 10**6,
            half_max,
            max_raw, max_raw - 1,
        ]
        for n in (2, 3, 4, 10):
            sq = (n ** 2) * one
            out += [sq, sq - 1, sq + 1]
        return out
    if func_name == "cbrt":
        # Exact cubes (n^3) plus the ±1-LSB round-decision neighbours,
        # the sign mirror, and the MAX / MAX-ULP boundary pair.
        out = [
            0, 1, -1,
            one, -one,
            8 * one, -8 * one,
            27 * one, -27 * one,
            half_max, -half_max,
            max_raw, -max_raw,
            max_raw - 1, -(max_raw - 1),
        ]
        for n in (2, 3, 4, 5, 10):
            cube = (n ** 3) * one
            out += [cube, cube - 1, cube + 1, -cube, -(cube - 1), -(cube + 1)]
        return out

    # ── New single-arg edge rosters (taxonomy §per-function checklist) ──

    if func_name == "exp2":
        # exp2(0)=1, exp2(k)=2^k exact at integer k; boundary magnitudes.
        # The integer-k roster spans [1, cap] where cap can be ~1000 at
        # the widest tier; enumerating all of it blows the budget, so we
        # sample a bounded geometric spread of exponents (each is an
        # exact algebraic point exp2(k)=2^k regardless of how many we
        # take) plus the small integers and the boundary pair.
        cap = max(1, int(0.5 * mp.log(mpf(max_raw) / mpf(one), 2)))
        out = [0, 1, -1, one, -one, one // 2, -(one // 2)]
        ks = sorted(set(list(range(1, min(cap, 16) + 1))
                        + [round(cap * i / 8) for i in range(1, 9)]))
        for k in ks:
            if 1 <= k <= cap:
                out += [k * one, -(k * one)]
        return out

    if func_name == "log2":
        # Exact powers of two log2(2^k)=k (algebraic-exact roster), plus
        # log2(1)=0, near-1, near-0+, and the MAX edge.
        out = [1, one, one - 1, one + 1, one // 2, 2 * one]
        v = 2 * one
        k = 1
        while v <= max_raw and k <= 60:
            out.append(v)            # x = 2^k -> log2 = k exact
            v *= 2
            k += 1
        out += [one * 10 ** 6, max_raw, max_raw - 1]
        return out

    if func_name == "log10":
        # Exact powers of ten log10(10^k)=k (the headline decade roster),
        # plus log10(1)=0, near-1, near-0+, MAX edge.
        out = [1, one, one - 1, one + 1, one // 2, 5 * one]
        v = 10 * one
        k = 1
        while v <= max_raw and k <= 40:
            out.append(v)            # x = 10^k -> log10 = k exact
            v *= 10
            k += 1
        out += [max_raw, max_raw - 1]
        return out

    if func_name in ("asin", "acos"):
        # Endpoint branch-points ±one and the √-cancellation sweep
        # ±(one - 10^k) walking toward ±1, plus 0 and small magnitudes.
        out = [0, one, -one, one - 1, -(one - 1), one // 2, -(one // 2)]
        for k in sweep_exponents(scale):
            out += [one - 10 ** k, -(one - 10 ** k)]
        return out

    if func_name in ("sinh", "cosh"):
        # cosh(0)=1 (global min, monotonicity turn), sinh(0)=0, near 0,
        # ±1, and the largest-fitting magnitude (overflow edge handled by
        # the just-fits pair below).
        cap = max(1, int(0.5 * mp.log(mpf(max_raw) / mpf(one))))
        return [
            0, 1, -1, one, -one, one // 2, -(one // 2),
            cap * one, -(cap * one),
        ]

    if func_name == "tanh":
        # tanh(0)=0, near 0, sign mirror, plus saturation: outputs crowd
        # the ±1 grid line. The exact saturation grid-line input is added
        # by the saturation roster; here cover the approach band.
        return [
            0, 1, -1, one, -one, one // 2, -(one // 2),
            5 * one, -5 * one, 10 * one, -10 * one,
        ]

    if func_name == "asinh":
        # asinh(0)=0, near 0, sign mirror, large (ln(2x) path).
        return [
            0, 1, -1, one, -one, one // 2, -(one // 2),
            10 * one, -10 * one, one * 10 ** 6, -(one * 10 ** 6),
            max_raw, -max_raw,
        ]

    if func_name == "acosh":
        # acosh(1)=0 branch-point; near-1+ sweep one + 10^k; large x.
        out = [one, one + 1, 2 * one, 10 * one, one * 10 ** 6]
        for k in sweep_exponents(scale):
            if one + 10 ** k <= max_raw:
                out.append(one + 10 ** k)
        return out

    if func_name == "atanh":
        # atanh(0)=0; near ±1 asymptote sweep ±(one - 10^k).
        out = [0, one // 2, -(one // 2), one - 1, -(one - 1)]
        for k in sweep_exponents(scale):
            out += [one - 10 ** k, -(one - 10 ** k)]
        return out

    return []


def overflow_edge_inputs(func_name: str, oracle: Callable[[mpf], mpf],
                         scale: int, max_raw: int) -> list[int]:
    """The largest input whose result still FITS storage (the just-fits
    boundary of taxonomy class 6), found by integer bisection on the raw
    input.

    The generators previously DROPPED every overflowing case rather than
    asserting the boundary; this pins the just-fits case as a golden line
    so the overflow edge is exercised, not silently skipped. The
    +1-ULP-overflows sibling is covered by the dedicated saturation/panic
    unit tests (`*_panic.rs`), which assert the kernel's documented
    contract beyond storage range.
    """
    if func_name not in ("exp", "exp2", "sinh", "cosh"):
        return []
    one = 10 ** scale

    # Analytic upper bound on the just-fits input: the result reaches
    # `max_raw / 10^scale` when x ≈ inverse_growth(max_raw/one). Bisecting
    # blindly to `max_raw` would feed the oracle astronomically large
    # arguments (e.g. exp(1e10)) that never raise but never return; cap
    # `hi` an LSB above the analytic threshold so the search stays cheap.
    ratio = mpf(max_raw) / mpf(one)
    if func_name == "exp2":
        x_thresh = mp.log(ratio, 2)
    elif func_name in ("sinh", "cosh"):
        # sinh/cosh ≈ e^x / 2, so x ≈ ln(2 * ratio).
        x_thresh = mp.log(2 * ratio)
    else:  # exp
        x_thresh = mp.log(ratio)
    hi_cap = int(mp.ceil(x_thresh * one)) + one
    hi_cap = min(hi_cap, max_raw)

    def fits(raw: int) -> bool:
        if abs(raw) > max_raw:
            return False
        y = safe_call(oracle, from_raw(raw, scale))
        if y is None:
            return False
        floor_raw, _ = floor_and_class(y, scale)
        return abs(floor_raw) + 1 <= max_raw

    # Bisect for the largest positive raw whose result fits.
    lo, hi = one, hi_cap
    if hi <= lo or not fits(lo):
        return []
    while lo < hi:
        mid = (lo + hi + 1) >> 1
        if fits(mid):
            lo = mid
        else:
            hi = mid - 1
    out = [lo]
    # cosh is even (negative side mirrors); sinh/exp negatives never
    # overflow (they shrink toward 0), so only the positive edge matters.
    return out


def large_trig_inputs(func_name: str, scale: int, max_raw: int) -> list[int]:
    """Very-large trig arguments K·π (taxonomy class 9) that stress the
    full Payne-Hanek range reduction far beyond the 8–12π cap of the
    random/near-boundary samplers."""
    if func_name not in ("sin", "cos", "tan"):
        return []
    one = 10 ** scale
    out: list[int] = []
    # K spans an order of magnitude beyond the 8–12π cap of the random
    # samplers (1000π ≈ 3142 radians) — deep enough to drive a multi-stage
    # range reduction without exceeding the shipped reduction precision.
    for K in (50, 100, 500, 1000):
        raw = int(mpf(K) * mp.pi * one)
        if 0 < raw <= max_raw:
            if func_name == "tan":
                # Skip if within 0.001 of a pole.
                x = from_raw(raw, scale)
                k = round(float(x / mp.pi - mpf("0.5")))
                pole = (mpf(k) + mpf("0.5")) * mp.pi
                if abs(x - pole) < mpf("0.001"):
                    continue
            out += [raw, -raw]
    return out


def saturation_inputs(func_name: str, scale: int, max_raw: int) -> list[int]:
    """Saturation grid-line inputs (taxonomy class 5): for tanh, the
    input where the output crowds the ±1 grid line so the round-to-1 vs
    round-to-(1−ULP) decision is exercised. atan saturates to ±π/2 which
    is well inside storage, so its large-x draws already cover it."""
    if func_name != "tanh":
        return []
    one = 10 ** scale
    out: list[int] = []
    # tanh(x) -> 1; find x where tanh(x) = 1 - c/10^scale for small c via
    # atanh(1 - c·10^-scale). Sweep a few c so several grid lines near 1
    # are probed.
    for c in (1, 2, 5, 10):
        target = mpf(1) - mpf(c) / (mpf(10) ** scale)
        x = atanh(target)
        raw = int(mp.floor(x * one))
        for r in (raw, raw + 1, raw + 2):
            if 0 < r <= max_raw:
                out += [r, -r]
    return out


def two_arg_inputs(func_name: str, scale: int, max_raw: int, count: int,
                   rng: random.Random) -> list[tuple[int, int]]:
    """`(input_raw, input2_raw)` pairs for the two-argument oracles, all
    inside the kernel's legal domain.

      * `log`   — `(value, base)`: value > 0, base > 0, base != 1. Adds
        the exact roster `log_b(b^k)=k` and the base-near-1 ill-
        conditioning probe.
      * `atan2` — `(y, x)`: the eight octants, the four axes, and the
        negative-x branch-cut neighbourhood.
      * `powf`  — `(base, exp)`: base > 0 (negative-base ^ non-integer is
        a domain error), with the integer-exp exact roster `x^0=1`,
        `x^1=x`, `x^2`, `x^3`.
    """
    one = 10 ** scale
    out: list[tuple[int, int]] = []

    if func_name == "log":
        # Exact roster: base b in {2,3,10}, value = b^k -> log = k exact.
        for b in (2, 3, 10):
            v = b * one
            k = 1
            while v <= max_raw and k <= 30:
                out.append((v, b * one))   # log_b(b^k) = k
                v *= b
                k += 1
            out.append((one, b * one))     # log_b(1) = 0
            out.append((b * one, b * one))  # log_b(b) = 1
        # Base-near-1 ill-conditioning (denominator ln(base) -> 0).
        for kk in range(1, min(scale, 6) + 1):
            out.append((2 * one, one + 10 ** (scale - kk)))
        # Random in-domain pairs.
        cap = min(max_raw, one * 10 ** 4)
        while len(out) < count:
            v = rng.randint(1, cap)
            b = rng.randint(2, cap)
            if b == one:
                continue
            out.append((v, b))
        return out[:count] if count >= 0 else out

    if func_name == "atan2":
        # Axes and octant representatives plus the branch-cut neighbours.
        u = one
        axes = [
            (u, 0), (0, u), (-u, 0), (0, -u),       # +x +y -x -y axes
            (u, u), (-u, u), (-u, -u), (u, -u),     # four diagonals
        ]
        out.extend(axes)
        # Negative-x branch cut: x<0 with y just above / below 0.
        for dy in (1, 2, 5):
            out.append((dy, -u))         # y -> 0+ on the cut
            out.append((-dy, -u))        # y -> 0- on the cut
        cap = min(max_raw, one * 10 ** 4)
        while len(out) < count:
            y = rng.randint(-cap, cap)
            x = rng.randint(-cap, cap)
            out.append((y, x))
        return out[:count] if count >= 0 else out

    if func_name == "powf":
        # Integer-exp exact roster: base^0=1, base^1=base, base^2, base^3.
        for b in (2, 3, 5, 10):
            base = b * one
            if base <= max_raw:
                out.append((base, 0))             # b^0 = 1
                out.append((base, one))           # b^1 = b
                if base * base // one <= max_raw:  # crude fit guard
                    out.append((base, 2 * one))   # b^2
        out.append((one, 7 * one))                # 1^y = 1
        out.append((4 * one, one // 2))           # 4^0.5 = 2 (exact sqrt)
        out.append((8 * one, one // 3 if one % 3 == 0 else one // 3))  # cube-ish
        # Random in-domain pairs (positive base, moderate exponent).
        cap = min(max_raw, one * 100)
        while len(out) < count:
            b = rng.randint(1, cap)
            e = rng.randint(-3 * one, 3 * one)
            # Reject pairs whose result clearly overflows or underflows.
            y = safe_call_two(power, from_raw(b, scale), from_raw(e, scale))
            if y is None:
                continue
            out.append((b, e))
        return out[:count] if count >= 0 else out

    return out


def safe_call_two(oracle: Callable[[mpf, mpf], mpf], a: mpf, b: mpf) -> mpf | None:
    """Two-argument `safe_call`."""
    try:
        y = oracle(a, b)
    except (ValueError, ZeroDivisionError, OverflowError, ArithmeticError):
        return None
    if isinstance(y, mpc):
        return None
    return y


def monotonicity_inputs(base_inputs: list[int], max_raw: int) -> list[int]:
    """For taxonomy class 8: emit `x+1` next to a sample of base inputs
    so adjacent rounded outputs can be cross-checked for ordering. The
    pair lands as two golden lines; the harness asserts each is correctly
    rounded, and `ulp_proptest` carries the explicit ordering relation."""
    out: list[int] = []
    for x in base_inputs[:8]:
        if abs(x) + 1 <= max_raw:
            out.append(x + 1)
    return out


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


TWO_ARG_HEADER_LINES = [
    "# golden precision oracle table (two-argument)",
    "# generated by scripts/gen_golden_precision.py",
    "# each line: <input_raw>\\t<input2_raw>\\t<floor_raw>\\t<cls>",
    "# input_raw   first  storage integer (value/y/base) at the tier scale.",
    "# input2_raw  second storage integer (base/x/exp) at the tier scale.",
    "# floor_raw   floor(f(a,b) * 10**scale), rounded toward negative infinity.",
    "# cls         fractional class of f(a,b)*10**scale - floor_raw, in [0,1):",
    "#               Z = exact (frac == 0), L = 0<frac<0.5, E = frac==0.5, G = 0.5<frac<1.",
    "# The correctly-rounded result under any RoundingMode is floor_raw or",
    "# floor_raw+1, derived from (floor_raw, cls, sign) by the harness.",
    "# Computed by mpmath at max(700, 2*SCALE + 64)-digit working precision.",
]


def emit_file(path: Path, cases: list[tuple[int, int, str]]) -> int:
    """Write a single-argument golden table. Returns the file byte count."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as f:
        for line in HEADER_LINES:
            f.write(line + "\n")
        for raw_in, floor_raw, cls in cases:
            f.write(f"{raw_in}\t{floor_raw}\t{cls}\n")
    return path.stat().st_size


def emit_two_arg_file(path: Path,
                      cases: list[tuple[int, int, int, str]]) -> int:
    """Write a two-argument golden table. Returns the file byte count."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="\n") as f:
        for line in TWO_ARG_HEADER_LINES:
            f.write(line + "\n")
        for a, b, floor_raw, cls in cases:
            f.write(f"{a}\t{b}\t{floor_raw}\t{cls}\n")
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


# The expanded surface (22 functions vs the original 8) would blow the
# 5 MB committed budget at the original per-cell counts. We scale every
# category count by this factor before splitting; the edge rosters and
# the small explicit class lists (overflow / saturation / large-trig /
# two-arg) are NOT scaled, so every function and edge class stays
# represented even at the narrowest per-cell budget.
COUNT_SCALE = 0.34


def _csv_filter(flag: str) -> set[str] | None:
    """Parse `--flag=a,b,c` from argv into a lowercase set, or None if absent.
    When None the corresponding axis is unrestricted (full-corpus behaviour)."""
    prefix = f"--{flag}="
    for arg in sys.argv[1:]:
        if arg.startswith(prefix):
            return {v.strip().lower() for v in arg[len(prefix):].split(",") if v.strip()}
    return None


def main() -> None:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    total_bytes = 0
    total_cases = 0

    # Optional scoping filters (absent => full corpus, the default behaviour):
    #   --only-alias=d307,d462   --only-scale=30   --only-func=exp
    only_alias = _csv_filter("only-alias")
    only_scale = _csv_filter("only-scale")
    only_func = _csv_filter("only-func")

    for alias, capacity, scale, base_count in TIERS:
        if only_alias is not None and alias.lower() not in only_alias:
            continue
        if only_scale is not None and str(scale) not in only_scale:
            continue
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
        counts = category_counts(max(8, int(base_count * COUNT_SCALE)))
        # Lift mpmath working precision so the oracle's intermediate
        # squarings stay safely above the tier's storage LSB. The
        # `2*SCALE + 64` floor covers the worst case where the oracle
        # squares an LSB-scale residual; the global lower bound of 700
        # keeps the narrow tiers from running unnecessarily slow on
        # small `2*SCALE` values.
        # `2*scale + 64` covers the canonical cells (where x is moderate and
        # the result is ~scale-sized). At a LOW scale the representable input
        # range is huge, so the result can fill the tier's whole `capacity`
        # (e.g. exp(x) at D924<30> reaches ~894 integer digits); the oracle
        # then needs `>= capacity` significant digits or it truncates the true
        # value. Take the max so both regimes are covered.
        mp.dps = max(700, 2 * scale + 64, capacity + 96)

        for func_name, oracle, _domain in FUNCS:
            if only_func is not None and func_name.lower() not in only_func:
                continue
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

            # Overflow just-fits boundary (asserted, not dropped).
            inputs.extend(overflow_edge_inputs(func_name, oracle, scale, max_raw))

            # Very-large trig arguments (full Payne-Hanek).
            inputs.extend(large_trig_inputs(func_name, scale, max_raw))

            # Saturation grid-lines (tanh -> ±1).
            inputs.extend(saturation_inputs(func_name, scale, max_raw))

            # Monotonicity pairs: x+1 beside a sample of the inputs above.
            inputs.extend(monotonicity_inputs(inputs, max_raw))

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
                # Exact algebraic points (perfect squares for sqrt,
                # perfect cubes for cbrt) are classified symbolically via
                # integer arithmetic, bypassing the oracle's finite-
                # precision residual: the exact result is the `Z`
                # (no-bump) class under every rounding mode.
                exact = exact_algebraic_root(func_name, raw_in, scale)
                if exact is not None:
                    floor_raw, cls = exact, "Z"
                    if abs(floor_raw) > max_raw or abs(floor_raw) + 1 > max_raw:
                        continue
                    cases.append((raw_in, floor_raw, cls))
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

        # ── Two-argument oracles (log / atan2 / powf) ──────────────────
        two_arg_count = max(20, int(base_count * COUNT_SCALE))
        for func_name, oracle2, _domain in TWO_ARG_FUNCS:
            if only_func is not None and func_name.lower() not in only_func:
                continue
            seed_key = f"{alias}-{scale}-{func_name}-2arg-v1"
            rng = random.Random(seed_key)
            pairs = two_arg_inputs(func_name, scale, max_raw, two_arg_count, rng)

            # Dedupe pairs, preserving order.
            seen2: set[tuple[int, int]] = set()
            deduped2: list[tuple[int, int]] = []
            for p in pairs:
                if p not in seen2:
                    seen2.add(p)
                    deduped2.append(p)

            cases2: list[tuple[int, int, int, str]] = []
            for a_raw, b_raw in deduped2:
                if abs(a_raw) > max_raw or abs(b_raw) > max_raw:
                    continue
                # Exact-power points: `log_b(b^k) = k` and the
                # perfect-power `base**(p/q)` are exact integers whose
                # finite-precision `log(v)/log(b)` / `exp(y·ln x)` oracle
                # value carries a sub-LSB residual. Classify them
                # symbolically via integer arithmetic (mirroring
                # `exact_algebraic_root` for sqrt / cbrt) so they pin to
                # the `Z` (no-bump) class under every rounding mode.
                exact2: int | None = None
                if func_name == "log":
                    exact2 = exact_log_base(a_raw, b_raw, scale)
                elif func_name == "powf":
                    exact2 = exact_powf(a_raw, b_raw, scale)
                if exact2 is not None:
                    if abs(exact2) > max_raw or abs(exact2) + 1 > max_raw:
                        continue
                    cases2.append((a_raw, b_raw, exact2, "Z"))
                    continue
                a = from_raw(a_raw, scale)
                b = from_raw(b_raw, scale)
                y = safe_call_two(oracle2, a, b)
                if y is None:
                    continue
                floor_raw, cls = floor_and_class(y, scale)
                if abs(floor_raw) > max_raw or abs(floor_raw) + 1 > max_raw:
                    continue
                cases2.append((a_raw, b_raw, floor_raw, cls))

            if not cases2:
                continue

            out_path = OUT_DIR / f"{func_name}_{alias}_s{scale}.txt"
            file_bytes = emit_two_arg_file(out_path, cases2)
            total_bytes += file_bytes
            total_cases += len(cases2)
            print(f"  {out_path.relative_to(ROOT)}: "
                  f"{len(cases2)} cases, {file_bytes} bytes (2-arg)")

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

    # ── New single-arg near-boundary clusters ──────────────────────────

    if func_name == "exp2":
        # Cluster around 0 and ±1 (mirror of exp).
        out = []
        for _ in range(count // 2):
            out.append(rng.randint(-one // 100, one // 100))
        for _ in range(count - count // 2):
            out.append(one + rng.randint(-one // 100, one // 100))
        return out

    if func_name in ("log2", "log10"):
        # Cluster around x = 1 (the zero crossing) and the relevant base.
        base_x = 2 if func_name == "log2" else 10
        out = []
        for _ in range(count // 2):
            out.append(max(1, one + rng.randint(-one // 100, one // 100)))
        for _ in range(count - count // 2):
            out.append(max(1, base_x * one + rng.randint(-one // 100, one // 100)))
        return out

    if func_name in ("asin", "acos"):
        # Cluster around 0 and near the ±1 branch-points.
        out = []
        anchors = [0, one - one // 50, -(one - one // 50)]
        for _ in range(count):
            a = rng.choice(anchors)
            delta = rng.randint(-one // 100, one // 100)
            v = a + delta
            v = max(-one, min(one, v))
            out.append(v)
        return out

    if func_name in ("sinh", "cosh"):
        # Cluster around 0 and ±1.
        out = []
        anchors = [0, one, -one]
        for _ in range(count):
            a = rng.choice(anchors)
            out.append(a + rng.randint(-one // 100, one // 100))
        return out

    if func_name == "tanh":
        # Cluster around 0 (tanh≈x) and the saturation knee ±3..±5.
        out = []
        anchors = [0, 3 * one, -3 * one]
        for _ in range(count):
            a = rng.choice(anchors)
            out.append(a + rng.randint(-one // 100, one // 100))
        return out

    if func_name == "asinh":
        out = []
        anchors = [0, one, -one]
        for _ in range(count):
            a = rng.choice(anchors)
            out.append(a + rng.randint(-one // 100, one // 100))
        return out

    if func_name == "acosh":
        # Cluster just above the x = 1 branch-point.
        out = []
        for _ in range(count):
            out.append(one + rng.randint(1, max(2, one // 50)))
        return out

    if func_name == "atanh":
        # Cluster around 0 and near the ±1 asymptotes (inside the domain).
        out = []
        anchors = [0, one - one // 50, -(one - one // 50)]
        for _ in range(count):
            a = rng.choice(anchors)
            delta = rng.randint(-one // 100, one // 100)
            v = a + delta
            v = max(-(one - 1), min(one - 1, v))
            out.append(v)
        return out

    return []


if __name__ == "__main__":
    main()
