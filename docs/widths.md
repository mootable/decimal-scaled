# The width family

`decimal-scaled` ships <!-- BEGIN GENERATED:widths:count -->thirteen storage widths<!-- END GENERATED:widths:count -->. They all share the same
API shape — the same methods, the same const generic `SCALE`, the same
`Decimal` trait impl — and differ only in the size of the backing
integer, which sets the range and the maximum usable scale.

The number in each `D<N>` type name is **the maximum number of
all-nines base-10 digits the storage can hold**, not the bit-width of
the underlying integer. `MAX_SCALE = N − 1` on every width, which
guarantees at least one integer digit at every legal `SCALE` (and so
every representable value has a sign-and-integer-part to render). The
overall scaffold:

<!-- BEGIN GENERATED:widths:table -->
| Type | Constructor macro | Underlying signed integer | `MAX_SCALE` | Max value at SCALE 0 | Required feature |
|---|---|---|---|---|---|
| `D9<S>` | `d9!` | `i32` (32 bits) | 8 | ±2.1 × 10⁹ | always available |
| `D18<S>` | `d18!` | `i64` (64 bits) | 17 | ±9.2 × 10¹⁸ | always available |
| `D38<S>` | `d38!` | `i128` (128 bits) | 37 | ±1.7 × 10³⁸ | always available |
| `D57<S>` | `d57!` | `Int192` (192 bits) | 56 | ±3.1 × 10⁵⁷ | `d57` / `wide` |
| `D76<S>` | `d76!` | `Int256` (256 bits) | 75 | ±5.8 × 10⁷⁶ | `d76` / `wide` |
| `D115<S>` | `d115!` | `Int384` (384 bits) | 114 | ±2.0 × 10¹¹⁵ | `d115` / `wide` |
| `D153<S>` | `d153!` | `Int512` (512 bits) | 152 | ±6.7 × 10¹⁵³ | `d153` / `wide` |
| `D230<S>` | `d230!` | `Int768` (768 bits) | 229 | ±7.7 × 10²³⁰ | `d230` / `wide` |
| `D307<S>` | `d307!` | `Int1024` (1024 bits) | 306 | ±9.0 × 10³⁰⁷ | `d307` / `wide` |
| `D462<S>` | `d462!` | `Int1536` (1536 bits) | 461 | ±1.0 × 10⁴⁶² | `d462` / `x-wide` |
| `D616<S>` | `d616!` | `Int2048` (2048 bits) | 615 | ±1.6 × 10⁶¹⁶ | `d616` / `x-wide` |
| `D924<S>` | `d924!` | `Int3072` (3072 bits) | 923 | ±2.3 × 10⁹²⁴ | `d924` / `xx-wide` |
| `D1232<S>` | `d1232!` | `Int4096` (4096 bits) | 1231 | ±2.7 × 10¹²³² | `d1232` / `xx-wide` |
<!-- END GENERATED:widths:table -->

The half-width tiers (`D57`, `D115`, `D230`, `D462`, `D924`) fill in
the storage-cost gap between each pair of power-of-two widths, so you
only pay for the precision you actually need. The umbrellas: `wide`
enables D57–D307, `x-wide` adds D462 / D616, `xx-wide` adds D924 /
D1232.

`MAX_SCALE` is the largest `SCALE` for which every `MAX_SCALE`-digit
decimal value (`±999…9`) fits the signed storage and at least one
integer digit remains representable. Using a larger `SCALE` is a
compile-time error.

## Choosing a width

The range at a given scale is roughly `storage_max / 10^SCALE`. Pick
the *narrowest* width whose range covers your values at the scale you
need — narrower storage is faster and smaller.

- **`D9` / `D18`** — native integer storage, single-instruction add /
  sub / compare. `D18` at `SCALE = 9` covers ±9.2 × 10⁹ with
  nanosecond-grade fractional precision.
- **`D38`** — the default choice. At `SCALE = 12` the range is roughly
  ±1.7 × 10²⁶ — far beyond global-GDP-scale money.
- **`D57` / `D76` / `D115` / `D153` / `D230` / `D307`** — the *wide
  tier*. Backed by an in-tree hand-rolled wide-integer module; no
  external big-integer dependency. Opt-in via the matching feature
  (`d57`, `d76`, `d115`, `d153`, `d230`, `d307`, or umbrella `wide`).
  Half-width siblings (D57 / D115 / D230) let you size storage to your
  precision budget without paying for an unnecessary power-of-two
  jump. `D76` at `SCALE ≈ 70` still leaves ~10⁵ integer headroom.
- **`D462` / `D616`** — the *extra-wide tier*. Gated behind `x-wide`
  (or `d462` / `d616`). Use for scientific or cryptographic work that
  needs more than the ~307-digit budget of D307 but doesn't want the
  full xx-wide compile cost.
- **`D924` / `D1232`** — the *xx-wide tier*. Gated behind `xx-wide`
  (or `d924` / `d1232`). The widest shipped tier; transcendentals at
  `D1232<1231>` approach a second per call, so it's research-grade
  precision rather than a hot-path target.

## Scale aliases

Each width exposes curated `…s<N>` aliases for common scales:

```rust
use decimal_scaled::{D9s2, D18s9, D38s12};
# #[cfg(feature = "wide")]
use decimal_scaled::{D76s35, D153s75, D307s150};
```

`D38` has the full `D38s0` … `D38s37`. The other widths ship a
curated subset; the generic form `D18::<7>` is always available for
any in-range scale. No alias exists at `SCALE == name` (i.e. there is
no `D38s38`, `D76s76`, …); those scales are no longer legal — use the
`name − 1` ceiling (`D38s37`, `D76s75`, `D57s56`, …) or the
const-generic form directly.

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
signature varies per width or the trait can't represent them — see
the [trait reference](https://docs.rs/decimal-scaled/latest/decimal_scaled/trait.Decimal.html)
for the full out-of-scope list (`rescale<TARGET>` needs a
const-generic method param; `from_int` takes a per-width source
integer; transcendentals are feature-gated).

Reach for `Decimal` when writing code that must work across widths;
otherwise the concrete type is the canonical surface.

The trait splits into four halves (`DecimalArithmetic`,
`DecimalConvert`, `DecimalTranscendental`, `DecimalConstants`) for
generic code that only needs a subset. `T: Decimal` is the
kitchen-sink supertrait that combines all four.

## Moving between widths

Widening is lossless and infallible (`From`); narrowing is fallible
(`TryFrom`). See [conversions](conversions.md) for the details.

```rust
# use decimal_scaled::{D18s2, D38s2};
let small: D18s2  = D18s2::from_bits(150);
let wide:  D38s2 = small.into();              // lossless widen
let back:  D18s2  = wide.try_into().unwrap();  // fallible narrow
```

Every adjacent pair in the comprehensive ladder
(D9 → D18 → D38 → D57 → D76 → D115 → D153 → D230 → D307 →
D462 → D616 → D924 → D1232) has a `From` / `TryFrom` pair plus
`.widen()` / `.narrow()` helper methods that step **one rung**
in either direction. Chain them to skip multiple rungs, or use
the `From` / `TryFrom` matrix directly to jump straight to any
narrower or wider tier.

```rust
# #[cfg(feature = "wide")] {
use decimal_scaled::{D38, D57, D115};
let a: D38<6> = D38::<6>::from_int(7);
let b: D57<6> = a.widen();          // one rung up
let c: D115<6> = b.widen().widen(); // two more rungs: D57 → D76 → D115
let _: D38<6> = c.try_into().unwrap();   // skip-jump back via TryFrom
# }
```

## Wide-tier notes (`D57` … `D1232`)

- Enable per width (`d57`, `d76`, `d115`, `d153`, `d230`, `d307`,
  `d462`, `d616`, `d924`, `d1232`) or by umbrella (`wide` for
  D57–D307, `x-wide` adds D462 / D616, `xx-wide` adds D924 /
  D1232).
- Storage is the in-tree hand-rolled wide-integer module
  (`crate::wide_int`); there is no external big-integer
  dependency. The wide-int type never appears in your code — you
  work through `from_bits` / `to_bits` and the normal arithmetic.
- The full surface is shipped on the wide tier: cross-type
  `PartialEq` against every primitive integer and float, the
  0.5-ULP-correctly-rounded strict transcendentals (with
  mode-aware `*_strict_with` siblings and AGM alternates
  `ln_strict_agm` / `exp_strict_agm`), arithmetic, conversions,
  formatting, rescaling, rounding, and the full overflow-variant
  family.
