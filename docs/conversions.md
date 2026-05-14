# Conversions

## From integers

`From<iN>` / `From<uN>` scale the integer by `10^SCALE`:

```rust
# use decimal_scaled::D128s2;
let a: D128s2 = 7i32.into();          // 7.00
let b = D128s2::from_int(7i64);       // 7.00 — widest integer source
let c = D128s2::from_i32(-3);         // -3.00
```

`From` is provided for the integer types narrower than the storage.
For `i128` / `u128` into `D128`, where the scaled value can overflow,
the conversion is `TryFrom` instead:

```rust
# use decimal_scaled::D128s2;
let ok  = D128s2::try_from(100_i128);          // Ok
let bad = D128s2::try_from(i128::MAX);         // Err(ConvertError::Overflow)
```

## To integers — `to_int_lossy`

`to_int_lossy` rounds to the nearest integer and returns an `i64`,
saturating if the integer part is out of `i64` range. It is a lossy
method, so it comes as a `_with` pair:

```rust
use decimal_scaled::{D128s2, RoundingMode};

let v = D128s2::from_bits(250);   // 2.50
assert_eq!(v.to_int_lossy(), 2);                                  // HalfToEven
assert_eq!(v.to_int_lossy_with(RoundingMode::HalfAwayFromZero), 3);
assert_eq!(v.to_int_lossy_with(RoundingMode::Ceiling), 3);
```

## The float bridge

Float conversions are explicit and lossy — never silent. NaN maps to
`ZERO`, ±infinity saturate to `MAX` / `MIN`, and out-of-range finite
values saturate by sign.

```rust
use decimal_scaled::{D128s4, RoundingMode};

let v = D128s4::from_f64_lossy(3.14159);
let w = D128s4::from_f64_lossy_with(3.14159, RoundingMode::Trunc);

let back: f64 = v.to_f64_lossy();
let back32: f32 = v.to_f32_lossy();
```

`to_f64_lossy` / `to_f32_lossy` are available in `no_std`; the
`from_f64*` constructors need `std`. `TryFrom<f64>` / `TryFrom<f32>` are
also provided — they truncate and return `ConvertError` for non-finite
or out-of-range inputs.

On nightly with the `experimental-floats` feature, `f16` and `f128`
entry points (`from_f16_lossy`, `to_f128_lossy`, …) are also available.

> The float bridge is a *conversion*, not a transcendental operation —
> it is available regardless of the `strict` feature.

## Cross-width conversions

Widening (to a larger storage) is lossless and infallible — `From`:

```rust
# use decimal_scaled::{D32s2, D64s2, D128s2};
let small: D32s2  = D32s2::from_bits(150);
let mid:   D64s2  = small.into();
let wide:  D128s2 = small.into();      // skip-widening works too
```

Narrowing (to a smaller storage) is fallible — `TryFrom` — because the
value may not fit:

```rust
# use decimal_scaled::{D64s2, D128s2};
let wide = D128s2::from_bits(150);
let ok:  D64s2 = wide.try_into().unwrap();

let huge = D128s2::MAX;
let err: Result<D64s2, _> = huge.try_into();
assert!(err.is_err());
```

There is deliberately no infallible `From` for the narrowing direction —
a silent saturating conversion would violate the crate's exact-decimal
promise, so narrowing failure is always loud at the call site.

The same widen-`From` / narrow-`TryFrom` rule extends across the whole
family, including the wide tier (e.g. `D128 → D256` widening,
`D512 → D256` narrowing) when the relevant features are enabled.

Note that cross-width conversions keep the *scale* unchanged. To change
both width and scale, compose a cross-width conversion with a
[`rescale`](rounding.md).
