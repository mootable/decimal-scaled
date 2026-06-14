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
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 27.1 µs (0.51×) | 53.6 µs |
| D38 | 156 µs (2.7×) | 27.1 µs (0.47×) | 57.5 µs |
| D57 | · | 30 µs (2.2×) | 13.9 µs |
| D76 | 65.4 µs (3.8×) | 34.4 µs (2×) | 17.1 µs |
| D115 | · | 107 µs (2.4×) | 45.6 µs |
| D153 | · | 103 µs (2×) | 51.6 µs |
| D230 | · | 198 µs (2.6×) | 77.5 µs |
| D307 | · | 240 µs (2.6×) | 93.3 µs |
| D462 | · | 363 µs (2.3×) | 161 µs |
| D616 | · | 625 µs (2.5×) | 246 µs |
| D924 | · | 1.41 ms (2.7×) | 515 µs |
| D1232 | · | 5.14 ms (5.1×) | 1.01 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,151.7 160.5,177.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,202.9 88.2,202.9 124.4,199.9 160.5,195.9 196.7,162.6 232.9,163.7 269.1,144.6 305.3,139.0 341.5,126.9 377.6,111.1 413.8,87.4 450.0,49.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.9 88.2,180.8 124.4,222.3 160.5,216.3 196.7,187.6 232.9,184.0 269.1,172.1 305.3,166.7 341.5,150.7 377.6,138.3 413.8,116.8 450.0,97.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 28.5 µs (1×) | 28.6 µs |
| D38 | 102 µs (3.5×) | 29.9 µs (1×) | 29.5 µs |
| D57 | · | 71.8 µs (2.5×) | 29.2 µs |
| D76 | 37.5 µs (1.2×) | 79.6 µs (2.5×) | 32.4 µs |
| D115 | · | 314 µs (2.8×) | 113 µs |
| D153 | · | 338 µs (1.9×) | 182 µs |
| D230 | · | 719 µs (2×) | 361 µs |
| D307 | · | 900 µs (2.3×) | 383 µs |
| D462 | · | 1.62 ms (1.7×) | 943 µs |
| D616 | · | 5.88 ms (3.1×) | 1.92 ms |
| D924 | · | 13.5 ms (1.7×) | 8.04 ms |
| D1232 | · | 29.6 ms (1.7×) | 17.9 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,181.0 160.5,203.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,209.0 88.2,208.0 124.4,188.8 160.5,186.5 196.7,156.4 232.9,154.8 269.1,138.2 305.3,133.3 341.5,120.5 377.6,92.1 413.8,74.0 450.0,56.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,208.9 88.2,208.3 124.4,208.5 160.5,206.2 196.7,178.9 232.9,168.3 269.1,153.4 305.3,152.0 341.5,132.3 377.6,116.7 413.8,85.3 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 200 ns (0.8×) | 211 ns (0.84×) | 251 ns |
| D38 | 260 ns (0.85×) | 266 ns (0.87×) | 306 ns |
| D57 | · | 2.18 µs (2.8×) | 791 ns |
| D76 | 842 ns (0.8×) | 2.72 µs (2.6×) | 1.05 µs |
| D115 | · | 3.92 µs (1.3×) | 2.94 µs |
| D153 | 3.25 µs (0.71×) | 9.47 µs (2.1×) | 4.6 µs |
| D230 | 4.02 µs (0.43×) | 16 µs (1.7×) | 9.32 µs |
| D307 | 5.2 µs (0.37×) | 21.2 µs (1.5×) | 14.1 µs |
| D462 | · | 40.2 µs (1.4×) | 28.7 µs |
| D616 | · | 63.3 µs (1.4×) | 44 µs |
| D924 | · | 135 µs (1.3×) | 102 µs |
| D1232 | · | 223 µs (1.2×) | 179 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,216.8 88.2,211.0 160.5,185.3 232.9,155.7 269.1,151.0 305.3,145.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.6 88.2,210.5 124.4,164.4 160.5,159.5 196.7,151.5 232.9,132.2 269.1,120.6 305.3,114.5 341.5,100.5 377.6,90.5 413.8,73.9 450.0,62.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.8 88.2,207.5 124.4,186.6 160.5,180.4 196.7,157.8 232.9,148.0 269.1,132.5 305.3,123.5 341.5,107.8 377.6,98.5 413.8,80.1 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.92×) | 28.1 µs |
| D38 | 118 µs (3.9×) | 25.7 µs (0.85×) | 30 µs |
| D57 | · | 29 µs (2.1×) | 13.5 µs |
| D76 | 63.9 µs (3.8×) | 33.2 µs (2×) | 16.7 µs |
| D115 | · | 103 µs (2.3×) | 44.4 µs |
| D153 | · | 99.3 µs (1.9×) | 51.4 µs |
| D230 | · | 190 µs (2.5×) | 76.3 µs |
| D307 | 251 µs (2.7×) | 231 µs (2.5×) | 91.6 µs |
| D462 | · | 342 µs (2.2×) | 155 µs |
| D616 | · | 596 µs (2.5×) | 236 µs |
| D924 | · | 1.34 ms (2.7×) | 499 µs |
| D1232 | · | 5.11 ms (5.3×) | 955 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,159.8 160.5,177.8 305.3,137.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.3 88.2,204.4 124.4,200.9 160.5,196.9 196.7,163.8 232.9,164.9 269.1,145.9 305.3,140.2 341.5,128.7 377.6,112.4 413.8,88.8 450.0,49.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,201.8 88.2,199.8 124.4,223.1 160.5,217.0 196.7,188.4 232.9,184.1 269.1,172.6 305.3,167.2 341.5,151.8 377.6,139.6 413.8,117.7 450.0,98.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 30.8 µs (0.99×) | 31 µs |
| D38 | 114 µs (3.4×) | 32.9 µs (0.99×) | 33.1 µs |
| D57 | · | 20.8 µs (2.2×) | 9.42 µs |
| D76 | 37 µs (3.5×) | 23.6 µs (2.2×) | 10.6 µs |
| D115 | · | 74 µs (2.4×) | 30.7 µs |
| D153 | 72.7 µs (2×) | 71.6 µs (2×) | 35.9 µs |
| D230 | 125 µs (2.2×) | 139 µs (2.5×) | 56.3 µs |
| D307 | 169 µs (2.5×) | 176 µs (2.6×) | 67.9 µs |
| D462 | · | 264 µs (2.2×) | 122 µs |
| D616 | · | 451 µs (2.3×) | 193 µs |
| D924 | · | 977 µs (2.4×) | 411 µs |
| D1232 | · | 2.03 ms (2.6×) | 788 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,128.2 160.5,152.8 232.9,138.0 269.1,126.2 305.3,119.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,156.8 88.2,155.4 124.4,165.4 160.5,162.6 196.7,137.6 232.9,138.3 269.1,123.7 305.3,118.6 341.5,109.7 377.6,98.0 413.8,81.0 450.0,65.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,156.7 88.2,155.2 124.4,182.8 160.5,180.1 196.7,156.9 232.9,153.5 269.1,143.6 305.3,139.5 341.5,126.6 377.6,116.6 413.8,100.0 450.0,85.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.45×) | 57.3 µs |
| D38 | 180 µs (2.9×) | 27.3 µs (0.44×) | 61.6 µs |
| D57 | · | 29.3 µs (2.3×) | 12.5 µs |
| D76 | 59.8 µs (3.9×) | 32.6 µs (2.1×) | 15.2 µs |
| D115 | · | 103 µs (2.4×) | 42 µs |
| D153 | 120 µs (2.9×) | 86.2 µs (2.1×) | 40.9 µs |
| D230 | 185 µs (2.5×) | 189 µs (2.6×) | 73.6 µs |
| D307 | 228 µs (2.6×) | 231 µs (2.7×) | 86.4 µs |
| D462 | · | 335 µs (2.5×) | 137 µs |
| D616 | · | 593 µs (2.5×) | 236 µs |
| D924 | · | 1.36 ms (2.7×) | 505 µs |
| D1232 | · | 4.65 ms (4.7×) | 1e+03 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,147.4 160.5,179.7 232.9,159.3 269.1,146.7 305.3,140.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.3 88.2,202.6 124.4,200.6 160.5,197.5 196.7,163.9 232.9,169.0 269.1,146.1 305.3,140.2 341.5,129.3 377.6,112.6 413.8,88.3 450.0,52.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,181.0 88.2,178.8 124.4,225.4 160.5,219.8 196.7,190.0 232.9,190.8 269.1,173.6 305.3,168.9 341.5,155.5 377.6,139.5 413.8,117.3 450.0,97.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.3 µs (0.79×) | 32.1 µs |
| D38 | 145 µs (4.2×) | 26.3 µs (0.76×) | 34.4 µs |
| D57 | · | 29.6 µs (2.3×) | 12.8 µs |
| D76 | 63.1 µs (3.9×) | 34.2 µs (2.1×) | 16 µs |
| D115 | · | 107 µs (2.5×) | 43.1 µs |
| D153 | 172 µs (3.4×) | 104 µs (2.1×) | 50.8 µs |
| D230 | 285 µs (3.7×) | 197 µs (2.6×) | 77 µs |
| D307 | 352 µs (3.8×) | 242 µs (2.6×) | 93.7 µs |
| D462 | · | 361 µs (2.2×) | 163 µs |
| D616 | · | 634 µs (2.5×) | 252 µs |
| D924 | · | 1.43 ms (2.6×) | 540 µs |
| D1232 | · | 5.28 ms (5.1×) | 1.04 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.7 160.5,178.1 232.9,148.7 269.1,134.0 305.3,127.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.8 88.2,203.7 124.4,200.3 160.5,196.1 196.7,162.6 232.9,163.4 269.1,144.8 305.3,138.8 341.5,127.1 377.6,110.7 413.8,86.9 450.0,48.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,197.9 88.2,195.8 124.4,224.8 160.5,218.2 196.7,189.3 232.9,184.5 269.1,172.3 305.3,166.6 341.5,150.3 377.6,137.6 413.8,115.3 450.0,96.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 10.1 µs (0.98×) | 10.3 µs |
| D38 | 133 µs (12×) | 10.8 µs (0.97×) | 11.1 µs |
| D57 | · | 107 µs (2.1×) | 50.7 µs |
| D76 | 41.3 µs (0.72×) | 117 µs (2×) | 57.5 µs |
| D115 | · | 419 µs (2.3×) | 179 µs |
| D153 | · | 445 µs (1.7×) | 269 µs |
| D230 | · | 874 µs (1.8×) | 496 µs |
| D307 | 279 µs (0.54×) | 1.13 ms (2.2×) | 516 µs |
| D462 | · | 1.74 ms (1.6×) | 1.06 ms |
| D616 | · | 5.96 ms (1.2×) | 4.88 ms |
| D924 | · | 12.8 ms (1.4×) | 9.47 ms |
| D1232 | · | 24.6 ms (1.5×) | 16.4 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,175.2 160.5,200.9 305.3,159.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,231.9 88.2,230.3 124.4,180.1 160.5,178.1 196.7,150.1 232.9,148.8 269.1,134.0 305.3,128.2 341.5,118.8 377.6,91.9 413.8,75.1 450.0,60.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,231.4 88.2,229.7 124.4,196.4 160.5,193.6 196.7,168.8 232.9,159.8 269.1,146.4 305.3,145.5 341.5,129.7 377.6,96.2 413.8,81.7 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 15.3 µs (2.8×) | 5.44 µs |
| D38 | 14.6 µs (1.8×) | 15.3 µs (1.9×) | 8.09 µs |
| D57 | · | 3.94 µs (3×) | 1.3 µs |
| D76 | 6.37 µs (3.1×) | 5.45 µs (2.7×) | 2.05 µs |
| D115 | · | 14 µs (2.4×) | 5.9 µs |
| D153 | 19.6 µs (2.2×) | 18.6 µs (2.1×) | 8.73 µs |
| D230 | 30.7 µs (1.6×) | 34.7 µs (1.8×) | 18.9 µs |
| D307 | 41.7 µs (1.5×) | 44.9 µs (1.7×) | 27.1 µs |
| D462 | · | 81.6 µs (1.5×) | 54.6 µs |
| D616 | · | 138 µs (1.6×) | 88.4 µs |
| D924 | · | 283 µs (1.5×) | 183 µs |
| D1232 | · | 487 µs (1.5×) | 331 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.5 160.5,177.9 232.9,145.0 269.1,131.8 305.3,122.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,152.3 88.2,152.2 124.4,191.9 160.5,182.4 196.7,154.8 232.9,146.5 269.1,128.3 305.3,120.8 341.5,103.3 377.6,87.9 413.8,66.9 450.0,51.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.5 88.2,170.9 124.4,224.3 160.5,211.0 196.7,180.1 232.9,168.6 269.1,146.0 305.3,135.5 341.5,115.0 377.6,100.9 413.8,79.6 450.0,62.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6 µs (1×) | 5.97 µs |
| D38 | 77.3 µs (12×) | 6.37 µs (1×) | 6.37 µs |
| D57 | · | 19.3 µs (2.9×) | 6.59 µs |
| D76 | 29.4 µs (3.9×) | 22.2 µs (2.9×) | 7.55 µs |
| D115 | · | 69.4 µs (3.1×) | 22.4 µs |
| D153 | 28 µs (1.2×) | 61.2 µs (2.7×) | 22.7 µs |
| D230 | 33.1 µs (0.76×) | 140 µs (3.2×) | 43.4 µs |
| D307 | 39.9 µs (0.77×) | 176 µs (3.4×) | 52 µs |
| D462 | · | 265 µs (2.9×) | 90.6 µs |
| D616 | · | 455 µs (2.7×) | 166 µs |
| D924 | · | 1.06 ms (2.8×) | 383 µs |
| D1232 | · | 2.19 ms (2.9×) | 765 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,136.6 160.5,157.9 232.9,158.9 269.1,155.3 305.3,151.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.7 88.2,191.4 124.4,167.1 160.5,164.0 196.7,139.0 232.9,141.8 269.1,123.5 305.3,118.6 341.5,109.7 377.6,97.7 413.8,79.3 450.0,63.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.8 88.2,191.4 124.4,190.6 160.5,187.7 196.7,163.9 232.9,163.5 269.1,149.3 305.3,145.4 341.5,133.2 377.6,119.8 413.8,101.5 450.0,86.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.71 µs (0.99×) | 9.85 µs |
| D38 | 223 µs (21×) | 10.3 µs (0.99×) | 10.5 µs |
| D57 | · | 241 µs (27×) | 9.02 µs |
| D76 | 56.2 µs (5.1×) | 275 µs (25×) | 11 µs |
| D115 | · | 1.17 ms (42×) | 27.7 µs |
| D153 | 97.5 µs (2.8×) | 1.67 ms (48×) | 35.1 µs |
| D230 | 156 µs (3×) | 5.72 ms (1.1e+02×) | 51.9 µs |
| D307 | 207 µs (3.3×) | 7.03 ms (1.1e+02×) | 62.7 µs |
| D462 | · | 12.1 ms (1.1e+02×) | 112 µs |
| D616 | · | 35.2 ms (2.1e+02×) | 170 µs |
| D924 | · | 60.3 ms (1.6e+02×) | 373 µs |
| D1232 | · | 89.4 ms (1.3e+02×) | 698 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.3 232.9,151.6 269.1,143.3 305.3,138.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.8 160.5,133.5 196.7,108.0 232.9,101.8 269.1,80.2 305.3,76.6 341.5,67.1 377.6,48.3 413.8,38.9 450.0,32.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,190.8 124.4,193.4 160.5,189.9 196.7,173.7 232.9,169.6 269.1,162.7 305.3,159.4 341.5,149.2 377.6,141.9 413.8,128.1 450.0,117.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 220 ns (0.92×) | 210 ns (0.88×) | 240 ns |
| D38 | 266 ns (0.67×) | 270 ns (0.68×) | 400 ns |
| D57 | · | 2.59 µs (3.2×) | 806 ns |
| D76 | · | 3.19 µs (3.1×) | 1.02 µs |
| D115 | · | 6.85 µs (2.5×) | 2.75 µs |
| D153 | · | 10.8 µs (2.6×) | 4.1 µs |
| D230 | · | 18.4 µs (2.2×) | 8.23 µs |
| D307 | 14.4 µs (1.2×) | 21.4 µs (1.8×) | 12.2 µs |
| D462 | · | 43.9 µs (1.7×) | 26.1 µs |
| D616 | · | 71.6 µs (1.5×) | 48.7 µs |
| D924 | · | 147 µs (1.5×) | 98.9 µs |
| D1232 | · | 249 µs (1.5×) | 171 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,214.7 88.2,210.5 305.3,122.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.7 88.2,210.2 124.4,160.6 160.5,156.1 196.7,139.3 232.9,129.4 269.1,117.6 305.3,114.3 341.5,98.5 377.6,87.8 413.8,72.0 450.0,60.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,212.8 88.2,201.6 124.4,186.2 160.5,181.0 196.7,159.3 232.9,150.6 269.1,135.3 305.3,126.6 341.5,110.0 377.6,96.3 413.8,80.7 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.07 µs (0.98×) | 6.18 µs |
| D38 | 63.9 µs (9.5×) | 6.58 µs (0.98×) | 6.71 µs |
| D57 | · | 22.5 µs (4.1×) | 5.44 µs |
| D76 | 29 µs (3.8×) | 24 µs (3.1×) | 7.71 µs |
| D115 | · | 64.2 µs (2.3×) | 28 µs |
| D153 | 56.3 µs (1.6×) | 67 µs (1.9×) | 35 µs |
| D230 | 88.2 µs (1.8×) | 136 µs (2.7×) | 50 µs |
| D307 | 119 µs (1.9×) | 168 µs (2.7×) | 61.3 µs |
| D462 | · | 242 µs (1.9×) | 127 µs |
| D616 | · | 444 µs (2.4×) | 184 µs |
| D924 | · | 936 µs (2.3×) | 403 µs |
| D1232 | · | 1.76 ms (2.2×) | 815 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,140.8 160.5,158.1 232.9,143.6 269.1,133.8 305.3,127.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.4 88.2,190.7 124.4,163.7 160.5,162.3 196.7,140.7 232.9,139.8 269.1,124.3 305.3,119.6 341.5,111.6 377.6,98.3 413.8,82.0 450.0,68.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.0 88.2,190.3 124.4,194.9 160.5,187.2 196.7,158.9 232.9,154.0 269.1,146.2 305.3,141.7 341.5,125.8 377.6,117.6 413.8,100.4 450.0,85.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.19 µs (0.99×) | 6.27 µs |
| D38 | 64.8 µs (6.4×) | 6.59 µs (0.65×) | 10.2 µs |
| D57 | · | 227 µs (16×) | 14.3 µs |
| D76 | 30.3 µs (2×) | 240 µs (16×) | 15.4 µs |
| D115 | · | 1.15 ms (19×) | 61.1 µs |
| D153 | 59.8 µs (0.84×) | 1.64 ms (23×) | 70.8 µs |
| D230 | 96.5 µs (0.68×) | 5.45 ms (38×) | 143 µs |
| D307 | 129 µs (0.6×) | 6.99 ms (32×) | 215 µs |
| D462 | · | 11.6 ms (39×) | 299 µs |
| D616 | · | 34.3 ms (34×) | 1 ms |
| D924 | · | 60.7 ms (39×) | 1.54 ms |
| D1232 | · | 90.5 ms (10×) | 8.84 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,158.8 160.5,172.2 232.9,160.2 269.1,151.8 305.3,146.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,200.0 88.2,198.9 124.4,136.8 160.5,135.9 196.7,108.3 232.9,102.1 269.1,81.1 305.3,76.7 341.5,67.9 377.6,48.8 413.8,38.8 450.0,31.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,199.8 88.2,191.3 124.4,185.3 160.5,184.0 196.7,159.8 232.9,157.3 269.1,145.0 305.3,137.8 341.5,132.0 377.6,110.8 413.8,103.3 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 301 ns (1.2×) | 260 ns |
| D38 | 336 ns (0.93×) | 340 ns (0.94×) | 361 ns |
| D57 | · | 3.6 µs (4.2×) | 851 ns |
| D76 | 4.9 µs (3.4×) | 4.83 µs (3.4×) | 1.42 µs |
| D115 | · | 11.1 µs (3×) | 3.7 µs |
| D153 | 11.1 µs (2.1×) | 14.4 µs (2.7×) | 5.29 µs |
| D230 | 20.4 µs (2×) | 26.1 µs (2.5×) | 10.3 µs |
| D307 | 21.4 µs (1.5×) | 31 µs (2.1×) | 14.6 µs |
| D462 | · | 57.4 µs (1.9×) | 29.8 µs |
| D616 | · | 92.4 µs (2×) | 47.2 µs |
| D924 | · | 188 µs (1.8×) | 106 µs |
| D1232 | · | 304 µs (1.7×) | 183 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,205.4 160.5,146.6 232.9,128.8 269.1,115.4 305.3,114.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,207.8 88.2,205.2 124.4,153.4 160.5,147.0 196.7,128.7 232.9,122.9 269.1,110.0 305.3,106.2 341.5,92.7 377.6,82.2 413.8,66.6 450.0,56.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.0 88.2,203.8 124.4,185.0 160.5,173.8 196.7,152.8 232.9,145.0 269.1,130.4 305.3,122.7 341.5,107.1 377.6,97.0 413.8,79.1 450.0,67.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 8.02 µs (0.96×) | 8.31 µs |
| D38 | 167 µs (19×) | 8.64 µs (0.96×) | 8.96 µs |
| D57 | · | 23 µs (8.2×) | 2.81 µs |
| D76 | 40.2 µs (11×) | 27.1 µs (7.2×) | 3.75 µs |
| D115 | · | 36.6 µs (4×) | 9.07 µs |
| D153 | 120 µs (9.6×) | 40.8 µs (3.3×) | 12.5 µs |
| D230 | 249 µs (9.7×) | 82.6 µs (3.2×) | 25.7 µs |
| D307 | 272 µs (9.2×) | 197 µs (6.7×) | 29.6 µs |
| D462 | · | 291 µs (4.7×) | 61.9 µs |
| D616 | · | 484 µs (4.6×) | 106 µs |
| D924 | · | 1.08 ms (4.3×) | 250 µs |
| D1232 | · | 1.99 ms (3.9×) | 513 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.7 160.5,151.0 232.9,127.0 269.1,111.0 305.3,109.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,186.4 88.2,184.7 124.4,163.3 160.5,159.6 196.7,153.0 232.9,150.7 269.1,135.2 305.3,116.1 341.5,107.6 377.6,96.4 413.8,78.8 450.0,65.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,185.5 88.2,183.9 124.4,209.3 160.5,203.0 196.7,183.6 232.9,176.7 269.1,160.8 305.3,157.7 341.5,141.5 377.6,129.7 413.8,110.9 450.0,95.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 16.2 µs (0.98×) | 16.4 µs |
| D38 | 203 µs (12×) | 17.3 µs (0.98×) | 17.6 µs |
| D57 | · | 79.7 µs (5×) | 16.1 µs |
| D76 | 74.1 µs (4×) | 93.9 µs (5×) | 18.7 µs |
| D115 | · | 302 µs (4.7×) | 64.8 µs |
| D153 | · | 294 µs (3.7×) | 78.9 µs |
| D230 | 421 µs (3×) | 591 µs (4.1×) | 143 µs |
| D307 | 518 µs (2.5×) | 704 µs (3.4×) | 209 µs |
| D462 | · | 1.06 ms (3×) | 353 µs |
| D616 | · | 1.85 ms (2.7×) | 688 µs |
| D924 | · | 6.82 ms (4×) | 1.7 ms |
| D1232 | · | 12.8 ms (2.1×) | 5.98 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,165.9 160.5,188.1 269.1,150.0 305.3,145.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.4 88.2,220.0 124.4,186.5 160.5,182.9 196.7,157.3 232.9,157.8 269.1,142.5 305.3,138.7 341.5,129.8 377.6,117.5 413.8,88.9 450.0,75.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,221.1 88.2,219.6 124.4,221.6 160.5,218.3 196.7,191.0 232.9,186.7 269.1,173.7 305.3,165.3 341.5,153.8 377.6,139.2 413.8,119.4 450.0,91.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.98×) | 12.1 µs |
| D38 | 172 µs (13×) | 12.8 µs (0.98×) | 13.1 µs |
| D57 | · | 44.5 µs (4.9×) | 9.09 µs |
| D76 | 40.5 µs (3.9×) | 54.1 µs (5.2×) | 10.4 µs |
| D115 | · | 160 µs (4.5×) | 35.9 µs |
| D153 | 124 µs (2.9×) | 161 µs (3.7×) | 43.1 µs |
| D230 | 227 µs (3×) | 318 µs (4.1×) | 76.6 µs |
| D307 | 273 µs (2.4×) | 380 µs (3.4×) | 112 µs |
| D462 | · | 549 µs (3×) | 186 µs |
| D616 | · | 969 µs (2.7×) | 361 µs |
| D924 | · | 4.12 ms (4.8×) | 850 µs |
| D1232 | · | 6.59 ms (4×) | 1.65 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.1 160.5,150.9 232.9,126.3 269.1,113.0 305.3,109.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.7 88.2,176.1 124.4,148.7 160.5,144.5 196.7,120.7 232.9,120.5 269.1,105.6 305.3,101.7 341.5,93.7 377.6,81.2 413.8,49.5 450.0,39.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.4 88.2,175.6 124.4,183.6 160.5,180.7 196.7,153.5 232.9,149.5 269.1,136.8 305.3,128.5 341.5,117.4 377.6,102.9 413.8,84.1 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.98×) | 12.1 µs |
| D38 | 171 µs (13×) | 12.8 µs (0.98×) | 13 µs |
| D57 | · | 45 µs (4.9×) | 9.14 µs |
| D76 | 40.9 µs (3.9×) | 52.8 µs (5.1×) | 10.4 µs |
| D115 | · | 163 µs (4.5×) | 35.9 µs |
| D153 | 124 µs (2.9×) | 163 µs (3.8×) | 43.3 µs |
| D230 | 234 µs (3×) | 319 µs (4.1×) | 77 µs |
| D307 | 279 µs (2.5×) | 380 µs (3.4×) | 113 µs |
| D462 | · | 550 µs (2.9×) | 187 µs |
| D616 | · | 959 µs (2.7×) | 362 µs |
| D924 | · | 2.25 ms (2.6×) | 858 µs |
| D1232 | · | 6.65 ms (3.9×) | 1.69 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.2 160.5,150.6 232.9,126.3 269.1,112.3 305.3,108.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.7 88.2,176.2 124.4,148.5 160.5,145.0 196.7,120.3 232.9,120.2 269.1,105.5 305.3,101.7 341.5,93.6 377.6,81.4 413.8,62.7 450.0,38.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.4 88.2,175.7 124.4,183.5 160.5,180.7 196.7,153.5 232.9,149.4 269.1,136.7 305.3,128.4 341.5,117.3 377.6,102.8 413.8,83.9 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 320 ns (1.1×) | 231 ns (0.82×) | 281 ns |
| D38 | 290 ns (0.78×) | 271 ns (0.73×) | 371 ns |
| D57 | · | 2.06 µs (2.9×) | 711 ns |
| D76 | · | 2.59 µs (2.7×) | 961 ns |
| D115 | · | 5.44 µs (2.1×) | 2.62 µs |
| D153 | · | 8.59 µs (2.2×) | 3.92 µs |
| D230 | · | 15.2 µs (1.8×) | 8.47 µs |
| D307 | · | 19.6 µs (1.5×) | 12.8 µs |
| D462 | · | 37.7 µs (1.4×) | 27 µs |
| D616 | · | 62.6 µs (1.5×) | 42.3 µs |
| D924 | · | 139 µs (1.4×) | 99.1 µs |
| D1232 | · | 229 µs (1.4×) | 167 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,206.5 88.2,208.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,213.6 88.2,210.1 124.4,165.7 160.5,160.6 196.7,144.4 232.9,134.3 269.1,121.8 305.3,116.2 341.5,101.9 377.6,90.8 413.8,73.3 450.0,62.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,209.3 88.2,203.2 124.4,189.0 160.5,182.4 196.7,160.3 232.9,151.5 269.1,134.7 305.3,125.6 341.5,109.2 377.6,99.4 413.8,80.7 450.0,69.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 250 ns (0.76×) | 330 ns |
| D38 | 213 µs (5.1e+02×) | 326 ns (0.78×) | 416 ns |
| D57 | · | 63.3 µs (4.1×) | 15.4 µs |
| D76 | 66 µs (3.9×) | 72 µs (4.2×) | 17.1 µs |
| D115 | · | 232 µs (3.7×) | 63.1 µs |
| D153 | 161 µs (2.1×) | 227 µs (2.9×) | 77.3 µs |
| D230 | 296 µs (2×) | 442 µs (3×) | 145 µs |
| D307 | 365 µs (1.8×) | 535 µs (2.6×) | 205 µs |
| D462 | · | 778 µs (2.4×) | 331 µs |
| D616 | · | 1.35 ms (1.8×) | 747 µs |
| D924 | · | 5.76 ms (3×) | 1.95 ms |
| D1232 | · | 9.93 ms (1.7×) | 5.92 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,97.6 160.5,118.1 232.9,102.4 269.1,91.8 305.3,88.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.9 88.2,211.3 124.4,118.8 160.5,116.6 196.7,96.1 232.9,96.4 269.1,84.7 305.3,81.4 341.5,74.8 377.6,65.1 413.8,39.7 450.0,30.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.1 88.2,207.0 124.4,143.7 160.5,141.8 196.7,118.9 232.9,115.3 269.1,104.2 305.3,98.2 341.5,89.8 377.6,75.5 413.8,58.7 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 206 ns (0.85×) | 220 ns (0.91×) | 241 ns |
| D38 | 260 ns (0.81×) | 240 ns (0.75×) | 320 ns |
| D57 | · | 2.09 µs (3×) | 701 ns |
| D76 | · | 2.58 µs (2.8×) | 916 ns |
| D115 | · | 5.32 µs (2.1×) | 2.5 µs |
| D153 | · | 8.15 µs (2.2×) | 3.79 µs |
| D230 | · | 14.2 µs (1.8×) | 7.92 µs |
| D307 | · | 18.8 µs (1.6×) | 11.9 µs |
| D462 | · | 34.3 µs (1.4×) | 25.1 µs |
| D616 | · | 56.8 µs (1.4×) | 40.7 µs |
| D924 | · | 128 µs (1.3×) | 97.8 µs |
| D1232 | · | 207 µs (1.1×) | 182 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,216.1 88.2,211.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,214.7 88.2,212.8 124.4,165.3 160.5,160.7 196.7,144.8 232.9,135.5 269.1,123.3 305.3,117.1 341.5,104.0 377.6,92.9 413.8,75.1 450.0,64.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,212.7 88.2,206.5 124.4,189.3 160.5,183.4 196.7,161.4 232.9,152.3 269.1,136.1 305.3,127.2 341.5,110.8 377.6,100.2 413.8,81.0 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.15 µs (1×) | 6.12 µs |
| D38 | 83.1 µs (13×) | 6.51 µs (0.98×) | 6.62 µs |
| D57 | · | 18.9 µs (2.9×) | 6.42 µs |
| D76 | 28.9 µs (3.9×) | 21.8 µs (2.9×) | 7.49 µs |
| D115 | · | 72.1 µs (3.1×) | 22.9 µs |
| D153 | 21.2 µs (0.95×) | 59.5 µs (2.7×) | 22.3 µs |
| D230 | 30.9 µs (0.72×) | 144 µs (3.4×) | 42.9 µs |
| D307 | 46.5 µs (0.9×) | 171 µs (3.3×) | 51.7 µs |
| D462 | · | 259 µs (2.9×) | 89.9 µs |
| D616 | · | 457 µs (2.7×) | 167 µs |
| D924 | · | 1.03 ms (2.7×) | 382 µs |
| D1232 | · | 2.22 ms (2.9×) | 765 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,135.1 160.5,158.2 232.9,165.0 269.1,156.8 305.3,147.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.2 88.2,190.9 124.4,167.5 160.5,164.4 196.7,138.2 232.9,142.4 269.1,123.0 305.3,119.3 341.5,110.2 377.6,97.7 413.8,79.8 450.0,63.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.3 88.2,190.5 124.4,191.2 160.5,187.8 196.7,163.3 232.9,163.9 269.1,149.5 305.3,145.5 341.5,133.3 377.6,119.8 413.8,101.6 450.0,86.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.74 µs (0.99×) | 9.86 µs |
| D38 | 223 µs (21×) | 10.4 µs (0.99×) | 10.5 µs |
| D57 | · | 249 µs (27×) | 9.06 µs |
| D76 | 55.5 µs (5×) | 273 µs (25×) | 11 µs |
| D115 | · | 1.17 ms (42×) | 28.2 µs |
| D153 | 99.4 µs (2.7×) | 1.63 ms (45×) | 36.3 µs |
| D230 | 164 µs (3.2×) | 5.73 ms (1.1e+02×) | 51.3 µs |
| D307 | 211 µs (3.4×) | 7.01 ms (1.1e+02×) | 61.3 µs |
| D462 | · | 12.1 ms (1.1e+02×) | 112 µs |
| D616 | · | 35.1 ms (2.1e+02×) | 170 µs |
| D924 | · | 60.5 ms (1.6e+02×) | 369 µs |
| D1232 | · | 88.1 ms (1.3e+02×) | 690 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.5 232.9,151.3 269.1,142.5 305.3,138.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.2 160.5,133.6 196.7,108.0 232.9,102.2 269.1,80.2 305.3,76.6 341.5,67.1 377.6,48.4 413.8,38.8 450.0,32.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,190.8 124.4,193.3 160.5,189.9 196.7,173.4 232.9,169.0 269.1,162.9 305.3,159.8 341.5,149.3 377.6,141.9 413.8,128.3 450.0,117.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 211 ns (0.81×) | 260 ns |
| D38 | 251 ns (0.81×) | 240 ns (0.77×) | 310 ns |
| D57 | · | 2.61 µs (2.7×) | 952 ns |
| D76 | 3.81 µs (3×) | 3.4 µs (2.7×) | 1.27 µs |
| D115 | · | 8.64 µs (2.6×) | 3.36 µs |
| D153 | 9.06 µs (1.8×) | 10.6 µs (2.1×) | 5.13 µs |
| D230 | 15.8 µs (1.7×) | 19.8 µs (2.1×) | 9.39 µs |
| D307 | 19.2 µs (1.4×) | 24.3 µs (1.8×) | 13.5 µs |
| D462 | · | 46.1 µs (1.8×) | 25.2 µs |
| D616 | · | 67.9 µs (1.7×) | 40.4 µs |
| D924 | · | 149 µs (1.8×) | 82.8 µs |
| D1232 | · | 251 µs (1.9×) | 135 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,211.8 160.5,152.2 232.9,133.2 269.1,121.0 305.3,116.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.6 88.2,212.8 124.4,160.4 160.5,154.6 196.7,134.2 232.9,129.6 269.1,116.0 305.3,111.6 341.5,97.5 377.6,89.0 413.8,71.8 450.0,60.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.0 88.2,207.2 124.4,182.6 160.5,176.2 196.7,154.9 232.9,145.6 269.1,132.4 305.3,124.4 341.5,110.7 377.6,100.4 413.8,84.6 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 200 ns (0.85×) | 210 ns (0.89×) | 236 ns |
| D38 | 246 ns (0.82×) | 240 ns (0.8×) | 301 ns |
| D57 | · | 2.24 µs (2.9×) | 781 ns |
| D76 | 1.03 µs (1×) | 2.81 µs (2.8×) | 1 µs |
| D115 | · | 5.99 µs (2.1×) | 2.91 µs |
| D153 | 2.09 µs (0.46×) | 9.8 µs (2.2×) | 4.52 µs |
| D230 | 3.05 µs (0.32×) | 16.1 µs (1.7×) | 9.52 µs |
| D307 | 3.95 µs (0.28×) | 21.3 µs (1.5×) | 14.1 µs |
| D462 | · | 39.4 µs (1.4×) | 28.6 µs |
| D616 | · | 60.7 µs (1.4×) | 43.7 µs |
| D924 | · | 137 µs (1.3×) | 102 µs |
| D1232 | · | 225 µs (1.3×) | 178 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,216.8 88.2,212.3 160.5,180.8 232.9,165.3 269.1,157.1 305.3,151.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.7 88.2,212.8 124.4,163.8 160.5,158.8 196.7,142.3 232.9,131.4 269.1,120.6 305.3,114.4 341.5,100.9 377.6,91.5 413.8,73.6 450.0,62.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,213.2 88.2,207.8 124.4,186.9 160.5,181.5 196.7,158.1 232.9,148.4 269.1,132.1 305.3,123.5 341.5,108.0 377.6,98.6 413.8,80.0 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.2 µs (0.99×) | 11.3 µs |
| D38 | 164 µs (14×) | 11.9 µs (0.99×) | 12.1 µs |
| D57 | · | 22.8 µs (2.7×) | 8.3 µs |
| D76 | 37.5 µs (4×) | 25.3 µs (2.7×) | 9.39 µs |
| D115 | · | 78.5 µs (2.9×) | 27.5 µs |
| D153 | 40.8 µs (1.5×) | 67.7 µs (2.5×) | 27.3 µs |
| D230 | · | 160 µs (3.2×) | 50.7 µs |
| D307 | 67.2 µs (1.1×) | 194 µs (3.2×) | 60.6 µs |
| D462 | · | 283 µs (2.8×) | 103 µs |
| D616 | · | 504 µs (2.7×) | 187 µs |
| D924 | · | 1.14 ms (2.7×) | 419 µs |
| D1232 | · | 4.18 ms (5×) | 831 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,120.2 160.5,152.5 232.9,150.7 305.3,139.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.1 88.2,177.7 124.4,163.4 160.5,161.1 196.7,136.3 232.9,139.6 269.1,120.7 305.3,116.4 341.5,108.2 377.6,95.5 413.8,77.7 450.0,49.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,178.9 88.2,177.4 124.4,185.6 160.5,182.9 196.7,159.3 232.9,159.5 269.1,145.9 305.3,142.0 341.5,130.4 377.6,117.3 413.8,99.6 450.0,84.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 12.9 µs (1.3×) | 9.79 µs |
| D38 | 194 µs (19×) | 13.7 µs (1.3×) | 10.4 µs |
| D57 | · | 254 µs (25×) | 9.96 µs |
| D76 | 55.8 µs (4.9×) | 276 µs (24×) | 11.4 µs |
| D115 | · | 1.19 ms (37×) | 32.1 µs |
| D153 | 101 µs (2.5×) | 1.92 ms (47×) | 41.2 µs |
| D230 | 166 µs (3×) | 5.74 ms (1.1e+02×) | 54.6 µs |
| D307 | 219 µs (3.2×) | 7.67 ms (1.1e+02×) | 67.6 µs |
| D462 | · | 12.1 ms (94×) | 129 µs |
| D616 | · | 35.3 ms (1.9e+02×) | 189 µs |
| D924 | · | 60.7 ms (1.5e+02×) | 417 µs |
| D1232 | · | 90.2 ms (1.1e+02×) | 833 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,139.6 160.5,161.5 232.9,151.0 269.1,142.3 305.3,137.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,187.2 88.2,186.0 124.4,134.9 160.5,133.4 196.7,107.7 232.9,99.4 269.1,80.1 305.3,75.0 341.5,67.0 377.6,48.2 413.8,38.7 450.0,31.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.0 88.2,190.8 124.4,191.7 160.5,189.3 196.7,171.1 232.9,166.8 269.1,161.8 305.3,158.1 341.5,146.8 377.6,140.0 413.8,126.1 450.0,114.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body -->

For the full list of changes, see the [Changelog](CHANGELOG.md).
