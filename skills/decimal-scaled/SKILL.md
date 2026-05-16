---
name: decimal-scaled
description: Invoke when working with the `decimal-scaled` crate (const-generic base-10 fixed-point decimals D9/D18/D38/D76/D153/D307 with 0.5-ULP correctly-rounded integer-only transcendentals). Covers when to pick the crate over alternatives, picking the right width and SCALE, construction (`d38!` / FromStr / `from_bits` / `from_int`), strict-vs-fast routing, rounding modes via `*_with(mode)`, the `Decimal` trait for width-generic code, the `DecimalConsts` trait for pi/tau/e/etc, and common anti-patterns. Apply when the user writes Rust code involving currency, prices, measurements, scientific values that must round-trip exactly through human-readable decimals, or anywhere they need deterministic bit-identical arithmetic across platforms.
---

# `decimal-scaled` — agent usage guide

This skill teaches you how to use the `decimal-scaled` crate effectively. Apply it whenever the user's Rust code touches the crate, or whenever you're considering recommending it.

## What `decimal-scaled` IS

A family of const-generic base-10 fixed-point decimal types:

| type | storage | `MAX_SCALE` (= digits) | feature |
|------|---------|------------------------|---------|
| `D9<S>`   | `i32`             | 9   | always available |
| `D18<S>`  | `i64`             | 18  | always available |
| `D38<S>`  | `i128`            | 38  | always available |
| `D76<S>`  | in-tree `Int256`  | 76  | `d76` / `wide` |
| `D153<S>` | in-tree `Int512`  | 153 | `d153` / `wide` |
| `D307<S>` | in-tree `Int1024` | 307 | `d307` / `x-wide` |

The const generic `SCALE` is the number of fractional digits, baked into the type. The number in the type name (`9`, `18`, `38`, …) is `MAX_SCALE` — the largest scale that fits the storage at every value.

Stored as `value × 10^SCALE`. Decimals like `1.1` round-trip exactly. `0.1 + 0.2 == 0.3` holds.

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
- **`D9<S>` / `D18<S>`** when you need compact storage and your values fit (e.g. cents in a single tax-line table — `D18<2>` covers ±9.2×10¹⁶).
- **`D76` and above** for scientific work needing > 38 digits. Wide tiers are opt-in via the matching Cargo feature; widening (`From`) is free, narrowing (`TryFrom`) is fallible.

## Picking a scale

- The scale is the number of fractional decimal digits.
- Pick the smallest scale that covers your precision needs. Each extra digit halves the value range.
- Common picks: `SCALE = 2` (cents), `SCALE = 6` (µ / ppm), `SCALE = 12` (financial standard / picometres), `SCALE = 18` (atto).
- Per-scale aliases ship for D38 (e.g. `D38s12`) and curated subsets for the other widths.

## Construction (in order of ergonomics)

```rust
use decimal_scaled::{d38, D38s12};
use std::str::FromStr;

// 1. `d38!` macro — compile-time literal, scale inferred from the
//    written digits. Requires the `macros` Cargo feature.
let a = d38!(1.1, scale 12);          // D38<12>, exactly 1.1
let b = d38!(19.99);                  // D38<2>,  inferred from digits

// 2. FromStr — runtime parse. Works without `macros`.
let c: D38s12 = "2.2".parse().unwrap();

// 3. from_bits — for hot paths or when you already have the raw integer.
let d = D38s12::from_bits(3_300_000_000_000);  // 3.3 exactly

// 4. from_int — from a primitive integer.
let e = D38s12::from_int(42);

// 5. from_f64 — LOSSY (round-half-to-even via crate default).
//    Use sparingly: defeats the determinism guarantee for downstream
//    arithmetic IF the f64 was itself produced by binary math.
let f = D38s12::from_f64(1.5);
```

## Per-width macros and per-scale wrappers

Every width has a matching macro: `d9!`, `d18!`, `d38!`, `d76!`, `d153!`, `d307!`. Per-scale wrappers (e.g. `d38s12!`, `d18s6!`) skip the `, scale N` qualifier:

```rust
use decimal_scaled::{d38s12, d18s2};
let pico = d38s12!(1.234_567_890_123);   // D38<12>
let cents = d18s2!(19.99);                // D18<2>
```

Curated per-scale wrappers exist for the common scales; long-tail scales remain reachable via `dN!(value, scale N)`.

## Arithmetic

- `+`, `-`, `%`, unary `-` — **exact** (no rounding).
- `*`, `/` — **correctly rounded** (half-to-even by default).
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

## Strict vs Fast transcendentals — the dual API

Every transcendental method exists in **two named forms**, **both always compiled**:

| Method | Path | Determinism | Precision | Needs |
|---|---|---|---|---|
| `*_strict` (e.g. `ln_strict`, `sin_strict`) | integer-only, guard-digit | platform-deterministic, bit-identical everywhere | within **0.5 ULP** at storage scale | nothing extra |
| `*_fast` (e.g. `ln_fast`, `sin_fast`) | f64 bridge | platform-libm-dependent | ~16 decimal digits, libm-bounded | `feature = "std"` |
| plain `*` (e.g. `ln`, `sin`) | dispatcher → strict OR fast based on Cargo features | follows whichever | follows whichever | follows whichever |

**Default Cargo features include `strict`**, so plain `.ln()` resolves to `ln_strict`.

**Rules of thumb:**
- **Need cross-platform bit-determinism (consensus, audit, replay) → ALWAYS call `*_strict` explicitly.** Don't rely on the feature flag — a downstream crate could flip it.
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
decimal-scaled = { version = "0.2", features = ["rounding-half-away-from-zero"] }
```

## Mathematical constants — `DecimalConsts`

```rust
use decimal_scaled::DecimalConsts;
let pi = D38s12::pi();      // 3.141592653590
let e  = D38s12::e();       // 2.718281828459
let _  = D76::<35>::pi();   // 75-digit reference rescaled to 35
```

Every constant is **0.5-ULP correctly-rounded** at every supported scale on every width — except where the value's magnitude exceeds the type's storage range, in which case the method panics with a clear `out of storage range` message (e.g. `D38<38>::pi()` panics because π ≈ 3.14 doesn't fit ±1.70141).

## Width-generic code via the `Decimal` trait

```rust
use decimal_scaled::Decimal;

fn average<D: Decimal>(values: &[D]) -> D {
    D::sum(values.iter().copied()) / D::from_i32(values.len() as i32)
}
```

The `Decimal` trait carries the uniform surface every width implements:
- arithmetic + bitwise operators (supertrait bounds),
- sign methods (`abs`, `signum`, `is_positive`, `is_negative`),
- integer methods (`div_euclid`, `rem_euclid`, `div_floor`, `div_ceil`, `abs_diff`, `midpoint`, `mul_add`),
- `pow` + `powi` + the four `*_pow` overflow variants,
- the full `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` of `add` / `sub` / `mul` / `div` / `neg` / `rem`,
- integer conversion (`from_i32`, `to_int`, `to_int_with`),
- float bridge (`from_f64`, `from_f64_with`, `to_f64`, `to_f32`; gated on `std`),
- default reductions (`is_zero`, `is_one`, `is_normal`, `sum`, `product`).

**Out of scope for the trait** (use the concrete type):
- `rescale<TARGET>` — needs a const-generic method parameter,
- `from_int` — source-integer type varies per width (`i32` / `i64` / `i128`),
- transcendentals — feature-gated.

## Serde

Behind `feature = "serde"` (on by default). Human-readable formats use a **decimal-string** wire form; binary formats use little-endian raw-storage bytes:

```rust
let v = D38s12::from_int(42);
let json: String = serde_json::to_string(&v).unwrap();
// "1500000000000" (the raw i128 storage as a decimal string)
let back: D38s12 = serde_json::from_str(&json).unwrap();
assert_eq!(back, v);
```

The string form is bit-faithful and round-trips exactly. Floats are rejected by the deserializer — keep everything in `D38` end-to-end.

## Common mistakes — avoid these

| Anti-pattern | Why bad | Fix |
|---|---|---|
| Storing prices in `f64`, then converting to `D38` at output | `f64` already lost decimal precision | Stay in `D38` from input parsing through display |
| `D38s12::from_int(1) + D38s6::from_int(1)` | Cross-scale arithmetic doesn't compile | `.rescale::<6>()` or `.rescale::<12>()` first |
| Calling `.ln()` then expecting identical bits on Linux and macOS | With `strict` default it IS deterministic, but a downstream crate could flip `fast` | Call `.ln_strict()` **explicitly** when determinism is required |
| `D38::<38>::pi()` | Storage range is ~±1.7, π doesn't fit; panics | Use `D76::<38>::pi()` (or any wider tier) |
| `dN!` literal in a `no_std` build without `macros` feature | Compile error | Enable the `macros` feature or fall back to `FromStr` / `from_bits` |
| Serialising a `D38` via `serde_json::to_string(&v.to_f64())` | Lossy round-trip through `f64` | Serialise the `D38` directly — the impl emits a decimal string |
| Calling plain `.sin()` under `feature = "fast"` and expecting 0.5 ULP | `fast` flips the dispatcher to f64 bridge | Either: don't enable `fast`, or call `.sin_strict()` explicitly |

## Cargo features cheat sheet

| Feature | Default | What it does |
|---|---|---|
| `std` | ✓ | Enables the f64 bridge (`*_fast` methods); also pulls in `alloc` |
| `alloc` | ✓ | String formatting / parsing |
| `serde` | ✓ | `Serialize` / `Deserialize` on every width |
| `strict` | ✓ | Plain `*` dispatches to integer-only `*_strict`; `no_std`-compatible |
| `macros` | ✗ | Enables `d9!` … `d307!` proc-macros + per-scale wrappers |
| `fast` | ✗ | Forces plain `*` to dispatch to f64-bridge `*_fast` (overrides `strict`) |
| `wide` | ✗ | Enables `D76` + `D153` |
| `x-wide` | ✗ | Enables `D307` |
| `rounding-*` | ✗ | Flips crate-default `RoundingMode` at compile time |
| `experimental-floats` | ✗ | Nightly-only `f16` / `f128` bridges |

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

## Reference

- Crate: https://crates.io/crates/decimal-scaled
- Docs: https://docs.rs/decimal-scaled
- Repo: https://github.com/mootable/decimal-scaled
- In-repo guides: `docs/guides.md`, `docs/widths.md`, `docs/strict-mode.md`, `docs/rounding.md`, `docs/macros.md`, `docs/features.md`, `docs/benchmarks.md`
- Algorithm catalogue: `ALGORITHMS.md`
- Headline benchmark vs other fixed-point crates: `benches/g_math_comparison.rs` (run with `cargo bench --bench g_math_comparison --features wide`)
