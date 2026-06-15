# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.1 ns | 21.8 µs | 27.7 µs | 32.4 µs | 39.4 µs |
| D38 | 4.25 µs | 6.92 µs | 6.69 µs | 12.5 µs | 16.1 µs |
| D57 | 5.11 µs | 8.33 µs | 10.8 µs | 16.1 µs | 17.1 µs |
| D76 | 5.11 µs | 8.55 µs | 15.7 µs | 17.3 µs | 22 µs |
| D115 | 4.83 µs | 12 µs | 22.2 µs | 33.3 µs | 43.1 µs |
| D153 | 4.88 µs | 14.2 µs | 26.9 µs | 40.6 µs | 56.6 µs |
| D230 | 5.27 µs | 26.3 µs | 40.9 µs | 68.1 µs | 129 µs |
| D307 | 5.51 µs | 26.7 µs | 59 µs | 121 µs | 191 µs |
| D462 | 5.31 µs | 43.9 µs | 123 µs | 247 µs | 368 µs |
| D616 | 5.27 µs | 64.9 µs | 184 µs | 412 µs | 698 µs |
| D924 | 4.95 µs | 123 µs | 412 µs | 848 µs | 1.49 ms |
| D1232 | 3.2 µs | 215 µs | 561 µs | 1.51 ms | 3.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.0 88.2,106.3 124.4,104.0 160.5,104.0 196.7,104.7 232.9,104.6 269.1,103.7 305.3,103.1 341.5,103.6 377.6,103.7 413.8,104.4 450.0,109.9 450.0,23.0 413.8,33.6 377.6,43.0 341.5,51.0 305.3,59.1 269.1,63.9 232.9,74.2 196.7,77.6 160.5,85.9 124.4,89.1 88.2,89.8 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.0 88.2,106.3 124.4,104.0 160.5,104.0 196.7,104.7 232.9,104.6 269.1,103.7 305.3,103.1 341.5,103.6 377.6,103.7 413.8,104.4 450.0,109.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,100.3 124.4,98.0 160.5,97.7 196.7,93.4 232.9,91.3 269.1,83.7 305.3,83.5 341.5,77.4 377.6,72.5 413.8,64.6 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,100.7 124.4,94.8 160.5,90.1 196.7,85.8 232.9,83.4 269.1,78.2 305.3,73.7 341.5,64.6 377.6,59.6 413.8,49.6 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.0 124.4,89.8 160.5,88.9 196.7,80.8 232.9,78.3 269.1,71.9 305.3,64.8 341.5,55.9 377.6,49.6 413.8,40.6 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.8 124.4,89.1 160.5,85.9 196.7,77.6 232.9,74.2 269.1,63.9 305.3,59.1 341.5,51.0 377.6,43.0 413.8,33.6 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 29.2 µs | 33.2 µs | 39 µs | 38.4 µs |
| D38 | 25.7 µs | 33.2 µs | 21.3 µs | 57.4 µs | 66.1 µs |
| D57 | 3.62 µs | 4.26 µs | 4.93 µs | 6.57 µs | 7.5 µs |
| D76 | 3.68 µs | 4.54 µs | 6.3 µs | 7.66 µs | 10.1 µs |
| D115 | 6.02 µs | 9.36 µs | 11.2 µs | 18 µs | 23.3 µs |
| D153 | 6.08 µs | 9.76 µs | 14.3 µs | 21.1 µs | 27.9 µs |
| D230 | 8.63 µs | 16.7 µs | 26.5 µs | 42.5 µs | 73.7 µs |
| D307 | 13 µs | 26.3 µs | 47.4 µs | 83.8 µs | 137 µs |
| D462 | 12.9 µs | 38.5 µs | 85.4 µs | 169 µs | 258 µs |
| D616 | 21.9 µs | 74.1 µs | 149 µs | 308 µs | 547 µs |
| D924 | 31.2 µs | 155 µs | 373 µs | 732 µs | 1.3 ms |
| D1232 | 23.9 µs | 275 µs | 570 µs | 1.45 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,139.5 124.4,182.1 160.5,181.7 196.7,171.0 232.9,170.8 269.1,163.2 305.3,154.3 341.5,154.5 377.6,143.0 413.8,135.3 450.0,141.1 450.0,35.5 413.8,54.3 377.6,73.1 341.5,89.4 305.3,103.1 269.1,116.6 232.9,137.7 196.7,141.6 160.5,159.7 124.4,166.3 88.2,119.0 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,139.5 124.4,182.1 160.5,181.7 196.7,171.0 232.9,170.8 269.1,163.2 305.3,154.3 341.5,154.5 377.6,143.0 413.8,135.3 450.0,141.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,136.8 88.2,133.9 124.4,178.5 160.5,177.2 196.7,161.4 232.9,160.5 269.1,148.9 305.3,139.0 341.5,130.8 377.6,116.5 413.8,100.5 450.0,88.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.0 88.2,143.6 124.4,175.3 160.5,170.0 196.7,157.6 232.9,152.3 269.1,138.8 305.3,126.2 341.5,113.4 377.6,101.3 413.8,81.4 450.0,72.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,122.1 124.4,169.1 160.5,165.8 196.7,147.2 232.9,143.8 269.1,128.6 305.3,113.8 341.5,98.5 377.6,85.5 413.8,66.8 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,119.0 124.4,166.3 160.5,159.7 196.7,141.6 232.9,137.7 269.1,116.6 305.3,103.1 341.5,89.4 377.6,73.1 413.8,54.3 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 21.8 µs | 27.7 µs | 32.4 µs | 39.4 µs |
| D38 | 4.18 µs | 6.93 µs | 6.65 µs | 12.5 µs | 16 µs |
| D57 | 5.05 µs | 8.32 µs | 10.8 µs | 16 µs | 17.2 µs |
| D76 | 5.05 µs | 8.58 µs | 15.5 µs | 17.3 µs | 21.9 µs |
| D115 | 4.75 µs | 12 µs | 21.9 µs | 34.6 µs | 43.6 µs |
| D153 | 4.79 µs | 14.1 µs | 27.3 µs | 40.8 µs | 57.7 µs |
| D230 | 5.19 µs | 25.3 µs | 41.2 µs | 67.6 µs | 129 µs |
| D307 | 5.15 µs | 26.7 µs | 59.5 µs | 121 µs | 190 µs |
| D462 | 5.27 µs | 43.5 µs | 123 µs | 249 µs | 368 µs |
| D616 | 5.21 µs | 65.1 µs | 183 µs | 412 µs | 698 µs |
| D924 | 4.99 µs | 123 µs | 410 µs | 847 µs | 1.5 ms |
| D1232 | 3.23 µs | 216 µs | 558 µs | 1.51 ms | 3.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,106.5 124.4,104.2 160.5,104.2 196.7,104.9 232.9,104.8 269.1,103.8 305.3,103.9 341.5,103.7 377.6,103.8 413.8,104.3 450.0,109.8 450.0,23.0 413.8,33.6 377.6,43.0 341.5,51.0 305.3,59.2 269.1,64.0 232.9,74.0 196.7,77.5 160.5,86.0 124.4,89.0 88.2,89.9 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,106.5 124.4,104.2 160.5,104.2 196.7,104.9 232.9,104.8 269.1,103.8 305.3,103.9 341.5,103.7 377.6,103.8 413.8,104.3 450.0,109.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,100.3 124.4,98.0 160.5,97.6 196.7,93.4 232.9,91.4 269.1,84.2 305.3,83.6 341.5,77.5 377.6,72.5 413.8,64.6 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,100.8 124.4,94.8 160.5,90.2 196.7,86.0 232.9,83.2 269.1,78.2 305.3,73.6 341.5,64.6 377.6,59.6 413.8,49.6 450.0,45.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,92.9 124.4,89.9 160.5,88.9 196.7,80.3 232.9,78.3 269.1,72.0 305.3,64.8 341.5,55.8 377.6,49.6 413.8,40.6 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.9 124.4,89.0 160.5,86.0 196.7,77.5 232.9,74.0 269.1,64.0 305.3,59.2 341.5,51.0 377.6,43.0 413.8,33.6 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 21.5 µs | 27.1 µs | 31.8 µs | 38.7 µs |
| D38 | 3.74 ns | 27.1 µs | 16.2 µs | 43 µs | 55.2 µs |
| D57 | 2.11 ns | 5.22 µs | 7.28 µs | 9.55 µs | 11.4 µs |
| D76 | 2.23 ns | 6.22 µs | 9.33 µs | 11.7 µs | 15.2 µs |
| D115 | 13.4 ns | 13.7 µs | 17.1 µs | 27.2 µs | 36.5 µs |
| D153 | 16.5 ns | 14.8 µs | 22.1 µs | 33.5 µs | 41.3 µs |
| D230 | 31.6 ns | 26.6 µs | 41.3 µs | 61.8 µs | 102 µs |
| D307 | 53.4 ns | 37.5 µs | 67.5 µs | 113 µs | 174 µs |
| D462 | 75.5 ns | 58.5 µs | 117 µs | 206 µs | 314 µs |
| D616 | 88.4 ns | 113 µs | 202 µs | 395 µs | 667 µs |
| D924 | 106 ns | 231 µs | 490 µs | 857 µs | 1.45 ms |
| D1232 | 80.9 ns | 397 µs | 730 µs | 1.62 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.0 196.7,177.8 232.9,175.2 269.1,167.2 305.3,160.6 341.5,156.4 377.6,154.4 413.8,152.1 450.0,155.5 450.0,24.5 413.8,34.0 377.6,43.6 341.5,53.0 305.3,60.3 269.1,66.9 232.9,78.1 196.7,79.6 160.5,90.5 124.4,94.1 88.2,74.5 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,200.7 160.5,200.0 196.7,177.8 232.9,175.2 269.1,167.2 305.3,160.6 341.5,156.4 377.6,154.4 413.8,152.1 450.0,155.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.2 88.2,83.4 124.4,103.8 160.5,101.6 196.7,91.9 232.9,90.8 269.1,83.6 305.3,79.3 341.5,73.8 377.6,65.6 413.8,56.7 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,89.8 124.4,99.7 160.5,96.6 196.7,89.0 232.9,85.9 269.1,78.1 305.3,72.0 341.5,65.2 377.6,58.4 413.8,47.4 450.0,42.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,77.6 124.4,96.3 160.5,93.8 196.7,83.3 232.9,80.7 269.1,73.1 305.3,65.6 341.5,58.1 377.6,50.1 413.8,40.5 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,74.5 124.4,94.1 160.5,90.5 196.7,79.6 232.9,78.1 269.1,66.9 305.3,60.3 341.5,53.0 377.6,43.6 413.8,34.0 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.4 ns | 1.58 µs | 2.59 µs | 2.93 µs | 3.19 µs |
| D38 | 5.39 µs | 8.07 µs | 5.51 µs | 10.9 µs | 14.2 µs |
| D57 | 4.2 µs | 7.04 µs | 9.44 µs | 14.1 µs | 5.06 µs |
| D76 | 4.24 µs | 7.39 µs | 13.7 µs | 15.2 µs | 19.5 µs |
| D115 | 3.96 µs | 10.4 µs | 19.4 µs | 29.9 µs | 39 µs |
| D153 | 3.95 µs | 12.4 µs | 20.9 µs | 36.7 µs | 51.3 µs |
| D230 | 4.34 µs | 22.5 µs | 36.8 µs | 63.1 µs | 119 µs |
| D307 | 4.29 µs | 23.9 µs | 48.2 µs | 113 µs | 178 µs |
| D462 | 3.2 µs | 35.2 µs | 103 µs | 217 µs | 315 µs |
| D616 | 4.38 µs | 58.6 µs | 170 µs | 384 µs | 660 µs |
| D924 | 4.05 µs | 115 µs | 382 µs | 806 µs | 1.41 ms |
| D1232 | 2.77 µs | 200 µs | 528 µs | 1.45 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,103.4 124.4,106.5 160.5,106.4 196.7,107.2 232.9,107.2 269.1,106.1 305.3,106.2 341.5,109.9 377.6,106.0 413.8,106.9 450.0,111.7 450.0,23.6 413.8,34.3 377.6,43.7 341.5,52.9 305.3,60.0 269.1,65.0 232.9,75.4 196.7,78.8 160.5,87.4 124.4,104.2 88.2,91.4 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,103.4 124.4,106.5 160.5,106.4 196.7,107.2 232.9,107.2 269.1,106.1 305.3,106.2 341.5,109.9 377.6,106.0 413.8,106.9 450.0,111.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,98.4 124.4,100.1 160.5,99.5 196.7,95.2 232.9,93.0 269.1,85.7 305.3,84.9 341.5,80.1 377.6,73.8 413.8,65.4 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,103.1 124.4,96.4 160.5,91.8 196.7,87.5 232.9,86.6 269.1,79.5 305.3,76.2 341.5,66.7 377.6,60.5 413.8,50.5 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,94.7 124.4,91.5 160.5,90.5 196.7,82.1 232.9,79.6 269.1,72.9 305.3,65.7 341.5,57.6 377.6,50.4 413.8,41.3 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,91.4 124.4,104.2 160.5,87.4 196.7,78.8 232.9,75.4 269.1,65.0 305.3,60.0 341.5,52.9 377.6,43.7 413.8,34.3 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 7.37 µs | 8.82 µs | 10.3 µs | 11.3 µs |
| D38 | 3.74 ns | 8.83 µs | 6.45 µs | 13.9 µs | 16.1 µs |
| D57 | 599 ns | 5.42 µs | 6.84 µs | 9.06 µs | 11 µs |
| D76 | 606 ns | 5.92 µs | 8.68 µs | 11 µs | 15.1 µs |
| D115 | 1.04 µs | 12.7 µs | 16.1 µs | 27.1 µs | 36.2 µs |
| D153 | 1.06 µs | 13.3 µs | 21.9 µs | 32.8 µs | 44.8 µs |
| D230 | 1.44 µs | 24.2 µs | 40.8 µs | 70.4 µs | 126 µs |
| D307 | 2.28 µs | 40.1 µs | 77.9 µs | 144 µs | 243 µs |
| D462 | 2.32 µs | 59.3 µs | 147 µs | 299 µs | 473 µs |
| D616 | 4 µs | 118 µs | 260 µs | 549 µs | 1 ms |
| D924 | 5.42 µs | 264 µs | 662 µs | 1.35 ms | 2.41 ms |
| D1232 | 3.81 µs | 468 µs | 1.03 ms | 2.7 ms | 5.81 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,130.6 160.5,130.5 196.7,123.8 232.9,123.5 269.1,119.8 305.3,114.1 341.5,113.9 377.6,107.1 413.8,103.3 450.0,107.7 450.0,16.7 413.8,27.6 377.6,38.5 341.5,47.9 305.3,56.1 269.1,64.3 232.9,77.1 196.7,79.8 160.5,90.6 124.4,94.5 88.2,89.8 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,130.6 160.5,130.5 196.7,123.8 232.9,123.5 269.1,119.8 305.3,114.1 341.5,113.9 377.6,107.1 413.8,103.3 450.0,107.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,97.3 124.4,103.3 160.5,102.2 196.7,92.7 232.9,92.2 269.1,84.7 305.3,78.5 341.5,73.6 377.6,65.1 413.8,55.1 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,101.2 124.4,100.4 160.5,97.5 196.7,89.8 232.9,86.0 269.1,78.3 305.3,70.2 341.5,62.4 377.6,55.3 413.8,43.7 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.6 124.4,96.9 160.5,94.5 196.7,83.3 232.9,81.0 269.1,71.5 305.3,62.6 341.5,53.6 377.6,46.0 413.8,34.8 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,89.8 124.4,94.5 160.5,90.6 196.7,79.8 232.9,77.1 269.1,64.3 305.3,56.1 341.5,47.9 377.6,38.5 413.8,27.6 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 3.96 µs | 5.52 µs | 6.42 µs | 6.99 µs |
| D38 | 4.99 ns | 5.51 µs | 4.06 µs | 8.61 µs | 9.7 µs |
| D57 | 2.81 ns | 3.32 µs | 4.29 µs | 5.54 µs | 8.77 µs |
| D76 | 3.14 ns | 3.68 µs | 5.45 µs | 6.73 µs | 9.28 µs |
| D115 | 17.4 ns | 4.73 µs | 8.7 µs | 14.1 µs | 19.8 µs |
| D153 | 22.7 ns | 5.09 µs | 9.24 µs | 17.9 µs | 27.1 µs |
| D230 | 52.8 ns | 9.96 µs | 18 µs | 35.9 µs | 72.1 µs |
| D307 | 96.1 ns | 12.2 µs | 24.9 µs | 66.8 µs | 117 µs |
| D462 | 153 ns | 16.3 µs | 60 µs | 138 µs | 216 µs |
| D616 | 171 ns | 32.5 µs | 109 µs | 249 µs | 448 µs |
| D924 | 175 ns | 67.9 µs | 247 µs | 554 µs | 1.02 ms |
| D1232 | 210 ns | 129 µs | 366 µs | 1.04 ms | 2.46 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.2 269.1,160.8 305.3,153.3 341.5,147.6 377.6,146.2 413.8,145.9 450.0,143.7 450.0,27.4 413.8,38.4 377.6,48.5 341.5,57.6 305.3,65.2 269.1,71.2 232.9,83.3 196.7,87.2 160.5,96.6 124.4,97.3 88.2,96.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,190.1 124.4,197.2 160.5,195.8 196.7,174.5 232.9,171.2 269.1,160.8 305.3,153.3 341.5,147.6 377.6,146.2 413.8,145.9 450.0,143.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,103.1 124.4,109.4 160.5,108.1 196.7,105.0 232.9,104.1 269.1,95.8 305.3,93.2 341.5,89.6 377.6,81.1 413.8,72.0 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,106.9 124.4,106.2 160.5,103.2 196.7,97.4 232.9,96.7 269.1,88.4 305.3,84.4 341.5,73.5 377.6,66.0 413.8,55.9 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.6 124.4,103.0 160.5,100.6 196.7,91.5 232.9,88.5 269.1,79.9 305.3,72.1 341.5,63.1 377.6,55.8 413.8,45.9 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,96.1 124.4,97.3 160.5,96.6 196.7,87.2 232.9,83.3 269.1,71.2 305.3,65.2 341.5,57.6 377.6,48.5 413.8,38.4 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.34 µs | 9.44 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 9.41 µs | 6.6 µs | 14.5 µs | 16.5 µs |
| D57 | 2.47 ns | 5.1 µs | 6.78 µs | 8.41 µs | 9.91 µs |
| D76 | 3.17 ns | 5.54 µs | 7.93 µs | 9.93 µs | 12.7 µs |
| D115 | 10.3 ns | 12.7 µs | 10.6 µs | 22.6 µs | 27.7 µs |
| D153 | 22.7 ns | 7.6 µs | 14.7 µs | 22.6 µs | 32.9 µs |
| D230 | 52.5 ns | 14.3 µs | 22.7 µs | 43 µs | 83.7 µs |
| D307 | 97 ns | 15.9 µs | 49.3 µs | 78.1 µs | 123 µs |
| D462 | 159 ns | 26.2 µs | 80.3 µs | 165 µs | 227 µs |
| D616 | 169 ns | 39 µs | 124 µs | 264 µs | 443 µs |
| D924 | 170 ns | 81.6 µs | 262 µs | 548 µs | 889 µs |
| D1232 | 213 ns | 141 µs | 351 µs | 914 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.6 124.4,198.8 160.5,195.7 196.7,181.1 232.9,171.2 269.1,160.9 305.3,153.2 341.5,147.1 377.6,146.4 413.8,146.3 450.0,143.5 450.0,25.5 413.8,40.0 377.6,48.7 341.5,57.0 305.3,64.6 269.1,69.4 232.9,80.9 196.7,83.1 160.5,92.7 124.4,95.8 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.6 124.4,198.8 160.5,195.7 196.7,181.1 232.9,171.2 269.1,160.9 305.3,153.2 341.5,147.1 377.6,146.4 413.8,146.3 450.0,143.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.6 88.2,96.5 124.4,104.1 160.5,103.1 196.7,92.8 232.9,99.1 269.1,91.3 305.3,90.0 341.5,83.7 377.6,78.8 413.8,69.7 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,100.9 124.4,100.5 160.5,98.6 196.7,95.0 232.9,90.9 269.1,85.6 305.3,75.9 341.5,69.9 377.6,64.5 413.8,55.2 450.0,51.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,97.9 160.5,95.8 196.7,85.6 232.9,85.6 269.1,77.6 305.3,70.2 341.5,61.0 377.6,55.1 413.8,46.0 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,95.8 160.5,92.7 196.7,83.1 232.9,80.9 269.1,69.4 305.3,64.6 341.5,57.0 377.6,48.7 413.8,40.0 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 3.83 µs | 5.33 µs | 6.34 µs | 6.91 µs |
| D38 | 4.36 ns | 5.32 µs | 3.9 µs | 8.4 µs | 9.64 µs |
| D57 | 2.81 ns | 3.08 µs | 4.06 µs | 5.52 µs | 8.77 µs |
| D76 | 3.52 ns | 3.37 µs | 5.18 µs | 6.61 µs | 8.94 µs |
| D115 | 17.8 ns | 4.52 µs | 8.65 µs | 14.2 µs | 19.3 µs |
| D153 | 22.7 ns | 4.8 µs | 9.08 µs | 17 µs | 26.7 µs |
| D230 | 52.6 ns | 10 µs | 18.2 µs | 35.3 µs | 70.2 µs |
| D307 | 88.5 ns | 11.5 µs | 23.9 µs | 65.9 µs | 117 µs |
| D462 | 157 ns | 16.2 µs | 59.7 µs | 138 µs | 213 µs |
| D616 | 163 ns | 31.9 µs | 107 µs | 247 µs | 450 µs |
| D924 | 155 ns | 66.8 µs | 244 µs | 551 µs | 1 ms |
| D1232 | 201 ns | 127 µs | 362 µs | 1.04 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,191.7 124.4,197.2 160.5,194.4 196.7,174.3 232.9,171.2 269.1,160.8 305.3,154.4 341.5,147.3 377.6,146.8 413.8,147.5 450.0,144.2 450.0,27.5 413.8,38.5 377.6,48.5 341.5,57.8 305.3,65.2 269.1,71.5 232.9,83.5 196.7,87.6 160.5,97.1 124.4,97.3 88.2,96.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,191.7 124.4,197.2 160.5,194.4 196.7,174.3 232.9,171.2 269.1,160.8 305.3,154.4 341.5,147.3 377.6,146.8 413.8,147.5 450.0,144.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.6 88.2,103.5 124.4,110.3 160.5,109.2 196.7,105.6 232.9,104.8 269.1,95.7 305.3,93.9 341.5,89.7 377.6,81.3 413.8,72.1 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.5 88.2,107.4 124.4,106.9 160.5,103.9 196.7,97.5 232.9,96.9 269.1,88.3 305.3,84.9 341.5,73.5 377.6,66.3 413.8,56.1 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,97.9 124.4,103.1 160.5,100.8 196.7,91.3 232.9,89.1 269.1,80.1 305.3,72.3 341.5,63.2 377.6,55.9 413.8,46.0 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,96.2 124.4,97.3 160.5,97.1 196.7,87.6 232.9,83.5 269.1,71.5 305.3,65.2 341.5,57.8 377.6,48.5 413.8,38.5 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.35 µs | 9.44 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 9.42 µs | 6.61 µs | 14.5 µs | 16.5 µs |
| D57 | 12.2 ns | 5.15 µs | 6.8 µs | 8.43 µs | 9.91 µs |
| D76 | 12 ns | 5.58 µs | 7.97 µs | 9.93 µs | 12.8 µs |
| D115 | 10.3 ns | 12.7 µs | 10.2 µs | 22.8 µs | 27.9 µs |
| D153 | 22.7 ns | 7.63 µs | 14.6 µs | 22.8 µs | 33 µs |
| D230 | 53.1 ns | 14.2 µs | 22.6 µs | 43 µs | 83.9 µs |
| D307 | 87.5 ns | 16 µs | 48.7 µs | 78.5 µs | 123 µs |
| D462 | 148 ns | 25.2 µs | 80.1 µs | 165 µs | 228 µs |
| D616 | 170 ns | 39.2 µs | 124 µs | 264 µs | 444 µs |
| D924 | 169 ns | 81.7 µs | 263 µs | 547 µs | 889 µs |
| D1232 | 219 ns | 141 µs | 351 µs | 912 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.6 124.4,178.9 160.5,179.1 196.7,181.1 232.9,171.2 269.1,160.7 305.3,154.5 341.5,148.0 377.6,146.3 413.8,146.4 450.0,143.2 450.0,25.6 413.8,40.0 377.6,48.6 341.5,56.9 305.3,64.6 269.1,69.3 232.9,80.9 196.7,83.0 160.5,92.7 124.4,95.8 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.6 124.4,178.9 160.5,179.1 196.7,181.1 232.9,171.2 269.1,160.7 305.3,154.5 341.5,148.0 377.6,146.3 413.8,146.4 450.0,143.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,96.5 124.4,104.0 160.5,102.9 196.7,92.8 232.9,99.1 269.1,91.3 305.3,89.9 341.5,84.2 377.6,78.8 413.8,69.6 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,100.8 124.4,100.5 160.5,98.5 196.7,95.5 232.9,91.0 269.1,85.6 305.3,76.1 341.5,69.9 377.6,64.5 413.8,55.2 450.0,51.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,97.8 160.5,95.8 196.7,85.5 232.9,85.5 269.1,77.6 305.3,70.1 341.5,60.9 377.6,55.1 413.8,46.1 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,95.8 160.5,92.7 196.7,83.0 232.9,80.9 269.1,69.3 305.3,64.6 341.5,56.9 377.6,48.6 413.8,40.0 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 7.64 µs | 9.94 µs | 11.7 µs | 12.7 µs |
| D38 | 4.37 ns | 9.93 µs | 7.41 µs | 15.6 µs | 17.7 µs |
| D57 | 2.81 ns | 4.16 µs | 5.39 µs | 7.28 µs | 8.42 µs |
| D76 | 3.61 ns | 4.44 µs | 7.02 µs | 8.62 µs | 11.3 µs |
| D115 | 17.1 ns | 5.96 µs | 10.8 µs | 17.4 µs | 22.3 µs |
| D153 | 22.4 ns | 6.43 µs | 11.8 µs | 21.3 µs | 31.3 µs |
| D230 | 57.9 ns | 12.6 µs | 22 µs | 41.1 µs | 80.4 µs |
| D307 | 87.1 ns | 14.5 µs | 28.3 µs | 74 µs | 127 µs |
| D462 | 144 ns | 20.6 µs | 67.3 µs | 153 µs | 231 µs |
| D616 | 159 ns | 36.8 µs | 119 µs | 274 µs | 483 µs |
| D924 | 138 ns | 76.6 µs | 271 µs | 593 µs | 1.07 ms |
| D1232 | 209 ns | 141 µs | 390 µs | 1.11 ms | 2.59 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,191.7 124.4,197.2 160.5,194.1 196.7,174.8 232.9,171.4 269.1,159.6 305.3,154.6 341.5,148.4 377.6,147.1 413.8,148.8 450.0,143.7 450.0,26.7 413.8,37.7 377.6,47.6 341.5,56.7 305.3,64.2 269.1,69.9 232.9,81.5 196.7,85.7 160.5,94.2 124.4,97.8 88.2,88.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,191.7 124.4,197.2 160.5,194.1 196.7,174.8 232.9,171.4 269.1,159.6 305.3,154.6 341.5,148.4 377.6,147.1 413.8,148.8 450.0,143.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,95.8 124.4,106.6 160.5,105.8 196.7,102.1 232.9,101.2 269.1,92.8 305.3,91.1 341.5,86.7 377.6,79.6 413.8,70.4 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,99.4 124.4,103.4 160.5,100.1 196.7,94.8 232.9,93.7 269.1,85.9 305.3,82.8 341.5,72.1 377.6,65.0 413.8,54.8 450.0,50.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,90.2 124.4,99.7 160.5,97.6 196.7,88.8 232.9,86.4 269.1,78.2 305.3,70.9 341.5,61.8 377.6,54.7 413.8,45.1 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,88.6 124.4,97.8 160.5,94.2 196.7,85.7 232.9,81.5 269.1,69.9 305.3,64.2 341.5,56.7 377.6,47.6 413.8,37.7 450.0,26.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 7.36 µs | 9.56 µs | 11.2 µs | 12.1 µs |
| D38 | 3.74 ns | 9.56 µs | 6.63 µs | 14.6 µs | 16.5 µs |
| D57 | 2.88 µs | 5.27 µs | 7.09 µs | 8.71 µs | 10.2 µs |
| D76 | 2.85 µs | 5.66 µs | 8.26 µs | 10.3 µs | 13.1 µs |
| D115 | 5.42 µs | 13 µs | 10.7 µs | 23.5 µs | 28.4 µs |
| D153 | 2.85 µs | 7.94 µs | 15.2 µs | 23.3 µs | 33.6 µs |
| D230 | 3.15 µs | 14.8 µs | 23.3 µs | 43.9 µs | 84.7 µs |
| D307 | 3.43 µs | 16.6 µs | 50.3 µs | 79.2 µs | 126 µs |
| D462 | 3.62 µs | 25.6 µs | 81.5 µs | 167 µs | 230 µs |
| D616 | 3.71 µs | 39.9 µs | 125 µs | 266 µs | 448 µs |
| D924 | 3.72 µs | 82.1 µs | 265 µs | 554 µs | 896 µs |
| D1232 | 2.62 µs | 143 µs | 356 µs | 920 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,193.6 124.4,111.2 160.5,111.3 196.7,103.3 232.9,111.3 269.1,110.1 305.3,109.0 341.5,108.3 377.6,108.0 413.8,108.0 450.0,112.3 450.0,25.5 413.8,39.9 377.6,48.5 341.5,56.8 305.3,64.3 269.1,69.2 232.9,80.7 196.7,82.8 160.5,92.4 124.4,95.5 88.2,89.5 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,193.6 124.4,111.2 160.5,111.3 196.7,103.3 232.9,111.3 269.1,110.1 305.3,109.0 341.5,108.3 377.6,108.0 413.8,108.0 450.0,112.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,96.3 124.4,103.7 160.5,102.8 196.7,92.5 232.9,98.6 269.1,90.8 305.3,89.4 341.5,84.1 377.6,78.5 413.8,69.6 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,100.8 124.4,100.0 160.5,98.1 196.7,94.9 232.9,90.5 269.1,85.2 305.3,75.7 341.5,69.7 377.6,64.4 413.8,55.0 450.0,51.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,91.0 124.4,97.4 160.5,95.4 196.7,85.1 232.9,85.2 269.1,77.4 305.3,70.0 341.5,60.8 377.6,55.0 413.8,45.9 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,89.5 124.4,95.5 160.5,92.4 196.7,82.8 232.9,80.7 269.1,69.2 305.3,64.3 341.5,56.8 377.6,48.5 413.8,39.9 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.96 ns | 3 µs | 3.23 µs | 3.8 µs | 4.15 µs |
| D38 | 4.05 ns | 3.23 µs | 1.81 µs | 5.09 µs | 5.86 µs |
| D57 | 195 ns | 291 ns | 297 ns | 336 ns | 399 ns |
| D76 | 200 ns | 278 ns | 333 ns | 402 ns | 457 ns |
| D115 | 347 ns | 532 ns | 554 ns | 776 ns | 847 ns |
| D153 | 334 ns | 497 ns | 612 ns | 749 ns | 844 ns |
| D230 | 566 ns | 858 ns | 929 ns | 1.22 µs | 1.89 µs |
| D307 | 898 ns | 1.15 µs | 1.44 µs | 2.01 µs | 2.75 µs |
| D462 | 980 ns | 1.5 µs | 2.09 µs | 3.23 µs | 3.81 µs |
| D616 | 1.2 µs | 1.92 µs | 2.61 µs | 4 µs | 5.94 µs |
| D924 | 1.48 µs | 2.72 µs | 4.36 µs | 6.77 µs | 9.99 µs |
| D1232 | 1.23 µs | 4.06 µs | 5.72 µs | 10.8 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,185.7 124.4,118.4 160.5,118.0 196.7,108.4 232.9,109.0 269.1,99.9 305.3,91.9 341.5,90.4 377.6,86.8 413.8,83.2 450.0,86.5 450.0,30.7 413.8,50.0 377.6,59.1 341.5,66.7 305.3,72.4 269.1,78.9 232.9,92.9 196.7,92.9 160.5,103.6 124.4,106.0 88.2,59.3 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,185.7 124.4,118.4 160.5,118.0 196.7,108.4 232.9,109.0 269.1,99.9 305.3,91.9 341.5,90.4 377.6,86.8 413.8,83.2 450.0,86.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,70.9 88.2,69.6 124.4,111.5 160.5,112.2 196.7,101.0 232.9,102.2 269.1,92.7 305.3,87.6 341.5,83.0 377.6,78.7 413.8,72.6 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,79.7 124.4,111.1 160.5,109.1 196.7,100.3 232.9,98.5 269.1,91.3 305.3,83.6 341.5,77.2 377.6,73.3 413.8,64.4 450.0,59.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,61.7 124.4,108.9 160.5,105.8 196.7,94.4 232.9,95.0 269.1,86.6 305.3,77.8 341.5,69.6 377.6,65.9 413.8,56.8 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,59.3 124.4,106.0 160.5,103.6 196.7,92.9 232.9,92.9 269.1,78.9 305.3,72.4 341.5,66.7 377.6,59.1 413.8,50.0 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 147 ns | 185 ns | 203 ns | 205 ns |
| D38 | 4.36 ns | 185 ns | 175 ns | 205 ns | 207 ns |
| D57 | 314 ns | 392 ns | 403 ns | 450 ns | 494 ns |
| D76 | 303 ns | 385 ns | 451 ns | 513 ns | 589 ns |
| D115 | 605 ns | 712 ns | 729 ns | 1.01 µs | 1.06 µs |
| D153 | 574 ns | 710 ns | 827 ns | 970 ns | 1.07 µs |
| D230 | 948 ns | 1.21 µs | 1.28 µs | 1.64 µs | 2.26 µs |
| D307 | 1.46 µs | 1.64 µs | 1.97 µs | 2.61 µs | 3.32 µs |
| D462 | 1.55 µs | 1.99 µs | 2.68 µs | 3.82 µs | 4.4 µs |
| D616 | 1.91 µs | 2.48 µs | 3.17 µs | 4.73 µs | 6.75 µs |
| D924 | 2.42 µs | 3.6 µs | 5.28 µs | 7.73 µs | 11.1 µs |
| D1232 | 1.94 µs | 5.28 µs | 6.74 µs | 12.4 µs | 32.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,184.4 124.4,110.1 160.5,110.7 196.7,98.7 232.9,99.6 269.1,90.9 305.3,83.4 341.5,82.4 377.6,78.8 413.8,74.6 450.0,78.5 450.0,29.7 413.8,48.2 377.6,56.8 341.5,64.2 305.3,69.1 269.1,75.9 232.9,88.9 196.7,89.1 160.5,99.2 124.4,102.3 88.2,117.3 52.0,117.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,184.4 124.4,110.1 160.5,110.7 196.7,98.7 232.9,99.6 269.1,90.9 305.3,83.4 341.5,82.4 377.6,78.8 413.8,74.6 450.0,78.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.3 88.2,119.3 124.4,106.3 160.5,106.6 196.7,95.9 232.9,95.9 269.1,86.7 305.3,81.5 341.5,78.0 377.6,74.2 413.8,67.8 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,120.3 124.4,105.8 160.5,103.8 196.7,95.5 232.9,93.3 269.1,85.8 305.3,78.2 341.5,72.9 377.6,69.9 413.8,61.1 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.7 88.2,117.6 124.4,103.9 160.5,101.6 196.7,89.9 232.9,90.5 269.1,81.4 305.3,73.3 341.5,66.7 377.6,63.0 413.8,54.5 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,117.3 124.4,102.3 160.5,99.2 196.7,89.1 232.9,88.9 269.1,75.9 305.3,69.1 341.5,64.2 377.6,56.8 413.8,48.2 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
