# Cargo features

```toml
[dependencies]
decimal-scaled = { version = "0.2.5", default-features = false, features = ["alloc"] }
```

## Core features

| Feature | Default | Enables |
|---|---|---|
| `std` | yes | The `f64`-bridge transcendentals (trig, log/exp, sqrt, …) and `from_f64` constructors. Pulls in `alloc`. |
| `alloc` | yes | `Display::to_string` and `FromStr` on `no_std`. Required — targets without `alloc` are not supported. |
| `serde` | yes | `Serialize` / `Deserialize` via `serde_helpers` (canonical-string form). |
| `strict` | **yes** | Marks the build as on the strict path: plain `sqrt` / `ln` / etc. dispatch to the integer-only ≤ 0.5 ULP `*_strict` methods. `no_std`-friendly. Strict is *also* the dispatch when no feature is set at all — this feature mainly signals intent and survives a transitive `fast` flip from a downstream crate (which still resolves to strict). See [strict mode](strict-mode.md). |
| `macros` | no | The `d38!` / `d76!` / etc. compile-time literal macros. See [the macro guide](macros.md). |
| `fast` | no | Opt out of strict dispatch: plain methods forward to the f64 bridge for speed at the cost of platform-libm-dependent ≈ 16-digit precision. **Only takes effect when `strict` is NOT enabled.** Three-step opt-out: `default-features = false` + add `fast` + `std` + DON'T re-add `strict`. Both `*_strict` and `*_fast` named methods stay available regardless. |

Notes:

- The `*_strict` methods (`sqrt_strict`, `ln_strict`, …) are compiled
  **regardless of the `strict` feature** - only `fast` removes
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

The wide decimal types use an in-tree, hand-rolled wide-integer
module (`crate::wide_int`); there is no external big-integer
dependency. They are opt-in per width, with three umbrella features
covering increasing precision ranges.

| Feature | Enables |
|---|---|
| `d56` | `D56` (192-bit storage, `MAX_SCALE = 57`) — half-width between D38 and D76 |
| `d76` | `D76` (256-bit storage, `MAX_SCALE = 76`) |
| `d114` | `D114` (384-bit storage, `MAX_SCALE = 115`) — half-width between D76 and D153 |
| `d153` | `D153` (512-bit storage, `MAX_SCALE = 153`) |
| `d230` | `D230` (768-bit storage, `MAX_SCALE = 230`) — half-width between D153 and D307 |
| `d307` | `D307` (1024-bit storage, `MAX_SCALE = 307`) |
| `d461` | `D461` (1536-bit storage, `MAX_SCALE = 462`) — half-width between D307 and D615 |
| `d615` | `D615` (2048-bit storage, `MAX_SCALE = 616`) |
| `d923` | `D923` (3072-bit storage, `MAX_SCALE = 924`) — half-width between D615 and D1231 |
| `d1231` | `D1231` (4096-bit storage, `MAX_SCALE = 1232`) |
| `wide` | umbrella: enables D56 / D76 / D114 / D153 / D230 / D307 |
| `x-wide` | umbrella: adds D461 / D615 on top of `wide` |
| `xx-wide` | umbrella: adds D923 / D1231 on top of `x-wide` |

The half-width tiers exist so you only pay storage cost for the
precision you actually need — if your accumulator needs ~60 digits,
`D56` saves a third of the bytes per value vs `D76`, and the same
across every wider tier pair. Every adjacent pair in the ladder has
`From` / `TryFrom` impls plus `.widen()` / `.narrow()` helpers.

## Nightly-only

| Feature | Enables |
|---|---|
| `experimental-floats` | `f16` / `f128` entry points on the float bridge (`from_f16`, `to_f128`, …). Requires a nightly toolchain. |

## Common configurations

```toml
# Default — std, serde, and the integer-only ≤ 0.5 ULP `*_strict`
# transcendentals dispatched by plain `sin` / `ln` / `sqrt`.
decimal-scaled = "0.2.5"

# `no_std`, still with serde and the deterministic strict path.
decimal-scaled = { version = "0.2.5", default-features = false,
                   features = ["serde", "alloc", "strict"] }

# Add the half-width and wider tiers (D56–D307).
decimal-scaled = { version = "0.2.5", features = ["wide", "macros"] }

# Add the extra-wide tiers (D461 / D615) on top of wide.
decimal-scaled = { version = "0.2.5", features = ["x-wide", "macros"] }

# Add the xx-wide tiers (D923 / D1231) — research-grade precision.
decimal-scaled = { version = "0.2.5", features = ["xx-wide", "macros"] }

# Bank-statement rounding: HalfAwayFromZero as the crate-wide default.
decimal-scaled = { version = "0.2.5",
                   features = ["wide", "rounding-half-away-from-zero"] }

# Speed over determinism — plain transcendentals dispatch to the f64
# bridge (~16 decimal digits of platform-libm precision). The
# `*_strict` named methods remain available for the parts of your
# code that need them.
decimal-scaled = { version = "0.2.5", default-features = false,
                   features = ["std", "fast"] }
```
