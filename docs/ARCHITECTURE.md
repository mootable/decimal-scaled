# Architecture

This page is a map of how `decimal-scaled` is put together: the two
layers (integer **backends** and decimal **front-ends**), how a method
call is routed to one specific algorithm with **zero runtime dispatch**,
how unused algorithm variants are **pruned** at compile time, and how the
correctness and performance guarantees are **enforced by testing**.

## The model in one sentence

Every value is a plain integer that encodes `real_value × 10^SCALE`. All
core arithmetic is integer arithmetic, so results are **bit-identical on
every platform**, and the transcendental functions are computed with
integer-only kernels that are **correctly rounded** — within 0.5 ULP of
the true real value at the type's last representable place.

## Two layers, same shape

The crate is **two layers that mirror each other** — a decimal layer on
top of an integer layer. Each has the *same three tiers*: a typed
surface, a **const-folded width dispatch**, and an **algorithm library**.
A decimal kernel expresses its math in integer operations and never
reaches into limb internals directly.

![decimal-scaled layer architecture: the decimal layer (front-ends → dispatch → kernels) composed on the integer layer (backends → dispatch → algorithms → limb primitives), each layer carrying its own width dispatch and algorithm library](figures/architecture/layers.svg)

The key point the older sketch hid: **the integer layer is not just
primitives — it has its own dispatch policy and algorithm library**,
mirroring the decimal layer. A `BigInt` method (`mul`, `div`,
`root_int`, …) routes on the compile-time width / limb count to one
matched algorithm — schoolbook below the Karatsuba threshold and the
non-allocating Karatsuba above it, Möller–Granlund divide at the narrow
tiers and Burnikel–Ziegler at the wide ones — and the compiler prunes
the rest, exactly as the decimal policy keys on `(width, SCALE)`. Both
layers compile to a single direct call per monomorphisation.

## A call through the layers

`D57<20>::sqrt_strict()` traverses **both layers' dispatch + kernels**:
the front-end dispatches on `(width, SCALE)` to one decimal algorithm, which
calls the integer layer — itself dispatching on width to a matched
algorithm down to the limb primitives — and hands back a
correctly-rounded raw value.

```mermaid
sequenceDiagram
  autonumber
  participant U as caller
  participant FE as D57 front-end
  participant DP as decimal policy
  participant DK as sqrt algorithm
  participant IP as int policy
  participant IK as int algorithm
  participant L as limb primitives
  U->>FE: sqrt_strict()
  FE->>DP: dispatch (width 192, SCALE 20)
  DP->>DK: const-folded → lookup_d57_s20
  DK->>IP: root_int / isqrt on Int<3> (BigInt)
  IP->>IK: const-folded → width-matched isqrt
  IK->>L: limb ops on [u64; 3]
  L-->>IK: limbs
  IK-->>DK: integer root + residual
  DK-->>FE: correctly-rounded raw
  FE-->>U: D57 value
```

## Integer backends

The storage under a decimal type is an integer wide enough to hold
`10^MAX_SCALE`:

| Decimal tier | Storage |
|---|---|
| D18 / D38 | `Int<1>` / `Int<2>` |
| D57 … D1232 | `Int<3>` … `Int<64>` |

Every tier is built on a single const-generic pair —
`Uint<const N: usize>` and `Int<const N: usize>` — stored as `[u64; N]`
little-endian limbs (`N` = number of 64-bit limbs; bit width is `N·64`).
Choosing the **limb count** as the one type parameter sidesteps the
`LIMBS = ⌈BITS/64⌉` derivation that a bits-parameterised type cannot
express on stable Rust.

The arithmetic itself lives once, as **reusable width-matched limb
algorithms** (add/sub/mul/div/shift/compare, plus `sqr`, `cube`,
`root_int`, `isqrt`, …) operating over `&[u64]` slices. Because `N` is a
compile-time constant, the limb loops unroll per width and there is no
runtime length to carry. A `BigInt` trait exposes this surface with the
**same method names** as the decimal arithmetic trait, so the two layers
read as one vocabulary.

`src/`:

```
int/                const-generic integer layer
  types/            Int<N>/Uint<N>; the BigInt trait
  policy/           per-width / limb-count algorithm-selection dispatch
  algos/            reusable width-matched algorithms
  limbs/            raw slice limb primitives
```

## Decimal front-ends

Each width is a `Dxx<const SCALE: u32>` newtype around its storage
integer. The number in the name is `MAX_SCALE` (the largest `SCALE` the
storage can hold); `SCALE` is a const-generic so `D38<2>` (cents) and
`D38<18>` are distinct, zero-overhead types. Because a value has exactly
one representation at a fixed scale, `Eq`/`Ord`/`Hash` are derived
straight from the storage bits.

The cross-width API is four traits (`src/types/traits/`):

- `DecimalArithmetic` — operators, sign, integer methods, the
  checked/wrapping/saturating/overflowing families, reductions.
- `DecimalConvert` — round-trip, integer and float bridges.
- `DecimalTranscendental` — `sqrt`/`cbrt`/`exp`/`ln`/trig/hyperbolic/`pow`.
- `Decimal` — marker supertrait combining the above.

The typed method shells (`D57::<20>::sqrt_strict_with(mode)`) are emitted
by macros in `src/macros/` and immediately hand off to the dispatch layer.

## Algorithm choosing — and pruning

A single function (say `sqrt`) has many possible kernels: a narrow tier
widens to D38; D38 uses a hand-tuned 256-bit isqrt; D57 at SCALE 20 has a
bespoke lookup; everything else uses the generic wide isqrt. The choice
is made by a **per-family policy** that matches on the compile-time
`(width, SCALE)`:

```rust
match (W, SCALE) {
    (W_D38, _)  => algos::sqrt::mg_divide_d38::sqrt(x, SCALE, mode),
    (W_D57, 20) => algos::sqrt::lookup_d57_s20::sqrt(x, mode),
    (W_D57, _)  => algos::sqrt::generic_wide::sqrt_d57(x, SCALE, mode),
    // … one arm per cell
}
```

Three levels of choice live in that table: a **global default** (the
generic kernel), a **width override** (a whole tier picks a different
kernel), and a **scale-range override** (a bespoke kernel for one band of
scales). Top arm wins.

### Pruning = dead-arm elimination

`W` and `SCALE` are *constants in every monomorphisation*. So for the
concrete type `D57<20>`, the compiler evaluates the match at compile time,
**discards every arm that doesn't match**, and inlines the one that does.
`D57<20>::sqrt` compiles to a direct call to exactly one kernel — no
branch, no table, no vtable. Every other candidate kernel is pruned out
of that type's machine code. This is what makes the rich policy table
**zero runtime cost**.

### `base` / `std` / `no_std`

> **0.5.0:** this three-layer split is collapsed to a **single `core` tier** — see
> *Policy file structure* below. The triplet described here is the pre-0.5.0 form.

Each function is organised as three thin layers so the distinction
between portable and platform-assisted code is structural, not scattered
through the math:

- **`base`** — the real algorithm, the `(width, SCALE)` match.
- **`no_std`** — a direct pointer to `base` (the always-correct,
  pure-integer path).
- **`std`** — defaults to `base` and carries *only* the overrides (e.g. an
  `f64`-seeded fast path). Opening the `std` body shows exactly what
  differs and nothing else.

An `std` override is included for a cell **only if it is benchmarked
faster** than the `no_std` path; otherwise the cell stays on `base`.
Where `std` uses `f64`, it is only ever a **seed** to a self-correcting
integer iteration whose exact integer termination pins the unique
result — so determinism is preserved regardless of the platform's `f64`.

### Policy file structure (the per-function matcher)

Each dispatched function (`sqrt`, `mul`, `exp`, …) has **one policy file** with a
fixed shape. Agents implementing or extending a policy follow this template
exactly. The dispatch key is the compile-time width(s) and scale(s) —
`N` (int unary), `(N, M)` (int binary), `(N, SCALE)` (decimal unary), or
`(N, M, S1, S2)` (decimal binary).

```rust
// 1. the real algorithms for this function — NAMED, paper-based, no `Default`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Algorithm { Newton, Zimmermann }

// 2. the const verdict: a settled algorithm, or "the value decides".
#[derive(Clone, Copy)]
enum Select<const N: usize> {
    ByAlgorithm(Algorithm),                // width/scale settled it
    ByValue(fn(&Int<N>) -> Algorithm),     // value-dependent (non-capturing fn / closure)
}

// 3. the matcher: a `const fn`, keyed ONLY on the const generics, total over the key.
const fn select<const N: usize>() -> Select<N> {
    match N {
        0..=2 => Select::ByAlgorithm(Algorithm::Newton),
        3..=8 => Select::ByValue(|v: &Int<N>|           // value-matcher (see placement rule)
            if v.bit_length() <= 128 { Algorithm::Newton } else { Algorithm::Zimmermann }),
        _     => Select::ByAlgorithm(Algorithm::Zimmermann),   // the chosen default (a real algorithm)
    }
}

// 4. the public function: resolve the verdict, then dispatch — exhaustively, no panic.
pub fn isqrt<const N: usize>(x: Int<N>) -> Int<N> {
    const SEL: Select<N> = select::<N>();               // compile-time constant ⇒ folds
    let algo = match SEL {
        Select::ByAlgorithm(a) => a,
        Select::ByValue(f)     => f(&x),                // the ONE place a runtime value enters
    };
    match algo {                                        // exhaustive over `Algorithm` — no `_`, no `unreachable!()`
        Algorithm::Newton     => isqrt_newton::<N>(x),
        Algorithm::Zimmermann => isqrt_zimmermann::<N>(x),
    }
}
```

Rules that make this work:

- **`Algorithm` lists real algorithms only — no `Default` variant.** Completeness
  is structural: `select` is total over the key, and `match algo` is exhaustive
  over `Algorithm` (both compiler-enforced). The "default" for unspecialised
  widths is simply the real algorithm named in `select`'s `_` arm.
- **`select` is `const`, keyed only on the const generics.** Per
  monomorphisation `SEL` is a constant, so the matches fold and every unchosen
  arm is dead-arm-eliminated (the pruning above). The policy is zero runtime
  cost — *except* a `ByValue` arm, which keeps exactly one runtime comparison.
- **Value matcher** (`ByValue`) — for the rare case where the best algorithm
  depends on the operand's *value* (e.g. actual magnitude), not just its width.
  It is **non-capturing**, takes the value, and **returns an `Algorithm` tag**
  (never a function pointer — the tag keeps dispatch a direct call). Placement by
  size: **≤2 outcomes → inline closure `if`/`else`; 3–10 → inline closure
  `match`; >10 (or shared / unit-tested) → a named `#[inline]` fn called
  `<fn>_N<lo>_to_N<hi>`** (e.g. `sqrt_N5_to_N10`) encoding the width-band it serves.
- **Single tier.** There is one `core`-only policy per function — no
  `base`/`std`/`no_std` split (it superseded the triplet described above). A
  platform-specific override, if ever justified, rides the *same* mechanism: a
  `#[cfg(feature = "std")]` arm inside `select`, a cfg-gated value-matcher, or a
  cfg-gated `Algorithm` variant — never a parallel tier.
- **Acceptance gate:** the zero-runtime-branch property is a *release* property;
  it is proven per function by inspecting the release IR/asm (one direct call, no
  branch/table/vtable on the const path).

### Keeping the alternatives

Algorithms that lose at today's widths (FFT/NTT multiplication, AGM below
~D1232, …) are not deleted. They are preserved as documented references
and, where the implementation is genuinely different, as compiled-out
code — because a future CPU/LLVM instruction or a platform-specific build
can flip a today-loser into a winner. See `ALGORITHMS.md`.

## How the guarantees are enforced — by testing

The architecture's two headline promises are **platform determinism** and
**correct rounding**, and both are nailed down by tests rather than
asserted by hope.

**Determinism** falls out of the design (integer-only core, no floating
point in results) and is exercised by the cross-platform CI matrix and
bit-exact fixtures.

**Correct rounding** is the contract that the strict transcendentals
return the value within 0.5 ULP of the true real value — equivalently,
the **exact correctly-rounded value at the storage scale (0 LSB of
error)**, under *every* rounding mode and at *every* width. It is checked
by independent layers (see `precision-testing.md`):

1. **Hand-computed truth tables** at D38 — the smallest, human-audited net.
2. **Cross-witness** — compute at a tier, recompute the reference at a
   wider storage and rescale; catches storage-bit divergences.
3. **mpmath golden tables** — an external oracle (computed at working
   precision far wider than any tier) for every (function, tier); the
   kernel must match the correctly-rounded oracle **exactly (delta == 0)**
   for all six rounding modes across all widths.
4. **Property fuzz** — identities like `exp(ln x) ≈ x`, `sin²+cos² ≈ 1`,
   and sign symmetries, with deterministic seeds.

The integer backends carry their own bit-exact tests (each algorithm
checked against a schoolbook oracle across widths and edge cases). A
performance change can never silently cost accuracy: the delta == 0
precision suite is a permanent regression gate, so a faster kernel that
rounds wrong turns CI red.

## Where the rounding actually happens

The kernels compute at a wider *working scale* (`SCALE + GUARD` digits)
and then round to the storage scale. For the three nearest modes a fixed
guard is enough; for the directed modes (toward zero / ±∞) the rounding
decision needs the *sign and stickiness* of the sub-LSB residual — which
the divide already computes — and, on the rare inputs sitting within the
kernel's error of a tie, an adaptive widening step (Ziv iteration) settles
it. The result is correct rounding under all six modes with the common,
nearest-mode path paying nothing extra.

## Map of the source tree

```
src/
  int/        const-generic integer layer
    types/    Int<N>/Uint<N>; the BigInt trait
    policy/   per-width / limb-count algorithm-selection dispatch
    algos/    reusable width-matched algorithms
    limbs/    raw slice limb primitives
  types/      Dxx<SCALE> typed shells, the Decimal trait family, consts
  policy/     per-family (width, SCALE) dispatch → kernels
  algos/      the kernels (sqrt cbrt exp ln trig pow …)
  macros/     code generation for the per-type method shells
  support/    rounding modes, errors, display, serde helpers
```
