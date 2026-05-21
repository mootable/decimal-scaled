# Precision testing

This page explains how the crate validates its 0.5 ULP correctness
contract — the headline guarantee that strict transcendentals
(`ln_strict`, `exp_strict`, `sin_strict`, `cos_strict`, `tan_strict`,
`atan_strict`, `sqrt_strict`, `cbrt_strict`, …) return the
**correctly-rounded** result: the true real value of the function
rounded to the type's last representable place under the active
rounding mode. "Correctly rounded" is the strong reading of 0.5 ULP
— not merely *within* half a ULP (faithful rounding), but the
*exact* nearest representable, with **zero** tolerance.

The contract is proved at `delta == 0` storage LSB against a
high-precision [mpmath](https://mpmath.org/) oracle, for **every**
[`RoundingMode`](../src/support/rounding.rs) — `HalfToEven`,
`HalfAwayFromZero`, `HalfTowardZero`, `Trunc`, `Floor`, `Ceiling` —
across **all thirteen** decimal widths at their design-target scale.

There are four independent layers, each catching a different
failure mode.

## Layer 1 — hand-computed truth tables

`tests/precision_strict_05_ulp.rs` lists hand-computed truth values
at D38<12> for every constant and strict transcendental, computed
from canonical 35-digit references and rounded half-to-even at the
storage LSB. Each case asserts the kernel returns the *correctly
rounded* value at the storage scale.

It is the only table the crate maintainers transcribe by hand, and
it is compile-gated to `HalfToEven`. The full correctly-rounded
proof — bit-exact under *every* `RoundingMode`, across every tier —
lives in Layer 3 (`ulp_strict_golden.rs`); the other layers exist
because hand-computing per-mode tables for every tier × every
function is impractical.

## Layer 2 — internal cross-witness tables

`tests/wide_strict_transcendentals.rs` and
`tests/precision_wide_baseline.rs` validate the wide tiers by
cross-witness: compute a value at the target's storage and scale,
compute the reference at the same scale with a wider storage type,
rescale, and assert ±1 LSB agreement.

Cross-witness is fast and catches storage-bit divergences between
tiers — but a kernel bug *shared* between the narrow and wide
paths will be invisible to it, because both sides agree on the
wrong answer. The next layer addresses that.

## Layer 3 — mpmath-oracle golden tables (the correctness proof)

`tests/ulp_strict_golden.rs` reads pre-computed `.txt` tables from
`tests/golden/` and asserts kernel results are the **correctly
rounded** value — bit-exact (`delta == 0` storage LSB, ZERO
tolerance) with the [mpmath](https://mpmath.org/) oracle
(BSD-3-Clause) at every tier's design-target SCALE, under **every**
`RoundingMode`. This is the definitive proof of the headline
guarantee.

Tables live at `tests/golden/<func>_d<N>_s<S>.txt`, with one
`<input_raw>\t<floor_raw>\t<cls>` per line:

- `input_raw` — the storage integer of x at the tier scale.
- `floor_raw` — `floor(f(x) · 10^SCALE)`, rounded toward negative
  infinity. Mode-independent.
- `cls` — the fractional class of `f(x)·10^SCALE − floor_raw` in
  `[0, 1)`: `Z` exact, `L` below half, `E` exact half-tie, `G` above
  half.

From `(floor_raw, cls)` and the sign of the value, the harness
derives the correctly-rounded integer for **each** of the six
`RoundingMode` variants (`HalfToEven`, `HalfAwayFromZero`,
`HalfTowardZero`, `Trunc`, `Floor`, `Ceiling`) in-test — one table
covers all modes, no per-mode tables — and asserts the kernel's
`*_strict_with(mode)` output equals it exactly. The oracle computes
`floor_raw` / `cls` at `max(700, 2·SCALE + 64)` decimal digits of
working precision (see "Oracle working precision" below).

Tiers covered: **all thirteen** decimal widths at their design-target
SCALE — D9<4>, D18<9>, D38<19>, D57<28>, D76<35>, D115<57>, D153<76>,
D230<115>, D307<150>, D462<230>, D616<308>, D924<460>, D1232<615>.
Functions: ln, exp, sin, cos, tan, atan, sqrt, cbrt.

### Known kernel holes (ignored cells)

The shipped kernels are correctly rounded for the three *nearest*
modes across every tier. Two classes of cell are not yet correctly
rounded and are marked `#[ignore]` in the harness with a reason
string (run them with `cargo test --test ulp_strict_golden --
--include-ignored`):

- **Directed-rounding 1-LSB boundary.** Under `Trunc` / `Floor` /
  `Ceiling`, ln / sin / cos / tan / exp / cbrt are off by exactly one
  LSB when the true value sits sub-LSB on one side of an integer
  output (e.g. `cos` near ±1, `ln` near an integer LSB multiple). The
  nearest modes are exact for the same inputs.
- **D115<57> `exp` large-magnitude precision loss.** A genuine
  precision regression in the D115<57> exp kernel: many-LSB error for
  large `|x|`, failing under *every* mode. This is the one cell whose
  nearest-mode result is also wrong.

Each ignored cell carries a documented reason; removing the `ignore`
once a kernel is fixed and the band runs green is the witness.

### Regenerating the goldens

```sh
pip install mpmath
python scripts/gen_golden_precision.py
```

The script is deterministic — every random draw is seeded from a
fixed per-(width, scale, function) key, and mpmath's working
precision is wider than the widest tier's storage. **Two runs of
the generator must produce byte-identical files.** If they don't,
either the script or mpmath has changed in a way that affects
output; investigate before committing the new tables.

Regenerate when:

- adding a new tier × scale that isn't covered in `TIERS` in the
  generator
- adding a new function to `FUNCS` in the generator
- adding a new decimal width to the `TIERS` table
- increasing case counts (commit footprint budget is ≤ 5 MB; current
  generation lands ~1.5 MB across all thirteen widths)

Do **not** regenerate to "fix" a kernel that fails the gate — the
oracle is the source of truth, the kernel is what's wrong. The
right move when a golden case fails is to investigate the kernel.

### Categories per (tier, function) file

- **near-boundary** — small inputs around the function's natural
  boundary (ln near 1, exp near 0, trig near 0/π/4/π/2, roots near
  perfect squares/cubes).
- **half-ULP-tie** — inputs whose true output is bracketed within
  0.4 LSB of a half-tie point at the tier scale. Hardest tie-
  breaking edge.
- **random-uniform** — deterministic-seeded uniform samples across
  the natural input domain.
- **edge values** — a fixed roster of small / large magnitudes
  (MAX, MIN, MAX/2, ±ULP, …) appropriate to the function.

### Per-width local iteration

The harness is split per width so local iteration is fast:

```sh
cargo test --test ulp_strict_golden --features wide,x-wide,xx-wide,macros --release d76
```

Runs only the D76<35> band — eight functions, a few hundred cases
each, well under a second on a modern CPU.

### Interpreting a failure

The failure message prints the per-(case, mode) tuple for every
mismatch:

```
FAIL: exp d76 mode=Ceiling input=<raw> floor=<raw> cls=<Z|L|E|G> expected=<raw> actual=<raw>
```

The contract is `actual == expected` exactly — `delta == 0` storage
LSB, no tolerance. Any mismatch is either a precision regression or a
rounding-mode bug; investigate the kernel for the matching
`(width, scale, mode)` cell. `mode` and `cls` localise it: a failure
only in `Trunc`/`Floor`/`Ceiling` is a directed-rounding bug; a
failure in the nearest modes too is a precision bug.

The harness reports every failure per file (an audit run needs every
still-failing tuple surfaced so it can be triaged into an `ignore`
or fixed). Read the first few carefully — they usually point at the
worst-case input shape, which generalises.

## Layer 4 — proptest property fuzz

`tests/ulp_proptest.rs` exercises identities that hold without an
external oracle:

- `exp(ln(x)) ≈ x` for positive x
- `ln(exp(x)) ≈ x` for x in `[0, 30]`
- `sin² + cos² ≈ 1` over a wide real domain
- `sqrt(x)² ≈ x` for non-negative x
- `cbrt(x)³ ≈ x` for real x
- `atan(tan(x)) ≈ x` over (-π/4, π/4)
- `tanh(atanh(x)) ≈ x` over (-1, 1)
- sign symmetries: `sin(-x) = -sin(x)`, `cos(-x) = cos(x)`,
  `atan(-x) = -atan(x)`, `cbrt(-x) = -cbrt(x)`

Each block runs 100 cases with a deterministic seed key so a
counterexample minimises the same way every run.

### Tolerance budgets

Round-trip identities accumulate per-call rounding plus derivative
amplification at the second call's evaluation point. Their LSB
budget is **relative to the magnitude of the reference value**:
8 LSB floor plus `|reference| / 10^16` — coarse enough that
legitimate derivative amplification doesn't trip the gate, tight
enough that a kernel that silently drops 5+ guard digits fails the
property.

Sign symmetries use a flat **1 LSB** budget — they are single-pass
relations and the 0.5 ULP contract bounds them directly.

### How proptest seeds work

Each `proptest!` block configures a unique `source_file` label
which feeds the ChaCha RNG. Failure persistence is disabled
(`failure_persistence: None`) so CI runs are hermetic; a flake
isn't accidentally pinned to local-filesystem state.

When a property fails locally, proptest prints the *minimised*
counterexample (`minimal failing input: raw = …`). That's the
input to debug — copy it into a unit test under
`tests/precision_strict_05_ulp.rs` (or whichever file owns the
relevant tier) and iterate against it directly.

## CI gate

[`/.github/workflows/precision.yml`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/precision.yml)
runs all four layers on every PR and push to `main`. The
precision gate is *Required* in branch-protection — there is no
reviewer override. A failed precision check blocks merge full stop.

If your kernel change fails the gate: the contract is fixed at
0.5 ULP. Adjust the kernel (usually a wider `GUARD` constant, or a
different reduction shape) until the gate passes. Do not relax the
tolerance.

## What isn't covered yet

The golden suite now covers **all thirteen** widths × **every**
`RoundingMode` at the design-target SCALE. Remaining gaps:

- **Higher-derivative function families** — `powf`,
  `log_arbitrary_base`, `hypot`, `atan2`, the hyperbolic family. Not
  in the golden `FUNCS` set yet; currently covered by the existing
  wide-tier cross-witness suite.
- **Multiple scales per width** — each width is proved at one
  design-target SCALE; non-target scales rely on cross-witness.
- **Outstanding kernel holes** — the directed-rounding 1-LSB
  boundary cells and the D115<57> `exp` precision cell are tracked as
  `#[ignore]`d tests (see "Known kernel holes" above), not coverage
  gaps: the tests exist and assert, the kernels are what's pending.

When extending coverage, add the new tier × function to
`TIERS` / `FUNCS` in `scripts/gen_golden_precision.py`, regenerate,
commit the new `.txt` files under `tests/golden/`, and add a
matching `decl_band!` block (or per-function `#[test]`) to
`tests/ulp_strict_golden.rs`. Keep the committed footprint under
5 MB.

## Hard-input categories

`scripts/gen_hard_inputs.py` is a sibling generator that appends ten
literature-derived hard-input categories to each existing golden
file under a `## category:` header line. The harness treats
`#` lines as comments so existing parsers see the appended rows as
ordinary data.

> **Note:** this sibling generator still emits the legacy
> two-column (`<input_raw>\t<expected_raw>`) format. The Layer-3
> harness now requires the three-column
> `<input_raw>\t<floor_raw>\t<cls>` format and silently skips any
> row missing the `cls` column, so legacy hard-input rows are
> currently inert. Update `gen_hard_inputs.py` to emit the
> floor-and-class columns (mirroring `gen_golden_precision.py`'s
> `floor_and_class`) before relying on the hard-input corpus again.

The categories, each cited from the precision literature
(license-compatible — papers, not test vectors):

1. **Half-ULP-tie boundaries** — inputs whose true result lands
   within 0.45 LSB of a half-tie at the storage scale.
   *Reference:* Lefèvre, Muller & Toma, "Toward correctly rounded
   transcendentals" (1998); Muller, *Elementary Functions —
   Algorithms and Implementation* (3rd ed., 2016), §10 "Table
   maker's dilemma".
2. **Catastrophic cancellation** — `ln(1 + ε)`, `exp(tiny)`,
   `cos(tiny) ≈ 1 - x²/2`, `sin(tiny) ≈ x`, `sqrt(1 + ε)`,
   `cbrt(1 + ε)`.
   *Reference:* Goldberg, "What every computer scientist should
   know about floating-point arithmetic" (1991) §3; Higham,
   *Accuracy and Stability of Numerical Algorithms* (2nd ed.,
   2002), §1.7.
3. **Range-reduction breakpoints** — `sin`/`cos` near k·π/2,
   `tan` near k·π/4, `exp` near k·ln 2, `atan` near 1.
   *Reference:* Payne & Hanek, "Radian reduction for trigonometric
   functions" (1983); Muller (2016), §11.
4. **Removable singularity / asymptote** — `tan` near π/2 + δ,
   `ln` near 0+, `atan(huge)`, `sqrt` near 0+.
   *Reference:* Kahan archive, "Branch cuts for complex elementary
   functions" (1987).
5. **Inverse-identity round-trip stress** — `sin` near π/2,
   `atan(tan(k·π/8))`, `exp(ln(small))`, `sqrt(x²)`, `cbrt(x³)`.
   *Reference:* Brent & Zimmermann, *Modern Computer Arithmetic*
   (2010), §4.2 "Inverse functions".
6. **Perfect-power ± ULP for roots** — `sqrt(n² ± 1)`,
   `cbrt(n³ ± 1)` for small integer `n`.
   *Reference:* Brent & Zimmermann (2010), §3.5 "Square root", §3.6
   "k-th root".
7. **Constant edges** — inputs at named constants (π, π/2, π/4, e,
   ln 2, …) ± a few LSBs.
   *Reference:* IEEE 754-2019 standard, §9 "Recommended correctly
   rounded functions"; Muller (2016), §9.
8. **Argument-halving cascade** — `atan` near `tan(0.35 · 2⁻ⁿ)`
   for the per-width halving count `n`.
   *Reference:* Muller (2016), §6 on argument reduction; the
   per-width cascade table in `ALGORITHMS.md`.
9. **Stage-2 argument reduction edge for `exp`** — inputs near the
   chosen breakpoint `v / 2ⁿ` for `n ≈ √(precision_bits)`.
   *Reference:* Tang, "Table-driven implementation of the
   exponential function in IEEE floating-point arithmetic" (1989).
10. **Tang-lookup band edges** — `ln` and `exp` inputs at the
    table-index breakpoints `T_i = 1 + i / 2ᵏ` for `k ∈ {7, 8, 9}`
    and at the secondary-index breakpoints `j/N · ln 2` for
    `j = 0..N-1`, `N ∈ {32, 64, 128}`.
    *References:* Tang (1989); Gal & Bachelis, "An accurate
    elementary mathematical library for the IEEE floating-point
    standard" (1991).

The proptest harness `tests/ulp_proptest.rs` mirrors these
categories with per-category input strategies (100 cases each,
deterministic seed). Identity-based assertions
(round-trip / symmetry) substitute for the runtime oracle.

### Oracle working precision

The mpmath oracle's working precision must be wider than the
tier's storage LSB. The hard-input generator uses
`mp.dps = max(700, 2 * SCALE + 64)` per tier, where the factor
of two covers intermediate squarings inside the oracles and the
+64 is a margin against pathological inputs in any one category.

The 700-dps floor keeps narrow tiers (D38, D76, D153) fast while
still wider than the D1232<615> storage LSB. If you add a wider
tier than D1232<615>, the `2 * SCALE + 64` arm keeps the headroom
automatically — you only need to extend the `TIERS` table and
re-run the generator.

If you ever see a "spurious failure" pattern where the kernel's
output residual is small but the oracle disagrees by many digits,
double the working precision (e.g. floor 1400, `4 * SCALE + 64`)
and regenerate. The kernel is the source of truth only against an
oracle wider than its storage.

### Regenerating hard inputs

```sh
pip install mpmath
python scripts/gen_hard_inputs.py
```

The script is idempotent against existing inputs — it reads each
golden file's existing `<input_raw>` set and skips duplicates, so
re-running on top of an existing file is a no-op (after stripping
the previously-appended `## category:` sections).

To re-append from scratch, strip from the
`# ─── hard-input sections (see scripts/gen_hard_inputs.py) ───`
marker to end-of-file in every `.txt` first, then re-run.
