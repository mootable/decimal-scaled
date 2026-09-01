# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.31 ns | 21.8 µs | 29.6 µs | 32.5 µs | 39.4 µs |
| D38 | 4.3 µs | 5.09 µs | 7.25 µs | 11.3 µs | 16.1 µs |
| D57 | 4.36 µs | 8.05 µs | 8.42 µs | 12.3 µs | 17.4 µs |
| D76 | 5.15 µs | 5.77 µs | 12 µs | 17.3 µs | 16.9 µs |
| D115 | 4.76 µs | 6.62 µs | 23.4 µs | 33.2 µs | 43.6 µs |
| D153 | 4.74 µs | 15.6 µs | 21.1 µs | 41 µs | 59.2 µs |
| D230 | 5.03 µs | 26 µs | 45.3 µs | 64 µs | 131 µs |
| D307 | 5.13 µs | 24.6 µs | 64.9 µs | 131 µs | 192 µs |
| D462 | 4.87 µs | 43.7 µs | 131 µs | 234 µs | 398 µs |
| D616 | 4.34 µs | 60.1 µs | 207 µs | 421 µs | 709 µs |
| D924 | 5.36 µs | 114 µs | 419 µs | 862 µs | 1.12 ms |
| D1232 | 3.71 µs | 202 µs | 490 µs | 1.47 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.7 88.2,106.2 124.4,106.0 160.5,103.9 196.7,104.9 232.9,105.0 269.1,104.2 305.3,104.0 341.5,104.6 377.6,106.1 413.8,103.5 450.0,108.0 450.0,23.0 413.8,37.1 377.6,42.8 341.5,50.0 305.3,59.1 269.1,63.8 232.9,73.6 196.7,77.4 160.5,89.2 124.4,88.8 88.2,89.8 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.7 88.2,106.2 124.4,106.0 160.5,103.9 196.7,104.9 232.9,105.0 269.1,104.2 305.3,104.0 341.5,104.6 377.6,106.1 413.8,103.5 450.0,108.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,104.1 124.4,98.4 160.5,102.5 196.7,100.8 232.9,90.2 269.1,83.9 305.3,84.5 341.5,77.4 377.6,73.5 413.8,65.6 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.7 124.4,97.9 160.5,93.4 196.7,85.2 232.9,86.4 269.1,77.0 305.3,72.5 341.5,63.8 377.6,58.1 413.8,49.4 450.0,47.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,94.2 124.4,93.2 160.5,88.9 196.7,80.8 232.9,78.2 269.1,72.7 305.3,63.8 341.5,56.6 377.6,49.3 413.8,40.4 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.8 124.4,88.8 160.5,89.2 196.7,77.4 232.9,73.6 269.1,63.8 305.3,59.1 341.5,50.0 377.6,42.8 413.8,37.1 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 29.2 µs | 35.5 µs | 39 µs | 38.4 µs |
| D38 | 25.8 µs | 16.1 µs | 41.3 µs | 53.4 µs | 66.1 µs |
| D57 | 3.06 µs | 4.13 µs | 3.59 µs | 5.09 µs | 7.48 µs |
| D76 | 3.7 µs | 2.71 µs | 4.87 µs | 7.69 µs | 7.83 µs |
| D115 | 6.18 µs | 5.46 µs | 12.3 µs | 18.2 µs | 23.5 µs |
| D153 | 6.18 µs | 10.7 µs | 11.1 µs | 21.6 µs | 30.2 µs |
| D230 | 8.26 µs | 17 µs | 28.5 µs | 38.6 µs | 73.4 µs |
| D307 | 13.2 µs | 22.2 µs | 51.1 µs | 90.6 µs | 138 µs |
| D462 | 12.4 µs | 38.8 µs | 88.2 µs | 158 µs | 274 µs |
| D616 | 18.8 µs | 72.4 µs | 170 µs | 317 µs | 554 µs |
| D924 | 33.2 µs | 131 µs | 380 µs | 752 µs | 982 µs |
| D1232 | 29.8 µs | 259 µs | 483 µs | 1.41 ms | 3.06 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.4 88.2,139.5 124.4,185.7 160.5,181.6 196.7,170.5 232.9,170.4 269.1,164.2 305.3,153.9 341.5,155.3 377.6,146.3 413.8,133.9 450.0,136.3 450.0,35.7 413.8,60.4 377.6,72.8 341.5,88.1 305.3,103.0 269.1,116.7 232.9,136.0 196.7,141.5 160.5,165.3 124.4,166.3 88.2,119.0 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.4 88.2,139.5 124.4,185.7 160.5,181.6 196.7,170.5 232.9,170.4 269.1,164.2 305.3,153.9 341.5,155.3 377.6,146.3 413.8,133.9 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,136.8 88.2,149.6 124.4,179.2 160.5,188.4 196.7,173.1 232.9,158.6 269.1,148.4 305.3,142.7 341.5,130.5 377.6,117.0 413.8,104.1 450.0,89.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.5 88.2,129.2 124.4,182.3 160.5,175.6 196.7,155.5 232.9,157.6 269.1,137.3 305.3,124.6 341.5,112.7 377.6,98.5 413.8,81.0 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,123.6 124.4,174.6 160.5,165.7 196.7,146.9 232.9,143.3 269.1,130.7 305.3,112.2 341.5,100.1 377.6,84.9 413.8,66.2 450.0,52.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,119.0 124.4,166.3 160.5,165.3 196.7,141.5 232.9,136.0 269.1,116.7 305.3,103.0 341.5,88.1 377.6,72.8 413.8,60.4 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 21.8 µs | 29.7 µs | 32.5 µs | 39.4 µs |
| D38 | 4.23 µs | 5.06 µs | 7.25 µs | 11.3 µs | 16 µs |
| D57 | 4.32 µs | 7.86 µs | 8.45 µs | 12.3 µs | 17.4 µs |
| D76 | 5.09 µs | 5.46 µs | 12 µs | 17.3 µs | 16.9 µs |
| D115 | 4.72 µs | 6.59 µs | 23.5 µs | 33.2 µs | 44.1 µs |
| D153 | 4.69 µs | 15.6 µs | 21.6 µs | 41.1 µs | 59.4 µs |
| D230 | 5.01 µs | 25.5 µs | 44.2 µs | 64.8 µs | 131 µs |
| D307 | 5.31 µs | 24.8 µs | 64.4 µs | 131 µs | 193 µs |
| D462 | 4.81 µs | 44.1 µs | 130 µs | 233 µs | 396 µs |
| D616 | 4.41 µs | 60.5 µs | 205 µs | 420 µs | 701 µs |
| D924 | 5.33 µs | 113 µs | 422 µs | 859 µs | 1.12 ms |
| D1232 | 3.73 µs | 201 µs | 479 µs | 1.47 ms | 3.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,106.4 124.4,106.1 160.5,104.1 196.7,105.0 232.9,105.1 269.1,104.3 305.3,103.6 341.5,104.8 377.6,105.9 413.8,103.5 450.0,107.9 450.0,23.1 413.8,37.1 377.6,43.0 341.5,50.1 305.3,59.0 269.1,63.8 232.9,73.6 196.7,77.3 160.5,89.2 124.4,88.8 88.2,89.9 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,106.4 124.4,106.1 160.5,104.1 196.7,105.0 232.9,105.1 269.1,104.3 305.3,103.6 341.5,104.8 377.6,105.9 413.8,103.5 450.0,107.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.0 88.2,104.2 124.4,98.7 160.5,103.2 196.7,100.9 232.9,90.2 269.1,84.1 305.3,84.4 341.5,77.3 377.6,73.4 413.8,65.6 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,99.7 124.4,97.8 160.5,93.5 196.7,85.1 232.9,86.2 269.1,77.3 305.3,72.6 341.5,63.9 377.6,58.2 413.8,49.3 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,94.2 124.4,93.2 160.5,88.9 196.7,80.8 232.9,78.2 269.1,72.5 305.3,63.8 341.5,56.7 377.6,49.3 413.8,40.5 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,89.9 124.4,88.8 160.5,89.2 196.7,77.3 232.9,73.6 269.1,63.8 305.3,59.0 341.5,50.1 377.6,43.0 413.8,37.1 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 21.5 µs | 29 µs | 31.8 µs | 38.7 µs |
| D38 | 3.74 ns | 12.4 µs | 33.7 µs | 39.9 µs | 55.1 µs |
| D57 | 1.8 ns | 5.13 µs | 5.54 µs | 7.4 µs | 11.5 µs |
| D76 | 2.22 ns | 3.73 µs | 7.19 µs | 11.6 µs | 11.7 µs |
| D115 | 14 ns | 8.07 µs | 18.9 µs | 26.6 µs | 36.8 µs |
| D153 | 16.8 ns | 16.2 µs | 17.6 µs | 33.3 µs | 44.1 µs |
| D230 | 27.8 ns | 26.2 µs | 44.4 µs | 56 µs | 102 µs |
| D307 | 52.1 ns | 32 µs | 73.2 µs | 123 µs | 177 µs |
| D462 | 69.1 ns | 57.9 µs | 126 µs | 196 µs | 337 µs |
| D616 | 74.4 ns | 105 µs | 228 µs | 402 µs | 670 µs |
| D924 | 112 ns | 199 µs | 501 µs | 871 µs | 1.05 ms |
| D1232 | 115 ns | 364 µs | 614 µs | 1.61 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,202.7 160.5,200.1 196.7,177.2 232.9,175.0 269.1,168.8 305.3,161.0 341.5,157.4 377.6,156.5 413.8,151.4 450.0,151.2 450.0,24.5 413.8,38.0 377.6,43.5 341.5,52.1 305.3,60.0 269.1,66.9 232.9,77.3 196.7,79.5 160.5,93.8 124.4,94.0 88.2,74.5 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,202.7 160.5,200.1 196.7,177.2 232.9,175.0 269.1,168.8 305.3,161.0 341.5,157.4 377.6,156.5 413.8,151.4 450.0,151.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,86.2 88.2,93.1 124.4,104.0 160.5,108.0 196.7,98.4 232.9,89.7 269.1,83.8 305.3,81.3 341.5,73.9 377.6,66.5 413.8,58.6 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.5 88.2,80.6 124.4,103.1 160.5,99.8 196.7,87.8 232.9,88.7 269.1,77.2 305.3,71.0 341.5,64.3 377.6,56.9 413.8,47.1 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,78.5 124.4,99.5 160.5,93.9 196.7,83.6 232.9,80.8 269.1,74.3 305.3,64.6 341.5,58.8 377.6,49.9 413.8,40.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,74.5 124.4,94.0 160.5,93.8 196.7,79.5 232.9,77.3 269.1,66.9 305.3,60.0 341.5,52.1 377.6,43.5 413.8,38.0 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.57 µs | 2.67 µs | 2.94 µs | 3.21 µs |
| D38 | 5.32 µs | 5.5 µs | 5.91 µs | 9.76 µs | 14.1 µs |
| D57 | 3.58 µs | 7.11 µs | 7.3 µs | 10.8 µs | 5 µs |
| D76 | 4.2 µs | 4.71 µs | 10.5 µs | 15.3 µs | 15 µs |
| D115 | 3.88 µs | 5.7 µs | 21.6 µs | 30.3 µs | 39.4 µs |
| D153 | 3.89 µs | 13.7 µs | 17.6 µs | 37.2 µs | 53.9 µs |
| D230 | 4.18 µs | 22.7 µs | 39.9 µs | 58 µs | 120 µs |
| D307 | 4.23 µs | 22.4 µs | 52.8 µs | 122 µs | 177 µs |
| D462 | 2.97 µs | 35.6 µs | 111 µs | 205 µs | 337 µs |
| D616 | 3.57 µs | 55.6 µs | 190 µs | 392 µs | 664 µs |
| D924 | 4.56 µs | 105 µs | 391 µs | 816 µs | 1.06 ms |
| D1232 | 3.08 µs | 187 µs | 466 µs | 1.41 ms | 3.34 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,103.5 124.4,108.4 160.5,106.5 196.7,107.5 232.9,107.4 269.1,106.5 305.3,106.4 341.5,110.8 377.6,108.5 413.8,105.5 450.0,110.3 450.0,23.6 413.8,37.8 377.6,43.7 341.5,52.1 305.3,60.1 269.1,64.9 232.9,74.8 196.7,78.7 160.5,90.7 124.4,104.3 88.2,91.4 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,103.5 124.4,108.4 160.5,106.5 196.7,107.5 232.9,107.4 269.1,106.5 305.3,106.4 341.5,110.8 377.6,108.5 413.8,105.5 450.0,110.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,103.1 124.4,99.9 160.5,105.1 196.7,102.7 232.9,91.8 269.1,85.5 305.3,85.7 341.5,80.0 377.6,74.4 413.8,66.6 450.0,59.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,102.2 124.4,99.6 160.5,95.1 196.7,86.1 232.9,88.7 269.1,78.5 305.3,75.1 341.5,65.8 377.6,59.2 413.8,50.2 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,96.0 124.4,94.8 160.5,90.4 196.7,82.0 232.9,79.4 269.1,73.9 305.3,64.7 341.5,58.3 377.6,50.2 413.8,41.1 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,91.4 124.4,104.3 160.5,90.7 196.7,78.7 232.9,74.8 269.1,64.9 305.3,60.1 341.5,52.1 377.6,43.7 413.8,37.8 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.37 µs | 9.41 µs | 10.3 µs | 11.3 µs |
| D38 | 3.73 ns | 4.76 µs | 11 µs | 13 µs | 16 µs |
| D57 | 466 ns | 5.34 µs | 4.91 µs | 7.07 µs | 11 µs |
| D76 | 604 ns | 3.53 µs | 6.69 µs | 11.2 µs | 11.6 µs |
| D115 | 1.03 µs | 7.28 µs | 17.9 µs | 27.3 µs | 36 µs |
| D153 | 1.11 µs | 14.8 µs | 16.8 µs | 33.8 µs | 49.4 µs |
| D230 | 1.37 µs | 24.7 µs | 43.9 µs | 64.9 µs | 126 µs |
| D307 | 2.24 µs | 33.7 µs | 84.4 µs | 154 µs | 244 µs |
| D462 | 2.31 µs | 60.2 µs | 151 µs | 281 µs | 498 µs |
| D616 | 3.25 µs | 116 µs | 296 µs | 568 µs | 1.02 ms |
| D924 | 5.94 µs | 219 µs | 678 µs | 1.38 ms | 1.83 ms |
| D1232 | 5.54 µs | 442 µs | 908 µs | 2.61 ms | 5.75 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,133.8 160.5,130.5 196.7,123.9 232.9,123.0 269.1,120.4 305.3,114.3 341.5,113.9 377.6,109.7 413.8,102.2 450.0,103.0 450.0,16.9 413.8,31.1 377.6,38.4 341.5,47.2 305.3,56.1 269.1,64.3 232.9,75.9 196.7,79.8 160.5,93.9 124.4,94.5 88.2,89.8 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,133.8 160.5,130.5 196.7,123.9 232.9,123.0 269.1,120.4 305.3,114.3 341.5,113.9 377.6,109.7 413.8,102.2 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,104.9 124.4,103.5 160.5,108.7 196.7,99.7 232.9,90.8 269.1,84.5 305.3,80.6 341.5,73.4 377.6,65.3 413.8,57.4 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.5 88.2,94.5 124.4,104.5 160.5,100.7 196.7,88.5 232.9,89.2 269.1,77.4 305.3,69.3 341.5,62.0 377.6,53.7 413.8,43.4 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.5 124.4,100.0 160.5,94.3 196.7,83.3 232.9,80.6 269.1,72.5 305.3,61.8 341.5,54.3 377.6,45.6 413.8,34.5 450.0,26.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,89.8 124.4,94.5 160.5,93.9 196.7,79.8 232.9,75.9 269.1,64.3 305.3,56.1 341.5,47.2 377.6,38.4 413.8,31.1 450.0,16.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.98 ns | 3.98 µs | 5.97 µs | 6.41 µs | 6.99 µs |
| D38 | 4.98 ns | 2.95 µs | 6.76 µs | 7.92 µs | 9.7 µs |
| D57 | 2.02 ns | 3.19 µs | 3.24 µs | 4.31 µs | 8.76 µs |
| D76 | 3.15 ns | 2.29 µs | 4.25 µs | 6.95 µs | 7.24 µs |
| D115 | 17.4 ns | 2.56 µs | 9.51 µs | 14.4 µs | 18.8 µs |
| D153 | 22.8 ns | 5.47 µs | 7.14 µs | 18.2 µs | 29.2 µs |
| D230 | 48.9 ns | 10.1 µs | 20.1 µs | 32.5 µs | 73.2 µs |
| D307 | 96.1 ns | 11.1 µs | 26.6 µs | 73.6 µs | 116 µs |
| D462 | 140 ns | 15.8 µs | 64.4 µs | 130 µs | 232 µs |
| D616 | 171 ns | 30.4 µs | 123 µs | 255 µs | 455 µs |
| D924 | 180 ns | 62.4 µs | 256 µs | 568 µs | 783 µs |
| D1232 | 270 ns | 120 µs | 316 µs | 1.01 ms | 2.44 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,190.1 124.4,201.3 160.5,195.8 196.7,174.5 232.9,171.2 269.1,161.7 305.3,153.3 341.5,148.7 377.6,146.2 413.8,145.6 450.0,140.5 450.0,27.5 413.8,41.6 377.6,48.3 341.5,56.7 305.3,65.3 269.1,71.0 232.9,82.4 196.7,87.9 160.5,99.7 124.4,97.4 88.2,96.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,190.1 124.4,201.3 160.5,195.8 196.7,174.5 232.9,171.2 269.1,161.7 305.3,153.3 341.5,148.7 377.6,146.2 413.8,145.6 450.0,140.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,110.9 124.4,109.9 160.5,114.0 196.7,112.6 232.9,103.2 269.1,95.6 305.3,94.4 341.5,90.0 377.6,81.9 413.8,73.0 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.1 88.2,100.6 124.4,109.7 160.5,106.3 196.7,96.3 232.9,99.9 269.1,87.1 305.3,83.6 341.5,72.6 377.6,64.6 413.8,55.5 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,98.6 124.4,106.2 160.5,100.2 196.7,91.2 232.9,88.3 269.1,81.1 305.3,70.9 341.5,63.9 377.6,55.5 413.8,45.6 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,96.1 124.4,97.4 160.5,99.7 196.7,87.9 232.9,82.4 269.1,71.0 305.3,65.3 341.5,56.7 377.6,48.3 413.8,41.6 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 7.35 µs | 10.2 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 4.9 µs | 11.5 µs | 13.4 µs | 16.5 µs |
| D57 | 2.88 ns | 4.99 µs | 5.16 µs | 6.47 µs | 10 µs |
| D76 | 3.65 ns | 3.53 µs | 6.15 µs | 10.1 µs | 9.9 µs |
| D115 | 9.97 ns | 7.22 µs | 11.3 µs | 22.8 µs | 28.1 µs |
| D153 | 22.4 ns | 8.03 µs | 11.9 µs | 22.8 µs | 35 µs |
| D230 | 49.2 ns | 14.6 µs | 24.1 µs | 39.6 µs | 84 µs |
| D307 | 97.1 ns | 14.1 µs | 51.4 µs | 84.8 µs | 124 µs |
| D462 | 138 ns | 24.8 µs | 86.4 µs | 152 µs | 249 µs |
| D616 | 202 ns | 36.1 µs | 133 µs | 270 µs | 448 µs |
| D924 | 202 ns | 75.1 µs | 267 µs | 559 µs | 690 µs |
| D1232 | 257 ns | 132 µs | 309 µs | 894 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,192.6 124.4,196.9 160.5,193.9 196.7,181.5 232.9,171.4 269.1,161.7 305.3,153.2 341.5,148.9 377.6,144.2 413.8,144.1 450.0,141.1 450.0,25.5 413.8,43.2 377.6,48.5 341.5,55.8 305.3,64.4 269.1,69.3 232.9,80.2 196.7,82.9 160.5,95.8 124.4,95.7 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,192.6 124.4,196.9 160.5,193.9 196.7,181.5 232.9,171.4 269.1,161.7 305.3,153.2 341.5,148.9 377.6,144.2 413.8,144.1 450.0,141.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,104.6 124.4,104.3 160.5,108.6 196.7,99.8 232.9,98.4 269.1,91.0 305.3,91.4 341.5,84.5 377.6,79.8 413.8,70.7 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,94.0 124.4,103.9 160.5,101.8 196.7,94.2 232.9,93.6 269.1,84.8 305.3,75.4 341.5,69.0 377.6,63.6 413.8,55.0 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,101.1 160.5,95.6 196.7,85.5 232.9,85.5 269.1,78.6 305.3,69.2 341.5,61.9 377.6,54.8 413.8,45.8 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,95.7 160.5,95.8 196.7,82.9 232.9,80.2 269.1,69.3 305.3,64.4 341.5,55.8 377.6,48.5 413.8,43.2 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 3.85 µs | 5.79 µs | 6.33 µs | 6.91 µs |
| D38 | 4.36 ns | 2.81 µs | 6.57 µs | 7.74 µs | 9.64 µs |
| D57 | 2.02 ns | 2.93 µs | 3.06 µs | 4.29 µs | 8.76 µs |
| D76 | 4.67 ns | 2.12 µs | 4.03 µs | 6.91 µs | 6.95 µs |
| D115 | 17.7 ns | 2.43 µs | 9.34 µs | 14.1 µs | 18.4 µs |
| D153 | 23.1 ns | 5.38 µs | 6.95 µs | 17.6 µs | 28.1 µs |
| D230 | 49.2 ns | 10.6 µs | 20 µs | 32.3 µs | 71.8 µs |
| D307 | 88.6 ns | 10.9 µs | 26 µs | 72.8 µs | 115 µs |
| D462 | 127 ns | 15.9 µs | 63 µs | 130 µs | 232 µs |
| D616 | 157 ns | 29.4 µs | 120 µs | 251 µs | 450 µs |
| D924 | 194 ns | 62.1 µs | 250 µs | 563 µs | 776 µs |
| D1232 | 265 ns | 119 µs | 312 µs | 1.01 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,201.3 160.5,190.9 196.7,174.3 232.9,171.1 269.1,161.7 305.3,154.4 341.5,149.8 377.6,147.2 413.8,144.6 450.0,140.7 450.0,27.6 413.8,41.7 377.6,48.5 341.5,56.7 305.3,65.4 269.1,71.3 232.9,82.9 196.7,88.1 160.5,100.2 124.4,97.4 88.2,96.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,201.3 160.5,190.9 196.7,174.3 232.9,171.1 269.1,161.7 305.3,154.4 341.5,149.8 377.6,147.2 413.8,144.6 450.0,140.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.5 88.2,111.5 124.4,110.9 160.5,114.9 196.7,113.3 232.9,103.4 269.1,95.0 305.3,94.7 341.5,90.0 377.6,82.3 413.8,73.0 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.5 88.2,100.9 124.4,110.4 160.5,107.0 196.7,96.6 232.9,100.2 269.1,87.1 305.3,83.9 341.5,72.9 377.6,64.9 413.8,55.8 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,98.9 124.4,106.2 160.5,100.3 196.7,91.5 232.9,88.7 269.1,81.2 305.3,71.1 341.5,63.9 377.6,55.7 413.8,45.7 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,96.2 124.4,97.4 160.5,100.2 196.7,88.1 232.9,82.9 269.1,71.3 305.3,65.4 341.5,56.7 377.6,48.5 413.8,41.7 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 7.35 µs | 10.2 µs | 11 µs | 11.9 µs |
| D38 | 4.05 ns | 4.91 µs | 11.5 µs | 13.4 µs | 16.5 µs |
| D57 | 9.81 ns | 5.22 µs | 5.12 µs | 6.5 µs | 10 µs |
| D76 | 12.1 ns | 3.53 µs | 6.18 µs | 10.1 µs | 9.92 µs |
| D115 | 10.3 ns | 7.02 µs | 11.2 µs | 22.8 µs | 27.7 µs |
| D153 | 21.8 ns | 8.08 µs | 11.6 µs | 22.6 µs | 35.1 µs |
| D230 | 49.6 ns | 14.6 µs | 24.4 µs | 39.2 µs | 83.8 µs |
| D307 | 87.4 ns | 14.2 µs | 52.4 µs | 84.3 µs | 125 µs |
| D462 | 125 ns | 24.5 µs | 86.8 µs | 153 µs | 249 µs |
| D616 | 150 ns | 36.1 µs | 133 µs | 269 µs | 448 µs |
| D924 | 206 ns | 75.2 µs | 267 µs | 560 µs | 691 µs |
| D1232 | 255 ns | 133 µs | 311 µs | 894 µs | 2.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,192.6 124.4,181.7 160.5,179.1 196.7,181.1 232.9,171.7 269.1,161.6 305.3,154.5 341.5,150.1 377.6,147.8 413.8,143.9 450.0,141.3 450.0,25.5 413.8,43.2 377.6,48.5 341.5,55.8 305.3,64.4 269.1,69.3 232.9,80.1 196.7,83.1 160.5,95.8 124.4,95.7 88.2,89.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,192.6 124.4,181.7 160.5,179.1 196.7,181.1 232.9,171.7 269.1,161.6 305.3,154.5 341.5,150.1 377.6,147.8 413.8,143.9 450.0,141.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,104.5 124.4,103.8 160.5,108.6 196.7,100.1 232.9,98.4 269.1,91.0 305.3,91.3 341.5,84.6 377.6,79.8 413.8,70.7 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,94.0 124.4,104.0 160.5,101.7 196.7,94.4 232.9,93.9 269.1,84.7 305.3,75.2 341.5,68.9 377.6,63.6 413.8,54.9 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,101.1 160.5,95.6 196.7,85.5 232.9,85.6 269.1,78.8 305.3,69.3 341.5,61.9 377.6,54.8 413.8,45.8 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,89.5 124.4,95.7 160.5,95.8 196.7,83.1 232.9,80.1 269.1,69.3 305.3,64.4 341.5,55.8 377.6,48.5 413.8,43.2 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 7.64 µs | 10.8 µs | 11.6 µs | 12.7 µs |
| D38 | 4.36 ns | 5.39 µs | 12.2 µs | 14.4 µs | 17.7 µs |
| D57 | 2.88 ns | 3.96 µs | 3.96 µs | 5.64 µs | 8.63 µs |
| D76 | 3.5 ns | 2.79 µs | 5.41 µs | 8.81 µs | 8.73 µs |
| D115 | 17.1 ns | 3.2 µs | 12 µs | 16.6 µs | 23 µs |
| D153 | 22.5 ns | 6.94 µs | 8.94 µs | 21.1 µs | 33.5 µs |
| D230 | 48 ns | 13.7 µs | 23.1 µs | 37 µs | 80.8 µs |
| D307 | 84.6 ns | 13.1 µs | 31 µs | 80.4 µs | 127 µs |
| D462 | 121 ns | 20.1 µs | 72.1 µs | 144 µs | 248 µs |
| D616 | 145 ns | 33.8 µs | 135 µs | 278 µs | 487 µs |
| D924 | 161 ns | 71.2 µs | 275 µs | 604 µs | 829 µs |
| D1232 | 273 ns | 131 µs | 338 µs | 1.07 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,196.9 160.5,194.5 196.7,174.7 232.9,171.4 269.1,162.0 305.3,154.9 341.5,150.5 377.6,148.3 413.8,146.9 450.0,140.4 450.0,26.8 413.8,40.9 377.6,47.5 341.5,55.9 305.3,64.2 269.1,69.8 232.9,80.7 196.7,85.4 160.5,97.4 124.4,97.5 88.2,88.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,196.9 160.5,194.5 196.7,174.7 232.9,171.4 269.1,162.0 305.3,154.9 341.5,150.5 377.6,148.3 413.8,146.9 450.0,140.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.1 88.2,103.4 124.4,107.2 160.5,111.6 196.7,109.9 232.9,100.2 269.1,91.8 305.3,92.3 341.5,87.0 377.6,80.6 413.8,71.4 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,93.2 124.4,107.2 160.5,103.3 196.7,93.5 232.9,97.1 269.1,85.3 305.3,81.7 341.5,71.2 377.6,63.4 413.8,54.6 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,91.2 124.4,102.8 160.5,97.3 196.7,89.4 232.9,86.4 269.1,79.5 305.3,69.8 341.5,62.6 377.6,54.5 413.8,44.8 450.0,37.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,88.6 124.4,97.5 160.5,97.4 196.7,85.4 232.9,80.7 269.1,69.8 305.3,64.2 341.5,55.9 377.6,47.5 413.8,40.9 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.35 µs | 10.4 µs | 11.2 µs | 12.1 µs |
| D38 | 3.74 ns | 5.04 µs | 11.6 µs | 13.4 µs | 16.6 µs |
| D57 | 2.46 µs | 4.99 µs | 5.43 µs | 6.74 µs | 10.3 µs |
| D76 | 2.99 µs | 3.8 µs | 6.41 µs | 10.4 µs | 10.2 µs |
| D115 | 5.5 µs | 7.56 µs | 11.8 µs | 23.3 µs | 28.7 µs |
| D153 | 2.86 µs | 8.42 µs | 12 µs | 23.3 µs | 35.9 µs |
| D230 | 2.88 µs | 15 µs | 25.5 µs | 39.7 µs | 84.9 µs |
| D307 | 3.42 µs | 14.7 µs | 53.4 µs | 86.7 µs | 127 µs |
| D462 | 3.37 µs | 25.2 µs | 86.7 µs | 156 µs | 250 µs |
| D616 | 3.02 µs | 37.1 µs | 135 µs | 273 µs | 451 µs |
| D924 | 4.05 µs | 77.4 µs | 270 µs | 564 µs | 696 µs |
| D1232 | 3.11 µs | 133 µs | 315 µs | 899 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,113.1 160.5,110.7 196.7,103.1 232.9,111.2 269.1,111.2 305.3,109.0 341.5,109.2 377.6,110.6 413.8,106.9 450.0,110.2 450.0,25.5 413.8,43.1 377.6,48.5 341.5,55.8 305.3,64.2 269.1,69.2 232.9,79.9 196.7,82.6 160.5,95.5 124.4,95.3 88.2,89.4 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,113.1 160.5,110.7 196.7,103.1 232.9,111.2 269.1,111.2 305.3,109.0 341.5,109.2 377.6,110.6 413.8,106.9 450.0,110.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,99.5 88.2,104.2 124.4,104.3 160.5,107.7 196.7,99.2 232.9,97.8 269.1,90.7 305.3,90.9 341.5,84.3 377.6,79.5 413.8,70.3 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,93.9 124.4,103.3 160.5,101.2 196.7,93.7 232.9,93.5 269.1,84.1 305.3,74.9 341.5,68.9 377.6,63.4 413.8,54.8 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,92.1 124.4,100.6 160.5,95.3 196.7,85.2 232.9,85.2 269.1,78.6 305.3,68.9 341.5,61.6 377.6,54.7 413.8,45.7 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,89.4 124.4,95.3 160.5,95.5 196.7,82.6 232.9,79.9 269.1,69.2 305.3,64.2 341.5,55.8 377.6,48.5 413.8,43.1 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 3 µs | 3.47 µs | 3.8 µs | 4.14 µs |
| D38 | 4.36 ns | 1.4 µs | 4.01 µs | 4.72 µs | 5.85 µs |
| D57 | 165 ns | 287 ns | 218 ns | 259 ns | 396 ns |
| D76 | 198 ns | 157 ns | 260 ns | 401 ns | 359 ns |
| D115 | 348 ns | 294 ns | 591 ns | 755 ns | 839 ns |
| D153 | 343 ns | 562 ns | 512 ns | 759 ns | 904 ns |
| D230 | 528 ns | 904 ns | 1.1 µs | 1.19 µs | 1.93 µs |
| D307 | 892 ns | 973 ns | 1.62 µs | 2.17 µs | 2.79 µs |
| D462 | 867 ns | 1.53 µs | 2.26 µs | 2.91 µs | 4.16 µs |
| D616 | 967 ns | 1.75 µs | 2.79 µs | 4.04 µs | 5.93 µs |
| D924 | 1.61 µs | 2.38 µs | 4.32 µs | 6.75 µs | 7.78 µs |
| D1232 | 1.34 µs | 3.87 µs | 5.14 µs | 10.6 µs | 30.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,121.3 160.5,118.2 196.7,108.4 232.9,108.6 269.1,101.1 305.3,92.0 341.5,92.5 377.6,90.6 413.8,81.8 450.0,84.9 450.0,30.7 413.8,54.4 377.6,59.1 341.5,65.2 305.3,72.2 269.1,78.6 232.9,91.8 196.7,93.0 160.5,107.8 124.4,106.1 88.2,59.3 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,121.3 160.5,118.2 196.7,108.4 232.9,108.6 269.1,101.1 305.3,92.0 341.5,92.5 377.6,90.6 413.8,81.8 450.0,84.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,70.9 88.2,84.2 124.4,111.7 160.5,122.2 196.7,111.2 232.9,100.0 269.1,91.7 305.3,90.5 341.5,82.6 377.6,80.3 413.8,74.9 450.0,66.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.4 88.2,65.9 124.4,116.4 160.5,113.4 196.7,99.1 232.9,101.6 269.1,88.3 305.3,81.6 341.5,75.9 377.6,72.2 413.8,64.6 450.0,61.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,63.0 124.4,113.4 160.5,105.9 196.7,94.9 232.9,94.8 269.1,87.0 305.3,76.5 341.5,71.4 377.6,65.7 413.8,56.8 450.0,48.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,59.3 124.4,106.1 160.5,107.8 196.7,93.0 232.9,91.8 269.1,78.6 305.3,72.2 341.5,65.2 377.6,59.1 413.8,54.4 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 146 ns | 204 ns | 208 ns | 209 ns |
| D38 | 4.36 ns | 142 ns | 189 ns | 191 ns | 209 ns |
| D57 | 275 ns | 406 ns | 297 ns | 343 ns | 520 ns |
| D76 | 306 ns | 219 ns | 368 ns | 515 ns | 468 ns |
| D115 | 572 ns | 388 ns | 812 ns | 997 ns | 1.04 µs |
| D153 | 587 ns | 821 ns | 619 ns | 966 ns | 1.16 µs |
| D230 | 935 ns | 1.3 µs | 1.47 µs | 1.48 µs | 2.32 µs |
| D307 | 1.48 µs | 1.38 µs | 2.14 µs | 2.77 µs | 3.39 µs |
| D462 | 1.44 µs | 2.36 µs | 2.83 µs | 3.55 µs | 4.81 µs |
| D616 | 1.55 µs | 2.33 µs | 3.51 µs | 4.8 µs | 6.82 µs |
| D924 | 2.54 µs | 3.12 µs | 5.26 µs | 7.79 µs | 8.52 µs |
| D1232 | 2.14 µs | 5.03 µs | 5.76 µs | 12.1 µs | 32.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,112.4 160.5,110.5 196.7,99.7 232.9,99.3 269.1,91.2 305.3,83.2 341.5,83.7 377.6,82.3 413.8,73.8 450.0,76.8 450.0,29.7 413.8,52.8 377.6,56.6 341.5,62.7 305.3,68.8 269.1,75.4 232.9,87.4 196.7,89.2 160.5,103.2 124.4,101.4 88.2,117.2 52.0,117.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,112.4 160.5,110.5 196.7,99.7 232.9,99.3 269.1,91.2 305.3,83.2 341.5,83.7 377.6,82.3 413.8,73.8 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,123.9 124.4,105.6 160.5,116.4 196.7,106.4 232.9,93.4 269.1,85.5 305.3,84.4 341.5,75.0 377.6,75.3 413.8,70.3 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.6 88.2,118.9 124.4,111.1 160.5,107.4 196.7,93.6 232.9,98.3 269.1,83.3 305.3,76.8 341.5,71.9 377.6,68.2 413.8,61.2 450.0,59.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.3 88.2,118.8 124.4,108.6 160.5,101.5 196.7,90.0 232.9,90.6 269.1,83.2 305.3,72.3 341.5,68.0 377.6,62.8 413.8,54.3 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.2 88.2,117.2 124.4,101.4 160.5,103.2 196.7,89.2 232.9,87.4 269.1,75.4 305.3,68.8 341.5,62.7 377.6,56.6 413.8,52.8 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
