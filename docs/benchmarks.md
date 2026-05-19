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

> **Bench machine.** All numbers in this doc are from the v0.4.0
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

> **v0.4.0 sweep refresh.** Tables below come from the v0.4.0
> full_matrix sweep on GitHub-hosted `ubuntu-latest` standard
> runners (GHA run [`26069911678`](https://github.com/mootable/decimal-scaled/actions/runs/26069911678)).
> Narrow-tier ps-scale cells (D9 / D18 / D38 add / sub / neg)
> sit near the runner's resolution floor and carry ±20 %
> pipeline / steal-budget jitter; wide-tier ns+ values are
> reliable. See the [run-conditions preamble](#040-sweep--full-raw-data)
> at the bottom of this page for the full caveat.

### D9 - 32 bits

| op | s = 0 | s = 5 | s = 9 |
|---|---|---|---|
| add | 934.42 ps | 933.09 ps | 933.46 ps |
| sub | 933.84 ps | 933.37 ps | 933.80 ps |
| mul | 933.75 ps | 1.5556 ns | 1.5563 ns |
| div | 1.8658 ns | 2.1845 ns | 2.1782 ns |
| rem | 1.8655 ns | 1.8662 ns | 1.8657 ns |
| neg | 622.20 ps | 622.49 ps | 621.99 ps |

### D18 - 64 bits

| op | s = 0 | s = 9 | s = 18 |
|---|---|---|---|
| add | 935.49 ps | 933.97 ps | 936.63 ps |
| sub | 934.42 ps | 934.31 ps | 937.32 ps |
| mul | 936.63 ps | 5.6151 ns | 6.0023 ns |
| div | 5.9201 ns | 6.2239 ns | 6.2545 ns |
| rem | 1.8669 ns | 1.8667 ns | 2.1813 ns |
| neg | 622.51 ps | 622.35 ps | 624.40 ps |

### D38 - 128 bits

| op | s = 0 | s = 19 | s = 38 |
|---|---|---|---|
| add | 1.1859 ns | 1.1841 ns | 1.1842 ns |
| sub | 1.1973 ns | 1.1827 ns | 1.1808 ns |
| mul | 3.4690 ns | 20.922 ns | 21.163 ns |
| div | 6.7787 ns | 9.2041 ns | 654.60 ns |
| rem | 5.8664 ns | 5.8757 ns | 9.9953 ns |
| neg | 867.78 ps | 867.88 ps | 867.73 ps |

### D57 - 192 bits

| op | s = 0 | s = 28 | s = 56 |
|---|---|---|---|
| add | 2.6813 ns | 2.6801 ns | 2.6802 ns |
| sub | 2.8485 ns | 2.8475 ns | 2.8478 ns |
| mul | 80.446 ns | 290.95 ns | 318.52 ns |
| div | 190.20 ns | 449.22 ns | 495.24 ns |
| rem | 24.021 ns | 168.10 ns | 174.59 ns |
| neg | 2.1822 ns | 2.1812 ns | 2.1817 ns |

### D76 - 256 bits

| op | s = 0 | s = 35 | s = 76 |
|---|---|---|---|
| add | 3.7185 ns | 3.7207 ns | 3.7177 ns |
| sub | 3.7274 ns | 3.7283 ns | 3.7286 ns |
| mul | 79.068 ns | 288.20 ns | 352.83 ns |
| div | 176.64 ns | 464.47 ns | 564.51 ns |
| rem | 25.622 ns | 170.89 ns | 179.56 ns |
| neg | 2.5578 ns | 2.5567 ns | 2.5607 ns |

### D115 - 384 bits

| op | s = 0 | s = 57 | s = 114 |
|---|---|---|---|
| add | 4.8551 ns | 4.8561 ns | 4.8515 ns |
| sub | 5.6624 ns | 5.6541 ns | 5.6554 ns |
| mul | 55.254 ns | 229.39 ns | 324.85 ns |
| div | 162.08 ns | 420.85 ns | 483.67 ns |
| rem | 27.116 ns | 104.97 ns | 114.23 ns |
| neg | 3.1529 ns | 3.1497 ns | 3.1523 ns |

### D153 - 512 bits

| op | s = 0 | s = 75 | s = 153 |
|---|---|---|---|
| add | 7.3587 ns | 7.3488 ns | 7.3480 ns |
| sub | 13.865 ns | 13.864 ns | 13.864 ns |
| mul | 98.866 ns | 382.80 ns | 642.64 ns |
| div | 292.26 ns | 871.16 ns | 1.0327 µs |
| rem | 39.690 ns | 190.44 ns | 204.99 ns |
| neg | 5.0735 ns | 5.0724 ns | 5.0731 ns |

### D230 - 768 bits

| op | s = 0 | s = 115 | s = 230 |
|---|---|---|---|
| add | 16.739 ns | 16.736 ns | 16.744 ns |
| sub | 19.019 ns | 19.016 ns | 19.014 ns |
| mul | 65.709 ns | 431.16 ns | 944.22 ns |
| div | 267.70 ns | 870.79 ns | 1.1877 µs |
| rem | 52.693 ns | 138.62 ns | 153.42 ns |
| neg | 10.280 ns | 10.362 ns | 10.651 ns |

### D307 - 1024 bits

| op | s = 0 | s = 150 | s = 307 |
|---|---|---|---|
| add | 23.037 ns | 22.928 ns | 22.918 ns |
| sub | 26.622 ns | 26.608 ns | 26.621 ns |
| mul | 92.466 ns | 568.68 ns | 1.5216 µs |
| div | 375.04 ns | 1.3321 µs | 2.4394 µs |
| rem | 66.987 ns | 229.35 ns | 257.63 ns |
| neg | 13.953 ns | 14.377 ns | 14.772 ns |

### D462 - 1536 bits

| op | s = 0 | s = 230 | s = 461 |
|---|---|---|---|
| add | 26.347 ns | 26.372 ns | 26.389 ns |
| sub | 44.482 ns | 44.476 ns | 44.503 ns |
| mul | 148.97 ns | 1.2045 µs | 3.1619 µs |
| div | 529.19 ns | 2.2243 µs | 4.0998 µs |
| rem | 99.969 ns | 281.44 ns | 320.65 ns |
| neg | 35.756 ns | 36.281 ns | 38.117 ns |

### D616 - 2048 bits

| op | s = 0 | s = 308 | s = 615 |
|---|---|---|---|
| add | 44.720 ns | 44.609 ns | 44.735 ns |
| sub | 66.411 ns | 66.405 ns | 66.390 ns |
| mul | 117.66 ns | 1.7899 µs | 5.2515 µs |
| div | 465.22 ns | 2.5389 µs | 4.8258 µs |
| rem | 101.62 ns | 185.84 ns | 280.11 ns |
| neg | 38.610 ns | 39.818 ns | 41.372 ns |

### D924 - 3072 bits

| op | s = 0 | s = 461 | s = 923 |
|---|---|---|---|
| add | 94.060 ns | 94.035 ns | 94.078 ns |
| sub | 120.69 ns | 120.64 ns | 120.60 ns |
| mul | 205.57 ns | 3.6839 µs | 11.280 µs |
| div | 895.13 ns | 6.1591 µs | 11.506 µs |
| rem | 175.60 ns | 392.37 ns | 479.32 ns |
| neg | 65.160 ns | 67.274 ns | 70.182 ns |

### D1232 - 4096 bits

| op | s = 0 | s = 616 | s = 1231 |
|---|---|---|---|
| add | 120.02 ns | 119.90 ns | 119.95 ns |
| sub | 188.44 ns | 188.67 ns | 188.50 ns |
| mul | 259.39 ns | 6.2126 µs | 19.642 µs |
| div | 1.0629 µs | 8.9470 µs | 18.140 µs |
| rem | 217.59 ns | 475.36 ns | 593.95 ns |
| neg | 92.268 ns | 93.567 ns | 96.780 ns |

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

> **v0.4.0 sweep — narrow-tier caveat.** The narrow-tier
> (D9 / D18 / D38) values below were recorded on shared GHA
> standard runners over a multi-hour sweep window. Cold-machine
> micro-benches of the same code typically measure several ×
> faster; treat the absolute µs values here as an upper-bound
> ceiling, not a steady-state cost. Wide-tier numbers carry the
> same caveat at a smaller multiple (1.5–2×).

### D9 / D18 / D38 strict

| fn | D9 (s=5) | D18 (s=9) | D38 (s=19) |
|---|---|---|---|
| ln | 86.619 µs | 104.72 µs | 19.605 µs |
| exp | 80.799 µs | 95.852 µs | 16.333 µs |
| sin | 74.438 µs | 84.749 µs | 11.678 µs |
| sqrt | 20.697 ns | 20.297 ns | 34.859 ns |

### Wide-tier strict - D57 / D76 / D115 / D153 / D230 / D307 / D462 / D616 / D924 / D1232

Cost grows with both the work integer's bit width and the
guard-digit budget at each scale. The full v0.4.0 sweep now
populates every cell.

#### Wide (`wide` umbrella — D57 / D76 / D115 / D153 / D230 / D307)

| fn | D57 (s=28) | D76 (s=35) | D115 (s=57) | D153 (s=75) | D230 (s=115) | D307 (s=150) |
|---|---|---|---|---|---|---|
| ln | 36.425 µs | 40.569 µs | 31.353 µs | 67.527 µs | 83.541 µs | 161.25 µs |
| exp | 29.294 µs | 32.121 µs | 23.744 µs | 53.779 µs | 69.725 µs | 131.52 µs |
| sin | 20.661 µs | 22.591 µs | 18.288 µs | 40.827 µs | 54.728 µs | 104.43 µs |
| sqrt | 3.2400 µs | 3.5341 µs | 2.3271 µs | 4.9825 µs | 4.3601 µs | 7.8380 µs |

#### Extra-wide (`x-wide` adds D462 / D616)

| fn | D462 (s=230) | D616 (s=308) |
|---|---|---|
| ln | 229.97 µs | 368.73 µs |
| exp | 174.68 µs | 300.48 µs |
| sin | 159.36 µs | 252.42 µs |
| sqrt | 12.295 µs | 15.212 µs |

#### XX-wide (`xx-wide` adds D924 / D1232)

| fn | D924 (s=461) | D1232 (s=616) |
|---|---|---|
| ln | 873.53 µs | 1.5721 ms |
| exp | 715.05 µs | 1.2906 ms |
| sin | 690.32 µs | 1.2871 ms |
| sqrt | 34.839 µs | 53.822 µs |

**Historical comparison — 0.2.5 baseline.** On the same hardware,
0.2.5 measured D76<35> ln at 1.37 ms, D153<75> ln at 6.40 ms,
D307<150> ln at 34.1 ms. After this cycle's u64-native limbs, MG
2-by-1 reciprocal Knuth divide, Brent's two-stage exp argument
reduction, multi-level sqrt halving in ln, [0, π/4] sin range
reduction, sin_cos / sinh_cosh joint kernels, thread-local pi /
ln2 / ln10 cache, and pow10-cached mul/div per inner loop:

| op | 0.2 | 0.4 | speedup |
|---|---|---|---|
| D76<35>  ln   |  1.37 ms |  40.6 µs |  **34×** |
| D76<35>  exp  |  1.27 ms |  32.1 µs |  **40×** |
| D76<35>  sin  |  1.08 ms |  22.6 µs |  **48×** |
| D76<35>  sqrt | 20.5 µs  |  3.53 µs |   **6×** |
| D153<75> ln   |  6.40 ms |  67.5 µs |  **95×** |
| D153<75> exp  |  5.87 ms |  53.8 µs | **109×** |
| D153<75> sin  |  4.82 ms |  40.8 µs | **118×** |
| D153<75> sqrt | 83.6 µs  |  4.98 µs |  **17×** |
| D307<150> ln  | 34.1 ms  | 161.3 µs | **211×** |
| D307<150> exp | 31.2 ms  | 131.5 µs | **237×** |
| D307<150> sin | 25.5 ms  | 104.4 µs | **244×** |
| D307<150> sqrt|  313 µs  |  7.84 µs |  **40×** |

> The **0.2** column is the 0.2.5 baseline measured at the start of
> the 0.2.x cycle on the original dev box; the **0.4** column is
> from the v0.4.0 (GHA run [`26069911678`](https://github.com/mootable/decimal-scaled/actions/runs/26069911678))
> full_matrix sweep on GitHub-hosted `ubuntu-latest` standard
> runners. The two halves of each speedup are measured on
> **different machines** — the 0.2 column reflects cold-machine
> dev-box runs while the 0.4 column reflects shared CI runners
> that typically measure 1.5–2× slower per the run-conditions
> preamble below. The ratios are therefore conservative: a like-
> for-like cold-machine re-run of v0.4.0 would widen every
> speedup. Use these numbers as a directional "the algorithm
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

## 0.4.0 sweep — full raw data

Complete dump of the v0.4.0 full_matrix sweep (GitHub Actions run
[`26069911678`](https://github.com/mootable/decimal-scaled/actions/runs/26069911678)),
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
| `arith/D9_s0/add` | 934.42 ps |
| `arith/D9_s0/sub` | 933.84 ps |
| `arith/D9_s0/mul` | 933.75 ps |
| `arith/D9_s0/div` | 1.8658 ns |
| `arith/D9_s0/rem` | 1.8655 ns |
| `arith/D9_s0/neg` | 622.20 ps |
| `arith/D9_s5/add` | 933.09 ps |
| `arith/D9_s5/sub` | 933.37 ps |
| `arith/D9_s5/mul` | 1.5556 ns |
| `arith/D9_s5/div` | 2.1845 ns |
| `arith/D9_s5/rem` | 1.8662 ns |
| `arith/D9_s5/neg` | 622.49 ps |
| `arith/D9_s9/add` | 933.46 ps |
| `arith/D9_s9/sub` | 933.80 ps |
| `arith/D9_s9/mul` | 1.5563 ns |
| `arith/D9_s9/div` | 2.1782 ns |
| `arith/D9_s9/rem` | 1.8657 ns |
| `arith/D9_s9/neg` | 621.99 ps |
| `strict/D9_s0/ln` | 622.82 ps |
| `strict/D9_s0/exp` | 622.61 ps |
| `strict/D9_s0/sin` | 1.5576 ns |
| `strict/D9_s0/sqrt` | 7.0931 ns |
| `strict/D9_s5/ln` | 86.619 µs |
| `strict/D9_s5/exp` | 80.799 µs |
| `strict/D9_s5/sin` | 74.438 µs |
| `strict/D9_s5/sqrt` | 20.697 ns |
| `strict/D9_s9/ln` | 104.41 µs |
| `strict/D9_s9/exp` | 95.596 µs |
| `strict/D9_s9/sin` | 84.384 µs |
| `strict/D9_s9/sqrt` | 20.365 ns |

### `full_matrix_d18` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D18_s0/add` | 935.49 ps |
| `arith/D18_s0/sub` | 934.42 ps |
| `arith/D18_s0/mul` | 936.63 ps |
| `arith/D18_s0/div` | 5.9201 ns |
| `arith/D18_s0/rem` | 1.8669 ns |
| `arith/D18_s0/neg` | 622.51 ps |
| `arith/D18_s9/add` | 933.97 ps |
| `arith/D18_s9/sub` | 934.31 ps |
| `arith/D18_s9/mul` | 5.6151 ns |
| `arith/D18_s9/div` | 6.2239 ns |
| `arith/D18_s9/rem` | 1.8667 ns |
| `arith/D18_s9/neg` | 622.35 ps |
| `arith/D18_s18/add` | 936.63 ps |
| `arith/D18_s18/sub` | 937.32 ps |
| `arith/D18_s18/mul` | 6.0023 ns |
| `arith/D18_s18/div` | 6.2545 ns |
| `arith/D18_s18/rem` | 2.1813 ns |
| `arith/D18_s18/neg` | 624.40 ps |
| `arith/fixed_i64f64/add` | 1.8731 ns |
| `arith/fixed_i64f64/sub` | 1.8733 ns |
| `arith/fixed_i64f64/mul` | 2.4994 ns |
| `arith/fixed_i64f64/div` | 30.623 ns |
| `arith/fixed_i64f64/rem` | 13.138 ns |
| `arith/fixed_i64f64/neg` | 1.2483 ns |
| `strict/D18_s0/ln` | 624.26 ps |
| `strict/D18_s0/exp` | 625.91 ps |
| `strict/D18_s0/sin` | 1.2504 ns |
| `strict/D18_s0/sqrt` | 6.7349 ns |
| `strict/D18_s9/ln` | 104.72 µs |
| `strict/D18_s9/exp` | 95.852 µs |
| `strict/D18_s9/sin` | 84.749 µs |
| `strict/D18_s9/sqrt` | 20.297 ns |
| `strict/D18_s18/ln` | 145.71 µs |
| `strict/D18_s18/exp` | 127.64 µs |
| `strict/D18_s18/sin` | 110.04 µs |
| `strict/D18_s18/sqrt` | 29.942 ns |

### `full_matrix_d38` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D38_s0/add` | 1.1859 ns |
| `arith/D38_s0/sub` | 1.1973 ns |
| `arith/D38_s0/mul` | 3.4690 ns |
| `arith/D38_s0/div` | 6.7787 ns |
| `arith/D38_s0/rem` | 5.8664 ns |
| `arith/D38_s0/neg` | 867.78 ps |
| `arith/D38_s19/add` | 1.1841 ns |
| `arith/D38_s19/sub` | 1.1827 ns |
| `arith/D38_s19/mul` | 20.922 ns |
| `arith/D38_s19/div` | 9.2041 ns |
| `arith/D38_s19/rem` | 5.8757 ns |
| `arith/D38_s19/neg` | 867.88 ps |
| `arith/D38_s38/add` | 1.1842 ns |
| `arith/D38_s38/sub` | 1.1808 ns |
| `arith/D38_s38/mul` | 21.163 ns |
| `arith/D38_s38/div` | 654.60 ns |
| `arith/D38_s38/rem` | 9.9953 ns |
| `arith/D38_s38/neg` | 867.73 ps |
| `arith/rust_decimal_s19/add` | 9.2202 ns |
| `arith/rust_decimal_s19/sub` | 9.2870 ns |
| `arith/rust_decimal_s19/mul` | 38.262 ns |
| `arith/rust_decimal_s19/div` | 9.1609 ns |
| `arith/rust_decimal_s19/rem` | 17.741 ns |
| `arith/rust_decimal_s19/neg` | 1.9816 ns |
| `strict/D38_s0/ln` | 5.3432 µs |
| `strict/D38_s0/exp` | 105.97 ns |
| `strict/D38_s0/sin` | 12.033 µs |
| `strict/D38_s0/sqrt` | 7.4930 ns |
| `strict/D38_s19/ln` | 19.605 µs |
| `strict/D38_s19/exp` | 16.333 µs |
| `strict/D38_s19/sin` | 11.678 µs |
| `strict/D38_s19/sqrt` | 34.859 ns |
| `strict/D38_s38/ln` | 20.948 µs |
| `strict/D38_s38/exp` | 19.044 µs |
| `strict/D38_s38/sin` | 18.253 µs |
| `strict/D38_s38/sqrt` | 4.2224 µs |

### `full_matrix_d57` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D57_s0/add` | 2.6813 ns |
| `arith/D57_s0/sub` | 2.8485 ns |
| `arith/D57_s0/mul` | 80.446 ns |
| `arith/D57_s0/div` | 190.20 ns |
| `arith/D57_s0/rem` | 24.021 ns |
| `arith/D57_s0/neg` | 2.1822 ns |
| `arith/D57_s28/add` | 2.6801 ns |
| `arith/D57_s28/sub` | 2.8475 ns |
| `arith/D57_s28/mul` | 290.95 ns |
| `arith/D57_s28/div` | 449.22 ns |
| `arith/D57_s28/rem` | 168.10 ns |
| `arith/D57_s28/neg` | 2.1812 ns |
| `arith/D57_s56/add` | 2.6802 ns |
| `arith/D57_s56/sub` | 2.8478 ns |
| `arith/D57_s56/mul` | 318.52 ns |
| `arith/D57_s56/div` | 495.24 ns |
| `arith/D57_s56/rem` | 174.59 ns |
| `arith/D57_s56/neg` | 2.1817 ns |
| `strict_wide/D57_s0/ln` | 9.0868 µs |
| `strict_wide/D57_s0/exp` | 22.461 ns |
| `strict_wide/D57_s0/sin` | 19.572 µs |
| `strict_wide/D57_s0/sqrt` | 215.25 ns |
| `strict_wide/D57_s28/ln` | 36.425 µs |
| `strict_wide/D57_s28/exp` | 29.294 µs |
| `strict_wide/D57_s28/sin` | 20.661 µs |
| `strict_wide/D57_s28/sqrt` | 3.2400 µs |
| `strict_wide/D57_s56/ln` | 47.921 µs |
| `strict_wide/D57_s56/exp` | 29.499 µs |
| `strict_wide/D57_s56/sin` | 32.103 µs |
| `strict_wide/D57_s56/sqrt` | 3.7966 µs |

### `full_matrix_d76` (36 measurements)

| Op | Median |
|----|--------|
| `arith/D76_s0/add` | 3.7185 ns |
| `arith/D76_s0/sub` | 3.7274 ns |
| `arith/D76_s0/mul` | 79.068 ns |
| `arith/D76_s0/div` | 176.64 ns |
| `arith/D76_s0/rem` | 25.622 ns |
| `arith/D76_s0/neg` | 2.5578 ns |
| `arith/D76_s35/add` | 3.7207 ns |
| `arith/D76_s35/sub` | 3.7283 ns |
| `arith/D76_s35/mul` | 288.20 ns |
| `arith/D76_s35/div` | 464.47 ns |
| `arith/D76_s35/rem` | 170.89 ns |
| `arith/D76_s35/neg` | 2.5567 ns |
| `arith/D76_s76/add` | 3.7177 ns |
| `arith/D76_s76/sub` | 3.7286 ns |
| `arith/D76_s76/mul` | 352.83 ns |
| `arith/D76_s76/div` | 564.51 ns |
| `arith/D76_s76/rem` | 179.56 ns |
| `arith/D76_s76/neg` | 2.5607 ns |
| `arith/bnum_d76_s35/add` | 12.175 ns |
| `arith/bnum_d76_s35/sub` | 12.111 ns |
| `arith/bnum_d76_s35/mul` | 472.17 ns |
| `arith/bnum_d76_s35/div` | 482.05 ns |
| `arith/bnum_d76_s35/rem` | 70.253 ns |
| `arith/bnum_d76_s35/neg` | 17.054 ns |
| `strict_wide/D76_s0/ln` | 8.9485 µs |
| `strict_wide/D76_s0/exp` | 24.159 ns |
| `strict_wide/D76_s0/sin` | 19.561 µs |
| `strict_wide/D76_s0/sqrt` | 256.15 ns |
| `strict_wide/D76_s35/ln` | 40.569 µs |
| `strict_wide/D76_s35/exp` | 32.121 µs |
| `strict_wide/D76_s35/sin` | 22.591 µs |
| `strict_wide/D76_s35/sqrt` | 3.5341 µs |
| `strict_wide/D76_s76/ln` | 58.598 µs |
| `strict_wide/D76_s76/exp` | 41.436 µs |
| `strict_wide/D76_s76/sin` | 33.119 µs |
| `strict_wide/D76_s76/sqrt` | 4.4046 µs |

### `full_matrix_d115` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D115_s0/add` | 4.8551 ns |
| `arith/D115_s0/sub` | 5.6624 ns |
| `arith/D115_s0/mul` | 55.254 ns |
| `arith/D115_s0/div` | 162.08 ns |
| `arith/D115_s0/rem` | 27.116 ns |
| `arith/D115_s0/neg` | 3.1529 ns |
| `arith/D115_s57/add` | 4.8561 ns |
| `arith/D115_s57/sub` | 5.6541 ns |
| `arith/D115_s57/mul` | 229.39 ns |
| `arith/D115_s57/div` | 420.85 ns |
| `arith/D115_s57/rem` | 104.97 ns |
| `arith/D115_s57/neg` | 3.1497 ns |
| `arith/D115_s114/add` | 4.8515 ns |
| `arith/D115_s114/sub` | 5.6554 ns |
| `arith/D115_s114/mul` | 324.85 ns |
| `arith/D115_s114/div` | 483.67 ns |
| `arith/D115_s114/rem` | 114.23 ns |
| `arith/D115_s114/neg` | 3.1523 ns |
| `strict_wide/D115_s0/ln` | 5.4135 µs |
| `strict_wide/D115_s0/exp` | 32.031 ns |
| `strict_wide/D115_s0/sin` | 12.262 µs |
| `strict_wide/D115_s0/sqrt` | 195.40 ns |
| `strict_wide/D115_s57/ln` | 31.353 µs |
| `strict_wide/D115_s57/exp` | 23.744 µs |
| `strict_wide/D115_s57/sin` | 18.288 µs |
| `strict_wide/D115_s57/sqrt` | 2.3271 µs |
| `strict_wide/D115_s114/ln` | 46.322 µs |
| `strict_wide/D115_s114/exp` | 33.763 µs |
| `strict_wide/D115_s114/sin` | 28.187 µs |
| `strict_wide/D115_s114/sqrt` | 2.7028 µs |

### `full_matrix_d153` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D153_s0/add` | 7.3587 ns |
| `arith/D153_s0/sub` | 13.865 ns |
| `arith/D153_s0/mul` | 98.866 ns |
| `arith/D153_s0/div` | 292.26 ns |
| `arith/D153_s0/rem` | 39.690 ns |
| `arith/D153_s0/neg` | 5.0735 ns |
| `arith/D153_s75/add` | 7.3488 ns |
| `arith/D153_s75/sub` | 13.864 ns |
| `arith/D153_s75/mul` | 382.80 ns |
| `arith/D153_s75/div` | 871.16 ns |
| `arith/D153_s75/rem` | 190.44 ns |
| `arith/D153_s75/neg` | 5.0724 ns |
| `arith/D153_s153/add` | 7.3480 ns |
| `arith/D153_s153/sub` | 13.864 ns |
| `arith/D153_s153/mul` | 642.64 ns |
| `arith/D153_s153/div` | 1.0327 µs |
| `arith/D153_s153/rem` | 204.99 ns |
| `arith/D153_s153/neg` | 5.0731 ns |
| `strict_wide/D153_s0/ln` | 11.115 µs |
| `strict_wide/D153_s0/exp` | 42.388 ns |
| `strict_wide/D153_s0/sin` | 23.975 µs |
| `strict_wide/D153_s0/sqrt` | 280.37 ns |
| `strict_wide/D153_s75/ln` | 67.527 µs |
| `strict_wide/D153_s75/exp` | 53.779 µs |
| `strict_wide/D153_s75/sin` | 40.827 µs |
| `strict_wide/D153_s75/sqrt` | 4.9825 µs |
| `strict_wide/D153_s153/ln` | 109.49 µs |
| `strict_wide/D153_s153/exp` | 82.522 µs |
| `strict_wide/D153_s153/sin` | 68.564 µs |
| `strict_wide/D153_s153/sqrt` | 6.2663 µs |

### `full_matrix_d230` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D230_s0/add` | 16.739 ns |
| `arith/D230_s0/sub` | 19.019 ns |
| `arith/D230_s0/mul` | 65.709 ns |
| `arith/D230_s0/div` | 267.70 ns |
| `arith/D230_s0/rem` | 52.693 ns |
| `arith/D230_s0/neg` | 10.280 ns |
| `arith/D230_s115/add` | 16.736 ns |
| `arith/D230_s115/sub` | 19.016 ns |
| `arith/D230_s115/mul` | 431.16 ns |
| `arith/D230_s115/div` | 870.79 ns |
| `arith/D230_s115/rem` | 138.62 ns |
| `arith/D230_s115/neg` | 10.362 ns |
| `arith/D230_s230/add` | 16.744 ns |
| `arith/D230_s230/sub` | 19.014 ns |
| `arith/D230_s230/mul` | 944.22 ns |
| `arith/D230_s230/div` | 1.1877 µs |
| `arith/D230_s230/rem` | 153.42 ns |
| `arith/D230_s230/neg` | 10.651 ns |
| `strict_wide/D230_s0/ln` | 9.4737 µs |
| `strict_wide/D230_s0/exp` | 62.853 ns |
| `strict_wide/D230_s0/sin` | 23.549 µs |
| `strict_wide/D230_s0/sqrt` | 308.14 ns |
| `strict_wide/D230_s115/ln` | 83.541 µs |
| `strict_wide/D230_s115/exp` | 69.725 µs |
| `strict_wide/D230_s115/sin` | 54.728 µs |
| `strict_wide/D230_s115/sqrt` | 4.3601 µs |
| `strict_wide/D230_s230/ln` | 151.55 µs |
| `strict_wide/D230_s230/exp` | 116.86 µs |
| `strict_wide/D230_s230/sin` | 102.31 µs |
| `strict_wide/D230_s230/sqrt` | 7.0296 µs |

### `full_matrix_d307` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D307_s0/add` | 23.037 ns |
| `arith/D307_s0/sub` | 26.622 ns |
| `arith/D307_s0/mul` | 92.466 ns |
| `arith/D307_s0/div` | 375.04 ns |
| `arith/D307_s0/rem` | 66.987 ns |
| `arith/D307_s0/neg` | 13.953 ns |
| `arith/D307_s150/add` | 22.928 ns |
| `arith/D307_s150/sub` | 26.608 ns |
| `arith/D307_s150/mul` | 568.68 ns |
| `arith/D307_s150/div` | 1.3321 µs |
| `arith/D307_s150/rem` | 229.35 ns |
| `arith/D307_s150/neg` | 14.377 ns |
| `arith/D307_s307/add` | 22.918 ns |
| `arith/D307_s307/sub` | 26.621 ns |
| `arith/D307_s307/mul` | 1.5216 µs |
| `arith/D307_s307/div` | 2.4394 µs |
| `arith/D307_s307/rem` | 257.63 ns |
| `arith/D307_s307/neg` | 14.772 ns |
| `strict_wide/D307_s0/ln` | 16.426 µs |
| `strict_wide/D307_s0/exp` | 95.310 ns |
| `strict_wide/D307_s0/sin` | 38.240 µs |
| `strict_wide/D307_s0/sqrt` | 527.96 ns |
| `strict_wide/D307_s150/ln` | 161.25 µs |
| `strict_wide/D307_s150/exp` | 131.52 µs |
| `strict_wide/D307_s150/sin` | 104.43 µs |
| `strict_wide/D307_s150/sqrt` | 7.8380 µs |
| `strict_wide/D307_s307/ln` | 299.38 µs |
| `strict_wide/D307_s307/exp` | 233.39 µs |
| `strict_wide/D307_s307/sin` | 209.22 µs |
| `strict_wide/D307_s307/sqrt` | 19.518 µs |

### `full_matrix_d462` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D462_s0/add` | 26.347 ns |
| `arith/D462_s0/sub` | 44.482 ns |
| `arith/D462_s0/mul` | 148.97 ns |
| `arith/D462_s0/div` | 529.19 ns |
| `arith/D462_s0/rem` | 99.969 ns |
| `arith/D462_s0/neg` | 35.756 ns |
| `arith/D462_s230/add` | 26.372 ns |
| `arith/D462_s230/sub` | 44.476 ns |
| `arith/D462_s230/mul` | 1.2045 µs |
| `arith/D462_s230/div` | 2.2243 µs |
| `arith/D462_s230/rem` | 281.44 ns |
| `arith/D462_s230/neg` | 36.281 ns |
| `arith/D462_s461/add` | 26.389 ns |
| `arith/D462_s461/sub` | 44.503 ns |
| `arith/D462_s461/mul` | 3.1619 µs |
| `arith/D462_s461/div` | 4.0998 µs |
| `arith/D462_s461/rem` | 320.65 ns |
| `arith/D462_s461/neg` | 38.117 ns |
| `strict_wide/D462_s0/ln` | 16.914 µs |
| `strict_wide/D462_s0/exp` | 125.95 ns |
| `strict_wide/D462_s0/sin` | 39.960 µs |
| `strict_wide/D462_s0/sqrt` | 699.46 ns |
| `strict_wide/D462_s230/ln` | 229.97 µs |
| `strict_wide/D462_s230/exp` | 174.68 µs |
| `strict_wide/D462_s230/sin` | 159.36 µs |
| `strict_wide/D462_s230/sqrt` | 12.295 µs |
| `strict_wide/D462_s461/ln` | 503.23 µs |
| `strict_wide/D462_s461/exp` | 347.27 µs |
| `strict_wide/D462_s461/sin` | 357.09 µs |
| `strict_wide/D462_s461/sqrt` | 28.223 µs |

### `full_matrix_d616` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D616_s0/add` | 44.720 ns |
| `arith/D616_s0/sub` | 66.411 ns |
| `arith/D616_s0/mul` | 117.66 ns |
| `arith/D616_s0/div` | 465.22 ns |
| `arith/D616_s0/rem` | 101.62 ns |
| `arith/D616_s0/neg` | 38.610 ns |
| `arith/D616_s308/add` | 44.609 ns |
| `arith/D616_s308/sub` | 66.405 ns |
| `arith/D616_s308/mul` | 1.7899 µs |
| `arith/D616_s308/div` | 2.5389 µs |
| `arith/D616_s308/rem` | 185.84 ns |
| `arith/D616_s308/neg` | 39.818 ns |
| `arith/D616_s615/add` | 44.735 ns |
| `arith/D616_s615/sub` | 66.390 ns |
| `arith/D616_s615/mul` | 5.2515 µs |
| `arith/D616_s615/div` | 4.8258 µs |
| `arith/D616_s615/rem` | 280.11 ns |
| `arith/D616_s615/neg` | 41.372 ns |
| `strict_wide/D616_s0/ln` | 19.279 µs |
| `strict_wide/D616_s0/exp` | 122.24 ns |
| `strict_wide/D616_s0/sin` | 40.503 µs |
| `strict_wide/D616_s0/sqrt` | 608.99 ns |
| `strict_wide/D616_s308/ln` | 368.73 µs |
| `strict_wide/D616_s308/exp` | 300.48 µs |
| `strict_wide/D616_s308/sin` | 252.42 µs |
| `strict_wide/D616_s308/sqrt` | 15.212 µs |
| `strict_wide/D616_s615/ln` | 847.92 µs |
| `strict_wide/D616_s615/exp` | 646.61 µs |
| `strict_wide/D616_s615/sin` | 644.67 µs |
| `strict_wide/D616_s615/sqrt` | 35.257 µs |

### `full_matrix_d924` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D924_s0/add` | 94.060 ns |
| `arith/D924_s0/sub` | 120.69 ns |
| `arith/D924_s0/mul` | 205.57 ns |
| `arith/D924_s0/div` | 895.13 ns |
| `arith/D924_s0/rem` | 175.60 ns |
| `arith/D924_s0/neg` | 65.160 ns |
| `arith/D924_s461/add` | 94.035 ns |
| `arith/D924_s461/sub` | 120.64 ns |
| `arith/D924_s461/mul` | 3.6839 µs |
| `arith/D924_s461/div` | 6.1591 µs |
| `arith/D924_s461/rem` | 392.37 ns |
| `arith/D924_s461/neg` | 67.274 ns |
| `arith/D924_s923/add` | 94.078 ns |
| `arith/D924_s923/sub` | 120.60 ns |
| `arith/D924_s923/mul` | 11.280 µs |
| `arith/D924_s923/div` | 11.506 µs |
| `arith/D924_s923/rem` | 479.32 ns |
| `arith/D924_s923/neg` | 70.182 ns |
| `strict_wide/D924_s0/ln` | 34.127 µs |
| `strict_wide/D924_s0/exp` | 386.77 ns |
| `strict_wide/D924_s0/sin` | 73.407 µs |
| `strict_wide/D924_s0/sqrt` | 1.0426 µs |
| `strict_wide/D924_s461/ln` | 873.53 µs |
| `strict_wide/D924_s461/exp` | 715.05 µs |
| `strict_wide/D924_s461/sin` | 690.32 µs |
| `strict_wide/D924_s461/sqrt` | 34.839 µs |
| `strict_wide/D924_s923/ln` | 2.1913 ms |
| `strict_wide/D924_s923/exp` | 1.6598 ms |
| `strict_wide/D924_s923/sin` | 1.8844 ms |
| `strict_wide/D924_s923/sqrt` | 82.661 µs |

### `full_matrix_d1232` (30 measurements)

| Op | Median |
|----|--------|
| `arith/D1232_s0/add` | 120.02 ns |
| `arith/D1232_s0/sub` | 188.44 ns |
| `arith/D1232_s0/mul` | 259.39 ns |
| `arith/D1232_s0/div` | 1.0629 µs |
| `arith/D1232_s0/rem` | 217.59 ns |
| `arith/D1232_s0/neg` | 92.268 ns |
| `arith/D1232_s616/add` | 119.90 ns |
| `arith/D1232_s616/sub` | 188.67 ns |
| `arith/D1232_s616/mul` | 6.2126 µs |
| `arith/D1232_s616/div` | 8.9470 µs |
| `arith/D1232_s616/rem` | 475.36 ns |
| `arith/D1232_s616/neg` | 93.567 ns |
| `arith/D1232_s1231/add` | 119.95 ns |
| `arith/D1232_s1231/sub` | 188.50 ns |
| `arith/D1232_s1231/mul` | 19.642 µs |
| `arith/D1232_s1231/div` | 18.140 µs |
| `arith/D1232_s1231/rem` | 593.95 ns |
| `arith/D1232_s1231/neg` | 96.780 ns |
| `strict_wide/D1232_s0/ln` | 44.591 µs |
| `strict_wide/D1232_s0/exp` | 425.45 ns |
| `strict_wide/D1232_s0/sin` | 93.371 µs |
| `strict_wide/D1232_s0/sqrt` | 1.4626 µs |
| `strict_wide/D1232_s616/ln` | 1.5721 ms |
| `strict_wide/D1232_s616/exp` | 1.2906 ms |
| `strict_wide/D1232_s616/sin` | 1.2871 ms |
| `strict_wide/D1232_s616/sqrt` | 53.822 µs |
| `strict_wide/D1232_s1231/ln` | 4.1530 ms |
| `strict_wide/D1232_s1231/exp` | 3.1336 ms |
| `strict_wide/D1232_s1231/sin` | 3.8070 ms |
| `strict_wide/D1232_s1231/sqrt` | 132.36 µs |
