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

The contract is proved at `delta == 0` storage LSB against two
independently generated high-precision oracles — the committed
full-surface golden set (Layer 1) and an
[mpmath](https://mpmath.org/) table set (Layer 3) — for **every**
[`RoundingMode`](https://github.com/mootable/decimal-scaled/blob/main/src/support/rounding.rs) — `HalfToEven`,
`HalfAwayFromZero`, `HalfTowardZero`, `Trunc`, `Floor`, `Ceiling` —
across **all twelve** decimal widths.

There are four independent layers, each catching a different
failure mode.

## Layer 1 — the full-surface golden gate (the CI-enforced proof)

The consolidated test crate
[`decimal-scale-test`](https://github.com/mootable/decimal-scaled/tree/main/decimal-scale-test)
drives the library-agnostic
[`decimal-scaled-golden`](https://github.com/mootable/decimal-scaled/tree/main/decimal-scaled-golden)
harness over **every band-edge `(width, scale)` cell** — 88 cells
from `D18<0>` to `D1232<1231>` — for every strict function under
**all six** rounding modes. The oracle is the committed golden set:
one width-agnostic `.golden` file per function (28 functions), each
value stored once to 1233 fractional digits and folded in-harness to
the correctly-rounded integer at the subject's cell and mode. A gate
run passes only at **0 bad / 0 panic** — out-of-range cells must
match the declared overflow contract (panic), in-range cells must be
bit-exact.

This is the gate CI enforces on every push (see "CI gate" below)
and the broadest net: every cell, every mode, zero tolerance. See
the [`decimal-scale-test` README](https://github.com/mootable/decimal-scaled/blob/main/decimal-scale-test/README.md)
for running it locally and the `GOLDEN_*` filter variables.

## Layer 2 — internal cross-witness tables

`tests/wide_strict_transcendentals.rs` (and its narrow-tier sibling
`tests/narrow_strict_transcendentals.rs`) validates the wide tiers by
cross-witness: compute a value at the target's storage and scale,
compute the reference at the same scale with a wider storage type,
rescale, and assert ±1 LSB agreement.

Cross-witness is fast and catches storage-bit divergences between
tiers — but a kernel bug *shared* between the narrow and wide
paths will be invisible to it, because both sides agree on the
wrong answer. The next layer addresses that.

## Layer 3 — mpmath-oracle golden tables (the independent second oracle)

`tests/ulp_strict_golden.rs` reads pre-computed `.txt` tables from
`tests/golden/` and asserts kernel results are the **correctly
rounded** value — bit-exact (`delta == 0` storage LSB, ZERO
tolerance) with the [mpmath](https://mpmath.org/) oracle
(BSD-3-Clause) over each tier's five-point scale set, under **every**
`RoundingMode`. Its oracle is generated independently of the Layer-1
golden set, so the two golden nets cross-check each other. The suite
is gated behind the root crate's `golden` Cargo feature (it is a
heavy run, kept out of the regular `cargo test`); run it locally
with the feature enabled as shown below.

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

Tiers covered: **all twelve** decimal widths, each over the five-point
scale set `{0, S/4, S/2, 3S/4, S-1}` (S = the tier's digit capacity,
floor division) — scale 0 (integer regime), S-1 (MAX_SCALE, the
near-overflow / deep-underflow edge), and the three interior quarters.
E.g. D18 = {0, 4, 9, 13, 17}, D38 = {0, 9, 19, 28, 37}, D76 =
{0, 19, 38, 57, 75}, D1232 = {0, 308, 616, 924, 1231}.
Functions: ln, exp, sin, cos, tan, atan, sqrt, cbrt.

### Known kernel holes (ignored cells)

The shipped kernels are correctly rounded for the three *nearest*
modes across every tier. The remaining cell that is not yet correctly
rounded is marked `#[ignore]` in the harness with a reason string (run
it with `cargo test --test ulp_strict_golden --features
wide,x-wide,xx-wide,golden -- --include-ignored`):

- **Narrow-path `atan` directed-rounding 1-LSB boundary.** On the
  narrow (`not(feature = "wide")`) `atan` path at D18 s9 and D38 s19,
  under `Trunc` / `Floor` / `Ceiling`, the result is off by exactly one
  LSB when the true value sits sub-LSB on one side of the output. The
  nearest modes are exact for the same inputs, and the wide-feature
  `atan` path is correctly rounded under every mode.

The ignored cell carries a documented reason; removing the `ignore`
once the kernel is fixed and the band runs green is the witness.

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

- adding a new decimal width to the `TIERS` table (its five-point
  scale set is derived from the capacity by `scale_set_for`)
- changing the per-tier scale sampling set (`scale_set_for`)
- adding a new function to `FUNCS` in the generator
- increasing case counts (commit footprint budget is ≤ 5 MB; current
  generation lands ~1.5 MB across all twelve widths)

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
cargo test --test ulp_strict_golden --features wide,x-wide,xx-wide,golden --release d76
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

`decimal-scale-test/tests/proptest_identities.rs` exercises
identities that hold without an external oracle:

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
input to debug — pin it as a reproducer under
`decimal-scale-test/tests/regressions/` (or whichever file owns the
relevant tier) and iterate against it directly.

## CI gate

[`/.github/workflows/ci.yml`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/ci.yml)
enforces the contract on every PR and push. Its `golden-quick` stripe
fleet runs the Layer-1 full-surface gate in release under the default
rounding mode — every golden row, partitioned across parallel stripes
(`GOLDEN_STRIPE`), nothing sampled away — and blocks the merge; the
`golden-comprehensive` fleet repeats the full surface across all six
rounding modes on every push as a non-blocking advisory check. The
`tests (gate)` job runs the full `cargo test` — which includes the
Layer-2 cross-witness suites and the Layer-4 proptest fuzz. A failed correctness check blocks merge full stop — there
is no reviewer override.

If your kernel change fails the gate: the contract is fixed at
0.5 ULP. Adjust the kernel (usually a wider `GUARD` constant, or a
different reduction shape) until the gate passes. Do not relax the
tolerance.

## What isn't covered yet

The golden suite now covers **all twelve** widths × **every**
`RoundingMode` over the five-point scale set `{0, S/4, S/2, 3S/4, S-1}`.
Remaining gaps:

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
> `floor_and_class`) before relying on the hard-input rows again.

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

The proptest harness `decimal-scale-test/tests/proptest_identities.rs`
mirrors these categories with per-category input strategies (100
cases each, deterministic seed). Identity-based assertions
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
