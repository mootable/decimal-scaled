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

## Correctly rounded — and the only crate that is

Worst-case error of each transcendental, measured against a
high-precision oracle (worst result across every tested input). Each
cell shows the **LSBe** — *least significant bits in error*, the count
of low-order bits of the stored value that are wrong — with the worst
**ULP** distance from the true value in parentheses:

| Function | decimal-scaled | g_math | fastnum | rust_decimal | dashu-float | decimal-rs |
|---|:--:|:--:|:--:|:--:|:--:|:--:|
| **exp**  | ✓ 0 (0.50) | ✗ 65 (2.3e19) | ✓ 0 (1e-16)  | ✓ 0 (2.7e-6) | ✓ 0 (1e-16) | ✓ 0 (1.3e-15) |
| **ln**   | ✓ 0 (0.50) | ✗ 6 (49.5)    | ✓ 0 (1e-16)  | ✗ 31 (1.1e9) | ✗ 1 (1.00) | ✓ 0 (1e-16) |
| **sin**  | ✓ 0 (0.50) | ✗ 64 (1.7e19) | ✓ 0 (1.7e-12)| ✓ 0 (2.1e-9) | — | — |
| **cos**  | ✓ 0 (0.50) | ✗ 6 (50.0)    | ✓ 0 (5.2e-12)| ✓ 0 (2.4e-9) | — | — |
| **tan**  | ✓ 0 (0.50) | ✗ 65 (2.1e19) | ✓ 0 (1.4e-12)| ✗ 36 (4.3e10) | — | — |
| **atan** | ✓ 0 (0.50) | ✗ 64 (1.6e19) | ✓ 0 (1e-16)  | — | — | — |
| **sqrt** | ✓ 0 (0.50) | ✗ 6 (49.6)    | ✓ 0 (1e-16)  | ✓ 0 (7.2e-7) | ✓ 0 (1e-16) | ✓ 0 (3e-16) |
| **cbrt** | ✓ 0 (0.50) | — | ✓ 0 (1e-16) | — | — | — |
| *rounding* | **all 6, caller-chosen** | nearest | HalfUp | HalfEven | HalfAway | unspec. |

**✓** = **0 LSBe** (correctly rounded — the stored value is exactly
right, ≤ 0.5 ULP from true) on *every* tested input. **✗** = at least
one input with **≥ 1 LSBe**. **—** = not implemented by that crate.
First number = worst-case **LSBe** (least significant bits in error);
parenthesised = worst ULP distance from the true value.

`decimal-scaled` is the only crate ✓ on every function — and its ✓ holds
for all six rounding modes and all thirteen widths (D9 … D1232).

**At deep precision the field collapses.** Repeat the test at a 150-digit
scale and only `decimal-scaled` computes **all eight** functions
correctly; the fixed-precision crates (g_math ≈ 19, rust_decimal ≈ 28,
fastnum ≈ 34, decimal-rs ≈ 38 digits) can no longer represent the value
at all, and arbitrary-precision `dashu-float` — the lone peer that
reaches it — still misses `ln`.

*Scope of this table:* measured at a 19-digit scale (`D38<19>`) and a
150-digit scale (`D307<150>`, with `D616<308>` also verified), under
`HalfToEven`; each cell is the worst case across the scales a library
supports. It samples a slice of the full matrix — decimal-scaled's
guarantee holds across **all six rounding modes** and **all thirteen
widths** (`D9` … `D1232`). For the complete per-scale, per-width,
per-mode tables and the methodology, see the
[benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/).

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
