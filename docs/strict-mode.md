# Strict mode - integer-only, correctly-rounded transcendentals

Transcendental functions (logarithms, exponentials, roots,
trigonometry) come in two forms:

- the **f64 bridge** - convert to `f64`, apply the platform `f64`
  intrinsic, convert back. Fast, depends on `std` and the platform
  libm.
- the **strict** form - an integer-only implementation. Platform-
  independent, bit-identical on every target, `no_std`-compatible.

## The `*_strict` / `*_fast` dual API

**Every transcendental that has a strict implementation exposes it
under a `*_strict` name; every transcendental that has an f64-bridge
implementation exposes it under a `*_fast` name. Both surfaces are
always compiled regardless of which Cargo feature is active** (subject
only to dependency gates - `*_fast` needs `feature = "std"`). The
`strict` / `fast` Cargo features only choose what the plain `*` form
resolves to.

```rust
use decimal_scaled::D38s12;

let x = D38s12::from_int(2);

// Always available - the integer-only path, explicitly:
let r1 = x.sqrt_strict();
let l1 = x.ln_strict();

// Also always available - the f64-bridge path, explicitly (needs
// `feature = "std"`):
let r2 = x.sqrt_fast();
let l2 = x.ln_fast();

// The plain method dispatches by feature:
//   * with `strict` (default)             -> calls `*_strict`
//   * with `fast` (overrides `strict`)    -> calls `*_fast`
//   * with neither + `std`                -> calls `*_fast`
let r3 = x.sqrt();
```

Why a dual API:

- **Run both side by side** - benchmark or cross-check the strict path
  against the f64 bridge in the same build.
- **Mix and match** - call `ln_strict()` for the values that must be
  deterministic and `ln_fast()` (or plain `ln()`) for the rest.
- **Guaranteed strict regardless of feature toggles** - `*_strict`
  means strict, full stop; it cannot be silently swapped for the f64
  bridge by a downstream crate flipping a feature. The same applies in
  reverse: `*_fast` always reaches the f64 bridge.

The `*_strict` surface covers, on `D38` (and on `D9` / `D18` by
widen-compute-narrow delegation):

| Group | `*_strict` methods |
|---|---|
| Logarithms | `ln_strict`, `log_strict`, `log2_strict`, `log10_strict` |
| Exponentials | `exp_strict`, `exp2_strict` |
| Roots / powers | `sqrt_strict`, `cbrt_strict`, `powf_strict`, `hypot_strict` |
| Forward trig | `sin_strict`, `cos_strict`, `tan_strict` |
| Inverse trig | `asin_strict`, `acos_strict`, `atan_strict`, `atan2_strict` |
| Hyperbolic | `sinh_strict`, `cosh_strict`, `tanh_strict` |
| Inverse hyperbolic | `asinh_strict`, `acosh_strict`, `atanh_strict` |
| Angle conversion | `to_degrees_strict`, `to_radians_strict` |

## The `strict` feature

```toml
decimal-scaled = { version = "0.2.5", features = ["strict"] }
```

With `strict` enabled, the plain methods (`sqrt`, `ln`, `sin`, …)
dispatch to their `*_strict` form. `strict` does not require `std`; the
integer algorithms compile under `no_std + alloc`. The explicit
float-conversion methods (`to_f64`, `from_f64`,
`TryFrom<f64>`, …) remain available - they are type conversions, not
transcendental operations.

## The `fast` feature

```toml
decimal-scaled = { version = "0.2.5", default-features = false, features = ["std", "fast"] }
```

`fast` makes the plain methods (`sqrt`, `ln`, `sin`, …) dispatch
through the f64 bridge for speed at the cost of ~16-digit
platform-libm precision. It only affects what plain `*` resolves
to: **both the `*_strict` integer-only methods and the `*_fast`
f64-bridge methods are always emitted** so per-call selection
stays available.

**Strict-by-default — and `strict` wins when both are enabled.**
0.3.0 changes the dispatcher rule: strict is the default plain
dispatch *regardless* of whether the `strict` feature is
explicitly enabled, and `fast` only takes over plain dispatch
when `strict` is explicitly absent. Reasoning: the strict path
is now fast enough (`ln_strict` at D38<19> is ~1.5 µs, sin at
39 µs) that staying on the deterministic correctly-rounded path
by default is the right call across more codepaths. The only
way to land on `*_fast` for plain `sqrt` / `ln` / etc. is to
build with `default-features = false` AND explicitly enable
`fast` AND NOT re-enable `strict` — a deliberate three-step
opt-out. Mixing `strict` with `fast` (e.g. a downstream crate
flips `fast` on a transitive build that also has `strict`)
keeps you on the strict path.

| Features | `*_strict` named methods | `*_fast` named methods (needs `std`) | plain `sqrt` / `ln` / … |
|---|---|---|---|
| *(none)*                                    | present | present | **dispatches to `*_strict`** |
| `strict` (default)                          | present | present | dispatches to `*_strict` |
| `fast` (and not `strict`)                   | present | present | dispatches to `*_fast` (needs `std`) |
| `strict` + `fast`                           | present | present | **dispatches to `*_strict`** |

**Strict is the default dispatcher** — explicit, intentional. To
get the f64 bridge as the plain dispatch you have to (a) build
with `default-features = false` (which drops the `strict` feature
along with `std`+`serde`) and (b) add the `fast` feature plus
`std`. Mixing `strict` with `fast` keeps you on the strict path —
the only way to land on `*_fast` for `sqrt` / `ln` / etc. is to
explicitly opt out of strict.

## The 0.5 ULP accuracy guarantee

Every strict method is held to the **[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
correctly-rounded standard**: the returned value is within **0.5
[ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place)** (unit in
the last place) of the mathematically exact result - i.e. it is the
exact result rounded to the nearest representable value at the type's
last decimal place.

How it is achieved, per function family:

- **Algebraic roots** (`sqrt`, `cbrt`) form the exact wide-integer
  radicand (`r · 10^SCALE` for sqrt as a 256-bit value, `r · 10^(2·SCALE)`
  for cbrt as a 384-bit value), take its *exact* integer root, and
  decide the rounding with an exact integer comparison
  (`8·N ≥ (2q+1)³` for cbrt, etc.). No approximation enters.
- **Transcendentals** (`ln`, `log`, `log2`, `log10`, `exp`, `exp2`,
  `powf`, and the whole trig / hyperbolic / angle-conversion family)
  evaluate their range reduction and series in the in-tree
  `wide_int::Fixed` intermediate - a 256-bit value at `SCALE + 30`
  decimal *guard digits*. The 30 guard digits bound the total
  accumulated rounding error to roughly `1e-17` of an output ULP, far
  inside the 0.5 ULP margin, and the value is rounded once
  (half-to-even) at the very end.

This holds across the whole `SCALE` range, including `SCALE = 38`,
because the guard-digit intermediate is wider than `i128`. Every
strict transcendental is cross-checked against the platform `f64`
implementation at `D38<9>` (where `f64` is comfortably more precise
than the type's ULP) - see the in-crate tests.

All wide tiers (`D56` / `D76` / `D114` / `D153` / `D230` / `D307`
under the `wide` umbrella; `D461` / `D615` under `x-wide`; `D923` /
`D1231` under `xx-wide`) ship the full strict transcendental
surface — every method has a `*_strict` form plus a mode-aware
`*_strict_with(mode)` sibling. Two alternate implementations are
also exposed: `ln_strict_agm` and `exp_strict_agm` use the
quadratically-convergent Brent–Salamin / Newton path that scales
better than the artanh / Taylor canonical at very high working
scales; the canonical paths remain the default until a bench at
the relevant working scale shows AGM winning. The accuracy
contract is the same ≤ 0.5 ULP at storage as D38.

## Choosing the configuration

| You want… | Use |
|---|---|
| Bit-identical results everywhere; correct rounding | default (`strict` is on by default) |
| Max speed at the cost of platform-libm precision | `fast` |
| Per-call explicit choice in the same build | always available via `*_strict` / `*_fast` regardless of features |
| `no_std + alloc` | default works (`strict` is `no_std`-compatible) |
