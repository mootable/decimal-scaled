# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.33 ns | 2 µs | 3.47 µs | 3.35 µs | 4.12 µs |
| D38 | 986 ns | 5.17 µs | 4.05 µs | 8.05 µs | 10.3 µs |
| D57 | 1.25 µs | 6.29 µs | 8 µs | 10.3 µs | 14.9 µs |
| D76 | 993 ns | 7.07 µs | 10 µs | 15.3 µs | 11.1 µs |
| D115 | 1.03 µs | 8.5 µs | 12.9 µs | 35.3 µs | 45.9 µs |
| D153 | 1.54 µs | 7.64 µs | 25.2 µs | 36.8 µs | 61 µs |
| D230 | 1.47 µs | 20.1 µs | 45 µs | 62.6 µs | 106 µs |
| D307 | 1.56 µs | 25.5 µs | 43.1 µs | 135 µs | 192 µs |
| D462 | 1.52 µs | 45.1 µs | 132 µs | 254 µs | 407 µs |
| D616 | 1.57 µs | 65.9 µs | 207 µs | 447 µs | 660 µs |
| D924 | 1e+03 ns | 95.1 µs | 446 µs | 874 µs | 1.67 ms |
| D1232 | 2.03 µs | 207 µs | 714 µs | 1.3 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.8 88.2,124.5 124.4,121.5 160.5,124.4 196.7,123.9 232.9,119.0 269.1,119.5 305.3,118.8 341.5,119.1 377.6,118.7 413.8,124.3 450.0,115.5 450.0,24.4 413.8,32.2 377.6,43.7 341.5,49.7 305.3,59.0 269.1,66.4 232.9,73.3 196.7,76.8 160.5,94.4 124.4,90.7 88.2,95.3 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.8 88.2,124.5 124.4,121.5 160.5,124.4 196.7,123.9 232.9,119.0 269.1,119.5 305.3,118.8 341.5,119.1 377.6,118.7 413.8,124.3 450.0,115.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,103.9 124.4,101.5 160.5,100.0 196.7,97.7 232.9,99.1 269.1,87.1 305.3,84.1 341.5,77.0 377.6,72.3 413.8,67.8 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.9 124.4,98.5 160.5,95.7 196.7,92.6 232.9,84.2 269.1,77.1 305.3,77.6 341.5,63.7 377.6,58.1 413.8,48.6 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,98.4 124.4,95.4 160.5,90.4 196.7,80.1 232.9,79.5 269.1,73.0 305.3,63.4 341.5,55.6 377.6,48.6 413.8,40.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,95.3 124.4,90.7 160.5,94.4 196.7,76.8 232.9,73.3 269.1,66.4 305.3,59.0 341.5,49.7 377.6,43.7 413.8,32.2 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.2 µs | 3.57 µs | 6.05 µs | 6.18 µs | 7.12 µs |
| D38 | 2.79 µs | 6.06 µs | 7.1 µs | 8.36 µs | 10.1 µs |
| D57 | 2.88 µs | 4.71 µs | 4.91 µs | 6.12 µs | 8.14 µs |
| D76 | 2.21 µs | 5.03 µs | 5.87 µs | 8.39 µs | 5.6 µs |
| D115 | 4.65 µs | 8.8 µs | 7.8 µs | 18.4 µs | 23.7 µs |
| D153 | 6.1 µs | 8.75 µs | 14.3 µs | 19.3 µs | 30.4 µs |
| D230 | 8.38 µs | 16.8 µs | 28.5 µs | 36.9 µs | 59.6 µs |
| D307 | 13.1 µs | 26.5 µs | 31 µs | 88.9 µs | 139 µs |
| D462 | 12.5 µs | 38.5 µs | 89.2 µs | 167 µs | 277 µs |
| D616 | 21.7 µs | 75.8 µs | 171 µs | 330 µs | 519 µs |
| D924 | 20.3 µs | 108 µs | 404 µs | 766 µs | 1.45 ms |
| D1232 | 43.3 µs | 259 µs | 734 µs | 1.21 ms | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.8 88.2,187.7 124.4,187.1 160.5,192.7 196.7,176.6 232.9,170.7 269.1,163.8 305.3,154.1 341.5,155.2 377.6,143.1 413.8,144.6 450.0,128.2 450.0,37.3 413.8,52.0 377.6,74.2 341.5,87.9 305.3,102.9 269.1,121.2 232.9,135.8 196.7,141.3 160.5,172.6 124.4,164.5 88.2,159.7 52.0,167.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.8 88.2,187.7 124.4,187.1 160.5,192.7 196.7,176.6 232.9,170.7 269.1,163.8 305.3,154.1 341.5,155.2 377.6,143.1 413.8,144.6 450.0,128.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.3 88.2,170.9 124.4,176.3 160.5,174.9 196.7,162.8 232.9,162.9 269.1,148.8 305.3,138.8 341.5,130.7 377.6,116.0 413.8,108.2 450.0,89.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,167.4 124.4,175.4 160.5,171.6 196.7,165.4 232.9,152.2 269.1,137.3 305.3,135.4 341.5,112.5 377.6,98.3 413.8,79.7 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.4 88.2,163.9 124.4,170.7 160.5,163.8 196.7,146.7 232.9,145.7 269.1,131.7 305.3,112.6 341.5,98.8 377.6,84.1 413.8,65.8 450.0,55.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.4 88.2,159.7 124.4,164.5 160.5,172.6 196.7,141.3 232.9,135.8 269.1,121.2 305.3,102.9 341.5,87.9 377.6,74.2 413.8,52.0 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.68 ns | 1.99 µs | 3.48 µs | 3.34 µs | 4.1 µs |
| D38 | 901 ns | 5.14 µs | 4 µs | 7.99 µs | 10.3 µs |
| D57 | 1.19 µs | 6.26 µs | 7.98 µs | 10.3 µs | 14.9 µs |
| D76 | 925 ns | 7.07 µs | 9.98 µs | 15.3 µs | 11.1 µs |
| D115 | 931 ns | 8.51 µs | 13.4 µs | 34.8 µs | 45.7 µs |
| D153 | 1.36 µs | 7.71 µs | 25.1 µs | 37.3 µs | 61.7 µs |
| D230 | 1.37 µs | 20.4 µs | 46.7 µs | 62.8 µs | 106 µs |
| D307 | 1.46 µs | 25.9 µs | 40.4 µs | 133 µs | 193 µs |
| D462 | 1.45 µs | 45.8 µs | 132 µs | 252 µs | 405 µs |
| D616 | 1.42 µs | 66.1 µs | 207 µs | 445 µs | 660 µs |
| D924 | 932 ns | 95.6 µs | 448 µs | 879 µs | 1.67 ms |
| D1232 | 1.87 µs | 208 µs | 716 µs | 1.29 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.8 88.2,125.6 124.4,122.1 160.5,125.3 196.7,125.2 232.9,120.5 269.1,120.3 305.3,119.6 341.5,119.7 377.6,119.9 413.8,125.2 450.0,116.5 450.0,24.4 413.8,32.2 377.6,43.7 341.5,49.8 305.3,59.0 269.1,66.4 232.9,73.1 196.7,76.9 160.5,94.4 124.4,90.8 88.2,95.4 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.8 88.2,125.6 124.4,122.1 160.5,125.3 196.7,125.2 232.9,120.5 269.1,120.3 305.3,119.6 341.5,119.7 377.6,119.9 413.8,125.2 450.0,116.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,104.0 124.4,101.5 160.5,100.0 196.7,97.7 232.9,98.9 269.1,86.9 305.3,83.9 341.5,76.8 377.6,72.3 413.8,67.7 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,107.1 124.4,98.5 160.5,95.7 196.7,92.1 232.9,84.3 269.1,76.6 305.3,78.4 341.5,63.7 377.6,58.1 413.8,48.5 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,98.5 124.4,95.4 160.5,90.5 196.7,80.3 232.9,79.4 269.1,72.9 305.3,63.6 341.5,55.7 377.6,48.6 413.8,40.2 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,95.4 124.4,90.8 160.5,94.4 196.7,76.9 232.9,73.1 269.1,66.4 305.3,59.0 341.5,49.8 377.6,43.7 413.8,32.2 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.09 µs | 1.72 µs | 2.65 µs | 2.74 µs | 3.11 µs |
| D38 | 1.38 µs | 2.65 µs | 3.09 µs | 3.7 µs | 4.51 µs |
| D57 | 3.8 µs | 5.83 µs | 7.17 µs | 8.99 µs | 12.6 µs |
| D76 | 2.98 µs | 6.75 µs | 8.71 µs | 12.7 µs | 8.41 µs |
| D115 | 6.51 µs | 12.6 µs | 12.4 µs | 26 µs | 36.2 µs |
| D153 | 8.46 µs | 13.4 µs | 21.7 µs | 28.8 µs | 43.7 µs |
| D230 | 11.1 µs | 25.9 µs | 44.8 µs | 52.7 µs | 83.3 µs |
| D307 | 17.7 µs | 37.5 µs | 41.8 µs | 124 µs | 176 µs |
| D462 | 16.4 µs | 59 µs | 128 µs | 209 µs | 337 µs |
| D616 | 28.6 µs | 110 µs | 224 µs | 423 µs | 611 µs |
| D924 | 27.2 µs | 164 µs | 533 µs | 874 µs | 1.6 ms |
| D1232 | 58.3 µs | 365 µs | 943 µs | 1.39 ms | 2.61 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.0 88.2,203.0 124.4,181.0 160.5,186.3 196.7,169.3 232.9,163.6 269.1,157.7 305.3,147.5 341.5,149.3 377.6,137.2 413.8,138.3 450.0,121.7 450.0,39.2 413.8,49.8 377.6,70.7 341.5,83.6 305.3,97.7 269.1,114.0 232.9,128.0 196.7,132.1 160.5,163.8 124.4,155.1 88.2,177.3 52.0,185.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.0 88.2,203.0 124.4,181.0 160.5,186.3 196.7,169.3 232.9,163.6 269.1,157.7 305.3,147.5 341.5,149.3 377.6,137.2 413.8,138.3 450.0,121.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.3 88.2,188.8 124.4,171.7 160.5,168.5 196.7,155.0 232.9,153.6 269.1,139.4 305.3,131.3 341.5,121.5 377.6,107.9 413.8,99.3 450.0,81.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.8 88.2,185.5 124.4,167.2 160.5,163.0 196.7,155.4 232.9,143.2 269.1,127.4 305.3,128.9 341.5,104.7 377.6,92.5 413.8,73.7 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.1 88.2,181.6 124.4,162.3 160.5,154.8 196.7,139.2 232.9,137.0 269.1,123.9 305.3,105.4 341.5,94.0 377.6,78.7 413.8,62.9 450.0,52.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.3 88.2,177.3 124.4,155.1 160.5,163.8 196.7,132.1 232.9,128.0 269.1,114.0 305.3,97.7 341.5,83.6 377.6,70.7 413.8,49.8 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.6 µs | 1.57 µs | 2.92 µs | 2.8 µs | 3.5 µs |
| D38 | 1.24 µs | 2.91 µs | 3.11 µs | 3.52 µs | 4.23 µs |
| D57 | 1.47 µs | 3.08 µs | 3.41 µs | 3.98 µs | 5.49 µs |
| D76 | 3.91 µs | 5.54 µs | 7.85 µs | 12.6 µs | 9.22 µs |
| D115 | 4.15 µs | 6.55 µs | 11.2 µs | 31.5 µs | 41.3 µs |
| D153 | 5.91 µs | 5.71 µs | 16.3 µs | 33.4 µs | 55.2 µs |
| D230 | 5.6 µs | 17.1 µs | 40.9 µs | 56.5 µs | 96.4 µs |
| D307 | 6.01 µs | 23.3 µs | 33.4 µs | 124 µs | 179 µs |
| D462 | 2.83 µs | 36.5 µs | 114 µs | 221 µs | 341 µs |
| D616 | 5.71 µs | 59.9 µs | 193 µs | 420 µs | 623 µs |
| D924 | 3.67 µs | 88.1 µs | 422 µs | 830 µs | 1.58 ms |
| D1232 | 6.06 µs | 191 µs | 679 µs | 1.24 ms | 3.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.8 88.2,205.4 124.4,201.6 160.5,180.4 196.7,179.1 232.9,171.4 269.1,172.6 305.3,171.1 341.5,187.4 377.6,172.2 413.8,181.8 450.0,170.9 450.0,36.1 413.8,50.1 377.6,70.3 341.5,83.4 305.3,97.3 269.1,110.8 232.9,122.9 196.7,129.2 160.5,161.8 124.4,173.0 88.2,178.7 52.0,182.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.8 88.2,205.4 124.4,201.6 160.5,180.4 196.7,179.1 232.9,171.4 269.1,172.6 305.3,171.1 341.5,187.4 377.6,172.2 413.8,181.8 450.0,170.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,200.2 88.2,186.8 124.4,185.6 160.5,172.8 196.7,169.2 232.9,172.2 269.1,148.3 305.3,141.6 341.5,131.9 377.6,121.1 413.8,112.8 450.0,95.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.8 88.2,185.4 124.4,183.3 160.5,165.3 196.7,157.6 232.9,149.4 269.1,129.4 305.3,133.8 341.5,107.2 377.6,95.8 413.8,78.7 450.0,68.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,182.7 124.4,180.0 160.5,155.0 196.7,135.1 232.9,133.8 269.1,122.4 305.3,105.4 341.5,92.8 377.6,78.8 413.8,64.0 450.0,55.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.8 88.2,178.7 124.4,173.0 160.5,161.8 196.7,129.2 232.9,122.9 269.1,110.8 305.3,97.3 341.5,83.4 377.6,70.3 413.8,50.1 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.7 ns | 1.73 µs | 2.98 µs | 3.06 µs | 3.58 µs |
| D38 | 2.44 ns | 2.98 µs | 3.57 µs | 4.16 µs | 5.1 µs |
| D57 | 402 ns | 6.16 µs | 6.98 µs | 8.66 µs | 12.1 µs |
| D76 | 285 ns | 6.63 µs | 8.3 µs | 12.1 µs | 8.51 µs |
| D115 | 735 ns | 12.2 µs | 11.4 µs | 27.9 µs | 36.9 µs |
| D153 | 921 ns | 12.1 µs | 21.8 µs | 30.1 µs | 49.6 µs |
| D230 | 1.32 µs | 24.4 µs | 43.4 µs | 62.9 µs | 103 µs |
| D307 | 2.06 µs | 40.4 µs | 50.1 µs | 152 µs | 245 µs |
| D462 | 2.03 µs | 59.4 µs | 152 µs | 297 µs | 499 µs |
| D616 | 3.62 µs | 121 µs | 301 µs | 591 µs | 956 µs |
| D924 | 3.08 µs | 177 µs | 725 µs | 1.41 ms | 2.68 ms |
| D1232 | 6.94 µs | 442 µs | 1.31 ms | 2.25 ms | 5.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.7 88.2,199.0 124.4,135.6 160.5,139.8 196.7,128.1 232.9,125.3 269.1,120.8 305.3,115.3 341.5,115.5 377.6,108.3 413.8,110.3 450.0,100.2 450.0,17.8 413.8,26.3 377.6,39.1 341.5,47.2 305.3,56.0 269.1,66.8 232.9,75.8 196.7,79.5 160.5,97.7 124.4,93.3 88.2,104.1 52.0,108.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.7 88.2,199.0 124.4,135.6 160.5,139.8 196.7,128.1 232.9,125.3 269.1,120.8 305.3,115.3 341.5,115.5 377.6,108.3 413.8,110.3 450.0,100.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,110.7 124.4,101.7 160.5,100.8 196.7,93.3 232.9,93.4 269.1,84.6 305.3,78.4 341.5,73.6 377.6,64.8 413.8,60.1 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,108.5 124.4,100.2 160.5,98.0 196.7,94.0 232.9,86.0 269.1,77.5 305.3,75.7 341.5,61.9 377.6,53.5 413.8,42.6 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,106.6 124.4,97.5 160.5,93.3 196.7,83.0 232.9,82.0 269.1,72.9 305.3,62.0 341.5,53.6 377.6,45.1 413.8,34.3 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,104.1 124.4,93.3 160.5,97.7 196.7,79.5 232.9,75.8 269.1,66.8 305.3,56.0 341.5,47.2 377.6,39.1 413.8,26.3 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 947 ns | 1.19 µs | 2.69 µs | 2.18 µs | 3.07 µs |
| D38 | 1.19 µs | 2.7 µs | 3 µs | 3.47 µs | 4.13 µs |
| D57 | 2.66 µs | 3.75 µs | 4.6 µs | 5.44 µs | 9.56 µs |
| D76 | 2.04 µs | 4.03 µs | 5.53 µs | 7.83 µs | 5.31 µs |
| D115 | 2.17 µs | 4.8 µs | 6.07 µs | 15.2 µs | 19.8 µs |
| D153 | 3.03 µs | 4.18 µs | 9.51 µs | 16.4 µs | 30.5 µs |
| D230 | 3.08 µs | 10.1 µs | 19.8 µs | 31.3 µs | 60 µs |
| D307 | 3.26 µs | 12.3 µs | 18 µs | 74.2 µs | 118 µs |
| D462 | 1.89 µs | 16 µs | 66.7 µs | 139 µs | 237 µs |
| D616 | 3.1 µs | 32.9 µs | 123 µs | 268 µs | 431 µs |
| D924 | 2.04 µs | 50.3 µs | 276 µs | 577 µs | 1.14 ms |
| D1232 | 3.61 µs | 124 µs | 462 µs | 890 µs | 2.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.9 88.2,167.0 124.4,153.0 160.5,157.6 196.7,156.6 232.9,150.8 269.1,150.5 305.3,149.5 341.5,158.9 377.6,150.3 413.8,157.6 450.0,147.7 450.0,35.6 413.8,47.7 377.6,64.6 341.5,75.0 305.3,87.1 269.1,98.9 232.9,110.6 196.7,118.1 160.5,141.0 124.4,130.8 88.2,145.4 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.9 88.2,167.0 124.4,153.0 160.5,157.6 196.7,156.6 232.9,150.8 269.1,150.5 305.3,149.5 341.5,158.9 377.6,150.3 413.8,157.6 450.0,147.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,167.0 88.2,152.8 124.4,147.1 160.5,145.8 196.7,142.7 232.9,145.2 269.1,129.8 305.3,126.4 341.5,121.8 377.6,109.3 413.8,101.9 450.0,86.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.8 88.2,150.9 124.4,143.5 160.5,140.3 196.7,138.7 232.9,130.9 269.1,118.2 305.3,119.7 341.5,97.0 377.6,86.4 413.8,72.4 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.4 88.2,148.4 124.4,140.6 160.5,134.2 196.7,122.7 232.9,121.5 269.1,110.2 305.3,95.2 341.5,84.2 377.6,72.9 413.8,59.6 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,145.4 124.4,130.8 160.5,141.0 196.7,118.1 232.9,110.6 269.1,98.9 305.3,87.1 341.5,75.0 377.6,64.6 413.8,47.7 450.0,35.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.13 µs | 1.73 µs | 3.62 µs | 3.12 µs | 4.13 µs |
| D38 | 1.45 µs | 3.63 µs | 3.91 µs | 4.39 µs | 5.36 µs |
| D57 | 5.04 µs | 5.93 µs | 7 µs | 7.94 µs | 11.1 µs |
| D76 | 4.06 µs | 6.32 µs | 7.62 µs | 11 µs | 7.39 µs |
| D115 | 9.54 µs | 12.1 µs | 7.74 µs | 23.2 µs | 28.3 µs |
| D153 | 5.81 µs | 6.65 µs | 15 µs | 20.2 µs | 36.6 µs |
| D230 | 5.89 µs | 14.5 µs | 25 µs | 37.9 µs | 68.3 µs |
| D307 | 6.53 µs | 16.6 µs | 31.6 µs | 85.4 µs | 125 µs |
| D462 | 6.09 µs | 25 µs | 86.7 µs | 168 µs | 251 µs |
| D616 | 6.19 µs | 39.8 µs | 135 µs | 290 µs | 416 µs |
| D924 | 4.03 µs | 62.2 µs | 291 µs | 564 µs | 1e+03 µs |
| D1232 | 6.93 µs | 135 µs | 451 µs | 784 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,207.3 88.2,202.0 124.4,174.9 160.5,179.6 196.7,161.0 232.9,171.8 269.1,171.5 305.3,169.2 341.5,170.8 377.6,170.4 413.8,179.7 450.0,168.0 450.0,37.4 413.8,60.0 377.6,79.0 341.5,90.0 305.3,105.2 269.1,118.3 232.9,131.8 196.7,137.4 160.5,166.6 124.4,157.8 88.2,173.6 52.0,179.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,207.3 88.2,202.0 124.4,174.9 160.5,179.6 196.7,161.0 232.9,171.8 269.1,171.5 305.3,169.2 341.5,170.8 377.6,170.4 413.8,179.7 450.0,168.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.2 88.2,182.0 124.4,171.4 160.5,169.9 196.7,155.9 232.9,168.9 269.1,151.9 305.3,149.0 341.5,140.1 377.6,130.0 413.8,120.3 450.0,103.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,180.4 124.4,167.7 160.5,165.9 196.7,165.6 232.9,151.2 269.1,140.1 305.3,135.0 341.5,113.1 377.6,103.5 413.8,86.8 450.0,77.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.3 88.2,177.9 124.4,165.0 160.5,157.8 196.7,141.7 232.9,144.8 269.1,131.0 305.3,113.4 341.5,98.7 377.6,86.9 413.8,72.4 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,173.6 124.4,157.8 160.5,166.6 196.7,137.4 232.9,131.8 269.1,118.3 305.3,105.2 341.5,90.0 377.6,79.0 413.8,60.0 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 999 ns | 1.08 µs | 2.52 µs | 2.15 µs | 3.01 µs |
| D38 | 1.29 µs | 2.53 µs | 2.85 µs | 3.31 µs | 4.09 µs |
| D57 | 2.83 µs | 3.53 µs | 4.3 µs | 5.36 µs | 9.57 µs |
| D76 | 2.2 µs | 3.82 µs | 5.31 µs | 7.82 µs | 5.14 µs |
| D115 | 2.32 µs | 4.49 µs | 6.24 µs | 15.3 µs | 19.6 µs |
| D153 | 3.16 µs | 3.87 µs | 9.68 µs | 15 µs | 29.3 µs |
| D230 | 3.16 µs | 10 µs | 19.3 µs | 30.8 µs | 57.2 µs |
| D307 | 3.38 µs | 12.1 µs | 16 µs | 71.2 µs | 115 µs |
| D462 | 1.87 µs | 17.3 µs | 63.3 µs | 140 µs | 233 µs |
| D616 | 3.23 µs | 32.8 µs | 122 µs | 265 µs | 428 µs |
| D924 | 2.1 µs | 50.5 µs | 270 µs | 572 µs | 1.13 ms |
| D1232 | 3.6 µs | 120 µs | 457 µs | 889 µs | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.0 88.2,165.6 124.4,152.0 160.5,156.3 196.7,155.4 232.9,150.0 269.1,150.0 305.3,148.8 341.5,159.1 377.6,149.6 413.8,157.1 450.0,147.8 450.0,35.7 413.8,47.9 377.6,64.8 341.5,75.3 305.3,87.6 269.1,99.7 232.9,111.3 196.7,118.3 160.5,141.6 124.4,130.8 88.2,145.5 52.0,150.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.0 88.2,165.6 124.4,152.0 160.5,156.3 196.7,155.4 232.9,150.0 269.1,150.0 305.3,148.8 341.5,159.1 377.6,149.6 413.8,157.1 450.0,147.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,168.7 88.2,153.9 124.4,148.1 160.5,146.7 196.7,143.9 232.9,146.5 269.1,130.0 305.3,126.7 341.5,120.4 377.6,109.3 413.8,101.9 450.0,86.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,151.8 124.4,144.7 160.5,141.0 196.7,138.2 232.9,130.6 269.1,118.6 305.3,121.8 341.5,97.9 377.6,86.6 413.8,72.8 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,149.2 124.4,140.8 160.5,134.3 196.7,122.6 232.9,122.9 269.1,110.4 305.3,95.9 341.5,84.2 377.6,73.1 413.8,59.7 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.9 88.2,145.5 124.4,130.8 160.5,141.6 196.7,118.3 232.9,111.3 269.1,99.7 305.3,87.6 341.5,75.3 377.6,64.8 413.8,47.9 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.13 µs | 1.74 µs | 3.63 µs | 3.13 µs | 4.14 µs |
| D38 | 1.46 µs | 3.64 µs | 3.92 µs | 4.41 µs | 5.36 µs |
| D57 | 5.05 µs | 5.92 µs | 7 µs | 7.99 µs | 11.1 µs |
| D76 | 4.04 µs | 6.3 µs | 7.65 µs | 11 µs | 7.4 µs |
| D115 | 9.54 µs | 12.2 µs | 7.53 µs | 23.3 µs | 28.6 µs |
| D153 | 5.88 µs | 6.1 µs | 15 µs | 20.4 µs | 36.1 µs |
| D230 | 5.91 µs | 14.6 µs | 25 µs | 38 µs | 68.3 µs |
| D307 | 6.54 µs | 17.1 µs | 31.1 µs | 85.7 µs | 125 µs |
| D462 | 6.15 µs | 25.5 µs | 86.9 µs | 168 µs | 251 µs |
| D616 | 6.32 µs | 39.9 µs | 135 µs | 291 µs | 417 µs |
| D924 | 4.12 µs | 60.8 µs | 292 µs | 565 µs | 998 µs |
| D1232 | 7.11 µs | 135 µs | 452 µs | 784 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,207.3 88.2,201.8 124.4,174.8 160.5,179.7 196.7,161.0 232.9,171.5 269.1,171.4 305.3,169.2 341.5,170.5 377.6,170.0 413.8,179.3 450.0,167.4 450.0,37.4 413.8,60.1 377.6,79.0 341.5,90.0 305.3,105.2 269.1,118.3 232.9,132.1 196.7,137.2 160.5,166.5 124.4,157.8 88.2,173.5 52.0,179.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,207.3 88.2,201.8 124.4,174.8 160.5,179.7 196.7,161.0 232.9,171.5 269.1,171.4 305.3,169.2 341.5,170.5 377.6,170.0 413.8,179.3 450.0,167.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.0 88.2,182.0 124.4,171.4 160.5,170.0 196.7,155.8 232.9,170.7 269.1,151.8 305.3,148.4 341.5,139.7 377.6,129.9 413.8,120.8 450.0,103.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,180.3 124.4,167.7 160.5,165.8 196.7,166.1 232.9,151.2 269.1,140.1 305.3,135.3 341.5,113.0 377.6,103.5 413.8,86.7 450.0,77.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.3 88.2,177.8 124.4,164.9 160.5,157.9 196.7,141.6 232.9,144.5 269.1,131.0 305.3,113.3 341.5,98.8 377.6,86.8 413.8,72.4 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,173.5 124.4,157.8 160.5,166.5 196.7,137.2 232.9,132.1 269.1,118.3 305.3,105.2 341.5,90.0 377.6,79.0 413.8,60.1 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.86 µs | 2.19 µs | 4.29 µs | 3.81 µs | 5.06 µs |
| D38 | 2.37 µs | 4.3 µs | 4.78 µs | 5.55 µs | 6.65 µs |
| D57 | 3.56 µs | 4.66 µs | 5.71 µs | 7.04 µs | 9.75 µs |
| D76 | 2.82 µs | 5.06 µs | 6.97 µs | 9.97 µs | 6.46 µs |
| D115 | 2.99 µs | 5.97 µs | 7.77 µs | 18 µs | 23.2 µs |
| D153 | 4.01 µs | 5.17 µs | 12.4 µs | 18.5 µs | 34.1 µs |
| D230 | 4.03 µs | 12.7 µs | 24.2 µs | 36.2 µs | 63.9 µs |
| D307 | 4.56 µs | 14.7 µs | 18.7 µs | 80.4 µs | 127 µs |
| D462 | 2.47 µs | 19.5 µs | 73.4 µs | 155 µs | 250 µs |
| D616 | 4.1 µs | 37.3 µs | 135 µs | 295 µs | 462 µs |
| D924 | 2.69 µs | 57.4 µs | 296 µs | 618 µs | 1.2 ms |
| D1232 | 4.52 µs | 134 µs | 496 µs | 950 µs | 2.39 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.5 88.2,191.3 124.4,182.4 160.5,187.5 196.7,186.2 232.9,179.8 269.1,179.7 305.3,177.0 341.5,190.4 377.6,179.4 413.8,188.5 450.0,177.3 450.0,41.0 413.8,56.0 377.6,76.8 341.5,90.1 305.3,104.8 269.1,119.7 232.9,133.4 196.7,141.7 160.5,169.5 124.4,160.5 88.2,168.8 52.0,174.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.5 88.2,191.3 124.4,182.4 160.5,187.5 196.7,186.2 232.9,179.8 269.1,179.7 305.3,177.0 341.5,190.4 377.6,179.4 413.8,188.5 450.0,177.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,193.0 88.2,178.3 124.4,176.6 160.5,174.8 196.7,171.2 232.9,174.3 269.1,154.8 305.3,151.7 341.5,145.5 377.6,131.4 413.8,122.1 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,176.0 124.4,172.2 160.5,167.9 196.7,165.5 232.9,155.3 269.1,140.8 305.3,146.4 341.5,116.7 377.6,103.5 413.8,86.4 450.0,75.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.9 88.2,172.8 124.4,167.6 160.5,160.1 196.7,147.2 232.9,146.6 269.1,132.1 305.3,114.7 341.5,100.5 377.6,86.5 413.8,70.4 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,174.8 88.2,168.8 124.4,160.5 160.5,169.5 196.7,141.7 232.9,133.4 269.1,119.7 305.3,104.8 341.5,90.1 377.6,76.8 413.8,56.0 450.0,41.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.22 µs | 1.77 µs | 3.81 µs | 3.28 µs | 4.35 µs |
| D38 | 1.56 µs | 3.8 µs | 3.96 µs | 4.42 µs | 5.39 µs |
| D57 | 5.15 µs | 6.12 µs | 7.42 µs | 8.39 µs | 11.5 µs |
| D76 | 4.17 µs | 6.51 µs | 8.03 µs | 11.4 µs | 7.62 µs |
| D115 | 9.93 µs | 12.8 µs | 8.02 µs | 24 µs | 29.1 µs |
| D153 | 6.05 µs | 6.93 µs | 15.9 µs | 20.6 µs | 36.6 µs |
| D230 | 6.08 µs | 15.1 µs | 25.3 µs | 38.1 µs | 69.2 µs |
| D307 | 6.69 µs | 16.9 µs | 32.8 µs | 86.5 µs | 127 µs |
| D462 | 6.35 µs | 26.6 µs | 88.1 µs | 172 µs | 255 µs |
| D616 | 6.55 µs | 41.6 µs | 137 µs | 293 µs | 422 µs |
| D924 | 4.38 µs | 61.7 µs | 295 µs | 570 µs | 1.01 ms |
| D1232 | 7.45 µs | 136 µs | 456 µs | 788 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.8 88.2,200.4 124.4,174.4 160.5,179.0 196.7,160.1 232.9,170.9 269.1,170.8 305.3,168.7 341.5,169.9 377.6,169.2 413.8,177.9 450.0,166.4 450.0,37.3 413.8,59.8 377.6,78.7 341.5,89.7 305.3,104.8 269.1,118.0 232.9,131.8 196.7,136.8 160.5,165.9 124.4,156.9 88.2,173.4 52.0,178.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.8 88.2,200.4 124.4,174.4 160.5,179.0 196.7,160.1 232.9,170.9 269.1,170.8 305.3,168.7 341.5,169.9 377.6,169.2 413.8,177.9 450.0,166.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.7 88.2,181.0 124.4,170.6 160.5,169.3 196.7,154.7 232.9,168.0 269.1,151.0 305.3,148.6 341.5,138.8 377.6,129.1 413.8,120.5 450.0,103.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.0 88.2,180.1 124.4,166.5 160.5,164.8 196.7,164.8 232.9,149.9 269.1,139.9 305.3,134.2 341.5,112.7 377.6,103.1 413.8,86.5 450.0,77.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.2 88.2,177.7 124.4,163.8 160.5,157.1 196.7,141.0 232.9,144.4 269.1,130.9 305.3,113.2 341.5,98.3 377.6,86.7 413.8,72.2 450.0,65.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.1 88.2,173.4 124.4,156.9 160.5,165.9 196.7,136.8 232.9,131.8 269.1,118.0 305.3,104.8 341.5,89.7 377.6,78.7 413.8,59.8 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 87.3 ns | 135 ns | 173 ns | 159 ns | 178 ns |
| D38 | 109 ns | 173 ns | 165 ns | 185 ns | 207 ns |
| D57 | 203 ns | 337 ns | 322 ns | 322 ns | 450 ns |
| D76 | 149 ns | 329 ns | 328 ns | 444 ns | 266 ns |
| D115 | 317 ns | 566 ns | 415 ns | 891 ns | 969 ns |
| D153 | 480 ns | 473 ns | 741 ns | 693 ns | 983 ns |
| D230 | 595 ns | 929 ns | 1.12 µs | 1.17 µs | 1.59 µs |
| D307 | 1.03 µs | 1.23 µs | 972 ns | 2.26 µs | 2.8 µs |
| D462 | 1.05 µs | 1.66 µs | 2.36 µs | 3.32 µs | 4.26 µs |
| D616 | 1.26 µs | 2.17 µs | 2.98 µs | 4.43 µs | 5.76 µs |
| D924 | 1.03 µs | 2.03 µs | 4.95 µs | 7.07 µs | 11.4 µs |
| D1232 | 2.6 µs | 4.3 µs | 7.58 µs | 9.47 µs | 31.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,162.9 88.2,158.1 124.4,144.6 160.5,151.3 196.7,135.0 232.9,126.0 269.1,121.3 305.3,109.4 341.5,109.0 377.6,105.0 413.8,109.5 450.0,89.2 450.0,35.3 413.8,57.2 377.6,72.0 341.5,78.5 305.3,87.6 269.1,99.9 232.9,110.4 196.7,110.7 160.5,138.8 124.4,127.4 88.2,144.2 52.0,147.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,162.9 88.2,158.1 124.4,144.6 160.5,151.3 196.7,135.0 232.9,126.0 269.1,121.3 305.3,109.4 341.5,109.0 377.6,105.0 413.8,109.5 450.0,89.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.5 88.2,148.1 124.4,133.6 160.5,134.1 196.7,122.4 232.9,126.2 269.1,111.6 305.3,105.5 341.5,99.0 377.6,93.2 413.8,94.6 450.0,78.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.0 88.2,149.1 124.4,134.6 160.5,134.2 196.7,129.1 232.9,116.5 269.1,107.5 305.3,110.6 341.5,91.4 377.6,86.3 413.8,75.3 450.0,66.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.9 88.2,146.7 124.4,134.6 160.5,127.6 196.7,112.5 232.9,117.9 269.1,106.6 305.3,92.3 341.5,83.9 377.6,77.7 413.8,67.5 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.5 88.2,144.2 124.4,127.4 160.5,138.8 196.7,110.7 232.9,110.4 269.1,99.9 305.3,87.6 341.5,78.5 377.6,72.0 413.8,57.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 115 ns | 156 ns | 211 ns | 193 ns | 219 ns |
| D38 | 145 ns | 207 ns | 198 ns | 198 ns | 220 ns |
| D57 | 262 ns | 400 ns | 379 ns | 396 ns | 529 ns |
| D76 | 184 ns | 410 ns | 405 ns | 535 ns | 307 ns |
| D115 | 402 ns | 686 ns | 466 ns | 1.04 µs | 1.1 µs |
| D153 | 603 ns | 500 ns | 879 ns | 753 ns | 1.16 µs |
| D230 | 801 ns | 1.08 µs | 1.24 µs | 1.32 µs | 1.72 µs |
| D307 | 1.35 µs | 1.46 µs | 1.25 µs | 2.56 µs | 3.15 µs |
| D462 | 1.38 µs | 1.82 µs | 2.76 µs | 3.63 µs | 4.7 µs |
| D616 | 1.67 µs | 2.32 µs | 3.38 µs | 4.93 µs | 6.23 µs |
| D924 | 1.37 µs | 2.36 µs | 5.54 µs | 7.63 µs | 12.1 µs |
| D1232 | 3.24 µs | 4.89 µs | 8.28 µs | 10.2 µs | 32.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.8 88.2,199.2 124.4,182.1 160.5,192.4 196.7,169.7 232.9,158.0 269.1,149.8 305.3,134.7 341.5,134.1 377.6,128.5 413.8,134.2 450.0,109.3 450.0,42.8 413.8,71.1 377.6,90.4 341.5,98.6 305.3,110.1 269.1,127.7 232.9,139.1 196.7,140.6 160.5,177.5 124.4,161.8 88.2,187.1 52.0,187.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.8 88.2,199.2 124.4,182.1 160.5,192.4 196.7,169.7 232.9,158.0 269.1,149.8 305.3,134.7 341.5,134.1 377.6,128.5 413.8,134.2 450.0,109.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.1 88.2,189.0 124.4,169.9 160.5,169.1 196.7,154.2 232.9,163.4 269.1,141.1 305.3,132.3 341.5,126.1 377.6,119.0 413.8,118.5 450.0,97.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.4 88.2,190.2 124.4,171.4 160.5,169.5 196.7,165.4 232.9,147.1 269.1,137.2 305.3,136.9 341.5,114.0 377.6,108.1 413.8,93.8 450.0,82.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.9 88.2,190.3 124.4,170.1 160.5,161.5 196.7,142.1 232.9,151.5 269.1,135.4 305.3,116.1 341.5,106.0 377.6,97.1 413.8,84.5 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.4 88.2,187.1 124.4,161.8 160.5,177.5 196.7,140.6 232.9,139.1 269.1,127.7 305.3,110.1 341.5,98.6 377.6,90.4 413.8,71.1 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
