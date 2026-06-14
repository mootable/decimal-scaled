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
| D18 | · | 27.1 µs (0.51×) | 53.4 µs |
| D38 | 155 µs (2.7×) | 27.1 µs (0.47×) | 57.5 µs |
| D57 | · | 30 µs (2.2×) | 13.8 µs |
| D76 | 64 µs (3.7×) | 34.3 µs (2×) | 17.2 µs |
| D115 | · | 107 µs (2.3×) | 45.7 µs |
| D153 | · | 103 µs (2×) | 51.4 µs |
| D230 | · | 186 µs (2.4×) | 78.8 µs |
| D307 | · | 245 µs (2.6×) | 93.5 µs |
| D462 | · | 328 µs (1.9×) | 170 µs |
| D616 | · | 613 µs (2.4×) | 259 µs |
| D924 | · | 1.39 ms (2.9×) | 487 µs |
| D1232 | · | 5.16 ms (5.1×) | 1.02 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,151.8 160.5,177.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,202.9 88.2,202.9 124.4,199.9 160.5,196.0 196.7,162.7 232.9,163.7 269.1,146.5 305.3,138.5 341.5,129.9 377.6,111.6 413.8,87.7 450.0,49.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,183.0 88.2,180.8 124.4,222.5 160.5,216.2 196.7,187.5 232.9,184.1 269.1,171.6 305.3,166.6 341.5,149.2 377.6,136.8 413.8,118.4 450.0,96.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 28.5 µs (1×) | 28.6 µs |
| D38 | 102 µs (3.5×) | 29.5 µs (1×) | 29.1 µs |
| D57 | · | 71.2 µs (2.4×) | 29.3 µs |
| D76 | 37.3 µs (1.1×) | 77.3 µs (2.4×) | 32.4 µs |
| D115 | · | 315 µs (2.8×) | 113 µs |
| D153 | · | 338 µs (1.8×) | 183 µs |
| D230 | · | 699 µs (1.8×) | 384 µs |
| D307 | · | 900 µs (2.3×) | 388 µs |
| D462 | · | 1.5 ms (1.5×) | 1.02 ms |
| D616 | · | 5.86 ms (2.8×) | 2.1 ms |
| D924 | · | 12.9 ms (1.8×) | 7.31 ms |
| D1232 | · | 29.3 ms (1.6×) | 18.1 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,181.1 160.5,203.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,209.0 88.2,208.3 124.4,189.0 160.5,187.1 196.7,156.4 232.9,154.8 269.1,138.9 305.3,133.3 341.5,122.2 377.6,92.2 413.8,74.9 450.0,56.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,209.0 88.2,208.6 124.4,208.4 160.5,206.2 196.7,178.8 232.9,168.2 269.1,152.0 305.3,151.8 341.5,130.6 377.6,114.7 413.8,87.4 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 215 ns (0.84×) | 215 ns (0.84×) | 256 ns |
| D38 | 256 ns (0.86×) | 251 ns (0.85×) | 296 ns |
| D57 | · | 2.16 µs (2.7×) | 802 ns |
| D76 | 856 ns (0.84×) | 2.66 µs (2.6×) | 1.02 µs |
| D115 | · | 6.13 µs (2.1×) | 2.97 µs |
| D153 | 2.93 µs (0.64×) | 9.69 µs (2.1×) | 4.58 µs |
| D230 | 4.44 µs (0.46×) | 16.8 µs (1.7×) | 9.64 µs |
| D307 | 5.21 µs (0.37×) | 22 µs (1.6×) | 14.1 µs |
| D462 | · | 43.4 µs (1.4×) | 31.7 µs |
| D616 | · | 66.6 µs (1.3×) | 49.6 µs |
| D924 | · | 123 µs (1.3×) | 92.6 µs |
| D1232 | · | 223 µs (1.3×) | 174 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,215.2 88.2,211.4 160.5,184.9 232.9,157.9 269.1,148.8 305.3,145.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.2 88.2,211.8 124.4,164.6 160.5,160.0 196.7,141.7 232.9,131.7 269.1,119.7 305.3,113.7 341.5,98.8 377.6,89.4 413.8,75.9 450.0,62.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.4 88.2,208.2 124.4,186.3 160.5,181.0 196.7,157.6 232.9,148.1 269.1,131.8 305.3,123.4 341.5,105.7 377.6,95.9 413.8,82.2 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.9 µs (0.92×) | 28.1 µs |
| D38 | 117 µs (3.9×) | 25.8 µs (0.86×) | 29.9 µs |
| D57 | · | 29 µs (2.2×) | 13.4 µs |
| D76 | 62.6 µs (3.7×) | 33.1 µs (2×) | 16.8 µs |
| D115 | · | 103 µs (2.3×) | 44.3 µs |
| D153 | · | 99.8 µs (1.9×) | 51.5 µs |
| D230 | · | 181 µs (2.3×) | 77.3 µs |
| D307 | 250 µs (2.7×) | 232 µs (2.5×) | 91.4 µs |
| D462 | · | 313 µs (1.9×) | 164 µs |
| D616 | · | 590 µs (2.4×) | 249 µs |
| D924 | · | 1.34 ms (2.9×) | 465 µs |
| D1232 | · | 4.35 ms (4.5×) | 967 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,160.0 160.5,178.4 305.3,137.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.1 88.2,204.2 124.4,200.9 160.5,197.0 196.7,163.7 232.9,164.7 269.1,147.3 305.3,140.0 341.5,131.3 377.6,112.8 413.8,88.9 450.0,54.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,201.8 88.2,199.9 124.4,223.3 160.5,216.9 196.7,188.5 232.9,184.1 269.1,172.2 305.3,167.3 341.5,150.3 377.6,138.0 413.8,119.7 450.0,98.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 30.8 µs (1×) | 31 µs |
| D38 | 114 µs (3.4×) | 32.9 µs (0.99×) | 33.1 µs |
| D57 | · | 20.8 µs (2.2×) | 9.4 µs |
| D76 | 37.1 µs (3.5×) | 23 µs (2.1×) | 10.7 µs |
| D115 | · | 74 µs (2.4×) | 30.6 µs |
| D153 | 71.1 µs (2×) | 73.1 µs (2×) | 36 µs |
| D230 | 114 µs (2×) | 132 µs (2.3×) | 57.5 µs |
| D307 | 151 µs (2.2×) | 172 µs (2.6×) | 67.4 µs |
| D462 | · | 243 µs (1.9×) | 129 µs |
| D616 | · | 440 µs (2.2×) | 204 µs |
| D924 | · | 1.01 ms (2.6×) | 385 µs |
| D1232 | · | 1.93 ms (2.4×) | 797 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,128.2 160.5,152.8 232.9,138.5 269.1,128.1 305.3,121.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,156.8 88.2,155.4 124.4,165.4 160.5,163.2 196.7,137.6 232.9,137.9 269.1,124.8 305.3,119.1 341.5,111.5 377.6,98.5 413.8,80.3 450.0,66.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,156.7 88.2,155.2 124.4,182.8 160.5,180.0 196.7,156.9 232.9,153.4 269.1,143.1 305.3,139.7 341.5,125.4 377.6,115.4 413.8,101.4 450.0,85.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.45×) | 57.3 µs |
| D38 | 180 µs (2.9×) | 27.3 µs (0.44×) | 61.6 µs |
| D57 | · | 29.2 µs (2.3×) | 12.5 µs |
| D76 | 58.2 µs (3.8×) | 32.1 µs (2.1×) | 15.3 µs |
| D115 | · | 102 µs (2.4×) | 41.7 µs |
| D153 | 118 µs (2.9×) | 86.1 µs (2.1×) | 41.2 µs |
| D230 | 190 µs (2.5×) | 180 µs (2.4×) | 75.2 µs |
| D307 | 227 µs (2.6×) | 229 µs (2.7×) | 86.6 µs |
| D462 | · | 306 µs (2.1×) | 144 µs |
| D616 | · | 576 µs (2.3×) | 247 µs |
| D924 | · | 1.37 ms (2.9×) | 474 µs |
| D1232 | · | 4.43 ms (4.4×) | 996 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,147.5 160.5,180.5 232.9,159.9 269.1,146.0 305.3,140.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.2 88.2,202.6 124.4,200.7 160.5,197.9 196.7,164.2 232.9,169.1 269.1,147.4 305.3,140.4 341.5,132.0 377.6,113.5 413.8,88.1 450.0,53.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,181.0 88.2,178.8 124.4,225.5 160.5,219.6 196.7,190.2 232.9,190.6 269.1,173.0 305.3,168.9 341.5,154.0 377.6,138.2 413.8,119.1 450.0,97.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.5 µs (0.79×) | 32.1 µs |
| D38 | 146 µs (4.2×) | 26.4 µs (0.77×) | 34.4 µs |
| D57 | · | 29.6 µs (2.3×) | 12.8 µs |
| D76 | 61.7 µs (3.8×) | 34.2 µs (2.1×) | 16.1 µs |
| D115 | · | 107 µs (2.5×) | 43.1 µs |
| D153 | 170 µs (3.3×) | 104 µs (2×) | 51 µs |
| D230 | 295 µs (3.8×) | 186 µs (2.4×) | 78.4 µs |
| D307 | 388 µs (4.1×) | 243 µs (2.6×) | 93.6 µs |
| D462 | · | 338 µs (2×) | 172 µs |
| D616 | · | 612 µs (2.3×) | 266 µs |
| D924 | · | 1.38 ms (2.7×) | 507 µs |
| D1232 | · | 5.21 ms (5×) | 1.04 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.7 160.5,178.8 232.9,149.2 269.1,133.0 305.3,125.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,204.7 88.2,203.6 124.4,200.3 160.5,196.0 196.7,162.7 232.9,163.5 269.1,146.6 305.3,138.7 341.5,129.1 377.6,111.7 413.8,88.0 450.0,49.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,197.9 88.2,195.8 124.4,224.8 160.5,218.1 196.7,189.3 232.9,184.4 269.1,171.8 305.3,166.6 341.5,148.9 377.6,136.1 413.8,117.2 450.0,96.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 10.1 µs (0.98×) | 10.3 µs |
| D38 | 132 µs (12×) | 10.8 µs (0.98×) | 11.1 µs |
| D57 | · | 106 µs (2.1×) | 50.9 µs |
| D76 | 41.4 µs (0.72×) | 114 µs (2×) | 57.7 µs |
| D115 | · | 419 µs (2.3×) | 179 µs |
| D153 | · | 444 µs (1.6×) | 270 µs |
| D230 | · | 815 µs (1.6×) | 525 µs |
| D307 | 279 µs (0.54×) | 1.13 ms (2.2×) | 519 µs |
| D462 | · | 1.65 ms (1.5×) | 1.13 ms |
| D616 | · | 5.94 ms (2×) | 2.93 ms |
| D924 | · | 12.4 ms (1.8×) | 7.01 ms |
| D1232 | · | 24.6 ms (1.5×) | 16 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,175.4 160.5,200.8 305.3,159.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,231.8 88.2,230.2 124.4,180.2 160.5,178.6 196.7,150.1 232.9,148.8 269.1,135.5 305.3,128.3 341.5,120.1 377.6,91.9 413.8,75.8 450.0,60.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,231.4 88.2,229.7 124.4,196.3 160.5,193.6 196.7,168.7 232.9,159.8 269.1,145.1 305.3,145.4 341.5,128.3 377.6,107.4 413.8,88.3 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 15.3 µs (2.8×) | 5.45 µs |
| D38 | 14.6 µs (1.8×) | 15.3 µs (1.9×) | 8.09 µs |
| D57 | · | 3.98 µs (3.1×) | 1.29 µs |
| D76 | 6.15 µs (3×) | 5.4 µs (2.6×) | 2.04 µs |
| D115 | · | 14.1 µs (2.4×) | 5.77 µs |
| D153 | 19.9 µs (2.3×) | 18.6 µs (2.1×) | 8.7 µs |
| D230 | 32.1 µs (1.6×) | 35.7 µs (1.8×) | 19.5 µs |
| D307 | 42 µs (1.5×) | 45.1 µs (1.7×) | 27.2 µs |
| D462 | · | 87.1 µs (1.6×) | 54.2 µs |
| D616 | · | 140 µs (1.6×) | 87.1 µs |
| D924 | · | 265 µs (1.5×) | 178 µs |
| D1232 | · | 493 µs (1.5×) | 335 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,153.5 160.5,178.9 232.9,144.6 269.1,130.6 305.3,122.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,152.3 88.2,152.2 124.4,191.6 160.5,182.7 196.7,154.6 232.9,146.5 269.1,127.4 305.3,120.6 341.5,101.4 377.6,87.5 413.8,68.8 450.0,50.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.4 88.2,170.9 124.4,224.5 160.5,211.1 196.7,180.7 232.9,168.7 269.1,145.1 305.3,135.4 341.5,115.2 377.6,101.4 413.8,80.4 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 5.96 µs (1×) | 5.98 µs |
| D38 | 77.2 µs (12×) | 6.34 µs (0.99×) | 6.38 µs |
| D57 | · | 19.1 µs (2.9×) | 6.63 µs |
| D76 | 29.3 µs (3.9×) | 21.6 µs (2.8×) | 7.59 µs |
| D115 | · | 70 µs (3.1×) | 22.5 µs |
| D153 | 26.6 µs (1.2×) | 61.2 µs (2.7×) | 22.7 µs |
| D230 | 31.7 µs (0.71×) | 130 µs (2.9×) | 44.7 µs |
| D307 | 40.2 µs (0.78×) | 177 µs (3.4×) | 51.8 µs |
| D462 | · | 245 µs (2.6×) | 93.5 µs |
| D616 | · | 446 µs (2.5×) | 177 µs |
| D924 | · | 1.05 ms (2.9×) | 356 µs |
| D1232 | · | 2.21 ms (2.9×) | 754 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,136.7 160.5,157.9 232.9,160.0 269.1,156.2 305.3,151.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.9 88.2,191.5 124.4,167.3 160.5,164.6 196.7,138.8 232.9,141.8 269.1,125.2 305.3,118.5 341.5,111.3 377.6,98.2 413.8,79.5 450.0,63.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.8 88.2,191.3 124.4,190.5 160.5,187.6 196.7,163.7 232.9,163.5 269.1,148.7 305.3,145.4 341.5,132.5 377.6,118.5 413.8,103.1 450.0,86.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.71 µs (0.99×) | 9.8 µs |
| D38 | 223 µs (21×) | 10.3 µs (0.99×) | 10.5 µs |
| D57 | · | 241 µs (27×) | 9.07 µs |
| D76 | 56.2 µs (5.1×) | 270 µs (24×) | 11.1 µs |
| D115 | · | 1.17 ms (42×) | 27.9 µs |
| D153 | 100 µs (2.8×) | 1.68 ms (47×) | 35.6 µs |
| D230 | 150 µs (2.8×) | 5.85 ms (1.1e+02×) | 54 µs |
| D307 | 205 µs (3.3×) | 7.02 ms (1.1e+02×) | 62.7 µs |
| D462 | · | 12.3 ms (1e+02×) | 121 µs |
| D616 | · | 35.6 ms (2e+02×) | 182 µs |
| D924 | · | 58.7 ms (1.7e+02×) | 340 µs |
| D1232 | · | 88.4 ms (1.3e+02×) | 700 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.3 232.9,151.2 269.1,144.1 305.3,138.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.8 160.5,133.8 196.7,108.1 232.9,101.7 269.1,79.8 305.3,76.6 341.5,66.8 377.6,48.1 413.8,39.3 450.0,32.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.0 88.2,190.8 124.4,193.3 160.5,189.8 196.7,173.6 232.9,169.3 269.1,162.0 305.3,159.4 341.5,147.8 377.6,140.7 413.8,129.7 450.0,117.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 200 ns (0.9×) | 191 ns (0.86×) | 221 ns |
| D38 | 296 ns (0.79×) | 255 ns (0.68×) | 376 ns |
| D57 | · | 2.57 µs (3.3×) | 791 ns |
| D76 | · | 3.19 µs (3.2×) | 1 µs |
| D115 | · | 6.92 µs (2.5×) | 2.77 µs |
| D153 | · | 10.5 µs (2.6×) | 4.05 µs |
| D230 | · | 20 µs (2.4×) | 8.49 µs |
| D307 | 14.3 µs (1.1×) | 22.3 µs (1.8×) | 12.6 µs |
| D462 | · | 45.5 µs (1.6×) | 29 µs |
| D616 | · | 71.8 µs (1.6×) | 45.7 µs |
| D924 | · | 134 µs (1.4×) | 92.8 µs |
| D1232 | · | 241 µs (1.4×) | 172 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,216.8 88.2,208.2 305.3,123.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,217.8 88.2,211.5 124.4,160.8 160.5,156.0 196.7,139.1 232.9,129.9 269.1,115.8 305.3,113.4 341.5,97.8 377.6,87.8 413.8,74.1 450.0,61.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,214.6 88.2,203.0 124.4,186.6 160.5,181.5 196.7,159.2 232.9,150.8 269.1,134.6 305.3,126.0 341.5,107.6 377.6,97.7 413.8,82.1 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.05 µs (0.98×) | 6.16 µs |
| D38 | 63.9 µs (9.6×) | 6.58 µs (0.98×) | 6.69 µs |
| D57 | · | 22.5 µs (4.1×) | 5.47 µs |
| D76 | 29.1 µs (3.6×) | 24.1 µs (3×) | 8.15 µs |
| D115 | · | 63.8 µs (2.3×) | 28 µs |
| D153 | 55.4 µs (1.6×) | 67.2 µs (1.9×) | 35 µs |
| D230 | 88.2 µs (1.7×) | 130 µs (2.5×) | 52.1 µs |
| D307 | 117 µs (1.9×) | 171 µs (2.8×) | 61.4 µs |
| D462 | · | 231 µs (1.7×) | 135 µs |
| D616 | · | 434 µs (2.2×) | 200 µs |
| D924 | · | 889 µs (2.4×) | 371 µs |
| D1232 | · | 1.71 ms (2.1×) | 812 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,140.8 160.5,158.1 232.9,143.9 269.1,133.8 305.3,127.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.5 88.2,190.7 124.4,163.7 160.5,162.2 196.7,140.9 232.9,139.7 269.1,125.2 305.3,119.3 341.5,112.6 377.6,98.8 413.8,83.1 450.0,68.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,190.3 124.4,194.7 160.5,186.0 196.7,158.9 232.9,154.0 269.1,145.3 305.3,141.7 341.5,124.5 377.6,115.9 413.8,102.3 450.0,85.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.19 µs (0.99×) | 6.24 µs |
| D38 | 64.7 µs (9.7×) | 6.59 µs (0.99×) | 6.67 µs |
| D57 | · | 238 µs (17×) | 14.1 µs |
| D76 | 30.4 µs (2×) | 240 µs (16×) | 15.5 µs |
| D115 | · | 1.15 ms (19×) | 60.3 µs |
| D153 | 59.8 µs (0.84×) | 1.64 ms (23×) | 70.9 µs |
| D230 | 92 µs (0.6×) | 5.55 ms (36×) | 153 µs |
| D307 | 131 µs (0.61×) | 6.99 ms (33×) | 214 µs |
| D462 | · | 12.3 ms (38×) | 324 µs |
| D616 | · | 34.6 ms (32×) | 1.09 ms |
| D924 | · | 59.6 ms (42×) | 1.4 ms |
| D1232 | · | 90.7 ms (11×) | 8.42 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,158.8 160.5,172.1 232.9,160.2 269.1,152.7 305.3,146.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,200.0 88.2,198.9 124.4,136.0 160.5,135.8 196.7,108.4 232.9,102.1 269.1,80.7 305.3,76.7 341.5,66.8 377.6,48.6 413.8,39.1 450.0,31.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,199.9 88.2,198.7 124.4,185.6 160.5,184.0 196.7,160.1 232.9,157.2 269.1,143.7 305.3,137.8 341.5,130.6 377.6,109.3 413.8,104.8 450.0,73.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 290 ns (1.1×) | 260 ns |
| D38 | 341 ns (0.92×) | 336 ns (0.91×) | 370 ns |
| D57 | · | 3.6 µs (4.3×) | 841 ns |
| D76 | 4.6 µs (3.3×) | 4.66 µs (3.3×) | 1.4 µs |
| D115 | · | 11.1 µs (2.9×) | 3.81 µs |
| D153 | 11.2 µs (2.2×) | 14.5 µs (2.8×) | 5.19 µs |
| D230 | 18.2 µs (1.8×) | 25.1 µs (2.4×) | 10.4 µs |
| D307 | 21.3 µs (1.5×) | 30.8 µs (2.1×) | 14.6 µs |
| D462 | · | 59.8 µs (2×) | 30.6 µs |
| D616 | · | 95.1 µs (1.9×) | 50.6 µs |
| D924 | · | 177 µs (1.8×) | 99.7 µs |
| D1232 | · | 307 µs (1.7×) | 184 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,205.1 160.5,148.0 232.9,128.6 269.1,117.9 305.3,114.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,208.6 88.2,205.4 124.4,153.4 160.5,147.8 196.7,128.7 232.9,122.9 269.1,110.8 305.3,106.4 341.5,91.8 377.6,81.6 413.8,68.0 450.0,55.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.0 88.2,203.3 124.4,185.3 160.5,174.1 196.7,152.2 232.9,145.4 269.1,130.2 305.3,122.7 341.5,106.5 377.6,95.5 413.8,80.6 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 8.01 µs (0.97×) | 8.29 µs |
| D38 | 167 µs (19×) | 8.65 µs (0.97×) | 8.93 µs |
| D57 | · | 22.9 µs (8.2×) | 2.81 µs |
| D76 | 40.3 µs (11×) | 27.1 µs (7.2×) | 3.75 µs |
| D115 | · | 36.6 µs (4.1×) | 8.97 µs |
| D153 | 123 µs (9.8×) | 40.8 µs (3.3×) | 12.5 µs |
| D230 | 209 µs (8.1×) | 77.1 µs (3×) | 25.7 µs |
| D307 | 261 µs (8.9×) | 199 µs (6.8×) | 29.5 µs |
| D462 | · | 267 µs (4.1×) | 64.8 µs |
| D616 | · | 478 µs (4.3×) | 112 µs |
| D924 | · | 1.07 ms (4.6×) | 234 µs |
| D1232 | · | 1.95 ms (3.8×) | 509 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.7 160.5,151.0 232.9,126.5 269.1,114.8 305.3,109.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,186.4 88.2,184.7 124.4,163.3 160.5,159.6 196.7,153.0 232.9,150.7 269.1,136.7 305.3,115.9 341.5,109.5 377.6,96.7 413.8,79.0 450.0,65.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,185.6 88.2,184.0 124.4,209.3 160.5,203.0 196.7,183.9 232.9,176.6 269.1,160.8 305.3,157.8 341.5,140.5 377.6,128.6 413.8,112.3 450.0,95.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 16.1 µs (0.99×) | 16.4 µs |
| D38 | 205 µs (12×) | 17.3 µs (0.98×) | 17.6 µs |
| D57 | · | 79.6 µs (5×) | 16.1 µs |
| D76 | 74 µs (4×) | 94.1 µs (5.1×) | 18.6 µs |
| D115 | · | 299 µs (4.6×) | 64.9 µs |
| D153 | · | 294 µs (3.7×) | 79.4 µs |
| D230 | 370 µs (2.5×) | 534 µs (3.6×) | 147 µs |
| D307 | 512 µs (2.4×) | 700 µs (3.3×) | 210 µs |
| D462 | · | 957 µs (2.7×) | 357 µs |
| D616 | · | 1.82 ms (2.5×) | 724 µs |
| D924 | · | 6.85 ms (4.2×) | 1.64 ms |
| D1232 | · | 12.7 ms (2.1×) | 5.95 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,165.7 160.5,188.1 269.1,152.8 305.3,145.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,221.5 88.2,220.0 124.4,186.5 160.5,182.8 196.7,157.5 232.9,157.8 269.1,144.7 305.3,138.8 341.5,132.0 377.6,117.8 413.8,88.8 450.0,75.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,221.2 88.2,219.6 124.4,221.6 160.5,218.4 196.7,191.0 232.9,186.6 269.1,173.1 305.3,165.3 341.5,153.6 377.6,138.1 413.8,120.1 450.0,91.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.98×) | 12.1 µs |
| D38 | 172 µs (13×) | 12.8 µs (0.98×) | 13 µs |
| D57 | · | 44.5 µs (4.9×) | 9.1 µs |
| D76 | 40.5 µs (3.9×) | 51.5 µs (5×) | 10.4 µs |
| D115 | · | 160 µs (4.4×) | 36 µs |
| D153 | 124 µs (2.9×) | 162 µs (3.8×) | 43 µs |
| D230 | 235 µs (3×) | 288 µs (3.7×) | 78.7 µs |
| D307 | 264 µs (2.3×) | 381 µs (3.4×) | 112 µs |
| D462 | · | 499 µs (2.7×) | 187 µs |
| D616 | · | 944 µs (2.5×) | 378 µs |
| D924 | · | 2.14 ms (2.6×) | 828 µs |
| D1232 | · | 6.6 ms (3.9×) | 1.69 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.1 160.5,150.8 232.9,126.3 269.1,112.3 305.3,109.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.8 88.2,176.1 124.4,148.7 160.5,145.5 196.7,120.7 232.9,120.5 269.1,107.8 305.3,101.7 341.5,95.8 377.6,81.8 413.8,63.8 450.0,39.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.4 88.2,175.7 124.4,183.6 160.5,180.7 196.7,153.4 232.9,149.5 269.1,136.2 305.3,128.5 341.5,117.3 377.6,101.8 413.8,84.6 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.98×) | 12.1 µs |
| D38 | 171 µs (13×) | 12.8 µs (0.98×) | 13 µs |
| D57 | · | 44.7 µs (4.9×) | 9.16 µs |
| D76 | 40.8 µs (3.9×) | 52.3 µs (5×) | 10.4 µs |
| D115 | · | 161 µs (4.5×) | 36.1 µs |
| D153 | 125 µs (2.9×) | 164 µs (3.8×) | 43.2 µs |
| D230 | 203 µs (2.6×) | 289 µs (3.7×) | 78.7 µs |
| D307 | 261 µs (2.3×) | 385 µs (3.4×) | 113 µs |
| D462 | · | 498 µs (2.7×) | 188 µs |
| D616 | · | 942 µs (2.5×) | 380 µs |
| D924 | · | 4.17 ms (5×) | 831 µs |
| D1232 | · | 6.59 ms (3.9×) | 1.67 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.2 160.5,150.6 232.9,126.2 269.1,115.5 305.3,110.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,177.7 88.2,176.1 124.4,148.6 160.5,145.2 196.7,120.5 232.9,120.2 269.1,107.7 305.3,101.4 341.5,95.8 377.6,81.8 413.8,49.2 450.0,39.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,177.4 88.2,175.7 124.4,183.4 160.5,180.7 196.7,153.4 232.9,149.4 269.1,136.2 305.3,128.4 341.5,117.2 377.6,101.7 413.8,84.6 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 240 ns (0.92×) | 231 ns (0.89×) | 260 ns |
| D38 | 291 ns (0.69×) | 270 ns (0.64×) | 421 ns |
| D57 | · | 2.06 µs (2.9×) | 711 ns |
| D76 | · | 2.57 µs (2.8×) | 936 ns |
| D115 | · | 5.57 µs (2.2×) | 2.56 µs |
| D153 | · | 8.87 µs (2.2×) | 3.97 µs |
| D230 | · | 16 µs (1.9×) | 8.65 µs |
| D307 | · | 19.5 µs (1.5×) | 12.7 µs |
| D462 | · | 41.7 µs (1.4×) | 28.8 µs |
| D616 | · | 68.7 µs (1.5×) | 44.7 µs |
| D924 | · | 128 µs (1.4×) | 92.5 µs |
| D1232 | · | 232 µs (1.4×) | 167 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,212.8 88.2,208.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,213.6 88.2,210.2 124.4,165.6 160.5,160.8 196.7,143.8 232.9,133.6 269.1,120.7 305.3,116.4 341.5,99.7 377.6,88.7 413.8,75.1 450.0,62.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.0 88.2,200.5 124.4,189.0 160.5,183.0 196.7,160.9 232.9,151.3 269.1,134.2 305.3,125.7 341.5,107.8 377.6,98.2 413.8,82.2 450.0,69.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 260 ns (0.81×) | 321 ns |
| D38 | 211 µs (5e+02×) | 331 ns (0.79×) | 421 ns |
| D57 | · | 62.7 µs (4.1×) | 15.3 µs |
| D76 | 65.9 µs (3.8×) | 72.3 µs (4.2×) | 17.1 µs |
| D115 | · | 233 µs (3.7×) | 63.3 µs |
| D153 | 167 µs (2.2×) | 227 µs (3×) | 76.8 µs |
| D230 | 272 µs (1.8×) | 405 µs (2.7×) | 148 µs |
| D307 | 365 µs (1.8×) | 537 µs (2.6×) | 205 µs |
| D462 | · | 712 µs (2.1×) | 344 µs |
| D616 | · | 1.33 ms (1.7×) | 776 µs |
| D924 | · | 5.71 ms (3.2×) | 1.81 ms |
| D1232 | · | 10.3 ms (1.7×) | 5.92 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="63.7" x2="450" y2="63.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="66.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,120.1 160.5,137.1 232.9,123.5 269.1,116.4 305.3,112.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,218.0 88.2,214.5 124.4,137.8 160.5,135.7 196.7,118.6 232.9,119.0 269.1,110.6 305.3,106.4 341.5,102.3 377.6,93.1 413.8,71.9 450.0,63.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,214.9 88.2,211.0 124.4,158.4 160.5,156.8 196.7,137.7 232.9,134.9 269.1,125.3 305.3,120.5 341.5,112.9 377.6,101.0 413.8,88.7 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 266 ns (1.1×) | 200 ns (0.83×) | 240 ns |
| D38 | 250 ns (0.83×) | 230 ns (0.77×) | 300 ns |
| D57 | · | 2.1 µs (3×) | 711 ns |
| D76 | · | 2.6 µs (2.8×) | 926 ns |
| D115 | · | 5.37 µs (2.2×) | 2.49 µs |
| D153 | · | 8.12 µs (2.1×) | 3.8 µs |
| D230 | · | 15.2 µs (1.8×) | 8.2 µs |
| D307 | · | 18.2 µs (1.5×) | 11.8 µs |
| D462 | · | 37.7 µs (1.4×) | 27.8 µs |
| D616 | · | 61.9 µs (1.4×) | 43.9 µs |
| D924 | · | 115 µs (1.2×) | 92.9 µs |
| D1232 | · | 201 µs (1.2×) | 167 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,210.5 88.2,211.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,216.8 88.2,213.7 124.4,165.2 160.5,160.5 196.7,144.6 232.9,135.6 269.1,121.9 305.3,117.8 341.5,101.9 377.6,91.0 413.8,77.4 450.0,65.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,212.8 88.2,207.9 124.4,189.0 160.5,183.2 196.7,161.5 232.9,152.2 269.1,135.3 305.3,127.3 341.5,108.6 377.6,98.6 413.8,82.1 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.16 µs (1×) | 6.15 µs |
| D38 | 83 µs (13×) | 6.52 µs (0.99×) | 6.57 µs |
| D57 | · | 18.8 µs (2.9×) | 6.44 µs |
| D76 | 28.9 µs (3.9×) | 21.2 µs (2.8×) | 7.49 µs |
| D115 | · | 72 µs (3.2×) | 22.7 µs |
| D153 | 21.2 µs (0.95×) | 59.4 µs (2.7×) | 22.3 µs |
| D230 | 29.3 µs (0.67×) | 130 µs (3×) | 43.6 µs |
| D307 | 36.5 µs (0.71×) | 170 µs (3.3×) | 51.4 µs |
| D462 | · | 241 µs (2.6×) | 93.7 µs |
| D616 | · | 451 µs (2.6×) | 176 µs |
| D924 | · | 1.04 ms (2.9×) | 357 µs |
| D1232 | · | 2.21 ms (2.9×) | 755 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,135.1 160.5,158.2 232.9,165.0 269.1,157.9 305.3,153.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,190.9 124.4,167.6 160.5,165.0 196.7,138.2 232.9,142.4 269.1,125.2 305.3,119.3 341.5,111.7 377.6,98.0 413.8,79.7 450.0,63.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.2 88.2,190.7 124.4,191.2 160.5,187.8 196.7,163.5 232.9,163.9 269.1,149.2 305.3,145.6 341.5,132.4 377.6,118.7 413.8,103.1 450.0,86.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.73 µs (0.99×) | 9.82 µs |
| D38 | 223 µs (21×) | 10.3 µs (0.99×) | 10.5 µs |
| D57 | · | 249 µs (27×) | 9.09 µs |
| D76 | 55.6 µs (5.1×) | 269 µs (25×) | 11 µs |
| D115 | · | 1.17 ms (42×) | 28 µs |
| D153 | 101 µs (2.8×) | 1.63 ms (45×) | 36 µs |
| D230 | 157 µs (2.9×) | 5.85 ms (1.1e+02×) | 54 µs |
| D307 | 213 µs (3.5×) | 7.02 ms (1.1e+02×) | 61.4 µs |
| D462 | · | 12.3 ms (1e+02×) | 120 µs |
| D616 | · | 35.6 ms (2e+02×) | 181 µs |
| D924 | · | 58.6 ms (1.7e+02×) | 339 µs |
| D1232 | · | 90.2 ms (1.3e+02×) | 689 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,137.1 160.5,161.5 232.9,151.1 269.1,143.3 305.3,137.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,191.0 124.4,135.2 160.5,133.8 196.7,108.1 232.9,102.2 269.1,79.8 305.3,76.6 341.5,66.8 377.6,48.1 413.8,39.4 450.0,31.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,190.8 124.4,193.3 160.5,190.0 196.7,173.5 232.9,169.1 269.1,162.0 305.3,159.8 341.5,148.0 377.6,140.7 413.8,129.8 450.0,117.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 211 ns (0.81×) | 261 ns |
| D38 | 241 ns (0.78×) | 251 ns (0.81×) | 310 ns |
| D57 | · | 2.63 µs (2.8×) | 941 ns |
| D76 | 3.71 µs (2.9×) | 3.4 µs (2.7×) | 1.28 µs |
| D115 | · | 8.61 µs (2.5×) | 3.39 µs |
| D153 | 8.84 µs (1.7×) | 10.6 µs (2.1×) | 5.14 µs |
| D230 | 14.7 µs (1.5×) | 19.6 µs (2×) | 9.61 µs |
| D307 | 19.1 µs (1.4×) | 24.3 µs (1.8×) | 13.5 µs |
| D462 | · | 46.4 µs (1.8×) | 26.3 µs |
| D616 | · | 76.2 µs (1.8×) | 42.4 µs |
| D924 | · | 138 µs (1.7×) | 79.9 µs |
| D1232 | · | 250 µs (1.9×) | 135 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,212.7 160.5,152.8 232.9,133.7 269.1,122.5 305.3,116.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,215.6 88.2,211.8 124.4,160.3 160.5,154.6 196.7,134.3 232.9,129.6 269.1,116.2 305.3,111.5 341.5,97.3 377.6,86.5 413.8,73.4 450.0,60.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,211.0 88.2,207.2 124.4,182.8 160.5,176.1 196.7,154.8 232.9,145.6 269.1,131.9 305.3,124.5 341.5,109.8 377.6,99.3 413.8,85.4 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 196 ns (0.85×) | 201 ns (0.87×) | 230 ns |
| D38 | 220 ns (0.69×) | 236 ns (0.74×) | 320 ns |
| D57 | · | 2.23 µs (2.9×) | 781 ns |
| D76 | 1 µs (1×) | 2.74 µs (2.7×) | 1 µs |
| D115 | · | 6.14 µs (2.1×) | 2.9 µs |
| D153 | 1.82 µs (0.4×) | 9.73 µs (2.2×) | 4.51 µs |
| D230 | 3.21 µs (0.33×) | 17.4 µs (1.8×) | 9.85 µs |
| D307 | 3.85 µs (0.27×) | 21.7 µs (1.5×) | 14.3 µs |
| D462 | · | 44.9 µs (1.4×) | 31.1 µs |
| D616 | · | 70.4 µs (1.5×) | 48.3 µs |
| D924 | · | 125 µs (1.4×) | 92.2 µs |
| D1232 | · | 227 µs (1.3×) | 176 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,217.2 88.2,214.7 160.5,181.5 232.9,168.3 269.1,155.9 305.3,151.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,216.7 88.2,213.2 124.4,163.9 160.5,159.4 196.7,141.7 232.9,131.6 269.1,118.9 305.3,114.0 341.5,98.1 377.6,88.2 413.8,75.6 450.0,62.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,213.7 88.2,206.5 124.4,186.9 160.5,181.5 196.7,158.2 232.9,148.5 269.1,131.3 305.3,123.2 341.5,106.1 377.6,96.5 413.8,82.3 450.0,68.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.1 µs (0.99×) | 11.3 µs |
| D38 | 162 µs (13×) | 11.9 µs (0.99×) | 12.1 µs |
| D57 | · | 22.7 µs (2.7×) | 8.34 µs |
| D76 | 36.9 µs (3.9×) | 24.7 µs (2.6×) | 9.44 µs |
| D115 | · | 78.7 µs (2.9×) | 27.4 µs |
| D153 | 40 µs (1.5×) | 69.6 µs (2.5×) | 27.4 µs |
| D230 | · | 148 µs (2.9×) | 51.5 µs |
| D307 | 67.4 µs (1.1×) | 195 µs (3.2×) | 60.6 µs |
| D462 | · | 260 µs (2.4×) | 108 µs |
| D616 | · | 496 µs (2.5×) | 197 µs |
| D924 | · | 1.11 ms (2.8×) | 393 µs |
| D1232 | · | 2.33 ms (2.8×) | 823 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,120.4 160.5,152.9 232.9,151.1 305.3,139.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.1 88.2,177.7 124.4,163.5 160.5,161.7 196.7,136.2 232.9,138.9 269.1,122.3 305.3,116.3 341.5,110.1 377.6,95.9 413.8,78.2 450.0,62.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,178.9 88.2,177.4 124.4,185.5 160.5,182.8 196.7,159.4 232.9,159.4 269.1,145.6 305.3,142.0 341.5,129.3 377.6,116.1 413.8,101.0 450.0,84.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
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
| D76 | 55.8 µs (4.9×) | 273 µs (24×) | 11.5 µs |
| D115 | · | 1.38 ms (43×) | 32 µs |
| D153 | 102 µs (2.5×) | 1.72 ms (42×) | 41.3 µs |
| D230 | 159 µs (2.7×) | 6.17 ms (1.1e+02×) | 58.3 µs |
| D307 | 217 µs (3.1×) | 7.78 ms (1.1e+02×) | 70.1 µs |
| D462 | · | 12.3 ms (91×) | 136 µs |
| D616 | · | 35.7 ms (1.8e+02×) | 203 µs |
| D924 | · | 59.5 ms (1.5e+02×) | 393 µs |
| D1232 | · | 90 ms (1.1e+02×) | 855 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="191.6" x2="450" y2="191.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="194.6" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="151.2" x2="450" y2="151.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="154.2" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="110.8" x2="450" y2="110.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.8" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="70.4" x2="450" y2="70.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="73.4" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,139.6 160.5,161.4 232.9,150.9 269.1,143.0 305.3,137.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,187.2 88.2,186.1 124.4,134.9 160.5,133.6 196.7,105.1 232.9,101.3 269.1,78.9 305.3,74.8 341.5,66.8 377.6,48.1 413.8,39.1 450.0,31.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,192.0 88.2,190.9 124.4,191.6 160.5,189.2 196.7,171.2 232.9,166.7 269.1,160.7 305.3,157.4 341.5,145.9 377.6,138.8 413.8,127.2 450.0,113.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body -->

For the full list of changes, see the [Changelog](CHANGELOG.md).
