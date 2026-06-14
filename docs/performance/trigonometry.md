# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.16 ns | 21.8 µs | 27.7 µs | 32.4 µs | 39.5 µs |
| D38 | 4.62 µs | 6.73 µs | 7.22 µs | 11.5 µs | 12.4 µs |
| D57 | 4.69 µs | 8.3 µs | 12.5 µs | 16.4 µs | 17.4 µs |
| D76 | 4.87 µs | 9.52 µs | 14.4 µs | 17.6 µs | 19.9 µs |
| D115 | 5.19 µs | 12 µs | 25.2 µs | 31.4 µs | 43.6 µs |
| D153 | 4.78 µs | 16.1 µs | 28.9 µs | 40.6 µs | 64 µs |
| D230 | 4.76 µs | 25.6 µs | 43.5 µs | 65.5 µs | 122 µs |
| D307 | 4.92 µs | 26.9 µs | 57.8 µs | 121 µs | 189 µs |
| D462 | 5.31 µs | 43.8 µs | 123 µs | 236 µs | 373 µs |
| D616 | 5.02 µs | 56.7 µs | 215 µs | 422 µs | 700 µs |
| D924 | 4.9 µs | 124 µs | 450 µs | 820 µs | 1.66 ms |
| D1232 | 5.34 µs | 213 µs | 699 µs | 1.5 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,105.3 124.4,105.1 160.5,104.6 196.7,103.8 232.9,104.9 269.1,104.9 305.3,104.5 341.5,103.6 377.6,104.3 413.8,104.6 450.0,103.5 450.0,23.1 413.8,32.3 377.6,43.0 341.5,50.8 305.3,59.2 269.1,64.7 232.9,72.7 196.7,77.4 160.5,87.2 124.4,88.8 88.2,93.0 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,105.3 124.4,105.1 160.5,104.6 196.7,103.8 232.9,104.9 269.1,104.9 305.3,104.5 341.5,103.6 377.6,104.3 413.8,104.6 450.0,103.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,100.6 124.4,98.0 160.5,96.3 196.7,93.4 232.9,89.8 269.1,84.0 305.3,83.5 341.5,77.4 377.6,74.2 413.8,64.5 450.0,57.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.8 124.4,92.9 160.5,91.2 196.7,84.2 232.9,82.6 269.1,77.5 305.3,73.9 341.5,64.6 377.6,57.7 413.8,48.5 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,94.0 124.4,89.6 160.5,88.7 196.7,81.5 232.9,78.3 269.1,72.4 305.3,64.8 341.5,56.5 377.6,49.3 413.8,41.0 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,93.0 124.4,88.8 160.5,87.2 196.7,77.4 232.9,72.7 269.1,64.7 305.3,59.2 341.5,50.8 377.6,43.0 413.8,32.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 29.2 µs | 33.2 µs | 39 µs | 38.4 µs |
| D38 | 27.4 µs | 17.1 µs | 41.3 µs | 53.3 µs | 51.3 µs |
| D57 | 3.29 µs | 4.27 µs | 5.45 µs | 6.62 µs | 7.5 µs |
| D76 | 3.3 µs | 5.09 µs | 5.7 µs | 7.68 µs | 9.13 µs |
| D115 | 6.57 µs | 9.31 µs | 13.3 µs | 16.6 µs | 23.5 µs |
| D153 | 6.07 µs | 10.7 µs | 16 µs | 21.4 µs | 33.5 µs |
| D230 | 8.39 µs | 16.9 µs | 28.5 µs | 38.5 µs | 69.2 µs |
| D307 | 12.4 µs | 26.1 µs | 44.6 µs | 84.3 µs | 137 µs |
| D462 | 13 µs | 38.4 µs | 85.8 µs | 161 µs | 261 µs |
| D616 | 21.1 µs | 60.9 µs | 181 µs | 314 µs | 557 µs |
| D924 | 31.7 µs | 155 µs | 397 µs | 690 µs | 1.44 ms |
| D1232 | 41.6 µs | 270 µs | 714 µs | 1.43 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.5 88.2,138.1 124.4,184.1 160.5,184.1 196.7,169.1 232.9,170.9 269.1,163.8 305.3,155.3 341.5,154.3 377.6,143.8 413.8,135.0 450.0,129.0 450.0,35.7 413.8,52.0 377.6,72.7 341.5,89.2 305.3,103.1 269.1,118.0 232.9,133.7 196.7,141.4 160.5,162.0 124.4,166.2 88.2,124.5 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.5 88.2,138.1 124.4,184.1 160.5,184.1 196.7,169.1 232.9,170.9 269.1,163.8 305.3,155.3 341.5,154.3 377.6,143.8 413.8,135.0 450.0,129.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,136.8 88.2,148.4 124.4,178.5 160.5,174.7 196.7,161.6 232.9,158.4 269.1,148.6 305.3,139.2 341.5,130.8 377.6,120.8 413.8,100.5 450.0,88.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,129.2 124.4,173.2 160.5,172.2 196.7,153.8 232.9,149.8 269.1,137.2 305.3,127.5 341.5,113.3 377.6,97.1 413.8,80.0 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,123.7 124.4,169.0 160.5,165.7 196.7,149.0 232.9,143.4 269.1,130.7 305.3,113.7 341.5,99.7 377.6,85.2 413.8,68.0 450.0,52.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,124.5 124.4,166.2 160.5,162.0 196.7,141.4 232.9,133.7 269.1,118.0 305.3,103.1 341.5,89.2 377.6,72.7 413.8,52.0 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 21.8 µs | 27.7 µs | 32.4 µs | 39.5 µs |
| D38 | 4.62 µs | 6.75 µs | 7.23 µs | 11.5 µs | 12.4 µs |
| D57 | 4.64 µs | 8.25 µs | 12.5 µs | 16.3 µs | 17.4 µs |
| D76 | 4.89 µs | 9.38 µs | 14.3 µs | 17.5 µs | 19.9 µs |
| D115 | 5.15 µs | 12 µs | 25.7 µs | 31.3 µs | 44.3 µs |
| D153 | 4.68 µs | 16 µs | 29 µs | 40.8 µs | 63.9 µs |
| D230 | 4.72 µs | 25.6 µs | 43.5 µs | 64.9 µs | 122 µs |
| D307 | 4.87 µs | 27.5 µs | 58.8 µs | 122 µs | 189 µs |
| D462 | 5.25 µs | 43.6 µs | 123 µs | 238 µs | 372 µs |
| D616 | 4.88 µs | 56.9 µs | 215 µs | 423 µs | 703 µs |
| D924 | 4.88 µs | 124 µs | 445 µs | 824 µs | 1.66 ms |
| D1232 | 5.26 µs | 212 µs | 702 µs | 1.5 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,105.3 124.4,105.2 160.5,104.6 196.7,103.9 232.9,105.1 269.1,105.0 305.3,104.7 341.5,103.7 377.6,104.6 413.8,104.6 450.0,103.7 450.0,23.1 413.8,32.3 377.6,42.9 341.5,50.8 305.3,59.3 269.1,64.7 232.9,72.7 196.7,77.2 160.5,87.2 124.4,88.8 88.2,93.1 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,105.3 124.4,105.2 160.5,104.6 196.7,103.9 232.9,105.1 269.1,105.0 305.3,104.7 341.5,103.7 377.6,104.6 413.8,104.6 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,100.6 124.4,98.1 160.5,96.5 196.7,93.5 232.9,89.9 269.1,84.0 305.3,83.1 341.5,77.5 377.6,74.1 413.8,64.5 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.7 124.4,93.0 160.5,91.3 196.7,84.0 232.9,82.5 269.1,77.5 305.3,73.7 341.5,64.6 377.6,57.6 413.8,48.6 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,94.0 124.4,89.6 160.5,88.8 196.7,81.6 232.9,78.3 269.1,72.5 305.3,64.7 341.5,56.4 377.6,49.2 413.8,41.0 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,93.1 124.4,88.8 160.5,87.2 196.7,77.2 232.9,72.7 269.1,64.7 305.3,59.3 341.5,50.8 377.6,42.9 413.8,32.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 21.5 µs | 27.1 µs | 31.8 µs | 38.7 µs |
| D38 | 4.22 ns | 12.9 µs | 33.7 µs | 39.9 µs | 42.8 µs |
| D57 | 1.95 ns | 5.35 µs | 8.06 µs | 9.71 µs | 11.6 µs |
| D76 | 2.02 ns | 6.94 µs | 8.73 µs | 11.8 µs | 13.9 µs |
| D115 | 12.4 ns | 14.2 µs | 20.7 µs | 24 µs | 35.7 µs |
| D153 | 16.5 ns | 16.7 µs | 24 µs | 33.4 µs | 47.7 µs |
| D230 | 27.9 ns | 26.2 µs | 45 µs | 56.9 µs | 95.4 µs |
| D307 | 44.7 ns | 37.6 µs | 62.7 µs | 112 µs | 175 µs |
| D462 | 75.3 ns | 58.8 µs | 118 µs | 197 µs | 318 µs |
| D616 | 80.7 ns | 91.6 µs | 243 µs | 404 µs | 671 µs |
| D924 | 102 ns | 232 µs | 536 µs | 791 µs | 1.6 ms |
| D1232 | 142 ns | 385 µs | 926 µs | 1.61 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,201.7 160.5,201.3 196.7,178.7 232.9,175.2 269.1,168.7 305.3,162.8 341.5,156.4 377.6,155.5 413.8,152.6 450.0,148.5 450.0,24.5 413.8,32.7 377.6,43.5 341.5,52.8 305.3,60.2 269.1,67.7 232.9,76.3 196.7,79.9 160.5,91.6 124.4,93.9 88.2,77.7 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,201.7 160.5,201.3 196.7,178.7 232.9,175.2 269.1,168.7 305.3,162.8 341.5,156.4 377.6,155.5 413.8,152.6 450.0,148.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.2 88.2,92.5 124.4,103.5 160.5,100.2 196.7,91.4 232.9,89.4 269.1,83.8 305.3,79.3 341.5,73.7 377.6,68.2 413.8,56.7 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,80.6 124.4,98.4 160.5,97.4 196.7,86.7 232.9,84.8 269.1,77.0 305.3,72.9 341.5,65.1 377.6,56.1 413.8,46.3 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,78.5 124.4,96.1 160.5,93.7 196.7,84.9 232.9,80.8 269.1,74.1 305.3,65.7 341.5,58.8 377.6,49.8 413.8,41.5 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,77.7 124.4,93.9 160.5,91.6 196.7,79.9 232.9,76.3 269.1,67.7 305.3,60.2 341.5,52.8 377.6,43.5 413.8,32.7 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.58 µs | 2.57 µs | 2.93 µs | 3.23 µs |
| D38 | 6.22 µs | 7.28 µs | 6.01 µs | 9.84 µs | 10.9 µs |
| D57 | 3.86 µs | 7.25 µs | 10.8 µs | 14.3 µs | 4.98 µs |
| D76 | 3.95 µs | 8.11 µs | 12.4 µs | 15.5 µs | 17.8 µs |
| D115 | 4.26 µs | 10.3 µs | 22.2 µs | 28.4 µs | 39.3 µs |
| D153 | 3.89 µs | 14.1 µs | 22.3 µs | 36.7 µs | 58.9 µs |
| D230 | 3.91 µs | 22.8 µs | 39.7 µs | 59.7 µs | 114 µs |
| D307 | 3.92 µs | 24.1 µs | 48.1 µs | 113 µs | 175 µs |
| D462 | 3.22 µs | 35.9 µs | 103 µs | 208 µs | 319 µs |
| D616 | 4.03 µs | 51.9 µs | 201 µs | 394 µs | 664 µs |
| D924 | 4.1 µs | 115 µs | 415 µs | 780 µs | 1.56 ms |
| D1232 | 4.4 µs | 201 µs | 663 µs | 1.44 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,101.6 124.4,107.5 160.5,107.2 196.7,106.3 232.9,107.4 269.1,107.4 305.3,107.4 341.5,109.8 377.6,107.0 413.8,106.8 450.0,105.9 450.0,23.6 413.8,33.0 377.6,43.6 341.5,52.7 305.3,60.2 269.1,65.6 232.9,73.7 196.7,78.7 160.5,88.6 124.4,104.4 88.2,94.7 52.0,109.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,101.6 124.4,107.5 160.5,107.2 196.7,106.3 232.9,107.4 269.1,107.4 305.3,107.4 341.5,109.8 377.6,107.0 413.8,106.8 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,99.7 124.4,99.7 160.5,98.3 196.7,95.3 232.9,91.5 269.1,85.5 305.3,84.8 341.5,79.9 377.6,75.3 413.8,65.4 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,102.0 124.4,94.8 160.5,93.0 196.7,85.8 232.9,85.8 269.1,78.6 305.3,76.2 341.5,66.7 377.6,58.5 413.8,49.5 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,95.9 124.4,91.3 160.5,90.3 196.7,82.8 232.9,79.6 269.1,73.5 305.3,65.6 341.5,58.1 377.6,50.1 413.8,41.7 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,94.7 124.4,104.4 160.5,88.6 196.7,78.7 232.9,73.7 269.1,65.6 305.3,60.2 341.5,52.7 377.6,43.6 413.8,33.0 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.35 µs | 8.83 µs | 10.3 µs | 11.3 µs |
| D38 | 4.22 ns | 5.17 µs | 11 µs | 12.9 µs | 12.5 µs |
| D57 | 498 ns | 5.42 µs | 7.36 µs | 9.06 µs | 11 µs |
| D76 | 501 ns | 6.52 µs | 7.95 µs | 11.1 µs | 13.8 µs |
| D115 | 1.27 µs | 12.8 µs | 19.2 µs | 25.2 µs | 36.8 µs |
| D153 | 1.05 µs | 14.7 µs | 24 µs | 33.4 µs | 54.4 µs |
| D230 | 1.4 µs | 24.5 µs | 44.7 µs | 63.7 µs | 119 µs |
| D307 | 2.25 µs | 39.6 µs | 70.3 µs | 145 µs | 241 µs |
| D462 | 2.39 µs | 59.5 µs | 147 µs | 289 µs | 473 µs |
| D616 | 3.73 µs | 97.3 µs | 313 µs | 563 µs | 1.01 ms |
| D924 | 5.46 µs | 258 µs | 703 µs | 1.28 ms | 2.67 ms |
| D1232 | 7.56 µs | 460 µs | 1.28 ms | 2.65 ms | 5.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,132.9 160.5,132.9 196.7,121.3 232.9,123.7 269.1,120.1 305.3,114.2 341.5,113.5 377.6,107.9 413.8,103.2 450.0,99.2 450.0,16.8 413.8,26.4 377.6,38.4 341.5,47.9 305.3,56.2 269.1,65.0 232.9,74.7 196.7,79.6 160.5,91.7 124.4,94.5 88.2,93.0 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,132.9 160.5,132.9 196.7,121.3 232.9,123.7 269.1,120.1 305.3,114.2 341.5,113.5 377.6,107.9 413.8,103.2 450.0,99.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,103.9 124.4,103.3 160.5,101.0 196.7,92.7 232.9,90.9 269.1,84.6 305.3,78.6 341.5,73.6 377.6,67.5 413.8,55.4 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,94.5 124.4,99.5 160.5,98.6 196.7,87.6 232.9,84.9 269.1,77.1 305.3,71.5 341.5,62.3 377.6,53.0 413.8,42.9 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.5 124.4,96.9 160.5,94.4 196.7,84.3 232.9,80.8 269.1,72.7 305.3,62.5 341.5,54.0 377.6,45.7 413.8,35.6 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,93.0 124.4,94.5 160.5,91.7 196.7,79.6 232.9,74.7 269.1,65.0 305.3,56.2 341.5,47.9 377.6,38.4 413.8,26.4 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 3.96 µs | 5.5 µs | 6.4 µs | 6.98 µs |
| D38 | 5.62 ns | 3.32 µs | 6.75 µs | 7.92 µs | 7.52 µs |
| D57 | 2.18 ns | 3.33 µs | 4.83 µs | 5.66 µs | 8.76 µs |
| D76 | 3.12 ns | 3.95 µs | 5.17 µs | 6.96 µs | 8.76 µs |
| D115 | 16.8 ns | 4.7 µs | 9.92 µs | 13.8 µs | 19.3 µs |
| D153 | 22.7 ns | 5.56 µs | 10.1 µs | 18.1 µs | 31.8 µs |
| D230 | 48.8 ns | 10.7 µs | 19.5 µs | 33.1 µs | 68.3 µs |
| D307 | 81.5 ns | 12 µs | 24 µs | 67.3 µs | 114 µs |
| D462 | 177 ns | 16.2 µs | 60.1 µs | 133 µs | 219 µs |
| D616 | 146 ns | 27.6 µs | 127 µs | 257 µs | 451 µs |
| D924 | 163 ns | 69.1 µs | 269 µs | 549 µs | 1.13 ms |
| D1232 | 415 ns | 129 µs | 450 µs | 1.04 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,188.6 124.4,200.3 160.5,195.9 196.7,175.0 232.9,171.2 269.1,161.8 305.3,155.4 341.5,145.8 377.6,148.2 413.8,146.8 450.0,135.2 450.0,27.5 413.8,37.1 377.6,48.4 341.5,57.4 305.3,65.5 269.1,71.9 232.9,81.4 196.7,87.6 160.5,97.4 124.4,97.4 88.2,99.2 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,188.6 124.4,200.3 160.5,195.9 196.7,175.0 232.9,171.2 269.1,161.8 305.3,155.4 341.5,145.8 377.6,148.2 413.8,146.8 450.0,135.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,109.4 124.4,109.3 160.5,107.2 196.7,105.1 232.9,103.0 269.1,94.8 305.3,93.4 341.5,89.8 377.6,83.1 413.8,71.7 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,100.6 124.4,104.7 160.5,103.9 196.7,95.8 232.9,95.5 269.1,87.4 305.3,84.9 341.5,73.5 377.6,64.2 413.8,54.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,98.6 124.4,102.8 160.5,100.2 196.7,91.8 232.9,88.4 269.1,80.9 305.3,72.1 341.5,63.6 377.6,55.4 413.8,46.0 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,99.2 124.4,97.4 160.5,97.4 196.7,87.6 232.9,81.4 269.1,71.9 305.3,65.5 341.5,57.4 377.6,48.4 413.8,37.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 7.34 µs | 9.42 µs | 11 µs | 11.9 µs |
| D38 | 4.57 ns | 5.45 µs | 11.5 µs | 13.4 µs | 12.8 µs |
| D57 | 2.49 ns | 5.25 µs | 7.44 µs | 8.36 µs | 10.1 µs |
| D76 | 3.43 ns | 6.09 µs | 7.42 µs | 10.1 µs | 11.8 µs |
| D115 | 10.9 ns | 12.7 µs | 13.1 µs | 20.9 µs | 28 µs |
| D153 | 22.7 ns | 8.07 µs | 15.9 µs | 22.4 µs | 39.2 µs |
| D230 | 48.8 ns | 14.3 µs | 24.6 µs | 39.9 µs | 78.6 µs |
| D307 | 82.2 ns | 16.2 µs | 46.1 µs | 79.3 µs | 123 µs |
| D462 | 182 ns | 24.5 µs | 82 µs | 154 µs | 231 µs |
| D616 | 146 ns | 33.4 µs | 141 µs | 268 µs | 445 µs |
| D924 | 184 ns | 82.5 µs | 288 µs | 542 µs | 996 µs |
| D1232 | 388 ns | 140 µs | 443 µs | 909 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,191.1 124.4,198.7 160.5,194.7 196.7,180.4 232.9,171.2 269.1,161.8 305.3,155.3 341.5,145.4 377.6,148.1 413.8,145.3 450.0,136.0 450.0,25.6 413.8,38.6 377.6,48.6 341.5,56.8 305.3,64.6 269.1,70.1 232.9,78.8 196.7,82.9 160.5,93.6 124.4,95.6 88.2,92.6 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,191.1 124.4,198.7 160.5,194.7 196.7,180.4 232.9,171.2 269.1,161.8 305.3,155.3 341.5,145.4 377.6,148.1 413.8,145.3 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.6 88.2,103.2 124.4,103.7 160.5,101.9 196.7,92.8 232.9,98.4 269.1,91.3 305.3,89.7 341.5,84.6 377.6,80.8 413.8,69.5 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.0 124.4,99.4 160.5,99.4 196.7,92.4 232.9,89.9 269.1,84.5 305.3,76.8 341.5,69.6 377.6,62.9 413.8,54.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,97.9 160.5,95.6 196.7,86.6 232.9,85.7 269.1,78.6 305.3,70.0 341.5,61.8 377.6,54.9 413.8,46.2 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,92.6 124.4,95.6 160.5,93.6 196.7,82.9 232.9,78.8 269.1,70.1 305.3,64.6 341.5,56.8 377.6,48.6 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 3.83 µs | 5.32 µs | 6.33 µs | 6.9 µs |
| D38 | 4.92 ns | 3.18 µs | 6.57 µs | 7.74 µs | 7.47 µs |
| D57 | 2.19 ns | 3.09 µs | 4.61 µs | 5.65 µs | 8.73 µs |
| D76 | 3.74 ns | 3.69 µs | 4.85 µs | 6.87 µs | 8.41 µs |
| D115 | 16.8 ns | 4.45 µs | 9.95 µs | 12.9 µs | 18.4 µs |
| D153 | 23.4 ns | 5.3 µs | 10.7 µs | 17.9 µs | 31.2 µs |
| D230 | 48.8 ns | 10.2 µs | 19.5 µs | 32.8 µs | 67.4 µs |
| D307 | 76.9 ns | 11.4 µs | 23.4 µs | 66.2 µs | 113 µs |
| D462 | 178 ns | 16.3 µs | 58.9 µs | 132 µs | 215 µs |
| D616 | 141 ns | 26.6 µs | 128 µs | 253 µs | 452 µs |
| D924 | 156 ns | 69 µs | 265 µs | 543 µs | 1.12 ms |
| D1232 | 417 ns | 126 µs | 451 µs | 1.03 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,190.2 124.4,200.3 160.5,193.6 196.7,175.0 232.9,170.9 269.1,161.8 305.3,156.1 341.5,145.7 377.6,148.6 413.8,147.4 450.0,135.1 450.0,27.6 413.8,37.1 377.6,48.4 341.5,57.7 305.3,65.6 269.1,72.0 232.9,81.6 196.7,88.1 160.5,97.9 124.4,97.4 88.2,99.3 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,190.2 124.4,200.3 160.5,193.6 196.7,175.0 232.9,170.9 269.1,161.8 305.3,156.1 341.5,145.7 377.6,148.6 413.8,147.4 450.0,135.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.6 88.2,109.9 124.4,110.3 160.5,108.1 196.7,105.8 232.9,103.6 269.1,95.5 305.3,94.0 341.5,89.6 377.6,83.6 413.8,71.7 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,100.9 124.4,105.3 160.5,104.7 196.7,95.8 232.9,94.9 269.1,87.4 305.3,85.2 341.5,73.7 377.6,64.1 413.8,55.0 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,98.9 124.4,102.8 160.5,100.4 196.7,92.5 232.9,88.5 269.1,81.0 305.3,72.3 341.5,63.7 377.6,55.6 413.8,46.2 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,99.3 124.4,97.4 160.5,97.9 196.7,88.1 232.9,81.6 269.1,72.0 305.3,65.6 341.5,57.7 377.6,48.4 413.8,37.1 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.34 µs | 9.43 µs | 11 µs | 12 µs |
| D38 | 4.57 ns | 5.46 µs | 11.5 µs | 13.4 µs | 12.8 µs |
| D57 | 10.6 ns | 5.21 µs | 7.46 µs | 8.38 µs | 10.1 µs |
| D76 | 10.5 ns | 6.08 µs | 7.43 µs | 10.1 µs | 11.8 µs |
| D115 | 11.2 ns | 12.6 µs | 12 µs | 20.8 µs | 27.9 µs |
| D153 | 23 ns | 8.12 µs | 15.8 µs | 22.5 µs | 38.6 µs |
| D230 | 49.6 ns | 14.3 µs | 24.1 µs | 40.3 µs | 78.8 µs |
| D307 | 76.6 ns | 16 µs | 45.6 µs | 79 µs | 123 µs |
| D462 | 175 ns | 24.6 µs | 81.9 µs | 155 µs | 231 µs |
| D616 | 140 ns | 33.5 µs | 141 µs | 269 µs | 446 µs |
| D924 | 173 ns | 82 µs | 289 µs | 543 µs | 995 µs |
| D1232 | 384 ns | 141 µs | 444 µs | 908 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,191.1 124.4,180.7 160.5,180.9 196.7,180.0 232.9,171.1 269.1,161.5 305.3,156.2 341.5,145.9 377.6,148.7 413.8,146.0 450.0,136.2 450.0,25.6 413.8,38.6 377.6,48.6 341.5,56.8 305.3,64.6 269.1,70.1 232.9,78.9 196.7,83.0 160.5,93.6 124.4,95.6 88.2,92.6 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,191.1 124.4,180.7 160.5,180.9 196.7,180.0 232.9,171.1 269.1,161.5 305.3,156.2 341.5,145.9 377.6,148.7 413.8,146.0 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,103.2 124.4,103.8 160.5,101.9 196.7,92.9 232.9,98.3 269.1,91.3 305.3,89.9 341.5,84.5 377.6,80.7 413.8,69.6 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,94.0 124.4,99.3 160.5,99.4 196.7,93.5 232.9,90.0 269.1,84.8 305.3,76.9 341.5,69.6 377.6,62.9 413.8,54.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,97.9 160.5,95.6 196.7,86.6 232.9,85.7 269.1,78.4 305.3,70.1 341.5,61.7 377.6,54.9 413.8,46.2 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,92.6 124.4,95.6 160.5,93.6 196.7,83.0 232.9,78.9 269.1,70.1 305.3,64.6 341.5,56.8 377.6,48.6 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 7.64 µs | 9.92 µs | 11.6 µs | 12.7 µs |
| D38 | 4.92 ns | 6.04 µs | 12.2 µs | 14.4 µs | 13.8 µs |
| D57 | 2.81 ns | 4.16 µs | 6.04 µs | 7.47 µs | 8.6 µs |
| D76 | 3.74 ns | 4.86 µs | 6.53 µs | 8.79 µs | 10.7 µs |
| D115 | 16.8 ns | 6.08 µs | 13.6 µs | 15.9 µs | 22.3 µs |
| D153 | 22.6 ns | 7.13 µs | 12.8 µs | 21 µs | 36.5 µs |
| D230 | 47.3 ns | 13 µs | 23.3 µs | 38 µs | 75.9 µs |
| D307 | 74.6 ns | 14.3 µs | 27.9 µs | 74.5 µs | 125 µs |
| D462 | 168 ns | 19.4 µs | 67.4 µs | 147 µs | 236 µs |
| D616 | 179 ns | 31.5 µs | 140 µs | 278 µs | 487 µs |
| D924 | 141 ns | 77.9 µs | 291 µs | 584 µs | 1.19 ms |
| D1232 | 396 ns | 138 µs | 487 µs | 1.1 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.2 124.4,197.2 160.5,193.6 196.7,175.0 232.9,171.3 269.1,162.1 305.3,156.5 341.5,146.4 377.6,145.6 413.8,148.6 450.0,135.8 450.0,26.9 413.8,36.4 377.6,47.5 341.5,56.5 305.3,64.3 269.1,70.6 232.9,79.7 196.7,85.7 160.5,94.9 124.4,97.6 88.2,91.8 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.2 124.4,197.2 160.5,193.6 196.7,175.0 232.9,171.3 269.1,162.1 305.3,156.5 341.5,146.4 377.6,145.6 413.8,148.6 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,102.0 124.4,106.6 160.5,104.7 196.7,101.9 232.9,99.9 269.1,92.4 305.3,91.3 341.5,87.5 377.6,81.5 413.8,70.2 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,93.2 124.4,102.0 160.5,101.0 196.7,91.9 232.9,92.7 269.1,85.2 305.3,83.0 341.5,72.0 377.6,63.0 413.8,53.9 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,91.2 124.4,99.3 160.5,97.3 196.7,89.9 232.9,86.5 269.1,79.1 305.3,70.8 341.5,62.4 377.6,54.4 413.8,45.3 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,91.8 124.4,97.6 160.5,94.9 196.7,85.7 232.9,79.7 269.1,70.6 305.3,64.3 341.5,56.5 377.6,47.5 413.8,36.4 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.36 µs | 9.56 µs | 11.2 µs | 12.1 µs |
| D38 | 4.22 ns | 5.61 µs | 11.5 µs | 13.4 µs | 12.8 µs |
| D57 | 2.63 µs | 5.3 µs | 7.75 µs | 8.73 µs | 10.5 µs |
| D76 | 2.67 µs | 6.26 µs | 7.73 µs | 10.4 µs | 12.2 µs |
| D115 | 5.98 µs | 13.5 µs | 12.7 µs | 21.7 µs | 29.1 µs |
| D153 | 2.86 µs | 8.37 µs | 16.4 µs | 23.1 µs | 39.3 µs |
| D230 | 2.94 µs | 14.9 µs | 24.8 µs | 40.8 µs | 79.6 µs |
| D307 | 3.19 µs | 16.5 µs | 46.7 µs | 80.4 µs | 125 µs |
| D462 | 3.67 µs | 25.4 µs | 83.2 µs | 156 µs | 233 µs |
| D616 | 3.52 µs | 34.5 µs | 142 µs | 271 µs | 448 µs |
| D924 | 3.7 µs | 83.4 µs | 293 µs | 548 µs | 1 ms |
| D1232 | 4.47 µs | 143 µs | 449 µs | 919 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,112.3 160.5,112.1 196.7,102.1 232.9,111.2 269.1,110.9 305.3,109.9 341.5,108.2 377.6,108.7 413.8,108.0 450.0,105.7 450.0,25.5 413.8,38.6 377.6,48.5 341.5,56.6 305.3,64.4 269.1,70.0 232.9,78.7 196.7,82.4 160.5,93.2 124.4,95.1 88.2,92.6 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,112.3 160.5,112.1 196.7,102.1 232.9,111.2 269.1,110.9 305.3,109.9 341.5,108.2 377.6,108.7 413.8,108.0 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,102.9 124.4,103.6 160.5,101.5 196.7,92.0 232.9,97.9 269.1,90.8 305.3,89.5 341.5,84.2 377.6,80.4 413.8,69.4 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,93.9 124.4,98.9 160.5,98.9 196.7,92.8 232.9,89.6 269.1,84.4 305.3,76.6 341.5,69.4 377.6,62.8 413.8,53.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,92.0 124.4,97.4 160.5,95.2 196.7,86.1 232.9,85.4 269.1,78.3 305.3,69.8 341.5,61.6 377.6,54.8 413.8,46.0 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,92.6 124.4,95.1 160.5,93.2 196.7,82.4 232.9,78.7 269.1,70.0 305.3,64.4 341.5,56.6 377.6,48.5 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 3 µs | 3.23 µs | 3.8 µs | 4.14 µs |
| D38 | 4.57 ns | 1.46 µs | 4 µs | 4.72 µs | 4.54 µs |
| D57 | 183 ns | 292 ns | 330 ns | 337 ns | 389 ns |
| D76 | 177 ns | 317 ns | 308 ns | 401 ns | 440 ns |
| D115 | 376 ns | 517 ns | 650 ns | 690 ns | 834 ns |
| D153 | 337 ns | 572 ns | 694 ns | 754 ns | 1.01 µs |
| D230 | 508 ns | 853 ns | 1.05 µs | 1.14 µs | 1.73 µs |
| D307 | 853 ns | 1.14 µs | 1.36 µs | 2.04 µs | 2.75 µs |
| D462 | 969 ns | 1.49 µs | 2.1 µs | 2.94 µs | 3.84 µs |
| D616 | 1.09 µs | 1.58 µs | 3.05 µs | 4.06 µs | 5.96 µs |
| D924 | 1.43 µs | 2.64 µs | 4.73 µs | 6.65 µs | 11.1 µs |
| D1232 | 2.21 µs | 4.07 µs | 7.24 µs | 10.8 µs | 30.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,183.6 124.4,119.5 160.5,120.0 196.7,107.0 232.9,108.9 269.1,101.8 305.3,92.8 341.5,90.5 377.6,88.5 413.8,83.7 450.0,76.3 450.0,30.7 413.8,48.1 377.6,59.0 341.5,66.6 305.3,72.4 269.1,80.5 232.9,89.8 196.7,93.1 160.5,104.3 124.4,106.4 88.2,63.7 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,183.6 124.4,119.5 160.5,120.0 196.7,107.0 232.9,108.9 269.1,101.8 305.3,92.8 341.5,90.5 377.6,88.5 413.8,83.7 450.0,76.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,70.9 88.2,83.4 124.4,111.4 160.5,109.9 196.7,101.5 232.9,99.7 269.1,92.8 305.3,87.7 341.5,83.0 377.6,82.1 413.8,73.2 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.7 88.2,65.9 124.4,109.3 160.5,110.4 196.7,97.5 232.9,96.4 269.1,89.1 305.3,84.7 341.5,77.1 377.6,70.6 413.8,63.0 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,63.1 124.4,108.9 160.5,105.9 196.7,96.5 232.9,94.9 269.1,87.7 305.3,77.6 341.5,71.2 377.6,65.7 413.8,57.1 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,63.7 124.4,106.4 160.5,104.3 196.7,93.1 232.9,89.8 269.1,80.5 305.3,72.4 341.5,66.6 377.6,59.0 413.8,48.1 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 142 ns | 183 ns | 204 ns | 205 ns |
| D38 | 4.92 ns | 171 ns | 187 ns | 187 ns | 160 ns |
| D57 | 275 ns | 401 ns | 433 ns | 454 ns | 502 ns |
| D76 | 271 ns | 413 ns | 418 ns | 515 ns | 547 ns |
| D115 | 620 ns | 723 ns | 886 ns | 924 ns | 1.04 µs |
| D153 | 596 ns | 760 ns | 905 ns | 977 ns | 1.25 µs |
| D230 | 885 ns | 1.24 µs | 1.38 µs | 1.46 µs | 2.12 µs |
| D307 | 1.38 µs | 1.65 µs | 1.86 µs | 2.6 µs | 3.36 µs |
| D462 | 1.52 µs | 2 µs | 2.73 µs | 3.58 µs | 4.47 µs |
| D616 | 1.73 µs | 1.98 µs | 3.79 µs | 4.82 µs | 6.8 µs |
| D924 | 2.36 µs | 3.53 µs | 5.73 µs | 7.48 µs | 12.3 µs |
| D1232 | 3.37 µs | 5.29 µs | 8.46 µs | 12.2 µs | 31.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,182.3 124.4,112.4 160.5,112.7 196.7,98.3 232.9,99.0 269.1,92.1 305.3,84.4 341.5,82.8 377.6,80.5 413.8,75.1 450.0,68.9 450.0,29.8 413.8,46.5 377.6,56.7 341.5,64.0 305.3,68.9 269.1,76.9 232.9,86.1 196.7,89.4 160.5,100.5 124.4,102.0 88.2,121.9 52.0,117.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,182.3 124.4,112.4 160.5,112.7 196.7,98.3 232.9,99.0 269.1,92.1 305.3,84.4 341.5,82.8 377.6,80.5 413.8,75.1 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.9 88.2,120.7 124.4,105.9 160.5,105.3 196.7,95.6 232.9,94.8 269.1,86.2 305.3,81.2 341.5,77.9 377.6,78.1 413.8,68.1 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.5 88.2,119.2 124.4,104.6 160.5,105.2 196.7,92.1 232.9,91.7 269.1,84.4 305.3,79.2 341.5,72.6 377.6,66.8 413.8,59.7 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,119.2 124.4,103.7 160.5,101.5 196.7,91.4 232.9,90.4 269.1,83.5 305.3,73.4 341.5,67.8 377.6,62.7 413.8,55.0 450.0,46.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,121.9 124.4,102.0 160.5,100.5 196.7,89.4 232.9,86.1 269.1,76.9 305.3,68.9 341.5,64.0 377.6,56.7 413.8,46.5 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
