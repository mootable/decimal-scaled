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
use decimal_scaled::D128s12;

let x = D128s12::from_int(2);

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

The `*_strict` surface covers, on `D128` (and on `D32` / `D64` by
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

The strict methods are held to the **IEEE-754 correctly-rounded
standard**: the returned value is within **0.5 ULP** of the
mathematically exact result — i.e. it is the exact result rounded to
the nearest representable value at the type's last decimal place.

Concretely, a `D128<38>` result is computed to at least 39 decimal
places internally (one guard digit beyond the type's precision, more
where an algorithm's error analysis requires it) and rounded once at
the end. The implementation forms exact wide-integer intermediates
where an exact integer formulation exists, and otherwise carries a
guard-digit Q-format intermediate with a proven error bound below
0.5 ULP of the output.

**Implementation status.** `sqrt_strict` is correctly-rounded today —
it forms the exact 256-bit radicand `r · 10^SCALE`, takes its exact
integer square root, and rounds with an exact integer comparison. The
remaining transcendentals (`ln` / `exp` / the trig family) are being
reworked from their earlier ~10-ULP series implementations to the
correctly-rounded guard-digit form; until that lands they do not yet
meet the 0.5 ULP bound. Track progress in
`research/strict_transcendentals_research.md`.

## Choosing the configuration

| You want… | Use |
|---|---|
| Max speed, `std`, mainstream platform | default (f64 bridge) |
| Bit-identical results everywhere, or `no_std` | `strict` |
| The strict path *and* the bridge in one build | default or `strict` — call `*_strict` explicitly for the former |
| The smallest possible build, no strict surface | `no_strict` |
