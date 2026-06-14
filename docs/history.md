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
| Unit | In nanoseconds |
| :-- | --: |
| ns | 10⁰ ns |
| µs | 10³ ns |
| ms | 10⁶ ns |
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
## Arithmetic

### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 210 ns (0.79×) | 215 ns (0.81×) | 266 ns |
| D38 | 241 ns (0.72×) | 256 ns (0.76×) | 335 ns |
| D57 | · | 2.15 µs (2.7×) | 791 ns |
| D76 | 872 ns (0.84×) | 2.73 µs (2.6×) | 1.04 µs |
| D115 | · | 6.08 µs (2.1×) | 2.94 µs |
| D153 | 1.99 µs (0.79×) | 5.54 µs (2.2×) | 2.51 µs |
| D230 | 4.14 µs (0.43×) | 16.1 µs (1.7×) | 9.52 µs |
| D307 | 5.04 µs (0.36×) | 21.4 µs (1.5×) | 14.2 µs |
| D462 | · | 40 µs (1.4×) | 29.2 µs |
| D616 | · | 63.7 µs (1.4×) | 44.9 µs |
| D924 | · | 125 µs (1.3×) | 92.6 µs |
| D1232 | · | 201 µs (1.3×) | 158 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,215.7 88.2,212.7 160.5,184.5 232.9,166.4 269.1,150.4 305.3,146.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.2 88.2,211.4 124.4,164.7 160.5,159.4 196.7,141.9 232.9,144.0 269.1,120.6 305.3,114.3 341.5,100.6 377.6,90.4 413.8,75.6 450.0,65.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,210.5 88.2,205.5 124.4,186.6 160.5,180.6 196.7,157.8 232.9,161.3 269.1,132.1 305.3,123.3 341.5,107.5 377.6,98.0 413.8,82.2 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 230 ns (0.92×) | 200 ns (0.8×) | 251 ns |
| D38 | 265 ns (0.68×) | 256 ns (0.65×) | 391 ns |
| D57 | · | 2.6 µs (3.3×) | 796 ns |
| D76 | · | 3.15 µs (3.1×) | 1.01 µs |
| D115 | · | 6.83 µs (2.5×) | 2.71 µs |
| D153 | · | 6.13 µs (2.8×) | 2.19 µs |
| D230 | · | 19 µs (2.3×) | 8.28 µs |
| D307 | 15.5 µs (1.2×) | 22.1 µs (1.8×) | 12.5 µs |
| D462 | · | 43.5 µs (1.7×) | 26.2 µs |
| D616 | · | 72.6 µs (1.8×) | 41.1 µs |
| D924 | · | 137 µs (1.4×) | 95.7 µs |
| D1232 | · | 220 µs (1.4×) | 163 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,213.7 88.2,210.6 305.3,121.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,216.8 88.2,211.4 124.4,160.6 160.5,156.3 196.7,139.4 232.9,141.7 269.1,117.0 305.3,113.6 341.5,98.8 377.6,87.5 413.8,73.5 450.0,63.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.8 88.2,202.1 124.4,186.5 160.5,181.3 196.7,159.6 232.9,164.3 269.1,135.1 305.3,126.1 341.5,109.8 377.6,100.0 413.8,81.5 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 240 ns (0.82×) | 221 ns (0.76×) | 291 ns |
| D38 | 270 ns (0.67×) | 271 ns (0.68×) | 401 ns |
| D57 | · | 2.06 µs (2.9×) | 716 ns |
| D76 | · | 2.55 µs (2.7×) | 941 ns |
| D115 | · | 5.44 µs (2.1×) | 2.63 µs |
| D153 | · | 5.12 µs (2.4×) | 2.16 µs |
| D230 | · | 15.3 µs (1.8×) | 8.47 µs |
| D307 | · | 19.8 µs (1.6×) | 12.6 µs |
| D462 | · | 38.3 µs (1.4×) | 26.7 µs |
| D616 | · | 62.3 µs (1.5×) | 40.9 µs |
| D924 | · | 128 µs (1.4×) | 89.7 µs |
| D1232 | · | 211 µs (1.3×) | 157 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,212.8 88.2,210.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,214.6 88.2,210.1 124.4,165.6 160.5,161.0 196.7,144.4 232.9,145.7 269.1,121.6 305.3,116.1 341.5,101.6 377.6,90.9 413.8,75.1 450.0,64.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,208.6 88.2,201.5 124.4,188.8 160.5,182.8 196.7,160.3 232.9,164.6 269.1,134.7 305.3,125.9 341.5,109.5 377.6,100.1 413.8,82.9 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 205 ns (0.76×) | 201 ns (0.74×) | 270 ns |
| D38 | 241 ns (0.73×) | 230 ns (0.7×) | 330 ns |
| D57 | · | 2.05 µs (2.9×) | 701 ns |
| D76 | · | 2.58 µs (2.6×) | 976 ns |
| D115 | · | 5.44 µs (2.2×) | 2.52 µs |
| D153 | · | 4.68 µs (2.3×) | 2.07 µs |
| D230 | · | 14 µs (1.8×) | 7.92 µs |
| D307 | · | 18.4 µs (1.5×) | 11.9 µs |
| D462 | · | 35.3 µs (1.5×) | 24.2 µs |
| D616 | · | 56.7 µs (1.4×) | 40 µs |
| D924 | · | 115 µs (1.3×) | 89.4 µs |
| D1232 | · | 191 µs (1.2×) | 159 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,216.3 88.2,212.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,216.7 88.2,213.7 124.4,165.7 160.5,160.7 196.7,144.4 232.9,147.7 269.1,123.7 305.3,117.6 341.5,103.3 377.6,92.9 413.8,77.5 450.0,66.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,210.2 88.2,205.8 124.4,189.3 160.5,182.0 196.7,161.2 232.9,165.5 269.1,136.1 305.3,127.1 341.5,111.6 377.6,100.6 413.8,83.0 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 196 ns (0.77×) | 206 ns (0.81×) | 255 ns |
| D38 | 235 ns (0.73×) | 231 ns (0.72×) | 320 ns |
| D57 | · | 2.23 µs (2.9×) | 771 ns |
| D76 | 1.04 µs (1×) | 2.72 µs (2.7×) | 1 µs |
| D115 | · | 6.11 µs (2.1×) | 2.94 µs |
| D153 | 921 ns (0.37×) | 5.67 µs (2.3×) | 2.5 µs |
| D230 | 3.03 µs (0.31×) | 16.1 µs (1.7×) | 9.64 µs |
| D307 | 3.8 µs (0.26×) | 21.1 µs (1.5×) | 14.4 µs |
| D462 | · | 40.4 µs (1.4×) | 28.9 µs |
| D616 | · | 61.5 µs (1.4×) | 43 µs |
| D924 | · | 124 µs (1.3×) | 95.3 µs |
| D1232 | · | 211 µs (1.3×) | 163 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,217.2 88.2,213.3 160.5,180.7 232.9,183.3 269.1,157.2 305.3,152.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,216.1 88.2,213.6 124.4,163.9 160.5,159.5 196.7,141.8 232.9,143.4 269.1,120.6 305.3,114.6 341.5,100.4 377.6,91.2 413.8,75.8 450.0,64.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.5 88.2,206.5 124.4,187.2 160.5,181.5 196.7,157.8 232.9,161.4 269.1,131.8 305.3,123.1 341.5,107.7 377.6,99.0 413.8,81.6 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

## Transcendentals

### `acos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 26.9 µs (0.5×) | 53.5 µs |
| D38 | 156 µs (2.7×) | 27 µs (0.47×) | 57.5 µs |
| D57 | · | 29.9 µs (2.2×) | 13.9 µs |
| D76 | 63.8 µs (3.7×) | 34.5 µs (2×) | 17.1 µs |
| D115 | · | 107 µs (2.3×) | 45.6 µs |
| D153 | · | 55.5 µs (2×) | 27.7 µs |
| D230 | · | 197 µs (2.5×) | 78 µs |
| D307 | · | 244 µs (2.6×) | 92.8 µs |
| D462 | · | 366 µs (2.3×) | 160 µs |
| D616 | · | 626 µs (2.6×) | 245 µs |
| D924 | · | 1.38 ms (2.9×) | 484 µs |
| D1232 | · | 5.14 ms (5.5×) | 928 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,151.7 160.5,177.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,203.0 88.2,202.9 124.4,199.9 160.5,195.8 196.7,162.7 232.9,181.9 269.1,144.8 305.3,138.6 341.5,126.7 377.6,111.0 413.8,87.9 450.0,49.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.9 88.2,180.8 124.4,222.3 160.5,216.3 196.7,187.6 232.9,202.2 269.1,171.9 305.3,166.9 341.5,150.9 377.6,138.5 413.8,118.6 450.0,99.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 28.5 µs (0.99×) | 28.6 µs |
| D38 | 102 µs (3.5×) | 29 µs (0.98×) | 29.5 µs |
| D57 | · | 71.4 µs (2.4×) | 29.2 µs |
| D76 | 37.3 µs (1.2×) | 77.6 µs (2.4×) | 32.4 µs |
| D115 | · | 314 µs (2.8×) | 113 µs |
| D153 | · | 184 µs (1.9×) | 96.1 µs |
| D230 | · | 732 µs (2×) | 360 µs |
| D307 | · | 894 µs (2.3×) | 382 µs |
| D462 | · | 1.62 ms (1.7×) | 929 µs |
| D616 | · | 5.9 ms (3×) | 1.95 ms |
| D924 | · | 13 ms (1.5×) | 8.89 ms |
| D1232 | · | 30.3 ms (1.8×) | 16.6 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,181.1 160.5,203.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,209.0 88.2,208.7 124.4,188.9 160.5,187.1 196.7,156.4 232.9,168.1 269.1,137.9 305.3,133.5 341.5,120.4 377.6,92.1 413.8,74.7 450.0,56.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,208.9 88.2,208.3 124.4,208.5 160.5,206.2 196.7,178.9 232.9,182.4 269.1,153.4 305.3,152.1 341.5,132.6 377.6,116.4 413.8,83.1 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.7 µs (0.92×) | 28.1 µs |
| D38 | 118 µs (3.9×) | 25.6 µs (0.86×) | 29.9 µs |
| D57 | · | 29 µs (2.1×) | 13.5 µs |
| D76 | 62.3 µs (3.7×) | 33.4 µs (2×) | 16.7 µs |
| D115 | · | 103 µs (2.3×) | 44.1 µs |
| D153 | · | 54.1 µs (1.9×) | 27.8 µs |
| D230 | · | 190 µs (2.5×) | 76.1 µs |
| D307 | 254 µs (2.8×) | 232 µs (2.6×) | 90.6 µs |
| D462 | · | 345 µs (2.2×) | 155 µs |
| D616 | · | 596 µs (2.5×) | 236 µs |
| D924 | · | 1.31 ms (2.8×) | 468 µs |
| D1232 | · | 5.16 ms (5.8×) | 889 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,159.9 160.5,178.5 305.3,137.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.4 88.2,204.5 124.4,200.8 160.5,196.8 196.7,163.7 232.9,182.6 269.1,145.9 305.3,140.0 341.5,128.5 377.6,112.4 413.8,89.5 450.0,49.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,201.8 88.2,199.9 124.4,223.1 160.5,217.0 196.7,188.6 232.9,202.1 269.1,172.7 305.3,167.6 341.5,151.9 377.6,139.6 413.8,119.6 450.0,100.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 30.8 µs (0.99×) | 31 µs |
| D38 | 114 µs (3.4×) | 32.9 µs (0.99×) | 33.1 µs |
| D57 | · | 20.7 µs (2.2×) | 9.4 µs |
| D76 | 36.7 µs (3.4×) | 23.1 µs (2.2×) | 10.7 µs |
| D115 | · | 74.3 µs (2.4×) | 30.7 µs |
| D153 | 38.1 µs (2×) | 39.3 µs (2.1×) | 19.1 µs |
| D230 | 124 µs (2.2×) | 140 µs (2.5×) | 56.7 µs |
| D307 | 150 µs (2.2×) | 172 µs (2.6×) | 67.3 µs |
| D462 | · | 263 µs (2.1×) | 124 µs |
| D616 | · | 455 µs (2.4×) | 193 µs |
| D924 | · | 1.05 ms (2.7×) | 386 µs |
| D1232 | · | 2.05 ms (2.8×) | 740 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,128.2 160.5,153.0 232.9,152.2 269.1,126.2 305.3,122.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,156.8 88.2,155.4 124.4,165.5 160.5,163.1 196.7,137.5 232.9,151.5 269.1,123.7 305.3,119.1 341.5,109.8 377.6,97.8 413.8,79.5 450.0,64.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,156.7 88.2,155.2 124.4,182.8 160.5,180.1 196.7,156.9 232.9,167.3 269.1,143.4 305.3,139.7 341.5,126.3 377.6,116.5 413.8,101.4 450.0,87.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.45×) | 57.3 µs |
| D38 | 180 µs (2.9×) | 27.2 µs (0.44×) | 61.5 µs |
| D57 | · | 29.2 µs (2.3×) | 12.6 µs |
| D76 | 58.1 µs (3.8×) | 32.4 µs (2.1×) | 15.2 µs |
| D115 | · | 102 µs (2.5×) | 41.6 µs |
| D153 | 62.3 µs (2.8×) | 48.7 µs (2.2×) | 21.9 µs |
| D230 | 186 µs (2.5×) | 191 µs (2.6×) | 74.1 µs |
| D307 | 227 µs (2.6×) | 231 µs (2.7×) | 86.6 µs |
| D462 | · | 340 µs (2.5×) | 136 µs |
| D616 | · | 586 µs (2.5×) | 235 µs |
| D924 | · | 1.38 ms (2.9×) | 468 µs |
| D1232 | · | 4.6 ms (4.9×) | 930 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,147.5 160.5,180.5 232.9,178.5 269.1,146.5 305.3,140.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.3 88.2,202.7 124.4,200.7 160.5,197.6 196.7,164.0 232.9,185.7 269.1,145.8 305.3,140.2 341.5,128.9 377.6,112.9 413.8,87.9 450.0,52.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,180.9 88.2,178.9 124.4,225.3 160.5,219.8 196.7,190.3 232.9,209.0 269.1,173.4 305.3,168.9 341.5,155.6 377.6,139.6 413.8,119.5 450.0,99.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.3 µs (0.79×) | 32.1 µs |
| D38 | 145 µs (4.2×) | 26.4 µs (0.76×) | 34.5 µs |
| D57 | · | 29.5 µs (2.3×) | 12.7 µs |
| D76 | 61.6 µs (3.9×) | 34.4 µs (2.2×) | 16 µs |
| D115 | · | 107 µs (2.5×) | 43.2 µs |
| D153 | 91.4 µs (3.4×) | 55.3 µs (2×) | 27.3 µs |
| D230 | 261 µs (3.4×) | 196 µs (2.5×) | 77.1 µs |
| D307 | 364 µs (3.9×) | 242 µs (2.6×) | 93.5 µs |
| D462 | · | 358 µs (2.2×) | 163 µs |
| D616 | · | 627 µs (2.5×) | 252 µs |
| D924 | · | 1.38 ms (2.7×) | 508 µs |
| D1232 | · | 5.23 ms (5.4×) | 967 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.7 160.5,178.8 232.9,167.3 269.1,136.6 305.3,126.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.8 88.2,203.7 124.4,200.4 160.5,195.9 196.7,162.7 232.9,182.0 269.1,144.9 305.3,138.8 341.5,127.4 377.6,111.0 413.8,87.9 450.0,49.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,197.9 88.2,195.8 124.4,224.9 160.5,218.3 196.7,189.2 232.9,202.7 269.1,172.3 305.3,166.6 341.5,150.4 377.6,137.6 413.8,117.1 450.0,98.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 10.1 µs (0.98×) | 10.3 µs |
| D38 | 132 µs (12×) | 10.8 µs (0.98×) | 11.1 µs |
| D57 | · | 106 µs (2.1×) | 50.6 µs |
| D76 | 41.4 µs (0.71×) | 114 µs (2×) | 57.9 µs |
| D115 | · | 420 µs (2.4×) | 179 µs |
| D153 | · | 236 µs (1.7×) | 141 µs |
| D230 | · | 873 µs (1.8×) | 497 µs |
| D307 | 2.31 ms (4.5×) | 1.13 ms (2.2×) | 516 µs |
| D462 | · | 1.76 ms (1.6×) | 1.07 ms |
| D616 | · | 5.95 ms (1.5×) | 3.99 ms |
| D924 | · | 12.4 ms (1.8×) | 7.01 ms |
| D1232 | · | 24.6 ms (1.9×) | 13.1 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,175.4 160.5,200.9 305.3,112.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,231.8 88.2,230.2 124.4,180.2 160.5,178.6 196.7,150.0 232.9,162.7 269.1,134.0 305.3,128.3 341.5,118.6 377.6,91.9 413.8,75.8 450.0,60.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,231.3 88.2,229.7 124.4,196.4 160.5,193.5 196.7,168.8 232.9,173.9 269.1,146.4 305.3,145.5 341.5,129.6 377.6,100.6 413.8,88.3 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 15.3 µs (2.8×) | 5.46 µs |
| D38 | 14.6 µs (1.8×) | 15.3 µs (1.9×) | 8.11 µs |
| D57 | · | 3.97 µs (3×) | 1.3 µs |
| D76 | 6.12 µs (3×) | 5.36 µs (2.6×) | 2.05 µs |
| D115 | · | 14.1 µs (2.4×) | 5.93 µs |
| D153 | 10.4 µs (2.3×) | 10 µs (2.2×) | 4.62 µs |
| D230 | 31.3 µs (1.7×) | 34.3 µs (1.8×) | 18.9 µs |
| D307 | 41.8 µs (1.5×) | 44.8 µs (1.7×) | 27 µs |
| D462 | · | 79.4 µs (1.5×) | 54.5 µs |
| D616 | · | 136 µs (1.5×) | 88.5 µs |
| D924 | · | 264 µs (1.5×) | 177 µs |
| D1232 | · | 456 µs (1.4×) | 318 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.5 160.5,179.0 232.9,163.4 269.1,131.3 305.3,122.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,152.3 88.2,152.2 124.4,191.7 160.5,182.9 196.7,154.6 232.9,164.6 269.1,128.6 305.3,120.8 341.5,104.1 377.6,88.3 413.8,68.9 450.0,53.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.4 88.2,170.8 124.4,224.3 160.5,211.0 196.7,179.9 232.9,187.3 269.1,146.0 305.3,135.6 341.5,115.1 377.6,100.9 413.8,80.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 5.98 µs (1×) | 5.97 µs |
| D38 | 78.7 µs (12×) | 6.33 µs (0.99×) | 6.39 µs |
| D57 | · | 19.1 µs (2.9×) | 6.57 µs |
| D76 | 29.3 µs (3.9×) | 21.6 µs (2.9×) | 7.56 µs |
| D115 | · | 70 µs (3.1×) | 22.2 µs |
| D153 | 14.9 µs (1.2×) | 34.2 µs (2.8×) | 12.2 µs |
| D230 | 32.4 µs (0.75×) | 141 µs (3.2×) | 43.4 µs |
| D307 | 40.1 µs (0.78×) | 174 µs (3.4×) | 51.7 µs |
| D462 | · | 260 µs (2.9×) | 90.5 µs |
| D616 | · | 461 µs (2.8×) | 166 µs |
| D924 | · | 1.02 ms (2.8×) | 359 µs |
| D1232 | · | 2.19 ms (3.1×) | 697 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,136.3 160.5,157.9 232.9,172.7 269.1,155.7 305.3,151.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.8 88.2,191.5 124.4,167.3 160.5,164.6 196.7,138.8 232.9,154.5 269.1,123.5 305.3,118.9 341.5,110.1 377.6,97.5 413.8,80.0 450.0,63.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.8 88.2,191.3 124.4,190.7 160.5,187.6 196.7,164.0 232.9,177.1 269.1,149.3 305.3,145.5 341.5,133.2 377.6,119.9 413.8,103.0 450.0,88.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.71 µs (0.99×) | 9.82 µs |
| D38 | 223 µs (21×) | 10.3 µs (0.98×) | 10.5 µs |
| D57 | · | 241 µs (27×) | 9.07 µs |
| D76 | 56.2 µs (5.1×) | 270 µs (25×) | 11 µs |
| D115 | · | 1.17 ms (42×) | 27.6 µs |
| D153 | 52 µs (2.9×) | 766 µs (43×) | 17.9 µs |
| D230 | 160 µs (3.1×) | 5.72 ms (1.1e+02×) | 52.1 µs |
| D307 | 209 µs (3.4×) | 7.02 ms (1.1e+02×) | 62 µs |
| D462 | · | 12.1 ms (1.1e+02×) | 114 µs |
| D616 | · | 35.2 ms (2.1e+02×) | 170 µs |
| D924 | · | 58.7 ms (1.7e+02×) | 344 µs |
| D1232 | · | 91.5 ms (1.4e+02×) | 635 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.3 232.9,162.7 269.1,142.9 305.3,138.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.8 160.5,133.8 196.7,108.0 232.9,115.5 269.1,80.2 305.3,76.6 341.5,67.1 377.6,48.3 413.8,39.3 450.0,31.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,190.8 124.4,193.3 160.5,189.9 196.7,173.8 232.9,181.3 269.1,162.6 305.3,159.6 341.5,149.0 377.6,141.9 413.8,129.5 450.0,118.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.06 µs (0.98×) | 6.18 µs |
| D38 | 63.9 µs (9.5×) | 6.59 µs (0.98×) | 6.71 µs |
| D57 | · | 22.5 µs (4.2×) | 5.42 µs |
| D76 | 29.1 µs (3.7×) | 24.1 µs (3.1×) | 7.82 µs |
| D115 | · | 63.8 µs (2.3×) | 27.9 µs |
| D153 | 29.3 µs (1.6×) | 35.9 µs (2×) | 18.2 µs |
| D230 | 89.5 µs (1.8×) | 135 µs (2.7×) | 50.3 µs |
| D307 | 121 µs (2×) | 169 µs (2.8×) | 61.3 µs |
| D462 | · | 245 µs (1.9×) | 128 µs |
| D616 | · | 446 µs (2.4×) | 185 µs |
| D924 | · | 886 µs (2.4×) | 367 µs |
| D1232 | · | 1.76 ms (2.4×) | 746 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,140.8 160.5,158.1 232.9,157.9 269.1,133.4 305.3,126.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.5 88.2,190.7 124.4,163.7 160.5,162.2 196.7,140.8 232.9,153.5 269.1,124.3 305.3,119.5 341.5,111.3 377.6,98.2 413.8,83.2 450.0,68.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,190.2 124.4,194.9 160.5,186.9 196.7,159.0 232.9,168.3 269.1,146.1 305.3,141.7 341.5,125.5 377.6,117.5 413.8,102.5 450.0,86.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.19 µs (0.99×) | 6.27 µs |
| D38 | 64.6 µs (9.6×) | 6.58 µs (0.98×) | 6.71 µs |
| D57 | · | 224 µs (16×) | 13.7 µs |
| D76 | 30.3 µs (2×) | 240 µs (16×) | 15.4 µs |
| D115 | · | 1.15 ms (19×) | 61 µs |
| D153 | 32.3 µs (0.9×) | 710 µs (20×) | 36 µs |
| D230 | 99.4 µs (0.69×) | 5.46 ms (38×) | 143 µs |
| D307 | 132 µs (0.62×) | 6.98 ms (33×) | 214 µs |
| D462 | · | 11.6 ms (39×) | 298 µs |
| D616 | · | 34.2 ms (34×) | 1 ms |
| D924 | · | 59.6 ms (44×) | 1.35 ms |
| D1232 | · | 95 ms (12×) | 8.26 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,158.9 160.5,172.2 232.9,171.0 269.1,151.3 305.3,146.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,200.0 88.2,198.9 124.4,137.1 160.5,135.9 196.7,108.3 232.9,116.8 269.1,81.0 305.3,76.7 341.5,67.8 377.6,48.8 413.8,39.1 450.0,30.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,199.8 88.2,198.6 124.4,186.1 160.5,184.0 196.7,159.9 232.9,169.1 269.1,144.9 305.3,137.8 341.5,132.1 377.6,110.8 413.8,105.5 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 290 ns (1.1×) | 271 ns |
| D38 | 336 ns (0.91×) | 341 ns (0.92×) | 370 ns |
| D57 | · | 3.57 µs (4.1×) | 866 ns |
| D76 | 4.63 µs (3.4×) | 4.73 µs (3.5×) | 1.37 µs |
| D115 | · | 11.4 µs (3×) | 3.84 µs |
| D153 | 5.92 µs (2×) | 8.07 µs (2.8×) | 2.9 µs |
| D230 | 19.4 µs (1.9×) | 26 µs (2.6×) | 10.1 µs |
| D307 | 21.5 µs (1.5×) | 31 µs (2.1×) | 14.7 µs |
| D462 | · | 58.3 µs (2×) | 29.6 µs |
| D616 | · | 90.2 µs (1.9×) | 46.7 µs |
| D924 | · | 176 µs (1.8×) | 100 µs |
| D1232 | · | 280 µs (1.6×) | 172 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,205.4 160.5,147.9 232.9,142.5 269.1,116.4 305.3,114.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,208.6 88.2,205.1 124.4,153.6 160.5,147.4 196.7,128.2 232.9,135.7 269.1,110.0 305.3,106.2 341.5,92.3 377.6,82.8 413.8,68.1 450.0,57.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,210.1 88.2,203.3 124.4,184.7 160.5,174.6 196.7,152.0 232.9,158.1 269.1,130.7 305.3,122.5 341.5,107.2 377.6,97.2 413.8,80.5 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 8.02 µs (0.96×) | 8.32 µs |
| D38 | 167 µs (19×) | 8.64 µs (0.96×) | 8.96 µs |
| D57 | · | 22.9 µs (8.1×) | 2.83 µs |
| D76 | 40.5 µs (11×) | 27.1 µs (7.2×) | 3.77 µs |
| D115 | · | 36.6 µs (4×) | 9.08 µs |
| D153 | 65.7 µs (10×) | 22.1 µs (3.4×) | 6.49 µs |
| D230 | 220 µs (8.5×) | 83.3 µs (3.2×) | 25.8 µs |
| D307 | 271 µs (9.2×) | 195 µs (6.6×) | 29.5 µs |
| D462 | · | 290 µs (4.7×) | 62 µs |
| D616 | · | 487 µs (4.6×) | 106 µs |
| D924 | · | 1.06 ms (4.6×) | 234 µs |
| D1232 | · | 4.62 ms (9.7×) | 475 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.8 160.5,150.8 232.9,140.2 269.1,113.7 305.3,109.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,186.4 88.2,184.7 124.4,163.3 160.5,159.6 196.7,153.1 232.9,164.1 269.1,135.0 305.3,116.3 341.5,107.7 377.6,96.3 413.8,79.1 450.0,47.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,185.5 88.2,183.9 124.4,209.2 160.5,202.9 196.7,183.6 232.9,191.0 269.1,160.7 305.3,157.7 341.5,141.5 377.6,129.8 413.8,112.4 450.0,96.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 16.2 µs (0.99×) | 16.4 µs |
| D38 | 204 µs (12×) | 17.3 µs (0.99×) | 17.5 µs |
| D57 | · | 79.6 µs (4.9×) | 16.1 µs |
| D76 | 74.2 µs (4×) | 94.1 µs (5×) | 18.6 µs |
| D115 | · | 301 µs (4.6×) | 64.8 µs |
| D153 | · | 159 µs (3.9×) | 41.1 µs |
| D230 | 423 µs (3×) | 587 µs (4.1×) | 143 µs |
| D307 | 514 µs (2.5×) | 707 µs (3.4×) | 209 µs |
| D462 | · | 1.04 ms (3×) | 353 µs |
| D616 | · | 1.88 ms (2.7×) | 688 µs |
| D924 | · | 6.81 ms (4.1×) | 1.67 ms |
| D1232 | · | 13 ms (2.2×) | 5.86 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,165.8 160.5,188.1 269.1,149.9 305.3,145.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.4 88.2,220.0 124.4,186.5 160.5,182.8 196.7,157.4 232.9,171.3 269.1,142.7 305.3,138.6 341.5,130.1 377.6,117.1 413.8,88.9 450.0,74.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,219.7 124.4,221.5 160.5,218.3 196.7,191.0 232.9,201.0 269.1,173.7 305.3,165.3 341.5,153.9 377.6,139.2 413.8,119.8 450.0,92.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.98×) | 12.1 µs |
| D38 | 172 µs (13×) | 12.8 µs (0.98×) | 13.1 µs |
| D57 | · | 44.5 µs (4.9×) | 9.1 µs |
| D76 | 40.5 µs (3.9×) | 51.9 µs (5×) | 10.3 µs |
| D115 | · | 161 µs (4.5×) | 35.9 µs |
| D153 | 71.2 µs (3.1×) | 90.5 µs (4×) | 22.6 µs |
| D230 | 228 µs (3×) | 316 µs (4.1×) | 76.6 µs |
| D307 | 277 µs (2.5×) | 380 µs (3.4×) | 112 µs |
| D462 | · | 548 µs (2.9×) | 186 µs |
| D616 | · | 953 µs (2.6×) | 361 µs |
| D924 | · | 2.16 ms (2.6×) | 826 µs |
| D1232 | · | 6.72 ms (4.2×) | 1.6 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.1 160.5,150.8 232.9,138.5 269.1,112.9 305.3,108.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.8 88.2,176.1 124.4,148.8 160.5,145.4 196.7,120.5 232.9,133.2 269.1,105.7 305.3,101.7 341.5,93.7 377.6,81.5 413.8,63.6 450.0,38.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.3 88.2,175.6 124.4,183.6 160.5,180.8 196.7,153.5 232.9,163.6 269.1,136.8 305.3,128.5 341.5,117.4 377.6,102.9 413.8,84.7 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.8 µs (0.98×) | 12.1 µs |
| D38 | 171 µs (13×) | 12.8 µs (0.98×) | 13.1 µs |
| D57 | · | 44.7 µs (4.9×) | 9.15 µs |
| D76 | 40.9 µs (3.9×) | 52.5 µs (5.1×) | 10.4 µs |
| D115 | · | 164 µs (4.5×) | 36 µs |
| D153 | 68.1 µs (3×) | 86.2 µs (3.8×) | 22.5 µs |
| D230 | 227 µs (3×) | 320 µs (4.2×) | 76.6 µs |
| D307 | 266 µs (2.4×) | 385 µs (3.4×) | 113 µs |
| D462 | · | 552 µs (3×) | 186 µs |
| D616 | · | 964 µs (2.7×) | 360 µs |
| D924 | · | 4.42 ms (5.3×) | 833 µs |
| D1232 | · | 6.69 ms (4.2×) | 1.58 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.2 160.5,150.6 232.9,139.4 269.1,113.1 305.3,109.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.8 88.2,176.1 124.4,148.6 160.5,145.1 196.7,120.2 232.9,134.3 269.1,105.5 305.3,101.4 341.5,93.5 377.6,81.3 413.8,47.9 450.0,38.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.4 88.2,175.7 124.4,183.4 160.5,180.7 196.7,153.4 232.9,163.7 269.1,136.8 305.3,128.4 341.5,117.4 377.6,102.9 413.8,84.5 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 251 ns (0.76×) | 331 ns |
| D38 | 215 µs (5.1e+02×) | 340 ns (0.8×) | 426 ns |
| D57 | · | 62.7 µs (4.1×) | 15.3 µs |
| D76 | 66.1 µs (3.8×) | 72.5 µs (4.2×) | 17.5 µs |
| D115 | · | 232 µs (3.7×) | 63.4 µs |
| D153 | 85.2 µs (2.1×) | 123 µs (3.1×) | 39.8 µs |
| D230 | 297 µs (2×) | 441 µs (3×) | 145 µs |
| D307 | 364 µs (1.7×) | 538 µs (2.6×) | 210 µs |
| D462 | · | 780 µs (2.4×) | 331 µs |
| D616 | · | 1.35 ms (1.8×) | 734 µs |
| D924 | · | 5.72 ms (3.1×) | 1.82 ms |
| D1232 | · | 10.6 ms (1.8×) | 5.72 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.8 160.5,137.1 232.9,133.3 269.1,115.1 305.3,112.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,218.5 88.2,214.1 124.4,137.8 160.5,135.7 196.7,118.7 232.9,128.0 269.1,109.3 305.3,106.4 341.5,101.0 377.6,92.9 413.8,71.8 450.0,62.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,214.5 88.2,210.8 124.4,158.4 160.5,156.5 196.7,137.7 232.9,144.5 269.1,125.6 305.3,120.1 341.5,113.5 377.6,101.8 413.8,88.6 450.0,71.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.08 µs (0.99×) | 6.14 µs |
| D38 | 83.3 µs (13×) | 6.52 µs (0.98×) | 6.62 µs |
| D57 | · | 18.8 µs (2.9×) | 6.47 µs |
| D76 | 28.9 µs (3.9×) | 21.4 µs (2.9×) | 7.44 µs |
| D115 | · | 72.6 µs (3.2×) | 22.8 µs |
| D153 | 10.8 µs (0.9×) | 34.2 µs (2.8×) | 12.1 µs |
| D230 | 30.1 µs (0.7×) | 144 µs (3.3×) | 43.1 µs |
| D307 | 37.5 µs (0.72×) | 171 µs (3.3×) | 51.8 µs |
| D462 | · | 258 µs (2.9×) | 89.8 µs |
| D616 | · | 454 µs (2.7×) | 165 µs |
| D924 | · | 1.01 ms (2.8×) | 357 µs |
| D1232 | · | 2.21 ms (3.2×) | 691 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,135.0 160.5,158.2 232.9,179.7 269.1,157.3 305.3,152.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.4 88.2,190.9 124.4,167.7 160.5,164.9 196.7,138.0 232.9,154.5 269.1,123.0 305.3,119.2 341.5,110.2 377.6,97.8 413.8,80.3 450.0,63.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.2 88.2,190.5 124.4,191.0 160.5,188.0 196.7,163.4 232.9,177.4 269.1,149.5 305.3,145.4 341.5,133.4 377.6,120.0 413.8,103.1 450.0,88.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.73 µs (0.99×) | 9.85 µs |
| D38 | 224 µs (21×) | 10.4 µs (0.99×) | 10.5 µs |
| D57 | · | 249 µs (27×) | 9.09 µs |
| D76 | 55.4 µs (5.1×) | 269 µs (25×) | 10.9 µs |
| D115 | · | 1.17 ms (41×) | 28.3 µs |
| D153 | 52.2 µs (2.9×) | 774 µs (42×) | 18.3 µs |
| D230 | 161 µs (3.1×) | 5.73 ms (1.1e+02×) | 51.6 µs |
| D307 | 211 µs (3.5×) | 7.02 ms (1.2e+02×) | 61 µs |
| D462 | · | 12.1 ms (1.1e+02×) | 112 µs |
| D616 | · | 34.6 ms (2.1e+02×) | 167 µs |
| D924 | · | 58.6 ms (1.7e+02×) | 346 µs |
| D1232 | · | 92.1 ms (1.5e+02×) | 635 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.6 232.9,162.6 269.1,142.8 305.3,138.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.2 160.5,133.8 196.7,108.0 232.9,115.3 269.1,80.2 305.3,76.6 341.5,67.0 377.6,48.6 413.8,39.4 450.0,31.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,190.7 124.4,193.3 160.5,190.0 196.7,173.3 232.9,181.0 269.1,162.8 305.3,159.9 341.5,149.1 377.6,142.2 413.8,129.4 450.0,118.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 220 ns (0.78×) | 281 ns |
| D38 | 241 ns (0.73×) | 230 ns (0.69×) | 331 ns |
| D57 | · | 2.62 µs (2.8×) | 951 ns |
| D76 | 3.69 µs (2.9×) | 3.44 µs (2.7×) | 1.28 µs |
| D115 | · | 8.61 µs (2.6×) | 3.35 µs |
| D153 | 4.91 µs (1.8×) | 6.08 µs (2.2×) | 2.77 µs |
| D230 | 15.7 µs (1.7×) | 19.9 µs (2.1×) | 9.41 µs |
| D307 | 18.7 µs (1.4×) | 24.8 µs (1.8×) | 13.5 µs |
| D462 | · | 45.5 µs (1.8×) | 25.4 µs |
| D616 | · | 69.1 µs (1.7×) | 40.4 µs |
| D924 | · | 138 µs (1.7×) | 79.3 µs |
| D1232 | · | 232 µs (1.8×) | 128 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,212.7 160.5,152.9 232.9,146.6 269.1,121.1 305.3,117.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,214.7 88.2,213.7 124.4,160.3 160.5,154.4 196.7,134.3 232.9,141.9 269.1,115.9 305.3,111.1 341.5,97.8 377.6,88.6 413.8,73.5 450.0,62.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,209.3 88.2,205.7 124.4,182.6 160.5,176.1 196.7,155.0 232.9,159.2 269.1,132.3 305.3,124.5 341.5,110.6 377.6,100.4 413.8,85.6 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.1 µs (0.99×) | 11.3 µs |
| D38 | 163 µs (13×) | 11.9 µs (0.98×) | 12.1 µs |
| D57 | · | 22.8 µs (2.8×) | 8.22 µs |
| D76 | 36.7 µs (3.9×) | 24.7 µs (2.6×) | 9.42 µs |
| D115 | · | 78.3 µs (2.9×) | 27.4 µs |
| D153 | 21.9 µs (1.5×) | 36.8 µs (2.5×) | 14.8 µs |
| D230 | · | 159 µs (3.1×) | 50.7 µs |
| D307 | 67.3 µs (1.1×) | 196 µs (3.2×) | 60.6 µs |
| D462 | · | 285 µs (2.8×) | 102 µs |
| D616 | · | 504 µs (2.7×) | 187 µs |
| D924 | · | 1.12 ms (2.9×) | 388 µs |
| D1232 | · | 4.35 ms (5.7×) | 763 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,120.2 160.5,153.0 232.9,164.3 305.3,139.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.1 88.2,177.6 124.4,163.4 160.5,161.6 196.7,136.4 232.9,152.9 269.1,120.9 305.3,116.3 341.5,108.0 377.6,95.5 413.8,78.0 450.0,48.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,178.9 88.2,177.3 124.4,185.8 160.5,182.8 196.7,159.4 232.9,172.9 269.1,145.9 305.3,142.0 341.5,130.5 377.6,117.2 413.8,101.3 450.0,86.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 12.9 µs (1.3×) | 9.79 µs |
| D38 | 194 µs (19×) | 13.7 µs (1.3×) | 10.4 µs |
| D57 | · | 254 µs (25×) | 10 µs |
| D76 | 56.3 µs (4.9×) | 273 µs (24×) | 11.5 µs |
| D115 | · | 1.32 ms (41×) | 31.9 µs |
| D153 | 55.6 µs (2.6×) | 781 µs (37×) | 21.1 µs |
| D230 | 169 µs (3.1×) | 5.46 ms (1e+02×) | 54.8 µs |
| D307 | 218 µs (3.2×) | 7.78 ms (1.1e+02×) | 68.9 µs |
| D462 | · | 12.1 ms (92×) | 132 µs |
| D616 | · | 35.3 ms (1.9e+02×) | 191 µs |
| D924 | · | 59.4 ms (1.5e+02×) | 386 µs |
| D1232 | · | 93.9 ms (1.2e+02×) | 777 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,139.6 160.5,161.3 232.9,161.5 269.1,142.0 305.3,137.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,187.2 88.2,186.1 124.4,134.8 160.5,133.6 196.7,105.9 232.9,115.1 269.1,81.0 305.3,74.8 341.5,67.1 377.6,48.3 413.8,39.1 450.0,31.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.0 88.2,190.8 124.4,191.6 160.5,189.2 196.7,171.2 232.9,178.5 269.1,161.8 305.3,157.7 341.5,146.3 377.6,139.9 413.8,127.5 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body -->

For the full list of changes, see the [Changelog](CHANGELOG.md).
