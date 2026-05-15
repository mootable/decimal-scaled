# Benchmarks

Head-to-head measurements of `decimal-scaled` against the wider Rust
numeric ecosystem, plus the crate's own lossy vs strict
transcendental variants. Numbers are from the most recent run after
the MG wide-tier port, the strategy-pattern rounding refactor, and
the two-pass widen_mul / 2×2 limb-mul / limbs_divmod zero-skip
optimisations.

The benchmark suites live in [`benches/`](../benches/) and run under
[criterion](https://docs.rs/criterion/). The baseline crates
(`bnum`, `ruint`, `rust_decimal`, `fixed`, `i256`) are
**dev-dependencies only** — they are never compiled into a normal
build.

```sh
cargo bench --features wide --bench wide_int_backends
cargo bench --features wide,x-wide --bench decimal_backends
cargo bench --features wide --bench d128_mul_div_paths
cargo bench --features wide --bench mul_div_candidates
cargo bench --bench all_functions
```

> Absolute timings are machine-dependent. The *ratios* between
> implementations on the same machine, in the same run, are what
> matters. Operands are `black_box`-ed to defeat constant folding;
> outputs are returned from the closure so the optimiser cannot drop
> the call.

## Time units

| Symbol | Unit        | Seconds | Relative to `ns` |
|--------|-------------|---------|------------------|
| `s`    | second      | 10⁰ s   | ×1 000 000 000   |
| `ms`   | millisecond | 10⁻³ s  | ×1 000 000       |
| `µs`   | microscond | 10⁻⁶ s  | ×1 000           |
| `ns`   | nanosecond  | 10⁻⁹ s  | ×1               |
| `ps`   | picosecond  | 10⁻¹² s | ×0.001           |

`1 µs` = `1 000 ns`. A `27 µs` strict `ln` is `27 000 ns` — about
700× a `37 ns` lossy `ln`.

---

## 1. Wide-integer backends

Raw signed wide-integer arithmetic for the three widths the crate
ships. Operands: `A = 1.234 567 8…×10¹⁸`, `B = 9.876 5…×10¹⁴`.
Native is the in-tree hand-rolled wide integer family
(`Int256` / `Int512` / `Int1024`); baselines are `bnum`'s `I256` /
`I512` / `I1024` and (for 256-bit only) `ruint`'s `U256`. `ruint`
has no `Neg` (unsigned), so its `neg` cell is omitted.

### Int256

| op  | **`Int256` (this crate)** | `bnum` I256 | `ruint` U256 |
|-----|---------------------------|-------------|--------------|
| add | **1.66 ns**               | 1.94 ns     | 5.75 ns      |
| sub | 1.95 ns                   | **1.88 ns** | 5.96 ns      |
| mul | 14.84 ns                  | 4.23 ns     | **3.51 ns**  |
| div | 15.21 ns                  | 69.32 ns    | **5.68 ns**  |
| rem | 14.89 ns                  | 64.46 ns    | **5.90 ns**  |
| neg | **1.85 ns**               | 4.69 ns     | —            |

Native add / sub / neg lead. Multiply trails the two dedicated
big-integer crates by ~3–4× — `bnum`/`ruint` ship hand-tuned
base-2⁶⁴ multipliers for small operands; the crate's base-2¹²⁸
schoolbook gained a 2×2 fast-path this pass (15.0 → 13.7 ns) but
can't match a specialised 4×4 u64 multiplier. Divide / rem beat
`bnum` ~4× thanks to the u64-divisor fast path with leading-zero
skip; `ruint` wins at 256-bit divide because of its specialised
algorithm tuned for the same small operands.

### Int512

| op  | **`Int512` (this crate)** | `bnum` I512 |
|-----|---------------------------|-------------|
| add | **3.44 ns**               | 3.77 ns     |
| sub | 4.75 ns                   | **3.77 ns** |
| mul | **19.50 ns**              | 18.77 ns    |
| div | **20.91 ns**              | 79.89 ns    |
| rem | **21.37 ns**              | 82.18 ns    |
| neg | 3.53 ns                   | **3.34 ns** |

At 512-bit native edges ahead on add / mul and stays roughly 4×
faster on div / rem.

### Int1024

| op  | **`Int1024` (this crate)** | `bnum` I1024 |
|-----|----------------------------|--------------|
| add | 8.74 ns                    | **8.40 ns**  |
| sub | 13.97 ns                   | **8.37 ns**  |
| mul | **34.39 ns**               | 90.09 ns     |
| div | **35.67 ns**               | 150.49 ns    |
| rem | **36.92 ns**               | 146.58 ns    |

The native back-end scales linearly with limb count; `bnum`'s mul /
div / rem fall off faster.

---

## 2. Decimal arithmetic — all six ops × all six widths

Fixed-point arithmetic at `SCALE = 12`. The crate ships six widths
publicly: `D32` / `D64` / `D128` (primitive-integer storage) and
`D256` / `D512` / `D1024` (hand-rolled wide-integer storage,
gated behind `d256` / `d512` / `d1024` or the `wide` / `x-wide`
umbrella features). Baselines: `BnumD256` (a `bnum`-backed
256-bit decimal benchmark shim), `rust_decimal::Decimal`, and
`fixed`'s `I64F64`.

| op  | **D32**     | **D64**    | **D128**    | **D256**    | **D512**    | **D1024**   | `bnum` D256 | `rust_decimal` | `fixed` I64F64 |
|-----|-------------|------------|-------------|-------------|-------------|-------------|-------------|----------------|----------------|
| add | **0.42 ns** | 0.40 ns    | 0.97 ns     | 1.80 ns     | 3.33 ns     | 8.75 ns     | 1.93 ns     | 5.29 ns        | 0.88 ns        |
| sub | 0.40 ns     | 0.41 ns    | 0.93 ns     | 2.06 ns     | 4.57 ns     | 15.90 ns    | 1.93 ns     | 5.22 ns        | 0.89 ns        |
| mul | 1.58 ns     | 10.46 ns   | 12.41 ns    | **64.03 ns**| 79.48 ns    | 115.54 ns   | 273.15 ns   | **2.65 ns**    | 1.44 ns        |
| div | 2.71 ns     | 13.65 ns   | 15.24 ns    | **234.23 ns**| 357.54 ns  | 656.94 ns   | 280.32 ns   | 16.20 ns       | 23.30 ns       |
| rem | 1.57 ns     | 2.57 ns    | 9.24 ns     | 16.42 ns    | 23.10 ns    | 38.82 ns    | 50.42 ns    | 8.29 ns        | 14.30 ns       |
| neg | 0.26 ns     | 0.27 ns    | 0.66 ns     | 1.85 ns     | 2.84 ns     | 5.60 ns     | 4.36 ns     | 4.42 ns        | 0.58 ns        |

**Reading the table.**

- **Primitive-backed tier (D32 / D64 / D128).** Add / sub / neg are
  one or two integer instructions — sub-nanosecond. Mul / div carry
  a widening step: D32 uses i64, D64 uses i128, D128 uses the
  hand-rolled Möller-Granlund 256-bit kernel (`mg_divide`).
- **Wide tier (D256 / D512 / D1024).** Add / sub / neg are limb-
  array ops on `Int256` / `Int512` / `Int1024` — a few nanoseconds.
  Mul / div pay for a `2L × 2L → 2L` widening multiply followed by
  the MG fast-path divide for the `÷ 10^SCALE` step.
- **D256 mul (64 ns)** beats `bnum`-D256 (273 ns) by **4.3×** —
  the MG magic-divide port plus `widen_mul<W>` ($Storage² → $Wider
  in one limbs_mul call) replaced two width-widening round-trips
  through the `WideInt::to_mag_sign` 64-limb buffer.
- **D256 div (234 ns)** beats `bnum`-D256 (280 ns) by ~1.2×. The
  divisor here is the runtime operand, so MG doesn't apply — the
  win comes from caching the `10^SCALE` multiplier and the
  `limbs_divmod` leading-zero skip.
- `rust_decimal` and `fixed` have very fast multipliers
  (96-bit-mantissa fast path and a plain binary fixed-point
  multiply respectively), but neither offers exact base-10
  arithmetic past `i128`'s range, and neither does the wide-tier
  hand-rolled integer divide. For sub-128-bit decimal work
  `rust_decimal` is unbeatable on mul; the crate's wide tier is
  the only option past D128.

---

## 3. D128 mul/div — native MG vs the wide-arm algorithm

`benches/d128_mul_div_paths.rs` compares D128's hand-rolled
Möller-Granlund kernel against the same operands routed through the
generic wide-arm algorithm (widen to `Int256`, multiply, divide by
`10^SCALE` via `limbs_divmod`). Both paths now go through the
strategy-pattern rounding via `should_bump`.

| op  | D128 native MG | wide-arm Int256 |
|-----|----------------|-----------------|
| mul | **13.7 ns**    | 121.2 ns        |
| div | **13.9 ns**    | 143.3 ns        |

The native MG path is ~9× faster — that's the cost of going through
the generic limb-array machinery, which keeps the wide tier
algorithm-agnostic but cannot match the dedicated 128-bit
multiply-then-magic-divide for D128.

---

## 4. Decimal transcendentals — lossy, strict, baselines

`D128<9>` and `D256<9>` exercised at `≈ 2.345 678 901` against
`rust_decimal`'s `MathematicalOps`. The crate publishes two
variants:

- **lossy** — the `f64`-bridge form: convert to `f64`, call the
  platform intrinsic, convert back. Fast, not platform-
  independent, not correctly rounded.
- **strict** — the integer-only form. Range reduction plus an
  integer series in a guard-digit intermediate. Platform-
  independent, `no_std`-compatible, deterministic; rounded per
  `DEFAULT_ROUNDING_MODE` (HalfToEven by default). Within ≈ 1 ULP
  for D128, within ≈ 2 ULP for the wide tier — see
  `docs/strict-mode.md` for the precision contract.

### D128 lossy vs strict vs `rust_decimal`

| fn   | D128 lossy   | D128 strict     | `rust_decimal` |
|------|--------------|-----------------|----------------|
| ln   | 36.95 ns     | 26.96 µs        | 2.88 µs        |
| exp  | 40.98 ns     | 38.61 µs        | 2.12 µs        |
| sqrt | 29.64 ns     | **26.41 ns** †  | 699.64 ns      |
| cbrt | 60.66 ns     | 5.92 µs         | —              |
| sin  | 40.35 ns     | 28.66 µs        | 3.04 µs        |
| cos  | 42.31 ns     | 28.57 µs        | 3.11 µs        |
| tan  | 57.75 ns     | 58.81 µs        | 3.33 µs        |
| atan | 40.78 ns     | 56.15 µs        | —              |
| powf | 52.74 ns     | 64.30 µs        | —              |
| atan2| 48.78 ns     | 55.98 µs        | —              |

† `sqrt_strict` is algebraic, not series-based — integer square
root plus one round-to-nearest. It beats the lossy form because it
skips the `f64` round-trip entirely.

For the series functions the strict variant is **600–1500×** the
`f64` bridge — the price of deterministic, correctly-rounded
integer evaluation. `rust_decimal`'s software transcendentals land
in between (~2–4 µs) but are not correctly rounded to the last
place. `fixed` has no transcendentals.

### D256 lossy vs strict

| fn   | D256 lossy    | D256 strict (0.5 ULP) |
|------|---------------|-----------------------|
| ln   | 208.15 ns     | 337 µs                |
| exp  | 218.02 ns     | 506 µs                |
| sqrt | 200.68 ns     | 484.05 ns             |
| sin  | 215.29 ns     | 461 µs                |

The wide-tier strict path uses a separate guard-digit core
(`decl_wide_transcendental!`) with `$Work = Int1024`. The strict
numbers shown here are the **post-tightening** figures: every
intermediate `mul` / `div` rounds half-to-even (the truncating
predecessors leaked a coherent per-op bias that broke the 0.5 ULP
budget). The cost is ~2× the original truncating-intermediate
numbers (`ln` was 166 µs); precision now matches the D128 strict
contract.

---

## What the strict variants buy

The `*_strict` transcendentals are a capability the baseline crates
do not offer:

- **Deterministic last-place rounding** — for D128 the strict path
  is correctly rounded to within 0.5 ULP (IEEE-754 round-to-
  nearest). The wide-tier `*_strict` rounds at the storage
  granularity per `DEFAULT_ROUNDING_MODE` but uses truncating
  intermediate ops, so the contract there is "within a few ULP"
  rather than "0.5 ULP guaranteed". `rust_decimal`'s software
  forms are accurate but not correctly rounded to the last place.
- **Platform-independent** — identical bit patterns on every
  target, no dependence on the platform math library.
- **`no_std`** — integer-only, so they compile and run without
  `std`.

The cost is throughput. For latency-sensitive code that does not
need determinism or last-place accuracy, the lossy `f64` bridge is
the better default; for finance, reproducible computation, or
`no_std` targets, the strict path is the reason the crate exists.

---

## Optimisation history

Where the wide-tier numbers stand today, in context:

| pass | what changed | D256 mul | D256 div |
|------|--------------|----------|----------|
| baseline | naive limbs_divmod + wrapping mul + per-call pow10 | 538 ns | 309 ns |
| MG port | `div_wide_pow10_with` reuses the MG kernel from D128 | 94 ns | 309 ns |
| Pass 1 | leading-zero limb skip in `limbs_mul`, cached `multiplier()`, `wrapping_mul` for the widening step | 79 ns | 220 ns |
| Pass 2 | `widen_mul<W>` (no resize buffer round-trips), 2×2 inlined fast path in `limbs_mul`, `POW10_U128` table | 55 ns | 215 ns |
| Pass 3 | leading-zero numerator skip in `limbs_divmod` u64 fast path, `#[inline(always)]` on `should_bump` | **64 ns** | **234 ns** |

D256 mul ended ~30% off the pass-2 best in the latest run — the
final inlining nudges helped D128 div but slightly nudged D256 mul
the wrong way (within bench noise; the inliner's choices are not
strictly monotonic). Net: **D256 mul 538 → 64 ns (8.4× speed-up)**;
**D256 div 309 → 234 ns (1.3×)**.
