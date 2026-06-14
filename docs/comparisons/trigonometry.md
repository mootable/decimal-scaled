# Comparisons — Trigonometry

Speed of `decimal-scaled` against the peer decimal crates on the trigonometric and
hyperbolic functions. See the [Comparisons overview](../comparisons.md) for the time
units, the per-library precision model, and how to read the timings.

<!-- BEGIN GENERATED:comparisons:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 69 µs | 510 µs (7.4×) |
| D38 | 128 µs | 506 µs (3.9×) |
| D57 | 15.6 µs | 495 µs (32×) |
| D76 | 16.4 µs | 499 µs (30×) |
| D115 | 17 µs | 782 µs (46×) |
| D153 | 28.5 µs | 1.08 ms (38×) |
| D230 | 17.5 µs | 404 µs (23×) |
| D307 | 31.5 µs | 1.03 ms (33×) |
| D462 | 35.9 µs | 1.04 ms (29×) |
| D616 | 39.6 µs | 1.05 ms (26×) |
| D924 | 49.6 µs | 1.06 ms (21×) |
| D1232 | 68.1 µs | 631 µs (9.3×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,199.6 88.2,180.2 124.4,246.1 160.5,244.6 196.7,243.5 232.9,227.2 269.1,242.6 305.3,224.2 341.5,220.1 377.6,216.9 413.8,209.9 450.0,200.0" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,137.1 88.2,137.3 124.4,138.0 160.5,137.7 196.7,123.7 232.9,113.6 269.1,144.4 305.3,114.9 341.5,114.7 377.6,114.5 413.8,114.2 450.0,130.4" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 43.2 µs | 267 µs (6.2×) |
| D38 | 62.9 µs | 267 µs (4.3×) |
| D57 | 55.9 µs | 261 µs (4.7×) |
| D76 | 58.8 µs | 263 µs (4.5×) |
| D115 | 97.6 µs | 414 µs (4.2×) |
| D153 | 155 µs | 543 µs (3.5×) |
| D230 | 82.5 µs | 211 µs (2.6×) |
| D307 | 141 µs | 541 µs (3.8×) |
| D462 | 145 µs | 536 µs (3.7×) |
| D616 | 148 µs | 546 µs (3.7×) |
| D924 | 161 µs | 536 µs (3.3×) |
| D1232 | 182 µs | 334 µs (1.8×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,191.4 88.2,173.8 124.4,179.3 160.5,176.9 196.7,153.1 232.9,131.6 269.1,161.0 305.3,135.8 341.5,134.5 377.6,133.7 413.8,129.5 450.0,124.0" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,105.9 124.4,106.9 160.5,106.6 196.7,85.4 232.9,72.6 269.1,117.0 305.3,72.8 341.5,73.2 377.6,72.4 413.8,73.2 450.0,95.5" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 70.4 µs | 511 µs (7.3×) |
| D38 | 129 µs | 509 µs (3.9×) |
| D57 | 15.4 µs | 491 µs (32×) |
| D76 | 16.2 µs | 497 µs (31×) |
| D115 | 17.2 µs | 781 µs (45×) |
| D153 | 28.3 µs | 1.01 ms (36×) |
| D230 | 17.3 µs | 401 µs (23×) |
| D307 | 31.1 µs | 1.03 ms (33×) |
| D462 | 35.5 µs | 1.08 ms (30×) |
| D616 | 39.5 µs | 1.04 ms (26×) |
| D924 | 51.4 µs | 1.01 ms (20×) |
| D1232 | 68.9 µs | 625 µs (9.1×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,199.0 88.2,180.0 124.4,246.4 160.5,244.9 196.7,243.1 232.9,227.4 269.1,242.8 305.3,224.5 341.5,220.4 377.6,217.0 413.8,208.8 450.0,199.7" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,137.0 88.2,137.1 124.4,138.2 160.5,137.9 196.7,123.7 232.9,115.6 269.1,144.6 305.3,115.1 341.5,113.7 377.6,114.9 413.8,115.6 450.0,130.7" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 37 µs | 301 µs (8.1×) |
| D38 | 54.1 µs | 303 µs (5.6×) |
| D57 | 13.7 µs | 297 µs (22×) |
| D76 | 12.7 µs | 297 µs (23×) |
| D115 | 19.9 µs | 469 µs (24×) |
| D153 | 29.8 µs | 597 µs (20×) |
| D230 | 17.7 µs | 239 µs (14×) |
| D307 | 32.9 µs | 608 µs (18×) |
| D462 | 38.8 µs | 600 µs (15×) |
| D616 | 44.1 µs | 614 µs (14×) |
| D924 | 58 µs | 595 µs (10×) |
| D1232 | 71.4 µs | 369 µs (5.2×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,198.6 88.2,180.8 124.4,245.2 160.5,248.9 196.7,227.8 232.9,208.8 269.1,233.3 305.3,204.1 341.5,196.4 377.6,190.4 413.8,177.5 450.0,167.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,100.3 88.2,100.0 124.4,100.9 160.5,101.0 196.7,79.5 232.9,68.2 269.1,111.1 305.3,67.4 341.5,67.9 377.6,66.8 413.8,68.4 450.0,90.8" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 6.43 µs | 245 µs (38×) |
| D38 | 49.3 µs | 245 µs (5×) |
| D57 | 13.2 µs | 257 µs (19×) |
| D76 | 13.3 µs | 256 µs (19×) |
| D115 | 13.9 µs | 379 µs (27×) |
| D153 | 25.7 µs | 487 µs (19×) |
| D230 | 15.8 µs | 186 µs (12×) |
| D307 | 28.8 µs | 490 µs (17×) |
| D462 | 27.7 µs | 487 µs (18×) |
| D616 | 39.4 µs | 489 µs (12×) |
| D924 | 53.7 µs | 483 µs (9×) |
| D1232 | 69.6 µs | 286 µs (4.1×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,201.8 88.2,138.1 124.4,179.3 160.5,179.0 196.7,177.7 232.9,158.5 269.1,173.6 305.3,154.9 341.5,156.2 377.6,145.1 413.8,135.4 450.0,127.3" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,88.0 88.2,88.0 124.4,86.5 160.5,86.6 196.7,74.3 232.9,66.5 269.1,96.6 305.3,66.3 341.5,66.5 377.6,66.4 413.8,66.8 450.0,83.2" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 34.7 µs | 399 µs (12×) |
| D38 | 89.2 µs | 403 µs (4.5×) |
| D57 | 14.2 µs | 398 µs (28×) |
| D76 | 14.9 µs | 402 µs (27×) |
| D115 | 16.1 µs | 623 µs (39×) |
| D153 | 27 µs | 773 µs (29×) |
| D230 | 17.5 µs | 321 µs (18×) |
| D307 | 32.9 µs | 798 µs (24×) |
| D462 | 41 µs | 799 µs (19×) |
| D616 | 49 µs | 818 µs (17×) |
| D924 | 72.9 µs | 921 µs (13×) |
| D1232 | 98.2 µs | 504 µs (5.1×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,201.6 88.2,157.4 124.4,243.7 160.5,241.3 196.7,237.8 232.9,213.4 269.1,233.8 305.3,204.1 341.5,193.8 377.6,185.5 413.8,166.8 450.0,152.9" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,87.1 88.2,86.6 124.4,87.2 160.5,86.7 196.7,66.2 232.9,56.1 269.1,97.3 305.3,54.6 341.5,54.5 377.6,53.4 413.8,47.8 450.0,76.1" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 10.7 µs | 193 µs (18×) |
| D38 | 15.6 µs | 191 µs (12×) |
| D57 | 52 µs | 206 µs (4×) |
| D76 | 54 µs | 201 µs (3.7×) |
| D115 | 88.7 µs | 292 µs (3.3×) |
| D153 | 138 µs | 389 µs (2.8×) |
| D230 | 73.1 µs | 168 µs (2.3×) |
| D307 | 127 µs | 401 µs (3.1×) |
| D462 | 130 µs | 390 µs (3×) |
| D616 | 136 µs | 391 µs (2.9×) |
| D924 | 148 µs | 390 µs (2.6×) |
| D1232 | 172 µs | 248 µs (1.4×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,257.0 88.2,239.3 124.4,182.7 160.5,180.9 196.7,157.6 232.9,137.0 269.1,166.7 305.3,140.6 341.5,139.7 377.6,137.7 413.8,133.5 450.0,126.6" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,121.2 88.2,121.5 124.4,118.1 160.5,119.3 196.7,101.7 232.9,88.3 269.1,127.6 305.3,86.8 341.5,88.1 377.6,88.0 413.8,88.2 450.0,109.4" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 6.53 µs | 1.13 ms (1.7e+02×) | 214 µs (33×) | 3.46 µs (0.53×) |
| D38 | 9.61 µs | 1.08 ms (1.1e+02×) | 214 µs (22×) | 3.44 µs (0.36×) |
| D57 | 6.04 µs | 1e+03 µs (1.7e+02×) | 228 µs (38×) | 3.27 µs (0.54×) |
| D76 | 7.02 µs | 988 µs (1.4e+02×) | 225 µs (32×) | 3.27 µs (0.47×) |
| D115 | 8.3 µs | 1.07 ms (1.3e+02×) | 325 µs (39×) | 3.67 µs (0.44×) |
| D153 | 12.9 µs | 1.52 ms (1.2e+02×) | 421 µs (33×) | 4.67 µs (0.36×) |
| D230 | 8.59 µs | 881 µs (1e+02×) | 192 µs (22×) | 2.81 µs (0.33×) |
| D307 | 15.8 µs | 1.42 ms (90×) | 435 µs (28×) | 4.68 µs (0.3×) |
| D462 | 18.1 µs | 1.44 ms (79×) | 421 µs (23×) | 4.67 µs (0.26×) |
| D616 | 25.1 µs | 2.46 ms (98×) | 436 µs (17×) | 4.63 µs (0.18×) |
| D924 | 36.2 µs | 1.41 ms (39×) | 417 µs (12×) | 4.61 µs (0.13×) |
| D1232 | 53.7 µs | 1.24 ms (23×) | 275 µs (5.1×) | 4.84 µs (0.09×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="206.0" x2="450" y2="206.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="209.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="98.0" x2="450" y2="98.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="101.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,216.0 88.2,206.9 124.4,217.8 160.5,214.3 196.7,210.4 232.9,200.0 269.1,209.6 305.3,195.3 341.5,192.0 377.6,184.4 413.8,175.8 450.0,166.6" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,95.1 88.2,96.2 124.4,98.0 160.5,98.3 196.7,96.3 232.9,88.2 269.1,101.0 305.3,89.8 341.5,89.5 377.6,76.9 413.8,89.9 450.0,93.0" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,134.2 88.2,134.2 124.4,132.7 160.5,133.0 196.7,124.3 232.9,118.3 269.1,136.7 305.3,117.5 341.5,118.3 377.6,117.5 413.8,118.5 450.0,128.2" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,230.9 88.2,231.0 124.4,232.2 160.5,232.2 196.7,229.5 232.9,223.8 269.1,235.7 305.3,223.8 341.5,223.9 377.6,224.0 413.8,224.1 450.0,223.0" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 10.7 µs | 317 µs (29×) |
| D38 | 15.1 µs | 315 µs (21×) |
| D57 | 9.61 µs | 297 µs (31×) |
| D76 | 11.3 µs | 299 µs (27×) |
| D115 | 23.3 µs | 480 µs (21×) |
| D153 | 21.6 µs | 614 µs (28×) |
| D230 | 13.7 µs | 235 µs (17×) |
| D307 | 26.9 µs | 623 µs (23×) |
| D462 | 54.8 µs | 615 µs (11×) |
| D616 | 95.9 µs | 627 µs (6.5×) |
| D924 | 148 µs | 611 µs (4.1×) |
| D1232 | 211 µs | 363 µs (1.7×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,185.8 88.2,175.1 124.4,189.3 160.5,184.3 196.7,161.6 232.9,163.9 269.1,178.2 305.3,157.0 341.5,134.8 377.6,117.3 413.8,103.8 450.0,92.6" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,80.0 88.2,80.1 124.4,81.9 160.5,81.7 196.7,67.0 232.9,59.3 269.1,89.3 305.3,58.8 341.5,59.2 377.6,58.6 413.8,59.4 450.0,75.7" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 6.67 µs | 450 µs (67×) | 207 µs (31×) | 3.37 µs (0.5×) |
| D38 | 9.63 µs | 444 µs (46×) | 206 µs (21×) | 3.37 µs (0.35×) |
| D57 | 5.99 µs | 411 µs (69×) | 219 µs (37×) | 3.24 µs (0.54×) |
| D76 | 6.96 µs | 407 µs (58×) | 219 µs (31×) | 3.25 µs (0.47×) |
| D115 | 8.19 µs | 451 µs (55×) | 322 µs (39×) | 3.6 µs (0.44×) |
| D153 | 12.7 µs | 562 µs (44×) | 410 µs (32×) | 4.64 µs (0.37×) |
| D230 | 8.21 µs | 381 µs (46×) | 187 µs (23×) | 3.39 µs (0.41×) |
| D307 | 15.3 µs | 561 µs (37×) | 422 µs (28×) | 4.62 µs (0.3×) |
| D462 | 17.9 µs | 563 µs (31×) | 406 µs (23×) | 4.64 µs (0.26×) |
| D616 | 23.6 µs | 606 µs (26×) | 424 µs (18×) | 4.6 µs (0.19×) |
| D924 | 37.4 µs | 574 µs (15×) | 405 µs (11×) | 4.52 µs (0.12×) |
| D1232 | 52.8 µs | 547 µs (10×) | 268 µs (5.1×) | 4.81 µs (0.091×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,200.7 88.2,189.2 124.4,204.0 160.5,199.3 196.7,194.2 232.9,180.5 269.1,194.2 305.3,174.6 341.5,169.7 377.6,161.2 413.8,146.7 450.0,136.0" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,69.0 88.2,69.4 124.4,71.8 160.5,72.1 196.7,68.9 232.9,62.0 269.1,74.2 305.3,62.1 341.5,61.9 377.6,59.7 413.8,61.4 450.0,62.9" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,93.3 88.2,93.4 124.4,91.4 160.5,91.5 196.7,79.4 232.9,71.9 269.1,96.5 305.3,70.9 341.5,72.2 377.6,70.8 413.8,72.2 450.0,85.2" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,222.1 88.2,222.1 124.4,223.3 160.5,223.2 196.7,219.9 232.9,212.0 269.1,221.8 305.3,212.2 341.5,212.0 377.6,212.3 413.8,212.9 450.0,210.9" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 10.8 µs | 318 µs (30×) |
| D38 | 15.1 µs | 317 µs (21×) |
| D57 | 9.72 µs | 298 µs (31×) |
| D76 | 11.2 µs | 300 µs (27×) |
| D115 | 21.7 µs | 483 µs (22×) |
| D153 | 21.4 µs | 620 µs (29×) |
| D230 | 24.9 µs | 235 µs (9.4×) |
| D307 | 54.2 µs | 629 µs (12×) |
| D462 | 60.5 µs | 614 µs (10×) |
| D616 | 102 µs | 631 µs (6.2×) |
| D924 | 158 µs | 615 µs (3.9×) |
| D1232 | 220 µs | 365 µs (1.7×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,185.7 88.2,175.0 124.4,188.9 160.5,184.5 196.7,163.8 232.9,164.2 269.1,159.4 305.3,135.1 341.5,131.7 377.6,115.3 413.8,101.7 450.0,91.4" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,79.8 88.2,80.0 124.4,81.9 160.5,81.6 196.7,66.8 232.9,59.0 269.1,89.2 305.3,58.5 341.5,59.3 377.6,58.4 413.8,59.2 450.0,75.5" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 12.2 µs | 1.48 ms (1.2e+02×) | 225 µs (18×) | 4.56 µs (0.37×) |
| D38 | 17.6 µs | 1.48 ms (84×) | 224 µs (13×) | 4.53 µs (0.26×) |
| D57 | 7.86 µs | 1.36 ms (1.7e+02×) | 242 µs (31×) | 3.84 µs (0.49×) |
| D76 | 8.73 µs | 1.35 ms (1.5e+02×) | 243 µs (28×) | 3.85 µs (0.44×) |
| D115 | 10.1 µs | 1.88 ms (1.9e+02×) | 344 µs (34×) | 3.65 µs (0.36×) |
| D153 | 15.6 µs | 2.02 ms (1.3e+02×) | 446 µs (29×) | 4.7 µs (0.3×) |
| D230 | 10.2 µs | 1.62 ms (1.6e+02×) | 201 µs (20×) | 3.32 µs (0.33×) |
| D307 | 18.4 µs | 4.58 ms (2.5e+02×) | 467 µs (25×) | 4.69 µs (0.26×) |
| D462 | 20 µs | 4.49 ms (2.2e+02×) | 449 µs (22×) | 4.75 µs (0.24×) |
| D616 | 27.7 µs | 4.63 ms (1.7e+02×) | 467 µs (17×) | 4.69 µs (0.17×) |
| D924 | 38.9 µs | 3.78 ms (97×) | 447 µs (11×) | 4.71 µs (0.12×) |
| D1232 | 56.7 µs | 2.47 ms (44×) | 291 µs (5.1×) | 4.73 µs (0.083×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="206.0" x2="450" y2="206.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="209.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="98.0" x2="450" y2="98.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="101.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,201.3 88.2,192.8 124.4,211.6 160.5,209.2 196.7,205.7 232.9,195.6 269.1,205.5 305.3,191.7 341.5,189.8 377.6,182.1 413.8,174.2 450.0,165.3" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,88.8 88.2,88.8 124.4,90.9 160.5,91.0 196.7,83.1 232.9,81.5 269.1,86.7 305.3,62.3 341.5,62.8 377.6,62.1 413.8,66.8 450.0,76.8" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,133.1 124.4,131.3 160.5,131.2 196.7,123.0 232.9,116.9 269.1,135.6 305.3,115.9 341.5,116.8 377.6,115.9 413.8,116.9 450.0,126.9" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,224.4 88.2,224.6 124.4,228.5 160.5,228.4 196.7,229.7 232.9,223.7 269.1,231.8 305.3,223.8 341.5,223.5 377.6,223.8 413.8,223.7 450.0,223.6" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 10.6 µs | 188 µs (18×) |
| D38 | 15.1 µs | 188 µs (12×) |
| D57 | 10.1 µs | 180 µs (18×) |
| D76 | 11.2 µs | 179 µs (16×) |
| D115 | 21.7 µs | 284 µs (13×) |
| D153 | 21.3 µs | 370 µs (17×) |
| D230 | 13 µs | 141 µs (11×) |
| D307 | 24.2 µs | 373 µs (15×) |
| D462 | 29.5 µs | 366 µs (12×) |
| D616 | 34.5 µs | 371 µs (11×) |
| D924 | 48.4 µs | 368 µs (7.6×) |
| D1232 | 65.3 µs | 221 µs (3.4×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polyline points="52.0,257.1 88.2,240.6 124.4,259.7 160.5,254.7 196.7,223.6 232.9,224.5 269.1,247.7 305.3,218.6 341.5,209.3 377.6,201.9 413.8,186.0 450.0,172.0" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,122.3 124.4,124.6 160.5,124.8 196.7,103.0 232.9,90.6 269.1,135.8 305.3,90.3 341.5,91.2 377.6,90.5 413.8,90.9 450.0,114.9" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>
<!-- END GENERATED:comparisons:body:trig -->
