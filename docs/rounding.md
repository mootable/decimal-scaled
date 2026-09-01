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
`div_with` / `rem_with` for arithmetic, `quantize_with` for scale
changes, `ln_strict_with` / `exp_strict_with` / `sin_strict_with` /
every strict transcendental for the correctly-rounded path. So if you
need ASTM E29 banker's rounding for one codepath and bank-statement
away-from-zero for another, both are first-class — no global state,
no thread-locals, no library fork required to bit-match an external
system.

## `RoundingMode`

The eight variants are the full General Decimal Arithmetic rounding set,
so each one carries the spec's name alongside it:

```rust
pub enum RoundingMode {
    /// Round to nearest, ties to even. IEEE-754 default; unbiased.
    /// This is the crate default. GDA `round-half-even`.
    HalfToEven,
    /// Round to nearest, ties away from zero. Commercial/retail rule.
    /// GDA `round-half-up`.
    HalfAwayFromZero,
    /// Round to nearest, ties toward zero. GDA `round-half-down`.
    HalfTowardZero,
    /// Truncate toward zero (what an `as` integer cast does).
    /// GDA `round-down`.
    Trunc,
    /// Round toward negative infinity. GDA `round-floor`.
    Floor,
    /// Round toward positive infinity. GDA `round-ceiling`.
    Ceiling,
    /// Round away from zero whenever anything was discarded - the exact
    /// mirror of `Trunc`. GDA `round-up`.
    AwayFromZero,
    /// Round away from zero only when the last *kept* digit is `0` or
    /// `5`; otherwise truncate. GDA `round-05up`.
    ZeroFiveUp,
}
```

## Where each mode rounds

Most modes agree most of the time, so a table of arbitrary values teaches
nothing. Every row below makes at least one mode disagree with the rest,
and every mode differs from every other mode in at least one row — so any
two columns you compare are genuinely distinguishable here.

Each value is rounded **to an integer** (target scale `0`), so the digit
being kept is the units digit and everything after the point is
discarded:

```rust
use decimal_scaled::{D38, RoundingMode};

let v: D38<1> = "5.7".parse().unwrap();
let r: D38<0> = v.quantize_with::<0>(RoundingMode::ZeroFiveUp);
assert_eq!(r.to_bits(), 6);
```

| Value | HalfToEven | HalfAwayFromZero | HalfTowardZero | Trunc | Floor | Ceiling | AwayFromZero | ZeroFiveUp |
|---|---|---|---|---|---|---|---|---|
| `2.0` | `2` | `2` | `2` | `2` | `2` | `2` | `2` | `2` |
| `0.2` | `0` | `0` | `0` | `0` | `0` | `1` | `1` | `1` |
| `0.5` | `0` | `1` | `0` | `0` | `0` | `1` | `1` | `1` |
| `1.5` | `2` | `2` | `1` | `1` | `1` | `2` | `2` | `1` |
| `2.5` | `2` | `3` | `2` | `2` | `2` | `3` | `3` | `2` |
| `0.7` | `1` | `1` | `1` | `0` | `0` | `1` | `1` | `1` |
| `1.7` | `2` | `2` | `2` | `1` | `1` | `2` | `2` | `1` |
| `4.7` | `5` | `5` | `5` | `4` | `4` | `5` | `5` | `4` |
| `5.7` | `6` | `6` | `6` | `5` | `5` | `6` | `6` | `6` |
| `-0.5` | `0` | `-1` | `0` | `0` | `-1` | `0` | `-1` | `-1` |
| `-1.5` | `-2` | `-2` | `-1` | `-1` | `-2` | `-1` | `-2` | `-1` |
| `-0.7` | `-1` | `-1` | `-1` | `0` | `-1` | `0` | `-1` | `-1` |
| `-5.7` | `-6` | `-6` | `-6` | `-5` | `-6` | `-5` | `-6` | `-6` |

Reading it:

- **`2.0` is exact.** Nothing is discarded, so no mode moves it. Every
  mode leaves an exact value alone — rounding only ever acts on a
  non-zero discarded part.
- **`0.2` separates `AwayFromZero` from `HalfAwayFromZero`.**
  `AwayFromZero` steps up on *any* discarded part; `HalfAwayFromZero`
  only steps up from the half-way point, so it stays at `0` here. Below
  half is the only place those two differ.
- **`0.5` / `1.5` / `2.5` separate the three half rules.** `HalfToEven`
  splits `1.5` and `2.5`, sending each to its even neighbour;
  `HalfAwayFromZero` and `HalfTowardZero` take opposite sides of every
  tie.
- **The negatives separate `Floor` / `Ceiling` from `Trunc` /
  `AwayFromZero`.** `Ceiling` and `AwayFromZero` agree on every positive
  value, and `Floor` and `Trunc` likewise; each pair parts company as
  soon as the value is negative and something is discarded. Without a
  negative row you cannot tell either pair apart.
- **`0.7` / `1.7` / `4.7` / `5.7` isolate `ZeroFiveUp`.** It is the only
  mode whose answer depends on the digit being *kept* rather than the
  digit being discarded. The discarded part is `0.7` in all four, yet
  `1.7` and `4.7` still truncate: the kept digits `1` and `4` are not
  pivots. `0.7` and `5.7` step away from zero because their kept digits
  `0` and `5` are.

Because this crate stores an integer coefficient, it has no signed zero:
a negative value that rounds to zero reads back as `0`, where the
specification would write `-0`.

### Why `ZeroFiveUp` exists

It is the legacy accountancy rule, and the one mode that survives a
second rounding intact. It reserves `0` and `5` as the only final digits
that can absorb a discarded remainder, so rounding again to one fewer
digit never meets a half-way tie that the *first* rounding manufactured —
the "round for reround" rule. That is what it buys: it avoids the upward
bias of `AwayFromZero` without ever truncating down onto a value that
already looks rounded.

## The `_with` pairs

```rust
use decimal_scaled::{D38s4, D38s2, RoundingMode};

let v: D38s4 = "1.2345".parse().unwrap();   // 1.2345

// Default mode (HalfToEven unless a `rounding-*` feature changes it):
let a: D38s2 = v.quantize::<2>();

// Explicit mode:
let b: D38s2 = v.quantize_with::<2>(RoundingMode::Floor);
let c: D38s2 = v.quantize_with::<2>(RoundingMode::Ceiling);
```

The same pairing applies to `to_int` / `to_int_with`,
`from_f64` / `from_f64_with`, and any other lossy method.

## `quantize` - changing the scale of a value

`quantize::<TARGET>()` converts a value to a different `SCALE` at the
*same width*:

- `TARGET == SCALE` - bit-identity.
- `TARGET > SCALE` - scale up: multiply by `10^(TARGET-SCALE)`.
  Lossless; panics on overflow.
- `TARGET < SCALE` - scale down: divide by `10^(SCALE-TARGET)`, applying
  the rounding mode to the discarded fractional digits.

```rust
use decimal_scaled::{D38s3, RoundingMode};

let v: D38s3 = "1.235".parse().unwrap();    // 1.235

// Scale down to 2 digits - the trailing `5` must be rounded.
let down  = v.quantize::<2>();                // HalfToEven -> 1.24
let trunc = v.quantize_with::<2>(RoundingMode::Trunc);   // -> 1.23
assert_eq!(down.to_bits(),  124i128);
assert_eq!(trunc.to_bits(), 123i128);

// Scale up is always lossless.
let up = v.quantize::<6>();
assert_eq!(up.to_bits(), 1_235_000);
```

> **Renamed in 0.5.1.** This operation was `rescale` in 0.5.0. The old
> spellings — `rescale` / `rescale_with`, and `DynDecimal::rescale_to` /
> `rescale_to_with` — still work as deprecated aliases and are removed
> in 0.6.0. `quantize` is the name the decimal arithmetic specification
> uses for setting the quantum; that specification marks `rescale` as
> its deprecated spelling.

## `requantize` - changing width and scale together

`quantize` holds the storage width fixed. `requantize` moves both axes
at once, to any width and any scale, in either direction. The target is
inferred from the binding, so a call site never spells a limb count:

```rust
use decimal_scaled::{D18, D38, RoundingMode};

let a = D18::<2>::try_from(7i64).unwrap();   // 7.00, narrow storage
let wide: D38<6> = a.requantize();           // 7.000000, wider storage
let back: D18<2> = wide.requantize_with(RoundingMode::Trunc);
```

The two steps are ordered by direction — widen before a scale-up, scale
down before narrowing — so a value the target width can hold never
overflows on the way there. Overflow panics, as with any other
operation.

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
| `rounding-away-from-zero` | `AwayFromZero` |
| `rounding-zero-five-up` | `ZeroFiveUp` |

```toml
[dependencies]
decimal-scaled = { version = "0.5", features = ["rounding-half-away-from-zero"] }
```

The features are mutually exclusive in intent. If more than one is
enabled, a fixed priority order in `src/support/rounding.rs` picks one
deterministically - but you should enable at most one.

The `_with` methods are unaffected by these features: they always honour
the mode you pass.
