# Cargo features

```toml
[dependencies]
decimal-scaled = { version = "0.1.1", default-features = false, features = ["alloc"] }
```

## Core features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | The `f64`-bridge transcendentals (trig, log/exp, sqrt, …) and `from_f64_lossy` constructors. Pulls in `alloc`. |
| `alloc` | yes | `Display::to_string` and `FromStr` on `no_std`. Required — targets without `alloc` are not supported. |
| `serde` | yes | `Serialize` / `Deserialize` via `serde_helpers` (canonical-string form). |
| `macros` | no | The `d128!` compile-time literal macro. See [the macro guide](macros.md). |
| `strict` | no | Integer-only transcendentals instead of the `f64` bridge. Platform-independent; works under `no_std`. See [strict mode](strict-mode.md). |

`std` and `strict` are mutually exclusive *in effect*: with `strict` on,
transcendentals always use the integer path even if `std` is also
enabled.

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
| `d256` | `D256` (256-bit storage, `MAX_SCALE = 76`) |
| `d512` | `D512` (512-bit storage, `MAX_SCALE = 153`) |
| `d1024` | `D1024` (1024-bit storage, `MAX_SCALE = 307`) |
| `wide` | all three of the above |

`D32` / `D64` / `D128` never pull in `bnum` — the dependency is added
only when a wide feature is active.

## Nightly-only

| Feature | Enables |
|---|---|
| `experimental-floats` | `f16` / `f128` entry points on the float bridge (`from_f16_lossy`, `to_f128_lossy`, …). Requires a nightly toolchain. |

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
```
