# Comparisons — Roots and Exponents

Speed of `decimal-scaled` against the peer decimal crates on the root and exponential
functions. See the [Comparisons overview](../comparisons.md) for the time units, the
per-library precision model, and how to read the timings.

<!-- BEGIN GENERATED:comparisons:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | bigdecimal | fastnum |
| :-- | --: | --: | --: |
| D18 | 6.01 µs | 118 µs (20×) | 819 µs (1.4e+02×) |
| D38 | 11.3 µs | 118 µs (10×) | 816 µs (72×) |
| D57 | 1.37 µs | 107 µs (78×) | 746 µs (5.4e+02×) |
| D76 | 2.07 µs | 107 µs (51×) | 739 µs (3.6e+02×) |
| D115 | 3.42 µs | 157 µs (46×) | 1.39 ms (4.1e+02×) |
| D153 | 6.14 µs | 202 µs (33×) | 1.77 ms (2.9e+02×) |
| D230 | 5.3 µs | 113 µs (21×) | 656 µs (1.2e+02×) |
| D307 | 8.65 µs | 203 µs (23×) | 1.71 ms (2e+02×) |
| D462 | 15 µs | 202 µs (13×) | 1.31 ms (87×) |
| D616 | 20.2 µs | 202 µs (10×) | 2.14 ms (1.1e+02×) |
| D924 | 35.2 µs | 202 µs (5.7×) | 4.02 ms (1.1e+02×) |
| D1232 | 50.9 µs | 179 µs (3.5×) | 1.59 ms (31×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="206.0" x2="450" y2="206.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="209.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="98.0" x2="450" y2="98.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="101.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,217.9 88.2,203.1 124.4,252.6 160.5,242.9 196.7,231.1 232.9,217.4 269.1,220.9 305.3,209.4 341.5,196.4 377.6,189.5 413.8,176.5 450.0,167.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,148.2 88.2,148.2 124.4,150.5 160.5,150.5 196.7,141.5 232.9,135.5 269.1,149.1 305.3,135.4 341.5,135.5 377.6,135.5 413.8,135.5 450.0,138.3" fill="none" stroke="#C68A2E" stroke-width="1.6"/><polyline points="52.0,102.7 88.2,102.8 124.4,104.9 160.5,105.1 196.7,90.3 232.9,84.6 269.1,107.9 305.3,85.4 341.5,91.6 377.6,80.1 413.8,65.4 450.0,87.1" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#C68A2E"/><text x="164" y="20" font-size="8.5" fill="currentColor">bigdecimal</text><rect x="228" y="12" width="9" height="9" fill="#9C5BA6"/><text x="240" y="20" font-size="8.5" fill="currentColor">fastnum</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | decimal-rs | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.13 µs | 9.52 µs (1.3×) | 470 µs (66×) | 153 µs (21×) | 4.48 µs (0.63×) |
| D38 | 10.1 µs | 9.51 µs (0.94×) | 467 µs (46×) | 151 µs (15×) | 4.52 µs (0.45×) |
| D57 | 5.61 µs | 8.61 µs (1.5×) | 436 µs (78×) | 145 µs (26×) | 4.18 µs (0.75×) |
| D76 | 8.15 µs | 8.77 µs (1.1×) | 433 µs (53×) | 145 µs (18×) | 4.2 µs (0.52×) |
| D115 | 12.6 µs | 9.35 µs (0.74×) | 461 µs (36×) | 231 µs (18×) | 4.5 µs (0.36×) |
| D153 | 20.5 µs | 11.9 µs (0.58×) | 602 µs (29×) | 295 µs (14×) | 5.83 µs (0.28×) |
| D230 | 24.9 µs | 7.61 µs (0.31×) | 400 µs (16×) | 117 µs (4.7×) | 4.07 µs (0.16×) |
| D307 | 64.2 µs | 12 µs (0.19×) | 593 µs (9.2×) | 302 µs (4.7×) | 5.72 µs (0.089×) |
| D462 | 68.1 µs | 12.1 µs (0.18×) | 607 µs (8.9×) | 293 µs (4.3×) | 5.71 µs (0.084×) |
| D616 | 102 µs | 11.9 µs (0.12×) | 616 µs (6×) | 301 µs (2.9×) | 5.81 µs (0.057×) |
| D924 | 153 µs | 11.9 µs (0.078×) | 607 µs (4×) | 294 µs (1.9×) | 5.7 µs (0.037×) |
| D1232 | 229 µs | 10.7 µs (0.047×) | 575 µs (2.5×) | 177 µs (0.77×) | 5.86 µs (0.026×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,198.6 88.2,187.8 124.4,206.1 160.5,194.4 196.7,180.7 232.9,165.5 269.1,159.5 305.3,129.9 341.5,128.0 377.6,115.3 413.8,102.6 450.0,90.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,189.5 88.2,189.6 124.4,192.7 160.5,192.1 196.7,190.1 232.9,182.5 269.1,196.6 305.3,182.4 341.5,182.1 377.6,182.5 413.8,182.5 450.0,185.9" fill="none" stroke="#367594" stroke-width="1.6"/><polyline points="52.0,67.6 88.2,67.8 124.4,70.0 160.5,70.2 196.7,68.2 232.9,59.9 269.1,72.6 305.3,60.4 341.5,59.6 377.6,59.1 413.8,59.6 450.0,61.3" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,102.8 88.2,103.0 124.4,104.4 160.5,104.4 196.7,89.9 232.9,82.2 269.1,111.1 305.3,81.4 341.5,82.4 377.6,81.5 413.8,82.3 450.0,98.2" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,213.1 88.2,212.8 124.4,215.3 160.5,215.1 196.7,213.0 232.9,204.9 269.1,216.1 305.3,205.5 341.5,205.5 377.6,205.0 413.8,205.6 450.0,204.7" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#367594"/><text x="164" y="20" font-size="8.5" fill="currentColor">decimal-rs</text><rect x="228" y="12" width="9" height="9" fill="#9C5BA6"/><text x="240" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="286" y="12" width="9" height="9" fill="#5E8C3A"/><text x="298" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="338" y="12" width="9" height="9" fill="#B5663C"/><text x="350" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `exp2`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum |
| :-- | --: | --: |
| D18 | 6.95 µs | 1.69 ms (2.4e+02×) |
| D38 | 10.3 µs | 1.69 ms (1.6e+02×) |
| D57 | 13.3 µs | 1.54 ms (1.2e+02×) |
| D76 | 15.2 µs | 1.54 ms (1e+02×) |
| D115 | 38.9 µs | 1.63 ms (42×) |
| D153 | 50.4 µs | 2.14 ms (42×) |
| D230 | 48.7 µs | 1.35 ms (28×) |
| D307 | 117 µs | 2.15 ms (18×) |
| D462 | 121 µs | 2.18 ms (18×) |
| D616 | 333 µs | 4 ms (12×) |
| D924 | 332 µs | 2.23 ms (6.7×) |
| D1232 | 1.03 ms | 1.87 ms (1.8×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="206.0" x2="450" y2="206.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="209.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="98.0" x2="450" y2="98.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="101.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,214.5 88.2,205.2 124.4,199.3 160.5,196.1 196.7,174.1 232.9,168.1 269.1,168.9 305.3,148.3 341.5,147.5 377.6,123.8 413.8,123.9 450.0,97.2" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,85.7 88.2,85.7 124.4,87.8 160.5,87.9 196.7,86.5 232.9,80.2 269.1,91.0 305.3,80.1 341.5,79.7 377.6,65.5 413.8,79.2 450.0,83.3" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled |
| :-- | --: |
| D18 | 261 ns |
| D38 | 581 ns |
| D57 | 861 ns |
| D76 | 1.38 µs |
| D115 | 2.05 µs |
| D153 | 4.38 µs |
| D230 | 4.63 µs |
| D307 | 9.85 µs |
| D462 | 18.4 µs |
| D616 | 26.5 µs |
| D924 | 50.1 µs |
| D1232 | 75.5 µs |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,230.0 88.2,205.0 124.4,192.7 160.5,178.0 196.7,165.5 232.9,141.8 269.1,140.1 305.3,116.5 341.5,96.9 377.6,85.5 413.8,65.6 450.0,52.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | dashu-float | decimal-rs | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: | --: | --: |
| D18 | 6.9 µs | 65 ms (9.4e+03×) | 5.4 µs (0.78×) | 3.55 ms (5.1e+02×) | 200 µs (29×) | 8.75 µs (1.3×) |
| D38 | 10.3 µs | 64.9 ms (6.3e+03×) | 5.39 µs (0.52×) | 3.56 ms (3.5e+02×) | 185 µs (18×) | 8.8 µs (0.86×) |
| D57 | 2.75 µs | 59.5 ms (2.2e+04×) | 4.91 µs (1.8×) | 3.23 ms (1.2e+03×) | 197 µs (72×) | 8.24 µs (3×) |
| D76 | 3.46 µs | 59.2 ms (1.7e+04×) | 4.99 µs (1.4×) | 3.22 ms (9.3e+02×) | 195 µs (57×) | 8.11 µs (2.3×) |
| D115 | 4.05 µs | 173 ms (4.3e+04×) | 5.37 µs (1.3×) | 9.33 ms (2.3e+03×) | 279 µs (69×) | 9.03 µs (2.2×) |
| D153 | 7.23 µs | 225 ms (3.1e+04×) | 6.94 µs (0.96×) | 10.6 ms (1.5e+03×) | 362 µs (50×) | 11.5 µs (1.6×) |
| D230 | 5.24 µs | 133 ms (2.5e+04×) | 4.49 µs (0.86×) | 6.18 ms (1.2e+03×) | 163 µs (31×) | 8 µs (1.5×) |
| D307 | 11 µs | 222 ms (2e+04×) | 6.84 µs (0.62×) | 12.1 ms (1.1e+03×) | 375 µs (34×) | 12.1 µs (1.1×) |
| D462 | 20.5 µs | 216 ms (1.1e+04×) | 6.96 µs (0.34×) | 10.5 ms (5.1e+02×) | 363 µs (18×) | 11.6 µs (0.57×) |
| D616 | 24.7 µs | 220 ms (8.9e+03×) | 6.99 µs (0.28×) | 10.4 ms (4.2e+02×) | 374 µs (15×) | 11.3 µs (0.46×) |
| D924 | 39.4 µs | 223 ms (5.7e+03×) | 6.88 µs (0.17×) | 10.4 ms (2.6e+02×) | 363 µs (9.2×) | 11.7 µs (0.3×) |
| D1232 | 61.3 µs | 209 ms (3.4e+03×) | 6.23 µs (0.1×) | 9.6 ms (1.6e+02×) | 235 µs (3.8×) | 11.9 µs (0.19×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 s</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,229.8 88.2,223.6 124.4,244.2 160.5,240.6 196.7,238.1 232.9,229.1 269.1,234.1 305.3,222.5 341.5,212.8 377.6,209.8 413.8,202.6 450.0,195.7" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,86.7 88.2,86.8 124.4,88.1 160.5,88.2 196.7,71.4 232.9,67.3 269.1,75.6 305.3,67.6 341.5,67.9 377.6,67.7 413.8,67.5 450.0,68.5" fill="none" stroke="#7A6A8E" stroke-width="1.6"/><polyline points="52.0,233.6 88.2,233.7 124.4,235.1 160.5,234.9 196.7,233.7 232.9,229.7 269.1,236.5 305.3,229.9 341.5,229.7 377.6,229.6 413.8,229.8 450.0,231.4" fill="none" stroke="#367594" stroke-width="1.6"/><polyline points="52.0,132.2 88.2,132.2 124.4,133.7 160.5,133.7 196.7,117.1 232.9,115.0 269.1,123.5 305.3,113.0 341.5,115.3 377.6,115.3 413.8,115.3 450.0,116.6" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,177.1 88.2,178.4 124.4,177.4 160.5,177.5 196.7,172.0 232.9,167.9 269.1,180.4 305.3,167.3 341.5,167.8 377.6,167.4 413.8,167.8 450.0,174.7" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,226.1 88.2,226.0 124.4,227.0 160.5,227.3 196.7,225.6 232.9,221.8 269.1,227.5 305.3,221.1 341.5,221.6 377.6,222.0 413.8,221.5 450.0,221.3" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#7A6A8E"/><text x="164" y="20" font-size="8.5" fill="currentColor">dashu-float</text><rect x="234" y="12" width="9" height="9" fill="#367594"/><text x="246" y="20" font-size="8.5" fill="currentColor">decimal-rs</text><rect x="310" y="12" width="9" height="9" fill="#9C5BA6"/><text x="322" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="368" y="12" width="9" height="9" fill="#5E8C3A"/><text x="380" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="52" y="25" width="9" height="9" fill="#B5663C"/><text x="64" y="33" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled |
| :-- | --: |
| D18 | 17.6 µs |
| D38 | 26.7 µs |
| D57 | 15.1 µs |
| D76 | 17.2 µs |
| D115 | 36.1 µs |
| D153 | 51.6 µs |
| D230 | 39.9 µs |
| D307 | 96.5 µs |
| D462 | 105 µs |
| D616 | 167 µs |
| D924 | 262 µs |
| D1232 | 360 µs |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,233.6 88.2,213.9 124.4,240.5 160.5,234.6 196.7,199.8 232.9,183.0 269.1,195.1 305.3,153.7 341.5,149.7 377.6,128.0 413.8,106.9 450.0,91.9" fill="none" stroke="#2563eb" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | rust_decimal |
| :-- | --: | --: | --: |
| D18 | 11.1 µs | 3.56 ms (3.2e+02×) | 9.58 µs (0.87×) |
| D38 | 18.9 µs | 3.56 ms (1.9e+02×) | 9.55 µs (0.5×) |
| D57 | 8.8 µs | 3.23 ms (3.7e+02×) | 9 µs (1×) |
| D76 | 9.77 µs | 3.22 ms (3.3e+02×) | 9.02 µs (0.92×) |
| D115 | 20.6 µs | 9.26 ms (4.5e+02×) | 10.1 µs (0.49×) |
| D153 | 29.5 µs | 10.6 ms (3.6e+02×) | 13 µs (0.44×) |
| D230 | 22.9 µs | 5.87 ms (2.6e+02×) | 9.76 µs (0.43×) |
| D307 | 55 µs | 10.4 ms (1.9e+02×) | 13 µs (0.24×) |
| D462 | 60 µs | 10.4 ms (1.7e+02×) | 13 µs (0.22×) |
| D616 | 96.1 µs | 10.4 ms (1.1e+02×) | 13.1 µs (0.14×) |
| D924 | 153 µs | 10.5 ms (68×) | 12.6 µs (0.082×) |
| D1232 | 210 µs | 10 ms (48×) | 13.5 µs (0.064×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="216.8" x2="450" y2="216.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="219.8" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="173.6" x2="450" y2="173.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="176.6" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="130.4" x2="450" y2="130.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.4" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="87.2" x2="450" y2="87.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="90.2" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,214.9 88.2,204.8 124.4,219.2 160.5,217.2 196.7,203.2 232.9,196.5 269.1,201.3 305.3,184.8 341.5,183.2 377.6,174.4 413.8,165.6 450.0,159.7" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,106.6 88.2,106.6 124.4,108.4 160.5,108.4 196.7,88.6 232.9,86.1 269.1,97.2 305.3,86.4 341.5,86.4 377.6,86.4 413.8,86.3 450.0,87.2" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,217.6 88.2,217.7 124.4,218.8 160.5,218.7 196.7,216.6 232.9,211.9 269.1,217.2 305.3,211.9 341.5,211.9 377.6,211.7 413.8,212.5 450.0,211.1" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#B5663C"/><text x="222" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum |
| :-- | --: | --: |
| D18 | 11.4 µs | 3.56 ms (3.1e+02×) |
| D38 | 17.4 µs | 3.56 ms (2e+02×) |
| D57 | 8.82 µs | 3.24 ms (3.7e+02×) |
| D76 | 9.92 µs | 3.23 ms (3.3e+02×) |
| D115 | 20.8 µs | 8.33 ms (4e+02×) |
| D153 | 29.2 µs | 12 ms (4.1e+02×) |
| D230 | 23 µs | 6.02 ms (2.6e+02×) |
| D307 | 54.7 µs | 10.4 ms (1.9e+02×) |
| D462 | 60 µs | 10.4 ms (1.7e+02×) |
| D616 | 96.7 µs | 10.3 ms (1.1e+02×) |
| D924 | 151 µs | 13.2 ms (87×) |
| D1232 | 209 µs | 10 ms (48×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="216.8" x2="450" y2="216.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="219.8" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="173.6" x2="450" y2="173.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="176.6" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="130.4" x2="450" y2="130.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.4" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="87.2" x2="450" y2="87.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="90.2" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,214.3 88.2,206.4 124.4,219.2 160.5,216.9 196.7,203.0 232.9,196.7 269.1,201.2 305.3,184.9 341.5,183.2 377.6,174.2 413.8,165.8 450.0,159.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,106.6 88.2,106.6 124.4,108.4 160.5,108.4 196.7,90.6 232.9,83.8 269.1,96.7 305.3,86.4 341.5,86.4 377.6,86.6 413.8,81.9 450.0,87.1" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | decimal-rs | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: | --: |
| D18 | 481 ns | 15.1 µs (31×) | 5.35 ms (1.1e+04×) | 363 µs (7.5e+02×) | 12.7 µs (26×) |
| D38 | 19.9 µs | 15.1 µs (0.76×) | 5.35 ms (2.7e+02×) | 360 µs (18×) | 12.7 µs (0.64×) |
| D57 | 15.1 µs | 13.8 µs (0.92×) | 4.86 ms (3.2e+02×) | 371 µs (25×) | 12.1 µs (0.8×) |
| D76 | 16.1 µs | 13.8 µs (0.86×) | 4.84 ms (3e+02×) | 368 µs (23×) | 11.9 µs (0.74×) |
| D115 | 35.5 µs | 14.8 µs (0.42×) | 11.2 ms (3.1e+02×) | 561 µs (16×) | 13.1 µs (0.37×) |
| D153 | 51.1 µs | 19.1 µs (0.37×) | 15.7 ms (3.1e+02×) | 713 µs (14×) | 17.1 µs (0.33×) |
| D230 | 42.5 µs | 12.3 µs (0.29×) | 10.2 ms (2.4e+02×) | 298 µs (7×) | 10.7 µs (0.25×) |
| D307 | 97.3 µs | 19.2 µs (0.2×) | 18.5 ms (1.9e+02×) | 774 µs (8×) | 16.7 µs (0.17×) |
| D462 | 121 µs | 19.3 µs (0.16×) | 18 ms (1.5e+02×) | 737 µs (6.1×) | 17 µs (0.14×) |
| D616 | 227 µs | 19.3 µs (0.085×) | 17.8 ms (79×) | 750 µs (3.3×) | 17 µs (0.075×) |
| D924 | 401 µs | 19 µs (0.047×) | 18.5 ms (46×) | 729 µs (1.8×) | 16.8 µs (0.042×) |
| D1232 | 678 µs | 17.4 µs (0.026×) | 13 ms (19×) | 450 µs (0.66×) | 17.6 µs (0.026×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,235.4 88.2,177.2 124.4,181.6 160.5,180.6 196.7,168.2 232.9,162.5 269.1,165.4 305.3,152.4 341.5,149.0 377.6,139.2 413.8,130.3 450.0,122.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,181.6 88.2,181.5 124.4,182.9 160.5,183.0 196.7,181.9 232.9,177.9 269.1,184.8 305.3,177.8 341.5,177.7 377.6,177.8 413.8,177.9 450.0,179.4" fill="none" stroke="#367594" stroke-width="1.6"/><polyline points="52.0,89.8 88.2,89.8 124.4,91.3 160.5,91.3 196.7,78.3 232.9,73.0 269.1,79.7 305.3,70.4 341.5,70.9 377.6,71.0 413.8,70.4 450.0,76.0" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,131.9 88.2,132.0 124.4,131.5 160.5,131.6 196.7,125.0 232.9,121.3 269.1,134.9 305.3,120.0 341.5,120.8 377.6,120.5 413.8,120.9 450.0,128.5" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,184.2 88.2,184.3 124.4,185.1 160.5,185.2 196.7,183.8 232.9,179.6 269.1,186.9 305.3,179.9 341.5,179.7 377.6,179.7 413.8,179.9 450.0,179.1" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#367594"/><text x="164" y="20" font-size="8.5" fill="currentColor">decimal-rs</text><rect x="228" y="12" width="9" height="9" fill="#9C5BA6"/><text x="240" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="286" y="12" width="9" height="9" fill="#5E8C3A"/><text x="298" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="338" y="12" width="9" height="9" fill="#B5663C"/><text x="350" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | bigdecimal | decimal-rs | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: | --: | --: |
| D18 | 250 ns | 71.4 µs (2.9e+02×) | 5.21 µs (21×) | 146 µs (5.8e+02×) | 139 µs (5.5e+02×) | 2.4 µs (9.6×) |
| D38 | 3.06 µs | 71.1 µs (23×) | 5.17 µs (1.7×) | 144 µs (47×) | 138 µs (45×) | 2.4 µs (0.78×) |
| D57 | 1.13 µs | 65.7 µs (58×) | 4.64 µs (4.1×) | 131 µs (1.2e+02×) | 127 µs (1.1e+02×) | 2.18 µs (1.9×) |
| D76 | 1.4 µs | 65.6 µs (47×) | 4.68 µs (3.3×) | 131 µs (93×) | 129 µs (92×) | 2.23 µs (1.6×) |
| D115 | 1.75 µs | 89.5 µs (51×) | 4.97 µs (2.8×) | 144 µs (82×) | 210 µs (1.2e+02×) | 2.24 µs (1.3×) |
| D153 | 4.25 µs | 115 µs (27×) | 6.34 µs (1.5×) | 190 µs (45×) | 270 µs (63×) | 2.9 µs (0.68×) |
| D230 | 3.89 µs | 69.4 µs (18×) | 4.29 µs (1.1×) | 120 µs (31×) | 98.8 µs (25×) | 1.78 µs (0.46×) |
| D307 | 7.34 µs | 116 µs (16×) | 6.4 µs (0.87×) | 187 µs (25×) | 271 µs (37×) | 2.91 µs (0.4×) |
| D462 | 12.6 µs | 116 µs (9.2×) | 6.43 µs (0.51×) | 185 µs (15×) | 271 µs (21×) | 2.92 µs (0.23×) |
| D616 | 17.9 µs | 116 µs (6.5×) | 6.41 µs (0.36×) | 188 µs (10×) | 277 µs (15×) | 2.9 µs (0.16×) |
| D924 | 31.3 µs | 116 µs (3.7×) | 6.43 µs (0.21×) | 187 µs (6×) | 270 µs (8.6×) | 2.93 µs (0.094×) |
| D1232 | 46.8 µs | 106 µs (2.3×) | 5.72 µs (0.12×) | 182 µs (3.9×) | 173 µs (3.7×) | 3.16 µs (0.067×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="206.0" x2="450" y2="206.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="209.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="98.0" x2="450" y2="98.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="101.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,238.5 88.2,179.7 124.4,203.1 160.5,198.1 196.7,192.8 232.9,172.1 269.1,174.1 305.3,159.2 341.5,146.6 377.6,138.3 413.8,125.2 450.0,115.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,106.0 124.4,107.9 160.5,107.9 196.7,100.6 232.9,94.7 269.1,106.6 305.3,94.5 341.5,94.5 377.6,94.5 413.8,94.6 450.0,96.6" fill="none" stroke="#C68A2E" stroke-width="1.6"/><polyline points="52.0,167.3 88.2,167.5 124.4,170.0 160.5,169.8 196.7,168.4 232.9,162.7 269.1,171.8 305.3,162.5 341.5,162.3 377.6,162.4 413.8,162.3 450.0,165.1" fill="none" stroke="#367594" stroke-width="1.6"/><polyline points="52.0,89.2 88.2,89.5 124.4,91.6 160.5,91.7 196.7,89.5 232.9,82.9 269.1,93.7 305.3,83.3 341.5,83.6 377.6,83.2 413.8,83.3 450.0,84.0" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,90.3 88.2,90.4 124.4,92.5 160.5,92.1 196.7,80.6 232.9,74.7 269.1,98.3 305.3,74.6 341.5,74.7 377.6,74.1 413.8,74.7 450.0,85.1" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,185.5 88.2,185.4 124.4,187.7 160.5,187.2 196.7,187.1 232.9,181.0 269.1,192.4 305.3,180.9 341.5,180.9 377.6,181.0 413.8,180.8 450.0,179.0" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#C68A2E"/><text x="164" y="20" font-size="8.5" fill="currentColor">bigdecimal</text><rect x="228" y="12" width="9" height="9" fill="#367594"/><text x="240" y="20" font-size="8.5" fill="currentColor">decimal-rs</text><rect x="304" y="12" width="9" height="9" fill="#9C5BA6"/><text x="316" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="362" y="12" width="9" height="9" fill="#5E8C3A"/><text x="374" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="52" y="25" width="9" height="9" fill="#B5663C"/><text x="64" y="33" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>
<!-- END GENERATED:comparisons:body:roots -->
