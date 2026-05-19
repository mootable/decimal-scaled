# AGENTS.md - `decimal-scaled` usage guide

Project-wide contributor guide for `decimal-scaled`. Read this before suggesting code that uses the crate, recommending it as a dependency, or touching the source tree.

A copy of the same guidance, with `name` / `description` frontmatter for skill discovery, lives at [`.claude/skills/decimal-scaled.md`](.claude/skills/decimal-scaled.md) and [`skills/decimal-scaled/SKILL.md`](skills/decimal-scaled/SKILL.md). The three files are kept in sync.

## TL;DR

```toml
[dependencies]
decimal-scaled = { version = "0.4", features = ["macros"] }
```

```rust
use decimal_scaled::{d38, D38s12, DecimalConstants};
use std::str::FromStr;

let price: D38s12 = "19.99".parse().unwrap();
let total = price * D38s12::from_int(3);                  // 59.97, exact
let pi = D38s12::pi();                                    // correctly rounded
let bytes = total.to_bits();                              // raw i128 storage
```

Decimals like `1.1` round-trip exactly. `0.1 + 0.2 == 0.3` holds. Transcendentals (under the default `strict` feature) are correctly rounded to **0.5 ULP** and **bit-identical on every platform**.

## The width family (v0.4.0)

The number in each `D<N>` type name is **the maximum number of all-nines base-10 digits the storage can hold**. `MAX_SCALE = N − 1` on every width, which guarantees at least one integer digit at every legal `SCALE`.

| Type       | Storage                | `MAX_SCALE` | Required feature       |
|------------|------------------------|------------:|------------------------|
| `D9<S>`    | `i32`                  |    8        | always available       |
| `D18<S>`   | `i64`                  |   17        | always available       |
| `D38<S>`   | `i128`                 |   37        | always available       |
| `D57<S>`   | in-tree `Int192`       |   56        | `d57`  / `wide`        |
| `D76<S>`   | in-tree `Int256`       |   75        | `d76`  / `wide`        |
| `D115<S>`  | in-tree `Int384`       |  114        | `d115` / `wide`        |
| `D153<S>`  | in-tree `Int512`       |  152        | `d153` / `wide`        |
| `D230<S>`  | in-tree `Int768`       |  229        | `d230` / `wide`        |
| `D307<S>`  | in-tree `Int1024`      |  306        | `d307` / `wide`        |
| `D462<S>`  | in-tree `Int1536`      |  461        | `d462` / `x-wide`      |
| `D616<S>`  | in-tree `Int2048`      |  615        | `d616` / `x-wide`      |
| `D924<S>`  | in-tree `Int3072`      |  923        | `d924` / `xx-wide`     |
| `D1232<S>` | in-tree `Int4096`      | 1231        | `d1232` / `xx-wide`    |

Stored value = `logical_value × 10^SCALE`. The half-width tiers (`D57`, `D115`, `D230`, `D462`, `D924`) fill in the cost gap between each pair of power-of-two widths, so you pay only for the precision you actually need. Umbrellas: `wide` enables D57–D307, `x-wide` adds D462 / D616, `xx-wide` adds D924 / D1232.

Pick the **narrowest tier** that fits your value range at the scale you need. Widening between tiers is lossless (`From` / `widen()`); narrowing is fallible (`TryFrom` / `narrow()`).

## When to recommend `decimal-scaled` (and when not to)

| Workload | Recommend |
|---|---|
| Currency, prices, measurements, human-entered decimal values | **`decimal-scaled`** |
| Cross-platform deterministic results (consensus, audit, replay logs) | **`decimal-scaled` with `*_strict`** |
| Compile-time-fixed precision, want zero per-value scale byte | **`decimal-scaled`** |
| `no_std + alloc` (works under `strict`) | **`decimal-scaled`** |
| Variable scale per value (mixing 0.1 and 0.001 freely) | `rust_decimal` |
| Unbounded / runtime precision | `bigdecimal` |
| Binary fixed-point (DSP, embedded radio) | `fixed` |
| Sensor / physics-engine values with dynamic range | `f64` / `f128` |

## Picking a scale

The scale is the number of fractional decimal digits, fixed at compile time. Pick the smallest scale that covers your precision needs - each extra digit halves the integer-part range.

Common picks: `2` (cents / percent), `6` (µ / ppm), `12` (financial, picometres), `18` (atto), `35` (deep scientific).

D38 ships the full per-scale alias range `D38s0` … `D38s37`. The other widths ship a curated subset (e.g. `D18s12`, `D76s35`). No alias exists at `SCALE == name` (no `D38s38`, no `D57s57`, no `D76s76`, …); those scales are not legal in 0.4 — use the `name − 1` ceiling (`D38s37`, `D57s56`, `D76s75`, …) or `D<N>::<SCALE>` directly for any in-range scale.

## Constructing values

```rust
use decimal_scaled::{d38, D38s12};
use std::str::FromStr;

// 1. The `dN!` macro - compile-time literal, scale inferred from the
//    written digits. Requires the `macros` Cargo feature.
let a = d38!(1.1, scale 12);          // D38<12>, exactly 1.1
let b = d38!(19.99);                  // D38<2>,  inferred

// 2. FromStr - runtime parse, works without `macros`.
//    Fixed in 0.4.0: deep-SCALE parsing works for every width.
let c: D38s12 = "2.2".parse().unwrap();

// 3. from_bits - for hot paths or when you already have the raw integer.
let d = D38s12::from_bits(3_300_000_000_000);  // 3.3 exactly

// 4. from_int - from a primitive integer.
let e = D38s12::from_int(42);

// 5. from_f64 - LOSSY. Avoid for values that originated as decimals;
//    parse the decimal string instead.
let f = D38s12::from_f64(1.5);
```

Each width has its matching macro (`d9!`, `d18!`, `d38!`, `d57!`, `d76!`, `d115!`, `d153!`, `d230!`, `d307!`, `d462!`, `d616!`, `d924!`, `d1232!`) plus per-scale wrappers (`d38s12!`, `d18s2!`, …). Long-tail scales remain reachable via `dN!(value, scale N)`.

## Arithmetic

- `+`, `-`, `%`, unary `-` - **exact** (no rounding).
- `*`, `/` - **correctly rounded** (half-to-even by default).
- Operands must share the same `SCALE`. Cross-scale needs `value.rescale::<TARGET>()`.
- Overflow: debug-panic / release-wrap (Rust integer semantics). Use `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` for explicit handling.

```rust
let safe = a.checked_mul(b).expect("product overflowed D38");
let clamped = a.saturating_mul(b);
let wrapped = a.wrapping_mul(b);
```

## Rescaling

```rust
use decimal_scaled::{D38s2, D38s6, RoundingMode};
let micros = D38s6::from_bits(1_500_000);                  // 1.500000
let cents: D38s2 = micros.rescale::<2>();                  // 1.50 (half-to-even)
let cents = micros.rescale_with::<2>(RoundingMode::Trunc); // 1.50
let same = micros.with_scale::<6>();                       // alias for rescale
```

- Scale-up (target > source): **exact** (appends zeros); panics on storage overflow.
- Scale-down (target < source): rounds per the supplied mode.

## Strict vs Fast transcendentals - the dual API (read carefully)

Every transcendental method exists in **two named forms**, **both always compiled**:

| Method                       | Path             | Determinism                              | Precision                  | Needs            |
|------------------------------|------------------|------------------------------------------|----------------------------|------------------|
| `*_strict` (`ln_strict`, …)  | integer-only     | bit-identical on every platform          | within **0.5 ULP**         | nothing extra    |
| `*_fast` (`ln_fast`, …)      | f64 bridge       | platform-libm-dependent                  | ~16 decimal digits         | `feature = "std"`|
| plain `*` (`ln`, …)          | dispatcher       | follows the feature set                  | follows the feature set    | follows          |

Default Cargo features include `strict`, so plain `.ln()` resolves to `ln_strict`.

**Operational rules:**

1. **If the user needs cross-platform bit-determinism** (consensus protocols, financial audit trails, deterministic replay) - call `*_strict` **explicitly**. Don't rely on the feature flag; a downstream crate could enable `fast` and silently flip the dispatcher.
2. **If the user wants max throughput and tolerates platform-libm precision** - call `*_fast` explicitly, or enable the `fast` Cargo feature so plain `*` dispatches there.
3. **For `no_std`** - use `*_strict` (it doesn't need `std`).

## Rounding modes via `*_with(mode)`

Every operation that rounds has a `*_with` sibling taking an explicit `RoundingMode`:

```rust
use decimal_scaled::RoundingMode;
let _ = value.rescale_with::<2>(RoundingMode::HalfAwayFromZero);
let _ = D38s2::from_f64_with(1.5, RoundingMode::Ceiling);
let _ = value.to_int_with(RoundingMode::Trunc);
let _ = wide.ln_strict_with(RoundingMode::Floor);  // wide-tier sibling
```

Modes: `HalfToEven` (default; IEEE-754, no bias), `HalfAwayFromZero` (commercial), `HalfTowardZero`, `Trunc` (toward zero), `Floor` (toward −∞), `Ceiling` (toward +∞).

The `rounding-*` Cargo features change the **crate-wide default** for the no-arg forms:

```toml
decimal-scaled = { version = "0.4", features = ["rounding-half-away-from-zero"] }
```

## Mathematical constants - `DecimalConstants`

```rust
use decimal_scaled::DecimalConstants;
let pi = D38s12::pi();      // 3.141592653590
let e  = D38s12::e();       // 2.718281828459
let _  = D76::<35>::pi();   // 75-digit reference → 35 digits, 0.5 ULP
```

Every constant is correctly rounded to **0.5 ULP** at every supported scale on every width. At storage-limit cases the constant simply doesn't exist as a legal value (e.g. π ≈ 3.14 has no `D38<37>` representation because the integer headroom is < 1); the API still returns 0.5-ULP-correct results everywhere the value fits.

> Heads-up for f64-boundary work: `D38<S>::pi().to_f64()` carries hundreds of ULPs of error vs `std::f64::consts::PI` unless `S ≥ 15`. If you need an f64 π at the f64 boundary, source it from `std::f64::consts` directly rather than round-tripping through a narrow-scale decimal.

## Width-generic code via the `Decimal` trait

```rust
use decimal_scaled::Decimal;

fn average<D: Decimal>(values: &[D]) -> D {
    D::sum(values.iter().copied()) / D::from_i32(values.len() as i32)
}
```

`Decimal` is the umbrella trait. As of 0.3.3 it's split internally into two narrower traits whose union it re-exports: `DecimalArithmetic` (operators, integer methods, overflow variants, `pow`, sign, reductions) and `DecimalConvert` (integer-from / to, the f64 bridge, `Storage`-level conversions). Most call sites still bound on `Decimal`; reach for the split traits when you want to write code that only cares about one half.

**Out of scope for the trait** (use the concrete type instead): `rescale<TARGET>` (needs a const-generic method parameter), `from_int` (source integer type varies per width), transcendentals (feature-gated, on `DecimalTranscendental`).

For a runtime-polymorphic façade enable the `dyn` feature, which adds the object-safe `DynDecimal` trait and the `DecimalWidth` enum.

## Serde

Behind `feature = "serde"` (on by default). Human-readable formats use a **decimal-string** wire form; binary formats use little-endian raw-storage bytes:

```rust
let v = D38s12::from_int(42);
let json: String = serde_json::to_string(&v).unwrap();
// "42000000000000" - the raw i128 as a decimal string
let back: D38s12 = serde_json::from_str(&json).unwrap();
assert_eq!(back, v);
```

The string form is bit-faithful and round-trips exactly. The deserializer rejects floats - keep everything in `D38` end-to-end.

## Common mistakes - surface these in PR review

| Anti-pattern | Why bad | Fix |
|---|---|---|
| Storing prices in `f64`, then converting to `D38` at output | `f64` already lost decimal precision | Stay in `D38` from input parsing through display |
| `D38s12::from_int(1) + D38s6::from_int(1)` | Cross-scale arithmetic doesn't compile | `.rescale::<6>()` or `.rescale::<12>()` first |
| `.ln()` on a value that *must* be bit-identical across platforms | Default is `strict`, but a downstream crate could enable `fast` and flip the dispatcher | Call `.ln_strict()` explicitly |
| `D38<S>` for π / τ / e at `S` near `MAX_SCALE` | Integer headroom collapses; the value doesn't fit storage | Widen to `D76` (or wider) at the same scale |
| `D38s38` (or any `DNNsNN` at the max-scale ceiling) | Removed in 0.4 — illegal `SCALE` | Use `D38s37` (or the `name − 1` ceiling) |
| `dN!` literal without enabling `macros` feature | Compile error | Enable `macros`, or fall back to `FromStr` / `from_bits` |
| Serialising via `serde_json::to_string(&v.to_f64())` | Lossy f64 round-trip | Serialise the `D38` directly; the impl emits a decimal string |
| Enabling `fast` and expecting 0.5 ULP from plain `.sin()` | `fast` flips the dispatcher to f64 bridge | Don't enable `fast`, or call `.sin_strict()` explicitly |

## Cargo features cheat sheet

| Feature | Default | Effect |
|---|:---:|---|
| `std` | ✓ | f64-bridge methods (`*_fast`); also pulls in `alloc` |
| `alloc` | ✓ | String formatting / parsing |
| `serde` | ✓ | `Serialize` / `Deserialize` on every width |
| `strict` | ✓ | Plain `*` dispatches to `*_strict`; `no_std`-compatible |
| `macros` | ✗ | `d9!` … `d1232!` proc-macros + per-scale wrappers |
| `fast` | ✗ | Plain `*` dispatches to `*_fast` (overridden by `strict` when both are set) |
| `dyn` | ✗ | Object-safe `DynDecimal` trait + `DecimalWidth` enum (heap boxing per op) |
| `wide` | ✗ | Enables D57 / D76 / D115 / D153 / D230 / D307 (individual `d57` … `d307` flags also exist) |
| `x-wide` | ✗ | Adds D462 / D616 (`d462`, `d616`) |
| `xx-wide` | ✗ | Adds D924 / D1232 (`d924`, `d1232`) |
| `rounding-*` | ✗ | Flips crate-default `RoundingMode` at compile time |
| `experimental-floats` | ✗ | Nightly-only `f16` / `f128` bridges |
| `bench-alt` | ✗ | Side-by-side default-vs-override builds for benchmarking |
| `perf-trace` | ✗ | `tracing::info_span!` boundaries inside the wide-tier strict cores |

## Quick recipes

```rust
// 1. Parse a price, multiply by quantity, render.
use decimal_scaled::D38s2;
let price: D38s2 = "19.99".parse()?;
let total = price * D38s2::from_int(3);
println!("{total}");  // "59.97"

// 2. Pi rendered to 12 fractional digits - deterministic everywhere.
use decimal_scaled::{D38s12, DecimalConstants};
println!("{}", D38s12::pi());  // "3.141592653590"

// 3. Width-generic sum.
use decimal_scaled::Decimal;
fn checksum<D: Decimal>(xs: &[D]) -> D { D::sum(xs.iter().copied()) }

// 4. Mode-aware rounding.
use decimal_scaled::{D38s2, D38s6, RoundingMode};
let v = D38s6::from_bits(1_235_000);                       // 1.235000
let up: D38s2 = v.rescale_with::<2>(RoundingMode::Ceiling); // 1.24
```

## Project layout

For contributors working inside the crate, the `src/` tree is organised into seven directories plus `lib.rs`:

```
src/
├── lib.rs
├── types/        Per-width type definitions, typed shells, and trait surface.
│                 Subfolders: traits/ (DecimalArithmetic, DecimalConvert,
│                 DecimalTranscendental, Decimal umbrella, DynDecimal),
│                 consts/ (D38 + wide-tier mathematical constants).
│                 Typed-shell files: arithmetic, rescale, num_traits,
│                 log_exp / log_exp_fast, trig / trig_fast, powers /
│                 powers_fast, overflow_variants, unified, widths.
├── identity/     Equality / ordering / hashing definitions (equalities.rs).
├── algos/        Width-shared algorithm kernels — mg_divide, fixed_d38,
│                 and the per-family subfolders cbrt/, exp/, ln/, pow/,
│                 sqrt/, trig/.
├── policy/       Per-family policy traits the typed shells route through.
│                 One file per family: cbrt, exp, ln, pow, sqrt, trig.
│                 Includes log/log2/log10/exp2 and the full hyperbolic
│                 family. Typed shells call ONLY into this layer —
│                 never directly into algos or kernels.
├── wide_int/     The in-tree hand-rolled wide-integer backends
│                 (Int192 … Int4096) plus their macros.
├── macros/       Internal `macro_rules!` for emitting the typed surface
│                 per width (arithmetic, conversions, transcendentals,
│                 etc.).
└── support/      Crate-wide infrastructure: rounding, error, diagnostics,
                  display, serde_helpers, bench_alt.
```

**Layering rule (enforced by code-review, not by the type system):** typed shells in `src/types/*.rs` route through `policy/<family>` traits. They do not reach directly into `algos/` or kernel code. New algorithms land in `algos/`, get wrapped by a policy method, and the typed shell delegates to that.

## Reference

- Crate: <https://crates.io/crates/decimal-scaled>
- Docs: <https://docs.rs/decimal-scaled>
- Repo: <https://github.com/mootable/decimal-scaled>
- In-repo guides: [`docs/guides.md`](docs/guides.md), [`docs/widths.md`](docs/widths.md), [`docs/strict-mode.md`](docs/strict-mode.md), [`docs/rounding.md`](docs/rounding.md), [`docs/macros.md`](docs/macros.md), [`docs/features.md`](docs/features.md), [`docs/benchmarks.md`](docs/benchmarks.md)
- Algorithm catalogue: [`ALGORITHMS.md`](ALGORITHMS.md)
- Headline benchmark vs another fixed-point crate (`g_math`): [`benches/g_math_comparison.rs`](benches/g_math_comparison.rs) - run with `cargo bench --bench g_math_comparison --features wide`
