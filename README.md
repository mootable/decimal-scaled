# decimal-scaled

**[Docs](https://mootable.github.io/decimal-scaled/)** • **[Benchmarks](https://mootable.github.io/decimal-scaled/benchmarks/)** • **[Algorithms](https://mootable.github.io/decimal-scaled/ALGORITHMS/)** • **[Roadmap](https://mootable.github.io/decimal-scaled/ROADMAP/)** • **[API reference](https://docs.rs/decimal-scaled)**

A Rust library providing const-generic base-10 fixed-point decimal
types with **correctly-rounded (≤ 0.5 ULP) integer-only
transcendentals**, deterministic across every platform, and
`no_std`-friendly.

📚 In-depth guides — getting started, scale aliases, the width
family, conversions, rounding modes, strict mode, the `d38!`
macro, every Cargo feature, benchmarks — live in
[**`docs/guides.md`**](docs/guides.md). API reference on
[docs.rs](https://docs.rs/decimal-scaled/).

> **0.5 ULP** — the strongest accuracy guarantee a finite numeric
> type can give — is the headline feature. Every `ln` / `exp` /
> `sin` / `cos` / `sqrt` / `cbrt` / `powf` / `atan` / `atan2` /
> `sinh` / `cosh` / `tanh` / `asinh` / `acosh` / `atanh` /
> `to_degrees` / `to_radians` lands within half an [ULP][ULP] of
> the exact result and the bit pattern is identical on every
> machine. No baseline numeric crate offers this for
> transcendentals. The algorithms, citations and per-function
> implementation notes are catalogued in
> [`ALGORITHMS.md`](ALGORITHMS.md).
>
> [ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
decimal-scaled = "0.2.5"
```

The default build pulls in the correctly-rounded transcendentals
(the `strict` feature is on by default). To opt in to the faster
`f64`-bridge ("fast") path instead — ~700× quicker on series
functions, but only ≈ 16 decimal digits of platform-libm precision
and **not** platform-deterministic — disable default features and
pick what you actually want:

```toml
[dependencies]
decimal-scaled = { version = "0.2.5", default-features = false, features = ["std", "serde"] }
```

For `no_std` targets (`alloc` is still required):

```toml
[dependencies]
decimal-scaled = { version = "0.2.5", default-features = false, features = ["serde", "alloc", "strict"] }
```

---

## Quick start

```toml
[dependencies]
decimal-scaled = { version = "0.2.5", features = ["macros"] }
```

There are three idiomatic ways to construct a value. Use whichever
fits your call site.

```rust
use decimal_scaled::{d38, D38s12};

// 1) `d38!` macro — the ergonomic constructor. Write the literal
//    as you'd read it; scale is inferred from the fractional
//    digits, or pinned explicitly with `, scale N`. (Requires the
//    `macros` feature.) One macro per width: `d9!`, `d18!`,
//    `d38!`, plus `d76!` / `d153!` / `d307!` under the wide
//    features. Pre-baked per-scale wrappers (`d38s12!`, `d18s6!`,
//    …) skip the `, scale N` and read more tersely at the call
//    site. Full grammar including the `radix N` qualifier in
//    [`macros/README.md`](macros/README.md).
let a = d38!(1.1, scale 12);                        // D38<12> — exactly 1.1

// 2) `FromStr` — parse a decimal string. Works without the
//    `macros` feature and accepts user input directly.
let b: D38s12 = "2.2".parse().unwrap();             // D38<12> — exactly 2.2

// 3) `from_bits` — for hot paths or when you already have the
//    raw integer (value × 10^SCALE). No parsing, no allocation.
let c = D38s12::from_bits(3_300_000_000_000);       // D38<12> — exactly 3.3

// Aliases like `D38s12` are just type aliases over `D38<12>`. The
// generic form works identically and is what you'd use when SCALE
// is itself a const generic in your code:
let _generic: decimal_scaled::D38<12> = D38s12::from_int(42);

// Arithmetic is plain operator overloads — exact for + / − / %,
// rounded (half-to-even) for × / ÷.
let sum     = a + b;                                // 3.3 exactly
let product = a * b;                                // 2.42 exactly
let half    = a / d38!(2, scale 12);

assert_eq!(sum, c);
assert_eq!(sum.to_string(), "3.3");
assert_eq!(a.to_bits(), 1_100_000_000_000);         // value × 10^12

// Constants are available where you need them.
let _zero = D38s12::ZERO;
let _one  = D38s12::ONE;
```

The `macros` feature is opt-in (it pulls in a `proc_macro` build
dependency). Without it, the `FromStr` and `from_bits` paths are
always available.

---

## Type names

The number in each `D<N>` type name is **the number of base-10
digits it can safely represent**, *not* the bit-width of the
underlying integer. The crate's home is decimal arithmetic, so it
names its types in the unit users actually reason about. Mapping:

| type | constructor macro | underlying signed integer | safe decimal digits (= `MAX_SCALE`) | max value at SCALE 0 | required feature | curated per-scale macros |
|---|---|---|---|---|---|---|
| `D9<S>`   | `d9!`   | `i32` (32 bits) | 9 | ±2.1 × 10⁹ | always available | `d9s0!`, `d9s2!`, `d9s4!`, `d9s6!`, `d9s9!` |
| `D18<S>`  | `d18!`  | `i64` (64 bits) | 18 | ±9.2 × 10¹⁸ | always available | `d18s0!`, `d18s2!`, `d18s4!`, `d18s6!`, `d18s9!`, `d18s12!`, `d18s18!` |
| `D38<S>`  | `d38!`  | `i128` (128 bits) | 38 | ±1.7 × 10³⁸ | always available | `d38s0!`, `d38s2!`, `d38s4!`, `d38s6!`, `d38s8!`, `d38s9!`, `d38s12!`, `d38s15!`, `d38s18!`, `d38s24!`, `d38s35!`, `d38s38!` |
| `D76<S>`  | `d76!`  | `Int256` (256 bits, in-tree wide-int) | 76 | ±5.8 × 10⁷⁶ | `d76` / `wide` | `d76s0!`, `d76s2!`, `d76s6!`, `d76s12!`, `d76s18!`, `d76s35!`, `d76s50!`, `d76s76!` |
| `D153<S>` | `d153!` | `Int512` (512 bits) | 153 | ±6.7 × 10¹⁵³ | `d153` / `wide` | `d153s0!`, `d153s35!`, `d153s75!`, `d153s150!`, `d153s153!` |
| `D307<S>` | `d307!` | `Int1024` (1024 bits) | 307 | ±9.0 × 10³⁰⁷ | `d307` / `x-wide` | `d307s0!`, `d307s35!`, `d307s150!`, `d307s300!`, `d307s307!` |

The number in each type name (`9`, `18`, `38`, …) is the type's
`MAX_SCALE` — equivalently, the safe-decimal-digits count
`⌊(bits − 1) · log₁₀ 2⌋`. The largest scale at which every
`MAX_SCALE`-digit decimal value (`±999…9`) fits the signed
storage; also the largest `S` you can pass as the const generic
parameter. `D38<38>` therefore represents values in `[−1.7, 1.7]`
with 38 fractional digits; `D38<0>` represents integers up to
`±10³⁸`.

All constructor macros require the `macros` feature in addition
to any per-tier feature listed above. Per-scale macros pre-bake
`scale N` and forward every other qualifier (`rounded`,
`radix N`) to the underlying constructor. Scales outside the
curated subset remain reachable via the explicit `, scale N`
qualifier on the main constructor.

Pick the narrowest tier whose range covers your values at the
scale you need. Widening is free (`From` / `widen()`); narrowing
is fallible (`TryFrom` / `narrow()`).

---

## Why another numeric type?

Every numeric type makes a choice about which numbers it can represent exactly. There is no universal answer - the right choice depends on where the numbers come from.

### The binary fraction problem

Standard floating-point types (`f32`, `f64`, `f128`) store values as:

```
value = mantissa × 2^exponent
```

This means the fractional part of any stored number must be expressible as a sum of negative powers of two: ½, ¼, ⅛, 1/16, …

The number `1.1` cannot be expressed this way. In binary it is:

```
1.0001100110011001100110011001100110011001100110011...  (repeating forever)
```

A 64-bit float truncates this at 52 mantissa bits. The value actually stored is:

```
1.100000000000000088817841970012523233890533447265625
```

This is not a bug - it is an unavoidable consequence of the representation. The same applies to `0.1`, `0.2`, `0.3`, and most everyday decimal fractions. This is why `0.1 + 0.2 == 0.3` is `false` in every binary floating-point system.

### The binary fixed-point alternative

The `fixed` crate (`I64F64`, `I32F32`, etc.) uses binary fixed-point: a fixed number of bits for the integer part and a fixed number of bits for the fractional part. A value is stored as:

```
value = raw_integer × 2^(-FRAC_BITS)
```

This eliminates the rounding from exponent adjustments, but the representable fractions are still powers of two. `I64F64` cannot represent `0.1` exactly either. It excels at signal processing, physics simulations, and anywhere numbers arrive as binary data or are generated by mathematical operations.

### Base-10 fixed-point: filling the gap

`decimal-scaled` uses base-10 fixed-point:

```
value = raw_integer × 10^(-SCALE)
```

With `SCALE = 12`, the number `1.1` is stored as the integer `1_100_000_000_000`. It is exact. Every number a human can write with up to `SCALE` decimal digits is represented exactly. The tradeoff is that numbers like `1/3` or `π` still cannot be represented exactly - no finite representation can hold every number. The question is always *which* numbers you need to be exact.

### Choosing the right number space

All numeric types have a finite number space. The choice is which region of the real line to cover densely and which values to round.

| System | Decimal places | Exact for | Rounds | Best suited for |
|---|---|---|---|---|
| `f64` | dynamic (binary exponent, not decimal) | powers-of-2 fractions | decimal fractions like 0.1 | scientific computation, computer-generated values |
| `f128` | dynamic (binary exponent, not decimal) | powers-of-2 fractions (more precision) | decimal fractions | high-precision scientific work |
| `fixed::I64F64` | fixed (64 binary fractional bits, not decimal) | binary fixed fractions | decimal fractions | digital signal processing, physics, binary data |
| `rust_decimal` | variable per value (0–28, stored alongside each number) | decimal fractions up to 28 digits | repeating decimals | finance, variable scale |
| `bigdecimal` | variable per value (unbounded, heap-allocated) | any terminating decimal | repeating decimals | arbitrary-precision decimal work |
| `D38<S>` (this crate) | **fixed to `S` at compile time** | decimal fractions up to `S` digits | repeating decimals | finance, computer-aided design, human-entered values |

**Use `decimal-scaled` when:**
- Values are entered by humans as decimal strings (prices, measurements, quantities)
- You need deterministic, platform-identical results across every machine
- The scale is known at compile time and you want zero-cost const-generic specialisation
- You need `no_std` compatibility
- You want a single canonical representation per value (no normalisation step)

**Use `f64` or `f128` when:**
- Values come from sensors, physics engines, or mathematical operations
- The number space is continuous and decimal fractions are not special
- You need the dynamic range of [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) binary floating-point (from ~10⁻³⁰⁸ to ~10³⁰⁸)

**Use `fixed` when:**
- Values are in a known integer-and-fraction format from binary protocols
- You are doing digital signal processing or embedded arithmetic where binary fractions are natural
- You need the best throughput on platforms without hardware decimal support

**Use `rust_decimal` when:**
- Scale varies between values (e.g. mixing 0.1 and 0.001 in the same collection)
- You need up to 28 significant decimal digits
- You are happy to carry a per-value scale byte and pay normalisation cost on equality/hash

**Use `bigdecimal` when:**
- Precision requirements are unbounded or unknown at compile time
- Throughput is not a concern

---

## What `decimal-scaled` provides

`D38<const SCALE: u32>` is a `#[repr(transparent)]` newtype around `i128`. The const generic `SCALE` is the base-10 exponent baked into the type at compile time. There is exactly one representation per value: no normalisation, no variable scale, no heap allocation.

```
stored = logical_value × 10^SCALE
```

With `SCALE = 12`, the value `1.5` is stored as `1_500_000_000_000i128`.

### Properties

- **Deterministic** - arithmetic is pure integer; identical bit-pattern outputs on every platform.
- **Canonical** - one scale means one representation per value. `Hash`, `Eq`, and `Ord` are derived directly from `i128`. Two values that are equal always hash identically, with no normalisation step.
- **`no_std` compatible** - compiles with `no_std + alloc` when default features are disabled.
- **`num-traits` compatible** - implements `Zero`, `One`, `Num`, `Bounded`, `Signed`, `FromPrimitive`, `ToPrimitive`, and the `Checked*` family.
- **`serde` support** - canonical-string serialize/deserialize behind the `serde` feature (on by default).
- **Const-generic scale** - additional scale variants (`D38<6>`, `D38<18>`) are free type aliases, not separate implementations.

---

## Numeric comparison table

| Type                           | Storage | Base | `0.1` exact | `1.1` exact | Range | Accuracy (error bound) | `no_std` |
|--------------------------------|---|---|---|---|---|---|---|
| `f32`                          | 32-bit IEEE 754 | 2 | No | No | ~±3.4 × 10³⁸ | basic ops: ≤ 0.5 [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) (IEEE 754); transcendentals: libm-defined, not guaranteed | Yes |
| `f64`                          | 64-bit IEEE 754 | 2 | No | No | ~±1.8 × 10³⁰⁸ | basic ops: ≤ 0.5 ULP (IEEE 754); transcendentals: libm-defined, not guaranteed | Yes |
| `f128`                         | 128-bit IEEE 754 | 2 | No | No | ~±1.2 × 10⁴⁹³² | basic ops: ≤ 0.5 ULP (IEEE 754); transcendentals: libm-defined, not guaranteed | Partial |
| `fixed::I64F64`                | 128-bit binary fixed | 2 | No | No | ~±9.2 × 10¹⁸ | add/sub: exact; mul/div: ≤ 1 ULP; no transcendentals | Yes |
| `fixed::I32F32`                | 64-bit binary fixed | 2 | No | No | ~±2.1 × 10⁹ | add/sub: exact; mul/div: ≤ 1 ULP; no transcendentals | Yes |
| `rust_decimal`                 | 96-bit + per-value scale (0..=28) | 10 | Yes | Yes | ±7.9 × 10²⁸ | add/sub: exact at common scale; mul/div: ≤ 1 ULP; transcendentals: software, **not** correctly rounded | Yes |
| `bigdecimal`                   | heap-allocated arbitrary precision | 10 | Yes | Yes | Unbounded | exact at the configured precision; transcendentals: limited | No |
| `D38<S>` (this)               | 128-bit integer, scale fixed at compile time, S ∈ 0..=38 | 10 | Yes | Yes | ±i128::MAX / 10ˢ | add/sub: **exact**; mul/div: ≤ 1 ULP; **strict transcendentals: ≤ 0.5 ULP (correctly rounded)** | Yes |
| `D76<S>` / `D153<S>` / `D307<S>` (this, `wide`) | 256 / 512 / 1024-bit integer, S up to 76 / 153 / 307 | 10 | Yes | Yes | wider, S-dependent | same accuracy as `D38<S>` | Yes |

The accuracy column gives the error bound on computed results, in [ULPs](https://en.wikipedia.org/wiki/Unit_in_the_last_place) (units in the last place). A 0.5 ULP bound — "correctly rounded" — is the IEEE-754 round-to-nearest contract and the strongest accuracy guarantee a finite numeric type can give. The floats meet it for basic arithmetic but not for transcendentals; `decimal-scaled`'s strict transcendentals meet it for transcendentals as well, which is the capability the alternatives do not offer. The position of the ULP — the absolute size of `1 ULP` — is the type's *scale*: for `f64` it's a relative ~2⁻⁵² of the value's magnitude, for `D38<S>` it's exactly `10⁻ˢ` at every value, fixed at compile time.

### Hash and equality contracts

A well-behaved numeric type must satisfy: if `a == b` then `hash(a) == hash(b)`. The way different types handle this for values like `1.1` and `1.10` varies significantly.

| Type | `1.10 == 1.1`? | `hash(1.10) == hash(1.1)`? | `Hash` implemented? | How |
|---|---|---|---|---|
| `f32` / `f64` | Yes (same bit pattern) | N/A | No - Not-a-Number breaks the contract | - |
| `f128` | Yes (same bit pattern) | N/A | No | - |
| `fixed::I64F64` | Yes (same binary approximation) | Yes | Yes | structural (one representation) |
| `rust_decimal` | Yes | Yes | Yes | normalises trailing zeros at comparison and hash time |
| `bigdecimal` | Yes | Yes | Yes | normalises at comparison and hash time |
| `D38<S>` (this) | Yes | Yes | Yes | structural - scale is fixed, one bit pattern per value |

`f32`, `f64`, and `f128` do not implement `Hash` in the Rust standard library because Not-a-Number values are not equal to themselves (`NaN != NaN`) while a structural hash would make all such values collide - the contract cannot be satisfied without special-casing.

For `rust_decimal` and `bigdecimal`, the normalisation is correct but carries a runtime cost on every comparison and hash call, and it means the stored representation is not canonical - you cannot memcmp two values.

`D38<S>` derives `Hash`, `Eq`, and `Ord` directly from `i128`. Because the scale is fixed at compile time there is exactly one `i128` value per logical number. `1.10` and `1.1` parsed via `FromStr` both produce `D38s12(1_100_000_000_000)` - the same bit pattern - so equality and hashing are a single integer comparison with no runtime normalisation.

### Key differences from `fixed`

The `fixed` crate's `I64F64` has 64 bits of integer and 64 bits of binary fraction. Its least significant bit is 2⁻⁶⁴ ≈ 5.4 × 10⁻²⁰, and its maximum value is 2⁶³ - 1 ≈ 9.2 × 10¹⁸.

`D38<20>` has a least significant decimal digit of 10⁻²⁰ and a maximum value of i128::MAX / 10²⁰ ≈ 1.7 × 10¹⁸ model units. The two types offer comparable precision and range in this configuration, but with opposite trade-offs: `I64F64` represents its fractional part in binary (exact for powers of two, rounded for decimal fractions), while `D38<20>` represents it in decimal (exact for decimal fractions, rounded for fractions like 1/3).

For human-scale decimal values `D38` gives decimal-exact results with no rounding on input or output. For values derived from binary arithmetic or mathematical operations, `I64F64` avoids the binary-to-decimal rounding boundary entirely.

---

## Performance and accuracy

`D38` arithmetic is a thin wrapper over `i128`: add / sub are a single
instruction (~1 ns), mul / div carry a 256-bit widening step (~10 ns).
The wide `D76` tier keeps add / sub almost free but its mul / div pay
for a 256-bit hand-rolled-integer divide — roughly 20× the `D38` cost.

Transcendentals (`ln`, `exp`, `sqrt`, trig, …) come in two forms. The
**fast** `f64`-bridge form (~40 ns) inherits `f64`'s precision
ceiling and is not platform-independent. The **strict** integer-only
form — **on by default** — is **correctly rounded to within 0.5 ULP**
of the exact result (the IEEE-754 round-to-nearest contract) and is
`no_std` and platform-deterministic. Build with `default-features =
false, features = ["std", "serde"]` to switch the plain `ln` / `exp` /
… surface to the fast f64 bridge.

The full per-algorithm catalogue with citations, equations, and the
Wikipedia / Wolfram MathWorld / author-homepage links lives in
[`ALGORITHMS.md`](ALGORITHMS.md).

### Transcendental accuracy comparison

A *correctly rounded* result is the exact mathematical value rounded
to the nearest representable number — i.e. the error is at most half a
ULP. It is the strongest accuracy guarantee a finite type can give,
and the capability the alternatives do not offer:

| Type | Transcendentals | Correctly rounded to 0.5 ULP | Platform-deterministic |
|---|---|---|---|
| `f32` / `f64` (platform libm) | yes | no — `libm` is not guaranteed correctly rounded | no |
| `fixed` (`I64F64`, …) | none | — | — |
| `bigdecimal` | none | — | — |
| `rust_decimal` (`MathematicalOps`) | yes | no — accurate, but not to the last place | yes |
| `decimal-scaled` — fast (`f64` bridge, opt-in) | yes | no — inherits `f64` | no |
| `decimal-scaled` — **strict** (default, `*_strict`) | yes | **yes — within 0.5 ULP** | **yes** |

For series functions the strict form costs ~700× the fast bridge;
`sqrt_strict` is the exception — algebraic, so it ties the fast form.
Full head-to-head measurements against `bnum`, `ruint`, `rust_decimal`,
and `fixed` are in [`docs/benchmarks.md`](docs/benchmarks.md).

---

## Scale aliases

| Alias | `SCALE` | 1 least significant decimal digit | Approximate range |
|---|---|---|---|
| `D38s0` | 0 | 1 | ±1.7 × 10³⁸ |
| `D38s2` | 2 | 0.01 (cents) | ±1.7 × 10³⁶ |
| `D38s6` | 6 | 10⁻⁶ (µ) | ±1.7 × 10³² |
| `D38s12` | 12 | 10⁻¹² (p) | ±1.7 × 10²⁶ |
| `D38s18` | 18 | 10⁻¹⁸ (a) | ±1.7 × 10²⁰ |
| `D38s38` | 38 | 10⁻³⁸ | ±1.7 |

Aliases `D38s0` through `D38s38` are all provided. `SCALE = 39` would overflow `i128`.

---

## The width family

`D38` is the foundation, but it is one of six storage widths that share
an identical API and the `Decimal` trait:

| Type | Storage | `MAX_SCALE` | Feature gate |
|---|---|---|---|
| `D9`   | 32-bit  | 9   | always on |
| `D18`   | 64-bit  | 18  | always on |
| `D38`  | 128-bit | 38  | always on |
| `D76`  | 256-bit | 76  | `d76` / `wide` |
| `D153`  | 512-bit | 153 | `d153` / `wide` |
| `D307` | 1024-bit| 307 | `d307` / `wide` |

Pick the narrowest width whose range covers your values at the scale you
need. Widening between widths is lossless (`From`); narrowing is fallible
(`TryFrom`). The wide tier is backed by an in-tree hand-rolled
wide-integer type — no external big-integer dependency — and is opt-in.
See [`docs/widths.md`](docs/widths.md).

## Compile-time literals

With the `macros` feature, `d38!` writes `D38` values at compile time
with automatic scale inference:

```rust
use decimal_scaled::d38;

let price = d38!(19.99);              // D38<2>
let micro = d38!(1.234_567, scale 6); // D38<6>
let rnd   = d38!(1.235, scale 2, rounded); // 1.24 (half-to-even)
```

See [`docs/macros.md`](docs/macros.md).

---

## Features

| Feature | Default | Description |
|---|---|---|
| `std` | yes | Platform `f64`-bridge transcendentals (used when `strict` is off). Pulls in `alloc`. |
| `alloc` | yes | String formatting and parsing on `no_std`. Required. |
| `serde` | yes | `Serialize` / `Deserialize` via `serde_helpers`. |
| `strict` | **yes** | Plain transcendentals (`ln` / `sin` / …) dispatch to the integer-only, 0.5 ULP, platform-deterministic path. `no_std`-compatible. The `*_strict` and `*_fast` named entry points are both always available; this feature only chooses what plain `*` resolves to. |
| `macros` | no | The `d38!` compile-time decimal-literal macro. |
| `fast` | no | Forces plain transcendentals to dispatch to the f64 bridge instead of `*_strict`, even when `strict` is also on. The integer-only `*_strict` and f64 `*_fast` named methods stay available either way. |
| `rounding-*` | no | Five mutually-exclusive flags that change the crate-wide default `RoundingMode` at compile time. |
| `d76` / `d153` / `d307` | no | The wide decimal tiers (256 / 512 / 1024-bit storage), backed by an in-tree hand-rolled wide-integer type. |
| `wide` | no | Umbrella over `d76` + `d153` + `d307`. |
| `experimental-floats` | no | Nightly-only `f16` / `f128` entry points on the float bridge. |

See [`docs/features.md`](docs/features.md) for the full reference and
common configurations.

---

## Documentation

In-depth usage guides live in [`docs/`](docs/guides.md):

- [Getting started](docs/getting-started.md) — constructing values, arithmetic, formatting, parsing.
- [The width family](docs/widths.md) — `D9` … `D307`, scale ranges, the `Decimal` trait.
- [Conversions](docs/conversions.md) — integers, floats, cross-width widening / narrowing.
- [Rounding](docs/rounding.md) — `RoundingMode`, `rescale`, the `rounding-*` features.
- [Strict mode](docs/strict-mode.md) — integer-only transcendentals.
- [The `d38!` macro](docs/macros.md) — compile-time decimal literals.
- [Cargo features](docs/features.md) — every feature flag.
- [Benchmarks](docs/benchmarks.md) — head-to-head against `bnum`, `ruint`, `rust_decimal`, and `fixed`, plus fast vs strict.

API reference: <https://docs.rs/decimal-scaled/>.

---

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

Copyright 2026 John Moxley.

Third-party code attributions are listed in [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY).
