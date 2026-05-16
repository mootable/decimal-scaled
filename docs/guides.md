# `decimal-scaled` - usage guides

**[Docs](https://mootable.github.io/decimal-scaled/)** • **[Benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/)** • **[Algorithms](https://mootable.github.io/decimal-scaled/ALGORITHMS/)** • **[Roadmap](https://mootable.github.io/decimal-scaled/ROADMAP/)** • **[API reference](https://docs.rs/decimal-scaled)**

`decimal-scaled` provides const-generic, base-10 fixed-point decimal
types with deterministic, bit-exact arithmetic. A value is stored as an
integer `raw` such that the logical value is `raw × 10^(-SCALE)`, where
`SCALE` is a compile-time const generic.

These guides cover how to use the library in depth. For the high-level
pitch and the "why another numeric type" comparison, see the
[crate README](../README.md). For the full API, see the
[rustdoc](https://docs.rs/decimal-scaled/).

## Guides

| Guide | What it covers |
|---|---|
| [Getting started](getting-started.md) | Installing, constructing values, basic arithmetic, formatting, parsing. |
| [The width family](widths.md) | `D9` / `D18` / `D38` / `D76` / `D153` / `D307` - when to pick which, scale ranges, the `Decimal` trait. |
| [Conversions](conversions.md) | Integer / float conversions, cross-width widening and narrowing, `to_int`, the float bridge. |
| [Rounding](rounding.md) | `RoundingMode`, the `_with` method pairs, `rescale`, and the compile-time `rounding-*` feature flags. |
| [Strict mode](strict-mode.md) | Integer-only transcendentals (`ln`, `exp`, `sqrt`, trig, …) under `--features strict`. |
| [The `d38!` macro](macros.md) | Compile-time decimal literals with automatic scale inference, scientific notation, explicit scale, rounding, and inline expressions. |
| [Cargo features](features.md) | Every feature flag and what it enables. |
| [Benchmarks](benchmarks.md) | Head-to-head timings against `bnum`, `ruint`, `rust_decimal`, and `fixed`; fast vs strict transcendentals; the 0.5 [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) guarantee. |

## A 30-second tour

```rust
use decimal_scaled::{D38s2, d38};

// Compile-time literal - scale inferred from the written digits.
let price = d38!(19.99);              // D38s2, exactly 19.99
let qty   = D38s2::from_int(3);       // 3.00

let total = price * qty;               // 59.97, exact - no binary rounding
assert_eq!(total, d38!(59.97));

// Deterministic: identical bit pattern on every platform.
assert_eq!(total.to_bits(), 5997);
```
