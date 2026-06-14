# Comparisons

<div class="bench-header" markdown>
<div markdown>

How `decimal-scaled` compares on **speed** against other Rust decimal crates,
timed on the same golden set. Each function has a table and a line chart with a
shaded min–max band: the median time of every library at one representative cell per
width, with `decimal-scaled` timed at **scale 30** — the precision the peer crates
effectively work at (see [Effective precision per library](#effective-precision-per-library)
below). The multiplier beside each peer is its time relative to
`decimal-scaled`; a library with **no line at a width** has no equivalent type
there. Generated from CI (`lib-perf`); refreshed on each release PR. The
per-function tables and graphs are split across
**[Arithmetic](comparisons/arithmetic.md)**,
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

`decimal-scaled` supports any scale up to each tier's maximum; this comparison fixes
the scale it is *timed* at so every library does a similar amount of work. It is
**pinned to scale 30** in most cases; on the lower tiers, where 30 is not one of the
benched band scales, the closest band ratio (¾, ½, or ¼ of the tier's max scale) is
used instead.

| Width | Scale timed |
| :-- | --: |
| D18 | 9 |
| D38 | 28 |
| D57 | **30** |
| D76 | 38 |
| D115 | 28 |
| D153 | 38 |
| D230 | **30** |
| D307 | **30** |
| D462 | **30** |
| D616 | **30** |
| D924 | **30** |
| D1232 | **30** |

**The libraries don't all measure precision the same way.** `decimal-scaled` and
`g_math` are *fixed-point* — precision is a fixed number of *fractional* digits. The
others cap *total significant figures*, so their fractional-digit count depends on the
size of the integer part.

| Library | Precision model | Working precision |
| :-- | :-- | :-- |
| `decimal-scaled` | Fixed-point — fixed fractional digits (the scale) | the timed scale (9–38; 30 at most tiers) |
| `g_math` | Fixed-point — Q128.128 | ~38 fractional digits |
| `rust_decimal` | Significant-figure, input-driven scale | up to 28 significant digits |
| `decimal-rs` | Significant-figure, input-driven scale | up to 38 significant digits |
| `fastnum` (D512) | Significant-figure, fixed coefficient | up to ~154 significant digits |
| `dashu-float` | Significant-figure, fixed working precision | ~1280 significant digits |
| `bigdecimal` | Exact arithmetic; fixed context for roots and division | exact, or ~1320 significant digits |

**Integer in, fractional out.** How much work a call does is set by how many output
digits the library produces, which depends on the operation:

- **Arithmetic** (`add`, `sub`, `mul`, `rem`) terminates — an integer in gives an
  integer out and no library adds fractional digits, so the work is set by the size of
  the operands. The arithmetic inputs here are integer-heavy and can be thousands of
  digits wide, so the exact-precision libraries do proportionally more work on the
  large operands.
- **Transcendentals and division** don't terminate — even an integer input gives an
  irrational or repeating result, so each library produces fractional digits up to its
  *own* working precision (the table above). For `decimal-scaled` that amount is the
  timed scale above — 30 at most tiers, 9–38 on the lower ones — close to what
  `rust_decimal`, `decimal-rs` and `g_math` produce.

**So how to read the timings.** Pinned to ~30 fractional digits, `decimal-scaled`
carries a precision close to `rust_decimal`, `decimal-rs` and `g_math` at every width,
so the comparison is like-for-like on *precision*. What it does **not** equalise is
*storage*: a `decimal-scaled` value keeps its full tier width — D1232 is a 1232-digit
fixed type — even when only 30 fractional digits are in play, so the wide tiers cost
more per call than a small-coefficient library doing the same 30-digit maths.
`dashu-float` and `bigdecimal` carry far more than 30 digits throughout, so they read
as slow everywhere; and a library producing fewer digits than 30 is doing **less
work**, not the same work faster.

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
