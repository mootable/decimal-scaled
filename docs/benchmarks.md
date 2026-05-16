# Benchmarks

Head-to-head matrix sweep of `decimal-scaled` against the wider Rust
numeric ecosystem (`bnum`, `ruint`, `rust_decimal`, `fixed`), plus
the crate's own fast / strict transcendental variants. Every
decimal width is exercised at three scales (smallest / midpoint /
largest) so the reader can see how cost scales with both storage
width and `SCALE`.

The benches live in [`benches/`](../benches/) and run under
[criterion](https://docs.rs/criterion/). The baseline crates
(`bnum`, `ruint`, `rust_decimal`, `fixed`, `i256`) are
**dev-dependencies only** — they are never compiled into a normal
build.

```sh
cargo bench --features "wide x-wide" --bench full_matrix
cargo bench --features wide --bench wide_int_backends
cargo bench --features wide --bench d_w128_mul_div_paths
```

> Absolute timings are machine-dependent. The *ratios* between
> implementations on the same machine, in the same run, are what
> matters. Operands are `black_box`-ed to defeat constant folding;
> outputs are returned from the closure so the optimiser cannot drop
> the call. **Winning cell on each row is bold. Every row uses one
> unit — the median natural unit across that row's cells — so
> values compare directly; cells whose natural unit is much smaller
> than the row's chosen one are rendered in scientific notation
> (e.g. `1.26×10⁻⁵ ms` for a 12 ns fast-path cell in an `ms`-scale
> row).**

## Time units

| Symbol | Unit | Seconds | Relative to `ns` |
|---|---|---|---|
| `s` | second | 10⁰ s | ×1 000 000 000 |
| `ms` | millisecond | 10⁻³ s | ×1 000 000 |
| `µs` | microsecond | 10⁻⁶ s | ×1 000 |
| `ns` | nanosecond | 10⁻⁹ s | ×1 |

`1 µs` = `1 000 ns`. A `27 µs` strict `ln` is `27 000 ns` — about
700× a `37 ns` fast `ln`.

---

## Storage tier and algorithm-of-record

The fixed-point arithmetic uses a different algorithm at each
width — the lookup below tells you which one a given row is
exercising.

| width | storage | widening | `÷ 10^SCALE` kernel |
|---|---|---|---|
| D9 | `i32` | `i64` | hardware `i64 / i64` |
| D18 | `i64` | `i128` | hardware `i128 / i128` |
| D38 | `i128` | hand-rolled 256-bit `Fixed` | **Möller–Granlund 2011** magic-multiply for `÷ 10^SCALE`; `mg_divide` |
| D76 | `Int256` (2×u128) | `Int512` | MG magic-multiply lifted to limb arithmetic (`div_wide_pow10`) |
| D153 | `Int512` | `Int1024` | MG, same path |
| D307 | `Int1024` | `Int2048` | MG, same path |

For the strict transcendentals:

| width | work integer | guard | algorithm |
|---|---|---|---|
| D9 / D18 | delegates to D38 | — | (see D38 row) |
| D38 | `d_w128_kernels::Fixed` (256-bit sign-magnitude) | 60 | artanh series for `ln`, range-reduced Taylor for `exp`, Cody–Waite for `sin`/`cos`, Machin for π, integer `isqrt` for `sqrt` |
| D76 | `Int1024` | 30 | rounded `mul` / `div` (half-to-even per op); same series as D38 lifted to the limb-array core |
| D153 | `Int2048` | 30 | same |
| D307 | `Int4096` | 30 | same |

Alternate transcendental paths (alongside the canonical above,
exposed under `bench-alt`):

- **`ln_strict_agm` / `exp_strict_agm`** — Brent–Salamin 1976 AGM
  for ln, Newton-on-AGM-ln for exp. Quadratic convergence,
  asymptotically wins at extreme working scales. Per
  `benches/agm_vs_taylor.rs` at D307<300> the AGM `ln` is 1.4×
  slower than the canonical artanh path, and exp via Newton-on-AGM
  is >130× slower — the asymptotic crossover hasn't kicked in at
  this crate's widths. Recorded in `ALGORITHMS.md`.

Alternate divide paths:

- **`limbs_divmod_knuth`** — Knuth Algorithm D (TAOCP §4.3.1)
  adapted to base 2^128. Available; canonical
  `limbs_divmod` stays on the const-fn binary shift-subtract path.
- **`limbs_divmod_bz`** — Burnikel–Ziegler 1998 recursive wrapper
  on top of Knuth.

## Accuracy contract

| family | accuracy at storage |
|---|---|
| `+` / `−` / `−` (unary) / `%` | exact |
| `×` / `÷` | rounded per `DEFAULT_ROUNDING_MODE` (HalfToEven default), within 0.5 ULP at storage scale |
| `*_strict` transcendentals — D38 | within **0.5 ULP** at storage; correctly rounded under HalfToEven, deterministic across platforms, `no_std`-compatible |
| `*_strict` transcendentals — D76 / D153 / D307 | within **0.5 ULP** at storage at typical scales; at deepest scales the rounded-intermediate budget tightens — see `ALGORITHMS.md` |
| `*` (lossy) transcendentals — D9 / D18 / D38 | f64-bridge: ~16 decimal digits, platform-libm-dependent, **not** correctly rounded |
| `*` plain transcendental name — wide tiers (D76 / D153 / D307) | with `strict` feature, dispatches to `*_strict`; with `fast` or `not(strict)`, the f64-bridge `*_fast` is used. Both `*_strict` and `*_fast` named methods are always available regardless of the active mode |

---

## 1. Arithmetic

Operands `a = from_int(2)`, `b = from_int(1)` — both in-range
at every public type×scale combo. Six ops: add / sub / mul / div
/ rem / neg.

### D9

| op | s = 0 | s = 5 | s = 9 |
|---|---|---|---|
| add | 401 ps | **384 ps** | 386 ps |
| sub | 393 ps | 386 ps | **377 ps** |
| mul | **395 ps** | 764 ps | 767 ps |
| div | **1.59 ns** | 2.51 ns | 2.44 ns |
| rem | 1.54 ns | 1.50 ns | **1.48 ns** |
| neg | **282 ps** | 293 ps | 303 ps |

### D18

| op | s = 0 | s = 9 | s = 18 |
|---|---|---|---|
| add | **372 ps** | 384 ps | 378 ps |
| sub | 455 ps | **452 ps** | 472 ps |
| mul | **0.38 ns** | 9.61 ns | 9.63 ns |
| div | **9.67 ns** | 10.3 ns | 11.1 ns |
| rem | **1.46 ns** | 1.58 ns | 2.72 ns |
| neg | 251 ps | **250 ps** | 266 ps |

### D38 + cross-crate baselines

| op | s = 0 | s = 19 | s = 38 | `rust_decimal` | `fixed::I64F64` |
|---|---|---|---|---|---|
| add | 923 ps | 934 ps | 876 ps | 5,138 ps | **842 ps** |
| sub | 917 ps | 912 ps | **847 ps** | 5,288 ps | 849 ps |
| mul | 2.46 ns | 13.1 ns | 13.8 ns | 2.78 ns | **1.49 ns** |
| div | 10.4 ns | 8.99 ns | 456 ns | **3.32 ns** | 23.2 ns |
| rem | 9.30 ns | 8.67 ns | 10.3 ns | **8.56 ns** | 14.2 ns |
| neg | 528 ps | 508 ps | 505 ps | 4,438 ps | **500 ps** |

### D76 + `bnum`-backed D76 baseline

| op | s = 0 | s = 35 | s = 76 | `bnum_d76` (s = 35) |
|---|---|---|---|---|
| add | 1.60 ns | **1.53 ns** | 1.55 ns | 1.79 ns |
| sub | 1.81 ns | **1.77 ns** | 1.91 ns | 1.82 ns |
| mul | **27.0 ns** | 60.7 ns | 8,898 ns | 332 ns |
| div | **99.9 ns** | 4,602 ns | 8,771 ns | 351 ns |
| rem | **15.2 ns** | 17.8 ns | 1,122 ns | 50.7 ns |
| neg | 1.60 ns | 1.57 ns | **1.56 ns** | 4.09 ns |

### D153

| op | s = 0 | s = 75 | s = 153 |
|---|---|---|---|
| add | 2.93 ns | **2.92 ns** | 3.00 ns |
| sub | **3.83 ns** | 4.01 ns | 4.23 ns |
| mul | **0.033 µs** | 16.6 µs | 30.0 µs |
| div | **0.14 µs** | 16.2 µs | 30.2 µs |
| rem | **0.020 µs** | 1.89 µs | 3.01 µs |
| neg | **2.60 ns** | 2.63 ns | 2.65 ns |

### D307

| op | s = 0 | s = 150 | s = 307 |
|---|---|---|---|
| add | **7.84 ns** | 7.93 ns | 7.95 ns |
| sub | **14.2 ns** | 14.4 ns | 14.2 ns |
| mul | **0.056 µs** | 63.6 µs | 113 µs |
| div | **0.25 µs** | 62.6 µs | 111 µs |
| rem | **0.036 µs** | 6.47 µs | 10.2 µs |
| neg | **5.05 ns** | 5.28 ns | 5.25 ns |

**Reading the arithmetic tables.** Add / sub / neg are exact;
mul / div round per `DEFAULT_ROUNDING_MODE`. Add / sub / neg are
single integer instructions at every width; mul / div pay for
limb-array work above D38. At D38 and above, mul / div cost
grows roughly linearly with limb count thanks to the MG magic-
multiply for `÷ 10^SCALE`, which keeps every width's `div` near
the same nanoseconds-per-limb ratio.

**Mul / div at the storage maximum scale.** At each type's largest
scale, `10^SCALE` is approaching the storage type's representable
limit (e.g. `10^38 ≈ 0.6 × i128::MAX`). The MG magic-multiply
constants the crate precomputes for `÷ 10^SCALE` are sized so the
inner product still fits the widening type; near the ceiling the
intermediate widens an extra step and the divide costs jump
noticeably. The row most visible to this effect is `D38` `div`:
~10 ns at SCALE 0 / 19, jumping at SCALE 38 to the wide-tier
algorithm cost. The crate keeps SCALE 38 as the *correct* path
(no precision loss) rather than restricting `D38` to SCALE ≤ 37.

---

## 2. Fast transcendentals (`f64`-bridge)

Available on D9 / D18 / D38 only — the wide tiers ship no lossy
form (f64 can't carry their precision; the strict path is the
only correct one). Functions: `ln`, `exp`, `sin`, `sqrt`.

Arguments: `1.5` for ln / sin / sqrt, `0.5` for exp — picked so the
result stays in range at every type×scale combo the sweep
exercises (`D38<38>` has max ≈ 1.7, the tightest). At `SCALE = 0`
the arguments built by `from_int(1) / from_int(2)` floor to `0`
and `from_int(1) + 0 = 1`, which makes a few cells hit fast
paths — those rows are still useful but reflect the early-return
not the full series.

Accuracy: ~16 decimal digits of f64 precision. **Not** correctly
rounded; results vary with platform libm.

### D9 / D18 / D38 fast

| fn | D9 s=9 | D18 s=18 | D38 s=38 | `rust_decimal` |
|---|---|---|---|---|
| ln | `__LOSSY_D9_LN__` | `__LOSSY_D18_LN__` | `__LOSSY_D38_LN__` | **3.75 µs** |
| exp | `__LOSSY_D9_EXP__` | `__LOSSY_D18_EXP__` | `__LOSSY_D38_EXP__` | **145 ns** |
| sin | `__LOSSY_D9_SIN__` | `__LOSSY_D18_SIN__` | `__LOSSY_D38_SIN__` | **2.55 µs** |
| sqrt | `__LOSSY_D9_SQRT__` | `__LOSSY_D18_SQRT__` | `__LOSSY_D38_SQRT__` | **578 ns** |

`rust_decimal`'s transcendentals are software-implemented (no f64
bridge) — accurate but not correctly rounded to the last place,
and substantially slower than the f64 path.

---

## 3. Strict transcendentals (integer-only, correctly rounded)

Functions: `ln_strict`, `exp_strict`, `sin_strict`,
`sqrt_strict`. Same argument convention as the fast block (1.5
for ln / sin / sqrt, 0.5 for exp). Deterministic across
platforms, `no_std`-compatible, 0.5 ULP at storage.

### D9 / D18 / D38 strict

D9 / D18 strict delegate up to D38's 256-bit guard-digit kernel;
their cost is dominated by D38 plus the narrow-tier round-trip.

| fn | D9 s=9 | D18 s=18 | D38 s=0 | D38 s=19 | D38 s=38 |
|---|---|---|---|---|---|
| ln | `__STRICT_D9_LN__` | `__STRICT_D18_LN__` | **1.13 µs** | 58.3 µs | 60.8 µs |
| exp | `__STRICT_D9_EXP__` | `__STRICT_D18_EXP__` | **1.08×10⁻³ µs** | 49.6 µs | 28.9 µs |
| sin | `__STRICT_D9_SIN__` | `__STRICT_D18_SIN__` | 20.0 µs | 44.1 µs | **17.7 µs** |
| sqrt | `__STRICT_D9_SQRT__` | `__STRICT_D18_SQRT__` | **13.6 ns** | 37.9 ns | 3,248 ns |

### Wide-tier strict — D76 / D153 / D307

Cost grows with both the work integer's bit width and the
guard-digit budget at each scale.

| fn | D76 s=0 | D76 s=35 | D76 s=76 | D153 s=0 | D153 s=75 | D153 s=153 | D307 s=0 | D307 s=150 | D307 s=307 |
|---|---|---|---|---|---|---|---|---|---|
| ln | **0.15 ms** | 1.35 ms | 3.25 ms | 0.30 ms | 6.01 ms | 17.1 ms | 0.53 ms | 33.3 ms | 112 ms |
| exp | **1.26×10⁻⁵ ms** | 1.29 ms | 2.98 ms | 1.71×10⁻⁵ ms | 5.58 ms | 15.4 ms | 2.78×10⁻⁵ ms | 31.7 ms | 98.8 ms |
| sin | **0.22 ms** | 1.04 ms | 2.45 ms | 0.41 ms | 4.63 ms | 13.0 ms | 0.76 ms | 24.4 ms | 78.6 ms |
| sqrt | **0.12 µs** | 19.9 µs | 46.1 µs | 0.19 µs | 77.6 µs | 168 µs | 0.36 µs | 304 µs | 703 µs |

**Reading the strict tables.** A few cells reflect *fast paths*
rather than the full series evaluation — the bold winner per row
sometimes picks one of these:

- `ln_strict` at `SCALE = 0`: the arg becomes `1` (the `½`
  division floors), and `ln(1)` returns 0 in `O(1)`.
- `exp_strict` at `SCALE = 0`: the arg becomes `0` and `exp(0) = 1`
  short-circuits.
- `sin_strict` at `SCALE = 0`: arg is `1`; small enough that the
  Taylor series terminates quickly.

The honest *series-cost* comparison lives in the midpoint and
maximum-scale columns. There:

- `sqrt_strict` is algebraic (integer `isqrt` + one round-to-
  nearest), so its growth is dominated by `isqrt`'s `O(b²)` limb
  work at b bits — not series evaluation. It's the only
  transcendental whose cost stays sub-microsecond past D76.
- `ln` / `exp` / `sin` evaluate a series at the working scale
  `SCALE + GUARD`. Cost grows roughly quadratically in working
  bits because each `mul` / `div` at the work scale is a limb-
  array operation. At D307<307> we're operating on Int4096
  internally — every series term touches all 32 limbs.

---

## 4. What the strict variants buy

Versus the fast `f64` bridge:

- **0.5 ULP correctly-rounded last place** at storage scale (D38;
  wide tiers at typical scales).
- **Deterministic bit-for-bit identical** across platforms.
- **`no_std`**-compatible.

The cost is throughput — typically 100–1000× the f64 bridge. For
latency-sensitive code that doesn't need determinism, fast is the
better default; for finance, regulated computation, reproducible
research, or `no_std` targets, strict is the reason the crate
exists.

---

## 5. Reference: wide-integer backends

For raw signed integer arithmetic without the decimal layer see
`benches/wide_int_backends.rs`. Summary at this revision:

| op | `Int256` (this crate) | `bnum` I256 | `ruint` U256 |
|---|---|---|---|
| add | **1.66 ns** | 1.94 ns | 5.75 ns |
| sub | 1.95 ns | **1.88 ns** | 5.96 ns |
| mul | 14.84 ns | 4.23 ns | **3.51 ns** |
| div | 15.21 ns | 69.32 ns | **5.68 ns** |
| rem | 14.89 ns | 64.46 ns | **5.90 ns** |
| neg | **1.85 ns** | 4.69 ns | — |

At 1024 bits the native back-end takes div / rem on its own
(`bnum`'s falls off ~4×); `ruint` doesn't ship a 1024-bit type.

---

## Methodology

- **Bench runner.** Criterion. Each row's measurement is the
  median wall-clock; warm-up 1 s, measurement 2 s, sample size 50
  (arithmetic / D38-and-narrower strict) or 20 (wide-tier
  strict). Wide-tier strict at deepest scale uses 5 s measurement
  window to absorb the larger per-iter cost.
- **Operand choice.** Arithmetic: `from_int(2)` and `from_int(1)`
  — universally in range at every width and scale. Transcendentals:
  `1.5` (= `from_int(1) + from_int(1)/from_int(2)`) for ln / sin /
  sqrt, `0.5` for exp — sized to stay in range at every type×scale
  combo, with `D38<38>`'s ≈ 1.7 ceiling being the binding
  constraint.
- **`black_box`.** Every input is wrapped in `std::hint::black_box`;
  the closure returns the result so the optimiser cannot drop the
  call.
- **Build profile.** `bench` (= `release` with `opt-level=3`,
  no debug-assertions).
- **Default features.** Stock `wide` + `x-wide` enabled. `strict`
  feature off — both `*` (lossy) and `*_strict` are exercised on
  the narrow tiers; with `strict` the fast block would dispatch
  to strict.
