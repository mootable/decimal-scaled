# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.2 ns | 2.01 µs | 3.21 µs | 3.53 µs | 3.36 µs |
| D38 | 1.05 µs | 5.21 µs | 4.55 µs | 6.62 µs | 9.77 µs |
| D57 | 1.43 µs | 5.33 µs | 8.05 µs | 10.3 µs | 14 µs |
| D76 | 1.61 µs | 7.31 µs | 8.17 µs | 11.9 µs | 18.5 µs |
| D115 | 1.56 µs | 8.15 µs | 20.4 µs | 30.8 µs | 37.6 µs |
| D153 | 1.5 µs | 10.3 µs | 26.8 µs | 42.9 µs | 54.8 µs |
| D230 | 935 ns | 12.2 µs | 38 µs | 70.1 µs | 124 µs |
| D307 | 1.24 µs | 26.2 µs | 62.4 µs | 132 µs | 182 µs |
| D462 | 1.1 µs | 33.4 µs | 110 µs | 254 µs | 244 µs |
| D616 | 1.56 µs | 61.6 µs | 219 µs | 358 µs | 541 µs |
| D924 | 1.68 µs | 126 µs | 360 µs | 940 µs | 1.55 ms |
| D1232 | 2.03 µs | 219 µs | 713 µs | 1.54 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.4 88.2,123.7 124.4,119.8 160.5,118.4 196.7,118.8 232.9,119.3 269.1,125.1 305.3,121.7 341.5,123.1 377.6,118.8 413.8,117.8 450.0,115.5 450.0,23.6 413.8,33.2 377.6,46.2 341.5,56.1 305.3,59.7 269.1,64.5 232.9,74.6 196.7,79.3 160.5,88.1 124.4,91.5 88.2,96.0 52.0,109.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.4 88.2,123.7 124.4,119.8 160.5,118.4 196.7,118.8 232.9,119.3 269.1,125.1 305.3,121.7 341.5,123.1 377.6,118.8 413.8,117.8 450.0,115.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.6 88.2,103.8 124.4,103.5 160.5,99.6 196.7,98.3 232.9,95.4 269.1,93.3 305.3,83.8 341.5,80.8 377.6,73.2 413.8,64.2 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,105.5 124.4,98.4 160.5,98.2 196.7,86.9 232.9,83.5 269.1,79.2 305.3,73.0 341.5,66.0 377.6,57.4 413.8,51.3 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,100.8 124.4,95.3 160.5,93.6 196.7,81.8 232.9,77.6 269.1,71.6 305.3,63.7 341.5,55.6 377.6,51.3 413.8,39.3 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,96.0 124.4,91.5 160.5,88.1 196.7,79.3 232.9,74.6 269.1,64.5 305.3,59.7 341.5,56.1 377.6,46.2 413.8,33.2 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.64 µs | 3.66 µs | 5.85 µs | 6.45 µs | 6.05 µs |
| D38 | 2.94 µs | 5.91 µs | 7.12 µs | 6.49 µs | 9.61 µs |
| D57 | 3.3 µs | 3.65 µs | 5 µs | 6.04 µs | 7.7 µs |
| D76 | 3.75 µs | 4.96 µs | 4.84 µs | 6.53 µs | 9.32 µs |
| D115 | 7.04 µs | 9.13 µs | 13.3 µs | 15.3 µs | 19.6 µs |
| D153 | 6.57 µs | 10.4 µs | 15.5 µs | 22.7 µs | 27.5 µs |
| D230 | 5.64 µs | 10 µs | 23.6 µs | 44.9 µs | 72.4 µs |
| D307 | 10.1 µs | 28.1 µs | 47.8 µs | 88.2 µs | 131 µs |
| D462 | 8.61 µs | 24.7 µs | 69.6 µs | 166 µs | 167 µs |
| D616 | 22.4 µs | 73.5 µs | 181 µs | 271 µs | 424 µs |
| D924 | 32.4 µs | 155 µs | 319 µs | 803 µs | 1.35 ms |
| D1232 | 45.6 µs | 278 µs | 729 µs | 1.47 ms | 2.96 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.9 88.2,186.6 124.4,184.1 160.5,181.3 196.7,167.6 232.9,169.1 269.1,172.4 305.3,159.9 341.5,163.2 377.6,142.5 413.8,134.4 450.0,127.1 450.0,36.4 413.8,53.5 377.6,78.6 341.5,98.8 305.3,104.1 269.1,117.0 232.9,138.0 196.7,145.3 160.5,161.5 124.4,165.7 88.2,160.9 52.0,170.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.9 88.2,186.6 124.4,184.1 160.5,181.3 196.7,167.6 232.9,169.1 269.1,172.4 305.3,159.9 341.5,163.2 377.6,142.5 413.8,134.4 450.0,127.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.8 88.2,171.4 124.4,181.9 160.5,175.2 196.7,162.0 232.9,159.1 269.1,159.9 305.3,137.6 341.5,140.4 377.6,116.7 413.8,100.6 450.0,87.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,167.4 124.4,175.1 160.5,175.7 196.7,153.8 232.9,150.5 269.1,141.3 305.3,126.0 341.5,117.9 377.6,97.1 413.8,84.8 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.5 88.2,169.4 124.4,170.9 160.5,169.3 196.7,150.8 232.9,142.2 269.1,127.4 305.3,112.7 341.5,99.0 377.6,88.4 413.8,64.8 450.0,51.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,160.9 124.4,165.7 160.5,161.5 196.7,145.3 232.9,138.0 269.1,117.0 305.3,104.1 341.5,98.8 377.6,78.6 413.8,53.5 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 2 µs | 3.2 µs | 3.51 µs | 3.36 µs |
| D38 | 930 ns | 5.19 µs | 4.48 µs | 6.59 µs | 9.74 µs |
| D57 | 1.32 µs | 5.29 µs | 8.06 µs | 10.3 µs | 14 µs |
| D76 | 1.5 µs | 7.66 µs | 8.16 µs | 11.9 µs | 18.4 µs |
| D115 | 1.48 µs | 8.15 µs | 20 µs | 30.8 µs | 37.3 µs |
| D153 | 1.39 µs | 10.2 µs | 25.1 µs | 42.3 µs | 54.5 µs |
| D230 | 836 ns | 12.2 µs | 37.4 µs | 69.5 µs | 123 µs |
| D307 | 1.14 µs | 26.8 µs | 62 µs | 133 µs | 183 µs |
| D462 | 1.02 µs | 28.6 µs | 113 µs | 253 µs | 238 µs |
| D616 | 1.43 µs | 61.9 µs | 221 µs | 359 µs | 542 µs |
| D924 | 1.56 µs | 127 µs | 359 µs | 940 µs | 1.54 ms |
| D1232 | 1.96 µs | 218 µs | 715 µs | 1.53 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,125.2 124.4,120.9 160.5,119.2 196.7,119.4 232.9,120.2 269.1,126.5 305.3,122.7 341.5,124.1 377.6,119.9 413.8,118.8 450.0,115.9 450.0,23.6 413.8,33.2 377.6,46.2 341.5,56.4 305.3,59.6 269.1,64.5 232.9,74.7 196.7,79.4 160.5,88.1 124.4,91.6 88.2,96.0 52.0,109.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,125.2 124.4,120.9 160.5,119.2 196.7,119.4 232.9,120.2 269.1,126.5 305.3,122.7 341.5,124.1 377.6,119.9 413.8,118.8 450.0,115.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,103.9 124.4,103.6 160.5,99.0 196.7,98.3 232.9,95.4 269.1,93.3 305.3,83.5 341.5,82.7 377.6,73.1 413.8,64.2 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,105.7 124.4,98.4 160.5,98.2 196.7,87.1 232.9,84.3 269.1,79.3 305.3,73.1 341.5,65.6 377.6,57.3 413.8,51.3 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,100.9 124.4,95.4 160.5,93.6 196.7,81.8 232.9,77.8 269.1,71.7 305.3,63.6 341.5,55.6 377.6,51.3 413.8,39.3 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,96.0 124.4,91.6 160.5,88.1 196.7,79.4 232.9,74.7 269.1,64.5 305.3,59.6 341.5,56.4 377.6,46.2 413.8,33.2 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.73 µs | 2.55 µs | 2.88 µs | 2.66 µs |
| D38 | 2.65 ns | 2.62 µs | 3.16 µs | 2.94 µs | 4.24 µs |
| D57 | 1.95 ns | 4.56 µs | 7.42 µs | 9.02 µs | 11.8 µs |
| D76 | 2.23 ns | 6.76 µs | 7.31 µs | 10 µs | 14.3 µs |
| D115 | 12.4 ns | 12.9 µs | 19 µs | 22.4 µs | 29.5 µs |
| D153 | 16.5 ns | 15.3 µs | 22.5 µs | 34 µs | 39.2 µs |
| D230 | 19.2 ns | 14.9 µs | 36.7 µs | 61.8 µs | 96.1 µs |
| D307 | 43.1 ns | 40.1 µs | 68.3 µs | 124 µs | 168 µs |
| D462 | 65.2 ns | 41.1 µs | 100 µs | 210 µs | 200 µs |
| D616 | 79.7 ns | 104 µs | 241 µs | 341 µs | 489 µs |
| D924 | 104 ns | 228 µs | 423 µs | 937 µs | 1.47 ms |
| D1232 | 157 ns | 390 µs | 936 µs | 1.63 ms | 2.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.6 88.2,197.9 124.4,201.7 160.5,200.1 196.7,178.8 232.9,175.2 269.1,173.3 305.3,163.3 341.5,158.2 377.6,155.7 413.8,152.4 450.0,147.3 450.0,25.3 413.8,33.8 377.6,47.4 341.5,58.5 305.3,60.7 269.1,67.6 232.9,78.7 196.7,82.3 160.5,91.3 124.4,93.7 88.2,106.4 52.0,112.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.6 88.2,197.9 124.4,201.7 160.5,200.1 196.7,178.8 232.9,175.2 269.1,173.3 305.3,163.3 341.5,158.2 377.6,155.7 413.8,152.4 450.0,147.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,112.3 124.4,105.5 160.5,100.6 196.7,92.6 232.9,90.4 269.1,90.8 305.3,78.5 341.5,78.2 377.6,66.6 413.8,56.9 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.0 124.4,99.4 160.5,99.6 196.7,87.8 232.9,85.7 269.1,79.6 305.3,71.9 341.5,67.1 377.6,56.2 413.8,49.2 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,110.9 124.4,97.0 160.5,95.7 196.7,85.7 232.9,80.5 269.1,73.1 305.3,64.5 341.5,57.9 377.6,51.9 413.8,39.4 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,106.4 124.4,93.7 160.5,91.3 196.7,82.3 232.9,78.7 269.1,67.6 305.3,60.7 341.5,58.5 377.6,47.4 413.8,33.8 450.0,25.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.59 µs | 2.69 µs | 2.93 µs | 2.89 µs |
| D38 | 466 ns | 4.41 µs | 3.2 µs | 5.25 µs | 7.64 µs |
| D57 | 544 ns | 4.12 µs | 6.25 µs | 8.15 µs | 5.25 µs |
| D76 | 638 ns | 5.63 µs | 6.42 µs | 9.71 µs | 15.4 µs |
| D115 | 591 ns | 6.28 µs | 17 µs | 26.9 µs | 33.7 µs |
| D153 | 566 ns | 8.16 µs | 16.7 µs | 38.6 µs | 49.5 µs |
| D230 | 340 ns | 10.1 µs | 33.6 µs | 64.5 µs | 114 µs |
| D307 | 459 ns | 23 µs | 50.3 µs | 122 µs | 169 µs |
| D462 | 488 ns | 23.8 µs | 95.5 µs | 222 µs | 206 µs |
| D616 | 626 ns | 57.4 µs | 205 µs | 339 µs | 510 µs |
| D924 | 751 ns | 117 µs | 337 µs | 899 µs | 1.46 ms |
| D1232 | 1.02 µs | 206 µs | 673 µs | 1.47 ms | 3.22 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,133.8 124.4,131.8 160.5,129.9 196.7,130.8 232.9,131.4 269.1,137.7 305.3,133.9 341.5,133.2 377.6,130.1 413.8,127.8 450.0,124.1 450.0,24.1 413.8,33.9 377.6,46.9 341.5,58.2 305.3,60.6 269.1,65.5 232.9,75.9 196.7,80.6 160.5,90.4 124.4,103.7 88.2,99.1 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,133.8 124.4,131.8 160.5,129.9 196.7,130.8 232.9,131.4 269.1,137.7 305.3,133.9 341.5,133.2 377.6,130.1 413.8,127.8 450.0,124.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.5 88.2,105.9 124.4,106.7 160.5,102.8 196.7,101.5 232.9,98.2 269.1,95.6 305.3,85.4 341.5,85.0 377.6,74.0 413.8,65.2 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.8 124.4,101.6 160.5,101.2 196.7,89.2 232.9,89.4 269.1,80.7 305.3,75.7 341.5,67.7 377.6,58.2 413.8,52.1 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,103.7 124.4,98.3 160.5,96.1 196.7,83.4 232.9,79.0 269.1,72.6 305.3,64.7 341.5,57.3 377.6,52.0 413.8,39.9 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,99.1 124.4,103.7 160.5,90.4 196.7,80.6 232.9,75.9 269.1,65.5 305.3,60.6 341.5,58.2 377.6,46.9 413.8,33.9 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.77 µs | 2.87 µs | 3.19 µs | 3.09 µs |
| D38 | 2.83 ns | 2.9 µs | 3.6 µs | 3.22 µs | 4.81 µs |
| D57 | 451 ns | 4.68 µs | 6.98 µs | 8.65 µs | 11.5 µs |
| D76 | 547 ns | 6.54 µs | 6.75 µs | 9.34 µs | 14.1 µs |
| D115 | 1.12 µs | 12.8 µs | 19.3 µs | 23.2 µs | 31.5 µs |
| D153 | 945 ns | 14.9 µs | 23.9 µs | 36.1 µs | 45.7 µs |
| D230 | 826 ns | 14.6 µs | 37.1 µs | 75.8 µs | 124 µs |
| D307 | 1.63 µs | 42.1 µs | 79.2 µs | 150 µs | 233 µs |
| D462 | 1.41 µs | 33.9 µs | 120 µs | 294 µs | 294 µs |
| D616 | 3.53 µs | 121 µs | 317 µs | 482 µs | 773 µs |
| D924 | 4.95 µs | 258 µs | 568 µs | 1.47 ms | 2.51 ms |
| D1232 | 6.99 µs | 475 µs | 1.32 ms | 2.75 ms | 5.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,197.1 124.4,134.2 160.5,131.8 196.7,122.9 232.9,125.0 269.1,126.7 305.3,118.2 341.5,120.0 377.6,108.6 413.8,104.4 450.0,100.2 450.0,17.3 413.8,27.1 377.6,41.8 341.5,53.7 305.3,56.6 269.1,64.5 232.9,76.8 196.7,81.5 160.5,91.4 124.4,94.0 88.2,104.8 52.0,110.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,197.1 124.4,134.2 160.5,131.8 196.7,122.9 232.9,125.0 269.1,126.7 305.3,118.2 341.5,120.0 377.6,108.6 413.8,104.4 450.0,100.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,111.1 124.4,105.1 160.5,101.0 196.7,92.7 232.9,90.8 269.1,91.0 305.3,77.9 341.5,80.6 377.6,64.8 413.8,55.4 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,108.4 124.4,100.2 160.5,100.6 196.7,87.6 232.9,84.9 269.1,79.4 305.3,70.0 341.5,64.9 377.6,52.8 413.8,45.6 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,109.8 124.4,97.5 160.5,96.6 196.7,85.3 232.9,79.8 269.1,70.6 305.3,62.1 341.5,53.7 377.6,47.6 413.8,33.8 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,104.8 124.4,94.0 160.5,91.4 196.7,81.5 232.9,76.8 269.1,64.5 305.3,56.6 341.5,53.7 377.6,41.8 413.8,27.1 450.0,17.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.69 ns | 1.17 µs | 2.51 µs | 2.84 µs | 2.19 µs |
| D38 | 3.13 ns | 2.73 µs | 3.26 µs | 2.91 µs | 3.79 µs |
| D57 | 2.18 ns | 3.15 µs | 4.78 µs | 5.54 µs | 9.17 µs |
| D76 | 3.15 ns | 4.12 µs | 4.37 µs | 6.04 µs | 9.24 µs |
| D115 | 13 ns | 4.74 µs | 9.74 µs | 12.4 µs | 16.4 µs |
| D153 | 16.9 ns | 5.63 µs | 9.78 µs | 18.6 µs | 26.6 µs |
| D230 | 21.6 ns | 5.85 µs | 15.7 µs | 36.1 µs | 69.9 µs |
| D307 | 59 ns | 12.7 µs | 25.1 µs | 74.5 µs | 112 µs |
| D462 | 134 ns | 9.64 µs | 52.1 µs | 140 µs | 141 µs |
| D616 | 115 ns | 31.4 µs | 129 µs | 217 µs | 354 µs |
| D924 | 151 ns | 71.1 µs | 216 µs | 615 µs | 1.06 ms |
| D1232 | 372 ns | 133 µs | 458 µs | 1.06 ms | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.8 88.2,195.8 124.4,200.3 160.5,195.8 196.7,178.2 232.9,174.9 269.1,171.9 305.3,159.4 341.5,149.2 377.6,151.1 413.8,147.8 450.0,136.6 450.0,27.8 413.8,37.9 377.6,51.4 341.5,62.9 305.3,65.8 269.1,71.6 232.9,83.6 196.7,89.6 160.5,96.7 124.4,96.8 88.2,107.8 52.0,114.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.8 88.2,195.8 124.4,200.3 160.5,195.8 196.7,178.2 232.9,174.9 269.1,171.9 305.3,159.4 341.5,149.2 377.6,151.1 413.8,147.8 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,111.8 124.4,110.0 160.5,106.7 196.7,105.0 232.9,102.8 269.1,102.4 305.3,92.8 341.5,96.2 377.6,81.5 413.8,71.4 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,109.6 124.4,104.9 160.5,106.0 196.7,96.0 232.9,96.0 269.1,90.1 305.3,84.3 341.5,75.2 377.6,64.0 413.8,57.6 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,111.0 124.4,103.0 160.5,102.0 196.7,93.0 232.9,88.0 269.1,79.8 305.3,70.8 341.5,62.9 377.6,57.5 413.8,44.6 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.6 88.2,107.8 124.4,96.8 160.5,96.7 196.7,89.6 232.9,83.6 269.1,71.6 305.3,65.8 341.5,62.9 377.6,51.4 413.8,37.9 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.78 µs | 3.4 µs | 3.95 µs | 3.12 µs |
| D38 | 2.56 ns | 3.69 µs | 4.36 µs | 3.8 µs | 4.93 µs |
| D57 | 2.8 ns | 4.78 µs | 7.21 µs | 8.08 µs | 10.5 µs |
| D76 | 3.52 ns | 6.37 µs | 6.46 µs | 8.78 µs | 12.4 µs |
| D115 | 10.9 ns | 12.3 µs | 12 µs | 19.4 µs | 23 µs |
| D153 | 16.2 ns | 8 µs | 15.6 µs | 23.1 µs | 31.7 µs |
| D230 | 21.5 ns | 8.53 µs | 20.8 µs | 43.4 µs | 79.5 µs |
| D307 | 59.7 ns | 18 µs | 51.6 µs | 85.4 µs | 118 µs |
| D462 | 104 ns | 15.2 µs | 68.8 µs | 167 µs | 157 µs |
| D616 | 119 ns | 36.1 µs | 143 µs | 230 µs | 346 µs |
| D924 | 165 ns | 82.3 µs | 229 µs | 609 µs | 923 µs |
| D1232 | 362 ns | 142 µs | 451 µs | 925 µs | 3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,198.3 124.4,197.2 160.5,194.4 196.7,180.4 232.9,175.4 269.1,171.9 305.3,159.3 341.5,152.4 377.6,150.7 413.8,146.6 450.0,136.9 450.0,24.9 413.8,39.6 377.6,51.7 341.5,61.5 305.3,65.1 269.1,70.0 232.9,81.4 196.7,85.4 160.5,93.1 124.4,95.1 88.2,104.5 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,198.3 124.4,197.2 160.5,194.4 196.7,180.4 232.9,175.4 269.1,171.9 305.3,159.3 341.5,152.4 377.6,150.7 413.8,146.6 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,108.1 124.4,104.9 160.5,101.3 196.7,93.1 232.9,98.5 269.1,97.7 305.3,88.4 341.5,90.5 377.6,79.8 413.8,69.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.1 88.2,106.0 124.4,99.8 160.5,101.1 196.7,93.5 232.9,90.2 269.1,86.6 305.3,75.4 341.5,71.8 377.6,62.7 413.8,56.9 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,107.7 124.4,98.4 160.5,97.3 196.7,87.5 232.9,85.3 269.1,77.5 305.3,69.1 341.5,60.8 377.6,56.8 413.8,44.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,104.5 124.4,95.1 160.5,93.1 196.7,85.4 232.9,81.4 269.1,70.0 305.3,65.1 341.5,61.5 377.6,51.7 413.8,39.6 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 ns | 1.07 µs | 2.34 µs | 2.8 µs | 2.17 µs |
| D38 | 2.62 ns | 2.55 µs | 3.08 µs | 2.78 µs | 3.74 µs |
| D57 | 2.18 ns | 2.94 µs | 4.5 µs | 5.46 µs | 9.14 µs |
| D76 | 3.51 ns | 3.93 µs | 4.13 µs | 6 µs | 8.95 µs |
| D115 | 13 ns | 4.44 µs | 9.72 µs | 11.9 µs | 16.4 µs |
| D153 | 16.9 ns | 5.35 µs | 10.2 µs | 18.1 µs | 25.8 µs |
| D230 | 21.5 ns | 6.49 µs | 15.6 µs | 36 µs | 66.7 µs |
| D307 | 52.1 ns | 12.7 µs | 24.3 µs | 70.9 µs | 109 µs |
| D462 | 113 ns | 9.47 µs | 50.9 µs | 139 µs | 138 µs |
| D616 | 110 ns | 30.2 µs | 127 µs | 216 µs | 353 µs |
| D924 | 154 ns | 68.8 µs | 217 µs | 609 µs | 1.05 ms |
| D1232 | 347 ns | 130 µs | 454 µs | 1.05 ms | 2.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.3 88.2,198.1 124.4,200.3 160.5,194.4 196.7,178.1 232.9,174.9 269.1,171.9 305.3,160.9 341.5,151.4 377.6,151.6 413.8,147.5 450.0,137.4 450.0,27.9 413.8,37.9 377.6,51.5 341.5,63.1 305.3,66.1 269.1,72.2 232.9,84.0 196.7,89.6 160.5,97.1 124.4,96.8 88.2,107.9 52.0,114.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.3 88.2,198.1 124.4,200.3 160.5,194.4 196.7,178.1 232.9,174.9 269.1,171.9 305.3,160.9 341.5,151.4 377.6,151.6 413.8,147.5 450.0,137.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,112.7 124.4,110.9 160.5,107.3 196.7,105.8 232.9,103.5 269.1,101.1 305.3,92.7 341.5,96.4 377.6,82.0 413.8,71.8 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.8 88.2,110.3 124.4,105.6 160.5,106.7 196.7,96.1 232.9,95.4 269.1,90.2 305.3,84.7 341.5,75.5 377.6,64.2 413.8,57.5 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,111.6 124.4,103.2 160.5,102.1 196.7,93.5 232.9,88.3 269.1,79.8 305.3,71.4 341.5,63.0 377.6,57.6 413.8,44.7 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.7 88.2,107.9 124.4,96.8 160.5,97.1 196.7,89.6 232.9,84.0 269.1,72.2 305.3,66.1 341.5,63.1 377.6,51.5 413.8,37.9 450.0,27.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.78 µs | 3.39 µs | 3.95 µs | 3.12 µs |
| D38 | 2.65 ns | 3.69 µs | 4.36 µs | 3.8 µs | 4.92 µs |
| D57 | 10.6 ns | 4.81 µs | 7.24 µs | 8.14 µs | 10.5 µs |
| D76 | 12.1 ns | 6.38 µs | 6.53 µs | 8.8 µs | 12.4 µs |
| D115 | 11.2 ns | 12.2 µs | 11.8 µs | 19.7 µs | 23.6 µs |
| D153 | 16.2 ns | 8.04 µs | 15.7 µs | 23.2 µs | 31.6 µs |
| D230 | 21.5 ns | 8.96 µs | 20.2 µs | 43.8 µs | 79.6 µs |
| D307 | 52.4 ns | 17.6 µs | 50.5 µs | 85.9 µs | 118 µs |
| D462 | 136 ns | 15.3 µs | 70.9 µs | 168 µs | 161 µs |
| D616 | 111 ns | 36 µs | 144 µs | 230 µs | 347 µs |
| D924 | 160 ns | 82.4 µs | 229 µs | 611 µs | 923 µs |
| D1232 | 366 ns | 143 µs | 449 µs | 927 µs | 3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,197.9 124.4,180.7 160.5,179.1 196.7,180.0 232.9,175.4 269.1,171.9 305.3,160.9 341.5,149.1 377.6,151.5 413.8,147.0 450.0,136.7 450.0,24.9 413.8,39.6 377.6,51.7 341.5,61.2 305.3,65.1 269.1,70.0 232.9,81.4 196.7,85.1 160.5,93.0 124.4,95.1 88.2,104.5 52.0,110.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,197.9 124.4,180.7 160.5,179.1 196.7,180.0 232.9,175.4 269.1,171.9 305.3,160.9 341.5,149.1 377.6,151.5 413.8,147.0 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,108.1 124.4,104.8 160.5,101.3 196.7,93.3 232.9,98.4 269.1,97.1 305.3,88.7 341.5,90.5 377.6,79.8 413.8,69.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.1 88.2,106.0 124.4,99.7 160.5,101.0 196.7,93.6 232.9,90.1 269.1,87.0 305.3,75.6 341.5,71.4 377.6,62.7 413.8,56.9 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,107.7 124.4,98.3 160.5,97.3 196.7,87.3 232.9,85.2 269.1,77.4 305.3,69.0 341.5,60.7 377.6,56.8 413.8,44.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.1 88.2,104.5 124.4,95.1 160.5,93.0 196.7,85.1 232.9,81.4 269.1,70.0 305.3,65.1 341.5,61.2 377.6,51.7 413.8,39.6 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.3 ns | 2.12 µs | 3.97 µs | 4.62 µs | 3.85 µs |
| D38 | 2.59 ns | 4.36 µs | 5.17 µs | 4.63 µs | 6.11 µs |
| D57 | 3.11 ns | 3.89 µs | 5.91 µs | 7.1 µs | 9.11 µs |
| D76 | 3.63 ns | 5.12 µs | 5.6 µs | 7.67 µs | 11.1 µs |
| D115 | 9.21 ns | 5.89 µs | 12.4 µs | 14.2 µs | 19.6 µs |
| D153 | 16.5 ns | 7.06 µs | 12.3 µs | 22.3 µs | 30.4 µs |
| D230 | 21.9 ns | 8.1 µs | 18.9 µs | 41.4 µs | 75.9 µs |
| D307 | 52.1 ns | 16.6 µs | 29.4 µs | 80.8 µs | 120 µs |
| D462 | 101 ns | 11.9 µs | 59.8 µs | 155 µs | 153 µs |
| D616 | 105 ns | 35.1 µs | 143 µs | 238 µs | 382 µs |
| D924 | 139 ns | 78.7 µs | 237 µs | 658 µs | 1.12 ms |
| D1232 | 361 ns | 143 µs | 495 µs | 1.12 ms | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.2 88.2,198.2 124.4,195.9 160.5,194.0 196.7,182.4 232.9,175.2 269.1,171.7 305.3,160.9 341.5,152.8 377.6,152.2 413.8,148.8 450.0,136.9 450.0,27.2 413.8,37.2 377.6,50.5 341.5,61.9 305.3,64.9 269.1,70.6 232.9,81.9 196.7,87.3 160.5,94.4 124.4,96.9 88.2,101.8 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.2 88.2,198.2 124.4,195.9 160.5,194.0 196.7,182.4 232.9,175.2 269.1,171.7 305.3,160.9 341.5,152.8 377.6,152.2 413.8,148.8 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,106.0 124.4,107.4 160.5,104.0 196.7,102.3 232.9,100.0 269.1,98.3 305.3,89.4 341.5,93.6 377.6,80.1 413.8,70.1 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,103.9 124.4,102.2 160.5,102.9 196.7,93.0 232.9,93.2 269.1,87.8 305.3,82.3 341.5,73.5 377.6,62.7 413.8,56.4 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,105.3 124.4,100.0 160.5,99.0 196.7,91.3 232.9,85.8 269.1,78.1 305.3,69.8 341.5,61.7 377.6,56.4 413.8,43.8 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,101.8 124.4,96.9 160.5,94.4 196.7,87.3 232.9,81.9 269.1,70.6 305.3,64.9 341.5,61.9 377.6,50.5 413.8,37.2 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.83 µs | 3.57 µs | 4.14 µs | 3.28 µs |
| D38 | 2.51 ns | 3.87 µs | 4.41 µs | 3.81 µs | 4.96 µs |
| D57 | 2.63 µs | 4.94 µs | 7.58 µs | 8.51 µs | 11 µs |
| D76 | 2.91 µs | 6.59 µs | 6.82 µs | 9.09 µs | 12.9 µs |
| D115 | 5.74 µs | 13 µs | 12.7 µs | 20.1 µs | 23.7 µs |
| D153 | 2.77 µs | 8.41 µs | 16.6 µs | 24 µs | 32.4 µs |
| D230 | 1.73 µs | 8.97 µs | 20.8 µs | 44.4 µs | 80.9 µs |
| D307 | 2.56 µs | 18.1 µs | 52.6 µs | 88.1 µs | 120 µs |
| D462 | 2.3 µs | 15.7 µs | 71.5 µs | 171 µs | 157 µs |
| D616 | 3.29 µs | 37.5 µs | 145 µs | 233 µs | 351 µs |
| D924 | 3.45 µs | 84 µs | 232 µs | 614 µs | 932 µs |
| D1232 | 4.51 µs | 146 µs | 455 µs | 931 µs | 3.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,198.6 124.4,112.3 160.5,111.0 196.7,102.6 232.9,111.6 269.1,117.5 305.3,112.6 341.5,114.0 377.6,109.5 413.8,108.9 450.0,105.6 450.0,24.9 413.8,39.4 377.6,51.6 341.5,61.6 305.3,64.8 269.1,69.8 232.9,81.1 196.7,85.0 160.5,92.6 124.4,94.6 88.2,104.4 52.0,109.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,198.6 124.4,112.3 160.5,111.0 196.7,102.6 232.9,111.6 269.1,117.5 305.3,112.6 341.5,114.0 377.6,109.5 413.8,108.9 450.0,105.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,107.5 124.4,104.5 160.5,100.9 196.7,92.4 232.9,97.9 269.1,97.1 305.3,88.3 341.5,90.1 377.6,79.3 413.8,69.3 450.0,62.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.9 124.4,99.2 160.5,100.5 196.7,92.7 232.9,89.4 269.1,86.6 305.3,75.1 341.5,71.3 377.6,62.6 413.8,56.7 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,107.7 124.4,97.7 160.5,96.9 196.7,87.1 232.9,84.9 269.1,77.2 305.3,68.7 341.5,60.5 377.6,56.6 413.8,44.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.6 88.2,104.4 124.4,94.6 160.5,92.6 196.7,85.0 232.9,81.1 269.1,69.8 305.3,64.8 341.5,61.6 377.6,51.6 413.8,39.4 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 132 ns | 156 ns | 173 ns | 135 ns |
| D38 | 2.69 ns | 170 ns | 178 ns | 154 ns | 186 ns |
| D57 | 182 ns | 259 ns | 327 ns | 332 ns | 419 ns |
| D76 | 198 ns | 328 ns | 266 ns | 350 ns | 437 ns |
| D115 | 464 ns | 551 ns | 716 ns | 645 ns | 786 ns |
| D153 | 417 ns | 623 ns | 740 ns | 879 ns | 930 ns |
| D230 | 334 ns | 511 ns | 877 ns | 1.31 µs | 1.83 µs |
| D307 | 623 ns | 1.31 µs | 1.51 µs | 2.24 µs | 2.59 µs |
| D462 | 577 ns | 847 ns | 1.76 µs | 3.21 µs | 2.58 µs |
| D616 | 1.08 µs | 1.86 µs | 3.14 µs | 3.49 µs | 4.72 µs |
| D924 | 1.43 µs | 2.74 µs | 3.87 µs | 7.61 µs | 10.4 µs |
| D1232 | 2.31 µs | 4.23 µs | 7.43 µs | 11.1 µs | 33.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,192.8 124.4,119.6 160.5,118.1 196.7,103.4 232.9,105.2 269.1,109.1 305.3,98.2 341.5,99.5 377.6,88.6 413.8,83.8 450.0,75.5 450.0,29.1 413.8,49.3 377.6,63.0 341.5,73.5 305.3,73.5 269.1,79.5 232.9,91.3 196.7,94.2 160.5,104.4 124.4,105.1 88.2,119.3 52.0,124.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,192.8 124.4,119.6 160.5,118.1 196.7,103.4 232.9,105.2 269.1,109.1 305.3,98.2 341.5,99.5 377.6,88.6 413.8,83.8 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.1 88.2,120.8 124.4,113.5 160.5,109.3 196.7,100.4 232.9,98.2 269.1,101.7 305.3,85.3 341.5,92.9 377.6,79.2 413.8,72.5 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.3 88.2,120.0 124.4,109.4 160.5,113.0 196.7,95.8 232.9,95.2 269.1,92.3 305.3,82.8 341.5,80.1 377.6,70.1 413.8,66.5 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.4 88.2,122.5 124.4,109.2 160.5,108.2 196.7,97.6 232.9,92.2 269.1,85.3 305.3,76.0 341.5,69.7 377.6,68.3 413.8,54.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.7 88.2,119.3 124.4,105.1 160.5,104.4 196.7,94.2 232.9,91.3 269.1,79.5 305.3,73.5 341.5,73.5 377.6,63.0 413.8,49.3 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 155 ns | 197 ns | 209 ns | 170 ns |
| D38 | 2.75 ns | 210 ns | 210 ns | 165 ns | 198 ns |
| D57 | 291 ns | 345 ns | 426 ns | 457 ns | 528 ns |
| D76 | 314 ns | 426 ns | 355 ns | 440 ns | 565 ns |
| D115 | 723 ns | 785 ns | 937 ns | 830 ns | 960 ns |
| D153 | 653 ns | 902 ns | 999 ns | 1.13 µs | 1.14 µs |
| D230 | 608 ns | 720 ns | 1.12 µs | 1.73 µs | 2.22 µs |
| D307 | 1.04 µs | 1.83 µs | 2.04 µs | 2.79 µs | 3.16 µs |
| D462 | 953 ns | 1.08 µs | 2.23 µs | 3.85 µs | 2.97 µs |
| D616 | 1.83 µs | 2.47 µs | 3.88 µs | 4.18 µs | 5.32 µs |
| D924 | 2.36 µs | 3.63 µs | 4.63 µs | 8.58 µs | 11.4 µs |
| D1232 | 3.49 µs | 5.43 µs | 8.67 µs | 12.6 µs | 34.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,192.4 124.4,111.4 160.5,110.1 196.7,95.6 232.9,97.4 269.1,98.7 305.3,89.3 341.5,90.8 377.6,79.5 413.8,75.1 450.0,68.3 450.0,28.3 413.8,47.7 377.6,61.0 341.5,71.1 305.3,70.0 269.1,76.2 232.9,87.7 196.7,90.7 160.5,99.9 124.4,101.1 88.2,118.1 52.0,120.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,192.4 124.4,111.4 160.5,110.1 196.7,95.6 232.9,97.4 269.1,98.7 305.3,89.3 341.5,90.8 377.6,79.5 413.8,75.1 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.4 88.2,117.1 124.4,108.5 160.5,104.8 196.7,94.2 232.9,91.8 269.1,95.7 305.3,79.5 341.5,88.6 377.6,74.3 413.8,67.6 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,117.1 124.4,104.8 160.5,108.0 196.7,91.1 232.9,90.0 269.1,88.1 305.3,77.6 341.5,76.1 377.6,66.5 413.8,63.4 450.0,52.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.2 88.2,121.3 124.4,103.6 160.5,104.3 196.7,93.2 232.9,87.9 269.1,80.5 305.3,72.2 341.5,66.6 377.6,65.1 413.8,52.7 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,118.1 124.4,101.1 160.5,99.9 196.7,90.7 232.9,87.7 269.1,76.2 305.3,70.0 341.5,71.1 377.6,61.0 413.8,47.7 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
