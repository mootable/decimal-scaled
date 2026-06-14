# History — Roots and Exponents

How the root and exponential functions have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 14.6 µs (2.7×) | 5.38 µs |
| D38 | 17.1 µs (1.7×) | 17.1 µs (1.7×) | 10.1 µs |
| D57 | · | 4.11 µs (3.1×) | 1.34 µs |
| D76 | 6.12 µs (3.3×) | 5.13 µs (2.8×) | 1.86 µs |
| D115 | · | 9.96 µs (2.4×) | 4.14 µs |
| D153 | 11.5 µs (2.2×) | 10.7 µs (2×) | 5.27 µs |
| D230 | 19.7 µs (2.6×) | 18.4 µs (2.5×) | 7.46 µs |
| D307 | 24.5 µs (2.5×) | 23.6 µs (2.4×) | 9.66 µs |
| D462 | · | 32.8 µs (2.2×) | 15 µs |
| D616 | · | 43.5 µs (2.3×) | 18.8 µs |
| D924 | · | 64.1 µs (2×) | 32.7 µs |
| D1232 | · | 81.8 µs (1.6×) | 51 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,107.4 160.5,152.5 232.9,124.7 269.1,101.2 305.3,91.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,114.4 88.2,107.4 124.4,170.0 160.5,160.3 196.7,131.2 232.9,128.2 269.1,104.1 305.3,93.3 341.5,78.9 377.6,66.5 413.8,49.5 450.0,38.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,158.2 88.2,130.5 124.4,219.1 160.5,204.7 196.7,169.7 232.9,159.1 269.1,143.8 305.3,132.5 341.5,113.3 377.6,103.3 413.8,79.0 450.0,59.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.08 µs (0.99×) | 6.13 µs |
| D38 | 172 µs (19×) | 9.06 µs (0.98×) | 9.26 µs |
| D57 | · | 21.9 µs (3.9×) | 5.66 µs |
| D76 | 34.4 µs (4.8×) | 28.1 µs (3.9×) | 7.24 µs |
| D115 | · | 53.8 µs (3.7×) | 14.6 µs |
| D153 | 50.5 µs (3.6×) | 44.5 µs (3.2×) | 14 µs |
| D230 | 80.9 µs (4.5×) | 69.6 µs (3.9×) | 17.9 µs |
| D307 | 114 µs (5.7×) | 91.8 µs (4.6×) | 19.9 µs |
| D462 | · | 94.9 µs (3.8×) | 24.9 µs |
| D616 | · | 120 µs (4.2×) | 28.4 µs |
| D924 | · | 184 µs (4.6×) | 40.4 µs |
| D1232 | · | 234 µs (4×) | 58.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,81.5 160.5,128.5 232.9,117.3 269.1,103.5 305.3,93.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.2 88.2,167.6 124.4,141.8 160.5,134.5 196.7,115.4 232.9,121.0 269.1,107.9 305.3,99.8 341.5,98.9 377.6,91.9 413.8,79.5 450.0,72.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,179.0 88.2,166.9 124.4,181.3 160.5,174.1 196.7,153.6 232.9,154.8 269.1,147.7 305.3,144.5 341.5,138.0 377.6,134.1 413.8,123.9 450.0,113.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `exp2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.16 µs (0.99×) | 6.21 µs |
| D38 | 168 µs (18×) | 9.16 µs (0.98×) | 9.34 µs |
| D57 | · | 231 µs (17×) | 13.7 µs |
| D76 | 35.7 µs (2.5×) | 247 µs (17×) | 14.1 µs |
| D115 | · | 794 µs (18×) | 45.3 µs |
| D153 | 51.8 µs (1.3×) | 580 µs (14×) | 40.4 µs |
| D230 | 84 µs (1.1×) | 980 µs (13×) | 76.9 µs |
| D307 | 118 µs (1.1×) | 1.36 ms (12×) | 109 µs |
| D462 | · | 1.3 ms (11×) | 115 µs |
| D616 | · | 5.51 ms (17×) | 332 µs |
| D924 | · | 5.43 ms (15×) | 373 µs |
| D1232 | · | 5.24 ms (4.3×) | 1.22 ms |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,119.6 160.5,153.6 232.9,145.4 269.1,134.8 305.3,127.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,192.1 88.2,183.4 124.4,112.6 160.5,111.1 196.7,85.6 232.9,92.5 269.1,80.9 305.3,73.8 341.5,74.8 377.6,43.1 413.8,43.4 450.0,44.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,191.9 88.2,183.0 124.4,174.6 160.5,173.9 196.7,148.4 232.9,150.9 269.1,136.8 305.3,129.1 341.5,127.8 377.6,104.7 413.8,102.1 450.0,76.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 290 ns (1.1×) | 266 ns |
| D38 | 6 µs (11×) | 6.03 µs (11×) | 561 ns |
| D57 | · | 3.69 µs (4.3×) | 862 ns |
| D76 | 5.93 µs (4.3×) | 4.97 µs (3.6×) | 1.39 µs |
| D115 | · | 7.52 µs (3×) | 2.53 µs |
| D153 | 8.16 µs (2.5×) | 8.92 µs (2.7×) | 3.31 µs |
| D230 | 11.7 µs (1.6×) | 15.2 µs (2.1×) | 7.14 µs |
| D307 | 15 µs (1.6×) | 19.4 µs (2.1×) | 9.45 µs |
| D462 | · | 26.9 µs (1.5×) | 18.4 µs |
| D616 | · | 33.1 µs (1.3×) | 26 µs |
| D924 | · | 47.5 µs (0.94×) | 50.8 µs |
| D1232 | · | 62.5 µs (0.78×) | 80.2 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,112.3 160.5,112.6 232.9,103.3 269.1,92.7 305.3,85.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,200.9 88.2,112.1 124.4,126.5 160.5,117.7 196.7,105.7 232.9,100.7 269.1,85.1 305.3,77.9 341.5,68.5 377.6,62.4 413.8,51.7 450.0,43.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,203.4 88.2,181.6 124.4,169.0 160.5,155.1 196.7,137.5 232.9,129.6 269.1,107.2 305.3,99.0 341.5,79.4 377.6,69.3 413.8,49.8 450.0,36.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 8 µs (0.96×) | 8.31 µs |
| D38 | 262 µs (23×) | 11.1 µs (0.96×) | 11.6 µs |
| D57 | · | 23.6 µs (8×) | 2.96 µs |
| D76 | 49.4 µs (15×) | 32.2 µs (9.5×) | 3.4 µs |
| D115 | · | 51.7 µs (10×) | 5.1 µs |
| D153 | 65.8 µs (12×) | 46.6 µs (8.2×) | 5.69 µs |
| D230 | 104 µs (12×) | 70 µs (8.2×) | 8.52 µs |
| D307 | 141 µs (13×) | 96.4 µs (8.6×) | 11.2 µs |
| D462 | · | 96.5 µs (4.6×) | 20.8 µs |
| D616 | · | 114 µs (4.8×) | 23.8 µs |
| D924 | · | 174 µs (4.6×) | 37.7 µs |
| D1232 | · | 217 µs (3.5×) | 61.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,69.2 160.5,118.0 232.9,109.6 269.1,96.2 305.3,87.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,171.2 88.2,161.5 124.4,139.6 160.5,130.5 196.7,116.6 232.9,119.7 269.1,107.7 305.3,98.4 341.5,98.4 377.6,93.4 413.8,81.1 450.0,74.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,170.1 88.2,160.3 124.4,200.2 160.5,196.2 196.7,184.4 232.9,181.2 269.1,169.3 305.3,161.3 341.5,143.2 377.6,139.3 413.8,125.9 450.0,111.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 16.1 µs (0.99×) | 16.3 µs |
| D38 | 393 µs (16×) | 24.7 µs (0.99×) | 25 µs |
| D57 | · | 81.8 µs (5×) | 16.5 µs |
| D76 | 89.6 µs (5.3×) | 112 µs (6.6×) | 16.9 µs |
| D115 | · | 183 µs (4.1×) | 44.4 µs |
| D153 | 120 µs (2.9×) | 162 µs (3.9×) | 41.6 µs |
| D230 | 194 µs (3×) | 248 µs (3.9×) | 63.9 µs |
| D307 | 263 µs (2.6×) | 329 µs (3.3×) | 99.6 µs |
| D462 | · | 324 µs (3×) | 107 µs |
| D616 | · | 408 µs (2.5×) | 164 µs |
| D924 | · | 615 µs (2.4×) | 252 µs |
| D1232 | · | 787 µs (2.2×) | 358 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,71.0 160.5,135.8 232.9,123.0 269.1,102.0 305.3,88.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,211.1 88.2,192.4 124.4,139.8 160.5,126.2 196.7,104.6 232.9,110.0 269.1,91.2 305.3,78.7 341.5,79.4 377.6,69.3 413.8,51.3 450.0,40.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,210.4 88.2,191.8 124.4,210.0 160.5,208.9 196.7,166.6 232.9,169.5 269.1,150.6 305.3,131.2 341.5,127.8 377.6,109.4 413.8,90.5 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.9 µs (0.99×) | 12 µs |
| D38 | 238 µs (14×) | 16.4 µs (0.98×) | 16.7 µs |
| D57 | · | 46.1 µs (4.9×) | 9.31 µs |
| D76 | 48.8 µs (5.2×) | 61.5 µs (6.5×) | 9.42 µs |
| D115 | · | 102 µs (4×) | 25.2 µs |
| D153 | 66.2 µs (2.9×) | 90.8 µs (4×) | 22.8 µs |
| D230 | 110 µs (3.1×) | 140 µs (3.9×) | 36 µs |
| D307 | 146 µs (2.6×) | 186 µs (3.3×) | 55.8 µs |
| D462 | · | 170 µs (2.8×) | 60.6 µs |
| D616 | · | 232 µs (2.5×) | 93.1 µs |
| D924 | · | 344 µs (2.4×) | 142 µs |
| D1232 | · | 439 µs (2.2×) | 203 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,72.0 160.5,118.3 232.9,109.4 269.1,94.5 305.3,86.3" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,159.7 88.2,150.2 124.4,120.0 160.5,111.5 196.7,96.9 232.9,100.1 269.1,87.6 305.3,79.1 341.5,81.8 377.6,72.7 413.8,61.2 450.0,54.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,159.3 88.2,149.7 124.4,166.7 160.5,166.4 196.7,137.6 232.9,140.5 269.1,127.2 305.3,114.4 341.5,112.0 377.6,99.4 413.8,87.1 450.0,76.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.8 µs (0.99×) | 12 µs |
| D38 | 243 µs (14×) | 16.5 µs (0.98×) | 16.8 µs |
| D57 | · | 46.3 µs (5×) | 9.29 µs |
| D76 | 49.3 µs (5.2×) | 62.4 µs (6.6×) | 9.4 µs |
| D115 | · | 103 µs (4.1×) | 25.4 µs |
| D153 | 67.6 µs (3×) | 92.1 µs (4×) | 22.8 µs |
| D230 | 109 µs (3×) | 140 µs (3.9×) | 36 µs |
| D307 | 143 µs (2.6×) | 185 µs (3.3×) | 55.9 µs |
| D462 | · | 170 µs (2.8×) | 60.2 µs |
| D616 | · | 223 µs (2.4×) | 91.9 µs |
| D924 | · | 348 µs (2.4×) | 144 µs |
| D1232 | · | 435 µs (2.2×) | 199 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,71.3 160.5,118.0 232.9,108.8 269.1,94.8 305.3,86.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,159.7 88.2,150.0 124.4,119.9 160.5,111.1 196.7,96.4 232.9,99.7 269.1,87.6 305.3,79.3 341.5,81.8 377.6,73.9 413.8,60.9 450.0,54.4" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,159.3 88.2,149.4 124.4,166.8 160.5,166.5 196.7,137.4 232.9,140.6 269.1,127.2 305.3,114.3 341.5,112.2 377.6,99.8 413.8,86.7 450.0,77.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 251 ns (0.76×) | 330 ns |
| D38 | 364 µs (19×) | 18.3 µs (0.98×) | 18.7 µs |
| D57 | · | 64.7 µs (4.1×) | 15.7 µs |
| D76 | 79.2 µs (5×) | 86 µs (5.4×) | 15.9 µs |
| D115 | · | 147 µs (3.3×) | 44.2 µs |
| D153 | 111 µs (2.7×) | 127 µs (3.1×) | 40.9 µs |
| D230 | 179 µs (2.6×) | 201 µs (2.9×) | 68.5 µs |
| D307 | 243 µs (2.4×) | 268 µs (2.6×) | 103 µs |
| D462 | · | 270 µs (2.3×) | 116 µs |
| D616 | · | 337 µs (1.6×) | 215 µs |
| D924 | · | 497 µs (1.3×) | 377 µs |
| D1232 | · | 631 µs (1.1×) | 574 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,52.2 160.5,85.6 232.9,78.3 269.1,67.7 305.3,61.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,211.8 88.2,117.8 124.4,90.0 160.5,83.8 196.7,72.0 232.9,75.2 269.1,65.2 305.3,58.9 341.5,58.8 377.6,53.8 413.8,45.3 450.0,40.1" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,205.8 88.2,117.3 124.4,121.1 160.5,120.9 196.7,98.4 232.9,100.1 269.1,88.8 305.3,79.9 341.5,77.3 377.6,63.7 413.8,51.4 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 220 ns (0.85×) | 260 ns |
| D38 | 5.09 µs (2.7×) | 5.09 µs (2.7×) | 1.86 µs |
| D57 | · | 2.75 µs (2.5×) | 1.09 µs |
| D76 | 4.66 µs (3.8×) | 3.74 µs (3×) | 1.23 µs |
| D115 | · | 5.41 µs (2.2×) | 2.41 µs |
| D153 | 5.88 µs (1.9×) | 6.08 µs (1.9×) | 3.15 µs |
| D230 | 8.85 µs (1.6×) | 10.8 µs (2×) | 5.51 µs |
| D307 | 11.6 µs (1.6×) | 12.6 µs (1.7×) | 7.27 µs |
| D462 | · | 18.5 µs (1.5×) | 12.2 µs |
| D616 | · | 23.4 µs (1.5×) | 15.9 µs |
| D924 | · | 31.8 µs (1.1×) | 28.4 µs |
| D1232 | · | 42.3 µs (1×) | 40.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,117.1 160.5,119.7 232.9,112.9 269.1,100.9 305.3,93.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,208.9 88.2,117.1 124.4,135.0 160.5,126.1 196.7,115.3 232.9,111.9 269.1,95.0 305.3,90.7 341.5,79.3 377.6,72.5 413.8,63.5 450.0,55.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,204.1 88.2,146.5 124.4,162.1 160.5,158.6 196.7,138.9 232.9,131.2 269.1,114.8 305.3,106.6 341.5,91.6 377.6,83.8 413.8,66.9 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:roots -->
