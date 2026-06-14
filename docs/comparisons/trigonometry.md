# Comparisons — Trigonometry

Speed of `decimal-scaled` against the peer decimal crates on the trigonometric and
hyperbolic functions. See the [Comparisons overview](../comparisons.md) for the time
units, the per-library precision model, and how to read the timings.

<!-- BEGIN GENERATED:comparisons:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 54.1 µs | 497 µs (9.2×) |
| D38 | 85.6 µs | 498 µs (5.8×) |
| D57 | 15.6 µs | 492 µs (32×) |
| D76 | 17.8 µs | 508 µs (29×) |
| D115 | 13 µs | 404 µs (31×) |
| D153 | 28.7 µs | 1.08 ms (38×) |
| D230 | 30.2 µs | 995 µs (33×) |
| D307 | 31.8 µs | 1.02 ms (32×) |
| D462 | 35.8 µs | 1.03 ms (29×) |
| D616 | 40.4 µs | 1.1 ms (27×) |
| D924 | 51.4 µs | 1.08 ms (21×) |
| D1232 | 68.8 µs | 1.1 ms (16×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.4 88.2,139.5 124.4,163.5 160.5,161.5 196.7,100.1 232.9,87.9 269.1,98.6 305.3,87.8 341.5,87.8 377.6,81.5 413.8,85.4 450.0,87.8 450.0,163.0 413.8,167.5 377.6,170.9 341.5,177.4 305.3,176.7 269.1,177.8 232.9,178.5 196.7,192.4 160.5,185.4 124.4,187.3 88.2,237.9 52.0,248.3" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,120.0 88.2,119.2 124.4,120.9 160.5,118.7 196.7,76.6 232.9,73.1 269.1,74.8 305.3,73.0 341.5,77.1 377.6,79.3 413.8,75.6 450.0,80.8 450.0,196.4 413.8,196.5 377.6,196.6 341.5,196.3 305.3,196.4 269.1,196.3 232.9,196.6 196.7,211.5 160.5,206.1 124.4,205.9 88.2,206.1 52.0,205.9" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,161.6 88.2,154.4 124.4,181.0 160.5,179.0 196.7,183.9 232.9,171.5 269.1,170.7 305.3,169.9 341.5,168.1 377.6,166.2 413.8,162.4 450.0,157.8" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,126.9 88.2,126.9 124.4,127.1 160.5,126.6 196.7,130.2 232.9,114.8 269.1,116.1 305.3,115.7 341.5,115.6 377.6,114.5 413.8,114.7 450.0,114.6" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 39.8 µs | 275 µs (6.9×) |
| D38 | 58.9 µs | 274 µs (4.6×) |
| D57 | 56 µs | 272 µs (4.8×) |
| D76 | 63.3 µs | 281 µs (4.4×) |
| D115 | 72.4 µs | 225 µs (3.1×) |
| D153 | 155 µs | 597 µs (3.9×) |
| D230 | 140 µs | 559 µs (4×) |
| D307 | 141 µs | 573 µs (4.1×) |
| D462 | 144 µs | 563 µs (3.9×) |
| D616 | 149 µs | 597 µs (4×) |
| D924 | 163 µs | 586 µs (3.6×) |
| D1232 | 177 µs | 587 µs (3.3×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.9 88.2,156.5 124.4,152.5 160.5,155.2 196.7,87.8 232.9,81.3 269.1,83.2 305.3,87.6 341.5,87.5 377.6,81.4 413.8,81.4 450.0,87.4 450.0,163.3 413.8,170.0 377.6,178.1 341.5,178.9 305.3,175.5 269.1,185.6 232.9,190.2 196.7,205.2 160.5,201.9 124.4,205.1 88.2,235.8 52.0,236.8" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,125.9 88.2,123.5 124.4,125.9 160.5,125.7 196.7,76.6 232.9,80.9 269.1,76.7 305.3,82.9 341.5,76.0 377.6,74.4 413.8,82.0 450.0,80.1 450.0,223.2 413.8,218.0 377.6,222.4 341.5,221.6 305.3,221.5 269.1,220.9 232.9,225.0 196.7,226.4 160.5,227.3 124.4,230.4 88.2,231.2 52.0,230.7" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,166.4 88.2,160.3 124.4,161.1 160.5,159.1 196.7,157.1 232.9,145.2 269.1,146.8 305.3,146.7 341.5,146.3 377.6,145.8 413.8,144.4 450.0,143.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,136.2 88.2,136.2 124.4,136.4 160.5,135.8 196.7,139.3 232.9,124.1 269.1,125.1 305.3,124.7 341.5,125.0 377.6,124.1 413.8,124.4 450.0,124.3" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 54.1 µs | 498 µs (9.2×) |
| D38 | 85.6 µs | 497 µs (5.8×) |
| D57 | 15.5 µs | 490 µs (32×) |
| D76 | 17.5 µs | 507 µs (29×) |
| D115 | 12.9 µs | 405 µs (31×) |
| D153 | 28.3 µs | 1.09 ms (39×) |
| D230 | 30.1 µs | 979 µs (33×) |
| D307 | 31.7 µs | 998 µs (31×) |
| D462 | 35.7 µs | 1.01 ms (28×) |
| D616 | 39.3 µs | 1.05 ms (27×) |
| D924 | 50 µs | 1.08 ms (22×) |
| D1232 | 63.5 µs | 1.07 ms (17×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.0 88.2,142.6 124.4,163.9 160.5,162.2 196.7,105.0 232.9,83.4 269.1,94.4 305.3,87.8 341.5,87.8 377.6,83.4 413.8,81.5 450.0,87.8 450.0,164.9 413.8,172.2 377.6,183.5 341.5,189.1 305.3,197.2 269.1,201.3 232.9,206.4 196.7,219.1 160.5,219.2 124.4,222.2 88.2,233.3 52.0,245.0" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,122.0 88.2,116.4 124.4,123.0 160.5,122.2 196.7,73.3 232.9,78.9 269.1,75.9 305.3,76.6 341.5,70.8 377.6,69.9 413.8,84.9 450.0,75.6 450.0,239.1 413.8,240.0 377.6,238.3 341.5,240.4 305.3,240.8 269.1,239.5 232.9,240.0 196.7,245.9 160.5,243.9 124.4,245.7 88.2,243.9 52.0,243.9" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,161.6 88.2,154.4 124.4,181.1 160.5,179.2 196.7,184.0 232.9,171.7 269.1,170.8 305.3,170.0 341.5,168.1 377.6,166.6 413.8,162.8 450.0,159.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,126.9 88.2,126.9 124.4,127.1 160.5,126.6 196.7,130.1 232.9,114.6 269.1,116.3 305.3,116.0 341.5,115.8 377.6,115.2 413.8,114.8 450.0,114.9" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 34.8 µs | 301 µs (8.6×) |
| D38 | 50.9 µs | 299 µs (5.9×) |
| D57 | 13.5 µs | 299 µs (22×) |
| D76 | 14.7 µs | 304 µs (21×) |
| D115 | 14.7 µs | 246 µs (17×) |
| D153 | 29.1 µs | 607 µs (21×) |
| D230 | 30.4 µs | 584 µs (19×) |
| D307 | 32.4 µs | 585 µs (18×) |
| D462 | 37 µs | 591 µs (16×) |
| D616 | 42 µs | 615 µs (15×) |
| D924 | 56.9 µs | 605 µs (11×) |
| D1232 | 74.8 µs | 613 µs (8.2×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,162.5 88.2,149.0 124.4,171.0 160.5,162.9 196.7,95.4 232.9,85.4 269.1,87.8 305.3,87.7 341.5,85.3 377.6,81.1 413.8,83.0 450.0,77.9 450.0,170.2 413.8,176.1 377.6,184.7 341.5,189.9 305.3,199.3 269.1,203.9 232.9,210.7 196.7,223.7 160.5,224.0 124.4,228.0 88.2,234.2 52.0,244.5" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,124.8 88.2,125.9 124.4,125.8 160.5,125.8 196.7,73.3 232.9,76.0 269.1,77.4 305.3,80.2 341.5,79.1 377.6,71.1 413.8,76.0 450.0,80.2 450.0,139.1 413.8,133.1 377.6,133.1 341.5,138.9 305.3,141.5 269.1,134.3 232.9,139.4 196.7,148.4 160.5,143.4 124.4,144.9 88.2,144.6 52.0,144.6" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,168.5 88.2,162.6 124.4,183.3 160.5,182.0 196.7,182.0 232.9,171.3 269.1,170.6 305.3,169.6 341.5,167.6 377.6,165.5 413.8,160.8 450.0,156.5" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,134.8 88.2,134.9 124.4,134.9 160.5,134.6 196.7,138.0 232.9,123.8 269.1,124.4 305.3,124.4 341.5,124.2 377.6,123.6 413.8,123.9 450.0,123.6" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 6.31 µs | 236 µs (37×) |
| D38 | 46.3 µs | 241 µs (5.2×) |
| D57 | 13.2 µs | 236 µs (18×) |
| D76 | 14.6 µs | 224 µs (15×) |
| D115 | 11.8 µs | 185 µs (16×) |
| D153 | 25.5 µs | 489 µs (19×) |
| D230 | 27.6 µs | 480 µs (17×) |
| D307 | 29.2 µs | 494 µs (17×) |
| D462 | 27.4 µs | 494 µs (18×) |
| D616 | 38.5 µs | 490 µs (13×) |
| D924 | 52.3 µs | 491 µs (9.4×) |
| D1232 | 71.2 µs | 486 µs (6.8×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,158.3 88.2,151.0 124.4,165.7 160.5,170.4 196.7,110.1 232.9,81.6 269.1,98.5 305.3,81.5 341.5,83.4 377.6,81.2 413.8,85.2 450.0,83.0 450.0,162.8 413.8,167.7 377.6,172.4 341.5,177.5 305.3,178.5 269.1,181.0 232.9,180.3 196.7,196.2 160.5,188.2 124.4,189.5 88.2,234.5 52.0,242.3" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,116.5 88.2,123.6 124.4,125.3 160.5,122.8 196.7,80.9 232.9,77.1 269.1,80.1 305.3,80.0 341.5,79.9 377.6,82.6 413.8,80.5 450.0,75.6 450.0,245.0 413.8,245.7 377.6,245.1 341.5,242.8 305.3,243.4 269.1,243.9 232.9,243.8 196.7,249.1 160.5,248.4 124.4,248.4 88.2,248.4 52.0,249.1" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,195.2 88.2,164.0 124.4,183.7 160.5,182.1 196.7,185.4 232.9,173.3 269.1,172.1 305.3,171.3 341.5,172.2 377.6,166.9 413.8,162.1 450.0,157.3" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,138.6 88.2,138.3 124.4,138.6 160.5,139.4 196.7,142.4 232.9,127.2 269.1,127.5 305.3,127.0 341.5,127.0 377.6,127.2 413.8,127.1 450.0,127.3" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atan2`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 54.2 µs | 396 µs (7.3×) |
| D38 | 84.6 µs | 396 µs (4.7×) |
| D57 | 14.1 µs | 392 µs (28×) |
| D76 | 16.1 µs | 399 µs (25×) |
| D115 | 12.3 µs | 326 µs (27×) |
| D153 | 26.9 µs | 937 µs (35×) |
| D230 | 30.7 µs | 881 µs (29×) |
| D307 | 33.2 µs | 888 µs (27×) |
| D462 | 41.1 µs | 890 µs (22×) |
| D616 | 49.4 µs | 961 µs (19×) |
| D924 | 68.8 µs | 944 µs (14×) |
| D1232 | 97.6 µs | 946 µs (9.7×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="216.8" x2="450" y2="216.8" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="219.8" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="173.6" x2="450" y2="173.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="176.6" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="130.4" x2="450" y2="130.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.4" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="87.2" x2="450" y2="87.2" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="90.2" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.5 88.2,166.7 124.4,189.0 160.5,186.1 196.7,100.8 232.9,96.6 269.1,96.6 305.3,96.6 341.5,96.6 377.6,83.7 413.8,96.5 450.0,96.4 450.0,178.7 413.8,190.7 377.6,195.2 341.5,196.0 305.3,203.5 269.1,201.7 232.9,204.8 196.7,218.3 160.5,214.5 124.4,215.0 88.2,231.7 52.0,235.9" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,140.1 88.2,136.6 124.4,139.2 160.5,129.2 196.7,79.0 232.9,78.6 269.1,78.4 305.3,87.1 341.5,87.0 377.6,78.3 413.8,79.0 450.0,82.1 450.0,169.9 413.8,170.0 377.6,169.4 341.5,170.6 305.3,168.5 269.1,170.6 232.9,168.3 196.7,189.0 160.5,181.8 124.4,183.3 88.2,183.2 52.0,183.2" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,185.1 88.2,176.7 124.4,210.4 160.5,207.8 196.7,212.9 232.9,198.2 269.1,195.7 305.3,194.3 341.5,190.3 377.6,186.8 413.8,180.6 450.0,174.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,147.8 88.2,147.8 124.4,148.0 160.5,147.6 196.7,151.4 232.9,131.6 269.1,132.8 305.3,132.6 341.5,132.6 377.6,131.1 413.8,131.5 450.0,131.4" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 9.76 µs | 204 µs (21×) |
| D38 | 14.7 µs | 203 µs (14×) |
| D57 | 51.8 µs | 206 µs (4×) |
| D76 | 58.5 µs | 194 µs (3.3×) |
| D115 | 65.2 µs | 173 µs (2.7×) |
| D153 | 140 µs | 402 µs (2.9×) |
| D230 | 124 µs | 407 µs (3.3×) |
| D307 | 126 µs | 401 µs (3.2×) |
| D462 | 130 µs | 397 µs (3×) |
| D616 | 136 µs | 407 µs (3×) |
| D924 | 149 µs | 393 µs (2.6×) |
| D1232 | 166 µs | 393 µs (2.4×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,173.3 88.2,171.5 124.4,154.2 160.5,150.7 196.7,87.7 232.9,87.6 269.1,87.6 305.3,77.0 341.5,74.6 377.6,87.6 413.8,85.2 450.0,87.5 450.0,163.9 413.8,170.7 377.6,182.4 341.5,184.5 305.3,191.3 269.1,192.6 232.9,209.3 196.7,204.1 160.5,222.1 124.4,210.4 88.2,236.1 52.0,244.4" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,130.5 88.2,130.4 124.4,127.8 160.5,132.0 196.7,76.9 232.9,76.6 269.1,81.3 305.3,72.1 341.5,76.7 377.6,76.2 413.8,76.2 450.0,70.5 450.0,169.6 413.8,169.5 377.6,169.5 341.5,171.0 305.3,171.1 269.1,168.6 232.9,170.5 196.7,187.6 160.5,180.1 124.4,181.6 88.2,181.9 52.0,181.9" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,188.4 88.2,182.0 124.4,162.3 160.5,160.4 196.7,158.7 232.9,146.7 269.1,148.6 305.3,148.4 341.5,147.9 377.6,147.2 413.8,145.8 450.0,144.1" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,140.9 88.2,141.0 124.4,140.7 160.5,141.6 196.7,143.4 232.9,130.3 269.1,130.1 305.3,130.3 341.5,130.4 377.6,130.1 413.8,130.6 450.0,130.6" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 6.07 µs | 985 µs (1.6e+02×) | 219 µs (36×) | 3.23 µs (0.53×) |
| D38 | 8.87 µs | 984 µs (1.1e+02×) | 221 µs (25×) | 3.26 µs (0.37×) |
| D57 | 6.01 µs | 983 µs (1.6e+02×) | 221 µs (37×) | 3.24 µs (0.54×) |
| D76 | 7.31 µs | 1.08 ms (1.5e+02×) | 209 µs (29×) | 3.42 µs (0.47×) |
| D115 | 6.39 µs | 1.01 ms (1.6e+02×) | 189 µs (30×) | 3.27 µs (0.51×) |
| D153 | 12.9 µs | 1.46 ms (1.1e+02×) | 423 µs (33×) | 4.62 µs (0.36×) |
| D230 | 14.4 µs | 1.32 ms (91×) | 426 µs (30×) | 4.39 µs (0.3×) |
| D307 | 15.7 µs | 1.3 ms (83×) | 430 µs (27×) | 4.42 µs (0.28×) |
| D462 | 17.8 µs | 1.31 ms (73×) | 426 µs (24×) | 4.45 µs (0.25×) |
| D616 | 24.1 µs | 1.55 ms (64×) | 440 µs (18×) | 4.7 µs (0.19×) |
| D924 | 36.6 µs | 1.44 ms (39×) | 427 µs (12×) | 4.65 µs (0.13×) |
| D1232 | 52.9 µs | 1.52 ms (29×) | 427 µs (8.1×) | 4.66 µs (0.088×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.5 88.2,171.2 124.4,177.7 160.5,172.4 196.7,107.9 232.9,93.3 269.1,94.2 305.3,90.7 341.5,162.5 377.6,146.3 413.8,87.9 450.0,87.8 450.0,167.3 413.8,175.8 377.6,181.6 341.5,190.6 305.3,192.6 269.1,194.1 232.9,199.1 196.7,210.4 160.5,208.3 124.4,210.9 88.2,196.4 52.0,199.4" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,73.1 88.2,73.0 124.4,73.1 160.5,71.6 196.7,56.5 232.9,51.2 269.1,54.2 305.3,52.5 341.5,51.8 377.6,52.9 413.8,50.6 450.0,50.7 450.0,145.8 413.8,145.8 377.6,145.7 341.5,146.0 305.3,147.3 269.1,147.2 232.9,145.7 196.7,151.2 160.5,149.2 124.4,150.7 88.2,150.7 52.0,150.7" fill="#9C5BA6" fill-opacity="0.12"/><polygon points="52.0,123.4 88.2,135.7 124.4,135.7 160.5,131.7 196.7,73.4 232.9,80.8 269.1,82.7 305.3,80.9 341.5,73.2 377.6,77.9 413.8,86.2 450.0,82.6 450.0,194.9 413.8,195.4 377.6,195.2 341.5,196.2 305.3,195.4 269.1,196.3 232.9,195.4 196.7,203.6 160.5,202.8 124.4,203.7 88.2,203.7 52.0,203.7" fill="#5E8C3A" fill-opacity="0.12"/><polygon points="52.0,181.1 88.2,170.8 124.4,200.0 160.5,185.6 196.7,126.1 232.9,140.4 269.1,172.5 305.3,176.2 341.5,166.4 377.6,175.8 413.8,104.7 450.0,165.7 450.0,233.3 413.8,235.2 377.6,235.5 341.5,235.8 305.3,235.1 269.1,235.8 232.9,234.8 196.7,240.9 160.5,239.1 124.4,239.5 88.2,238.7 52.0,239.1" fill="#B5663C" fill-opacity="0.12"/><polyline points="52.0,195.8 88.2,189.9 124.4,196.0 160.5,192.9 196.7,195.0 232.9,184.0 269.1,182.3 305.3,181.0 341.5,179.0 377.6,174.2 413.8,167.7 450.0,161.9" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,116.2 88.2,116.2 124.4,116.3 160.5,114.8 196.7,115.9 232.9,110.1 269.1,111.7 305.3,111.9 341.5,111.8 377.6,109.2 413.8,110.3 450.0,109.4" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,139.7 88.2,139.6 124.4,139.6 160.5,140.5 196.7,142.1 232.9,129.4 269.1,129.3 305.3,129.2 341.5,129.4 377.6,128.8 413.8,129.3 450.0,129.3" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,205.7 88.2,205.5 124.4,205.6 160.5,204.8 196.7,205.5 232.9,200.1 269.1,200.9 305.3,200.8 341.5,200.6 377.6,199.8 413.8,200.0 450.0,199.9" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 9.82 µs | 298 µs (30×) |
| D38 | 14 µs | 298 µs (21×) |
| D57 | 9.5 µs | 297 µs (31×) |
| D76 | 11.9 µs | 316 µs (27×) |
| D115 | 18.2 µs | 234 µs (13×) |
| D153 | 21.6 µs | 619 µs (29×) |
| D230 | 23.4 µs | 594 µs (25×) |
| D307 | 42.1 µs | 588 µs (14×) |
| D462 | 56.4 µs | 589 µs (10×) |
| D616 | 95.7 µs | 628 µs (6.6×) |
| D924 | 148 µs | 620 µs (4.2×) |
| D1232 | 206 µs | 619 µs (3×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,169.9 88.2,163.3 124.4,167.1 160.5,153.6 196.7,105.3 232.9,87.3 269.1,87.6 305.3,84.2 341.5,78.4 377.6,60.7 413.8,53.9 450.0,45.2 450.0,164.2 413.8,168.9 377.6,180.3 341.5,178.0 305.3,185.5 269.1,184.0 232.9,184.2 196.7,187.5 160.5,193.9 124.4,195.7 88.2,185.3 52.0,189.2" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,127.2 88.2,128.0 124.4,120.9 160.5,129.5 196.7,76.7 232.9,76.3 269.1,74.1 305.3,73.0 341.5,73.1 377.6,76.1 413.8,73.0 450.0,76.1 450.0,237.9 413.8,237.5 377.6,237.5 341.5,237.6 305.3,237.2 269.1,237.2 232.9,236.8 196.7,244.4 160.5,240.4 124.4,240.0 88.2,240.0 52.0,239.5" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,188.3 88.2,182.8 124.4,188.8 160.5,185.3 196.7,178.7 232.9,176.0 269.1,174.7 305.3,165.5 341.5,161.0 377.6,152.7 413.8,145.8 450.0,140.7" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,134.9 88.2,134.9 124.4,135.0 160.5,134.0 196.7,138.7 232.9,123.5 269.1,124.2 305.3,124.3 341.5,124.3 377.6,123.3 413.8,123.5 450.0,123.5" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 6.14 µs | 394 µs (64×) | 219 µs (36×) | 3.15 µs (0.51×) |
| D38 | 8.84 µs | 395 µs (45×) | 218 µs (25×) | 3.17 µs (0.36×) |
| D57 | 5.93 µs | 392 µs (66×) | 220 µs (37×) | 3.16 µs (0.53×) |
| D76 | 7.24 µs | 429 µs (59×) | 206 µs (28×) | 3.29 µs (0.46×) |
| D115 | 6.35 µs | 433 µs (68×) | 186 µs (29×) | 3.25 µs (0.51×) |
| D153 | 12.8 µs | 598 µs (47×) | 413 µs (32×) | 4.57 µs (0.36×) |
| D230 | 14.2 µs | 553 µs (39×) | 419 µs (29×) | 4.29 µs (0.3×) |
| D307 | 15.5 µs | 581 µs (37×) | 424 µs (27×) | 4.34 µs (0.28×) |
| D462 | 17.8 µs | 541 µs (30×) | 424 µs (24×) | 4.34 µs (0.24×) |
| D616 | 24.3 µs | 602 µs (25×) | 434 µs (18×) | 4.58 µs (0.19×) |
| D924 | 38.6 µs | 607 µs (16×) | 417 µs (11×) | 4.56 µs (0.12×) |
| D1232 | 51.1 µs | 613 µs (12×) | 414 µs (8.1×) | 4.62 µs (0.09×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,173.3 88.2,167.2 124.4,177.7 160.5,173.8 196.7,105.6 232.9,90.7 269.1,88.1 305.3,87.9 341.5,83.4 377.6,87.9 413.8,87.8 450.0,87.8 450.0,169.6 413.8,173.7 377.6,180.8 341.5,186.7 305.3,191.1 269.1,193.4 232.9,199.9 196.7,210.2 160.5,209.6 124.4,212.1 88.2,194.4 52.0,249.1" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,73.0 88.2,73.0 124.4,73.0 160.5,71.5 196.7,56.5 232.9,49.8 269.1,49.6 305.3,54.2 341.5,58.3 377.6,50.3 413.8,57.5 450.0,52.7 450.0,196.9 413.8,196.6 377.6,196.8 341.5,196.2 305.3,196.5 269.1,196.6 232.9,196.1 196.7,198.4 160.5,200.3 124.4,201.1 88.2,201.6 52.0,201.1" fill="#9C5BA6" fill-opacity="0.12"/><polygon points="52.0,135.1 88.2,129.0 124.4,135.3 160.5,136.1 196.7,76.8 232.9,69.6 269.1,80.8 305.3,76.5 341.5,82.6 377.6,77.8 413.8,80.7 450.0,76.5 450.0,195.1 413.8,195.3 377.6,195.1 341.5,196.4 305.3,195.4 269.1,196.2 232.9,195.2 196.7,203.6 160.5,203.0 124.4,203.6 88.2,203.6 52.0,203.6" fill="#5E8C3A" fill-opacity="0.12"/><polygon points="52.0,181.8 88.2,181.3 124.4,181.7 160.5,184.4 196.7,185.3 232.9,105.0 269.1,105.0 305.3,103.5 341.5,173.6 377.6,119.5 413.8,173.1 450.0,106.7 450.0,241.8 413.8,243.9 377.6,242.3 341.5,243.4 305.3,240.8 269.1,243.4 232.9,241.3 196.7,247.7 160.5,243.9 124.4,246.3 88.2,247.7 52.0,246.3" fill="#B5663C" fill-opacity="0.12"/><polyline points="52.0,195.6 88.2,189.9 124.4,196.2 160.5,193.0 196.7,195.1 232.9,184.1 269.1,182.5 305.3,181.1 341.5,179.0 377.6,174.1 413.8,166.9 450.0,162.5" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,130.6 88.2,130.5 124.4,130.6 160.5,129.2 196.7,129.1 232.9,124.0 269.1,125.3 305.3,124.5 341.5,125.6 377.6,123.9 413.8,123.8 450.0,123.7" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,139.8 88.2,139.8 124.4,139.7 160.5,140.7 196.7,142.3 232.9,129.8 269.1,129.6 305.3,129.4 341.5,129.4 377.6,129.1 413.8,129.7 450.0,129.8" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,206.1 88.2,206.0 124.4,206.0 160.5,205.4 196.7,205.6 232.9,200.3 269.1,201.2 305.3,201.1 341.5,201.1 377.6,200.2 413.8,200.3 450.0,200.1" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 9.84 µs | 299 µs (30×) |
| D38 | 14 µs | 299 µs (21×) |
| D57 | 9.3 µs | 297 µs (32×) |
| D76 | 11.9 µs | 318 µs (27×) |
| D115 | 18.2 µs | 236 µs (13×) |
| D153 | 21.9 µs | 622 µs (28×) |
| D230 | 38.6 µs | 594 µs (15×) |
| D307 | 53.6 µs | 589 µs (11×) |
| D462 | 58.7 µs | 590 µs (10×) |
| D616 | 101 µs | 631 µs (6.2×) |
| D924 | 155 µs | 621 µs (4×) |
| D1232 | 214 µs | 624 µs (2.9×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="229.1" x2="450" y2="229.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="232.1" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="167.4" x2="450" y2="167.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="170.4" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="136.6" x2="450" y2="136.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="139.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="105.7" x2="450" y2="105.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="108.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="74.9" x2="450" y2="74.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="77.9" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 s</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.9 88.2,178.9 124.4,177.5 160.5,178.1 196.7,112.5 232.9,108.5 269.1,110.5 305.3,109.5 341.5,107.6 377.6,88.9 413.8,84.9 450.0,71.6 450.0,180.3 413.8,187.7 377.6,195.8 341.5,200.4 305.3,209.5 269.1,211.0 232.9,219.2 196.7,229.3 160.5,234.1 124.4,234.1 88.2,237.6 52.0,243.6" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,146.6 88.2,148.2 124.4,147.7 160.5,148.2 196.7,106.6 232.9,102.6 269.1,102.6 305.3,99.8 341.5,104.8 377.6,102.5 413.8,107.6 450.0,109.3 450.0,240.8 413.8,240.4 377.6,240.4 341.5,240.5 305.3,240.4 269.1,240.7 232.9,240.1 196.7,246.1 160.5,242.8 124.4,242.8 88.2,242.5 52.0,242.4" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,198.5 88.2,193.8 124.4,199.3 160.5,196.0 196.7,190.3 232.9,187.8 269.1,180.2 305.3,175.8 341.5,174.6 377.6,167.3 413.8,161.5 450.0,157.2" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,152.8 88.2,152.7 124.4,152.8 160.5,151.9 196.7,155.9 232.9,142.9 269.1,143.5 305.3,143.7 341.5,143.6 377.6,142.7 413.8,143.0 450.0,142.9" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | fastnum | g_math | rust_decimal |
| :-- | --: | --: | --: | --: |
| D18 | 11.2 µs | 1.35 ms (1.2e+02×) | 239 µs (21×) | 3.82 µs (0.34×) |
| D38 | 16.2 µs | 1.35 ms (84×) | 238 µs (15×) | 3.81 µs (0.24×) |
| D57 | 7.75 µs | 1.35 ms (1.7e+02×) | 238 µs (31×) | 3.82 µs (0.49×) |
| D76 | 9.14 µs | 1.49 ms (1.6e+02×) | 224 µs (25×) | 4.54 µs (0.5×) |
| D115 | 7.8 µs | 1.75 ms (2.2e+02×) | 202 µs (26×) | 3.51 µs (0.45×) |
| D153 | 15.6 µs | 4.51 ms (2.9e+02×) | 453 µs (29×) | 4.69 µs (0.3×) |
| D230 | 17.3 µs | 4.33 ms (2.5e+02×) | 458 µs (26×) | 4.35 µs (0.25×) |
| D307 | 18.6 µs | 4.37 ms (2.4e+02×) | 456 µs (25×) | 4.37 µs (0.24×) |
| D462 | 20 µs | 4.34 ms (2.2e+02×) | 456 µs (23×) | 4.37 µs (0.22×) |
| D616 | 27.2 µs | 4.6 ms (1.7e+02×) | 473 µs (17×) | 4.66 µs (0.17×) |
| D924 | 40.2 µs | 4.62 ms (1.1e+02×) | 448 µs (11×) | 4.68 µs (0.12×) |
| D1232 | 54.4 µs | 4.6 ms (84×) | 457 µs (8.4×) | 4.65 µs (0.085×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="229.1" x2="450" y2="229.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="232.1" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="198.3" x2="450" y2="198.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="201.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="167.4" x2="450" y2="167.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="170.4" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="136.6" x2="450" y2="136.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="139.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="105.7" x2="450" y2="105.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="108.7" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="74.9" x2="450" y2="74.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="77.9" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">1 s</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.8 88.2,184.8 124.4,181.6 160.5,178.7 196.7,144.1 232.9,121.7 269.1,112.5 305.3,112.5 341.5,112.5 377.6,112.4 413.8,112.4 450.0,112.2 450.0,181.4 413.8,186.0 377.6,190.3 341.5,195.3 305.3,198.1 269.1,199.4 232.9,201.9 196.7,210.9 160.5,209.7 124.4,211.7 88.2,236.9 52.0,250.0" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,90.5 88.2,90.4 124.4,90.5 160.5,89.2 196.7,76.2 232.9,77.0 269.1,75.5 305.3,75.4 341.5,73.6 377.6,73.1 413.8,72.0 450.0,74.1 450.0,136.6 413.8,136.5 377.6,136.6 341.5,137.7 305.3,137.5 269.1,137.6 232.9,136.5 196.7,140.3 160.5,139.4 124.4,140.6 88.2,140.8 52.0,140.6" fill="#9C5BA6" fill-opacity="0.12"/><polygon points="52.0,145.0 88.2,151.1 124.4,151.1 160.5,152.7 196.7,112.0 232.9,102.6 269.1,100.7 305.3,99.9 341.5,106.3 377.6,102.6 413.8,102.6 450.0,102.7 450.0,194.9 413.8,195.0 377.6,194.6 341.5,195.7 305.3,195.3 269.1,195.7 232.9,195.1 196.7,205.8 160.5,202.8 124.4,203.2 88.2,203.3 52.0,203.2" fill="#5E8C3A" fill-opacity="0.12"/><polygon points="52.0,193.4 88.2,191.8 124.4,185.2 160.5,193.7 196.7,133.1 232.9,125.9 269.1,185.8 305.3,177.1 341.5,147.8 377.6,188.4 413.8,136.0 450.0,125.5 450.0,238.1 413.8,236.9 377.6,238.1 341.5,236.9 305.3,236.9 269.1,235.7 232.9,236.7 196.7,242.5 160.5,241.4 124.4,243.2 88.2,242.5 52.0,241.7" fill="#B5663C" fill-opacity="0.12"/><polyline points="52.0,196.8 88.2,191.9 124.4,201.7 160.5,199.5 196.7,201.6 232.9,192.3 269.1,190.9 305.3,190.0 341.5,189.0 377.6,184.9 413.8,179.6 450.0,175.6" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,132.5 88.2,132.5 124.4,132.5 160.5,131.3 196.7,129.1 232.9,116.4 269.1,116.9 305.3,116.8 341.5,116.9 377.6,116.1 413.8,116.1 450.0,116.1" fill="none" stroke="#9C5BA6" stroke-width="1.6"/><polyline points="52.0,155.8 88.2,155.8 124.4,155.8 160.5,156.6 196.7,158.0 232.9,147.2 269.1,147.0 305.3,147.1 341.5,147.1 377.6,146.6 413.8,147.3 450.0,147.1" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><polyline points="52.0,211.2 88.2,211.2 124.4,211.2 160.5,208.9 196.7,212.3 232.9,208.4 269.1,209.4 305.3,209.4 341.5,209.4 377.6,208.5 413.8,208.5 450.0,208.6" fill="none" stroke="#B5663C" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#9C5BA6"/><text x="164" y="20" font-size="8.5" fill="currentColor">fastnum</text><rect x="210" y="12" width="9" height="9" fill="#5E8C3A"/><text x="222" y="20" font-size="8.5" fill="currentColor">g_math</text><rect x="262" y="12" width="9" height="9" fill="#B5663C"/><text x="274" y="20" font-size="8.5" fill="currentColor">rust_decimal</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | decimal-scaled | g_math |
| :-- | --: | --: |
| D18 | 9.82 µs | 179 µs (18×) |
| D38 | 14 µs | 179 µs (13×) |
| D57 | 9.92 µs | 179 µs (18×) |
| D76 | 11.9 µs | 188 µs (16×) |
| D115 | 17.3 µs | 142 µs (8.2×) |
| D153 | 21 µs | 373 µs (18×) |
| D230 | 21.9 µs | 353 µs (16×) |
| D307 | 24 µs | 354 µs (15×) |
| D462 | 28.7 µs | 355 µs (12×) |
| D616 | 34 µs | 375 µs (11×) |
| D924 | 48.5 µs | 372 µs (7.7×) |
| D1232 | 66.4 µs | 371 µs (5.6×) |

<figure>
<svg viewBox="0 0 460 290" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="260.0" x2="450" y2="260.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="263.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="224.0" x2="450" y2="224.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="227.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="188.0" x2="450" y2="188.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="191.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="152.0" x2="450" y2="152.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="116.0" x2="450" y2="116.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="119.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="80.0" x2="450" y2="80.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="83.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="44.0" x2="450" y2="44.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="47.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="272" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="272" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="272" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="272" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="272" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="272" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="272" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="272" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="272" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="272" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,176.6 88.2,172.4 124.4,169.4 160.5,161.8 196.7,90.7 232.9,87.9 269.1,87.8 305.3,87.6 341.5,85.5 377.6,87.4 413.8,83.4 450.0,85.1 450.0,166.6 413.8,174.5 377.6,185.3 341.5,190.6 305.3,200.1 269.1,203.8 232.9,211.9 196.7,224.9 160.5,230.7 124.4,232.2 88.2,233.9 52.0,243.8" fill="#2563eb" fill-opacity="0.12"/><polygon points="52.0,135.3 88.2,137.2 124.4,137.4 160.5,133.7 196.7,77.0 232.9,86.8 269.1,81.0 305.3,79.4 341.5,81.0 377.6,78.1 413.8,80.9 450.0,81.0 450.0,169.7 413.8,168.8 377.6,168.9 341.5,170.5 305.3,170.4 269.1,170.4 232.9,169.2 196.7,184.9 160.5,179.1 124.4,180.6 88.2,181.0 52.0,181.1" fill="#5E8C3A" fill-opacity="0.12"/><polyline points="52.0,188.3 88.2,182.8 124.4,188.1 160.5,185.3 196.7,179.4 232.9,176.4 269.1,175.8 305.3,174.3 341.5,171.5 377.6,168.9 413.8,163.3 450.0,158.4" fill="none" stroke="#2563eb" stroke-width="1.6"/><polyline points="52.0,142.9 88.2,142.9 124.4,142.9 160.5,142.1 196.7,146.6 232.9,131.4 269.1,132.3 305.3,132.2 341.5,132.2 377.6,131.3 413.8,131.5 450.0,131.5" fill="none" stroke="#5E8C3A" stroke-width="1.6"/><rect x="52" y="12" width="9" height="9" fill="#2563eb"/><text x="64" y="20" font-size="8.5" fill="currentColor">decimal-scaled</text><rect x="152" y="12" width="9" height="9" fill="#5E8C3A"/><text x="164" y="20" font-size="8.5" fill="currentColor">g_math</text><line x1="52" y1="44" x2="52" y2="260" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="260" x2="450" y2="260" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time per library at each width (log scale; decimal-scaled at scale 30, or the nearest compiled scale per width), with a shaded min–max band; a gap means that library has no equivalent at that width.</figcaption>
</figure>

</div>
<!-- END GENERATED:comparisons:body:trig -->
