# The width family

`decimal-scaled` ships thirteen storage widths. They all have the same
API shape - the same methods, the same const generic `SCALE`, the same
`Decimal` trait impl - and differ only in the size of the backing
integer, which sets the range and the maximum usable scale.

| Type | Storage | `MAX_SCALE` | Feature gate | Typical use |
|---|---|---|---|---|
| `D9`    | `i32`     | 9    | always on        | embedded / register-sized ledgers |
| `D18`   | `i64`     | 18   | always on        | interchange size; maps to SQL `DECIMAL(18, s)` |
| `D38`   | `i128`    | 38   | always on        | the financial standard; satoshi-grade at `SCALE = 12` |
| `D56`   | 192-bit   | 57   | `d56` / `wide`   | half-width between D38 and D76 |
| `D76`   | 256-bit   | 76   | `d76` / `wide`   | crypto ratios, statistical accumulation |
| `D114`  | 384-bit   | 115  | `d114` / `wide`  | half-width between D76 and D153 |
| `D153`  | 512-bit   | 153  | `d153` / `wide`  | wide-scientific / actuarial work |
| `D230`  | 768-bit   | 230  | `d230` / `wide`  | half-width between D153 and D307 |
| `D307`  | 1024-bit  | 307  | `d307` / `wide`  | deep arbitrary-precision determinism |
| `D461`  | 1536-bit  | 462  | `d461` / `x-wide`  | half-width between D307 and D615 |
| `D615`  | 2048-bit  | 616  | `d615` / `x-wide`  | extended-precision scientific / cryptography |
| `D923`  | 3072-bit  | 924  | `d923` / `xx-wide` | half-width between D615 and D1231 |
| `D1231` | 4096-bit  | 1232 | `d1231` / `xx-wide`| widest shipped tier; arbitrary-precision research |

`MAX_SCALE` is the largest `SCALE` for which `10^SCALE` fits the storage
integer. Using a larger `SCALE` is a compile-time error (the
`multiplier()` const overflows when the type is instantiated).

## Choosing a width

The range at a given scale is roughly `storage_max / 10^SCALE`. Pick the
*narrowest* width whose range covers your values at the scale you need -
narrower storage is faster and smaller.

- **`D9` / `D18`** - native integer storage, single-instruction add /
  sub / compare. `D18` at `SCALE = 9` covers ±9.2 × 10⁹ with
  nanosecond-grade fractional precision.
- **`D38`** - the default choice. At `SCALE = 12` the range is roughly
  ±1.7 × 10²⁶ - far beyond global-GDP-scale money.
- **`D56` / `D76` / `D114` / `D153` / `D230` / `D307`** - the *wide tier*.
  Backed by an in-tree hand-rolled wide-integer module; no external
  big-integer dependency. Opt-in via the matching feature
  (`d56`, `d76`, `d114`, `d153`, `d230`, `d307`, or umbrella `wide`).
  Half-width siblings (D56 / D114 / D230) let you size storage to your
  precision budget without paying for an unnecessary power-of-two jump.
  `D76` at `SCALE ≈ 70` still leaves ~10⁵ integer headroom.
- **`D461` / `D615`** - the *extra-wide tier*. Gated behind `x-wide`
  (or `d461` / `d615`). Use for scientific or cryptographic work that
  needs more than the ~307-digit budget of D307 but doesn't want the
  full xx-wide compile cost.
- **`D923` / `D1231`** - the *xx-wide tier*. Gated behind `xx-wide`
  (or `d923` / `d1231`). The widest shipped tier; transcendentals at
  D1231<1231> approach a second per call, so it's research-grade
  precision rather than a hot-path target.

## Scale aliases

Each width exposes curated `…s<N>` aliases for common scales:

```rust
use decimal_scaled::{D9s2, D18s9, D38s12};
# #[cfg(feature = "wide")]
use decimal_scaled::{D76s35, D153s75, D307s150};
```

`D38` has the full `D38s0` … `D38s38`. The other widths ship a
curated subset; the generic form `D18::<7>` is always available for any
in-range scale.

## The `Decimal` trait

Every width implements `Decimal`, the width-generic surface:

```rust
use decimal_scaled::Decimal;

fn sum_all<D: Decimal + core::ops::Add<Output = D>>(xs: &[D]) -> D {
    D::sum(xs.iter().copied())
}
```

`Decimal` carries the full uniform surface every width implements:

- **Type info**: `Storage`, `SCALE`, `MAX_SCALE`.
- **Constants**: `ZERO`, `ONE`, `MAX`, `MIN`, plus `multiplier()`.
- **Round-trip**: `from_bits` / `to_bits` / `scale`.
- **Arithmetic & bitwise operators** (via supertrait bounds):
  `+` `-` `*` `/` `%` `-` (unary) and the `*Assign` variants;
  `&` `|` `^` `!` `<<u32>` `>>u32`.
- **Sign**: `abs`, `signum`, `is_positive`, `is_negative`.
- **Integer methods**: `div_euclid`, `rem_euclid`, `div_floor`,
  `div_ceil`, `abs_diff`, `midpoint`, `mul_add`; the float-shape
  predicates `is_nan` / `is_infinite` / `is_finite`.
- **Pow**: `pow`, `powi`, and the four `*_pow` overflow variants.
- **Overflow variants**: every `checked_*` / `wrapping_*` /
  `saturating_*` / `overflowing_*` of `add` / `sub` / `mul` / `div` /
  `rem` / `neg`.
- **Integer conversion**: `from_i32`, `to_int`, `to_int_with`.
- **Float bridge** (gated on `std`): `from_f64`, `from_f64_with`,
  `to_f64`, `to_f32`.
- **Default reductions**: `is_zero`, `is_one`, `is_normal`, `sum`,
  `product`.

A few methods are deliberately not on the trait because their
signature varies per width or the trait can't represent them - see
the [trait reference](https://docs.rs/decimal-scaled/latest/decimal_scaled/trait.Decimal.html)
for the full out-of-scope list (`rescale<TARGET>` needs a const-generic
method param; `from_int` takes a per-width source integer;
transcendentals are feature-gated).

Reach for `Decimal` when writing code that must work across widths;
otherwise the concrete type is the canonical surface.

## Moving between widths

Widening is lossless and infallible (`From`); narrowing is fallible
(`TryFrom`). See [conversions](conversions.md) for the details.

```rust
# use decimal_scaled::{D18s2, D38s2};
let small: D18s2  = D18s2::from_bits(150);
let wide:  D38s2 = small.into();              // lossless widen
let back:  D18s2  = wide.try_into().unwrap();  // fallible narrow
```

## Wide-tier notes (`D56` … `D1231`)

- Enable per width (`d56`, `d76`, `d114`, `d153`, `d230`, `d307`,
  `d461`, `d615`, `d923`, `d1231`) or by umbrella (`wide` for
  D56–D230, `x-wide` adds D461–D615, `xx-wide` adds D923–D1231).
- Storage is the in-tree hand-rolled wide-integer module
  (`crate::wide_int`); there is no external big-integer dependency. The
  wide-int type never appears in your code - you work through
  `from_bits` / `to_bits` and the normal arithmetic.
- The full surface is shipped on the wide tier: cross-type `PartialEq`
  against every primitive integer and float, the 0.5-ULP-correctly-
  rounded strict transcendentals (with mode-aware `*_strict_with`
  siblings and AGM alternates `ln_strict_agm` / `exp_strict_agm`),
  arithmetic, conversions, formatting, rescaling, rounding, and the
  full overflow-variant family.
