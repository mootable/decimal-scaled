# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.6 ns | 2.11 µs | 3.21 µs | 3.48 µs | 3.77 µs |
| D38 | 1.36 µs | 4.72 µs | 4.23 µs | 6.7 µs | 5.8 µs |
| D57 | 1.47 µs | 5.76 µs | 8.2 µs | 10.6 µs | 11.6 µs |
| D76 | 1.57 µs | 5.03 µs | 9.76 µs | 14.4 µs | 18.5 µs |
| D115 | 1.55 µs | 8.56 µs | 19.7 µs | 33 µs | 43.3 µs |
| D153 | 1.49 µs | 9.94 µs | 24.5 µs | 42.2 µs | 61 µs |
| D230 | 1.48 µs | 20.2 µs | 28.1 µs | 42.7 µs | 107 µs |
| D307 | 1.45 µs | 23.3 µs | 66.8 µs | 108 µs | 194 µs |
| D462 | 1.67 µs | 42.4 µs | 125 µs | 239 µs | 406 µs |
| D616 | 1.13 µs | 62.1 µs | 172 µs | 308 µs | 708 µs |
| D924 | 1.67 µs | 126 µs | 273 µs | 747 µs | 1.54 ms |
| D1232 | 1.33 µs | 149 µs | 658 µs | 1.29 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.8 88.2,120.5 124.4,119.5 160.5,118.7 196.7,118.9 232.9,119.3 269.1,119.4 305.3,119.7 341.5,117.9 377.6,122.8 413.8,117.9 450.0,120.8 450.0,24.4 413.8,33.2 377.6,42.9 341.5,49.7 305.3,58.9 269.1,66.3 232.9,73.3 196.7,77.5 160.5,88.1 124.4,93.8 88.2,102.5 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.8 88.2,120.5 124.4,119.5 160.5,118.7 196.7,118.9 232.9,119.3 269.1,119.4 305.3,119.7 341.5,117.9 377.6,122.8 413.8,117.9 450.0,120.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,105.0 124.4,102.6 160.5,104.2 196.7,97.6 232.9,95.8 269.1,87.0 305.3,85.2 341.5,77.8 377.6,73.1 413.8,64.3 450.0,62.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.4 124.4,98.2 160.5,96.0 196.7,87.3 232.9,84.6 269.1,82.9 305.3,72.1 341.5,64.4 377.6,60.4 413.8,54.7 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,100.7 124.4,94.9 160.5,91.2 196.7,80.9 232.9,77.8 269.1,77.7 305.3,66.2 341.5,56.3 377.6,53.2 413.8,42.2 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,102.5 124.4,93.8 160.5,88.1 196.7,77.5 232.9,73.3 269.1,66.3 305.3,58.9 341.5,49.7 377.6,42.9 413.8,33.2 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.21 µs | 3.76 µs | 5.8 µs | 6.38 µs | 6.86 µs |
| D38 | 3.26 µs | 5.81 µs | 7.1 µs | 7.14 µs | 6.75 µs |
| D57 | 3.43 µs | 4.32 µs | 4.99 µs | 6.28 µs | 6.39 µs |
| D76 | 3.69 µs | 3.28 µs | 5.77 µs | 7.79 µs | 9.4 µs |
| D115 | 6.86 µs | 8.69 µs | 12.3 µs | 17 µs | 21.7 µs |
| D153 | 6.17 µs | 9.92 µs | 14.6 µs | 21.6 µs | 30.7 µs |
| D230 | 8.52 µs | 16.9 µs | 17.3 µs | 24.8 µs | 58.5 µs |
| D307 | 12.8 µs | 23.3 µs | 51.3 µs | 71.6 µs | 139 µs |
| D462 | 13.4 µs | 36.7 µs | 85.2 µs | 163 µs | 277 µs |
| D616 | 15.1 µs | 71.1 µs | 140 µs | 227 µs | 558 µs |
| D924 | 32.6 µs | 154 µs | 233 µs | 633 µs | 1.33 ms |
| D1232 | 31.6 µs | 181 µs | 672 µs | 1.21 ms | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.7 88.2,184.3 124.4,183.2 160.5,181.6 196.7,168.2 232.9,170.5 269.1,163.5 305.3,154.7 341.5,153.7 377.6,151.1 413.8,134.3 450.0,135.0 450.0,37.5 413.8,53.8 377.6,72.7 341.5,87.9 305.3,102.9 269.1,121.7 232.9,135.6 196.7,143.2 160.5,161.3 124.4,169.7 88.2,168.5 52.0,168.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.7 88.2,184.3 124.4,183.2 160.5,181.6 196.7,168.2 232.9,170.5 269.1,163.5 305.3,154.7 341.5,153.7 377.6,151.1 413.8,134.3 450.0,135.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,171.8 124.4,178.2 160.5,184.2 196.7,163.0 232.9,160.2 269.1,148.6 305.3,141.6 341.5,131.8 377.6,117.4 413.8,100.6 450.0,97.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.8 88.2,167.4 124.4,175.1 160.5,172.0 196.7,155.4 232.9,151.8 269.1,148.1 305.3,124.5 341.5,113.5 377.6,102.6 413.8,91.7 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,167.3 124.4,170.1 160.5,165.4 196.7,148.5 232.9,143.3 269.1,140.3 305.3,117.3 341.5,99.4 377.6,92.2 413.8,69.9 450.0,55.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.2 88.2,168.5 124.4,169.7 160.5,161.3 196.7,143.2 232.9,135.6 269.1,121.7 305.3,102.9 341.5,87.9 377.6,72.7 413.8,53.8 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.1 µs | 3.2 µs | 3.47 µs | 3.75 µs |
| D38 | 1.2 µs | 4.71 µs | 4.18 µs | 6.65 µs | 5.84 µs |
| D57 | 1.36 µs | 5.74 µs | 8.16 µs | 10.6 µs | 11.6 µs |
| D76 | 1.46 µs | 4.95 µs | 9.83 µs | 14.3 µs | 18.5 µs |
| D115 | 1.44 µs | 8.52 µs | 19.8 µs | 33 µs | 42.6 µs |
| D153 | 1.37 µs | 10 µs | 24.5 µs | 42.1 µs | 60.7 µs |
| D230 | 1.36 µs | 20.7 µs | 28.5 µs | 42.9 µs | 106 µs |
| D307 | 1.35 µs | 23.1 µs | 66.4 µs | 108 µs | 193 µs |
| D462 | 1.57 µs | 42.4 µs | 124 µs | 239 µs | 403 µs |
| D616 | 1 µs | 61.5 µs | 171 µs | 307 µs | 706 µs |
| D924 | 1.53 µs | 126 µs | 271 µs | 748 µs | 1.53 ms |
| D1232 | 1.26 µs | 148 µs | 660 µs | 1.29 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,122.0 124.4,120.5 160.5,119.6 196.7,119.8 232.9,120.4 269.1,120.5 305.3,120.6 341.5,118.7 377.6,124.3 413.8,119.0 450.0,121.4 450.0,24.4 413.8,33.3 377.6,42.9 341.5,49.9 305.3,59.0 269.1,66.4 232.9,73.3 196.7,77.7 160.5,88.1 124.4,93.9 88.2,102.4 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,122.0 124.4,120.5 160.5,119.6 196.7,119.8 232.9,120.4 269.1,120.5 305.3,120.6 341.5,118.7 377.6,124.3 413.8,119.0 450.0,121.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,105.1 124.4,102.6 160.5,104.4 196.7,97.7 232.9,95.7 269.1,86.7 305.3,85.3 341.5,77.8 377.6,73.2 413.8,64.3 450.0,62.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,106.5 124.4,98.2 160.5,95.9 196.7,87.2 232.9,84.6 269.1,82.7 305.3,72.2 341.5,64.4 377.6,60.5 413.8,54.8 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,100.8 124.4,95.0 160.5,91.3 196.7,80.9 232.9,77.9 269.1,77.6 305.3,66.2 341.5,56.4 377.6,53.2 413.8,42.2 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.4 124.4,93.9 160.5,88.1 196.7,77.7 232.9,73.3 269.1,66.4 305.3,59.0 341.5,49.9 377.6,42.9 413.8,33.3 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.65 µs | 1.81 µs | 2.52 µs | 2.81 µs | 2.96 µs |
| D38 | 1.63 µs | 2.53 µs | 3.09 µs | 3.17 µs | 2.95 µs |
| D57 | 4.48 µs | 5.27 µs | 7.26 µs | 8.89 µs | 9.73 µs |
| D76 | 5.06 µs | 4.5 µs | 8.62 µs | 11.6 µs | 14 µs |
| D115 | 9.42 µs | 13 µs | 18.9 µs | 24.8 µs | 33.7 µs |
| D153 | 8.36 µs | 14.8 µs | 21.7 µs | 33 µs | 43.8 µs |
| D230 | 11.2 µs | 26.3 µs | 26.7 µs | 38.1 µs | 82.6 µs |
| D307 | 16.4 µs | 32.2 µs | 73.3 µs | 96 µs | 176 µs |
| D462 | 17.6 µs | 55.9 µs | 120 µs | 197 µs | 336 µs |
| D616 | 19.6 µs | 104 µs | 185 µs | 286 µs | 671 µs |
| D924 | 42.7 µs | 230 µs | 309 µs | 726 µs | 1.46 ms |
| D1232 | 41.6 µs | 257 µs | 859 µs | 1.38 ms | 2.6 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.2 88.2,199.4 124.4,177.4 160.5,174.8 196.7,161.3 232.9,163.9 269.1,157.5 305.3,149.3 341.5,147.7 377.6,145.4 413.8,128.5 450.0,129.1 450.0,39.3 413.8,51.8 377.6,68.7 341.5,83.7 305.3,97.7 269.1,114.1 232.9,127.9 196.7,133.6 160.5,152.7 124.4,160.6 88.2,186.5 52.0,186.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.2 88.2,199.4 124.4,177.4 160.5,174.8 196.7,161.3 232.9,163.9 269.1,157.5 305.3,149.3 341.5,147.7 377.6,145.4 413.8,128.5 450.0,129.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.1 88.2,189.9 124.4,173.9 160.5,177.3 196.7,154.3 232.9,151.5 269.1,139.0 305.3,134.6 341.5,122.6 377.6,109.1 413.8,92.0 450.0,89.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.9 88.2,185.5 124.4,167.0 160.5,163.2 196.7,146.2 232.9,143.1 269.1,138.7 305.3,116.7 341.5,106.0 377.6,96.6 413.8,85.5 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,185.0 124.4,162.5 160.5,156.7 196.7,140.2 232.9,134.1 269.1,131.0 305.3,110.9 341.5,95.3 377.6,87.2 413.8,67.0 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.4 88.2,186.5 124.4,160.6 160.5,152.7 196.7,133.6 232.9,127.9 269.1,114.1 305.3,97.7 341.5,83.7 377.6,68.7 413.8,51.8 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.41 µs | 1.65 µs | 2.69 µs | 2.94 µs | 3.21 µs |
| D38 | 1.78 µs | 2.78 µs | 3.15 µs | 2.82 µs | 2.33 µs |
| D57 | 1.79 µs | 2.86 µs | 3.42 µs | 4.13 µs | 4.3 µs |
| D76 | 5.99 µs | 3.98 µs | 7.59 µs | 11.8 µs | 15.5 µs |
| D115 | 5.97 µs | 6.57 µs | 16.6 µs | 30.1 µs | 39.6 µs |
| D153 | 5.67 µs | 7.62 µs | 16.7 µs | 38.4 µs | 55.9 µs |
| D230 | 5.68 µs | 17 µs | 25.6 µs | 40.3 µs | 98.8 µs |
| D307 | 5.59 µs | 19.9 µs | 54.8 µs | 98.8 µs | 181 µs |
| D462 | 3.39 µs | 34.2 µs | 106 µs | 209 µs | 340 µs |
| D616 | 4.24 µs | 56.8 µs | 162 µs | 288 µs | 664 µs |
| D924 | 5.86 µs | 115 µs | 280 µs | 712 µs | 1.46 ms |
| D1232 | 4.44 µs | 138 µs | 620 µs | 1.25 ms | 3.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,197.5 124.4,197.3 160.5,171.1 196.7,171.2 232.9,172.3 269.1,172.3 305.3,172.6 341.5,183.5 377.6,178.6 413.8,171.6 450.0,177.6 450.0,36.1 413.8,51.8 377.6,68.9 341.5,83.4 305.3,97.2 269.1,110.3 232.9,122.6 196.7,130.1 160.5,150.5 124.4,178.3 88.2,191.7 52.0,184.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,197.5 124.4,197.3 160.5,171.1 196.7,171.2 232.9,172.3 269.1,172.3 305.3,172.6 341.5,183.5 377.6,178.6 413.8,171.6 450.0,177.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.2 88.2,187.8 124.4,187.2 160.5,180.0 196.7,169.1 232.9,165.9 269.1,148.5 305.3,145.1 341.5,133.3 377.6,122.3 413.8,106.9 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.5 88.2,185.1 124.4,183.3 160.5,166.0 196.7,149.0 232.9,148.9 269.1,139.6 305.3,123.1 341.5,108.8 377.6,99.5 413.8,87.6 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.6 88.2,187.5 124.4,179.2 160.5,156.3 196.7,136.1 232.9,130.8 269.1,129.7 305.3,110.3 341.5,94.0 377.6,87.0 413.8,67.4 450.0,55.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.7 88.2,191.7 124.4,178.3 160.5,150.5 196.7,130.1 232.9,122.6 269.1,110.3 305.3,97.2 341.5,83.4 377.6,68.9 413.8,51.8 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.87 ns | 1.82 µs | 2.85 µs | 3.15 µs | 3.43 µs |
| D38 | 4.04 ns | 2.85 µs | 3.56 µs | 3.56 µs | 3.39 µs |
| D57 | 450 ns | 5.69 µs | 6.98 µs | 8.94 µs | 9.37 µs |
| D76 | 531 ns | 4.26 µs | 8.14 µs | 11.4 µs | 14.1 µs |
| D115 | 1.08 µs | 12.1 µs | 18.2 µs | 25.6 µs | 34 µs |
| D153 | 915 ns | 14 µs | 22.1 µs | 33.7 µs | 50.2 µs |
| D230 | 1.29 µs | 24.4 µs | 26.4 µs | 42.7 µs | 99.9 µs |
| D307 | 1.96 µs | 35.8 µs | 85.1 µs | 123 µs | 249 µs |
| D462 | 2.12 µs | 57.6 µs | 146 µs | 292 µs | 502 µs |
| D616 | 2.45 µs | 115 µs | 247 µs | 403 µs | 1.01 ms |
| D924 | 5.19 µs | 259 µs | 414 µs | 1.17 ms | 2.49 ms |
| D1232 | 5.06 µs | 308 µs | 1.22 ms | 2.25 ms | 5.33 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.2 88.2,192.7 124.4,134.2 160.5,132.1 196.7,123.4 232.9,125.4 269.1,121.1 305.3,116.0 341.5,115.0 377.6,113.2 413.8,103.9 450.0,104.2 450.0,17.8 413.8,27.3 377.6,38.5 341.5,47.1 305.3,55.8 269.1,67.2 232.9,75.7 196.7,80.5 160.5,91.4 124.4,96.5 88.2,109.1 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.2 88.2,192.7 124.4,134.2 160.5,132.1 196.7,123.4 232.9,125.4 269.1,121.1 305.3,116.0 341.5,115.0 377.6,113.2 413.8,103.9 450.0,104.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,111.3 124.4,102.7 160.5,106.3 196.7,93.4 232.9,91.5 269.1,84.7 305.3,79.9 341.5,74.0 377.6,65.5 413.8,55.3 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,108.5 124.4,100.2 160.5,98.3 196.7,88.3 232.9,85.9 269.1,83.7 305.3,69.1 341.5,62.4 377.6,55.9 413.8,49.5 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,108.5 124.4,97.1 160.5,94.1 196.7,84.1 232.9,80.6 269.1,77.7 305.3,64.6 341.5,53.8 377.6,49.9 413.8,36.6 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,109.1 124.4,96.5 160.5,91.4 196.7,80.5 232.9,75.7 269.1,67.2 305.3,55.8 341.5,47.1 377.6,38.5 413.8,27.3 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.25 µs | 1.25 µs | 2.51 µs | 2.63 µs | 2.84 µs |
| D38 | 1.27 µs | 2.5 µs | 2.99 µs | 3.03 µs | 2.33 µs |
| D57 | 3.06 µs | 3.45 µs | 4.63 µs | 5.55 µs | 7.48 µs |
| D76 | 3.2 µs | 2.94 µs | 5.37 µs | 7.16 µs | 9.03 µs |
| D115 | 3.21 µs | 4.9 µs | 9.83 µs | 13.7 µs | 18.5 µs |
| D153 | 3.05 µs | 5.37 µs | 9.65 µs | 18.9 µs | 29.8 µs |
| D230 | 3.12 µs | 10.5 µs | 12 µs | 21.8 µs | 57.4 µs |
| D307 | 2.98 µs | 11.4 µs | 27.5 µs | 57.5 µs | 116 µs |
| D462 | 2.24 µs | 15.3 µs | 62.7 µs | 133 µs | 232 µs |
| D616 | 2.29 µs | 29.7 µs | 102 µs | 189 µs | 453 µs |
| D924 | 3.27 µs | 70.8 µs | 167 µs | 501 µs | 1.05 ms |
| D1232 | 2.54 µs | 86.3 µs | 424 µs | 897 µs | 2.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.1 88.2,204.8 124.4,185.7 160.5,184.7 196.7,184.7 232.9,185.8 269.1,185.3 305.3,186.3 341.5,192.4 377.6,192.0 413.8,184.3 450.0,189.7 450.0,41.9 413.8,58.8 377.6,77.2 341.5,91.7 305.3,106.7 269.1,122.1 232.9,136.3 196.7,146.6 160.5,162.2 124.4,166.3 88.2,191.7 52.0,187.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.1 88.2,204.8 124.4,185.7 160.5,184.7 196.7,184.7 232.9,185.8 269.1,185.3 305.3,186.3 341.5,192.4 377.6,192.0 413.8,184.3 450.0,189.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,205.2 88.2,190.1 124.4,183.1 160.5,186.6 196.7,175.5 232.9,173.5 269.1,159.0 305.3,157.1 341.5,150.8 377.6,136.4 413.8,117.5 450.0,113.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.0 88.2,186.2 124.4,176.7 160.5,173.5 196.7,160.4 232.9,160.8 269.1,156.1 305.3,138.0 341.5,120.1 377.6,109.6 413.8,98.9 450.0,78.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.0 88.2,185.9 124.4,172.8 160.5,167.3 196.7,153.1 232.9,146.2 269.1,143.1 305.3,122.0 341.5,103.7 377.6,96.2 413.8,75.0 450.0,62.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.3 88.2,191.7 124.4,166.3 160.5,162.2 196.7,146.6 232.9,136.3 269.1,122.1 305.3,106.7 341.5,91.7 377.6,77.2 413.8,58.8 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.89 µs | 3.31 µs | 3.53 µs | 3.76 µs |
| D38 | 1.55 µs | 3.32 µs | 3.92 µs | 3.84 µs | 3.09 µs |
| D57 | 5.61 µs | 5.42 µs | 7.09 µs | 8.01 µs | 8.51 µs |
| D76 | 6.19 µs | 4.65 µs | 7.6 µs | 10.3 µs | 12.1 µs |
| D115 | 13.5 µs | 12.4 µs | 12 µs | 22 µs | 26.6 µs |
| D153 | 5.81 µs | 7.81 µs | 15 µs | 23.3 µs | 36.2 µs |
| D230 | 5.87 µs | 14.6 µs | 15 µs | 25.4 µs | 68.4 µs |
| D307 | 5.99 µs | 15.2 µs | 54.5 µs | 68.5 µs | 125 µs |
| D462 | 6.58 µs | 23.5 µs | 82 µs | 158 µs | 251 µs |
| D616 | 4.63 µs | 36.7 µs | 115 µs | 200 µs | 449 µs |
| D924 | 6.39 µs | 82.8 µs | 181 µs | 491 µs | 907 µs |
| D1232 | 5.11 µs | 96.1 µs | 413 µs | 782 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.3 88.2,200.4 124.4,172.6 160.5,170.4 196.7,153.4 232.9,171.8 269.1,171.6 305.3,171.1 341.5,169.1 377.6,176.7 413.8,169.7 450.0,174.6 450.0,37.4 413.8,62.1 377.6,77.4 341.5,90.0 305.3,105.2 269.1,118.3 232.9,132.0 196.7,138.7 160.5,155.8 124.4,163.5 88.2,185.5 52.0,181.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.3 88.2,200.4 124.4,172.6 160.5,170.4 196.7,153.4 232.9,171.8 269.1,171.6 305.3,171.1 341.5,169.1 377.6,176.7 413.8,169.7 450.0,174.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,196.2 88.2,183.9 124.4,173.3 160.5,176.6 196.7,155.3 232.9,165.4 269.1,151.8 305.3,150.9 341.5,141.5 377.6,131.8 413.8,114.1 450.0,110.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.0 88.2,180.4 124.4,167.5 160.5,166.0 196.7,156.0 232.9,151.2 269.1,151.3 305.3,123.2 341.5,114.3 377.6,107.0 413.8,97.1 450.0,79.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.6 88.2,180.8 124.4,164.8 160.5,159.3 196.7,142.9 232.9,141.7 269.1,139.7 305.3,118.2 341.5,100.1 377.6,95.0 413.8,75.5 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.3 88.2,185.5 124.4,163.5 160.5,155.8 196.7,138.7 232.9,132.0 269.1,118.3 305.3,105.2 341.5,90.0 377.6,77.4 413.8,62.1 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.34 µs | 1.13 µs | 2.34 µs | 2.57 µs | 2.79 µs |
| D38 | 1.35 µs | 2.35 µs | 2.84 µs | 2.89 µs | 2.32 µs |
| D57 | 3.11 µs | 3.25 µs | 4.36 µs | 5.5 µs | 7.51 µs |
| D76 | 3.37 µs | 2.7 µs | 5.13 µs | 7.06 µs | 8.76 µs |
| D115 | 3.35 µs | 4.55 µs | 9.81 µs | 13.5 µs | 18.1 µs |
| D153 | 3.15 µs | 5.13 µs | 9.63 µs | 18 µs | 28.8 µs |
| D230 | 3.24 µs | 10.6 µs | 13.1 µs | 22.2 µs | 56.1 µs |
| D307 | 3.14 µs | 10.8 µs | 26.1 µs | 56.4 µs | 116 µs |
| D462 | 2.21 µs | 15.1 µs | 59.4 µs | 134 µs | 227 µs |
| D616 | 2.44 µs | 29 µs | 99.2 µs | 186 µs | 452 µs |
| D924 | 3.38 µs | 68 µs | 164 µs | 496 µs | 1.05 ms |
| D1232 | 2.69 µs | 85 µs | 423 µs | 886 µs | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.6 88.2,203.5 124.4,185.4 160.5,183.6 196.7,183.8 232.9,185.1 269.1,184.5 305.3,185.1 341.5,192.8 377.6,190.6 413.8,183.5 450.0,188.5 450.0,42.1 413.8,59.0 377.6,77.3 341.5,92.2 305.3,106.8 269.1,122.6 232.9,137.0 196.7,147.1 160.5,162.9 124.4,166.2 88.2,191.7 52.0,187.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.6 88.2,203.5 124.4,185.4 160.5,183.6 196.7,183.8 232.9,185.1 269.1,184.5 305.3,185.1 341.5,192.8 377.6,190.6 413.8,183.5 450.0,188.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,207.3 88.2,191.5 124.4,184.4 160.5,188.4 196.7,177.1 232.9,174.5 269.1,158.8 305.3,158.4 341.5,151.0 377.6,136.9 413.8,118.4 450.0,113.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,191.5 88.2,187.3 124.4,178.0 160.5,174.5 196.7,160.4 232.9,160.8 269.1,154.2 305.3,139.1 341.5,121.3 377.6,110.2 413.8,99.3 450.0,78.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.5 88.2,186.9 124.4,173.0 160.5,167.6 196.7,153.5 232.9,147.3 269.1,142.7 305.3,122.4 341.5,103.7 377.6,96.5 413.8,75.2 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.8 88.2,191.7 124.4,166.2 160.5,162.9 196.7,147.1 232.9,137.0 269.1,122.6 305.3,106.8 341.5,92.2 377.6,77.3 413.8,59.0 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.57 µs | 1.89 µs | 3.32 µs | 3.55 µs | 3.78 µs |
| D38 | 1.56 µs | 3.32 µs | 3.93 µs | 3.84 µs | 3.09 µs |
| D57 | 5.64 µs | 5.47 µs | 7.11 µs | 8.02 µs | 8.53 µs |
| D76 | 6.2 µs | 4.74 µs | 7.67 µs | 10.4 µs | 12.2 µs |
| D115 | 13.6 µs | 12.4 µs | 11.8 µs | 22 µs | 26.7 µs |
| D153 | 5.89 µs | 7.89 µs | 15 µs | 23.3 µs | 36.2 µs |
| D230 | 5.91 µs | 14.6 µs | 15 µs | 25.3 µs | 68.5 µs |
| D307 | 6.05 µs | 15.3 µs | 53.5 µs | 68.6 µs | 125 µs |
| D462 | 6.69 µs | 23.6 µs | 82.1 µs | 158 µs | 251 µs |
| D616 | 4.73 µs | 36.4 µs | 114 µs | 202 µs | 448 µs |
| D924 | 6.53 µs | 82.8 µs | 179 µs | 492 µs | 906 µs |
| D1232 | 5.2 µs | 97.4 µs | 412 µs | 779 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,200.2 88.2,200.3 124.4,172.4 160.5,170.4 196.7,153.4 232.9,171.5 269.1,171.4 305.3,170.9 341.5,168.7 377.6,176.2 413.8,169.2 450.0,174.2 450.0,37.4 413.8,62.2 377.6,77.4 341.5,90.0 305.3,105.1 269.1,118.2 232.9,132.1 196.7,138.6 160.5,155.7 124.4,163.5 88.2,185.5 52.0,181.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,200.2 88.2,200.3 124.4,172.4 160.5,170.4 196.7,153.4 232.9,171.5 269.1,171.4 305.3,170.9 341.5,168.7 377.6,176.2 413.8,169.2 450.0,174.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,196.2 88.2,183.9 124.4,173.1 160.5,176.2 196.7,155.4 232.9,165.2 269.1,151.7 305.3,150.8 341.5,141.4 377.6,132.0 413.8,114.1 450.0,110.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,180.3 124.4,167.4 160.5,165.8 196.7,156.3 232.9,151.2 269.1,151.1 305.3,123.6 341.5,114.3 377.6,107.2 413.8,97.3 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.5 88.2,180.8 124.4,164.8 160.5,159.2 196.7,142.9 232.9,141.6 269.1,139.9 305.3,118.2 341.5,100.1 377.6,94.7 413.8,75.4 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.2 88.2,185.5 124.4,163.5 160.5,155.7 196.7,138.6 232.9,132.1 269.1,118.2 305.3,105.1 341.5,90.0 377.6,77.4 413.8,62.2 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.49 µs | 2.27 µs | 3.99 µs | 4.24 µs | 4.6 µs |
| D38 | 2.49 µs | 3.98 µs | 4.76 µs | 4.86 µs | 4.15 µs |
| D57 | 3.94 µs | 4.34 µs | 5.83 µs | 7.24 µs | 7.6 µs |
| D76 | 4.3 µs | 3.54 µs | 6.89 µs | 8.98 µs | 10.9 µs |
| D115 | 4.28 µs | 6.05 µs | 12.7 µs | 16.4 µs | 21.7 µs |
| D153 | 3.97 µs | 6.76 µs | 12.1 µs | 22.3 µs | 33.8 µs |
| D230 | 4.12 µs | 13.4 µs | 14.1 µs | 25 µs | 64 µs |
| D307 | 3.96 µs | 13.7 µs | 31.7 µs | 64.1 µs | 126 µs |
| D462 | 2.88 µs | 18.4 µs | 68 µs | 147 µs | 249 µs |
| D616 | 3.06 µs | 33.7 µs | 111 µs | 205 µs | 488 µs |
| D924 | 4.24 µs | 77.1 µs | 183 µs | 534 µs | 1.11 ms |
| D1232 | 3.39 µs | 95.9 µs | 457 µs | 947 µs | 2.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.2 88.2,190.2 124.4,180.2 160.5,178.3 196.7,178.4 232.9,180.0 269.1,179.2 305.3,180.1 341.5,187.0 377.6,185.7 413.8,178.6 450.0,183.5 450.0,41.0 413.8,57.6 377.6,75.6 341.5,90.2 305.3,105.0 269.1,119.7 232.9,133.5 196.7,143.2 160.5,158.1 124.4,166.0 88.2,179.1 52.0,176.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.2 88.2,190.2 124.4,180.2 160.5,178.3 196.7,178.4 232.9,180.0 269.1,179.2 305.3,180.1 341.5,187.0 377.6,185.7 413.8,178.6 450.0,183.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,192.2 88.2,180.0 124.4,178.1 160.5,182.5 196.7,170.9 232.9,168.5 269.1,153.7 305.3,153.2 341.5,146.8 377.6,133.6 413.8,115.7 450.0,110.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,176.1 124.4,171.7 160.5,168.1 196.7,154.9 232.9,155.8 269.1,152.5 305.3,134.9 341.5,118.4 377.6,107.8 413.8,96.9 450.0,77.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,175.7 124.4,167.0 160.5,162.3 196.7,149.3 232.9,142.6 269.1,140.1 305.3,119.7 341.5,101.6 377.6,94.4 413.8,73.6 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,176.9 88.2,179.1 124.4,166.0 160.5,158.1 196.7,143.2 232.9,133.5 269.1,119.7 305.3,105.0 341.5,90.2 377.6,75.6 413.8,57.6 450.0,41.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.67 µs | 1.92 µs | 3.48 µs | 3.72 µs | 3.94 µs |
| D38 | 1.65 µs | 3.48 µs | 3.97 µs | 3.85 µs | 3.18 µs |
| D57 | 5.8 µs | 5.68 µs | 7.57 µs | 8.51 µs | 8.88 µs |
| D76 | 6.31 µs | 4.6 µs | 8 µs | 10.8 µs | 12.7 µs |
| D115 | 14.1 µs | 13 µs | 12.8 µs | 23.1 µs | 27.6 µs |
| D153 | 6.03 µs | 8.35 µs | 15.8 µs | 23.8 µs | 37.3 µs |
| D230 | 6.08 µs | 15.4 µs | 15.4 µs | 26.2 µs | 69.3 µs |
| D307 | 6.25 µs | 16.2 µs | 55.1 µs | 69.8 µs | 127 µs |
| D462 | 6.86 µs | 24.4 µs | 83.6 µs | 161 µs | 255 µs |
| D616 | 4.89 µs | 37.5 µs | 115 µs | 202 µs | 453 µs |
| D924 | 6.99 µs | 83.9 µs | 181 µs | 495 µs | 916 µs |
| D1232 | 5.58 µs | 98.2 µs | 417 µs | 785 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,199.1 124.4,171.8 160.5,170.0 196.7,152.6 232.9,171.0 269.1,170.8 305.3,170.2 341.5,168.2 377.6,175.5 413.8,167.8 450.0,172.7 450.0,37.3 413.8,61.9 377.6,77.2 341.5,89.7 305.3,104.8 269.1,117.9 232.9,131.4 196.7,137.9 160.5,154.9 124.4,162.6 88.2,184.9 52.0,180.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,199.1 124.4,171.8 160.5,170.0 196.7,152.6 232.9,171.0 269.1,170.8 305.3,170.2 341.5,168.2 377.6,175.5 413.8,167.8 450.0,172.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.8 88.2,182.9 124.4,172.3 160.5,176.9 196.7,154.2 232.9,163.9 269.1,150.7 305.3,149.6 341.5,140.7 377.6,131.3 413.8,113.8 450.0,110.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.9 88.2,180.1 124.4,166.0 160.5,164.8 196.7,154.6 232.9,150.1 269.1,150.7 305.3,122.9 341.5,113.9 377.6,107.1 413.8,97.1 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.5 88.2,180.7 124.4,163.5 160.5,158.4 196.7,141.8 232.9,141.1 269.1,139.1 305.3,117.8 341.5,99.7 377.6,94.7 413.8,75.3 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,184.9 124.4,162.6 160.5,154.9 196.7,137.9 232.9,131.4 269.1,117.9 305.3,104.8 341.5,89.7 377.6,77.2 413.8,61.9 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 134 ns | 148 ns | 156 ns | 160 ns | 163 ns |
| D38 | 137 ns | 159 ns | 165 ns | 157 ns | 124 ns |
| D57 | 237 ns | 319 ns | 330 ns | 326 ns | 347 ns |
| D76 | 258 ns | 202 ns | 336 ns | 416 ns | 436 ns |
| D115 | 575 ns | 565 ns | 712 ns | 787 ns | 892 ns |
| D153 | 466 ns | 611 ns | 719 ns | 849 ns | 1.01 µs |
| D230 | 605 ns | 910 ns | 748 ns | 737 ns | 1.53 µs |
| D307 | 1 µs | 1.13 µs | 1.72 µs | 1.84 µs | 2.85 µs |
| D462 | 1.15 µs | 1.45 µs | 2.21 µs | 3.09 µs | 4.23 µs |
| D616 | 899 ns | 1.88 µs | 2.48 µs | 3.08 µs | 6.1 µs |
| D924 | 1.7 µs | 2.86 µs | 3.06 µs | 6.2 µs | 10.3 µs |
| D1232 | 1.63 µs | 2.87 µs | 7.13 µs | 9.48 µs | 31.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.5 88.2,200.9 124.4,185.1 160.5,182.6 196.7,159.3 232.9,165.4 269.1,157.9 305.3,143.2 341.5,139.2 377.6,146.4 413.8,128.0 450.0,129.2 450.0,43.7 413.8,75.8 377.6,91.0 341.5,101.6 305.3,113.0 269.1,131.0 232.9,143.0 196.7,146.6 160.5,167.4 124.4,174.0 88.2,203.7 52.0,195.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.5 88.2,200.9 124.4,185.1 160.5,182.6 196.7,159.3 232.9,165.4 269.1,157.9 305.3,143.2 341.5,139.2 377.6,146.4 413.8,128.0 450.0,129.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,198.6 88.2,196.5 124.4,176.4 160.5,189.7 196.7,159.9 232.9,157.6 269.1,146.1 305.3,139.9 341.5,132.7 377.6,125.0 413.8,112.9 450.0,112.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,197.1 88.2,195.5 124.4,175.4 160.5,174.9 196.7,153.2 232.9,152.9 269.1,151.7 305.3,127.6 341.5,120.3 377.6,117.0 413.8,111.0 450.0,86.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,196.4 88.2,196.9 124.4,175.8 160.5,168.7 196.7,150.3 232.9,148.1 269.1,152.2 305.3,125.7 341.5,110.7 377.6,110.7 413.8,90.5 450.0,78.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,195.8 88.2,203.7 124.4,174.0 160.5,167.4 196.7,146.6 232.9,143.0 269.1,131.0 305.3,113.0 341.5,101.6 377.6,91.0 413.8,75.8 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 155 ns | 168 ns | 194 ns | 201 ns | 201 ns |
| D38 | 149 ns | 192 ns | 198 ns | 177 ns | 152 ns |
| D57 | 328 ns | 369 ns | 395 ns | 417 ns | 417 ns |
| D76 | 340 ns | 242 ns | 412 ns | 495 ns | 532 ns |
| D115 | 725 ns | 682 ns | 827 ns | 923 ns | 978 ns |
| D153 | 610 ns | 702 ns | 840 ns | 952 ns | 1.16 µs |
| D230 | 822 ns | 1.08 µs | 704 ns | 863 ns | 1.67 µs |
| D307 | 1.33 µs | 1.38 µs | 2.02 µs | 2.11 µs | 3.21 µs |
| D462 | 1.42 µs | 1.68 µs | 2.58 µs | 3.48 µs | 4.6 µs |
| D616 | 1.11 µs | 2.11 µs | 2.8 µs | 3.43 µs | 6.57 µs |
| D924 | 2.22 µs | 3.34 µs | 3.42 µs | 6.72 µs | 11.1 µs |
| D1232 | 2.18 µs | 3.31 µs | 7.78 µs | 10.2 µs | 32.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.3 88.2,198.4 124.4,175.6 160.5,174.5 196.7,152.7 232.9,157.7 269.1,149.0 305.3,135.0 341.5,133.3 377.6,140.3 413.8,120.3 450.0,120.8 450.0,42.8 413.8,73.6 377.6,88.8 341.5,99.1 305.3,109.6 269.1,128.4 232.9,139.0 196.7,144.0 160.5,161.6 124.4,168.7 88.2,197.9 52.0,189.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.3 88.2,198.4 124.4,175.6 160.5,174.5 196.7,152.7 232.9,157.7 269.1,149.0 305.3,135.0 341.5,133.3 377.6,140.3 413.8,120.3 450.0,120.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,195.0 88.2,191.1 124.4,172.2 160.5,184.4 196.7,154.4 232.9,153.6 269.1,141.0 305.3,134.0 341.5,128.4 377.6,121.7 413.8,108.4 450.0,108.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,190.8 88.2,190.2 124.4,170.3 160.5,169.0 196.7,148.8 232.9,148.4 269.1,153.5 305.3,123.0 341.5,115.8 377.6,113.5 413.8,107.7 450.0,83.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,193.4 124.4,168.6 160.5,163.7 196.7,145.7 232.9,144.8 269.1,147.6 305.3,121.8 341.5,107.2 377.6,107.7 413.8,88.2 450.0,76.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.8 88.2,197.9 124.4,168.7 160.5,161.6 196.7,144.0 232.9,139.0 269.1,128.4 305.3,109.6 341.5,99.1 377.6,88.8 413.8,73.6 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
