# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.17 ns | 23.2 µs | 27.7 µs | 32.4 µs | 39.5 µs |
| D38 | 4.31 µs | 7.65 µs | 7.21 µs | 12.4 µs | 15.1 µs |
| D57 | 5.02 µs | 9 µs | 11 µs | 14.6 µs | 17.5 µs |
| D76 | 2.93 µs | 8.65 µs | 15.9 µs | 17.6 µs | 22.3 µs |
| D115 | 5.19 µs | 12.1 µs | 23.8 µs | 32.6 µs | 41.6 µs |
| D153 | 4.83 µs | 14.2 µs | 27.5 µs | 40.5 µs | 60.3 µs |
| D230 | 4.74 µs | 22.3 µs | 41.8 µs | 74.2 µs | 121 µs |
| D307 | 5.26 µs | 28.8 µs | 60 µs | 121 µs | 188 µs |
| D462 | 5.28 µs | 44.2 µs | 133 µs | 238 µs | 372 µs |
| D616 | 4.94 µs | 65.3 µs | 214 µs | 444 µs | 703 µs |
| D924 | 4.93 µs | 124 µs | 444 µs | 931 µs | 1.66 ms |
| D1232 | 5.62 µs | 218 µs | 608 µs | 1.64 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,106.1 124.4,104.3 160.5,111.0 196.7,103.9 232.9,104.7 269.1,105.0 305.3,103.7 341.5,103.7 377.6,104.5 413.8,104.5 450.0,102.9 450.0,23.1 413.8,32.3 377.6,43.0 341.5,50.8 305.3,59.3 269.1,64.7 232.9,73.4 196.7,78.0 160.5,85.8 124.4,88.8 88.2,90.6 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,106.1 124.4,104.3 160.5,111.0 196.7,103.9 232.9,104.7 269.1,105.0 305.3,103.7 341.5,103.7 377.6,104.5 413.8,104.5 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.0 124.4,97.0 160.5,97.5 196.7,93.3 232.9,91.3 269.1,85.8 305.3,82.6 341.5,77.3 377.6,72.4 413.8,64.5 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.8 124.4,94.6 160.5,89.9 196.7,84.9 232.9,83.2 269.1,78.0 305.3,73.5 341.5,63.6 377.6,57.7 413.8,48.7 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.1 124.4,91.0 160.5,88.7 196.7,81.0 232.9,78.3 269.1,70.8 305.3,64.8 341.5,56.4 377.6,48.6 413.8,39.5 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.6 124.4,88.8 160.5,85.8 196.7,78.0 232.9,73.4 269.1,64.7 305.3,59.3 341.5,50.8 377.6,43.0 413.8,32.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 31 µs | 33.2 µs | 39 µs | 38.4 µs |
| D38 | 25.7 µs | 35.5 µs | 41.3 µs | 57.3 µs | 61.5 µs |
| D57 | 3.74 µs | 4.82 µs | 4.97 µs | 6.04 µs | 7.49 µs |
| D76 | 1.87 µs | 4.53 µs | 6.31 µs | 7.72 µs | 10.1 µs |
| D115 | 6.76 µs | 9.4 µs | 12.1 µs | 16.7 µs | 21.2 µs |
| D153 | 6.06 µs | 9.92 µs | 14.6 µs | 21.4 µs | 29.9 µs |
| D230 | 8.34 µs | 13.9 µs | 27 µs | 46.4 µs | 69.6 µs |
| D307 | 12.8 µs | 27.9 µs | 47.1 µs | 83.9 µs | 138 µs |
| D462 | 13 µs | 38.2 µs | 89.7 µs | 161 µs | 262 µs |
| D616 | 21.2 µs | 75.6 µs | 182 µs | 332 µs | 556 µs |
| D924 | 32.2 µs | 155 µs | 398 µs | 798 µs | 1.44 ms |
| D1232 | 43.1 µs | 271 µs | 612 µs | 1.54 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.5 88.2,139.5 124.4,181.4 160.5,196.4 196.7,168.5 232.9,170.9 269.1,163.9 305.3,154.6 341.5,154.3 377.6,143.7 413.8,134.6 450.0,128.3 450.0,35.7 413.8,52.1 377.6,72.7 341.5,89.1 305.3,103.1 269.1,117.9 232.9,136.2 196.7,143.7 160.5,159.8 124.4,166.3 88.2,120.6 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.5 88.2,139.5 124.4,181.4 160.5,196.4 196.7,168.5 232.9,170.9 269.1,163.9 305.3,154.6 341.5,154.3 377.6,143.7 413.8,134.6 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,132.5 124.4,175.9 160.5,177.2 196.7,161.3 232.9,160.2 269.1,152.9 305.3,137.7 341.5,130.9 377.6,116.1 413.8,100.5 450.0,88.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,129.2 124.4,175.2 160.5,170.0 196.7,155.9 232.9,151.8 269.1,138.4 305.3,126.4 341.5,112.4 377.6,97.0 413.8,80.0 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,122.1 124.4,170.9 160.5,165.6 196.7,148.8 232.9,143.4 269.1,126.7 305.3,113.8 341.5,99.6 377.6,83.9 413.8,64.9 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,120.6 124.4,166.3 160.5,159.8 196.7,143.7 232.9,136.2 269.1,117.9 305.3,103.1 341.5,89.1 377.6,72.7 413.8,52.1 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 23.2 µs | 27.7 µs | 32.4 µs | 39.4 µs |
| D38 | 4.28 µs | 7.61 µs | 7.18 µs | 12.4 µs | 15.2 µs |
| D57 | 5.01 µs | 9.11 µs | 11 µs | 14.5 µs | 17.5 µs |
| D76 | 2.89 µs | 8.7 µs | 15.9 µs | 17.6 µs | 22.1 µs |
| D115 | 5.13 µs | 12 µs | 24 µs | 31.4 µs | 40.6 µs |
| D153 | 4.74 µs | 14.2 µs | 26.7 µs | 40.6 µs | 60.6 µs |
| D230 | 4.71 µs | 22.3 µs | 41.7 µs | 73.5 µs | 122 µs |
| D307 | 5.15 µs | 29.3 µs | 59.3 µs | 120 µs | 189 µs |
| D462 | 5.24 µs | 45.2 µs | 130 µs | 239 µs | 373 µs |
| D616 | 4.84 µs | 65.2 µs | 214 µs | 443 µs | 702 µs |
| D924 | 4.96 µs | 123 µs | 445 µs | 935 µs | 1.66 ms |
| D1232 | 5.55 µs | 215 µs | 611 µs | 1.64 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,106.3 124.4,104.3 160.5,111.1 196.7,104.0 232.9,105.0 269.1,105.1 305.3,104.0 341.5,103.7 377.6,104.7 413.8,104.4 450.0,103.0 450.0,23.1 413.8,32.3 377.6,43.0 341.5,50.8 305.3,59.3 269.1,64.7 232.9,73.4 196.7,78.3 160.5,85.9 124.4,88.8 88.2,90.5 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,106.3 124.4,104.3 160.5,111.1 196.7,104.0 232.9,105.0 269.1,105.1 305.3,104.0 341.5,103.7 377.6,104.7 413.8,104.4 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.3 88.2,99.1 124.4,96.9 160.5,97.4 196.7,93.4 232.9,91.4 269.1,85.7 305.3,82.4 341.5,77.0 377.6,72.4 413.8,64.5 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.8 124.4,94.6 160.5,89.9 196.7,84.9 232.9,83.5 269.1,78.0 305.3,73.6 341.5,63.9 377.6,57.7 413.8,48.6 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.0 124.4,91.1 160.5,88.7 196.7,81.5 232.9,78.3 269.1,71.0 305.3,64.8 341.5,56.4 377.6,48.7 413.8,39.4 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.5 124.4,88.8 160.5,85.9 196.7,78.3 232.9,73.4 269.1,64.7 305.3,59.3 341.5,50.8 377.6,43.0 413.8,32.3 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 23 µs | 27.1 µs | 31.8 µs | 38.7 µs |
| D38 | 3.74 ns | 29 µs | 33.7 µs | 42.9 µs | 51.3 µs |
| D57 | 2.11 ns | 6.03 µs | 7.28 µs | 8.91 µs | 11.6 µs |
| D76 | 1.04 ns | 6.2 µs | 9.44 µs | 11.7 µs | 15.2 µs |
| D115 | 12.4 ns | 13.7 µs | 18.6 µs | 24 µs | 33.6 µs |
| D153 | 16.5 ns | 14.8 µs | 22.2 µs | 33.4 µs | 43.7 µs |
| D230 | 28 ns | 21.5 µs | 41.6 µs | 68.3 µs | 94.9 µs |
| D307 | 52.4 ns | 39 µs | 66.7 µs | 112 µs | 173 µs |
| D462 | 75.4 ns | 58.1 µs | 125 µs | 198 µs | 317 µs |
| D616 | 80.4 ns | 113 µs | 245 µs | 432 µs | 664 µs |
| D924 | 101 ns | 232 µs | 531 µs | 948 µs | 1.6 ms |
| D1232 | 155 ns | 385 µs | 767 µs | 1.75 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,209.5 196.7,178.7 232.9,175.2 269.1,168.7 305.3,160.9 341.5,156.4 377.6,155.6 413.8,152.7 450.0,147.4 450.0,24.5 413.8,32.7 377.6,43.7 341.5,52.8 305.3,60.4 269.1,67.8 232.9,77.4 196.7,80.7 160.5,90.5 124.4,93.8 88.2,75.4 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,209.5 196.7,178.7 232.9,175.2 269.1,168.7 305.3,160.9 341.5,156.4 377.6,155.6 413.8,152.7 450.0,147.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,82.5 124.4,102.0 160.5,101.6 196.7,91.8 232.9,90.8 269.1,86.2 305.3,78.8 341.5,73.9 377.6,65.6 413.8,56.7 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,80.6 124.4,99.7 160.5,96.4 196.7,88.0 232.9,85.8 269.1,78.0 305.3,72.2 341.5,64.4 377.6,56.0 413.8,46.4 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,77.6 124.4,97.1 160.5,93.7 196.7,84.8 232.9,80.8 269.1,71.9 305.3,65.7 341.5,58.7 377.6,49.0 413.8,39.2 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,75.4 124.4,93.8 160.5,90.5 196.7,80.7 232.9,77.4 269.1,67.8 305.3,60.4 341.5,52.8 377.6,43.7 413.8,32.7 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.63 µs | 2.57 µs | 2.93 µs | 3.2 µs |
| D38 | 5.42 µs | 8.94 µs | 6 µs | 10.7 µs | 13.5 µs |
| D57 | 4.2 µs | 7.64 µs | 9.54 µs | 13 µs | 5.07 µs |
| D76 | 2.57 µs | 7.55 µs | 14 µs | 15.5 µs | 19.7 µs |
| D115 | 4.28 µs | 10.3 µs | 20.9 µs | 28.9 µs | 37.1 µs |
| D153 | 3.92 µs | 12.4 µs | 21.5 µs | 36.8 µs | 54.4 µs |
| D230 | 3.9 µs | 19.7 µs | 37.8 µs | 68.4 µs | 113 µs |
| D307 | 4.27 µs | 25.7 µs | 48.7 µs | 112 µs | 176 µs |
| D462 | 3.19 µs | 35.6 µs | 111 µs | 208 µs | 319 µs |
| D616 | 4.24 µs | 58.8 µs | 200 µs | 415 µs | 667 µs |
| D924 | 4.04 µs | 114 µs | 416 µs | 886 µs | 1.57 ms |
| D1232 | 4.63 µs | 198 µs | 575 µs | 1.57 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,103.3 124.4,106.5 160.5,112.6 196.7,106.2 232.9,107.3 269.1,107.4 305.3,106.3 341.5,109.9 377.6,106.4 413.8,106.9 450.0,105.3 450.0,23.6 413.8,33.0 377.6,43.6 341.5,52.7 305.3,60.1 269.1,65.6 232.9,74.7 196.7,79.4 160.5,87.3 124.4,104.1 88.2,92.0 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,103.3 124.4,106.5 160.5,112.6 196.7,106.2 232.9,107.3 269.1,107.4 305.3,106.3 341.5,109.9 377.6,106.4 413.8,106.9 450.0,105.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,97.1 124.4,99.1 160.5,99.2 196.7,95.4 232.9,93.0 269.1,87.3 305.3,84.0 341.5,80.0 377.6,73.7 413.8,65.5 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,102.1 124.4,96.3 160.5,91.6 196.7,86.6 232.9,86.2 269.1,79.2 305.3,76.1 341.5,65.9 377.6,58.6 413.8,49.5 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,94.9 124.4,92.5 160.5,90.3 196.7,82.6 232.9,79.5 269.1,71.9 305.3,65.7 341.5,58.1 377.6,49.5 413.8,40.1 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,92.0 124.4,104.1 160.5,87.3 196.7,79.4 232.9,74.7 269.1,65.6 305.3,60.1 341.5,52.7 377.6,43.6 413.8,33.0 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.81 µs | 8.83 µs | 10.3 µs | 11.3 µs |
| D38 | 3.74 ns | 9.4 µs | 11 µs | 13.9 µs | 15 µs |
| D57 | 608 ns | 5.98 µs | 6.91 µs | 8.45 µs | 11 µs |
| D76 | 285 ns | 5.97 µs | 8.56 µs | 11.1 µs | 14.8 µs |
| D115 | 1.21 µs | 12.9 µs | 17.5 µs | 24.9 µs | 33 µs |
| D153 | 1.03 µs | 13.6 µs | 22.1 µs | 33.3 µs | 48.1 µs |
| D230 | 1.39 µs | 20.1 µs | 41.2 µs | 78.3 µs | 118 µs |
| D307 | 2.26 µs | 42 µs | 77.4 µs | 143 µs | 241 µs |
| D462 | 2.4 µs | 59.2 µs | 154 µs | 287 µs | 473 µs |
| D616 | 3.76 µs | 122 µs | 315 µs | 593 µs | 1.02 ms |
| D924 | 5.49 µs | 258 µs | 710 µs | 1.47 ms | 2.67 ms |
| D1232 | 7.6 µs | 458 µs | 1.11 ms | 2.86 ms | 5.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,130.5 160.5,139.9 196.7,121.9 232.9,123.9 269.1,120.2 305.3,114.2 341.5,113.4 377.6,107.9 413.8,103.2 450.0,99.1 450.0,16.9 413.8,26.4 377.6,38.3 341.5,47.9 305.3,56.2 269.1,65.1 232.9,76.2 196.7,80.9 160.5,90.8 124.4,94.5 88.2,90.7 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,130.5 160.5,139.9 196.7,121.9 232.9,123.9 269.1,120.2 305.3,114.2 341.5,113.4 377.6,107.9 413.8,103.2 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,96.5 124.4,102.1 160.5,102.1 196.7,92.5 232.9,91.9 269.1,87.0 305.3,77.9 341.5,73.6 377.6,64.7 413.8,55.4 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,94.5 124.4,100.3 160.5,97.6 196.7,88.7 232.9,85.9 269.1,78.1 305.3,70.3 341.5,61.8 377.6,52.9 413.8,42.8 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,91.6 124.4,97.8 160.5,94.4 196.7,84.4 232.9,80.8 269.1,70.2 305.3,62.7 341.5,54.1 377.6,45.0 413.8,33.8 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,90.7 124.4,94.5 160.5,90.8 196.7,80.9 232.9,76.2 269.1,65.1 305.3,56.2 341.5,47.9 377.6,38.3 413.8,26.4 450.0,16.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.66 ns | 4.27 µs | 5.5 µs | 6.41 µs | 6.98 µs |
| D38 | 4.98 ns | 5.96 µs | 6.75 µs | 8.6 µs | 8.93 µs |
| D57 | 2.81 ns | 3.62 µs | 4.38 µs | 5.13 µs | 8.78 µs |
| D76 | 2.16 ns | 3.58 µs | 5.5 µs | 6.96 µs | 9.38 µs |
| D115 | 16.8 ns | 4.69 µs | 10.1 µs | 13.8 µs | 17.9 µs |
| D153 | 22.7 ns | 5.15 µs | 9.72 µs | 18.2 µs | 30.1 µs |
| D230 | 48.8 ns | 8.91 µs | 18.2 µs | 39.3 µs | 69.9 µs |
| D307 | 95.9 ns | 12.3 µs | 25.7 µs | 67.6 µs | 115 µs |
| D462 | 154 ns | 16 µs | 65.9 µs | 133 µs | 219 µs |
| D616 | 145 ns | 32.2 µs | 127 µs | 269 µs | 450 µs |
| D924 | 167 ns | 68.9 µs | 268 µs | 611 µs | 1.12 ms |
| D1232 | 403 ns | 127 µs | 402 µs | 1.13 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,200.4 196.7,175.0 232.9,171.2 269.1,161.8 305.3,153.4 341.5,147.5 377.6,148.2 413.8,146.5 450.0,135.6 450.0,27.5 413.8,37.1 377.6,48.5 341.5,57.4 305.3,65.4 269.1,71.6 232.9,82.0 196.7,88.5 160.5,96.5 124.4,97.3 88.2,97.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.1 124.4,197.2 160.5,200.4 196.7,175.0 232.9,171.2 269.1,161.8 305.3,153.4 341.5,147.5 377.6,148.2 413.8,146.5 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,102.1 124.4,108.3 160.5,108.4 196.7,105.1 232.9,104.0 269.1,97.2 305.3,93.1 341.5,89.9 377.6,81.2 413.8,71.8 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,100.6 124.4,106.0 160.5,103.1 196.7,95.6 232.9,96.1 269.1,88.3 305.3,84.0 341.5,72.3 377.6,64.1 413.8,54.9 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,97.6 124.4,104.0 160.5,100.2 196.7,91.7 232.9,88.3 269.1,78.7 305.3,72.0 341.5,63.6 377.6,54.9 413.8,44.7 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,97.1 124.4,97.3 160.5,96.5 196.7,88.5 232.9,82.0 269.1,71.6 305.3,65.4 341.5,57.4 377.6,48.5 413.8,37.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 7.91 µs | 9.42 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 14.5 µs | 15.3 µs |
| D57 | 2.49 ns | 5.78 µs | 6.97 µs | 7.94 µs | 10.1 µs |
| D76 | 2.28 ns | 5.72 µs | 8 µs | 10 µs | 12.8 µs |
| D115 | 10.9 ns | 12.7 µs | 11.3 µs | 20.7 µs | 25.3 µs |
| D153 | 22.7 ns | 7.45 µs | 15 µs | 22.3 µs | 35.1 µs |
| D230 | 48.7 ns | 12.7 µs | 22.3 µs | 46.9 µs | 78.4 µs |
| D307 | 97 ns | 17.2 µs | 49.1 µs | 79.5 µs | 123 µs |
| D462 | 159 ns | 24.6 µs | 85.5 µs | 155 µs | 231 µs |
| D616 | 145 ns | 39.4 µs | 141 µs | 288 µs | 445 µs |
| D924 | 181 ns | 82.2 µs | 290 µs | 608 µs | 995 µs |
| D1232 | 390 ns | 142 µs | 399 µs | 993 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,192.7 124.4,198.7 160.5,199.8 196.7,180.4 232.9,171.2 269.1,161.8 305.3,153.2 341.5,147.1 377.6,148.2 413.8,145.5 450.0,136.0 450.0,25.5 413.8,38.6 377.6,48.6 341.5,56.8 305.3,64.6 269.1,70.2 232.9,80.1 196.7,84.2 160.5,92.7 124.4,95.6 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,192.7 124.4,198.7 160.5,199.8 196.7,180.4 232.9,171.2 269.1,161.8 305.3,153.2 341.5,147.1 377.6,148.2 413.8,145.5 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,102.5 160.5,102.6 196.7,92.8 232.9,99.4 269.1,92.8 305.3,89.0 341.5,84.5 377.6,78.7 413.8,69.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.0 124.4,100.2 160.5,98.5 196.7,94.2 232.9,90.6 269.1,85.7 305.3,76.0 341.5,69.1 377.6,62.9 413.8,53.9 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.6 160.5,95.7 196.7,86.7 232.9,85.8 269.1,76.5 305.3,70.0 341.5,61.7 377.6,54.0 413.8,44.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,95.6 160.5,92.7 196.7,84.2 232.9,80.1 269.1,70.2 305.3,64.6 341.5,56.8 377.6,48.6 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `neg`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 0.622 ns | 0.702 ns | 0.622 ns | 0.703 ns | 0.703 ns |
| D38 | 1.33 ns | 1.45 ns | 1.33 ns | 1.45 ns | 1.32 ns |
| D57 | 1.74 ns | 1.74 ns | 1.87 ns | 1.87 ns | 1.87 ns |
| D76 | 1.33 ns | 2.1 ns | 2.16 ns | 2.1 ns | 2.63 ns |
| D115 | 3.17 ns | 3.17 ns | 2.86 ns | 3.25 ns | 3.25 ns |
| D153 | 3.81 ns | 3.82 ns | 4.29 ns | 4.3 ns | 4.29 ns |
| D230 | 5.86 ns | 4.29 ns | 7.16 ns | 7.24 ns | 7.16 ns |
| D307 | 12.3 ns | 12.5 ns | 11.1 ns | 11.1 ns | 12.5 ns |
| D462 | 17.5 ns | 16.7 ns | 28.7 ns | 15.3 ns | 14.9 ns |
| D616 | 19 ns | 35.1 ns | 21.8 ns | 21.9 ns | 22 ns |
| D924 | 54.9 ns | 75.5 ns | 84.8 ns | 86.7 ns | 94 ns |
| D1232 | 54.4 ns | 69.9 ns | 44 ns | 69.7 ns | 69.9 ns |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,135.0 196.7,110.0 232.9,104.6 269.1,92.1 305.3,70.8 341.5,60.4 377.6,58.1 413.8,27.3 450.0,27.6 450.0,20.4 413.8,11.8 377.6,53.9 341.5,65.1 305.3,70.2 269.1,86.3 232.9,101.1 196.7,109.2 160.5,115.3 124.4,125.2 88.2,135.2 52.0,153.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,157.1 88.2,135.2 124.4,127.3 160.5,135.0 196.7,110.0 232.9,104.6 269.1,92.1 305.3,70.8 341.5,60.4 377.6,58.1 413.8,27.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.6 88.2,132.6 124.4,127.3 160.5,121.8 196.7,110.0 232.9,104.5 269.1,101.2 305.3,70.2 341.5,61.9 377.6,40.3 413.8,18.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.1 88.2,135.1 124.4,125.2 160.5,121.0 196.7,112.9 232.9,101.2 269.1,86.3 305.3,73.7 341.5,46.1 377.6,54.1 413.8,14.8 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,132.6 124.4,125.2 160.5,121.9 196.7,109.2 232.9,101.1 269.1,86.0 305.3,73.7 341.5,64.4 377.6,53.9 413.8,14.1 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,135.2 124.4,125.2 160.5,115.3 196.7,109.2 232.9,101.1 269.1,86.3 305.3,70.2 341.5,65.1 377.6,53.9 413.8,11.8 450.0,20.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 4.12 µs | 5.31 µs | 6.33 µs | 6.9 µs |
| D38 | 4.36 ns | 5.76 µs | 6.57 µs | 8.38 µs | 8.88 µs |
| D57 | 2.78 ns | 3.35 µs | 4.07 µs | 5.06 µs | 8.76 µs |
| D76 | 2.29 ns | 3.36 µs | 5.23 µs | 6.85 µs | 8.97 µs |
| D115 | 16.8 ns | 4.47 µs | 9.46 µs | 13 µs | 17.2 µs |
| D153 | 23.4 ns | 4.93 µs | 10.1 µs | 18.2 µs | 29.3 µs |
| D230 | 48.8 ns | 8.71 µs | 17.8 µs | 39.4 µs | 67.6 µs |
| D307 | 88.3 ns | 12 µs | 24.8 µs | 66.5 µs | 116 µs |
| D462 | 156 ns | 15.7 µs | 63.7 µs | 132 µs | 218 µs |
| D616 | 140 ns | 31.8 µs | 126 µs | 264 µs | 450 µs |
| D924 | 157 ns | 68.5 µs | 265 µs | 607 µs | 1.13 ms |
| D1232 | 403 ns | 127 µs | 398 µs | 1.12 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.3 160.5,199.7 196.7,175.0 232.9,170.9 269.1,161.8 305.3,154.4 341.5,147.3 377.6,148.7 413.8,147.3 450.0,135.6 450.0,27.6 413.8,37.1 377.6,48.5 341.5,57.5 305.3,65.3 269.1,72.0 232.9,82.4 196.7,89.0 160.5,97.1 124.4,97.4 88.2,97.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.3 160.5,199.7 196.7,175.0 232.9,170.9 269.1,161.8 305.3,154.4 341.5,147.3 377.6,148.7 413.8,147.3 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,102.6 124.4,109.3 160.5,109.3 196.7,105.7 232.9,104.5 269.1,97.4 305.3,93.5 341.5,90.2 377.6,81.3 413.8,71.8 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,100.9 124.4,106.9 160.5,103.8 196.7,96.4 232.9,95.5 269.1,88.6 305.3,84.4 341.5,72.7 377.6,64.3 413.8,55.0 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,97.9 124.4,104.2 160.5,100.4 196.7,92.5 232.9,88.3 269.1,78.7 305.3,72.2 341.5,63.7 377.6,55.1 413.8,44.8 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,97.2 124.4,97.4 160.5,97.1 196.7,89.0 232.9,82.4 269.1,72.0 305.3,65.3 341.5,57.5 377.6,48.5 413.8,37.1 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.91 µs | 9.43 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 14.5 µs | 15.2 µs |
| D57 | 12.2 ns | 5.73 µs | 7.01 µs | 7.98 µs | 10.1 µs |
| D76 | 6.99 ns | 5.59 µs | 8.01 µs | 10 µs | 12.8 µs |
| D115 | 11.2 ns | 12.6 µs | 11 µs | 21 µs | 25.3 µs |
| D153 | 23.1 ns | 7.49 µs | 14.8 µs | 22.4 µs | 35.2 µs |
| D230 | 49.7 ns | 12.6 µs | 22.4 µs | 47.8 µs | 78.7 µs |
| D307 | 87.4 ns | 17.3 µs | 48.3 µs | 79.8 µs | 122 µs |
| D462 | 149 ns | 24.9 µs | 85.6 µs | 155 µs | 231 µs |
| D616 | 140 ns | 40 µs | 141 µs | 288 µs | 446 µs |
| D924 | 170 ns | 82.2 µs | 289 µs | 607 µs | 995 µs |
| D1232 | 376 ns | 142 µs | 399 µs | 995 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.6 124.4,179.0 160.5,185.9 196.7,180.0 232.9,171.1 269.1,161.5 305.3,154.5 341.5,147.9 377.6,148.7 413.8,146.3 450.0,136.4 450.0,25.6 413.8,38.6 377.6,48.6 341.5,56.7 305.3,64.6 269.1,70.1 232.9,80.1 196.7,84.2 160.5,92.6 124.4,95.6 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.6 124.4,179.0 160.5,185.9 196.7,180.0 232.9,171.1 269.1,161.5 305.3,154.5 341.5,147.9 377.6,148.7 413.8,146.3 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,102.6 160.5,102.9 196.7,92.9 232.9,99.3 269.1,92.8 305.3,88.9 341.5,84.4 377.6,78.5 413.8,69.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,94.0 124.4,100.1 160.5,98.5 196.7,94.5 232.9,90.9 269.1,85.7 305.3,76.2 341.5,69.1 377.6,62.9 413.8,54.0 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,91.1 124.4,98.5 160.5,95.7 196.7,86.5 232.9,85.7 269.1,76.3 305.3,69.9 341.5,61.7 377.6,54.0 413.8,44.8 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,95.6 160.5,92.6 196.7,84.2 232.9,80.1 269.1,70.1 305.3,64.6 341.5,56.7 377.6,48.6 413.8,38.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 8.21 µs | 9.92 µs | 11.6 µs | 12.7 µs |
| D38 | 4.36 ns | 10.7 µs | 12.2 µs | 15.6 µs | 16.4 µs |
| D57 | 3.17 ns | 4.52 µs | 5.48 µs | 6.69 µs | 8.56 µs |
| D76 | 2.45 ns | 4.42 µs | 7.05 µs | 8.78 µs | 11.3 µs |
| D115 | 16.8 ns | 5.92 µs | 12 µs | 16.3 µs | 21.2 µs |
| D153 | 22.6 ns | 6.62 µs | 11.8 µs | 20.9 µs | 33.7 µs |
| D230 | 47.3 ns | 11.2 µs | 21.5 µs | 45.7 µs | 76.3 µs |
| D307 | 86 ns | 15.1 µs | 29.4 µs | 74.5 µs | 126 µs |
| D462 | 144 ns | 19.2 µs | 72.9 µs | 146 µs | 235 µs |
| D616 | 139 ns | 36.9 µs | 139 µs | 291 µs | 488 µs |
| D924 | 138 ns | 77.8 µs | 292 µs | 652 µs | 1.2 ms |
| D1232 | 392 ns | 139 µs | 431 µs | 1.19 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,191.7 124.4,195.7 160.5,198.9 196.7,175.0 232.9,171.3 269.1,162.1 305.3,154.7 341.5,148.4 377.6,148.8 413.8,148.8 450.0,135.9 450.0,26.8 413.8,36.3 377.6,47.5 341.5,56.6 305.3,64.3 269.1,70.5 232.9,80.6 196.7,86.4 160.5,94.2 124.4,97.7 88.2,89.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,191.7 124.4,195.7 160.5,198.9 196.7,175.0 232.9,171.3 269.1,162.1 305.3,154.7 341.5,148.4 377.6,148.8 413.8,148.8 450.0,135.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.2 88.2,94.8 124.4,105.6 160.5,105.8 196.7,102.2 232.9,100.8 269.1,94.3 305.3,90.6 341.5,87.6 377.6,79.5 413.8,70.3 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,93.2 124.4,103.2 160.5,100.1 196.7,93.4 232.9,93.7 269.1,86.2 305.3,82.3 341.5,71.1 377.6,63.0 413.8,53.9 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,90.2 124.4,100.7 160.5,97.3 196.7,89.7 232.9,86.6 269.1,76.9 305.3,70.8 341.5,62.4 377.6,53.9 413.8,43.9 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,89.6 124.4,97.7 160.5,94.2 196.7,86.4 232.9,80.6 269.1,70.5 305.3,64.3 341.5,56.6 377.6,47.5 413.8,36.3 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.92 µs | 9.56 µs | 11.2 µs | 12.1 µs |
| D38 | 3.74 ns | 10.3 µs | 11.5 µs | 14.5 µs | 15.3 µs |
| D57 | 2.84 µs | 5.86 µs | 7.33 µs | 8.24 µs | 10.5 µs |
| D76 | 1.63 µs | 5.74 µs | 8.27 µs | 10.4 µs | 13.2 µs |
| D115 | 5.95 µs | 13.1 µs | 11.6 µs | 21.5 µs | 26.4 µs |
| D153 | 2.88 µs | 7.82 µs | 15.5 µs | 23 µs | 35.9 µs |
| D230 | 2.92 µs | 13 µs | 23.5 µs | 47.9 µs | 79.7 µs |
| D307 | 3.42 µs | 17.6 µs | 50.1 µs | 81.1 µs | 125 µs |
| D462 | 3.62 µs | 25.2 µs | 86.9 µs | 157 µs | 234 µs |
| D616 | 3.49 µs | 40.4 µs | 142 µs | 292 µs | 451 µs |
| D924 | 3.72 µs | 83.3 µs | 292 µs | 611 µs | 1 ms |
| D1232 | 4.77 µs | 143 µs | 403 µs | 1 ms | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,118.2 196.7,102.2 232.9,111.1 269.1,111.0 305.3,109.0 341.5,108.3 377.6,108.8 413.8,108.0 450.0,104.9 450.0,25.5 413.8,38.6 377.6,48.5 341.5,56.6 305.3,64.4 269.1,70.0 232.9,79.8 196.7,83.7 160.5,92.3 124.4,95.1 88.2,90.5 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,118.2 196.7,102.2 232.9,111.1 269.1,111.0 305.3,109.0 341.5,108.3 377.6,108.8 413.8,108.0 450.0,104.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.3 124.4,102.3 160.5,102.6 196.7,92.4 232.9,98.8 269.1,92.4 305.3,88.7 341.5,84.2 377.6,78.4 413.8,69.4 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,93.9 124.4,99.6 160.5,98.1 196.7,93.8 232.9,90.3 269.1,85.1 305.3,75.7 341.5,68.9 377.6,62.8 413.8,53.9 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,91.1 124.4,98.1 160.5,95.2 196.7,86.2 232.9,85.4 269.1,76.3 305.3,69.7 341.5,61.5 377.6,53.9 413.8,44.7 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,90.5 124.4,95.1 160.5,92.3 196.7,83.7 232.9,79.8 269.1,70.0 305.3,64.4 341.5,56.6 377.6,48.5 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 3.21 µs | 3.23 µs | 3.8 µs | 4.14 µs |
| D38 | 4.05 ns | 3.46 µs | 4 µs | 5.08 µs | 5.43 µs |
| D57 | 198 ns | 336 ns | 300 ns | 307 ns | 388 ns |
| D76 | 93.9 ns | 284 ns | 337 ns | 401 ns | 464 ns |
| D115 | 376 ns | 519 ns | 587 ns | 707 ns | 761 ns |
| D153 | 343 ns | 502 ns | 614 ns | 752 ns | 898 ns |
| D230 | 505 ns | 691 ns | 932 ns | 1.36 µs | 1.72 µs |
| D307 | 893 ns | 1.27 µs | 1.46 µs | 2.05 µs | 2.71 µs |
| D462 | 952 ns | 1.5 µs | 2.3 µs | 2.96 µs | 3.84 µs |
| D616 | 1.1 µs | 1.95 µs | 3.04 µs | 4.32 µs | 5.95 µs |
| D924 | 1.45 µs | 2.65 µs | 4.7 µs | 7.44 µs | 11.2 µs |
| D1232 | 2.26 µs | 4.08 µs | 6.31 µs | 11.7 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,185.7 124.4,118.2 160.5,131.1 196.7,107.0 232.9,108.6 269.1,101.9 305.3,92.0 341.5,90.8 377.6,88.4 413.8,83.6 450.0,75.8 450.0,30.7 413.8,48.1 377.6,59.0 341.5,66.6 305.3,72.7 269.1,80.5 232.9,91.9 196.7,94.8 160.5,103.3 124.4,106.4 88.2,60.6 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,185.7 124.4,118.2 160.5,131.1 196.7,107.0 232.9,108.6 269.1,101.9 305.3,92.0 341.5,90.8 377.6,88.4 413.8,83.6 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,68.4 124.4,108.9 160.5,111.9 196.7,101.4 232.9,102.0 269.1,96.4 305.3,85.8 341.5,83.0 377.6,78.4 413.8,73.1 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,65.9 124.4,110.9 160.5,108.9 196.7,99.3 232.9,98.5 269.1,91.2 305.3,83.4 341.5,75.5 377.6,70.7 413.8,63.1 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,61.8 124.4,110.5 160.5,105.9 196.7,96.0 232.9,95.0 269.1,84.6 305.3,77.5 341.5,71.2 377.6,64.6 413.8,55.1 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,60.6 124.4,106.4 160.5,103.3 196.7,94.8 232.9,91.9 269.1,80.5 305.3,72.7 341.5,66.6 377.6,59.0 413.8,48.1 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 157 ns | 183 ns | 205 ns | 205 ns |
| D38 | 4.36 ns | 201 ns | 187 ns | 203 ns | 187 ns |
| D57 | 311 ns | 412 ns | 419 ns | 427 ns | 506 ns |
| D76 | 149 ns | 380 ns | 457 ns | 527 ns | 597 ns |
| D115 | 615 ns | 706 ns | 801 ns | 921 ns | 963 ns |
| D153 | 596 ns | 715 ns | 860 ns | 962 ns | 1.15 µs |
| D230 | 876 ns | 961 ns | 1.29 µs | 1.79 µs | 2.12 µs |
| D307 | 1.48 µs | 1.78 µs | 1.96 µs | 2.6 µs | 3.32 µs |
| D462 | 1.53 µs | 1.97 µs | 2.91 µs | 3.57 µs | 4.47 µs |
| D616 | 1.74 µs | 2.52 µs | 3.83 µs | 5.1 µs | 6.75 µs |
| D924 | 2.36 µs | 3.55 µs | 5.68 µs | 8.5 µs | 12.3 µs |
| D1232 | 3.48 µs | 5.25 µs | 7.28 µs | 13.2 µs | 31.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,110.3 160.5,123.0 196.7,98.4 232.9,99.0 269.1,92.3 305.3,83.2 341.5,82.6 377.6,80.4 413.8,75.1 450.0,68.4 450.0,29.8 413.8,46.4 377.6,56.8 341.5,64.0 305.3,69.1 269.1,77.0 232.9,87.6 196.7,90.7 160.5,99.0 124.4,101.8 88.2,119.1 52.0,117.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,110.3 160.5,123.0 196.7,98.4 232.9,99.0 269.1,92.3 305.3,83.2 341.5,82.6 377.6,80.4 413.8,75.1 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,117.8 124.4,105.4 160.5,106.8 196.7,96.0 232.9,95.8 269.1,90.7 305.3,80.0 341.5,78.2 377.6,73.9 413.8,68.0 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.5 88.2,119.2 124.4,105.1 160.5,103.6 196.7,93.8 232.9,92.6 269.1,85.5 305.3,78.3 341.5,71.4 377.6,66.7 413.8,59.8 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,117.7 124.4,104.8 160.5,101.1 196.7,91.4 232.9,90.7 269.1,79.9 305.3,73.4 341.5,67.9 377.6,61.7 413.8,52.8 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,119.1 124.4,101.8 160.5,99.0 196.7,90.7 232.9,87.6 269.1,77.0 305.3,69.1 341.5,64.0 377.6,56.8 413.8,46.4 450.0,29.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
