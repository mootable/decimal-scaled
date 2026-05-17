# Rounding

Any operation that discards precision needs a rounding policy. In
`decimal-scaled` that policy is the `RoundingMode` enum, and the rule is
consistent across the whole API:

> **Every method that loses precision ships as a pair:** a plain form
> that uses the crate default, and a `_with` form that takes an explicit
> `RoundingMode`. The plain form just delegates to `_with` with the
> default.

The default is **HalfToEven** (banker's rounding, the IEEE 754 default).
You can change the default crate-wide at compile time via a single
`rounding-*` Cargo feature (see below), or override per-call with the
`_with` sibling. The `_with` family is comprehensive — `mul_with` /
`div_with` / `rem_with` for arithmetic, `rescale_with` for scale
changes, `ln_strict_with` / `exp_strict_with` / `sin_strict_with` /
every strict transcendental for the correctly-rounded path. So if you
need ASTM E29 banker's rounding for one codepath and bank-statement
away-from-zero for another, both are first-class — no global state,
no thread-locals, no library fork required to bit-match an external
system.

## `RoundingMode`

```rust
pub enum RoundingMode {
    /// Round to nearest, ties to even. IEEE-754 default; unbiased.
    /// This is the crate default.
    HalfToEven,
    /// Round to nearest, ties away from zero. Commercial/retail rule.
    HalfAwayFromZero,
    /// Round to nearest, ties toward zero.
    HalfTowardZero,
    /// Truncate toward zero (what an `as` integer cast does).
    Trunc,
    /// Round toward negative infinity.
    Floor,
    /// Round toward positive infinity.
    Ceiling,
}
```

## The `_with` pairs

```rust
use decimal_scaled::{D38s4, D38s2, RoundingMode};

let v = D38s4::from_bits(12_345);   // 1.2345

// Default mode (HalfToEven unless a `rounding-*` feature changes it):
let a: D38s2 = v.rescale::<2>();

// Explicit mode:
let b: D38s2 = v.rescale_with::<2>(RoundingMode::Floor);
let c: D38s2 = v.rescale_with::<2>(RoundingMode::Ceiling);
```

The same pairing applies to `to_int` / `to_int_with`,
`from_f64` / `from_f64_with`, and any other lossy method.

## `rescale` - changing the scale of a value

`rescale::<TARGET>()` converts a value to a different `SCALE` of the
*same width*:

- `TARGET == SCALE` - bit-identity.
- `TARGET > SCALE` - scale up: multiply by `10^(TARGET-SCALE)`.
  Lossless; panics on overflow.
- `TARGET < SCALE` - scale down: divide by `10^(SCALE-TARGET)`, applying
  the rounding mode to the discarded fractional digits.

```rust
use decimal_scaled::{D38s3, RoundingMode};

let v = D38s3::from_bits(1_235);            // 1.235

// Scale down to 2 digits - the trailing `5` must be rounded.
let down  = v.rescale::<2>();                // HalfToEven -> 1.24
let trunc = v.rescale_with::<2>(RoundingMode::Trunc);   // -> 1.23
assert_eq!(down.to_bits(),  124);
assert_eq!(trunc.to_bits(), 123);

// Scale up is always lossless.
let up = v.rescale::<6>();
assert_eq!(up.to_bits(), 1_235_000);
```

## Compile-time default selection: the `rounding-*` features

The crate default is `HalfToEven`. To change it *globally at compile
time* - so every plain (non-`_with`) lossy method uses a different mode
- enable exactly one `rounding-*` feature:

| Feature | Sets the default to |
|---|---|
| *(none)* | `HalfToEven` |
| `rounding-half-away-from-zero` | `HalfAwayFromZero` |
| `rounding-half-toward-zero` | `HalfTowardZero` |
| `rounding-trunc` | `Trunc` |
| `rounding-floor` | `Floor` |
| `rounding-ceiling` | `Ceiling` |

```toml
[dependencies]
decimal-scaled = { version = "0.3.0", features = ["rounding-half-away-from-zero"] }
```

The features are mutually exclusive in intent. If more than one is
enabled, a fixed priority order in `src/rounding.rs` picks one
deterministically - but you should enable at most one.

The `_with` methods are unaffected by these features: they always honour
the mode you pass.
