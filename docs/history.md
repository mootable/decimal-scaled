# History

<div class="bench-header" markdown>
<div markdown>

How correctness and performance have moved release over release.

Generated from CI: the *history-gates* run measures the live crate beside pinned
past releases over the same golden set, and commits the per-version medians to
`results/history/`, which this page renders. Refreshed on each release PR.

Each function's table gives the median time at one representative cell per width
(the middle-of-band scale), one column per release, with the slowdown relative to
the latest release in parentheses — the latest column shows the time alone. The
graph plots the same medians on a log scale, one line per release. Units are
shown per cell; the legend maps each to its size in nanoseconds.

<!-- BEGIN GENERATED:history:units -->
_Pending the first history-gates CI run — this renders from `results/history/history.tsv`._
<!-- END GENERATED:history:units -->

</div>

<!-- BEGIN GENERATED:history:widths -->
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
<!-- END GENERATED:history:widths -->

</div>

<!-- BEGIN GENERATED:history:body -->
_Pending the first history-gates CI run — this renders from `results/history/history.tsv`._
<!-- END GENERATED:history:body -->

For the full list of changes, see the [Changelog](CHANGELOG.md).
