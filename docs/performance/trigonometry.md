# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.36 ns | 1.26 µs | 3.21 µs | 3.48 µs | 4.08 µs |
| D38 | 1.3 µs | 4.66 µs | 2.62 µs | 5.35 µs | 8.01 µs |
| D57 | 1.55 µs | 5.21 µs | 8.1 µs | 10.5 µs | 14 µs |
| D76 | 1.41 µs | 6.61 µs | 10.6 µs | 14.6 µs | 13.3 µs |
| D115 | 1.45 µs | 8.12 µs | 20.1 µs | 33 µs | 34.3 µs |
| D153 | 1.46 µs | 9.86 µs | 20.4 µs | 41.8 µs | 61.2 µs |
| D230 | 1.64 µs | 20.1 µs | 42.7 µs | 70.4 µs | 94.1 µs |
| D307 | 1.47 µs | 21.9 µs | 61.7 µs | 125 µs | 193 µs |
| D462 | 1.65 µs | 42.3 µs | 125 µs | 256 µs | 407 µs |
| D616 | 1.64 µs | 60.8 µs | 218 µs | 425 µs | 661 µs |
| D924 | 1.66 µs | 118 µs | 450 µs | 771 µs | 1.56 ms |
| D1232 | 2.02 µs | 207 µs | 717 µs | 1.04 ms | 3.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.2 88.2,121.0 124.4,118.8 160.5,120.0 196.7,119.7 232.9,119.6 269.1,118.1 305.3,119.5 341.5,118.0 377.6,118.1 413.8,118.0 450.0,115.6 450.0,23.9 413.8,33.1 377.6,43.7 341.5,49.7 305.3,59.0 269.1,67.9 232.9,73.2 196.7,80.4 160.5,92.2 124.4,91.6 88.2,98.5 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.2 88.2,121.0 124.4,118.8 160.5,120.0 196.7,119.7 232.9,119.6 269.1,118.1 305.3,119.5 341.5,118.0 377.6,118.1 413.8,118.0 450.0,115.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.5 88.2,105.2 124.4,103.8 160.5,100.9 196.7,98.3 232.9,95.9 269.1,87.0 305.3,86.0 341.5,77.8 377.6,73.3 413.8,65.1 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,112.3 124.4,98.3 160.5,95.0 196.7,87.1 232.9,86.8 269.1,77.7 305.3,73.1 341.5,64.4 377.6,57.5 413.8,48.5 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,103.5 124.4,95.1 160.5,91.0 196.7,80.9 232.9,78.0 269.1,71.5 305.3,64.3 341.5,55.5 377.6,49.2 413.8,41.8 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,98.5 124.4,91.6 160.5,92.2 196.7,80.4 232.9,73.2 269.1,67.9 305.3,59.0 341.5,49.7 377.6,43.7 413.8,33.1 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.58 µs | 2.26 µs | 5.88 µs | 6.38 µs | 7.13 µs |
| D38 | 3.22 µs | 5.83 µs | 4.9 µs | 6.41 µs | 8.34 µs |
| D57 | 15.4 µs | 18.8 µs | 30.1 µs | 34.4 µs | 37.3 µs |
| D76 | 14.2 µs | 25.4 µs | 36.8 µs | 37.4 µs | 29.3 µs |
| D115 | 14.2 µs | 41 µs | 71.6 µs | 111 µs | 107 µs |
| D153 | 14.2 µs | 49.8 µs | 80.8 µs | 134 µs | 148 µs |
| D230 | 15.4 µs | 67.4 µs | 187 µs | 273 µs | 237 µs |
| D307 | 14.6 µs | 85.7 µs | 256 µs | 516 µs | 615 µs |
| D462 | 15.4 µs | 191 µs | 521 µs | 648 µs | 795 µs |
| D616 | 15.4 µs | 308 µs | 980 µs | 2.91 ms | 3.44 ms |
| D924 | 14.5 µs | 562 µs | 2.2 ms | 5.29 ms | 7.22 ms |
| D1232 | 15.9 µs | 913 µs | 7.72 ms | 5.74 ms | 11.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.6 88.2,189.7 124.4,162.6 160.5,163.9 196.7,163.9 232.9,163.9 269.1,162.5 305.3,163.4 341.5,162.5 377.6,162.5 413.8,163.5 450.0,161.9 450.0,47.7 413.8,55.7 377.6,68.5 341.5,94.0 305.3,98.5 269.1,115.0 232.9,123.2 196.7,128.9 160.5,151.3 124.4,147.1 88.2,173.1 52.0,175.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.6 88.2,189.7 124.4,162.6 160.5,163.9 196.7,163.9 232.9,163.9 269.1,162.5 305.3,163.4 341.5,162.5 377.6,162.5 413.8,163.5 450.0,161.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.8 88.2,179.4 124.4,159.0 160.5,153.8 196.7,145.5 232.9,142.1 269.1,136.8 305.3,132.7 341.5,118.7 377.6,110.4 413.8,100.0 450.0,91.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,182.4 124.4,150.9 160.5,147.4 196.7,135.8 232.9,133.7 269.1,119.1 305.3,113.7 341.5,101.3 377.6,90.4 413.8,76.3 450.0,54.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.8 88.2,177.7 124.4,148.5 160.5,147.1 196.7,128.2 232.9,124.9 269.1,112.5 305.3,101.5 341.5,97.5 377.6,71.4 413.8,61.1 450.0,59.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,175.9 88.2,173.1 124.4,147.1 160.5,151.3 196.7,128.9 232.9,123.2 269.1,115.0 305.3,98.5 341.5,94.0 377.6,68.5 413.8,55.7 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.44 ns | 1.24 µs | 3.2 µs | 3.47 µs | 4.07 µs |
| D38 | 1.21 µs | 4.65 µs | 2.32 µs | 5.34 µs | 7.98 µs |
| D57 | 1.44 µs | 5.21 µs | 8.06 µs | 10.4 µs | 14 µs |
| D76 | 1.3 µs | 6.55 µs | 10.5 µs | 14.5 µs | 13.3 µs |
| D115 | 1.32 µs | 8.07 µs | 20.8 µs | 33 µs | 33.7 µs |
| D153 | 1.33 µs | 9.95 µs | 20.4 µs | 42.2 µs | 61 µs |
| D230 | 1.54 µs | 20 µs | 42.3 µs | 70.4 µs | 92.3 µs |
| D307 | 1.35 µs | 22.2 µs | 61.4 µs | 125 µs | 193 µs |
| D462 | 1.57 µs | 42.7 µs | 126 µs | 257 µs | 405 µs |
| D616 | 1.55 µs | 61.2 µs | 219 µs | 425 µs | 661 µs |
| D924 | 1.52 µs | 118 µs | 450 µs | 765 µs | 1.55 ms |
| D1232 | 1.92 µs | 207 µs | 710 µs | 1.03 ms | 3.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,122.0 124.4,119.8 160.5,121.0 196.7,120.8 232.9,120.7 269.1,119.0 305.3,120.6 341.5,118.6 377.6,118.8 413.8,119.1 450.0,116.2 450.0,23.9 413.8,33.2 377.6,43.7 341.5,49.8 305.3,59.0 269.1,68.1 232.9,73.3 196.7,80.6 160.5,92.2 124.4,91.6 88.2,98.5 52.0,106.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,122.0 124.4,119.8 160.5,121.0 196.7,120.8 232.9,120.7 269.1,119.0 305.3,120.6 341.5,118.6 377.6,118.8 413.8,119.1 450.0,116.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.6 88.2,105.2 124.4,103.8 160.5,101.0 196.7,98.4 232.9,95.8 269.1,87.1 305.3,85.8 341.5,77.7 377.6,73.2 413.8,65.1 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,113.8 124.4,98.4 160.5,95.1 196.7,86.6 232.9,86.9 269.1,77.8 305.3,73.2 341.5,64.3 377.6,57.4 413.8,48.5 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,103.5 124.4,95.2 160.5,91.1 196.7,80.9 232.9,77.9 269.1,71.5 305.3,64.4 341.5,55.4 377.6,49.2 413.8,41.9 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,98.5 124.4,91.6 160.5,92.2 196.7,80.6 232.9,73.3 269.1,68.1 305.3,59.0 341.5,49.8 377.6,43.7 413.8,33.2 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.6 ns | 1.08 µs | 2.54 µs | 2.81 µs | 3.12 µs |
| D38 | 3.73 ns | 2.52 µs | 2.06 µs | 2.84 µs | 3.66 µs |
| D57 | 2.11 ns | 4.52 µs | 7.21 µs | 8.87 µs | 11.7 µs |
| D76 | 2.42 ns | 6.29 µs | 9.39 µs | 11.9 µs | 10.1 µs |
| D115 | 13.7 ns | 12.8 µs | 20.6 µs | 24.7 µs | 27 µs |
| D153 | 16.5 ns | 14.9 µs | 17.8 µs | 33.2 µs | 44.2 µs |
| D230 | 31.5 ns | 23.9 µs | 41.3 µs | 61.7 µs | 71.6 µs |
| D307 | 44.9 ns | 32.2 µs | 68.4 µs | 117 µs | 177 µs |
| D462 | 75.3 ns | 55.8 µs | 121 µs | 210 µs | 338 µs |
| D616 | 95.1 ns | 103 µs | 246 µs | 399 µs | 613 µs |
| D924 | 102 ns | 204 µs | 531 µs | 743 µs | 1.47 ms |
| D1232 | 156 ns | 369 µs | 921 µs | 1.03 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,193.7 124.4,200.7 160.5,199.0 196.7,177.5 232.9,175.2 269.1,167.2 305.3,162.8 341.5,156.4 377.6,153.5 413.8,152.6 450.0,147.3 450.0,25.4 413.8,33.8 377.6,44.6 341.5,52.0 305.3,60.1 269.1,71.3 232.9,77.3 196.7,83.4 160.5,95.6 124.4,93.7 88.2,108.2 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,193.7 124.4,200.7 160.5,199.0 196.7,177.5 232.9,175.2 269.1,167.2 305.3,162.8 341.5,156.4 377.6,153.5 413.8,152.6 450.0,147.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.3 88.2,112.8 124.4,105.6 160.5,101.5 196.7,92.7 232.9,90.8 269.1,84.9 305.3,81.2 341.5,74.4 377.6,66.7 413.8,58.3 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,115.3 124.4,99.8 160.5,96.5 196.7,86.8 232.9,88.5 269.1,78.1 305.3,71.8 341.5,64.8 377.6,56.0 413.8,46.4 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,111.3 124.4,97.2 160.5,93.6 196.7,84.5 232.9,80.8 269.1,73.1 305.3,65.2 341.5,58.0 377.6,50.0 413.8,42.3 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,108.2 124.4,93.7 160.5,95.6 196.7,83.4 232.9,77.3 269.1,71.3 305.3,60.1 341.5,52.0 377.6,44.6 413.8,33.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.47 ns | 978 ns | 2.69 µs | 2.94 µs | 3.53 µs |
| D38 | 632 ns | 4.04 µs | 1.68 µs | 4.19 µs | 6.37 µs |
| D57 | 590 ns | 4.26 µs | 6.27 µs | 8.24 µs | 5.3 µs |
| D76 | 536 ns | 5.22 µs | 8.27 µs | 12 µs | 11 µs |
| D115 | 550 ns | 6.3 µs | 17 µs | 29.8 µs | 30.8 µs |
| D153 | 549 ns | 7.68 µs | 13.4 µs | 37.9 µs | 55.7 µs |
| D230 | 665 ns | 17.1 µs | 38.6 µs | 64.3 µs | 85.6 µs |
| D307 | 555 ns | 19.3 µs | 49.6 µs | 115 µs | 178 µs |
| D462 | 691 ns | 34.4 µs | 106 µs | 223 µs | 343 µs |
| D616 | 707 ns | 55.6 µs | 204 µs | 396 µs | 625 µs |
| D924 | 747 ns | 109 µs | 423 µs | 729 µs | 1.46 ms |
| D1232 | 1.02 µs | 192 µs | 674 µs | 1.01 ms | 3.14 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.6 88.2,130.0 124.4,130.8 160.5,132.0 196.7,131.7 232.9,131.7 269.1,129.4 305.3,131.6 341.5,128.9 377.6,128.6 413.8,127.9 450.0,124.0 450.0,24.4 413.8,33.9 377.6,44.4 341.5,51.8 305.3,60.0 269.1,69.1 232.9,74.4 196.7,81.7 160.5,94.5 124.4,103.6 88.2,101.3 52.0,108.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.6 88.2,130.0 124.4,130.8 160.5,132.0 196.7,131.7 232.9,131.7 269.1,129.4 305.3,131.6 341.5,128.9 377.6,128.6 413.8,127.9 450.0,124.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.6 88.2,107.0 124.4,106.3 160.5,103.8 196.7,101.4 232.9,99.0 269.1,89.1 305.3,87.6 341.5,80.4 377.6,74.4 413.8,66.1 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,117.8 124.4,101.5 160.5,98.1 196.7,89.1 232.9,92.1 269.1,78.9 305.3,75.8 341.5,66.5 377.6,58.3 413.8,49.2 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,106.5 124.4,98.1 160.5,93.4 196.7,82.2 232.9,79.2 269.1,72.6 305.3,65.4 341.5,57.2 377.6,50.1 413.8,42.5 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,101.3 124.4,103.6 160.5,94.5 196.7,81.7 232.9,74.4 269.1,69.1 305.3,60.0 341.5,51.8 377.6,44.4 413.8,33.9 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.56 ns | 1.07 µs | 2.87 µs | 3.16 µs | 3.6 µs |
| D38 | 3.73 ns | 2.85 µs | 2.43 µs | 3.26 µs | 4.19 µs |
| D57 | 530 ns | 4.78 µs | 7.21 µs | 8.8 µs | 11.3 µs |
| D76 | 441 ns | 6.1 µs | 8.8 µs | 11.6 µs | 10.2 µs |
| D115 | 927 ns | 12.3 µs | 20.3 µs | 25.4 µs | 27.4 µs |
| D153 | 944 ns | 13.4 µs | 17.8 µs | 34.1 µs | 49.7 µs |
| D230 | 1.4 µs | 23.3 µs | 41.2 µs | 70.9 µs | 88 µs |
| D307 | 1.93 µs | 34 µs | 79.5 µs | 146 µs | 248 µs |
| D462 | 2.09 µs | 58.2 µs | 149 µs | 300 µs | 501 µs |
| D616 | 3.64 µs | 115 µs | 318 µs | 567 µs | 954 µs |
| D924 | 4.96 µs | 223 µs | 720 µs | 1.21 ms | 2.53 ms |
| D1232 | 7.1 µs | 451 µs | 1.3 ms | 1.78 ms | 5.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.3 88.2,193.7 124.4,132.2 160.5,134.4 196.7,125.2 232.9,125.0 269.1,120.1 305.3,116.2 341.5,115.1 377.6,108.3 413.8,104.4 450.0,100.0 450.0,17.5 413.8,27.0 377.6,39.2 341.5,47.2 305.3,55.9 269.1,68.7 232.9,75.8 196.7,83.2 160.5,95.4 124.4,94.2 88.2,106.5 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.3 88.2,193.7 124.4,132.2 160.5,134.4 196.7,125.2 232.9,125.0 269.1,120.1 305.3,116.2 341.5,115.1 377.6,108.3 413.8,104.4 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.5 88.2,111.3 124.4,104.9 160.5,101.8 196.7,93.2 232.9,92.0 269.1,85.2 305.3,80.5 341.5,73.9 377.6,65.4 413.8,57.2 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,113.2 124.4,99.8 160.5,97.3 196.7,86.9 232.9,88.5 269.1,78.1 305.3,70.0 341.5,62.2 377.6,52.8 413.8,42.6 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,109.6 124.4,97.3 160.5,93.9 196.7,84.2 232.9,80.5 269.1,71.4 305.3,62.5 341.5,53.5 377.6,45.6 413.8,36.2 450.0,31.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,106.5 124.4,94.2 160.5,95.4 196.7,83.2 232.9,75.8 269.1,68.7 305.3,55.9 341.5,47.2 377.6,39.2 413.8,27.0 450.0,17.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.61 ns | 752 ns | 2.55 µs | 2.66 µs | 3.06 µs |
| D38 | 4.98 ns | 2.51 µs | 1.77 µs | 2.39 µs | 3.36 µs |
| D57 | 2.81 ns | 3.1 µs | 4.71 µs | 5.58 µs | 9.11 µs |
| D76 | 3.12 ns | 3.96 µs | 5.93 µs | 7.43 µs | 6.41 µs |
| D115 | 11.6 ns | 4.65 µs | 10.4 µs | 14 µs | 14 µs |
| D153 | 16.8 ns | 5.4 µs | 7.75 µs | 18.4 µs | 29.6 µs |
| D230 | 40.8 ns | 9.96 µs | 18.8 µs | 35.9 µs | 51.8 µs |
| D307 | 64.7 ns | 11 µs | 25.3 µs | 69.7 µs | 118 µs |
| D462 | 123 ns | 16.6 µs | 62.5 µs | 141 µs | 232 µs |
| D616 | 134 ns | 30.2 µs | 128 µs | 259 µs | 430 µs |
| D924 | 140 ns | 64.5 µs | 273 µs | 515 µs | 1.06 ms |
| D1232 | 363 ns | 123 µs | 459 µs | 749 µs | 2.31 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.1 88.2,190.1 124.4,197.2 160.5,195.9 196.7,179.6 232.9,175.0 269.1,164.0 305.3,158.3 341.5,150.3 377.6,149.2 413.8,148.7 450.0,136.8 450.0,28.2 413.8,37.9 377.6,49.0 341.5,56.7 305.3,65.1 269.1,75.3 232.9,82.2 196.7,91.5 160.5,101.2 124.4,96.9 88.2,109.2 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.1 88.2,190.1 124.4,197.2 160.5,195.9 196.7,179.6 232.9,175.0 269.1,164.0 305.3,158.3 341.5,150.3 377.6,149.2 413.8,148.7 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,127.8 88.2,112.9 124.4,110.3 160.5,107.2 196.7,105.2 232.9,103.4 269.1,95.8 305.3,94.5 341.5,89.4 377.6,82.0 413.8,72.6 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,117.2 124.4,105.1 160.5,102.2 196.7,95.2 232.9,98.9 269.1,87.9 305.3,84.2 341.5,73.0 377.6,64.1 413.8,54.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,113.5 124.4,102.9 160.5,99.4 196.7,91.6 232.9,88.1 269.1,79.9 305.3,71.6 341.5,62.9 377.6,55.3 413.8,46.8 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,109.2 124.4,96.9 160.5,101.2 196.7,91.5 232.9,82.2 269.1,75.3 305.3,65.1 341.5,56.7 377.6,49.0 413.8,37.9 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.58 ns | 1.07 µs | 3.36 µs | 3.6 µs | 4.15 µs |
| D38 | 4.05 ns | 3.3 µs | 2.42 µs | 3.06 µs | 4.27 µs |
| D57 | 3.17 ns | 4.66 µs | 7.05 µs | 7.98 µs | 10.2 µs |
| D76 | 3.75 ns | 5.84 µs | 8.21 µs | 10.4 µs | 8.93 µs |
| D115 | 9.97 ns | 12.3 µs | 13 µs | 21.8 µs | 20.7 µs |
| D153 | 16.5 ns | 7.79 µs | 12.5 µs | 22.9 µs | 36.2 µs |
| D230 | 40.8 ns | 14.1 µs | 23.3 µs | 43.5 µs | 61.3 µs |
| D307 | 65.4 ns | 14.4 µs | 50.5 µs | 80.7 µs | 125 µs |
| D462 | 125 ns | 23.6 µs | 82.4 µs | 168 µs | 251 µs |
| D616 | 134 ns | 36.7 µs | 145 µs | 271 µs | 415 µs |
| D924 | 163 ns | 78.9 µs | 294 µs | 505 µs | 925 µs |
| D1232 | 362 ns | 136 µs | 449 µs | 641 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,192.7 124.4,195.7 160.5,193.6 196.7,181.5 232.9,175.2 269.1,164.0 305.3,158.1 341.5,150.1 377.6,149.3 413.8,146.8 450.0,136.9 450.0,26.4 413.8,39.5 377.6,49.5 341.5,55.7 305.3,64.3 269.1,73.2 232.9,79.8 196.7,86.7 160.5,97.1 124.4,95.4 88.2,106.3 52.0,106.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,192.7 124.4,195.7 160.5,193.6 196.7,181.5 232.9,175.2 269.1,164.0 305.3,158.1 341.5,150.1 377.6,149.3 413.8,146.8 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.5 88.2,109.5 124.4,105.2 160.5,102.4 196.7,93.1 232.9,98.8 269.1,91.5 305.3,91.2 341.5,85.1 377.6,79.6 413.8,70.1 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,113.3 124.4,100.1 160.5,98.2 196.7,92.5 232.9,93.0 269.1,85.2 305.3,75.6 341.5,69.5 377.6,62.6 413.8,53.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,110.4 124.4,98.5 160.5,95.2 196.7,86.0 232.9,85.4 269.1,77.5 305.3,69.8 341.5,60.7 377.6,54.8 413.8,47.0 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.6 88.2,106.3 124.4,95.4 160.5,97.1 196.7,86.7 232.9,79.8 269.1,73.2 305.3,64.3 341.5,55.7 377.6,49.5 413.8,39.5 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.36 ns | 668 ns | 2.39 µs | 2.65 µs | 3.01 µs |
| D38 | 4.36 ns | 2.35 µs | 1.66 µs | 2.25 µs | 3.33 µs |
| D57 | 2.81 ns | 2.9 µs | 4.44 µs | 5.56 µs | 9.08 µs |
| D76 | 3.74 ns | 3.55 µs | 5.65 µs | 7.33 µs | 6.19 µs |
| D115 | 11.7 ns | 4.33 µs | 10.1 µs | 14 µs | 13.4 µs |
| D153 | 17.1 ns | 5.22 µs | 7.65 µs | 18.4 µs | 29.1 µs |
| D230 | 40.8 ns | 10.2 µs | 19.2 µs | 35.5 µs | 49 µs |
| D307 | 60.8 ns | 10.8 µs | 24.3 µs | 66.7 µs | 114 µs |
| D462 | 126 ns | 15.7 µs | 59.7 µs | 142 µs | 230 µs |
| D616 | 128 ns | 29.5 µs | 126 µs | 255 µs | 430 µs |
| D924 | 149 ns | 64 µs | 271 µs | 509 µs | 1.05 ms |
| D1232 | 363 ns | 121 µs | 457 µs | 755 µs | 2.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.0 88.2,191.7 124.4,197.2 160.5,193.6 196.7,179.5 232.9,174.7 269.1,164.0 305.3,159.0 341.5,150.0 377.6,149.8 413.8,147.9 450.0,136.9 450.0,28.3 413.8,37.9 377.6,49.1 341.5,56.8 305.3,65.5 269.1,76.0 232.9,82.5 196.7,92.1 160.5,101.7 124.4,96.9 88.2,109.4 52.0,110.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.0 88.2,191.7 124.4,197.2 160.5,193.6 196.7,179.5 232.9,174.7 269.1,164.0 305.3,159.0 341.5,150.0 377.6,149.8 413.8,147.9 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,129.3 88.2,113.7 124.4,111.1 160.5,108.6 196.7,106.1 232.9,103.8 269.1,95.5 305.3,94.8 341.5,90.1 377.6,82.3 413.8,72.7 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.5 88.2,118.0 124.4,105.8 160.5,102.8 196.7,95.6 232.9,99.0 269.1,87.6 305.3,84.7 341.5,73.5 377.6,64.3 413.8,54.8 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,114.2 124.4,103.0 160.5,99.6 196.7,91.5 232.9,88.2 269.1,80.0 305.3,72.2 341.5,62.8 377.6,55.5 413.8,46.9 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.6 88.2,109.4 124.4,96.9 160.5,101.7 196.7,92.1 232.9,82.5 269.1,76.0 305.3,65.5 341.5,56.8 377.6,49.1 413.8,37.9 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.41 ns | 1.09 µs | 3.37 µs | 3.6 µs | 4.16 µs |
| D38 | 4.04 ns | 3.32 µs | 2.42 µs | 3.07 µs | 4.27 µs |
| D57 | 12.3 ns | 4.69 µs | 7.08 µs | 7.97 µs | 10.3 µs |
| D76 | 10.5 ns | 5.88 µs | 8.26 µs | 10.5 µs | 8.95 µs |
| D115 | 10.3 ns | 12.3 µs | 12.8 µs | 22.2 µs | 20.5 µs |
| D153 | 15.9 ns | 7.87 µs | 12.6 µs | 23.1 µs | 35.8 µs |
| D230 | 39.9 ns | 14 µs | 23.4 µs | 43.7 µs | 59.4 µs |
| D307 | 60.3 ns | 14.5 µs | 51.5 µs | 81.2 µs | 125 µs |
| D462 | 123 ns | 23.6 µs | 82.6 µs | 168 µs | 250 µs |
| D616 | 125 ns | 36.8 µs | 144 µs | 271 µs | 415 µs |
| D924 | 183 ns | 78.5 µs | 292 µs | 508 µs | 929 µs |
| D1232 | 356 ns | 136 µs | 450 µs | 639 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.1 88.2,192.7 124.4,178.9 160.5,180.8 196.7,181.1 232.9,175.7 269.1,164.3 305.3,159.1 341.5,150.3 377.6,150.1 413.8,145.4 450.0,137.1 450.0,26.4 413.8,39.5 377.6,49.5 341.5,55.7 305.3,64.4 269.1,73.6 232.9,79.9 196.7,86.8 160.5,97.1 124.4,95.4 88.2,106.3 52.0,106.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.1 88.2,192.7 124.4,178.9 160.5,180.8 196.7,181.1 232.9,175.7 269.1,164.3 305.3,159.1 341.5,150.3 377.6,150.1 413.8,145.4 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.3 88.2,109.4 124.4,105.1 160.5,102.3 196.7,93.2 232.9,98.7 269.1,91.6 305.3,91.1 341.5,85.0 377.6,79.6 413.8,70.2 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,113.3 124.4,100.0 160.5,98.1 196.7,92.7 232.9,92.9 269.1,85.1 305.3,75.4 341.5,69.5 377.6,62.6 413.8,53.8 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,110.4 124.4,98.5 160.5,95.2 196.7,85.8 232.9,85.3 269.1,77.4 305.3,69.7 341.5,60.7 377.6,54.8 413.8,47.0 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.6 88.2,106.3 124.4,95.4 160.5,97.1 196.7,86.8 232.9,79.9 269.1,73.6 305.3,64.4 341.5,55.7 377.6,49.5 413.8,39.5 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.37 ns | 1.37 µs | 4.02 µs | 4.28 µs | 4.92 µs |
| D38 | 4.36 ns | 3.99 µs | 3.1 µs | 4.11 µs | 5.48 µs |
| D57 | 2.81 ns | 3.87 µs | 5.95 µs | 7.3 µs | 9.24 µs |
| D76 | 3.56 ns | 4.69 µs | 7.59 µs | 9.4 µs | 7.73 µs |
| D115 | 10.6 ns | 5.84 µs | 12.7 µs | 16.7 µs | 16.6 µs |
| D153 | 16.4 ns | 6.79 µs | 9.73 µs | 21.8 µs | 34 µs |
| D230 | 41.2 ns | 12.7 µs | 22.9 µs | 41.3 µs | 55.6 µs |
| D307 | 58.8 ns | 13.3 µs | 28.7 µs | 75.9 µs | 128 µs |
| D462 | 114 ns | 18.8 µs | 69.1 µs | 156 µs | 250 µs |
| D616 | 140 ns | 34.3 µs | 140 µs | 280 µs | 462 µs |
| D924 | 139 ns | 72.4 µs | 300 µs | 541 µs | 1.12 ms |
| D1232 | 357 ns | 135 µs | 493 µs | 780 µs | 2.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,191.7 124.4,197.2 160.5,194.2 196.7,180.7 232.9,175.3 269.1,163.9 305.3,159.4 341.5,151.2 377.6,148.6 413.8,148.8 450.0,137.1 450.0,27.6 413.8,37.2 377.6,48.2 341.5,55.8 305.3,64.0 269.1,74.4 232.9,80.5 196.7,89.4 160.5,98.9 124.4,96.7 88.2,103.2 52.0,104.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,191.7 124.4,197.2 160.5,194.2 196.7,180.7 232.9,175.3 269.1,163.9 305.3,159.4 341.5,151.2 377.6,148.6 413.8,148.8 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.4 88.2,107.1 124.4,107.5 160.5,105.1 196.7,102.4 232.9,100.5 269.1,92.7 305.3,92.2 341.5,87.9 377.6,80.4 413.8,71.1 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,110.3 124.4,102.2 160.5,99.1 196.7,92.7 232.9,96.1 269.1,85.5 305.3,82.7 341.5,71.7 377.6,63.0 413.8,53.5 450.0,47.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,106.7 124.4,99.6 160.5,96.5 196.7,89.4 232.9,86.1 269.1,78.1 305.3,70.6 341.5,61.7 377.6,54.4 413.8,46.2 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,103.2 124.4,96.7 160.5,98.9 196.7,89.4 232.9,80.5 269.1,74.4 305.3,64.0 341.5,55.8 377.6,48.2 413.8,37.2 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.92 ns | 1.11 µs | 3.53 µs | 3.76 µs | 4.35 µs |
| D38 | 3.74 ns | 3.47 µs | 2.51 µs | 3.08 µs | 4.3 µs |
| D57 | 2.85 µs | 4.86 µs | 7.46 µs | 8.49 µs | 10.8 µs |
| D76 | 2.56 µs | 6.03 µs | 8.53 µs | 10.9 µs | 9.19 µs |
| D115 | 5.36 µs | 13 µs | 13.3 µs | 22.6 µs | 21.5 µs |
| D153 | 2.76 µs | 8.33 µs | 12.8 µs | 23.9 µs | 36.8 µs |
| D230 | 3.04 µs | 14.9 µs | 24.1 µs | 44.5 µs | 59.6 µs |
| D307 | 2.92 µs | 14.9 µs | 52.6 µs | 82.7 µs | 127 µs |
| D462 | 3.3 µs | 25 µs | 84 µs | 170 µs | 255 µs |
| D616 | 3.54 µs | 37.9 µs | 147 µs | 274 µs | 421 µs |
| D924 | 3.48 µs | 79.7 µs | 295 µs | 512 µs | 932 µs |
| D1232 | 4.36 µs | 138 µs | 454 µs | 643 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.7 88.2,193.6 124.4,111.3 160.5,112.6 196.7,103.5 232.9,111.7 269.1,110.5 305.3,111.0 341.5,109.5 377.6,108.6 413.8,108.8 450.0,106.0 450.0,26.3 413.8,39.4 377.6,49.3 341.5,55.5 305.3,64.2 269.1,73.6 232.9,79.5 196.7,86.2 160.5,96.8 124.4,94.8 88.2,106.2 52.0,106.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.7 88.2,193.6 124.4,111.3 160.5,112.6 196.7,103.5 232.9,111.7 269.1,110.5 305.3,111.0 341.5,109.5 377.6,108.6 413.8,108.8 450.0,106.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.0 88.2,108.8 124.4,104.7 160.5,102.0 196.7,92.5 232.9,98.0 269.1,90.8 305.3,90.7 341.5,84.3 377.6,79.2 413.8,70.0 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.6 88.2,112.9 124.4,99.3 160.5,97.7 196.7,92.2 232.9,92.6 269.1,84.8 305.3,75.1 341.5,69.3 377.6,62.3 413.8,53.7 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,110.3 124.4,97.7 160.5,94.6 196.7,85.6 232.9,84.9 269.1,77.2 305.3,69.5 341.5,60.6 377.6,54.6 413.8,46.9 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.0 88.2,106.2 124.4,94.8 160.5,96.8 196.7,86.2 232.9,79.5 269.1,73.6 305.3,64.2 341.5,55.5 377.6,49.3 413.8,39.4 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.88 ns | 79 ns | 157 ns | 159 ns | 178 ns |
| D38 | 4.36 ns | 155 ns | 99.1 ns | 130 ns | 158 ns |
| D57 | 197 ns | 258 ns | 325 ns | 329 ns | 409 ns |
| D76 | 179 ns | 294 ns | 354 ns | 409 ns | 319 ns |
| D115 | 407 ns | 565 ns | 783 ns | 784 ns | 659 ns |
| D153 | 395 ns | 575 ns | 559 ns | 839 ns | 975 ns |
| D230 | 582 ns | 814 ns | 984 ns | 1.3 µs | 1.38 µs |
| D307 | 826 ns | 1.01 µs | 1.52 µs | 2.1 µs | 2.81 µs |
| D462 | 964 ns | 1.4 µs | 2.22 µs | 3.3 µs | 4.21 µs |
| D616 | 1.18 µs | 1.88 µs | 3.1 µs | 4.19 µs | 5.71 µs |
| D924 | 1.46 µs | 2.51 µs | 4.84 µs | 6.18 µs | 10.4 µs |
| D1232 | 2.27 µs | 4.08 µs | 7.42 µs | 7.6 µs | 28.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,184.4 124.4,118.2 160.5,119.9 196.7,105.6 232.9,106.1 269.1,99.4 305.3,93.3 341.5,90.6 377.6,87.2 413.8,83.4 450.0,75.8 450.0,31.5 413.8,49.2 377.6,59.7 341.5,65.0 305.3,72.0 269.1,84.4 232.9,90.4 196.7,97.2 160.5,109.9 124.4,105.5 88.2,122.1 52.0,119.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,184.4 124.4,118.2 160.5,119.9 196.7,105.6 232.9,106.1 269.1,99.4 305.3,93.3 341.5,90.6 377.6,87.2 413.8,83.4 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,134.1 88.2,122.4 124.4,113.5 160.5,111.2 196.7,99.9 232.9,99.6 269.1,93.6 305.3,89.9 341.5,84.1 377.6,79.1 413.8,74.0 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,130.1 124.4,109.5 160.5,108.0 196.7,94.3 232.9,100.1 269.1,90.3 305.3,82.8 341.5,76.1 377.6,70.3 413.8,62.6 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,125.5 124.4,109.3 160.5,105.5 196.7,94.2 232.9,93.1 269.1,85.5 305.3,77.1 341.5,69.3 377.6,65.1 413.8,58.4 450.0,54.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.9 88.2,122.1 124.4,105.5 160.5,109.9 196.7,97.2 232.9,90.4 269.1,84.4 305.3,72.0 341.5,65.0 377.6,59.7 413.8,49.2 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.92 ns | 95.4 ns | 195 ns | 198 ns | 215 ns |
| D38 | 4.36 ns | 195 ns | 137 ns | 156 ns | 183 ns |
| D57 | 315 ns | 334 ns | 416 ns | 434 ns | 521 ns |
| D76 | 291 ns | 429 ns | 509 ns | 545 ns | 401 ns |
| D115 | 651 ns | 781 ns | 1.01 µs | 1.06 µs | 780 ns |
| D153 | 606 ns | 814 ns | 698 ns | 1.04 µs | 1.23 µs |
| D230 | 960 ns | 1.16 µs | 1.29 µs | 1.68 µs | 1.56 µs |
| D307 | 1.36 µs | 1.44 µs | 2.05 µs | 2.69 µs | 3.44 µs |
| D462 | 1.54 µs | 1.89 µs | 2.81 µs | 3.94 µs | 4.92 µs |
| D616 | 1.85 µs | 2.4 µs | 3.87 µs | 4.93 µs | 6.51 µs |
| D924 | 2.38 µs | 3.25 µs | 5.86 µs | 7.02 µs | 11.3 µs |
| D1232 | 3.51 µs | 5.21 µs | 8.63 µs | 8.56 µs | 30 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.4 88.2,184.4 124.4,110.1 160.5,111.4 196.7,97.5 232.9,98.7 269.1,90.7 305.3,84.7 341.5,82.5 377.6,79.3 413.8,75.0 450.0,68.2 450.0,30.9 413.8,47.9 377.6,57.4 341.5,62.3 305.3,68.5 269.1,82.2 232.9,86.4 196.7,94.3 160.5,105.9 124.4,101.3 88.2,119.5 52.0,116.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.4 88.2,184.4 124.4,110.1 160.5,111.4 196.7,97.5 232.9,98.7 269.1,90.7 305.3,84.7 341.5,82.5 377.6,79.3 413.8,75.0 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,130.8 88.2,118.4 124.4,109.1 160.5,104.7 196.7,94.3 232.9,93.6 269.1,87.4 305.3,83.7 341.5,78.9 377.6,74.8 413.8,69.5 450.0,61.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.4 88.2,124.6 124.4,105.2 160.5,101.7 196.7,89.8 232.9,96.2 269.1,85.5 305.3,77.5 341.5,72.1 377.6,66.5 413.8,59.3 450.0,52.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,122.3 124.4,104.5 160.5,100.6 196.7,89.0 232.9,89.3 269.1,81.0 305.3,72.8 341.5,66.2 377.6,62.3 413.8,56.1 450.0,52.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.7 88.2,119.5 124.4,101.3 160.5,105.9 196.7,94.3 232.9,86.4 269.1,82.2 305.3,68.5 341.5,62.3 377.6,57.4 413.8,47.9 450.0,30.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
