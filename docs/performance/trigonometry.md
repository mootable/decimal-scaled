# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.17 ns | 21.8 µs | 13.5 µs | 30.3 µs | 36.8 µs |
| D38 | 4.28 µs | 7.68 µs | 8.29 µs | 12.4 µs | 16 µs |
| D57 | 5.03 µs | 8.3 µs | 12.4 µs | 14.6 µs | 19.3 µs |
| D76 | 5.2 µs | 9.41 µs | 16 µs | 19.4 µs | 19.9 µs |
| D115 | 4.2 µs | 11 µs | 23.6 µs | 33.1 µs | 44.7 µs |
| D153 | 2.9 µs | 14.2 µs | 26.7 µs | 40.9 µs | 59.7 µs |
| D230 | 5.27 µs | 25.3 µs | 43.8 µs | 68 µs | 121 µs |
| D307 | 5.19 µs | 29.2 µs | 59.6 µs | 120 µs | 177 µs |
| D462 | 4.89 µs | 46.8 µs | 131 µs | 250 µs | 396 µs |
| D616 | 4.92 µs | 60.2 µs | 213 µs | 445 µs | 701 µs |
| D924 | 4.92 µs | 132 µs | 419 µs | 936 µs | 1.52 ms |
| D1232 | 5.6 µs | 167 µs | 373 µs | 1.5 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,106.2 124.4,104.2 160.5,103.8 196.7,106.5 232.9,111.1 269.1,103.7 305.3,103.9 341.5,104.6 377.6,104.5 413.8,104.5 450.0,102.9 450.0,23.1 413.8,33.4 377.6,43.0 341.5,50.1 305.3,60.1 269.1,64.7 232.9,73.5 196.7,77.1 160.5,87.2 124.4,87.6 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,106.2 124.4,104.2 160.5,103.8 196.7,106.5 232.9,111.1 269.1,103.7 305.3,103.9 341.5,104.6 377.6,104.5 413.8,104.5 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,99.0 124.4,98.0 160.5,96.5 196.7,94.5 232.9,91.4 269.1,84.2 305.3,82.4 341.5,76.6 377.6,73.4 413.8,63.7 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.0 88.2,98.0 124.4,93.0 160.5,89.9 196.7,85.0 232.9,83.5 269.1,77.4 305.3,73.6 341.5,63.8 377.6,57.8 413.8,49.4 450.0,50.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.1 124.4,91.0 160.5,87.5 196.7,80.8 232.9,78.2 269.1,71.9 305.3,64.9 341.5,55.8 377.6,48.6 413.8,39.4 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,87.6 160.5,87.2 196.7,77.1 232.9,73.5 269.1,64.7 305.3,60.1 341.5,50.1 377.6,43.0 413.8,33.4 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 28.8 µs | 17.1 µs | 36.4 µs | 35.9 µs |
| D38 | 25.7 µs | 35.5 µs | 44.2 µs | 57.4 µs | 66.1 µs |
| D57 | 3.7 µs | 4.28 µs | 5.43 µs | 6.07 µs | 8.19 µs |
| D76 | 3.72 µs | 5.1 µs | 6.3 µs | 8.37 µs | 9.18 µs |
| D115 | 5.49 µs | 8.63 µs | 12.1 µs | 16.7 µs | 23.6 µs |
| D153 | 3.64 µs | 9.88 µs | 14.6 µs | 21.3 µs | 29.9 µs |
| D230 | 8.76 µs | 17.1 µs | 28.5 µs | 43 µs | 70.1 µs |
| D307 | 12.8 µs | 27.6 µs | 47.2 µs | 83.7 µs | 132 µs |
| D462 | 12.4 µs | 39 µs | 89.3 µs | 168 µs | 277 µs |
| D616 | 21.6 µs | 70.8 µs | 179 µs | 333 µs | 551 µs |
| D924 | 31.7 µs | 162 µs | 383 µs | 799 µs | 1.34 ms |
| D1232 | 43 µs | 213 µs | 387 µs | 1.44 ms | 3.07 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.5 88.2,139.5 124.4,181.6 160.5,181.5 196.7,173.0 232.9,181.9 269.1,162.9 305.3,154.6 341.5,155.3 377.6,143.3 413.8,135.0 450.0,128.3 450.0,35.6 413.8,53.6 377.6,72.9 341.5,87.9 305.3,104.0 269.1,117.7 232.9,136.2 196.7,141.4 160.5,161.8 124.4,164.3 88.2,119.0 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.5 88.2,139.5 124.4,181.6 160.5,181.5 196.7,173.0 232.9,181.9 269.1,162.9 305.3,154.6 341.5,155.3 377.6,143.3 413.8,135.0 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,137.1 88.2,132.5 124.4,178.4 160.5,174.6 196.7,163.2 232.9,160.3 269.1,148.4 305.3,138.0 341.5,130.4 377.6,117.5 413.8,99.5 450.0,93.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.4 88.2,127.7 124.4,173.3 160.5,170.0 196.7,155.8 232.9,151.8 269.1,137.3 305.3,126.3 341.5,112.5 377.6,97.4 413.8,80.8 450.0,80.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,122.0 124.4,170.8 160.5,163.9 196.7,148.9 232.9,143.6 269.1,128.3 305.3,113.9 341.5,98.8 377.6,83.9 413.8,64.9 450.0,52.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,119.0 124.4,164.3 160.5,161.8 196.7,141.4 232.9,136.2 269.1,117.7 305.3,104.0 341.5,87.9 377.6,72.9 413.8,53.6 450.0,35.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 21.8 µs | 13.5 µs | 30.3 µs | 36.8 µs |
| D38 | 4.23 µs | 7.67 µs | 8.21 µs | 12.4 µs | 16 µs |
| D57 | 5.09 µs | 8.39 µs | 12.4 µs | 14.6 µs | 19.2 µs |
| D76 | 5.14 µs | 9.49 µs | 16 µs | 19.4 µs | 19.7 µs |
| D115 | 4.15 µs | 11 µs | 23.5 µs | 31.7 µs | 43.3 µs |
| D153 | 2.97 µs | 14.2 µs | 26.5 µs | 41.9 µs | 59.7 µs |
| D230 | 5.21 µs | 25.3 µs | 43.7 µs | 67.6 µs | 121 µs |
| D307 | 5.18 µs | 28.7 µs | 59.2 µs | 121 µs | 178 µs |
| D462 | 4.8 µs | 43.4 µs | 131 µs | 249 µs | 397 µs |
| D616 | 4.85 µs | 60.4 µs | 214 µs | 442 µs | 702 µs |
| D924 | 4.86 µs | 132 µs | 420 µs | 930 µs | 1.52 ms |
| D1232 | 5.54 µs | 168 µs | 375 µs | 1.5 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,106.4 124.4,104.1 160.5,104.0 196.7,106.6 232.9,110.8 269.1,103.8 305.3,103.9 341.5,104.8 377.6,104.7 413.8,104.7 450.0,103.0 450.0,23.1 413.8,33.4 377.6,43.0 341.5,50.0 305.3,60.0 269.1,64.7 232.9,73.5 196.7,77.5 160.5,87.3 124.4,87.6 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,106.4 124.4,104.1 160.5,104.0 196.7,106.6 232.9,110.8 269.1,103.8 305.3,103.9 341.5,104.8 377.6,104.7 413.8,104.7 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.1 88.2,99.0 124.4,97.9 160.5,96.4 196.7,94.6 232.9,91.4 269.1,84.2 305.3,82.6 341.5,77.5 377.6,73.4 413.8,63.7 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.0 88.2,98.2 124.4,93.1 160.5,89.9 196.7,85.1 232.9,83.6 269.1,77.4 305.3,73.7 341.5,63.7 377.6,57.7 413.8,49.3 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.1 124.4,91.0 160.5,87.5 196.7,81.4 232.9,77.9 269.1,72.0 305.3,64.8 341.5,55.8 377.6,48.7 413.8,39.5 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,87.6 160.5,87.3 196.7,77.5 232.9,73.5 269.1,64.7 305.3,60.0 341.5,50.0 377.6,43.0 413.8,33.4 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 21.4 µs | 12.9 µs | 29.7 µs | 36.1 µs |
| D38 | 3.73 ns | 29 µs | 36.2 µs | 43 µs | 55.2 µs |
| D57 | 2.11 ns | 5.39 µs | 8.01 µs | 8.89 µs | 12.8 µs |
| D76 | 2.22 ns | 6.96 µs | 9.89 µs | 12.9 µs | 13.8 µs |
| D115 | 9.64 ns | 12.5 µs | 18.8 µs | 24.2 µs | 37 µs |
| D153 | 7.59 ns | 15.2 µs | 22.2 µs | 33.3 µs | 44.3 µs |
| D230 | 32.2 ns | 26.5 µs | 45.2 µs | 62.2 µs | 94.9 µs |
| D307 | 52.3 ns | 39.3 µs | 66.9 µs | 113 µs | 164 µs |
| D462 | 69.4 ns | 57.5 µs | 125 µs | 206 µs | 335 µs |
| D616 | 80.5 ns | 108 µs | 248 µs | 428 µs | 670 µs |
| D924 | 103 ns | 252 µs | 498 µs | 947 µs | 1.47 ms |
| D1232 | 154 ns | 310 µs | 494 µs | 1.62 ms | 3.1 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,200.1 196.7,181.9 232.9,184.9 269.1,166.9 305.3,160.9 341.5,157.4 377.6,155.5 413.8,152.5 450.0,147.5 450.0,24.5 413.8,33.8 377.6,43.5 341.5,52.1 305.3,61.0 269.1,67.8 232.9,77.2 196.7,79.5 160.5,91.7 124.4,92.6 88.2,74.5 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,200.1 196.7,181.9 232.9,184.9 269.1,166.9 305.3,160.9 341.5,157.4 377.6,155.5 413.8,152.5 450.0,147.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.3 88.2,82.5 124.4,103.4 160.5,100.2 196.7,92.9 232.9,90.5 269.1,83.6 305.3,78.7 341.5,74.0 377.6,66.2 413.8,55.7 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.5 88.2,79.8 124.4,98.5 160.5,95.9 196.7,87.9 232.9,85.8 269.1,77.0 305.3,72.1 341.5,64.4 377.6,55.9 413.8,47.2 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,77.6 124.4,97.2 160.5,92.6 196.7,84.7 232.9,80.8 269.1,73.0 305.3,65.6 341.5,58.2 377.6,49.1 413.8,39.2 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,74.5 124.4,92.6 160.5,91.7 196.7,79.5 232.9,77.2 269.1,67.8 305.3,61.0 341.5,52.1 377.6,43.5 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.35 ns | 1.58 µs | 2.4 µs | 2.82 µs | 3.06 µs |
| D38 | 5.41 µs | 8.96 µs | 6.9 µs | 10.7 µs | 14 µs |
| D57 | 4.14 µs | 7.16 µs | 10.7 µs | 13 µs | 5.4 µs |
| D76 | 4.25 µs | 8.12 µs | 14.2 µs | 17.1 µs | 17.7 µs |
| D115 | 3.54 µs | 9.48 µs | 21.5 µs | 28.5 µs | 39.5 µs |
| D153 | 2.36 µs | 12.4 µs | 20.8 µs | 37.2 µs | 54.6 µs |
| D230 | 4.35 µs | 22.6 µs | 39.9 µs | 63.4 µs | 113 µs |
| D307 | 4.34 µs | 25.7 µs | 48.7 µs | 112 µs | 165 µs |
| D462 | 3.05 µs | 35.3 µs | 112 µs | 221 µs | 336 µs |
| D616 | 4.03 µs | 54.8 µs | 199 µs | 415 µs | 660 µs |
| D924 | 4.12 µs | 123 µs | 393 µs | 892 µs | 1.43 ms |
| D1232 | 4.64 µs | 156 µs | 357 µs | 1.44 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,103.3 124.4,106.7 160.5,106.3 196.7,108.6 232.9,113.7 269.1,106.0 305.3,106.1 341.5,110.4 377.6,107.0 413.8,106.7 450.0,105.3 450.0,23.6 413.8,34.1 377.6,43.7 341.5,52.1 305.3,60.9 269.1,65.6 232.9,74.6 196.7,78.7 160.5,88.6 124.4,103.4 88.2,91.5 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,103.3 124.4,106.7 160.5,106.3 196.7,108.6 232.9,113.7 269.1,106.0 305.3,106.1 341.5,110.4 377.6,107.0 413.8,106.7 450.0,105.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,97.1 124.4,99.9 160.5,98.3 196.7,96.4 232.9,93.0 269.1,85.6 305.3,84.0 341.5,80.1 377.6,74.6 413.8,64.6 450.0,61.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,100.3 124.4,94.8 160.5,91.3 196.7,86.2 232.9,86.6 269.1,78.6 305.3,76.1 341.5,65.8 377.6,58.6 413.8,50.2 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,94.9 124.4,92.5 160.5,89.1 196.7,82.7 232.9,79.4 269.1,72.8 305.3,65.7 341.5,57.3 377.6,49.5 413.8,40.0 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,91.5 124.4,103.4 160.5,88.6 196.7,78.7 232.9,74.6 269.1,65.6 305.3,60.9 341.5,52.1 377.6,43.7 413.8,34.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.13 µs | 5.17 µs | 9.69 µs | 10.6 µs |
| D38 | 3.73 ns | 9.4 µs | 11.8 µs | 13.9 µs | 16 µs |
| D57 | 620 ns | 5.43 µs | 7.32 µs | 8.51 µs | 11.9 µs |
| D76 | 609 ns | 6.52 µs | 8.61 µs | 11.8 µs | 13.8 µs |
| D115 | 1.11 µs | 11.9 µs | 17.7 µs | 25.2 µs | 36.6 µs |
| D153 | 656 ns | 13.5 µs | 21.9 µs | 33.3 µs | 48.5 µs |
| D230 | 1.46 µs | 25 µs | 43.8 µs | 72.3 µs | 118 µs |
| D307 | 2.25 µs | 41.1 µs | 77.5 µs | 143 µs | 233 µs |
| D462 | 2.17 µs | 60.3 µs | 153 µs | 299 µs | 497 µs |
| D616 | 3.77 µs | 114 µs | 314 µs | 593 µs | 1.01 ms |
| D924 | 5.47 µs | 273 µs | 682 µs | 1.46 ms | 2.5 ms |
| D1232 | 7.61 µs | 363 µs | 679 µs | 2.68 ms | 5.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,130.2 160.5,130.4 196.7,123.0 232.9,129.5 269.1,119.5 305.3,114.2 341.5,114.7 377.6,107.8 413.8,103.2 450.0,99.1 450.0,16.8 413.8,27.2 377.6,38.5 341.5,47.2 305.3,56.6 269.1,65.0 232.9,76.1 196.7,79.6 160.5,91.8 124.4,93.6 88.2,89.8 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,130.2 160.5,130.4 196.7,123.0 232.9,129.5 269.1,119.5 305.3,114.2 341.5,114.7 377.6,107.8 413.8,103.2 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.9 88.2,96.5 124.4,103.3 160.5,101.0 196.7,93.6 232.9,92.0 269.1,84.3 305.3,78.2 341.5,73.4 377.6,65.5 413.8,54.7 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.9 88.2,93.7 124.4,99.6 160.5,97.6 196.7,88.6 232.9,86.0 269.1,77.4 305.3,70.3 341.5,61.9 377.6,53.0 413.8,43.3 450.0,43.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,91.6 124.4,97.7 160.5,93.7 196.7,84.2 232.9,80.8 269.1,71.2 305.3,62.7 341.5,53.6 377.6,45.1 413.8,33.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,89.8 124.4,93.6 160.5,91.8 196.7,79.6 232.9,76.1 269.1,65.0 305.3,56.6 341.5,47.2 377.6,38.5 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 3.96 µs | 3.33 µs | 5.9 µs | 6.44 µs |
| D38 | 4.98 ns | 5.96 µs | 7.33 µs | 8.6 µs | 9.7 µs |
| D57 | 2.81 ns | 3.33 µs | 4.77 µs | 5.19 µs | 9.5 µs |
| D76 | 3.13 ns | 3.97 µs | 5.51 µs | 7.42 µs | 8.71 µs |
| D115 | 13 ns | 4.39 µs | 9.5 µs | 13.8 µs | 19.1 µs |
| D153 | 10.2 ns | 5.19 µs | 9.45 µs | 18.3 µs | 29.5 µs |
| D230 | 52.9 ns | 10.2 µs | 19.5 µs | 35.7 µs | 68.1 µs |
| D307 | 96.4 ns | 12.6 µs | 25.3 µs | 68 µs | 109 µs |
| D462 | 131 ns | 15.9 µs | 64.8 µs | 139 µs | 229 µs |
| D616 | 146 ns | 30 µs | 127 µs | 268 µs | 452 µs |
| D924 | 175 ns | 74.2 µs | 254 µs | 617 µs | 1.03 ms |
| D1232 | 384 ns | 102 µs | 233 µs | 1.04 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,195.8 196.7,178.2 232.9,181.1 269.1,160.8 305.3,153.3 341.5,149.5 377.6,148.2 413.8,145.9 450.0,136.2 450.0,27.5 413.8,38.1 377.6,48.4 341.5,56.8 305.3,66.1 269.1,71.9 232.9,82.3 196.7,87.7 160.5,97.4 124.4,96.4 88.2,96.1 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,195.8 196.7,178.2 232.9,181.1 269.1,160.8 305.3,153.3 341.5,149.5 377.6,148.2 413.8,145.9 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,102.1 124.4,109.4 160.5,107.2 196.7,105.9 232.9,103.9 269.1,95.4 305.3,92.9 341.5,90.0 377.6,82.1 413.8,70.8 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,99.6 124.4,104.9 160.5,103.1 196.7,96.3 232.9,96.4 269.1,87.4 305.3,84.2 341.5,72.5 377.6,64.2 413.8,55.6 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.3 88.2,97.6 124.4,103.9 160.5,99.4 196.7,91.7 232.9,88.2 269.1,79.9 305.3,71.9 341.5,63.1 377.6,54.9 413.8,44.6 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,96.1 124.4,96.4 160.5,97.4 196.7,87.7 232.9,82.3 269.1,71.9 305.3,66.1 341.5,56.8 377.6,48.4 413.8,38.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 7.25 µs | 5.45 µs | 10.2 µs | 11.1 µs |
| D38 | 4.05 ns | 10.2 µs | 12.5 µs | 14.5 µs | 16.5 µs |
| D57 | 2.47 ns | 5.19 µs | 7.41 µs | 7.87 µs | 10.9 µs |
| D76 | 3.17 ns | 6.12 µs | 8.04 µs | 10.9 µs | 11.8 µs |
| D115 | 8.45 ns | 11.9 µs | 11.4 µs | 20.8 µs | 27.8 µs |
| D153 | 10.1 ns | 7.47 µs | 15 µs | 22.5 µs | 35.3 µs |
| D230 | 52.6 ns | 14.7 µs | 24 µs | 42.4 µs | 78.4 µs |
| D307 | 97.3 ns | 17 µs | 49.1 µs | 79 µs | 116 µs |
| D462 | 131 ns | 24.7 µs | 85.7 µs | 165 µs | 247 µs |
| D616 | 150 ns | 36 µs | 141 µs | 288 µs | 445 µs |
| D924 | 190 ns | 88.6 µs | 268 µs | 605 µs | 910 µs |
| D1232 | 392 ns | 112 µs | 233 µs | 910 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,192.7 124.4,198.8 160.5,195.7 196.7,183.5 232.9,181.3 269.1,160.8 305.3,153.2 341.5,149.5 377.6,147.8 413.8,144.9 450.0,135.9 450.0,25.6 413.8,39.7 377.6,48.6 341.5,55.9 305.3,65.3 269.1,70.2 232.9,80.0 196.7,83.0 160.5,93.7 124.4,94.7 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,192.7 124.4,198.8 160.5,195.7 196.7,183.5 232.9,181.3 269.1,160.8 305.3,153.2 341.5,149.5 377.6,147.8 413.8,144.9 450.0,135.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.7 88.2,95.5 124.4,103.8 160.5,101.8 196.7,93.6 232.9,99.3 269.1,91.0 305.3,89.1 341.5,84.5 377.6,79.8 413.8,68.7 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.3 88.2,93.0 124.4,99.4 160.5,98.4 196.7,94.1 232.9,90.7 269.1,84.9 305.3,76.0 341.5,69.1 377.6,62.9 413.8,54.9 450.0,56.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,91.1 124.4,98.7 160.5,94.6 196.7,86.6 232.9,85.7 269.1,77.8 305.3,70.1 341.5,60.9 377.6,54.0 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.7 160.5,93.7 196.7,83.0 232.9,80.0 269.1,70.2 305.3,65.3 341.5,55.9 377.6,48.6 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 3.83 µs | 3.18 µs | 5.85 µs | 6.36 µs |
| D38 | 4.36 ns | 5.76 µs | 7.12 µs | 8.39 µs | 9.64 µs |
| D57 | 2.78 ns | 3.08 µs | 4.56 µs | 5.09 µs | 9.49 µs |
| D76 | 3.87 ns | 3.69 µs | 5.21 µs | 7.3 µs | 8.34 µs |
| D115 | 13.2 ns | 4.1 µs | 9.33 µs | 13.1 µs | 18.5 µs |
| D153 | 10.3 ns | 4.95 µs | 10 µs | 17.5 µs | 28.8 µs |
| D230 | 52.6 ns | 10.2 µs | 19.5 µs | 35.4 µs | 66.8 µs |
| D307 | 88.5 ns | 12.1 µs | 24.4 µs | 66.4 µs | 108 µs |
| D462 | 127 ns | 16.2 µs | 63.2 µs | 140 µs | 231 µs |
| D616 | 147 ns | 29 µs | 126 µs | 269 µs | 455 µs |
| D924 | 163 ns | 73 µs | 250 µs | 610 µs | 1.03 ms |
| D1232 | 401 ns | 99.7 µs | 231 µs | 1.03 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.3 160.5,193.2 196.7,178.0 232.9,181.0 269.1,160.8 305.3,154.4 341.5,149.9 377.6,148.0 413.8,146.8 450.0,135.6 450.0,27.5 413.8,38.2 377.6,48.4 341.5,56.8 305.3,66.1 269.1,72.1 232.9,82.6 196.7,88.1 160.5,98.0 124.4,96.4 88.2,96.2 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.3 160.5,193.2 196.7,178.0 232.9,181.0 269.1,160.8 305.3,154.4 341.5,149.9 377.6,148.0 413.8,146.8 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.6 88.2,102.6 124.4,110.3 160.5,108.1 196.7,106.8 232.9,104.4 269.1,95.5 305.3,93.3 341.5,89.7 377.6,82.5 413.8,71.0 450.0,67.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,99.9 124.4,105.5 160.5,103.8 196.7,96.6 232.9,95.7 269.1,87.4 305.3,84.6 341.5,72.8 377.6,64.2 413.8,55.8 450.0,56.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,97.9 124.4,104.1 160.5,99.6 196.7,92.4 232.9,88.7 269.1,80.0 305.3,72.2 341.5,63.0 377.6,54.9 413.8,44.7 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,96.2 124.4,96.4 160.5,98.0 196.7,88.1 232.9,82.6 269.1,72.1 305.3,66.1 341.5,56.8 377.6,48.4 413.8,38.2 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.29 µs | 5.46 µs | 10.2 µs | 11.1 µs |
| D38 | 4.05 ns | 10.2 µs | 12.5 µs | 14.5 µs | 16.5 µs |
| D57 | 12.2 ns | 5.18 µs | 7.44 µs | 7.9 µs | 10.9 µs |
| D76 | 12.1 ns | 6.12 µs | 8.08 µs | 10.9 µs | 11.8 µs |
| D115 | 8.72 ns | 11.8 µs | 11.2 µs | 21.2 µs | 27.9 µs |
| D153 | 9.99 ns | 7.5 µs | 14.8 µs | 23.3 µs | 35.2 µs |
| D230 | 53.1 ns | 14.3 µs | 24.2 µs | 43.1 µs | 78.7 µs |
| D307 | 87.4 ns | 17.1 µs | 49 µs | 79.3 µs | 116 µs |
| D462 | 124 ns | 24.7 µs | 86 µs | 165 µs | 248 µs |
| D616 | 140 ns | 36.1 µs | 141 µs | 288 µs | 446 µs |
| D924 | 171 ns | 87.9 µs | 268 µs | 607 µs | 911 µs |
| D1232 | 380 ns | 112 µs | 233 µs | 914 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.7 124.4,179.0 160.5,179.0 196.7,183.1 232.9,181.4 269.1,160.7 305.3,154.5 341.5,150.2 377.6,148.7 413.8,146.2 450.0,136.3 450.0,25.6 413.8,39.7 377.6,48.6 341.5,55.9 305.3,65.3 269.1,70.1 232.9,80.1 196.7,83.0 160.5,93.6 124.4,94.7 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.7 124.4,179.0 160.5,179.0 196.7,183.1 232.9,181.4 269.1,160.7 305.3,154.5 341.5,150.2 377.6,148.7 413.8,146.2 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.6 88.2,95.5 124.4,103.9 160.5,101.8 196.7,93.7 232.9,99.3 269.1,91.3 305.3,89.0 341.5,84.5 377.6,79.8 413.8,68.7 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.2 88.2,93.0 124.4,99.4 160.5,98.4 196.7,94.3 232.9,90.8 269.1,84.8 305.3,76.0 341.5,69.0 377.6,62.9 413.8,54.9 450.0,56.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,91.1 124.4,98.6 160.5,94.6 196.7,86.4 232.9,85.2 269.1,77.6 305.3,70.0 341.5,60.9 377.6,54.0 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.7 160.5,93.6 196.7,83.0 232.9,80.1 269.1,70.1 305.3,65.3 341.5,55.9 377.6,48.6 413.8,39.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 7.64 µs | 6.04 µs | 10.8 µs | 11.7 µs |
| D38 | 4.36 ns | 10.7 µs | 13.2 µs | 15.6 µs | 17.7 µs |
| D57 | 3.16 ns | 4.18 µs | 6.07 µs | 6.68 µs | 9.4 µs |
| D76 | 4.22 ns | 4.89 µs | 7.19 µs | 9.38 µs | 10.4 µs |
| D115 | 13 ns | 5.53 µs | 12 µs | 16.3 µs | 22.9 µs |
| D153 | 9.91 ns | 6.63 µs | 11.7 µs | 21.5 µs | 33.6 µs |
| D230 | 58.3 ns | 13 µs | 23.2 µs | 41.3 µs | 75.7 µs |
| D307 | 85.9 ns | 15.2 µs | 28.8 µs | 74.5 µs | 119 µs |
| D462 | 121 ns | 20.3 µs | 73.5 µs | 154 µs | 246 µs |
| D616 | 135 ns | 33.4 µs | 139 µs | 293 µs | 484 µs |
| D924 | 142 ns | 82.5 µs | 276 µs | 653 µs | 1.1 ms |
| D1232 | 388 ns | 110 µs | 254 µs | 1.1 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,191.7 124.4,195.7 160.5,192.1 196.7,178.1 232.9,181.5 269.1,159.5 305.3,154.7 341.5,150.5 377.6,149.2 413.8,148.5 450.0,136.0 450.0,26.8 413.8,37.3 377.6,47.6 341.5,56.0 305.3,65.0 269.1,70.6 232.9,80.7 196.7,85.4 160.5,95.3 124.4,96.5 88.2,88.6 52.0,93.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,191.7 124.4,195.7 160.5,192.1 196.7,178.1 232.9,181.5 269.1,159.5 305.3,154.7 341.5,150.5 377.6,149.2 413.8,148.5 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,94.8 124.4,106.5 160.5,104.6 196.7,103.1 232.9,100.8 269.1,92.5 305.3,90.5 341.5,86.9 377.6,80.7 413.8,69.5 450.0,65.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.0 88.2,92.2 124.4,101.9 160.5,99.8 196.7,93.4 232.9,93.8 269.1,85.3 305.3,82.6 341.5,71.0 377.6,63.0 413.8,54.5 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,90.2 124.4,100.7 160.5,96.5 196.7,89.7 232.9,86.2 269.1,78.1 305.3,70.8 341.5,61.8 377.6,53.8 413.8,43.9 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.7 88.2,88.6 124.4,96.5 160.5,95.3 196.7,85.4 232.9,80.7 269.1,70.6 305.3,65.0 341.5,56.0 377.6,47.6 413.8,37.3 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.26 µs | 5.61 µs | 10.4 µs | 11.2 µs |
| D38 | 3.73 ns | 10.3 µs | 12.5 µs | 14.6 µs | 16.5 µs |
| D57 | 2.85 µs | 5.31 µs | 7.73 µs | 8.18 µs | 11.2 µs |
| D76 | 2.86 µs | 6.3 µs | 8.29 µs | 11.2 µs | 12.2 µs |
| D115 | 5.03 µs | 12.4 µs | 11.7 µs | 21.5 µs | 28.7 µs |
| D153 | 1.75 µs | 7.83 µs | 15.5 µs | 23.2 µs | 36 µs |
| D230 | 3.17 µs | 14.8 µs | 24.7 µs | 43.2 µs | 79.8 µs |
| D307 | 3.37 µs | 17.7 µs | 49.8 µs | 80.7 µs | 118 µs |
| D462 | 3.42 µs | 25.2 µs | 87 µs | 167 µs | 251 µs |
| D616 | 3.49 µs | 37.2 µs | 142 µs | 292 µs | 449 µs |
| D924 | 3.74 µs | 88.2 µs | 271 µs | 613 µs | 917 µs |
| D1232 | 4.75 µs | 113 µs | 238 µs | 917 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,111.3 196.7,104.3 232.9,117.4 269.1,110.0 305.3,109.2 341.5,109.0 377.6,108.8 413.8,107.9 450.0,105.0 450.0,25.5 413.8,39.6 377.6,48.5 341.5,55.7 305.3,65.0 269.1,69.9 232.9,79.8 196.7,82.7 160.5,93.3 124.4,94.3 88.2,89.5 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,111.3 196.7,104.3 232.9,117.4 269.1,110.0 305.3,109.2 341.5,109.0 377.6,108.8 413.8,107.9 450.0,105.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.7 88.2,95.3 124.4,103.6 160.5,101.4 196.7,93.0 232.9,98.8 269.1,90.8 305.3,88.6 341.5,84.3 377.6,79.4 413.8,68.7 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.9 88.2,93.0 124.4,98.9 160.5,98.0 196.7,93.7 232.9,90.3 269.1,84.5 305.3,75.8 341.5,68.9 377.6,62.8 413.8,54.8 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.0 124.4,98.2 160.5,94.3 196.7,86.2 232.9,85.3 269.1,77.6 305.3,69.8 341.5,60.8 377.6,53.9 413.8,44.6 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,89.5 124.4,94.3 160.5,93.3 196.7,82.7 232.9,79.8 269.1,69.9 305.3,65.0 341.5,55.7 377.6,48.5 413.8,39.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 2.93 µs | 1.46 µs | 3.53 µs | 3.85 µs |
| D38 | 4.05 ns | 3.48 µs | 4.31 µs | 5.08 µs | 5.85 µs |
| D57 | 197 ns | 292 ns | 330 ns | 307 ns | 433 ns |
| D76 | 200 ns | 316 ns | 336 ns | 442 ns | 420 ns |
| D115 | 310 ns | 471 ns | 591 ns | 708 ns | 838 ns |
| D153 | 211 ns | 496 ns | 614 ns | 755 ns | 897 ns |
| D230 | 561 ns | 872 ns | 1.03 µs | 1.23 µs | 1.73 µs |
| D307 | 926 ns | 1.24 µs | 1.45 µs | 2.04 µs | 2.54 µs |
| D462 | 869 ns | 1.49 µs | 2.25 µs | 3.19 µs | 4.16 µs |
| D616 | 1.09 µs | 1.75 µs | 3.02 µs | 4.35 µs | 6.01 µs |
| D924 | 1.43 µs | 2.84 µs | 4.31 µs | 7.46 µs | 10 µs |
| D1232 | 2.28 µs | 3.26 µs | 3.85 µs | 10.8 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,185.7 124.4,118.2 160.5,117.9 196.7,110.3 232.9,117.0 269.1,100.0 305.3,91.3 341.5,92.4 377.6,88.6 413.8,83.8 450.0,75.7 450.0,30.7 413.8,49.9 377.6,58.8 341.5,65.2 305.3,73.8 269.1,80.5 232.9,91.9 196.7,93.1 160.5,105.1 124.4,104.6 88.2,59.3 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,185.7 124.4,118.2 160.5,117.9 196.7,110.3 232.9,117.0 269.1,100.0 305.3,91.3 341.5,92.4 377.6,88.6 413.8,83.8 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,71.3 88.2,68.3 124.4,111.4 160.5,110.0 196.7,103.1 232.9,102.2 269.1,92.4 305.3,86.3 341.5,83.0 377.6,80.3 413.8,71.8 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.5 88.2,64.6 124.4,109.2 160.5,109.0 196.7,99.1 232.9,98.5 269.1,89.5 305.3,83.5 341.5,75.9 377.6,70.8 413.8,64.6 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,61.8 124.4,110.5 160.5,104.2 196.7,96.0 232.9,94.9 269.1,86.4 305.3,77.6 341.5,69.9 377.6,64.5 413.8,55.1 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,59.3 124.4,104.6 160.5,105.1 196.7,93.1 232.9,91.9 269.1,80.5 305.3,73.8 341.5,65.2 377.6,58.8 413.8,49.9 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 136 ns | 174 ns | 184 ns | 184 ns |
| D38 | 4.36 ns | 201 ns | 206 ns | 203 ns | 207 ns |
| D57 | 311 ns | 412 ns | 425 ns | 425 ns | 534 ns |
| D76 | 307 ns | 411 ns | 454 ns | 554 ns | 546 ns |
| D115 | 506 ns | 685 ns | 800 ns | 921 ns | 1.04 µs |
| D153 | 320 ns | 717 ns | 857 ns | 966 ns | 1.15 µs |
| D230 | 983 ns | 1.24 µs | 1.4 µs | 1.64 µs | 2.12 µs |
| D307 | 1.5 µs | 1.81 µs | 1.97 µs | 2.57 µs | 3.11 µs |
| D462 | 1.45 µs | 2.04 µs | 2.84 µs | 3.87 µs | 4.79 µs |
| D616 | 1.74 µs | 2.32 µs | 3.83 µs | 5.1 µs | 6.82 µs |
| D924 | 2.37 µs | 3.76 µs | 5.29 µs | 8.56 µs | 11.3 µs |
| D1232 | 3.51 µs | 4.18 µs | 4.42 µs | 12.3 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,110.3 160.5,110.5 196.7,101.8 232.9,109.8 269.1,90.3 305.3,83.0 341.5,83.5 377.6,80.3 413.8,75.0 450.0,68.2 450.0,29.8 413.8,47.9 377.6,56.6 341.5,62.8 305.3,70.3 269.1,77.0 232.9,87.6 196.7,89.2 160.5,100.5 124.4,100.9 88.2,117.4 52.0,119.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,110.3 160.5,110.5 196.7,101.8 232.9,109.8 269.1,90.3 305.3,83.0 341.5,83.5 377.6,80.3 413.8,75.0 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.6 88.2,117.9 124.4,105.4 160.5,105.5 196.7,96.6 232.9,95.8 269.1,86.3 305.3,79.7 341.5,77.6 377.6,75.4 413.8,67.0 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.3 88.2,117.4 124.4,104.9 160.5,103.7 196.7,93.9 232.9,92.7 269.1,84.2 305.3,78.3 341.5,71.9 377.6,66.7 413.8,61.1 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,117.7 124.4,104.9 160.5,100.2 196.7,91.4 232.9,90.6 269.1,81.4 305.3,73.6 341.5,66.5 377.6,61.7 413.8,52.7 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,117.4 124.4,100.9 160.5,100.5 196.7,89.2 232.9,87.6 269.1,77.0 305.3,70.3 341.5,62.8 377.6,56.6 413.8,47.9 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
