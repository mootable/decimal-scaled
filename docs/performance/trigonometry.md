# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.01 ns | 2 µs | 3.21 µs | 3.49 µs | 4.08 µs |
| D38 | 1.37 µs | 4.65 µs | 4.52 µs | 8.14 µs | 9.75 µs |
| D57 | 1.49 µs | 6.32 µs | 8.02 µs | 10.4 µs | 15.1 µs |
| D76 | 1.46 µs | 5.83 µs | 9.76 µs | 11.8 µs | 18.4 µs |
| D115 | 1.45 µs | 4.79 µs | 20 µs | 34.5 µs | 45.1 µs |
| D153 | 1.5 µs | 8.2 µs | 26.3 µs | 42.1 µs | 61.1 µs |
| D230 | 1.63 µs | 20 µs | 41.9 µs | 48.6 µs | 121 µs |
| D307 | 1.43 µs | 25 µs | 53.9 µs | 124 µs | 181 µs |
| D462 | 1.65 µs | 45.2 µs | 124 µs | 241 µs | 402 µs |
| D616 | 948 ns | 66.8 µs | 207 µs | 428 µs | 710 µs |
| D924 | 1.8 µs | 126 µs | 429 µs | 751 µs | 1.02 ms |
| D1232 | 2.02 µs | 206 µs | 711 µs | 1.35 ms | 1.9 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,120.3 124.4,119.3 160.5,119.6 196.7,119.7 232.9,119.2 269.1,118.2 305.3,119.8 341.5,118.1 377.6,124.9 413.8,117.0 450.0,115.6 450.0,30.6 413.8,38.4 377.6,42.8 341.5,49.9 305.3,59.8 269.1,64.8 232.9,73.3 196.7,77.0 160.5,88.2 124.4,90.6 88.2,96.0 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,120.3 124.4,119.3 160.5,119.6 196.7,119.7 232.9,119.2 269.1,118.2 305.3,119.8 341.5,118.1 377.6,124.9 413.8,117.0 450.0,115.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.2 124.4,101.4 160.5,102.4 196.7,104.8 232.9,98.2 269.1,87.1 305.3,84.3 341.5,77.0 377.6,72.1 413.8,64.3 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,105.6 124.4,98.4 160.5,96.0 196.7,87.1 232.9,83.7 269.1,77.9 305.3,74.8 341.5,64.5 377.6,58.1 413.8,49.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.3 124.4,95.3 160.5,93.7 196.7,80.3 232.9,77.9 269.1,76.1 305.3,64.5 341.5,56.2 377.6,49.1 413.8,42.1 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,96.0 124.4,90.6 160.5,88.2 196.7,77.0 232.9,73.3 269.1,64.8 305.3,59.8 341.5,49.9 377.6,42.8 413.8,38.4 450.0,30.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.24 µs | 3.61 µs | 5.88 µs | 6.41 µs | 7.13 µs |
| D38 | 3.25 µs | 5.86 µs | 7.42 µs | 8.36 µs | 9.65 µs |
| D57 | 3.33 µs | 4.71 µs | 4.95 µs | 6.02 µs | 8.23 µs |
| D76 | 3.36 µs | 3.79 µs | 5.8 µs | 6.43 µs | 9.3 µs |
| D115 | 6.08 µs | 5.56 µs | 12.3 µs | 18.1 µs | 23.1 µs |
| D153 | 6.25 µs | 8.76 µs | 15.6 µs | 21.8 µs | 29.9 µs |
| D230 | 8.9 µs | 15.7 µs | 26 µs | 27.6 µs | 68.8 µs |
| D307 | 12.6 µs | 27 µs | 40 µs | 85 µs | 133 µs |
| D462 | 13.3 µs | 38.5 µs | 84.6 µs | 161 µs | 274 µs |
| D616 | 14 µs | 75.8 µs | 170 µs | 314 µs | 554 µs |
| D924 | 34.6 µs | 158 µs | 387 µs | 635 µs | 892 µs |
| D1232 | 42.7 µs | 258 µs | 718 µs | 1.29 ms | 1.63 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,184.4 124.4,183.9 160.5,183.7 196.7,170.8 232.9,170.2 269.1,162.5 305.3,154.9 341.5,153.8 377.6,152.7 413.8,133.0 450.0,128.5 450.0,49.5 413.8,62.5 377.6,72.8 341.5,88.1 305.3,103.7 269.1,118.1 232.9,136.2 196.7,141.8 160.5,161.6 124.4,164.2 88.2,160.8 52.0,167.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,184.4 124.4,183.9 160.5,183.7 196.7,170.8 232.9,170.2 269.1,162.5 305.3,154.9 341.5,153.8 377.6,152.7 413.8,133.0 450.0,128.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.1 88.2,171.6 124.4,176.3 160.5,181.1 196.7,172.8 232.9,162.9 269.1,150.2 305.3,138.4 341.5,130.7 377.6,116.0 413.8,100.1 450.0,89.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.5 88.2,166.5 124.4,175.3 160.5,171.8 196.7,155.5 232.9,150.3 269.1,139.2 305.3,129.9 341.5,113.6 377.6,98.5 413.8,80.6 450.0,67.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.9 124.4,171.0 160.5,169.6 196.7,147.2 232.9,143.1 269.1,138.0 305.3,113.5 341.5,99.7 377.6,85.1 413.8,69.9 450.0,54.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.3 88.2,160.8 124.4,164.2 160.5,161.6 196.7,141.8 232.9,136.2 269.1,118.1 305.3,103.7 341.5,88.1 377.6,72.8 413.8,62.5 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.99 µs | 3.21 µs | 3.48 µs | 4.08 µs |
| D38 | 1.2 µs | 4.6 µs | 4.48 µs | 8.08 µs | 9.68 µs |
| D57 | 1.33 µs | 6.29 µs | 8.02 µs | 10.3 µs | 15 µs |
| D76 | 1.32 µs | 5.82 µs | 9.7 µs | 11.8 µs | 18.4 µs |
| D115 | 1.32 µs | 4.78 µs | 19.9 µs | 35.4 µs | 44.8 µs |
| D153 | 1.38 µs | 8.17 µs | 25.8 µs | 41.9 µs | 61.2 µs |
| D230 | 1.53 µs | 19.9 µs | 42 µs | 47.5 µs | 122 µs |
| D307 | 1.35 µs | 24.4 µs | 54 µs | 124 µs | 180 µs |
| D462 | 1.56 µs | 44.8 µs | 125 µs | 239 µs | 402 µs |
| D616 | 917 ns | 66.1 µs | 208 µs | 427 µs | 711 µs |
| D924 | 1.69 µs | 126 µs | 431 µs | 743 µs | 1.02 ms |
| D1232 | 1.88 µs | 205 µs | 709 µs | 1.34 ms | 1.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,122.0 124.4,120.8 160.5,120.8 196.7,120.8 232.9,120.3 269.1,119.0 305.3,120.5 341.5,118.7 377.6,125.4 413.8,117.8 450.0,116.4 450.0,30.6 413.8,38.4 377.6,42.8 341.5,49.9 305.3,59.8 269.1,64.7 232.9,73.2 196.7,77.1 160.5,88.2 124.4,90.7 88.2,96.1 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,122.0 124.4,120.8 160.5,120.8 196.7,120.8 232.9,120.3 269.1,119.0 305.3,120.5 341.5,118.7 377.6,125.4 413.8,117.8 450.0,116.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.4 124.4,101.5 160.5,102.4 196.7,104.9 232.9,98.2 269.1,87.2 305.3,84.6 341.5,77.1 377.6,72.3 413.8,64.2 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,105.7 124.4,98.5 160.5,96.1 196.7,87.2 232.9,84.0 269.1,77.9 305.3,74.8 341.5,64.4 377.6,58.1 413.8,49.0 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.4 124.4,95.3 160.5,93.7 196.7,80.0 232.9,77.9 269.1,76.4 305.3,64.5 341.5,56.3 377.6,49.1 413.8,42.3 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,96.1 124.4,90.7 160.5,88.2 196.7,77.1 232.9,73.2 269.1,64.7 305.3,59.8 341.5,49.9 377.6,42.8 413.8,38.4 450.0,30.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.72 µs | 2.52 µs | 2.81 µs | 3.12 µs |
| D38 | 3.73 ns | 2.55 µs | 3.26 µs | 3.72 µs | 4.27 µs |
| D57 | 2.51 ns | 5.88 µs | 7.33 µs | 8.83 µs | 12.9 µs |
| D76 | 3.21 ns | 5.26 µs | 8.58 µs | 9.76 µs | 13.8 µs |
| D115 | 8.1 ns | 8.32 µs | 19 µs | 26.8 µs | 36.2 µs |
| D153 | 10.6 ns | 13.9 µs | 23.5 µs | 33.3 µs | 43.8 µs |
| D230 | 19.5 ns | 23.8 µs | 41.1 µs | 40.3 µs | 94.3 µs |
| D307 | 24.5 ns | 37.6 µs | 56.7 µs | 115 µs | 167 µs |
| D462 | 49.3 ns | 59.9 µs | 118 µs | 197 µs | 337 µs |
| D616 | 33.7 ns | 111 µs | 226 µs | 399 µs | 665 µs |
| D924 | 36.1 ns | 231 µs | 500 µs | 744 µs | 940 µs |
| D1232 | 66.3 ns | 359 µs | 915 µs | 1.38 ms | 1.71 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.7 124.4,198.6 160.5,195.5 196.7,184.0 232.9,180.7 269.1,173.2 305.3,170.3 341.5,161.6 377.6,166.4 413.8,165.5 450.0,158.0 450.0,31.9 413.8,39.3 377.6,43.6 341.5,52.1 305.3,60.8 269.1,67.9 232.9,77.4 196.7,79.8 160.5,91.7 124.4,92.5 88.2,106.3 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.7 124.4,198.6 160.5,195.5 196.7,184.0 232.9,180.7 269.1,173.2 305.3,170.3 341.5,161.6 377.6,166.4 413.8,165.5 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,112.7 124.4,102.3 160.5,103.7 196.7,98.0 232.9,91.6 269.1,84.9 305.3,79.3 341.5,73.5 377.6,65.9 413.8,56.8 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,109.6 124.4,99.6 160.5,97.6 196.7,87.7 232.9,85.1 269.1,78.2 305.3,74.2 341.5,65.0 377.6,57.0 413.8,47.2 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,108.0 124.4,97.3 160.5,96.0 196.7,83.5 232.9,80.8 269.1,78.4 305.3,65.4 341.5,58.8 377.6,50.0 413.8,42.2 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,106.3 124.4,92.5 160.5,91.7 196.7,79.8 232.9,77.4 269.1,67.9 305.3,60.8 341.5,52.1 377.6,43.6 413.8,39.3 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.57 µs | 2.69 µs | 2.95 µs | 3.5 µs |
| D38 | 2.8 ns | 2.8 µs | 3.36 µs | 3.5 µs | 4.06 µs |
| D57 | 1.73 ns | 3.12 µs | 3.55 µs | 4.1 µs | 5.65 µs |
| D76 | 537 ns | 4.66 µs | 7.65 µs | 9.65 µs | 15.3 µs |
| D115 | 537 ns | 3.68 µs | 16.3 µs | 31.2 µs | 40.9 µs |
| D153 | 560 ns | 6.41 µs | 16.9 µs | 38.1 µs | 55.6 µs |
| D230 | 649 ns | 16.7 µs | 37.9 µs | 43.6 µs | 113 µs |
| D307 | 579 ns | 20.9 µs | 45.4 µs | 115 µs | 168 µs |
| D462 | 796 ns | 36.6 µs | 104 µs | 210 µs | 340 µs |
| D616 | 439 ns | 61.1 µs | 193 µs | 399 µs | 675 µs |
| D924 | 811 ns | 117 µs | 400 µs | 704 µs | 963 µs |
| D1232 | 996 ns | 191 µs | 669 µs | 1.29 ms | 1.81 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,197.2 124.4,203.2 160.5,132.0 196.7,132.0 232.9,131.5 269.1,129.6 305.3,131.1 341.5,127.1 377.6,134.5 413.8,126.9 450.0,124.3 450.0,31.2 413.8,39.0 377.6,43.4 341.5,52.0 305.3,60.7 269.1,65.6 232.9,74.4 196.7,78.2 160.5,90.5 124.4,102.8 88.2,106.9 52.0,108.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,197.2 124.4,203.2 160.5,132.0 196.7,132.0 232.9,131.5 269.1,129.6 305.3,131.1 341.5,127.1 377.6,134.5 413.8,126.9 450.0,124.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,111.5 124.4,110.1 160.5,105.2 196.7,108.1 232.9,101.2 269.1,89.3 305.3,86.6 341.5,79.6 377.6,73.3 413.8,65.2 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.3 124.4,108.6 160.5,99.0 196.7,89.7 232.9,89.2 269.1,79.2 305.3,76.9 341.5,66.6 377.6,59.0 413.8,49.9 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,108.8 124.4,106.8 160.5,96.2 196.7,81.6 232.9,79.1 269.1,77.4 305.3,65.4 341.5,57.9 377.6,50.0 413.8,42.9 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,106.9 124.4,102.8 160.5,90.5 196.7,78.2 232.9,74.4 269.1,65.6 305.3,60.7 341.5,52.0 377.6,43.4 413.8,39.0 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.74 µs | 2.88 µs | 3.17 µs | 3.6 µs |
| D38 | 3.74 ns | 2.86 µs | 3.75 µs | 4.15 µs | 4.85 µs |
| D57 | 446 ns | 6.11 µs | 6.89 µs | 8.58 µs | 12.1 µs |
| D76 | 443 ns | 5.06 µs | 8.19 µs | 9.24 µs | 14 µs |
| D115 | 883 ns | 7.52 µs | 18.3 µs | 27.6 µs | 36.3 µs |
| D153 | 914 ns | 12.1 µs | 23.7 µs | 34.1 µs | 48.8 µs |
| D230 | 1.41 µs | 22.3 µs | 40.4 µs | 46.5 µs | 117 µs |
| D307 | 1.92 µs | 41.2 µs | 66.7 µs | 147 µs | 239 µs |
| D462 | 2.14 µs | 60.1 µs | 145 µs | 288 µs | 499 µs |
| D616 | 2.15 µs | 122 µs | 298 µs | 564 µs | 1.01 ms |
| D924 | 5.38 µs | 265 µs | 697 µs | 1.17 ms | 1.66 ms |
| D1232 | 6.88 µs | 444 µs | 1.33 ms | 2.35 ms | 3.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,134.4 196.7,125.8 232.9,125.4 269.1,120.0 305.3,116.2 341.5,114.8 377.6,114.8 413.8,103.4 450.0,100.3 450.0,24.9 413.8,32.3 377.6,38.4 341.5,47.2 305.3,56.3 269.1,65.2 232.9,76.0 196.7,79.7 160.5,91.5 124.4,93.4 88.2,104.7 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,134.4 196.7,125.8 232.9,125.4 269.1,120.0 305.3,116.2 341.5,114.8 377.6,114.8 413.8,103.4 450.0,100.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,111.2 124.4,101.8 160.5,104.2 196.7,99.3 232.9,93.4 269.1,85.8 305.3,78.1 341.5,73.5 377.6,64.6 413.8,55.0 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.9 124.4,100.3 160.5,98.2 196.7,88.2 232.9,85.0 269.1,78.4 305.3,72.2 341.5,62.5 377.6,53.6 413.8,43.1 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,97.6 160.5,96.7 196.7,83.1 232.9,80.5 269.1,76.6 305.3,62.4 341.5,54.0 377.6,45.7 413.8,36.6 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.7 124.4,93.4 160.5,91.5 196.7,79.7 232.9,76.0 269.1,65.2 305.3,56.3 341.5,47.2 377.6,38.4 413.8,32.3 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.98 ns | 1.19 µs | 2.56 µs | 2.67 µs | 3.06 µs |
| D38 | 4.98 ns | 2.55 µs | 3.23 µs | 3.52 µs | 3.89 µs |
| D57 | 2.18 ns | 3.76 µs | 4.62 µs | 5.62 µs | 9.75 µs |
| D76 | 3.11 ns | 3.35 µs | 5.42 µs | 5.83 µs | 9 µs |
| D115 | 11.6 ns | 2.65 µs | 9.65 µs | 14.2 µs | 19 µs |
| D153 | 16.6 ns | 4.55 µs | 10.4 µs | 18.2 µs | 30 µs |
| D230 | 40.8 ns | 9.87 µs | 18.5 µs | 23.7 µs | 68.1 µs |
| D307 | 64.8 ns | 12.4 µs | 21.9 µs | 69.6 µs | 110 µs |
| D462 | 125 ns | 16.2 µs | 61.2 µs | 134 µs | 235 µs |
| D616 | 93.1 ns | 33 µs | 123 µs | 260 µs | 463 µs |
| D924 | 188 ns | 71.8 µs | 263 µs | 495 µs | 715 µs |
| D1232 | 366 ns | 122 µs | 457 µs | 954 µs | 1.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,190.1 124.4,200.3 160.5,195.9 196.7,179.6 232.9,175.2 269.1,164.0 305.3,158.2 341.5,150.1 377.6,153.7 413.8,145.0 450.0,136.8 450.0,35.3 413.8,42.7 377.6,48.1 341.5,56.6 305.3,65.9 269.1,71.9 232.9,82.1 196.7,87.8 160.5,97.0 124.4,96.0 88.2,107.4 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,190.1 124.4,200.3 160.5,195.9 196.7,179.6 232.9,175.2 269.1,164.0 305.3,158.2 341.5,150.1 377.6,153.7 413.8,145.0 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.2 88.2,112.7 124.4,107.8 160.5,109.3 196.7,112.2 232.9,105.5 269.1,95.9 305.3,93.0 341.5,89.7 377.6,80.9 413.8,71.3 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,109.8 124.4,105.3 160.5,103.3 196.7,96.2 232.9,95.2 269.1,88.1 305.3,86.0 341.5,73.2 377.6,64.6 413.8,55.2 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,108.7 124.4,102.9 160.5,102.4 196.7,91.4 232.9,88.3 269.1,85.0 305.3,71.6 341.5,63.5 377.6,55.3 413.8,47.3 450.0,39.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,107.4 124.4,96.0 160.5,97.0 196.7,87.8 232.9,82.1 269.1,71.9 305.3,65.9 341.5,56.6 377.6,48.1 413.8,42.7 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.73 µs | 3.35 µs | 3.6 µs | 4.12 µs |
| D38 | 4.05 ns | 3.34 µs | 4.3 µs | 4.46 µs | 4.94 µs |
| D57 | 2.49 ns | 5.95 µs | 7.08 µs | 7.94 µs | 11.1 µs |
| D76 | 3.75 ns | 5.22 µs | 7.59 µs | 8.64 µs | 12.1 µs |
| D115 | 10.3 ns | 7.75 µs | 12 µs | 23.7 µs | 28.5 µs |
| D153 | 16.2 ns | 6.48 µs | 16.6 µs | 23 µs | 35.8 µs |
| D230 | 40.4 ns | 14.1 µs | 23 µs | 28.6 µs | 80.1 µs |
| D307 | 65.1 ns | 16.2 µs | 42.5 µs | 80.6 µs | 119 µs |
| D462 | 122 ns | 25.1 µs | 81.5 µs | 158 µs | 253 µs |
| D616 | 94.7 ns | 40.4 µs | 136 µs | 272 µs | 449 µs |
| D924 | 203 ns | 83.8 µs | 273 µs | 480 µs | 641 µs |
| D1232 | 373 ns | 135 µs | 449 µs | 839 µs | 1.65 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,192.7 124.4,198.7 160.5,193.6 196.7,181.1 232.9,175.4 269.1,164.1 305.3,158.2 341.5,150.4 377.6,153.5 413.8,144.1 450.0,136.5 450.0,32.4 413.8,44.1 377.6,48.5 341.5,55.6 305.3,65.0 269.1,69.9 232.9,79.9 196.7,82.7 160.5,93.4 124.4,94.4 88.2,104.5 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,192.7 124.4,198.7 160.5,193.6 196.7,181.1 232.9,175.4 269.1,164.1 305.3,158.2 341.5,150.4 377.6,153.5 413.8,144.1 450.0,136.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,109.3 124.4,102.2 160.5,103.8 196.7,98.9 232.9,101.1 269.1,91.5 305.3,89.7 341.5,84.3 377.6,78.4 413.8,69.3 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,106.2 124.4,100.0 160.5,99.1 196.7,93.4 232.9,89.4 269.1,85.4 305.3,77.8 341.5,69.7 377.6,63.4 413.8,54.7 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,105.7 124.4,98.6 160.5,97.5 196.7,85.0 232.9,85.4 269.1,82.7 305.3,69.8 341.5,61.5 377.6,54.7 413.8,47.7 450.0,40.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,104.5 124.4,94.4 160.5,93.4 196.7,82.7 232.9,79.9 269.1,69.9 305.3,65.0 341.5,55.6 377.6,48.5 413.8,44.1 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.08 µs | 2.4 µs | 2.62 µs | 3.01 µs |
| D38 | 4.36 ns | 2.39 µs | 3.06 µs | 3.36 µs | 3.85 µs |
| D57 | 2.18 ns | 3.57 µs | 4.36 µs | 5.45 µs | 9.74 µs |
| D76 | 3.74 ns | 3.12 µs | 5.15 µs | 5.75 µs | 8.69 µs |
| D115 | 11.6 ns | 2.53 µs | 9.96 µs | 14.3 µs | 18.7 µs |
| D153 | 16.9 ns | 4.32 µs | 10.4 µs | 18.6 µs | 29.5 µs |
| D230 | 40.8 ns | 9.89 µs | 18.8 µs | 23.3 µs | 65 µs |
| D307 | 59.9 ns | 11.7 µs | 21.2 µs | 66.2 µs | 108 µs |
| D462 | 219 ns | 16 µs | 58.6 µs | 133 µs | 229 µs |
| D616 | 133 ns | 32.1 µs | 121 µs | 257 µs | 456 µs |
| D924 | 191 ns | 69 µs | 258 µs | 492 µs | 715 µs |
| D1232 | 362 ns | 120 µs | 457 µs | 951 µs | 1.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,191.7 124.4,200.3 160.5,193.6 196.7,179.5 232.9,174.9 269.1,164.0 305.3,159.2 341.5,143.1 377.6,149.3 413.8,144.8 450.0,136.9 450.0,35.3 413.8,42.7 377.6,48.3 341.5,56.8 305.3,66.2 269.1,72.5 232.9,82.3 196.7,87.9 160.5,97.5 124.4,96.0 88.2,107.6 52.0,110.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,191.7 124.4,200.3 160.5,193.6 196.7,179.5 232.9,174.9 269.1,164.0 305.3,159.2 341.5,143.1 377.6,149.3 413.8,144.8 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,113.5 124.4,108.5 160.5,110.2 196.7,112.8 232.9,106.1 269.1,95.9 305.3,93.8 341.5,89.9 377.6,81.3 413.8,71.7 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,110.4 124.4,106.0 160.5,104.0 196.7,95.8 232.9,95.3 269.1,87.9 305.3,86.4 341.5,73.8 377.6,64.8 413.8,55.4 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,109.3 124.4,103.2 160.5,102.6 196.7,91.3 232.9,88.0 269.1,85.2 305.3,72.3 341.5,63.7 377.6,55.4 413.8,47.4 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.6 88.2,107.6 124.4,96.0 160.5,97.5 196.7,87.9 232.9,82.3 269.1,72.5 305.3,66.2 341.5,56.8 377.6,48.3 413.8,42.7 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.74 µs | 3.36 µs | 3.6 µs | 4.13 µs |
| D38 | 4.05 ns | 3.35 µs | 4.3 µs | 4.47 µs | 4.97 µs |
| D57 | 10.6 ns | 5.95 µs | 7.06 µs | 7.94 µs | 11.1 µs |
| D76 | 10.5 ns | 5.22 µs | 7.63 µs | 8.61 µs | 12.1 µs |
| D115 | 10.6 ns | 7.7 µs | 12 µs | 23.6 µs | 28.5 µs |
| D153 | 16.5 ns | 6.51 µs | 16.4 µs | 23.1 µs | 35.8 µs |
| D230 | 39.9 ns | 14.2 µs | 23.1 µs | 28.9 µs | 80.1 µs |
| D307 | 60.5 ns | 16.3 µs | 43 µs | 80.9 µs | 119 µs |
| D462 | 201 ns | 25.3 µs | 81.9 µs | 158 µs | 250 µs |
| D616 | 114 ns | 40 µs | 136 µs | 272 µs | 448 µs |
| D924 | 201 ns | 83.5 µs | 274 µs | 479 µs | 641 µs |
| D1232 | 383 ns | 135 µs | 451 µs | 837 µs | 1.63 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.6 124.4,180.7 160.5,180.9 196.7,180.7 232.9,175.2 269.1,164.3 305.3,159.1 341.5,144.2 377.6,151.2 413.8,144.2 450.0,136.2 450.0,32.5 413.8,44.1 377.6,48.5 341.5,55.8 305.3,65.0 269.1,69.9 232.9,79.9 196.7,82.7 160.5,93.3 124.4,94.4 88.2,104.4 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.6 124.4,180.7 160.5,180.9 196.7,180.7 232.9,175.2 269.1,164.3 305.3,159.1 341.5,144.2 377.6,151.2 413.8,144.2 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,109.3 124.4,102.2 160.5,103.8 196.7,99.0 232.9,101.0 269.1,91.4 305.3,89.7 341.5,84.2 377.6,78.5 413.8,69.4 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,106.2 124.4,100.0 160.5,99.1 196.7,93.4 232.9,89.6 269.1,85.3 305.3,77.6 341.5,69.6 377.6,63.3 413.8,54.7 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,105.7 124.4,98.6 160.5,97.6 196.7,85.0 232.9,85.3 269.1,82.5 305.3,69.8 341.5,61.5 377.6,54.7 413.8,47.7 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,104.4 124.4,94.4 160.5,93.3 196.7,82.7 232.9,79.9 269.1,69.9 305.3,65.0 341.5,55.8 377.6,48.5 413.8,44.1 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.14 µs | 4.02 µs | 4.28 µs | 4.94 µs |
| D38 | 4.37 ns | 4.02 µs | 5.11 µs | 5.59 µs | 6.27 µs |
| D57 | 3.12 ns | 4.74 µs | 5.91 µs | 7.23 µs | 9.98 µs |
| D76 | 3.56 ns | 4.09 µs | 7 µs | 7.39 µs | 10.9 µs |
| D115 | 10.9 ns | 3.36 µs | 12.4 µs | 18.3 µs | 22.7 µs |
| D153 | 16.4 ns | 5.94 µs | 12.7 µs | 22 µs | 34.3 µs |
| D230 | 41.2 ns | 12.3 µs | 22.4 µs | 26.8 µs | 74.3 µs |
| D307 | 58.2 ns | 14.3 µs | 25.5 µs | 75.2 µs | 120 µs |
| D462 | 114 ns | 19.8 µs | 67.2 µs | 146 µs | 249 µs |
| D616 | 96.9 ns | 37.3 µs | 134 µs | 282 µs | 490 µs |
| D924 | 158 ns | 78.6 µs | 285 µs | 524 µs | 760 µs |
| D1232 | 369 ns | 133 µs | 493 µs | 1.01 ms | 1.39 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,195.9 160.5,194.2 196.7,180.4 232.9,175.3 269.1,163.9 305.3,159.6 341.5,151.2 377.6,153.2 413.8,147.2 450.0,136.6 450.0,34.4 413.8,42.0 377.6,47.4 341.5,55.8 305.3,64.9 269.1,70.8 232.9,80.4 196.7,85.6 160.5,94.6 124.4,95.7 88.2,101.5 52.0,104.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,195.9 160.5,194.2 196.7,180.4 232.9,175.3 269.1,163.9 305.3,159.6 341.5,151.2 377.6,153.2 413.8,147.2 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,107.0 124.4,105.0 160.5,106.8 196.7,109.2 232.9,102.2 269.1,93.1 305.3,91.3 341.5,87.3 377.6,79.4 413.8,70.1 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.0 124.4,102.2 160.5,100.1 196.7,93.1 232.9,92.8 269.1,85.7 305.3,84.1 341.5,72.1 377.6,63.5 413.8,54.2 450.0,47.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,102.9 124.4,99.7 160.5,99.5 196.7,88.2 232.9,85.9 269.1,83.5 305.3,70.7 341.5,62.4 377.6,54.3 413.8,46.6 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,101.5 124.4,95.7 160.5,94.6 196.7,85.6 232.9,80.4 269.1,70.8 305.3,64.9 341.5,55.8 377.6,47.4 413.8,42.0 450.0,34.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.76 µs | 3.53 µs | 3.76 µs | 4.33 µs |
| D38 | 3.73 ns | 3.53 µs | 4.36 µs | 4.5 µs | 4.98 µs |
| D57 | 2.59 µs | 6.15 µs | 7.43 µs | 8.38 µs | 11.6 µs |
| D76 | 2.61 µs | 5.39 µs | 7.99 µs | 8.92 µs | 12.5 µs |
| D115 | 5.32 µs | 8.29 µs | 12.6 µs | 25.1 µs | 29.6 µs |
| D153 | 2.74 µs | 6.81 µs | 17.1 µs | 23.9 µs | 36.8 µs |
| D230 | 3.05 µs | 14.6 µs | 23.9 µs | 28.8 µs | 81.2 µs |
| D307 | 2.9 µs | 17 µs | 44.2 µs | 82 µs | 120 µs |
| D462 | 3.41 µs | 25.8 µs | 83.2 µs | 159 µs | 253 µs |
| D616 | 1.98 µs | 41.5 µs | 138 µs | 275 µs | 453 µs |
| D924 | 3.9 µs | 84.9 µs | 277 µs | 484 µs | 642 µs |
| D1232 | 4.17 µs | 137 µs | 455 µs | 843 µs | 1.63 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.7 124.4,112.5 160.5,112.4 196.7,103.5 232.9,111.8 269.1,110.4 305.3,111.1 341.5,109.1 377.6,115.8 413.8,107.4 450.0,106.6 450.0,32.5 413.8,44.1 377.6,48.4 341.5,55.6 305.3,64.9 269.1,69.7 232.9,79.5 196.7,82.2 160.5,92.9 124.4,93.9 88.2,104.4 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.7 124.4,112.5 160.5,112.4 196.7,103.5 232.9,111.8 269.1,110.4 305.3,111.1 341.5,109.1 377.6,115.8 413.8,107.4 450.0,106.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,108.6 124.4,101.8 160.5,103.4 196.7,98.0 232.9,100.5 269.1,91.0 305.3,89.1 341.5,84.0 377.6,78.1 413.8,69.2 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,106.0 124.4,99.4 160.5,98.5 196.7,92.9 232.9,89.0 269.1,84.9 305.3,77.3 341.5,69.4 377.6,63.2 413.8,54.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,105.6 124.4,97.9 160.5,97.1 196.7,84.3 232.9,84.9 269.1,82.6 305.3,69.6 341.5,61.4 377.6,54.6 413.8,47.6 450.0,40.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,104.4 124.4,93.9 160.5,92.9 196.7,82.2 232.9,79.5 269.1,69.7 305.3,64.9 341.5,55.6 377.6,48.4 413.8,44.1 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 133 ns | 157 ns | 160 ns | 178 ns |
| D38 | 4.36 ns | 162 ns | 177 ns | 189 ns | 192 ns |
| D57 | 179 ns | 339 ns | 326 ns | 327 ns | 446 ns |
| D76 | 187 ns | 243 ns | 333 ns | 337 ns | 441 ns |
| D115 | 407 ns | 328 ns | 697 ns | 920 ns | 974 ns |
| D153 | 385 ns | 548 ns | 834 ns | 862 ns | 996 ns |
| D230 | 583 ns | 824 ns | 994 ns | 849 ns | 1.81 µs |
| D307 | 831 ns | 1.24 µs | 1.33 µs | 2.17 µs | 2.67 µs |
| D462 | 1.03 µs | 1.62 µs | 2.2 µs | 3.07 µs | 4.23 µs |
| D616 | 714 ns | 2.07 µs | 2.94 µs | 4.19 µs | 6.07 µs |
| D924 | 1.68 µs | 2.85 µs | 4.61 µs | 6.09 µs | 7.08 µs |
| D1232 | 2.39 µs | 4.31 µs | 7.59 µs | 9.89 µs | 17.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,119.9 160.5,119.1 196.7,105.6 232.9,106.6 269.1,99.4 305.3,93.2 341.5,89.6 377.6,95.9 413.8,81.0 450.0,74.9 450.0,40.1 413.8,56.0 377.6,58.7 341.5,65.0 305.3,72.9 269.1,79.7 232.9,90.1 196.7,90.5 160.5,104.2 124.4,104.0 88.2,118.7 52.0,120.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,119.9 160.5,119.1 196.7,105.6 232.9,106.6 269.1,99.4 305.3,93.2 341.5,89.6 377.6,95.9 413.8,81.0 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.1 88.2,121.7 124.4,108.8 160.5,114.6 196.7,109.4 232.9,100.5 269.1,93.4 305.3,86.3 341.5,81.7 377.6,77.4 413.8,71.8 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,120.1 124.4,109.5 160.5,109.1 196.7,96.3 232.9,93.1 269.1,90.1 305.3,85.0 341.5,76.3 377.6,71.3 413.8,63.5 450.0,54.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,119.0 124.4,109.4 160.5,108.9 196.7,91.4 232.9,92.6 269.1,92.8 305.3,76.5 341.5,70.5 377.6,65.1 413.8,58.6 450.0,50.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.0 88.2,118.7 124.4,104.0 160.5,104.2 196.7,90.5 232.9,90.1 269.1,79.7 305.3,72.9 341.5,65.0 377.6,58.7 413.8,56.0 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 153 ns | 196 ns | 200 ns | 216 ns |
| D38 | 4.36 ns | 195 ns | 216 ns | 199 ns | 202 ns |
| D57 | 264 ns | 440 ns | 413 ns | 427 ns | 573 ns |
| D76 | 276 ns | 339 ns | 451 ns | 435 ns | 566 ns |
| D115 | 661 ns | 437 ns | 936 ns | 1.14 µs | 1.16 µs |
| D153 | 633 ns | 690 ns | 1.01 µs | 1.06 µs | 1.25 µs |
| D230 | 965 ns | 1.15 µs | 1.27 µs | 1.06 µs | 2.14 µs |
| D307 | 1.36 µs | 1.74 µs | 1.76 µs | 2.67 µs | 3.23 µs |
| D462 | 1.55 µs | 2.02 µs | 2.7 µs | 3.64 µs | 4.87 µs |
| D616 | 1.07 µs | 2.57 µs | 3.6 µs | 4.89 µs | 6.91 µs |
| D924 | 2.54 µs | 3.63 µs | 5.44 µs | 6.82 µs | 7.7 µs |
| D1232 | 3.38 µs | 5.18 µs | 8.63 µs | 11 µs | 18.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,113.1 160.5,112.3 196.7,97.2 232.9,97.9 269.1,90.6 305.3,84.7 341.5,82.4 377.6,88.9 413.8,73.8 450.0,68.8 450.0,39.6 413.8,54.5 377.6,56.4 341.5,62.5 305.3,69.6 269.1,76.8 232.9,86.1 196.7,87.4 160.5,99.9 124.4,99.7 88.2,117.8 52.0,116.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,113.1 160.5,112.3 196.7,97.2 232.9,97.9 269.1,90.6 305.3,84.7 341.5,82.4 377.6,88.9 413.8,73.8 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.6 88.2,118.4 124.4,104.3 160.5,108.8 196.7,104.4 232.9,96.4 269.1,87.5 305.3,80.4 341.5,77.8 377.6,73.6 413.8,67.6 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,116.6 124.4,105.4 160.5,103.8 196.7,91.2 232.9,89.8 269.1,85.9 305.3,80.2 341.5,72.8 377.6,67.8 413.8,60.6 450.0,52.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.9 88.2,118.1 124.4,104.8 160.5,104.5 196.7,87.8 232.9,89.0 269.1,89.0 305.3,72.9 341.5,67.6 377.6,62.4 413.8,56.7 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.6 88.2,117.8 124.4,99.7 160.5,99.9 196.7,87.4 232.9,86.1 269.1,76.8 305.3,69.6 341.5,62.5 377.6,56.4 413.8,54.5 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
