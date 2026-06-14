# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.17 ns | 23.3 µs | 27.7 µs | 30.3 µs | 39.5 µs |
| D38 | 4.59 µs | 7.63 µs | 8.31 µs | 11.6 µs | 15.1 µs |
| D57 | 4.05 µs | 8.34 µs | 10.9 µs | 16.3 µs | 19.2 µs |
| D76 | 4.77 µs | 8.5 µs | 14.2 µs | 19.4 µs | 22.1 µs |
| D115 | 5.2 µs | 12.1 µs | 24.1 µs | 31.4 µs | 43.9 µs |
| D153 | 4.28 µs | 14.1 µs | 28.6 µs | 25 µs | 64.3 µs |
| D230 | 4.73 µs | 25.7 µs | 44.1 µs | 67.7 µs | 121 µs |
| D307 | 4.74 µs | 28.7 µs | 56.4 µs | 121 µs | 178 µs |
| D462 | 5.37 µs | 44 µs | 122 µs | 198 µs | 372 µs |
| D616 | 5.44 µs | 64.2 µs | 219 µs | 423 µs | 654 µs |
| D924 | 4.93 µs | 124 µs | 420 µs | 930 µs | 1.53 ms |
| D1232 | 5.57 µs | 113 µs | 700 µs | 1.5 ms | 2.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,105.4 124.4,106.9 160.5,104.9 196.7,103.8 232.9,106.2 269.1,105.0 305.3,105.0 341.5,103.4 377.6,103.3 413.8,104.5 450.0,103.0 450.0,26.1 413.8,33.3 377.6,43.8 341.5,50.8 305.3,60.0 269.1,64.8 232.9,72.6 196.7,77.4 160.5,85.9 124.4,87.6 88.2,90.6 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,105.4 124.4,106.9 160.5,104.9 196.7,103.8 232.9,106.2 269.1,105.0 305.3,105.0 341.5,103.4 377.6,103.3 413.8,104.5 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.1 124.4,98.0 160.5,97.7 196.7,93.4 232.9,91.4 269.1,84.0 305.3,82.6 341.5,77.3 377.6,72.6 413.8,64.4 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,98.0 124.4,94.6 160.5,91.3 196.7,84.8 232.9,82.7 269.1,77.3 305.3,74.3 341.5,64.6 377.6,57.4 413.8,49.3 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.8 124.4,89.7 160.5,87.5 196.7,81.5 232.9,84.4 269.1,72.0 305.3,64.8 341.5,58.6 377.6,49.3 413.8,39.5 450.0,33.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.6 124.4,87.6 160.5,85.9 196.7,77.4 232.9,72.6 269.1,64.8 305.3,60.0 341.5,50.8 377.6,43.8 413.8,33.3 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 31.1 µs | 33.2 µs | 36.4 µs | 38.4 µs |
| D38 | 27.4 µs | 35.5 µs | 44.2 µs | 53.4 µs | 61.5 µs |
| D57 | 2.87 µs | 4.25 µs | 4.95 µs | 6.64 µs | 8.23 µs |
| D76 | 3.28 µs | 4.57 µs | 5.71 µs | 8.38 µs | 10.1 µs |
| D115 | 6.58 µs | 9.33 µs | 12.3 µs | 16.6 µs | 23.3 µs |
| D153 | 5.57 µs | 9.77 µs | 15.8 µs | 12.3 µs | 33.3 µs |
| D230 | 8.38 µs | 17 µs | 29 µs | 43 µs | 69 µs |
| D307 | 12.6 µs | 27.8 µs | 42.2 µs | 84.4 µs | 132 µs |
| D462 | 13.5 µs | 39.2 µs | 85.5 µs | 133 µs | 262 µs |
| D616 | 22.2 µs | 76.3 µs | 179 µs | 314 µs | 519 µs |
| D924 | 31.9 µs | 155 µs | 382 µs | 798 µs | 1.34 ms |
| D1232 | 42.6 µs | 141 µs | 709 µs | 1.43 ms | 2.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.5 88.2,138.1 124.4,187.1 160.5,184.2 196.7,169.1 232.9,172.7 269.1,163.8 305.3,155.1 341.5,153.5 377.6,142.6 413.8,134.8 450.0,128.5 450.0,40.9 413.8,53.6 377.6,74.2 341.5,89.1 305.3,104.0 269.1,118.0 232.9,133.9 196.7,141.6 160.5,159.8 124.4,164.2 88.2,120.6 52.0,130.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.5 88.2,138.1 124.4,187.1 160.5,184.2 196.7,169.1 232.9,172.7 269.1,163.8 305.3,155.1 341.5,153.5 377.6,142.6 413.8,134.8 450.0,128.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,135.4 88.2,132.5 124.4,178.6 160.5,177.0 196.7,161.5 232.9,160.5 269.1,148.4 305.3,137.8 341.5,130.3 377.6,115.9 413.8,100.5 450.0,102.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,127.7 124.4,175.3 160.5,172.2 196.7,155.6 232.9,150.0 269.1,136.9 305.3,128.7 341.5,113.4 377.6,97.4 413.8,80.9 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,123.6 124.4,168.9 160.5,163.8 196.7,149.0 232.9,155.5 269.1,128.3 305.3,113.7 341.5,103.8 377.6,85.2 413.8,64.9 450.0,52.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.8 88.2,120.6 124.4,164.2 160.5,159.8 196.7,141.6 232.9,133.9 269.1,118.0 305.3,104.0 341.5,89.1 377.6,74.2 413.8,53.6 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 23.2 µs | 27.7 µs | 30.3 µs | 39.4 µs |
| D38 | 4.59 µs | 7.65 µs | 8.23 µs | 11.6 µs | 15 µs |
| D57 | 4.03 µs | 8.29 µs | 10.9 µs | 16.2 µs | 19.1 µs |
| D76 | 4.73 µs | 8.58 µs | 14.3 µs | 19.4 µs | 22.1 µs |
| D115 | 5.25 µs | 12.1 µs | 23.8 µs | 31.6 µs | 44.4 µs |
| D153 | 4.3 µs | 14.1 µs | 28.9 µs | 24.7 µs | 64 µs |
| D230 | 4.69 µs | 25.9 µs | 43.6 µs | 68.2 µs | 122 µs |
| D307 | 4.73 µs | 29.1 µs | 56.2 µs | 120 µs | 177 µs |
| D462 | 5.28 µs | 43.6 µs | 122 µs | 198 µs | 372 µs |
| D616 | 5.55 µs | 64.4 µs | 218 µs | 425 µs | 657 µs |
| D924 | 4.92 µs | 124 µs | 419 µs | 933 µs | 1.52 ms |
| D1232 | 5.56 µs | 114 µs | 698 µs | 1.5 ms | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,105.4 124.4,107.0 160.5,105.0 196.7,103.7 232.9,106.2 269.1,105.1 305.3,105.0 341.5,103.6 377.6,103.0 413.8,104.5 450.0,103.0 450.0,26.1 413.8,33.4 377.6,43.8 341.5,50.8 305.3,60.0 269.1,64.7 232.9,72.7 196.7,77.2 160.5,85.9 124.4,87.7 88.2,90.7 52.0,78.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,105.4 124.4,107.0 160.5,105.0 196.7,103.7 232.9,106.2 269.1,105.1 305.3,105.0 341.5,103.6 377.6,103.0 413.8,104.5 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.2 88.2,99.0 124.4,98.0 160.5,97.6 196.7,93.4 232.9,91.4 269.1,83.9 305.3,82.5 341.5,77.4 377.6,72.6 413.8,64.5 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,98.1 124.4,94.7 160.5,91.3 196.7,85.0 232.9,82.5 269.1,77.4 305.3,74.3 341.5,64.7 377.6,57.5 413.8,49.4 450.0,43.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.0 88.2,93.9 124.4,89.7 160.5,87.5 196.7,81.4 232.9,84.5 269.1,71.9 305.3,64.8 341.5,58.7 377.6,49.2 413.8,39.4 450.0,33.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.7 88.2,90.7 124.4,87.7 160.5,85.9 196.7,77.2 232.9,72.7 269.1,64.7 305.3,60.0 341.5,50.8 377.6,43.8 413.8,33.4 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 23 µs | 27.1 µs | 29.7 µs | 38.7 µs |
| D38 | 4.22 ns | 29 µs | 36.2 µs | 39.9 µs | 51.3 µs |
| D57 | 1.64 ns | 5.31 µs | 7.29 µs | 9.74 µs | 12.7 µs |
| D76 | 2.01 ns | 6.18 µs | 8.74 µs | 12.9 µs | 15.2 µs |
| D115 | 12.4 ns | 14 µs | 18.9 µs | 24.3 µs | 36.3 µs |
| D153 | 14.6 ns | 14.9 µs | 24.4 µs | 20.2 µs | 47.8 µs |
| D230 | 28 ns | 26.3 µs | 44.7 µs | 61.9 µs | 95.1 µs |
| D307 | 44.8 ns | 39.4 µs | 58.3 µs | 113 µs | 163 µs |
| D462 | 75.3 ns | 58.1 µs | 119 µs | 164 µs | 316 µs |
| D616 | 88.7 ns | 112 µs | 244 µs | 402 µs | 618 µs |
| D924 | 103 ns | 233 µs | 498 µs | 950 µs | 1.47 ms |
| D1232 | 155 ns | 200 µs | 923 µs | 1.62 ms | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,203.9 160.5,201.3 196.7,178.7 232.9,176.8 269.1,168.6 305.3,162.8 341.5,156.4 377.6,154.3 413.8,152.5 450.0,147.4 450.0,27.5 413.8,33.8 377.6,44.5 341.5,52.9 305.3,61.1 269.1,67.8 232.9,76.3 196.7,79.7 160.5,90.5 124.4,92.7 88.2,75.4 52.0,78.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,203.9 160.5,201.3 196.7,178.7 232.9,176.8 269.1,168.6 305.3,162.8 341.5,156.4 377.6,154.3 413.8,152.5 450.0,147.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.4 88.2,82.5 124.4,103.6 160.5,101.7 196.7,91.5 232.9,90.8 269.1,83.7 305.3,78.7 341.5,73.9 377.6,65.7 413.8,56.6 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,79.8 124.4,99.6 160.5,97.4 196.7,87.8 232.9,84.6 269.1,77.1 305.3,73.8 341.5,65.0 377.6,56.1 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.2 88.2,78.5 124.4,96.0 160.5,92.5 196.7,84.7 232.9,87.0 269.1,73.1 305.3,65.7 341.5,61.0 377.6,49.9 413.8,39.2 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,78.9 88.2,75.4 124.4,92.7 160.5,90.5 196.7,79.7 232.9,76.3 269.1,67.8 305.3,61.1 341.5,52.9 377.6,44.5 413.8,33.8 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.63 µs | 2.57 µs | 2.82 µs | 3.19 µs |
| D38 | 6.24 µs | 8.94 µs | 6.9 µs | 9.89 µs | 13.1 µs |
| D57 | 3.35 µs | 7.04 µs | 9.45 µs | 14.4 µs | 5.5 µs |
| D76 | 3.88 µs | 7.38 µs | 12.4 µs | 17.1 µs | 19.6 µs |
| D115 | 4.27 µs | 10.4 µs | 21.4 µs | 29 µs | 39.3 µs |
| D153 | 3.49 µs | 12.4 µs | 22.1 µs | 23.1 µs | 59.2 µs |
| D230 | 3.88 µs | 22.6 µs | 39.7 µs | 62.2 µs | 113 µs |
| D307 | 3.92 µs | 26 µs | 46.9 µs | 112 µs | 164 µs |
| D462 | 3.18 µs | 38.4 µs | 103 µs | 173 µs | 319 µs |
| D616 | 4.43 µs | 60.5 µs | 202 µs | 393 µs | 619 µs |
| D924 | 4.08 µs | 114 µs | 390 µs | 890 µs | 1.44 ms |
| D1232 | 4.6 µs | 105 µs | 662 µs | 1.44 ms | 2.61 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,101.6 124.4,109.3 160.5,107.5 196.7,106.3 232.9,108.8 269.1,107.5 305.3,107.3 341.5,109.9 377.6,105.8 413.8,106.8 450.0,105.3 450.0,26.6 413.8,34.1 377.6,44.5 341.5,52.7 305.3,61.0 269.1,65.6 232.9,73.6 196.7,78.7 160.5,87.4 124.4,103.1 88.2,92.4 52.0,109.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,101.6 124.4,109.3 160.5,107.5 196.7,106.3 232.9,108.8 269.1,107.5 305.3,107.3 341.5,109.9 377.6,105.8 413.8,106.8 450.0,105.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,97.1 124.4,100.1 160.5,99.5 196.7,95.2 232.9,93.1 269.1,85.6 305.3,83.9 341.5,79.0 377.6,73.4 413.8,65.5 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,100.3 124.4,96.4 160.5,93.0 196.7,86.2 232.9,85.9 269.1,78.6 305.3,76.5 341.5,66.8 377.6,58.4 413.8,50.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,95.9 124.4,91.2 160.5,89.1 196.7,82.5 232.9,85.3 269.1,73.0 305.3,65.8 341.5,60.3 377.6,50.1 413.8,40.0 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,92.4 124.4,103.1 160.5,87.4 196.7,78.7 232.9,73.6 269.1,65.6 305.3,61.0 341.5,52.7 377.6,44.5 413.8,34.1 450.0,26.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.81 µs | 8.83 µs | 9.69 µs | 11.3 µs |
| D38 | 4.22 ns | 9.4 µs | 11.8 µs | 12.9 µs | 15 µs |
| D57 | 473 ns | 5.39 µs | 6.9 µs | 9.08 µs | 11.9 µs |
| D76 | 501 ns | 5.94 µs | 8.02 µs | 11.8 µs | 14.9 µs |
| D115 | 1.23 µs | 12.9 µs | 17.9 µs | 25 µs | 36.5 µs |
| D153 | 885 ns | 13.5 µs | 23.7 µs | 18.9 µs | 53.8 µs |
| D230 | 1.35 µs | 24.8 µs | 44.4 µs | 71.4 µs | 117 µs |
| D307 | 2.21 µs | 41.8 µs | 69.2 µs | 145 µs | 234 µs |
| D462 | 2.56 µs | 59.4 µs | 146 µs | 237 µs | 474 µs |
| D616 | 3.99 µs | 122 µs | 312 µs | 562 µs | 948 µs |
| D924 | 5.55 µs | 257 µs | 680 µs | 1.46 ms | 2.5 ms |
| D1232 | 7.62 µs | 241 µs | 1.28 ms | 2.66 ms | 4.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,133.6 160.5,132.9 196.7,121.7 232.9,125.8 269.1,120.6 305.3,114.4 341.5,112.6 377.6,107.1 413.8,103.0 450.0,99.1 450.0,19.9 413.8,27.2 377.6,39.2 341.5,47.8 305.3,56.6 269.1,65.2 232.9,74.8 196.7,79.7 160.5,90.8 124.4,93.5 88.2,90.7 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,133.6 160.5,132.9 196.7,121.7 232.9,125.8 269.1,120.6 305.3,114.4 341.5,112.6 377.6,107.1 413.8,103.0 450.0,99.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.8 88.2,96.5 124.4,103.4 160.5,102.2 196.7,92.6 232.9,92.0 269.1,84.4 305.3,78.0 341.5,73.6 377.6,64.7 413.8,55.4 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,93.7 124.4,100.3 160.5,98.5 196.7,88.5 232.9,85.0 269.1,77.2 305.3,71.7 341.5,62.4 377.6,53.0 413.8,43.4 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.1 88.2,92.5 124.4,96.9 160.5,93.7 196.7,84.3 232.9,87.8 269.1,71.3 305.3,62.5 341.5,56.4 377.6,45.7 413.8,33.9 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,90.7 124.4,93.5 160.5,90.8 196.7,79.7 232.9,74.8 269.1,65.2 305.3,56.6 341.5,47.8 377.6,39.2 413.8,27.2 450.0,19.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 4.26 µs | 5.5 µs | 5.9 µs | 6.98 µs |
| D38 | 5.62 ns | 5.96 µs | 7.33 µs | 7.92 µs | 8.94 µs |
| D57 | 2.18 ns | 3.33 µs | 4.37 µs | 5.64 µs | 9.53 µs |
| D76 | 3.12 ns | 3.62 µs | 5.18 µs | 7.45 µs | 9.34 µs |
| D115 | 16.8 ns | 4.69 µs | 10 µs | 14.5 µs | 19.2 µs |
| D153 | 19.9 ns | 5.16 µs | 9.83 µs | 11 µs | 31.7 µs |
| D230 | 48.8 ns | 10.3 µs | 19.5 µs | 35.8 µs | 69.2 µs |
| D307 | 81.6 ns | 13.1 µs | 22.6 µs | 67.3 µs | 109 µs |
| D462 | 167 ns | 16.1 µs | 60.2 µs | 112 µs | 218 µs |
| D616 | 168 ns | 32.6 µs | 129 µs | 256 µs | 428 µs |
| D924 | 171 ns | 68.7 µs | 254 µs | 621 µs | 1.04 ms |
| D1232 | 388 ns | 65.3 µs | 453 µs | 1.04 ms | 1.92 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,188.6 124.4,200.3 160.5,195.9 196.7,175.0 232.9,172.9 269.1,161.8 305.3,155.4 341.5,146.5 377.6,146.4 413.8,146.2 450.0,136.0 450.0,30.5 413.8,38.1 377.6,49.1 341.5,57.5 305.3,66.1 269.1,71.7 232.9,81.4 196.7,87.6 160.5,96.6 124.4,96.3 88.2,97.1 52.0,100.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,188.6 124.4,200.3 160.5,195.9 196.7,175.0 232.9,172.9 269.1,161.8 305.3,155.4 341.5,146.5 377.6,146.4 413.8,146.2 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.3 88.2,102.1 124.4,109.4 160.5,108.3 196.7,105.1 232.9,103.9 269.1,95.3 305.3,92.3 341.5,89.8 377.6,81.0 413.8,71.8 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,99.6 124.4,106.0 160.5,103.9 196.7,95.7 232.9,95.9 269.1,87.4 305.3,85.6 341.5,73.4 377.6,64.0 413.8,55.6 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.3 88.2,98.6 124.4,102.8 160.5,99.4 196.7,91.1 232.9,94.5 269.1,79.9 305.3,72.1 341.5,65.7 377.6,55.5 413.8,44.5 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.2 88.2,97.1 124.4,96.3 160.5,96.6 196.7,87.6 232.9,81.4 269.1,71.7 305.3,66.1 341.5,57.5 377.6,49.1 413.8,38.1 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 7.91 µs | 9.43 µs | 10.2 µs | 11.9 µs |
| D38 | 4.57 ns | 10.2 µs | 12.5 µs | 13.4 µs | 15.3 µs |
| D57 | 1.92 ns | 5.18 µs | 7 µs | 8.35 µs | 10.9 µs |
| D76 | 3.43 ns | 5.57 µs | 7.36 µs | 10.9 µs | 12.8 µs |
| D115 | 10.9 ns | 12.5 µs | 11.3 µs | 21.4 µs | 28.2 µs |
| D153 | 19.9 ns | 7.44 µs | 16.1 µs | 12.9 µs | 38.5 µs |
| D230 | 48.8 ns | 14.4 µs | 24.5 µs | 42.3 µs | 78.7 µs |
| D307 | 82.6 ns | 17 µs | 42.3 µs | 79 µs | 117 µs |
| D462 | 168 ns | 24.6 µs | 80.4 µs | 131 µs | 231 µs |
| D616 | 170 ns | 39.7 µs | 142 µs | 270 µs | 412 µs |
| D924 | 177 ns | 82.2 µs | 268 µs | 605 µs | 912 µs |
| D1232 | 399 ns | 71.5 µs | 448 µs | 911 µs | 2.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,191.1 124.4,201.9 160.5,194.7 196.7,180.4 232.9,172.9 269.1,161.8 305.3,155.2 341.5,146.4 377.6,146.3 413.8,145.8 450.0,135.7 450.0,28.7 413.8,39.7 377.6,49.6 341.5,56.8 305.3,65.2 269.1,70.1 232.9,79.0 196.7,82.9 160.5,92.6 124.4,94.7 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,191.1 124.4,201.9 160.5,194.7 196.7,180.4 232.9,172.9 269.1,161.8 305.3,155.2 341.5,146.4 377.6,146.3 413.8,145.8 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,103.9 160.5,103.0 196.7,93.0 232.9,99.4 269.1,91.2 305.3,89.2 341.5,84.5 377.6,78.6 413.8,69.6 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,93.0 124.4,100.1 160.5,99.5 196.7,94.1 232.9,89.8 269.1,84.6 305.3,77.8 341.5,69.9 377.6,62.8 413.8,54.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.5 88.2,92.1 124.4,97.9 160.5,94.6 196.7,86.3 232.9,92.5 269.1,77.8 305.3,70.1 341.5,63.8 377.6,54.8 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,94.7 160.5,92.6 196.7,82.9 232.9,79.0 269.1,70.1 305.3,65.2 341.5,56.8 377.6,49.6 413.8,39.7 450.0,28.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 4.12 µs | 5.32 µs | 5.85 µs | 6.9 µs |
| D38 | 4.92 ns | 5.77 µs | 7.11 µs | 7.74 µs | 8.9 µs |
| D57 | 2.16 ns | 3.19 µs | 4.1 µs | 5.63 µs | 9.52 µs |
| D76 | 3.74 ns | 3.36 µs | 4.87 µs | 7.34 µs | 8.94 µs |
| D115 | 16.9 ns | 4.45 µs | 9.49 µs | 13.9 µs | 18.5 µs |
| D153 | 20.3 ns | 4.85 µs | 9.82 µs | 10.7 µs | 31.1 µs |
| D230 | 48.8 ns | 10.5 µs | 19.5 µs | 35.5 µs | 67.6 µs |
| D307 | 76.9 ns | 12.1 µs | 21.9 µs | 66.2 µs | 107 µs |
| D462 | 158 ns | 15.7 µs | 58.9 µs | 113 µs | 215 µs |
| D616 | 161 ns | 32.4 µs | 129 µs | 253 µs | 427 µs |
| D924 | 165 ns | 68.6 µs | 250 µs | 615 µs | 1.03 ms |
| D1232 | 396 ns | 64.6 µs | 448 µs | 1.03 ms | 1.9 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,190.2 124.4,200.4 160.5,193.6 196.7,174.9 232.9,172.6 269.1,161.8 305.3,156.1 341.5,147.2 377.6,147.0 413.8,146.7 450.0,135.8 450.0,30.6 413.8,38.1 377.6,49.1 341.5,57.6 305.3,66.2 269.1,72.0 232.9,81.6 196.7,88.1 160.5,97.1 124.4,96.3 88.2,97.2 52.0,100.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,190.2 124.4,200.4 160.5,193.6 196.7,174.9 232.9,172.6 269.1,161.8 305.3,156.1 341.5,147.2 377.6,147.0 413.8,146.7 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.7 88.2,102.5 124.4,109.9 160.5,109.3 196.7,105.8 232.9,104.7 269.1,95.1 305.3,93.3 341.5,90.2 377.6,81.1 413.8,71.8 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,99.9 124.4,106.8 160.5,104.6 196.7,96.4 232.9,95.9 269.1,87.4 305.3,86.0 341.5,73.7 377.6,64.0 413.8,55.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,98.9 124.4,102.8 160.5,99.5 196.7,91.6 232.9,94.9 269.1,80.0 305.3,72.3 341.5,65.6 377.6,55.6 413.8,44.6 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,97.2 124.4,96.3 160.5,97.1 196.7,88.1 232.9,81.6 269.1,72.0 305.3,66.2 341.5,57.6 377.6,49.1 413.8,38.1 450.0,30.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.92 µs | 9.43 µs | 10.2 µs | 11.9 µs |
| D38 | 4.57 ns | 10.2 µs | 12.5 µs | 13.4 µs | 15.3 µs |
| D57 | 9.47 ns | 5.2 µs | 7 µs | 8.4 µs | 10.9 µs |
| D76 | 10.5 ns | 5.58 µs | 7.45 µs | 10.9 µs | 12.8 µs |
| D115 | 11.2 ns | 12.5 µs | 11.5 µs | 21.1 µs | 27.8 µs |
| D153 | 19.8 ns | 7.51 µs | 15.7 µs | 13.1 µs | 38.4 µs |
| D230 | 49.7 ns | 14.5 µs | 24.5 µs | 42.8 µs | 79.2 µs |
| D307 | 76.7 ns | 17 µs | 42.2 µs | 79.5 µs | 116 µs |
| D462 | 152 ns | 25 µs | 81.1 µs | 131 µs | 231 µs |
| D616 | 160 ns | 39.8 µs | 142 µs | 271 µs | 412 µs |
| D924 | 175 ns | 82.2 µs | 269 µs | 606 µs | 910 µs |
| D1232 | 378 ns | 72.5 µs | 450 µs | 911 µs | 2.23 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,191.1 124.4,182.1 160.5,180.8 196.7,180.0 232.9,173.0 269.1,161.5 305.3,156.2 341.5,147.6 377.6,147.0 413.8,145.9 450.0,136.4 450.0,28.6 413.8,39.7 377.6,49.6 341.5,56.8 305.3,65.3 269.1,70.0 232.9,79.0 196.7,83.0 160.5,92.6 124.4,94.7 88.2,90.5 52.0,93.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,191.1 124.4,182.1 160.5,180.8 196.7,180.0 232.9,173.0 269.1,161.5 305.3,156.2 341.5,147.6 377.6,147.0 413.8,145.9 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.5 124.4,103.8 160.5,102.9 196.7,92.9 232.9,99.3 269.1,91.1 305.3,89.1 341.5,84.4 377.6,78.6 413.8,69.6 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,93.0 124.4,100.1 160.5,99.4 196.7,94.0 232.9,90.1 269.1,84.6 305.3,77.8 341.5,69.7 377.6,62.8 413.8,54.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.4 88.2,92.1 124.4,97.9 160.5,94.6 196.7,86.4 232.9,92.4 269.1,77.7 305.3,70.0 341.5,63.8 377.6,54.8 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.5 88.2,90.5 124.4,94.7 160.5,92.6 196.7,83.0 232.9,79.0 269.1,70.0 305.3,65.3 341.5,56.8 377.6,49.6 413.8,39.7 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 8.22 µs | 9.93 µs | 10.8 µs | 12.7 µs |
| D38 | 4.92 ns | 10.7 µs | 13.2 µs | 14.4 µs | 16.4 µs |
| D57 | 2.46 ns | 4.13 µs | 5.48 µs | 7.44 µs | 9.47 µs |
| D76 | 3.74 ns | 4.43 µs | 6.53 µs | 9.46 µs | 11.3 µs |
| D115 | 16.8 ns | 5.92 µs | 12.5 µs | 16 µs | 22.6 µs |
| D153 | 19.7 ns | 6.59 µs | 12.6 µs | 13 µs | 36.5 µs |
| D230 | 47.3 ns | 12.9 µs | 24.9 µs | 40.6 µs | 75.5 µs |
| D307 | 74.6 ns | 15.2 µs | 26.1 µs | 75.1 µs | 120 µs |
| D462 | 150 ns | 19.2 µs | 67.2 µs | 124 µs | 233 µs |
| D616 | 153 ns | 37.5 µs | 141 µs | 281 µs | 458 µs |
| D924 | 174 ns | 77.1 µs | 274 µs | 654 µs | 1.1 ms |
| D1232 | 386 ns | 71.2 µs | 483 µs | 1.11 ms | 2.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.2 124.4,198.9 160.5,193.6 196.7,175.0 232.9,173.0 269.1,162.1 305.3,156.5 341.5,147.9 377.6,147.6 413.8,146.0 450.0,136.1 450.0,29.9 413.8,37.4 377.6,48.3 341.5,56.7 305.3,64.9 269.1,70.6 232.9,79.6 196.7,85.6 160.5,94.2 124.4,96.4 88.2,89.6 52.0,92.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.2 124.4,198.9 160.5,193.6 196.7,175.0 232.9,173.0 269.1,162.1 305.3,156.5 341.5,147.9 377.6,147.6 413.8,146.0 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.2 88.2,94.8 124.4,106.7 160.5,105.8 196.7,102.2 232.9,100.9 269.1,92.5 305.3,90.5 341.5,87.6 377.6,79.3 413.8,70.4 450.0,71.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,92.2 124.4,103.2 160.5,101.0 196.7,92.9 232.9,92.9 269.1,84.4 305.3,83.8 341.5,72.1 377.6,62.9 413.8,54.6 450.0,47.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.8 88.2,91.2 124.4,99.4 160.5,96.4 196.7,89.9 232.9,92.5 269.1,78.3 305.3,70.7 341.5,64.5 377.6,54.3 413.8,43.8 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,92.8 88.2,89.6 124.4,96.4 160.5,94.2 196.7,85.6 232.9,79.6 269.1,70.6 305.3,64.9 341.5,56.7 377.6,48.3 413.8,37.4 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 7.93 µs | 9.57 µs | 10.4 µs | 12.1 µs |
| D38 | 4.22 ns | 10.3 µs | 12.5 µs | 13.4 µs | 15.3 µs |
| D57 | 2.28 µs | 5.31 µs | 7.27 µs | 8.7 µs | 11.3 µs |
| D76 | 2.63 µs | 5.7 µs | 7.71 µs | 11.2 µs | 13.1 µs |
| D115 | 5.84 µs | 13 µs | 11.8 µs | 21.5 µs | 29.1 µs |
| D153 | 2.58 µs | 7.82 µs | 16.2 µs | 13.3 µs | 40.4 µs |
| D230 | 2.92 µs | 14.9 µs | 24.7 µs | 43.4 µs | 80.3 µs |
| D307 | 3.16 µs | 17.9 µs | 43.9 µs | 80.5 µs | 118 µs |
| D462 | 3.61 µs | 25.2 µs | 82.1 µs | 133 µs | 233 µs |
| D616 | 3.72 µs | 40.4 µs | 143 µs | 272 µs | 416 µs |
| D924 | 3.77 µs | 83.5 µs | 271 µs | 613 µs | 917 µs |
| D1232 | 4.75 µs | 73.7 µs | 452 µs | 917 µs | 2.23 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,114.1 160.5,112.3 196.7,102.4 232.9,112.5 269.1,111.0 305.3,110.0 341.5,108.4 377.6,108.0 413.8,107.8 450.0,104.9 450.0,28.6 413.8,39.6 377.6,49.5 341.5,56.6 305.3,65.1 269.1,69.9 232.9,78.4 196.7,82.5 160.5,92.3 124.4,94.2 88.2,90.5 52.0,93.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,114.1 160.5,112.3 196.7,102.4 232.9,112.5 269.1,111.0 305.3,110.0 341.5,108.4 377.6,108.0 413.8,107.8 450.0,104.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,98.6 88.2,95.3 124.4,103.6 160.5,102.7 196.7,92.5 232.9,98.8 269.1,90.7 305.3,88.5 341.5,84.2 377.6,78.4 413.8,69.4 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.3 88.2,93.0 124.4,99.7 160.5,98.9 196.7,93.6 232.9,89.8 269.1,84.5 305.3,77.4 341.5,69.6 377.6,62.7 413.8,54.8 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.0 124.4,97.4 160.5,94.3 196.7,86.2 232.9,92.2 269.1,77.5 305.3,69.8 341.5,63.6 377.6,54.7 413.8,44.6 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.3 88.2,90.5 124.4,94.2 160.5,92.3 196.7,82.5 232.9,78.4 269.1,69.9 305.3,65.1 341.5,56.6 377.6,49.5 413.8,39.6 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 3.22 µs | 3.23 µs | 3.53 µs | 4.14 µs |
| D38 | 4.57 ns | 3.47 µs | 4.3 µs | 4.72 µs | 5.43 µs |
| D57 | 152 ns | 292 ns | 301 ns | 336 ns | 436 ns |
| D76 | 180 ns | 278 ns | 309 ns | 432 ns | 465 ns |
| D115 | 381 ns | 526 ns | 586 ns | 690 ns | 841 ns |
| D153 | 302 ns | 493 ns | 702 ns | 443 ns | 1.02 µs |
| D230 | 518 ns | 874 ns | 1.04 µs | 1.23 µs | 1.73 µs |
| D307 | 834 ns | 1.24 µs | 1.35 µs | 2.03 µs | 2.55 µs |
| D462 | 976 ns | 1.5 µs | 2.11 µs | 2.53 µs | 3.85 µs |
| D616 | 1.17 µs | 1.92 µs | 3.02 µs | 4.04 µs | 5.53 µs |
| D924 | 1.44 µs | 2.63 µs | 4.32 µs | 7.48 µs | 10.2 µs |
| D1232 | 2.27 µs | 2.07 µs | 7.24 µs | 10.8 µs | 23.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,183.6 124.4,122.7 160.5,119.8 196.7,106.8 232.9,110.8 269.1,101.4 305.3,93.2 341.5,90.4 377.6,87.3 413.8,83.7 450.0,75.7 450.0,35.0 413.8,49.7 377.6,60.3 341.5,66.6 305.3,73.8 269.1,80.5 232.9,89.6 196.7,93.0 160.5,103.3 124.4,104.4 88.2,60.6 52.0,65.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,183.6 124.4,122.7 160.5,119.8 196.7,106.8 232.9,110.8 269.1,101.4 305.3,93.2 341.5,90.4 377.6,87.3 413.8,83.7 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,69.7 88.2,68.4 124.4,111.4 160.5,112.2 196.7,101.2 232.9,102.3 269.1,92.3 305.3,86.2 341.5,83.0 377.6,78.7 413.8,73.2 450.0,77.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,64.6 124.4,110.9 160.5,110.4 196.7,99.3 232.9,96.1 269.1,89.3 305.3,84.8 341.5,77.0 377.6,70.8 413.8,64.6 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,68.1 88.2,63.1 124.4,109.0 160.5,104.6 196.7,96.4 232.9,104.2 269.1,86.4 305.3,77.7 341.5,73.9 377.6,65.7 413.8,55.1 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,65.3 88.2,60.6 124.4,104.4 160.5,103.3 196.7,93.0 232.9,89.6 269.1,80.5 305.3,73.8 341.5,66.6 377.6,60.3 413.8,49.7 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 158 ns | 183 ns | 184 ns | 206 ns |
| D38 | 4.92 ns | 201 ns | 201 ns | 187 ns | 187 ns |
| D57 | 239 ns | 394 ns | 408 ns | 454 ns | 532 ns |
| D76 | 272 ns | 378 ns | 418 ns | 551 ns | 597 ns |
| D115 | 614 ns | 755 ns | 799 ns | 919 ns | 1.06 µs |
| D153 | 483 ns | 717 ns | 922 ns | 545 ns | 1.27 µs |
| D230 | 877 ns | 1.22 µs | 1.38 µs | 1.64 µs | 2.11 µs |
| D307 | 1.4 µs | 1.78 µs | 1.79 µs | 2.6 µs | 3.12 µs |
| D462 | 1.56 µs | 1.98 µs | 2.68 µs | 3.07 µs | 4.48 µs |
| D616 | 1.88 µs | 2.49 µs | 3.81 µs | 4.8 µs | 6.3 µs |
| D924 | 2.36 µs | 3.54 µs | 5.32 µs | 8.57 µs | 11.2 µs |
| D1232 | 3.55 µs | 2.67 µs | 8.53 µs | 12.3 µs | 25 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,182.3 124.4,114.9 160.5,112.6 196.7,98.5 232.9,102.6 269.1,92.3 305.3,84.1 341.5,82.3 377.6,79.0 413.8,75.1 450.0,68.0 450.0,34.1 413.8,48.0 377.6,58.0 341.5,64.0 305.3,70.3 269.1,77.0 232.9,85.8 196.7,89.0 160.5,98.9 124.4,101.0 88.2,119.1 52.0,117.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,182.3 124.4,114.9 160.5,112.6 196.7,98.5 232.9,102.6 269.1,92.3 305.3,84.1 341.5,82.3 377.6,79.0 413.8,75.1 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.0 88.2,117.9 124.4,106.2 160.5,106.9 196.7,94.9 232.9,95.8 269.1,86.5 305.3,79.9 341.5,78.1 377.6,74.2 413.8,68.0 450.0,72.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.5 88.2,117.9 124.4,105.6 160.5,105.2 196.7,93.9 232.9,91.4 269.1,84.4 305.3,79.8 341.5,72.9 377.6,66.8 413.8,61.0 450.0,52.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.4 88.2,119.2 124.4,103.7 160.5,100.4 196.7,91.5 232.9,100.5 269.1,81.4 305.3,73.4 341.5,70.5 377.6,62.8 413.8,52.7 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,119.1 124.4,101.0 160.5,98.9 196.7,89.0 232.9,85.8 269.1,77.0 305.3,70.3 341.5,64.0 377.6,58.0 413.8,48.0 450.0,34.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
