# decimal-scaled

[![crates.io](https://img.shields.io/crates/v/decimal-scaled.svg)](https://crates.io/crates/decimal-scaled) [![docs.rs](https://docs.rs/decimal-scaled/badge.svg)](https://docs.rs/decimal-scaled) [![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](https://blog.rust-lang.org/) [![License](https://img.shields.io/crates/l/decimal-scaled.svg)](#license) [![site](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml/badge.svg?branch=main&label=site)](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml) [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/mootable/decimal-scaled?utm_source=badge) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/12895/badge)](https://www.bestpractices.dev/projects/12895) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/mootable/decimal-scaled/badge)](https://securityscorecards.dev/viewer/?uri=github.com/mootable/decimal-scaled) [![cargo-audit](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml/badge.svg?branch=main)](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml)

**[Docs](https://mootable.github.io/decimal-scaled/)** • **[Benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/)** • **[Algorithms](https://mootable.github.io/decimal-scaled/ALGORITHMS/)** • **[Roadmap](https://mootable.github.io/decimal-scaled/ROADMAP/)** • **[API reference](https://docs.rs/decimal-scaled)**

Const-generic base-10 fixed-point decimals for Rust — **bit-exact**,
**≤ 0.5 [ULP][ULP] correctly-rounded** integer-only transcendentals,
deterministic on every platform, `no_std`-friendly.

[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place

---

## Install

```toml
[dependencies]
decimal-scaled = { version = "0.4", features = ["macros"] }
```

## First use

```rust
use decimal_scaled::{d38, D38s12};

let price: D38s12 = "19.99".parse().unwrap();
let qty   = D38s12::from_int(3);
let total = price * qty;                       // 59.97 exactly

assert_eq!(total.to_string(), "59.97");
assert_eq!(total, d38!(59.97, scale 12));

// Transcendentals are correctly rounded to ≤ 0.5 ULP, integer-only,
// and bit-identical across platforms.
let sqrt2 = d38!(2, scale 12).sqrt_strict();
```

## What it does

Every value is `raw × 10^(-SCALE)` for a compile-time `SCALE`. There is
exactly one bit pattern per logical value — no normalisation, no
per-value scale byte, no heap allocation. `0.1 + 0.2 == 0.3` is `true`,
and so is `hash(1.10) == hash(1.1)` at the same scale.

Thirteen storage widths from `D9` (32-bit, ~9 decimal digits) to
`D1232` (4096-bit, ~1232 decimal digits) share an identical API.
Pick the narrowest width that covers your range.

The two guarantees nothing else on crates.io currently combines:

1. **≤ 0.5 ULP correctness on every transcendental** — `ln` / `exp` /
   `sin` / `cos` / `tan` / `sqrt` / `cbrt` / `powf` / `asin` / `acos` /
   `atan` / `atan2` / `sinh` / `cosh` / `tanh` / `asinh` / `acosh` /
   `atanh` / `to_degrees` / `to_radians` lands within half an [ULP][ULP]
   of the mathematically exact result, and the bit pattern is identical
   on every machine. The default `strict` path is integer-only and
   `no_std`-compatible.
2. **Caller-chosen rounding mode at every lossy operation.** The
   default is `HalfToEven` (IEEE 754 default). Every lossy entry point
   (`*` / `/` / `%`, the `rescale` family, every strict transcendental)
   ships a `*_with(mode)` sibling that takes a `RoundingMode`:
   `HalfToEven` · `HalfAwayFromZero` · `HalfTowardZero` · `Ceiling` ·
   `Floor` · `Trunc`. The crate-wide default is also selectable at
   compile time via the `rounding-*` Cargo features.

---

## Documentation

In-depth guides live under [`docs/`](docs/):

- [Getting started](docs/getting-started.md) — constructing values, arithmetic, formatting, parsing.
- [The width family](docs/widths.md) — `D9` through `D1232`, scale aliases, the `Decimal` trait, picking a tier.
- [Conversions](docs/conversions.md) — integers, floats, cross-width widening / narrowing, the float bridge.
- [Cross-scale operations](docs/cross-scale.md) — `mul_of` / `add_of` / `cmp_of` / `clamp_of` / etc. on every width for mixed-width mixed-SCALE expressions, plus the nightly-gated `cross::mul(a, b)` auto-inferred form.
- [Rounding](docs/rounding.md) — `RoundingMode`, the `_with` pairs, `rescale`, the compile-time `rounding-*` features.
- [Strict mode](docs/strict-mode.md) — integer-only `*_strict` transcendentals, the ≤ 0.5 ULP guarantee.
- [The `d*!` macros](docs/macros.md) — compile-time decimal literals, scale inference, scientific / radix notation.
- [Cargo features](docs/features.md) — every feature flag with what it enables and the common configurations.
- [Benchmarks](docs/benchmarks.md) — head-to-head against `bnum`, `ruint`, `rust_decimal`, `fixed`, `fastnum`, `dashu-float`, `bigdecimal`, and the fast-vs-strict trade.
- [Algorithms](ALGORITHMS.md) — every kernel with its citation (Möller–Granlund, Brent, Knuth, Karatsuba, Burnikel–Ziegler, Mercator / Cody–Waite, …).
- [Roadmap](ROADMAP.md) — what's queued (signed `SCALE`, RNG surface, wide-tier perf catch-up, downstream adapter / ecosystem crates).
- [Changelog](CHANGELOG.md) — release-by-release notes.
- [Contributing](CONTRIBUTORS.md) — algorithm-library tour, adding a per-(width, scale) override, performance gates, license-compatibility rules.

API reference: <https://docs.rs/decimal-scaled/>.

---

## License

Licensed under either of:

- MIT license ([LICENSES/MIT.md](LICENSES/MIT.md))
- Apache License, Version 2.0 ([LICENSES/Apache-2.0.md](LICENSES/Apache-2.0.md))

at your option.

Copyright 2026 John Moxley. Third-party code attributions are listed in
[LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).
