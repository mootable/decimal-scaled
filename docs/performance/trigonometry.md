# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.64 ns | 23.2 µs | 29.7 µs | 32.4 µs | 36.8 µs |
| D38 | 4.29 µs | 6.88 µs | 7.28 µs | 12.4 µs | 16 µs |
| D57 | 4.67 µs | 8.4 µs | 11 µs | 14.6 µs | 19.3 µs |
| D76 | 5.39 µs | 8.22 µs | 14.3 µs | 19.4 µs | 19.8 µs |
| D115 | 5.19 µs | 12 µs | 25.3 µs | 31.4 µs | 26.4 µs |
| D153 | 5.2 µs | 12.5 µs | 29 µs | 41.1 µs | 63.8 µs |
| D230 | 5.24 µs | 23.9 µs | 46.5 µs | 67.6 µs | 121 µs |
| D307 | 4.75 µs | 27.1 µs | 59.3 µs | 129 µs | 191 µs |
| D462 | 4.9 µs | 40.7 µs | 131 µs | 237 µs | 371 µs |
| D616 | 4.92 µs | 59.1 µs | 213 µs | 422 µs | 704 µs |
| D924 | 4.46 µs | 124 µs | 420 µs | 930 µs | 1.67 ms |
| D1232 | 5.6 µs | 202 µs | 701 µs | 1.63 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.5 88.2,106.2 124.4,105.2 160.5,103.4 196.7,103.8 232.9,103.8 269.1,103.7 305.3,104.9 341.5,104.6 377.6,104.5 413.8,105.7 450.0,102.9 450.0,23.1 413.8,32.2 377.6,42.9 341.5,50.9 305.3,59.1 269.1,64.8 232.9,72.7 196.7,83.7 160.5,87.3 124.4,87.5 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.5 88.2,106.2 124.4,105.2 160.5,103.4 196.7,103.8 232.9,103.8 269.1,103.7 305.3,104.9 341.5,104.6 377.6,104.5 413.8,105.7 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,100.4 124.4,97.9 160.5,98.2 196.7,93.4 232.9,93.0 269.1,84.9 305.3,83.3 341.5,78.3 377.6,73.7 413.8,64.5 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.6 124.4,94.5 160.5,91.3 196.7,84.2 232.9,82.5 269.1,76.6 305.3,73.6 341.5,63.8 377.6,57.8 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.0 124.4,91.0 160.5,87.5 196.7,81.5 232.9,78.2 269.1,72.0 305.3,64.0 341.5,56.4 377.6,49.3 413.8,39.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,87.5 160.5,87.3 196.7,83.7 232.9,72.7 269.1,64.8 305.3,59.1 341.5,50.9 377.6,42.9 413.8,32.2 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 21.2 µs | 31 µs | 35.5 µs | 39 µs | 35.9 µs |
| D38 | 25.8 µs | 33.2 µs | 41.3 µs | 57.4 µs | 66.1 µs |
| D57 | 3.33 µs | 4.25 µs | 5 µs | 6.09 µs | 8.18 µs |
| D76 | 3.68 µs | 4.21 µs | 5.69 µs | 8.41 µs | 9.1 µs |
| D115 | 6.65 µs | 9.26 µs | 13.6 µs | 16.4 µs | 13.4 µs |
| D153 | 6.6 µs | 9.06 µs | 15.9 µs | 21.7 µs | 33.2 µs |
| D230 | 8.83 µs | 15.9 µs | 28.4 µs | 42.8 µs | 69.5 µs |
| D307 | 12.6 µs | 26.4 µs | 46.9 µs | 88.4 µs | 138 µs |
| D462 | 12.3 µs | 36.8 µs | 89.6 µs | 161 µs | 261 µs |
| D616 | 21.3 µs | 70.9 µs | 180 µs | 314 µs | 553 µs |
| D924 | 26.5 µs | 155 µs | 381 µs | 804 µs | 1.43 ms |
| D1232 | 43 µs | 256 µs | 710 µs | 1.54 ms | 3.07 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,143.6 88.2,139.5 124.4,183.9 160.5,181.7 196.7,168.9 232.9,169.0 269.1,162.7 305.3,155.0 341.5,155.5 377.6,143.6 413.8,138.9 450.0,128.3 450.0,35.7 413.8,52.2 377.6,72.9 341.5,89.2 305.3,103.0 269.1,117.9 232.9,133.9 196.7,153.7 160.5,162.0 124.4,164.4 88.2,119.0 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,143.6 88.2,139.5 124.4,183.9 160.5,181.7 196.7,168.9 232.9,169.0 269.1,162.7 305.3,155.0 341.5,155.5 377.6,143.6 413.8,138.9 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,133.9 124.4,178.6 160.5,178.8 196.7,161.7 232.9,162.1 269.1,149.9 305.3,138.9 341.5,131.7 377.6,117.5 413.8,100.4 450.0,89.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.5 88.2,129.2 124.4,175.0 160.5,172.2 196.7,153.4 232.9,150.0 269.1,137.4 305.3,126.4 341.5,112.4 377.6,97.2 413.8,80.9 450.0,67.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,122.1 124.4,170.8 160.5,163.8 196.7,149.2 232.9,143.2 269.1,128.4 305.3,112.7 341.5,99.6 377.6,85.1 413.8,64.8 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,119.0 124.4,164.4 160.5,162.0 196.7,153.7 232.9,133.9 269.1,117.9 305.3,103.0 341.5,89.2 377.6,72.9 413.8,52.2 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 23.2 µs | 29.7 µs | 32.4 µs | 36.8 µs |
| D38 | 4.27 µs | 6.92 µs | 7.24 µs | 12.4 µs | 16 µs |
| D57 | 4.62 µs | 8.36 µs | 11 µs | 14.5 µs | 19.2 µs |
| D76 | 5.35 µs | 8.17 µs | 14.2 µs | 19.4 µs | 19.6 µs |
| D115 | 5.12 µs | 12 µs | 25.6 µs | 31.4 µs | 26.2 µs |
| D153 | 5.14 µs | 12.4 µs | 29.7 µs | 40.9 µs | 63.7 µs |
| D230 | 5.21 µs | 23.9 µs | 43.5 µs | 67.9 µs | 121 µs |
| D307 | 4.72 µs | 27.1 µs | 59.8 µs | 130 µs | 189 µs |
| D462 | 4.88 µs | 41 µs | 131 µs | 237 µs | 373 µs |
| D616 | 4.88 µs | 59.3 µs | 214 µs | 424 µs | 701 µs |
| D924 | 4.39 µs | 124 µs | 420 µs | 933 µs | 1.67 ms |
| D1232 | 5.81 µs | 202 µs | 698 µs | 1.64 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,106.3 124.4,105.3 160.5,103.5 196.7,104.0 232.9,104.0 269.1,103.8 305.3,105.0 341.5,104.6 377.6,104.6 413.8,105.9 450.0,102.5 450.0,23.1 413.8,32.2 377.6,43.0 341.5,50.8 305.3,59.2 269.1,64.7 232.9,72.7 196.7,83.8 160.5,87.3 124.4,87.6 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,106.3 124.4,105.3 160.5,103.5 196.7,104.0 232.9,104.0 269.1,103.8 305.3,105.0 341.5,104.6 377.6,104.6 413.8,105.9 450.0,102.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.3 88.2,100.3 124.4,97.9 160.5,98.2 196.7,93.5 232.9,93.0 269.1,84.9 305.3,83.4 341.5,78.2 377.6,73.6 413.8,64.4 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.7 124.4,94.5 160.5,91.3 196.7,84.0 232.9,82.2 269.1,77.5 305.3,73.5 341.5,63.8 377.6,57.7 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.1 124.4,91.1 160.5,87.5 196.7,81.5 232.9,78.2 269.1,71.9 305.3,63.9 341.5,56.4 377.6,49.2 413.8,39.4 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,87.6 160.5,87.3 196.7,83.8 232.9,72.7 269.1,64.7 305.3,59.2 341.5,50.8 377.6,43.0 413.8,32.2 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 23 µs | 29 µs | 31.8 µs | 36.1 µs |
| D38 | 3.73 ns | 27.1 µs | 33.7 µs | 42.9 µs | 55.2 µs |
| D57 | 1.95 ns | 5.32 µs | 7.32 µs | 8.88 µs | 12.8 µs |
| D76 | 2.23 ns | 5.67 µs | 8.73 µs | 12.9 µs | 13.8 µs |
| D115 | 12.4 ns | 13.6 µs | 20.4 µs | 24.2 µs | 21.6 µs |
| D153 | 15.9 ns | 13.5 µs | 24.3 µs | 33.5 µs | 48.1 µs |
| D230 | 32.1 ns | 24.6 µs | 45 µs | 62.1 µs | 95.8 µs |
| D307 | 44.7 ns | 36.9 µs | 67.3 µs | 120 µs | 173 µs |
| D462 | 69.6 ns | 55.3 µs | 125 µs | 197 µs | 316 µs |
| D616 | 84.5 ns | 108 µs | 247 µs | 405 µs | 680 µs |
| D924 | 75.4 ns | 233 µs | 496 µs | 944 µs | 1.6 ms |
| D1232 | 154 ns | 368 µs | 928 µs | 1.76 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,193.6 124.4,201.7 160.5,200.0 196.7,178.7 232.9,175.7 269.1,166.9 305.3,162.8 341.5,157.4 377.6,155.0 413.8,156.4 450.0,147.5 450.0,24.5 413.8,32.7 377.6,43.4 341.5,52.9 305.3,60.4 269.1,67.7 232.9,76.2 196.7,86.1 160.5,91.7 124.4,92.7 88.2,74.5 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,193.6 124.4,201.7 160.5,200.0 196.7,178.7 232.9,175.7 269.1,166.9 305.3,162.8 341.5,157.4 377.6,155.0 413.8,156.4 450.0,147.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,83.3 124.4,103.6 160.5,102.8 196.7,91.9 232.9,92.0 269.1,84.6 305.3,79.5 341.5,74.5 377.6,66.2 413.8,56.6 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.5 88.2,80.6 124.4,99.6 160.5,97.4 196.7,86.9 232.9,84.7 269.1,77.0 305.3,72.1 341.5,64.3 377.6,55.9 413.8,47.3 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,77.6 124.4,97.2 160.5,92.5 196.7,84.7 232.9,80.7 269.1,73.1 305.3,64.9 341.5,58.8 377.6,49.8 413.8,39.3 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,74.5 124.4,92.7 160.5,91.7 196.7,86.1 232.9,76.2 269.1,67.7 305.3,60.4 341.5,52.9 377.6,43.4 413.8,32.7 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.63 µs | 2.69 µs | 2.93 µs | 3.07 µs |
| D38 | 5.37 µs | 8.14 µs | 5.98 µs | 10.7 µs | 14 µs |
| D57 | 3.83 µs | 7.07 µs | 9.49 µs | 12.9 µs | 5.35 µs |
| D76 | 4.43 µs | 7.14 µs | 12.4 µs | 17.1 µs | 17.7 µs |
| D115 | 4.23 µs | 10.3 µs | 22.5 µs | 28.3 µs | 24.7 µs |
| D153 | 4.27 µs | 10.9 µs | 22.5 µs | 37.4 µs | 58.7 µs |
| D230 | 4.32 µs | 21.4 µs | 40.1 µs | 62.8 µs | 113 µs |
| D307 | 3.92 µs | 24 µs | 48.4 µs | 120 µs | 177 µs |
| D462 | 3.03 µs | 33.1 µs | 113 µs | 208 µs | 319 µs |
| D616 | 4.03 µs | 54.2 µs | 199 µs | 393 µs | 663 µs |
| D924 | 3.64 µs | 114 µs | 390 µs | 888 µs | 1.57 ms |
| D1232 | 4.63 µs | 186 µs | 665 µs | 1.57 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,103.4 124.4,107.6 160.5,105.8 196.7,106.4 232.9,106.3 269.1,106.1 305.3,107.3 341.5,110.5 377.6,107.0 413.8,108.3 450.0,105.3 450.0,23.6 413.8,33.0 377.6,43.7 341.5,52.7 305.3,60.1 269.1,65.7 232.9,73.8 196.7,84.5 160.5,88.7 124.4,103.5 88.2,91.5 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,103.4 124.4,107.6 160.5,105.8 196.7,106.4 232.9,106.3 269.1,106.1 305.3,107.3 341.5,110.5 377.6,107.0 413.8,108.3 450.0,105.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,98.3 124.4,100.0 160.5,99.9 196.7,95.3 232.9,94.6 269.1,86.2 305.3,84.9 341.5,80.9 377.6,74.7 413.8,65.5 450.0,59.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,102.1 124.4,96.4 160.5,93.0 196.7,85.6 232.9,85.7 269.1,78.5 305.3,76.2 341.5,65.6 377.6,58.6 413.8,50.3 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,94.8 124.4,92.5 160.5,89.0 196.7,82.8 232.9,79.3 269.1,72.9 305.3,64.9 341.5,58.1 377.6,50.2 413.8,40.0 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,91.5 124.4,103.5 160.5,88.7 196.7,84.5 232.9,73.8 269.1,65.7 305.3,60.1 341.5,52.7 377.6,43.7 413.8,33.0 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 7.81 µs | 9.4 µs | 10.3 µs | 10.6 µs |
| D38 | 3.74 ns | 8.82 µs | 11 µs | 13.9 µs | 16 µs |
| D57 | 503 ns | 5.43 µs | 6.95 µs | 8.47 µs | 11.8 µs |
| D76 | 612 ns | 5.52 µs | 7.98 µs | 11.8 µs | 13.7 µs |
| D115 | 1.25 µs | 12.8 µs | 19.5 µs | 24.9 µs | 19.8 µs |
| D153 | 1.21 µs | 12.1 µs | 23.6 µs | 33.4 µs | 54.1 µs |
| D230 | 1.45 µs | 23.3 µs | 44.4 µs | 71.7 µs | 117 µs |
| D307 | 2.22 µs | 39.7 µs | 77.5 µs | 150 µs | 244 µs |
| D462 | 2.2 µs | 57.1 µs | 152 µs | 289 µs | 473 µs |
| D616 | 3.78 µs | 114 µs | 310 µs | 557 µs | 1.01 ms |
| D924 | 4.41 µs | 258 µs | 676 µs | 1.48 ms | 2.67 ms |
| D1232 | 7.6 µs | 439 µs | 1.28 ms | 2.85 ms | 5.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,193.6 124.4,132.8 160.5,130.4 196.7,121.5 232.9,121.9 269.1,119.7 305.3,114.4 341.5,114.5 377.6,107.8 413.8,105.9 450.0,99.1 450.0,16.8 413.8,26.4 377.6,38.4 341.5,47.9 305.3,56.1 269.1,65.1 232.9,74.8 196.7,87.2 160.5,91.8 124.4,93.7 88.2,89.8 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,193.6 124.4,132.8 160.5,130.4 196.7,121.5 232.9,121.9 269.1,119.7 305.3,114.4 341.5,114.5 377.6,107.8 413.8,105.9 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,97.3 124.4,103.3 160.5,103.1 196.7,92.7 232.9,93.3 269.1,85.2 305.3,78.6 341.5,74.1 377.6,65.6 413.8,55.4 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.5 124.4,100.2 160.5,98.5 196.7,87.5 232.9,85.0 269.1,77.2 305.3,70.3 341.5,61.9 377.6,53.1 413.8,43.4 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.6 124.4,97.8 160.5,93.7 196.7,84.4 232.9,80.8 269.1,71.3 305.3,62.1 341.5,54.0 377.6,45.8 413.8,33.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,89.8 124.4,93.7 160.5,91.8 196.7,87.2 232.9,74.8 269.1,65.1 305.3,56.1 341.5,47.9 377.6,38.4 413.8,26.4 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.69 ns | 4.26 µs | 5.96 µs | 6.41 µs | 6.44 µs |
| D38 | 4.98 ns | 5.5 µs | 6.75 µs | 8.59 µs | 9.7 µs |
| D57 | 2.18 ns | 3.34 µs | 4.31 µs | 5.16 µs | 9.49 µs |
| D76 | 3.13 ns | 3.49 µs | 5.28 µs | 7.42 µs | 8.89 µs |
| D115 | 17.1 ns | 4.7 µs | 10.1 µs | 13.6 µs | 11.2 µs |
| D153 | 22.4 ns | 4.31 µs | 10.5 µs | 18.4 µs | 31.8 µs |
| D230 | 52.8 ns | 9.53 µs | 19.7 µs | 35.5 µs | 68.1 µs |
| D307 | 81.7 ns | 11.9 µs | 25.4 µs | 72.5 µs | 113 µs |
| D462 | 135 ns | 15 µs | 65.7 µs | 133 µs | 218 µs |
| D616 | 146 ns | 30.2 µs | 128 µs | 259 µs | 455 µs |
| D924 | 115 ns | 69.1 µs | 253 µs | 611 µs | 1.12 ms |
| D1232 | 386 ns | 120 µs | 450 µs | 1.12 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.8 88.2,190.1 124.4,200.3 160.5,195.8 196.7,174.8 232.9,171.4 269.1,160.8 305.3,155.4 341.5,149.1 377.6,148.2 413.8,151.2 450.0,136.1 450.0,27.5 413.8,37.1 377.6,48.3 341.5,57.5 305.3,65.6 269.1,71.9 232.9,81.3 196.7,94.4 160.5,97.2 124.4,96.4 88.2,96.1 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.8 88.2,190.1 124.4,200.3 160.5,195.8 196.7,174.8 232.9,171.4 269.1,160.8 305.3,155.4 341.5,149.1 377.6,148.2 413.8,151.2 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,103.1 124.4,109.3 160.5,108.8 196.7,105.1 232.9,106.2 269.1,96.3 305.3,93.5 341.5,90.7 377.6,82.0 413.8,71.7 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.1 88.2,100.6 124.4,106.1 160.5,103.6 196.7,95.6 232.9,95.1 269.1,87.3 305.3,84.2 341.5,72.4 377.6,64.0 413.8,55.6 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.6 124.4,103.9 160.5,99.4 196.7,91.9 232.9,88.2 269.1,80.0 305.3,71.1 341.5,63.6 377.6,55.3 413.8,44.7 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,96.1 124.4,96.4 160.5,97.2 196.7,94.4 232.9,81.3 269.1,71.9 305.3,65.6 341.5,57.5 377.6,48.3 413.8,37.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 7.91 µs | 10.2 µs | 11 µs | 11.1 µs |
| D38 | 4.05 ns | 9.45 µs | 11.5 µs | 14.5 µs | 16.5 µs |
| D57 | 2.49 ns | 5.17 µs | 7.01 µs | 7.83 µs | 10.9 µs |
| D76 | 3.17 ns | 5.25 µs | 7.38 µs | 10.9 µs | 11.8 µs |
| D115 | 10.9 ns | 12.6 µs | 12.7 µs | 20.9 µs | 15.6 µs |
| D153 | 21.6 ns | 6.32 µs | 15.6 µs | 22.4 µs | 38.9 µs |
| D230 | 52.6 ns | 13.4 µs | 24 µs | 42.6 µs | 78.6 µs |
| D307 | 82.2 ns | 16 µs | 49.3 µs | 84.1 µs | 123 µs |
| D462 | 136 ns | 22.9 µs | 85.7 µs | 156 µs | 231 µs |
| D616 | 146 ns | 36.2 µs | 141 µs | 269 µs | 446 µs |
| D924 | 118 ns | 81.8 µs | 267 µs | 607 µs | 995 µs |
| D1232 | 399 ns | 132 µs | 444 µs | 992 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,192.7 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,160.8 305.3,155.3 341.5,149.1 377.6,148.2 413.8,150.8 450.0,135.7 450.0,25.6 413.8,38.6 377.6,48.6 341.5,56.7 305.3,64.5 269.1,70.1 232.9,78.9 196.7,90.2 160.5,93.7 124.4,94.7 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,192.7 124.4,198.7 160.5,195.7 196.7,180.4 232.9,171.9 269.1,160.8 305.3,155.3 341.5,149.1 377.6,148.2 413.8,150.8 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.4 124.4,103.9 160.5,103.7 196.7,92.8 232.9,101.4 269.1,92.1 305.3,89.9 341.5,85.5 377.6,79.8 413.8,69.6 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.9 124.4,100.1 160.5,99.5 196.7,92.7 232.9,90.2 269.1,84.8 305.3,75.9 341.5,69.1 377.6,62.9 413.8,55.0 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.7 160.5,94.7 196.7,86.5 232.9,85.7 269.1,77.7 305.3,69.3 341.5,61.7 377.6,54.9 413.8,44.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.7 160.5,93.7 196.7,90.2 232.9,78.9 269.1,70.1 305.3,64.5 341.5,56.7 377.6,48.6 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 ns | 4.12 µs | 5.76 µs | 6.33 µs | 6.36 µs |
| D38 | 4.36 ns | 5.32 µs | 6.57 µs | 8.38 µs | 9.63 µs |
| D57 | 2.18 ns | 3.14 µs | 4.06 µs | 5.05 µs | 9.46 µs |
| D76 | 3.87 ns | 3.25 µs | 4.85 µs | 7.32 µs | 8.37 µs |
| D115 | 17 ns | 4.46 µs | 9.95 µs | 13.1 µs | 10.6 µs |
| D153 | 22.5 ns | 4.12 µs | 9.86 µs | 17.6 µs | 31.3 µs |
| D230 | 52.5 ns | 9.55 µs | 19.4 µs | 35.1 µs | 66.6 µs |
| D307 | 77 ns | 11.4 µs | 24.5 µs | 70.8 µs | 113 µs |
| D462 | 122 ns | 14.9 µs | 63.7 µs | 133 µs | 215 µs |
| D616 | 140 ns | 29.2 µs | 125 µs | 253 µs | 450 µs |
| D924 | 107 ns | 68.6 µs | 250 µs | 607 µs | 1.12 ms |
| D1232 | 395 ns | 119 µs | 448 µs | 1.11 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.3 88.2,191.7 124.4,200.3 160.5,193.2 196.7,174.8 232.9,171.3 269.1,160.8 305.3,156.1 341.5,150.3 377.6,148.7 413.8,152.0 450.0,135.8 450.0,27.5 413.8,37.2 377.6,48.5 341.5,57.7 305.3,65.7 269.1,72.2 232.9,81.6 196.7,95.0 160.5,97.9 124.4,96.4 88.2,96.2 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.3 88.2,191.7 124.4,200.3 160.5,193.2 196.7,174.8 232.9,171.3 269.1,160.8 305.3,156.1 341.5,150.3 377.6,148.7 413.8,152.0 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,103.6 124.4,110.1 160.5,109.7 196.7,105.7 232.9,106.7 269.1,96.3 305.3,94.1 341.5,90.8 377.6,82.4 413.8,71.8 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.6 88.2,100.9 124.4,106.9 160.5,104.7 196.7,95.8 232.9,95.9 269.1,87.5 305.3,84.6 341.5,72.7 377.6,64.4 413.8,55.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,97.9 124.4,104.2 160.5,99.6 196.7,92.4 232.9,88.7 269.1,80.1 305.3,71.4 341.5,63.6 377.6,55.6 413.8,44.8 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,96.2 124.4,96.4 160.5,97.9 196.7,95.0 232.9,81.6 269.1,72.2 305.3,65.7 341.5,57.7 377.6,48.5 413.8,37.2 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 7.92 µs | 10.2 µs | 11 µs | 11.1 µs |
| D38 | 4.05 ns | 9.42 µs | 11.5 µs | 14.5 µs | 16.5 µs |
| D57 | 10.6 ns | 5.18 µs | 7.05 µs | 7.85 µs | 10.9 µs |
| D76 | 12.1 ns | 5.28 µs | 7.38 µs | 10.9 µs | 11.8 µs |
| D115 | 11.3 ns | 12.6 µs | 12.1 µs | 20.8 µs | 15.6 µs |
| D153 | 20.8 ns | 6.34 µs | 15.6 µs | 22.6 µs | 38.6 µs |
| D230 | 53.2 ns | 13.6 µs | 24.3 µs | 42.4 µs | 79.2 µs |
| D307 | 76.6 ns | 16.1 µs | 48.8 µs | 84.1 µs | 123 µs |
| D462 | 131 ns | 23.2 µs | 85.5 µs | 155 µs | 231 µs |
| D616 | 140 ns | 36.9 µs | 141 µs | 268 µs | 446 µs |
| D924 | 118 ns | 81.6 µs | 268 µs | 607 µs | 996 µs |
| D1232 | 381 ns | 132 µs | 446 µs | 991 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,192.7 124.4,180.7 160.5,179.1 196.7,180.0 232.9,172.4 269.1,160.7 305.3,156.2 341.5,149.5 377.6,148.7 413.8,150.8 450.0,136.3 450.0,25.6 413.8,38.6 377.6,48.6 341.5,56.8 305.3,64.6 269.1,70.0 232.9,78.9 196.7,90.2 160.5,93.7 124.4,94.7 88.2,89.5 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,192.7 124.4,180.7 160.5,179.1 196.7,180.0 232.9,172.4 269.1,160.7 305.3,156.2 341.5,149.5 377.6,148.7 413.8,150.8 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.5 124.4,103.9 160.5,103.6 196.7,92.9 232.9,101.4 269.1,91.9 305.3,89.8 341.5,85.3 377.6,79.5 413.8,69.7 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,93.9 124.4,100.0 160.5,99.5 196.7,93.3 232.9,90.2 269.1,84.7 305.3,76.0 341.5,69.1 377.6,62.9 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.7 160.5,94.6 196.7,86.6 232.9,85.6 269.1,77.8 305.3,69.3 341.5,61.7 377.6,54.9 413.8,44.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,89.5 124.4,94.7 160.5,93.7 196.7,90.2 232.9,78.9 269.1,70.0 305.3,64.6 341.5,56.8 377.6,48.6 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.3 ns | 8.22 µs | 10.7 µs | 11.6 µs | 11.7 µs |
| D38 | 4.36 ns | 9.92 µs | 12.2 µs | 15.6 µs | 17.7 µs |
| D57 | 2.81 ns | 4.14 µs | 5.5 µs | 6.72 µs | 9.38 µs |
| D76 | 4.22 ns | 4.24 µs | 6.5 µs | 9.44 µs | 10.4 µs |
| D115 | 16.8 ns | 5.95 µs | 12.5 µs | 16.1 µs | 13 µs |
| D153 | 22.2 ns | 5.51 µs | 12.9 µs | 21.2 µs | 36.4 µs |
| D230 | 58.2 ns | 12 µs | 23.2 µs | 40.5 µs | 76 µs |
| D307 | 74.7 ns | 14.3 µs | 29.2 µs | 79.2 µs | 128 µs |
| D462 | 120 ns | 17.8 µs | 72.8 µs | 146 µs | 234 µs |
| D616 | 135 ns | 33.9 µs | 140 µs | 279 µs | 490 µs |
| D924 | 102 ns | 77.3 µs | 274 µs | 652 µs | 1.2 ms |
| D1232 | 414 ns | 133 µs | 484 µs | 1.19 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,191.7 124.4,197.2 160.5,192.1 196.7,175.0 232.9,171.5 269.1,159.6 305.3,156.5 341.5,150.6 377.6,149.2 413.8,152.7 450.0,135.2 450.0,26.9 413.8,36.3 377.6,47.4 341.5,56.6 305.3,64.1 269.1,70.5 232.9,79.7 196.7,92.5 160.5,95.2 124.4,96.5 88.2,88.6 52.0,93.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,191.7 124.4,197.2 160.5,192.1 196.7,175.0 232.9,171.5 269.1,159.6 305.3,156.5 341.5,150.6 377.6,149.2 413.8,152.7 450.0,135.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.1 88.2,95.8 124.4,106.7 160.5,106.4 196.7,102.2 232.9,103.1 269.1,93.4 305.3,91.3 341.5,88.6 377.6,80.6 413.8,70.3 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,93.2 124.4,103.1 160.5,101.1 196.7,92.9 232.9,92.5 269.1,85.3 305.3,82.4 341.5,71.1 377.6,63.0 413.8,54.6 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,90.2 124.4,100.6 160.5,96.4 196.7,89.8 232.9,86.4 269.1,78.4 305.3,70.0 341.5,62.4 377.6,54.4 413.8,43.9 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,88.6 124.4,96.5 160.5,95.2 196.7,92.5 232.9,79.7 269.1,70.5 305.3,64.1 341.5,56.6 377.6,47.4 413.8,36.3 450.0,26.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 7.92 µs | 10.3 µs | 11.2 µs | 11.2 µs |
| D38 | 3.73 ns | 9.58 µs | 11.5 µs | 14.6 µs | 16.5 µs |
| D57 | 2.63 µs | 5.39 µs | 7.33 µs | 8.18 µs | 11.2 µs |
| D76 | 2.87 µs | 5.41 µs | 7.66 µs | 11.2 µs | 12.1 µs |
| D115 | 6.03 µs | 13.1 µs | 12.8 µs | 22 µs | 16 µs |
| D153 | 3.08 µs | 6.54 µs | 16.4 µs | 23.2 µs | 39.2 µs |
| D230 | 3.16 µs | 14 µs | 24.9 µs | 43.5 µs | 80.3 µs |
| D307 | 3.15 µs | 16.5 µs | 50.2 µs | 85.4 µs | 125 µs |
| D462 | 3.35 µs | 23.7 µs | 87 µs | 157 µs | 234 µs |
| D616 | 3.49 µs | 36.9 µs | 142 µs | 272 µs | 451 µs |
| D924 | 3.46 µs | 83.1 µs | 270 µs | 611 µs | 1.01 ms |
| D1232 | 4.75 µs | 134 µs | 449 µs | 1 ms | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,193.6 124.4,112.3 160.5,111.2 196.7,102.0 232.9,110.3 269.1,110.0 305.3,110.0 341.5,109.3 377.6,108.8 413.8,108.9 450.0,105.0 450.0,25.5 413.8,38.5 377.6,48.5 341.5,56.6 305.3,64.4 269.1,69.9 232.9,78.8 196.7,89.9 160.5,93.3 124.4,94.3 88.2,89.5 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,193.6 124.4,112.3 160.5,111.2 196.7,102.0 232.9,110.3 269.1,110.0 305.3,110.0 341.5,109.3 377.6,108.8 413.8,108.9 450.0,105.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,96.2 124.4,103.4 160.5,103.3 196.7,92.4 232.9,101.0 269.1,91.5 305.3,89.5 341.5,85.0 377.6,79.5 413.8,69.4 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,93.9 124.4,99.6 160.5,99.0 196.7,92.6 232.9,89.6 269.1,84.4 305.3,75.7 341.5,68.9 377.6,62.8 413.8,54.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,91.0 124.4,98.2 160.5,94.3 196.7,85.9 232.9,85.3 269.1,77.5 305.3,69.1 341.5,61.5 377.6,54.7 413.8,44.7 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,89.5 124.4,94.3 160.5,93.3 196.7,89.9 232.9,78.8 269.1,69.9 305.3,64.4 341.5,56.6 377.6,48.5 413.8,38.5 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.08 ns | 3.21 µs | 3.46 µs | 3.8 µs | 3.85 µs |
| D38 | 4.05 ns | 3.22 µs | 4.01 µs | 5.08 µs | 5.85 µs |
| D57 | 183 ns | 291 ns | 302 ns | 312 ns | 433 ns |
| D76 | 200 ns | 257 ns | 308 ns | 432 ns | 423 ns |
| D115 | 380 ns | 516 ns | 646 ns | 694 ns | 486 ns |
| D153 | 399 ns | 462 ns | 703 ns | 751 ns | 1.02 µs |
| D230 | 570 ns | 776 ns | 1.03 µs | 1.24 µs | 1.72 µs |
| D307 | 847 ns | 1.13 µs | 1.45 µs | 2.15 µs | 2.73 µs |
| D462 | 867 ns | 1.31 µs | 2.26 µs | 2.96 µs | 3.83 µs |
| D616 | 1.08 µs | 1.73 µs | 3.02 µs | 4.06 µs | 5.93 µs |
| D924 | 1.19 µs | 2.62 µs | 4.33 µs | 7.44 µs | 11.2 µs |
| D1232 | 2.29 µs | 3.9 µs | 7.24 µs | 11.7 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.5 88.2,185.7 124.4,119.5 160.5,117.9 196.7,106.8 232.9,105.9 269.1,99.8 305.3,92.9 341.5,92.5 377.6,88.7 413.8,86.9 450.0,75.6 450.0,30.7 413.8,48.1 377.6,59.1 341.5,66.7 305.3,72.5 269.1,80.6 232.9,89.7 196.7,102.5 160.5,104.9 124.4,104.5 88.2,59.3 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.5 88.2,185.7 124.4,119.5 160.5,117.9 196.7,106.8 232.9,105.9 269.1,99.8 305.3,92.9 341.5,92.5 377.6,88.7 413.8,86.9 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,69.7 124.4,111.4 160.5,113.6 196.7,101.5 232.9,103.4 269.1,94.4 305.3,87.9 341.5,85.4 377.6,80.5 413.8,73.3 450.0,66.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.4 88.2,65.9 124.4,110.8 160.5,110.4 196.7,97.6 232.9,96.1 269.1,89.5 305.3,83.5 341.5,75.9 377.6,70.8 413.8,64.5 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,61.8 124.4,110.3 160.5,104.6 196.7,96.3 232.9,95.0 269.1,86.2 305.3,76.7 341.5,71.1 377.6,65.7 413.8,55.1 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,59.3 124.4,104.5 160.5,104.9 196.7,102.5 232.9,89.7 269.1,80.6 305.3,72.5 341.5,66.7 377.6,59.1 413.8,48.1 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 157 ns | 201 ns | 204 ns | 186 ns |
| D38 | 4.36 ns | 183 ns | 187 ns | 203 ns | 206 ns |
| D57 | 275 ns | 395 ns | 414 ns | 430 ns | 529 ns |
| D76 | 307 ns | 352 ns | 418 ns | 548 ns | 549 ns |
| D115 | 624 ns | 718 ns | 860 ns | 931 ns | 582 ns |
| D153 | 626 ns | 636 ns | 903 ns | 961 ns | 1.26 µs |
| D230 | 953 ns | 1.17 µs | 1.37 µs | 1.63 µs | 2.12 µs |
| D307 | 1.39 µs | 1.65 µs | 1.96 µs | 2.75 µs | 3.38 µs |
| D462 | 1.47 µs | 1.79 µs | 2.84 µs | 3.56 µs | 4.45 µs |
| D616 | 1.76 µs | 2.31 µs | 3.82 µs | 4.78 µs | 6.8 µs |
| D924 | 1.91 µs | 3.55 µs | 5.29 µs | 8.48 µs | 12.3 µs |
| D1232 | 3.5 µs | 5.02 µs | 8.44 µs | 13.3 µs | 32 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,184.4 124.4,112.4 160.5,110.5 196.7,98.2 232.9,98.1 269.1,90.8 305.3,84.2 341.5,83.3 377.6,80.2 413.8,78.8 450.0,68.2 450.0,29.8 413.8,46.4 377.6,56.7 341.5,64.1 305.3,68.8 269.1,76.9 232.9,86.0 196.7,99.4 160.5,100.4 124.4,101.1 88.2,117.4 52.0,119.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,184.4 124.4,112.4 160.5,110.5 196.7,98.2 232.9,98.1 269.1,90.8 305.3,84.2 341.5,83.3 377.6,80.2 413.8,78.8 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,119.5 124.4,106.1 160.5,108.1 196.7,95.8 232.9,97.9 269.1,87.3 305.3,81.3 341.5,79.9 377.6,75.5 413.8,68.0 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,119.2 124.4,105.3 160.5,105.2 196.7,92.6 232.9,91.8 269.1,84.5 305.3,78.3 341.5,71.9 377.6,66.7 413.8,61.1 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,117.7 124.4,104.7 160.5,100.5 196.7,91.2 232.9,90.7 269.1,81.5 305.3,72.4 341.5,68.0 377.6,62.8 413.8,52.9 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.3 88.2,117.4 124.4,101.1 160.5,100.4 196.7,99.4 232.9,86.0 269.1,76.9 305.3,68.8 341.5,64.1 377.6,56.7 413.8,46.4 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
