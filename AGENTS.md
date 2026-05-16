# AGENTS.md — `decimal-scaled` usage guide

Project-wide guidance for AI coding assistants (Claude Code, Cursor, Continue, Codeium, Aider, …) working with the `decimal-scaled` crate. Read this before suggesting code that uses the crate or recommending it as a dependency.

A Claude-Code-flavoured copy of the same guidance, with `name` / `description` frontmatter for skill discovery, lives at [`.claude/skills/decimal-scaled.md`](.claude/skills/decimal-scaled.md). The two files are kept in sync.

## TL;DR

```toml
[dependencies]
decimal-scaled = { version = "0.2", features = ["macros"] }
```

```rust
use decimal_scaled::{d38, D38s12, DecimalConsts};
use std::str::FromStr;

let price: D38s12 = "19.99".parse().unwrap();
let total = price * D38s12::from_int(3);                  // 59.97, exact
let pi = D38s12::pi();                                    // correctly rounded
let bytes = total.to_bits();                              // raw i128 storage
```

Decimals like `1.1` round-trip exactly. `0.1 + 0.2 == 0.3` holds. Transcendentals (under the default `strict` feature) are correctly rounded to **0.5 ULP** and **bit-identical on every platform**.

## The width family

| type      | storage           | `MAX_SCALE` | feature                |
|-----------|-------------------|------------:|------------------------|
| `D9<S>`   | `i32`             | 9           | always available       |
| `D18<S>`  | `i64`             | 18          | always available       |
| `D38<S>`  | `i128`            | 38          | always available       |
| `D76<S>`  | in-tree `Int256`  | 76          | `d76`  / `wide`        |
| `D153<S>` | in-tree `Int512`  | 153         | `d153` / `wide`        |
| `D307<S>` | in-tree `Int1024` | 307         | `d307` / `x-wide`      |

The number in the type name (`9`, `18`, `38`, …) is the type's `MAX_SCALE` — the largest scale at which every value of that digit count fits storage. Stored value = `logical_value × 10^SCALE`.

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

The scale is the number of fractional decimal digits, fixed at compile time. Pick the smallest scale that covers your precision needs — each extra digit halves the integer-part range.

Common picks: `2` (cents / percent), `6` (µ / ppm), `12` (financial, picometres), `18` (atto), `35` (deep scientific).

Per-scale aliases ship for D38 (`D38s0` … `D38s38`) and curated subsets for the other widths (e.g. `D18s12`, `D76s35`).

## Constructing values

```rust
use decimal_scaled::{d38, D38s12};
use std::str::FromStr;

// 1. The `dN!` macro — compile-time literal, scale inferred from the
//    written digits. Requires the `macros` Cargo feature.
let a = d38!(1.1, scale 12);          // D38<12>, exactly 1.1
let b = d38!(19.99);                  // D38<2>,  inferred

// 2. FromStr — runtime parse, works without `macros`.
let c: D38s12 = "2.2".parse().unwrap();

// 3. from_bits — for hot paths or when you already have the raw integer.
let d = D38s12::from_bits(3_300_000_000_000);  // 3.3 exactly

// 4. from_int — from a primitive integer.
let e = D38s12::from_int(42);

// 5. from_f64 — LOSSY. Avoid for values that originated as decimals;
//    parse the decimal string instead.
let f = D38s12::from_f64(1.5);
```

Each width has its matching macro (`d9!`, `d18!`, `d38!`, `d76!`, `d153!`, `d307!`) plus per-scale wrappers (`d38s12!`, `d18s2!`, …). Long-tail scales remain reachable via `dN!(value, scale N)`.

## Arithmetic

- `+`, `-`, `%`, unary `-` — **exact** (no rounding).
- `*`, `/` — **correctly rounded** (half-to-even by default).
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

## Strict vs Fast transcendentals — the dual API (read carefully)

Every transcendental method exists in **two named forms**, **both always compiled**:

| Method                       | Path             | Determinism                              | Precision                  | Needs            |
|------------------------------|------------------|------------------------------------------|----------------------------|------------------|
| `*_strict` (`ln_strict`, …)  | integer-only     | bit-identical on every platform          | within **0.5 ULP**         | nothing extra    |
| `*_fast` (`ln_fast`, …)      | f64 bridge       | platform-libm-dependent                  | ~16 decimal digits         | `feature = "std"`|
| plain `*` (`ln`, …)          | dispatcher       | follows the feature set                  | follows the feature set    | follows          |

Default Cargo features include `strict`, so plain `.ln()` resolves to `ln_strict`.

**Operational rules for agents:**

1. **If the user needs cross-platform bit-determinism** (consensus protocols, financial audit trails, deterministic replay) — call `*_strict` **explicitly**. Don't rely on the feature flag; a downstream crate could enable `fast` and silently flip the dispatcher.
2. **If the user wants max throughput and tolerates platform-libm precision** — call `*_fast` explicitly, or enable the `fast` Cargo feature so plain `*` dispatches there.
3. **For `no_std`** — use `*_strict` (it doesn't need `std`).

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
decimal-scaled = { version = "0.2", features = ["rounding-half-away-from-zero"] }
```

## Mathematical constants — `DecimalConsts`

```rust
use decimal_scaled::DecimalConsts;
let pi = D38s12::pi();      // 3.141592653590
let e  = D38s12::e();       // 2.718281828459
let _  = D76::<35>::pi();   // 75-digit reference → 35 digits, 0.5 ULP
```

Every constant is correctly rounded to **0.5 ULP** at every supported scale on every width, with one storage-limit case: at `D38<38>` the type's value range is approximately ±1.70141, so `pi` (3.14), `tau` (6.28), and `e` (2.72) overflow `i128` and the corresponding methods panic with a clear "constant out of storage range" message. `half_pi` (1.57), `quarter_pi` (0.79), and `golden` (1.62) fit and remain correctly rounded.

## Width-generic code via the `Decimal` trait

```rust
use decimal_scaled::Decimal;

fn average<D: Decimal>(values: &[D]) -> D {
    D::sum(values.iter().copied()) / D::from_i32(values.len() as i32)
}
```

The trait carries the uniform surface every width implements: arithmetic + bitwise operators (supertrait bounds); sign methods; integer methods (`div_euclid`, `rem_euclid`, `div_floor`, `div_ceil`, `abs_diff`, `midpoint`, `mul_add`); `pow` + `powi` + the four `*_pow` overflow variants; the full `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*` of `add` / `sub` / `mul` / `div` / `neg` / `rem`; integer conversion (`from_i32`, `to_int`, `to_int_with`); float bridge (gated on `std`); defaults for `is_zero` / `is_one` / `is_normal` / `sum` / `product`.

**Out of scope for the trait** (use the concrete type instead): `rescale<TARGET>` (needs a const-generic method parameter), `from_int` (source integer type varies per width), transcendentals (feature-gated).

## Serde

Behind `feature = "serde"` (on by default). Human-readable formats use a **decimal-string** wire form; binary formats use little-endian raw-storage bytes:

```rust
let v = D38s12::from_int(42);
let json: String = serde_json::to_string(&v).unwrap();
// "42000000000000" — the raw i128 as a decimal string
let back: D38s12 = serde_json::from_str(&json).unwrap();
assert_eq!(back, v);
```

The string form is bit-faithful and round-trips exactly. The deserializer rejects floats — keep everything in `D38` end-to-end.

## Common mistakes — surface these in PR review

| Anti-pattern | Why bad | Fix |
|---|---|---|
| Storing prices in `f64`, then converting to `D38` at output | `f64` already lost decimal precision | Stay in `D38` from input parsing through display |
| `D38s12::from_int(1) + D38s6::from_int(1)` | Cross-scale arithmetic doesn't compile | `.rescale::<6>()` or `.rescale::<12>()` first |
| `.ln()` on a value that *must* be bit-identical across platforms | Default is `strict`, but a downstream crate could enable `fast` and flip the dispatcher | Call `.ln_strict()` explicitly |
| `D38::<38>::pi()` | Storage range is ~±1.7, π doesn't fit; panics | Use `D76::<38>::pi()` (or wider) |
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
| `macros` | ✗ | `d9!` … `d307!` proc-macros + per-scale wrappers |
| `fast` | ✗ | Plain `*` dispatches to `*_fast` (overrides `strict`) |
| `wide` | ✗ | Enables `D76` + `D153` |
| `x-wide` | ✗ | Enables `D307` |
| `rounding-*` | ✗ | Flips crate-default `RoundingMode` at compile time |
| `experimental-floats` | ✗ | Nightly-only `f16` / `f128` bridges |

## Quick recipes

```rust
// 1. Parse a price, multiply by quantity, render.
use decimal_scaled::D38s2;
let price: D38s2 = "19.99".parse()?;
let total = price * D38s2::from_int(3);
println!("{total}");  // "59.97"

// 2. Pi rendered to 12 fractional digits — deterministic everywhere.
use decimal_scaled::{D38s12, DecimalConsts};
println!("{}", D38s12::pi());  // "3.141592653590"

// 3. Width-generic sum.
use decimal_scaled::Decimal;
fn checksum<D: Decimal>(xs: &[D]) -> D { D::sum(xs.iter().copied()) }

// 4. Mode-aware rounding.
use decimal_scaled::{D38s2, D38s6, RoundingMode};
let v = D38s6::from_bits(1_235_000);                       // 1.235000
let up: D38s2 = v.rescale_with::<2>(RoundingMode::Ceiling); // 1.24
```

## Reference

- Crate: <https://crates.io/crates/decimal-scaled>
- Docs: <https://docs.rs/decimal-scaled>
- Repo: <https://github.com/mootable/decimal-scaled>
- In-repo guides: [`docs/guides.md`](docs/guides.md), [`docs/widths.md`](docs/widths.md), [`docs/strict-mode.md`](docs/strict-mode.md), [`docs/rounding.md`](docs/rounding.md), [`docs/macros.md`](docs/macros.md), [`docs/features.md`](docs/features.md), [`docs/benchmarks.md`](docs/benchmarks.md)
- Algorithm catalogue: [`ALGORITHMS.md`](ALGORITHMS.md)
- Headline benchmark vs another fixed-point crate (`g_math`): [`benches/g_math_comparison.rs`](benches/g_math_comparison.rs) — run with `cargo bench --bench g_math_comparison --features wide`
