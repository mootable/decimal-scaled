# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.06 ns | 2 µs | 3.47 µs | 3.74 µs | 2.93 µs |
| D38 | 782 ns | 4.61 µs | 4.56 µs | 7.93 µs | 10.5 µs |
| D57 | 1.47 µs | 3.7 µs | 8.19 µs | 10.5 µs | 14.2 µs |
| D76 | 1.44 µs | 5.85 µs | 10.6 µs | 11.9 µs | 12.9 µs |
| D115 | 1.23 µs | 6.81 µs | 12.3 µs | 28.9 µs | 42.4 µs |
| D153 | 1.59 µs | 10 µs | 25.5 µs | 45.3 µs | 60.8 µs |
| D230 | 1.65 µs | 19.6 µs | 46.5 µs | 75.5 µs | 123 µs |
| D307 | 1.45 µs | 15.7 µs | 42.5 µs | 124 µs | 181 µs |
| D462 | 1.56 µs | 43.1 µs | 127 µs | 243 µs | 343 µs |
| D616 | 1.38 µs | 61.8 µs | 172 µs | 277 µs | 659 µs |
| D924 | 1.67 µs | 126 µs | 422 µs | 939 µs | 1.52 ms |
| D1232 | 1.97 µs | 217 µs | 511 µs | 1.65 ms | 3.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.1 88.2,127.3 124.4,119.5 160.5,119.8 196.7,121.7 232.9,118.5 269.1,118.1 305.3,119.6 341.5,118.7 377.6,120.3 413.8,117.9 450.0,115.9 450.0,23.8 413.8,33.4 377.6,43.7 341.5,51.9 305.3,59.8 269.1,64.6 232.9,73.3 196.7,77.8 160.5,92.5 124.4,91.4 88.2,95.2 52.0,111.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.1 88.2,127.3 124.4,119.5 160.5,119.8 196.7,121.7 232.9,118.5 269.1,118.1 305.3,119.6 341.5,118.7 377.6,120.3 413.8,117.9 450.0,115.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.3 124.4,108.1 160.5,102.4 196.7,100.5 232.9,95.7 269.1,87.3 305.3,90.1 341.5,77.6 377.6,73.1 413.8,64.3 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,105.5 124.4,98.2 160.5,95.0 196.7,93.1 232.9,84.1 269.1,76.6 305.3,77.8 341.5,64.2 377.6,60.4 413.8,49.3 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,98.6 124.4,95.1 160.5,93.6 196.7,82.6 232.9,77.0 269.1,70.6 305.3,64.5 341.5,56.1 377.6,54.5 413.8,39.3 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,95.2 124.4,91.4 160.5,92.5 196.7,77.8 232.9,73.3 269.1,64.6 305.3,59.8 341.5,51.9 377.6,43.7 413.8,33.4 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.22 µs | 3.6 µs | 6.1 µs | 6.67 µs | 5.43 µs |
| D38 | 2.18 µs | 5.85 µs | 7.41 µs | 8.4 µs | 10.1 µs |
| D57 | 3.46 µs | 2.54 µs | 5.07 µs | 6.24 µs | 7.76 µs |
| D76 | 3.47 µs | 3.9 µs | 6.3 µs | 6.53 µs | 6.57 µs |
| D115 | 5.58 µs | 7.61 µs | 7.35 µs | 14.5 µs | 21 µs |
| D153 | 6.71 µs | 9.82 µs | 16.4 µs | 23.9 µs | 30.6 µs |
| D230 | 8.95 µs | 15.9 µs | 28.4 µs | 46 µs | 70.2 µs |
| D307 | 12.8 µs | 16.8 µs | 31.3 µs | 86 µs | 134 µs |
| D462 | 12.8 µs | 37.8 µs | 86.3 µs | 163 µs | 238 µs |
| D616 | 19.1 µs | 69.9 µs | 146 µs | 199 µs | 516 µs |
| D924 | 32.7 µs | 156 µs | 378 µs | 806 µs | 1.33 ms |
| D1232 | 43.7 µs | 273 µs | 508 µs | 1.55 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,193.0 124.4,183.1 160.5,183.0 196.7,172.7 232.9,168.7 269.1,162.4 305.3,154.7 341.5,154.6 377.6,146.0 413.8,134.3 450.0,128.0 450.0,37.0 413.8,53.8 377.6,74.4 341.5,91.1 305.3,103.6 269.1,117.7 232.9,135.7 196.7,143.9 160.5,169.1 124.4,165.5 88.2,159.7 52.0,173.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,193.0 124.4,183.1 160.5,183.0 196.7,172.7 232.9,168.7 269.1,162.4 305.3,154.7 341.5,154.6 377.6,146.0 413.8,134.3 450.0,128.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.2 88.2,171.6 124.4,189.8 160.5,180.4 196.7,165.9 232.9,160.4 269.1,149.9 305.3,148.8 341.5,131.1 377.6,117.8 413.8,100.4 450.0,88.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.7 88.2,166.5 124.4,174.7 160.5,170.0 196.7,166.7 232.9,149.3 269.1,137.3 305.3,135.2 341.5,113.2 377.6,101.7 413.8,81.1 450.0,74.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.8 88.2,163.8 124.4,170.3 160.5,169.3 196.7,151.9 232.9,141.1 269.1,126.8 305.3,113.3 341.5,99.3 377.6,95.1 413.8,64.7 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.3 88.2,159.7 124.4,165.5 160.5,169.1 196.7,143.9 232.9,135.7 269.1,117.7 305.3,103.6 341.5,91.1 377.6,74.4 413.8,53.8 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.99 µs | 3.46 µs | 3.72 µs | 2.9 µs |
| D38 | 711 ns | 4.6 µs | 4.49 µs | 7.85 µs | 10.4 µs |
| D57 | 1.32 µs | 3.68 µs | 8.14 µs | 10.5 µs | 14.1 µs |
| D76 | 1.35 µs | 5.87 µs | 10.5 µs | 11.8 µs | 12.9 µs |
| D115 | 1.18 µs | 6.79 µs | 12.1 µs | 28.5 µs | 41.8 µs |
| D153 | 1.49 µs | 10 µs | 25.8 µs | 45.4 µs | 61.4 µs |
| D230 | 1.51 µs | 19.5 µs | 46.1 µs | 75.8 µs | 123 µs |
| D307 | 1.33 µs | 16.2 µs | 42.1 µs | 124 µs | 181 µs |
| D462 | 1.43 µs | 43.6 µs | 127 µs | 244 µs | 340 µs |
| D616 | 1.31 µs | 60.9 µs | 170 µs | 277 µs | 661 µs |
| D924 | 1.59 µs | 126 µs | 421 µs | 950 µs | 1.52 ms |
| D1232 | 1.84 µs | 217 µs | 505 µs | 1.66 ms | 3.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,128.5 124.4,120.8 160.5,120.5 196.7,122.2 232.9,119.3 269.1,119.1 305.3,120.7 341.5,119.9 377.6,120.9 413.8,118.5 450.0,116.7 450.0,23.8 413.8,33.4 377.6,43.7 341.5,51.9 305.3,59.8 269.1,64.6 232.9,73.2 196.7,78.0 160.5,92.6 124.4,91.4 88.2,95.2 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,128.5 124.4,120.8 160.5,120.5 196.7,122.2 232.9,119.3 269.1,119.1 305.3,120.7 341.5,119.9 377.6,120.9 413.8,118.5 450.0,116.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.4 124.4,108.1 160.5,102.3 196.7,100.5 232.9,95.7 269.1,87.4 305.3,89.8 341.5,77.5 377.6,73.3 413.8,64.2 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,105.6 124.4,98.3 160.5,95.1 196.7,93.4 232.9,84.0 269.1,76.8 305.3,77.9 341.5,64.2 377.6,60.6 413.8,49.3 450.0,47.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,98.7 124.4,95.1 160.5,93.7 196.7,82.7 232.9,76.9 269.1,70.6 305.3,64.4 341.5,56.1 377.6,54.5 413.8,39.2 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,95.2 124.4,91.4 160.5,92.6 196.7,78.0 232.9,73.2 269.1,64.6 305.3,59.8 341.5,51.9 377.6,43.7 413.8,33.4 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.64 µs | 1.71 µs | 2.66 µs | 2.95 µs | 2.35 µs |
| D38 | 1.08 µs | 2.54 µs | 3.24 µs | 3.74 µs | 4.47 µs |
| D57 | 4.47 µs | 3.81 µs | 7.3 µs | 8.99 µs | 11.6 µs |
| D76 | 4.53 µs | 5.22 µs | 9.3 µs | 9.92 µs | 9.8 µs |
| D115 | 7.82 µs | 11.4 µs | 11.9 µs | 21 µs | 33.2 µs |
| D153 | 9.23 µs | 15 µs | 23.6 µs | 35.7 µs | 43.8 µs |
| D230 | 12.4 µs | 24.6 µs | 44.9 µs | 67.3 µs | 96.3 µs |
| D307 | 16.9 µs | 23.1 µs | 45.2 µs | 116 µs | 168 µs |
| D462 | 16.9 µs | 55.4 µs | 120 µs | 200 µs | 285 µs |
| D616 | 25.5 µs | 103 µs | 195 µs | 252 µs | 615 µs |
| D924 | 42.6 µs | 228 µs | 491 µs | 933 µs | 1.46 ms |
| D1232 | 57.4 µs | 385 µs | 648 µs | 1.77 ms | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.3 88.2,208.2 124.4,177.5 160.5,177.2 196.7,165.3 232.9,161.7 269.1,155.3 305.3,148.6 341.5,148.7 377.6,139.7 413.8,128.5 450.0,122.1 450.0,37.2 413.8,51.8 377.6,70.5 341.5,87.3 305.3,98.8 269.1,110.8 232.9,127.9 196.7,133.9 160.5,160.4 124.4,156.7 88.2,177.5 52.0,191.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.3 88.2,208.2 124.4,177.5 160.5,177.2 196.7,165.3 232.9,161.7 269.1,155.3 305.3,148.6 341.5,148.7 377.6,139.7 413.8,128.5 450.0,122.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.4 88.2,189.8 124.4,181.0 160.5,174.1 196.7,157.1 232.9,151.2 269.1,140.5 305.3,141.9 341.5,122.8 377.6,109.4 413.8,92.1 450.0,80.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.7 88.2,184.4 124.4,166.8 160.5,161.6 196.7,156.2 232.9,141.3 269.1,127.4 305.3,127.3 341.5,106.1 377.6,95.5 413.8,75.5 450.0,69.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.5 88.2,181.4 124.4,162.3 160.5,160.2 196.7,143.9 232.9,132.4 269.1,118.6 305.3,106.8 341.5,94.9 377.6,89.9 413.8,61.5 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.5 88.2,177.5 124.4,156.7 160.5,160.4 196.7,133.9 232.9,127.9 269.1,110.8 305.3,98.8 341.5,87.3 377.6,70.5 413.8,51.8 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.4 µs | 1.57 µs | 2.87 µs | 3.17 µs | 2.56 µs |
| D38 | 958 ns | 2.84 µs | 3.36 µs | 3.56 µs | 4.34 µs |
| D57 | 1.85 µs | 1.66 µs | 3.58 µs | 4.25 µs | 5.44 µs |
| D76 | 5.57 µs | 4.83 µs | 8.3 µs | 9.77 µs | 10.7 µs |
| D115 | 4.89 µs | 5.22 µs | 10.1 µs | 25.7 µs | 38.6 µs |
| D153 | 6.01 µs | 7.75 µs | 17.2 µs | 40.6 µs | 56.7 µs |
| D230 | 6.01 µs | 16.4 µs | 43.7 µs | 69.5 µs | 115 µs |
| D307 | 5.61 µs | 13.6 µs | 33.4 µs | 115 µs | 169 µs |
| D462 | 2.87 µs | 34.9 µs | 108 µs | 213 µs | 295 µs |
| D616 | 5.1 µs | 55.8 µs | 160 µs | 256 µs | 624 µs |
| D924 | 5.82 µs | 115 µs | 392 µs | 890 µs | 1.44 ms |
| D1232 | 6.02 µs | 203 µs | 478 µs | 1.58 ms | 3.16 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,154.8 88.2,170.7 124.4,159.3 160.5,140.2 196.7,142.4 232.9,138.9 269.1,138.8 305.3,140.0 341.5,151.7 377.6,141.7 413.8,139.4 450.0,138.8 450.0,30.0 413.8,43.7 377.6,58.2 341.5,71.2 305.3,80.8 269.1,87.5 232.9,99.9 196.7,106.5 160.5,128.8 124.4,140.6 88.2,144.5 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,154.8 88.2,170.7 124.4,159.3 160.5,140.2 196.7,142.4 232.9,138.9 269.1,138.8 305.3,140.0 341.5,151.7 377.6,141.7 413.8,139.4 450.0,138.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,162.2 88.2,151.8 124.4,161.2 160.5,142.7 196.7,141.3 232.9,134.4 269.1,121.4 305.3,124.6 341.5,108.3 377.6,100.1 413.8,87.5 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,149.0 124.4,147.8 160.5,133.2 196.7,129.8 232.9,120.6 269.1,104.4 305.3,109.1 341.5,88.7 377.6,81.9 413.8,66.3 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.9 88.2,147.9 124.4,144.9 160.5,130.4 196.7,113.6 232.9,105.6 269.1,96.3 305.3,87.5 341.5,76.8 377.6,73.7 413.8,52.0 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,144.5 124.4,140.6 160.5,128.8 196.7,106.5 232.9,99.9 269.1,87.5 305.3,80.8 341.5,71.2 377.6,58.2 413.8,43.7 450.0,30.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.74 ns | 1.72 µs | 2.98 µs | 3.3 µs | 2.65 µs |
| D38 | 2.31 ns | 2.87 µs | 3.73 µs | 4.17 µs | 5.1 µs |
| D57 | 456 ns | 3.29 µs | 7.17 µs | 8.97 µs | 11.7 µs |
| D76 | 445 ns | 5.14 µs | 8.76 µs | 9.39 µs | 9.97 µs |
| D115 | 947 ns | 10.4 µs | 10.7 µs | 21.9 µs | 32.7 µs |
| D153 | 1.07 µs | 13.8 µs | 25.1 µs | 37.4 µs | 49.8 µs |
| D230 | 1.39 µs | 23.4 µs | 43.7 µs | 77.4 µs | 120 µs |
| D307 | 1.92 µs | 25.6 µs | 49.4 µs | 150 µs | 239 µs |
| D462 | 2.01 µs | 57.8 µs | 148 µs | 291 µs | 428 µs |
| D616 | 3.04 µs | 113 µs | 248 µs | 357 µs | 958 µs |
| D924 | 5.04 µs | 263 µs | 679 µs | 1.49 ms | 2.48 ms |
| D1232 | 7.1 µs | 466 µs | 926 µs | 2.86 ms | 5.46 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,199.6 124.4,134.0 160.5,134.3 196.7,125.0 232.9,123.5 269.1,120.2 305.3,116.2 341.5,115.6 377.6,110.5 413.8,104.2 450.0,100.0 450.0,17.5 413.8,27.3 377.6,39.1 341.5,49.1 305.3,56.3 269.1,64.9 232.9,75.8 196.7,81.0 160.5,95.7 124.4,93.7 88.2,104.1 52.0,112.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,199.6 124.4,134.0 160.5,134.3 196.7,125.0 232.9,123.5 269.1,120.2 305.3,116.2 341.5,115.6 377.6,110.5 413.8,104.2 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,111.2 124.4,109.5 160.5,104.0 196.7,95.3 232.9,91.7 269.1,85.2 305.3,84.0 341.5,73.9 377.6,65.6 413.8,55.2 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,107.9 124.4,99.8 160.5,97.4 196.7,94.8 232.9,84.3 269.1,77.4 305.3,75.9 341.5,62.3 377.6,55.9 413.8,43.4 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,106.6 124.4,97.1 160.5,96.5 196.7,86.0 232.9,79.4 269.1,70.3 305.3,62.1 341.5,53.9 377.6,51.4 413.8,33.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,104.1 124.4,93.7 160.5,95.7 196.7,81.0 232.9,75.8 269.1,64.9 305.3,56.3 341.5,49.1 377.6,39.1 413.8,27.3 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 µs | 1.17 µs | 2.7 µs | 2.81 µs | 2.01 µs |
| D38 | 953 ns | 2.51 µs | 3.23 µs | 3.46 µs | 4.13 µs |
| D57 | 3.03 µs | 2.22 µs | 4.81 µs | 5.69 µs | 9.14 µs |
| D76 | 2.98 µs | 3.4 µs | 5.94 µs | 6.03 µs | 6.19 µs |
| D115 | 2.64 µs | 3.93 µs | 5.97 µs | 11.4 µs | 18 µs |
| D153 | 3.23 µs | 5.34 µs | 10.1 µs | 19.2 µs | 29.7 µs |
| D230 | 3.28 µs | 9.73 µs | 19.2 µs | 39.3 µs | 69.9 µs |
| D307 | 3.09 µs | 7.61 µs | 16.7 µs | 69.8 µs | 112 µs |
| D462 | 1.93 µs | 15.6 µs | 63.5 µs | 135 µs | 205 µs |
| D616 | 2.84 µs | 29.6 µs | 102 µs | 163 µs | 429 µs |
| D924 | 3.27 µs | 71.1 µs | 256 µs | 616 µs | 1.04 ms |
| D1232 | 3.5 µs | 129 µs | 334 µs | 1.12 ms | 2.33 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,166.1 88.2,170.8 124.4,150.7 160.5,151.0 196.7,153.1 232.9,149.6 269.1,149.4 305.3,150.4 341.5,158.6 377.6,151.8 413.8,149.4 450.0,148.2 450.0,35.3 413.8,49.3 377.6,64.7 341.5,77.5 305.3,88.0 269.1,96.2 232.9,111.1 196.7,119.8 160.5,138.3 124.4,131.6 88.2,145.4 52.0,157.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,166.1 88.2,170.8 124.4,150.7 160.5,151.0 196.7,153.1 232.9,149.6 269.1,149.4 305.3,150.4 341.5,158.6 377.6,151.8 413.8,149.4 450.0,148.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,167.2 88.2,154.0 124.4,156.2 160.5,148.7 196.7,146.2 232.9,140.9 269.1,130.5 305.3,134.8 341.5,122.3 377.6,111.1 413.8,95.9 450.0,85.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.7 88.2,149.6 124.4,142.7 160.5,139.1 196.7,139.0 232.9,129.9 269.1,118.6 305.3,121.1 341.5,97.9 377.6,89.6 413.8,73.7 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.0 88.2,148.4 124.4,139.8 160.5,138.8 196.7,127.7 232.9,118.7 269.1,106.2 305.3,96.2 341.5,84.7 377.6,81.5 413.8,58.4 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.9 88.2,145.4 124.4,131.6 160.5,138.3 196.7,119.8 232.9,111.1 269.1,96.2 305.3,88.0 341.5,77.5 377.6,64.7 413.8,49.3 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.55 µs | 1.73 µs | 3.63 µs | 3.89 µs | 2.7 µs |
| D38 | 1.13 µs | 3.3 µs | 4.3 µs | 4.39 µs | 5.36 µs |
| D57 | 5.74 µs | 3.46 µs | 7.18 µs | 8 µs | 10.5 µs |
| D76 | 5.64 µs | 5.1 µs | 8.1 µs | 8.53 µs | 8.68 µs |
| D115 | 11.2 µs | 11 µs | 7.44 µs | 18.3 µs | 26.5 µs |
| D153 | 6.55 µs | 7.81 µs | 16.2 µs | 24.6 µs | 36.2 µs |
| D230 | 6.36 µs | 14 µs | 25 µs | 48.3 µs | 81.7 µs |
| D307 | 6.08 µs | 10.9 µs | 31.9 µs | 81.1 µs | 118 µs |
| D462 | 6.12 µs | 23.4 µs | 82.3 µs | 158 µs | 212 µs |
| D616 | 5.47 µs | 36.9 µs | 113 µs | 178 µs | 417 µs |
| D924 | 6.66 µs | 82.5 µs | 270 µs | 610 µs | 917 µs |
| D1232 | 6.91 µs | 144 µs | 330 µs | 995 µs | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.4 88.2,207.3 124.4,172.1 160.5,172.4 196.7,157.5 232.9,169.2 269.1,169.8 305.3,170.8 341.5,170.7 377.6,173.1 413.8,168.8 450.0,168.0 450.0,38.4 413.8,61.9 377.6,79.0 341.5,93.7 305.3,106.3 269.1,114.4 232.9,132.1 196.7,138.9 160.5,163.1 124.4,159.0 88.2,173.6 52.0,188.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.4 88.2,207.3 124.4,172.1 160.5,172.4 196.7,157.5 232.9,169.2 269.1,169.8 305.3,170.8 341.5,170.7 377.6,173.1 413.8,168.8 450.0,168.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.2 88.2,184.1 124.4,183.1 160.5,174.6 196.7,158.0 232.9,165.4 269.1,152.6 305.3,158.2 341.5,141.6 377.6,131.7 413.8,114.2 450.0,102.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,178.3 124.4,167.2 160.5,164.6 196.7,166.4 232.9,149.6 269.1,140.1 305.3,134.8 341.5,114.2 377.6,107.3 413.8,88.4 450.0,84.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.5 88.2,177.9 124.4,164.9 160.5,163.5 196.7,146.9 232.9,140.5 269.1,125.8 305.3,114.6 341.5,100.0 377.6,97.5 413.8,70.7 450.0,60.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,173.6 124.4,159.0 160.5,163.1 196.7,138.9 232.9,132.1 269.1,114.4 305.3,106.3 341.5,93.7 377.6,79.0 413.8,61.9 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.34 µs | 1.07 µs | 2.54 µs | 2.77 µs | 1.92 µs |
| D38 | 999 ns | 2.35 µs | 3.06 µs | 3.32 µs | 4.1 µs |
| D57 | 3.11 µs | 2.05 µs | 4.56 µs | 5.61 µs | 9.15 µs |
| D76 | 3.11 µs | 3.25 µs | 5.63 µs | 6.01 µs | 5.98 µs |
| D115 | 2.77 µs | 3.77 µs | 6.13 µs | 11.9 µs | 18 µs |
| D153 | 3.39 µs | 5.03 µs | 10.5 µs | 19.5 µs | 29.2 µs |
| D230 | 3.41 µs | 9.8 µs | 19.7 µs | 40 µs | 66.9 µs |
| D307 | 3.16 µs | 7.41 µs | 16.2 µs | 67.5 µs | 109 µs |
| D462 | 1.88 µs | 15.6 µs | 60.3 µs | 135 µs | 196 µs |
| D616 | 2.98 µs | 29 µs | 100 µs | 165 µs | 426 µs |
| D924 | 3.39 µs | 68.4 µs | 253 µs | 612 µs | 1.04 ms |
| D1232 | 3.6 µs | 126 µs | 338 µs | 1.13 ms | 2.31 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,164.9 88.2,170.0 124.4,150.3 160.5,150.3 196.7,152.3 232.9,148.8 269.1,148.7 305.3,150.0 341.5,159.1 377.6,151.0 413.8,148.8 450.0,147.7 450.0,35.4 413.8,49.3 377.6,64.8 341.5,78.3 305.3,88.5 269.1,97.0 232.9,111.4 196.7,119.8 160.5,138.9 124.4,131.5 88.2,145.5 52.0,158.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,164.9 88.2,170.0 124.4,150.3 160.5,150.3 196.7,152.3 232.9,148.8 269.1,148.7 305.3,150.0 341.5,159.1 377.6,151.0 413.8,148.8 450.0,147.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,168.9 88.2,155.1 124.4,157.5 160.5,149.5 196.7,146.9 232.9,141.9 269.1,130.4 305.3,135.2 341.5,122.2 377.6,111.5 413.8,96.6 450.0,85.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,150.6 124.4,143.6 160.5,140.0 196.7,138.5 232.9,129.1 269.1,118.2 305.3,121.7 341.5,98.8 377.6,90.0 413.8,73.9 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.3 88.2,149.1 124.4,140.0 160.5,138.8 196.7,127.0 232.9,118.4 269.1,105.9 305.3,96.8 341.5,84.8 377.6,81.3 413.8,58.5 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,145.5 124.4,131.5 160.5,138.9 196.7,119.8 232.9,111.4 269.1,97.0 305.3,88.5 341.5,78.3 377.6,64.8 413.8,49.3 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.56 µs | 1.72 µs | 3.64 µs | 3.89 µs | 2.74 µs |
| D38 | 1.32 µs | 3.31 µs | 4.31 µs | 4.41 µs | 5.37 µs |
| D57 | 5.74 µs | 3.46 µs | 7.19 µs | 8.05 µs | 10.5 µs |
| D76 | 5.66 µs | 5.11 µs | 8.11 µs | 8.53 µs | 8.69 µs |
| D115 | 11.2 µs | 11 µs | 7.38 µs | 18.4 µs | 26.1 µs |
| D153 | 6.43 µs | 7.86 µs | 16.2 µs | 24.5 µs | 36.2 µs |
| D230 | 6.4 µs | 14.1 µs | 25.1 µs | 47.7 µs | 81.1 µs |
| D307 | 6.16 µs | 10.8 µs | 31 µs | 81.2 µs | 118 µs |
| D462 | 6.21 µs | 23.4 µs | 82.5 µs | 159 µs | 213 µs |
| D616 | 5.65 µs | 36.8 µs | 115 µs | 177 µs | 416 µs |
| D924 | 6.73 µs | 82.9 µs | 271 µs | 609 µs | 918 µs |
| D1232 | 7.08 µs | 142 µs | 327 µs | 996 µs | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.3 88.2,204.0 124.4,172.1 160.5,172.4 196.7,157.5 232.9,169.6 269.1,169.7 305.3,170.5 341.5,170.4 377.6,172.4 413.8,168.6 450.0,167.5 450.0,38.4 413.8,61.9 377.6,79.0 341.5,93.5 305.3,106.3 269.1,114.6 232.9,132.0 196.7,139.1 160.5,163.0 124.4,158.9 88.2,173.5 52.0,188.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.3 88.2,204.0 124.4,172.1 160.5,172.4 196.7,157.5 232.9,169.6 269.1,169.7 305.3,170.5 341.5,170.4 377.6,172.4 413.8,168.6 450.0,167.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,184.0 124.4,183.0 160.5,174.6 196.7,158.0 232.9,165.2 269.1,152.5 305.3,158.3 341.5,141.5 377.6,131.7 413.8,114.1 450.0,102.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,178.3 124.4,167.2 160.5,164.5 196.7,166.6 232.9,149.6 269.1,140.0 305.3,135.4 341.5,114.2 377.6,107.0 413.8,88.4 450.0,84.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.5 88.2,177.8 124.4,164.7 160.5,163.5 196.7,146.8 232.9,140.6 269.1,126.1 305.3,114.5 341.5,100.0 377.6,97.6 413.8,70.8 450.0,60.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.2 88.2,173.5 124.4,158.9 160.5,163.0 196.7,139.1 232.9,132.0 269.1,114.6 305.3,106.3 341.5,93.5 377.6,79.0 413.8,61.9 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.48 µs | 2.13 µs | 4.38 µs | 4.55 µs | 3.48 µs |
| D38 | 1.87 µs | 3.98 µs | 5.12 µs | 5.5 µs | 6.65 µs |
| D57 | 4.05 µs | 2.72 µs | 6.03 µs | 7.45 µs | 9.29 µs |
| D76 | 4.07 µs | 4.13 µs | 7.5 µs | 7.85 µs | 7.5 µs |
| D115 | 3.57 µs | 4.92 µs | 7.62 µs | 13.9 µs | 21.9 µs |
| D153 | 4.36 µs | 6.85 µs | 12.7 µs | 23.4 µs | 34 µs |
| D230 | 4.41 µs | 12.5 µs | 23.4 µs | 45.3 µs | 75.3 µs |
| D307 | 4.09 µs | 9.33 µs | 20 µs | 75.7 µs | 121 µs |
| D462 | 2.49 µs | 19.2 µs | 69.3 µs | 148 µs | 213 µs |
| D616 | 3.74 µs | 33.8 µs | 111 µs | 178 µs | 460 µs |
| D924 | 4.34 µs | 77.4 µs | 276 µs | 662 µs | 1.1 ms |
| D1232 | 4.56 µs | 142 µs | 366 µs | 1.21 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.3 88.2,196.4 124.4,179.6 160.5,179.5 196.7,182.4 232.9,178.0 269.1,177.8 305.3,179.4 341.5,190.2 377.6,181.3 413.8,178.1 450.0,177.1 450.0,40.6 413.8,57.9 377.6,76.8 341.5,93.6 305.3,105.9 269.1,116.2 232.9,133.4 196.7,143.0 160.5,166.2 124.4,161.6 88.2,168.9 52.0,182.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.3 88.2,196.4 124.4,179.6 160.5,179.5 196.7,182.4 232.9,178.0 269.1,177.8 305.3,179.4 341.5,190.2 377.6,181.3 413.8,178.1 450.0,177.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.6 88.2,180.0 124.4,188.3 160.5,179.2 196.7,175.4 232.9,168.2 269.1,155.2 305.3,161.5 341.5,145.9 377.6,133.6 413.8,115.6 450.0,102.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.9 88.2,174.5 124.4,171.0 160.5,166.3 196.7,165.9 232.9,154.8 269.1,141.5 305.3,145.0 341.5,118.0 377.6,107.7 413.8,88.0 450.0,81.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.1 88.2,173.0 124.4,166.4 160.5,165.3 196.7,152.8 232.9,141.5 269.1,127.2 305.3,116.1 341.5,101.5 377.6,97.4 413.8,68.9 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.9 88.2,168.9 124.4,161.6 160.5,166.2 196.7,143.0 232.9,133.4 269.1,116.2 305.3,105.9 341.5,93.6 377.6,76.8 413.8,57.9 450.0,40.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.66 µs | 1.76 µs | 3.83 µs | 4.09 µs | 2.9 µs |
| D38 | 1.22 µs | 3.47 µs | 4.36 µs | 4.42 µs | 5.41 µs |
| D57 | 5.86 µs | 3.53 µs | 7.58 µs | 8.5 µs | 11 µs |
| D76 | 5.87 µs | 5.25 µs | 8.43 µs | 8.82 µs | 8.95 µs |
| D115 | 11.5 µs | 11.2 µs | 7.69 µs | 19.2 µs | 27 µs |
| D153 | 6.47 µs | 8.3 µs | 16.8 µs | 25.4 µs | 37.1 µs |
| D230 | 6.56 µs | 14.5 µs | 25.4 µs | 48.8 µs | 82.4 µs |
| D307 | 6.26 µs | 11 µs | 32.2 µs | 82.6 µs | 120 µs |
| D462 | 6.44 µs | 24.1 µs | 83.8 µs | 161 µs | 218 µs |
| D616 | 5.76 µs | 37.6 µs | 116 µs | 180 µs | 422 µs |
| D924 | 6.93 µs | 84.2 µs | 275 µs | 614 µs | 915 µs |
| D1232 | 7.46 µs | 146 µs | 330 µs | 1.01 ms | 2.71 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.0 88.2,205.7 124.4,171.6 160.5,171.6 196.7,156.9 232.9,169.4 269.1,169.2 305.3,170.2 341.5,169.6 377.6,172.0 413.8,168.0 450.0,166.4 450.0,38.3 413.8,61.9 377.6,78.8 341.5,93.1 305.3,106.0 269.1,114.2 232.9,131.5 196.7,138.4 160.5,162.4 124.4,157.9 88.2,173.4 52.0,186.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,205.7 124.4,171.6 160.5,171.6 196.7,156.9 232.9,169.4 269.1,169.2 305.3,170.2 341.5,169.6 377.6,172.0 413.8,168.0 450.0,166.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.8 88.2,183.0 124.4,182.6 160.5,174.0 196.7,157.5 232.9,164.0 269.1,152.0 305.3,158.0 341.5,140.9 377.6,131.2 413.8,113.7 450.0,101.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.8 88.2,178.0 124.4,166.0 160.5,163.7 196.7,165.7 232.9,148.7 269.1,139.8 305.3,134.6 341.5,113.8 377.6,106.8 413.8,88.1 450.0,84.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,177.7 124.4,163.5 160.5,162.7 196.7,145.8 232.9,139.8 269.1,125.6 305.3,114.2 341.5,99.7 377.6,97.2 413.8,70.6 450.0,59.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.9 88.2,173.4 124.4,157.9 160.5,162.4 196.7,138.4 232.9,131.5 269.1,114.2 305.3,106.0 341.5,93.1 377.6,78.8 413.8,61.9 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 137 ns | 136 ns | 175 ns | 179 ns | 121 ns |
| D38 | 84.7 ns | 160 ns | 179 ns | 185 ns | 207 ns |
| D57 | 230 ns | 173 ns | 319 ns | 320 ns | 410 ns |
| D76 | 232 ns | 255 ns | 355 ns | 344 ns | 309 ns |
| D115 | 447 ns | 545 ns | 427 ns | 625 ns | 892 ns |
| D153 | 563 ns | 589 ns | 817 ns | 992 ns | 1 µs |
| D230 | 674 ns | 829 ns | 1.12 µs | 1.44 µs | 1.83 µs |
| D307 | 968 ns | 752 ns | 999 ns | 2.16 µs | 2.65 µs |
| D462 | 1.04 µs | 1.45 µs | 2.21 µs | 3.11 µs | 3.63 µs |
| D616 | 1.13 µs | 1.91 µs | 2.6 µs | 2.78 µs | 5.68 µs |
| D924 | 1.7 µs | 2.84 µs | 4.56 µs | 7.7 µs | 10.4 µs |
| D1232 | 2.66 µs | 4.39 µs | 5.27 µs | 12 µs | 28.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,153.2 88.2,163.6 124.4,141.9 160.5,141.7 196.7,127.5 232.9,122.5 269.1,118.6 305.3,110.7 341.5,109.2 377.6,107.4 413.8,98.5 450.0,88.8 450.0,36.9 413.8,59.1 377.6,72.3 341.5,82.0 305.3,88.8 269.1,96.8 232.9,109.9 196.7,112.5 160.5,135.5 124.4,129.4 88.2,144.2 52.0,155.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,153.2 88.2,163.6 124.4,141.9 160.5,141.7 196.7,127.5 232.9,122.5 269.1,118.6 305.3,110.7 341.5,109.2 377.6,107.4 413.8,98.5 450.0,88.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.4 88.2,149.8 124.4,148.1 160.5,139.7 196.7,123.2 232.9,121.5 269.1,114.1 305.3,116.2 341.5,102.0 377.6,95.9 413.8,87.3 450.0,77.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.9 88.2,147.3 124.4,134.8 160.5,132.5 196.7,128.5 232.9,114.4 269.1,107.5 305.3,110.0 341.5,92.7 377.6,89.3 413.8,77.1 450.0,73.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.4 88.2,146.7 124.4,134.7 160.5,133.1 196.7,120.2 232.9,110.2 269.1,102.1 305.3,93.3 341.5,85.4 377.6,87.8 413.8,65.7 450.0,56.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.9 88.2,144.2 124.4,129.4 160.5,135.5 196.7,112.5 232.9,109.9 269.1,96.8 305.3,88.8 341.5,82.0 377.6,72.3 413.8,59.1 450.0,36.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 156 ns | 156 ns | 211 ns | 218 ns | 168 ns |
| D38 | 133 ns | 191 ns | 215 ns | 196 ns | 220 ns |
| D57 | 309 ns | 233 ns | 382 ns | 399 ns | 499 ns |
| D76 | 316 ns | 313 ns | 442 ns | 405 ns | 357 ns |
| D115 | 567 ns | 620 ns | 481 ns | 720 ns | 959 ns |
| D153 | 690 ns | 711 ns | 934 ns | 1.08 µs | 1.17 µs |
| D230 | 882 ns | 981 ns | 1.25 µs | 1.66 µs | 2.01 µs |
| D307 | 1.31 µs | 984 ns | 1.18 µs | 2.49 µs | 3.02 µs |
| D462 | 1.38 µs | 1.68 µs | 2.57 µs | 3.48 µs | 4.01 µs |
| D616 | 1.47 µs | 2.15 µs | 2.82 µs | 3.12 µs | 6.15 µs |
| D924 | 2.25 µs | 3.33 µs | 5.09 µs | 8.34 µs | 11 µs |
| D1232 | 3.27 µs | 5.03 µs | 5.79 µs | 13 µs | 30 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.1 88.2,201.7 124.4,177.3 160.5,176.7 196.7,159.8 232.9,154.1 269.1,147.0 305.3,135.6 341.5,134.0 377.6,132.1 413.8,119.8 450.0,109.0 450.0,44.9 413.8,74.0 377.6,90.7 341.5,103.1 305.3,111.3 269.1,123.1 232.9,138.9 196.7,144.6 160.5,173.2 124.4,163.5 88.2,187.2 52.0,195.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.1 88.2,201.7 124.4,177.3 160.5,176.7 196.7,159.8 232.9,154.1 269.1,147.0 305.3,135.6 341.5,134.0 377.6,132.1 413.8,119.8 450.0,109.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.1 88.2,191.3 124.4,185.5 160.5,176.9 196.7,157.2 232.9,153.2 269.1,143.9 305.3,143.8 341.5,128.3 377.6,121.2 413.8,108.5 450.0,96.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,187.8 124.4,171.2 160.5,166.9 196.7,164.5 232.9,145.3 269.1,136.9 305.3,138.6 341.5,116.1 377.6,113.3 413.8,96.2 450.0,92.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,190.5 124.4,169.9 160.5,169.5 196.7,152.8 232.9,141.2 269.1,128.7 305.3,116.9 341.5,107.3 377.6,110.4 413.8,81.9 450.0,69.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.1 88.2,187.2 124.4,163.5 160.5,173.2 196.7,144.6 232.9,138.9 269.1,123.1 305.3,111.3 341.5,103.1 377.6,90.7 413.8,74.0 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
