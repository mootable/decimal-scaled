# Performance

<div class="bench-header" markdown>
<div markdown>

How fast each operation is, by storage width and scale.

The numbers on this page are **generated from CI**: the `bench-branch-compare`
run measures every `(operation, width, scale)` on a GitHub-hosted runner,
compares it against the previous release, and commits the medians to
`results/timing/`, which this page renders. They are refreshed on each release
PR.

> Absolute timings are machine-dependent — the *ratios* between operations,
> measured in the same run, are what to read. Ratios **between widths** are a
> weaker reading: each width is benchmarked in its own job, on its own machine,
> so a width-to-width gap carries runner variance as well as code. The
> calibration section below measures that variance — treat a width gap smaller
> than it as unresolved rather than real. Operands are `black_box`-ed
> so the optimiser can't fold the work away.

Times are the unit shown in each cell; the legend maps each unit to its size in
nanoseconds. Each function's graph plots median time (log scale) against storage
width: the two solid lines are scale `0` and the maximum scale, the dashed lines
the band-edge scales in between, and the shaded band is the spread between scale
`0` and the maximum.

The per-function tables and graphs are split across three category pages:
**[Arithmetic](performance/arithmetic.md)**,
**[Roots and Exponents](performance/roots-and-exponents.md)**, and
**[Trigonometry](performance/trigonometry.md)**.

<!-- BEGIN GENERATED:performance:units -->
| Unit | In nanoseconds |
| :-- | --: |
| ns | 10⁰ ns |
| µs | 10³ ns |
| ms | 10⁶ ns |
<!-- END GENERATED:performance:units -->

</div>

<!-- BEGIN GENERATED:performance:widths -->
| Width | Decimals | Integer | Bits |
| :-- | --: | :-- | --: |
| D18 | 18 | `Int<1>` | 64 |
| D38 | 38 | `Int<2>` | 128 |
| D57 | 57 | `Int<3>` | 192 |
| D76 | 76 | `Int<4>` | 256 |
| D115 | 115 | `Int<6>` | 384 |
| D153 | 153 | `Int<8>` | 512 |
| D230 | 230 | `Int<12>` | 768 |
| D307 | 307 | `Int<16>` | 1024 |
| D462 | 462 | `Int<24>` | 1536 |
| D616 | 616 | `Int<32>` | 2048 |
| D924 | 924 | `Int<48>` | 3072 |
| D1232 | 1232 | `Int<64>` | 4096 |
<!-- END GENERATED:performance:widths -->

</div>

## How much difference is real?

Every timing on these pages was measured on a shared, GitHub-hosted machine, and
some of the spread between any two of them is the machine rather than the code.
So each run also measures its own instrument, and publishes what it found:

<!-- BEGIN GENERATED:performance:calibration -->
| Reference cost | Repeatability (spread over repeated measurement) |
| :-- | --: |
| 1.86 ns | ±4.7% |
| 6.22 ns | ±2.0% |
| 25 ns | ±2.0% |
| 99.7 ns | ±2.7% |
| 398 ns | ±2.2% |
| 1.59 µs | ±1.8% |
| 6.38 µs | ±1.7% |
| 25.5 µs | ±2.0% |

**Two timings on these pages that differ by less than about ±5% are not distinguishable at this run's resolution.** The table above is a synthetic reference workload — arithmetic with no library code in it — measured repeatedly at each cost during the same run that produced the timings. The spread of those repeats is measurement, not code, so it is the floor under any difference read off this page.

A fixed library operation — `decimal_scaled::D18<0> add` — is measured in the same runs, at 0.979 ns (±4.7%). It is held constant deliberately, so its movement across releases is a check on the library itself rather than on the machine. It is reported separately from the synthetic reference above and never combined with it: a combined figure could not tell a slower runner from slower code.

The runner's clock resolves 40 ns, but that is not the floor: each timing is an average over a long loop between two clock readings, so it can be — and for the cheapest operations is — far finer than a single tick. Repeatability, not the clock, is what bounds what can be read here.
<!-- END GENERATED:performance:calibration -->
