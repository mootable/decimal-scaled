# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.02 ns | 2.01 µs | 3.26 µs | 3.2 µs | 3.75 µs |
| D38 | 1.49 µs | 4.03 µs | 4.55 µs | 5.78 µs | 10.1 µs |
| D57 | 1.28 µs | 4.39 µs | 6.13 µs | 10.7 µs | 15.1 µs |
| D76 | 1.33 µs | 6.67 µs | 10.6 µs | 8.53 µs | 19.8 µs |
| D115 | 1.54 µs | 8.22 µs | 20.9 µs | 33.1 µs | 42.9 µs |
| D153 | 1.69 µs | 10 µs | 22.4 µs | 40.4 µs | 61.5 µs |
| D230 | 1.4 µs | 20.6 µs | 39.1 µs | 70.2 µs | 115 µs |
| D307 | 1.65 µs | 26 µs | 58.6 µs | 126 µs | 183 µs |
| D462 | 1.59 µs | 45.4 µs | 98.8 µs | 240 µs | 313 µs |
| D616 | 1.15 µs | 53.8 µs | 219 µs | 423 µs | 708 µs |
| D924 | 1.78 µs | 88.3 µs | 448 µs | 864 µs | 1.02 ms |
| D1232 | 1.75 µs | 206 µs | 380 µs | 1.04 ms | 2.96 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,119.4 124.4,121.3 160.5,120.7 196.7,118.9 232.9,117.8 269.1,120.1 305.3,118.1 341.5,118.6 377.6,122.5 413.8,117.1 450.0,117.3 450.0,25.1 413.8,38.3 377.6,42.8 341.5,53.0 305.3,59.7 269.1,65.4 232.9,73.2 196.7,77.6 160.5,87.3 124.4,90.6 88.2,95.6 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,119.4 124.4,121.3 160.5,120.7 196.7,118.9 232.9,117.8 269.1,120.1 305.3,118.1 341.5,118.6 377.6,122.5 413.8,117.1 450.0,117.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.6 88.2,107.0 124.4,105.9 160.5,100.7 196.7,98.1 232.9,95.7 269.1,86.7 305.3,83.8 341.5,76.9 377.6,74.8 413.8,68.7 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,105.5 124.4,101.8 160.5,94.9 196.7,86.6 232.9,85.7 269.1,78.8 305.3,73.8 341.5,67.3 377.6,57.4 413.8,48.5 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,102.5 124.4,94.9 160.5,97.7 196.7,80.9 232.9,78.4 269.1,71.5 305.3,64.3 341.5,56.3 377.6,49.2 413.8,40.4 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,95.6 124.4,90.6 160.5,87.3 196.7,77.6 232.9,73.2 269.1,65.4 305.3,59.7 341.5,53.0 377.6,42.8 413.8,38.3 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.39 µs | 3.63 µs | 5.88 µs | 5.85 µs | 6.79 µs |
| D38 | 3.38 µs | 4.56 µs | 7.11 µs | 6.52 µs | 9.51 µs |
| D57 | 2.78 µs | 3.04 µs | 3.54 µs | 6.15 µs | 8.14 µs |
| D76 | 2.83 µs | 4.63 µs | 6.26 µs | 5.14 µs | 10.1 µs |
| D115 | 6.44 µs | 9.13 µs | 13.9 µs | 17.6 µs | 23.3 µs |
| D153 | 7.03 µs | 10.2 µs | 13.8 µs | 20.5 µs | 31 µs |
| D230 | 7.81 µs | 16.9 µs | 24.4 µs | 45.3 µs | 66.2 µs |
| D307 | 12.8 µs | 27.7 µs | 41.2 µs | 83.2 µs | 130 µs |
| D462 | 12.3 µs | 38 µs | 60.8 µs | 156 µs | 216 µs |
| D616 | 11.7 µs | 58.6 µs | 178 µs | 313 µs | 553 µs |
| D924 | 32.9 µs | 103 µs | 397 µs | 750 µs | 908 µs |
| D1232 | 36.9 µs | 260 µs | 390 µs | 963 µs | 2.56 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,183.5 124.4,187.8 160.5,187.4 196.7,169.5 232.9,167.7 269.1,165.4 305.3,154.7 341.5,155.4 377.6,156.5 413.8,134.2 450.0,131.6 450.0,39.6 413.8,62.1 377.6,72.9 341.5,93.2 305.3,104.3 269.1,119.0 232.9,135.4 196.7,141.7 160.5,159.8 124.4,164.5 88.2,161.1 52.0,168.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,183.5 124.4,187.8 160.5,187.4 196.7,169.5 232.9,167.7 269.1,165.4 305.3,154.7 341.5,155.4 377.6,156.5 413.8,134.2 450.0,131.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.0 88.2,177.0 124.4,185.8 160.5,176.7 196.7,162.0 232.9,159.5 269.1,148.6 305.3,137.9 341.5,131.0 377.6,121.6 413.8,109.4 450.0,89.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.5 88.2,167.4 124.4,182.6 160.5,170.2 196.7,152.9 232.9,153.1 269.1,140.6 305.3,129.3 341.5,120.8 377.6,97.5 413.8,80.1 450.0,80.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,169.3 124.4,170.5 160.5,174.5 196.7,147.7 232.9,144.4 269.1,127.2 305.3,114.0 341.5,100.3 377.6,85.2 413.8,66.2 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.4 88.2,161.1 124.4,164.5 160.5,159.8 196.7,141.7 232.9,135.4 269.1,119.0 305.3,104.3 341.5,93.2 377.6,72.9 413.8,62.1 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.58 ns | 2 µs | 3.24 µs | 3.18 µs | 3.74 µs |
| D38 | 1.37 µs | 4.01 µs | 4.49 µs | 5.75 µs | 10.1 µs |
| D57 | 1.16 µs | 4.38 µs | 5.94 µs | 10.7 µs | 15 µs |
| D76 | 1.25 µs | 6.81 µs | 10.6 µs | 8.5 µs | 19.7 µs |
| D115 | 1.43 µs | 8.23 µs | 20.6 µs | 33.3 µs | 43.1 µs |
| D153 | 1.58 µs | 9.9 µs | 22.8 µs | 40.1 µs | 61.6 µs |
| D230 | 1.31 µs | 20.6 µs | 37.4 µs | 70.4 µs | 116 µs |
| D307 | 1.59 µs | 28 µs | 58.2 µs | 125 µs | 182 µs |
| D462 | 1.48 µs | 45.3 µs | 96.9 µs | 241 µs | 314 µs |
| D616 | 1.02 µs | 54.4 µs | 218 µs | 423 µs | 708 µs |
| D924 | 1.65 µs | 88.4 µs | 448 µs | 863 µs | 1.02 ms |
| D1232 | 1.63 µs | 206 µs | 380 µs | 1.03 ms | 2.96 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,120.4 124.4,122.4 160.5,121.6 196.7,119.9 232.9,118.6 269.1,120.9 305.3,118.5 341.5,119.4 377.6,124.0 413.8,118.0 450.0,118.2 450.0,25.1 413.8,38.3 377.6,42.9 341.5,52.9 305.3,59.7 269.1,65.4 232.9,73.1 196.7,77.6 160.5,87.3 124.4,90.7 88.2,95.6 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,120.4 124.4,122.4 160.5,121.6 196.7,119.9 232.9,118.6 269.1,120.9 305.3,118.5 341.5,119.4 377.6,124.0 413.8,118.0 450.0,118.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,107.1 124.4,106.0 160.5,100.5 196.7,98.1 232.9,95.8 269.1,86.8 305.3,82.9 341.5,77.0 377.6,74.7 413.8,68.7 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,105.6 124.4,102.2 160.5,95.0 196.7,86.8 232.9,85.5 269.1,79.3 305.3,73.9 341.5,67.5 377.6,57.5 413.8,48.5 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,102.6 124.4,94.9 160.5,97.7 196.7,80.8 232.9,78.5 269.1,71.5 305.3,64.4 341.5,56.2 377.6,49.3 413.8,40.4 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,95.6 124.4,90.7 160.5,87.3 196.7,77.6 232.9,73.1 269.1,65.4 305.3,59.7 341.5,52.9 377.6,42.9 413.8,38.3 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.72 µs | 2.59 µs | 2.64 µs | 2.97 µs |
| D38 | 4.22 ns | 2.01 µs | 3.14 µs | 2.9 µs | 4.21 µs |
| D57 | 1.39 ns | 3.88 µs | 5.26 µs | 9.08 µs | 12.9 µs |
| D76 | 1.73 ns | 6.22 µs | 9.39 µs | 7.05 µs | 15.1 µs |
| D115 | 14 ns | 12.9 µs | 20.3 µs | 24.3 µs | 33.5 µs |
| D153 | 16 ns | 14.9 µs | 19.7 µs | 31.3 µs | 43.8 µs |
| D230 | 24.5 ns | 25 µs | 37.2 µs | 63.2 µs | 88.5 µs |
| D307 | 52 ns | 40.3 µs | 59.5 µs | 115 µs | 166 µs |
| D462 | 82 ns | 58.4 µs | 89.1 µs | 199 µs | 263 µs |
| D616 | 43.9 ns | 85.8 µs | 238 µs | 397 µs | 662 µs |
| D924 | 106 ns | 158 µs | 527 µs | 863 µs | 933 µs |
| D1232 | 111 ns | 364 µs | 509 µs | 1.07 ms | 2.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,205.9 160.5,203.2 196.7,177.2 232.9,175.6 269.1,170.3 305.3,161.0 341.5,155.3 377.6,163.1 413.8,152.2 450.0,151.5 450.0,27.7 413.8,39.4 377.6,43.7 341.5,55.1 305.3,60.8 269.1,68.7 232.9,77.4 196.7,80.7 160.5,90.6 124.4,92.5 88.2,106.4 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,205.9 160.5,203.2 196.7,177.2 232.9,175.6 269.1,170.3 305.3,161.0 341.5,155.3 377.6,163.1 413.8,152.2 450.0,151.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,115.6 124.4,107.5 160.5,101.6 196.7,92.5 232.9,90.8 269.1,84.4 305.3,78.4 341.5,73.8 377.6,69.0 413.8,61.5 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,110.1 124.4,103.7 160.5,96.5 196.7,86.9 232.9,87.3 269.1,79.4 305.3,73.6 341.5,68.6 377.6,56.4 413.8,46.5 450.0,47.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,111.1 124.4,96.9 160.5,100.0 196.7,84.7 232.9,81.5 269.1,72.8 305.3,65.4 341.5,58.6 377.6,50.0 413.8,40.4 450.0,37.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.4 124.4,92.5 160.5,90.6 196.7,80.7 232.9,77.4 269.1,68.7 305.3,60.8 341.5,55.1 377.6,43.7 413.8,39.4 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.4 ns | 1.59 µs | 2.69 µs | 2.65 µs | 3.21 µs |
| D38 | 753 ns | 3.44 µs | 3.13 µs | 4.48 µs | 8.04 µs |
| D57 | 484 ns | 3.39 µs | 5.03 µs | 8.56 µs | 5.47 µs |
| D76 | 544 ns | 5.27 µs | 8.35 µs | 6.97 µs | 16.5 µs |
| D115 | 620 ns | 6.35 µs | 17.2 µs | 30.4 µs | 38.9 µs |
| D153 | 702 ns | 7.77 µs | 14.9 µs | 36.7 µs | 56.2 µs |
| D230 | 577 ns | 17 µs | 34.5 µs | 65.3 µs | 107 µs |
| D307 | 704 ns | 25 µs | 48.3 µs | 117 µs | 170 µs |
| D462 | 652 ns | 36.6 µs | 81 µs | 211 µs | 269 µs |
| D616 | 491 ns | 49.3 µs | 202 µs | 393 µs | 667 µs |
| D924 | 798 ns | 83.4 µs | 425 µs | 826 µs | 974 µs |
| D1232 | 823 ns | 191 µs | 362 µs | 999 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,127.8 124.4,133.3 160.5,131.9 196.7,130.2 232.9,128.7 269.1,131.1 305.3,128.6 341.5,129.6 377.6,133.1 413.8,127.1 450.0,126.7 450.0,25.6 413.8,38.9 377.6,43.6 341.5,54.9 305.3,60.5 269.1,66.3 232.9,74.3 196.7,78.9 160.5,89.5 124.4,103.2 88.2,98.4 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,127.8 124.4,133.3 160.5,131.9 196.7,130.2 232.9,128.7 269.1,131.1 305.3,128.6 341.5,129.6 377.6,133.1 413.8,127.1 450.0,126.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.5 88.2,108.9 124.4,109.1 160.5,103.7 196.7,101.3 232.9,98.8 269.1,89.1 305.3,84.3 341.5,79.6 377.6,75.9 413.8,69.4 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,110.1 124.4,104.3 160.5,98.0 196.7,89.0 232.9,90.8 269.1,80.3 305.3,76.2 341.5,69.8 377.6,58.4 413.8,49.2 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,105.7 124.4,97.6 160.5,100.2 196.7,81.9 232.9,79.6 269.1,72.4 305.3,65.2 341.5,57.9 377.6,50.2 413.8,40.9 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,98.4 124.4,103.2 160.5,89.5 196.7,78.9 232.9,74.3 269.1,66.3 305.3,60.5 341.5,54.9 377.6,43.6 413.8,38.9 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.75 µs | 2.89 µs | 2.94 µs | 3.45 µs |
| D38 | 4.22 ns | 2.25 µs | 3.6 µs | 3.24 µs | 4.83 µs |
| D57 | 322 ns | 3.99 µs | 5 µs | 8.74 µs | 12 µs |
| D76 | 398 ns | 6.21 µs | 8.8 µs | 6.66 µs | 15.1 µs |
| D115 | 931 ns | 13 µs | 21.4 µs | 27.4 µs | 37.3 µs |
| D153 | 1.08 µs | 14.3 µs | 21.2 µs | 32 µs | 50.3 µs |
| D230 | 1.24 µs | 25.1 µs | 40 µs | 76.4 µs | 115 µs |
| D307 | 2 µs | 41.3 µs | 67 µs | 143 µs | 230 µs |
| D462 | 1.9 µs | 58.4 µs | 105 µs | 279 µs | 396 µs |
| D616 | 1.79 µs | 94.2 µs | 313 µs | 562 µs | 1.01 ms |
| D924 | 5 µs | 173 µs | 709 µs | 1.38 ms | 1.73 ms |
| D1232 | 5.68 µs | 447 µs | 707 µs | 1.84 ms | 4.88 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,138.3 160.5,135.7 196.7,125.2 232.9,123.3 269.1,121.6 305.3,115.7 341.5,116.3 377.6,117.0 413.8,104.3 450.0,102.7 450.0,18.9 413.8,31.8 377.6,38.5 341.5,50.1 305.3,56.8 269.1,65.4 232.9,75.7 196.7,79.4 160.5,90.6 124.4,93.5 88.2,104.7 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,138.3 160.5,135.7 196.7,125.2 232.9,123.3 269.1,121.6 305.3,115.7 341.5,116.3 377.6,117.0 413.8,104.3 450.0,102.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,114.2 124.4,107.1 160.5,101.6 196.7,92.5 232.9,91.3 269.1,84.3 305.3,78.1 341.5,73.8 377.6,67.9 413.8,60.3 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,108.4 124.4,104.3 160.5,97.3 196.7,86.3 232.9,86.4 269.1,78.5 305.3,72.1 341.5,66.5 377.6,53.0 413.8,42.8 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,109.7 124.4,97.4 160.5,100.8 196.7,83.2 232.9,81.3 269.1,70.5 305.3,62.7 341.5,54.4 377.6,45.7 413.8,34.6 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.7 124.4,93.5 160.5,90.6 196.7,79.4 232.9,75.7 269.1,65.4 305.3,56.8 341.5,50.1 377.6,38.5 413.8,31.8 450.0,18.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 1.16 µs | 2.72 µs | 2.17 µs | 2.8 µs |
| D38 | 5.62 ns | 2.12 µs | 3.26 µs | 2.3 µs | 3.8 µs |
| D57 | 1.98 ns | 2.62 µs | 3.4 µs | 5.62 µs | 9.7 µs |
| D76 | 2.46 ns | 3.79 µs | 5.91 µs | 4.16 µs | 9.86 µs |
| D115 | 11.6 ns | 4.93 µs | 10.1 µs | 13.9 µs | 18.4 µs |
| D153 | 18.1 ns | 5.43 µs | 8.69 µs | 17 µs | 29.5 µs |
| D230 | 31.6 ns | 9.92 µs | 16.5 µs | 36.6 µs | 63.6 µs |
| D307 | 73.3 ns | 12.6 µs | 23.3 µs | 69.2 µs | 110 µs |
| D462 | 104 ns | 16.5 µs | 47.7 µs | 133 µs | 184 µs |
| D616 | 105 ns | 26.3 µs | 132 µs | 253 µs | 455 µs |
| D924 | 151 ns | 49.2 µs | 271 µs | 567 µs | 740 µs |
| D1232 | 225 ns | 122 µs | 239 µs | 753 µs | 2.14 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,201.5 160.5,198.8 196.7,179.6 232.9,174.0 269.1,167.1 305.3,156.7 341.5,152.3 377.6,152.2 413.8,147.8 450.0,142.8 450.0,29.2 413.8,42.3 377.6,48.3 341.5,59.6 305.3,66.0 269.1,72.8 232.9,82.3 196.7,88.1 160.5,95.9 124.4,96.1 88.2,107.7 52.0,111.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,201.5 160.5,198.8 196.7,179.6 232.9,174.0 269.1,167.1 305.3,156.7 341.5,152.3 377.6,152.2 413.8,147.8 450.0,142.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.5 88.2,115.0 124.4,112.3 160.5,107.8 196.7,104.5 232.9,103.3 269.1,95.8 305.3,92.9 341.5,89.5 377.6,83.7 413.8,76.0 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.9 88.2,109.6 124.4,109.1 160.5,102.2 196.7,95.6 232.9,97.5 269.1,89.5 305.3,85.2 341.5,76.3 377.6,63.7 413.8,54.8 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.7 88.2,114.0 124.4,102.9 160.5,106.6 196.7,91.7 232.9,89.1 269.1,79.6 305.3,71.7 341.5,63.6 377.6,55.6 413.8,45.6 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,107.7 124.4,96.1 160.5,95.9 196.7,88.1 232.9,82.3 269.1,72.8 305.3,66.0 341.5,59.6 377.6,48.3 413.8,42.3 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.77 µs | 3.69 µs | 3.1 µs | 3.82 µs |
| D38 | 4.57 ns | 2.86 µs | 4.35 µs | 3.15 µs | 4.92 µs |
| D57 | 2.96 ns | 4.09 µs | 5.32 µs | 8.15 µs | 11.3 µs |
| D76 | 3 ns | 5.9 µs | 8.22 µs | 6.18 µs | 13.3 µs |
| D115 | 9.98 ns | 12.4 µs | 13.4 µs | 21.8 µs | 26.1 µs |
| D153 | 18 ns | 7.84 µs | 13.8 µs | 21.3 µs | 36.1 µs |
| D230 | 31.3 ns | 14.1 µs | 21.1 µs | 43.8 µs | 74.9 µs |
| D307 | 75.8 ns | 17.5 µs | 44 µs | 79.7 µs | 117 µs |
| D462 | 108 ns | 25.2 µs | 61.1 µs | 156 µs | 197 µs |
| D616 | 94.8 ns | 32.1 µs | 142 µs | 269 µs | 447 µs |
| D924 | 171 ns | 57.4 µs | 292 µs | 560 µs | 646 µs |
| D1232 | 224 ns | 133 µs | 237 µs | 691 µs | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,196.5 160.5,196.4 196.7,181.4 232.9,174.1 269.1,167.3 305.3,156.3 341.5,151.9 377.6,153.5 413.8,146.2 450.0,142.9 450.0,26.1 413.8,44.0 377.6,48.6 341.5,58.7 305.3,65.2 269.1,70.7 232.9,79.8 196.7,83.8 160.5,92.2 124.4,94.2 88.2,104.5 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,196.5 160.5,196.4 196.7,181.4 232.9,174.1 269.1,167.3 305.3,156.3 341.5,151.9 377.6,153.5 413.8,146.2 450.0,142.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,111.3 124.4,106.8 160.5,102.3 196.7,93.0 232.9,98.7 269.1,91.4 305.3,88.8 341.5,84.3 377.6,81.2 413.8,74.0 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,106.0 124.4,103.5 160.5,98.2 196.7,92.1 232.9,91.7 269.1,86.5 305.3,77.3 341.5,73.3 377.6,62.8 413.8,53.9 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,110.0 124.4,98.3 160.5,101.7 196.7,86.0 232.9,86.3 269.1,77.4 305.3,70.0 341.5,61.6 377.6,54.9 413.8,45.8 450.0,43.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,104.5 124.4,94.2 160.5,92.2 196.7,83.8 232.9,79.8 269.1,70.7 305.3,65.2 341.5,58.7 377.6,48.6 413.8,44.0 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 1.07 µs | 2.54 µs | 2.14 µs | 2.75 µs |
| D38 | 4.92 ns | 1.98 µs | 3.08 µs | 2.18 µs | 3.75 µs |
| D57 | 2.29 ns | 2.41 µs | 3.17 µs | 5.47 µs | 9.68 µs |
| D76 | 2.73 ns | 3.52 µs | 5.66 µs | 4.11 µs | 9.61 µs |
| D115 | 11.6 ns | 4.47 µs | 10.2 µs | 14 µs | 19.1 µs |
| D153 | 18.1 ns | 5.16 µs | 8.83 µs | 16.4 µs | 29.6 µs |
| D230 | 31.7 ns | 9.97 µs | 17 µs | 36 µs | 61.7 µs |
| D307 | 66.7 ns | 12.5 µs | 22.6 µs | 66.2 µs | 107 µs |
| D462 | 98.3 ns | 17.4 µs | 44.9 µs | 133 µs | 180 µs |
| D616 | 94.9 ns | 25.4 µs | 128 µs | 251 µs | 452 µs |
| D924 | 174 ns | 47.2 µs | 266 µs | 562 µs | 728 µs |
| D1232 | 235 ns | 120 µs | 236 µs | 766 µs | 2.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,199.7 160.5,197.5 196.7,179.5 232.9,174.0 269.1,167.1 305.3,157.9 341.5,153.1 377.6,153.5 413.8,146.0 450.0,142.3 450.0,29.2 413.8,42.5 377.6,48.4 341.5,59.9 305.3,66.3 269.1,73.1 232.9,82.2 196.7,87.7 160.5,96.2 124.4,96.1 88.2,107.9 52.0,111.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,199.7 160.5,197.5 196.7,179.5 232.9,174.0 269.1,167.1 305.3,157.9 341.5,153.1 377.6,153.5 413.8,146.0 450.0,142.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,115.8 124.4,113.4 160.5,108.7 196.7,105.7 232.9,103.9 269.1,95.8 305.3,92.9 341.5,88.8 377.6,84.1 413.8,76.5 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.3 124.4,110.0 160.5,102.8 196.7,95.5 232.9,97.3 269.1,89.1 305.3,85.6 341.5,77.1 377.6,64.1 413.8,55.0 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.8 88.2,114.6 124.4,103.2 160.5,106.8 196.7,91.5 232.9,89.6 269.1,79.8 305.3,72.3 341.5,63.6 377.6,55.7 413.8,45.7 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,107.9 124.4,96.1 160.5,96.2 196.7,87.7 232.9,82.2 269.1,73.1 305.3,66.3 341.5,59.9 377.6,48.4 413.8,42.5 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.77 µs | 3.69 µs | 3.12 µs | 3.82 µs |
| D38 | 4.57 ns | 2.86 µs | 4.36 µs | 3.16 µs | 4.93 µs |
| D57 | 9.38 ns | 4.06 µs | 5.73 µs | 8.15 µs | 11.3 µs |
| D76 | 9.3 ns | 5.9 µs | 8.25 µs | 7.23 µs | 13.3 µs |
| D115 | 10.3 ns | 12.5 µs | 13 µs | 21.7 µs | 26.1 µs |
| D153 | 17.4 ns | 7.9 µs | 13.8 µs | 21.4 µs | 35.8 µs |
| D230 | 31 ns | 14.1 µs | 20.4 µs | 43.9 µs | 75 µs |
| D307 | 65.2 ns | 17.9 µs | 43.6 µs | 80 µs | 117 µs |
| D462 | 101 ns | 25.5 µs | 61.9 µs | 157 µs | 196 µs |
| D616 | 87 ns | 32.2 µs | 141 µs | 269 µs | 449 µs |
| D924 | 172 ns | 57.2 µs | 289 µs | 560 µs | 650 µs |
| D1232 | 222 ns | 134 µs | 238 µs | 684 µs | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,182.2 160.5,182.3 196.7,181.1 232.9,174.6 269.1,167.4 305.3,158.2 341.5,152.7 377.6,154.6 413.8,146.2 450.0,143.0 450.0,26.1 413.8,43.9 377.6,48.5 341.5,58.8 305.3,65.2 269.1,70.7 232.9,79.9 196.7,83.8 160.5,92.2 124.4,94.2 88.2,104.5 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,182.2 160.5,182.3 196.7,181.1 232.9,174.6 269.1,167.4 305.3,158.2 341.5,152.7 377.6,154.6 413.8,146.2 450.0,143.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,111.3 124.4,106.9 160.5,102.3 196.7,93.0 232.9,98.6 269.1,91.4 305.3,88.5 341.5,84.1 377.6,81.2 413.8,74.1 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,106.0 124.4,102.6 160.5,98.1 196.7,92.5 232.9,91.7 269.1,86.9 305.3,77.5 341.5,73.1 377.6,62.9 413.8,54.0 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,110.0 124.4,98.3 160.5,99.7 196.7,86.1 232.9,86.3 269.1,77.4 305.3,69.9 341.5,61.6 377.6,54.9 413.8,45.8 450.0,43.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,104.5 124.4,94.2 160.5,92.2 196.7,83.8 232.9,79.9 269.1,70.7 305.3,65.2 341.5,58.8 377.6,48.5 413.8,43.9 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 2.14 µs | 4.34 µs | 3.79 µs | 4.58 µs |
| D38 | 4.92 ns | 3.37 µs | 5.17 µs | 3.94 µs | 6.09 µs |
| D57 | 2.62 ns | 3.28 µs | 4.23 µs | 7.18 µs | 10.2 µs |
| D76 | 3 ns | 4.64 µs | 7.43 µs | 5.26 µs | 11.9 µs |
| D115 | 10.6 ns | 5.93 µs | 12.9 µs | 16.5 µs | 22.6 µs |
| D153 | 18 ns | 6.86 µs | 11.4 µs | 19.9 µs | 34.6 µs |
| D230 | 31.9 ns | 12.6 µs | 20.2 µs | 41.5 µs | 70.1 µs |
| D307 | 62.8 ns | 15.2 µs | 27 µs | 75 µs | 120 µs |
| D462 | 95 ns | 19.5 µs | 51.8 µs | 145 µs | 196 µs |
| D616 | 98.4 ns | 29.8 µs | 141 µs | 277 µs | 489 µs |
| D924 | 138 ns | 54.8 µs | 292 µs | 608 µs | 769 µs |
| D1232 | 236 ns | 134 µs | 256 µs | 783 µs | 2.23 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,198.1 160.5,196.4 196.7,180.7 232.9,174.1 269.1,167.0 305.3,158.6 341.5,153.5 377.6,153.1 413.8,148.8 450.0,142.2 450.0,28.6 413.8,41.8 377.6,47.5 341.5,58.8 305.3,64.9 269.1,71.5 232.9,80.3 196.7,85.6 160.5,93.6 124.4,95.5 88.2,101.9 52.0,105.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,198.1 160.5,196.4 196.7,180.7 232.9,174.1 269.1,167.0 305.3,158.6 341.5,153.5 377.6,153.1 413.8,148.8 450.0,142.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,109.2 124.4,109.5 160.5,105.2 196.7,102.2 232.9,100.4 269.1,92.8 305.3,90.5 341.5,87.4 377.6,82.2 413.8,74.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,103.9 124.4,106.4 160.5,99.4 196.7,92.5 232.9,94.1 269.1,87.0 305.3,83.4 341.5,75.3 377.6,62.9 413.8,53.8 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,107.3 124.4,99.8 160.5,103.7 196.7,89.5 232.9,87.2 269.1,78.0 305.3,70.7 341.5,62.5 377.6,54.5 413.8,44.8 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.4 88.2,101.9 124.4,95.5 160.5,93.6 196.7,85.6 232.9,80.3 269.1,71.5 305.3,64.9 341.5,58.8 377.6,47.5 413.8,41.8 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.17 ns | 1.8 µs | 3.87 µs | 3.27 µs | 3.99 µs |
| D38 | 4.22 ns | 3 µs | 4.39 µs | 3.15 µs | 4.97 µs |
| D57 | 2.22 µs | 4.5 µs | 5.44 µs | 8.57 µs | 11.8 µs |
| D76 | 2.29 µs | 6.03 µs | 8.58 µs | 6.4 µs | 13.8 µs |
| D115 | 5.34 µs | 13 µs | 13.9 µs | 22.5 µs | 27 µs |
| D153 | 3.02 µs | 8.32 µs | 14.6 µs | 22 µs | 36.9 µs |
| D230 | 2.47 µs | 14.7 µs | 21 µs | 44.9 µs | 76.4 µs |
| D307 | 3.23 µs | 18 µs | 45.5 µs | 81.4 µs | 119 µs |
| D462 | 3.11 µs | 25.9 µs | 63.8 µs | 159 µs | 198 µs |
| D616 | 2.02 µs | 33.7 µs | 145 µs | 272 µs | 452 µs |
| D924 | 3.54 µs | 59.1 µs | 292 µs | 565 µs | 650 µs |
| D1232 | 3.58 µs | 135 µs | 241 µs | 681 µs | 2.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,192.1 124.4,114.4 160.5,114.0 196.7,103.5 232.9,110.6 269.1,113.1 305.3,109.7 341.5,110.2 377.6,115.5 413.8,108.6 450.0,108.5 450.0,25.9 413.8,43.9 377.6,48.4 341.5,58.6 305.3,65.0 269.1,70.5 232.9,79.5 196.7,83.4 160.5,91.7 124.4,93.7 88.2,104.4 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,192.1 124.4,114.4 160.5,114.0 196.7,103.5 232.9,110.6 269.1,113.1 305.3,109.7 341.5,110.2 377.6,115.5 413.8,108.6 450.0,108.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.0 88.2,110.7 124.4,105.6 160.5,102.0 196.7,92.5 232.9,98.0 269.1,91.0 305.3,88.4 341.5,83.9 377.6,80.6 413.8,73.7 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,105.9 124.4,103.3 160.5,97.6 196.7,91.6 232.9,91.0 269.1,86.5 305.3,76.9 341.5,72.7 377.6,62.5 413.8,53.8 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,110.1 124.4,97.6 160.5,101.3 196.7,85.7 232.9,85.9 269.1,77.1 305.3,69.7 341.5,61.4 377.6,54.7 413.8,45.7 450.0,43.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.4 124.4,93.7 160.5,91.7 196.7,83.4 232.9,79.5 269.1,70.5 305.3,65.0 341.5,58.6 377.6,48.4 413.8,43.9 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 130 ns | 169 ns | 156 ns | 160 ns |
| D38 | 4.92 ns | 132 ns | 175 ns | 134 ns | 181 ns |
| D57 | 135 ns | 204 ns | 221 ns | 327 ns | 446 ns |
| D76 | 153 ns | 298 ns | 355 ns | 246 ns | 489 ns |
| D115 | 393 ns | 553 ns | 804 ns | 795 ns | 864 ns |
| D153 | 479 ns | 608 ns | 689 ns | 766 ns | 1.02 µs |
| D230 | 482 ns | 828 ns | 905 ns | 1.3 µs | 1.73 µs |
| D307 | 885 ns | 1.31 µs | 1.39 µs | 2.09 µs | 2.57 µs |
| D462 | 860 ns | 1.53 µs | 1.51 µs | 3.01 µs | 3.3 µs |
| D616 | 606 ns | 1.58 µs | 3.1 µs | 4.12 µs | 6.06 µs |
| D924 | 1.47 µs | 1.93 µs | 4.91 µs | 6.98 µs | 7.3 µs |
| D1232 | 1.7 µs | 4.12 µs | 3.98 µs | 7.64 µs | 29.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,124.8 160.5,122.6 196.7,106.2 232.9,102.8 269.1,102.7 305.3,92.1 341.5,92.6 377.6,98.7 413.8,83.3 450.0,80.7 450.0,31.1 413.8,55.5 377.6,58.7 341.5,69.3 305.3,73.6 269.1,80.5 232.9,89.7 196.7,92.5 160.5,102.4 124.4,104.0 88.2,119.7 52.0,121.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,124.8 160.5,122.6 196.7,106.2 232.9,102.8 269.1,102.7 305.3,92.1 341.5,92.6 377.6,98.7 413.8,83.3 450.0,80.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.4 88.2,125.2 124.4,117.6 160.5,111.0 196.7,100.3 232.9,98.6 269.1,93.3 305.3,85.3 341.5,82.6 377.6,82.0 413.8,78.6 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.9 88.2,120.3 124.4,116.2 160.5,108.0 196.7,93.8 232.9,96.5 269.1,91.7 305.3,84.3 341.5,82.8 377.6,70.3 413.8,62.4 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,124.9 124.4,109.4 160.5,114.4 196.7,94.0 232.9,94.6 269.1,85.5 305.3,77.2 341.5,70.8 377.6,65.4 413.8,56.3 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,119.7 124.4,104.0 160.5,102.4 196.7,92.5 232.9,89.7 269.1,80.5 305.3,73.6 341.5,69.3 377.6,58.7 413.8,55.5 450.0,31.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 159 ns | 211 ns | 180 ns | 197 ns |
| D38 | 4.92 ns | 163 ns | 210 ns | 147 ns | 198 ns |
| D57 | 207 ns | 269 ns | 287 ns | 446 ns | 564 ns |
| D76 | 237 ns | 410 ns | 477 ns | 307 ns | 603 ns |
| D115 | 641 ns | 805 ns | 1.02 µs | 1.06 µs | 1.11 µs |
| D153 | 741 ns | 857 ns | 865 ns | 954 ns | 1.3 µs |
| D230 | 825 ns | 1.21 µs | 1.21 µs | 1.71 µs | 2.08 µs |
| D307 | 1.42 µs | 1.81 µs | 1.8 µs | 2.64 µs | 3.16 µs |
| D462 | 1.4 µs | 1.99 µs | 1.93 µs | 3.6 µs | 3.77 µs |
| D616 | 945 ns | 1.94 µs | 3.86 µs | 4.83 µs | 6.89 µs |
| D924 | 2.39 µs | 2.53 µs | 5.89 µs | 7.87 µs | 8.27 µs |
| D1232 | 2.66 µs | 5.33 µs | 4.7 µs | 8.53 µs | 32.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,117.3 160.5,115.0 196.7,97.7 232.9,95.2 269.1,93.3 305.3,83.9 341.5,84.2 377.6,91.0 413.8,74.9 450.0,73.0 450.0,29.7 413.8,53.3 377.6,56.5 341.5,66.9 305.3,70.0 269.1,77.3 232.9,85.5 196.7,88.2 160.5,98.8 124.4,99.9 88.2,118.1 52.0,118.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,117.3 160.5,115.0 196.7,97.7 232.9,95.2 269.1,93.3 305.3,83.9 341.5,84.2 377.6,91.0 413.8,74.9 450.0,73.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,121.6 124.4,112.8 160.5,105.5 196.7,93.8 232.9,92.7 269.1,86.7 305.3,79.6 341.5,78.0 377.6,78.5 413.8,73.9 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.0 88.2,117.1 124.4,111.7 160.5,102.9 196.7,89.7 232.9,92.5 269.1,86.7 305.3,79.8 341.5,78.6 377.6,66.5 413.8,59.2 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.8 88.2,123.4 124.4,104.0 160.5,110.5 196.7,89.0 232.9,90.8 269.1,80.7 305.3,73.1 341.5,67.8 377.6,62.6 413.8,54.2 450.0,52.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.1 124.4,99.9 160.5,98.8 196.7,88.2 232.9,85.5 269.1,77.3 305.3,70.0 341.5,66.9 377.6,56.5 413.8,53.3 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
