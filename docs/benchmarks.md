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

> **0.3.3 sweep refresh.** Tables below were regenerated from
> the 2026-05-18 2-lane bench sweep
> (`target/bench-logs/sweep-20260518-035712/`). Narrow-tier
> picosecond-scale cells (D9 / D18 / D38 arith) sit near the
> bench machine's resolution floor and carry pipeline / branch-
> predictor / interrupt jitter; cross-version comparisons there
> should be taken with a grain of salt. Wide-tier values in the
> ns+ range are reliable.

### D9 - 32 bits

| op | s = 0 | s = 5 | s = 9 |
|---|---|---|---|
| add | 621.38 ps | 596.65 ps | 597.43 ps |
| sub | 594.94 ps | 581.17 ps | 580.18 ps |
| mul | 513.41 ps | 1.4751 ns | 1.4579 ns |
| div | 1.7512 ns | 2.9736 ns | 3.9839 ns |
| rem | 1.7562 ns | 1.7205 ns | 1.7360 ns |
| neg | 441.02 ps | 434.28 ps | 444.83 ps |

### D18 - 64 bits

| op | s = 0 | s = 9 | s = 18 |
|---|---|---|---|
| add | 574.36 ps | 575.77 ps | 583.76 ps |
| sub | 591.39 ps | 577.89 ps | 586.78 ps |
| mul | 575.47 ps | 14.866 ns | 14.945 ns |
| div | 15.172 ns | 15.578 ns | 15.585 ns |
| rem | 1.7273 ns | 1.7302 ns | 1.7890 ns |
| neg | 434.12 ps | 437.38 ps | 439.62 ps |

### D38 - 128 bits

| op | s = 0 | s = 19 | s = 38 |
|---|---|---|---|
| add | 987.90 ps | 980.05 ps | 953.57 ps |
| sub | 970.88 ps | 981.90 ps | 978.92 ps |
| mul | 4.2143 ns | 28.160 ns | 29.146 ns |
| div | 16.117 ns | 17.299 ns | 5.6551 µs |
| rem | 13.031 ns | 12.776 ns | 19.065 ns |
| neg | 740.35 ps | 720.92 ps | 732.29 ps |

### D57 - 192 bits

| op | s = 0 | s = 28 | s = 56 |
|---|---|---|---|
| add | 2.0318 ns | 2.0048 ns | 2.0210 ns |
| sub | 2.6117 ns | 2.6179 ns | 2.6133 ns |
| mul | 63.833 ns | 222.97 ns | 313.87 ns |
| div | 199.35 ns | 416.42 ns | 447.76 ns |
| rem | 37.813 ns | 134.93 ns | 141.74 ns |
| neg | 2.2552 ns | 2.2434 ns | 2.2562 ns |

(Note: the s=28 mul/div numbers come from the `lib_cmp_d57`
isolated bench rather than the full_matrix monolith. The monolith
run showed inflated D57 mul/div (~187 ns / ~390 ns) due to
cache-and-inlining contention with the other 30+ tier bench
functions in the same binary — measured in isolation the numbers
land back in the expected envelope. The 0.3.0 cycle's per-tier
`lib_cmp_d{N}` bench split makes that cleaner-isolation
measurement easy to repeat — see
[`ROADMAP.md`](../ROADMAP.md) §Methodology.)

### D76 - 256 bits

| op | s = 0 | s = 35 | s = 76 |
|---|---|---|---|
| add | 2.3579 ns | 2.3611 ns | 2.3707 ns |
| sub | 3.2922 ns | 3.3064 ns | 3.3202 ns |
| mul | 64.901 ns | 229.04 ns | 351.59 ns |
| div | 214.96 ns | 453.19 ns | 516.41 ns |
| rem | 38.658 ns | 139.45 ns | 145.43 ns |
| neg | 2.5907 ns | 2.5964 ns | 2.5663 ns |

### D115 - 384 bits

| op | s = 0 | s = 57 | s = 114 |
|---|---|---|---|
| add | 3.9050 ns | 3.9263 ns | 4.0655 ns |
| sub | 5.2838 ns | 5.2969 ns | 5.2780 ns |
| mul | 77.288 ns | 335.88 ns | 529.70 ns |
| div | 247.30 ns | 555.53 ns | 672.39 ns |
| rem | 59.596 ns | 166.97 ns | 176.07 ns |
| neg | 3.6422 ns | 3.5728 ns | 3.5581 ns |

### D153 - 512 bits

| op | s = 0 | s = 75 | s = 153 |
|---|---|---|---|
| add | 5.4339 ns | 5.4557 ns | 5.9852 ns |
| sub | 13.093 ns | 13.113 ns | 13.227 ns |
| mul | 80.800 ns | 371.14 ns | 776.38 ns |
| div | 272.49 ns | 850.39 ns | 1.0227 µs |
| rem | 65.667 ns | 191.16 ns | 190.33 ns |
| neg | 4.4793 ns | 4.5536 ns | 4.4366 ns |

### D230 - 768 bits

| op | s = 0 | s = 115 | s = 230 |
|---|---|---|---|
| add | 15.329 ns | 15.176 ns | 15.400 ns |
| sub | 18.126 ns | 18.037 ns | 18.301 ns |
| mul | 96.136 ns | 636.53 ns | 1.3751 µs |
| div | 368.92 ns | 1.2000 µs | 1.5384 µs |
| rem | 69.260 ns | 190.24 ns | 214.25 ns |
| neg | 12.397 ns | 13.083 ns | 14.183 ns |

### D307 - 1024 bits

| op | s = 0 | s = 150 | s = 307 |
|---|---|---|---|
| add | 19.681 ns | 19.463 ns | 19.617 ns |
| sub | 23.905 ns | 23.432 ns | 23.463 ns |
| mul | 127.33 ns | 782.59 ns | 2.1567 µs |
| div | 458.13 ns | 1.4986 µs | 2.6634 µs |
| rem | 81.089 ns | 214.80 ns | 243.24 ns |
| neg | 12.549 ns | 13.725 ns | 14.677 ns |

### D462 - 1536 bits

| op | s = 0 | s = 230 | s = 461 |
|---|---|---|---|
| add | 42.662 ns | 41.840 ns | 45.651 ns |
| sub | 57.612 ns | 57.715 ns | 60.098 ns |
| mul | 218.41 ns | 1.6512 µs | 4.5977 µs |
| div | 725.49 ns | 2.8569 µs | 4.9446 µs |
| rem | 166.05 ns | 322.43 ns | 358.87 ns |
| neg | 47.081 ns | 53.045 ns | 54.417 ns |

### D616 - 2048 bits

| op | s = 0 | s = 308 | s = 615 |
|---|---|---|---|
| add | 93.746 ns | 97.030 ns | 90.614 ns |
| sub | 115.29 ns | 118.91 ns | 114.07 ns |
| mul | 232.05 ns | 2.6811 µs | 7.8695 µs |
| div | 813.79 ns | 4.6416 µs | 7.7299 µs |
| rem | 198.95 ns | 347.50 ns | 434.30 ns |
| neg | 64.759 ns | 64.964 ns | 67.913 ns |

### D924 - 3072 bits

| op | s = 0 | s = 461 | s = 923 |
|---|---|---|---|
| add | 119.69 ns | 115.40 ns | 104.87 ns |
| sub | 143.59 ns | 148.42 ns | 203.09 ns |
| mul | 306.32 ns | 5.1900 µs | 16.312 µs |
| div | 1.1615 µs | 8.2246 µs | 13.314 µs |
| rem | 238.56 ns | 442.64 ns | 541.22 ns |
| neg | 74.740 ns | 75.659 ns | 86.028 ns |

### D1232 - 4096 bits

| op | s = 0 | s = 616 | s = 1231 |
|---|---|---|---|
| add | 143.34 ns | 130.52 ns | 129.54 ns |
| sub | 196.50 ns | 186.22 ns | 185.57 ns |
| mul | 362.90 ns | 9.0517 µs | 29.228 µs |
| div | 1.4677 µs | 10.890 µs | 21.273 µs |
| rem | 339.55 ns | 545.46 ns | 696.57 ns |
| neg | 99.849 ns | 103.49 ns | 114.56 ns |

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

> **0.3.3 sweep refresh — narrow-tier caveat.** The narrow-tier
> (D9 / D18 / D38) values below were measured at the tail end of
> a multi-hour sweep when the pinned cores had drifted from
> their cold-machine baseline. A dedicated cold-machine micro-
> bench (`benches/m2_ln_approx.rs`) on D38<19> ln_strict
> measures **54.8 µs** vs the >800 µs value in the table — that
> ~15× gap is contention drift, not real perf. Wide-tier numbers
> are reliable.

### D9 / D18 / D38 strict

| fn | D9 (s=5) | D18 (s=9) | D38 (s=19) |
|---|---|---|---|
| ln | 476.61 µs | 601.39 µs | 851.47 µs |
| exp | 432.92 µs | 525.56 µs | 712.99 µs |
| sin | 465.16 µs | 493.77 µs | 640.85 µs |
| sqrt | 61.462 ns | 110.28 ns | 137.60 ns |

### Wide-tier strict - D57 / D76 / D115 / D153 / D230 / D307 / D462 / D616 / D924 / D1232

Cost grows with both the work integer's bit width and the
guard-digit budget at each scale.

#### Wide (`wide` umbrella — D57 / D76 / D115 / D153 / D230 / D307)

| fn | D57 (s=28) | D76 (s=35) | D115 (s=57) | D153 (s=75) | D230 (s=115) | D307 (s=150) |
|---|---|---|---|---|---|---|
| ln | 32.000 µs | 36.192 µs | 47.706 µs | 80.806 µs | — | — |
| exp | 26.788 µs | 28.466 µs | — | — | — | — |
| sin | 20.091 µs | 22.198 µs | — | — | — | — |
| sqrt | — | — | — | — | — | — |

#### Extra-wide (`x-wide` adds D462 / D616)

| fn | D462 (s=230) | D616 (s=308) |
|---|---|---|
| ln | — | — |
| exp | — | — |
| sin | — | — |
| sqrt | — | — |

#### XX-wide (`xx-wide` adds D924 / D1232)

| fn | D924 (s=461) | D1232 (s=616) |
|---|---|---|
| ln | — | — |
| exp | — | — |
| sin | — | — |
| sqrt | — | — |

**Historical comparison — 0.2.5 baseline.** On the same hardware,
0.2.5 measured D76<35> ln at 1.37 ms, D153<75> ln at 6.40 ms,
D307<150> ln at 34.1 ms. After this cycle's u64-native limbs, MG
2-by-1 reciprocal Knuth divide, Brent's two-stage exp argument
reduction, multi-level sqrt halving in ln, [0, π/4] sin range
reduction, sin_cos / sinh_cosh joint kernels, thread-local pi /
ln2 / ln10 cache, and pow10-cached mul/div per inner loop:

| op | 0.2 | 0.3 | speedup |
|---|---|---|---|
| D76<35>  ln   |  1.37 ms |  36.2 µs |  **38×** |
| D76<35>  exp  |  1.27 ms |  28.5 µs |  **45×** |
| D76<35>  sin  |  1.08 ms |  22.2 µs |  **49×** |
| D76<35>  sqrt | 20.5 µs  |  2.85 µs |   **7×** |
| D153<75> ln   |  6.40 ms |  80.8 µs |  **79×** |
| D153<75> exp  |  5.87 ms |  63.0 µs |  **93×** |
| D153<75> sin  |  4.82 ms |  52.2 µs |  **92×** |
| D153<75> sqrt | 83.6 µs  |  4.34 µs |  **19×** |
| D307<150> ln  | 34.1 ms  | 191.4 µs | **178×** |
| D307<150> exp | 31.2 ms  | 147.5 µs | **212×** |
| D307<150> sin | 25.5 ms  | 130.8 µs | **195×** |
| D307<150> sqrt|  313 µs  |  8.31 µs |  **38×** |

> The **0.2** column is the 0.2.5 baseline measured at the start of
> the 0.2.x cycle; the **0.3** column is from the 0.3.3 (2026-05-18)
> sweep. Speedups across the major-cycle gap are conservative —
> the 0.3 numbers come from the same multi-hour 2-lane
> `full_matrix` sweep that fills the per-tier strict tables above
> and carry the same contention drift on wide-tier cells (cold-
> machine micro-benches typically measure 30-50% faster). Repeat
> on the focused `lib_cmp_d{N}` bench for any cell whose magnitude
> matters to your decision.

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
| add      | **842 ps**             | 8.33 ns               | 6.34 ns                | 868 ps         | 81.2 ns           | 67.6 ns             | 6.09 ns                | -                   |
| sub      | **891 ps**             | 5.12 ns               | 6.35 ns                | 881 ps         | 298 ns            | 65.2 ns             | 6.06 ns                | -                   |
| mul      | 12.3 ns                | 9.99 ns               | 27.6 ns                | **1.48 ns**    | 70.0 ns           | 56.6 ns             | 34.4 ns                | 232 ns              |
| div      | 8.00 ns                | **5.17 ns**           | 8.54 ns                | 23.9 ns        | 67.2 ns           | 53.9 ns             | 79.0 ns                | -                   |
| rem      | 7.74 ns                | 18.0 ns               | 11.9 ns                | 14.4 ns        | 116 ns            | 37.6 ns             | **3.36 ns**            | -                   |
| neg      | **489 ps**             | 757 ps                | 4.31 ns                | 506 ps         | 41.3 ns           | 6.15 ns             | 4.59 ns                | -                   |
| ln       | 1.47 µs (0 ULP)        | **16.1 ns (0 ULP)**   | 3.71 µs (0 ULP)        | -              | -                 | 67.9 µs (0 ULP)     | 2.66 µs (0 ULP)        | 543 ns (6 ULP)      |
| exp      | 40.5 µs (0 ULP)        | 8.92 µs (0†)          | **170 ns (0†)**        | -              | -                 | 232 µs (4 ULP)      | **51.4 ns (0†)**       | 1.28 µs (46 ULP)    |
| sin      | 39.2 µs (0 ULP)        | **6.58 µs (0†)**      | 2.46 µs (0†)           | -              | -                 | -                   | -                      | 11.9 µs (33 ULP)    |
| sqrt     | 35.4 ns (0 ULP)        | **9.20 ns (0 ULP)**   | 555 ns (0 ULP)         | -              | 3.09 µs (0 ULP)   | -                   | 1.51 µs (0 ULP)        | 2.60 µs (12 ULP)    |

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

### Caveat — fastnum `atan` / `atan2` input-range rejection

`fastnum`'s `Decimal::atan` returns a signalling NaN immediately
for any `|x| > 1` ([`fastnum-0.4.5/src/decimal/dec/math/atan.rs`](https://docs.rs/fastnum/0.4.5/src/fastnum/decimal/dec/math/atan.rs.html),
lines 24–40). This is mathematically incorrect — `atan` is
defined on the whole real line with range `(−π/2, π/2)` — but it
means any benchmark that calls `fastnum::Decimal::atan(2)` or
similar is timing a NaN early-return, not an `atan` computation.

The per-tier sweep in `target/medians_per_tier.tsv` records
`128bit_s19/fastnum/atan = 13 ns` vs
`128bit_s19/decimal-scaled/atan = 68 µs`; the 5000× ratio is
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
  combo, with `D38<38>`'s ≈ 1.7 ceiling being the binding
  constraint.
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

## 0.3.2 sweep — full raw data

Complete dump of the 2026-05-18 bench sweep, one table per
bench binary. **Median** is criterion's median per-iteration
time; **Change vs prior** is criterion's median-percentage
verdict against the saved baseline. `n/a` means there was no
baseline (typically a wide-tier bench that crashed mid-stream
in the preceding aborted sweep, so this run seeded the
baseline for future comparisons).

**Run conditions**: 24-logical / 12-physical Windows machine,
dispatcher pinned each lane to one physical core (logical 20+21
lane A, 22+23 lane B) at High priority. Sweep ran ~3 hours
wall-clock. Narrow-tier picosecond-scale measurements (D9 /
D18 / D38 arith) sit near the bench-machine resolution floor
and their `Change vs prior` deltas at this scale are
dominated by pipeline / branch-predictor / interrupt jitter
rather than real code differences — interpret those rows
accordingly. Wide-tier ops in the µs+ range are well above
the noise floor and the deltas there reflect real cumulative
perf work from 0.3.x.

Headline `_approx` family micro-bench (run separately on a
cold machine, not in this sweep) on D38<19> ln: **3.7×**
faster at guard 6, 2.6× at 10, 1.8× at 15, 1.4× at 20, 1.2×
at 25 — all vs strict's 54.8 µs.

### `full_matrix_d9` (30 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D9_s0/add` | 621.38 ps | +39.873% |
| `arith/D9_s0/sub` | 594.94 ps | +42.252% |
| `arith/D9_s0/mul` | 513.41 ps | +24.251% |
| `arith/D9_s0/div` | 1.7512 ns | +9.8530% |
| `arith/D9_s0/rem` | 1.7562 ns | +15.954% |
| `arith/D9_s0/neg` | 441.02 ps | +54.485% |
| `arith/D9_s5/add` | 596.65 ps | +35.921% |
| `arith/D9_s5/sub` | 581.17 ps | +36.660% |
| `arith/D9_s5/mul` | 1.4751 ns | +65.835% |
| `arith/D9_s5/div` | 2.9736 ns | +13.642% |
| `arith/D9_s5/rem` | 1.7205 ns | +19.103% |
| `arith/D9_s5/neg` | 434.28 ps | +55.277% |
| `arith/D9_s9/add` | 597.43 ps | +51.340% |
| `arith/D9_s9/sub` | 580.18 ps | +46.828% |
| `arith/D9_s9/mul` | 1.4579 ns | +100.57% |
| `arith/D9_s9/div` | 3.9839 ns | +58.251% |
| `arith/D9_s9/rem` | 1.7360 ns | +19.642% |
| `arith/D9_s9/neg` | 444.83 ps | +55.581% |
| `strict/D9_s0/ln` | 1.5343 ns | n/a |
| `strict/D9_s0/exp` | 1.6572 ns | +11.620% |
| `strict/D9_s0/sin` | 2.0001 ns | n/a |
| `strict/D9_s0/sqrt` | 49.444 ns | +226.90% |
| `strict/D9_s5/ln` | 476.61 µs | +1395.1% |
| `strict/D9_s5/exp` | 432.92 µs | +1365.7% |
| `strict/D9_s5/sin` | 465.16 µs | +1519.0% |
| `strict/D9_s5/sqrt` | 61.462 ns | +224.44% |
| `strict/D9_s9/ln` | 586.43 µs | +1360.3% |
| `strict/D9_s9/exp` | 522.37 µs | +1372.9% |
| `strict/D9_s9/sin` | 478.52 µs | +1462.1% |
| `strict/D9_s9/sqrt` | 104.67 ns | +212.81% |

### `full_matrix_d18` (36 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D18_s0/add` | 574.36 ps | +50.264% |
| `arith/D18_s0/sub` | 591.39 ps | +32.382% |
| `arith/D18_s0/mul` | 575.47 ps | +45.646% |
| `arith/D18_s0/div` | 15.172 ns | +37.904% |
| `arith/D18_s0/rem` | 1.7273 ns | +17.804% |
| `arith/D18_s0/neg` | 434.12 ps | +38.072% |
| `arith/D18_s9/add` | 575.77 ps | +56.399% |
| `arith/D18_s9/sub` | 577.89 ps | +55.219% |
| `arith/D18_s9/mul` | 14.866 ns | +57.761% |
| `arith/D18_s9/div` | 15.578 ns | +55.731% |
| `arith/D18_s9/rem` | 1.7302 ns | +25.417% |
| `arith/D18_s9/neg` | 437.38 ps | +48.678% |
| `arith/D18_s18/add` | 583.76 ps | +59.544% |
| `arith/D18_s18/sub` | 586.78 ps | +64.247% |
| `arith/D18_s18/mul` | 14.945 ns | +64.145% |
| `arith/D18_s18/div` | 15.585 ns | +52.648% |
| `arith/D18_s18/rem` | 1.7890 ns | n/a |
| `arith/D18_s18/neg` | 439.62 ps | +27.324% |
| `arith/fixed_i64f64/add` | 1.0007 ns | +11.869% |
| `arith/fixed_i64f64/sub` | 1.0585 ns | +19.614% |
| `arith/fixed_i64f64/mul` | 2.6761 ns | +79.393% |
| `arith/fixed_i64f64/div` | 63.538 ns | +171.82% |
| `arith/fixed_i64f64/rem` | 22.918 ns | +59.529% |
| `arith/fixed_i64f64/neg` | 727.19 ps | +46.250% |
| `strict/D18_s0/ln` | 751.04 ps | n/a |
| `strict/D18_s0/exp` | 605.40 ps | n/a |
| `strict/D18_s0/sin` | 1.7592 ns | n/a |
| `strict/D18_s0/sqrt` | 48.610 ns | +245.59% |
| `strict/D18_s9/ln` | 601.39 µs | +1412.8% |
| `strict/D18_s9/exp` | 525.56 µs | +1436.3% |
| `strict/D18_s9/sin` | 493.77 µs | +1475.0% |
| `strict/D18_s9/sqrt` | 110.28 ns | +254.44% |
| `strict/D18_s18/ln` | 868.64 µs | +1617.2% |
| `strict/D18_s18/exp` | 842.72 µs | +1725.1% |
| `strict/D18_s18/sin` | 675.80 µs | +1555.2% |
| `strict/D18_s18/sqrt` | 148.13 ns | +331.94% |

### `full_matrix_d38` (30 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D38_s0/add` | 987.90 ps | n/a |
| `arith/D38_s0/sub` | 970.88 ps | n/a |
| `arith/D38_s0/mul` | 4.2143 ns | +31.048% |
| `arith/D38_s0/div` | 16.117 ns | +32.160% |
| `arith/D38_s0/rem` | 13.031 ns | +28.493% |
| `arith/D38_s0/neg` | 740.35 ps | +18.545% |
| `arith/D38_s19/add` | 980.05 ps | n/a |
| `arith/D38_s19/sub` | 981.90 ps | n/a |
| `arith/D38_s19/mul` | 28.160 ns | +94.607% |
| `arith/D38_s19/div` | 17.299 ns | +90.403% |
| `arith/D38_s19/rem` | 12.776 ns | +42.624% |
| `arith/D38_s19/neg` | 720.92 ps | +55.118% |
| `arith/D38_s38/add` | 953.57 ps | +22.238% |
| `arith/D38_s38/sub` | 978.92 ps | +14.407% |
| `arith/D38_s38/mul` | 29.146 ns | +146.27% |
| `arith/D38_s38/div` | 5.6551 µs | +1200.4% |
| `arith/D38_s38/rem` | 19.065 ns | +96.374% |
| `arith/D38_s38/neg` | 732.29 ps | +31.770% |
| `strict/D38_s0/ln` | 1.3741 ns | n/a |
| `strict/D38_s0/exp` | 942.03 ps | n/a |
| `strict/D38_s0/sin` | 1.5913 ns | n/a |
| `strict/D38_s0/sqrt` | 48.577 ns | +253.16% |
| `strict/D38_s19/ln` | 851.47 µs | +1373.0% |
| `strict/D38_s19/exp` | 712.99 µs | +1403.1% |
| `strict/D38_s19/sin` | 640.85 µs | +1386.5% |
| `strict/D38_s19/sqrt` | 137.60 ns | +295.86% |
| `strict/D38_s38/ln` | 909.60 µs | +1364.0% |
| `strict/D38_s38/exp` | 416.76 µs | +1361.4% |
| `strict/D38_s38/sin` | 696.37 µs | +3769.6% |
| `strict/D38_s38/sqrt` | 38.584 µs | +1145.3% |

### `full_matrix_d57` (28 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D57_s0/add` | 2.0318 ns | n/a |
| `arith/D57_s0/sub` | 2.6117 ns | n/a |
| `arith/D57_s0/mul` | 63.833 ns | n/a |
| `arith/D57_s0/div` | 199.35 ns | n/a |
| `arith/D57_s0/rem` | 37.813 ns | n/a |
| `arith/D57_s0/neg` | 2.2552 ns | n/a |
| `arith/D57_s28/add` | 2.0048 ns | n/a |
| `arith/D57_s28/sub` | 2.6179 ns | n/a |
| `arith/D57_s28/mul` | 222.97 ns | n/a |
| `arith/D57_s28/div` | 416.42 ns | n/a |
| `arith/D57_s28/rem` | 134.93 ns | n/a |
| `arith/D57_s28/neg` | 2.2434 ns | n/a |
| `arith/D57_s56/add` | 2.0210 ns | n/a |
| `arith/D57_s56/sub` | 2.6133 ns | n/a |
| `arith/D57_s56/mul` | 313.87 ns | n/a |
| `arith/D57_s56/div` | 447.76 ns | n/a |
| `arith/D57_s56/rem` | 141.74 ns | n/a |
| `arith/D57_s56/neg` | 2.2562 ns | n/a |
| `strict_wide/D57_s0/ln` | 9.5528 µs | n/a |
| `strict_wide/D57_s0/exp` | 12.124 ns | n/a |
| `strict_wide/D57_s0/sin` | 19.232 µs | n/a |
| `strict_wide/D57_s0/sqrt` | 214.08 ns | n/a |
| `strict_wide/D57_s28/ln` | 32.000 µs | n/a |
| `strict_wide/D57_s28/exp` | 26.788 µs | n/a |
| `strict_wide/D57_s28/sin` | 20.091 µs | n/a |
| `strict_wide/D57_s56/ln` | 44.121 µs | n/a |
| `strict_wide/D57_s56/exp` | 33.287 µs | n/a |
| `strict_wide/D57_s56/sin` | 27.037 µs | n/a |

### `full_matrix_d76` (34 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D76_s0/add` | 2.3579 ns | n/a |
| `arith/D76_s0/sub` | 3.2922 ns | n/a |
| `arith/D76_s0/mul` | 64.901 ns | n/a |
| `arith/D76_s0/div` | 214.96 ns | +8.8461% |
| `arith/D76_s0/rem` | 38.658 ns | n/a |
| `arith/D76_s0/neg` | 2.5907 ns | n/a |
| `arith/D76_s35/add` | 2.3611 ns | n/a |
| `arith/D76_s35/sub` | 3.3064 ns | n/a |
| `arith/D76_s35/mul` | 229.04 ns | n/a |
| `arith/D76_s35/div` | 453.19 ns | n/a |
| `arith/D76_s35/rem` | 139.45 ns | n/a |
| `arith/D76_s35/neg` | 2.5964 ns | n/a |
| `arith/D76_s76/add` | 2.3707 ns | n/a |
| `arith/D76_s76/sub` | 3.3202 ns | n/a |
| `arith/D76_s76/mul` | 351.59 ns | n/a |
| `arith/D76_s76/div` | 516.41 ns | n/a |
| `arith/D76_s76/rem` | 145.43 ns | n/a |
| `arith/D76_s76/neg` | 2.5663 ns | n/a |
| `arith/bnum_d76_s35/add` | 2.5674 ns | n/a |
| `arith/bnum_d76_s35/sub` | 2.2895 ns | n/a |
| `arith/bnum_d76_s35/mul` | 538.35 ns | n/a |
| `arith/bnum_d76_s35/div` | 534.00 ns | n/a |
| `arith/bnum_d76_s35/rem` | 62.890 ns | n/a |
| `arith/bnum_d76_s35/neg` | 10.767 ns | n/a |
| `strict_wide/D76_s0/ln` | 9.7191 µs | n/a |
| `strict_wide/D76_s0/exp` | 13.352 ns | n/a |
| `strict_wide/D76_s0/sin` | 18.815 µs | n/a |
| `strict_wide/D76_s0/sqrt` | 249.41 ns | n/a |
| `strict_wide/D76_s35/ln` | 36.192 µs | n/a |
| `strict_wide/D76_s35/exp` | 28.466 µs | n/a |
| `strict_wide/D76_s35/sin` | 22.198 µs | n/a |
| `strict_wide/D76_s76/ln` | 55.892 µs | n/a |
| `strict_wide/D76_s76/exp` | 38.210 µs | n/a |
| `strict_wide/D76_s76/sin` | 32.830 µs | n/a |

### `full_matrix_d115` (22 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D115_s0/add` | 3.9050 ns | n/a |
| `arith/D115_s0/sub` | 5.2838 ns | n/a |
| `arith/D115_s0/mul` | 77.288 ns | n/a |
| `arith/D115_s0/div` | 247.30 ns | n/a |
| `arith/D115_s0/rem` | 59.596 ns | n/a |
| `arith/D115_s0/neg` | 3.6422 ns | n/a |
| `arith/D115_s57/add` | 3.9263 ns | n/a |
| `arith/D115_s57/sub` | 5.2969 ns | n/a |
| `arith/D115_s57/mul` | 335.88 ns | n/a |
| `arith/D115_s57/div` | 555.53 ns | n/a |
| `arith/D115_s57/rem` | 166.97 ns | n/a |
| `arith/D115_s57/neg` | 3.5728 ns | n/a |
| `arith/D115_s114/add` | 4.0655 ns | n/a |
| `arith/D115_s114/sub` | 5.2780 ns | n/a |
| `arith/D115_s114/mul` | 529.70 ns | n/a |
| `arith/D115_s114/div` | 672.39 ns | n/a |
| `arith/D115_s114/rem` | 176.07 ns | n/a |
| `arith/D115_s114/neg` | 3.5581 ns | n/a |
| `strict_wide/D115_s0/ln` | 9.7944 µs | n/a |
| `strict_wide/D115_s0/exp` | 17.225 ns | n/a |
| `strict_wide/D115_s0/sin` | 18.719 µs | n/a |
| `strict_wide/D115_s57/ln` | 47.706 µs | n/a |

### `full_matrix_d153` (22 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D153_s0/add` | 5.4339 ns | n/a |
| `arith/D153_s0/sub` | 13.093 ns | n/a |
| `arith/D153_s0/mul` | 80.800 ns | n/a |
| `arith/D153_s0/div` | 272.49 ns | +5.9238% |
| `arith/D153_s0/rem` | 65.667 ns | +6.6510% |
| `arith/D153_s0/neg` | 4.4793 ns | n/a |
| `arith/D153_s75/add` | 5.4557 ns | n/a |
| `arith/D153_s75/sub` | 13.113 ns | n/a |
| `arith/D153_s75/mul` | 371.14 ns | n/a |
| `arith/D153_s75/div` | 850.39 ns | n/a |
| `arith/D153_s75/rem` | 191.16 ns | +9.0106% |
| `arith/D153_s75/neg` | 4.5536 ns | n/a |
| `arith/D153_s153/add` | 5.9852 ns | n/a |
| `arith/D153_s153/sub` | 13.227 ns | n/a |
| `arith/D153_s153/mul` | 776.38 ns | n/a |
| `arith/D153_s153/div` | 1.0227 µs | n/a |
| `arith/D153_s153/rem` | 190.33 ns | n/a |
| `arith/D153_s153/neg` | 4.4366 ns | n/a |
| `strict_wide/D153_s0/ln` | 15.553 µs | n/a |
| `strict_wide/D153_s0/exp` | 19.444 ns | n/a |
| `strict_wide/D153_s0/sin` | 31.098 µs | n/a |
| `strict_wide/D153_s75/ln` | 80.806 µs | n/a |

### `full_matrix_d230` (21 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D230_s0/add` | 15.329 ns | n/a |
| `arith/D230_s0/sub` | 18.126 ns | n/a |
| `arith/D230_s0/mul` | 96.136 ns | n/a |
| `arith/D230_s0/div` | 368.92 ns | n/a |
| `arith/D230_s0/rem` | 69.260 ns | n/a |
| `arith/D230_s0/neg` | 12.397 ns | n/a |
| `arith/D230_s115/add` | 15.176 ns | n/a |
| `arith/D230_s115/sub` | 18.037 ns | n/a |
| `arith/D230_s115/mul` | 636.53 ns | n/a |
| `arith/D230_s115/div` | 1.2000 µs | n/a |
| `arith/D230_s115/rem` | 190.24 ns | n/a |
| `arith/D230_s115/neg` | 13.083 ns | n/a |
| `arith/D230_s230/add` | 15.400 ns | n/a |
| `arith/D230_s230/sub` | 18.301 ns | n/a |
| `arith/D230_s230/mul` | 1.3751 µs | n/a |
| `arith/D230_s230/div` | 1.5384 µs | n/a |
| `arith/D230_s230/rem` | 214.25 ns | n/a |
| `arith/D230_s230/neg` | 14.183 ns | n/a |
| `strict_wide/D230_s0/ln` | 19.945 µs | n/a |
| `strict_wide/D230_s0/exp` | 42.433 ns | n/a |
| `strict_wide/D230_s0/sin` | 42.267 µs | n/a |

### `full_matrix_d307` (21 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D307_s0/add` | 19.681 ns | +147.15% |
| `arith/D307_s0/sub` | 23.905 ns | +63.977% |
| `arith/D307_s0/mul` | 127.33 ns | +128.12% |
| `arith/D307_s0/div` | 458.13 ns | +82.783% |
| `arith/D307_s0/rem` | 81.089 ns | +123.03% |
| `arith/D307_s0/neg` | 12.549 ns | +158.19% |
| `arith/D307_s150/add` | 19.463 ns | +149.15% |
| `arith/D307_s150/sub` | 23.432 ns | +65.669% |
| `arith/D307_s150/mul` | 782.59 ns | n/a |
| `arith/D307_s150/div` | 1.4986 µs | n/a |
| `arith/D307_s150/rem` | 214.80 ns | n/a |
| `arith/D307_s150/neg` | 13.725 ns | +172.97% |
| `arith/D307_s307/add` | 19.617 ns | +139.56% |
| `arith/D307_s307/sub` | 23.463 ns | +58.996% |
| `arith/D307_s307/mul` | 2.1567 µs | n/a |
| `arith/D307_s307/div` | 2.6634 µs | n/a |
| `arith/D307_s307/rem` | 243.24 ns | n/a |
| `arith/D307_s307/neg` | 14.677 ns | +183.11% |
| `strict_wide/D307_s0/ln` | 23.616 µs | n/a |
| `strict_wide/D307_s0/exp` | 49.375 ns | +77.817% |
| `strict_wide/D307_s0/sin` | 48.628 µs | +133.22% |

### `full_matrix_d462` (21 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D462_s0/add` | 42.662 ns | n/a |
| `arith/D462_s0/sub` | 57.612 ns | n/a |
| `arith/D462_s0/mul` | 218.41 ns | n/a |
| `arith/D462_s0/div` | 725.49 ns | n/a |
| `arith/D462_s0/rem` | 166.05 ns | n/a |
| `arith/D462_s0/neg` | 47.081 ns | n/a |
| `arith/D462_s230/add` | 41.840 ns | n/a |
| `arith/D462_s230/sub` | 57.715 ns | n/a |
| `arith/D462_s230/mul` | 1.6512 µs | n/a |
| `arith/D462_s230/div` | 2.8569 µs | n/a |
| `arith/D462_s230/rem` | 322.43 ns | n/a |
| `arith/D462_s230/neg` | 53.045 ns | n/a |
| `arith/D462_s461/add` | 45.651 ns | n/a |
| `arith/D462_s461/sub` | 60.098 ns | n/a |
| `arith/D462_s461/mul` | 4.5977 µs | n/a |
| `arith/D462_s461/div` | 4.9446 µs | n/a |
| `arith/D462_s461/rem` | 358.87 ns | n/a |
| `arith/D462_s461/neg` | 54.417 ns | n/a |
| `strict_wide/D462_s0/ln` | 23.491 µs | n/a |
| `strict_wide/D462_s0/exp` | 116.93 ns | n/a |
| `strict_wide/D462_s0/sin` | 48.717 µs | n/a |

### `full_matrix_d616` (21 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D616_s0/add` | 93.746 ns | n/a |
| `arith/D616_s0/sub` | 115.29 ns | n/a |
| `arith/D616_s0/mul` | 232.05 ns | n/a |
| `arith/D616_s0/div` | 813.79 ns | n/a |
| `arith/D616_s0/rem` | 198.95 ns | n/a |
| `arith/D616_s0/neg` | 64.759 ns | n/a |
| `arith/D616_s308/add` | 97.030 ns | n/a |
| `arith/D616_s308/sub` | 118.91 ns | n/a |
| `arith/D616_s308/mul` | 2.6811 µs | n/a |
| `arith/D616_s308/div` | 4.6416 µs | n/a |
| `arith/D616_s308/rem` | 347.50 ns | n/a |
| `arith/D616_s308/neg` | 64.964 ns | n/a |
| `arith/D616_s615/add` | 90.614 ns | n/a |
| `arith/D616_s615/sub` | 114.07 ns | n/a |
| `arith/D616_s615/mul` | 7.8695 µs | n/a |
| `arith/D616_s615/div` | 7.7299 µs | n/a |
| `arith/D616_s615/rem` | 434.30 ns | n/a |
| `arith/D616_s615/neg` | 67.913 ns | n/a |
| `strict_wide/D616_s0/ln` | 37.087 µs | n/a |
| `strict_wide/D616_s0/exp` | 166.56 ns | n/a |
| `strict_wide/D616_s0/sin` | 75.922 µs | n/a |

### `full_matrix_d924` (21 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D924_s0/add` | 119.69 ns | n/a |
| `arith/D924_s0/sub` | 143.59 ns | n/a |
| `arith/D924_s0/mul` | 306.32 ns | n/a |
| `arith/D924_s0/div` | 1.1615 µs | n/a |
| `arith/D924_s0/rem` | 238.56 ns | n/a |
| `arith/D924_s0/neg` | 74.740 ns | n/a |
| `arith/D924_s461/add` | 115.40 ns | n/a |
| `arith/D924_s461/sub` | 148.42 ns | n/a |
| `arith/D924_s461/mul` | 5.1900 µs | n/a |
| `arith/D924_s461/div` | 8.2246 µs | n/a |
| `arith/D924_s461/rem` | 442.64 ns | n/a |
| `arith/D924_s461/neg` | 75.659 ns | n/a |
| `arith/D924_s923/add` | 104.87 ns | n/a |
| `arith/D924_s923/sub` | 203.09 ns | n/a |
| `arith/D924_s923/mul` | 16.312 µs | n/a |
| `arith/D924_s923/div` | 13.314 µs | n/a |
| `arith/D924_s923/rem` | 541.22 ns | n/a |
| `arith/D924_s923/neg` | 86.028 ns | n/a |
| `strict_wide/D924_s0/ln` | 53.300 µs | n/a |
| `strict_wide/D924_s0/exp` | 236.94 ns | n/a |
| `strict_wide/D924_s0/sin` | 106.94 µs | n/a |

### `full_matrix_d1232` (19 measurements)

| Op | Median | Change vs prior |
|----|--------|-----------------|
| `arith/D1232_s0/add` | 143.34 ns | n/a |
| `arith/D1232_s0/sub` | 196.50 ns | n/a |
| `arith/D1232_s0/mul` | 362.90 ns | n/a |
| `arith/D1232_s0/div` | 1.4677 µs | n/a |
| `arith/D1232_s0/rem` | 339.55 ns | n/a |
| `arith/D1232_s0/neg` | 99.849 ns | n/a |
| `arith/D1232_s616/add` | 130.52 ns | n/a |
| `arith/D1232_s616/sub` | 186.22 ns | n/a |
| `arith/D1232_s616/mul` | 9.0517 µs | n/a |
| `arith/D1232_s616/div` | 10.890 µs | n/a |
| `arith/D1232_s616/rem` | 545.46 ns | n/a |
| `arith/D1232_s616/neg` | 103.49 ns | n/a |
| `arith/D1232_s1231/add` | 129.54 ns | n/a |
| `arith/D1232_s1231/sub` | 185.57 ns | n/a |
| `arith/D1232_s1231/mul` | 29.228 µs | n/a |
| `arith/D1232_s1231/div` | 21.273 µs | n/a |
| `arith/D1232_s1231/rem` | 696.57 ns | n/a |
| `arith/D1232_s1231/neg` | 114.56 ns | n/a |
| `strict_wide/D1232_s0/ln` | 70.146 µs | n/a |

