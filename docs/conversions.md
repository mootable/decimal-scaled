# Conversions

## From integers

`From<iN>` / `From<uN>` scale the integer by `10^SCALE`:

```rust
# use decimal_scaled::D38s2;
let a: D38s2 = 7i32.into();          // 7.00
let b: D38s2 = 7i64.into();          // 7.00 - widest infallible integer source
let c: D38s2 = (-3i32).into();       // -3.00
```

`From` is provided for the integer types narrower than the storage.
For `i128` / `u128` into `D38`, where the scaled value can overflow,
the conversion is `TryFrom` instead:

```rust
# use decimal_scaled::D38s2;
let ok  = D38s2::try_from(100_i128);          // Ok
let bad = D38s2::try_from(i128::MAX);         // Err(ConvertError::Overflow)
```

## To integers - `to_int`

`to_int` rounds to the nearest integer and returns an `i64`,
saturating if the integer part is out of `i64` range. It is a lossy
method, so it comes as a `_with` pair:

```rust
use decimal_scaled::{D38s2, RoundingMode};

let v: D38s2 = "2.50".parse().unwrap();   // 2.50
assert_eq!(v.to_int(), 2);                                  // HalfToEven
assert_eq!(v.to_int_with(RoundingMode::HalfAwayFromZero), 3);
assert_eq!(v.to_int_with(RoundingMode::Ceiling), 3);
```

## The float bridge

Float conversions are explicit and lossy - never silent. NaN maps to
`ZERO`, ±infinity saturate to `MAX` / `MIN`, and out-of-range finite
values saturate by sign.

```rust
use decimal_scaled::{D38s4, RoundingMode};

let v = D38s4::from_f64(3.14159);
let w = D38s4::from_f64_with(3.14159, RoundingMode::Trunc);

let back: f64 = v.to_f64();
let back32: f32 = v.to_f32();
```

`to_f64` / `to_f32` are available in `no_std`; the
`from_f64*` constructors need `std`. `TryFrom<f64>` / `TryFrom<f32>` are
also provided - they truncate and return `ConvertError` for non-finite
or out-of-range inputs.

On nightly with the `experimental-floats` feature, `f16` and `f128`
entry points (`from_f16`, `to_f128`, …) are also available.

> The float bridge is a *conversion*, not a transcendental operation -
> it is available regardless of the `strict` feature.

## Cross-width conversions

Widening (to a larger storage) is lossless and infallible - `From`:

```rust
# use decimal_scaled::{D18s2, D38s2};
let small: D18s2 = "1.50".parse().unwrap();
let wide:  D38s2 = small.into();      // lossless widen to a larger storage
```

Narrowing (to a smaller storage) is fallible - `TryFrom` - because the
value may not fit:

```rust
# use decimal_scaled::{D18s2, D38s2};
let wide: D38s2 = "1.50".parse().unwrap();
let ok:  D18s2 = wide.try_into().unwrap();

let huge = D38s2::MAX;
let err: Result<D18s2, _> = huge.try_into();
assert!(err.is_err());
```

There is deliberately no infallible `From` for the narrowing direction -
a silent saturating conversion would violate the crate's exact-decimal
promise, so narrowing failure is always loud at the call site.

The same widen-`From` / narrow-`TryFrom` rule extends across the whole
family, including the wide tier (e.g. `D38 → D76` widening,
`D153 → D76` narrowing) when the relevant features are enabled.

Note that cross-width conversions keep the *scale* unchanged. To change
both width and scale, compose a cross-width conversion with a
[`rescale`](rounding.md).
