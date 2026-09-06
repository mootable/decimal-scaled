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
_Pending the first bench-branch-compare CI run with the timing probes — this renders from `results/timing/bbc_probe.tsv`._
<!-- END GENERATED:performance:calibration -->
