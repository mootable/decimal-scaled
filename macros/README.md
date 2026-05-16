# `decimal_scaled_macros`

Compile-time construction macros for the [`decimal-scaled`](..)
type family. The crate exists so users can write decimal literals
the way they read them — `d38!(19.99, scale 2)` — instead of
hand-computing the scaled `i128` storage value.

This is the proc-macro implementation crate. The parent crate
re-exports the macros under `decimal_scaled` when the `macros`
feature is enabled. Users normally depend on the parent crate
only.

```toml
[dependencies]
decimal-scaled = { version = "0.2.4", features = ["macros"] }
```

```rust
use decimal_scaled::d38;
let cost = d38!(19.99, scale 2);   // D38<2>::from_bits(1999)
```

---

## Per-width entry points

| macro | target | storage | `MAX_SCALE` | feature gate |
|---|---|---|---|---|
| `d9!` | `D9<SCALE>` | `i32` | 9 | always |
| `d18!` | `D18<SCALE>` | `i64` | 18 | always |
| `d38!` | `D38<SCALE>` | `i128` | 38 | always |
| `d76!` | `D76<SCALE>` | `Int256` | 76 | `d76` / `wide` |
| `d153!` | `D153<SCALE>` | `Int512` | 153 | `d153` / `wide` |
| `d307!` | `D307<SCALE>` | `Int1024` | 307 | `d307` / `x-wide` |

Each entry point accepts the same argument grammar.

---

## Argument grammar

```
dN!( value [, qualifier]* )

value     := decimal-literal | radix-literal | expression
qualifier := scale N
           | radix N
           | rounded
```

Qualifiers are unordered. Each may appear at most once.

### Value

- **Decimal literal**: `1.23`, `-1.23`, `+1.23`, `1`, `1.0`, `0`,
  `0.5`, `1_234.567_89`. Scientific notation `1.5e3`, `1.5e-3`,
  `6.022e23`, `1e-9` also accepted.
- **Radix-prefixed integer**: `0x...`, `0o...`, `0b...`. The parsed
  magnitude is the storage bits *directly* — the target scale
  only labels the resulting type, no scale-shift is applied.
  Equivalent to `radix 16 / 8 / 2` with the appropriate base.
- **Inline expression**: `my_var`, `a + b * 2`. Scale is mandatory
  for the expression form (`, scale N`).

### `scale N`

Sets the target scale explicitly.

- `N` is a non-negative integer literal.
- `N ≤ MAX_SCALE` for the entry point's width, else a compile
  error pointing at `N`.
- Absent + decimal literal → scale is inferred from the literal's
  fractional digit count and any scientific-notation exponent
  (`inferred = max(0, fractional_digits − exponent)`).
- Absent + radix-prefixed literal → scale defaults to 0.
- Absent + expression → compile error: scale is required.

### `radix N`

Reinterprets the digit characters in `value` under base `N`. `N`
must be one of `2`, `8`, `10`, `16`. Default is `10`. If a Rust
prefix (`0x` / `0o` / `0b`) is also present, the prefix's implied
radix and the explicit qualifier must agree.

Fractional non-decimal literals are supported. The macro splits
the value at the `.`, treats the left side as the integer-portion
digits and the right side as the fractional-portion digits, parses
both halves under the given radix, and concatenates them into the
storage bits.

The radix value-position grammar is:

```
radix-value := [ + | - ]  ( IDENT-or-INT . IDENT-or-INT | IDENT-or-INT )
```

where the trailing `IDENT-or-INT` alternative is the
non-fractional (integer-only) case. `IDENT-or-INT` means either
a Rust identifier or an integer literal token — important
because Rust's tokeniser splits a value like `1.A3` into three
tokens (`1`, `.`, `A3`) and a value like `FF.AA` into another
three (`FF`, `.`, `AA`), neither of which is a valid Rust
literal; the macro reassembles them itself.

The target scale must be supplied explicitly with `, scale N` —
nothing about the source string tells us what target scale you
want, since the parsed magnitude IS the storage bits.

```rust
d38!(1.A3,   radix 16, scale 2)   // → D38<2>::from_bits(0x1A3)   == 419
d38!(11.0110, radix 2, scale 4)   // → D38<4>::from_bits(0b110110) == 54
d38!(17.3,    radix 8, scale 2)   // → D38<2>::from_bits(0o173)    == 123
d38!(FF.AA,  radix 16, scale 2)   // → D38<2>::from_bits(0xFFAA)   == 65450
d38!(BEEF,   radix 16)            // → D38<0>::from_bits(0xBEEF)   == 48879
d38!(-1.A3,  radix 16, scale 2)   // → D38<2>::from_bits(-0x1A3)   == -419
```

The `radix` qualifier requires a literal value — it doesn't
combine with the inline-expression form. (`d38!(my_var, radix
16, scale 2)` is a compile error: `radix qualifier is only
valid with a literal value`.) Radix is a property of the source
*digit string*, and an expression doesn't have one — it's
already a typed integer at runtime.

### `rounded`

Opts into half-to-even rounding when the literal carries more
fractional digits than the target scale. Without `rounded`, a
lossy compile-time conversion is a compile error pointing at the
literal.

```rust
d38!(1.234_567, scale 2)            // ❌ compile error
d38!(1.234_567, scale 2, rounded)   // ✅ → 1.23 (half-to-even)
```

Scale-up (target > literal scale) is always exact (trailing
zeros) and needs no opt-in.

`rounded` has no effect on radix-prefixed literals — they don't
do scale arithmetic.

---

## Auto-scale inference for decimals

| literal | inferred scale | rationale |
|---|---|---|
| `123` | 0 | no fractional part |
| `123.0` | 1 | trailing zero counts |
| `1.23` | 2 | two fractional digits |
| `1.230` | 3 | trailing zero preserved |
| `0.001` | 3 | leading zeros after the point count |
| `-1.23` | 2 | sign doesn't affect scale |
| `1_234.567_89` | 5 | underscores stripped, not counted |
| `1.5e3` | 0 | `max(0, 1 − 3)` |
| `1.5e-3` | 4 | `max(0, 1 − (−3))` |
| `6.022e23` | 0 | `max(0, 3 − 23)` |
| `1e-9` | 9 | `max(0, 0 − (−9))` |

If the inferred scale exceeds the entry point's `MAX_SCALE`, the
macro emits a compile error pointing at the literal — switch to
an explicit `scale N` (with `rounded` if precision is lost), or
use a wider entry point.

---

## Examples

```rust
use decimal_scaled::{d9, d18, d38, D9s2, D18s12, D38s2, D38s12, D38};

// Auto-scale inference.
let a = d38!(1.23);                  // D38<2>
let b = d38!(1.23000);               // D38<5>
let c = d18!(1.5e-9);                // D18<10>... wait, 10 > D18::MAX_SCALE
//                                  → compile error: use d38! or scale up.

// Explicit scale + rounding.
let cents = d38!(1.234_567, scale 2, rounded);  // 1.23
let micros: D38<6> = d38!(2.5, scale 6);        // 2.500000 exactly

// Radix-prefix literal — storage bits = parsed magnitude.
let raw_hex: D38<0> = d38!(0xFF);    // 255 at scale 0
let raw_oct: D38<0> = d38!(0o755);   // 493
let raw_bin: D38<0> = d38!(0b1010);  // 10
let labelled: D38<2> = d38!(0x7B, scale 2);
                       // 123 at scale 2 → Display "1.23"

// Inline expression form — scale mandatory.
let basis: i128 = 12_345;
let with_scale = d38!(basis, scale 4);  // basis × 10^4 at runtime

// Per-scale wrappers (curated subset; pre-bake `scale N`).
let pi  = d38s12!(3.141_592_653_590); // == d38!(…, scale 12)
let cents_alias = d9s2!(0.50);        // == d9!(0.50, scale 2)
```

Wide-tier (requires `wide` / `x-wide`):

```rust
#[cfg(feature = "wide")]
{
    use decimal_scaled::{d76, d153, D76s35, D153s75};
    let e: D76s35 = d76!(2.718_281_828_459_045_235_360_287_471_352_662_50);
    let half: D153s75 = d153!(0.5, scale 75);
}
```

---

## Curated per-scale wrappers

These pre-bake `, scale N` for the most common scale at each
width — the long tail remains reachable via the explicit
`, scale N` qualifier.

| width | wrappers |
|---|---|
| D9 | `d9s0!`, `d9s2!`, `d9s4!`, `d9s6!`, `d9s9!` |
| D18 | `d18s0!`, `d18s2!`, `d18s4!`, `d18s6!`, `d18s9!`, `d18s12!`, `d18s18!` |
| D38 | `d38s0!`, `d38s2!`, `d38s4!`, `d38s6!`, `d38s8!`, `d38s9!`, `d38s12!`, `d38s15!`, `d38s18!`, `d38s24!`, `d38s35!`, `d38s38!` |
| D76 | `d76s0!`, `d76s2!`, `d76s6!`, `d76s12!`, `d76s18!`, `d76s35!`, `d76s50!`, `d76s76!` |
| D153 | `d153s0!`, `d153s35!`, `d153s75!`, `d153s150!`, `d153s153!` |
| D307 | `d307s0!`, `d307s35!`, `d307s150!`, `d307s300!`, `d307s307!` |

Each wrapper forwards every other qualifier unchanged:

```rust
d38s2!(1.234_567, rounded)
// == d38!(1.234_567, scale 2, rounded)
```

---

## Compile-time errors

| cause | sketched message |
|---|---|
| Scale exceeds width max | `scale 40 exceeds max for D38 (max = 38)` |
| Scale required for expression | `scale must be specified for an expression value: d38!(expr, scale N)` |
| Lossy literal without `rounded` | `literal 1.234567 has 6 fractional digits, target scale 2 would lose precision; pass rounded to opt into half-to-even rounding` |
| Bits overflow storage | `scaled value 1234567890000000000000000 overflows D9's storage (i32)` |
| Bare leading/trailing dot | `decimal literals require a digit on each side of the dot (write 0.5 not .5)` |
| Inferred scale exceeds max | `1e-50 implies scale 50, which exceeds D38::MAX_SCALE (38); use an explicit scale or a wider entry point` |
| Type suffix on literal | `type suffixes (e.g. _i64, _f32) are not accepted in decimal-scaled literals` |
| Unknown qualifier | `unknown qualifier 'precision'; expected one of: scale, radix, rounded` |
| Radix disagreement | `radix qualifier (10) disagrees with literal prefix (radix 16)` |
| Bad radix value | `radix must be one of 2, 8, 10, 16 (got 7)` |
| Fractional radix literal | `fractional non-decimal literals are not supported (use an explicit scale N with an integer-only digit string instead)` |

---

## Generated code

For literal input, the macro evaluates the bit pattern at
proc-macro time:

- **Narrow tiers** (D9 / D18 / D38) emit a typed integer literal
  directly:
  ```rust
  d38!(1.23)            // → ::decimal_scaled::D38::<2>::from_bits(123_i128)
  d18!(1.5, scale 6)    // → ::decimal_scaled::D18::<6>::from_bits(1_500_000_i64)
  d9!(0x7F)             // → ::decimal_scaled::D9::<0>::from_bits(127_i32)
  ```
- **Wide tiers** (D76 / D153 / D307) materialise the bits via the
  storage type's `from_str_radix` (a `const fn`):
  ```rust
  d76!(1.23)
  // → ::decimal_scaled::D76::<2>::from_bits({
  //       const BITS: ::decimal_scaled::Int256 =
  //           match Int256::from_str_radix("123", 10) {
  //               Ok(v) => v,
  //               Err(_) => panic!("d76! bits parse failed"),
  //           };
  //       BITS
  //   })
  ```

For inline expressions the macro emits a small runtime
`checked_mul`:

```rust
d38!(my_i128, scale 4)
// → ::decimal_scaled::D38::<4>::from_bits({
//      let _v: i128 = (my_i128);
//      _v.checked_mul(10_000_i128).expect("d38! overflow …")
//   })
```

The runtime cost is a single multiply plus an overflow check;
the check fires on `i128::MIN * 10^k`-style edges and panics with
a clear message.

---

## Const-context usage

Because the macro emits `D::<SCALE>::from_bits(integer_literal)`
or `from_bits({ const BITS: ... ; BITS })` and `from_bits` is
`const fn`, the literal form is usable in `const` items at every
width:

```rust
use decimal_scaled::{d38, D38};
const RATE: D38<4> = d38!(0.0535);                 // ✅

#[cfg(feature = "wide")]
{
    use decimal_scaled::{d76, D76};
    const PI: D76<35> = d76!(3.14159265358979323846264338327950288);  // ✅
}
```

The inline-expression form is *not* usable in `const` items
because its emitted body contains a runtime `checked_mul`. Use
the literal form when you need a const.

---

## Test corpus

`tests/macros.rs` (in the parent crate) exercises every entry
point, every literal shape, every error path, the radix-prefix
forms, the inline-expression form, and the curated per-scale
wrappers. Run under `cargo test --features "wide x-wide macros"`.
