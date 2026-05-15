# `decimal_scaled_macros`

Compile-time construction macros for the [`decimal-scaled`](..)
type family. The crate exists so users can write decimal literals
the way they read them — `d38!(19.99, scale 2)` — instead of
hand-computing the scaled `i128` storage value.

This is the proc-macro implementation crate. The parent crate
re-exports `d38!` under `decimal_scaled::d38` when the `macros`
feature is enabled. Users normally depend on the parent crate
only.

```toml
[dependencies]
decimal-scaled = { version = "0.1.1", features = ["macros"] }
```

```rust
use decimal_scaled::d38;
let cost = d38!(19.99, scale 2);   // D38<2>::from_bits(1999)
```

---

## What ships today

| macro | target | semantics |
|---|---|---|
| `d38!(literal)` | `D38<inferred>` | scale inferred from the literal's fractional-digit count |
| `d38!(literal, scale N)` | `D38<N>` | explicit target scale; `N ≤ 38` |
| `d38!(literal, rounded)` | `D38<inferred>` | opt into half-to-even rounding when the literal has more fractional digits than the target scale |
| `d38!(literal, scale N, rounded)` | `D38<N>` | both |
| `d38!(expr, scale N)` | `D38<N>` | inline expression form; the expression is scaled by `10^N` at runtime |

Supported literal shapes:

- Plain decimals: `1.23`, `-1.23`, `+1.23`, `1`, `1.0`, `0`, `0.5`.
- Underscore digit separators: `1_234.567_89`.
- Scientific notation: `1.5e3`, `1.5e-3`, `6.022e23`, `1e-9`.
- Inline expressions: `d38!(my_var, scale 4)`,
  `d38!(10 * 12 + 3, scale 0)`. `scale N` is mandatory when the
  value is anything other than a numeric literal.

Examples:

```rust
use decimal_scaled::{d38, D38, D38s12};

// Auto-scale inference.
let a: D38<2>  = d38!(1.23);          // scale = 2 from the literal
let b: D38<5>  = d38!(1.23000);       // scale = 5 (trailing zeros count)
let c: D38<10> = d38!(1.5e-9);        // mantissa 1.5 (scale 1) − exponent (−9) = 10

// Explicit scale and rounding.
let d: D38<2>  = d38!(1.234_567, scale 2, rounded);  // 1.23 (half-to-even)
let e: D38s12  = d38!(0.5, scale 12);                // 0.500_000_000_000

// Inline expression — scale mandatory.
let mul_basis: i128 = 12_345;
let f: D38<4> = d38!(mul_basis, scale 4);   // f.to_bits() == 12_345 * 10^4
```

---

## Semantic model

`d38!(value, …)` produces `D38::<SCALE>::from_bits(bits)` where:

- `SCALE` is the target scale — either explicit (`scale N`), or
  inferred from the literal as written.
- `bits` is an `i128` literal equal to the exact mathematical
  value of `value × 10^SCALE`.

The macro is **exact by default**. Lossy compile-time conversions
(literal precision exceeding the target scale) are a compile error
unless the user opts in with `rounded`. Scale-up by appending
trailing zeros is always exact and needs no opt-in.

Auto-scale inference rules (per the literal as written):

| literal | inferred scale | note |
|---|---|---|
| `123` | 0 | no fractional part |
| `123.0` | 1 | trailing zero counts |
| `1.23` | 2 | two fractional digits |
| `1.230` | 3 | trailing zero preserved |
| `0.001` | 3 | leading zeros after the point count |
| `-1.23` | 2 | sign doesn't affect scale |
| `1_234.567_89` | 5 | underscores stripped, not counted |

For scientific notation `mantissa e exponent`:

```
inferred_scale = max(0, mantissa_fractional_digits − exponent)
```

| literal | mantissa scale | exponent | inferred scale | decimal |
|---|---|---|---|---|
| `1e6` | 0 | +6 | 0 | `1_000_000` |
| `1.5e3` | 1 | +3 | 0 | `1500` |
| `1.500e2` | 3 | +2 | 1 | `150.0` |
| `1.5e-3` | 1 | −3 | 4 | `0.0015` |
| `1e-9` | 0 | −9 | 9 | `0.000_000_001` |
| `-2.5e-2` | 1 | −2 | 3 | `-0.025` |
| `6.022e23` | 3 | +23 | 0 | `602_200_000_000_000_000_000_000` |

If the inferred scale exceeds `D38::MAX_SCALE = 38`, the macro
emits a compile error pointing at the literal. Use an explicit
`scale N` (with `rounded` if precision is lost) to override.

---

## Compile-time errors

The macro is eager about producing clear errors. All errors carry
the offending token's span.

| cause | sketched message |
|---|---|
| Scale exceeds the width's max | `scale 40 exceeds max for D38 (max = 38)` |
| Scale required for expression | `scale must be specified for an expression value: d38!(expr, scale N)` |
| Lossy literal without `rounded` | `literal 1.234567 has 6 fractional digits, target scale 2 would lose precision; pass rounded to opt into rounding` |
| Bits overflow storage | `value 1.23e30 at scale 12 overflows D38's i128 storage` |
| Bare leading/trailing dot | `decimal literals require a digit on each side of the dot (write 0.5 not .5)` |
| Inferred scale exceeds max | `1e-50 implies scale 50, which exceeds D38::MAX_SCALE (38); use an explicit scale or a wider type` |
| Type suffix on literal | `type suffixes (e.g. _i64, _f32) are not accepted in d38! literals` |
| Unknown qualifier | `unknown qualifier 'precision'; expected one of: scale, rounded` |

---

## Generated code

For literal input, the macro evaluates the bit pattern at
proc-macro time and emits an integer literal directly:

```rust
d38!(1.23)            // → D38::<2>::from_bits(123_i128)
d38!(1.23, scale 5)   // → D38::<5>::from_bits(123_000_i128)
d38!(1.5e-3)          // → D38::<4>::from_bits(15_i128)
```

The emitted token tree never carries the original literal — only
the computed bits — so debug builds see no parser cost.

For inline expression input, the macro emits a small `const`
multiply with an overflow check:

```rust
d38!(my_i128, scale 4)
// → D38::<4>::from_bits({
//      const MULT: i128 = 10_i128.pow(4);
//      let _v: i128 = my_i128;       // type-check anchor
//      _v.checked_mul(MULT).expect("d38! overflow: expression * 10^SCALE exceeds i128 range")
//   })
```

The runtime cost is a single multiply plus an overflow check;
the check fires on `i128::MIN * 10^k`-style edges and panics with
a clear message.

---

## Const-context usage

Because the macro emits `D38::<SCALE>::from_bits(integer_literal)`
and `from_bits` is `const fn`, the literal form is usable in
`const` items:

```rust
use decimal_scaled::{d38, D38};
const RATE: D38<4> = d38!(0.0535);   // ✅ scale 4 inferred
```

The inline-expression form is *not* usable in `const` items because
its emitted body contains a runtime `checked_mul`. Use the literal
form when you need a const.

---

## Not yet shipped

These shapes are part of the original design but not implemented:

- **`radix N` qualifier** for non-decimal literals
  (`d38!(0x7B, radix 16, scale 2)`). Hex/oct/bin Rust-prefixed
  literals (`0x...`, `0o...`, `0b...`) are also not accepted.
- **Per-scale wrappers** (`d38s12!`, `d38s2!`, etc.). The pre-baked
  scale form was deferred — the `, scale N` qualifier is the
  current vehicle.
- **Per-width entry points** for the other tiers (`d9!`, `d18!`,
  `d76!`, `d153!`, `d307!`). Today only `d38!` is exported. The
  decimal-digit naming convention is already in place; the macro
  family is one of the pre-1.0 follow-ups (see
  `research/FOLLOWUPS.md`).

If you need one of the unshipped shapes, parse the value from a
string with `FromStr` or construct via `from_bits` directly.

---

## Test corpus

`tests/macros.rs` (in the parent crate) exercises every literal
shape, every error path, and the inline-expression form. The
tests run under `cargo test --features macros`.
