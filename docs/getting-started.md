# Getting started

## Install

```toml
[dependencies]
decimal-scaled = "0.5"
```

`no_std` (drops `std` and `serde`, keeps `alloc`):

```toml
[dependencies]
decimal-scaled = { version = "0.5", default-features = false }
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
scale aliases `D38s0` … `D38s37` name specific scales:

```rust
use decimal_scaled::{D38, D38s2, D38s12};

type Cents = D38s2;          // == D38<2>
type Pico  = D38<12>;        // same as D38s12
```

For the narrower (`D18`) and wider tiers (`D57` / `D76` /
`D115` / `D153` / `D230` / `D307` for the `wide` umbrella; `D462` /
`D616` under `x-wide`; `D924` / `D1232` under `xx-wide`), see [the
width family](widths.md).

## Constructing values

```rust
use decimal_scaled::{D38s2, d38};

// 1. Compile-time literal macro (scale inferred - see the macro guide).
let a = d38!(19.99);

// 2. From an integer, scaled by 10^SCALE - fallible (TryFrom), because
//    scaling can overflow the storage near a width's top scale.
use core::convert::TryFrom;
let b = D38s2::try_from(20i64).unwrap();    // 20.00
let c = D38s2::try_from(-5i64).unwrap();    // -5.00

// 3. The same TryFrom surface covers the widest integer sources (i128 / u128).
let d = D38s2::try_from(19i128).unwrap();   // 19.00

// 4. Parsing a string.
use core::str::FromStr;
let e = D38s2::from_str("19.99").unwrap();

// 5. Constants.
let zero = D38s2::ZERO;
let one  = D38s2::ONE;                  // raw = 100 at scale 2
```

`from_bits` / `to_bits` are the exact round-trip into and out of the
storage integer. The storage is the const-generic `Int<N>` (`Int<2>`
for `D38`), so build the raw value through it:

```rust
# use decimal_scaled::{D38s2, Int};
let v = D38s2::from_bits(Int::<2>::from(1999i64));
assert_eq!(v.to_bits(), 1999i128);      // `Int<2>` compares with `i128`
assert_eq!(D38s2::multiplier(), Int::<2>::from(100i64));   // 10^SCALE
assert_eq!(v.scale(), 2);                // the const generic, as a value
```

## Arithmetic

`Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg` and their `*Assign` forms are
implemented. Operands must share the same type (same width *and* scale)
- mixing scales is deliberately a compile error; convert explicitly.

```rust
# use decimal_scaled::{D38s2, Int};
let x = D38s2::from_bits(Int::<2>::from(1050i64));   // 10.50
let y = D38s2::from_bits(Int::<2>::from(300i64));    //  3.00

assert_eq!((x + y).to_bits(), 1350i128);   // 13.50
assert_eq!((x - y).to_bits(),  750i128);   //  7.50
assert_eq!((x * y).to_bits(), 3150i128);   // 31.50  (rescaled by 10^SCALE)
assert_eq!((x / y).to_bits(),  350i128);   //  3.50
```

The default operators panic on overflow — in debug *and* release
builds, identically at every width and scale. A fixed-width decimal
has no infinity or NaN to absorb an out-of-range result, and silently
returning a wrapped or saturated value would be a wrong number with no
signal, so the default fails loudly. For explicit control there is the
full `checked_* / wrapping_* / saturating_* / overflowing_*` family:

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
# use decimal_scaled::{D38s2, Int};
let v = D38s2::from_bits(Int::<2>::from(-2050i64));
assert_eq!(format!("{v}"), "-20.50");
assert_eq!(format!("{v:?}"), "D38<2>(-20.50)");
```

`FromStr` parses the same canonical form. `1.10` and `1.1` parsed at the
same scale produce the *same* bit pattern, so equality and hashing are a
single integer comparison. Parsing works at every legal `SCALE` on
every width — including the deep wide-tier scales (`D307<300>`,
`D1232<1231>`) — without the u128-intermediate ceiling that older
releases imposed.

## Rounding helpers

`floor`, `ceil`, `round` (half away from zero), `trunc`, and `fract`
operate at the type's scale:

```rust
# use decimal_scaled::{D38s2, Int};
let v = D38s2::from_bits(Int::<2>::from(1250i64));   // 12.50
assert_eq!(v.floor().to_bits(), 1200i128);
assert_eq!(v.ceil().to_bits(),  1300i128);
assert_eq!(v.round().to_bits(), 1300i128);
assert_eq!(v.trunc().to_bits(), 1200i128);
assert_eq!(v.fract().to_bits(),   50i128);
```

To round to a *different* scale, use `rescale` - see the
[rounding guide](rounding.md).

## Next steps

- [Conversions](conversions.md) — integers, floats, and cross-width.
- [Cross-scale operations](cross-scale.md) — mixing widths and SCALEs in one expression via `mul_of` / `add_of` / `cmp_of` / `clamp_of` / etc., plus the nightly-gated `cross::mul(a, b)` auto-inferred form.
- [The width family](widths.md) — choosing one of the twelve widths from D18 to D1232.
- [The `d38!` macro](macros.md) — ergonomic compile-time literals.
- [Rounding modes](rounding.md) — switching from HalfToEven to Floor, Ceiling, Trunc, etc., per call or crate-wide.
- [Strict vs fast transcendentals](strict-mode.md) — when to reach for `*_fast` and what you give up.
