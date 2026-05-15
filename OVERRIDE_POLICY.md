# Per-type kernel override policy

How `decimal-scaled` selects between a generic macro-emitted
implementation and a per-type hand-tuned override, on a function-by-
function basis.

## Naming convention

For any precision-bearing method `f` (e.g. `ln_strict`,
`exp_strict`, `powf_strict`, `to_radians_lossy`, …) the crate
recognises four names:

| Symbol                | What it is                                                                |
|-----------------------|---------------------------------------------------------------------------|
| `f_strict_default`    | Macro-emitted generic implementation (from `decl_wide_transcendental!`).  |
| `f_strict_override`   | Per-type hand-tuned implementation (lives in `<type>_kernels.rs`).        |
| `f_lossy_default`     | Macro-emitted f64-bridge form (from `decl_lossy_transcendentals_via_f64!`).|
| `f_lossy_override`    | Per-type lossy override (rare; landed if it ever beats the bridge).       |
| `f_strict` / `f_lossy`| **Canonical name**. A `#[inline]` wrapper around exactly one of the
                          above, picked at compile time. The pointer disappears at
                          link time.                                                |

Every variant that exists is `pub`. Benches can call any of them
directly without recompiling.

## Compile-time selection

The canonical `f_strict` calls the chosen winner via an inline
wrapper:

```rust
impl<const SCALE: u32> D128<SCALE> {
    /// Canonical `ln_strict`. Picks the implementation chosen for
    /// this type. Inline so the indirection is free at runtime.
    #[inline]
    pub fn ln_strict(self) -> Self {
        // D128 chose `_override` per the bench-driven policy.
        self.ln_strict_override()
    }
}
```

Whether `_default` or `_override` is the chosen winner is decided
*per type, per method* by bench. The selection lives at the call
site in `core_type.rs` so the policy is auditable.

## When to write an override

Run the function on the type under `cargo bench`. Compare
`f_default` (macro) vs the hand-tuned candidate.

- **Override wins by < 1.5×**: keep the macro. Macro-only —
  delete the override candidate. The simpler, less duplicated code
  is the better default.
- **Override wins by ≥ 1.5×**: ship both. `_override` is canonical;
  `_default` stays compiled only under `feature = "bench-alt"` so
  comparison is still possible without recompiling against a
  different version of the crate.

A 1.5× perf gain is large enough to be worth the maintenance cost
of a per-type kernel; below that the cost of two parallel
implementations outweighs the win.

## Feature flags

- **`bench-alt`** (default off) — additionally compiles every
  `_default` / `_override` variant that *wasn't* the chosen winner,
  so a single binary can bench both. With `bench-alt` off, only the
  chosen winner (plus the canonical inline wrapper) is compiled —
  saves build time and binary size.

## Current decisions (per the session bench data)

| Type | Method        | Default (µs) | Override (µs) | Winner   |
|------|---------------|-------------:|--------------:|----------|
| D128 | `ln_strict`   | ~100 (est)   | 29            | override |
| D128 | `exp_strict`  | ~150 (est)   | 42            | override |
| D128 | `sin_strict`  | ~100 (est)   | 30            | override |
| D128 | `tan_strict`  | ~200 (est)   | 59            | override |
| D128 | `sqrt_strict` | ~50 (est)    | 0.029         | override |
| D128 | `cbrt_strict` | ~12 (est)    | 5.9           | override |
| D128 | `powf_strict` | ~200 (est)   | 68            | override |
| D128 | every other transcendental | ~100+ (est) | ~30-60 | override |
| D256+| every transcendental       | the macro is the only impl |     | default  |

D128 wins everywhere because of the specialised 256-bit
`d128_kernels::Fixed` work integer. Wider types use the macro
because the storage is already wide-int and the macro carries the
right type-generic shape.

## How to flip a decision

1. Add the new candidate under the appropriate `_default` /
   `_override` name.
2. Run `cargo bench --features wide --bench decimal_backends`.
3. Edit the canonical inline wrapper in `core_type.rs` to call the
   new winner.
4. Make sure the loser is gated on `feature = "bench-alt"` so
   default builds don't compile it.

The wrapper file lives alongside the type definition, so the
decision is one git diff per type, never mixed with the algorithm
code.

## Not yet implemented in source

This document defines the policy; the source today reflects only
half of it. Specifically:

- The macro currently emits canonical names (`ln_strict`), not
  `_default`-suffixed names. Renaming the macro's output to
  `*_default` and adding canonical-wrapper invocations is the
  remaining mechanical change.
- The `bench-alt` feature flag does not exist yet — gating the
  loser variant on it is part of the same refactor.

When that lands, the table above becomes a generated artefact of
the bench instead of a hand-edited one.
