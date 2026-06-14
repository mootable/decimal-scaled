# Comparisons

<div class="bench-header" markdown>
<div markdown>

How `decimal-scaled` compares on **speed** against other fixed-precision Rust decimal
crates, timed on the same golden set. Each peer works at a fixed precision, so
`decimal-scaled` is timed at four matching precisions — **17, 28, 37 and 152 fractional
digits** (see [Effective precision per library](#effective-precision-per-library) below)
— and each peer is read against the `decimal-scaled` line doing a similar amount of
work. Each function has a chart: `decimal-scaled` as one line per comparison precision
across its width tiers (with a shaded min–max band), and each peer as a single **marker
at its significant-digit capacity** (with a min–max whisker). Generated from CI
(`lib-perf`); refreshed on each release PR. The per-function tables and graphs are split
across **[Arithmetic](comparisons/arithmetic.md)**,
**[Roots and Exponents](comparisons/roots-and-exponents.md)**, and
**[Trigonometry](comparisons/trigonometry.md)**.

</div>

<!-- BEGIN GENERATED:comparisons:units -->
| Unit | In nanoseconds |
| :-- | --: |
| ns | 10⁰ ns |
| µs | 10³ ns |
| ms | 10⁶ ns |
<!-- END GENERATED:comparisons:units -->

</div>

## Effective precision per library

`decimal-scaled` supports any scale up to each tier's maximum; this comparison times it
at **four fixed precisions**, one per peer-precision level, so each peer is matched
against `decimal-scaled` doing a comparable amount of work. Each line is drawn across
every width that can hold that scale.

| Comparison scale | Matches |
| :-- | :-- |
| `@17` | the narrow D18 ceiling — 18 significant digits |
| `@28` | `rust_decimal` — 28 significant digits |
| `@37` | the D38 ceiling = `decimal-rs` / `g_math` — 38 significant digits |
| `@152` | the D153 ceiling ≈ `fastnum` — ~154 significant digits |

**The libraries don't all measure precision the same way.** `decimal-scaled` and
`g_math` are *fixed-point* — precision is a fixed number of *fractional* digits. The
others cap *total significant figures*, so their fractional-digit count depends on the
size of the integer part.

| Library | Precision model | Working precision |
| :-- | :-- | :-- |
| `decimal-scaled` | Fixed-point — fixed fractional digits (the scale) | the four comparison scales: 17 / 28 / 37 / 152 |
| `g_math` | Fixed-point — Q128.128 | ~38 fractional digits |
| `rust_decimal` | Significant-figure, input-driven scale | up to 28 significant digits |
| `decimal-rs` | Significant-figure, input-driven scale | up to 38 significant digits |
| `fastnum` (D512) | Significant-figure, fixed coefficient | up to ~154 significant digits |

`bigdecimal` and `dashu-float` are **not timed here**: their working width is driven by
the input values rather than a fixed capacity, so they have no single marker position on
a width axis. Recovering their per-call operating width is deferred to a later release.

**Integer in, fractional out.** How much work a call does is set by how many output
digits the library produces, which depends on the operation:

- **Arithmetic** (`add`, `sub`, `mul`, `rem`) terminates — an integer in gives an
  integer out and no library adds fractional digits, so the work is set by the size of
  the operands. For `decimal-scaled` that work is set by the *tier width*, not the
  scale, so its four scale-lines nearly overlap on the arithmetic pages.
- **Transcendentals and division** don't terminate — even an integer input gives an
  irrational or repeating result, so each library produces fractional digits up to its
  *own* working precision (the table above), and `decimal-scaled`'s cost rises with the
  comparison scale. That is why the scale-lines fan apart on those pages, and why each
  peer is read against the `decimal-scaled` line nearest its precision.

**So how to read the timings.** At a matching precision `decimal-scaled` does
like-for-like *work* against each peer. What it does **not** equalise is *storage*: a
`decimal-scaled` value keeps its full tier width — D1232 is a 1232-digit fixed type —
even when only a few fractional digits are in play, so the wide tiers cost more per call
than a small-coefficient library doing the same maths. A peer producing fewer digits
than the matching line is doing **less work**, not the same work faster.

## Input distribution

The table below characterises the golden inputs each comparison runs over — per
function, how many input values it exercises and how their fractional- and
significant-digit counts are spread, plus the share that are exact integers (scale 0).

<!-- BEGIN GENERATED:comparisons:inputs -->
| Function | Inputs | Fractional digits (min/mean/max) | Significant digits (min/mean/max) | % integer |
| :-- | --: | :-- | :-- | --: |
| `add` | 160 | 0 / 6.8 / 28 | 0 / 22.8 / 28 | 61% |
| `div` | 132 | 0 / 2.8 / 26 | 1 / 12.9 / 28 | 86% |
| `mul` | 272 | 0 / 31.1 / 642 | 1 / 12.9 / 28 | 64% |
| `rem` | 130 | 0 / 2.9 / 26 | 1 / 13.4 / 28 | 85% |
| `sub` | 160 | 0 / 6.8 / 28 | 0 / 22.8 / 28 | 61% |
| `cbrt` | 5,680 | 0 / 14.1 / 1231 | 0 / 22.5 / 28 | 42% |
| `exp` | 4,636 | 0 / 23.2 / 1231 | 0 / 21.3 / 28 | 9% |
| `exp2` | 4,725 | 0 / 22.5 / 1231 | 0 / 20.7 / 28 | 12% |
| `hypot` | 260 | 0 / 3.9 / 27 | 0 / 16.4 / 28 | 79% |
| `ln` | 5,051 | 0 / 17.7 / 924 | 1 / 21.6 / 119 | 17% |
| `log` | 3,292 | 0 / 15.6 / 31 | 1 / 18.1 / 28 | 20% |
| `log10` | 5,264 | 0 / 17.7 / 924 | 1 / 22.2 / 1232 | 17% |
| `log2` | 5,034 | 0 / 17.2 / 924 | 1 / 21.2 / 28 | 19% |
| `powf` | 8,046 | 0 / 19.1 / 31 | 0 / 20.3 / 28 | 14% |
| `sqrt` | 5,599 | 0 / 12.4 / 1231 | 0 / 22.7 / 28 | 44% |
| `acos` | 4,015 | 0 / 23.3 / 118 | 0 / 23.0 / 28 | 0% |
| `acosh` | 4,302 | 0 / 15.7 / 118 | 1 / 20.5 / 119 | 20% |
| `asin` | 4,356 | 0 / 23.6 / 118 | 0 / 23.2 / 28 | 0% |
| `asinh` | 5,152 | 0 / 17.7 / 1231 | 0 / 24.1 / 28 | 33% |
| `atan` | 5,026 | 0 / 14.4 / 118 | 0 / 24.3 / 28 | 33% |
| `atan2` | 9,874 | 0 / 18.5 / 924 | 0 / 19.0 / 28 | 24% |
| `atanh` | 4,149 | 0 / 23.6 / 168 | 0 / 23.1 / 28 | 0% |
| `cos` | 4,377 | 0 / 22.3 / 118 | 0 / 23.1 / 28 | 2% |
| `cosh` | 5,123 | 0 / 24.1 / 1231 | 0 / 21.8 / 28 | 8% |
| `sin` | 4,381 | 0 / 22.3 / 118 | 0 / 23.1 / 28 | 2% |
| `sinh` | 4,841 | 0 / 24.3 / 1231 | 0 / 21.5 / 28 | 9% |
| `tan` | 4,418 | 0 / 22.6 / 118 | 0 / 23.0 / 28 | 2% |
| `tanh` | 4,517 | 0 / 26.2 / 1231 | 0 / 22.8 / 28 | 1% |
<!-- END GENERATED:comparisons:inputs -->
