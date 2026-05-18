# Cargo features

```toml
[dependencies]
decimal-scaled = { version = "0.4", default-features = false, features = ["alloc"] }
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
| `dyn` | no | Object-safe `DynDecimal` trait + `DecimalWidth` / `RawStorage` enums for runtime-polymorphic decimal handles. Ships impls for D9 / D18 / D38. See [Runtime polymorphism](#runtime-polymorphism). |

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

The wide decimal types use an in-tree, hand-rolled wide-integer
module (`crate::wide_int`); there is no external big-integer
dependency. They are opt-in per width, with three umbrella features
covering increasing precision ranges.

| Feature | Enables |
|---|---|
| `d57` | `D57` (192-bit storage, `MAX_SCALE = 56`) — half-width between D38 and D76 |
| `d76` | `D76` (256-bit storage, `MAX_SCALE = 75`) |
| `d115` | `D115` (384-bit storage, `MAX_SCALE = 114`) — half-width between D76 and D153 |
| `d153` | `D153` (512-bit storage, `MAX_SCALE = 152`) |
| `d230` | `D230` (768-bit storage, `MAX_SCALE = 229`) — half-width between D153 and D307 |
| `d307` | `D307` (1024-bit storage, `MAX_SCALE = 306`) |
| `d462` | `D462` (1536-bit storage, `MAX_SCALE = 461`) — half-width between D307 and D616 |
| `d616` | `D616` (2048-bit storage, `MAX_SCALE = 615`) |
| `d924` | `D924` (3072-bit storage, `MAX_SCALE = 923`) — half-width between D616 and D1232 |
| `d1232` | `D1232` (4096-bit storage, `MAX_SCALE = 1231`) |
| `wide` | umbrella: enables D57 / D76 / D115 / D153 / D230 / D307 |
| `x-wide` | umbrella: adds D462 / D616 on top of `wide` |
| `xx-wide` | umbrella: adds D924 / D1232 on top of `x-wide` |

The half-width tiers exist so you only pay storage cost for the
precision you actually need — if your accumulator needs ~60 digits,
`D57` saves a third of the bytes per value vs `D76`, and the same
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
decimal-scaled = "0.4"

# `no_std`, still with serde and the deterministic strict path.
decimal-scaled = { version = "0.4", default-features = false,
                   features = ["serde", "alloc", "strict"] }

# Add the half-width and wider tiers (D57–D307).
decimal-scaled = { version = "0.4", features = ["wide", "macros"] }

# Add the extra-wide tiers (D462 / D616) on top of wide.
decimal-scaled = { version = "0.4", features = ["x-wide", "macros"] }

# Add the xx-wide tiers (D924 / D1232) — research-grade precision.
decimal-scaled = { version = "0.4", features = ["xx-wide", "macros"] }

# Bank-statement rounding: HalfAwayFromZero as the crate-wide default.
decimal-scaled = { version = "0.4",
                   features = ["wide", "rounding-half-away-from-zero"] }

# Speed over determinism — plain transcendentals dispatch to the f64
# bridge (~16 decimal digits of platform-libm precision). The
# `*_strict` named methods remain available for the parts of your
# code that need them.
decimal-scaled = { version = "0.4", default-features = false,
                   features = ["std", "fast"] }
```

## Runtime polymorphism

The typed [`Decimal`](https://docs.rs/decimal-scaled/latest/decimal_scaled/trait.Decimal.html)
trait is monomorphised: every `Dxx<S>` is a distinct compile-time type
and generic code over `T: Decimal` pays no runtime cost. That model
breaks down when the width or scale is chosen at runtime — config-driven
types, plugin interfaces, a `Vec<Box<…>>` of mixed decimals.

The `dyn` feature adds a deliberately small, object-safe trait
`DynDecimal` for exactly that case:

```rust,ignore
use decimal_scaled::{D38, DynDecimal, DecimalWidth};

let values: Vec<Box<dyn DynDecimal>> = vec![
    Box::new(D38::<2>::from_i32(150)),  // 1.50
    Box::new(D38::<5>::from_i32(2)),    // 2.00000
];
let sum = values[0].add(&*values[1]).unwrap();
assert_eq!(sum.width(), DecimalWidth::D38);
assert_eq!(sum.scale_dyn(), 5);          // auto-rescale to wider scale
```

Semantics:

- Binary ops on **different widths** return `None`. No implicit widening
  across storage tiers.
- Binary ops on the **same width but different scales** losslessly
  rescale both sides to the wider scale and return the result at that
  scale.
- Overflow at any step returns `None` instead of panicking.
- Downcast back to the typed surface via
  `DynDecimal::as_any().downcast_ref::<Dxx<S>>()` once you know the
  concrete type.

Scope: the `dyn` feature ships impls for **D9, D18, and D38** only.
Wider tiers would require enumerating up to 1232 scale instantiations
per binary op per width and serve compute-bound code where the boxing
cost of `dyn` is wrong anyway. The `DecimalWidth` / `RawStorage` enums
carry variants for every shipped width so the API is forward-compatible
if those impls land later.

Cost: each binary op heap-allocates one `Box<dyn DynDecimal>` (plus
intermediate boxes when auto-rescale is needed). Use the typed
`Decimal` surface in hot paths.
