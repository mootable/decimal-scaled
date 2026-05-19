---
name: decimal-scaled
description: Invoke when working with the `decimal-scaled` crate (const-generic base-10 fixed-point decimals D9/D18/D38/D57/D76/D115/D153/D230/D307/D462/D616/D924/D1232 with 0.5-ULP correctly-rounded integer-only transcendentals). Covers when to pick the crate over alternatives, picking the right width and SCALE, construction (`d38!` / FromStr / `from_bits` / `from_int`), strict-vs-fast routing, rounding modes via `*_with(mode)`, the `Decimal` trait for width-generic code, the `DecimalConsts` trait for pi/tau/e/etc, and common anti-patterns. Apply when the user writes Rust code involving currency, prices, measurements, scientific values that must round-trip exactly through human-readable decimals, or anywhere they need deterministic bit-identical arithmetic across platforms.
---

# `decimal-scaled` - agent usage guide

This skill teaches you how to use the `decimal-scaled` crate effectively. Apply it whenever the user's Rust code touches the crate, or whenever you're considering recommending it.

## What `decimal-scaled` IS

A family of const-generic base-10 fixed-point decimal types:

The number in each `D<N>` type name is **the maximum number of all-nines base-10 digits the storage can hold**. `MAX_SCALE = N − 1` on every width, guaranteeing at least one integer digit at every legal `SCALE`.

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

The const generic `SCALE` is the number of fractional digits, baked into the type. Stored as `value × 10^SCALE`. Decimals like `1.1` round-trip exactly. `0.1 + 0.2 == 0.3` holds.

The half-width tiers (`D57`, `D115`, `D230`, `D462`, `D924`) fill the cost gap between each pair of power-of-two widths, so you pay only for the precision you need. Umbrellas: `wide` enables D57–D307, `x-wide` adds D462 / D616, `xx-wide` adds D924 / D1232.

## When to reach for `decimal-scaled` (vs alternatives)

| Use case | Pick |
|---|---|
| Currency, prices, measurements, human-entered decimals | **`decimal-scaled`** |
| Need bit-identical results on every platform (consensus, audit, replay) | **`decimal-scaled` + `*_strict`** |
| Compile-time-fixed precision, want zero per-value scale byte | **`decimal-scaled`** |
| `no_std + alloc` | **`decimal-scaled`** (works under `strict` too) |
| Variable scale per value (e.g. mixing 0.1 and 0.001) | `rust_decimal` |
| Unbounded / runtime precision | `bigdecimal` |
| Binary fixed-point (DSP, embedded radio) | `fixed` |
| Sensor or physics-engine values with dynamic range | `f64` / `f128` |

## Picking a width

- **`D38<S>`** is the default. 38 digits handles every reasonable money-or-measurement scenario with comfortable headroom.
- **`D9<S>` / `D18<S>`** when you need compact storage and your values fit (e.g. cents in a single tax-line table - `D18<2>` covers ±9.2×10¹⁶).
- **`D57` … `D307`** for scientific work needing > 38 digits. The wide tier is opt-in via the matching Cargo feature; widening (`From`) is free, narrowing (`TryFrom`) is fallible. Half-width siblings (`D57`, `D115`, `D230`) let you size storage to your precision budget without paying for an unnecessary power-of-two jump.
- **`D462` / `D616`** (extra-wide, `x-wide`) for cryptographic or scientific work that needs > 307 digits but not the full xx-wide compile cost.
- **`D924` / `D1232`** (xx-wide) is research-grade — `D1232<1231>` transcendentals approach a second per call.

## Picking a scale

- The scale is the number of fractional decimal digits.
- Pick the smallest scale that covers your precision needs. Each extra digit halves the value range.
- Common picks: `SCALE = 2` (cents), `SCALE = 6` (µ / ppm), `SCALE = 12` (financial standard / picometres), `SCALE = 18` (atto).
- D38 ships the full `D38s0` … `D38s37` alias range. The other widths ship curated subsets. **No alias exists at `SCALE == name`** (no `D38s38`, no `D76s76`, no `D57s57`, …); those scales are not legal in 0.4 — use the `name − 1` ceiling (`D38s37`, `D76s75`, …) or `D<N>::<SCALE>` directly for any in-range scale.

## Construction (in order of ergonomics)

```rust
use decimal_scaled::{d38, D38s12};
use std::str::FromStr;

// 1. `d38!` macro - compile-time literal, scale inferred from the
//    written digits. Requires the `macros` Cargo feature.
let a = d38!(1.1, scale 12);          // D38<12>, exactly 1.1
let b = d38!(19.99);                  // D38<2>,  inferred from digits

// 2. FromStr - runtime parse. Works without `macros`.
//    Fixed in 0.4.0: deep-SCALE parsing works for every width
//    (was previously capped at SCALE <= 38 on wide tiers).
let c: D38s12 = "2.2".parse().unwrap();

// 3. from_bits - for hot paths or when you already have the raw integer.
let d = D38s12::from_bits(3_300_000_000_000);  // 3.3 exactly

// 4. from_int - from a primitive integer.
let e = D38s12::from_int(42);

// 5. from_f64 - LOSSY (round-half-to-even via crate default).
//    Use sparingly: defeats the determinism guarantee for downstream
//    arithmetic IF the f64 was itself produced by binary math.
let f = D38s12::from_f64(1.5);
```

## Per-width macros and per-scale wrappers

Every width has a matching macro: `d9!`, `d18!`, `d38!`, `d57!`, `d76!`, `d115!`, `d153!`, `d230!`, `d307!`, `d462!`, `d616!`, `d924!`, `d1232!`. Per-scale wrappers (e.g. `d38s12!`, `d18s6!`) skip the `, scale N` qualifier:

```rust
use decimal_scaled::{d38s12, d18s2};
let pico = d38s12!(1.234_567_890_123);   // D38<12>
let cents = d18s2!(19.99);                // D18<2>
```

Curated per-scale wrappers exist for the common scales; long-tail scales remain reachable via `dN!(value, scale N)`.

## Arithmetic

- `+`, `-`, `%`, unary `-` - **exact** (no rounding).
- `*`, `/` - **correctly rounded** (half-to-even by default).
- Operands must share the same `SCALE`. Cross-scale needs `value.rescale::<TARGET>()`.
- Overflow: debug-panic / release-wrap (matches Rust integer semantics). Use `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` for explicit handling.

```rust
let a = d38!(1.5, scale 12);
let b = d38!(2.0, scale 12);
let sum = a + b;                          // 3.5, exact
let prod = a * b;                         // 3.0, rounded (here, exact)
let safe = a.checked_mul(b).unwrap();     // explicit overflow check
```

## Rescaling between scales

```rust
use decimal_scaled::RoundingMode;
let micros = D38s6::from_bits(1_500_000);                  // 1.500000
let cents: D38s2 = micros.rescale::<2>();                  // 1.50, half-to-even
let cents = micros.rescale_with::<2>(RoundingMode::Trunc); // 1.50 (no half here)
let same = micros.with_scale::<6>();                       // alias for rescale
```

- Scale-up (target > source): **exact**, panics on storage overflow.
- Scale-down (target < source): rounds per the supplied mode (default `HalfToEven`).

## Strict vs Fast transcendentals - the dual API

Every transcendental method exists in **two named forms**, **both always compiled**:

| Method | Path | Determinism | Precision | Needs |
|---|---|---|---|---|
| `*_strict` (e.g. `ln_strict`, `sin_strict`) | integer-only, guard-digit | platform-deterministic, bit-identical everywhere | within **0.5 ULP** at storage scale | nothing extra |
| `*_fast` (e.g. `ln_fast`, `sin_fast`) | f64 bridge | platform-libm-dependent | ~16 decimal digits, libm-bounded | `feature = "std"` |
| plain `*` (e.g. `ln`, `sin`) | dispatcher → strict OR fast based on Cargo features | follows whichever | follows whichever | follows whichever |

**Default Cargo features include `strict`**, so plain `.ln()` resolves to `ln_strict`.

**Rules of thumb:**
- **Need cross-platform bit-determinism (consensus, audit, replay) → ALWAYS call `*_strict` explicitly.** Don't rely on the feature flag - a downstream crate could flip it.
- **Need maximum throughput and platform-libm precision is fine → call `*_fast` explicitly**, or enable the `fast` feature so plain `*` dispatches there.
- The `*_strict` family is also the one you want under `no_std`.

## Rounding modes via `*_with(mode)`

Every operation that rounds has a `*_with` sibling taking an explicit [`RoundingMode`]:

```rust
use decimal_scaled::RoundingMode;
let _ = value.rescale_with::<2>(RoundingMode::HalfAwayFromZero);
let _ = value.from_f64_with(1.5, RoundingMode::Ceiling);
let _ = value.to_int_with(RoundingMode::Trunc);
let _ = wide.ln_strict_with(RoundingMode::Floor);  // wide-tier transcendentals
```

Modes: `HalfToEven` (default; IEEE-754, no bias), `HalfAwayFromZero` (commercial), `HalfTowardZero`, `Trunc` (toward zero), `Floor` (toward −∞), `Ceiling` (toward +∞).

The `rounding-*` Cargo features change the **crate-wide default**:

```toml
decimal-scaled = { version = "0.4", features = ["rounding-half-away-from-zero"] }
```

Transcendental coverage at the policy layer includes `ln` / `log` / `log2` / `log10`, `exp` / `exp2`, the full trig family (`sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`atan2`), the full hyperbolic family (`sinh`/`cosh`/`tanh`/`asinh`/`acosh`/`atanh`), `sqrt` / `cbrt`, and `pow` / `powi`.

## Mathematical constants - `DecimalConsts`

```rust
use decimal_scaled::DecimalConsts;
let pi = D38s12::pi();      // 3.141592653590
let e  = D38s12::e();       // 2.718281828459
let _  = D76::<35>::pi();   // 75-digit reference rescaled to 35
```

Every constant is **0.5-ULP correctly-rounded** at every supported scale on every width — except where the value's magnitude exceeds the storage range, in which case the value simply doesn't exist and the method panics with a clear `out of storage range` message (e.g. `D38<37>::pi()` cannot fit ±3.14 because the integer headroom is < 1).

**f64-boundary caveat:** `D38<S>::pi().to_f64()` carries hundreds of ULPs of error vs `std::f64::consts::PI` for `S < 15`. If the f64 is the final consumer, source the constant from `std::f64::consts` directly rather than round-tripping through a narrow-scale decimal.

## Width-generic code via the `Decimal` trait

```rust
use decimal_scaled::Decimal;

fn average<D: Decimal>(values: &[D]) -> D {
    D::sum(values.iter().copied()) / D::from_i32(values.len() as i32)
}
```

`Decimal` is the umbrella trait. Since 0.3.3 it's split internally into two narrower traits whose union it re-exports:

- **`DecimalArithmetic`** — arithmetic + bitwise operators, sign methods (`abs`, `signum`, `is_positive`, `is_negative`), integer methods (`div_euclid`, `rem_euclid`, `div_floor`, `div_ceil`, `abs_diff`, `midpoint`, `mul_add`), `pow` / `powi` and the four `*_pow` overflow variants, the full `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` of `add` / `sub` / `mul` / `div` / `neg` / `rem`, default reductions (`is_zero`, `is_one`, `is_normal`, `sum`, `product`).
- **`DecimalConvert`** — `from_i32`, `to_int`, `to_int_with`, the f64 bridge (`from_f64`, `from_f64_with`, `to_f64`, `to_f32`; gated on `std`), and `Storage`-level conversions.

Most call sites still bound on `Decimal`. Use the split traits when a generic only needs one half.

**Out of scope for the trait** (use the concrete type):
- `rescale<TARGET>` - needs a const-generic method parameter,
- `from_int` - source-integer type varies per width (`i32` / `i64` / `i128` / `Int192` / …),
- transcendentals - feature-gated, on `DecimalTranscendental`.

For runtime polymorphism, enable the `dyn` feature: the object-safe `DynDecimal` trait + the `DecimalWidth` enum erase the storage type at the cost of a heap allocation per binary op.

## Serde

Behind `feature = "serde"` (on by default). Human-readable formats use a **decimal-string** wire form; binary formats use little-endian raw-storage bytes:

```rust
let v = D38s12::from_int(42);
let json: String = serde_json::to_string(&v).unwrap();
// "42000000000000" (the raw i128 storage as a decimal string)
let back: D38s12 = serde_json::from_str(&json).unwrap();
assert_eq!(back, v);
```

The string form is bit-faithful and round-trips exactly. Floats are rejected by the deserializer - keep everything in `D38` end-to-end.

## Common mistakes - avoid these

| Anti-pattern | Why bad | Fix |
|---|---|---|
| Storing prices in `f64`, then converting to `D38` at output | `f64` already lost decimal precision | Stay in `D38` from input parsing through display |
| `D38s12::from_int(1) + D38s6::from_int(1)` | Cross-scale arithmetic doesn't compile | `.rescale::<6>()` or `.rescale::<12>()` first |
| Calling `.ln()` then expecting identical bits on Linux and macOS | With `strict` default it IS deterministic, but a downstream crate could flip `fast` | Call `.ln_strict()` **explicitly** when determinism is required |
| `D38<37>::pi()` (or any constant that exceeds the type's value range) | The value simply doesn't fit; method panics | Widen the storage (`D76<37>::pi()` etc.) |
| `D38s38` (or any `DNNsNN` at the max-scale ceiling) | Removed in 0.4 — illegal `SCALE` | Use `D38s37` (`name − 1`) or `D<N>::<SCALE>` directly |
| `dN!` literal in a `no_std` build without `macros` feature | Compile error | Enable the `macros` feature or fall back to `FromStr` / `from_bits` |
| Serialising a `D38` via `serde_json::to_string(&v.to_f64())` | Lossy round-trip through `f64` | Serialise the `D38` directly - the impl emits a decimal string |
| Calling plain `.sin()` under `feature = "fast"` and expecting 0.5 ULP | `fast` flips the dispatcher to f64 bridge | Either: don't enable `fast`, or call `.sin_strict()` explicitly |

## Cargo features cheat sheet

| Feature | Default | What it does |
|---|---|---|
| `std` | ✓ | Enables the f64 bridge (`*_fast` methods); also pulls in `alloc` |
| `alloc` | ✓ | String formatting / parsing |
| `serde` | ✓ | `Serialize` / `Deserialize` on every width |
| `strict` | ✓ | Plain `*` dispatches to integer-only `*_strict`; `no_std`-compatible |
| `macros` | ✗ | Enables `d9!` … `d1232!` proc-macros + per-scale wrappers |
| `fast` | ✗ | Forces plain `*` to dispatch to f64-bridge `*_fast` (overridden by `strict` when both are set) |
| `dyn` | ✗ | Object-safe `DynDecimal` trait + `DecimalWidth` enum (heap boxing per op) |
| `wide` | ✗ | Enables D57 / D76 / D115 / D153 / D230 / D307 (individual `d57` … `d307` flags also exist) |
| `x-wide` | ✗ | Adds D462 / D616 (`d462`, `d616`) |
| `xx-wide` | ✗ | Adds D924 / D1232 (`d924`, `d1232`) |
| `rounding-*` | ✗ | Flips crate-default `RoundingMode` at compile time |
| `experimental-floats` | ✗ | Nightly-only `f16` / `f128` bridges |
| `bench-alt` | ✗ | Side-by-side default-vs-override builds for benchmarking |
| `perf-trace` | ✗ | `tracing::info_span!` boundaries inside wide-tier strict cores |

## Quick recipes

```rust
// 1. Parse a price, multiply by quantity, render.
use decimal_scaled::{D38s2};
use std::str::FromStr;
let price: D38s2 = "19.99".parse()?;
let total = price * D38s2::from_int(3);
println!("{}", total);  // "59.97"

// 2. Pi rendered to 12 fractional digits, deterministic everywhere.
use decimal_scaled::{D38s12, DecimalConsts};
let pi = D38s12::pi();
println!("{pi}");  // "3.141592653590"

// 3. Width-generic sum.
use decimal_scaled::Decimal;
fn checksum<D: Decimal>(xs: &[D]) -> D { D::sum(xs.iter().copied()) }

// 4. Mode-aware rounding.
use decimal_scaled::{D38s6, RoundingMode};
let v = D38s6::from_bits(1_235_000);                       // 1.235000
let cents = v.rescale_with::<2>(RoundingMode::Ceiling);    // 1.24
```

## Project layout (for contributors)

The `src/` tree is organised into seven directories plus `lib.rs`:

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
│                 family. Typed shells call ONLY into this layer.
├── wide_int/     The in-tree hand-rolled wide-integer backends
│                 (Int192 … Int4096) plus their macros.
├── macros/       Internal `macro_rules!` for emitting the typed surface
│                 per width (arithmetic, conversions, transcendentals,
│                 etc.).
└── support/      Crate-wide infrastructure: rounding, error, diagnostics,
                  display, serde_helpers, bench_alt.
```

**Layering rule (enforced by code-review, not by the type system):** typed shells in `src/types/*.rs` route through `policy/<family>` traits. They never reach directly into `algos/` or kernel code. New algorithms land in `algos/`, get wrapped by a policy method, and the typed shell delegates to that.

## Reference

- Crate: https://crates.io/crates/decimal-scaled
- Docs: https://docs.rs/decimal-scaled
- Repo: https://github.com/mootable/decimal-scaled
- In-repo guides: `docs/guides.md`, `docs/widths.md`, `docs/strict-mode.md`, `docs/rounding.md`, `docs/macros.md`, `docs/features.md`, `docs/benchmarks.md`
- Algorithm catalogue: `ALGORITHMS.md`
- Headline benchmark vs other fixed-point crates: `benches/g_math_comparison.rs` (run with `cargo bench --bench g_math_comparison --features wide`)
