# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.33 ns | 2 µs | 3.22 µs | 3.48 µs | 3 µs |
| D38 | 1.41 µs | 4.67 µs | 4.55 µs | 8.1 µs | 7.06 µs |
| D57 | 1.44 µs | 5.78 µs | 8.16 µs | 11.1 µs | 8.3 µs |
| D76 | 1.56 µs | 7.17 µs | 10.6 µs | 14.3 µs | 10.9 µs |
| D115 | 1.27 µs | 8.17 µs | 20 µs | 30.8 µs | 44.6 µs |
| D153 | 1.08 µs | 9.36 µs | 25.7 µs | 28.5 µs | 65 µs |
| D230 | 1.68 µs | 20.4 µs | 33.7 µs | 46 µs | 131 µs |
| D307 | 1.48 µs | 24.8 µs | 66.2 µs | 124 µs | 192 µs |
| D462 | 1.56 µs | 43.1 µs | 95 µs | 256 µs | 244 µs |
| D616 | 1.64 µs | 60.7 µs | 171 µs | 427 µs | 663 µs |
| D924 | 998 ns | 110 µs | 430 µs | 950 µs | 1.68 ms |
| D1232 | 1.48 µs | 149 µs | 712 µs | 1.53 ms | 2.95 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.2 88.2,120.0 124.4,119.8 160.5,118.8 196.7,121.4 232.9,123.3 269.1,117.9 305.3,119.5 341.5,118.7 377.6,118.2 413.8,124.3 450.0,119.4 450.0,25.1 413.8,32.1 377.6,43.7 341.5,56.1 305.3,59.0 269.1,63.8 232.9,72.5 196.7,77.2 160.5,94.6 124.4,98.0 88.2,100.0 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.2 88.2,120.0 124.4,119.8 160.5,118.8 196.7,121.4 232.9,123.3 269.1,117.9 305.3,119.5 341.5,118.7 377.6,118.2 413.8,124.3 450.0,119.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.2 124.4,102.5 160.5,99.8 196.7,98.2 232.9,96.5 269.1,86.9 305.3,84.4 341.5,77.6 377.6,73.3 413.8,66.0 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,105.5 124.4,98.2 160.5,95.0 196.7,87.1 232.9,84.0 269.1,80.6 305.3,72.3 341.5,67.8 377.6,60.5 413.8,49.0 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.3 124.4,94.4 160.5,91.3 196.7,81.7 232.9,82.7 269.1,76.8 305.3,64.4 341.5,55.5 377.6,49.1 413.8,39.2 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,100.0 124.4,98.0 160.5,94.6 196.7,77.2 232.9,72.5 269.1,63.8 305.3,59.0 341.5,56.1 377.6,43.7 413.8,32.1 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.54 µs | 3.61 µs | 5.85 µs | 6.41 µs | 5.48 µs |
| D38 | 3.39 µs | 5.84 µs | 7.43 µs | 8.34 µs | 7.94 µs |
| D57 | 3.42 µs | 4.31 µs | 5.06 µs | 6.57 µs | 4.44 µs |
| D76 | 3.7 µs | 4.99 µs | 6.29 µs | 7.76 µs | 5.58 µs |
| D115 | 5.55 µs | 8.67 µs | 12.3 µs | 15 µs | 23 µs |
| D153 | 4.66 µs | 8.95 µs | 14.5 µs | 14 µs | 34.3 µs |
| D230 | 8.98 µs | 16.9 µs | 19.9 µs | 27.5 µs | 73.1 µs |
| D307 | 12.9 µs | 27.2 µs | 51.2 µs | 86.5 µs | 139 µs |
| D462 | 13 µs | 37.1 µs | 61.3 µs | 171 µs | 170 µs |
| D616 | 21.7 µs | 71 µs | 146 µs | 315 µs | 520 µs |
| D924 | 20.1 µs | 131 µs | 390 µs | 810 µs | 1.45 ms |
| D1232 | 36.6 µs | 189 µs | 734 µs | 1.44 ms | 2.62 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.7 88.2,183.5 124.4,183.3 160.5,181.6 196.7,172.8 232.9,176.6 269.1,162.3 305.3,154.5 341.5,154.3 377.6,143.1 413.8,144.9 450.0,131.8 450.0,39.1 413.8,51.9 377.6,74.2 341.5,98.5 305.3,102.8 269.1,116.8 232.9,133.3 196.7,141.9 160.5,172.7 124.4,177.6 88.2,165.0 52.0,173.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.7 88.2,183.5 124.4,183.3 160.5,181.6 196.7,172.8 232.9,176.6 269.1,162.3 305.3,154.5 341.5,154.3 377.6,143.1 413.8,144.9 450.0,131.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.1 88.2,171.7 124.4,178.3 160.5,175.1 196.7,163.1 232.9,162.4 269.1,148.7 305.3,138.3 341.5,131.5 377.6,117.4 413.8,104.1 450.0,96.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,166.5 124.4,174.8 160.5,170.1 196.7,155.5 232.9,151.9 269.1,145.0 305.3,124.5 341.5,120.6 377.6,101.7 413.8,80.4 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.9 124.4,169.1 160.5,165.5 196.7,151.1 232.9,152.7 269.1,138.1 305.3,113.1 341.5,98.4 377.6,85.1 413.8,64.6 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.1 88.2,165.0 124.4,177.6 160.5,172.7 196.7,141.9 232.9,133.3 269.1,116.8 305.3,102.8 341.5,98.5 377.6,74.2 413.8,51.9 450.0,39.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.4 ns | 1.98 µs | 3.2 µs | 3.47 µs | 2.99 µs |
| D38 | 1.3 µs | 4.64 µs | 4.51 µs | 8.06 µs | 7.11 µs |
| D57 | 1.32 µs | 5.73 µs | 8.14 µs | 11.1 µs | 8.27 µs |
| D76 | 1.46 µs | 7.15 µs | 10.5 µs | 14.2 µs | 10.9 µs |
| D115 | 1.16 µs | 8.14 µs | 19.6 µs | 30.7 µs | 44.8 µs |
| D153 | 988 ns | 9.29 µs | 25.3 µs | 30.2 µs | 64.8 µs |
| D230 | 1.56 µs | 19.9 µs | 33.4 µs | 45.8 µs | 132 µs |
| D307 | 1.34 µs | 25.8 µs | 66.1 µs | 124 µs | 192 µs |
| D462 | 1.45 µs | 43 µs | 94.6 µs | 256 µs | 245 µs |
| D616 | 1.44 µs | 60.4 µs | 172 µs | 426 µs | 663 µs |
| D924 | 925 ns | 109 µs | 429 µs | 947 µs | 1.68 ms |
| D1232 | 1.37 µs | 149 µs | 713 µs | 1.52 ms | 2.95 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,121.0 124.4,120.8 160.5,119.6 196.7,122.5 232.9,124.4 269.1,118.7 305.3,120.7 341.5,119.6 377.6,119.8 413.8,125.3 450.0,120.4 450.0,25.2 413.8,32.1 377.6,43.7 341.5,56.0 305.3,59.1 269.1,63.7 232.9,72.5 196.7,77.1 160.5,94.6 124.4,98.1 88.2,99.9 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,121.0 124.4,120.8 160.5,119.6 196.7,122.5 232.9,124.4 269.1,118.7 305.3,120.7 341.5,119.6 377.6,119.8 413.8,125.3 450.0,120.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.2 124.4,102.6 160.5,99.9 196.7,98.3 232.9,96.6 269.1,87.2 305.3,84.0 341.5,77.6 377.6,73.4 413.8,66.0 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,105.6 124.4,98.3 160.5,95.1 196.7,87.4 232.9,84.2 269.1,80.7 305.3,72.3 341.5,67.8 377.6,60.4 413.8,49.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,94.4 160.5,91.3 196.7,81.8 232.9,82.0 269.1,76.8 305.3,64.5 341.5,55.5 377.6,49.2 413.8,39.2 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,99.9 124.4,98.1 160.5,94.6 196.7,77.1 232.9,72.5 269.1,63.7 305.3,59.1 341.5,56.0 377.6,43.7 413.8,32.1 450.0,25.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.69 ns | 1.72 µs | 2.51 µs | 2.82 µs | 2.36 µs |
| D38 | 4.22 ns | 2.53 µs | 3.26 µs | 3.69 µs | 3.59 µs |
| D57 | 1.96 ns | 5.35 µs | 7.21 µs | 9.54 µs | 7.01 µs |
| D76 | 2.54 ns | 6.73 µs | 9.28 µs | 11.7 µs | 8.78 µs |
| D115 | 9.84 ns | 12.7 µs | 19.1 µs | 22.9 µs | 36.2 µs |
| D153 | 11.3 ns | 13.8 µs | 21.8 µs | 22.2 µs | 47.7 µs |
| D230 | 31.6 ns | 25.9 µs | 31.5 µs | 39.7 µs | 102 µs |
| D307 | 45 ns | 38.3 µs | 73.5 µs | 114 µs | 177 µs |
| D462 | 69.5 ns | 56.1 µs | 86.6 µs | 210 µs | 202 µs |
| D616 | 79.4 ns | 104 µs | 192 µs | 402 µs | 620 µs |
| D924 | 76.3 ns | 198 µs | 500 µs | 942 µs | 1.6 ms |
| D1232 | 104 ns | 258 µs | 929 µs | 1.62 ms | 2.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.7 88.2,192.1 124.4,201.6 160.5,198.4 196.7,181.6 232.9,179.9 269.1,167.1 305.3,162.8 341.5,157.4 377.6,155.7 413.8,156.2 450.0,152.4 450.0,27.6 413.8,32.7 377.6,44.5 341.5,58.4 305.3,60.1 269.1,66.9 232.9,76.3 196.7,79.7 160.5,97.3 124.4,100.1 88.2,108.4 52.0,113.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.7 88.2,192.1 124.4,201.6 160.5,198.4 196.7,181.6 232.9,179.9 269.1,167.1 305.3,162.8 341.5,157.4 377.6,155.7 413.8,156.2 450.0,152.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,112.8 124.4,103.5 160.5,100.6 196.7,92.8 232.9,91.7 269.1,83.9 305.3,79.0 341.5,74.3 377.6,66.6 413.8,58.7 450.0,55.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,109.6 124.4,99.8 160.5,96.6 196.7,87.7 232.9,86.0 269.1,81.5 305.3,71.0 341.5,68.9 377.6,59.1 413.8,47.2 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,108.1 124.4,96.3 160.5,93.8 196.7,85.5 232.9,85.8 269.1,78.6 305.3,65.5 341.5,57.9 377.6,49.9 413.8,39.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.7 88.2,108.4 124.4,100.1 160.5,97.3 196.7,79.7 232.9,76.3 269.1,66.9 305.3,60.1 341.5,58.4 377.6,44.5 413.8,32.7 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.37 ns | 1.57 µs | 2.69 µs | 2.95 µs | 2.56 µs |
| D38 | 3.16 ns | 2.84 µs | 3.4 µs | 3.46 µs | 2.89 µs |
| D57 | 1.74 ns | 2.94 µs | 3.55 µs | 4.35 µs | 3.03 µs |
| D76 | 599 ns | 5.55 µs | 8.24 µs | 11.8 µs | 9.09 µs |
| D115 | 475 ns | 6.29 µs | 16.9 µs | 28.4 µs | 40.7 µs |
| D153 | 387 ns | 7.31 µs | 16.1 µs | 25.5 µs | 59.8 µs |
| D230 | 661 ns | 16.8 µs | 30.3 µs | 42.9 µs | 122 µs |
| D307 | 565 ns | 21.8 µs | 55.1 µs | 116 µs | 181 µs |
| D462 | 607 ns | 34.5 µs | 80 µs | 224 µs | 242 µs |
| D616 | 621 ns | 56.6 µs | 160 µs | 398 µs | 627 µs |
| D924 | 442 ns | 101 µs | 401 µs | 907 µs | 1.59 ms |
| D1232 | 687 ns | 139 µs | 687 µs | 1.46 ms | 2.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,195.7 124.4,203.2 160.5,130.6 196.7,133.5 232.9,136.1 269.1,129.4 305.3,131.4 341.5,130.5 377.6,130.2 413.8,134.4 450.0,128.9 450.0,25.9 413.8,32.8 377.6,44.4 341.5,56.2 305.3,59.8 269.1,64.7 232.9,73.5 196.7,78.3 160.5,96.9 124.4,110.5 88.2,111.1 52.0,112.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,195.7 124.4,203.2 160.5,130.6 196.7,133.5 232.9,136.1 269.1,129.4 305.3,131.4 341.5,130.5 377.6,130.2 413.8,134.4 450.0,128.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,111.3 124.4,110.9 160.5,103.0 196.7,101.5 232.9,99.6 269.1,89.3 305.3,86.1 341.5,80.3 377.6,74.2 413.8,67.0 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.1 124.4,108.6 160.5,98.1 196.7,89.2 232.9,89.8 269.1,82.0 305.3,74.5 341.5,69.9 377.6,61.3 413.8,49.9 450.0,43.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,108.9 124.4,106.0 160.5,93.7 196.7,82.8 232.9,84.1 269.1,77.6 305.3,65.3 341.5,57.1 377.6,50.0 413.8,39.8 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,111.1 124.4,110.5 160.5,96.9 196.7,78.3 232.9,73.5 269.1,64.7 305.3,59.8 341.5,56.2 377.6,44.4 413.8,32.8 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.55 ns | 1.73 µs | 2.86 µs | 3.17 µs | 2.74 µs |
| D38 | 4.22 ns | 2.85 µs | 3.75 µs | 4.14 µs | 4.01 µs |
| D57 | 437 ns | 5.69 µs | 7.1 µs | 9.3 µs | 6.66 µs |
| D76 | 507 ns | 6.65 µs | 8.71 µs | 11.3 µs | 8.47 µs |
| D115 | 874 ns | 11.9 µs | 18 µs | 22.6 µs | 36.3 µs |
| D153 | 765 ns | 12.3 µs | 22 µs | 21.8 µs | 54.7 µs |
| D230 | 1.45 µs | 24.4 µs | 30.8 µs | 45.6 µs | 125 µs |
| D307 | 2 µs | 42.4 µs | 84.8 µs | 148 µs | 245 µs |
| D462 | 2.05 µs | 58.1 µs | 105 µs | 307 µs | 312 µs |
| D616 | 3.53 µs | 118 µs | 248 µs | 565 µs | 952 µs |
| D924 | 3.07 µs | 219 µs | 691 µs | 1.49 ms | 2.7 ms |
| D1232 | 5.49 µs | 319 µs | 1.32 ms | 2.7 ms | 4.94 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.4 88.2,192.1 124.4,134.6 160.5,132.7 196.7,126.0 232.9,127.6 269.1,119.7 305.3,115.7 341.5,115.4 377.6,108.6 413.8,110.3 450.0,103.1 450.0,18.7 413.8,26.3 377.6,39.2 341.5,53.0 305.3,56.0 269.1,64.3 232.9,74.6 196.7,79.7 160.5,97.8 124.4,100.8 88.2,107.0 52.0,111.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.4 88.2,192.1 124.4,134.6 160.5,132.7 196.7,126.0 232.9,127.6 269.1,119.7 305.3,115.7 341.5,115.4 377.6,108.6 413.8,110.3 450.0,103.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,111.3 124.4,102.7 160.5,100.8 196.7,93.5 232.9,93.1 269.1,84.6 305.3,77.8 341.5,73.9 377.6,65.1 413.8,57.4 450.0,52.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.9 124.4,100.0 160.5,97.4 196.7,88.5 232.9,85.9 269.1,81.7 305.3,69.2 341.5,66.5 377.6,55.9 413.8,43.2 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.7 124.4,96.6 160.5,94.2 196.7,85.6 232.9,86.0 269.1,76.9 305.3,62.3 341.5,53.2 377.6,45.7 413.8,33.6 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.8 88.2,107.0 124.4,100.8 160.5,97.8 196.7,79.7 232.9,74.6 269.1,64.3 305.3,56.0 341.5,53.0 377.6,39.2 413.8,26.3 450.0,18.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.61 ns | 1.19 µs | 2.56 µs | 2.67 µs | 1.99 µs |
| D38 | 5.62 ns | 2.56 µs | 3.23 µs | 3.52 µs | 2.75 µs |
| D57 | 2.18 ns | 3.51 µs | 4.81 µs | 6.02 µs | 5.24 µs |
| D76 | 3.15 ns | 4.06 µs | 5.94 µs | 7.25 µs | 5.26 µs |
| D115 | 10.1 ns | 4.66 µs | 9.95 µs | 12.5 µs | 18.9 µs |
| D153 | 13.8 ns | 5.09 µs | 9.83 µs | 11.9 µs | 32.8 µs |
| D230 | 40.7 ns | 10.2 µs | 13.7 µs | 23.5 µs | 72.2 µs |
| D307 | 64.5 ns | 12.1 µs | 27.1 µs | 70.4 µs | 117 µs |
| D462 | 105 ns | 15.8 µs | 45.5 µs | 143 µs | 147 µs |
| D616 | 157 ns | 29.6 µs | 104 µs | 261 µs | 432 µs |
| D924 | 106 ns | 62.4 µs | 261 µs | 621 µs | 1.14 ms |
| D1232 | 230 ns | 88.3 µs | 465 µs | 1.05 ms | 2.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.1 88.2,188.6 124.4,200.3 160.5,195.8 196.7,181.3 232.9,177.4 269.1,164.0 305.3,158.3 341.5,152.2 377.6,147.3 413.8,152.1 450.0,142.5 450.0,29.3 413.8,37.0 377.6,49.0 341.5,62.4 305.3,65.2 269.1,71.2 232.9,81.0 196.7,87.8 160.5,103.7 124.4,103.7 88.2,111.7 52.0,115.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.1 88.2,188.6 124.4,200.3 160.5,195.8 196.7,181.3 232.9,177.4 269.1,164.0 305.3,158.3 341.5,152.2 377.6,147.3 413.8,152.1 450.0,142.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,112.6 124.4,108.7 160.5,106.9 196.7,105.2 232.9,104.1 269.1,95.5 305.3,93.3 341.5,90.1 377.6,82.2 413.8,73.0 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,109.7 124.4,104.8 160.5,102.2 196.7,95.8 232.9,95.9 269.1,91.8 305.3,83.4 341.5,76.9 377.6,66.6 413.8,55.2 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,108.7 124.4,102.0 160.5,99.7 196.7,92.9 232.9,93.5 269.1,85.1 305.3,71.5 341.5,62.7 377.6,55.2 413.8,44.5 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,111.7 124.4,103.7 160.5,103.7 196.7,87.8 232.9,81.0 269.1,71.2 305.3,65.2 341.5,62.4 377.6,49.0 413.8,37.0 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.54 ns | 1.74 µs | 3.35 µs | 3.6 µs | 2.79 µs |
| D38 | 4.57 ns | 3.34 µs | 4.3 µs | 4.46 µs | 3.68 µs |
| D57 | 2.49 ns | 5.51 µs | 7.05 µs | 8.48 µs | 6.14 µs |
| D76 | 4.22 ns | 6.35 µs | 8.12 µs | 10.3 µs | 7.34 µs |
| D115 | 8.46 ns | 12.1 µs | 12.3 µs | 20.2 µs | 28.6 µs |
| D153 | 13.2 ns | 7.27 µs | 15.9 µs | 14.9 µs | 39.3 µs |
| D230 | 40.8 ns | 14.5 µs | 17.6 µs | 28.1 µs | 85.1 µs |
| D307 | 65.1 ns | 16.2 µs | 54.2 µs | 80.5 µs | 126 µs |
| D462 | 106 ns | 23.8 µs | 59.9 µs | 169 µs | 154 µs |
| D616 | 120 ns | 36.8 µs | 114 µs | 271 µs | 418 µs |
| D924 | 117 ns | 69.8 µs | 274 µs | 612 µs | 999 µs |
| D1232 | 227 ns | 96.4 µs | 451 µs | 920 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.4 88.2,191.1 124.4,198.7 160.5,192.1 196.7,183.5 232.9,178.0 269.1,164.0 305.3,158.2 341.5,152.1 377.6,150.6 413.8,150.9 450.0,142.7 450.0,26.3 413.8,38.6 377.6,49.4 341.5,61.8 305.3,64.3 269.1,69.1 232.9,78.7 196.7,82.7 160.5,99.6 124.4,101.8 88.2,108.1 52.0,111.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.4 88.2,191.1 124.4,198.7 160.5,192.1 196.7,183.5 232.9,178.0 269.1,164.0 305.3,158.2 341.5,152.1 377.6,150.6 413.8,150.9 450.0,142.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,109.3 124.4,103.1 160.5,101.4 196.7,93.3 232.9,99.7 269.1,91.1 305.3,89.7 341.5,84.9 377.6,79.6 413.8,71.6 450.0,67.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,106.2 124.4,100.0 160.5,98.3 196.7,93.2 232.9,90.0 269.1,88.7 305.3,74.7 341.5,73.5 377.6,65.5 413.8,54.6 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,105.7 124.4,97.8 160.5,95.4 196.7,87.0 232.9,90.8 269.1,82.9 305.3,69.8 341.5,60.6 377.6,54.8 413.8,44.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,108.1 124.4,101.8 160.5,99.6 196.7,82.7 232.9,78.7 269.1,69.1 305.3,64.3 341.5,61.8 377.6,49.4 413.8,38.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.35 ns | 1.07 µs | 2.4 µs | 2.63 µs | 1.96 µs |
| D38 | 4.92 ns | 2.4 µs | 3.06 µs | 3.36 µs | 2.73 µs |
| D57 | 2.18 ns | 3.28 µs | 4.5 µs | 5.99 µs | 5.23 µs |
| D76 | 3.51 ns | 3.85 µs | 5.69 µs | 7.18 µs | 5.1 µs |
| D115 | 10.1 ns | 4.39 µs | 10.2 µs | 12.2 µs | 19 µs |
| D153 | 14 ns | 4.77 µs | 9.7 µs | 11.2 µs | 31.6 µs |
| D230 | 40.7 ns | 11.1 µs | 13.7 µs | 23.7 µs | 69.4 µs |
| D307 | 59.9 ns | 12 µs | 26.4 µs | 66.7 µs | 115 µs |
| D462 | 99 ns | 15.5 µs | 44.2 µs | 141 µs | 141 µs |
| D616 | 111 ns | 29.4 µs | 102 µs | 256 µs | 431 µs |
| D924 | 108 ns | 60.2 µs | 259 µs | 616 µs | 1.14 ms |
| D1232 | 224 ns | 87.1 µs | 467 µs | 1.05 ms | 2.08 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.0 88.2,190.2 124.4,200.3 160.5,194.4 196.7,181.3 232.9,177.2 269.1,164.0 305.3,159.2 341.5,153.0 377.6,151.6 413.8,151.9 450.0,142.8 450.0,29.5 413.8,37.0 377.6,49.0 341.5,62.9 305.3,65.5 269.1,71.7 232.9,81.4 196.7,87.7 160.5,104.1 124.4,103.8 88.2,111.8 52.0,115.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.0 88.2,190.2 124.4,200.3 160.5,194.4 196.7,181.3 232.9,177.2 269.1,164.0 305.3,159.2 341.5,153.0 377.6,151.6 413.8,151.9 450.0,142.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,113.4 124.4,109.5 160.5,107.6 196.7,105.9 232.9,104.9 269.1,94.4 305.3,93.5 341.5,90.3 377.6,82.3 413.8,73.4 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,110.4 124.4,105.6 160.5,102.7 196.7,95.4 232.9,96.1 269.1,91.8 305.3,83.7 341.5,77.3 377.6,66.9 413.8,55.4 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,109.3 124.4,102.1 160.5,99.8 196.7,93.3 232.9,94.3 269.1,85.0 305.3,72.2 341.5,62.8 377.6,55.5 413.8,44.6 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.9 88.2,111.8 124.4,103.8 160.5,104.1 196.7,87.7 232.9,81.4 269.1,71.7 305.3,65.5 341.5,62.9 377.6,49.0 413.8,37.0 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.42 ns | 1.75 µs | 3.36 µs | 3.6 µs | 2.79 µs |
| D38 | 4.57 ns | 3.36 µs | 4.3 µs | 4.49 µs | 3.66 µs |
| D57 | 10.6 ns | 5.64 µs | 7.09 µs | 8.51 µs | 6.18 µs |
| D76 | 12.1 ns | 6.37 µs | 8.17 µs | 10.3 µs | 7.35 µs |
| D115 | 8.73 ns | 12.1 µs | 12 µs | 20.3 µs | 28.5 µs |
| D153 | 12.9 ns | 7.29 µs | 15.3 µs | 14.9 µs | 39.3 µs |
| D230 | 39.9 ns | 14.6 µs | 17.6 µs | 29.5 µs | 86.4 µs |
| D307 | 60.5 ns | 16.3 µs | 53 µs | 80.7 µs | 125 µs |
| D462 | 102 ns | 23.9 µs | 59.8 µs | 168 µs | 153 µs |
| D616 | 112 ns | 37.3 µs | 113 µs | 272 µs | 419 µs |
| D924 | 113 ns | 71.5 µs | 274 µs | 613 µs | 997 µs |
| D1232 | 221 ns | 97.4 µs | 451 µs | 920 µs | 2.62 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.0 88.2,191.2 124.4,180.7 160.5,179.0 196.7,183.1 232.9,178.3 269.1,164.3 305.3,159.1 341.5,152.6 377.6,151.4 413.8,151.3 450.0,143.0 450.0,26.6 413.8,38.6 377.6,49.4 341.5,61.9 305.3,64.4 269.1,69.0 232.9,78.7 196.7,82.7 160.5,99.5 124.4,101.7 88.2,108.2 52.0,111.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.0 88.2,191.2 124.4,180.7 160.5,179.0 196.7,183.1 232.9,178.3 269.1,164.3 305.3,159.1 341.5,152.6 377.6,151.4 413.8,151.3 450.0,143.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,109.3 124.4,102.8 160.5,101.3 196.7,93.4 232.9,99.6 269.1,91.1 305.3,89.6 341.5,84.9 377.6,79.4 413.8,71.3 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,106.2 124.4,100.0 160.5,98.2 196.7,93.4 232.9,90.4 269.1,88.7 305.3,75.0 341.5,73.5 377.6,65.6 413.8,54.6 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,105.6 124.4,97.7 160.5,95.3 196.7,86.9 232.9,90.7 269.1,82.3 305.3,69.8 341.5,60.7 377.6,54.7 413.8,44.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,108.2 124.4,101.7 160.5,99.5 196.7,82.7 232.9,78.7 269.1,69.0 305.3,64.4 341.5,61.9 377.6,49.4 413.8,38.6 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.32 ns | 2.13 µs | 4.02 µs | 4.28 µs | 3.56 µs |
| D38 | 4.92 ns | 4.03 µs | 5.12 µs | 5.57 µs | 4.89 µs |
| D57 | 3.12 ns | 4.4 µs | 6.05 µs | 7.78 µs | 5.26 µs |
| D76 | 3.63 ns | 5.14 µs | 7.56 µs | 9.2 µs | 6.37 µs |
| D115 | 7.15 ns | 5.86 µs | 12.6 µs | 15 µs | 23.5 µs |
| D153 | 13.5 ns | 6.31 µs | 12.1 µs | 14.3 µs | 36.9 µs |
| D230 | 41.3 ns | 12.9 µs | 16.9 µs | 26.7 µs | 79.3 µs |
| D307 | 58.2 ns | 14.4 µs | 31.7 µs | 76.6 µs | 126 µs |
| D462 | 93.9 ns | 19.1 µs | 50.4 µs | 158 µs | 152 µs |
| D616 | 108 ns | 34.3 µs | 112 µs | 281 µs | 465 µs |
| D924 | 87.2 ns | 67.1 µs | 282 µs | 664 µs | 1.21 ms |
| D1232 | 217 ns | 97.4 µs | 504 µs | 1.11 ms | 2.16 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.1 88.2,190.2 124.4,195.9 160.5,194.0 196.7,185.6 232.9,177.7 269.1,163.8 305.3,159.6 341.5,153.6 377.6,151.9 413.8,154.6 450.0,143.2 450.0,29.0 413.8,36.3 377.6,48.1 341.5,61.9 305.3,64.3 269.1,70.0 232.9,79.5 196.7,85.1 160.5,101.3 124.4,103.7 88.2,104.6 52.0,108.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.1 88.2,190.2 124.4,195.9 160.5,194.0 196.7,185.6 232.9,177.7 269.1,163.8 305.3,159.6 341.5,153.6 377.6,151.9 413.8,154.6 450.0,143.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,107.0 124.4,105.9 160.5,104.0 196.7,102.3 232.9,101.4 269.1,92.6 305.3,91.2 341.5,87.7 377.6,80.4 413.8,72.1 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.0 124.4,101.9 160.5,99.2 196.7,92.9 232.9,93.4 269.1,89.2 305.3,81.4 341.5,75.6 377.6,65.8 413.8,54.3 450.0,47.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,103.0 124.4,98.8 160.5,96.8 196.7,90.7 232.9,91.3 269.1,83.5 305.3,70.5 341.5,61.4 377.6,54.3 413.8,43.6 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,104.6 124.4,103.7 160.5,101.3 196.7,85.1 232.9,79.5 269.1,70.0 305.3,64.3 341.5,61.9 377.6,48.1 413.8,36.3 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.54 ns | 1.76 µs | 3.52 µs | 3.76 µs | 2.94 µs |
| D38 | 4.22 ns | 3.53 µs | 4.36 µs | 4.49 µs | 3.67 µs |
| D57 | 2.55 µs | 5.72 µs | 7.47 µs | 8.84 µs | 6.36 µs |
| D76 | 2.82 µs | 6.56 µs | 8.53 µs | 10.8 µs | 7.59 µs |
| D115 | 4.83 µs | 13 µs | 12.6 µs | 21 µs | 29 µs |
| D153 | 2.06 µs | 7.73 µs | 16.2 µs | 15.6 µs | 40.6 µs |
| D230 | 3.11 µs | 15.5 µs | 18.3 µs | 29 µs | 86.1 µs |
| D307 | 2.95 µs | 17.2 µs | 55.3 µs | 81.8 µs | 127 µs |
| D462 | 3.14 µs | 24.6 µs | 61.4 µs | 173 µs | 156 µs |
| D616 | 3.26 µs | 38.1 µs | 115 µs | 275 µs | 422 µs |
| D924 | 2.27 µs | 73.5 µs | 277 µs | 616 µs | 1.01 ms |
| D1232 | 3.34 µs | 99 µs | 457 µs | 932 µs | 2.65 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,192.1 124.4,112.7 160.5,111.4 196.7,104.7 232.9,115.3 269.1,110.2 305.3,110.9 341.5,110.1 377.6,109.6 413.8,114.1 450.0,109.3 450.0,26.5 413.8,38.5 377.6,49.3 341.5,61.7 305.3,64.1 269.1,69.0 232.9,78.3 196.7,82.5 160.5,99.1 124.4,101.3 88.2,108.2 52.0,110.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,192.1 124.4,112.7 160.5,111.4 196.7,104.7 232.9,115.3 269.1,110.2 305.3,110.9 341.5,110.1 377.6,109.6 413.8,114.1 450.0,109.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,108.6 124.4,102.6 160.5,100.9 196.7,92.5 232.9,98.9 269.1,90.2 305.3,89.0 341.5,84.5 377.6,79.1 413.8,71.0 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,106.0 124.4,99.3 160.5,97.7 196.7,92.8 232.9,89.7 269.1,88.2 305.3,74.5 341.5,73.2 377.6,65.4 413.8,54.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,105.6 124.4,97.2 160.5,94.8 196.7,86.5 232.9,90.2 269.1,82.5 305.3,69.6 341.5,60.3 377.6,54.6 413.8,44.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,108.2 124.4,101.3 160.5,99.1 196.7,82.5 232.9,78.3 269.1,69.0 305.3,64.1 341.5,61.7 377.6,49.3 413.8,38.5 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.81 ns | 133 ns | 157 ns | 159 ns | 119 ns |
| D38 | 4.92 ns | 163 ns | 177 ns | 189 ns | 140 ns |
| D57 | 178 ns | 311 ns | 327 ns | 347 ns | 245 ns |
| D76 | 197 ns | 329 ns | 357 ns | 410 ns | 265 ns |
| D115 | 384 ns | 567 ns | 694 ns | 678 ns | 1.01 µs |
| D153 | 261 ns | 499 ns | 721 ns | 554 ns | 1.12 µs |
| D230 | 584 ns | 897 ns | 743 ns | 840 ns | 1.92 µs |
| D307 | 793 ns | 1.21 µs | 1.69 µs | 2.15 µs | 2.87 µs |
| D462 | 882 ns | 1.42 µs | 1.52 µs | 3.31 µs | 2.57 µs |
| D616 | 1.09 µs | 1.85 µs | 2.52 µs | 4.14 µs | 5.67 µs |
| D924 | 885 ns | 2.37 µs | 4.51 µs | 7.7 µs | 11.3 µs |
| D1232 | 1.64 µs | 2.76 µs | 7.43 µs | 11 µs | 27.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,182.3 124.4,120.0 160.5,118.3 196.7,106.6 232.9,113.4 269.1,99.3 305.3,94.0 341.5,92.2 377.6,88.4 413.8,92.1 450.0,81.4 450.0,32.1 413.8,47.9 377.6,59.9 341.5,73.6 305.3,71.7 269.1,78.6 232.9,88.0 196.7,89.9 160.5,113.1 124.4,114.4 88.2,124.2 52.0,126.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,182.3 124.4,120.0 160.5,118.3 196.7,106.6 232.9,113.4 269.1,99.3 305.3,94.0 341.5,92.2 377.6,88.4 413.8,92.1 450.0,81.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.0 88.2,121.5 124.4,110.3 160.5,109.3 196.7,99.9 232.9,102.1 269.1,91.9 305.3,86.6 341.5,83.9 377.6,79.3 413.8,75.0 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,120.1 124.4,109.4 160.5,107.9 196.7,96.3 232.9,95.7 269.1,95.2 305.3,80.9 341.5,82.7 377.6,74.0 413.8,63.8 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,119.0 124.4,108.4 160.5,105.5 196.7,96.8 232.9,100.2 269.1,93.0 305.3,76.7 341.5,69.2 377.6,65.3 413.8,54.5 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,126.9 88.2,124.2 124.4,114.4 160.5,113.1 196.7,89.9 232.9,88.0 269.1,78.6 305.3,71.7 341.5,73.6 377.6,59.9 413.8,47.9 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.8 ns | 153 ns | 196 ns | 200 ns | 165 ns |
| D38 | 4.92 ns | 195 ns | 216 ns | 199 ns | 172 ns |
| D57 | 272 ns | 409 ns | 420 ns | 470 ns | 307 ns |
| D76 | 317 ns | 471 ns | 496 ns | 543 ns | 351 ns |
| D115 | 576 ns | 786 ns | 932 ns | 902 ns | 1.19 µs |
| D153 | 418 ns | 677 ns | 947 ns | 646 ns | 1.38 µs |
| D230 | 949 ns | 1.26 µs | 951 ns | 1.08 µs | 2.28 µs |
| D307 | 1.35 µs | 1.74 µs | 2.26 µs | 2.7 µs | 3.47 µs |
| D462 | 1.45 µs | 1.91 µs | 1.96 µs | 3.94 µs | 3.11 µs |
| D616 | 1.76 µs | 2.37 µs | 3.05 µs | 4.89 µs | 6.44 µs |
| D924 | 1.41 µs | 3.17 µs | 5.44 µs | 8.73 µs | 12.4 µs |
| D1232 | 2.54 µs | 3.57 µs | 8.77 µs | 12.4 µs | 30.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,182.3 124.4,112.6 160.5,110.0 196.7,99.6 232.9,105.2 269.1,90.9 305.3,84.8 341.5,83.6 377.6,80.2 413.8,84.0 450.0,73.8 450.0,30.9 413.8,46.3 377.6,57.6 341.5,70.3 305.3,68.4 269.1,75.7 232.9,84.4 196.7,86.9 160.5,108.2 124.4,110.5 88.2,120.6 52.0,121.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,182.3 124.4,112.6 160.5,110.0 196.7,99.6 232.9,105.2 269.1,90.9 305.3,84.8 341.5,83.6 377.6,80.2 413.8,84.0 450.0,73.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.6 88.2,118.4 124.4,105.5 160.5,103.1 196.7,94.2 232.9,96.8 269.1,86.0 305.3,80.3 341.5,78.7 377.6,75.0 413.8,70.0 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,116.6 124.4,105.1 160.5,102.2 196.7,91.2 232.9,90.9 269.1,90.9 305.3,75.8 341.5,78.3 377.6,70.6 413.8,60.6 450.0,52.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.9 88.2,118.0 124.4,103.1 160.5,100.6 196.7,91.8 232.9,97.6 269.1,88.7 305.3,72.7 341.5,66.2 377.6,62.4 413.8,52.4 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.3 88.2,120.6 124.4,110.5 160.5,108.2 196.7,86.9 232.9,84.4 269.1,75.7 305.3,68.4 341.5,70.3 377.6,57.6 413.8,46.3 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
