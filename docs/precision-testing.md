# Precision testing

This page explains how the crate validates its 0.5 ULP correctness
contract — the headline guarantee that strict transcendentals
(`ln_strict`, `exp_strict`, `sin_strict`, `cos_strict`, `tan_strict`,
`atan_strict`, `sqrt_strict`, `cbrt_strict`, …) return results within
half a Unit in the Last Place of the true real value at the type's
storage scale.

There are three independent layers, each catching a different
failure mode.

## Layer 1 — hand-computed truth tables

`tests/precision_strict_05_ulp.rs` lists hand-computed truth values
at D38<12> for every constant and strict transcendental, computed
from canonical 35-digit references and rounded half-to-even at the
storage LSB. Each case asserts `|kernel_result − truth| ≤ 1 LSB`,
the 0.5 ULP contract.

This is the strongest contract in the test suite and the only
table the crate maintainers transcribe by hand. The other layers
exist because hand-computing tables for every tier × every function
is impractical.

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

## Layer 3 — mpmath-oracle golden tables

`tests/ulp_strict_golden.rs` reads pre-computed `.txt` tables from
`tests/golden/` and asserts kernel results match the
[mpmath](https://mpmath.org/) oracle (BSD-3-Clause) within ±1
storage LSB at every tier's design-target SCALE.

Tables live at `tests/golden/<func>_d<N>_s<S>.txt`, with one
`<input_raw>\t<expected_raw>` per line. `input_raw` is the storage
integer of x at the tier scale; `expected_raw` is the half-to-even
rounding of f(x) at the tier scale, computed by mpmath at 500
decimal digits of working precision.

Tiers covered: D38<19>, D76<35>, D153<76>, D307<150>, D616<308>,
D1232<615>. Functions: ln, exp, sin, cos, tan, atan, sqrt, cbrt.

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
- increasing case counts (commit footprint budget is ≤ 5 MB; current
  generation lands ~1.1 MB)

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

The failure message prints the per-case tuple before panicking:

```
FAIL: exp d76 input=<raw_int> expected=<raw_int> actual=<raw_int> delta=<LSB count>
```

`delta` is the storage-LSB distance from the oracle. The contract
allows `delta <= 1` (one storage LSB of rounding + at-most-one-LSB
of mpmath transcription room). Anything above that is a real
precision regression; investigate the kernel for the matching
`(width, scale)` cell.

The harness reports up to 16 failures per file then bails; if more
exist they are suppressed in the output. Read the first one
carefully — it usually points at the worst-case input shape, which
generalises.

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

Coverage gaps deferred to ROADMAP:

- **D9 / D18 / D57 / D115 / D230 / D462 / D924 golden tables** —
  D9/D18 are well-covered by `narrow_strict_transcendentals.rs`,
  the half-width tiers (D57/D115/D230/D462/D924) are exercised by
  cross-witness against their neighbours but don't yet have direct
  mpmath oracles at the bespoke kernel slots.
- **Higher-derivative function families** — `powf`, `log_arbitrary_base`,
  `hypot`, `atan2`, the hyperbolic family. Currently covered by
  the existing wide-tier suite at the contract's softened tolerance.
- **Non-default rounding modes** — directed rounding (Floor /
  Ceiling / Trunc) has a different contract per mode and the
  existing suites are gated to `HalfToEven`.

When extending coverage, add the new tier × function to
`TIERS` / `FUNCS` in `scripts/gen_golden_precision.py`, regenerate,
commit the new `.txt` files under `tests/golden/`, and add a
matching `decl_wide_band!` block (or per-function `#[test]`) to
`tests/ulp_strict_golden.rs`. Keep the committed footprint under
5 MB.

## Hard-input categories

`scripts/gen_hard_inputs.py` is a sibling generator that appends ten
literature-derived hard-input categories to each existing golden
file under a `## category:` header line. The harness treats
`#` lines as comments so existing parsers see the appended rows as
ordinary data.

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
