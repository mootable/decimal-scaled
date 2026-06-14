# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.26 ns | 23.3 µs | 27.7 µs | 30.3 µs | 36.8 µs |
| D38 | 4.63 µs | 7.62 µs | 8.29 µs | 12.4 µs | 15 µs |
| D57 | 4.36 µs | 8.03 µs | 10.9 µs | 16.4 µs | 17.5 µs |
| D76 | 5.18 µs | 9.43 µs | 16 µs | 16 µs | 22 µs |
| D115 | 4.97 µs | 11 µs | 14.4 µs | 33.7 µs | 36.5 µs |
| D153 | 5.17 µs | 16.2 µs | 28.6 µs | 43.7 µs | 60.4 µs |
| D230 | 4.77 µs | 25.9 µs | 46.7 µs | 65.1 µs | 105 µs |
| D307 | 4.76 µs | 27.4 µs | 63.8 µs | 106 µs | 188 µs |
| D462 | 4.86 µs | 41.4 µs | 107 µs | 238 µs | 347 µs |
| D616 | 4.93 µs | 64.1 µs | 203 µs | 444 µs | 657 µs |
| D924 | 5.47 µs | 132 µs | 421 µs | 937 µs | 1.52 ms |
| D1232 | 4.68 µs | 215 µs | 699 µs | 1.63 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,105.3 124.4,106.0 160.5,103.9 196.7,104.4 232.9,103.9 269.1,104.9 305.3,104.9 341.5,104.7 377.6,104.5 413.8,103.2 450.0,105.1 450.0,23.1 413.8,33.4 377.6,43.8 341.5,51.7 305.3,59.3 269.1,66.5 232.9,73.4 196.7,79.6 160.5,85.9 124.4,88.8 88.2,90.7 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,105.3 124.4,106.0 160.5,103.9 196.7,104.4 232.9,103.9 269.1,104.9 305.3,104.9 341.5,104.7 377.6,104.5 413.8,103.2 450.0,105.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.1 124.4,98.4 160.5,96.4 196.7,94.6 232.9,89.7 269.1,83.9 305.3,83.2 341.5,78.1 377.6,72.7 413.8,63.7 450.0,57.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,98.0 124.4,94.6 160.5,89.9 196.7,91.2 232.9,82.7 269.1,76.6 305.3,72.7 341.5,66.3 377.6,58.4 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.0 124.4,89.6 160.5,89.9 196.7,80.6 232.9,77.4 269.1,72.5 305.3,66.4 341.5,56.4 377.6,48.6 413.8,39.4 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,90.7 124.4,88.8 160.5,85.9 196.7,79.6 232.9,73.4 269.1,66.5 305.3,59.3 341.5,51.7 377.6,43.8 413.8,33.4 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 31.1 µs | 33.2 µs | 36.4 µs | 35.9 µs |
| D38 | 27.4 µs | 35.5 µs | 44.2 µs | 57.3 µs | 61.5 µs |
| D57 | 3.03 µs | 3.95 µs | 4.96 µs | 6.66 µs | 7.44 µs |
| D76 | 3.69 µs | 5.07 µs | 6.31 µs | 7.07 µs | 10.1 µs |
| D115 | 6.08 µs | 8.68 µs | 7.37 µs | 17.9 µs | 19 µs |
| D153 | 6.62 µs | 10.7 µs | 16.1 µs | 24.3 µs | 30.3 µs |
| D230 | 8.37 µs | 17.1 µs | 28.8 µs | 38.5 µs | 60.4 µs |
| D307 | 12.3 µs | 26.2 µs | 50.5 µs | 70.3 µs | 138 µs |
| D462 | 12.2 µs | 36.9 µs | 71.1 µs | 161 µs | 247 µs |
| D616 | 21.1 µs | 76.3 µs | 170 µs | 333 µs | 520 µs |
| D924 | 33.8 µs | 161 µs | 380 µs | 804 µs | 1.34 ms |
| D1232 | 35.6 µs | 273 µs | 714 µs | 1.53 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,138.1 124.4,185.9 160.5,181.7 196.7,170.8 232.9,169.0 269.1,163.9 305.3,155.5 341.5,155.7 377.6,143.8 413.8,133.6 450.0,132.5 450.0,35.7 413.8,53.7 377.6,74.2 341.5,90.3 305.3,103.0 269.1,121.0 232.9,136.0 196.7,146.1 160.5,159.7 124.4,166.4 88.2,120.6 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,138.1 124.4,185.9 160.5,181.7 196.7,170.8 232.9,169.0 269.1,163.9 305.3,155.5 341.5,155.7 377.6,143.8 413.8,133.6 450.0,132.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,132.5 124.4,180.2 160.5,174.7 196.7,163.1 232.9,158.5 269.1,148.4 305.3,139.1 341.5,131.6 377.6,115.9 413.8,99.7 450.0,88.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,127.7 124.4,175.2 160.5,170.0 196.7,166.6 232.9,149.7 269.1,137.0 305.3,124.8 341.5,117.4 377.6,98.5 413.8,81.0 450.0,67.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,122.1 124.4,168.8 160.5,167.5 196.7,147.3 232.9,140.7 269.1,130.7 305.3,117.7 341.5,99.7 377.6,83.9 413.8,64.7 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,120.6 124.4,166.4 160.5,159.7 196.7,146.1 232.9,136.0 269.1,121.0 305.3,103.0 341.5,90.3 377.6,74.2 413.8,53.7 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 23.3 µs | 27.7 µs | 30.3 µs | 36.8 µs |
| D38 | 4.6 µs | 7.66 µs | 8.2 µs | 12.4 µs | 15 µs |
| D57 | 4.31 µs | 8.02 µs | 10.9 µs | 16.3 µs | 17.4 µs |
| D76 | 5.16 µs | 9.46 µs | 16 µs | 16 µs | 22 µs |
| D115 | 4.88 µs | 10.9 µs | 14.7 µs | 33.3 µs | 36.4 µs |
| D153 | 5.2 µs | 16.1 µs | 28.8 µs | 43.6 µs | 60.1 µs |
| D230 | 4.73 µs | 25.3 µs | 43.5 µs | 65 µs | 105 µs |
| D307 | 4.69 µs | 26.8 µs | 63.6 µs | 107 µs | 189 µs |
| D462 | 4.79 µs | 40.9 µs | 105 µs | 236 µs | 346 µs |
| D616 | 4.84 µs | 64.3 µs | 203 µs | 446 µs | 659 µs |
| D924 | 5.43 µs | 131 µs | 421 µs | 934 µs | 1.52 ms |
| D1232 | 4.7 µs | 212 µs | 701 µs | 1.64 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,105.3 124.4,106.1 160.5,103.9 196.7,104.6 232.9,103.8 269.1,105.0 305.3,105.1 341.5,104.8 377.6,104.7 413.8,103.3 450.0,105.1 450.0,23.1 413.8,33.4 377.6,43.7 341.5,51.7 305.3,59.2 269.1,66.5 232.9,73.5 196.7,79.7 160.5,85.9 124.4,88.8 88.2,90.7 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,105.3 124.4,106.1 160.5,103.9 196.7,104.6 232.9,103.8 269.1,105.0 305.3,105.1 341.5,104.8 377.6,104.7 413.8,103.3 450.0,105.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.0 124.4,98.4 160.5,96.4 196.7,94.6 232.9,89.8 269.1,84.2 305.3,83.5 341.5,78.3 377.6,72.6 413.8,63.8 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,98.2 124.4,94.6 160.5,89.9 196.7,90.9 232.9,82.6 269.1,77.5 305.3,72.7 341.5,66.6 377.6,58.3 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.0 124.4,89.7 160.5,89.9 196.7,80.8 232.9,77.4 269.1,72.5 305.3,66.4 341.5,56.5 377.6,48.6 413.8,39.4 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,90.7 124.4,88.8 160.5,85.9 196.7,79.7 232.9,73.5 269.1,66.5 305.3,59.2 341.5,51.7 377.6,43.7 413.8,33.4 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 23 µs | 27.1 µs | 29.7 µs | 36.1 µs |
| D38 | 4.22 ns | 29 µs | 36.2 µs | 42.9 µs | 51.3 µs |
| D57 | 1.51 ns | 4.97 µs | 7.28 µs | 9.74 µs | 11.6 µs |
| D76 | 2.22 ns | 6.93 µs | 9.4 µs | 10.6 µs | 15.2 µs |
| D115 | 13.4 ns | 12.5 µs | 11.6 µs | 26.3 µs | 29.8 µs |
| D153 | 15.9 ns | 16.6 µs | 23.4 µs | 36.5 µs | 43.8 µs |
| D230 | 28 ns | 26 µs | 45.3 µs | 57 µs | 83.5 µs |
| D307 | 44.8 ns | 37.3 µs | 71.6 µs | 95.4 µs | 173 µs |
| D462 | 77.3 ns | 54.6 µs | 98.7 µs | 197 µs | 289 µs |
| D616 | 79.4 ns | 112 µs | 229 µs | 433 µs | 619 µs |
| D924 | 115 ns | 254 µs | 499 µs | 944 µs | 1.47 ms |
| D1232 | 125 ns | 383 µs | 916 µs | 1.75 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,204.9 160.5,200.1 196.7,177.8 232.9,175.7 269.1,168.6 305.3,162.8 341.5,156.0 377.6,155.7 413.8,151.1 450.0,150.1 450.0,24.5 413.8,33.8 377.6,44.5 341.5,54.0 305.3,60.3 269.1,69.4 232.9,77.4 196.7,82.2 160.5,90.5 124.4,93.9 88.2,75.4 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,204.9 160.5,200.1 196.7,177.8 232.9,175.7 269.1,168.6 305.3,162.8 341.5,156.0 377.6,155.7 413.8,151.1 450.0,150.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,82.5 124.4,104.4 160.5,100.3 196.7,92.9 232.9,89.4 269.1,83.9 305.3,79.4 341.5,74.7 377.6,65.8 413.8,55.6 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,79.8 124.4,99.7 160.5,96.5 196.7,93.9 232.9,85.1 269.1,77.0 305.3,71.3 341.5,67.3 377.6,56.8 413.8,47.2 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,77.6 124.4,96.0 160.5,95.0 196.7,83.7 232.9,79.6 269.1,74.1 305.3,67.7 341.5,58.7 377.6,49.0 413.8,39.3 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,75.4 124.4,93.9 160.5,90.5 196.7,82.2 232.9,77.4 269.1,69.4 305.3,60.3 341.5,54.0 377.6,44.5 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.63 µs | 2.57 µs | 2.81 µs | 3.07 µs |
| D38 | 6.23 µs | 8.96 µs | 6.89 µs | 10.7 µs | 12.9 µs |
| D57 | 3.52 µs | 6.88 µs | 9.47 µs | 14.2 µs | 5.06 µs |
| D76 | 4.24 µs | 8.12 µs | 14 µs | 14.1 µs | 19.5 µs |
| D115 | 4.12 µs | 9.49 µs | 12.8 µs | 31.1 µs | 33.1 µs |
| D153 | 4.26 µs | 14.1 µs | 22.2 µs | 39.4 µs | 55.7 µs |
| D230 | 3.94 µs | 22.4 µs | 40.2 µs | 59.7 µs | 97.7 µs |
| D307 | 3.93 µs | 24 µs | 52.5 µs | 97.3 µs | 178 µs |
| D462 | 3.02 µs | 35 µs | 90.1 µs | 207 µs | 296 µs |
| D616 | 4.18 µs | 59.4 µs | 188 µs | 418 µs | 622 µs |
| D924 | 4.55 µs | 121 µs | 391 µs | 891 µs | 1.43 ms |
| D1232 | 3.8 µs | 196 µs | 661 µs | 1.58 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,101.6 124.4,108.7 160.5,106.4 196.7,106.7 232.9,106.3 269.1,107.3 305.3,107.3 341.5,110.6 377.6,106.5 413.8,105.5 450.0,107.7 450.0,23.6 413.8,34.1 377.6,44.5 341.5,53.7 305.3,60.0 269.1,67.4 232.9,74.4 196.7,80.9 160.5,87.4 124.4,104.2 88.2,92.5 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,101.6 124.4,108.7 160.5,106.4 196.7,106.7 232.9,106.3 269.1,107.3 305.3,107.3 341.5,110.6 377.6,106.5 413.8,105.5 450.0,107.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,97.1 124.4,100.4 160.5,98.3 196.7,96.4 232.9,91.5 269.1,85.7 305.3,84.9 341.5,80.2 377.6,73.6 413.8,64.8 450.0,58.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,100.3 124.4,96.4 160.5,91.5 196.7,92.7 232.9,85.8 269.1,78.4 305.3,75.1 341.5,68.4 377.6,59.3 413.8,50.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,94.8 124.4,91.3 160.5,91.5 196.7,81.7 232.9,78.7 269.1,73.6 305.3,67.5 341.5,58.1 377.6,49.4 413.8,40.0 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,92.5 124.4,104.2 160.5,87.4 196.7,80.9 232.9,74.4 269.1,67.4 305.3,60.0 341.5,53.7 377.6,44.5 413.8,34.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.81 µs | 8.83 µs | 9.69 µs | 10.6 µs |
| D38 | 4.22 ns | 9.45 µs | 11.8 µs | 13.9 µs | 15 µs |
| D57 | 403 ns | 5.09 µs | 6.92 µs | 9.09 µs | 10.9 µs |
| D76 | 603 ns | 6.53 µs | 8.56 µs | 10.3 µs | 14.9 µs |
| D115 | 1.04 µs | 12.1 µs | 10.3 µs | 27.2 µs | 29.4 µs |
| D153 | 1.19 µs | 14.8 µs | 23.8 µs | 37.3 µs | 48.3 µs |
| D230 | 1.4 µs | 24.9 µs | 44.4 µs | 63.9 µs | 102 µs |
| D307 | 2.17 µs | 39.8 µs | 82.3 µs | 120 µs | 246 µs |
| D462 | 2.2 µs | 57.1 µs | 122 µs | 287 µs | 450 µs |
| D616 | 3.74 µs | 121 µs | 296 µs | 590 µs | 948 µs |
| D924 | 5.74 µs | 269 µs | 676 µs | 1.47 ms | 2.5 ms |
| D1232 | 6.16 µs | 469 µs | 1.28 ms | 2.84 ms | 5.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,135.6 160.5,130.6 196.7,123.8 232.9,122.1 269.1,120.1 305.3,114.6 341.5,114.5 377.6,107.9 413.8,102.6 450.0,101.7 450.0,16.8 413.8,27.2 377.6,39.2 341.5,48.5 305.3,56.0 269.1,66.9 232.9,76.2 196.7,82.3 160.5,90.7 124.4,94.6 88.2,90.7 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,135.6 160.5,130.6 196.7,123.8 232.9,122.1 269.1,120.1 305.3,114.6 341.5,114.5 377.6,107.9 413.8,102.6 450.0,101.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,96.4 124.4,104.1 160.5,101.0 196.7,93.3 232.9,90.8 269.1,84.4 305.3,78.6 341.5,74.1 377.6,64.8 413.8,54.8 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,93.7 124.4,100.3 160.5,97.6 196.7,95.3 232.9,85.0 269.1,77.2 305.3,69.6 341.5,64.6 377.6,53.7 413.8,43.4 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,91.6 124.4,96.9 160.5,95.4 196.7,83.3 232.9,79.4 269.1,72.7 305.3,64.9 341.5,54.1 377.6,45.1 413.8,33.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,90.7 124.4,94.6 160.5,90.7 196.7,82.3 232.9,76.2 269.1,66.9 305.3,56.0 341.5,48.5 377.6,39.2 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 4.27 µs | 5.5 µs | 5.91 µs | 6.44 µs |
| D38 | 5.62 ns | 5.96 µs | 7.32 µs | 8.59 µs | 8.94 µs |
| D57 | 2.3 ns | 3.34 µs | 4.36 µs | 5.7 µs | 8.73 µs |
| D76 | 3.14 ns | 3.96 µs | 5.49 µs | 6.4 µs | 9.38 µs |
| D115 | 17.5 ns | 4.34 µs | 6.11 µs | 14.4 µs | 16.7 µs |
| D153 | 22.4 ns | 5.57 µs | 9.87 µs | 19.3 µs | 29.6 µs |
| D230 | 48.8 ns | 10.2 µs | 19.6 µs | 33.4 µs | 58.5 µs |
| D307 | 81.6 ns | 12.2 µs | 26.9 µs | 58.5 µs | 114 µs |
| D462 | 130 ns | 15.4 µs | 53.7 µs | 134 µs | 206 µs |
| D616 | 145 ns | 32.5 µs | 121 µs | 273 µs | 429 µs |
| D924 | 194 ns | 73.4 µs | 253 µs | 619 µs | 1.03 ms |
| D1232 | 260 ns | 126 µs | 455 µs | 1.12 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,199.6 160.5,195.8 196.7,174.5 232.9,171.4 269.1,161.8 305.3,155.4 341.5,149.6 377.6,148.2 413.8,144.6 450.0,141.0 450.0,27.5 413.8,38.1 377.6,49.1 341.5,58.2 305.3,65.5 269.1,73.8 232.9,82.3 196.7,89.3 160.5,96.5 124.4,97.4 88.2,97.1 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,199.6 160.5,195.8 196.7,174.5 232.9,171.4 269.1,161.8 305.3,155.4 341.5,149.6 377.6,148.2 413.8,144.6 450.0,141.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,102.1 124.4,109.3 160.5,107.2 196.7,106.1 232.9,103.0 269.1,95.5 305.3,93.3 341.5,90.4 377.6,81.1 413.8,71.0 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,99.6 124.4,106.0 160.5,103.2 196.7,101.8 232.9,95.9 269.1,87.4 305.3,83.4 341.5,74.9 377.6,64.7 413.8,55.6 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.2 88.2,97.6 124.4,102.7 160.5,101.3 196.7,91.2 232.9,87.6 269.1,80.7 305.3,73.8 341.5,63.5 377.6,54.7 413.8,44.5 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.1 124.4,97.4 160.5,96.5 196.7,89.3 232.9,82.3 269.1,73.8 305.3,65.5 341.5,58.2 377.6,49.1 413.8,38.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.91 µs | 9.42 µs | 10.2 µs | 11.1 µs |
| D38 | 4.57 ns | 10.2 µs | 12.5 µs | 14.5 µs | 15.3 µs |
| D57 | 2.27 ns | 4.89 µs | 7.02 µs | 8.43 µs | 10.1 µs |
| D76 | 3.17 ns | 6.12 µs | 8.03 µs | 9.13 µs | 12.8 µs |
| D115 | 9.98 ns | 12 µs | 7.07 µs | 22.9 µs | 22.5 µs |
| D153 | 21.6 ns | 8.05 µs | 15.7 µs | 24 µs | 35.1 µs |
| D230 | 48.8 ns | 14.6 µs | 24.1 µs | 39.9 µs | 66.8 µs |
| D307 | 82.5 ns | 16.1 µs | 52 µs | 67 µs | 124 µs |
| D462 | 131 ns | 22.9 µs | 69.6 µs | 154 µs | 220 µs |
| D616 | 144 ns | 39.7 µs | 134 µs | 287 µs | 414 µs |
| D924 | 194 ns | 86.5 µs | 268 µs | 607 µs | 914 µs |
| D1232 | 264 ns | 141 µs | 444 µs | 992 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,199.8 160.5,195.7 196.7,181.5 232.9,171.9 269.1,161.8 305.3,155.2 341.5,149.5 377.6,148.3 413.8,144.6 450.0,140.8 450.0,25.6 413.8,39.7 377.6,49.5 341.5,57.3 305.3,64.4 269.1,72.2 232.9,80.1 196.7,85.6 160.5,92.7 124.4,95.6 88.2,90.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,199.8 160.5,195.7 196.7,181.5 232.9,171.9 269.1,161.8 305.3,155.2 341.5,149.5 377.6,148.3 413.8,144.6 450.0,140.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,104.6 160.5,101.8 196.7,93.5 232.9,98.4 269.1,91.0 305.3,89.8 341.5,85.5 377.6,78.6 413.8,68.9 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,93.0 124.4,100.1 160.5,98.4 196.7,100.0 232.9,90.1 269.1,84.8 305.3,75.3 341.5,71.6 377.6,63.6 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,91.1 124.4,97.8 160.5,96.8 196.7,85.4 232.9,84.8 269.1,78.5 305.3,72.1 341.5,61.8 377.6,54.1 413.8,44.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,90.5 124.4,95.6 160.5,92.7 196.7,85.6 232.9,80.1 269.1,72.2 305.3,64.4 341.5,57.3 377.6,49.5 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 4.12 µs | 5.31 µs | 5.85 µs | 6.36 µs |
| D38 | 4.92 ns | 5.77 µs | 7.12 µs | 8.38 µs | 8.88 µs |
| D57 | 2.21 ns | 2.97 µs | 4.05 µs | 5.61 µs | 8.71 µs |
| D76 | 3.87 ns | 3.71 µs | 5.22 µs | 6.3 µs | 9.01 µs |
| D115 | 17.5 ns | 4.07 µs | 6.12 µs | 14.1 µs | 16.2 µs |
| D153 | 22.5 ns | 5.32 µs | 9.94 µs | 18.7 µs | 28.7 µs |
| D230 | 48.8 ns | 10.4 µs | 19.5 µs | 33 µs | 57 µs |
| D307 | 77 ns | 11.6 µs | 26.2 µs | 56.6 µs | 114 µs |
| D462 | 125 ns | 14.8 µs | 51.7 µs | 134 µs | 202 µs |
| D616 | 139 ns | 32.1 µs | 119 µs | 267 µs | 426 µs |
| D924 | 184 ns | 73.1 µs | 250 µs | 613 µs | 1.03 ms |
| D1232 | 255 ns | 123 µs | 449 µs | 1.13 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,200.1 160.5,193.2 196.7,174.5 232.9,171.4 269.1,161.8 305.3,156.1 341.5,150.1 377.6,148.8 413.8,145.3 450.0,141.2 450.0,27.6 413.8,38.2 377.6,49.2 341.5,58.4 305.3,65.5 269.1,74.1 232.9,82.6 196.7,89.7 160.5,97.0 124.4,97.4 88.2,97.2 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,200.1 160.5,193.2 196.7,174.5 232.9,171.4 269.1,161.8 305.3,156.1 341.5,150.1 377.6,148.8 413.8,145.3 450.0,141.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,102.5 124.4,110.8 160.5,108.0 196.7,106.9 232.9,103.5 269.1,95.3 305.3,93.9 341.5,90.9 377.6,81.2 413.8,71.0 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,99.9 124.4,106.9 160.5,103.8 196.7,101.8 232.9,95.8 269.1,87.4 305.3,83.8 341.5,75.3 377.6,65.0 413.8,55.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,97.9 124.4,102.9 160.5,101.5 196.7,91.5 232.9,87.9 269.1,80.9 305.3,74.2 341.5,63.5 377.6,54.9 413.8,44.6 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,97.2 124.4,97.4 160.5,97.0 196.7,89.7 232.9,82.6 269.1,74.1 305.3,65.5 341.5,58.4 377.6,49.2 413.8,38.2 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.92 µs | 9.42 µs | 10.2 µs | 11.1 µs |
| D38 | 4.57 ns | 10.2 µs | 12.5 µs | 14.5 µs | 15.2 µs |
| D57 | 8.37 ns | 4.9 µs | 7.02 µs | 8.43 µs | 10.1 µs |
| D76 | 12.1 ns | 6.14 µs | 8.06 µs | 9.12 µs | 12.9 µs |
| D115 | 10.3 ns | 11.7 µs | 6.72 µs | 22.8 µs | 22.4 µs |
| D153 | 20.8 ns | 8.14 µs | 15.6 µs | 24 µs | 35.2 µs |
| D230 | 49.7 ns | 14.5 µs | 24.3 µs | 39.8 µs | 66.6 µs |
| D307 | 76.7 ns | 16.6 µs | 51 µs | 68.6 µs | 125 µs |
| D462 | 124 ns | 23.2 µs | 68.8 µs | 154 µs | 221 µs |
| D616 | 140 ns | 39.7 µs | 134 µs | 288 µs | 414 µs |
| D924 | 241 ns | 86.7 µs | 269 µs | 605 µs | 914 µs |
| D1232 | 247 ns | 141 µs | 446 µs | 995 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,183.6 160.5,179.1 196.7,181.1 232.9,172.4 269.1,161.5 305.3,156.2 341.5,150.1 377.6,148.7 413.8,141.9 450.0,141.6 450.0,25.6 413.8,39.7 377.6,49.5 341.5,57.3 305.3,64.3 269.1,72.2 232.9,80.1 196.7,85.7 160.5,92.6 124.4,95.6 88.2,90.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,183.6 160.5,179.1 196.7,181.1 232.9,172.4 269.1,161.5 305.3,156.2 341.5,150.1 377.6,148.7 413.8,141.9 450.0,141.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,104.6 160.5,101.8 196.7,93.8 232.9,98.3 269.1,91.1 305.3,89.4 341.5,85.3 377.6,78.6 413.8,68.9 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,93.0 124.4,100.1 160.5,98.4 196.7,100.7 232.9,90.2 269.1,84.7 305.3,75.5 341.5,71.8 377.6,63.5 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,91.1 124.4,97.8 160.5,96.9 196.7,85.5 232.9,84.8 269.1,78.6 305.3,71.8 341.5,61.8 377.6,54.0 413.8,44.8 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,90.5 124.4,95.6 160.5,92.6 196.7,85.7 232.9,80.1 269.1,72.2 305.3,64.3 341.5,57.3 377.6,49.5 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 8.23 µs | 9.92 µs | 10.8 µs | 11.7 µs |
| D38 | 4.92 ns | 10.7 µs | 13.2 µs | 15.6 µs | 16.4 µs |
| D57 | 2.87 ns | 3.97 µs | 5.48 µs | 7.43 µs | 8.61 µs |
| D76 | 4.22 ns | 4.84 µs | 7.04 µs | 8 µs | 11.3 µs |
| D115 | 17.5 ns | 5.55 µs | 7.6 µs | 17.2 µs | 19.4 µs |
| D153 | 22.3 ns | 7.14 µs | 12.6 µs | 22.3 µs | 33.9 µs |
| D230 | 47.3 ns | 13.3 µs | 23.4 µs | 37.9 µs | 66 µs |
| D307 | 74.6 ns | 14.1 µs | 30.8 µs | 65.1 µs | 125 µs |
| D462 | 124 ns | 18.1 µs | 59 µs | 147 µs | 219 µs |
| D616 | 135 ns | 37.6 µs | 133 µs | 291 µs | 459 µs |
| D924 | 160 ns | 82.5 µs | 274 µs | 656 µs | 1.1 ms |
| D1232 | 243 ns | 138 µs | 487 µs | 1.19 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,196.9 160.5,192.1 196.7,174.5 232.9,171.5 269.1,162.2 305.3,156.5 341.5,150.1 377.6,149.2 413.8,147.0 450.0,141.8 450.0,26.9 413.8,37.4 377.6,48.2 341.5,57.4 305.3,64.4 269.1,72.3 232.9,80.6 196.7,87.5 160.5,94.2 124.4,97.6 88.2,89.6 52.0,93.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,196.9 160.5,192.1 196.7,174.5 232.9,171.5 269.1,162.2 305.3,156.5 341.5,150.1 377.6,149.2 413.8,147.0 450.0,141.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.1 88.2,94.8 124.4,107.2 160.5,104.7 196.7,103.0 232.9,99.9 269.1,92.2 305.3,91.4 341.5,88.4 377.6,79.3 413.8,69.5 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,92.2 124.4,103.2 160.5,100.1 196.7,99.1 232.9,92.9 269.1,85.2 305.3,81.8 341.5,73.7 377.6,63.6 413.8,54.6 450.0,47.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,90.2 124.4,99.4 160.5,98.5 196.7,89.0 232.9,85.7 269.1,79.2 305.3,72.5 341.5,62.4 377.6,53.9 413.8,43.8 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,89.6 124.4,97.6 160.5,94.2 196.7,87.5 232.9,80.6 269.1,72.3 305.3,64.4 341.5,57.4 377.6,48.2 413.8,37.4 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.93 µs | 9.56 µs | 10.4 µs | 11.2 µs |
| D38 | 4.22 ns | 10.3 µs | 12.5 µs | 14.5 µs | 15.3 µs |
| D57 | 2.39 µs | 5.11 µs | 7.28 µs | 8.74 µs | 10.4 µs |
| D76 | 2.88 µs | 6.24 µs | 8.28 µs | 9.46 µs | 13.1 µs |
| D115 | 5.4 µs | 12.1 µs | 7.17 µs | 23.3 µs | 23.1 µs |
| D153 | 3.1 µs | 8.43 µs | 16.6 µs | 24.6 µs | 36.2 µs |
| D230 | 2.92 µs | 14.8 µs | 25.1 µs | 41.1 µs | 67.8 µs |
| D307 | 3.13 µs | 16.4 µs | 52.5 µs | 67.9 µs | 125 µs |
| D462 | 3.35 µs | 23.6 µs | 70.4 µs | 157 µs | 222 µs |
| D616 | 3.51 µs | 40.3 µs | 134 µs | 293 µs | 418 µs |
| D924 | 4.07 µs | 87.5 µs | 270 µs | 612 µs | 918 µs |
| D1232 | 3.8 µs | 143 µs | 451 µs | 1e+03 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,113.5 160.5,111.2 196.7,103.4 232.9,110.2 269.1,111.0 305.3,110.1 341.5,109.3 377.6,108.7 413.8,106.9 450.0,107.7 450.0,25.5 413.8,39.6 377.6,49.4 341.5,57.2 305.3,64.3 269.1,72.0 232.9,79.7 196.7,85.3 160.5,92.3 124.4,95.2 88.2,90.5 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,113.5 160.5,111.2 196.7,103.4 232.9,110.2 269.1,111.0 305.3,110.1 341.5,109.3 377.6,108.7 413.8,106.9 450.0,107.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.3 124.4,104.0 160.5,101.6 196.7,93.3 232.9,97.8 269.1,90.9 305.3,89.5 341.5,85.1 377.6,78.4 413.8,68.8 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,93.0 124.4,99.6 160.5,98.1 196.7,99.8 232.9,89.4 269.1,84.3 305.3,75.1 341.5,71.5 377.6,63.5 413.8,54.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.1 124.4,97.4 160.5,96.4 196.7,85.2 232.9,84.6 269.1,78.2 305.3,72.0 341.5,61.6 377.6,53.8 413.8,44.7 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,90.5 124.4,95.2 160.5,92.3 196.7,85.3 232.9,79.7 269.1,72.0 305.3,64.3 341.5,57.2 377.6,49.4 413.8,39.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.97 ns | 3.21 µs | 3.23 µs | 3.53 µs | 3.85 µs |
| D38 | 4.57 ns | 3.47 µs | 4.3 µs | 5.08 µs | 5.43 µs |
| D57 | 143 ns | 264 ns | 300 ns | 340 ns | 389 ns |
| D76 | 202 ns | 314 ns | 340 ns | 364 ns | 467 ns |
| D115 | 377 ns | 471 ns | 348 ns | 762 ns | 685 ns |
| D153 | 394 ns | 570 ns | 692 ns | 852 ns | 901 ns |
| D230 | 498 ns | 853 ns | 1.03 µs | 1.14 µs | 1.52 µs |
| D307 | 843 ns | 1.14 µs | 1.6 µs | 1.68 µs | 2.74 µs |
| D462 | 875 ns | 1.31 µs | 1.8 µs | 2.94 µs | 3.76 µs |
| D616 | 1.1 µs | 1.93 µs | 2.85 µs | 4.31 µs | 5.56 µs |
| D924 | 1.63 µs | 2.85 µs | 4.31 µs | 7.46 µs | 10.2 µs |
| D1232 | 1.66 µs | 4.08 µs | 7.2 µs | 11.7 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.0 88.2,183.6 124.4,123.8 160.5,117.8 196.7,107.0 232.9,106.2 269.1,102.1 305.3,93.0 341.5,92.3 377.6,88.4 413.8,81.6 450.0,81.2 450.0,30.7 413.8,49.6 377.6,60.2 341.5,67.0 305.3,72.5 269.1,82.7 232.9,91.8 196.7,96.6 160.5,103.2 124.4,106.4 88.2,60.6 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.0 88.2,183.6 124.4,123.8 160.5,117.8 196.7,107.0 232.9,106.2 269.1,102.1 305.3,93.0 341.5,92.3 377.6,88.4 413.8,81.6 450.0,81.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,68.4 124.4,113.1 160.5,110.1 196.7,103.1 232.9,99.8 269.1,92.8 305.3,87.8 341.5,85.3 377.6,78.6 413.8,71.8 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.7 88.2,64.6 124.4,110.9 160.5,108.8 196.7,108.3 232.9,96.4 269.1,89.4 305.3,81.8 341.5,79.8 377.6,71.8 413.8,64.6 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,61.8 124.4,108.7 160.5,107.6 196.7,94.7 232.9,92.8 269.1,87.7 305.3,81.0 341.5,71.3 377.6,64.6 413.8,55.1 450.0,47.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,60.6 124.4,106.4 160.5,103.2 196.7,96.6 232.9,91.8 269.1,82.7 305.3,72.5 341.5,67.0 377.6,60.2 413.8,49.6 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 157 ns | 183 ns | 184 ns | 184 ns |
| D38 | 4.92 ns | 201 ns | 204 ns | 203 ns | 187 ns |
| D57 | 241 ns | 369 ns | 407 ns | 454 ns | 500 ns |
| D76 | 306 ns | 411 ns | 457 ns | 481 ns | 606 ns |
| D115 | 587 ns | 680 ns | 458 ns | 994 ns | 842 ns |
| D153 | 627 ns | 776 ns | 898 ns | 1.04 µs | 1.15 µs |
| D230 | 881 ns | 1.23 µs | 1.37 µs | 1.46 µs | 1.83 µs |
| D307 | 1.39 µs | 1.65 µs | 2.17 µs | 2.15 µs | 3.35 µs |
| D462 | 1.46 µs | 1.79 µs | 2.28 µs | 3.58 µs | 4.31 µs |
| D616 | 1.76 µs | 2.49 µs | 3.54 µs | 5.09 µs | 6.32 µs |
| D924 | 2.53 µs | 3.75 µs | 5.3 µs | 8.52 µs | 11.3 µs |
| D1232 | 2.64 µs | 5.37 µs | 8.45 µs | 13.2 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,114.7 160.5,110.6 196.7,99.3 232.9,98.1 269.1,92.2 305.3,84.3 341.5,83.4 377.6,80.2 413.8,73.8 450.0,73.1 450.0,29.8 413.8,47.9 377.6,58.0 341.5,64.6 305.3,69.0 269.1,79.5 232.9,87.6 196.7,93.0 160.5,98.7 124.4,102.0 88.2,119.1 52.0,119.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,114.7 160.5,110.6 196.7,99.3 232.9,98.1 269.1,92.2 305.3,84.3 341.5,83.4 377.6,80.2 413.8,73.8 450.0,73.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,117.9 124.4,107.3 160.5,105.4 196.7,96.7 232.9,94.4 269.1,86.4 305.3,81.3 341.5,79.9 377.6,74.1 413.8,67.1 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.5 88.2,117.6 124.4,105.6 160.5,103.6 196.7,103.5 232.9,91.9 269.1,84.6 305.3,76.5 341.5,75.7 377.6,68.0 413.8,61.0 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,117.7 124.4,103.7 160.5,102.7 196.7,90.1 232.9,89.3 269.1,83.4 305.3,76.7 341.5,67.9 377.6,61.7 413.8,52.8 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,119.1 124.4,102.0 160.5,98.7 196.7,93.0 232.9,87.6 269.1,79.5 305.3,69.0 341.5,64.6 377.6,58.0 413.8,47.9 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
