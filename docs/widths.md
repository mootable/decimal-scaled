# The width family

`decimal-scaled` ships six storage widths. They all have the same API
shape — the same methods, the same const generic `SCALE`, the same
`Decimal` trait impl — and differ only in the size of the backing
integer, which sets the range and the maximum usable scale.

| Type | Storage | `MAX_SCALE` | Feature gate | Typical use |
|---|---|---|---|---|
| `D32`   | `i32`   | 9   | always on | embedded / register-sized ledgers |
| `D64`   | `i64`   | 18  | always on | interchange size; maps to SQL `DECIMAL(18, s)` |
| `D128`  | `i128`  | 38  | always on | the financial standard; satoshi-grade at `SCALE = 12` |
| `D256`  | 256-bit | 76  | `d256` / `wide` | crypto ratios, statistical accumulation |
| `D512`  | 512-bit | 153 | `d512` / `wide` | wide-scientific / actuarial work |
| `D1024` | 1024-bit| 307 | `d1024` / `wide` | deep arbitrary-precision determinism |

`MAX_SCALE` is the largest `SCALE` for which `10^SCALE` fits the storage
integer. Using a larger `SCALE` is a compile-time error (the
`multiplier()` const overflows when the type is instantiated).

## Choosing a width

The range at a given scale is roughly `storage_max / 10^SCALE`. Pick the
*narrowest* width whose range covers your values at the scale you need —
narrower storage is faster and smaller.

- **`D32` / `D64`** — native integer storage, single-instruction add /
  sub / compare. `D64` at `SCALE = 9` covers ±9.2 × 10⁹ with
  nanosecond-grade fractional precision.
- **`D128`** — the default choice. At `SCALE = 12` the range is roughly
  ±1.7 × 10²⁶ — far beyond global-GDP-scale money.
- **`D256` / `D512` / `D1024`** — the *wide tier*. Backed by the `bnum`
  big-integer crate, pulled in only when you enable the matching
  feature. `D256` at `SCALE ≈ 70` still leaves ~10⁵ integer headroom.

## Scale aliases

Each width exposes curated `…s<N>` aliases for common scales:

```rust
use decimal_scaled::{D32s2, D64s9, D128s12};
# #[cfg(feature = "wide")]
use decimal_scaled::{D256s35, D512s75, D1024s150};
```

`D128` has the full `D128s0` … `D128s38`. The other widths ship a
curated subset; the generic form `D64::<7>` is always available for any
in-range scale.

## The `Decimal` trait

Every width implements `Decimal`, the width-generic surface:

```rust
use decimal_scaled::Decimal;

fn sum_all<D: Decimal + core::ops::Add<Output = D>>(xs: &[D]) -> D {
    D::sum(xs.iter().copied())
}
```

`Decimal` exposes `Storage`, `SCALE`, `MAX_SCALE`, the `ZERO` / `ONE` /
`MAX` / `MIN` constants, `multiplier()`, `from_bits` / `to_bits`,
`scale()`, and the `is_zero` / `is_one` / `sum` helpers. Reach for it
when writing code that must work across widths; otherwise the concrete
type is the canonical surface.

## Moving between widths

Widening is lossless and infallible (`From`); narrowing is fallible
(`TryFrom`). See [conversions](conversions.md) for the details.

```rust
# use decimal_scaled::{D64s2, D128s2};
let small: D64s2  = D64s2::from_bits(150);
let wide:  D128s2 = small.into();              // lossless widen
let back:  D64s2  = wide.try_into().unwrap();  // fallible narrow
```

## Wide-tier notes (`D256` / `D512` / `D1024`)

- Enable per width (`d256`, `d512`, `d1024`) or all at once (`wide`).
- The storage backend is `bnum`; it never appears in your code — you
  work through `from_bits` / `to_bits` and the normal arithmetic.
- A few surfaces are still interim on the wide tier: cross-type
  `PartialEq` against primitive integers, and strict-mode
  transcendentals, are not yet wired for `D256+`. Arithmetic,
  conversions, formatting, rescaling, and rounding are complete.
