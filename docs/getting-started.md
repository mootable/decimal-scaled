# Getting started

## Install

```toml
[dependencies]
decimal-scaled = "0.3.2"
```

`no_std` (drops `std` and `serde`, keeps `alloc`):

```toml
[dependencies]
decimal-scaled = { version = "0.3.2", default-features = false }
```

See [Cargo features](features.md) for the full list.

## The model

Every type is a `#[repr(transparent)]` newtype around an integer. The
const generic `SCALE` is the base-10 exponent baked into the type:

```text
logical value  =  raw_integer × 10^(-SCALE)
```

With `SCALE = 2`, the integer `1999` is the logical value `19.99`. There
is exactly one representation per value - no normalisation, no per-value
scale byte, no heap allocation.

The primary type is `D38<const SCALE: u32>` (128-bit storage). The
scale aliases `D38s0` … `D38s38` name specific scales:

```rust
use decimal_scaled::{D38, D38s2, D38s12};

type Cents = D38s2;          // == D38<2>
type Pico  = D38<12>;        // same as D38s12
```

For the narrower (`D9`, `D18`) and wider tiers (`D56` / `D76` /
`D114` / `D153` / `D230` / `D307` for the `wide` umbrella; `D461` /
`D615` under `x-wide`; `D923` / `D1231` under `xx-wide`), see [the
width family](widths.md).

## Constructing values

```rust
use decimal_scaled::{D38s2, d38};

// 1. Compile-time literal macro (scale inferred - see the macro guide).
let a = d38!(19.99);

// 2. From an integer, scaled by 10^SCALE.
let b = D38s2::from_int(20);            // 20.00
let c = D38s2::from_i32(-5);            // -5.00

// 3. From the raw storage integer directly (raw = value × 10^SCALE).
let d = D38s2::from_bits(1999);         // 19.99

// 4. Parsing a string.
use core::str::FromStr;
let e = D38s2::from_str("19.99").unwrap();

// 5. Constants.
let zero = D38s2::ZERO;
let one  = D38s2::ONE;                  // raw = 100 at scale 2
```

`from_bits` / `to_bits` are the exact round-trip into and out of the
storage integer:

```rust
# use decimal_scaled::D38s2;
let v = D38s2::from_bits(1999);
assert_eq!(v.to_bits(), 1999);
assert_eq!(D38s2::multiplier(), 100);   // 10^SCALE
assert_eq!(v.scale(), 2);                // the const generic, as a value
```

## Arithmetic

`Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg` and their `*Assign` forms are
implemented. Operands must share the same type (same width *and* scale)
- mixing scales is deliberately a compile error; convert explicitly.

```rust
# use decimal_scaled::D38s2;
let x = D38s2::from_bits(1050);   // 10.50
let y = D38s2::from_bits(300);    //  3.00

assert_eq!((x + y).to_bits(), 1350);   // 13.50
assert_eq!((x - y).to_bits(),  750);   //  7.50
assert_eq!((x * y).to_bits(), 3150);   // 31.50  (rescaled by 10^SCALE)
assert_eq!((x / y).to_bits(),  350);   //  3.50
```

Overflow follows Rust's integer convention: debug builds panic, release
builds wrap. For explicit control there is the full
`checked_* / wrapping_* / saturating_* / overflowing_*` family:

```rust
# use decimal_scaled::D38s2;
assert_eq!(D38s2::MAX.checked_add(D38s2::ONE), None);
assert_eq!(D38s2::MAX.saturating_add(D38s2::ONE), D38s2::MAX);
```

## Formatting and parsing

`Display` renders the canonical decimal string; `Debug` shows the type
and value; `LowerHex` / `UpperHex` / `Octal` / `Binary` format the raw
storage integer.

```rust
# use decimal_scaled::D38s2;
let v = D38s2::from_bits(-2050);
assert_eq!(format!("{v}"), "-20.50");
assert_eq!(format!("{v:?}"), "D38<2>(-20.50)");
```

`FromStr` parses the same canonical form. `1.10` and `1.1` parsed at the
same scale produce the *same* bit pattern, so equality and hashing are a
single integer comparison.

## Rounding helpers

`floor`, `ceil`, `round` (half away from zero), `trunc`, and `fract`
operate at the type's scale:

```rust
# use decimal_scaled::D38s2;
let v = D38s2::from_bits(1250);   // 12.50
assert_eq!(v.floor().to_bits(), 1200);
assert_eq!(v.ceil().to_bits(),  1300);
assert_eq!(v.round().to_bits(), 1300);
assert_eq!(v.trunc().to_bits(), 1200);
assert_eq!(v.fract().to_bits(),   50);
```

To round to a *different* scale, use `rescale` - see the
[rounding guide](rounding.md).

## Next steps

- [Conversions](conversions.md) — integers, floats, and cross-width.
- [The width family](widths.md) — choosing one of the thirteen widths from D9 to D1231.
- [The `d38!` macro](macros.md) — ergonomic compile-time literals.
- [Rounding modes](rounding.md) — switching from HalfToEven to Floor, Ceiling, Trunc, etc., per call or crate-wide.
- [Strict vs fast transcendentals](strict-mode.md) — when to reach for `*_fast` and what you give up.
