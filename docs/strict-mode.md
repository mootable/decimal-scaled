# Strict mode — integer-only, correctly-rounded transcendentals

Transcendental functions (logarithms, exponentials, roots,
trigonometry) come in two forms:

- the **f64 bridge** — convert to `f64`, apply the platform `f64`
  intrinsic, convert back. Fast, depends on `std` and the platform
  libm.
- the **strict** form — an integer-only implementation. Platform-
  independent, bit-identical on every target, `no_std`-compatible.

## The `*_strict` dual API

**Every transcendental that has a strict implementation exposes it
under a `*_strict` name, and that `*_strict` method is always
compiled** — it does *not* depend on the `strict` Cargo feature being
enabled (it is only removed by `no_strict`, below).

```rust
use decimal_scaled::D38s12;

let x = D38s12::from_int(2);

// Always available — the integer-only path, explicitly:
let r1 = x.sqrt_strict();
let l1 = x.ln_strict();

// The plain method dispatches by feature:
//   * with `strict`      -> calls `*_strict`
//   * without `strict`   -> the f64 bridge (needs `std`)
let r2 = x.sqrt();
```

Why a dual API:

- **Run both side by side** — benchmark or cross-check the strict path
  against the f64 bridge in the same build.
- **Mix and match** — call `ln_strict()` for the values that must be
  deterministic and `ln()` for the rest.
- **Guaranteed strict regardless of feature toggles** — `*_strict`
  means strict, full stop; it cannot be silently swapped for the f64
  bridge by a downstream crate flipping a feature.

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
decimal-scaled = { version = "0.1.1", features = ["strict"] }
```

With `strict` enabled, the plain methods (`sqrt`, `ln`, `sin`, …)
dispatch to their `*_strict` form. `strict` does not require `std`; the
integer algorithms compile under `no_std + alloc`. The explicit
float-conversion methods (`to_f64_lossy`, `from_f64_lossy`,
`TryFrom<f64>`, …) remain available — they are type conversions, not
transcendental operations.

## The `no_strict` feature

```toml
decimal-scaled = { version = "0.1.1", features = ["no_strict"] }
```

`no_strict` drops the entire `*_strict` surface for a smaller build. It
**overrides `strict`**: with both set, the crate behaves as if neither
were — the plain methods are the f64 bridge (when `std` is available),
and no `*_strict` methods are emitted at all.

| Features | `*_strict` methods | plain `sqrt` / `ln` / … |
|---|---|---|
| *(none)* | present | f64 bridge (needs `std`) |
| `strict` | present | dispatch to `*_strict` |
| `no_strict` | **absent** | f64 bridge (needs `std`) |
| `strict`, `no_strict` | **absent** | f64 bridge (needs `std`) |

## The 0.5 ULP accuracy guarantee

Every strict method is held to the **[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
correctly-rounded standard**: the returned value is within **0.5
[ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place)** (unit in
the last place) of the mathematically exact result — i.e. it is the
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
  `wide_int::Fixed` intermediate — a 256-bit value at `SCALE + 30`
  decimal *guard digits*. The 30 guard digits bound the total
  accumulated rounding error to roughly `1e-17` of an output ULP, far
  inside the 0.5 ULP margin, and the value is rounded once
  (half-to-even) at the very end.

This holds across the whole `SCALE` range, including `SCALE = 38`,
because the guard-digit intermediate is wider than `i128`. Every
strict transcendental is cross-checked against the platform `f64`
implementation at `D38<9>` (where `f64` is comfortably more precise
than the type's ULP) — see the in-crate tests.

The wide tiers (`D76` / `D153` / `D307`) do not yet have the strict
transcendental surface; that follow-up is tracked in
`research/strict_transcendentals_research.md`.

## Choosing the configuration

| You want… | Use |
|---|---|
| Max speed, `std`, mainstream platform | default (f64 bridge) |
| Bit-identical results everywhere, or `no_std` | `strict` |
| The strict path *and* the bridge in one build | default or `strict` — call `*_strict` explicitly for the former |
| The smallest possible build, no strict surface | `no_strict` |
