# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.77 ns | 2.1 µs | 3.27 µs | 3.52 µs | 2.48 µs |
| D38 | 4.13 µs | 7.83 µs | 7.49 µs | 9.05 µs | 15.6 µs |
| D57 | 5.18 µs | 9.63 µs | 11.8 µs | 15.4 µs | 18 µs |
| D76 | 4.75 µs | 9.96 µs | 16.1 µs | 18.3 µs | 15.6 µs |
| D115 | 4.72 µs | 11.8 µs | 24.6 µs | 35 µs | 42.4 µs |
| D153 | 5.16 µs | 15.5 µs | 25.3 µs | 45.1 µs | 65.6 µs |
| D230 | 5.23 µs | 26.1 µs | 42.2 µs | 53.7 µs | 107 µs |
| D307 | 4.78 µs | 28.2 µs | 56.9 µs | 125 µs | 192 µs |
| D462 | 4.84 µs | 45.2 µs | 112 µs | 239 µs | 401 µs |
| D616 | 5.28 µs | 61.7 µs | 122 µs | 421 µs | 657 µs |
| D924 | 5.02 µs | 134 µs | 452 µs | 945 µs | 1.55 ms |
| D1232 | 5.63 µs | 171 µs | 713 µs | 1.52 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,106.7 124.4,103.9 160.5,104.9 196.7,105.0 232.9,103.9 269.1,103.7 305.3,104.9 341.5,104.7 377.6,103.7 413.8,104.3 450.0,102.8 450.0,23.6 413.8,33.1 377.6,43.8 341.5,49.9 305.3,59.1 269.1,66.4 232.9,72.4 196.7,77.8 160.5,90.2 124.4,88.4 88.2,90.2 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,106.7 124.4,103.9 160.5,104.9 196.7,105.0 232.9,103.9 269.1,103.7 305.3,104.9 341.5,104.7 377.6,103.7 413.8,104.3 450.0,102.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,98.8 124.4,96.2 160.5,95.8 196.7,93.6 232.9,90.3 269.1,83.8 305.3,82.8 341.5,77.0 377.6,73.1 413.8,63.5 450.0,60.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,99.3 124.4,93.7 160.5,89.8 196.7,84.6 232.9,84.2 269.1,77.8 305.3,74.1 341.5,65.7 377.6,64.7 413.8,48.4 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,97.0 124.4,90.3 160.5,88.2 196.7,80.2 232.9,77.0 269.1,74.9 305.3,64.4 341.5,56.3 377.6,49.3 413.8,39.3 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,90.2 124.4,88.4 160.5,90.2 196.7,77.8 232.9,72.4 269.1,66.4 305.3,59.1 341.5,49.9 377.6,43.8 413.8,33.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 µs | 3.76 µs | 5.89 µs | 6.43 µs | 4.45 µs |
| D38 | 3.28 µs | 5.93 µs | 7 µs | 7.35 µs | 9.67 µs |
| D57 | 3.67 µs | 4.74 µs | 4.99 µs | 6.15 µs | 7.62 µs |
| D76 | 3.33 µs | 4.96 µs | 6.22 µs | 7.73 µs | 6.71 µs |
| D115 | 6.46 µs | 8.96 µs | 12.9 µs | 18.8 µs | 21.9 µs |
| D153 | 7.12 µs | 10.2 µs | 13.7 µs | 24.1 µs | 34.3 µs |
| D230 | 9.3 µs | 17.7 µs | 27.5 µs | 33.5 µs | 60 µs |
| D307 | 12.4 µs | 26.3 µs | 40.5 µs | 82.3 µs | 137 µs |
| D462 | 12.6 µs | 37.9 µs | 71.1 µs | 159 µs | 273 µs |
| D616 | 23 µs | 72 µs | 97.2 µs | 313 µs | 518 µs |
| D924 | 33 µs | 164 µs | 405 µs | 810 µs | 1.36 ms |
| D1232 | 45.5 µs | 218 µs | 722 µs | 1.45 ms | 2.93 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,184.2 124.4,181.8 160.5,183.9 196.7,169.5 232.9,167.4 269.1,161.6 305.3,155.4 341.5,155.0 377.6,141.9 413.8,134.1 450.0,127.1 450.0,36.7 413.8,53.4 377.6,74.3 341.5,88.2 305.3,103.2 269.1,121.1 232.9,133.2 196.7,143.0 160.5,168.7 124.4,165.9 88.2,160.7 52.0,177.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,184.2 124.4,181.8 160.5,183.9 196.7,169.5 232.9,167.4 269.1,161.6 305.3,155.4 341.5,155.0 377.6,141.9 413.8,134.1 450.0,127.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,171.4 124.4,176.2 160.5,175.2 196.7,162.4 232.9,159.5 269.1,147.6 305.3,139.0 341.5,131.1 377.6,117.1 413.8,99.2 450.0,93.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.5 88.2,167.7 124.4,175.1 160.5,170.3 196.7,154.5 232.9,153.2 269.1,138.0 305.3,129.6 341.5,117.4 377.6,110.6 413.8,79.6 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.6 88.2,166.7 124.4,170.5 160.5,165.6 196.7,146.3 232.9,140.9 269.1,133.7 305.3,114.2 341.5,99.9 377.6,85.2 413.8,64.6 450.0,51.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.6 88.2,160.7 124.4,165.9 160.5,168.7 196.7,143.0 232.9,133.2 269.1,121.1 305.3,103.2 341.5,88.2 377.6,74.3 413.8,53.4 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.08 µs | 3.25 µs | 3.5 µs | 2.47 µs |
| D38 | 4.01 µs | 7.82 µs | 7.46 µs | 9.01 µs | 15.6 µs |
| D57 | 5.32 µs | 9.6 µs | 11.8 µs | 15.4 µs | 18.1 µs |
| D76 | 4.59 µs | 9.95 µs | 16.1 µs | 18.3 µs | 15.6 µs |
| D115 | 4.59 µs | 11.8 µs | 24.5 µs | 36 µs | 42.8 µs |
| D153 | 5.03 µs | 15.4 µs | 25.1 µs | 45.2 µs | 65.3 µs |
| D230 | 5.13 µs | 25.9 µs | 41.8 µs | 54.4 µs | 106 µs |
| D307 | 4.66 µs | 28.4 µs | 56.4 µs | 124 µs | 192 µs |
| D462 | 4.92 µs | 45.7 µs | 113 µs | 240 µs | 398 µs |
| D616 | 5.17 µs | 62.3 µs | 123 µs | 422 µs | 657 µs |
| D924 | 4.95 µs | 133 µs | 452 µs | 945 µs | 1.55 ms |
| D1232 | 5.55 µs | 172 µs | 716 µs | 1.52 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,107.0 124.4,103.5 160.5,105.4 196.7,105.4 232.9,104.2 269.1,104.0 305.3,105.2 341.5,104.5 377.6,103.9 413.8,104.4 450.0,103.0 450.0,23.6 413.8,33.1 377.6,43.8 341.5,50.0 305.3,59.0 269.1,66.4 232.9,72.4 196.7,77.7 160.5,90.2 124.4,88.4 88.2,90.2 52.0,113.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,107.0 124.4,103.5 160.5,105.4 196.7,105.4 232.9,104.2 269.1,104.0 305.3,105.2 341.5,104.5 377.6,103.9 413.8,104.4 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,98.8 124.4,96.2 160.5,95.8 196.7,93.6 232.9,90.4 269.1,83.9 305.3,82.8 341.5,76.9 377.6,73.0 413.8,63.6 450.0,60.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,99.4 124.4,93.7 160.5,89.8 196.7,84.6 232.9,84.3 269.1,78.0 305.3,74.3 341.5,65.6 377.6,64.6 413.8,48.4 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,97.0 124.4,90.4 160.5,88.2 196.7,79.8 232.9,77.0 269.1,74.7 305.3,64.5 341.5,56.3 377.6,49.3 413.8,39.3 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.1 88.2,90.2 124.4,88.4 160.5,90.2 196.7,77.7 232.9,72.4 269.1,66.4 305.3,59.0 341.5,50.0 377.6,43.8 413.8,33.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.81 µs | 2.6 µs | 2.87 µs | 1.95 µs |
| D38 | 3.74 ns | 2.6 µs | 3.08 µs | 3.27 µs | 4.31 µs |
| D57 | 2.11 ns | 5.89 µs | 7.44 µs | 9.05 µs | 11.9 µs |
| D76 | 2.02 ns | 6.84 µs | 9.39 µs | 11.8 µs | 10.2 µs |
| D115 | 14 ns | 12.8 µs | 18.9 µs | 26.6 µs | 33.5 µs |
| D153 | 16 ns | 15.3 µs | 19.9 µs | 36 µs | 47.9 µs |
| D230 | 31.6 ns | 26.1 µs | 41.8 µs | 46.2 µs | 81.5 µs |
| D307 | 50 ns | 38.7 µs | 57.9 µs | 116 µs | 178 µs |
| D462 | 69.3 ns | 59 µs | 102 µs | 198 µs | 342 µs |
| D616 | 87.6 ns | 104 µs | 134 µs | 399 µs | 616 µs |
| D924 | 104 ns | 245 µs | 529 µs | 946 µs | 1.47 ms |
| D1232 | 158 ns | 305 µs | 921 µs | 1.62 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,201.3 196.7,177.2 232.9,175.6 269.1,167.1 305.3,161.5 341.5,157.4 377.6,154.5 413.8,152.4 450.0,147.2 450.0,25.4 413.8,33.8 377.6,44.6 341.5,51.9 305.3,60.0 269.1,69.7 232.9,76.3 196.7,80.7 160.5,95.4 124.4,93.6 88.2,106.2 52.0,116.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,200.7 160.5,201.3 196.7,177.2 232.9,175.6 269.1,167.1 305.3,161.5 341.5,157.4 377.6,154.5 413.8,152.4 450.0,147.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.0 88.2,112.4 124.4,102.3 160.5,100.4 196.7,92.6 232.9,90.4 269.1,83.8 305.3,78.9 341.5,73.7 377.6,66.6 413.8,56.0 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,110.3 124.4,99.4 160.5,96.5 196.7,87.8 232.9,87.2 269.1,78.0 305.3,73.9 341.5,66.9 377.6,63.5 413.8,46.5 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,109.6 124.4,96.9 160.5,93.7 196.7,83.6 232.9,79.8 269.1,76.7 305.3,65.3 341.5,58.7 377.6,50.0 413.8,39.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.0 88.2,106.2 124.4,93.6 160.5,95.4 196.7,80.7 232.9,76.3 269.1,69.7 305.3,60.0 341.5,51.9 377.6,44.6 413.8,33.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.63 µs | 2.69 µs | 2.93 µs | 2.1 µs |
| D38 | 4.72 µs | 8.68 µs | 6.16 µs | 7.89 µs | 13.7 µs |
| D57 | 4.12 µs | 8.26 µs | 10.2 µs | 13.7 µs | 5.29 µs |
| D76 | 3.8 µs | 8.54 µs | 14 µs | 16.2 µs | 13.4 µs |
| D115 | 3.82 µs | 10.3 µs | 21.8 µs | 31.3 µs | 39.1 µs |
| D153 | 4.14 µs | 13.5 µs | 19.8 µs | 40.8 µs | 60.2 µs |
| D230 | 4.23 µs | 23.3 µs | 38.5 µs | 50.2 µs | 101 µs |
| D307 | 3.92 µs | 25.3 µs | 47.2 µs | 115 µs | 180 µs |
| D462 | 2.94 µs | 37 µs | 94 µs | 211 µs | 340 µs |
| D616 | 4.31 µs | 56.1 µs | 114 µs | 394 µs | 621 µs |
| D924 | 4.03 µs | 124 µs | 423 µs | 900 µs | 1.47 ms |
| D1232 | 4.6 µs | 160 µs | 674 µs | 1.46 ms | 3.21 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,105.0 124.4,106.7 160.5,107.7 196.7,107.7 232.9,106.6 269.1,106.4 305.3,107.3 341.5,110.9 377.6,106.2 413.8,107.0 450.0,105.4 450.0,24.1 413.8,33.8 377.6,44.5 341.5,52.0 305.3,59.9 269.1,67.0 232.9,73.4 196.7,78.8 160.5,92.1 124.4,103.6 88.2,91.8 52.0,115.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,105.0 124.4,106.7 160.5,107.7 196.7,107.7 232.9,106.6 269.1,106.4 305.3,107.3 341.5,110.9 377.6,106.2 413.8,107.0 450.0,105.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.2 88.2,97.5 124.4,98.1 160.5,97.7 196.7,95.4 232.9,92.0 269.1,85.2 305.3,84.2 341.5,79.5 377.6,74.3 413.8,64.5 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,101.7 124.4,95.4 160.5,91.5 196.7,86.1 232.9,87.3 269.1,79.0 305.3,76.5 341.5,67.9 377.6,65.5 413.8,49.2 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,98.7 124.4,91.8 160.5,89.7 196.7,81.6 232.9,78.3 269.1,75.7 305.3,65.4 341.5,57.9 377.6,50.1 413.8,39.9 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.1 88.2,91.8 124.4,103.6 160.5,92.1 196.7,78.8 232.9,73.4 269.1,67.0 305.3,59.9 341.5,52.0 377.6,44.5 413.8,33.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.9 µs | 3.19 µs | 2.26 µs |
| D38 | 3.74 ns | 2.9 µs | 3.57 µs | 3.7 µs | 4.84 µs |
| D57 | 528 ns | 6.06 µs | 7.09 µs | 8.87 µs | 11.5 µs |
| D76 | 449 ns | 6.5 µs | 8.72 µs | 11.2 µs | 10.1 µs |
| D115 | 920 ns | 12.6 µs | 19 µs | 28.3 µs | 35.6 µs |
| D153 | 1.1 µs | 14.4 µs | 20.8 µs | 37.8 µs | 56.6 µs |
| D230 | 1.38 µs | 26 µs | 43.3 µs | 54.9 µs | 103 µs |
| D307 | 1.84 µs | 39.6 µs | 66.1 µs | 141 µs | 241 µs |
| D462 | 2 µs | 57.5 µs | 121 µs | 284 µs | 494 µs |
| D616 | 3.67 µs | 116 µs | 170 µs | 560 µs | 951 µs |
| D924 | 5 µs | 276 µs | 720 µs | 1.48 ms | 2.53 ms |
| D1232 | 7.03 µs | 371 µs | 1.31 ms | 2.7 ms | 5.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,132.2 160.5,134.2 196.7,125.3 232.9,123.1 269.1,120.3 305.3,116.7 341.5,115.7 377.6,108.2 413.8,104.3 450.0,100.1 450.0,17.4 413.8,27.0 377.6,39.2 341.5,47.3 305.3,56.2 269.1,66.8 232.9,74.2 196.7,80.0 160.5,95.6 124.4,94.0 88.2,104.7 52.0,114.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,132.2 160.5,134.2 196.7,125.3 232.9,123.1 269.1,120.3 305.3,116.7 341.5,115.7 377.6,108.2 413.8,104.3 450.0,100.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,111.1 124.4,101.9 160.5,101.1 196.7,92.8 232.9,91.2 269.1,83.8 305.3,78.6 341.5,74.0 377.6,65.3 413.8,54.6 450.0,50.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,108.5 124.4,100.0 160.5,97.4 196.7,87.8 232.9,86.6 269.1,77.5 305.3,72.3 341.5,64.8 377.6,60.5 413.8,42.7 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,108.0 124.4,97.2 160.5,94.3 196.7,82.8 232.9,79.2 269.1,74.6 305.3,62.9 341.5,54.2 377.6,45.8 413.8,33.7 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.2 88.2,104.7 124.4,94.0 160.5,95.6 196.7,80.0 232.9,74.2 269.1,66.8 305.3,56.2 341.5,47.3 377.6,39.2 413.8,27.0 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.98 ns | 1.27 µs | 2.73 µs | 2.84 µs | 1.6 µs |
| D38 | 4.98 ns | 2.73 µs | 2.98 µs | 2.67 µs | 4.14 µs |
| D57 | 2.81 ns | 3.77 µs | 4.72 µs | 5.6 µs | 9.2 µs |
| D76 | 3.43 ns | 4.09 µs | 5.83 µs | 7.27 µs | 6.39 µs |
| D115 | 11.6 ns | 4.73 µs | 9.73 µs | 14.4 µs | 17.9 µs |
| D153 | 18.1 ns | 5.44 µs | 9.63 µs | 19.2 µs | 32.7 µs |
| D230 | 40.8 ns | 10.7 µs | 18.4 µs | 27.4 µs | 57.5 µs |
| D307 | 64.3 ns | 12.3 µs | 22.6 µs | 69.6 µs | 116 µs |
| D462 | 104 ns | 16.2 µs | 54.8 µs | 134 µs | 229 µs |
| D616 | 149 ns | 30.3 µs | 67.5 µs | 257 µs | 426 µs |
| D924 | 150 ns | 75.4 µs | 272 µs | 622 µs | 1.06 ms |
| D1232 | 356 ns | 104 µs | 463 µs | 1.05 ms | 2.37 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,190.1 124.4,197.2 160.5,194.7 196.7,179.6 232.9,174.0 269.1,164.0 305.3,158.3 341.5,152.4 377.6,147.9 413.8,147.8 450.0,137.1 450.0,27.8 413.8,37.9 377.6,49.2 341.5,56.9 305.3,65.3 269.1,74.0 232.9,81.0 196.7,88.5 160.5,101.3 124.4,96.8 88.2,106.7 52.0,118.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,190.1 124.4,197.2 160.5,194.7 196.7,179.6 232.9,174.0 269.1,164.0 305.3,158.3 341.5,152.4 377.6,147.9 413.8,147.8 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.3 88.2,111.8 124.4,107.8 160.5,106.8 196.7,105.0 232.9,103.3 269.1,94.9 305.3,93.1 341.5,89.7 377.6,82.0 413.8,70.6 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.8 88.2,110.7 124.4,105.0 160.5,102.4 196.7,96.0 232.9,96.2 269.1,88.1 305.3,85.6 341.5,74.6 377.6,72.0 413.8,54.7 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,112.1 124.4,102.9 160.5,99.7 196.7,91.2 232.9,87.6 269.1,83.2 305.3,71.6 341.5,63.5 377.6,55.5 413.8,44.5 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.4 88.2,106.7 124.4,96.8 160.5,101.3 196.7,88.5 232.9,81.0 269.1,74.0 305.3,65.3 341.5,56.9 377.6,49.2 413.8,37.9 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 1.94 µs | 3.7 µs | 3.94 µs | 2.3 µs |
| D38 | 4.05 ns | 3.69 µs | 3.98 µs | 3.62 µs | 5.4 µs |
| D57 | 3.16 ns | 6 µs | 7.24 µs | 8.15 µs | 10.5 µs |
| D76 | 3.74 ns | 6.34 µs | 8.3 µs | 10.4 µs | 9.09 µs |
| D115 | 9.97 ns | 12.4 µs | 12.2 µs | 23.4 µs | 26.4 µs |
| D153 | 18 ns | 7.85 µs | 13.8 µs | 24.4 µs | 39.3 µs |
| D230 | 40.4 ns | 14.6 µs | 23.2 µs | 33 µs | 69.1 µs |
| D307 | 65.1 ns | 17.2 µs | 42.6 µs | 81.2 µs | 128 µs |
| D462 | 108 ns | 25.5 µs | 70.9 µs | 158 µs | 249 µs |
| D616 | 134 ns | 36 µs | 72.7 µs | 270 µs | 416 µs |
| D924 | 173 ns | 87.5 µs | 293 µs | 610 µs | 925 µs |
| D1232 | 357 ns | 113 µs | 447 µs | 925 µs | 3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,192.7 124.4,195.7 160.5,193.6 196.7,181.5 232.9,174.1 269.1,164.1 305.3,158.2 341.5,151.9 377.6,149.2 413.8,146.0 450.0,137.0 450.0,25.0 413.8,39.5 377.6,49.4 341.5,55.8 305.3,64.1 269.1,71.7 232.9,78.7 196.7,83.7 160.5,96.9 124.4,95.1 88.2,103.4 52.0,114.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,192.7 124.4,195.7 160.5,193.6 196.7,181.5 232.9,174.1 269.1,164.1 305.3,158.2 341.5,151.9 377.6,149.2 413.8,146.0 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.0 88.2,108.1 124.4,102.1 160.5,101.4 196.7,93.0 232.9,98.7 269.1,91.0 305.3,89.0 341.5,84.1 377.6,79.8 413.8,68.8 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,107.1 124.4,99.7 160.5,98.0 196.7,93.3 232.9,91.7 269.1,85.3 305.3,77.7 341.5,71.4 377.6,71.1 413.8,53.8 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,108.3 124.4,98.3 160.5,95.2 196.7,85.2 232.9,84.6 269.1,80.9 305.3,69.7 341.5,61.5 377.6,54.8 413.8,44.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.0 88.2,103.4 124.4,95.1 160.5,96.9 196.7,83.7 232.9,78.7 269.1,71.7 305.3,64.1 341.5,55.8 377.6,49.4 413.8,39.5 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.17 µs | 2.55 µs | 2.79 µs | 1.59 µs |
| D38 | 4.36 ns | 2.56 µs | 2.82 µs | 2.53 µs | 4.1 µs |
| D57 | 2.81 ns | 3.56 µs | 4.38 µs | 5.45 µs | 9.17 µs |
| D76 | 3.43 ns | 3.92 µs | 5.53 µs | 7.15 µs | 6.23 µs |
| D115 | 11.6 ns | 4.34 µs | 9.72 µs | 14.4 µs | 17.4 µs |
| D153 | 18.1 ns | 5.11 µs | 9.01 µs | 19.3 µs | 31.7 µs |
| D230 | 40.8 ns | 10.1 µs | 19.3 µs | 27.1 µs | 56 µs |
| D307 | 60 ns | 12 µs | 21.8 µs | 66 µs | 113 µs |
| D462 | 172 ns | 16.5 µs | 52.8 µs | 134 µs | 227 µs |
| D616 | 127 ns | 29.4 µs | 67.7 µs | 253 µs | 425 µs |
| D924 | 165 ns | 73.4 µs | 270 µs | 619 µs | 1.05 ms |
| D1232 | 348 ns | 102 µs | 459 µs | 1.05 ms | 2.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.7 196.7,179.5 232.9,174.1 269.1,164.0 305.3,159.2 341.5,146.1 377.6,149.9 413.8,146.6 450.0,137.4 450.0,27.9 413.8,37.9 377.6,49.2 341.5,57.0 305.3,65.6 269.1,74.3 232.9,81.4 196.7,88.8 160.5,101.6 124.4,96.8 88.2,106.8 52.0,118.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.7 196.7,179.5 232.9,174.1 269.1,164.0 305.3,159.2 341.5,146.1 377.6,149.9 413.8,146.6 450.0,137.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.4 88.2,112.6 124.4,108.5 160.5,107.3 196.7,106.1 232.9,104.0 269.1,95.5 305.3,93.4 341.5,89.5 377.6,82.3 413.8,71.0 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,111.4 124.4,105.9 160.5,103.1 196.7,96.1 232.9,97.0 269.1,87.5 305.3,86.0 341.5,75.1 377.6,72.0 413.8,54.8 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,112.8 124.4,103.2 160.5,99.9 196.7,91.2 232.9,87.6 269.1,83.3 305.3,72.3 341.5,63.5 377.6,55.6 413.8,44.5 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.5 88.2,106.8 124.4,96.8 160.5,101.6 196.7,88.8 232.9,81.4 269.1,74.3 305.3,65.6 341.5,57.0 377.6,49.2 413.8,37.9 450.0,27.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.95 µs | 3.7 µs | 3.93 µs | 2.31 µs |
| D38 | 4.05 ns | 3.69 µs | 3.97 µs | 3.6 µs | 5.41 µs |
| D57 | 12.3 ns | 6.21 µs | 7.29 µs | 8.2 µs | 10.5 µs |
| D76 | 10.4 ns | 6.34 µs | 8.33 µs | 10.5 µs | 9.17 µs |
| D115 | 10.3 ns | 12.5 µs | 12.2 µs | 23.6 µs | 26.4 µs |
| D153 | 17.4 ns | 7.91 µs | 13.6 µs | 24.6 µs | 39.6 µs |
| D230 | 40 ns | 14.7 µs | 23.3 µs | 32.9 µs | 67.2 µs |
| D307 | 60.5 ns | 17.5 µs | 42.4 µs | 81.3 µs | 126 µs |
| D462 | 165 ns | 25.5 µs | 71.4 µs | 157 µs | 250 µs |
| D616 | 132 ns | 36.1 µs | 72.9 µs | 270 µs | 415 µs |
| D924 | 182 ns | 87.7 µs | 292 µs | 612 µs | 924 µs |
| D1232 | 380 ns | 114 µs | 448 µs | 924 µs | 2.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.7 124.4,178.9 160.5,180.9 196.7,181.1 232.9,174.6 269.1,164.2 305.3,159.1 341.5,146.7 377.6,149.4 413.8,145.4 450.0,136.3 450.0,25.0 413.8,39.6 377.6,49.5 341.5,55.8 305.3,64.3 269.1,72.1 232.9,78.6 196.7,83.6 160.5,96.8 124.4,95.1 88.2,103.3 52.0,113.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.7 124.4,178.9 160.5,180.9 196.7,181.1 232.9,174.6 269.1,164.2 305.3,159.1 341.5,146.7 377.6,149.4 413.8,145.4 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.0 88.2,108.1 124.4,101.6 160.5,101.4 196.7,93.0 232.9,98.6 269.1,91.0 305.3,88.8 341.5,84.1 377.6,79.8 413.8,68.8 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,107.2 124.4,99.6 160.5,98.0 196.7,93.3 232.9,91.9 269.1,85.2 305.3,77.8 341.5,71.3 377.6,71.1 413.8,53.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,108.4 124.4,98.2 160.5,95.1 196.7,85.0 232.9,84.6 269.1,80.9 305.3,69.7 341.5,61.5 377.6,54.8 413.8,44.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.9 88.2,103.3 124.4,95.1 160.5,96.8 196.7,83.6 232.9,78.6 269.1,72.1 305.3,64.3 341.5,55.8 377.6,49.5 413.8,39.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.33 µs | 4.34 µs | 4.6 µs | 2.86 µs |
| D38 | 4.36 ns | 4.36 µs | 4.72 µs | 4.6 µs | 6.65 µs |
| D57 | 2.81 ns | 4.79 µs | 5.75 µs | 7.11 µs | 9.13 µs |
| D76 | 4.05 ns | 5.13 µs | 7.29 µs | 9.12 µs | 7.79 µs |
| D115 | 10.6 ns | 5.87 µs | 12.2 µs | 18 µs | 21.1 µs |
| D153 | 18 ns | 6.79 µs | 11.4 µs | 23 µs | 37.1 µs |
| D230 | 41.1 ns | 13.3 µs | 22.2 µs | 31.5 µs | 65.6 µs |
| D307 | 58.5 ns | 14.7 µs | 26.5 µs | 75.5 µs | 127 µs |
| D462 | 97.4 ns | 20.2 µs | 60.6 µs | 146 µs | 246 µs |
| D616 | 135 ns | 34.4 µs | 74.1 µs | 279 µs | 460 µs |
| D924 | 139 ns | 81.8 µs | 298 µs | 662 µs | 1.12 ms |
| D1232 | 364 ns | 112 µs | 494 µs | 1.12 ms | 2.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,192.6 196.7,180.7 232.9,174.2 269.1,163.9 305.3,159.5 341.5,153.2 377.6,149.2 413.8,148.8 450.0,136.8 450.0,27.3 413.8,37.1 377.6,48.2 341.5,56.0 305.3,64.1 269.1,72.4 232.9,79.4 196.7,86.4 160.5,98.8 124.4,96.8 88.2,100.8 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,192.6 196.7,180.7 232.9,174.2 269.1,163.9 305.3,159.5 341.5,153.2 377.6,149.2 413.8,148.8 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,113.8 88.2,106.0 124.4,104.8 160.5,104.0 196.7,102.3 232.9,100.5 269.1,92.2 305.3,90.9 341.5,87.0 377.6,80.4 413.8,69.6 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.0 124.4,102.6 160.5,99.6 196.7,93.3 232.9,94.1 269.1,85.8 305.3,83.6 341.5,73.4 377.6,70.9 413.8,53.6 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,105.4 124.4,100.0 160.5,96.9 196.7,88.4 232.9,85.4 269.1,81.5 305.3,70.6 341.5,62.4 377.6,54.4 413.8,43.7 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,100.8 124.4,96.8 160.5,98.8 196.7,86.4 232.9,79.4 269.1,72.4 305.3,64.1 341.5,56.0 377.6,48.2 413.8,37.1 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.98 µs | 3.88 µs | 4.12 µs | 2.42 µs |
| D38 | 3.74 ns | 3.89 µs | 4 µs | 3.63 µs | 5.44 µs |
| D57 | 2.84 µs | 6.15 µs | 7.63 µs | 8.58 µs | 11 µs |
| D76 | 2.57 µs | 6.62 µs | 8.64 µs | 10.8 µs | 9.5 µs |
| D115 | 5.41 µs | 13 µs | 12.7 µs | 24 µs | 27.1 µs |
| D153 | 3.01 µs | 8.33 µs | 14.6 µs | 26.7 µs | 40.2 µs |
| D230 | 3.08 µs | 15.3 µs | 24.4 µs | 33.9 µs | 67.3 µs |
| D307 | 3.08 µs | 17.5 µs | 43.9 µs | 82.4 µs | 127 µs |
| D462 | 3.12 µs | 26.2 µs | 74.1 µs | 160 µs | 252 µs |
| D616 | 3.57 µs | 38 µs | 77.1 µs | 273 µs | 418 µs |
| D924 | 3.48 µs | 89.5 µs | 294 µs | 615 µs | 932 µs |
| D1232 | 4.51 µs | 114 µs | 455 µs | 932 µs | 3.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,112.6 196.7,103.3 232.9,110.6 269.1,110.3 305.3,110.3 341.5,110.2 377.6,108.5 413.8,108.8 450.0,105.6 450.0,24.9 413.8,39.4 377.6,49.4 341.5,55.7 305.3,64.2 269.1,72.1 232.9,78.5 196.7,83.3 160.5,96.4 124.4,94.6 88.2,103.3 52.0,113.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,111.3 160.5,112.6 196.7,103.3 232.9,110.6 269.1,110.3 305.3,110.3 341.5,110.2 377.6,108.5 413.8,108.8 450.0,105.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,107.4 124.4,101.7 160.5,100.8 196.7,92.5 232.9,98.0 269.1,90.5 305.3,88.8 341.5,83.8 377.6,79.2 413.8,68.5 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,107.1 124.4,99.1 160.5,97.5 196.7,92.8 232.9,91.0 269.1,84.6 305.3,77.4 341.5,70.9 377.6,70.4 413.8,53.8 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,108.3 124.4,97.6 160.5,94.7 196.7,84.9 232.9,83.5 269.1,80.6 305.3,69.5 341.5,61.3 377.6,54.7 413.8,44.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.3 88.2,103.3 124.4,94.6 160.5,96.4 196.7,83.3 232.9,78.5 269.1,72.1 305.3,64.2 341.5,55.7 377.6,49.4 413.8,39.4 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 142 ns | 169 ns | 173 ns | 99.8 ns |
| D38 | 4.36 ns | 170 ns | 167 ns | 154 ns | 198 ns |
| D57 | 198 ns | 336 ns | 321 ns | 320 ns | 414 ns |
| D76 | 179 ns | 331 ns | 356 ns | 411 ns | 316 ns |
| D115 | 400 ns | 567 ns | 697 ns | 876 ns | 889 ns |
| D153 | 512 ns | 617 ns | 689 ns | 1 µs | 1.13 µs |
| D230 | 583 ns | 915 ns | 995 ns | 981 ns | 1.52 µs |
| D307 | 782 ns | 1.24 µs | 1.29 µs | 2.07 µs | 2.8 µs |
| D462 | 851 ns | 1.53 µs | 1.81 µs | 3.02 µs | 4.18 µs |
| D616 | 1.2 µs | 1.85 µs | 1.63 µs | 4.14 µs | 5.66 µs |
| D924 | 1.49 µs | 2.95 µs | 4.85 µs | 7.58 µs | 10.4 µs |
| D1232 | 2.3 µs | 3.39 µs | 7.4 µs | 11.2 µs | 33.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,118.1 160.5,119.9 196.7,105.9 232.9,101.6 269.1,99.4 305.3,94.3 341.5,92.8 377.6,86.8 413.8,83.1 450.0,75.5 450.0,29.1 413.8,49.3 377.6,59.9 341.5,65.2 305.3,72.1 269.1,82.7 232.9,87.9 196.7,92.0 160.5,110.0 124.4,105.3 88.2,118.1 52.0,130.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,118.1 160.5,119.9 196.7,105.9 232.9,101.6 269.1,99.4 305.3,94.3 341.5,92.8 377.6,86.8 413.8,83.1 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.0 88.2,120.8 124.4,108.9 160.5,109.2 196.7,99.9 232.9,98.4 269.1,91.5 305.3,86.3 341.5,82.6 377.6,79.3 413.8,71.2 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.9 88.2,121.1 124.4,109.7 160.5,107.9 196.7,96.3 232.9,96.5 269.1,90.1 305.3,85.6 341.5,79.7 377.6,81.5 413.8,62.6 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.5 88.2,122.5 124.4,109.8 160.5,105.4 196.7,92.3 232.9,90.0 269.1,90.3 305.3,77.4 341.5,70.8 377.6,65.3 413.8,54.8 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.0 88.2,118.1 124.4,105.3 160.5,110.0 196.7,92.0 232.9,87.9 269.1,82.7 305.3,72.1 341.5,65.2 377.6,59.9 413.8,49.3 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 168 ns | 217 ns | 209 ns | 125 ns |
| D38 | 4.36 ns | 210 ns | 196 ns | 170 ns | 214 ns |
| D57 | 309 ns | 433 ns | 425 ns | 444 ns | 525 ns |
| D76 | 284 ns | 427 ns | 476 ns | 528 ns | 397 ns |
| D115 | 646 ns | 793 ns | 931 ns | 1.12 µs | 1.07 µs |
| D153 | 774 ns | 851 ns | 881 ns | 1.22 µs | 1.42 µs |
| D230 | 986 ns | 1.29 µs | 1.33 µs | 1.26 µs | 1.85 µs |
| D307 | 1.3 µs | 1.7 µs | 1.73 µs | 2.62 µs | 3.4 µs |
| D462 | 1.38 µs | 2 µs | 2.23 µs | 3.61 µs | 4.83 µs |
| D616 | 1.87 µs | 2.37 µs | 2.03 µs | 4.87 µs | 6.42 µs |
| D924 | 2.44 µs | 3.92 µs | 5.84 µs | 8.64 µs | 11.5 µs |
| D1232 | 3.48 µs | 4.33 µs | 8.67 µs | 12.6 µs | 34.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,110.4 160.5,111.8 196.7,97.6 232.9,94.4 269.1,90.2 305.3,85.4 341.5,84.4 377.6,79.1 413.8,74.5 450.0,68.4 450.0,28.3 413.8,47.6 377.6,57.7 341.5,62.7 305.3,68.8 269.1,79.3 232.9,83.9 196.7,88.9 160.5,106.0 124.4,101.2 88.2,116.8 52.0,126.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,110.4 160.5,111.8 196.7,97.6 232.9,94.4 269.1,90.2 305.3,85.4 341.5,84.4 377.6,79.1 413.8,74.5 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.0 88.2,117.1 124.4,104.6 160.5,104.8 196.7,94.0 232.9,92.8 269.1,85.6 305.3,80.8 341.5,77.9 377.6,75.0 413.8,66.3 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.5 88.2,118.3 124.4,104.9 160.5,102.9 196.7,91.2 232.9,92.2 269.1,85.0 305.3,80.4 341.5,76.1 377.6,77.7 413.8,59.3 450.0,52.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.2 88.2,120.8 124.4,104.1 160.5,101.1 196.7,88.0 232.9,86.6 269.1,86.0 305.3,73.3 341.5,67.7 377.6,62.5 413.8,52.5 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,126.1 88.2,116.8 124.4,101.2 160.5,106.0 196.7,88.9 232.9,83.9 269.1,79.3 305.3,68.8 341.5,62.7 377.6,57.7 413.8,47.6 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
