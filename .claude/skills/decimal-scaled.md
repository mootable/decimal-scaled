
---
name: decimal-scaled
description: Invoke when working with the `decimal-scaled` crate (const-generic base-10 fixed-point decimals D18/D38/D57/D76/D115/D153/D230/D307/D462/D616/D924/D1232 with 0.5-ULP correctly-rounded integer-only transcendentals). Covers when to pick the crate over alternatives, picking the right width and SCALE, construction (`d38!` / FromStr / `from_bits` / `from_num`), strict-vs-fast routing, rounding modes via `*_with(mode)`, the `Decimal` trait for width-generic code, the `DecimalConstants` trait for pi/tau/e/etc, num-traits interop, the `dyn` runtime-polymorphic façade, and common anti-patterns. Apply when the user writes Rust code involving currency, prices, measurements, scientific values that must round-trip exactly through human-readable decimals, or anywhere they need deterministic bit-identical arithmetic across platforms.
---

# `decimal-scaled` - agent usage guide

This skill teaches you how to use the `decimal-scaled` crate effectively. Apply it whenever the user's Rust code touches the crate, or whenever you're considering recommending it.

## What `decimal-scaled` IS

A family of const-generic base-10 fixed-point decimal types. Each width is a `D<digits><const SCALE: u32>` newtype with the same method surface; pick the narrowest that fits your range:

| type | storage | `MAX_SCALE` | feature gate |
|------|---------|------------:|--------------|
| `D18<S>`   | `i64`     |   17 | always available |
| `D38<S>`   | `i128`    |   37 | always available |
| `D57<S>`   | 192-bit   |   56 | `d57` or `wide` |
| `D76<S>`   | 256-bit   |   75 | `d76` or `wide` |
| `D115<S>`  | 384-bit   |  114 | `d115` or `wide` |
| `D153<S>`  | 512-bit   |  152 | `d153` or `wide` |
| `D230<S>`  | 768-bit   |  229 | `d230` or `wide` |
| `D307<S>`  | 1024-bit  |  306 | `d307` or `wide` |
| `D462<S>`  | 1536-bit  |  461 | `d462` or `x-wide` |
| `D616<S>`  | 2048-bit  |  615 | `d616` or `x-wide` |
| `D924<S>`  | 3072-bit  |  923 | `d924` or `xx-wide` |
| `D1232<S>` | 4096-bit  | 1231 | `d1232` or `xx-wide` |

- The number in the type name (`18`, `38`, `57`, …) is the type's **nominal precision in decimal digits**.
- `SCALE` is the const-generic number of fractional digits, baked into the type. `D38<2>` (cents) and `D38<18>` are distinct, zero-overhead types.
- **`MAX_SCALE = digits − 1`** (so `D38`'s top scale is 37, `D18`'s is 17, `D1232`'s is 1231). The v0.4.0 scale cap leaves at least one integer digit of headroom at every legal scale. **`SCALE = digits` is rejected at COMPILE TIME** (e.g. `D38<38>` won't compile) — not a runtime error.
- Stored as `value × 10^SCALE`. Decimals like `1.1` round-trip exactly. `0.1 + 0.2 == 0.3` holds.
- The wide tiers (D57+) are backed by the in-tree const-generic `Int<N>` integers (`[u64; N]` limbs) — **no external big-integer dependency**. D18/D38 use primitive `i64`/`i128`.

Every adjacent pair has lossless `.widen()` / fallible `.narrow()` helpers plus `From` / `TryFrom` impls.

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

- **`D38<S>`** is the default. 38 digits handles every reasonable money-or-measurement scenario with comfortable headroom, and it needs no extra feature.
- **`D18<S>`** when you need compact 64-bit storage and your values fit (e.g. cents in a single tax-line table — `D18<2>` covers ±9.2×10¹⁶).
- **`D57` and above** for scientific work needing > 38 digits. Wide tiers are opt-in via the matching Cargo feature (umbrellas: `wide` → D57…D307, `x-wide` → D462+D616, `xx-wide` → D924+D1232; or a single tier like `d307`). Widening (`From` / `.widen()`) is free; narrowing (`TryFrom` / `.narrow()`) is fallible.

## Picking a scale

- The scale is the number of fractional decimal digits.
- Pick the smallest scale that covers your precision needs. Each extra digit eats one digit of the value range.
- Common picks: `SCALE = 2` (cents), `SCALE = 6` (µ / ppm), `SCALE = 12` (financial standard / picometres), `SCALE = 18` (atto).
- Per-scale aliases ship for D18 and D38 at **every** scale (e.g. `D38s12`, `D18s2`), and a **curated subset** of scales for the wide tiers (e.g. `D76s35`, `D307s150`).

## Construction (in order of ergonomics)

```rust
use decimal_scaled::{d38, D38s12};
use std::str::FromStr;

// 1. `d38!` macro - compile-time literal, scale inferred from the
//    written digits. Requires the `macros` Cargo feature.
let a = d38!(1.1, scale 12);          // D38<12>, exactly 1.1
let b = d38!(19.99);                  // D38<2>,  inferred from digits

// 2. FromStr - runtime parse. Works without `macros`.
let c: D38s12 = "2.2".parse().unwrap();

// 3. from_bits - for hot paths or when you already have the raw integer.
//    Takes the type's raw storage (i128 for D38, an `Int<N>` for wide tiers).
let d = D38s12::from_bits(3_300_000_000_000);  // 3.3 exactly

// 4. from_num - from any primitive number (num-traits `ToPrimitive`).
//    Replaces the old `from_int`. Integer values are exact.
let e = D38s12::from_num(42);

// 5. from_f64 - LOSSY (round-half-to-even via crate default), `std` only.
//    Use sparingly: defeats determinism for downstream arithmetic IF the
//    f64 was itself produced by binary math. `from_f64_with(v, mode)` for
//    an explicit RoundingMode.
let f = D38s12::from_f64(1.5);
```

(`from_i32` / `to_int` / `to_int_with` also exist via the [`DecimalConvert`] / [`Decimal`] traits; `to_int` returns an `i64`. For other primitive targets use `to_num::<T>()`.)

## Per-width macros and per-scale wrappers

Every width has a matching macro: `d18!`, `d38!`, `d57!`, `d76!`, `d115!`, `d153!`, `d230!`, `d307!`, `d462!`, `d616!`, `d924!`, `d1232!` (the wide-tier macros are feature-gated to match their type). Per-scale wrappers (e.g. `d38s12!`, `d18s2!`, `d76s35!`) skip the `, scale N` qualifier:

```rust
use decimal_scaled::{d38s12, d18s2};
let pico = d38s12!(1.234_567_890_123);   // D38<12>
let cents = d18s2!(19.99);                // D18<2>
```

Curated per-scale wrappers exist for the common scales; long-tail scales remain reachable via `dN!(value, scale N)`.

## Arithmetic

- `+`, `-`, `%`, unary `-` - **exact** (no rounding).
- `*`, `/` - **correctly rounded** (half-to-even by default).
- Operands must share the same `SCALE` (and width). Cross-scale needs `value.rescale::<TARGET>()`, or the explicit-target `mul_of` (see below).
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

## Changing width

```rust
let small: D38s12 = "1.5".parse().unwrap();
let big = small.widen();                  // D57<12> (next tier up) — lossless
let back = big.narrow().unwrap();         // D38<12> — fallible (Result<_, ConvertError>)
let any  = small.widen_n::<6>();           // const-generic target: D<Int<6>, 12>
```

`From` / `TryFrom` impls back these. Widening always succeeds; narrowing returns `Result<_, ConvertError>` (or `Option` on the raw-integer layer).

## Cross-scale operations (mixing scales / widths)

The stable entry point is the explicit-target `mul_of` / `mul_of_with` (you name the result type):

```rust
// a: D38<6>, b: D38<4>  →  product at an explicit target scale, correctly rounded.
let p = D38s12::mul_of(a, b);                       // half-to-even
let p = D38s12::mul_of_with(a, b, RoundingMode::Floor);
```

With the **nightly** `cross-scale-ops` feature, the `decimal_scaled::cross` module adds auto-inferred free functions — `cross::mul(a, b)` / `cross::add(a, b)` / … — returning a decimal at `max(width)`, `max(SCALE)`.

## Strict vs Fast transcendentals - the dual API

Every transcendental method exists in **two named forms**, **both always compiled**:

| Method | Path | Determinism | Precision | Needs |
|---|---|---|---|---|
| `*_strict` (e.g. `ln_strict`, `sin_strict`) | integer-only, guard-digit | platform-deterministic, bit-identical everywhere | within **0.5 ULP** at storage scale | nothing extra |
| `*_fast` (e.g. `ln_fast`, `sin_fast`) | f64 bridge | platform-libm-dependent | ~16 decimal digits, libm-bounded | `feature = "std"` |
| plain `*` (e.g. `ln`, `sin`) | dispatcher → strict OR fast based on Cargo features | follows whichever | follows whichever | follows whichever |

**Default Cargo features include `strict`**, so plain `.ln()` resolves to `ln_strict`. (If both `strict` and `fast` are present, **strict wins**.)

The family covers `sqrt` / `cbrt` / `exp` / `exp2` / `ln` / `log` / `log2` / `log10` / `pow` / `powf` / `powi` / `hypot`, the trig set (`sin`/`cos`/`tan`/`asin`/`acos`/`atan`/`atan2`), the hyperbolics (`sinh`/`cosh`/`tanh`/`asinh`/`acosh`/`atanh`), and `to_radians` / `to_degrees`.

**Rules of thumb:**
- **Need cross-platform bit-determinism (consensus, audit, replay) → ALWAYS call `*_strict` explicitly.** Don't rely on the feature flag - a downstream crate could flip it.
- **Need maximum throughput and platform-libm precision is fine → call `*_fast` explicitly**, or enable the `fast` feature so plain `*` dispatches there.
- The `*_strict` family is also the one you want under `no_std`.

## Rounding modes via `*_with(mode)`

Every operation that rounds has a `*_with` sibling taking an explicit [`RoundingMode`]:

```rust
use decimal_scaled::RoundingMode;
let _ = value.rescale_with::<2>(RoundingMode::HalfAwayFromZero);
let _ = D38s12::from_f64_with(1.5, RoundingMode::Ceiling);
let _ = value.to_int_with(RoundingMode::Trunc);
let _ = wide.ln_strict_with(RoundingMode::Floor);  // wide-tier transcendentals
```

Modes: `HalfToEven` (default; IEEE-754, no bias), `HalfAwayFromZero` (commercial), `HalfTowardZero`, `Trunc` (toward zero), `Floor` (toward −∞), `Ceiling` (toward +∞).

The `rounding-*` Cargo features change the **crate-wide default** (`rounding-half-away-from-zero`, `rounding-half-toward-zero`, `rounding-trunc`, `rounding-floor`, `rounding-ceiling`):

```toml
decimal-scaled = { version = "0.5", features = ["rounding-half-away-from-zero"] }
```

## Mathematical constants - `DecimalConstants`

The trait is `DecimalConstants` (renamed from the old `DecimalConsts` — that name no longer exists).

```rust
use decimal_scaled::DecimalConstants;
let pi = D38s12::pi();        // 3.141592653590
let e  = D38s12::e();         // 2.718281828459
let _  = D76::<35>::pi();     // wide-tier reference, correctly rounded at scale 35
let _  = D38s12::pi_with(RoundingMode::Floor);   // mode-aware sibling
```

Members: `pi`, `tau`, `half_pi`, `quarter_pi`, `golden`, `e`, `deg_per_rad`, `rad_per_deg` — each with a `*_with(mode)` sibling.

Every constant is **0.5-ULP correctly-rounded** at every supported scale on every width — **except where the value doesn't fit the type's storage at that scale**, in which case the method panics with a clear `out of storage range` message. The integer part must fit in the headroom left after `SCALE` fractional digits: e.g. at the maximum scale `D38<37>` represents roughly ±17, so `D38<37>::pi()` (≈3.14) is fine but `D38<37>::deg_per_rad()` (≈57.3) panics — use a lower scale or a wider tier.

> **f64-boundary tip:** if you'll immediately convert a constant to `f64`, source it from `std::f64::consts` directly when `SCALE < 15` — below ~15 fractional digits the decimal constant is coarser than the nearest `f64` and `D38<12>::pi().to_f64()` is hundreds of ULPs off `std::f64::consts::PI`.

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

The cross-width surface is split across four traits, all re-exported at the crate root: [`DecimalArithmetic`], [`DecimalConvert`], [`DecimalTranscendental`], and [`Decimal`] (the marker supertrait combining them).

**Out of scope for the trait** (use the concrete type):
- `rescale<TARGET>` / `widen_n<M>` - need a const-generic method parameter,
- `from_num` / `from_bits` - inherent constructors,
- transcendentals - feature-gated and on `DecimalTranscendental`.

The concrete `D{N}<S>` types are aliases over the underlying unified type `D<Int<N>, SCALE>` (also re-exported as `D`), so advanced const-generic code can name storage directly.

## `num-traits` interop

Every width implements the standard `num-traits` 0.2 surface **unconditionally** (not feature-gated): `Zero`, `One`, `Num`, `Bounded`, `Signed`, `FromPrimitive`, `ToPrimitive`, and the `Checked{Add,Sub,Mul,Div,Rem,Neg}` family. Plus the convenience bridges `from_num::<T: ToPrimitive>(v)` and `to_num::<T: NumCast + Bounded>()`. Generic numeric code in the wider ecosystem consumes this by default.

## Runtime polymorphism - the `dyn` façade

Behind `feature = "dyn"`: the object-safe `DynDecimal` trait + `DecimalWidth` enum + `RawStorage`, with a blanket impl bridging every concrete `Dxx<S>`. Use it when the width/scale isn't known at compile time. The cost of erasure is a **heap allocation per binary op** (the façade's `Box`/`String` returns are the crate's only sanctioned heap path), so it pulls in `alloc`. The typed `Decimal` core stays heap-free — typed users pay nothing.

## Serde

Behind `feature = "serde"` (on by default). Human-readable formats use a **decimal-string** wire form; binary formats use little-endian raw-storage bytes. The string form is bit-faithful and round-trips exactly; floats are rejected by the deserializer — keep everything in the decimal type end-to-end.

```rust
let v: D38s12 = "1.5".parse().unwrap();
let json = serde_json::to_string(&v).unwrap();
let back: D38s12 = serde_json::from_str(&json).unwrap();
assert_eq!(back, v);   // exact round-trip
```

## Common mistakes - avoid these

| Anti-pattern | Why bad | Fix |
|---|---|---|
| Storing prices in `f64`, then converting to a decimal at output | `f64` already lost decimal precision | Stay in the decimal type from input parsing through display |
| `D38s12::from_int(1)` | `from_int` was removed | Use `from_num(1)`, `from_i32(1)` (trait), or `d38s12!(1)` |
| `use decimal_scaled::DecimalConsts;` | renamed | `use decimal_scaled::DecimalConstants;` |
| `D38<6>::from_num(1) + D38<12>::from_num(1)` | Cross-scale arithmetic doesn't compile | `.rescale::<…>()` to a common scale first, or use `mul_of` |
| Calling `.ln()` then expecting identical bits on Linux and macOS | With `strict` default it IS deterministic, but a downstream crate could flip `fast` | Call `.ln_strict()` **explicitly** when determinism is required |
| `D38<38>` | `SCALE = digits` exceeds `MAX_SCALE = digits − 1` | Use `D38<37>` (max), or a wider tier (`D57<38>`) |
| `D38<37>::deg_per_rad()` | ≈57.3 doesn't fit the ~±17 range at scale 37; panics | Lower the scale, or use a wider tier (`D57<37>::deg_per_rad()`) |
| `dN!` literal in a build without the `macros` feature | Compile error | Enable `macros`, or fall back to `FromStr` / `from_bits` |
| Serialising via `serde_json::to_string(&v.to_f64())` | Lossy round-trip through `f64` | Serialise the decimal directly - the impl emits a decimal string |
| Calling plain `.sin()` under `feature = "fast"` and expecting 0.5 ULP | `fast` flips the dispatcher to f64 bridge | Don't enable `fast`, or call `.sin_strict()` explicitly |

## Cargo features cheat sheet

| Feature | Default | What it does |
|---|---|---|
| `std` | ✓ | Enables the f64 bridge (`*_fast` methods); also pulls in `alloc` |
| `alloc` | ✓ | String formatting / parsing |
| `serde` | ✓ | `Serialize` / `Deserialize` on every width |
| `strict` | ✓ | Plain `*` dispatches to integer-only `*_strict`; `no_std`-compatible; wins over `fast` |
| `exact-scratch` | ✓ | Per-`Int<N>` exactly-sized root-kernel work scratch (smaller stack frames; stable) |
| `macros` | ✗ | Enables `d18!` … `d1232!` proc-macros + per-scale wrappers |
| `fast` | ✗ | Forces plain `*` to dispatch to the f64-bridge `*_fast` (implies `std`; only if `strict` is off) |
| `dyn` | ✗ | Runtime-polymorphic `DynDecimal` façade (`+ alloc`; boxing per op) |
| `wide` | ✗ | Enables D57 / D76 / D115 / D153 / D230 / D307 |
| `x-wide` | ✗ | Enables D462 + D616 |
| `xx-wide` | ✗ | Enables D924 + D1232 |
| `d57` … `d1232` | ✗ | Enable a single wide tier individually |
| `rounding-*` | ✗ | Flips the crate-default `RoundingMode` at compile time |
| `cross-scale-ops` | ✗ | Nightly: the `cross::*` auto-inferred cross-scale free functions |
| `exact-scratch-nightly` | ✗ | Nightly: per-`N` exact scratch via one blanket `generic_const_exprs` impl |
| `experimental-floats` | ✗ | Nightly-only `f16` / `f128` bridges |

(Internal/dev-only flags — not for downstream use: `bench-alt`, `golden`, `perf-trace`, `_wide-support`.)

## Quick recipes

```rust
// 1. Parse a price, multiply by quantity, render.
use decimal_scaled::D38s2;
let price: D38s2 = "19.99".parse()?;
let total = price * D38s2::from_num(3);
println!("{}", total);  // "59.97"

// 2. Pi rendered to 12 fractional digits, deterministic everywhere.
use decimal_scaled::{D38s12, DecimalConstants};
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
- Homepage: https://mootable.github.io/decimal-scaled/
- In-repo guides: `docs/getting-started.md`, `docs/guides.md`, `docs/widths.md`, `docs/strict-mode.md`, `docs/rounding.md`, `docs/macros.md`, `docs/features.md`, `docs/conversions.md`, `docs/cross-scale.md`, `docs/benchmarks.md`
- Architecture: `docs/ARCHITECTURE.md`
- Algorithm catalogue: `ALGORITHMS.md`
- Headline benchmark vs other fixed-point crates: `benches/backends/g_math_comparison.rs` (run with `cargo bench --bench g_math_comparison --features wide`)
