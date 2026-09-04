# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.16 ns | 1.99 µs | 3.21 µs | 3.48 µs | 4.08 µs |
| D38 | 1.39 µs | 4.69 µs | 4.15 µs | 8.66 µs | 8.11 µs |
| D57 | 1.49 µs | 5.76 µs | 6.71 µs | 6.12 µs | 11.3 µs |
| D76 | 1.6 µs | 7.15 µs | 5.85 µs | 14.5 µs | 18.6 µs |
| D115 | 1.27 µs | 6.61 µs | 20 µs | 32.7 µs | 45.5 µs |
| D153 | 1.22 µs | 10.2 µs | 25.5 µs | 45.5 µs | 66 µs |
| D230 | 1.63 µs | 14.5 µs | 37.5 µs | 64.3 µs | 96.6 µs |
| D307 | 1.58 µs | 15.6 µs | 66.8 µs | 124 µs | 192 µs |
| D462 | 1.53 µs | 34 µs | 124 µs | 202 µs | 399 µs |
| D616 | 1.65 µs | 54.1 µs | 207 µs | 445 µs | 703 µs |
| D924 | 1.13 µs | 108 µs | 318 µs | 948 µs | 1.54 ms |
| D1232 | 2.05 µs | 172 µs | 662 µs | 1.29 ms | 3.24 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.4 88.2,120.2 124.4,119.4 160.5,118.5 196.7,121.4 232.9,121.8 269.1,118.2 305.3,118.6 341.5,119.0 377.6,118.0 413.8,122.8 450.0,115.4 450.0,24.0 413.8,33.2 377.6,42.9 341.5,50.0 305.3,59.0 269.1,67.6 232.9,72.3 196.7,76.9 160.5,88.0 124.4,94.2 88.2,98.3 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.4 88.2,120.2 124.4,119.4 160.5,118.5 196.7,121.4 232.9,121.8 269.1,118.2 305.3,118.6 341.5,119.0 377.6,118.0 413.8,122.8 450.0,115.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,105.1 124.4,102.6 160.5,99.9 196.7,100.9 232.9,95.5 269.1,91.1 305.3,90.2 341.5,80.5 377.6,74.8 413.8,66.2 450.0,60.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.6 124.4,100.7 160.5,102.4 196.7,87.1 232.9,84.1 269.1,79.3 305.3,72.2 341.5,64.4 377.6,58.1 413.8,52.8 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,97.5 124.4,101.8 160.5,91.1 196.7,81.0 232.9,76.9 269.1,72.6 305.3,64.5 341.5,58.4 377.6,48.6 413.8,39.2 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,98.3 124.4,94.2 160.5,88.0 196.7,76.9 232.9,72.3 269.1,67.6 305.3,59.0 341.5,50.0 377.6,42.9 413.8,33.2 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 µs | 3.58 µs | 5.85 µs | 6.37 µs | 7.11 µs |
| D38 | 3.38 µs | 5.82 µs | 7.1 µs | 8.76 µs | 8.57 µs |
| D57 | 3.46 µs | 4.25 µs | 4.12 µs | 3.58 µs | 6.22 µs |
| D76 | 3.79 µs | 4.99 µs | 3.39 µs | 7.84 µs | 9.33 µs |
| D115 | 5.66 µs | 6.66 µs | 13.6 µs | 16.7 µs | 24 µs |
| D153 | 5.13 µs | 9.64 µs | 16.1 µs | 24 µs | 34.5 µs |
| D230 | 9.05 µs | 11.8 µs | 23.1 µs | 37.8 µs | 54.9 µs |
| D307 | 13 µs | 16.2 µs | 51.5 µs | 85.2 µs | 138 µs |
| D462 | 13 µs | 26.6 µs | 85.9 µs | 136 µs | 276 µs |
| D616 | 22.9 µs | 62.5 µs | 168 µs | 335 µs | 554 µs |
| D924 | 19 µs | 129 µs | 274 µs | 800 µs | 1.34 ms |
| D1232 | 45.5 µs | 216 µs | 680 µs | 1.21 ms | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.2 88.2,183.6 124.4,183.0 160.5,181.1 196.7,172.4 232.9,174.5 269.1,162.2 305.3,154.3 341.5,154.3 377.6,142.1 413.8,146.0 450.0,127.1 450.0,37.1 413.8,53.6 377.6,72.8 341.5,87.9 305.3,103.1 269.1,123.0 232.9,133.1 196.7,141.0 160.5,161.5 124.4,170.3 88.2,163.4 52.0,167.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.2 88.2,183.6 124.4,183.0 160.5,181.1 196.7,172.4 232.9,174.5 269.1,162.2 305.3,154.3 341.5,154.3 377.6,142.1 413.8,146.0 450.0,127.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.3 88.2,171.8 124.4,178.6 160.5,175.1 196.7,168.8 232.9,160.8 269.1,156.4 305.3,149.5 341.5,138.8 377.6,120.2 413.8,104.5 450.0,93.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.7 88.2,167.4 124.4,179.2 160.5,183.5 196.7,153.3 232.9,149.7 269.1,141.9 305.3,124.4 341.5,113.3 377.6,98.7 413.8,88.1 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.8 88.2,162.9 124.4,182.3 160.5,165.3 196.7,148.8 232.9,141.0 269.1,131.1 305.3,113.5 341.5,103.4 377.6,83.7 413.8,64.9 450.0,55.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.4 88.2,163.4 124.4,170.3 160.5,161.5 196.7,141.0 232.9,133.1 269.1,123.0 305.3,103.1 341.5,87.9 377.6,72.8 413.8,53.6 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 1.99 µs | 3.2 µs | 3.46 µs | 4.05 µs |
| D38 | 1.31 µs | 4.69 µs | 4.11 µs | 8.62 µs | 8.08 µs |
| D57 | 1.36 µs | 5.69 µs | 6.65 µs | 6.1 µs | 11.3 µs |
| D76 | 1.52 µs | 7.16 µs | 5.78 µs | 14.4 µs | 18.5 µs |
| D115 | 1.16 µs | 5.84 µs | 20.1 µs | 32.4 µs | 45 µs |
| D153 | 1.13 µs | 10.1 µs | 25.4 µs | 44.7 µs | 66.3 µs |
| D230 | 1.53 µs | 14.4 µs | 38.2 µs | 64.7 µs | 100 µs |
| D307 | 1.47 µs | 15.5 µs | 66.7 µs | 124 µs | 191 µs |
| D462 | 1.42 µs | 33.9 µs | 124 µs | 203 µs | 401 µs |
| D616 | 1.57 µs | 54.1 µs | 207 µs | 449 µs | 705 µs |
| D924 | 1.04 µs | 108 µs | 318 µs | 940 µs | 1.54 ms |
| D1232 | 1.96 µs | 172 µs | 663 µs | 1.3 ms | 3.24 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,120.9 124.4,120.5 160.5,119.1 196.7,122.4 232.9,122.7 269.1,119.0 305.3,119.5 341.5,119.9 377.6,118.7 413.8,123.8 450.0,116.0 450.0,24.0 413.8,33.2 377.6,42.9 341.5,49.9 305.3,59.1 269.1,67.1 232.9,72.2 196.7,77.1 160.5,88.1 124.4,94.2 88.2,98.4 52.0,106.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,120.9 124.4,120.5 160.5,119.1 196.7,122.4 232.9,122.7 269.1,119.0 305.3,119.5 341.5,119.9 377.6,118.7 413.8,123.8 450.0,116.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,105.1 124.4,102.7 160.5,99.9 196.7,102.4 232.9,95.6 269.1,91.2 305.3,90.3 341.5,80.6 377.6,74.8 413.8,66.2 450.0,60.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,106.7 124.4,100.8 160.5,102.5 196.7,87.0 232.9,84.1 269.1,79.1 305.3,72.2 341.5,64.4 377.6,58.1 413.8,52.8 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,97.6 124.4,101.9 160.5,91.2 196.7,81.1 232.9,77.1 269.1,72.5 305.3,64.5 341.5,58.4 377.6,48.5 413.8,39.3 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,98.4 124.4,94.2 160.5,88.1 196.7,77.1 232.9,72.2 269.1,67.1 305.3,59.1 341.5,49.9 377.6,42.9 413.8,33.2 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.47 ns | 1.71 µs | 2.52 µs | 2.8 µs | 3.11 µs |
| D38 | 4.57 ns | 2.53 µs | 3.1 µs | 3.92 µs | 3.81 µs |
| D57 | 2.5 ns | 5.31 µs | 6.07 µs | 5.3 µs | 9.51 µs |
| D76 | 3.29 ns | 6.72 µs | 5.15 µs | 11.6 µs | 14 µs |
| D115 | 6.33 ns | 10.1 µs | 20.4 µs | 24.4 µs | 35.7 µs |
| D153 | 9.14 ns | 14.4 µs | 23.8 µs | 35.4 µs | 48.9 µs |
| D230 | 19.5 ns | 18.3 µs | 37 µs | 55 µs | 79.6 µs |
| D307 | 28.4 ns | 23.9 µs | 72.3 µs | 115 µs | 180 µs |
| D462 | 42.3 ns | 40.4 µs | 119 µs | 163 µs | 337 µs |
| D616 | 47.6 ns | 89.2 µs | 226 µs | 430 µs | 662 µs |
| D924 | 15 ns | 192 µs | 362 µs | 930 µs | 1.47 ms |
| D1232 | 73.5 ns | 303 µs | 862 µs | 1.38 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.5 88.2,191.1 124.4,198.6 160.5,195.2 196.7,187.1 232.9,182.5 269.1,173.2 305.3,168.5 341.5,163.5 377.6,162.1 413.8,176.4 450.0,156.7 450.0,25.6 413.8,33.8 377.6,43.7 341.5,52.1 305.3,59.9 269.1,70.0 232.9,76.0 196.7,79.9 160.5,91.6 124.4,96.3 88.2,107.7 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.5 88.2,191.1 124.4,198.6 160.5,195.2 196.7,187.1 232.9,182.5 269.1,173.2 305.3,168.5 341.5,163.5 377.6,162.1 413.8,176.4 450.0,156.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,112.8 124.4,103.6 160.5,100.6 196.7,95.6 232.9,91.2 269.1,88.2 305.3,84.9 341.5,78.4 377.6,68.6 413.8,59.0 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,110.3 124.4,101.9 160.5,103.9 196.7,86.9 232.9,84.9 269.1,79.5 305.3,71.2 341.5,65.0 377.6,57.0 413.8,51.2 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,107.3 124.4,103.6 160.5,93.9 196.7,84.7 232.9,80.0 269.1,74.6 305.3,65.4 341.5,61.1 377.6,49.0 413.8,39.5 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,107.7 124.4,96.3 160.5,91.6 196.7,79.9 232.9,76.0 269.1,70.0 305.3,59.9 341.5,52.1 377.6,43.7 413.8,33.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.57 µs | 2.69 µs | 2.93 µs | 3.48 µs |
| D38 | 3.16 ns | 2.87 µs | 3.26 µs | 3.74 µs | 3.31 µs |
| D57 | 1.71 ns | 2.88 µs | 2.78 µs | 2.34 µs | 4.16 µs |
| D76 | 642 ns | 6 µs | 4.52 µs | 11.9 µs | 15.5 µs |
| D115 | 470 ns | 4.65 µs | 17 µs | 29.7 µs | 40.6 µs |
| D153 | 454 ns | 7.93 µs | 16.8 µs | 40.8 µs | 60.8 µs |
| D230 | 663 ns | 11.9 µs | 34 µs | 58.5 µs | 92.9 µs |
| D307 | 647 ns | 13.3 µs | 53.8 µs | 115 µs | 181 µs |
| D462 | 605 ns | 27.3 µs | 104 µs | 177 µs | 340 µs |
| D616 | 707 ns | 51 µs | 192 µs | 419 µs | 666 µs |
| D924 | 499 ns | 99.4 µs | 299 µs | 887 µs | 1.46 ms |
| D1232 | 1.01 µs | 161 µs | 625 µs | 1.26 ms | 3.11 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,195.7 124.4,203.4 160.5,129.8 196.7,133.6 232.9,134.1 269.1,129.4 305.3,129.7 341.5,130.5 377.6,128.6 413.8,132.9 450.0,124.2 450.0,24.5 413.8,33.9 377.6,43.6 341.5,51.9 305.3,59.8 269.1,68.1 232.9,73.3 196.7,78.3 160.5,90.3 124.4,106.6 88.2,109.4 52.0,108.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,195.7 124.4,203.4 160.5,129.8 196.7,133.6 232.9,134.1 269.1,129.4 305.3,129.7 341.5,130.5 377.6,128.6 413.8,132.9 450.0,124.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,111.2 124.4,111.2 160.5,102.1 196.7,105.2 232.9,98.6 269.1,93.5 305.3,92.2 341.5,83.2 377.6,75.5 413.8,67.2 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.6 124.4,111.6 160.5,105.6 196.7,89.1 232.9,89.3 269.1,80.5 305.3,74.8 341.5,66.6 377.6,59.1 413.8,53.6 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,107.9 124.4,113.7 160.5,93.6 196.7,82.2 232.9,78.3 269.1,73.8 305.3,65.4 341.5,60.1 377.6,49.4 413.8,40.1 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,109.4 124.4,106.6 160.5,90.3 196.7,78.3 232.9,73.3 269.1,68.1 305.3,59.8 341.5,51.9 377.6,43.6 413.8,33.9 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.45 ns | 1.74 µs | 2.87 µs | 3.16 µs | 3.58 µs |
| D38 | 4.57 ns | 2.86 µs | 3.58 µs | 4.36 µs | 4.31 µs |
| D57 | 471 ns | 5.59 µs | 5.74 µs | 5.09 µs | 9.39 µs |
| D76 | 524 ns | 6.62 µs | 4.78 µs | 11.2 µs | 14.1 µs |
| D115 | 892 ns | 9.89 µs | 19.5 µs | 25.6 µs | 37.4 µs |
| D153 | 764 ns | 13.4 µs | 24.2 µs | 37.4 µs | 56.5 µs |
| D230 | 1.39 µs | 17 µs | 36.5 µs | 63.8 µs | 98.2 µs |
| D307 | 2.1 µs | 24.7 µs | 83.3 µs | 145 µs | 246 µs |
| D462 | 2.03 µs | 41.5 µs | 147 µs | 240 µs | 502 µs |
| D616 | 3.72 µs | 99.1 µs | 294 µs | 590 µs | 1.02 ms |
| D924 | 2.84 µs | 213 µs | 490 µs | 1.46 ms | 2.5 ms |
| D1232 | 7.27 µs | 371 µs | 1.23 ms | 2.25 ms | 5.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.6 88.2,191.1 124.4,133.6 160.5,132.3 196.7,125.7 232.9,127.6 269.1,120.2 305.3,115.1 341.5,115.5 377.6,108.0 413.8,111.3 450.0,99.7 450.0,17.6 413.8,27.2 377.6,38.4 341.5,47.1 305.3,56.0 269.1,67.4 232.9,74.2 196.7,79.4 160.5,91.5 124.4,96.5 88.2,106.1 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.6 88.2,191.1 124.4,133.6 160.5,132.3 196.7,125.7 232.9,127.6 269.1,120.2 305.3,115.1 341.5,115.5 377.6,108.0 413.8,111.3 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,111.3 124.4,102.9 160.5,100.8 196.7,95.9 232.9,92.1 269.1,89.1 305.3,84.5 341.5,78.1 377.6,67.3 413.8,57.8 450.0,50.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,108.4 124.4,102.6 160.5,104.9 196.7,87.4 232.9,84.7 269.1,79.7 305.3,69.4 341.5,62.4 377.6,53.8 413.8,47.4 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.0 124.4,104.1 160.5,94.3 196.7,84.0 232.9,79.3 269.1,72.7 305.3,62.5 341.5,56.3 377.6,45.1 413.8,33.9 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,106.1 124.4,96.5 160.5,91.5 196.7,79.4 232.9,74.2 269.1,67.4 305.3,56.0 341.5,47.1 377.6,38.4 413.8,27.2 450.0,17.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.69 ns | 1.18 µs | 2.54 µs | 2.65 µs | 3.05 µs |
| D38 | 5.62 ns | 2.5 µs | 3.03 µs | 3.75 µs | 2.97 µs |
| D57 | 2.49 ns | 3.45 µs | 3.68 µs | 3.14 µs | 7.14 µs |
| D76 | 3.5 ns | 4.06 µs | 3.06 µs | 7.52 µs | 9.08 µs |
| D115 | 10.1 ns | 3.29 µs | 9.99 µs | 13.8 µs | 19.4 µs |
| D153 | 16.5 ns | 5.51 µs | 9.97 µs | 19.2 µs | 32.6 µs |
| D230 | 40.7 ns | 7.21 µs | 16.6 µs | 32.8 µs | 52.8 µs |
| D307 | 73.2 ns | 7.6 µs | 27.6 µs | 69.8 µs | 116 µs |
| D462 | 105 ns | 11.5 µs | 61.5 µs | 111 µs | 231 µs |
| D616 | 148 ns | 26.9 µs | 124 µs | 268 µs | 456 µs |
| D924 | 95.5 ns | 59.7 µs | 195 µs | 621 µs | 1.05 ms |
| D1232 | 356 ns | 104 µs | 430 µs | 897 µs | 2.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.8 88.2,188.6 124.4,198.7 160.5,194.4 196.7,181.3 232.9,175.2 269.1,164.0 305.3,156.7 341.5,152.2 377.6,148.0 413.8,153.4 450.0,137.1 450.0,28.2 413.8,37.9 377.6,48.3 341.5,56.8 305.3,65.3 269.1,75.1 232.9,81.0 196.7,87.5 160.5,96.9 124.4,99.9 88.2,110.8 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.8 88.2,188.6 124.4,198.7 160.5,194.4 196.7,181.3 232.9,175.2 269.1,164.0 305.3,156.7 341.5,152.2 377.6,148.0 413.8,153.4 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.2 88.2,112.9 124.4,108.9 160.5,106.9 196.7,109.5 232.9,103.1 269.1,99.8 305.3,99.1 341.5,93.9 377.6,83.5 413.8,73.5 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.6 124.4,108.1 160.5,110.4 196.7,95.7 232.9,95.8 269.1,89.4 305.3,83.1 341.5,73.2 377.6,64.5 413.8,58.9 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,107.9 124.4,110.1 160.5,99.3 196.7,91.7 232.9,87.6 269.1,81.0 305.3,71.6 341.5,65.8 377.6,54.9 413.8,44.5 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,110.8 124.4,99.9 160.5,96.9 196.7,87.5 232.9,81.0 269.1,75.1 305.3,65.3 341.5,56.8 377.6,48.3 413.8,37.9 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.77 ns | 1.71 µs | 3.35 µs | 3.58 µs | 4.11 µs |
| D38 | 4.57 ns | 3.31 µs | 3.93 µs | 4.82 µs | 4.12 µs |
| D57 | 3.11 ns | 5.43 µs | 5.9 µs | 4.76 µs | 8.43 µs |
| D76 | 3.17 ns | 6.28 µs | 4.54 µs | 10.3 µs | 12 µs |
| D115 | 8.46 ns | 9.28 µs | 12.9 µs | 21.6 µs | 28.6 µs |
| D153 | 15.7 ns | 7.88 µs | 16.3 µs | 24.3 µs | 39.3 µs |
| D230 | 40.8 ns | 10.4 µs | 20.2 µs | 38.6 µs | 65.9 µs |
| D307 | 75.8 ns | 10.5 µs | 52.9 µs | 80.6 µs | 126 µs |
| D462 | 107 ns | 18.1 µs | 82.1 µs | 133 µs | 250 µs |
| D616 | 140 ns | 32 µs | 135 µs | 290 µs | 449 µs |
| D924 | 111 ns | 72.7 µs | 207 µs | 612 µs | 923 µs |
| D1232 | 369 ns | 114 µs | 418 µs | 778 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.4 88.2,191.1 124.4,195.9 160.5,195.7 196.7,183.5 232.9,175.8 269.1,164.0 305.3,156.3 341.5,152.0 377.6,148.6 413.8,151.5 450.0,136.7 450.0,26.3 413.8,39.6 377.6,48.5 341.5,55.8 305.3,64.2 269.1,72.3 232.9,78.7 196.7,82.7 160.5,93.4 124.4,97.8 88.2,106.7 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.4 88.2,191.1 124.4,195.9 160.5,195.7 196.7,183.5 232.9,175.8 269.1,164.0 305.3,156.3 341.5,152.0 377.6,148.6 413.8,151.5 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,109.4 124.4,103.3 160.5,101.5 196.7,96.6 232.9,98.7 269.1,95.2 305.3,95.2 341.5,88.3 377.6,81.3 413.8,71.1 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,107.3 124.4,102.3 160.5,105.5 196.7,92.6 232.9,89.6 269.1,87.0 305.3,75.0 341.5,69.6 377.6,63.4 413.8,58.1 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,104.8 124.4,104.9 160.5,95.3 196.7,86.2 232.9,84.7 269.1,79.0 305.3,69.8 341.5,63.6 377.6,53.9 413.8,44.7 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,106.7 124.4,97.8 160.5,93.4 196.7,82.7 232.9,78.7 269.1,72.3 305.3,64.2 341.5,55.8 377.6,48.5 413.8,39.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 ns | 1.08 µs | 2.38 µs | 2.61 µs | 2.99 µs |
| D38 | 4.92 ns | 2.35 µs | 2.86 µs | 3.58 µs | 2.99 µs |
| D57 | 2.18 ns | 3.24 µs | 3.55 µs | 3.1 µs | 7.13 µs |
| D76 | 3.51 ns | 3.86 µs | 2.89 µs | 7.39 µs | 8.74 µs |
| D115 | 10.1 ns | 3.13 µs | 10 µs | 13.5 µs | 20.2 µs |
| D153 | 16.5 ns | 5.08 µs | 10.5 µs | 19.5 µs | 31.5 µs |
| D230 | 40.7 ns | 7.1 µs | 17 µs | 31.4 µs | 51.1 µs |
| D307 | 66.7 ns | 7.24 µs | 26.9 µs | 66.9 µs | 113 µs |
| D462 | 104 ns | 11.3 µs | 58.4 µs | 110 µs | 230 µs |
| D616 | 141 ns | 26.3 µs | 122 µs | 268 µs | 453 µs |
| D924 | 107 ns | 57.7 µs | 191 µs | 615 µs | 1.05 ms |
| D1232 | 357 ns | 99.9 µs | 431 µs | 897 µs | 2.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.3 88.2,190.2 124.4,200.3 160.5,194.4 196.7,181.3 232.9,175.2 269.1,164.0 305.3,157.9 341.5,152.4 377.6,148.6 413.8,152.1 450.0,137.1 450.0,28.4 413.8,37.9 377.6,48.4 341.5,56.8 305.3,65.6 269.1,75.5 232.9,81.5 196.7,87.0 160.5,97.4 124.4,99.9 88.2,110.7 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.3 88.2,190.2 124.4,200.3 160.5,194.4 196.7,181.3 232.9,175.2 269.1,164.0 305.3,157.9 341.5,152.4 377.6,148.6 413.8,152.1 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,113.7 124.4,109.7 160.5,107.5 196.7,110.1 232.9,104.1 269.1,100.0 305.3,99.7 341.5,94.2 377.6,83.7 413.8,74.0 450.0,67.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.5 88.2,111.2 124.4,108.6 160.5,111.1 196.7,95.7 232.9,95.1 269.1,89.2 305.3,83.4 341.5,73.8 377.6,64.6 413.8,59.1 450.0,49.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,108.5 124.4,110.3 160.5,99.5 196.7,92.0 232.9,87.4 269.1,81.5 305.3,72.1 341.5,66.0 377.6,54.9 413.8,44.6 450.0,39.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,110.7 124.4,99.9 160.5,97.4 196.7,87.0 232.9,81.5 269.1,75.5 305.3,65.6 341.5,56.8 377.6,48.4 413.8,37.9 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.13 ns | 1.73 µs | 3.35 µs | 3.6 µs | 4.12 µs |
| D38 | 4.57 ns | 3.32 µs | 3.94 µs | 4.85 µs | 4.19 µs |
| D57 | 10.4 ns | 5.46 µs | 5.91 µs | 4.77 µs | 8.43 µs |
| D76 | 13.1 ns | 6.29 µs | 4.55 µs | 10.3 µs | 12.1 µs |
| D115 | 8.73 ns | 9.34 µs | 12.8 µs | 21.7 µs | 29 µs |
| D153 | 15.3 ns | 7.94 µs | 16.3 µs | 24.4 µs | 39.1 µs |
| D230 | 39.9 ns | 10.5 µs | 20.3 µs | 39.5 µs | 64.9 µs |
| D307 | 65.2 ns | 10.7 µs | 53.9 µs | 80.9 µs | 125 µs |
| D462 | 100 ns | 18.1 µs | 81.5 µs | 133 µs | 250 µs |
| D616 | 139 ns | 32 µs | 135 µs | 290 µs | 447 µs |
| D924 | 107 ns | 70 µs | 208 µs | 608 µs | 922 µs |
| D1232 | 381 ns | 114 µs | 419 µs | 778 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,191.1 124.4,181.0 160.5,178.1 196.7,183.1 232.9,176.1 269.1,164.3 305.3,158.2 341.5,152.8 377.6,148.8 413.8,152.0 450.0,136.3 450.0,26.3 413.8,39.6 377.6,48.5 341.5,55.8 305.3,64.4 269.1,72.5 232.9,78.8 196.7,82.5 160.5,93.4 124.4,97.8 88.2,106.5 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,191.1 124.4,181.0 160.5,178.1 196.7,183.1 232.9,176.1 269.1,164.3 305.3,158.2 341.5,152.8 377.6,148.8 413.8,152.0 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,109.4 124.4,103.2 160.5,101.5 196.7,96.6 232.9,98.6 269.1,95.2 305.3,94.9 341.5,88.4 377.6,81.3 413.8,71.6 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,107.3 124.4,102.2 160.5,105.5 196.7,92.7 232.9,89.7 269.1,86.9 305.3,74.8 341.5,69.7 377.6,63.4 413.8,58.1 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.7 124.4,104.9 160.5,95.4 196.7,86.1 232.9,84.7 269.1,78.7 305.3,69.8 341.5,63.6 377.6,53.9 413.8,44.7 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,106.5 124.4,97.8 160.5,93.4 196.7,82.5 232.9,78.8 269.1,72.5 305.3,64.4 341.5,55.8 377.6,48.5 413.8,39.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.3 ns | 2.12 µs | 4.02 µs | 4.27 µs | 4.92 µs |
| D38 | 4.92 ns | 3.98 µs | 4.79 µs | 5.95 µs | 5.27 µs |
| D57 | 2.8 ns | 4.32 µs | 4.73 µs | 4.11 µs | 7.14 µs |
| D76 | 3.52 ns | 5.05 µs | 3.97 µs | 9.33 µs | 11.1 µs |
| D115 | 8.14 ns | 4.14 µs | 12.9 µs | 16.5 µs | 22.8 µs |
| D153 | 16 ns | 7.03 µs | 12.9 µs | 23.2 µs | 36.9 µs |
| D230 | 41.2 ns | 9.1 µs | 20.1 µs | 37 µs | 61.3 µs |
| D307 | 62.8 ns | 9.13 µs | 31 µs | 75.2 µs | 126 µs |
| D462 | 103 ns | 14.3 µs | 66.9 µs | 122 µs | 248 µs |
| D616 | 121 ns | 30.1 µs | 135 µs | 293 µs | 491 µs |
| D924 | 87.8 ns | 66 µs | 213 µs | 657 µs | 1.12 ms |
| D1232 | 365 ns | 111 µs | 463 µs | 945 µs | 2.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,190.2 124.4,197.2 160.5,194.4 196.7,184.0 232.9,175.6 269.1,163.9 305.3,158.6 341.5,152.5 377.6,150.5 413.8,154.5 450.0,136.8 450.0,27.7 413.8,37.2 377.6,47.4 341.5,55.9 305.3,64.3 269.1,73.2 232.9,79.5 196.7,85.5 160.5,94.4 124.4,99.9 88.2,103.7 52.0,104.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,190.2 124.4,197.2 160.5,194.4 196.7,184.0 232.9,175.6 269.1,163.9 305.3,158.6 341.5,152.5 377.6,150.5 413.8,154.5 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,107.2 124.4,106.1 160.5,104.2 196.7,106.7 232.9,100.1 269.1,96.9 305.3,96.8 341.5,91.2 377.6,82.0 413.8,72.3 450.0,65.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.9 124.4,105.0 160.5,107.2 196.7,92.6 232.9,92.6 269.1,87.0 305.3,81.7 341.5,72.1 377.6,63.4 413.8,57.8 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,102.2 124.4,106.8 160.5,96.6 196.7,89.5 232.9,85.2 269.1,79.5 305.3,70.7 341.5,64.7 377.6,53.8 413.8,43.8 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,103.7 124.4,99.9 160.5,94.4 196.7,85.5 232.9,79.5 269.1,73.2 305.3,64.3 341.5,55.9 377.6,47.4 413.8,37.2 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.05 ns | 1.76 µs | 3.52 µs | 3.76 µs | 4.32 µs |
| D38 | 4.57 ns | 3.48 µs | 3.98 µs | 4.86 µs | 4.12 µs |
| D57 | 2.61 µs | 5.67 µs | 6.18 µs | 4.94 µs | 8.76 µs |
| D76 | 2.82 µs | 6.5 µs | 4.73 µs | 10.8 µs | 12.6 µs |
| D115 | 4.83 µs | 10 µs | 13.6 µs | 22.4 µs | 29.3 µs |
| D153 | 2.35 µs | 8.41 µs | 17.1 µs | 25.1 µs | 39.9 µs |
| D230 | 3.07 µs | 11.1 µs | 21.3 µs | 39.2 µs | 62.7 µs |
| D307 | 3.26 µs | 10.9 µs | 54.7 µs | 82.3 µs | 126 µs |
| D462 | 3.09 µs | 18.8 µs | 82.9 µs | 136 µs | 254 µs |
| D616 | 3.53 µs | 32.9 µs | 137 µs | 292 µs | 453 µs |
| D924 | 2.12 µs | 71.6 µs | 215 µs | 620 µs | 931 µs |
| D1232 | 4.48 µs | 117 µs | 424 µs | 788 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.2 88.2,191.2 124.4,112.4 160.5,111.4 196.7,104.7 232.9,113.7 269.1,110.4 305.3,109.6 341.5,110.3 377.6,108.6 413.8,115.0 450.0,105.7 450.0,26.3 413.8,39.5 377.6,48.4 341.5,55.6 305.3,64.2 269.1,72.9 232.9,78.5 196.7,82.4 160.5,92.9 124.4,97.4 88.2,106.7 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.2 88.2,191.2 124.4,112.4 160.5,111.4 196.7,104.7 232.9,113.7 269.1,110.4 305.3,109.6 341.5,110.3 377.6,108.6 413.8,115.0 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,108.8 124.4,102.7 160.5,101.1 196.7,95.7 232.9,97.9 269.1,94.4 305.3,94.7 341.5,87.9 377.6,80.9 413.8,71.3 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,107.1 124.4,101.7 160.5,105.0 196.7,91.9 232.9,89.1 269.1,86.4 305.3,74.6 341.5,69.5 377.6,63.2 413.8,57.7 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,104.7 124.4,104.5 160.5,94.8 196.7,85.7 232.9,84.3 269.1,78.8 305.3,69.6 341.5,63.4 377.6,53.8 413.8,44.5 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,106.7 124.4,97.4 160.5,92.9 196.7,82.4 232.9,78.5 269.1,72.9 305.3,64.2 341.5,55.6 377.6,48.4 413.8,39.5 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.45 ns | 137 ns | 157 ns | 162 ns | 178 ns |
| D38 | 3.16 ns | 161 ns | 171 ns | 202 ns | 167 ns |
| D57 | 193 ns | 317 ns | 253 ns | 185 ns | 339 ns |
| D76 | 198 ns | 330 ns | 194 ns | 414 ns | 443 ns |
| D115 | 376 ns | 419 ns | 790 ns | 794 ns | 975 ns |
| D153 | 282 ns | 600 ns | 824 ns | 971 ns | 1.15 µs |
| D230 | 575 ns | 622 ns | 885 ns | 1.18 µs | 1.38 µs |
| D307 | 893 ns | 782 ns | 1.72 µs | 2.15 µs | 2.85 µs |
| D462 | 950 ns | 1.04 µs | 2.22 µs | 2.64 µs | 4.24 µs |
| D616 | 1.23 µs | 1.69 µs | 2.94 µs | 4.47 µs | 6.09 µs |
| D924 | 865 ns | 2.49 µs | 3.46 µs | 7.7 µs | 10.4 µs |
| D1232 | 2.44 µs | 3.56 µs | 7.19 µs | 9.47 µs | 28.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,190.0 124.4,118.6 160.5,118.1 196.7,107.0 232.9,112.0 269.1,99.6 305.3,92.0 341.5,90.9 377.6,86.3 413.8,92.5 450.0,74.5 450.0,31.6 413.8,49.2 377.6,58.6 341.5,64.9 305.3,71.8 269.1,84.4 232.9,87.5 196.7,90.4 160.5,104.1 124.4,108.8 88.2,121.1 52.0,120.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,190.0 124.4,118.6 160.5,118.1 196.7,107.0 232.9,112.0 269.1,99.6 305.3,92.0 341.5,90.9 377.6,86.3 413.8,92.5 450.0,74.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,121.8 124.4,110.0 160.5,109.3 196.7,105.1 232.9,98.9 269.1,98.3 305.3,94.3 341.5,89.3 377.6,80.9 413.8,74.2 450.0,67.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,120.6 124.4,113.9 160.5,118.5 196.7,94.1 232.9,93.4 269.1,92.1 305.3,80.6 341.5,76.1 377.6,71.2 413.8,68.4 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,117.8 124.4,119.3 160.5,105.3 196.7,94.0 232.9,90.5 269.1,87.2 305.3,76.7 341.5,73.2 377.6,64.0 413.8,54.5 450.0,50.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.0 88.2,121.1 124.4,108.8 160.5,104.1 196.7,90.4 232.9,87.5 269.1,84.4 305.3,71.8 341.5,64.9 377.6,58.6 413.8,49.2 450.0,31.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 155 ns | 193 ns | 202 ns | 218 ns |
| D38 | 1.06 ns | 194 ns | 196 ns | 215 ns | 189 ns |
| D57 | 260 ns | 383 ns | 291 ns | 223 ns | 379 ns |
| D76 | 259 ns | 399 ns | 231 ns | 494 ns | 524 ns |
| D115 | 485 ns | 476 ns | 906 ns | 924 ns | 1.09 µs |
| D153 | 382 ns | 727 ns | 922 ns | 1.08 µs | 1.29 µs |
| D230 | 794 ns | 746 ns | 1.01 µs | 1.35 µs | 1.55 µs |
| D307 | 1.24 µs | 921 ns | 1.94 µs | 2.42 µs | 3.17 µs |
| D462 | 1.21 µs | 1.19 µs | 2.53 µs | 2.93 µs | 4.62 µs |
| D616 | 1.56 µs | 1.85 µs | 3.32 µs | 4.94 µs | 6.55 µs |
| D924 | 1.1 µs | 2.74 µs | 3.78 µs | 8.22 µs | 11 µs |
| D1232 | 2.97 µs | 3.92 µs | 7.63 µs | 10.1 µs | 29.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,209.1 124.4,113.4 160.5,113.4 196.7,102.6 232.9,106.7 269.1,94.0 305.3,86.3 341.5,86.7 377.6,82.3 413.8,88.4 450.0,71.1 450.0,31.1 413.8,48.4 377.6,57.4 341.5,63.4 305.3,70.0 269.1,82.4 232.9,85.6 196.7,88.5 160.5,101.2 124.4,106.9 88.2,118.9 52.0,116.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,209.1 124.4,113.4 160.5,113.4 196.7,102.6 232.9,106.7 269.1,94.0 305.3,86.3 341.5,86.7 377.6,82.3 413.8,88.4 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,118.5 124.4,106.7 160.5,105.9 196.7,102.9 232.9,95.5 269.1,95.1 305.3,91.4 341.5,86.9 377.6,79.3 413.8,72.5 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.6 88.2,118.3 124.4,111.5 160.5,115.5 196.7,91.7 232.9,91.4 269.1,89.8 305.3,78.5 341.5,73.9 377.6,69.2 413.8,66.9 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,116.7 124.4,116.0 160.5,102.2 196.7,91.4 232.9,88.7 269.1,84.8 305.3,74.6 341.5,71.3 377.6,62.3 413.8,53.4 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.4 88.2,118.9 124.4,106.9 160.5,101.2 196.7,88.5 232.9,85.6 269.1,82.4 305.3,70.0 341.5,63.4 377.6,57.4 413.8,48.4 450.0,31.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
