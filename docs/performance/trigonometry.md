# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.06 ns | 2 µs | 3.22 µs | 2.32 µs | 3.29 µs |
| D38 | 914 ns | 4.64 µs | 4.09 µs | 6.65 µs | 5.63 µs |
| D57 | 1.06 µs | 5.77 µs | 8.17 µs | 10.3 µs | 14 µs |
| D76 | 1.55 µs | 4.31 µs | 5.62 µs | 14.4 µs | 19.5 µs |
| D115 | 1.43 µs | 6.98 µs | 20.4 µs | 29 µs | 42.4 µs |
| D153 | 1.51 µs | 8.27 µs | 25.6 µs | 32.9 µs | 56.3 µs |
| D230 | 1.03 µs | 20.5 µs | 42 µs | 70 µs | 116 µs |
| D307 | 1.02 µs | 20.1 µs | 66 µs | 124 µs | 154 µs |
| D462 | 1.33 µs | 42.3 µs | 125 µs | 256 µs | 403 µs |
| D616 | 1.61 µs | 61 µs | 171 µs | 389 µs | 557 µs |
| D924 | 1.54 µs | 127 µs | 390 µs | 715 µs | 1.17 ms |
| D1232 | 2.01 µs | 218 µs | 617 µs | 1.53 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.1 88.2,125.4 124.4,123.5 160.5,118.9 196.7,119.8 232.9,119.2 269.1,123.9 305.3,124.1 341.5,120.7 377.6,118.4 413.8,119.0 450.0,115.6 450.0,23.1 413.8,36.7 377.6,45.8 341.5,49.9 305.3,61.8 269.1,65.3 232.9,74.3 196.7,77.8 160.5,87.4 124.4,91.5 88.2,102.8 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.1 88.2,125.4 124.4,123.5 160.5,118.9 196.7,119.8 232.9,119.2 269.1,123.9 305.3,124.1 341.5,120.7 377.6,118.4 413.8,119.0 450.0,115.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.2 124.4,102.5 160.5,106.2 196.7,100.2 232.9,98.1 269.1,86.8 305.3,87.0 341.5,77.8 377.6,73.3 413.8,64.2 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.8 124.4,98.2 160.5,102.9 196.7,86.9 232.9,84.1 269.1,77.9 305.3,72.3 341.5,64.4 377.6,60.5 413.8,50.3 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,100.8 124.4,95.3 160.5,91.2 196.7,82.5 232.9,80.9 269.1,71.6 305.3,64.5 341.5,55.5 377.6,50.3 413.8,42.7 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,102.8 124.4,91.5 160.5,87.4 196.7,77.8 232.9,74.3 269.1,65.3 305.3,61.8 341.5,49.9 377.6,45.8 413.8,36.7 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.23 µs | 3.62 µs | 5.87 µs | 4.25 µs | 5.79 µs |
| D38 | 2.55 µs | 5.83 µs | 7.1 µs | 7.14 µs | 6.58 µs |
| D57 | 2.38 µs | 4.28 µs | 4.98 µs | 6.1 µs | 7.66 µs |
| D76 | 3.72 µs | 2.65 µs | 3.42 µs | 7.79 µs | 10.2 µs |
| D115 | 6.23 µs | 7.76 µs | 12.3 µs | 14.6 µs | 21.8 µs |
| D153 | 6.23 µs | 8.67 µs | 16.1 µs | 16.1 µs | 27.4 µs |
| D230 | 6.18 µs | 15.8 µs | 26.5 µs | 42.7 µs | 68.6 µs |
| D307 | 8.76 µs | 19.1 µs | 50.9 µs | 85.3 µs | 111 µs |
| D462 | 10.5 µs | 36.6 µs | 85.1 µs | 168 µs | 275 µs |
| D616 | 22 µs | 71.1 µs | 142 µs | 290 µs | 441 µs |
| D924 | 27.3 µs | 156 µs | 344 µs | 612 µs | 1.02 ms |
| D1232 | 43.3 µs | 276 µs | 622 µs | 1.45 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,189.7 124.4,191.2 160.5,181.5 196.7,170.3 232.9,170.3 269.1,170.4 305.3,162.9 341.5,158.8 377.6,142.9 413.8,138.2 450.0,128.2 450.0,35.7 413.8,59.6 377.6,77.8 341.5,88.1 305.3,107.8 269.1,118.2 232.9,138.1 196.7,143.1 160.5,159.5 124.4,165.8 88.2,169.1 52.0,171.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,189.7 124.4,191.2 160.5,181.5 196.7,170.3 232.9,170.3 269.1,170.4 305.3,162.9 341.5,158.8 377.6,142.9 413.8,138.2 450.0,128.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.1 88.2,171.7 124.4,178.4 160.5,188.8 196.7,165.5 232.9,163.1 269.1,150.1 305.3,146.0 341.5,131.9 377.6,117.4 413.8,100.4 450.0,88.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,167.4 124.4,175.1 160.5,183.3 196.7,155.5 232.9,149.6 269.1,138.8 305.3,124.7 341.5,113.5 377.6,102.4 413.8,83.2 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,167.3 124.4,170.7 160.5,165.4 196.7,151.8 232.9,149.7 269.1,128.5 305.3,113.5 341.5,98.7 377.6,86.9 413.8,70.7 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.9 88.2,169.1 124.4,165.8 160.5,159.5 196.7,143.1 232.9,138.1 269.1,118.2 305.3,107.8 341.5,88.1 377.6,77.8 413.8,59.6 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.99 µs | 3.2 µs | 2.31 µs | 3.28 µs |
| D38 | 860 ns | 4.65 µs | 4.04 µs | 6.62 µs | 5.6 µs |
| D57 | 957 ns | 5.75 µs | 8.11 µs | 10.3 µs | 14 µs |
| D76 | 1.42 µs | 4.12 µs | 5.43 µs | 14.3 µs | 19.5 µs |
| D115 | 1.3 µs | 6.94 µs | 21.3 µs | 29.5 µs | 42.3 µs |
| D153 | 1.36 µs | 8.25 µs | 27.2 µs | 32.9 µs | 55.1 µs |
| D230 | 956 ns | 19.5 µs | 42.2 µs | 70 µs | 116 µs |
| D307 | 944 ns | 18.8 µs | 65.3 µs | 123 µs | 152 µs |
| D462 | 1.24 µs | 42.4 µs | 125 µs | 255 µs | 403 µs |
| D616 | 1.45 µs | 60.9 µs | 171 µs | 389 µs | 559 µs |
| D924 | 1.43 µs | 127 µs | 390 µs | 717 µs | 1.16 ms |
| D1232 | 1.86 µs | 219 µs | 616 µs | 1.54 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,126.2 124.4,124.8 160.5,119.9 196.7,121.0 232.9,120.5 269.1,124.8 305.3,125.0 341.5,121.6 377.6,119.7 413.8,119.8 450.0,116.6 450.0,23.0 413.8,36.7 377.6,45.8 341.5,49.9 305.3,61.9 269.1,65.3 232.9,74.5 196.7,77.8 160.5,87.4 124.4,91.6 88.2,102.9 52.0,109.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,126.2 124.4,124.8 160.5,119.9 196.7,121.0 232.9,120.5 269.1,124.8 305.3,125.0 341.5,121.6 377.6,119.7 413.8,119.8 450.0,116.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.2 124.4,102.6 160.5,106.7 196.7,100.3 232.9,98.1 269.1,87.4 305.3,87.9 341.5,77.8 377.6,73.3 413.8,64.2 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,107.0 124.4,98.3 160.5,103.3 196.7,86.3 232.9,83.3 269.1,77.9 305.3,72.4 341.5,64.4 377.6,60.5 413.8,50.2 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,100.8 124.4,95.3 160.5,91.2 196.7,82.3 232.9,81.0 269.1,71.6 305.3,64.5 341.5,55.5 377.6,50.3 413.8,42.7 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,102.9 124.4,91.6 160.5,87.4 196.7,77.8 232.9,74.5 269.1,65.3 305.3,61.9 341.5,49.9 377.6,45.8 413.8,36.7 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 µs | 1.72 µs | 2.52 µs | 1.86 µs | 2.5 µs |
| D38 | 1.27 µs | 2.55 µs | 3.1 µs | 3.18 µs | 3.01 µs |
| D57 | 3.22 µs | 5.3 µs | 7.32 µs | 8.9 µs | 11.6 µs |
| D76 | 4.9 µs | 3.75 µs | 5.08 µs | 11.7 µs | 14.9 µs |
| D115 | 8.49 µs | 11.9 µs | 19 µs | 21.6 µs | 33.7 µs |
| D153 | 8.48 µs | 13.3 µs | 23.2 µs | 25.7 µs | 41.4 µs |
| D230 | 8.22 µs | 24.3 µs | 42.3 µs | 63.4 µs | 90.6 µs |
| D307 | 11.4 µs | 26.9 µs | 72.7 µs | 114 µs | 140 µs |
| D462 | 13.7 µs | 55.2 µs | 119 µs | 210 µs | 340 µs |
| D616 | 29.4 µs | 104 µs | 191 µs | 366 µs | 528 µs |
| D924 | 35.8 µs | 228 µs | 439 µs | 700 µs | 1.08 ms |
| D1232 | 57.3 µs | 384 µs | 769 µs | 1.62 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.3 88.2,204.9 124.4,184.6 160.5,175.5 196.7,163.5 232.9,163.6 269.1,164.2 305.3,157.2 341.5,153.1 377.6,136.6 413.8,132.3 450.0,122.1 450.0,35.4 413.8,58.4 377.6,73.9 341.5,83.5 305.3,102.7 269.1,112.2 232.9,129.2 196.7,133.6 160.5,151.4 124.4,156.8 88.2,186.1 52.0,190.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.3 88.2,204.9 124.4,184.6 160.5,175.5 196.7,163.5 232.9,163.6 269.1,164.2 305.3,157.2 341.5,153.1 377.6,136.6 413.8,132.3 450.0,122.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,189.6 124.4,173.8 160.5,181.3 196.7,156.3 232.9,153.8 269.1,140.7 305.3,138.5 341.5,122.9 377.6,109.1 413.8,92.1 450.0,80.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.0 88.2,185.4 124.4,166.8 160.5,174.7 196.7,146.1 232.9,141.7 269.1,128.7 305.3,116.9 341.5,106.3 377.6,95.9 413.8,77.9 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.6 88.2,184.9 124.4,162.5 160.5,156.6 196.7,143.2 232.9,139.5 269.1,119.9 305.3,107.2 341.5,93.9 377.6,81.9 413.8,67.8 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.1 88.2,186.1 124.4,156.8 160.5,151.4 196.7,133.6 232.9,129.2 269.1,112.2 305.3,102.7 341.5,83.5 377.6,73.9 413.8,58.4 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.4 µs | 1.57 µs | 2.69 µs | 1.95 µs | 2.82 µs |
| D38 | 1.12 µs | 2.74 µs | 3.07 µs | 2.83 µs | 2.24 µs |
| D57 | 1.22 µs | 2.85 µs | 3.41 µs | 4.01 µs | 5.24 µs |
| D76 | 5.97 µs | 3.32 µs | 4.29 µs | 11.9 µs | 16.4 µs |
| D115 | 5.56 µs | 5.29 µs | 16.4 µs | 27.4 µs | 38.2 µs |
| D153 | 5.65 µs | 6.46 µs | 19.4 µs | 30.1 µs | 50.8 µs |
| D230 | 4.17 µs | 16.6 µs | 38 µs | 64.2 µs | 107 µs |
| D307 | 4.15 µs | 15.8 µs | 53.7 µs | 115 µs | 144 µs |
| D462 | 2.54 µs | 34.1 µs | 105 µs | 224 µs | 341 µs |
| D616 | 5.66 µs | 56.5 µs | 160 µs | 364 µs | 529 µs |
| D924 | 5.55 µs | 117 µs | 366 µs | 682 µs | 1.11 ms |
| D1232 | 6.11 µs | 204 µs | 585 µs | 1.48 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.0 88.2,207.5 124.4,205.7 160.5,171.2 196.7,172.7 232.9,172.4 269.1,179.0 305.3,179.1 341.5,189.8 377.6,172.4 413.8,172.8 450.0,170.7 450.0,33.8 413.8,57.7 377.6,73.8 341.5,83.4 305.3,102.0 269.1,108.6 232.9,124.7 196.7,130.9 160.5,149.3 124.4,174.0 88.2,192.5 52.0,187.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.0 88.2,207.5 124.4,205.7 160.5,171.2 196.7,172.7 232.9,172.4 269.1,179.0 305.3,179.1 341.5,189.8 377.6,172.4 413.8,172.8 450.0,170.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.2 88.2,188.1 124.4,187.3 160.5,183.9 196.7,173.8 232.9,169.5 269.1,149.0 305.3,150.1 341.5,133.4 377.6,122.4 413.8,106.7 450.0,94.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.5 88.2,185.6 124.4,183.4 160.5,178.4 196.7,149.2 232.9,145.6 269.1,131.0 305.3,123.5 341.5,109.0 377.6,99.8 413.8,81.8 450.0,71.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.5 88.2,187.4 124.4,179.9 160.5,156.2 196.7,138.1 232.9,136.1 269.1,119.6 305.3,107.1 341.5,92.5 377.6,81.9 413.8,68.3 450.0,51.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.5 88.2,192.5 124.4,174.0 160.5,149.3 196.7,130.9 232.9,124.7 269.1,108.6 305.3,102.0 341.5,83.4 377.6,73.8 413.8,57.7 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.74 ns | 1.73 µs | 2.86 µs | 2.09 µs | 2.89 µs |
| D38 | 2.51 ns | 2.86 µs | 3.58 µs | 3.54 µs | 3.29 µs |
| D57 | 302 ns | 5.68 µs | 7.11 µs | 8.81 µs | 11.4 µs |
| D76 | 532 ns | 3.68 µs | 4.73 µs | 11.3 µs | 15.3 µs |
| D115 | 925 ns | 10.8 µs | 18 µs | 22.1 µs | 34 µs |
| D153 | 947 ns | 12.1 µs | 24.4 µs | 25 µs | 44 µs |
| D230 | 973 ns | 23 µs | 41.1 µs | 71.5 µs | 113 µs |
| D307 | 1.33 µs | 29 µs | 84.1 µs | 146 µs | 196 µs |
| D462 | 1.69 µs | 57.2 µs | 145 µs | 301 µs | 501 µs |
| D616 | 3.55 µs | 115 µs | 256 µs | 520 µs | 796 µs |
| D924 | 3.98 µs | 263 µs | 614 µs | 1.13 ms | 1.9 ms |
| D1232 | 7.29 µs | 470 µs | 1.13 ms | 2.7 ms | 5.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,198.6 124.4,139.2 160.5,132.1 196.7,125.3 232.9,125.0 269.1,124.6 305.3,120.7 341.5,117.8 377.6,108.6 413.8,107.1 450.0,99.6 450.0,16.8 413.8,30.6 377.6,41.4 341.5,47.1 305.3,58.8 269.1,65.7 232.9,77.3 196.7,80.5 160.5,90.4 124.4,94.0 88.2,109.5 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,198.6 124.4,139.2 160.5,132.1 196.7,125.3 232.9,125.0 269.1,124.6 305.3,120.7 341.5,117.8 377.6,108.6 413.8,107.1 450.0,99.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,111.2 124.4,102.7 160.5,108.1 196.7,94.8 232.9,93.4 269.1,85.4 305.3,82.5 341.5,74.1 377.6,65.4 413.8,55.1 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,108.5 124.4,100.0 160.5,105.0 196.7,88.4 232.9,84.6 269.1,78.2 305.3,69.3 341.5,62.5 377.6,55.5 413.8,44.6 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.2 88.2,108.6 124.4,97.3 160.5,94.2 196.7,85.9 232.9,84.3 269.1,71.3 305.3,62.4 341.5,53.5 377.6,46.7 413.8,37.1 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,109.5 124.4,94.0 160.5,90.4 196.7,80.5 232.9,77.3 269.1,65.7 305.3,58.8 341.5,47.1 377.6,41.4 413.8,30.6 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.26 µs | 1.18 µs | 2.51 µs | 1.55 µs | 2.48 µs |
| D38 | 1.12 µs | 2.51 µs | 3 µs | 3.04 µs | 2.24 µs |
| D57 | 2.29 µs | 3.52 µs | 4.64 µs | 5.48 µs | 8.81 µs |
| D76 | 3.3 µs | 2.37 µs | 3.01 µs | 7.16 µs | 9.89 µs |
| D115 | 3.01 µs | 3.86 µs | 9.77 µs | 12.5 µs | 18.7 µs |
| D153 | 3.05 µs | 4.57 µs | 9.95 µs | 13.6 µs | 26.1 µs |
| D230 | 2.2 µs | 9.73 µs | 18.6 µs | 35.9 µs | 67.3 µs |
| D307 | 2.21 µs | 9.44 µs | 27.7 µs | 68.8 µs | 93.3 µs |
| D462 | 1.74 µs | 15.1 µs | 62.1 µs | 141 µs | 231 µs |
| D616 | 3.16 µs | 30.3 µs | 102 µs | 243 µs | 361 µs |
| D924 | 3.04 µs | 71 µs | 241 µs | 479 µs | 817 µs |
| D1232 | 3.57 µs | 129 µs | 414 µs | 1.07 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.9 88.2,207.5 124.4,192.0 160.5,184.1 196.7,186.1 232.9,185.8 269.1,192.9 305.3,192.7 341.5,198.0 377.6,185.0 413.8,185.9 450.0,182.4 450.0,40.5 413.8,64.4 377.6,82.1 341.5,91.8 305.3,111.5 269.1,118.6 232.9,139.1 196.7,146.4 160.5,160.2 124.4,162.7 88.2,192.5 52.0,190.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.9 88.2,207.5 124.4,192.0 160.5,184.1 196.7,186.1 232.9,185.8 269.1,192.9 305.3,192.7 341.5,198.0 377.6,185.0 413.8,185.9 450.0,182.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,206.4 88.2,190.0 124.4,182.7 160.5,191.2 196.7,180.7 232.9,177.0 269.1,160.6 305.3,161.2 341.5,151.0 377.6,135.9 413.8,117.4 450.0,104.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.0 88.2,186.1 124.4,176.7 160.5,186.1 196.7,160.5 232.9,160.1 269.1,146.5 305.3,137.9 341.5,120.4 377.6,109.5 413.8,90.9 450.0,79.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.5 88.2,185.9 124.4,173.1 160.5,167.3 196.7,155.2 232.9,153.4 269.1,132.2 305.3,118.1 341.5,102.6 377.6,90.7 413.8,76.0 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.3 88.2,192.5 124.4,162.7 160.5,160.2 196.7,146.4 232.9,139.1 269.1,118.6 305.3,111.5 341.5,91.8 377.6,82.1 413.8,64.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.56 µs | 1.71 µs | 3.31 µs | 2.21 µs | 3.28 µs |
| D38 | 1.32 µs | 3.3 µs | 3.91 µs | 3.85 µs | 2.98 µs |
| D57 | 4.36 µs | 5.52 µs | 7.1 µs | 8 µs | 10.4 µs |
| D76 | 6.22 µs | 3.9 µs | 4.37 µs | 10.2 µs | 13.1 µs |
| D115 | 12.7 µs | 10.9 µs | 11.9 µs | 19 µs | 26.4 µs |
| D153 | 5.93 µs | 6.5 µs | 16.2 µs | 17.3 µs | 32.1 µs |
| D230 | 4.4 µs | 14 µs | 23.3 µs | 43.4 µs | 75.7 µs |
| D307 | 4.48 µs | 12.6 µs | 52.1 µs | 79.9 µs | 101 µs |
| D462 | 5.42 µs | 23.8 µs | 81.9 µs | 167 µs | 252 µs |
| D616 | 6.14 µs | 36.1 µs | 114 µs | 261 µs | 352 µs |
| D924 | 5.97 µs | 83.3 µs | 263 µs | 479 µs | 719 µs |
| D1232 | 7.04 µs | 145 µs | 408 µs | 932 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.4 88.2,204.0 124.4,178.0 160.5,170.3 196.7,154.8 232.9,171.4 269.1,177.8 305.3,177.5 341.5,173.3 377.6,170.6 413.8,171.2 450.0,167.6 450.0,37.4 413.8,67.2 377.6,82.7 341.5,89.9 305.3,109.8 269.1,116.0 232.9,134.7 196.7,138.9 160.5,154.2 124.4,159.1 88.2,186.3 52.0,184.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.4 88.2,204.0 124.4,178.0 160.5,170.3 196.7,154.8 232.9,171.4 269.1,177.8 305.3,177.5 341.5,173.3 377.6,170.6 413.8,171.2 450.0,167.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.4 88.2,184.1 124.4,172.9 160.5,180.4 196.7,158.2 232.9,169.4 269.1,152.8 305.3,155.0 341.5,141.2 377.6,132.1 413.8,114.0 450.0,101.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.0 88.2,180.4 124.4,167.5 160.5,178.0 196.7,156.2 232.9,149.5 269.1,141.7 305.3,124.2 341.5,114.3 377.6,107.2 413.8,89.0 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.8 88.2,180.7 124.4,164.9 160.5,159.5 196.7,146.1 232.9,148.1 269.1,128.1 305.3,114.9 341.5,98.8 377.6,89.1 413.8,76.0 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.2 88.2,186.3 124.4,159.1 160.5,154.2 196.7,138.9 232.9,134.7 269.1,116.0 305.3,109.8 341.5,89.9 377.6,82.7 413.8,67.2 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.35 µs | 1.07 µs | 2.34 µs | 1.52 µs | 2.43 µs |
| D38 | 1.27 µs | 2.35 µs | 2.85 µs | 2.9 µs | 2.22 µs |
| D57 | 2.38 µs | 3.27 µs | 4.31 µs | 5.34 µs | 8.79 µs |
| D76 | 3.35 µs | 2.22 µs | 2.85 µs | 7.04 µs | 9.49 µs |
| D115 | 3.25 µs | 3.68 µs | 9.94 µs | 13 µs | 18.3 µs |
| D153 | 3.29 µs | 4.33 µs | 10.2 µs | 13.1 µs | 25.2 µs |
| D230 | 2.35 µs | 10.1 µs | 19.3 µs | 35.8 µs | 63.3 µs |
| D307 | 2.35 µs | 8.78 µs | 26.7 µs | 65.3 µs | 91 µs |
| D462 | 1.71 µs | 16.1 µs | 58.8 µs | 141 µs | 230 µs |
| D616 | 3.28 µs | 29.6 µs | 101 µs | 240 µs | 359 µs |
| D924 | 3.18 µs | 68.4 µs | 240 µs | 478 µs | 820 µs |
| D1232 | 3.63 µs | 129 µs | 412 µs | 1.07 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.4 88.2,204.8 124.4,191.2 160.5,183.7 196.7,184.4 232.9,184.1 269.1,191.5 305.3,191.4 341.5,198.4 377.6,184.2 413.8,184.9 450.0,182.0 450.0,40.7 413.8,64.3 377.6,82.2 341.5,92.0 305.3,112.1 269.1,119.9 232.9,140.0 196.7,146.9 160.5,161.1 124.4,162.8 88.2,192.6 52.0,190.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.4 88.2,204.8 124.4,191.2 160.5,183.7 196.7,184.4 232.9,184.1 269.1,191.5 305.3,191.4 341.5,198.4 377.6,184.2 413.8,184.9 450.0,182.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,208.5 88.2,191.4 124.4,184.2 160.5,192.7 196.7,181.7 232.9,178.2 269.1,159.9 305.3,162.8 341.5,149.7 377.6,136.4 413.8,118.2 450.0,104.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.5 88.2,187.2 124.4,178.3 160.5,187.3 196.7,160.1 232.9,159.6 269.1,145.7 305.3,138.7 341.5,121.5 377.6,109.8 413.8,91.0 450.0,79.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.9 88.2,186.9 124.4,173.6 160.5,167.6 196.7,154.2 232.9,154.1 269.1,132.3 305.3,119.3 341.5,102.6 377.6,91.0 413.8,76.0 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.7 88.2,192.6 124.4,162.8 160.5,161.1 196.7,146.9 232.9,140.0 269.1,119.9 305.3,112.1 341.5,92.0 377.6,82.2 413.8,64.3 450.0,40.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.72 µs | 3.33 µs | 2.25 µs | 3.29 µs |
| D38 | 1.33 µs | 3.31 µs | 3.92 µs | 3.85 µs | 2.98 µs |
| D57 | 4.45 µs | 5.53 µs | 7.13 µs | 8.02 µs | 10.4 µs |
| D76 | 6.23 µs | 3.71 µs | 4.38 µs | 10.3 µs | 13.1 µs |
| D115 | 12.8 µs | 11 µs | 12.2 µs | 19.1 µs | 26.5 µs |
| D153 | 5.95 µs | 6.56 µs | 16.6 µs | 17.3 µs | 31.6 µs |
| D230 | 4.42 µs | 14 µs | 23.2 µs | 43.5 µs | 75.6 µs |
| D307 | 4.51 µs | 12.6 µs | 52.8 µs | 79.9 µs | 99.4 µs |
| D462 | 5.44 µs | 24 µs | 81.9 µs | 168 µs | 252 µs |
| D616 | 6.27 µs | 36.4 µs | 113 µs | 262 µs | 353 µs |
| D924 | 6.13 µs | 82.8 µs | 263 µs | 476 µs | 717 µs |
| D1232 | 7.2 µs | 145 µs | 408 µs | 931 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.3 88.2,203.7 124.4,177.6 160.5,170.3 196.7,154.7 232.9,171.3 269.1,177.7 305.3,177.3 341.5,173.2 377.6,170.1 413.8,170.6 450.0,167.1 450.0,37.4 413.8,67.2 377.6,82.6 341.5,89.9 305.3,110.1 269.1,116.1 232.9,135.0 196.7,138.9 160.5,154.1 124.4,159.1 88.2,186.3 52.0,184.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.3 88.2,203.7 124.4,177.6 160.5,170.3 196.7,154.7 232.9,171.3 269.1,177.7 305.3,177.3 341.5,173.2 377.6,170.1 413.8,170.6 450.0,167.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.2 88.2,184.0 124.4,172.9 160.5,181.5 196.7,157.9 232.9,169.1 269.1,152.6 305.3,155.1 341.5,141.0 377.6,132.0 413.8,114.1 450.0,101.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,180.3 124.4,167.3 160.5,177.9 196.7,155.7 232.9,149.0 269.1,141.7 305.3,123.9 341.5,114.3 377.6,107.3 413.8,89.0 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,192.4 88.2,180.7 124.4,164.8 160.5,159.4 196.7,146.0 232.9,148.1 269.1,128.1 305.3,114.9 341.5,98.7 377.6,89.1 413.8,76.1 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.1 88.2,186.3 124.4,159.1 160.5,154.1 196.7,138.9 232.9,135.0 269.1,116.1 305.3,110.1 341.5,89.9 377.6,82.6 413.8,67.2 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.48 µs | 2.12 µs | 3.99 µs | 2.73 µs | 4.04 µs |
| D38 | 2.2 µs | 3.98 µs | 4.79 µs | 4.88 µs | 3.99 µs |
| D57 | 3.11 µs | 4.32 µs | 5.79 µs | 6.97 µs | 8.97 µs |
| D76 | 4.29 µs | 2.91 µs | 3.73 µs | 8.96 µs | 11.8 µs |
| D115 | 4.08 µs | 4.87 µs | 12.3 µs | 15.1 µs | 22 µs |
| D153 | 4.18 µs | 5.76 µs | 12.8 µs | 16.5 µs | 29.8 µs |
| D230 | 3.03 µs | 12.6 µs | 22.7 µs | 40.8 µs | 71.3 µs |
| D307 | 2.99 µs | 11 µs | 31.8 µs | 75.2 µs | 101 µs |
| D462 | 2.19 µs | 18.6 µs | 67.6 µs | 156 µs | 249 µs |
| D616 | 4.16 µs | 34.2 µs | 112 µs | 260 µs | 389 µs |
| D924 | 4.09 µs | 77.9 µs | 261 µs | 513 µs | 863 µs |
| D1232 | 4.44 µs | 141 µs | 441 µs | 1.13 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.3 88.2,192.9 124.4,185.4 160.5,178.4 196.7,179.5 232.9,178.9 269.1,185.9 305.3,186.2 341.5,193.0 377.6,179.0 413.8,179.4 450.0,177.6 450.0,39.5 413.8,63.2 377.6,80.5 341.5,90.2 305.3,109.7 269.1,117.3 232.9,136.3 196.7,142.9 160.5,156.4 124.4,162.4 88.2,180.0 52.0,179.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.3 88.2,192.9 124.4,185.4 160.5,178.4 196.7,179.5 232.9,178.9 269.1,185.9 305.3,186.2 341.5,193.0 377.6,179.0 413.8,179.4 450.0,177.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.7 88.2,180.0 124.4,178.2 160.5,186.8 196.7,175.6 232.9,172.0 269.1,155.0 305.3,157.9 341.5,146.6 377.6,133.3 413.8,115.4 450.0,102.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,176.0 124.4,171.9 160.5,181.4 196.7,155.5 232.9,154.6 269.1,142.2 305.3,134.8 341.5,118.5 377.6,107.6 413.8,89.2 450.0,77.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.2 88.2,175.6 124.4,167.9 160.5,162.4 196.7,151.0 232.9,149.1 269.1,129.4 305.3,116.2 341.5,100.3 377.6,89.2 413.8,74.5 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,180.0 124.4,162.4 160.5,156.4 196.7,142.9 232.9,136.3 269.1,117.3 305.3,109.7 341.5,90.2 377.6,80.5 413.8,63.2 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.66 µs | 1.76 µs | 3.48 µs | 2.32 µs | 3.44 µs |
| D38 | 1.43 µs | 3.47 µs | 3.96 µs | 3.86 µs | 3.14 µs |
| D57 | 4.47 µs | 5.77 µs | 7.56 µs | 8.47 µs | 10.9 µs |
| D76 | 6.3 µs | 3.97 µs | 4.68 µs | 10.7 µs | 13.5 µs |
| D115 | 13.4 µs | 11.5 µs | 12.6 µs | 19.8 µs | 27.5 µs |
| D153 | 6.07 µs | 6.87 µs | 17.5 µs | 18.1 µs | 33.1 µs |
| D230 | 4.52 µs | 14.6 µs | 24.2 µs | 44.3 µs | 79.5 µs |
| D307 | 4.66 µs | 13.2 µs | 55.2 µs | 81.5 µs | 100 µs |
| D462 | 5.59 µs | 25 µs | 83.8 µs | 170 µs | 254 µs |
| D616 | 6.5 µs | 37.3 µs | 115 µs | 265 µs | 356 µs |
| D924 | 6.58 µs | 84.4 µs | 266 µs | 482 µs | 724 µs |
| D1232 | 7.57 µs | 145 µs | 413 µs | 941 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.0 88.2,202.3 124.4,177.5 160.5,170.0 196.7,153.7 232.9,170.8 269.1,177.2 305.3,176.6 341.5,172.6 377.6,169.3 413.8,169.1 450.0,166.0 450.0,37.3 413.8,67.0 377.6,82.4 341.5,89.8 305.3,110.0 269.1,115.0 232.9,134.0 196.7,138.0 160.5,153.4 124.4,158.1 88.2,185.1 52.0,183.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,202.3 124.4,177.5 160.5,170.0 196.7,153.7 232.9,170.8 269.1,177.2 305.3,176.6 341.5,172.6 377.6,169.3 413.8,169.1 450.0,166.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.8 88.2,183.0 124.4,171.9 160.5,180.0 196.7,156.9 232.9,168.2 269.1,151.7 305.3,153.9 341.5,140.1 377.6,131.4 413.8,113.7 450.0,101.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.9 88.2,180.1 124.4,166.1 160.5,176.5 196.7,155.0 232.9,147.8 269.1,140.9 305.3,122.9 341.5,113.8 377.6,107.0 413.8,88.7 450.0,79.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.7 88.2,180.7 124.4,163.6 160.5,158.5 196.7,145.2 232.9,147.1 269.1,127.7 305.3,114.4 341.5,98.5 377.6,88.8 413.8,75.8 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.2 88.2,185.1 124.4,158.1 160.5,153.4 196.7,138.0 232.9,134.0 269.1,115.0 305.3,110.0 341.5,89.8 377.6,82.4 413.8,67.0 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 134 ns | 134 ns | 159 ns | 103 ns | 139 ns |
| D38 | 102 ns | 160 ns | 164 ns | 158 ns | 127 ns |
| D57 | 163 ns | 304 ns | 323 ns | 326 ns | 415 ns |
| D76 | 267 ns | 170 ns | 189 ns | 416 ns | 485 ns |
| D115 | 478 ns | 462 ns | 683 ns | 725 ns | 851 ns |
| D153 | 503 ns | 548 ns | 812 ns | 604 ns | 851 ns |
| D230 | 462 ns | 825 ns | 989 ns | 1.29 µs | 1.72 µs |
| D307 | 649 ns | 886 ns | 1.69 µs | 2.11 µs | 2.23 µs |
| D462 | 905 ns | 1.45 µs | 2.22 µs | 3.29 µs | 4.22 µs |
| D616 | 1.25 µs | 1.89 µs | 2.53 µs | 4.03 µs | 4.84 µs |
| D924 | 1.4 µs | 2.8 µs | 4.35 µs | 5.96 µs | 8.22 µs |
| D1232 | 2.65 µs | 4.41 µs | 6.65 µs | 11.4 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.4 88.2,209.5 124.4,195.9 160.5,181.6 196.7,164.7 232.9,163.2 269.1,165.7 305.3,155.9 341.5,146.2 377.6,136.8 413.8,133.7 450.0,115.1 450.0,44.5 413.8,82.3 377.6,97.7 341.5,101.6 305.3,120.1 269.1,127.7 232.9,148.0 196.7,148.0 160.5,164.3 124.4,168.8 88.2,203.1 52.0,200.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.4 88.2,209.5 124.4,195.9 160.5,181.6 196.7,164.7 232.9,163.2 269.1,165.7 305.3,155.9 341.5,146.2 377.6,136.8 413.8,133.7 450.0,115.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,201.5 88.2,196.3 124.4,177.8 160.5,194.6 196.7,165.7 232.9,160.7 269.1,148.9 305.3,146.8 341.5,132.6 377.6,124.9 413.8,113.6 450.0,100.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.5 88.2,195.6 124.4,176.0 160.5,191.6 196.7,154.4 232.9,149.4 269.1,143.7 305.3,128.2 341.5,120.2 377.6,116.4 413.8,100.8 450.0,88.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,209.1 88.2,196.8 124.4,175.8 160.5,168.7 196.7,152.6 232.9,157.9 269.1,135.9 305.3,121.6 341.5,108.9 377.6,103.0 413.8,91.6 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.5 88.2,203.1 124.4,168.8 160.5,164.3 196.7,148.0 232.9,148.0 269.1,127.7 305.3,120.1 341.5,101.6 377.6,97.7 413.8,82.3 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 156 ns | 155 ns | 195 ns | 144 ns | 180 ns |
| D38 | 134 ns | 191 ns | 198 ns | 178 ns | 148 ns |
| D57 | 206 ns | 384 ns | 387 ns | 408 ns | 486 ns |
| D76 | 338 ns | 203 ns | 228 ns | 497 ns | 569 ns |
| D115 | 635 ns | 534 ns | 827 ns | 850 ns | 957 ns |
| D153 | 649 ns | 639 ns | 932 ns | 663 ns | 958 ns |
| D230 | 586 ns | 997 ns | 1.13 µs | 1.5 µs | 1.89 µs |
| D307 | 861 ns | 1.06 µs | 1.98 µs | 2.45 µs | 2.51 µs |
| D462 | 1.22 µs | 1.7 µs | 2.55 µs | 3.69 µs | 4.76 µs |
| D616 | 1.64 µs | 2.16 µs | 2.89 µs | 4.4 µs | 5.26 µs |
| D924 | 1.83 µs | 3.31 µs | 4.83 µs | 6.45 µs | 8.7 µs |
| D1232 | 3.29 µs | 5.05 µs | 7.25 µs | 12.3 µs | 31.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.1 88.2,201.6 124.4,189.1 160.5,174.7 196.7,156.5 232.9,155.8 269.1,158.8 305.3,147.7 341.5,137.5 377.6,129.0 413.8,125.8 450.0,108.8 450.0,43.7 413.8,80.7 377.6,95.2 341.5,98.2 305.3,116.7 269.1,124.9 232.9,144.6 196.7,144.6 160.5,159.7 124.4,164.2 88.2,198.7 52.0,193.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.1 88.2,201.6 124.4,189.1 160.5,174.7 196.7,156.5 232.9,155.8 269.1,158.8 305.3,147.7 341.5,137.5 377.6,129.0 413.8,125.8 450.0,108.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.3 88.2,191.2 124.4,171.1 160.5,189.5 196.7,161.5 232.9,156.3 269.1,143.4 305.3,141.7 341.5,128.0 377.6,121.0 413.8,108.7 450.0,96.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.6 88.2,190.2 124.4,170.8 160.5,186.1 196.7,148.8 232.9,145.4 269.1,139.8 305.3,123.5 341.5,116.2 377.6,112.6 413.8,97.8 450.0,86.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,199.4 88.2,193.3 124.4,169.3 160.5,163.6 196.7,148.1 232.9,155.2 269.1,131.6 305.3,117.4 341.5,105.6 377.6,100.4 413.8,89.4 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,193.0 88.2,198.7 124.4,164.2 160.5,159.7 196.7,144.6 232.9,144.6 269.1,124.9 305.3,116.7 341.5,98.2 377.6,95.2 413.8,80.7 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
