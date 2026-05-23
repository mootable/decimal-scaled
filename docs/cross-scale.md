# Cross-scale operations

The crate's same-width same-`SCALE` operators (`+`, `-`, `*`, `/`,
`%`) are the fast path: they're `const fn` for the narrow tiers and
compile down to inlined integer arithmetic with no rescaling step.
For everything else - mixing `SCALE`s, mixing widths, comparing
across types without an explicit `.widen()` - the crate ships a
**cross-scale operations** surface in two layers.

| Layer | API shape | Toolchain | Notes |
|-------|-----------|-----------|-------|
| 1 (stable) | `D{N}<SCALE>::{op}_of(a, b)` on every width | stable 1.85+ | Target width and SCALE chosen by the caller via the receiver type. |
| 2 (nightly) | `decimal_scaled::cross::{op}(a, b)` free functions | nightly | Output type auto-inferred (`max(SCALE_a, SCALE_b)`) via `generic_const_exprs`. |

Both layers preserve the same **0.5 ULP correctness contract** as
the same-width same-SCALE operators - the result is exact at the
target type's last representable place. The only rounding step is
the rescale of inputs to the target SCALE; that step uses the
crate's [default rounding mode](rounding.md) (`HalfToEven` unless a
`rounding-*` feature overrides it) or a caller-provided
`RoundingMode` via the matching `*_with(mode)` sibling.

## Layer 1 - stable, explicit target

The receiver type names the destination width and SCALE; the
operands may be any width less-than-or-equal to it and any SCALE.

```rust
use decimal_scaled::{D18s4, D18s6, D38, D38s12};

let a = D18s4::from(5i64);          // D18<4>
let b = D18s6::from(7i64);          // D18<6>

// Target = D38<12>. Operands widen to Int<2>, rescale to SCALE=12,
// then multiply at the same width / scale.
let product: D38s12 = D38s12::mul_of(a, b);
assert_eq!(product, D38s12::from(35i64));
```

The same shape is available for every op:

```rust
use decimal_scaled::{D38, D38s6, D38s12};

let x: D38s6 = D38::<6>::from(20i64);
let y: D38s12 = D38::<12>::from(3i64);

let sum:  D38<10> = D38::<10>::add_of(x, y);   // 23
let diff: D38<10> = D38::<10>::sub_of(x, y);   // 17
let prod: D38<10> = D38::<10>::mul_of(x, y);   // 60
let quot: D38<10> = D38::<10>::div_of(x, y);   // 6 (rem 2)
let rem:  D38<10> = D38::<10>::rem_of(x, y);   // 2
```

### Explicit rounding

Each constructor has a `_with(mode)` sibling that takes an explicit
[`RoundingMode`](https://docs.rs/decimal-scaled/latest/decimal_scaled/enum.RoundingMode.html):

```rust
use decimal_scaled::{D38, Int, RoundingMode};

let a: D38<1> = D38::<1>::from_bits(Int::<2>::from(15i64));  // 1.5
let b: D38<0> = D38::<0>::from(1i64);                        // 1

let trunc = D38::<0>::mul_of_with(a, b, RoundingMode::Trunc);
assert_eq!(trunc.to_bits(), 1i128);

let away  = D38::<0>::mul_of_with(a, b, RoundingMode::HalfAwayFromZero);
assert_eq!(away.to_bits(), 2i128);
```

### Max / min / clamp

`max_of`, `min_of`, and `clamp_of` accept any-width any-SCALE
operands and rescale the winner into the destination type:

```rust
use decimal_scaled::{D18s4, D18s6, D18s9, D38s12};

let a = D18s6::from(3i64);
let b = D18s9::from(2i64);
let m: D38s12 = D38s12::max_of(a, b);
assert_eq!(m, D38s12::from(3i64));

let v  = D38s12::from(15i64);
let lo = D18s4::from(0i64);
let hi = D18s9::from(10i64);
let c: D38s12 = D38s12::clamp_of(v, lo, hi);
assert_eq!(c, D38s12::from(10i64));
```

### Comparators

`cmp_of` and its boolean friends (`eq_of`, `ne_of`, `lt_of`,
`le_of`, `gt_of`, `ge_of`) compare a decimal against any
narrower-or-equal-width value at any SCALE. Both sides UP-rescale to
the higher SCALE (lossless) before the storage `Ord` is invoked:

```rust
use decimal_scaled::{D18s6, D38s12};

let a = D38s12::from(5i64);
let b = D18s6::from(5i64);

assert!(a.eq_of(b));
assert_eq!(a.cmp_of(b), std::cmp::Ordering::Equal);
```

### Cross-width `==` / `<` operator overloads (same SCALE)

For the common case of comparing across widths at the **same**
`SCALE`, the operator overloads work directly:

```rust
use decimal_scaled::{D18, D38};

let small: D18<12> = D18::<12>::from(5i64);
let big:   D38<12> = D38::<12>::from(5i64);
assert!(small == big);   // works without .widen()
assert!(big >= small);
```

(Cross-SCALE operator overloads are nightly-only - they require
`generic_const_exprs` to compute the common-scale type at the impl
site.)

## Layer 2 - nightly, auto-inferred output

With the `cross-scale-ops` feature enabled (nightly required), a
[`cross`](https://docs.rs/decimal-scaled/latest/decimal_scaled/cross/index.html)
free-function module infers the output `SCALE` from the operands:

```toml
# Cargo.toml
[dependencies]
decimal-scaled = { version = "0.5", features = ["cross-scale-ops"] }
```

```rust
#![feature(generic_const_exprs)]
use decimal_scaled::{D38, cross};

let a: D38<6>  = D38::<6>::from(7i64);
let b: D38<12> = D38::<12>::from(11i64);
let c = cross::mul(a, b);     // type: D38<12>, value: 77
```

The output SCALE is `max(SCALE_a, SCALE_b)` and the output width is
the operands' shared width (cross-width auto-inference would need a
type-level `WiderOf` chain on top of `generic_const_exprs`; in
practice it stresses the incomplete-feature corners enough that the
stable Layer-1 form is the recommended path for cross-width work).

Surface: `cross::mul`, `cross::add`, `cross::sub`, `cross::div`,
`cross::rem`, plus the `cross::max_const(a, b) -> u32` `const fn`
that backs the generic clauses (re-exported so user code can build
on it).

## 0.5 ULP guarantee

Every cross-scale op runs as: widen both operands → rescale to the
common precision → execute the same-width same-SCALE operator. The
rescale step is the *only* place rounding occurs, and it follows
exactly the same rule as the standalone `rescale_with(mode)`. The
arithmetic step inherits the same-width operator's 0.5 ULP contract.

For `max_of` / `min_of` / `cmp_of` the comparison runs at the higher
of the two operand SCALEs - both sides UP-rescale, which is exact -
so the comparison itself never loses precision, only the final
rescale to the destination type does (and only when the destination
SCALE is narrower than the operand SCALE).
