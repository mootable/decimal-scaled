# History — Arithmetic

How the arithmetic operations have moved release over release. See the
[History overview](../history.md) for the time units, the width reference map, and how
these timings are measured.

<!-- BEGIN GENERATED:history:body:arithmetic -->
### `add`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 206 ns (0.79×) | 235 ns (0.9×) | 260 ns |
| D38 | 331 ns (0.59×) | 350 ns (0.62×) | 561 ns |
| D57 | · | 2.17 µs (2.7×) | 791 ns |
| D76 | 972 ns (0.97×) | 2.6 µs (2.6×) | 1 µs |
| D115 | · | 4.93 µs (2.1×) | 2.34 µs |
| D153 | 1.33 µs (0.41×) | 6.14 µs (1.9×) | 3.24 µs |
| D230 | 3.13 µs (0.39×) | 11.7 µs (1.5×) | 7.94 µs |
| D307 | 2.67 µs (0.24×) | 14.1 µs (1.3×) | 11 µs |
| D462 | · | 21.8 µs (1.1×) | 20.6 µs |
| D616 | · | 27.7 µs (0.93×) | 29.9 µs |
| D924 | · | 41.5 µs (0.68×) | 61.2 µs |
| D1232 | · | 54.5 µs (0.58×) | 94.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,210.9 88.2,197.0 160.5,165.5 232.9,156.4 269.1,131.3 305.3,135.9" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,207.0 88.2,195.4 124.4,142.0 160.5,136.7 196.7,118.0 232.9,111.6 269.1,92.6 305.3,87.4 341.5,74.5 377.6,67.5 413.8,55.7 450.0,47.8" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,204.1 88.2,181.6 124.4,171.5 160.5,164.6 196.7,139.8 232.9,130.3 269.1,104.1 305.3,94.5 341.5,76.2 377.6,65.3 413.8,44.4 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `div`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 191 ns (0.83×) | 200 ns (0.87×) | 231 ns |
| D38 | 1.06 µs (0.83×) | 1.07 µs (0.84×) | 1.27 µs |
| D57 | · | 2.7 µs (3.3×) | 821 ns |
| D76 | · | 3.08 µs (3.2×) | 962 ns |
| D115 | · | 4.85 µs (2.2×) | 2.24 µs |
| D153 | · | 6.03 µs (2.2×) | 2.74 µs |
| D230 | 5.15 µs (0.78×) | 11.5 µs (1.7×) | 6.56 µs |
| D307 | 6.25 µs (0.66×) | 14.3 µs (1.5×) | 9.53 µs |
| D462 | · | 24.4 µs (1.3×) | 18.6 µs |
| D616 | · | 34 µs (1.2×) | 28.5 µs |
| D924 | · | 50.9 µs (0.89×) | 57.3 µs |
| D1232 | · | 70.6 µs (0.71×) | 99.1 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,213.1 88.2,162.9 269.1,116.8 305.3,111.1" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,211.7 88.2,162.6 124.4,135.6 160.5,131.8 196.7,118.5 232.9,112.1 269.1,93.4 305.3,86.8 341.5,71.3 377.6,61.6 413.8,49.7 450.0,40.2" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,207.5 88.2,157.6 124.4,170.4 160.5,165.8 196.7,141.0 232.9,135.1 269.1,109.7 305.3,98.7 341.5,79.2 377.6,66.7 413.8,46.3 450.0,30.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `mul`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 240 ns (0.86×) | 241 ns (0.86×) | 280 ns |
| D38 | 370 ns (0.65×) | 370 ns (0.65×) | 572 ns |
| D57 | · | 2.14 µs (2.9×) | 746 ns |
| D76 | · | 2.49 µs (2.8×) | 897 ns |
| D115 | · | 4.02 µs (1.9×) | 2.14 µs |
| D153 | · | 5.16 µs (2×) | 2.63 µs |
| D230 | 4.02 µs (0.67×) | 9.32 µs (1.5×) | 6.02 µs |
| D307 | 4.81 µs (0.53×) | 10.7 µs (1.2×) | 9.13 µs |
| D462 | · | 17.8 µs (1.1×) | 16.8 µs |
| D616 | · | 22.9 µs (0.89×) | 25.6 µs |
| D924 | · | 34 µs (0.71×) | 47.8 µs |
| D1232 | · | 45.5 µs (0.59×) | 76.6 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,206.4 88.2,193.7 269.1,124.0 305.3,118.7" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,206.3 88.2,193.7 124.4,142.4 160.5,138.0 196.7,124.0 232.9,116.7 269.1,99.4 305.3,95.3 341.5,80.5 377.6,73.2 413.8,61.6 450.0,53.0" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,201.9 88.2,181.0 124.4,173.2 160.5,167.8 196.7,142.4 232.9,136.3 269.1,112.2 305.3,100.0 341.5,82.2 377.6,69.9 413.8,51.6 450.0,37.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `rem`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 205 ns (0.89×) | 190 ns (0.82×) | 231 ns |
| D38 | 330 ns (0.63×) | 336 ns (0.64×) | 521 ns |
| D57 | · | 2.17 µs (3×) | 721 ns |
| D76 | · | 2.54 µs (2.8×) | 916 ns |
| D115 | · | 4.04 µs (1.9×) | 2.09 µs |
| D153 | · | 5.16 µs (2×) | 2.63 µs |
| D230 | 3.72 µs (0.58×) | 9.22 µs (1.4×) | 6.37 µs |
| D307 | 4.68 µs (0.52×) | 10.5 µs (1.2×) | 9.05 µs |
| D462 | · | 16 µs (0.89×) | 18 µs |
| D616 | · | 21.5 µs (0.79×) | 27.2 µs |
| D924 | · | 33.5 µs (0.62×) | 54.2 µs |
| D1232 | · | 44.1 µs (0.45×) | 99 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,211.0 88.2,197.1 269.1,126.3 305.3,119.5" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,213.2 88.2,196.6 124.4,142.0 160.5,137.5 196.7,123.9 232.9,116.7 269.1,99.7 305.3,95.9 341.5,83.5 377.6,75.0 413.8,62.0 450.0,53.9" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,207.5 88.2,183.7 124.4,174.2 160.5,167.2 196.7,143.1 232.9,136.4 269.1,110.5 305.3,100.3 341.5,80.2 377.6,68.1 413.8,47.9 450.0,30.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>

### `sub`

<div class="grid perf-grid" markdown>

| Width | 0.3.3 | 0.4.4 | 0.5.0 |
| :-- | --: | --: | --: |
| D18 | 190 ns (0.83×) | 196 ns (0.85×) | 230 ns |
| D38 | 341 ns (0.62×) | 351 ns (0.64×) | 551 ns |
| D57 | · | 2.16 µs (2.7×) | 791 ns |
| D76 | 1.09 µs (1.2×) | 2.67 µs (2.8×) | 942 ns |
| D115 | · | 4.87 µs (2×) | 2.44 µs |
| D153 | 1.82 µs (0.56×) | 6.13 µs (1.9×) | 3.27 µs |
| D230 | 3.24 µs (0.41×) | 11.8 µs (1.5×) | 7.84 µs |
| D307 | 3.94 µs (0.36×) | 13.9 µs (1.3×) | 10.9 µs |
| D462 | · | 21.9 µs (1×) | 20.9 µs |
| D616 | · | 28 µs (0.95×) | 29.4 µs |
| D924 | · | 41.3 µs (0.73×) | 56.9 µs |
| D1232 | · | 55.4 µs (0.58×) | 95.4 µs |

<figure>
<svg viewBox="0 0 460 262" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="232.0" x2="450" y2="232.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="235.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="164.7" x2="450" y2="164.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="167.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="97.3" x2="450" y2="97.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="100.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="30.0" x2="450" y2="30.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="33.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="244" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="244" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="244" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="244" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="244" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="244" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="244" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="244" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="244" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="244" text-anchor="middle" font-size="8" fill="currentColor">1232</text><line x1="52" y1="20" x2="66" y2="20" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="2"/><text x="69" y="23" font-size="9" fill="currentColor">0.3.3</text><line x1="111" y1="20" x2="125" y2="20" stroke="var(--md-accent-fg-color)" stroke-width="2"/><text x="128" y="23" font-size="9" fill="currentColor">0.4.4</text><line x1="170" y1="20" x2="184" y2="20" stroke="var(--md-primary-fg-color)" stroke-width="2"/><text x="187" y="23" font-size="9" fill="currentColor">0.5.0</text><polyline points="52.0,213.2 88.2,196.1 160.5,162.2 232.9,147.2 269.1,130.3 305.3,124.6" fill="none" stroke="var(--dusk-purple,#7A6A8E)" stroke-width="1.3"/><polyline points="52.0,212.3 88.2,195.3 124.4,142.1 160.5,136.0 196.7,118.4 232.9,111.6 269.1,92.5 305.3,87.8 341.5,74.4 377.6,67.2 413.8,55.8 450.0,47.3" fill="none" stroke="var(--md-accent-fg-color)" stroke-width="1.3"/><polyline points="52.0,207.6 88.2,182.1 124.4,171.5 160.5,166.4 196.7,138.5 232.9,130.0 269.1,104.4 305.3,94.7 341.5,75.8 377.6,65.8 413.8,46.5 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="2.0"/><line x1="52" y1="30" x2="52" y2="232" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="232" x2="450" y2="232" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale), one line per release with a shaded min–max band; the multiplier is the slowdown relative to the latest.</figcaption>
</figure>

</div>
<!-- END GENERATED:history:body:arithmetic -->
