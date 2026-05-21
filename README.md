# decimal-scaled

[![crates.io](https://img.shields.io/crates/v/decimal-scaled.svg)](https://crates.io/crates/decimal-scaled) [![docs.rs](https://docs.rs/decimal-scaled/badge.svg)](https://docs.rs/decimal-scaled) [![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](https://blog.rust-lang.org/) [![License](https://img.shields.io/crates/l/decimal-scaled.svg)](#license) [![site](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml/badge.svg?branch=main&label=site)](https://github.com/mootable/decimal-scaled/actions/workflows/docs.yml) [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/mootable/decimal-scaled?utm_source=badge) [![OpenSSF Best Practices](https://www.bestpractices.dev/projects/12895/badge)](https://www.bestpractices.dev/projects/12895) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/mootable/decimal-scaled/badge)](https://securityscorecards.dev/viewer/?uri=github.com/mootable/decimal-scaled) [![cargo-audit](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml/badge.svg?branch=main)](https://github.com/mootable/decimal-scaled/actions/workflows/cargo-audit.yml)

**[Docs](https://mootable.github.io/decimal-scaled/)** • **[Benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/)** • **[Algorithms](https://mootable.github.io/decimal-scaled/ALGORITHMS/)** • **[Roadmap](https://mootable.github.io/decimal-scaled/ROADMAP/)** • **[API reference](https://docs.rs/decimal-scaled)**

Const-generic base-10 fixed-point decimals for Rust — **bit-exact**,
**≤ 0.5 [ULP][ULP] correctly-rounded** integer-only transcendentals,
deterministic on every platform, `no_std`-friendly.

[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place

---

## Install

<!-- BEGIN GENERATED:install:dependency -->
```toml
[dependencies]
decimal-scaled = { version = "0.4", features = ["macros"] }
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

## Correctly rounded across the whole surface

Worst-case error of each transcendental, measured against a
high-precision [mpmath][mpmath] oracle (worst result across every tested
input). Each cell is the **LSBε** — *least significant bits in error*,
the count of low-order bits of the stored value that are wrong — with
the worst **[ULP][ULP]** distance from the true value in parentheses.
`0 (0)` means correctly rounded: the stored value is bit-exact under
that crate's reported mode.

[mpmath]: https://mpmath.org/

Measured at a 19-digit scale (`D38<19>`), generated straight from the
committed shootout result files:

<!-- BEGIN GENERATED:precision:D38:readme -->
| library | mode | sqrt | cbrt | exp | ln | sin | cos | tan | atan | asinh |
|---|---|---|---|---|---|---|---|---|---|---|
| decimal-scaled | HalfToEven | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) |
| fastnum | HalfAwayFromZero | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 0 (0) | 67 (1.1e20) | 0 (0) | 58 (2.0e17) |
| rust_decimal | HalfToEven | 1 (1.00) | n/a | 33 (7.2e9) | 31 (1.1e9) | 1 (1.00) | 1 (1.00) | 36 (4.3e10) | n/a | n/a |
| dashu-float | HalfAwayFromZero | 0 (0) | n/a | 0 (0) | 0 (0) | n/a | n/a | n/a | n/a | n/a |
| decimal-rs | HalfToEven | 1 (1.00) | n/a | 1 (1.00) | 1 (1.00) | n/a | n/a | n/a | n/a | n/a |
| bigdecimal | HalfToEven | 1 (1.00) | 1 (1.00) | n/a | n/a | n/a | n/a | n/a | n/a | n/a |
| g_math | HalfToEven | 6 (4.9e1) | n/a | 65 (2.3e19) | 6 (4.8e1) | 64 (1.5e19) | 6 (4.9e1) | 65 (2.0e19) | 64 (1.6e19) | 64 (1.8e19) |
<!-- END GENERATED:precision:D38:readme -->

`0 (0)` = correctly rounded (0 LSBε, bit-exact under that crate's
reported mode) on *every* tested input; the first number is worst-case
LSBε, the parenthesised one the worst ULP distance from true. `n/a` =
not exposed by that crate, or the width/scale isn't representable.

**`decimal-scaled` is `0 (0)` on the entire surface** — and that holds
for all six rounding modes and all thirteen widths (`D9` … `D1232`).
`fastnum` is the closest peer: correctly rounded almost everywhere,
missing only `tan` (67 LSBε) and `asinh` (58 LSBε) at this scale.
`dashu-float` is correctly rounded on the `exp` / `ln` / `sqrt` surface
it exposes. `rust_decimal`, `decimal-rs`, and `bigdecimal` carry genuine
1-or-more-LSB gaps on the functions they implement. `g_math` — which
advertises "0 ULP transcendentals" — is in fact tens of LSBε off on most
of its surface here (`exp` 65 LSBε, `sin` 64, `tan` 65), the empirical
refutation of that claim at the matched 19-digit width.

*Scope of this table:* it shows a representative slice of the full
22-function surface at `D38<19>` under each crate's native mode; the
complete per-method, per-width tables (`D38`, the `D76` subset, and the
deep-scale `D307<150>` tier) are generated from the same result files by
[`scripts/render_precision_table.py`](scripts/render_precision_table.py)
and reproduced in the
[benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/).
decimal-scaled's `0 (0)` holds across **all six rounding modes** and
**all thirteen widths**.

## Speed across the width range

Per-operation throughput against the wider numeric ecosystem (`bnum`,
`ruint`, `rust_decimal`, `fixed`), at two representative widths:

**i128 / D38** — 128-bit, scale 19:

![operations at 128-bit vs the ecosystem](https://raw.githubusercontent.com/mootable/decimal-scaled/main/docs/figures/library_comparison/summary_128bit.png)

**i1024 / D307** — 1024-bit, scale 150:

![operations at 1024-bit vs the ecosystem](https://raw.githubusercontent.com/mootable/decimal-scaled/main/docs/figures/library_comparison/summary_1024bit.png)

Full per-width charts (32-bit … 4096-bit) and the methodology are in the
[benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/).

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
