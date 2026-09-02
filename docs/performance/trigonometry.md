# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.36 ns | 11.4 µs | 27.7 µs | 14.1 µs | 36.8 µs |
| D38 | 3.6 µs | 7.37 µs | 7.55 µs | 11.5 µs | 12 µs |
| D57 | 5.14 µs | 8.94 µs | 9.67 µs | 16.8 µs | 20.5 µs |
| D76 | 5.15 µs | 6.87 µs | 10.6 µs | 15 µs | 20.9 µs |
| D115 | 5.13 µs | 11.8 µs | 26.3 µs | 29 µs | 34.2 µs |
| D153 | 4.77 µs | 10 µs | 28.2 µs | 42.8 µs | 61.2 µs |
| D230 | 5.52 µs | 24.8 µs | 45.7 µs | 75.7 µs | 82.8 µs |
| D307 | 4.76 µs | 28.5 µs | 60.4 µs | 124 µs | 180 µs |
| D462 | 5.22 µs | 42.4 µs | 132 µs | 241 µs | 403 µs |
| D616 | 4.93 µs | 66 µs | 220 µs | 360 µs | 708 µs |
| D924 | 5.36 µs | 126 µs | 428 µs | 881 µs | 1.56 ms |
| D1232 | 5.3 µs | 207 µs | 563 µs | 1.53 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,108.4 124.4,104.0 160.5,103.9 196.7,104.0 232.9,104.9 269.1,103.1 305.3,104.9 341.5,103.8 377.6,104.5 413.8,103.5 450.0,103.6 450.0,27.4 413.8,33.0 377.6,42.9 341.5,49.9 305.3,59.8 269.1,69.5 232.9,73.2 196.7,80.5 160.5,86.5 124.4,86.8 88.2,93.5 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,108.4 124.4,104.0 160.5,103.9 196.7,104.0 232.9,104.9 269.1,103.1 305.3,104.9 341.5,103.8 377.6,104.5 413.8,103.5 450.0,103.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,94.1 88.2,99.5 124.4,97.1 160.5,100.4 196.7,93.7 232.9,95.7 269.1,84.5 305.3,82.7 341.5,77.8 377.6,72.3 413.8,64.2 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.2 124.4,96.1 160.5,95.0 196.7,83.7 232.9,82.8 269.1,76.9 305.3,73.4 341.5,63.7 377.6,57.4 413.8,49.1 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,91.5 88.2,94.0 124.4,89.3 160.5,90.7 196.7,82.5 232.9,77.7 269.1,70.6 305.3,64.4 341.5,56.2 377.6,51.3 413.8,40.1 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,93.5 124.4,86.8 160.5,86.5 196.7,80.5 232.9,73.2 269.1,69.5 305.3,59.8 341.5,49.9 377.6,42.9 413.8,33.0 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 27.4 µs | 15.7 µs | 33.2 µs | 17.7 µs | 35.9 µs |
| D38 | 13.8 µs | 33.2 µs | 41.3 µs | 53.4 µs | 51.3 µs |
| D57 | 3.72 µs | 4.25 µs | 4.14 µs | 6.56 µs | 8.17 µs |
| D76 | 3.68 µs | 3.3 µs | 4.07 µs | 6.31 µs | 9.19 µs |
| D115 | 6.96 µs | 8.88 µs | 13.7 µs | 15.2 µs | 16.8 µs |
| D153 | 6.42 µs | 7.31 µs | 15.1 µs | 22.1 µs | 31.2 µs |
| D230 | 9.35 µs | 16.7 µs | 29.8 µs | 47.7 µs | 46.5 µs |
| D307 | 12.1 µs | 25.5 µs | 47 µs | 82.6 µs | 131 µs |
| D462 | 13 µs | 35.8 µs | 88 µs | 161 µs | 272 µs |
| D616 | 22 µs | 76.8 µs | 181 µs | 282 µs | 556 µs |
| D924 | 34.4 µs | 157 µs | 390 µs | 768 µs | 1.37 ms |
| D1232 | 43.1 µs | 267 µs | 564 µs | 1.47 ms | 2.17 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,138.1 88.2,152.9 124.4,181.5 160.5,181.7 196.7,167.9 232.9,169.6 269.1,161.5 305.3,155.8 341.5,154.3 377.6,142.8 413.8,133.2 450.0,128.3 450.0,43.1 413.8,53.1 377.6,72.7 341.5,88.3 305.3,104.2 269.1,126.6 232.9,135.3 196.7,148.7 160.5,161.8 124.4,164.4 88.2,124.5 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,138.1 88.2,152.9 124.4,181.5 160.5,181.7 196.7,167.9 232.9,169.6 269.1,161.5 305.3,155.8 341.5,154.3 377.6,142.8 413.8,133.2 450.0,128.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,150.2 88.2,134.0 124.4,178.6 160.5,184.1 196.7,162.6 232.9,166.8 269.1,148.8 305.3,139.7 341.5,132.3 377.6,115.7 413.8,100.3 450.0,88.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,129.2 124.4,179.1 160.5,179.5 196.7,153.1 232.9,151.0 269.1,136.3 305.3,126.4 341.5,112.8 377.6,97.1 413.8,80.4 450.0,72.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.6 88.2,123.6 124.4,169.1 160.5,170.0 196.7,150.9 232.9,142.7 269.1,126.1 305.3,114.2 341.5,99.7 377.6,87.5 413.8,65.7 450.0,51.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,124.5 124.4,164.4 160.5,161.8 196.7,148.7 232.9,135.3 269.1,126.6 305.3,104.2 341.5,88.3 377.6,72.7 413.8,53.1 450.0,43.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 11.3 µs | 27.7 µs | 14.1 µs | 36.8 µs |
| D38 | 3.49 µs | 7.32 µs | 7.54 µs | 11.5 µs | 11.9 µs |
| D57 | 5.04 µs | 8.89 µs | 9.66 µs | 16.8 µs | 20.3 µs |
| D76 | 5.03 µs | 6.95 µs | 10.5 µs | 14.9 µs | 20.9 µs |
| D115 | 5 µs | 11.8 µs | 25.9 µs | 29 µs | 33.8 µs |
| D153 | 4.64 µs | 9.99 µs | 28.7 µs | 42.5 µs | 61.3 µs |
| D230 | 5.09 µs | 25.1 µs | 45.2 µs | 76.7 µs | 81.1 µs |
| D307 | 4.68 µs | 28.7 µs | 60.8 µs | 123 µs | 180 µs |
| D462 | 5.15 µs | 42.4 µs | 134 µs | 240 µs | 403 µs |
| D616 | 4.76 µs | 66.1 µs | 219 µs | 357 µs | 711 µs |
| D924 | 5.34 µs | 126 µs | 426 µs | 887 µs | 1.57 ms |
| D1232 | 5.16 µs | 206 µs | 564 µs | 1.53 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,108.8 124.4,104.2 160.5,104.2 196.7,104.3 232.9,105.3 269.1,104.1 305.3,105.1 341.5,104.0 377.6,104.9 413.8,103.5 450.0,103.9 450.0,27.5 413.8,33.0 377.6,42.8 341.5,49.8 305.3,59.8 269.1,69.7 232.9,73.2 196.7,80.6 160.5,86.6 124.4,86.9 88.2,93.5 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,108.8 124.4,104.2 160.5,104.2 196.7,104.3 232.9,105.3 269.1,104.1 305.3,105.1 341.5,104.0 377.6,104.9 413.8,103.5 450.0,103.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,94.1 88.2,99.6 124.4,97.2 160.5,100.2 196.7,93.7 232.9,95.7 269.1,84.3 305.3,82.6 341.5,77.8 377.6,72.3 413.8,64.3 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.2 124.4,96.1 160.5,95.1 196.7,83.9 232.9,82.6 269.1,77.0 305.3,73.3 341.5,63.6 377.6,57.4 413.8,49.2 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,91.5 88.2,94.0 124.4,89.3 160.5,90.8 196.7,82.5 232.9,77.8 269.1,70.4 305.3,64.6 341.5,56.3 377.6,51.4 413.8,40.1 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,93.5 124.4,86.9 160.5,86.6 196.7,80.6 232.9,73.2 269.1,69.7 305.3,59.8 341.5,49.8 377.6,42.8 413.8,33.0 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 11.1 µs | 27.1 µs | 13.6 µs | 36.1 µs |
| D38 | 2.66 ns | 27.1 µs | 33.7 µs | 39.9 µs | 42.8 µs |
| D57 | 2.11 ns | 5.47 µs | 6.27 µs | 9.95 µs | 13.2 µs |
| D76 | 2.23 ns | 4.7 µs | 6.11 µs | 9.7 µs | 14.1 µs |
| D115 | 12.4 ns | 12.6 µs | 21.1 µs | 21.8 µs | 25.7 µs |
| D153 | 16.5 ns | 10.9 µs | 22.1 µs | 34.1 µs | 44.3 µs |
| D230 | 31.7 ns | 24.1 µs | 44.7 µs | 68 µs | 64 µs |
| D307 | 45.2 ns | 37.4 µs | 67.2 µs | 115 µs | 165 µs |
| D462 | 74.6 ns | 54.4 µs | 126 µs | 197 µs | 339 µs |
| D616 | 78.6 ns | 110 µs | 240 µs | 358 µs | 664 µs |
| D924 | 113 ns | 234 µs | 503 µs | 881 µs | 1.49 ms |
| D1232 | 151 ns | 368 µs | 710 µs | 1.63 ms | 2.05 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,197.8 124.4,200.7 160.5,200.0 196.7,178.8 232.9,175.2 269.1,167.1 305.3,162.7 341.5,156.5 377.6,155.9 413.8,151.4 450.0,147.7 450.0,29.7 413.8,33.6 377.6,43.6 341.5,52.0 305.3,60.9 269.1,72.7 232.9,77.2 196.7,84.0 160.5,91.4 124.4,92.2 88.2,77.7 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,197.8 124.4,200.7 160.5,200.0 196.7,178.8 232.9,175.2 269.1,167.1 305.3,162.7 341.5,156.5 377.6,155.9 413.8,151.4 450.0,147.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,94.4 88.2,83.3 124.4,103.2 160.5,105.1 196.7,92.8 232.9,94.7 269.1,84.8 305.3,79.3 341.5,74.7 377.6,66.0 413.8,56.6 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.3 88.2,80.6 124.4,101.5 160.5,101.8 196.7,86.5 232.9,85.9 269.1,77.1 305.3,72.1 341.5,64.3 377.6,56.3 413.8,47.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,91.9 88.2,78.5 124.4,95.8 160.5,96.1 196.7,86.0 232.9,80.5 269.1,71.9 305.3,65.4 341.5,58.7 377.6,51.3 413.8,40.1 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,77.7 124.4,92.2 160.5,91.4 196.7,84.0 232.9,77.2 269.1,72.7 305.3,60.9 341.5,52.0 377.6,43.6 413.8,33.6 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.47 µs | 2.57 µs | 2.23 µs | 3.05 µs |
| D38 | 3.97 µs | 8.01 µs | 6.26 µs | 9.96 µs | 10.5 µs |
| D57 | 4.17 µs | 7.63 µs | 8.39 µs | 14.8 µs | 5.42 µs |
| D76 | 4.17 µs | 5.99 µs | 9.23 µs | 13.2 µs | 18.7 µs |
| D115 | 4.15 µs | 10.3 µs | 23.4 µs | 26.7 µs | 31.4 µs |
| D153 | 3.82 µs | 8.72 µs | 22.2 µs | 38.7 µs | 56 µs |
| D230 | 4.24 µs | 21.9 µs | 41.2 µs | 69.6 µs | 75.2 µs |
| D307 | 3.9 µs | 25.7 µs | 49.7 µs | 115 µs | 168 µs |
| D462 | 3.05 µs | 34.2 µs | 115 µs | 211 µs | 345 µs |
| D616 | 4.04 µs | 60.5 µs | 203 µs | 336 µs | 671 µs |
| D924 | 4.35 µs | 116 µs | 400 µs | 837 µs | 1.47 ms |
| D1232 | 4.28 µs | 192 µs | 532 µs | 1.47 ms | 2.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,107.2 124.4,106.6 160.5,106.6 196.7,106.6 232.9,107.6 269.1,106.3 305.3,107.4 341.5,110.4 377.6,106.9 413.8,106.0 450.0,106.2 450.0,28.0 413.8,33.8 377.6,43.5 341.5,51.8 305.3,60.7 269.1,70.7 232.9,74.3 196.7,81.5 160.5,88.0 124.4,103.3 88.2,95.1 52.0,110.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,107.2 124.4,106.6 160.5,106.6 196.7,106.6 232.9,107.6 269.1,106.3 305.3,107.4 341.5,110.4 377.6,106.9 413.8,106.0 450.0,106.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.5 88.2,98.5 124.4,99.1 160.5,102.1 196.7,95.3 232.9,97.4 269.1,86.0 305.3,84.0 341.5,80.5 377.6,73.4 413.8,65.3 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,101.5 124.4,97.9 160.5,96.7 196.7,85.2 232.9,85.8 269.1,78.1 305.3,75.8 341.5,65.4 377.6,58.4 413.8,50.0 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.4 88.2,95.8 124.4,90.8 160.5,92.3 196.7,83.5 232.9,78.9 269.1,71.6 305.3,65.5 341.5,57.9 377.6,52.1 413.8,40.8 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,95.1 124.4,103.3 160.5,88.0 196.7,81.5 232.9,74.3 269.1,70.7 305.3,60.7 341.5,51.8 377.6,43.5 413.8,33.8 450.0,28.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 4.33 µs | 8.81 µs | 5.23 µs | 10.6 µs |
| D38 | 2.68 ns | 8.83 µs | 11 µs | 13 µs | 12.4 µs |
| D57 | 550 ns | 5.55 µs | 5.73 µs | 9.28 µs | 12 µs |
| D76 | 528 ns | 4.37 µs | 5.65 µs | 9.08 µs | 13.9 µs |
| D115 | 1.11 µs | 12.5 µs | 20.1 µs | 23.6 µs | 26.2 µs |
| D153 | 909 ns | 10.6 µs | 23.1 µs | 34.9 µs | 50.8 µs |
| D230 | 1.39 µs | 24.4 µs | 45.8 µs | 79.9 µs | 77.7 µs |
| D307 | 1.86 µs | 38.4 µs | 77.2 µs | 141 µs | 229 µs |
| D462 | 2.09 µs | 55.8 µs | 151 µs | 287 µs | 497 µs |
| D616 | 3.55 µs | 125 µs | 316 µs | 507 µs | 1.01 ms |
| D924 | 5.23 µs | 263 µs | 695 µs | 1.42 ms | 2.57 ms |
| D1232 | 6.79 µs | 461 µs | 1.02 ms | 2.74 ms | 4.1 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,197.8 124.4,131.7 160.5,132.2 196.7,123.0 232.9,125.5 269.1,120.2 305.3,116.6 341.5,115.2 377.6,108.6 413.8,103.8 450.0,100.5 450.0,21.1 413.8,26.9 377.6,38.4 341.5,47.2 305.3,56.8 269.1,70.3 232.9,75.6 196.7,83.8 160.5,91.6 124.4,93.5 88.2,93.0 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,197.8 124.4,131.7 160.5,132.2 196.7,123.0 232.9,125.5 269.1,120.2 305.3,116.6 341.5,115.2 377.6,108.6 413.8,103.8 450.0,100.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.1 88.2,97.3 124.4,103.0 160.5,106.0 196.7,92.9 232.9,94.9 269.1,84.7 305.3,79.0 341.5,74.4 377.6,64.4 413.8,55.1 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,94.5 124.4,102.6 160.5,102.8 196.7,87.1 232.9,85.3 269.1,76.8 305.3,70.3 341.5,62.1 377.6,52.9 413.8,43.1 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.8 88.2,92.5 124.4,96.6 160.5,96.9 196.7,85.1 232.9,80.2 269.1,69.9 305.3,62.9 341.5,54.1 377.6,47.0 413.8,34.2 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,93.0 124.4,93.5 160.5,91.6 196.7,83.8 232.9,75.6 269.1,70.3 305.3,56.8 341.5,47.2 377.6,38.4 413.8,26.9 450.0,21.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 2.5 µs | 5.5 µs | 3.12 µs | 6.44 µs |
| D38 | 3.16 ns | 5.54 µs | 6.8 µs | 7.99 µs | 7.54 µs |
| D57 | 2.81 ns | 3.5 µs | 3.95 µs | 5.97 µs | 9.61 µs |
| D76 | 3.17 ns | 2.84 µs | 3.62 µs | 5.69 µs | 9.21 µs |
| D115 | 13 ns | 4.66 µs | 9.98 µs | 12.5 µs | 14.2 µs |
| D153 | 16.8 ns | 3.46 µs | 9.87 µs | 19.9 µs | 31.1 µs |
| D230 | 40.8 ns | 9.59 µs | 19.4 µs | 39.7 µs | 44.6 µs |
| D307 | 64.4 ns | 12.8 µs | 24.6 µs | 68.7 µs | 111 µs |
| D462 | 125 ns | 15.3 µs | 65.8 µs | 134 µs | 235 µs |
| D616 | 119 ns | 32.9 µs | 130 µs | 216 µs | 458 µs |
| D924 | 172 ns | 70.8 µs | 258 µs | 584 µs | 1.07 ms |
| D1232 | 378 ns | 124 µs | 372 µs | 1.06 ms | 1.78 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,195.7 124.4,197.2 160.5,195.7 196.7,178.2 232.9,175.0 269.1,164.0 305.3,158.3 341.5,150.1 377.6,150.7 413.8,146.2 450.0,136.3 450.0,31.4 413.8,37.7 377.6,48.3 341.5,56.5 305.3,65.9 269.1,77.2 232.9,81.7 196.7,91.4 160.5,96.7 124.4,96.2 88.2,99.2 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,195.7 124.4,197.2 160.5,195.7 196.7,178.2 232.9,175.0 269.1,164.0 305.3,158.3 341.5,150.1 377.6,150.7 413.8,146.2 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.9 88.2,103.0 124.4,108.7 160.5,111.3 196.7,105.2 232.9,108.9 269.1,96.2 305.3,92.7 341.5,90.4 377.6,81.0 413.8,71.4 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.1 88.2,100.5 124.4,107.3 160.5,108.3 196.7,95.7 232.9,95.9 269.1,87.5 305.3,84.5 341.5,72.3 377.6,63.9 413.8,55.4 450.0,50.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,98.5 124.4,102.1 160.5,102.7 196.7,92.9 232.9,87.2 269.1,78.6 305.3,71.8 341.5,63.5 377.6,57.6 413.8,45.2 450.0,37.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,99.2 124.4,96.2 160.5,96.7 196.7,91.4 232.9,81.7 269.1,77.2 305.3,65.9 341.5,56.5 377.6,48.3 413.8,37.7 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 4.39 µs | 9.44 µs | 5.31 µs | 11.1 µs |
| D38 | 2.96 ns | 9.46 µs | 11.6 µs | 13.5 µs | 12.8 µs |
| D57 | 3.17 ns | 5.61 µs | 6.09 µs | 8.83 µs | 11.5 µs |
| D76 | 3.87 ns | 4.51 µs | 5.44 µs | 8.52 µs | 12.3 µs |
| D115 | 10.9 ns | 12.3 µs | 12.8 µs | 19 µs | 19.9 µs |
| D153 | 16.2 ns | 5.3 µs | 15.5 µs | 23.6 µs | 36.4 µs |
| D230 | 40.9 ns | 14.1 µs | 24.6 µs | 47.5 µs | 52 µs |
| D307 | 65.2 ns | 16.3 µs | 50.4 µs | 79.3 µs | 118 µs |
| D462 | 134 ns | 23.5 µs | 87 µs | 157 µs | 254 µs |
| D616 | 119 ns | 40.5 µs | 144 µs | 229 µs | 446 µs |
| D924 | 204 ns | 82.7 µs | 273 µs | 567 µs | 934 µs |
| D1232 | 370 ns | 135 µs | 363 µs | 934 µs | 2.16 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,196.5 124.4,195.7 160.5,193.2 196.7,180.4 232.9,175.4 269.1,164.0 305.3,158.2 341.5,149.2 377.6,150.7 413.8,144.0 450.0,136.6 450.0,29.0 413.8,39.4 377.6,48.6 341.5,55.6 305.3,65.1 269.1,75.3 232.9,79.7 196.7,87.2 160.5,93.2 124.4,94.0 88.2,92.6 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,196.5 124.4,195.7 160.5,193.2 196.7,180.4 232.9,175.4 269.1,164.0 305.3,158.2 341.5,149.2 377.6,150.7 413.8,144.0 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,96.4 124.4,102.9 160.5,105.6 196.7,93.2 232.9,103.6 269.1,91.5 305.3,89.7 341.5,85.1 377.6,78.3 413.8,69.5 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,93.9 124.4,101.9 160.5,103.3 196.7,92.7 232.9,90.3 269.1,84.5 305.3,75.6 341.5,68.9 377.6,62.7 413.8,54.7 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,92.0 124.4,97.3 160.5,97.7 196.7,87.8 232.9,85.1 269.1,76.4 305.3,70.0 341.5,61.5 377.6,56.8 413.8,45.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.6 124.4,94.0 160.5,93.2 196.7,87.2 232.9,79.7 269.1,75.3 305.3,65.1 341.5,55.6 377.6,48.6 413.8,39.4 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 2.38 µs | 5.31 µs | 3.1 µs | 6.37 µs |
| D38 | 3.17 ns | 5.36 µs | 6.62 µs | 7.81 µs | 7.49 µs |
| D57 | 2.81 ns | 3.25 µs | 3.78 µs | 5.9 µs | 9.6 µs |
| D76 | 3.52 ns | 2.64 µs | 3.44 µs | 5.62 µs | 8.85 µs |
| D115 | 13 ns | 4.35 µs | 10.4 µs | 12.6 µs | 14.7 µs |
| D153 | 16.9 ns | 3.34 µs | 10 µs | 18.3 µs | 30.4 µs |
| D230 | 40.7 ns | 9.61 µs | 19.9 µs | 39.2 µs | 42.9 µs |
| D307 | 60.4 ns | 12.9 µs | 24.2 µs | 66.4 µs | 107 µs |
| D462 | 127 ns | 15.2 µs | 63.1 µs | 134 µs | 229 µs |
| D616 | 114 ns | 32.4 µs | 127 µs | 214 µs | 457 µs |
| D924 | 191 ns | 68.2 µs | 256 µs | 579 µs | 1.06 ms |
| D1232 | 361 ns | 123 µs | 367 µs | 1.06 ms | 1.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,195.7 124.4,197.2 160.5,194.4 196.7,178.1 232.9,174.9 269.1,164.0 305.3,159.1 341.5,149.9 377.6,151.3 413.8,144.8 450.0,136.9 450.0,31.5 413.8,37.8 377.6,48.3 341.5,56.9 305.3,66.3 269.1,77.6 232.9,81.9 196.7,91.0 160.5,97.2 124.4,96.2 88.2,99.3 52.0,101.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,195.7 124.4,197.2 160.5,194.4 196.7,178.1 232.9,174.9 269.1,164.0 305.3,159.1 341.5,149.9 377.6,151.3 413.8,144.8 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,113.5 88.2,103.5 124.4,109.7 160.5,112.3 196.7,106.0 232.9,109.3 269.1,96.2 305.3,92.6 341.5,90.5 377.6,81.1 413.8,71.9 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,100.8 124.4,107.8 160.5,109.0 196.7,95.2 232.9,95.7 269.1,87.2 305.3,84.7 341.5,72.9 377.6,64.2 413.8,55.5 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,98.8 124.4,102.3 160.5,102.9 196.7,92.8 232.9,88.2 269.1,78.8 305.3,72.2 341.5,63.5 377.6,57.7 413.8,45.4 450.0,37.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.3 88.2,99.3 124.4,96.2 160.5,97.2 196.7,91.0 232.9,81.9 269.1,77.6 305.3,66.3 341.5,56.9 377.6,48.3 413.8,37.8 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 4.42 µs | 9.44 µs | 5.3 µs | 11.1 µs |
| D38 | 2.84 ns | 9.47 µs | 11.6 µs | 13.5 µs | 12.8 µs |
| D57 | 12.3 ns | 5.58 µs | 6.1 µs | 8.85 µs | 11.4 µs |
| D76 | 12 ns | 4.53 µs | 5.47 µs | 8.54 µs | 12.3 µs |
| D115 | 11.3 ns | 12.2 µs | 12.8 µs | 19.5 µs | 20.3 µs |
| D153 | 17.1 ns | 5.34 µs | 15.9 µs | 23.6 µs | 36.4 µs |
| D230 | 39.9 ns | 14.2 µs | 26.2 µs | 48 µs | 51.4 µs |
| D307 | 60.1 ns | 16.4 µs | 49.4 µs | 79.7 µs | 118 µs |
| D462 | 124 ns | 23.5 µs | 87 µs | 157 µs | 251 µs |
| D616 | 112 ns | 40.1 µs | 143 µs | 229 µs | 447 µs |
| D924 | 210 ns | 83 µs | 274 µs | 567 µs | 933 µs |
| D1232 | 362 ns | 136 µs | 362 µs | 933 µs | 2.16 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,197.0 124.4,178.9 160.5,179.2 196.7,180.0 232.9,174.8 269.1,164.3 305.3,159.2 341.5,150.2 377.6,151.4 413.8,143.6 450.0,136.9 450.0,29.0 413.8,39.4 377.6,48.6 341.5,55.7 305.3,65.1 269.1,75.4 232.9,79.7 196.7,86.9 160.5,93.1 124.4,94.0 88.2,92.6 52.0,94.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,197.0 124.4,178.9 160.5,179.2 196.7,180.0 232.9,174.8 269.1,164.3 305.3,159.2 341.5,150.2 377.6,151.4 413.8,143.6 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.8 88.2,96.4 124.4,103.0 160.5,105.5 196.7,93.3 232.9,103.5 269.1,91.3 305.3,89.6 341.5,85.1 377.6,78.5 413.8,69.5 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,93.9 124.4,101.8 160.5,103.2 196.7,92.6 232.9,89.9 269.1,83.8 305.3,75.9 341.5,68.9 377.6,62.7 413.8,54.6 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.6 88.2,92.0 124.4,97.2 160.5,97.7 196.7,87.4 232.9,85.1 269.1,76.2 305.3,70.0 341.5,61.5 377.6,56.8 413.8,45.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.6 124.4,94.0 160.5,93.1 196.7,86.9 232.9,79.7 269.1,75.4 305.3,65.1 341.5,55.7 377.6,48.6 413.8,39.4 450.0,29.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 4.76 µs | 9.93 µs | 5.81 µs | 11.7 µs |
| D38 | 3.33 ns | 9.97 µs | 12.3 µs | 14.5 µs | 13.8 µs |
| D57 | 2.81 ns | 4.35 µs | 4.88 µs | 7.77 µs | 9.89 µs |
| D76 | 3.87 ns | 3.5 µs | 4.65 µs | 7.22 µs | 11 µs |
| D115 | 9.22 ns | 5.77 µs | 13.5 µs | 14.8 µs | 17 µs |
| D153 | 16.5 ns | 4.39 µs | 12.7 µs | 22.2 µs | 34.5 µs |
| D230 | 41.3 ns | 12.4 µs | 23.4 µs | 45.2 µs | 49.5 µs |
| D307 | 58.8 ns | 15 µs | 29 µs | 75.4 µs | 120 µs |
| D462 | 113 ns | 18.7 µs | 73.3 µs | 148 µs | 249 µs |
| D616 | 108 ns | 37.5 µs | 141 µs | 239 µs | 491 µs |
| D924 | 159 ns | 77.9 µs | 284 µs | 631 µs | 1.13 ms |
| D1232 | 369 ns | 134 µs | 397 µs | 1.13 ms | 1.86 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,195.1 124.4,197.2 160.5,193.2 196.7,182.4 232.9,175.2 269.1,163.8 305.3,159.5 341.5,151.3 377.6,151.8 413.8,147.1 450.0,136.7 450.0,30.9 413.8,37.0 377.6,47.4 341.5,55.8 305.3,64.9 269.1,75.9 232.9,80.3 196.7,89.1 160.5,94.6 124.4,95.9 88.2,91.7 52.0,93.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,195.1 124.4,197.2 160.5,193.2 196.7,182.4 232.9,175.2 269.1,163.8 305.3,159.5 341.5,151.3 377.6,151.8 413.8,147.1 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.9 88.2,95.7 124.4,106.0 160.5,108.7 196.7,102.5 232.9,105.9 269.1,93.1 305.3,90.7 341.5,88.0 377.6,79.3 413.8,70.2 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,93.2 124.4,104.6 160.5,105.2 196.7,92.0 232.9,92.8 269.1,85.2 305.3,82.5 341.5,71.0 377.6,62.9 413.8,54.2 450.0,50.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.4 88.2,91.1 124.4,98.8 160.5,99.8 196.7,90.9 232.9,85.8 269.1,77.0 305.3,70.7 341.5,62.3 377.6,56.3 413.8,44.3 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.7 88.2,91.7 124.4,95.9 160.5,94.6 196.7,89.1 232.9,80.3 269.1,75.9 305.3,64.9 341.5,55.8 377.6,47.4 413.8,37.0 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 4.44 µs | 9.58 µs | 5.43 µs | 11.2 µs |
| D38 | 2.97 ns | 9.61 µs | 11.6 µs | 13.5 µs | 12.8 µs |
| D57 | 2.84 µs | 5.74 µs | 6.34 µs | 9.22 µs | 11.9 µs |
| D76 | 2.86 µs | 4.62 µs | 5.69 µs | 8.82 µs | 12.8 µs |
| D115 | 5.79 µs | 12.9 µs | 13.5 µs | 19.6 µs | 20.7 µs |
| D153 | 2.77 µs | 5.51 µs | 16.4 µs | 24.1 µs | 37.4 µs |
| D230 | 3.11 µs | 14.9 µs | 25.4 µs | 48.5 µs | 52.9 µs |
| D307 | 3.01 µs | 17.1 µs | 51.7 µs | 81 µs | 119 µs |
| D462 | 3.35 µs | 24.1 µs | 88.1 µs | 160 µs | 255 µs |
| D616 | 3.22 µs | 41.5 µs | 146 µs | 231 µs | 451 µs |
| D924 | 3.85 µs | 84.4 µs | 277 µs | 572 µs | 943 µs |
| D1232 | 4.16 µs | 137 µs | 367 µs | 941 µs | 2.17 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,196.5 124.4,111.3 160.5,111.2 196.7,102.5 232.9,111.6 269.1,110.2 305.3,110.6 341.5,109.3 377.6,109.8 413.8,107.6 450.0,106.6 450.0,28.9 413.8,39.3 377.6,48.5 341.5,55.5 305.3,65.0 269.1,75.0 232.9,79.3 196.7,86.7 160.5,92.7 124.4,93.6 88.2,92.6 52.0,94.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,196.5 124.4,111.3 160.5,111.2 196.7,102.5 232.9,111.6 269.1,110.2 305.3,110.6 341.5,109.3 377.6,109.8 413.8,107.6 450.0,106.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.8 88.2,96.2 124.4,102.6 160.5,105.3 196.7,92.6 232.9,103.1 269.1,90.8 305.3,89.0 341.5,84.8 377.6,78.1 413.8,69.2 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.2 88.2,93.9 124.4,101.4 160.5,102.7 196.7,92.0 232.9,89.6 269.1,84.1 305.3,75.3 341.5,68.7 377.6,62.5 413.8,54.5 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.3 88.2,92.0 124.4,96.7 160.5,97.3 196.7,87.4 232.9,84.8 269.1,76.1 305.3,69.8 341.5,61.3 377.6,56.7 413.8,45.5 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,92.6 124.4,93.6 160.5,92.7 196.7,86.7 232.9,79.3 269.1,75.0 305.3,65.0 341.5,55.5 377.6,48.5 413.8,39.3 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 1.47 µs | 3.23 µs | 1.53 µs | 3.86 µs |
| D38 | 3.69 ns | 3.24 µs | 4.01 µs | 4.73 µs | 4.54 µs |
| D57 | 198 ns | 304 ns | 273 ns | 352 ns | 451 ns |
| D76 | 198 ns | 202 ns | 220 ns | 328 ns | 434 ns |
| D115 | 460 ns | 578 ns | 771 ns | 710 ns | 619 ns |
| D153 | 412 ns | 424 ns | 765 ns | 880 ns | 1.02 µs |
| D230 | 579 ns | 822 ns | 1.12 µs | 1.45 µs | 1.18 µs |
| D307 | 795 ns | 1.22 µs | 1.54 µs | 2.08 µs | 2.59 µs |
| D462 | 955 ns | 1.4 µs | 2.31 µs | 3.02 µs | 4.17 µs |
| D616 | 1.11 µs | 2.06 µs | 3.07 µs | 3.5 µs | 6.12 µs |
| D924 | 1.61 µs | 2.82 µs | 4.54 µs | 7.12 µs | 10.6 µs |
| D1232 | 2.23 µs | 4.16 µs | 5.87 µs | 11.3 µs | 23.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,187.3 124.4,118.1 160.5,118.1 196.7,103.5 232.9,105.4 269.1,99.5 305.3,94.0 341.5,90.8 377.6,88.2 413.8,81.7 450.0,76.1 450.0,35.1 413.8,49.0 377.6,58.5 341.5,65.2 305.3,73.5 269.1,87.1 232.9,89.6 196.7,98.3 160.5,104.5 124.4,103.8 88.2,63.7 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,187.3 124.4,118.1 160.5,118.1 196.7,103.5 232.9,105.4 269.1,99.5 305.3,94.0 341.5,90.8 377.6,88.2 413.8,81.7 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,83.3 88.2,69.6 124.4,110.7 160.5,117.7 196.7,99.5 232.9,104.9 269.1,93.4 305.3,86.6 341.5,84.1 377.6,77.4 413.8,72.0 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,65.9 124.4,112.6 160.5,116.3 196.7,94.5 232.9,94.7 269.1,88.1 305.3,82.5 341.5,75.5 377.6,70.5 413.8,63.7 450.0,59.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,82.6 88.2,63.0 124.4,108.1 160.5,109.3 196.7,95.9 232.9,92.2 269.1,83.6 305.3,77.3 341.5,70.8 377.6,68.2 413.8,55.9 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,63.7 124.4,103.8 160.5,104.5 196.7,98.3 232.9,89.6 269.1,87.1 305.3,73.5 341.5,65.2 377.6,58.5 413.8,49.0 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 140 ns | 193 ns | 145 ns | 193 ns |
| D38 | 2.8 ns | 191 ns | 193 ns | 195 ns | 163 ns |
| D57 | 310 ns | 405 ns | 350 ns | 463 ns | 558 ns |
| D76 | 306 ns | 275 ns | 294 ns | 418 ns | 553 ns |
| D115 | 722 ns | 808 ns | 1.01 µs | 926 ns | 734 ns |
| D153 | 644 ns | 527 ns | 992 ns | 1.08 µs | 1.29 µs |
| D230 | 1 µs | 1.18 µs | 1.46 µs | 1.85 µs | 1.45 µs |
| D307 | 1.33 µs | 1.72 µs | 2.04 µs | 2.62 µs | 3.18 µs |
| D462 | 1.55 µs | 1.83 µs | 2.93 µs | 3.66 µs | 4.89 µs |
| D616 | 1.79 µs | 2.6 µs | 3.84 µs | 4.2 µs | 6.98 µs |
| D924 | 2.54 µs | 3.67 µs | 5.46 µs | 8 µs | 11.6 µs |
| D1232 | 3.38 µs | 5.27 µs | 6.85 µs | 12.8 µs | 24.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,192.1 124.4,110.4 160.5,110.6 196.7,95.7 232.9,97.6 269.1,90.0 305.3,85.1 341.5,82.4 377.6,79.9 413.8,73.8 450.0,68.8 450.0,34.3 413.8,47.4 377.6,56.2 341.5,62.4 305.3,69.9 269.1,83.6 232.9,85.6 196.7,95.4 160.5,100.3 124.4,100.1 88.2,121.5 52.0,118.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,192.1 124.4,110.4 160.5,110.6 196.7,95.7 232.9,97.6 269.1,90.0 305.3,85.1 341.5,82.4 377.6,79.9 413.8,73.8 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.2 88.2,118.8 124.4,105.7 160.5,112.4 196.7,93.7 232.9,101.1 269.1,87.1 305.3,80.6 341.5,79.5 377.6,73.4 413.8,67.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.6 88.2,118.6 124.4,108.2 160.5,111.3 196.7,89.8 232.9,90.1 269.1,83.4 305.3,77.6 341.5,71.3 377.6,66.6 413.8,60.5 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.6 88.2,118.4 124.4,103.4 160.5,105.1 196.7,91.3 232.9,88.6 269.1,79.3 305.3,73.3 341.5,67.5 377.6,65.1 413.8,53.9 450.0,45.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.5 88.2,121.5 124.4,100.1 160.5,100.3 196.7,95.4 232.9,85.6 269.1,83.6 305.3,69.9 341.5,62.4 377.6,56.2 413.8,47.4 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
