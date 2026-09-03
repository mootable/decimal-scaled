# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.86 ns | 2 µs | 3.23 µs | 3.49 µs | 3.78 µs |
| D38 | 1.43 µs | 4.63 µs | 3.27 µs | 8.09 µs | 9.71 µs |
| D57 | 1.05 µs | 5.91 µs | 6.93 µs | 11.2 µs | 14.3 µs |
| D76 | 1.44 µs | 6.74 µs | 10.6 µs | 11.8 µs | 13.1 µs |
| D115 | 1.54 µs | 8.71 µs | 12.5 µs | 28.8 µs | 41.6 µs |
| D153 | 1.52 µs | 9.7 µs | 16 µs | 41.6 µs | 65.2 µs |
| D230 | 1.46 µs | 19.3 µs | 26 µs | 69.5 µs | 123 µs |
| D307 | 1.56 µs | 25.7 µs | 66.3 µs | 107 µs | 179 µs |
| D462 | 1.54 µs | 45.1 µs | 125 µs | 241 µs | 379 µs |
| D616 | 1.15 µs | 54.5 µs | 195 µs | 365 µs | 664 µs |
| D924 | 1.66 µs | 107 µs | 449 µs | 574 µs | 1.54 ms |
| D1232 | 1.99 µs | 217 µs | 662 µs | 1.45 ms | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,119.9 124.4,123.7 160.5,119.8 196.7,118.9 232.9,119.1 269.1,119.6 305.3,118.8 341.5,118.9 377.6,122.5 413.8,118.0 450.0,115.8 450.0,25.7 413.8,33.2 377.6,43.6 341.5,50.6 305.3,59.9 269.1,64.6 232.9,72.4 196.7,78.0 160.5,92.4 124.4,91.2 88.2,96.1 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,119.9 124.4,123.7 160.5,119.8 196.7,118.9 232.9,119.1 269.1,119.6 305.3,118.8 341.5,118.9 377.6,122.5 413.8,118.0 450.0,115.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.3 124.4,102.2 160.5,100.6 196.7,97.4 232.9,96.1 269.1,87.5 305.3,84.0 341.5,77.0 377.6,74.7 413.8,66.3 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,109.6 124.4,100.3 160.5,95.0 196.7,92.9 232.9,89.9 269.1,83.9 305.3,72.2 341.5,64.4 377.6,58.8 413.8,48.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.3 124.4,94.3 160.5,93.6 196.7,82.6 232.9,78.0 269.1,71.7 305.3,66.4 341.5,56.2 377.6,51.1 413.8,45.5 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,96.1 124.4,91.2 160.5,92.4 196.7,78.0 232.9,72.4 269.1,64.6 305.3,59.9 341.5,50.6 377.6,43.6 413.8,33.2 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.23 µs | 3.59 µs | 5.84 µs | 6.4 µs | 6.88 µs |
| D38 | 3.39 µs | 5.67 µs | 6.46 µs | 8.36 µs | 9.7 µs |
| D57 | 2.33 µs | 4.26 µs | 4.18 µs | 6.59 µs | 7.55 µs |
| D76 | 3.48 µs | 4.56 µs | 6.31 µs | 6.5 µs | 6.97 µs |
| D115 | 6.53 µs | 9.55 µs | 7.82 µs | 14.8 µs | 21.3 µs |
| D153 | 6.24 µs | 9.61 µs | 9.33 µs | 21.3 µs | 33.4 µs |
| D230 | 8.45 µs | 16.1 µs | 16.1 µs | 42.5 µs | 69.8 µs |
| D307 | 13.2 µs | 26.8 µs | 51 µs | 70.1 µs | 134 µs |
| D462 | 12.7 µs | 38.8 µs | 86.6 µs | 163 µs | 265 µs |
| D616 | 15.5 µs | 60.4 µs | 157 µs | 266 µs | 522 µs |
| D924 | 32.6 µs | 126 µs | 402 µs | 498 µs | 1.35 ms |
| D1232 | 43.7 µs | 274 µs | 681 µs | 1.38 ms | 2.55 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,183.5 124.4,191.6 160.5,182.9 196.7,169.3 232.9,170.2 269.1,163.7 305.3,154.0 341.5,154.7 377.6,150.5 413.8,134.3 450.0,128.0 450.0,39.6 413.8,53.4 377.6,74.1 341.5,88.8 305.3,103.7 269.1,117.8 232.9,133.8 196.7,143.6 160.5,167.8 124.4,166.1 88.2,160.7 52.0,168.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,183.5 124.4,191.6 160.5,182.9 196.7,169.3 232.9,170.2 269.1,163.7 305.3,154.0 341.5,154.7 377.6,150.5 413.8,134.3 450.0,128.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.2 88.2,172.3 124.4,178.5 160.5,177.1 196.7,161.0 232.9,160.9 269.1,149.7 305.3,138.6 341.5,130.6 377.6,121.0 413.8,105.0 450.0,88.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.7 88.2,169.5 124.4,179.0 160.5,170.0 196.7,165.3 232.9,161.5 269.1,149.6 305.3,124.6 341.5,113.1 377.6,100.2 413.8,79.8 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.9 124.4,169.0 160.5,169.3 196.7,151.5 232.9,143.6 269.1,128.6 305.3,117.7 341.5,99.4 377.6,88.8 413.8,75.1 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.1 88.2,160.7 124.4,166.1 160.5,167.8 196.7,143.6 232.9,133.8 269.1,117.8 305.3,103.7 341.5,88.8 377.6,74.1 413.8,53.4 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.98 µs | 3.21 µs | 3.48 µs | 3.77 µs |
| D38 | 1.33 µs | 4.61 µs | 3.21 µs | 8.05 µs | 9.7 µs |
| D57 | 945 ns | 5.86 µs | 6.89 µs | 11.2 µs | 14.3 µs |
| D76 | 1.34 µs | 6.69 µs | 10.6 µs | 11.8 µs | 13.2 µs |
| D115 | 1.43 µs | 8.68 µs | 12.4 µs | 29.3 µs | 41.8 µs |
| D153 | 1.38 µs | 9.77 µs | 15.5 µs | 41.9 µs | 65.1 µs |
| D230 | 1.35 µs | 19.9 µs | 25.9 µs | 69 µs | 124 µs |
| D307 | 1.46 µs | 24.9 µs | 65.8 µs | 107 µs | 179 µs |
| D462 | 1.45 µs | 46.3 µs | 125 µs | 241 µs | 380 µs |
| D616 | 1.09 µs | 54.9 µs | 195 µs | 366 µs | 661 µs |
| D924 | 1.53 µs | 112 µs | 451 µs | 578 µs | 1.54 ms |
| D1232 | 1.85 µs | 218 µs | 662 µs | 1.45 ms | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,120.8 124.4,125.0 160.5,120.7 196.7,119.8 232.9,120.3 269.1,120.6 305.3,119.6 341.5,119.7 377.6,123.3 413.8,119.0 450.0,116.6 450.0,25.6 413.8,33.2 377.6,43.7 341.5,50.6 305.3,59.9 269.1,64.5 232.9,72.5 196.7,78.0 160.5,92.3 124.4,91.3 88.2,96.1 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,120.8 124.4,125.0 160.5,120.7 196.7,119.8 232.9,120.3 269.1,120.6 305.3,119.6 341.5,119.7 377.6,123.3 413.8,119.0 450.0,116.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.3 124.4,102.4 160.5,100.7 196.7,97.5 232.9,96.0 269.1,87.2 305.3,84.4 341.5,76.7 377.6,74.6 413.8,65.8 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,109.8 124.4,100.3 160.5,95.0 196.7,93.1 232.9,90.2 269.1,83.9 305.3,72.3 341.5,64.4 377.6,58.9 413.8,48.4 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,94.3 160.5,93.7 196.7,82.4 232.9,77.9 269.1,71.7 305.3,66.3 341.5,56.2 377.6,51.0 413.8,45.4 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,96.1 124.4,91.3 160.5,92.3 196.7,78.0 232.9,72.5 269.1,64.5 305.3,59.9 341.5,50.6 377.6,43.7 413.8,33.2 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.72 µs | 2.52 µs | 2.82 µs | 2.97 µs |
| D38 | 4.22 ns | 2.48 µs | 2.8 µs | 3.71 µs | 4.29 µs |
| D57 | 1.09 ns | 5.47 µs | 6.18 µs | 9.64 µs | 11.8 µs |
| D76 | 2.34 ns | 6.3 µs | 9.45 µs | 9.84 µs | 11 µs |
| D115 | 12.4 ns | 13.7 µs | 12.7 µs | 22.5 µs | 33.6 µs |
| D153 | 16.5 ns | 14.9 µs | 14.5 µs | 33.1 µs | 47.2 µs |
| D230 | 27.4 ns | 23.9 µs | 24.9 µs | 62.3 µs | 96 µs |
| D307 | 54.5 ns | 37.7 µs | 71.6 µs | 96.7 µs | 167 µs |
| D462 | 69.9 ns | 58.6 µs | 119 µs | 198 µs | 318 µs |
| D616 | 61.3 ns | 87.7 µs | 208 µs | 339 µs | 613 µs |
| D924 | 104 ns | 188 µs | 526 µs | 561 µs | 1.47 ms |
| D1232 | 154 ns | 385 µs | 858 µs | 1.45 ms | 2.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,209.0 160.5,199.5 196.7,178.7 232.9,175.2 269.1,168.9 305.3,160.4 341.5,157.3 377.6,158.9 413.8,152.4 450.0,147.5 450.0,27.6 413.8,33.8 377.6,44.6 341.5,52.8 305.3,60.8 269.1,67.6 232.9,76.5 196.7,80.7 160.5,94.5 124.4,93.7 88.2,106.2 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,209.0 160.5,199.5 196.7,178.7 232.9,175.2 269.1,168.9 305.3,160.4 341.5,157.3 377.6,158.9 413.8,152.4 450.0,147.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,113.0 124.4,103.2 160.5,101.4 196.7,91.8 232.9,90.8 269.1,84.9 305.3,79.3 341.5,73.8 377.6,68.8 413.8,59.3 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,111.5 124.4,101.7 160.5,96.4 196.7,92.8 232.9,91.1 269.1,84.4 305.3,71.3 341.5,65.0 377.6,58.0 413.8,46.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,108.0 124.4,96.2 160.5,95.9 196.7,85.6 232.9,80.9 269.1,73.0 305.3,67.6 341.5,58.7 377.6,52.0 413.8,45.8 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.2 124.4,93.7 160.5,94.5 196.7,80.7 232.9,76.5 269.1,67.6 305.3,60.8 341.5,52.8 377.6,44.6 413.8,33.8 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.57 µs | 2.7 µs | 2.95 µs | 3.23 µs |
| D38 | 717 ns | 3.76 µs | 2.32 µs | 6.28 µs | 7.69 µs |
| D57 | 347 ns | 4.55 µs | 5.3 µs | 8.89 µs | 5.3 µs |
| D76 | 536 ns | 5.38 µs | 8.31 µs | 9.74 µs | 10.8 µs |
| D115 | 590 ns | 6.74 µs | 10.4 µs | 26.5 µs | 38.2 µs |
| D153 | 572 ns | 7.66 µs | 10.2 µs | 37.8 µs | 60.4 µs |
| D230 | 565 ns | 16.7 µs | 24.2 µs | 64.5 µs | 115 µs |
| D307 | 612 ns | 21.3 µs | 53.5 µs | 99.1 µs | 168 µs |
| D462 | 612 ns | 36.4 µs | 105 µs | 212 µs | 326 µs |
| D616 | 509 ns | 49.9 µs | 184 µs | 341 µs | 623 µs |
| D924 | 725 ns | 102 µs | 423 µs | 545 µs | 1.46 ms |
| D1232 | 1.02 µs | 203 µs | 624 µs | 1.4 ms | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,128.4 124.4,137.4 160.5,132.0 196.7,130.8 232.9,131.2 269.1,131.4 305.3,130.4 341.5,130.4 377.6,132.7 413.8,128.3 450.0,124.1 450.0,26.3 413.8,33.9 377.6,44.4 341.5,52.5 305.3,60.7 269.1,65.5 232.9,73.4 196.7,79.1 160.5,94.7 124.4,103.6 88.2,99.0 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,128.4 124.4,137.4 160.5,132.0 196.7,130.8 232.9,131.2 269.1,131.4 305.3,130.4 341.5,130.4 377.6,132.7 413.8,128.3 450.0,124.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,107.8 124.4,105.5 160.5,103.4 196.7,100.6 232.9,99.0 269.1,89.3 305.3,86.3 341.5,79.7 377.6,75.8 413.8,66.9 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,113.8 124.4,103.6 160.5,98.0 196.7,95.2 232.9,95.5 269.1,84.8 305.3,74.9 341.5,66.5 377.6,59.6 413.8,49.2 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,101.5 124.4,97.2 160.5,96.0 196.7,83.6 232.9,79.2 269.1,72.6 305.3,67.3 341.5,57.8 377.6,51.9 413.8,46.1 450.0,34.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,99.0 124.4,103.6 160.5,94.7 196.7,79.1 232.9,73.4 269.1,65.5 305.3,60.7 341.5,52.5 377.6,44.4 413.8,33.9 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.73 µs | 2.86 µs | 3.16 µs | 3.44 µs |
| D38 | 4.22 ns | 2.78 µs | 3.26 µs | 4.16 µs | 4.88 µs |
| D57 | 276 ns | 5.54 µs | 5.84 µs | 9.34 µs | 11.3 µs |
| D76 | 436 ns | 6.08 µs | 8.71 µs | 9.38 µs | 10.5 µs |
| D115 | 1.04 µs | 12.8 µs | 11.3 µs | 22.3 µs | 33.3 µs |
| D153 | 894 ns | 13.8 µs | 13.9 µs | 33.7 µs | 53.6 µs |
| D230 | 1.33 µs | 23.1 µs | 23.5 µs | 72 µs | 119 µs |
| D307 | 2.03 µs | 41 µs | 84 µs | 120 µs | 238 µs |
| D462 | 2.01 µs | 60 µs | 149 µs | 291 µs | 482 µs |
| D616 | 2.25 µs | 97.5 µs | 278 µs | 469 µs | 956 µs |
| D924 | 4.87 µs | 210 µs | 715 µs | 913 µs | 2.53 ms |
| D1232 | 6.67 µs | 469 µs | 1.23 ms | 2.57 ms | 4.78 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,140.3 160.5,134.6 196.7,123.8 232.9,125.7 269.1,120.7 305.3,115.5 341.5,115.6 377.6,114.2 413.8,104.6 450.0,100.7 450.0,19.2 413.8,27.0 377.6,39.1 341.5,47.6 305.3,56.4 269.1,65.0 232.9,74.9 196.7,80.8 160.5,95.1 124.4,94.1 88.2,104.6 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,140.3 160.5,134.6 196.7,123.8 232.9,125.7 269.1,120.7 305.3,115.5 341.5,115.6 377.6,114.2 413.8,104.6 450.0,100.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,111.6 124.4,103.0 160.5,101.9 196.7,92.6 232.9,91.7 269.1,85.3 305.3,78.2 341.5,73.5 377.6,67.5 413.8,58.0 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,109.6 124.4,102.4 160.5,97.4 196.7,94.2 232.9,91.6 269.1,85.1 305.3,69.3 341.5,62.2 377.6,54.5 413.8,42.7 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,96.6 160.5,96.5 196.7,85.8 232.9,80.6 269.1,71.2 305.3,64.9 341.5,53.9 377.6,48.0 413.8,39.7 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.6 124.4,94.1 160.5,95.1 196.7,80.8 232.9,74.9 269.1,65.0 305.3,56.4 341.5,47.6 377.6,39.1 413.8,27.0 450.0,19.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.29 ns | 1.19 µs | 2.52 µs | 2.63 µs | 2.85 µs |
| D38 | 5.63 ns | 2.12 µs | 2.42 µs | 3.53 µs | 3.9 µs |
| D57 | 1.66 ns | 3.51 µs | 3.88 µs | 5.99 µs | 8.98 µs |
| D76 | 3.12 ns | 3.87 µs | 5.98 µs | 6.06 µs | 6.36 µs |
| D115 | 13 ns | 5.04 µs | 6.08 µs | 12.2 µs | 18.1 µs |
| D153 | 16.8 ns | 5.41 µs | 6.31 µs | 18 µs | 32.8 µs |
| D230 | 36.9 ns | 9.87 µs | 11 µs | 36.2 µs | 69.8 µs |
| D307 | 73.2 ns | 12.5 µs | 27.1 µs | 60 µs | 110 µs |
| D462 | 114 ns | 16.6 µs | 61.8 µs | 134 µs | 223 µs |
| D616 | 116 ns | 26.8 µs | 121 µs | 224 µs | 428 µs |
| D924 | 142 ns | 58.1 µs | 274 µs | 397 µs | 1.05 ms |
| D1232 | 378 ns | 129 µs | 430 µs | 1.04 ms | 2.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.3 88.2,188.6 124.4,203.7 160.5,195.9 196.7,178.2 232.9,175.0 269.1,165.2 305.3,156.7 341.5,151.2 377.6,151.0 413.8,148.5 450.0,136.3 450.0,29.2 413.8,37.9 377.6,49.1 341.5,57.2 305.3,65.9 269.1,71.6 232.9,81.0 196.7,88.4 160.5,101.3 124.4,97.0 88.2,107.4 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.3 88.2,188.6 124.4,203.7 160.5,195.9 196.7,178.2 232.9,175.0 269.1,165.2 305.3,156.7 341.5,151.2 377.6,151.0 413.8,148.5 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.2 88.2,115.0 124.4,108.7 160.5,107.5 196.7,104.2 232.9,103.3 269.1,95.9 305.3,92.9 341.5,89.4 377.6,83.5 413.8,73.9 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,113.3 124.4,107.5 160.5,102.1 196.7,101.9 232.9,101.4 269.1,94.5 305.3,83.4 341.5,73.1 377.6,64.8 413.8,54.6 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,108.6 124.4,102.1 160.5,101.9 196.7,93.2 232.9,88.4 269.1,79.8 305.3,73.5 341.5,63.5 377.6,57.2 413.8,50.0 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,107.4 124.4,97.0 160.5,101.3 196.7,88.4 232.9,81.0 269.1,71.6 305.3,65.9 341.5,57.2 377.6,49.1 413.8,37.9 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.74 ns | 1.73 µs | 3.32 µs | 3.57 µs | 3.77 µs |
| D38 | 4.57 ns | 2.93 µs | 3.32 µs | 4.47 µs | 4.95 µs |
| D57 | 2.52 ns | 5.57 µs | 6.02 µs | 8.56 µs | 10.4 µs |
| D76 | 3.43 ns | 5.91 µs | 8.12 µs | 8.58 µs | 8.87 µs |
| D115 | 10.9 ns | 13 µs | 7.66 µs | 18.9 µs | 26.6 µs |
| D153 | 16.5 ns | 7.98 µs | 9.91 µs | 22.9 µs | 39.3 µs |
| D230 | 37.3 ns | 14.1 µs | 14.2 µs | 43.7 µs | 80.5 µs |
| D307 | 75.7 ns | 16.5 µs | 52.6 µs | 68 µs | 119 µs |
| D462 | 109 ns | 25 µs | 81.8 µs | 158 µs | 235 µs |
| D616 | 122 ns | 32.2 µs | 126 µs | 237 µs | 418 µs |
| D924 | 180 ns | 69.8 µs | 290 µs | 389 µs | 923 µs |
| D1232 | 375 ns | 143 µs | 415 µs | 909 µs | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,191.1 124.4,198.5 160.5,194.7 196.7,180.4 232.9,175.2 269.1,165.1 305.3,156.3 341.5,151.7 377.6,150.4 413.8,145.6 450.0,136.5 450.0,27.5 413.8,39.6 377.6,49.4 341.5,56.5 305.3,64.9 269.1,69.8 232.9,78.7 196.7,83.6 160.5,97.2 124.4,95.2 88.2,104.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,191.1 124.4,198.5 160.5,194.7 196.7,180.4 232.9,175.2 269.1,165.1 305.3,156.3 341.5,151.7 377.6,150.4 413.8,145.6 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,110.9 124.4,103.0 160.5,102.2 196.7,92.5 232.9,98.5 269.1,91.5 305.3,89.5 341.5,84.4 377.6,81.2 413.8,71.6 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,109.4 124.4,102.0 160.5,98.3 196.7,99.0 232.9,95.8 269.1,91.4 305.3,75.1 341.5,69.6 377.6,64.3 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.7 124.4,97.6 160.5,97.6 196.7,87.8 232.9,85.4 269.1,77.4 305.3,71.9 341.5,61.4 377.6,56.4 413.8,50.3 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,104.4 124.4,95.2 160.5,97.2 196.7,83.6 232.9,78.7 269.1,69.8 305.3,64.9 341.5,56.5 377.6,49.4 413.8,39.6 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.07 µs | 2.35 µs | 2.57 µs | 2.79 µs |
| D38 | 4.92 ns | 1.97 µs | 2.26 µs | 3.35 µs | 3.85 µs |
| D57 | 1.93 ns | 3.27 µs | 3.67 µs | 5.95 µs | 8.98 µs |
| D76 | 3.43 ns | 3.54 µs | 5.61 µs | 6.01 µs | 6.14 µs |
| D115 | 13 ns | 4.82 µs | 6.03 µs | 12.3 µs | 17.7 µs |
| D153 | 17.1 ns | 5.06 µs | 5.98 µs | 17.6 µs | 32.1 µs |
| D230 | 37.2 ns | 10.2 µs | 11.4 µs | 35.9 µs | 66.3 µs |
| D307 | 66.7 ns | 12 µs | 26.1 µs | 58.1 µs | 108 µs |
| D462 | 108 ns | 16.1 µs | 58.6 µs | 135 µs | 220 µs |
| D616 | 111 ns | 26.9 µs | 116 µs | 221 µs | 428 µs |
| D924 | 164 ns | 58.2 µs | 270 µs | 419 µs | 1.05 ms |
| D1232 | 364 ns | 128 µs | 427 µs | 1.04 ms | 2.1 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.2 124.4,201.9 160.5,194.7 196.7,178.2 232.9,174.7 269.1,165.1 305.3,157.9 341.5,152.0 377.6,151.5 413.8,146.7 450.0,136.8 450.0,29.3 413.8,37.9 377.6,49.1 341.5,57.4 305.3,66.2 269.1,72.2 232.9,81.2 196.7,88.6 160.5,101.8 124.4,97.1 88.2,107.6 52.0,111.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.2 124.4,201.9 160.5,194.7 196.7,178.2 232.9,174.7 269.1,165.1 305.3,157.9 341.5,152.0 377.6,151.5 413.8,146.7 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.5 88.2,115.9 124.4,109.6 160.5,108.6 196.7,104.8 232.9,104.2 269.1,95.4 305.3,93.4 341.5,89.8 377.6,83.4 413.8,73.9 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.7 88.2,114.2 124.4,108.2 160.5,102.9 196.7,102.0 232.9,102.1 269.1,94.1 305.3,83.8 341.5,73.8 377.6,65.3 413.8,54.8 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,109.3 124.4,102.2 160.5,102.0 196.7,93.1 232.9,88.7 269.1,79.9 305.3,73.9 341.5,63.5 377.6,57.3 413.8,49.4 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,107.6 124.4,97.1 160.5,101.8 196.7,88.6 232.9,81.2 269.1,72.2 305.3,66.2 341.5,57.4 377.6,49.1 413.8,37.9 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.73 µs | 3.33 µs | 3.56 µs | 3.78 µs |
| D38 | 4.57 ns | 2.94 µs | 3.32 µs | 4.47 µs | 4.96 µs |
| D57 | 8.04 ns | 5.58 µs | 6.08 µs | 8.58 µs | 10.4 µs |
| D76 | 10.4 ns | 5.98 µs | 8.18 µs | 8.56 µs | 8.97 µs |
| D115 | 11.2 ns | 12.9 µs | 7.48 µs | 19.1 µs | 26.6 µs |
| D153 | 15.9 ns | 8.04 µs | 9.86 µs | 23 µs | 39.2 µs |
| D230 | 37.1 ns | 14 µs | 13.7 µs | 43.6 µs | 80.7 µs |
| D307 | 65.2 ns | 16.5 µs | 53.1 µs | 68.1 µs | 119 µs |
| D462 | 109 ns | 25 µs | 81.5 µs | 159 µs | 236 µs |
| D616 | 111 ns | 32 µs | 126 µs | 238 µs | 415 µs |
| D924 | 180 ns | 68.6 µs | 290 µs | 410 µs | 923 µs |
| D1232 | 386 ns | 143 µs | 415 µs | 908 µs | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,191.1 124.4,184.1 160.5,180.9 196.7,180.0 232.9,175.7 269.1,165.2 305.3,158.2 341.5,151.8 377.6,151.5 413.8,145.5 450.0,136.1 450.0,27.5 413.8,39.6 377.6,49.5 341.5,56.5 305.3,65.0 269.1,69.8 232.9,78.8 196.7,83.6 160.5,97.1 124.4,95.2 88.2,104.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,191.1 124.4,184.1 160.5,180.9 196.7,180.0 232.9,175.7 269.1,165.2 305.3,158.2 341.5,151.8 377.6,151.5 413.8,145.5 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,110.9 124.4,103.0 160.5,102.1 196.7,92.6 232.9,98.4 269.1,91.5 305.3,89.5 341.5,84.3 377.6,81.3 413.8,71.8 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,109.4 124.4,101.9 160.5,98.2 196.7,99.3 232.9,95.9 269.1,91.8 305.3,75.0 341.5,69.7 377.6,64.3 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.7 124.4,97.6 160.5,97.6 196.7,87.7 232.9,85.4 269.1,77.4 305.3,71.9 341.5,61.4 377.6,56.4 413.8,49.6 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,104.4 124.4,95.2 160.5,97.1 196.7,83.6 232.9,78.8 269.1,69.8 305.3,65.0 341.5,56.5 377.6,49.5 413.8,39.6 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.13 µs | 3.99 µs | 4.25 µs | 4.61 µs |
| D38 | 4.92 ns | 3.6 µs | 4.18 µs | 5.59 µs | 6.26 µs |
| D57 | 2.22 ns | 4.41 µs | 4.87 µs | 7.81 µs | 9.29 µs |
| D76 | 3.74 ns | 4.65 µs | 7.5 µs | 7.86 µs | 7.62 µs |
| D115 | 10.4 ns | 6.35 µs | 7.7 µs | 14.9 µs | 21.2 µs |
| D153 | 16.4 ns | 6.74 µs | 7.74 µs | 21.3 µs | 37.2 µs |
| D230 | 36.2 ns | 12.2 µs | 13.8 µs | 40.8 µs | 75.7 µs |
| D307 | 62.7 ns | 14.6 µs | 31.2 µs | 63.5 µs | 120 µs |
| D462 | 134 ns | 19.9 µs | 67.4 µs | 148 µs | 238 µs |
| D616 | 111 ns | 30.3 µs | 126 µs | 242 µs | 462 µs |
| D924 | 148 ns | 66.5 µs | 297 µs | 423 µs | 1.11 ms |
| D1232 | 377 ns | 143 µs | 461 µs | 1.1 ms | 2.18 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,190.2 124.4,200.1 160.5,193.6 196.7,180.9 232.9,175.3 269.1,165.5 305.3,158.6 341.5,149.3 377.6,151.5 413.8,148.0 450.0,136.4 450.0,28.9 413.8,37.2 377.6,48.1 341.5,56.4 305.3,64.9 269.1,70.6 232.9,79.4 196.7,86.4 160.5,99.1 124.4,96.6 88.2,101.5 52.0,105.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,190.2 124.4,200.1 160.5,193.6 196.7,180.9 232.9,175.3 269.1,165.5 305.3,158.6 341.5,149.3 377.6,151.5 413.8,148.0 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,108.4 124.4,105.9 160.5,105.2 196.7,101.3 232.9,100.6 269.1,93.2 305.3,91.0 341.5,87.2 377.6,82.0 413.8,72.2 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,106.5 124.4,104.6 160.5,99.3 196.7,99.0 232.9,98.9 269.1,91.7 305.3,81.6 341.5,72.0 377.6,64.3 413.8,53.6 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,102.9 124.4,98.8 160.5,98.7 196.7,90.8 232.9,86.3 269.1,78.3 305.3,72.8 341.5,62.3 377.6,56.2 413.8,49.3 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,101.5 124.4,96.6 160.5,99.1 196.7,86.4 232.9,79.4 269.1,70.6 305.3,64.9 341.5,56.4 377.6,48.1 413.8,37.2 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.76 µs | 3.49 µs | 3.72 µs | 3.95 µs |
| D38 | 4.22 ns | 3.1 µs | 3.33 µs | 4.49 µs | 4.99 µs |
| D57 | 1.87 µs | 5.73 µs | 6.31 µs | 9 µs | 10.9 µs |
| D76 | 2.6 µs | 6.04 µs | 8.58 µs | 8.9 µs | 9.23 µs |
| D115 | 5.68 µs | 13.5 µs | 7.9 µs | 19.8 µs | 27.3 µs |
| D153 | 2.81 µs | 8.39 µs | 10.3 µs | 23.7 µs | 40.8 µs |
| D230 | 2.79 µs | 15.4 µs | 14.1 µs | 44.5 µs | 81.7 µs |
| D307 | 3.17 µs | 17.3 µs | 54.3 µs | 70.6 µs | 121 µs |
| D462 | 3.1 µs | 26 µs | 83.1 µs | 160 µs | 239 µs |
| D616 | 2.46 µs | 33.4 µs | 127 µs | 240 µs | 422 µs |
| D924 | 3.44 µs | 71.4 µs | 295 µs | 401 µs | 931 µs |
| D1232 | 4.24 µs | 145 µs | 419 µs | 915 µs | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,116.5 160.5,112.4 196.7,102.7 232.9,111.5 269.1,111.6 305.3,110.0 341.5,110.2 377.6,113.1 413.8,109.0 450.0,106.4 450.0,27.2 413.8,39.5 377.6,49.3 341.5,56.3 305.3,64.8 269.1,69.6 232.9,78.3 196.7,83.3 160.5,96.7 124.4,94.6 88.2,104.3 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,116.5 160.5,112.4 196.7,102.7 232.9,111.5 269.1,111.6 305.3,110.0 341.5,110.2 377.6,113.1 413.8,109.0 450.0,106.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,110.3 124.4,102.6 160.5,102.0 196.7,92.0 232.9,97.9 269.1,90.4 305.3,88.9 341.5,83.9 377.6,80.7 413.8,71.3 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,109.4 124.4,101.4 160.5,97.6 196.7,98.6 232.9,95.3 269.1,91.4 305.3,74.7 341.5,69.4 377.6,64.2 413.8,53.7 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,105.7 124.4,97.0 160.5,97.2 196.7,87.3 232.9,85.0 269.1,77.2 305.3,71.5 341.5,61.3 377.6,56.3 413.8,49.9 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,104.3 124.4,94.6 160.5,96.7 196.7,83.3 232.9,78.3 269.1,69.6 305.3,64.8 341.5,56.3 377.6,49.3 413.8,39.5 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 135 ns | 158 ns | 159 ns | 162 ns |
| D38 | 4.92 ns | 153 ns | 133 ns | 180 ns | 186 ns |
| D57 | 111 ns | 312 ns | 260 ns | 347 ns | 420 ns |
| D76 | 178 ns | 304 ns | 353 ns | 343 ns | 332 ns |
| D115 | 455 ns | 638 ns | 419 ns | 683 ns | 873 ns |
| D153 | 420 ns | 599 ns | 475 ns | 866 ns | 1.17 µs |
| D230 | 496 ns | 826 ns | 562 ns | 1.29 µs | 1.79 µs |
| D307 | 889 ns | 1.25 µs | 1.67 µs | 1.76 µs | 2.65 µs |
| D462 | 871 ns | 1.57 µs | 2.17 µs | 3.05 µs | 3.92 µs |
| D616 | 804 ns | 1.63 µs | 2.84 µs | 3.62 µs | 5.72 µs |
| D924 | 1.45 µs | 2.16 µs | 4.88 µs | 5.36 µs | 10.3 µs |
| D1232 | 2.21 µs | 4.25 µs | 7 µs | 10.7 µs | 26.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,182.3 124.4,128.2 160.5,120.0 196.7,103.7 232.9,105.1 269.1,102.2 305.3,92.0 341.5,92.4 377.6,93.8 413.8,83.6 450.0,76.2 450.0,33.0 413.8,49.4 377.6,59.7 341.5,66.3 305.3,73.1 269.1,79.9 232.9,87.3 196.7,92.4 160.5,109.2 124.4,105.1 88.2,119.3 52.0,121.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,182.3 124.4,128.2 160.5,120.0 196.7,103.7 232.9,105.1 269.1,102.2 305.3,92.0 341.5,92.4 377.6,93.8 413.8,83.6 450.0,76.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.8 88.2,122.6 124.4,110.2 160.5,110.7 196.7,97.8 232.9,98.9 269.1,93.3 305.3,86.2 341.5,82.1 377.6,81.5 413.8,76.6 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.1 88.2,125.0 124.4,113.4 160.5,108.1 196.7,105.1 232.9,102.9 269.1,100.0 305.3,81.1 341.5,76.6 377.6,71.9 413.8,62.5 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,119.8 124.4,108.4 160.5,108.6 196.7,96.6 232.9,92.5 269.1,85.6 305.3,80.2 341.5,70.6 377.6,67.7 413.8,60.8 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,119.3 124.4,105.1 160.5,109.2 196.7,92.4 232.9,87.3 269.1,79.9 305.3,73.1 341.5,66.3 377.6,59.7 413.8,49.4 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 154 ns | 196 ns | 198 ns | 198 ns |
| D38 | 4.92 ns | 189 ns | 185 ns | 197 ns | 201 ns |
| D57 | 172 ns | 402 ns | 340 ns | 470 ns | 528 ns |
| D76 | 283 ns | 410 ns | 476 ns | 432 ns | 416 ns |
| D115 | 689 ns | 848 ns | 508 ns | 873 ns | 1.08 µs |
| D153 | 629 ns | 812 ns | 564 ns | 1.08 µs | 1.43 µs |
| D230 | 848 ns | 1.17 µs | 715 ns | 1.64 µs | 2.17 µs |
| D307 | 1.42 µs | 1.76 µs | 2.21 µs | 2.26 µs | 3.23 µs |
| D462 | 1.47 µs | 2.03 µs | 2.77 µs | 3.67 µs | 4.58 µs |
| D616 | 1.25 µs | 2.12 µs | 3.4 µs | 4.3 µs | 6.38 µs |
| D924 | 2.33 µs | 2.82 µs | 5.84 µs | 5.29 µs | 11.4 µs |
| D1232 | 3.46 µs | 5.44 µs | 8.17 µs | 12 µs | 28.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,182.3 124.4,120.6 160.5,111.9 196.7,96.5 232.9,98.1 269.1,92.9 305.3,83.9 341.5,83.3 377.6,86.1 413.8,75.3 450.0,68.4 450.0,32.0 413.8,47.7 377.6,57.8 341.5,63.6 305.3,69.6 269.1,76.5 232.9,83.8 196.7,88.7 160.5,105.2 124.4,101.1 88.2,117.9 52.0,118.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,182.3 124.4,120.6 160.5,111.9 196.7,96.5 232.9,98.1 269.1,92.9 305.3,83.9 341.5,83.3 377.6,86.1 413.8,75.3 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.5 88.2,118.9 124.4,105.9 160.5,105.5 196.7,92.9 232.9,93.6 269.1,87.3 305.3,80.2 341.5,77.7 377.6,77.0 413.8,72.0 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,119.3 124.4,108.8 160.5,102.9 196.7,101.8 232.9,100.0 269.1,95.8 305.3,76.2 341.5,72.3 377.6,68.7 413.8,59.3 450.0,53.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.2 124.4,103.1 160.5,104.6 196.7,92.4 232.9,88.6 269.1,81.4 305.3,75.9 341.5,67.4 377.6,64.6 413.8,61.1 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,117.9 124.4,101.1 160.5,105.2 196.7,88.7 232.9,83.8 269.1,76.5 305.3,69.6 341.5,63.6 377.6,57.8 413.8,47.7 450.0,32.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
