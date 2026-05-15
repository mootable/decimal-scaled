# Cargo features

```toml
[dependencies]
decimal-scaled = { version = "0.1.1", default-features = false, features = ["alloc"] }
```

## Core features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | The `f64`-bridge transcendentals (trig, log/exp, sqrt, …) and `from_f64_fast` constructors. Pulls in `alloc`. |
| `alloc` | yes | `Display::to_string` and `FromStr` on `no_std`. Required — targets without `alloc` are not supported. |
| `serde` | yes | `Serialize` / `Deserialize` via `serde_helpers` (canonical-string form). |
| `macros` | no | The `d38!` compile-time literal macro. See [the macro guide](macros.md). |
| `strict` | no | The plain transcendental methods dispatch to the integer-only `*_strict` path instead of the `f64` bridge. Platform-independent; works under `no_std`. See [strict mode](strict-mode.md). |
| `fast` | no | Drops the `*_strict` transcendental surface entirely for a smaller build. **Overrides `strict`** — with both set, the crate behaves as if neither were. |

Notes:

- The `*_strict` methods (`sqrt_strict`, `ln_strict`, …) are compiled
  **regardless of the `strict` feature** — only `fast` removes
  them. `strict` only controls whether the *plain* methods (`sqrt`,
  `ln`, …) dispatch to the strict path.
- With `strict` on, the plain transcendentals use the integer path even
  if `std` is also enabled.
- The strict methods are held to the
  [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) correctly-rounded
  standard (within 0.5
  [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) of the
  exact result). See [strict mode](strict-mode.md) for the dual-API
  rules and the current per-function implementation status.

## Default-rounding-mode features

Exactly one of these may be enabled to change the crate-wide default
`RoundingMode` at compile time (default is `HalfToEven`). See
[the rounding guide](rounding.md).

| Feature | Default mode |
|---|---|
| `rounding-half-away-from-zero` | `HalfAwayFromZero` |
| `rounding-half-toward-zero` | `HalfTowardZero` |
| `rounding-trunc` | `Trunc` |
| `rounding-floor` | `Floor` |
| `rounding-ceiling` | `Ceiling` |

## Wide-tier features

The wide decimal types pull in the `bnum` big-integer crate. They are
opt-in per width, with `wide` as an umbrella over all three.

| Feature | Enables |
|---|---|
| `d76` | `D76` (256-bit storage, `MAX_SCALE = 76`) |
| `d153` | `D153` (512-bit storage, `MAX_SCALE = 153`) |
| `d307` | `D307` (1024-bit storage, `MAX_SCALE = 307`) |
| `wide` | all three of the above |

`D9` / `D18` / `D38` never pull in `bnum` — the dependency is added
only when a wide feature is active.

## Nightly-only

| Feature | Enables |
|---|---|
| `experimental-floats` | `f16` / `f128` entry points on the float bridge (`from_f16_fast`, `to_f128_fast`, …). Requires a nightly toolchain. |

## Common configurations

```toml
# Default — std, serde, f64-bridge transcendentals.
decimal-scaled = "0.1.1"

# no_std, still with serde.
decimal-scaled = { version = "0.1.1", default-features = false, features = ["serde", "alloc"] }

# Deterministic, platform-independent transcendentals, no_std.
decimal-scaled = { version = "0.1.1", default-features = false, features = ["alloc", "strict"] }

# All six widths, with the literal macro.
decimal-scaled = { version = "0.1.1", features = ["wide", "macros"] }

# Smallest build — no strict surface at all.
decimal-scaled = { version = "0.1.1", features = ["fast"] }
```
