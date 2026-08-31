# Reference

The per-topic reference pages, in one screen. New to the crate? Start
with [Getting started](getting-started.md) instead; the deep-dive on
how correctness is proven lives under
[Precision testing](precision-testing.md).

| Topic | What it covers |
|---|---|
| [The width family](widths.md) | The storage widths from `D18` to `D1232` — when to pick which, scale aliases, the `Decimal` trait, the required Cargo features. |
| [Rounding](rounding.md) | `RoundingMode`, the `_with` method pairs, `rescale`, and the compile-time `rounding-*` feature flags. |
| [Conversions](conversions.md) | Integer / float conversions, cross-width widening and narrowing, `to_int`, the float bridge. |
| [Serde](serde.md) | The raw-storage wire format for human-readable and binary serialisers, and the cross-system scale contract. |
| [Cross-scale operations](cross-scale.md) | Mixing widths and `SCALE`s in one expression via `mul_of` / `add_of` / `cmp_of` / `clamp_of` and friends, plus the nightly auto-inferred form. |
| [Glossary](glossary.md) | Every acronym, shortening, and term of art used across the crate, its docs, and its benchmarks. |

Related pages elsewhere on the site:

- [Decimal literals — the `d*!` macros](macros.md), [strict vs fast
  transcendentals](strict-mode.md), and [Cargo features](features.md)
  under **Getting started**.
- [Benchmarks](benchmarks.md) and [library comparisons](comparisons.md)
  under **Benchmarks & precision**.
- [Architecture](ARCHITECTURE.md), [Algorithms](ALGORITHMS.md), and
  [precision testing](precision-testing.md) under **Architecture &
  internals**.
- The full [API reference (rustdoc)](https://mootable.github.io/decimal-scaled/api/decimal_scaled/).
