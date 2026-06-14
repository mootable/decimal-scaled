# History — Trigonometry

How the trigonometric and hyperbolic functions have moved release over release. See
the [History overview](../history.md) for the time units, the width reference map, and
how these timings are measured.

<!-- BEGIN GENERATED:history:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 27 µs (0.5×) | 53.5 µs |
| D38 | 279 µs (3.3×) | 31.9 µs (0.37×) | 85.2 µs |
| D57 | · | 30.6 µs (1.9×) | 16.5 µs |
| D76 | 73.2 µs (4.6×) | 40.3 µs (2.5×) | 16 µs |
| D115 | · | 67.2 µs (3.1×) | 21.8 µs |
| D153 | 109 µs (5×) | 59.5 µs (2.8×) | 21.5 µs |
| D230 | 195 µs (6.7×) | 92.2 µs (3.2×) | 29 µs |
| D307 | 258 µs (8.2×) | 124 µs (4×) | 31.4 µs |
| D462 | · | 126 µs (3.5×) | 35.5 µs |
| D616 | · | 162 µs (4×) | 40.3 µs |
| D924 | · | 244 µs (4.7×) | 52.2 µs |
| D1232 | · | 312 µs (4.7×) | 66 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,86.1 160.5,144.7 232.9,127.4 269.1,101.7 305.3,89.4" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,188.5 88.2,181.1 124.4,182.9 160.5,170.8 196.7,148.4 232.9,153.8 269.1,134.6 305.3,121.5 341.5,120.9 377.6,109.9 413.8,91.9 450.0,81.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,158.4 88.2,138.0 124.4,210.1 160.5,211.5 196.7,197.9 232.9,198.4 269.1,185.3 305.3,181.8 341.5,176.4 377.6,170.8 413.8,159.5 450.0,149.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 28.4 µs (1×) | 28.5 µs |
| D38 | 167 µs (3.7×) | 44.2 µs (0.99×) | 44.5 µs |
| D57 | · | 75 µs (2.4×) | 31.7 µs |
| D76 | 44.6 µs (1.5×) | 93.2 µs (3.1×) | 30.2 µs |
| D115 | · | 170 µs (2.7×) | 62.3 µs |
| D153 | 61.4 µs (0.96×) | 155 µs (2.4×) | 64.2 µs |
| D230 | 106 µs (1.5×) | 231 µs (3.3×) | 71 µs |
| D307 | 139 µs (1.9×) | 295 µs (4.1×) | 71.9 µs |
| D462 | · | 301 µs (3.8×) | 78.2 µs |
| D616 | · | 376 µs (4.6×) | 81.4 µs |
| D924 | · | 551 µs (6×) | 91.8 µs |
| D1232 | · | 702 µs (6.6×) | 106 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,108.6 160.5,166.4 232.9,152.4 269.1,128.5 305.3,116.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,186.1 88.2,166.8 124.4,143.6 160.5,134.1 196.7,107.8 232.9,111.9 269.1,94.3 305.3,83.5 341.5,82.7 377.6,73.0 413.8,56.1 450.0,45.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,186.0 88.2,166.5 124.4,181.4 160.5,183.6 196.7,151.8 232.9,150.4 269.1,146.0 305.3,145.5 341.5,141.8 377.6,140.0 413.8,134.8 450.0,128.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.92×) | 28.1 µs |
| D38 | 245 µs (2.9×) | 30.8 µs (0.36×) | 84.3 µs |
| D57 | · | 29.5 µs (1.8×) | 16.2 µs |
| D76 | 71.4 µs (4.6×) | 38.9 µs (2.5×) | 15.6 µs |
| D115 | · | 65 µs (3.1×) | 21.2 µs |
| D153 | 106 µs (5×) | 58.2 µs (2.8×) | 21.1 µs |
| D230 | 189 µs (6.7×) | 90 µs (3.2×) | 28.4 µs |
| D307 | 253 µs (8.2×) | 117 µs (3.8×) | 30.8 µs |
| D462 | · | 121 µs (3.4×) | 35.2 µs |
| D616 | · | 156 µs (3.9×) | 39.8 µs |
| D924 | · | 229 µs (4.5×) | 50.9 µs |
| D1232 | · | 301 µs (4.7×) | 64.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,91.6 160.5,145.8 232.9,128.4 269.1,103.0 305.3,90.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,190.5 88.2,182.7 124.4,184.5 160.5,172.4 196.7,149.9 232.9,154.7 269.1,135.6 305.3,124.1 341.5,122.8 377.6,111.5 413.8,94.6 450.0,82.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,186.7 88.2,138.5 124.4,210.9 160.5,212.6 196.7,199.0 232.9,199.3 269.1,186.2 305.3,182.6 341.5,176.8 377.6,171.5 413.8,160.6 450.0,150.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 30.8 µs (0.99×) | 31 µs |
| D38 | 183 µs (4×) | 45.2 µs (0.99×) | 45.4 µs |
| D57 | · | 21.1 µs (2.1×) | 10.1 µs |
| D76 | 43.8 µs (4.4×) | 26.8 µs (2.7×) | 9.94 µs |
| D115 | · | 47.6 µs (3.1×) | 15.1 µs |
| D153 | 61.5 µs (4.5×) | 39.6 µs (2.9×) | 13.7 µs |
| D230 | 104 µs (5.4×) | 62.1 µs (3.2×) | 19.1 µs |
| D307 | 139 µs (6.5×) | 81.6 µs (3.8×) | 21.5 µs |
| D462 | · | 83.4 µs (3.2×) | 26.2 µs |
| D616 | · | 109 µs (3.6×) | 30.4 µs |
| D924 | · | 160 µs (3.8×) | 42.4 µs |
| D1232 | · | 203 µs (3.6×) | 56.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,79.6 160.5,121.5 232.9,111.5 269.1,96.2 305.3,87.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,131.8 88.2,120.6 124.4,142.8 160.5,135.8 196.7,119.1 232.9,124.4 269.1,111.2 305.3,103.3 341.5,102.6 377.6,94.7 413.8,83.5 450.0,76.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,131.6 88.2,120.4 124.4,164.3 160.5,164.8 196.7,152.5 232.9,155.4 269.1,145.7 305.3,142.3 341.5,136.4 377.6,132.2 413.8,122.4 450.0,114.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.8 µs (0.45×) | 57.4 µs |
| D38 | 292 µs (3.4×) | 33.7 µs (0.39×) | 85.7 µs |
| D57 | · | 30.1 µs (2×) | 14.8 µs |
| D76 | 67.1 µs (4.7×) | 38.1 µs (2.7×) | 14.2 µs |
| D115 | · | 67.1 µs (3.4×) | 19.9 µs |
| D153 | 98.2 µs (5.2×) | 55.8 µs (2.9×) | 19 µs |
| D230 | 170 µs (6.6×) | 89 µs (3.4×) | 25.9 µs |
| D307 | 231 µs (8.2×) | 115 µs (4.1×) | 28.3 µs |
| D462 | · | 112 µs (4.3×) | 25.9 µs |
| D616 | · | 151 µs (4.2×) | 35.9 µs |
| D924 | · | 222 µs (4.6×) | 47.9 µs |
| D1232 | · | 285 µs (4.6×) | 62.5 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,84.0 160.5,148.5 232.9,131.8 269.1,107.6 305.3,94.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,190.4 88.2,178.7 124.4,183.7 160.5,173.3 196.7,148.5 232.9,156.6 269.1,136.1 305.3,124.8 341.5,126.2 377.6,112.9 413.8,96.0 450.0,85.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,155.4 88.2,137.8 124.4,214.8 160.5,216.8 196.7,201.9 232.9,203.9 269.1,190.2 305.3,186.3 341.5,190.3 377.6,176.0 413.8,163.3 450.0,151.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 25.4 µs (0.79×) | 32.1 µs |
| D38 | 245 µs (3×) | 31.2 µs (0.39×) | 80.7 µs |
| D57 | · | 30.5 µs (2×) | 15.3 µs |
| D76 | 71.2 µs (4.8×) | 40.4 µs (2.7×) | 15 µs |
| D115 | · | 66.9 µs (3.2×) | 20.6 µs |
| D153 | 104 µs (5×) | 60.2 µs (2.9×) | 20.8 µs |
| D230 | 181 µs (6×) | 94.5 µs (3.2×) | 30 µs |
| D307 | 243 µs (7.2×) | 124 µs (3.7×) | 33.9 µs |
| D462 | · | 124 µs (3×) | 41.7 µs |
| D616 | · | 165 µs (3.4×) | 48.8 µs |
| D924 | · | 244 µs (3.5×) | 70.2 µs |
| D1232 | · | 314 µs (3.2×) | 99.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,91.8 160.5,145.9 232.9,129.2 269.1,105.1 305.3,92.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,191.1 88.2,182.0 124.4,183.1 160.5,170.8 196.7,148.7 232.9,153.2 269.1,133.5 305.3,121.6 341.5,121.4 377.6,108.9 413.8,91.9 450.0,80.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,180.8 88.2,140.4 124.4,213.3 160.5,214.3 196.7,200.2 232.9,199.8 269.1,183.8 305.3,178.5 341.5,169.4 377.6,162.5 413.8,146.5 450.0,131.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 10 µs (0.97×) | 10.3 µs |
| D38 | 217 µs (14×) | 14.6 µs (0.97×) | 15 µs |
| D57 | · | 108 µs (1.9×) | 56.6 µs |
| D76 | 49.4 µs (0.92×) | 136 µs (2.5×) | 53.5 µs |
| D115 | · | 242 µs (2.3×) | 107 µs |
| D153 | 67.9 µs (0.63×) | 218 µs (2×) | 108 µs |
| D230 | 111 µs (0.9×) | 331 µs (2.7×) | 123 µs |
| D307 | 148 µs (1.2×) | 444 µs (3.6×) | 125 µs |
| D462 | · | 415 µs (3.2×) | 130 µs |
| D616 | · | 557 µs (4.1×) | 135 µs |
| D924 | · | 835 µs (5.8×) | 144 µs |
| D1232 | · | 1.08 ms (6.7×) | 162 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,142.0 160.5,185.3 232.9,176.0 269.1,161.7 305.3,153.2" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,231.9 88.2,220.9 124.4,162.5 160.5,155.7 196.7,138.8 232.9,141.9 269.1,129.6 305.3,121.1 341.5,123.1 377.6,114.4 413.8,102.6 450.0,95.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,231.0 88.2,220.1 124.4,181.3 160.5,183.0 196.7,162.7 232.9,162.4 269.1,158.5 305.3,158.1 341.5,157.0 377.6,156.0 413.8,153.9 450.0,150.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 5.98 µs (1×) | 5.94 µs |
| D38 | 126 µs (14×) | 8.84 µs (0.99×) | 8.94 µs |
| D57 | · | 19.9 µs (2.9×) | 6.78 µs |
| D76 | 34.3 µs (4.8×) | 26.1 µs (3.7×) | 7.11 µs |
| D115 | · | 44.6 µs (4.1×) | 11 µs |
| D153 | 49.2 µs (4.8×) | 38.4 µs (3.7×) | 10.2 µs |
| D230 | 81.3 µs (5.6×) | 60.9 µs (4.2×) | 14.5 µs |
| D307 | 109 µs (6.5×) | 81.6 µs (4.9×) | 16.7 µs |
| D462 | · | 82.4 µs (4.4×) | 18.7 µs |
| D616 | · | 104 µs (4×) | 25.9 µs |
| D924 | · | 155 µs (4.2×) | 37.2 µs |
| D1232 | · | 194 µs (3.7×) | 52.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,90.6 160.5,128.6 232.9,118.1 269.1,103.4 305.3,94.8" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.7 88.2,168.3 124.4,144.6 160.5,136.6 196.7,120.9 232.9,125.3 269.1,111.8 305.3,103.3 341.5,103.0 377.6,96.3 413.8,84.5 450.0,77.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,179.9 88.2,168.0 124.4,176.0 160.5,174.6 196.7,161.9 232.9,164.0 269.1,153.8 305.3,149.7 341.5,146.3 377.6,136.9 413.8,126.3 450.0,116.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.69 µs (0.99×) | 9.78 µs |
| D38 | 359 µs (26×) | 13.7 µs (0.98×) | 14 µs |
| D57 | · | 246 µs (27×) | 9.26 µs |
| D76 | 67.3 µs (6.5×) | 275 µs (27×) | 10.4 µs |
| D115 | · | 814 µs (33×) | 24.4 µs |
| D153 | 96.8 µs (6.5×) | 652 µs (44×) | 14.9 µs |
| D230 | 155 µs (8.3×) | 1.01 ms (54×) | 18.6 µs |
| D307 | 218 µs (10×) | 1.39 ms (65×) | 21.3 µs |
| D462 | · | 1.35 ms (52×) | 26.1 µs |
| D616 | · | 5.75 ms (1.9e+02×) | 29.9 µs |
| D924 | · | 5.48 ms (1.3e+02×) | 41.6 µs |
| D1232 | · | 5.35 ms (92×) | 57.9 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,103.0 160.5,139.7 232.9,131.7 269.1,121.3 305.3,113.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,182.2 88.2,174.6 124.4,111.2 160.5,108.8 196.7,85.0 232.9,89.9 269.1,80.4 305.3,73.3 341.5,74.0 377.6,42.1 413.8,43.2 450.0,43.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.0 88.2,174.2 124.4,183.2 160.5,180.7 196.7,162.0 232.9,172.7 269.1,167.8 305.3,164.9 341.5,160.4 377.6,157.5 413.8,150.2 450.0,143.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 6.1 µs (1×) | 6.08 µs |
| D38 | 124 µs (14×) | 8.8 µs (0.98×) | 8.93 µs |
| D57 | · | 19.5 µs (2.9×) | 6.66 µs |
| D76 | 33.9 µs (4.9×) | 26 µs (3.7×) | 6.98 µs |
| D115 | · | 44.8 µs (4.1×) | 10.9 µs |
| D153 | 48.7 µs (4.8×) | 37.5 µs (3.7×) | 10.1 µs |
| D230 | 80.6 µs (5.6×) | 60.9 µs (4.3×) | 14.3 µs |
| D307 | 108 µs (6.6×) | 79.2 µs (4.8×) | 16.5 µs |
| D462 | · | 80.3 µs (4.3×) | 18.8 µs |
| D616 | · | 105 µs (4.2×) | 25.4 µs |
| D924 | · | 156 µs (4.1×) | 37.7 µs |
| D1232 | · | 195 µs (3.8×) | 50.8 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,90.9 160.5,128.9 232.9,118.3 269.1,103.6 305.3,95.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,179.1 88.2,168.4 124.4,145.2 160.5,136.7 196.7,120.8 232.9,126.0 269.1,111.9 305.3,104.1 341.5,103.7 377.6,95.8 413.8,84.4 450.0,77.7" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,179.2 88.2,168.0 124.4,176.6 160.5,175.2 196.7,162.2 232.9,164.4 269.1,154.2 305.3,150.1 341.5,146.3 377.6,137.5 413.8,125.9 450.0,117.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 9.72 µs (0.99×) | 9.81 µs |
| D38 | 347 µs (25×) | 13.7 µs (0.98×) | 14 µs |
| D57 | · | 244 µs (26×) | 9.24 µs |
| D76 | 66.4 µs (6.4×) | 276 µs (27×) | 10.3 µs |
| D115 | · | 815 µs (33×) | 24.7 µs |
| D153 | 96.2 µs (6.5×) | 646 µs (44×) | 14.7 µs |
| D230 | 156 µs (8.5×) | 1.01 ms (55×) | 18.4 µs |
| D307 | 212 µs (10×) | 1.38 ms (66×) | 20.9 µs |
| D462 | · | 1.34 ms (52×) | 25.9 µs |
| D616 | · | 5.75 ms (1.9e+02×) | 29.6 µs |
| D924 | · | 5.41 ms (1.3e+02×) | 41.7 µs |
| D1232 | · | 5.32 ms (88×) | 60.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,103.7 160.5,140.0 232.9,131.8 269.1,121.2 305.3,114.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,182.1 88.2,174.6 124.4,111.4 160.5,108.7 196.7,85.0 232.9,90.1 269.1,80.4 305.3,73.4 341.5,74.1 377.6,42.1 413.8,43.5 450.0,43.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,181.9 88.2,174.1 124.4,183.2 160.5,180.8 196.7,161.6 232.9,173.0 269.1,168.2 305.3,165.3 341.5,160.7 377.6,157.7 413.8,150.2 450.0,142.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 11.2 µs (1×) | 11.2 µs |
| D38 | 257 µs (16×) | 16.3 µs (0.98×) | 16.6 µs |
| D57 | · | 22.7 µs (2.6×) | 8.75 µs |
| D76 | 43.1 µs (4.9×) | 29.5 µs (3.3×) | 8.89 µs |
| D115 | · | 51.9 µs (3.9×) | 13.5 µs |
| D153 | 62.9 µs (5.1×) | 44 µs (3.5×) | 12.4 µs |
| D230 | 109 µs (6.3×) | 71.4 µs (4.1×) | 17.4 µs |
| D307 | 144 µs (7.4×) | 94.9 µs (4.8×) | 19.6 µs |
| D462 | · | 91.1 µs (4.4×) | 20.9 µs |
| D616 | · | 122 µs (4.1×) | 29.4 µs |
| D924 | · | 177 µs (4.5×) | 39.3 µs |
| D1232 | · | 233 µs (4.3×) | 54.3 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,69.8 160.5,121.9 232.9,110.9 269.1,94.7 305.3,86.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,161.4 88.2,150.4 124.4,140.7 160.5,133.0 196.7,116.5 232.9,121.4 269.1,107.2 305.3,98.9 341.5,100.1 377.6,91.5 413.8,80.7 450.0,72.6" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,161.4 88.2,149.9 124.4,168.6 160.5,168.1 196.7,155.9 232.9,158.4 269.1,148.4 305.3,145.0 341.5,143.1 377.6,133.1 413.8,124.7 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | · | 12.9 µs (1.3×) | 9.75 µs |
| D38 | 329 µs (24×) | 18.4 µs (1.3×) | 14 µs |
| D57 | · | 248 µs (25×) | 9.76 µs |
| D76 | 66.5 µs (6.2×) | 279 µs (26×) | 10.8 µs |
| D115 | · | 836 µs (28×) | 29.6 µs |
| D153 | 97.3 µs (6.2×) | 659 µs (42×) | 15.8 µs |
| D230 | 159 µs (8×) | 1.05 ms (53×) | 19.8 µs |
| D307 | 217 µs (9.7×) | 1.82 ms (81×) | 22.4 µs |
| D462 | · | 1.77 ms (64×) | 27.5 µs |
| D616 | · | 5.82 ms (1.8e+02×) | 33 µs |
| D924 | · | 5.6 ms (1.3e+02×) | 43.3 µs |
| D1232 | · | 5.4 ms (83×) | 65.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="181.5" x2="450" y2="181.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.5" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="131.0" x2="450" y2="131.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="134.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="80.5" x2="450" y2="80.5" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.5" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="88.2,104.9 160.5,140.0 232.9,131.6 269.1,120.8 305.3,114.0" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,175.9 88.2,168.1 124.4,111.1 160.5,108.5 196.7,84.4 232.9,89.6 269.1,79.4 305.3,67.4 341.5,67.9 377.6,41.9 413.8,42.7 450.0,43.5" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,182.0 88.2,174.2 124.4,182.0 160.5,179.9 196.7,157.7 232.9,171.5 269.1,166.5 305.3,163.8 341.5,159.3 377.6,155.3 413.8,149.4 450.0,140.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:trig -->
