# decimal-scaled

[![crates.io](https://img.shields.io/crates/v/decimal-scaled.svg)](https://crates.io/crates/decimal-scaled) [![docs.rs](https://docs.rs/decimal-scaled/badge.svg)](https://docs.rs/decimal-scaled) [![MSRV](https://img.shields.io/badge/MSRV-1.89-blue.svg)](https://blog.rust-lang.org/) [![License](https://img.shields.io/crates/l/decimal-scaled.svg)](#license) [![site](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml/badge.svg?branch=main&label=site)](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/12895/badge)](https://www.bestpractices.dev/projects/12895) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/mootable/decimal-scaled/badge)](https://securityscorecards.dev/viewer/?uri=github.com/mootable/decimal-scaled) [![cargo-audit](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml/badge.svg?branch=main)](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml) [![CodeQL](https://github.com/mootable/decimal-scaled/actions/workflows/codeql.yml/badge.svg?branch=main)](https://github.com/mootable/decimal-scaled/actions/workflows/codeql.yml)

**[Docs](https://mootable.github.io/decimal-scaled/)** • **[Performance](https://mootable.github.io/decimal-scaled/performance/)** • **[Comparisons](https://mootable.github.io/decimal-scaled/comparisons/)** • **[Algorithms](https://mootable.github.io/decimal-scaled/ALGORITHMS/)** • **[Roadmap](https://mootable.github.io/decimal-scaled/ROADMAP/)** • **[API reference](https://docs.rs/decimal-scaled)**

A fast, precise, decimal library.

- **Decimal storage** — unlike floating point, `1.1` is exactly `1.1`.
- **Multiple rounding modes** — six in total, `HalfToEven` by default.
- **Up to 4 Kb numbers** — twelve widths, `D18` to `D1232`.
- **`no_std` friendly** — the strict, integer-only path needs no `std`.
- **Validated by <!-- BEGIN GENERATED:readme:tested -->56,198,568<!-- END GENERATED:readme:tested --> value tests** — every width × scale × rounding mode.

---

## Install

<!-- BEGIN GENERATED:install:dependency -->
```toml
[dependencies]
decimal-scaled = { version = "0.5", features = ["macros"] }
```
<!-- END GENERATED:install:dependency -->

## First use

```rust
use decimal_scaled::{d38, D38s12};

// The most common way to make a value: a literal, scale inferred from
// its digits.
let x = d38!(1.564232);                        // D38<6>
assert_eq!(x.to_string(), "1.564232");

let price: D38s12 = "19.99".parse().unwrap();
let qty = D38s12::try_from(3i64).unwrap();     // integer, scaled by 10^SCALE (fallible)
let total = price * qty;                       // 59.97 exactly

assert_eq!(total.to_string(), "59.97");
assert_eq!(total, d38!(59.97, scale 12));

// Transcendentals are correctly rounded to <= 0.5 ULP, integer-only,
// and bit-identical across platforms.
let sqrt2 = d38!(2, scale 12).sqrt_strict();
```

## Why decimal-scaled

| You need… | decimal-scaled gives you… |
|---|---|
| Decimal arithmetic that doesn't drift (`0.1 + 0.2 == 0.3`) | Base-10 storage; exact `+ - %`, correctly-rounded `* /`. |
| Bit-identical results across Linux / macOS / Windows / ARM / x86 | `*_strict` transcendentals — integer-only, no platform libm. |
| Compile-time-fixed precision with zero per-value scale byte | Const-generic `D38<19>`, `D76<35>` etc. — scale is in the type. |
| `no_std` (or `no_std + alloc`) | Builds under `no_std + alloc` with `default-features = false`; the strict, integer-only path needs no libm. |
| Correctly-rounded `ln` / `exp` / `sin` / `cos` / `tan` / `sqrt` / `atan` and friends — by default | Within 0.5 [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place), `HalfToEven` by default; switch per call via `*_with(mode)` or crate-wide via the `rounding-*` features. |

---

## Documentation

Full docs: <https://mootable.github.io/decimal-scaled/>.

- [Getting started](docs/getting-started.md) — constructing values, arithmetic, formatting, parsing.
- [The width family](docs/widths.md) — `D18` through `D1232`, scale aliases, the `Decimal` trait, picking a tier.
- [Conversions](docs/conversions.md) — integers, floats, cross-width widening / narrowing, the float bridge.
- [Cross-scale operations](docs/cross-scale.md) — mixed-width, mixed-`SCALE` expressions via `mul_of` / `add_of` / `cmp_of` / `clamp_of`, plus the nightly auto-inferred form.
- [Rounding](docs/rounding.md) — `RoundingMode`, the `_with` pairs, `rescale`, the compile-time `rounding-*` features.
- [Strict vs fast transcendentals](docs/strict-mode.md) — the integer-only `*_strict` path and the 0.5 ULP guarantee.
- [The `d*!` macros](docs/macros.md) — compile-time decimal literals and scale inference.
- [Cargo features](docs/features.md) — every feature flag and the common configurations.
- **Bench** — per-width [Performance](docs/performance.md), the [Precision](docs/precision.md) surface, version [History](docs/history.md), the golden [Harness](docs/golden.md), and the like-for-like [Comparisons](docs/comparisons.md) against the top crates.io peers.
- [Algorithms](ALGORITHMS.md) — every kernel with its citation (Möller–Granlund, Brent, Knuth, Karatsuba, Burnikel–Ziegler, Cody–Waite, …).
- [Roadmap](ROADMAP.md) · [Changelog](CHANGELOG.md) · [Contributing](CONTRIBUTORS.md).

API reference: <https://docs.rs/decimal-scaled/>.

---

## License

Licensed under either of:

- MIT license ([LICENSES/MIT.md](LICENSES/MIT.md))
- Apache License, Version 2.0 ([LICENSES/Apache-2.0.md](LICENSES/Apache-2.0.md))

at your option.

Copyright 2026 John Moxley. Third-party code attributions are listed in
[LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).
