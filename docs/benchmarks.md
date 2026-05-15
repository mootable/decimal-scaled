# Benchmarks

Head-to-head measurements of `decimal-scaled` against the wider Rust
numeric ecosystem, and of its own lossy vs strict transcendental
variants.

The benchmark suites live in [`benches/`](../benches/) and run under
[criterion](https://docs.rs/criterion/). The baseline crates
(`bnum`, `ruint`, `rust_decimal`, `fixed`) are **dev-dependencies
only** — they are never compiled into a normal build.

```sh
cargo bench --features wide --bench wide_int_backends
cargo bench --features wide --bench decimal_backends
```

> The figures below are indicative — absolute timings are
> machine-dependent. What matters is the *ratios* between
> implementations on the same machine, in the same run. Operands are
> `black_box`-ed to defeat constant folding.

## Time units

The tables below mix nanoseconds and microseconds. For reference:

| Symbol | Unit        | Seconds | Relative to `ns` |
|--------|-------------|---------|------------------|
| `s`    | second      | 10⁰ s   | ×1 000 000 000   |
| `ms`   | millisecond | 10⁻³ s  | ×1 000 000       |
| `µs`   | microsecond | 10⁻⁶ s  | ×1 000           |
| `ns`   | nanosecond  | 10⁻⁹ s  | ×1               |
| `ps`   | picosecond  | 10⁻¹² s | ×0.001           |

So `1 µs` = `1 000 ns`, and a `27 µs` strict `ln` is `27 000 ns` —
about 700× a `38 ns` lossy `ln`.

## 1. Wide-integer backends

Raw 256-bit integer arithmetic: the in-tree hand-rolled `Int256`
against the `bnum` and `ruint` big-integer crates.

| op  | `bnum` I256 | `ruint` U256 | **`Int256` (this crate)** |
|-----|-------------|--------------|---------------------------|
| add | 1.98 ns     | 6.38 ns      | **1.87 ns**               |
| sub | 1.96 ns     | 5.96 ns      | **1.89 ns**               |
| mul | 4.20 ns     | 3.58 ns      | 15.5 ns                   |
| div | 61.8 ns     | 5.47 ns      | 16.1 ns                   |

Add and sub beat both libraries. Multiply is ~4× off the dedicated
big-integer crates — the schoolbook limb multiply is not yet tuned for
the small-operand case. Divide uses hardware fast paths (one native
divide when both operands fit a 128-bit word; base-2⁶⁴ long division
when the divisor fits 64 bits) and lands between the two baselines.

## 2. Decimal arithmetic

Fixed-point arithmetic at `SCALE = 12`: the crate's `D128` (128-bit
primitive storage) and `D256` (256-bit hand-rolled-integer storage)
against a `bnum`-backed `D256`, `rust_decimal`, and `fixed`'s binary
fixed-point `I64F64`.

| op  | **D128** | **D256** | `bnum`-D256 | `rust_decimal` | `fixed` I64F64 |
|-----|----------|----------|-------------|----------------|----------------|
| add | 0.90 ns  | 1.69 ns  | 1.84 ns     | 5.31 ns        | 0.88 ns        |
| sub | 0.99 ns  | 1.89 ns  | 1.86 ns     | 5.60 ns        | 0.83 ns        |
| mul | 10.9 ns  | 209 ns   | 268 ns      | 3.05 ns        | 1.50 ns        |
| div | 9.2 ns   | 203 ns   | 272 ns      | 17.2 ns        | 23.4 ns        |

`D128` add/sub are a single `i128` instruction; mul/div carry a
256-bit widening step and land around 10 ns. The wide `D256` tier
keeps add/sub almost free (~1.8 ns) but its mul/div pay for a
256-bit hand-rolled-integer divide — roughly **20× the `D128` cost**.
That is the price of exact base-10 arithmetic past `i128`'s range;
`D256` still edges out the `bnum`-backed equivalent. `rust_decimal`
and `fixed` have very fast multiplies (96-bit-mantissa fast paths and
plain binary shifts respectively) but neither offers exact base-10
arithmetic across a 256-bit range.

## 3. Transcendentals — lossy vs strict vs `rust_decimal`

`ln` / `exp` / `sqrt` / `sin` on `D128<9>`, comparing the crate's two
variants against `rust_decimal`'s `MathematicalOps`:

- **lossy** — the `f64`-bridge form: convert to `f64`, call the
  platform intrinsic, convert back. Fast, but not platform-independent
  and not correctly rounded.
- **strict** — the integer-only form: range reduction plus an integer
  series evaluated in a guard-digit intermediate. Platform-independent,
  `no_std`-compatible, and **correctly rounded to within 0.5
  [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place)** of the
  exact result (the [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
  round-to-nearest contract).

Each row is shown in a single unit, chosen from the row's median
value, so the columns can be read off directly.

| fn   | D128 lossy  | D128 strict | `rust_decimal` |
|------|-------------|-------------|----------------|
| ln   | 0.038 µs    | 27.0 µs     | 3.56 µs        |
| exp  | 0.042 µs    | 39.4 µs     | 2.13 µs        |
| sqrt | 32 ns       | **31.6 ns** | 665 ns         |
| sin  | 0.040 µs    | 28.4 µs     | 3.96 µs        |

For the series functions the strict variant costs roughly **700× the
`f64` bridge** — that is the price of a deterministic, correctly-rounded
integer evaluation. `rust_decimal`'s software transcendentals land in
between (~2–4 µs) but are **not** correctly rounded to 0.5 ULP, and
`fixed` has no transcendentals at all.

`sqrt_strict` is the exception: at ~32 ns it *ties* the lossy form,
because it is algebraic — an exact integer square root with a single
round-to-nearest step, no series.

## What the strict variants buy

The `*_strict` transcendentals are a capability the baseline crates do
not offer:

- **Correctly rounded to 0.5 ULP** — the result is the exact value
  rounded to the type's last representable place, the IEEE-754
  round-to-nearest guarantee. `f64`-bridge results inherit `f64`'s
  ~15-digit precision ceiling; `rust_decimal`'s software forms are
  accurate but not correctly rounded to the last place.
- **Platform-independent** — identical bit patterns on every target,
  with no dependence on the platform math library.
- **`no_std`** — integer-only, so they compile and run without `std`.

The cost is throughput. For latency-sensitive code that does not need
determinism or last-place accuracy, the lossy `f64` bridge is the
better default; for finance, reproducible computation, or `no_std`
targets, the strict path is the reason the crate exists.
