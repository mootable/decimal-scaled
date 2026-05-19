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
**dev-dependencies only** - they are never compiled into a normal
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
> the call. **Every row uses one unit - the median natural unit
> across that row's cells - so values compare directly. Cells whose
> natural unit is smaller than the row's chosen one are rendered as
> plain decimals (e.g. `0.00146 µs` for a 1.5 ns cell in a µs-scale
> row); scientific notation is reserved for cells smaller than
> 10⁻⁵ of the row's unit. In §1 the winning cell per row is bold.
> In §2 onwards (transcendental tables) each width gets a single
> column showing only the **s = mid** measurement - the honest
> series-cost scale (s = 0 hits fast-path early returns and s = max
> sometimes shortens via Cody-Waite range reduction, so neither is
> a fair comparator). The bold mark goes on the row winner.**

> **Bench machine.** All numbers in this doc are from the v0.4.2
> full_matrix sweep on GitHub-hosted `ubuntu-latest` standard
> runners (2 vCPU shared, 7 GiB RAM, no core pinning). Per the
> Criterion author's standing caveat, shared CI runners carry
> 20–50 % wall-clock variance per cell. Cells are valid relative
> to each other within this run (matched compiler, matched
> features, one runner per width shard) but multi-hour sweep
> cells run roughly 1.5–2× slower than a cold-machine
> micro-bench of the same code; reach for a focused
> `lib_cmp_d{N}` bench when a single cell's magnitude matters.

## Time units

| Symbol | Unit | Relative to a second |
|---|---|---|
| `s`  | second      | 10⁰  s |
| `ms` | millisecond | 10⁻³ s |
| `µs` | microsecond | 10⁻⁶ s |
| `ns` | nanosecond  | 10⁻⁹ s |
| `ps` | picosecond  | 10⁻¹² s |

`1 µs` = `1 000 ns`. A `27 µs` strict `ln` is `27 000 ns` - about
700× a `37 ns` fast `ln`.

---

## Storage tier and algorithm-of-record

The fixed-point arithmetic uses a different algorithm at each
width - the lookup below tells you which one a given row is
exercising.

| width | storage | widening | `÷ 10^SCALE` kernel |
|---|---|---|---|
| D9 | `i32` | `i64` | hardware `i64 / i64` |
| D18 | `i64` | `i128` | hardware `i128 / i128` |
| D38 | `i128` | hand-rolled 256-bit `Fixed` | **Möller–Granlund 2011** magic-multiply for `÷ 10^SCALE`; `mg_divide` |
| D57 | `Int192` (3×u64) | `Int384` | MG magic-multiply lifted to limb arithmetic |
| D76 | `Int256` (4×u64) | `Int512` | MG, same path |
| D115 | `Int384` (6×u64) | `Int768` | MG, same path |
| D153 | `Int512` (8×u64) | `Int1024` | MG, same path |
| D230 | `Int768` (12×u64) | `Int1536` | MG, same path |
| D307 | `Int1024` (16×u64) | `Int2048` | MG, same path |
| D462 | `Int1536` (24×u64) | `Int3072` | MG, same path |
| D616 | `Int2048` (32×u64) | `Int4096` | MG, same path |
| D924 | `Int3072` (48×u64) | `Int6144` | MG, same path |
| D1232 | `Int4096` (64×u64) | `Int8192` | MG, same path |

For the strict transcendentals:

| width | work integer | guard | algorithm |
|---|---|---|---|
| D9 / D18 | delegates to D38 | - | (see D38 row) |
| D38 | `d_w128_kernels::Fixed` (256-bit sign-magnitude) | 60 | artanh series for `ln`, range-reduced Taylor for `exp`, Cody–Waite for `sin`/`cos`, Machin for π, integer `isqrt` for `sqrt` |
| D57 | `Int512` | 30 | same kernel family as D76, lifted to the half-width work integer |
| D76 | `Int1024` | 30 | rounded `mul` / `div` (half-to-even per op); same series as D38 lifted to the limb-array core |
| D115 | `Int1024` | 30 | same |
| D153 | `Int2048` | 30 | same |
| D230 | `Int3072` | 30 | same |
| D307 | `Int4096` | 30 | same |
| D462 | `Int4096` | 30 | same |
| D616 | `Int8192` | 30 | same |
| D924 | `Int12288` | 30 | same |
| D1232 | `Int16384` | 30 | same |

Alternate transcendental paths (alongside the canonical above,
exposed under `bench-alt`):

- **`ln_strict_agm` / `exp_strict_agm`** - Brent–Salamin 1976 AGM
  for ln, Newton-on-AGM-ln for exp. Quadratic convergence,
  asymptotically wins at extreme working scales. Per
  `benches/agm_vs_taylor.rs` at D307<300> the AGM `ln` is 1.4×
  slower than the canonical artanh path, and exp via Newton-on-AGM
  is >130× slower - the asymptotic crossover hasn't kicked in at
  this crate's widths. Recorded in `ALGORITHMS.md`.

Alternate divide paths:

- **`limbs_divmod_knuth`** - Knuth Algorithm D (TAOCP §4.3.1)
  adapted to base 2^128. Available; canonical
  `limbs_divmod` stays on the const-fn binary shift-subtract path.
- **`limbs_divmod_bz`** - Burnikel–Ziegler 1998 recursive wrapper
  on top of Knuth.

## Accuracy contract

| family | accuracy at storage |
|---|---|
| `+` / `−` / `−` (unary) / `%` | exact |
| `×` / `÷` | rounded per `DEFAULT_ROUNDING_MODE` (HalfToEven default), within 0.5 ULP at storage scale |
| `*_strict` transcendentals - D38 | within **0.5 ULP** at storage; correctly rounded under HalfToEven, deterministic across platforms, `no_std`-compatible |
| `*_strict` transcendentals - D76 / D153 / D307 | within **0.5 ULP** at storage at typical scales; at deepest scales the rounded-intermediate budget tightens - see `ALGORITHMS.md` |
| `*` (lossy) transcendentals - D9 / D18 / D38 | f64-bridge: ~16 decimal digits, platform-libm-dependent, **not** correctly rounded |
| `*` plain transcendental name - wide tiers (D76 / D153 / D307) | with `strict` feature, dispatches to `*_strict`; with `fast` or `not(strict)`, the f64-bridge `*_fast` is used. Both `*_strict` and `*_fast` named methods are always available regardless of the active mode |

---

## 1. Arithmetic

Operands `a = from_int(2)`, `b = from_int(1)` - both in-range
at every public type×scale combo. Six ops: add / sub / mul / div
/ rem / neg.

> **v0.4.2 sweep refresh.** Tables below come from the v0.4.2
> full_matrix sweep on GitHub-hosted `ubuntu-latest` standard
> runners (GHA run [`26099350473`](https://github.com/mootable/decimal-scaled/actions/runs/26099350473)).
> Narrow-tier ps-scale cells (D9 / D18 / D38 add / sub / neg)
> sit near the runner's resolution floor and carry ±20 %
> pipeline / steal-budget jitter; wide-tier ns+ values are
> reliable. See the [run-conditions preamble](#042-sweep--full-raw-data)
> at the bottom of this page for the full caveat.

### D9 - 32 bits

| op | s = 0 | s = 5 | s = 9 |
|---|---|---|---|
| add | 594.24 ps | 587.98 ps | 586.82 ps |
| sub | 601.54 ps | 597.18 ps | 588.83 ps |
| mul | 596.66 ps | 1.1301 ns | 1.0569 ns |
| div | 1.7269 ns | 2.8808 ns | 2.8780 ns |
| rem | 1.7273 ns | 1.7345 ns | 1.7276 ns |
| neg | 346.79 ps | 346.84 ps | 346.94 ps |

### D18 - 64 bits

| op | s = 0 | s = 9 | s = 18 |
|---|---|---|---|
| add | 933.63 ps | 933.46 ps | 933.59 ps |
| sub | 943.41 ps | 933.82 ps | 933.61 ps |
| mul | 933.84 ps | 2.2557 ns | 4.2330 ns |
| div | 1.8674 ns | 2.3349 ns | 6.0676 ns |
| rem | 1.8671 ns | 1.8673 ns | 2.1777 ns |
| neg | 622.78 ps | 622.76 ps | 622.76 ps |

### D38 - 128 bits

| op | s = 0 | s = 19 | s = 38 |
|---|---|---|---|
| add | 2.1088 ns | 2.1089 ns | 2.1087 ns |
| sub | 2.1085 ns | 2.1089 ns | 2.1084 ns |
| mul | 3.5150 ns | 10.525 ns | 23.457 ns |
| div | 7.0308 ns | 8.2440 ns | 1.0821 µs |
| rem | 7.0332 ns | 7.0333 ns | 9.7319 ns |
| neg | 1.4055 ns | 1.4055 ns | 1.4057 ns |

### D57 - 192 bits

| op | s = 0 | s = 28 | s = 56 |
|---|---|---|---|
| add | 2.9878 ns | 2.9885 ns | 2.9783 ns |
| sub | 2.9948 ns | 2.9927 ns | 2.9976 ns |
| mul | 15.269 ns | 91.082 ns | 141.77 ns |
| div | 73.261 ns | 301.63 ns | 544.93 ns |
| rem | 24.679 ns | 119.11 ns | 133.21 ns |
| neg | 2.1004 ns | 2.0982 ns | 2.0978 ns |

### D76 - 256 bits

| op | s = 0 | s = 35 | s = 76 |
|---|---|---|---|
| add | 3.7104 ns | 3.7049 ns | 3.7116 ns |
| sub | 3.7234 ns | 3.7218 ns | 3.7248 ns |
| mul | 16.586 ns | 83.476 ns | 153.11 ns |
| div | 68.331 ns | 360.49 ns | 633.41 ns |
| rem | 26.177 ns | 171.69 ns | 179.15 ns |
| neg | 2.8136 ns | 2.8138 ns | 2.8138 ns |

### D115 - 384 bits

| op | s = 0 | s = 57 | s = 114 |
|---|---|---|---|
| add | 5.7143 ns | 5.7047 ns | 5.7042 ns |
| sub | 6.5618 ns | 6.5581 ns | 6.5640 ns |
| mul | 24.333 ns | 130.26 ns | 266.40 ns |
| div | 83.600 ns | 411.91 ns | 762.21 ns |
| rem | 32.015 ns | 180.46 ns | 192.03 ns |
| neg | 3.7771 ns | 3.7795 ns | 3.7773 ns |

### D153 - 512 bits

| op | s = 0 | s = 75 | s = 153 |
|---|---|---|---|
| add | 8.1463 ns | 8.1447 ns | 8.1470 ns |
| sub | 15.689 ns | 15.681 ns | 15.703 ns |
| mul | 54.958 ns | 199.98 ns | 512.11 ns |
| div | 129.33 ns | 724.58 ns | 1.5000 µs |
| rem | 42.847 ns | 144.76 ns | 156.38 ns |
| neg | 5.5634 ns | 5.5573 ns | 5.5562 ns |

### D230 - 768 bits

| op | s = 0 | s = 115 | s = 230 |
|---|---|---|---|
| add | 16.096 ns | 16.093 ns | 16.098 ns |
| sub | 18.674 ns | 18.670 ns | 18.675 ns |
| mul | 49.385 ns | 333.56 ns | 870.83 ns |
| div | 173.05 ns | 775.24 ns | 1.6890 µs |
| rem | 50.141 ns | 132.56 ns | 155.14 ns |
| neg | 10.131 ns | 10.200 ns | 10.652 ns |

### D307 - 1024 bits

| op | s = 0 | s = 150 | s = 307 |
|---|---|---|---|
| add | 21.942 ns | 21.931 ns | 21.933 ns |
| sub | 25.491 ns | 25.493 ns | 25.493 ns |
| mul | 61.407 ns | 438.33 ns | 1.4223 µs |
| div | 181.51 ns | 963.74 ns | 2.5714 µs |
| rem | 62.780 ns | 154.60 ns | 179.93 ns |
| neg | 11.484 ns | 11.872 ns | 12.677 ns |

### D462 - 1536 bits

| op | s = 0 | s = 230 | s = 461 |
|---|---|---|---|
| add | 36.013 ns | 35.988 ns | 35.998 ns |
| sub | 44.831 ns | 44.846 ns | 44.835 ns |
| mul | 123.35 ns | 1.0312 µs | 3.0807 µs |
| div | 358.24 ns | 2.0829 µs | 5.7250 µs |
| rem | 104.22 ns | 276.76 ns | 321.42 ns |
| neg | 36.077 ns | 36.209 ns | 37.208 ns |

### D616 - 2048 bits

| op | s = 0 | s = 308 | s = 615 |
|---|---|---|---|
| add | 59.892 ns | 59.989 ns | 59.930 ns |
| sub | 76.871 ns | 76.834 ns | 76.847 ns |
| mul | 118.11 ns | 1.5602 µs | 4.5296 µs |
| div | 333.27 ns | 4.2382 µs | 6.9198 µs |
| rem | 96.230 ns | 191.51 ns | 259.38 ns |
| neg | 37.767 ns | 39.238 ns | 41.835 ns |

### D924 - 3072 bits

| op | s = 0 | s = 461 | s = 923 |
|---|---|---|---|
| add | 95.543 ns | 95.197 ns | 95.231 ns |
| sub | 141.40 ns | 141.44 ns | 141.31 ns |
| mul | 183.09 ns | 3.4634 µs | 11.099 µs |
| div | 591.24 ns | 5.7540 µs | 15.428 µs |
| rem | 172.01 ns | 396.47 ns | 480.31 ns |
| neg | 67.940 ns | 68.679 ns | 70.849 ns |

### D1232 - 4096 bits

| op | s = 0 | s = 616 | s = 1231 |
|---|---|---|---|
| add | 121.92 ns | 121.71 ns | 121.77 ns |
| sub | 172.10 ns | 172.03 ns | 172.26 ns |
| mul | 232.34 ns | 6.0995 µs | 19.643 µs |
| div | 769.06 ns | 8.6792 µs | 24.506 µs |
| rem | 219.94 ns | 468.78 ns | 602.10 ns |
| neg | 95.565 ns | 95.380 ns | 97.255 ns |

## 2. Fast transcendentals (`f64`-bridge)

Unlike the ≤ 0.5 ULP guarantee of the default `*_strict`
transcendentals, `*_fast` is meant to be exactly that: fast.
It deliberately makes no accuracy guarantees. It exists as an
escape hatch for situations where some precision can be
dropped — typically when the strict path is what you compute
with, but the result is then handed to something that only
makes sense at f64 precision anyway (graphics hardware, libm
shape-matching, an interop boundary that's f64 to begin with).
For anything where last-digit accuracy or cross-platform
determinism matters, stay on the default `*_strict` path.

The `*_fast` methods route through `f64::ln` / `f64::sin` / etc.
Available at every width - narrow tiers (D9 / D18 / D38) and wide
tiers (D76 / D153 / D307) all expose them - but only useful below
D38 where the f64 mantissa carries enough precision; on wide
tiers the result collapses to ~16 decimal digits regardless of
the storage width.

Bench arguments: D38 at `SCALE = 9` (`≈ 2.345678901`) and D76 at
`SCALE = 9` (`= 2`). Functions called explicitly via their
`*_fast` name so the result is the f64-bridge path regardless of
which crate feature flips the plain `*` dispatcher.

Accuracy: ~16 decimal digits of f64 precision. **Not** correctly
rounded; results vary with platform libm.

| fn | D38 `*_fast` | D76 `*_fast` | `rust_decimal` |
|---|---|---|---|
| ln   | **35.8 ns** | 201 ns | 3,000 ns |
| exp  | **42.6 ns** | 211 ns | 2,124 ns |
| sin  | **43.5 ns** | 226 ns | 2,955 ns |
| sqrt | **31.0 ns** | 197 ns |   658 ns |

D9 / D18 `*_fast` aren't separately benched: they share the D38
f64-bridge kernel through `to_f64` / `from_f64` and incur only a
sub-ns round-trip on top of the D38 numbers above.

`rust_decimal`'s transcendentals are software-implemented (no f64
bridge) - accurate but not correctly rounded to the last place,
and substantially slower than the f64 path.

### 2.1 Per-tier accuracy loss

Each `*_fast` result inherits f64's ~16 decimal-digit mantissa.
After scaling back into the type's `[u64; L]` storage, the
result's low-order digits are pure noise / zero-fill — the f64
output simply doesn't carry that precision.

The table below reports the number of trailing decimal digits of
the storage-scale result that diverge from the `*_strict`
reference, measured by
`examples/fast_vs_strict_ulp.rs`. Each row uses argument `1.5`
for `ln` / `sin` / `sqrt` and `0.5` for `exp`.

| type / s | ln noise | exp noise | sin noise | sqrt noise |
|----------|---------:|----------:|----------:|-----------:|
| D9<5>      |   0 |   0 |   0 |   0 |
| D9<9>      |   0 |   0 |   0 |   0 |
| D18<9>     |   0 |   0 |   0 |   0 |
| D18<18>    |   2 |   3 |   2 |   3 |
| D38<19>    |   3 |   4 |   3 |   3 |
| D38<38>    |  39 |  38 |  22 |  22 |
| D57<28>    |  12 |  12 |  12 |  13 |
| D76<35>    |  18 |  19 |  19 |  20 |
| D115<57>   |  41 |  42 |  41 |  41 |
| D153<75>   |  59 |  59 |  59 |  60 |
| D230<115>  |  99 |  99 |  98 |  98 |
| D307<150>  | 134 | 135 | 134 | 134 |
| D462<230>  | 214 | 215 | 214 | 213 |
| D616<308>  | 292 | 292 | 292 | 292 |
| D924<461>  | 461 | 462 | 461 | 462 |
| D1232<616> | 616 | 617 | 616 | 617 |

A noise count of **N** means the last N decimal digits at storage
scale are zero-fill / random; the leading `max(0, MAX_SCALE − N)`
digits agree with the strict reference. The empirical pattern
matches the analytical bound `noise ≈ max(0, SCALE + log₁₀|result| − 15)`
that f64's 53-bit mantissa imposes.

**Reading the table.**

- **D9 and D18 at low scale (≤ 9)** suffer no precision loss —
  the result has at most 9 fractional digits and f64 has ~16
  digits of headroom.
- **D38 and below, scale ≤ 19**, lose only 2–4 trailing digits.
  Acceptable for finance-grade work where last-digit precision
  is not load-bearing.
- **D38 at MAX_SCALE = 38** loses 22+ trailing digits; the f64
  bridge is *not* a substitute for `*_strict` if you need that
  precision.
- **Wide tiers** (D57 and above) lose roughly `SCALE − 15`
  trailing digits — at D1232<616> only the leading 15-16
  significant figures survive. Treat `*_fast` as a "speed-first,
  16-digit result rendered into the storage type" path on the
  wide tiers; reach for `*_strict` when the wider digits actually
  matter.

---

## 3. Strict transcendentals (integer-only, correctly rounded)

Functions: `ln_strict`, `exp_strict`, `sin_strict`,
`sqrt_strict`. Same argument convention as the fast block (1.5
for ln / sin / sqrt, 0.5 for exp). Deterministic across
platforms, `no_std`-compatible, 0.5 ULP at storage.

> **0.4.2 highlights — the Tang `ln` ladder.** Every wide tier
> ships a bespoke narrow-GUARD Tang-style lookup slot for `ln`
> at its popular mid-storage SCALE (D57<20>, D115<57>, D153<76>,
> D307<150>, D462<230>, D616<308>, D924<460>, D1232<615>). At
> the exact slot the speedup runs 12.7× (D115<57>) — 33.8×
> (D1232<616>) over v0.4.0; the wide-tier `ln` numbers below
> drop into the single-digit-µs range at every shipped tier.
> Tang `exp` lookup landed at the four narrowest wide tiers
> (D57<20>..D307<150>); beyond Int3072 the surface-Tang `exp`
> loses to the adaptive Smith r/2^n in `exp_fixed` and is not
> dispatched. Hyperbolic kernels consume `tang_exp_fixed` via
> the reciprocal-divide identity for an additional 1.2–1.31×.
> Cite Tang 1989/1990 (ACM TOMS 16(4)). Note that the
> mid-column entries below are at SCALE = MAX/2 per the
> long-standing column convention; the Tang slot is at the
> tier's *named* popular SCALE which doesn't always land
> exactly at MAX/2 (e.g. D57's slot is at SCALE 20, not 28).

> **v0.4.2 sweep — narrow-tier caveat.** The narrow-tier
> (D9 / D18 / D38) values below were recorded on shared GHA
> standard runners over a multi-hour sweep window. Cold-machine
> micro-benches of the same code typically measure several ×
> faster; treat the absolute µs values here as an upper-bound
> ceiling, not a steady-state cost. Wide-tier numbers carry the
> same caveat at a smaller multiple (1.5–2×).

### D9 / D18 / D38 strict

| fn | D9 (s=5) | D18 (s=9) | D38 (s=19) |
|---|---|---|---|
| ln | 4.0101 µs | 6.4932 µs | 8.7318 µs |
| exp | 3.6264 µs | 6.3317 µs | 8.3816 µs |
| sin | 4.4827 µs | 5.5055 µs | 7.3458 µs |
| sqrt | 21.379 ns | 19.691 ns | 33.204 ns |

### Wide-tier strict - D57 / D76 / D115 / D153 / D230 / D307 / D462 / D616 / D924 / D1232

Cost grows with both the work integer's bit width and the
guard-digit budget at each scale. The full v0.4.2 sweep now
populates every cell.

#### Wide (`wide` umbrella — D57 / D76 / D115 / D153 / D230 / D307)

| fn | D57 (s=28) | D76 (s=35) | D115 (s=57) | D153 (s=75) | D230 (s=115) | D307 (s=150) |
|---|---|---|---|---|---|---|
| ln | 20.891 µs | 26.193 µs | 2.4617 µs | 3.4834 µs | 62.979 µs | 5.9677 µs |
| exp | 18.018 µs | 24.587 µs | 21.658 µs | 31.927 µs | 55.263 µs | 83.119 µs |
| sin | 12.824 µs | 17.271 µs | 21.844 µs | 25.342 µs | 43.114 µs | 59.760 µs |
| sqrt | 1.0728 µs | 1.7237 µs | 2.4447 µs | 2.3983 µs | 3.0957 µs | 3.7041 µs |

The dramatic `ln` drops at D115 / D153 / D307 are the Tang
lookup slots landing exactly at the column's SCALE; D57<28>
and D76<35> sit one tier past their respective Tang slot
(SCALE 20 / SCALE 0 don't coincide with the column midpoint),
and D230 has no slot in this cycle so it shows the
shared-kernel-only improvement over v0.4.0.

#### Extra-wide (`x-wide` adds D462 / D616)

| fn | D462 (s=230) | D616 (s=308) |
|---|---|---|
| ln | 8.3266 µs | 14.114 µs |
| exp | 146.61 µs | 251.85 µs |
| sin | 120.65 µs | 213.50 µs |
| sqrt | 8.8315 µs | 10.751 µs |

D462<230> and D616<308> sit on their Tang `ln` slots (27.6×
and 26.1× over v0.4.0 respectively).

#### XX-wide (`xx-wide` adds D924 / D1232)

| fn | D924 (s=461) | D1232 (s=616) |
|---|---|---|
| ln | 28.364 µs | 46.472 µs |
| exp | 631.17 µs | 1.2167 ms |
| sin | 603.53 µs | 1.2035 ms |
| sqrt | 27.567 µs | 39.249 µs |

D924<461> sits one off its Tang slot (SCALE 460) and D1232<616>
sits one off its Tang slot (SCALE 615) — both are still
30.8× / 33.8× over v0.4.0, the largest Tang wins in the matrix.

**Historical comparison — 0.2.5 baseline.** On the same hardware,
0.2.5 measured D76<35> ln at 1.37 ms, D153<75> ln at 6.40 ms,
D307<150> ln at 34.1 ms. After this cycle's u64-native limbs, MG
2-by-1 reciprocal Knuth divide, Brent's two-stage exp argument
reduction, multi-level sqrt halving in ln, [0, π/4] sin range
reduction, sin_cos / sinh_cosh joint kernels, thread-local pi /
ln2 / ln10 cache, pow10-cached mul/div per inner loop, the 0.4.2
narrow-GUARD trig family, the Tang `ln` lookup ladder, and the
reciprocal-divide hyperbolic identity:

| op | 0.2 | 0.4.2 | speedup |
|---|---|---|---|
| D76<35>  ln   |  1.37 ms |  26.2 µs |  **52×** |
| D76<35>  exp  |  1.27 ms |  24.6 µs |  **52×** |
| D76<35>  sin  |  1.08 ms |  17.3 µs |  **63×** |
| D76<35>  sqrt | 20.5 µs  |  1.72 µs |  **12×** |
| D153<75> ln   |  6.40 ms |  3.48 µs | **1839×** |
| D153<75> exp  |  5.87 ms |  31.9 µs |  **184×** |
| D153<75> sin  |  4.82 ms |  25.3 µs |  **190×** |
| D153<75> sqrt | 83.6 µs  |  2.40 µs |  **35×** |
| D307<150> ln  | 34.1 ms  |  5.97 µs | **5712×** |
| D307<150> exp | 31.2 ms  |  83.1 µs |  **375×** |
| D307<150> sin | 25.5 ms  |  59.8 µs |  **427×** |
| D307<150> sqrt|  313 µs  |  3.70 µs |  **85×** |

> The **0.2** column is the 0.2.5 baseline measured at the start of
> the 0.2.x cycle on the original dev box; the **0.4.2** column is
> from the v0.4.2 (GHA run [`26099350473`](https://github.com/mootable/decimal-scaled/actions/runs/26099350473))
> full_matrix sweep on GitHub-hosted `ubuntu-latest` standard
> runners. The two halves of each speedup are measured on
> **different machines** — the 0.2 column reflects cold-machine
> dev-box runs while the 0.4.2 column reflects shared CI runners
> that typically measure 1.5–2× slower per the run-conditions
> preamble below. The four-digit `ln` ratios at D153<75> and
> D307<150> are real — the Tang lookup slot collapses the
> dominant artanh series cost — but the underlying CI / dev-box
> machine mismatch means they should be read as "the algorithm
> changed shape" rather than as a self-reproducing micro-bench
> ratio. Use these numbers as a directional "the algorithm
> family changed, here's the shape of the change" — not as a
> drop-in benchmark you can re-run yourself.

**Reading the strict tables.** Both tables sample at the
midpoint scale because the storage extremes hit shortcut paths
that aren't the algorithm-of-record cost:

- At `SCALE = 0`, `ln_strict`'s arg floors to `1` (so `ln(1)=0`
  returns in `O(1)`), `exp_strict`'s arg floors to `0` (so
  `exp(0)=1`), and `sin_strict`'s arg is `1` (Taylor terminates
  quickly). Cheap, but not the series cost.
- At `SCALE = MAX`, Cody-Waite range reduction sometimes lets
  the series start much closer to the answer than at the
  midpoint, producing a faster cell that misrepresents
  steady-state cost.

At s = mid:

- `sqrt_strict` is algebraic (integer `isqrt` + one round-to-
  nearest), so its growth is dominated by `isqrt`'s `O(b²)` limb
  work at b bits - not series evaluation. It's the only
  transcendental whose cost stays sub-microsecond past D76.
- `ln` / `exp` / `sin` evaluate a series at the working scale
  `SCALE + GUARD`. Cost grows roughly quadratically in working
  bits because each `mul` / `div` at the work scale is a limb-
  array operation. At D307<300> we're operating on Int4096
  internally - every series term touches all 32 limbs.

---

## 4. What the strict variants buy

Versus the fast `f64` bridge:

- **0.5 ULP correctly-rounded last place** at storage scale (D38;
  wide tiers at typical scales).
- **Deterministic bit-for-bit identical** across platforms.
- **`no_std`**-compatible.

The cost is throughput - typically 100–1000× the f64 bridge. For
latency-sensitive code that doesn't need determinism, fast is the
better default; for finance, regulated computation, reproducible
research, or `no_std` targets, strict is the reason the crate
exists.

---

## 5. Where each crate fits

### Two things this crate uniquely offers

Before the cell-by-cell numbers, here's what `decimal-scaled`
brings that no other crate in this comparison brings together:

1. **≤ 0.5 ULP correctness on every transcendental, at every
   shipped width, by default.** `ln` / `exp` / `sin` / `cos` /
   `tan` / `sqrt` / `cbrt` / `powf` / `asin` / `acos` / `atan` /
   `atan2` / `sinh` / `cosh` / `tanh` / `asinh` / `acosh` /
   `atanh` / `to_degrees` / `to_radians` land within half an ULP
   of the exact mathematical result, with bit-identical output
   on every platform. Peers that ship transcendentals are
   either fast-but-libm-precision (`g_math`), require manual
   render-mode management to match (`fastnum`, `rust_decimal`),
   or have an algorithmic precision gap (`dashu-float`).
2. **First-class caller-chosen rounding mode at every lossy
   operation.** The default is HalfToEven (the IEEE 754
   default), but every `*` / `/` / `%`, every `rescale`, and
   every strict transcendental has a `*_with(mode)` sibling
   accepting `RoundingMode::{HalfToEven, HalfAwayFromZero,
   HalfTowardZero, Ceiling, Floor, Trunc}`. The crate-wide
   default is also selectable at compile time via the
   `rounding-*` Cargo features. Useful when you need to
   bit-match an external system (ASTM E29, NUMERIC, MS-Excel,
   bank statement) without forking the library.

This isn't a competition — the crates below solve different
problems, and the right choice depends on the shape of your
problem rather than on who wins which row. The numbers exist
to help you decide *whether* you can use a given crate, not to
crown a winner.

A starter map of where each crate sits naturally:

| Crate                | Storage shape                    | Strength                                                                | Cost                                                                  |
|----------------------|----------------------------------|-------------------------------------------------------------------------|-----------------------------------------------------------------------|
| **decimal-scaled**   | Stack `[u64; L]`, compile-time SCALE | ≤ 0.5 ULP correctness on every transcendental, `*_with(mode)` siblings for every lossy op, `no_std`, const-fn arithmetic, deterministic | Wide-tier mul / div cost (catch-up work tracked in Roadmap)           |
| **fastnum**          | Stack fixed-width decimal (D128 / D256 / D512) | Very fast transcendentals at fixed internal precision (38 / 75 / 155 digits) | No SCALE generic; renders with truncation so the user picks the mode  |
| **rust_decimal**     | 96-bit mantissa, runtime scale   | The database `NUMERIC` shape; serde-friendly; widely deployed             | ~10× slower arithmetic than a stack decimal at the same precision     |
| **decimal-rs**       | 128-bit, runtime scale           | Compact, fast at D128                                                   | Capped at i128 width; no wide tiers                                   |
| **bigdecimal**       | Heap `BigInt` + scale            | Arbitrary precision at runtime                                          | Heap traversal on every op; no transcendentals                        |
| **dashu-float**      | Heap arbitrary-precision         | True arbitrary precision; ships transcendentals                          | Heap allocation; precision context limits result digits, not working  |
| **fixed::IxxFyy**    | Stack binary fixed-point         | Single-instruction add / sub at narrow widths                            | Binary, not decimal — different rounding semantics                    |
| **g_math**           | FP-expression DSL                | Fast for embedded expression eval                                       | 6–46 ULP off on transcendentals at the matched width                  |

Pick the row whose strengths match your constraints first; only
then look at the per-width charts below to see what cost you'd pay.

> **A note on intent.** This chapter isn't trying to poke holes
> in other people's libraries. The goal is a reproducible
> side-by-side at matched storage width and midpoint scale, so
> you can see what trade-off each crate is offering. Where a
> library's published claim doesn't match what the bench
> measures (`g_math`'s "0 ULP transcendentals" being the
> standing example), we say so with the numbers attached. If
> you maintain one of the libraries below and disagree with the
> analysis, please review
> [`benches/library_comparison.rs`](../benches/library_comparison.rs)
> and open a PR — we'll re-run the bench, refresh the tables,
> and credit the fix.

Bench source: `benches/library_comparison.rs`. The per-width
summary chart in each subsection plots x = operation (add / sub /
neg / mul / div / rem / sqrt / ln / exp / sin) against y = time
(log ns), one bar per library per op, at that width's centre
scale. Reading across charts shows how each library scales with
precision; reading down a single chart shows the within-library
trade between arithmetic and transcendentals at that width.


### Accuracy at 128-bit (1 ULP = 10⁻¹⁹)

Baseline: `D76<19>` integer-only `*_strict` (≥ 49 effective
working digits, rounded back to 19 under **HalfToEven**, which
is the crate-wide default and the IEEE 754 default mode). Bold
= correctly rounded to the last place under that baseline.

| op      | decimal-scaled | fastnum   | rust_decimal | dashu-float | decimal-rs | bigdecimal | g_math |
|---------|----------------|-----------|--------------|-------------|------------|------------|--------|
| ln(2)   | **0**          | **0**     | **0**        | **0**       | **0**      | -          | 6      |
| exp(1)  | **0**          | 1†        | 1†           | 4           | 1†         | -          | 46     |
| sin(1)  | **0**          | 1†        | 1†           | -           | -          | -          | 33     |
| sqrt(2) | **0**          | **0**     | **0**        | -           | **0**      | **0**      | 12     |

**† Rounding-mode artifact, not a computation error.** The
1-ULP cells for `fastnum` and `rust_decimal` are produced by a
different last-digit rounding choice, not by precision loss.
`examples/rounding_mode_probe.rs` confirms this: for `exp(1) = e`,

```
e = 2.71828182845904523536028747...
  HalfToEven       ->  2.7182818284590452354  <- decimal-scaled
  HalfAwayFromZero ->  2.7182818284590452354
  Trunc / Floor    ->  2.7182818284590452353  <- rust_decimal,
                                                 fastnum-rendered-at-s19
```

`fastnum` actually carries the full 38-digit `e` internally
(`2.71828182845904523536028747135266249776`); rendering it at
SCALE=19 with truncation gives `…2353`. `rust_decimal` does the
same. If both were rendered HalfToEven they would also score
0 ULP. The same explanation covers the `sin(1)` row.

`decimal-scaled`'s `*_strict_with(mode)` siblings let you reproduce
any of the other rounding modes if you need bit-compatibility with
a peer's choice; the default `*_strict` uses HalfToEven to match
IEEE 754 and to give round-trip stability across repeated
operations.

By contrast, `dashu-float`'s 4-ULP `exp(1)` and `g_math`'s
6 / 46 / 33 / 12 ULP errors are *not* rounding-mode artifacts —
they're genuine precision losses in the underlying computation
(`g_math`'s "0 ULP transcendentals" marketing claim is wrong by
those margins). Dashes mark "not implemented in this crate at
this version" — `bigdecimal` ships no `ln` / `exp` / `sin`;
`dashu-float` and `decimal-rs` ship no `sin`.

### I128 — 128-bit storage at scale 19

This is the width with the most company: every crate in the
starter map above is a candidate here. The chart shows where
each one lands across the operation surface.

![operations @ 128-bit at scale 19](figures/library_comparison/summary_128bit.png)

The richest comparator set. Cells: speed, plus `(ULP n)` for
transcendentals.

| op       | decimal-scaled         | fastnum (D128)        | rust_decimal (s=19)    | fixed::I64F64 | bigdecimal (s=19) | dashu-float (p=19)  | decimal-rs (s=19)      | g_math Q64.64       |
|----------|------------------------|-----------------------|------------------------|----------------|-------------------|---------------------|------------------------|---------------------|
| add      | **2.11 ns**            | 14.9 ns               | 12.0 ns                | 2.11 ns        | 56.2 ns           | 96.9 ns             | 12.1 ns                | -                   |
| sub      | **2.12 ns**            | 10.4 ns               | 13.2 ns                | 2.11 ns        | 49.7 ns           | 96.3 ns             | 12.1 ns                | -                   |
| mul      | 10.5 ns                | 10.6 ns               | 37.0 ns                | **2.47 ns**    | 52.3 ns           | 86.4 ns             | 60.6 ns                | 280 ns              |
| div      | 8.11 ns                | **7.45 ns**           | 9.83 ns                | 29.7 ns        | 42.2 ns           | 96.0 ns             | 129 ns                 | -                   |
| rem      | 7.04 ns                | 30.4 ns               | 20.7 ns                | 14.4 ns        | 73.2 ns           | 64.2 ns             | **5.15 ns**            | -                   |
| neg      | **1.41 ns**            | 7.17 ns               | 2.79 ns                | 1.41 ns        | 16.9 ns           | 6.35 ns             | 3.05 ns                | -                   |
| ln       | **395 ns (0 ULP)**     | 64.9 ns (0 ULP)       | 5.34 µs (0 ULP)        | -              | -                 | 80.1 µs (0 ULP)     | 3.89 µs (0 ULP)        | 1.12 µs (6 ULP)     |
| exp      | 8.44 µs (0 ULP)        | 12.7 µs (0†)          | 265 ns (0†)            | -              | -                 | 258 µs (4 ULP)      | **71.5 ns (0†)**       | 2.88 µs (46 ULP)    |
| sin      | 8.08 µs (0 ULP)        | **8.96 µs (0†)**      | 3.45 µs (0†)           | -              | -                 | -                   | -                      | 26.9 µs (33 ULP)    |
| sqrt     | 32.1 ns (0 ULP)        | **20.0 ns (0 ULP)**   | 877 ns (0 ULP)         | -              | 3.12 µs (0 ULP)   | -                   | 2.11 µs (0 ULP)        | 6.12 µs (12 ULP)    |

**† rendering-mode artifact, not a precision loss.** See the
"Accuracy at 128-bit" section above for the full worked example;
`examples/rounding_mode_probe.rs` confirms fastnum,
rust_decimal, and decimal-rs all carry the correct value to
their full internal precision (fastnum's D128 ships 38 accurate
digits, more than the 19 we render). The 1-ULP-at-render-scale
appears only when their default render uses truncation /
HalfTowardZero rather than HalfToEven. With matched rounding
modes the cells would also be 0 ULP.

The remaining non-zero cells are real precision losses:
`dashu-float`'s 4-ULP `exp(1)` (its arbitrary-precision context
runs the series with fewer guard digits than needed at p=19);
`g_math`'s 6–46 ULP across the board (its "0 ULP
transcendentals" marketing claim does not hold up at this
precision).

**`ln` at 0.4.2 — the Tang lookup slot.** The 395 ns cell for
`decimal-scaled` is the 0.4.2 Tang `ln` lookup hitting at
SCALE 19: the M=128 table plus narrow GUARD=8 collapses the
artanh series cost to a handful of small-correction terms.
v0.4.0 timed this cell at 10.5 µs; the 0.4.2 number is a
**~26× drop** at the same correctness contract. fastnum's
65 ns ln is still faster but spends a different precision
budget (fastnum runs at fixed 38-digit internal precision
regardless of the user's render SCALE).

### Caveat — fastnum `atan` / `atan2` input-range rejection

`fastnum`'s `Decimal::atan` returns a signalling NaN immediately
for any `|x| > 1` ([`fastnum-0.4.5/src/decimal/dec/math/atan.rs`](https://docs.rs/fastnum/0.4.5/src/fastnum/decimal/dec/math/atan.rs.html),
lines 24–40). This is mathematically incorrect — `atan` is
defined on the whole real line with range `(−π/2, π/2)` — but it
means any benchmark that calls `fastnum::Decimal::atan(2)` or
similar is timing a NaN early-return, not an `atan` computation.

The per-tier sweep in `target/medians_per_tier.tsv` records
`128bit_s19/fastnum/atan = 23 ns` vs
`128bit_s19/decimal-scaled/atan = 18.7 µs`; the ~810× ratio is
fastnum opting out of the computation rather than a faster
algorithm. fastnum's actual atan for in-range `|x| ≤ 1` reduces
`atan(x) = asin(x / sqrt(x² + 1))` then runs an asin half-angle
reduction + Taylor — comparable complexity to our own
`atan_taylor` + three-halvings path. atan2 inherits the same
rejection because it forwards to `atan(y / x)` unconditionally.

Numbers below are kept as-recorded so the artefact is auditable;
do not read the fastnum atan column as a real-world atan
benchmark.

### atan input-class comparison at D38<19>

A per-input bench (`benches/atan_inputs.rs`) compares the three
libraries that actually expose `atan` (decimal-scaled, fastnum,
g_math) across eight input classes. Times shown are criterion
medians on the same machine, with our zero / ±1 / small-x fast
paths in effect.

| input | decimal-scaled | fastnum         | g_math   |
|-------|----------------|-----------------|----------|
| 0     | 4.9 ns         | 1.4 ns          | 11.7 µs  |
| 1     | 552 ns ⚠       | 12.1 ns         | 11.6 µs  |
| −1    | 551 ns ⚠       | 13.1 ns         | 11.1 µs  |
| 1e-7  | **5.3 ns** 🏆  | 8.2 µs          | 1.06 µs  |
| 0.001 | 44.5 µs        | 11.1 µs         | 1.9 µs   |
| 0.5   | 62.4 µs        | 47.9 µs         | 12.4 µs  |
| 2     | 65.7 µs        | 12 ns ‡         | 13.0 µs  |
| 1e8   | 44.0 µs        | 6.2 ns ‡        | 12.3 µs  |

‡ fastnum returns signalling NaN for |x| > 1 (see previous
subsection); the 12 ns / 6 ns figures are early-return, not an
atan computation.

**Honest reading**:

- We're best-in-class at `atan(0)` and `atan(small)` thanks to the
  zero and small-x fast paths added in this round; **neither peer
  has a small-x linear shortcut**, so we're ~200× faster than g_math
  and ~1500× faster than fastnum at 1e-7.
- g_math is genuinely the perf leader on normal atan (~12 µs flat
  regardless of input class — almost certainly CORDIC or a
  precomputed Chebyshev expansion); we are 3-5× behind it on every
  real input.
- fastnum is 1.5-4× faster than us in the in-range non-special
  cases where it actually computes. The real algorithmic gap to
  fastnum is much smaller than the 5000× headline would suggest.
- `atan(±1)` at 552 ns is our remaining bottleneck — `Self::quarter_pi()`
  goes through a non-const multi-limb rescale per call. Making
  `quarter_pi_at_target` `const fn` would drop this to < 30 ns.

Headline reading: `decimal-scaled` is the only library that
**simultaneously** (a) ships 0-ULP HalfToEven by default,
**without** the caller having to switch rounding modes or
re-render at a different scale, and (b) keeps add / sub / neg
at the cost of a primitive `i128` instruction. `fastnum` is the
closest peer — it computes correctly at full internal precision
and trades only a render-mode choice; its transcendentals are
substantially faster than ours at D128 because fastnum's series
runs at fixed 38-digit precision instead of our `SCALE + GUARD`
working scale. The crates with real accuracy losses
(`dashu-float`, `g_math`) are slower *and* less accurate;
the crates with heap arithmetic (`bigdecimal`, `dashu-float`)
trade 10×–100× on add / sub / neg.

Where each crate fits at I128:

- **decimal-scaled / fastnum / rust_decimal / decimal-rs /
  fixed::I64F64** all live in the ns range for arithmetic.
  Pick on shape: SCALE generic + `no_std` (us); fixed precision
  with very fast transcendentals (fastnum); NUMERIC-shape
  (rust_decimal); plain i128 + scale (decimal-rs); binary
  fixed-point (fixed::I64F64).
- **bigdecimal / dashu-float** live an order of magnitude up
  because every op touches the heap. Use them when you need
  arbitrary precision at *runtime*; otherwise the stack peers
  fit the same shape with less overhead.
- **g_math** sits sub-µs on transcendentals but at 6–46 ULP off
  the correctly-rounded value at this precision (see "Accuracy
  at 128-bit" below). Fits the FP-expression-DSL workflow it's
  designed for; not a fit when last-digit correctness matters.

Inside the stack-decimal cluster, the transcendental costs
diverge: `decimal-scaled`'s strict path pays `SCALE + GUARD`
working precision (so its `ln` is µs-scale here, vs `fastnum`'s
tens of ns at fixed internal precision). Whether that cost is
worth paying depends on whether you need HalfToEven-at-storage
by default or are happy to manage the render mode yourself.

### I256 — 256-bit storage at scale 35

The candidate set narrows. `rust_decimal` / `decimal-rs` /
`fixed::I64F64` / `g_math` aren't available at this width.

![operations @ 256-bit at scale 35](figures/library_comparison/summary_256bit.png)

Three crates fit at I256:

- **decimal-scaled** — when you need a stack-allocated decimal
  with a compile-time SCALE generic and HalfToEven-by-default
  transcendentals.
- **fastnum** (D256) — when you need stack-allocated decimal
  arithmetic at fixed 75-digit internal precision and are
  happy to render transcendentals at whichever scale your
  application chooses. fastnum's `ln` and `sqrt` run an order
  of magnitude faster than ours here because its series cost
  doesn't grow with the user's SCALE.
- **dashu-float / bigdecimal** — when you need arbitrary
  runtime precision and the heap-allocation cost is acceptable.

### I512 — 512-bit storage at scale 75

![operations @ 512-bit at scale 75](figures/library_comparison/summary_512bit.png)

Same three-way fit as I256, scaled up. The within-crate trade
shifts a little: decimal-scaled's strict `exp` and `sin` start
to beat fastnum because the [0, π/4] reduction and the sin_cos
joint kernel benefit more from the wider working scale than
fastnum's fixed 155-digit series benefits from its fixed
precision.

### I1024 — 1024-bit storage at scale 150

![operations @ 1024-bit at scale 150](figures/library_comparison/summary_1024bit.png)

Beyond 1024-bit no fixed-precision stack peer remains. The
choice reduces to:

- **decimal-scaled** for stack + compile-time SCALE + 0-ULP
  transcendentals;
- **bigdecimal / dashu-float** for heap arbitrary precision.

`dashu-float` is cheapest on raw arithmetic (single heap
arbitrary-precision call per op, scale-flat); decimal-scaled
keeps arithmetic stack-allocated in the ns range and is the
only one of the three that ships transcendentals competitive
with what you'd want at 1024-bit precision.

### I2048 — 2048-bit storage at scale 308

x-wide territory. Same two-way choice as I1024.

![operations @ 2048-bit at scale 308](figures/library_comparison/summary_2048bit.png)

decimal-scaled is the only stack option at this width; the
chart's three bars per op are us + the two heap libraries.
`bigdecimal` ships no transcendentals; `dashu-float` ships them
but with multi-ULP rounding error.

### I4096 — 4096-bit storage at scale 616

xx-wide territory. Same shape as I2048 with everything an
order of magnitude slower in absolute terms.

![operations @ 4096-bit at scale 616](figures/library_comparison/summary_4096bit.png)

Use decimal-scaled at this width when you want fixed-at-compile-
time 1231-digit precision on the stack. Use dashu-float when
you want dynamic precision and heap allocation is fine.
`dashu-float`'s ln / exp weren't benched at 4096-bit (projection
from 3072-bit puts them at ~8–10 ms vs decimal-scaled's
0.4–0.7 ms).

### A note on what "0 ULP" means here

The strict transcendentals in this crate are 0-ULP **at storage
scale**, by default, under **HalfToEven** — call `.ln_strict()`,
get the IEEE 754 default rounding of the true result, no
render-time configuration required.

Several peers (`fastnum`, `rust_decimal`, `decimal-rs`) carry
the same true value internally — fastnum to 38 / 75 / 155
digits at D128 / D256 / D512, rust_decimal to its full 96-bit
mantissa — but render at the user's scale using a different
rounding mode (Trunc / Floor-equivalent). The "1 ULP" cells
attributed to these crates in the older measurements were
render-mode mismatches, not computation errors. If your
application controls the render mode (or re-rounds explicitly),
those crates are 0-ULP-equivalent.

`dashu-float`'s `exp(1)` at p=19 is 4 ULP from a correctly-
rounded HalfToEven answer. That's a genuine algorithmic gap:
its precision context controls the *result* width, not the
*working* width, so its series can fall short of the guard
digits a correctly-rounded final answer needs.

`g_math`'s transcendentals are 6–46 ULP off the correctly-
rounded value at D128<19>. Its "0 ULP transcendentals"
marketing claim doesn't hold up at this precision. It's still
a useful tool inside its expression-DSL workflow when an
approximate result is acceptable.

---

## 6. Reference: wide-integer backends

For raw signed integer arithmetic without the decimal layer see
`benches/wide_int_backends.rs`. Summary at this revision:

| op | `Int256` (this crate) | `bnum` I256 | `ruint` U256 |
|---|---|---|---|
| add | **1.51 ns** | 1.74 ns | 5.54 ns |
| sub | **1.77 ns** | 1.78 ns | 5.53 ns |
| mul | 13.57 ns | 3.57 ns | **3.19 ns** |
| div | 14.43 ns | 61.30 ns | **4.96 ns** |
| rem | 14.13 ns | 60.62 ns | **5.20 ns** |
| neg | **1.63 ns** | 4.29 ns | - |

At 1024 bits the native back-end takes div / rem on its own
(`bnum`'s falls off ~4×); `ruint` doesn't ship a 1024-bit type.

---

## Methodology

- **Bench runner.** Criterion. Each row's measurement is the
  median wall-clock; warm-up 3 s (criterion default), measurement
  window auto-tuned per function (5 s for cheap ops, scaled up to
  ~110 s for the deepest D307 strict transcendentals). Sample
  size 50 for arithmetic and D38-and-narrower strict; 20 for the
  wide-tier strict block where each iteration is expensive.
- **Operand choice.** Arithmetic: `from_int(2)` and `from_int(1)`
  - universally in range at every width and scale. Transcendentals:
  `1.5` (= `from_int(1) + from_int(1)/from_int(2)`) for ln / sin /
  sqrt, `0.5` for exp - sized to stay in range at every type×scale
  combo (at the old pre-0.4.0 `D38<38>` cap, the ≈ 1.7 ceiling was
  the binding constraint; under 0.4.0 the max is `D38<37>` so the
  ceiling sits at ≈ 17).
- **`black_box`.** Every input is wrapped in `std::hint::black_box`;
  the closure returns the result so the optimiser cannot drop the
  call.
- **Build profile.** `bench` (= `release` with `opt-level=3`,
  no debug-assertions).
- **Default features.** Stock `wide` + `x-wide` + `strict`
  enabled (crate defaults). The fast block calls `*_fast`
  explicitly (e.g. `.ln_fast()`) and the strict block calls
  `*_strict` explicitly, so both paths are exercised
  unambiguously regardless of which dispatcher the plain `*`
  methods resolve to under the active feature set.

---

## Roadmap

`decimal-scaled` already wins the narrow tier (D9 / D18 / D38)
and the 0-ULP accuracy column at every tier. The honest losses
are at D76 and above on `mul` / `div`, and on the throughput of
the correctly-rounded wide-tier transcendentals. They aren't
fundamental - they're algorithmic catch-up work, each with a
known fix waiting to be implemented:

- **Wide-tier `÷ 10^SCALE`** - Burnikel–Ziegler recursive
  divide and a Newton-reciprocal fast path are the right
  asymptote for D153+. Today's MG magic-multiply pays a
  serialised carry-propagation cost above D38.
- **Wide-tier `mul`** - Karatsuba (D153) and Toom-3 (D307)
  haven't been wired up yet; the kernel is straight schoolbook
  on the limb array.
- **Wide-tier transcendentals** - a planned `*_approx(working_digits)`
  family lets callers buy back throughput when they don't need
  the 0-ULP guarantee, without falling off the f64-bridge
  precision cliff that today's `*_fast` has at wide widths.

See [`ROADMAP.md`](ROADMAP.md) at the repo root for the full
list with expected wins per item and current status.

## History — cross-version improvement

Same harness, same input distributions across every cell — the
only thing changing per cell is the `decimal-scaled` dependency
version. The shipped harness lives in
[`bench-history/`](https://github.com/mootable/decimal-scaled/tree/main/bench-history)
and is driven by
[`bench-history.yml`](https://github.com/mootable/decimal-scaled/blob/main/.github/workflows/bench-history.yml);
each charted version is `cargo add`ed into a stub crate and
benched with the same Criterion settings. v0.3.0 / v0.3.1 /
v0.4.0 / v0.4.1 are omitted; widths and functions are
deliberately minimal per the bench-history scope note. The
charted endpoint labelled v0.4.2 is the current HEAD —
v0.4.0 / v0.4.1 are not on crates.io as separate
bench-history rows because each was a transient release in
the 0.4 cycle; v0.4.2 is the first 0.4 release shape that
moves the needle vs the v0.3.x baseline (Tang `ln` lookup
ladder, narrow-GUARD trig family, reciprocal-divide
hyperbolic identity).

![history at D38](figures/history/d38.png)

![history at D76](figures/history/d76.png)

![history at D307](figures/history/d307.png)

## 0.4.2 sweep — full raw data

Complete dump of the v0.4.2 full_matrix sweep (GitHub Actions run
[`26099350473`](https://github.com/mootable/decimal-scaled/actions/runs/26099350473)),
one table per bench binary. **Median** is criterion's median
per-iteration time. The `Change vs prior` column has been
dropped from this dump: cross-revision deltas measured across
shared CI runners conflate real code changes with noisy
neighbours and 1.5–2× wall-clock variance per the upstream
Criterion guidance.

**Run conditions**: GitHub-hosted `ubuntu-latest` standard
runners — 2 vCPU shared, 7 GiB RAM, no core pinning, no
priority bump. Each per-width matrix shard runs in its own VM
on its own runner so cross-shard contention is zero, but
within a shard the timer floor is the standard-runner steal
budget. Numbers here are valid **relative to each other within
this run** (matched algorithm-of-record families, matched
features, matched compiler); they are **not** directly
comparable to a cold dev-box micro-bench, which typically
measures 30–50 % faster on the wide tiers and several × faster
on the narrow tiers' picosecond cells.

Picosecond-scale narrow-tier cells (D9 / D18 / D38 `add` /
`sub` / `neg`) sit near the runner's resolution floor; treat
their absolute values as accurate to ±20 % at best. Wide-tier
ns+ values are reliable.

### `full_matrix_d9` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D9_s0/add` | 594.24 ps |
| `arith/D9_s0/sub` | 601.54 ps |
| `arith/D9_s0/mul` | 596.66 ps |
| `arith/D9_s0/div` | 1.727 ns |
| `arith/D9_s0/rem` | 1.727 ns |
| `arith/D9_s0/neg` | 346.79 ps |
| `arith/D9_s5/add` | 587.98 ps |
| `arith/D9_s5/sub` | 597.18 ps |
| `arith/D9_s5/mul` | 1.13 ns |
| `arith/D9_s5/div` | 2.881 ns |
| `arith/D9_s5/rem` | 1.735 ns |
| `arith/D9_s5/neg` | 346.84 ps |
| `arith/D9_s9/add` | 586.82 ps |
| `arith/D9_s9/sub` | 588.83 ps |
| `arith/D9_s9/mul` | 1.057 ns |
| `arith/D9_s9/div` | 2.878 ns |
| `arith/D9_s9/rem` | 1.728 ns |
| `arith/D9_s9/neg` | 346.94 ps |
| `strict/D9_s0/ln` | 461.06 ps |
| `strict/D9_s0/exp` | 582.80 ps |
| `strict/D9_s0/sin` | 1.584 ns |
| `strict/D9_s0/sqrt` | 7.218 ns |
| `strict/D9_s5/ln` | 4.01 µs |
| `strict/D9_s5/exp` | 3.626 µs |
| `strict/D9_s5/sin` | 4.483 µs |
| `strict/D9_s5/sqrt` | 21.38 ns |
| `strict/D9_s9/ln` | 4.684 µs |
| `strict/D9_s9/exp` | 4.134 µs |
| `strict/D9_s9/sin` | 3.337 µs |
| `strict/D9_s9/sqrt` | 21.42 ns |

### `full_matrix_d18` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D18_s0/add` | 933.63 ps |
| `arith/D18_s0/sub` | 943.41 ps |
| `arith/D18_s0/mul` | 933.84 ps |
| `arith/D18_s0/div` | 1.867 ns |
| `arith/D18_s0/rem` | 1.867 ns |
| `arith/D18_s0/neg` | 622.78 ps |
| `arith/D18_s9/add` | 933.46 ps |
| `arith/D18_s9/sub` | 933.82 ps |
| `arith/D18_s9/mul` | 2.256 ns |
| `arith/D18_s9/div` | 2.335 ns |
| `arith/D18_s9/rem` | 1.867 ns |
| `arith/D18_s9/neg` | 622.76 ps |
| `arith/D18_s18/add` | 933.59 ps |
| `arith/D18_s18/sub` | 933.61 ps |
| `arith/D18_s18/mul` | 4.233 ns |
| `arith/D18_s18/div` | 6.068 ns |
| `arith/D18_s18/rem` | 2.178 ns |
| `arith/D18_s18/neg` | 622.76 ps |
| `arith/fixed_i64f64/add` | 1.867 ns |
| `arith/fixed_i64f64/sub` | 1.867 ns |
| `arith/fixed_i64f64/mul` | 2.489 ns |
| `arith/fixed_i64f64/div` | 30.11 ns |
| `arith/fixed_i64f64/rem` | 13.08 ns |
| `arith/fixed_i64f64/neg` | 1.244 ns |
| `strict/D18_s0/ln` | 622.74 ps |
| `strict/D18_s0/exp` | 623.25 ps |
| `strict/D18_s0/sin` | 1.246 ns |
| `strict/D18_s0/sqrt` | 6.874 ns |
| `strict/D18_s9/ln` | 6.493 µs |
| `strict/D18_s9/exp` | 6.332 µs |
| `strict/D18_s9/sin` | 5.506 µs |
| `strict/D18_s9/sqrt` | 19.69 ns |
| `strict/D18_s18/ln` | 7.921 µs |
| `strict/D18_s18/exp` | 7.552 µs |
| `strict/D18_s18/sin` | 6.659 µs |
| `strict/D18_s18/sqrt` | 30.68 ns |

### `full_matrix_d38` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D38_s0/add` | 2.109 ns |
| `arith/D38_s0/sub` | 2.108 ns |
| `arith/D38_s0/mul` | 3.515 ns |
| `arith/D38_s0/div` | 7.031 ns |
| `arith/D38_s0/rem` | 7.033 ns |
| `arith/D38_s0/neg` | 1.406 ns |
| `arith/D38_s19/add` | 2.109 ns |
| `arith/D38_s19/sub` | 2.109 ns |
| `arith/D38_s19/mul` | 10.52 ns |
| `arith/D38_s19/div` | 8.244 ns |
| `arith/D38_s19/rem` | 7.033 ns |
| `arith/D38_s19/neg` | 1.405 ns |
| `arith/D38_s38/add` | 2.109 ns |
| `arith/D38_s38/sub` | 2.108 ns |
| `arith/D38_s38/mul` | 23.46 ns |
| `arith/D38_s38/div` | 1.082 µs |
| `arith/D38_s38/rem` | 9.732 ns |
| `arith/D38_s38/neg` | 1.406 ns |
| `arith/rust_decimal_s19/add` | 11.95 ns |
| `arith/rust_decimal_s19/sub` | 13.14 ns |
| `arith/rust_decimal_s19/mul` | 36.85 ns |
| `arith/rust_decimal_s19/div` | 9.792 ns |
| `arith/rust_decimal_s19/rem` | 21.57 ns |
| `arith/rust_decimal_s19/neg` | 2.789 ns |
| `strict/D38_s0/ln` | 1.406 ns |
| `strict/D38_s0/exp` | 1.406 ns |
| `strict/D38_s0/sin` | 1.407 ns |
| `strict/D38_s0/sqrt` | 7.531 ns |
| `strict/D38_s19/ln` | 8.732 µs |
| `strict/D38_s19/exp` | 8.382 µs |
| `strict/D38_s19/sin` | 7.346 µs |
| `strict/D38_s19/sqrt` | 33.2 ns |
| `strict/D38_s38/ln` | 9.745 µs |
| `strict/D38_s38/exp` | 10.88 µs |
| `strict/D38_s38/sin` | 10.54 µs |
| `strict/D38_s38/sqrt` | 7.489 µs |

### `full_matrix_d57` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D57_s0/add` | 2.988 ns |
| `arith/D57_s0/sub` | 2.995 ns |
| `arith/D57_s0/mul` | 15.27 ns |
| `arith/D57_s0/div` | 73.26 ns |
| `arith/D57_s0/rem` | 24.68 ns |
| `arith/D57_s0/neg` | 2.1 ns |
| `arith/D57_s28/add` | 2.988 ns |
| `arith/D57_s28/sub` | 2.993 ns |
| `arith/D57_s28/mul` | 91.08 ns |
| `arith/D57_s28/div` | 301.6 ns |
| `arith/D57_s28/rem` | 119.1 ns |
| `arith/D57_s28/neg` | 2.098 ns |
| `arith/D57_s56/add` | 2.978 ns |
| `arith/D57_s56/sub` | 2.998 ns |
| `arith/D57_s56/mul` | 141.8 ns |
| `arith/D57_s56/div` | 544.9 ns |
| `arith/D57_s56/rem` | 133.2 ns |
| `arith/D57_s56/neg` | 2.098 ns |
| `strict_wide/D57_s0/ln` | 3.24 µs |
| `strict_wide/D57_s0/exp` | 24.97 ns |
| `strict_wide/D57_s0/sin` | 11.76 µs |
| `strict_wide/D57_s0/sqrt` | 87.72 ns |
| `strict_wide/D57_s28/ln` | 20.89 µs |
| `strict_wide/D57_s28/exp` | 18.02 µs |
| `strict_wide/D57_s28/sin` | 12.82 µs |
| `strict_wide/D57_s28/sqrt` | 1.073 µs |
| `strict_wide/D57_s56/ln` | 27.41 µs |
| `strict_wide/D57_s56/exp` | 20.63 µs |
| `strict_wide/D57_s56/sin` | 22.29 µs |
| `strict_wide/D57_s56/sqrt` | 1.448 µs |

### `full_matrix_d76` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D76_s0/add` | 3.71 ns |
| `arith/D76_s0/sub` | 3.723 ns |
| `arith/D76_s0/mul` | 16.59 ns |
| `arith/D76_s0/div` | 68.33 ns |
| `arith/D76_s0/rem` | 26.18 ns |
| `arith/D76_s0/neg` | 2.814 ns |
| `arith/D76_s35/add` | 3.705 ns |
| `arith/D76_s35/sub` | 3.722 ns |
| `arith/D76_s35/mul` | 83.48 ns |
| `arith/D76_s35/div` | 360.5 ns |
| `arith/D76_s35/rem` | 171.7 ns |
| `arith/D76_s35/neg` | 2.814 ns |
| `arith/D76_s76/add` | 3.712 ns |
| `arith/D76_s76/sub` | 3.725 ns |
| `arith/D76_s76/mul` | 153.1 ns |
| `arith/D76_s76/div` | 633.4 ns |
| `arith/D76_s76/rem` | 179.1 ns |
| `arith/D76_s76/neg` | 2.814 ns |
| `arith/bnum_d76_s35/add` | 12.16 ns |
| `arith/bnum_d76_s35/sub` | 12.1 ns |
| `arith/bnum_d76_s35/mul` | 472.2 ns |
| `arith/bnum_d76_s35/div` | 482.6 ns |
| `arith/bnum_d76_s35/rem` | 70.01 ns |
| `arith/bnum_d76_s35/neg` | 17.04 ns |
| `strict_wide/D76_s0/ln` | 3.415 µs |
| `strict_wide/D76_s0/exp` | 26.19 ns |
| `strict_wide/D76_s0/sin` | 14.68 µs |
| `strict_wide/D76_s0/sqrt` | 106.8 ns |
| `strict_wide/D76_s35/ln` | 26.19 µs |
| `strict_wide/D76_s35/exp` | 24.59 µs |
| `strict_wide/D76_s35/sin` | 17.27 µs |
| `strict_wide/D76_s35/sqrt` | 1.724 µs |
| `strict_wide/D76_s76/ln` | 38.15 µs |
| `strict_wide/D76_s76/exp` | 31.85 µs |
| `strict_wide/D76_s76/sin` | 25.99 µs |
| `strict_wide/D76_s76/sqrt` | 2.296 µs |

### `full_matrix_d115` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D115_s0/add` | 5.714 ns |
| `arith/D115_s0/sub` | 6.562 ns |
| `arith/D115_s0/mul` | 24.33 ns |
| `arith/D115_s0/div` | 83.6 ns |
| `arith/D115_s0/rem` | 32.02 ns |
| `arith/D115_s0/neg` | 3.777 ns |
| `arith/D115_s57/add` | 5.705 ns |
| `arith/D115_s57/sub` | 6.558 ns |
| `arith/D115_s57/mul` | 130.3 ns |
| `arith/D115_s57/div` | 411.9 ns |
| `arith/D115_s57/rem` | 180.5 ns |
| `arith/D115_s57/neg` | 3.779 ns |
| `arith/D115_s114/add` | 5.704 ns |
| `arith/D115_s114/sub` | 6.564 ns |
| `arith/D115_s114/mul` | 266.4 ns |
| `arith/D115_s114/div` | 762.2 ns |
| `arith/D115_s114/rem` | 192 ns |
| `arith/D115_s114/neg` | 3.777 ns |
| `strict_wide/D115_s0/ln` | 3.4 µs |
| `strict_wide/D115_s0/exp` | 38.4 ns |
| `strict_wide/D115_s0/sin` | 14.76 µs |
| `strict_wide/D115_s0/sqrt` | 161.9 ns |
| `strict_wide/D115_s57/ln` | 2.462 µs |
| `strict_wide/D115_s57/exp` | 21.66 µs |
| `strict_wide/D115_s57/sin` | 21.84 µs |
| `strict_wide/D115_s57/sqrt` | 2.445 µs |
| `strict_wide/D115_s114/ln` | 49.55 µs |
| `strict_wide/D115_s114/exp` | 39.9 µs |
| `strict_wide/D115_s114/sin` | 34.22 µs |
| `strict_wide/D115_s114/sqrt` | 3.252 µs |

### `full_matrix_d153` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D153_s0/add` | 8.146 ns |
| `arith/D153_s0/sub` | 15.69 ns |
| `arith/D153_s0/mul` | 54.96 ns |
| `arith/D153_s0/div` | 129.3 ns |
| `arith/D153_s0/rem` | 42.85 ns |
| `arith/D153_s0/neg` | 5.563 ns |
| `arith/D153_s75/add` | 8.145 ns |
| `arith/D153_s75/sub` | 15.68 ns |
| `arith/D153_s75/mul` | 200 ns |
| `arith/D153_s75/div` | 724.6 ns |
| `arith/D153_s75/rem` | 144.8 ns |
| `arith/D153_s75/neg` | 5.557 ns |
| `arith/D153_s153/add` | 8.147 ns |
| `arith/D153_s153/sub` | 15.7 ns |
| `arith/D153_s153/mul` | 512.1 ns |
| `arith/D153_s153/div` | 1.5 µs |
| `arith/D153_s153/rem` | 156.4 ns |
| `arith/D153_s153/neg` | 5.556 ns |
| `strict_wide/D153_s0/ln` | 4.333 µs |
| `strict_wide/D153_s0/exp` | 45.22 ns |
| `strict_wide/D153_s0/sin` | 16.82 µs |
| `strict_wide/D153_s0/sqrt` | 229.9 ns |
| `strict_wide/D153_s75/ln` | 3.483 µs |
| `strict_wide/D153_s75/exp` | 31.93 µs |
| `strict_wide/D153_s75/sin` | 25.34 µs |
| `strict_wide/D153_s75/sqrt` | 2.398 µs |
| `strict_wide/D153_s153/ln` | 78.33 µs |
| `strict_wide/D153_s153/exp` | 64.14 µs |
| `strict_wide/D153_s153/sin` | 52.91 µs |
| `strict_wide/D153_s153/sqrt` | 3.378 µs |

### `full_matrix_d230` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D230_s0/add` | 16.1 ns |
| `arith/D230_s0/sub` | 18.67 ns |
| `arith/D230_s0/mul` | 49.38 ns |
| `arith/D230_s0/div` | 173.1 ns |
| `arith/D230_s0/rem` | 50.14 ns |
| `arith/D230_s0/neg` | 10.13 ns |
| `arith/D230_s115/add` | 16.09 ns |
| `arith/D230_s115/sub` | 18.67 ns |
| `arith/D230_s115/mul` | 333.6 ns |
| `arith/D230_s115/div` | 775.2 ns |
| `arith/D230_s115/rem` | 132.6 ns |
| `arith/D230_s115/neg` | 10.2 ns |
| `arith/D230_s230/add` | 16.1 ns |
| `arith/D230_s230/sub` | 18.68 ns |
| `arith/D230_s230/mul` | 870.8 ns |
| `arith/D230_s230/div` | 1.689 µs |
| `arith/D230_s230/rem` | 155.1 ns |
| `arith/D230_s230/neg` | 10.65 ns |
| `strict_wide/D230_s0/ln` | 4.835 µs |
| `strict_wide/D230_s0/exp` | 64.14 ns |
| `strict_wide/D230_s0/sin` | 17.33 µs |
| `strict_wide/D230_s0/sqrt` | 242.1 ns |
| `strict_wide/D230_s115/ln` | 62.98 µs |
| `strict_wide/D230_s115/exp` | 55.26 µs |
| `strict_wide/D230_s115/sin` | 43.11 µs |
| `strict_wide/D230_s115/sqrt` | 3.096 µs |
| `strict_wide/D230_s230/ln` | 118.4 µs |
| `strict_wide/D230_s230/exp` | 96.22 µs |
| `strict_wide/D230_s230/sin` | 83.98 µs |
| `strict_wide/D230_s230/sqrt` | 4.975 µs |

### `full_matrix_d307` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D307_s0/add` | 21.94 ns |
| `arith/D307_s0/sub` | 25.49 ns |
| `arith/D307_s0/mul` | 61.41 ns |
| `arith/D307_s0/div` | 181.5 ns |
| `arith/D307_s0/rem` | 62.78 ns |
| `arith/D307_s0/neg` | 11.48 ns |
| `arith/D307_s150/add` | 21.93 ns |
| `arith/D307_s150/sub` | 25.49 ns |
| `arith/D307_s150/mul` | 438.3 ns |
| `arith/D307_s150/div` | 963.7 ns |
| `arith/D307_s150/rem` | 154.6 ns |
| `arith/D307_s150/neg` | 11.87 ns |
| `arith/D307_s307/add` | 21.93 ns |
| `arith/D307_s307/sub` | 25.49 ns |
| `arith/D307_s307/mul` | 1.422 µs |
| `arith/D307_s307/div` | 2.571 µs |
| `arith/D307_s307/rem` | 179.9 ns |
| `arith/D307_s307/neg` | 12.68 ns |
| `strict_wide/D307_s0/ln` | 6.095 µs |
| `strict_wide/D307_s0/exp` | 71.78 ns |
| `strict_wide/D307_s0/sin` | 21.44 µs |
| `strict_wide/D307_s0/sqrt` | 318 ns |
| `strict_wide/D307_s150/ln` | 5.968 µs |
| `strict_wide/D307_s150/exp` | 83.12 µs |
| `strict_wide/D307_s150/sin` | 59.76 µs |
| `strict_wide/D307_s150/sqrt` | 3.704 µs |
| `strict_wide/D307_s307/ln` | 197 µs |
| `strict_wide/D307_s307/exp` | 160 µs |
| `strict_wide/D307_s307/sin` | 145.7 µs |
| `strict_wide/D307_s307/sqrt` | 8.425 µs |

### `full_matrix_d462` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D462_s0/add` | 36.01 ns |
| `arith/D462_s0/sub` | 44.83 ns |
| `arith/D462_s0/mul` | 123.3 ns |
| `arith/D462_s0/div` | 358.2 ns |
| `arith/D462_s0/rem` | 104.2 ns |
| `arith/D462_s0/neg` | 36.08 ns |
| `arith/D462_s230/add` | 35.99 ns |
| `arith/D462_s230/sub` | 44.85 ns |
| `arith/D462_s230/mul` | 1.031 µs |
| `arith/D462_s230/div` | 2.083 µs |
| `arith/D462_s230/rem` | 276.8 ns |
| `arith/D462_s230/neg` | 36.21 ns |
| `arith/D462_s461/add` | 36 ns |
| `arith/D462_s461/sub` | 44.84 ns |
| `arith/D462_s461/mul` | 3.081 µs |
| `arith/D462_s461/div` | 5.725 µs |
| `arith/D462_s461/rem` | 321.4 ns |
| `arith/D462_s461/neg` | 37.21 ns |
| `strict_wide/D462_s0/ln` | 7.767 µs |
| `strict_wide/D462_s0/exp` | 124.6 ns |
| `strict_wide/D462_s0/sin` | 29.23 µs |
| `strict_wide/D462_s0/sqrt` | 605.8 ns |
| `strict_wide/D462_s230/ln` | 8.327 µs |
| `strict_wide/D462_s230/exp` | 146.6 µs |
| `strict_wide/D462_s230/sin` | 120.7 µs |
| `strict_wide/D462_s230/sqrt` | 8.831 µs |
| `strict_wide/D462_s461/ln` | 386.1 µs |
| `strict_wide/D462_s461/exp` | 302 µs |
| `strict_wide/D462_s461/sin` | 302.1 µs |
| `strict_wide/D462_s461/sqrt` | 20.95 µs |

### `full_matrix_d616` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D616_s0/add` | 59.89 ns |
| `arith/D616_s0/sub` | 76.87 ns |
| `arith/D616_s0/mul` | 118.1 ns |
| `arith/D616_s0/div` | 333.3 ns |
| `arith/D616_s0/rem` | 96.23 ns |
| `arith/D616_s0/neg` | 37.77 ns |
| `arith/D616_s308/add` | 59.99 ns |
| `arith/D616_s308/sub` | 76.83 ns |
| `arith/D616_s308/mul` | 1.56 µs |
| `arith/D616_s308/div` | 4.238 µs |
| `arith/D616_s308/rem` | 191.5 ns |
| `arith/D616_s308/neg` | 39.24 ns |
| `arith/D616_s615/add` | 59.93 ns |
| `arith/D616_s615/sub` | 76.85 ns |
| `arith/D616_s615/mul` | 4.53 µs |
| `arith/D616_s615/div` | 6.92 µs |
| `arith/D616_s615/rem` | 259.4 ns |
| `arith/D616_s615/neg` | 41.84 ns |
| `strict_wide/D616_s0/ln` | 9.251 µs |
| `strict_wide/D616_s0/exp` | 135.8 ns |
| `strict_wide/D616_s0/sin` | 30.71 µs |
| `strict_wide/D616_s0/sqrt` | 601 ns |
| `strict_wide/D616_s308/ln` | 14.11 µs |
| `strict_wide/D616_s308/exp` | 251.9 µs |
| `strict_wide/D616_s308/sin` | 213.5 µs |
| `strict_wide/D616_s308/sqrt` | 10.75 µs |
| `strict_wide/D616_s615/ln` | 668.6 µs |
| `strict_wide/D616_s615/exp` | 558.9 µs |
| `strict_wide/D616_s615/sin` | 562.5 µs |
| `strict_wide/D616_s615/sqrt` | 23.28 µs |

### `full_matrix_d924` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D924_s0/add` | 95.54 ns |
| `arith/D924_s0/sub` | 141.4 ns |
| `arith/D924_s0/mul` | 183.1 ns |
| `arith/D924_s0/div` | 591.2 ns |
| `arith/D924_s0/rem` | 172 ns |
| `arith/D924_s0/neg` | 67.94 ns |
| `arith/D924_s461/add` | 95.2 ns |
| `arith/D924_s461/sub` | 141.4 ns |
| `arith/D924_s461/mul` | 3.463 µs |
| `arith/D924_s461/div` | 5.754 µs |
| `arith/D924_s461/rem` | 396.5 ns |
| `arith/D924_s461/neg` | 68.68 ns |
| `arith/D924_s923/add` | 95.23 ns |
| `arith/D924_s923/sub` | 141.3 ns |
| `arith/D924_s923/mul` | 11.1 µs |
| `arith/D924_s923/div` | 15.43 µs |
| `arith/D924_s923/rem` | 480.3 ns |
| `arith/D924_s923/neg` | 70.85 ns |
| `strict_wide/D924_s0/ln` | 16.67 µs |
| `strict_wide/D924_s0/exp` | 353.8 ns |
| `strict_wide/D924_s0/sin` | 55.15 µs |
| `strict_wide/D924_s0/sqrt` | 1.005 µs |
| `strict_wide/D924_s461/ln` | 28.36 µs |
| `strict_wide/D924_s461/exp` | 631.2 µs |
| `strict_wide/D924_s461/sin` | 603.5 µs |
| `strict_wide/D924_s461/sqrt` | 27.57 µs |
| `strict_wide/D924_s923/ln` | 1.853 ms |
| `strict_wide/D924_s923/exp` | 1.517 ms |
| `strict_wide/D924_s923/sin` | 1.712 ms |
| `strict_wide/D924_s923/sqrt` | 61.14 µs |

### `full_matrix_d1232` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D1232_s0/add` | 121.9 ns |
| `arith/D1232_s0/sub` | 172.1 ns |
| `arith/D1232_s0/mul` | 232.3 ns |
| `arith/D1232_s0/div` | 769.1 ns |
| `arith/D1232_s0/rem` | 219.9 ns |
| `arith/D1232_s0/neg` | 95.56 ns |
| `arith/D1232_s616/add` | 121.7 ns |
| `arith/D1232_s616/sub` | 172 ns |
| `arith/D1232_s616/mul` | 6.099 µs |
| `arith/D1232_s616/div` | 8.679 µs |
| `arith/D1232_s616/rem` | 468.8 ns |
| `arith/D1232_s616/neg` | 95.38 ns |
| `arith/D1232_s1231/add` | 121.8 ns |
| `arith/D1232_s1231/sub` | 172.3 ns |
| `arith/D1232_s1231/mul` | 19.64 µs |
| `arith/D1232_s1231/div` | 24.51 µs |
| `arith/D1232_s1231/rem` | 602.1 ns |
| `arith/D1232_s1231/neg` | 97.25 ns |
| `strict_wide/D1232_s0/ln` | 22.11 µs |
| `strict_wide/D1232_s0/exp` | 473 ns |
| `strict_wide/D1232_s0/sin` | 73.01 µs |
| `strict_wide/D1232_s0/sqrt` | 1.265 µs |
| `strict_wide/D1232_s616/ln` | 46.47 µs |
| `strict_wide/D1232_s616/exp` | 1.217 ms |
| `strict_wide/D1232_s616/sin` | 1.203 ms |
| `strict_wide/D1232_s616/sqrt` | 39.25 µs |
| `strict_wide/D1232_s1231/ln` | 3.735 ms |
| `strict_wide/D1232_s1231/exp` | 3.018 ms |
| `strict_wide/D1232_s1231/sin` | 3.663 ms |
| `strict_wide/D1232_s1231/sqrt` | 96.33 µs |

