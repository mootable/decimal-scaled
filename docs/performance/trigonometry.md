# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.27 ns | 23.3 µs | 23 µs | 30.3 µs | 36.8 µs |
| D38 | 4.6 µs | 6.96 µs | 8.27 µs | 12.4 µs | 16.1 µs |
| D57 | 5.05 µs | 8.98 µs | 11 µs | 14.6 µs | 19.4 µs |
| D76 | 5.17 µs | 9.42 µs | 14.4 µs | 15 µs | 19.9 µs |
| D115 | 4.75 µs | 12 µs | 27.1 µs | 28.7 µs | 28.9 µs |
| D153 | 5.19 µs | 16.3 µs | 30.6 µs | 43.5 µs | 60.3 µs |
| D230 | 5.27 µs | 24.1 µs | 45.5 µs | 64.9 µs | 121 µs |
| D307 | 4.73 µs | 26.8 µs | 59.6 µs | 121 µs | 190 µs |
| D462 | 4.88 µs | 39 µs | 122 µs | 252 µs | 376 µs |
| D616 | 5.36 µs | 65.1 µs | 186 µs | 423 µs | 658 µs |
| D924 | 5.69 µs | 133 µs | 421 µs | 941 µs | 1.54 ms |
| D1232 | 5.6 µs | 212 µs | 698 µs | 1.44 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,105.3 124.4,104.2 160.5,103.9 196.7,104.9 232.9,103.8 269.1,103.7 305.3,105.0 341.5,104.6 377.6,103.4 413.8,102.7 450.0,102.9 450.0,23.1 413.8,33.2 377.6,43.8 341.5,50.7 305.3,59.1 269.1,64.7 232.9,73.4 196.7,82.5 160.5,87.2 124.4,87.5 88.2,89.8 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,105.3 124.4,104.2 160.5,103.9 196.7,104.9 232.9,103.8 269.1,103.7 305.3,105.0 341.5,104.6 377.6,103.4 413.8,102.7 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,100.2 124.4,97.1 160.5,96.5 196.7,93.4 232.9,89.6 269.1,84.8 305.3,83.5 341.5,78.8 377.6,72.5 413.8,63.6 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,85.4 88.2,98.1 124.4,94.6 160.5,91.2 196.7,83.3 232.9,81.8 269.1,76.9 305.3,73.6 341.5,64.7 377.6,59.5 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.0 124.4,91.0 160.5,90.7 196.7,82.6 232.9,77.5 269.1,72.5 305.3,64.8 341.5,55.7 377.6,49.3 413.8,39.3 450.0,34.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.8 124.4,87.5 160.5,87.2 196.7,82.5 232.9,73.4 269.1,64.7 305.3,59.1 341.5,50.7 377.6,43.8 413.8,33.2 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 31.1 µs | 27.5 µs | 36.4 µs | 35.9 µs |
| D38 | 27.4 µs | 33.2 µs | 44.3 µs | 57.3 µs | 66.1 µs |
| D57 | 3.73 µs | 4.84 µs | 5.05 µs | 6.1 µs | 8.19 µs |
| D76 | 3.71 µs | 5.13 µs | 5.7 µs | 6.5 µs | 9.17 µs |
| D115 | 6.16 µs | 9.45 µs | 13.5 µs | 14.6 µs | 14.4 µs |
| D153 | 6.68 µs | 10.7 µs | 16.2 µs | 23.6 µs | 29.8 µs |
| D230 | 8.85 µs | 15.9 µs | 28.3 µs | 38.6 µs | 69.3 µs |
| D307 | 12.3 µs | 26.4 µs | 47 µs | 83.6 µs | 138 µs |
| D462 | 12.2 µs | 32.7 µs | 85.1 µs | 167 µs | 261 µs |
| D616 | 23 µs | 75.7 µs | 150 µs | 315 µs | 519 µs |
| D924 | 33.3 µs | 161 µs | 380 µs | 801 µs | 1.34 ms |
| D1232 | 43 µs | 268 µs | 712 µs | 1.35 ms | 3.07 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,138.1 124.4,181.4 160.5,181.5 196.7,170.5 232.9,168.7 269.1,162.6 305.3,155.4 341.5,155.6 377.6,141.9 413.8,133.8 450.0,128.3 450.0,35.6 413.8,53.6 377.6,74.3 341.5,89.2 305.3,103.0 269.1,118.0 232.9,136.3 196.7,152.1 160.5,161.9 124.4,164.3 88.2,119.0 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,138.1 124.4,181.4 160.5,181.5 196.7,170.5 232.9,168.7 269.1,162.6 305.3,155.4 341.5,155.6 377.6,141.9 413.8,133.8 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,133.9 124.4,175.8 160.5,174.5 196.7,161.2 232.9,158.5 269.1,149.9 305.3,138.9 341.5,134.3 377.6,116.1 413.8,99.7 450.0,88.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,138.0 88.2,127.7 124.4,174.8 160.5,172.2 196.7,153.6 232.9,149.6 269.1,137.4 305.3,126.4 341.5,113.5 377.6,101.2 413.8,81.0 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,122.1 124.4,170.7 160.5,169.3 196.7,151.8 232.9,141.4 269.1,130.7 305.3,113.9 341.5,98.8 377.6,85.1 413.8,64.8 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,119.0 124.4,164.3 160.5,161.9 196.7,152.1 232.9,136.3 269.1,118.0 305.3,103.0 341.5,89.2 377.6,74.3 413.8,53.6 450.0,35.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 23.2 µs | 23 µs | 30.3 µs | 36.8 µs |
| D38 | 4.6 µs | 6.89 µs | 8.21 µs | 12.4 µs | 16 µs |
| D57 | 5.01 µs | 8.98 µs | 10.9 µs | 14.5 µs | 19.2 µs |
| D76 | 5.15 µs | 9.54 µs | 14.3 µs | 15 µs | 19.8 µs |
| D115 | 4.73 µs | 12 µs | 25.3 µs | 28.9 µs | 27.8 µs |
| D153 | 5.11 µs | 16.1 µs | 28.6 µs | 43.6 µs | 59.8 µs |
| D230 | 5.22 µs | 23.9 µs | 43.7 µs | 65 µs | 120 µs |
| D307 | 4.7 µs | 26.7 µs | 59.4 µs | 120 µs | 190 µs |
| D462 | 4.87 µs | 38.5 µs | 123 µs | 251 µs | 374 µs |
| D616 | 5.31 µs | 65.3 µs | 185 µs | 423 µs | 656 µs |
| D924 | 5.48 µs | 134 µs | 419 µs | 931 µs | 1.53 ms |
| D1232 | 5.54 µs | 212 µs | 700 µs | 1.44 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.2 88.2,105.4 124.4,104.3 160.5,103.9 196.7,105.0 232.9,104.0 269.1,103.8 305.3,105.1 341.5,104.6 377.6,103.6 413.8,103.2 450.0,103.0 450.0,23.1 413.8,33.3 377.6,43.8 341.5,50.8 305.3,59.2 269.1,64.8 232.9,73.5 196.7,83.0 160.5,87.2 124.4,87.6 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.2 88.2,105.4 124.4,104.3 160.5,103.9 196.7,105.0 232.9,104.0 269.1,103.8 305.3,105.1 341.5,104.6 377.6,103.6 413.8,103.2 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,100.3 124.4,97.1 160.5,96.3 196.7,93.5 232.9,89.8 269.1,84.9 305.3,83.5 341.5,79.0 377.6,72.4 413.8,63.5 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,85.4 88.2,98.2 124.4,94.6 160.5,91.3 196.7,84.2 232.9,82.7 269.1,77.4 305.3,73.6 341.5,64.6 377.6,59.5 413.8,49.4 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.0 124.4,91.1 160.5,90.7 196.7,82.6 232.9,77.4 269.1,72.5 305.3,64.8 341.5,55.7 377.6,49.3 413.8,39.5 450.0,34.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,87.6 160.5,87.2 196.7,83.0 232.9,73.5 269.1,64.8 305.3,59.2 341.5,50.8 377.6,43.8 413.8,33.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 23 µs | 22.5 µs | 29.7 µs | 36.1 µs |
| D38 | 4.22 ns | 27.1 µs | 36.2 µs | 42.9 µs | 55.2 µs |
| D57 | 2.11 ns | 6.02 µs | 7.32 µs | 8.92 µs | 12.8 µs |
| D76 | 2.23 ns | 6.92 µs | 8.75 µs | 10.1 µs | 13.9 µs |
| D115 | 13.4 ns | 13.9 µs | 20.3 µs | 21.6 µs | 22.4 µs |
| D153 | 16 ns | 16 µs | 23.5 µs | 36.4 µs | 44.1 µs |
| D230 | 32.1 ns | 24.4 µs | 44.9 µs | 56.9 µs | 95 µs |
| D307 | 44.8 ns | 36.9 µs | 66 µs | 113 µs | 175 µs |
| D462 | 69.5 ns | 46.9 µs | 118 µs | 208 µs | 317 µs |
| D616 | 88.5 ns | 114 µs | 204 µs | 402 µs | 616 µs |
| D924 | 119 ns | 253 µs | 498 µs | 946 µs | 1.47 ms |
| D1232 | 154 ns | 384 µs | 940 µs | 1.48 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,200.0 196.7,177.8 232.9,175.6 269.1,166.9 305.3,162.8 341.5,157.4 377.6,154.4 413.8,150.7 450.0,147.5 450.0,24.5 413.8,33.8 377.6,44.6 341.5,52.8 305.3,60.2 269.1,67.8 232.9,77.3 196.7,85.7 160.5,91.7 124.4,92.7 88.2,74.5 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,200.7 160.5,200.0 196.7,177.8 232.9,175.6 269.1,166.9 305.3,162.8 341.5,157.4 377.6,154.4 413.8,150.7 450.0,147.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,83.3 124.4,102.0 160.5,100.3 196.7,91.6 232.9,89.9 269.1,84.6 305.3,79.5 341.5,76.5 377.6,65.6 413.8,55.6 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,85.6 88.2,79.8 124.4,99.6 160.5,97.4 196.7,86.9 232.9,85.1 269.1,77.1 305.3,72.3 341.5,65.1 377.6,58.3 413.8,47.2 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,77.6 124.4,97.1 160.5,95.6 196.7,86.1 232.9,79.7 269.1,74.1 305.3,65.7 341.5,58.0 377.6,49.9 413.8,39.3 450.0,33.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,74.5 124.4,92.7 160.5,91.7 196.7,85.7 232.9,77.3 269.1,67.8 305.3,60.2 341.5,52.8 377.6,44.6 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.63 µs | 2.08 µs | 2.82 µs | 3.06 µs |
| D38 | 6.23 µs | 8.12 µs | 6.88 µs | 10.7 µs | 14 µs |
| D57 | 4.14 µs | 7.78 µs | 9.49 µs | 13 µs | 5.42 µs |
| D76 | 4.26 µs | 8.18 µs | 12.5 µs | 13.3 µs | 17.8 µs |
| D115 | 3.9 µs | 10.3 µs | 23.2 µs | 25.4 µs | 25.3 µs |
| D153 | 4.25 µs | 14.1 µs | 24.5 µs | 39.1 µs | 55.4 µs |
| D230 | 4.37 µs | 21 µs | 39.6 µs | 61.5 µs | 113 µs |
| D307 | 3.89 µs | 23.6 µs | 48.7 µs | 112 µs | 180 µs |
| D462 | 2.97 µs | 30.9 µs | 103 µs | 219 µs | 318 µs |
| D616 | 4.4 µs | 59.6 µs | 172 µs | 394 µs | 620 µs |
| D924 | 4.57 µs | 122 µs | 390 µs | 885 µs | 1.44 ms |
| D1232 | 4.59 µs | 198 µs | 664 µs | 1.39 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,101.6 124.4,106.7 160.5,106.3 196.7,107.4 232.9,106.3 269.1,106.0 305.3,107.4 341.5,110.8 377.6,105.9 413.8,105.4 450.0,105.4 450.0,23.6 413.8,34.0 377.6,44.5 341.5,52.8 305.3,59.8 269.1,65.7 232.9,74.5 196.7,84.2 160.5,88.6 124.4,103.3 88.2,91.5 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,101.6 124.4,106.7 160.5,106.3 196.7,107.4 232.9,106.3 269.1,106.0 305.3,107.4 341.5,110.8 377.6,105.9 413.8,105.4 450.0,105.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,98.3 124.4,98.8 160.5,98.2 196.7,95.3 232.9,91.4 269.1,86.5 305.3,85.1 341.5,81.7 377.6,73.6 413.8,64.7 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.2 88.2,100.4 124.4,96.4 160.5,93.0 196.7,85.2 232.9,84.6 269.1,78.6 305.3,76.1 341.5,66.8 377.6,60.4 413.8,50.3 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,94.9 124.4,92.5 160.5,92.2 196.7,84.1 232.9,78.8 269.1,73.2 305.3,65.8 341.5,57.4 377.6,50.1 413.8,40.1 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,91.5 124.4,103.3 160.5,88.6 196.7,84.2 232.9,74.5 269.1,65.7 305.3,59.8 341.5,52.8 377.6,44.5 413.8,34.0 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.81 µs | 7.29 µs | 9.69 µs | 10.6 µs |
| D38 | 4.22 ns | 8.82 µs | 11.8 µs | 13.9 µs | 16 µs |
| D57 | 649 ns | 6.01 µs | 6.99 µs | 8.54 µs | 11.8 µs |
| D76 | 620 ns | 6.55 µs | 7.93 µs | 9.14 µs | 13.9 µs |
| D115 | 1.03 µs | 12.9 µs | 19.1 µs | 22 µs | 22.3 µs |
| D153 | 1.18 µs | 15 µs | 23.7 µs | 36.4 µs | 48.2 µs |
| D230 | 1.48 µs | 23.3 µs | 44.3 µs | 63.7 µs | 119 µs |
| D307 | 2.21 µs | 40.2 µs | 77.3 µs | 144 µs | 246 µs |
| D462 | 2.2 µs | 50.8 µs | 147 µs | 296 µs | 473 µs |
| D616 | 4.02 µs | 122 µs | 260 µs | 566 µs | 953 µs |
| D924 | 5.7 µs | 267 µs | 667 µs | 1.47 ms | 2.5 ms |
| D1232 | 7.63 µs | 458 µs | 1.28 ms | 2.52 ms | 5.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,129.6 160.5,130.2 196.7,123.9 232.9,122.2 269.1,119.5 305.3,114.5 341.5,114.5 377.6,107.0 413.8,102.7 450.0,99.1 450.0,16.8 413.8,27.2 377.6,39.2 341.5,47.9 305.3,56.0 269.1,65.0 232.9,76.2 196.7,85.8 160.5,91.6 124.4,93.6 88.2,89.9 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,129.6 160.5,130.2 196.7,123.9 232.9,122.2 269.1,119.5 305.3,114.5 341.5,114.5 377.6,107.0 413.8,102.7 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,97.3 124.4,102.0 160.5,101.0 196.7,92.5 232.9,90.7 269.1,85.2 305.3,78.4 341.5,75.6 377.6,64.7 413.8,54.9 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,93.7 124.4,100.2 160.5,98.6 196.7,87.7 232.9,85.0 269.1,77.2 305.3,70.3 341.5,62.4 377.6,55.3 413.8,43.6 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,91.7 124.4,97.7 160.5,96.8 196.7,86.0 232.9,79.7 269.1,72.7 305.3,62.6 341.5,53.7 377.6,45.6 413.8,33.8 450.0,27.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,89.9 124.4,93.6 160.5,91.6 196.7,85.8 232.9,76.2 269.1,65.0 305.3,56.0 341.5,47.9 377.6,39.2 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 4.26 µs | 4.63 µs | 5.91 µs | 6.43 µs |
| D38 | 5.62 ns | 5.5 µs | 7.33 µs | 8.6 µs | 9.69 µs |
| D57 | 2.81 ns | 3.63 µs | 4.36 µs | 5.19 µs | 9.46 µs |
| D76 | 3.14 ns | 3.96 µs | 5.16 µs | 5.78 µs | 8.74 µs |
| D115 | 17.4 ns | 4.68 µs | 10.2 µs | 13.2 µs | 11.5 µs |
| D153 | 22.4 ns | 5.56 µs | 10.1 µs | 19 µs | 29.5 µs |
| D230 | 52.8 ns | 9.53 µs | 20.6 µs | 33.3 µs | 68.2 µs |
| D307 | 81.6 ns | 11.8 µs | 26 µs | 67.2 µs | 116 µs |
| D462 | 135 ns | 13.9 µs | 60.4 µs | 142 µs | 219 µs |
| D616 | 167 ns | 33.4 µs | 111 µs | 257 µs | 428 µs |
| D924 | 212 ns | 73.6 µs | 255 µs | 616 µs | 1.04 ms |
| D1232 | 387 ns | 127 µs | 450 µs | 1.01 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.4 269.1,160.8 305.3,155.4 341.5,149.1 377.6,146.5 413.8,143.5 450.0,136.1 450.0,27.5 413.8,38.1 377.6,49.1 341.5,57.4 305.3,65.3 269.1,71.9 232.9,82.3 196.7,93.9 160.5,97.4 124.4,96.4 88.2,96.1 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.4 269.1,160.8 305.3,155.4 341.5,149.1 377.6,146.5 413.8,143.5 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,103.1 124.4,108.3 160.5,107.2 196.7,105.1 232.9,103.0 269.1,96.3 305.3,93.7 341.5,91.7 377.6,80.7 413.8,70.9 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,99.6 124.4,106.0 160.5,103.9 196.7,95.5 232.9,95.6 269.1,86.8 305.3,83.9 341.5,73.4 377.6,65.9 413.8,55.5 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.3 88.2,97.6 124.4,103.9 160.5,102.5 196.7,92.2 232.9,87.8 269.1,80.8 305.3,72.1 341.5,62.8 377.6,55.5 413.8,44.6 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,96.1 124.4,96.4 160.5,97.4 196.7,93.9 232.9,82.3 269.1,71.9 305.3,65.3 341.5,57.4 377.6,49.1 413.8,38.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.91 µs | 7.91 µs | 10.2 µs | 11.1 µs |
| D38 | 4.57 ns | 9.42 µs | 12.5 µs | 14.5 µs | 16.5 µs |
| D57 | 2.48 ns | 5.73 µs | 7.14 µs | 7.96 µs | 10.9 µs |
| D76 | 3.17 ns | 6.1 µs | 7.38 µs | 8.41 µs | 11.7 µs |
| D115 | 9.97 ns | 12.7 µs | 12.3 µs | 18.4 µs | 17.2 µs |
| D153 | 21.6 ns | 8.11 µs | 16.2 µs | 24.2 µs | 35.1 µs |
| D230 | 52.6 ns | 13.5 µs | 24 µs | 39.8 µs | 78.4 µs |
| D307 | 82.5 ns | 15.9 µs | 48.5 µs | 78.9 µs | 125 µs |
| D462 | 132 ns | 21.5 µs | 80.4 µs | 166 µs | 232 µs |
| D616 | 169 ns | 39.3 µs | 124 µs | 269 µs | 412 µs |
| D924 | 199 ns | 87.3 µs | 267 µs | 609 µs | 911 µs |
| D1232 | 391 ns | 141 µs | 445 µs | 896 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,198.7 160.5,195.7 196.7,181.5 232.9,171.9 269.1,160.8 305.3,155.2 341.5,149.4 377.6,146.3 413.8,144.3 450.0,135.9 450.0,25.6 413.8,39.7 377.6,49.6 341.5,56.7 305.3,64.4 269.1,70.2 232.9,80.1 196.7,89.0 160.5,93.8 124.4,94.6 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,198.7 160.5,195.7 196.7,181.5 232.9,171.9 269.1,160.8 305.3,155.2 341.5,149.4 377.6,146.3 413.8,144.3 450.0,135.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.5 124.4,102.6 160.5,101.9 196.7,92.8 232.9,98.3 269.1,92.0 305.3,89.9 341.5,86.2 377.6,78.7 413.8,68.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,93.0 124.4,99.9 160.5,99.5 196.7,93.1 232.9,89.7 269.1,84.8 305.3,76.1 341.5,69.9 377.6,64.4 413.8,55.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,91.1 124.4,98.5 160.5,97.9 196.7,88.1 232.9,84.8 269.1,78.6 305.3,70.1 341.5,60.9 377.6,54.9 413.8,44.7 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.6 160.5,93.8 196.7,89.0 232.9,80.1 269.1,70.2 305.3,64.4 341.5,56.7 377.6,49.6 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 4.12 µs | 4.47 µs | 5.85 µs | 6.36 µs |
| D38 | 4.92 ns | 5.32 µs | 7.11 µs | 8.38 µs | 9.63 µs |
| D57 | 2.78 ns | 3.33 µs | 4.05 µs | 5.06 µs | 9.45 µs |
| D76 | 3.86 ns | 3.69 µs | 4.84 µs | 5.67 µs | 8.42 µs |
| D115 | 17.5 ns | 4.46 µs | 9.81 µs | 12.4 µs | 12.2 µs |
| D153 | 22.5 ns | 5.31 µs | 9.78 µs | 19.9 µs | 28.7 µs |
| D230 | 52.6 ns | 9.66 µs | 19.5 µs | 33 µs | 66.9 µs |
| D307 | 76.9 ns | 11.3 µs | 24.9 µs | 65.8 µs | 115 µs |
| D462 | 127 ns | 13.6 µs | 59.1 µs | 140 µs | 216 µs |
| D616 | 161 ns | 32.9 µs | 109 µs | 253 µs | 425 µs |
| D924 | 196 ns | 73.1 µs | 249 µs | 607 µs | 1.03 ms |
| D1232 | 388 ns | 126 µs | 450 µs | 1.01 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.2 196.7,174.5 232.9,171.4 269.1,160.8 305.3,156.1 341.5,149.9 377.6,146.9 413.8,144.5 450.0,136.0 450.0,27.6 413.8,38.2 377.6,49.2 341.5,57.6 305.3,65.4 269.1,72.1 232.9,82.6 196.7,93.2 160.5,97.8 124.4,96.4 88.2,96.2 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,197.3 160.5,193.2 196.7,174.5 232.9,171.4 269.1,160.8 305.3,156.1 341.5,149.9 377.6,146.9 413.8,144.5 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,103.5 124.4,109.3 160.5,108.1 196.7,105.7 232.9,103.6 269.1,96.1 305.3,94.2 341.5,91.9 377.6,80.9 413.8,71.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.7 88.2,99.9 124.4,106.9 160.5,104.7 196.7,95.9 232.9,96.0 269.1,87.4 305.3,84.4 341.5,73.7 377.6,66.1 413.8,55.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,97.9 124.4,104.2 160.5,102.8 196.7,93.0 232.9,87.1 269.1,80.9 305.3,72.3 341.5,63.0 377.6,55.6 413.8,44.8 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,96.2 124.4,96.4 160.5,97.8 196.7,93.2 232.9,82.6 269.1,72.1 305.3,65.4 341.5,57.6 377.6,49.2 413.8,38.2 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.92 µs | 7.91 µs | 10.2 µs | 11.1 µs |
| D38 | 4.57 ns | 9.42 µs | 12.5 µs | 14.5 µs | 16.5 µs |
| D57 | 12.3 ns | 5.73 µs | 7.14 µs | 7.97 µs | 10.9 µs |
| D76 | 12.1 ns | 6.09 µs | 7.37 µs | 8.45 µs | 11.8 µs |
| D115 | 10.3 ns | 12.6 µs | 12.1 µs | 18.7 µs | 17.2 µs |
| D153 | 20.8 ns | 8.17 µs | 15.4 µs | 24 µs | 35.1 µs |
| D230 | 53.2 ns | 13.5 µs | 24 µs | 39.9 µs | 78.6 µs |
| D307 | 76.6 ns | 15.9 µs | 48.6 µs | 79.2 µs | 124 µs |
| D462 | 135 ns | 21.6 µs | 81.2 µs | 165 µs | 231 µs |
| D616 | 160 ns | 39.4 µs | 125 µs | 269 µs | 414 µs |
| D924 | 203 ns | 87.2 µs | 268 µs | 606 µs | 911 µs |
| D1232 | 378 ns | 141 µs | 445 µs | 897 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,178.9 160.5,179.0 196.7,181.1 232.9,172.4 269.1,160.7 305.3,156.2 341.5,149.1 377.6,147.0 413.8,144.1 450.0,136.4 450.0,25.6 413.8,39.7 377.6,49.5 341.5,56.7 305.3,64.5 269.1,70.1 232.9,80.1 196.7,89.0 160.5,93.7 124.4,94.6 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,178.9 160.5,179.0 196.7,181.1 232.9,172.4 269.1,160.7 305.3,156.2 341.5,149.1 377.6,147.0 413.8,144.1 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.5 124.4,102.6 160.5,101.9 196.7,92.8 232.9,98.2 269.1,92.0 305.3,89.9 341.5,86.1 377.6,78.7 413.8,68.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,93.0 124.4,99.9 160.5,99.5 196.7,93.3 232.9,90.4 269.1,84.8 305.3,76.1 341.5,69.7 377.6,64.4 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,91.1 124.4,98.5 160.5,97.8 196.7,88.0 232.9,84.9 269.1,78.6 305.3,70.0 341.5,61.0 377.6,54.9 413.8,44.8 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.6 160.5,93.7 196.7,89.0 232.9,80.1 269.1,70.1 305.3,64.5 341.5,56.7 377.6,49.5 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 8.22 µs | 8.33 µs | 10.8 µs | 11.7 µs |
| D38 | 4.92 ns | 9.92 µs | 13.2 µs | 15.6 µs | 17.7 µs |
| D57 | 3.17 ns | 4.67 µs | 5.49 µs | 6.7 µs | 9.43 µs |
| D76 | 4.22 ns | 4.86 µs | 6.56 µs | 7.27 µs | 10.5 µs |
| D115 | 17.5 ns | 5.93 µs | 13 µs | 15.1 µs | 13.7 µs |
| D153 | 22.6 ns | 7.11 µs | 12.9 µs | 22.4 µs | 33.6 µs |
| D230 | 59.1 ns | 12.3 µs | 23.2 µs | 37.8 µs | 75.4 µs |
| D307 | 74.6 ns | 14.1 µs | 28.9 µs | 74.8 µs | 130 µs |
| D462 | 148 ns | 16.7 µs | 67.2 µs | 156 µs | 233 µs |
| D616 | 159 ns | 37 µs | 121 µs | 278 µs | 460 µs |
| D924 | 159 ns | 82.2 µs | 274 µs | 656 µs | 1.11 ms |
| D1232 | 374 ns | 142 µs | 486 µs | 1.08 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,192.1 196.7,174.5 232.9,171.3 269.1,159.4 305.3,156.5 341.5,148.0 377.6,147.1 413.8,147.1 450.0,136.5 450.0,26.9 413.8,37.3 377.6,48.2 341.5,56.6 305.3,63.9 269.1,70.7 232.9,80.7 196.7,91.8 160.5,95.1 124.4,96.4 88.2,88.6 52.0,93.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,195.7 160.5,192.1 196.7,174.5 232.9,171.3 269.1,159.4 305.3,156.5 341.5,148.0 377.6,147.1 413.8,147.1 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.1 88.2,95.8 124.4,105.2 160.5,104.7 196.7,102.2 232.9,99.9 269.1,93.1 305.3,91.4 341.5,89.4 377.6,79.5 413.8,69.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.0 88.2,92.3 124.4,103.2 160.5,101.0 196.7,92.5 232.9,92.6 269.1,85.3 305.3,82.6 341.5,72.1 377.6,64.8 413.8,54.6 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,90.2 124.4,100.7 160.5,99.7 196.7,90.6 232.9,85.7 269.1,79.2 305.3,70.7 341.5,61.7 377.6,54.5 413.8,43.8 450.0,37.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,88.6 124.4,96.4 160.5,95.1 196.7,91.8 232.9,80.7 269.1,70.7 305.3,63.9 341.5,56.6 377.6,48.2 413.8,37.3 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.92 µs | 8.01 µs | 10.4 µs | 11.2 µs |
| D38 | 4.22 ns | 9.58 µs | 12.5 µs | 14.5 µs | 16.5 µs |
| D57 | 2.82 µs | 5.83 µs | 7.42 µs | 8.34 µs | 11.3 µs |
| D76 | 2.86 µs | 6.5 µs | 7.76 µs | 8.66 µs | 12.1 µs |
| D115 | 5.41 µs | 13 µs | 12.8 µs | 18.9 µs | 17.6 µs |
| D153 | 3.07 µs | 8.44 µs | 16.2 µs | 25.1 µs | 36.6 µs |
| D230 | 3.16 µs | 14 µs | 24.9 µs | 40.7 µs | 79.8 µs |
| D307 | 3.14 µs | 17.1 µs | 49.7 µs | 79.7 µs | 126 µs |
| D462 | 3.4 µs | 22.1 µs | 82.7 µs | 167 µs | 234 µs |
| D616 | 3.77 µs | 40.8 µs | 125 µs | 271 µs | 417 µs |
| D924 | 4.06 µs | 89.5 µs | 270 µs | 612 µs | 921 µs |
| D1232 | 4.76 µs | 143 µs | 450 µs | 904 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,111.3 196.7,103.3 232.9,110.4 269.1,110.0 305.3,110.1 341.5,109.1 377.6,107.8 413.8,106.9 450.0,104.9 450.0,25.5 413.8,39.6 377.6,49.4 341.5,56.6 305.3,64.3 269.1,69.9 232.9,79.6 196.7,88.7 160.5,93.3 124.4,94.2 88.2,89.5 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,111.4 160.5,111.3 196.7,103.3 232.9,110.4 269.1,110.0 305.3,110.1 341.5,109.1 377.6,107.8 413.8,106.9 450.0,104.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.2 124.4,102.4 160.5,101.1 196.7,92.4 232.9,97.8 269.1,91.6 305.3,89.1 341.5,85.9 377.6,78.3 413.8,68.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,93.0 124.4,99.4 160.5,98.9 196.7,92.7 232.9,89.8 269.1,84.4 305.3,75.8 341.5,69.5 377.6,64.4 413.8,54.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.1 124.4,98.0 160.5,97.5 196.7,87.8 232.9,84.3 269.1,78.3 305.3,70.0 341.5,60.8 377.6,54.8 413.8,44.7 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,89.5 124.4,94.2 160.5,93.3 196.7,88.7 232.9,79.6 269.1,69.9 305.3,64.3 341.5,56.6 377.6,49.4 413.8,39.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.97 ns | 3.22 µs | 2.69 µs | 3.53 µs | 3.85 µs |
| D38 | 4.57 ns | 3.23 µs | 4.3 µs | 5.08 µs | 5.85 µs |
| D57 | 201 ns | 337 ns | 303 ns | 307 ns | 444 ns |
| D76 | 209 ns | 317 ns | 309 ns | 337 ns | 420 ns |
| D115 | 345 ns | 521 ns | 662 ns | 617 ns | 509 ns |
| D153 | 388 ns | 577 ns | 699 ns | 850 ns | 899 ns |
| D230 | 558 ns | 782 ns | 1.04 µs | 1.14 µs | 1.73 µs |
| D307 | 849 ns | 1.12 µs | 1.45 µs | 2.04 µs | 2.71 µs |
| D462 | 868 ns | 1.25 µs | 2.1 µs | 3.16 µs | 3.83 µs |
| D616 | 1.16 µs | 1.91 µs | 2.61 µs | 4.05 µs | 5.56 µs |
| D924 | 1.57 µs | 2.87 µs | 4.31 µs | 7.45 µs | 10.2 µs |
| D1232 | 2.25 µs | 4.06 µs | 7.25 µs | 10.4 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.0 88.2,183.6 124.4,117.8 160.5,117.2 196.7,108.5 232.9,106.5 269.1,100.1 305.3,92.8 341.5,92.5 377.6,87.4 413.8,82.1 450.0,75.9 450.0,30.7 413.8,49.7 377.6,60.2 341.5,66.7 305.3,72.7 269.1,80.5 232.9,91.8 196.7,101.7 160.5,105.1 124.4,104.1 88.2,59.3 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.0 88.2,183.6 124.4,117.8 160.5,117.2 196.7,108.5 232.9,106.5 269.1,100.1 305.3,92.8 341.5,92.5 377.6,87.4 413.8,82.1 450.0,75.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,69.6 124.4,108.9 160.5,110.0 196.7,101.3 232.9,99.5 269.1,94.3 305.3,88.0 341.5,86.2 377.6,78.8 413.8,71.7 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,72.8 88.2,64.6 124.4,110.7 160.5,110.4 196.7,97.2 232.9,96.2 269.1,89.3 305.3,83.6 341.5,77.1 377.6,73.3 413.8,64.6 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,61.8 124.4,110.5 160.5,108.9 196.7,98.4 232.9,92.8 269.1,87.8 305.3,77.6 341.5,70.0 377.6,65.7 413.8,55.1 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,59.3 124.4,104.1 160.5,105.1 196.7,101.7 232.9,91.8 269.1,80.5 305.3,72.7 341.5,66.7 377.6,60.2 413.8,49.7 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 158 ns | 156 ns | 184 ns | 184 ns |
| D38 | 4.92 ns | 183 ns | 202 ns | 203 ns | 206 ns |
| D57 | 311 ns | 424 ns | 406 ns | 424 ns | 531 ns |
| D76 | 308 ns | 414 ns | 418 ns | 426 ns | 552 ns |
| D115 | 585 ns | 724 ns | 872 ns | 808 ns | 613 ns |
| D153 | 643 ns | 768 ns | 902 ns | 1.05 µs | 1.16 µs |
| D230 | 965 ns | 1.15 µs | 1.37 µs | 1.46 µs | 2.13 µs |
| D307 | 1.39 µs | 1.65 µs | 1.97 µs | 2.57 µs | 3.36 µs |
| D462 | 1.45 µs | 1.63 µs | 2.68 µs | 3.81 µs | 4.47 µs |
| D616 | 1.89 µs | 2.5 µs | 3.23 µs | 4.78 µs | 6.37 µs |
| D924 | 2.52 µs | 3.78 µs | 5.31 µs | 8.53 µs | 11.3 µs |
| D1232 | 3.5 µs | 5.23 µs | 8.48 µs | 11.7 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,110.5 196.7,99.3 232.9,97.7 269.1,90.6 305.3,84.3 341.5,83.6 377.6,78.9 413.8,73.9 450.0,68.2 450.0,29.8 413.8,47.9 377.6,57.8 341.5,64.0 305.3,68.9 269.1,76.9 232.9,87.5 196.7,98.5 160.5,100.3 124.4,101.0 88.2,117.5 52.0,119.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,110.3 160.5,110.5 196.7,99.3 232.9,97.7 269.1,90.6 305.3,84.3 341.5,83.6 377.6,78.9 413.8,73.9 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,119.5 124.4,104.9 160.5,105.3 196.7,95.6 232.9,94.6 269.1,87.6 305.3,81.3 341.5,81.5 377.6,74.1 413.8,66.9 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.3 88.2,117.8 124.4,105.7 160.5,105.2 196.7,92.4 232.9,91.8 269.1,84.5 305.3,78.2 341.5,72.9 377.6,69.6 413.8,61.0 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,117.7 124.4,104.9 160.5,104.8 196.7,93.7 232.9,89.1 269.1,83.5 305.3,73.6 341.5,66.7 377.6,62.8 413.8,52.8 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,117.5 124.4,101.0 160.5,100.3 196.7,98.5 232.9,87.5 269.1,76.9 305.3,68.9 341.5,64.0 377.6,57.8 413.8,47.9 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
