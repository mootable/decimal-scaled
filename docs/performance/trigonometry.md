# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.32 ns | 2 µs | 3.49 µs | 3.49 µs | 4.09 µs |
| D38 | 1.33 µs | 3.98 µs | 2.85 µs | 8.63 µs | 9.65 µs |
| D57 | 1.33 µs | 3.68 µs | 8.72 µs | 11.1 µs | 8.23 µs |
| D76 | 908 ns | 4.91 µs | 9.83 µs | 9.95 µs | 23 µs |
| D115 | 1.57 µs | 5.77 µs | 16.2 µs | 34.6 µs | 47.5 µs |
| D153 | 1.06 µs | 10.2 µs | 18.2 µs | 45.2 µs | 37 µs |
| D230 | 1.64 µs | 17.4 µs | 44.9 µs | 61.4 µs | 123 µs |
| D307 | 1.56 µs | 24.9 µs | 40.1 µs | 124 µs | 193 µs |
| D462 | 1.53 µs | 46.3 µs | 124 µs | 240 µs | 314 µs |
| D616 | 1.2 µs | 40.1 µs | 207 µs | 424 µs | 560 µs |
| D924 | 1.79 µs | 135 µs | 247 µs | 940 µs | 1.54 ms |
| D1232 | 2.08 µs | 175 µs | 663 µs | 1.17 ms | 3.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.3 88.2,120.7 124.4,120.7 160.5,125.5 196.7,118.7 232.9,123.6 269.1,118.1 305.3,118.8 341.5,119.0 377.6,122.1 413.8,117.1 450.0,115.2 450.0,23.1 413.8,33.2 377.6,45.8 341.5,53.0 305.3,59.0 269.1,64.5 232.9,79.5 196.7,76.4 160.5,85.4 124.4,98.1 88.2,96.2 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.3 88.2,120.7 124.4,120.7 160.5,125.5 196.7,118.7 232.9,123.6 269.1,118.1 305.3,118.8 341.5,119.0 377.6,122.1 413.8,117.1 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,107.2 124.4,108.1 160.5,104.5 196.7,102.5 232.9,95.5 269.1,88.9 305.3,84.4 341.5,76.7 377.6,78.5 413.8,63.4 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,111.3 124.4,97.4 160.5,95.9 196.7,89.8 232.9,88.3 269.1,77.1 305.3,78.5 341.5,64.5 377.6,58.1 413.8,55.9 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,97.5 124.4,94.4 160.5,95.8 196.7,80.3 232.9,77.0 269.1,73.2 305.3,64.4 341.5,56.3 377.6,49.2 413.8,39.3 450.0,36.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,96.2 124.4,98.1 160.5,85.4 196.7,76.4 232.9,79.5 269.1,64.5 305.3,59.0 341.5,53.0 377.6,45.8 413.8,33.2 450.0,23.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.54 µs | 3.59 µs | 6.07 µs | 6.4 µs | 7.12 µs |
| D38 | 3.21 µs | 5.09 µs | 5.55 µs | 8.76 µs | 9.68 µs |
| D57 | 3.14 µs | 2.51 µs | 5.43 µs | 6.58 µs | 4.45 µs |
| D76 | 1.98 µs | 3.25 µs | 5.79 µs | 5.41 µs | 11.8 µs |
| D115 | 6.62 µs | 6.43 µs | 10.2 µs | 18.5 µs | 23.9 µs |
| D153 | 4.67 µs | 9.61 µs | 11 µs | 24.2 µs | 17.4 µs |
| D230 | 8.96 µs | 13.7 µs | 28.3 µs | 37 µs | 69.4 µs |
| D307 | 13 µs | 26.2 µs | 30.2 µs | 85.1 µs | 139 µs |
| D462 | 12.9 µs | 38.8 µs | 85.6 µs | 161 µs | 223 µs |
| D616 | 16.1 µs | 44.4 µs | 168 µs | 316 µs | 440 µs |
| D924 | 33.8 µs | 159 µs | 209 µs | 799 µs | 1.34 ms |
| D1232 | 46.3 µs | 221 µs | 686 µs | 965 µs | 3.07 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.8 88.2,184.7 124.4,185.1 160.5,195.2 196.7,168.9 232.9,176.5 269.1,162.4 305.3,154.3 341.5,154.4 377.6,149.6 413.8,133.6 450.0,126.7 450.0,35.6 413.8,53.6 377.6,77.8 341.5,92.6 305.3,102.9 269.1,117.9 232.9,148.0 196.7,141.1 160.5,156.5 124.4,177.6 88.2,160.7 52.0,167.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.8 88.2,184.7 124.4,185.1 160.5,195.2 196.7,168.9 232.9,176.5 269.1,162.4 305.3,154.3 341.5,154.4 377.6,149.6 413.8,133.6 450.0,126.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.2 88.2,174.6 124.4,190.0 160.5,184.4 196.7,169.6 232.9,160.9 269.1,153.1 305.3,139.1 341.5,130.5 377.6,127.6 413.8,99.9 450.0,92.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,172.8 124.4,173.2 160.5,171.9 196.7,159.6 232.9,158.0 269.1,137.4 305.3,136.0 341.5,113.4 377.6,98.7 413.8,94.0 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,162.9 124.4,169.1 160.5,173.4 196.7,146.7 232.9,140.8 269.1,131.6 305.3,113.5 341.5,99.6 377.6,85.0 413.8,64.9 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,167.4 88.2,160.7 124.4,177.6 160.5,156.5 196.7,141.1 232.9,148.0 269.1,117.9 305.3,102.9 341.5,92.6 377.6,77.8 413.8,53.6 450.0,35.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.39 ns | 1.99 µs | 3.48 µs | 3.47 µs | 4.1 µs |
| D38 | 1.18 µs | 3.96 µs | 2.8 µs | 8.59 µs | 9.63 µs |
| D57 | 1.22 µs | 3.67 µs | 8.67 µs | 11.1 µs | 8.2 µs |
| D76 | 859 ns | 5.04 µs | 9.83 µs | 9.9 µs | 22.9 µs |
| D115 | 1.44 µs | 5.74 µs | 16.1 µs | 34.9 µs | 45.4 µs |
| D153 | 991 ns | 10.1 µs | 18.5 µs | 45.6 µs | 37.1 µs |
| D230 | 1.53 µs | 17.3 µs | 44.9 µs | 61.8 µs | 123 µs |
| D307 | 1.45 µs | 24.6 µs | 40 µs | 124 µs | 191 µs |
| D462 | 1.42 µs | 45 µs | 125 µs | 241 µs | 312 µs |
| D616 | 1.11 µs | 40.2 µs | 209 µs | 422 µs | 556 µs |
| D924 | 1.72 µs | 133 µs | 244 µs | 948 µs | 1.54 ms |
| D1232 | 1.94 µs | 176 µs | 663 µs | 1.18 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.9 88.2,122.2 124.4,121.8 160.5,126.2 196.7,119.8 232.9,124.4 269.1,119.0 305.3,119.7 341.5,120.0 377.6,123.0 413.8,117.6 450.0,116.1 450.0,23.0 413.8,33.2 377.6,45.8 341.5,53.0 305.3,59.1 269.1,64.5 232.9,79.5 196.7,76.9 160.5,85.4 124.4,98.2 88.2,96.2 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.9 88.2,122.2 124.4,121.8 160.5,126.2 196.7,119.8 232.9,124.4 269.1,119.0 305.3,119.7 341.5,120.0 377.6,123.0 413.8,117.6 450.0,116.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,107.2 124.4,108.2 160.5,104.2 196.7,102.6 232.9,95.6 269.1,88.9 305.3,84.5 341.5,77.1 377.6,78.4 413.8,63.6 450.0,60.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,111.5 124.4,97.5 160.5,95.9 196.7,89.8 232.9,88.1 269.1,77.1 305.3,78.5 341.5,64.4 377.6,58.0 413.8,56.1 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,97.6 124.4,94.4 160.5,95.8 196.7,80.2 232.9,76.9 269.1,73.1 305.3,64.4 341.5,56.3 377.6,49.3 413.8,39.2 450.0,36.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,96.2 124.4,98.2 160.5,85.4 196.7,76.9 232.9,79.5 269.1,64.5 305.3,59.1 341.5,53.0 377.6,45.8 413.8,33.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.89 ns | 1.71 µs | 2.65 µs | 2.81 µs | 3.12 µs |
| D38 | 4.04 ns | 2.2 µs | 2.39 µs | 3.92 µs | 4.26 µs |
| D57 | 2.01 ns | 3.23 µs | 7.83 µs | 9.54 µs | 6.96 µs |
| D76 | 1.62 ns | 4.43 µs | 8.6 µs | 8.22 µs | 17.5 µs |
| D115 | 8.17 ns | 9.49 µs | 15.5 µs | 27 µs | 36.5 µs |
| D153 | 7.69 ns | 14.8 µs | 16.7 µs | 36.4 µs | 26.9 µs |
| D230 | 19.5 ns | 21.4 µs | 44.9 µs | 55.1 µs | 95.9 µs |
| D307 | 28.5 ns | 37.7 µs | 43.2 µs | 116 µs | 177 µs |
| D462 | 42.9 ns | 59.2 µs | 119 µs | 197 µs | 270 µs |
| D616 | 46.5 ns | 64.7 µs | 225 µs | 402 µs | 526 µs |
| D924 | 36.5 ns | 240 µs | 286 µs | 930 µs | 1.47 ms |
| D1232 | 73.5 ns | 308 µs | 869 µs | 1.02 ms | 3.1 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,196.8 88.2,192.7 124.4,201.3 160.5,204.0 196.7,183.9 232.9,184.7 269.1,173.2 305.3,168.4 341.5,163.4 377.6,162.4 413.8,165.4 450.0,156.7 450.0,24.5 413.8,33.8 377.6,46.5 341.5,54.8 305.3,60.1 269.1,67.7 232.9,83.4 196.7,79.6 160.5,88.8 124.4,100.2 88.2,106.3 52.0,110.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,196.8 88.2,192.7 124.4,201.3 160.5,204.0 196.7,183.9 232.9,184.7 269.1,173.2 305.3,168.4 341.5,163.4 377.6,162.4 413.8,165.4 450.0,156.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.7 88.2,114.5 124.4,109.7 160.5,105.8 196.7,96.4 232.9,90.9 269.1,86.3 305.3,79.2 341.5,73.6 377.6,72.5 413.8,56.3 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,113.5 124.4,98.8 160.5,97.6 196.7,90.2 232.9,89.3 269.1,77.1 305.3,77.6 341.5,65.0 377.6,57.1 413.8,54.1 450.0,40.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,107.3 124.4,96.3 160.5,98.1 196.7,83.4 232.9,79.7 269.1,74.5 305.3,65.3 341.5,58.7 377.6,49.9 413.8,39.5 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,106.3 124.4,100.2 160.5,88.8 196.7,79.6 232.9,83.4 269.1,67.7 305.3,60.1 341.5,54.8 377.6,46.5 413.8,33.8 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.36 ns | 1.57 µs | 2.91 µs | 2.94 µs | 3.5 µs |
| D38 | 2.8 ns | 2.17 µs | 2.14 µs | 3.73 µs | 4.22 µs |
| D57 | 1.44 ns | 1.65 µs | 3.7 µs | 4.33 µs | 3.03 µs |
| D76 | 354 ns | 4.24 µs | 7.77 µs | 8.13 µs | 19.2 µs |
| D115 | 588 ns | 4.41 µs | 13.3 µs | 33.2 µs | 42 µs |
| D153 | 395 ns | 7.86 µs | 11.9 µs | 40.7 µs | 33.8 µs |
| D230 | 662 ns | 14.7 µs | 40.6 µs | 57.3 µs | 115 µs |
| D307 | 615 ns | 21 µs | 33.6 µs | 115 µs | 178 µs |
| D462 | 599 ns | 37.6 µs | 105 µs | 212 µs | 270 µs |
| D616 | 462 ns | 37.9 µs | 193 µs | 395 µs | 527 µs |
| D924 | 835 ns | 125 µs | 225 µs | 888 µs | 1.46 ms |
| D1232 | 1.02 µs | 165 µs | 623 µs | 983 µs | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.0 88.2,197.2 124.4,205.5 160.5,137.2 196.7,130.9 232.9,135.8 269.1,129.4 305.3,130.3 341.5,130.7 377.6,133.9 413.8,126.5 450.0,124.1 450.0,23.6 413.8,33.9 377.6,46.5 341.5,54.8 305.3,60.0 269.1,65.4 232.9,80.6 196.7,77.9 160.5,87.6 124.4,110.6 88.2,106.4 52.0,108.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.0 88.2,197.2 124.4,205.5 160.5,137.2 196.7,130.9 232.9,135.8 269.1,129.4 305.3,130.3 341.5,130.7 377.6,133.9 413.8,126.5 450.0,124.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,114.7 124.4,118.0 160.5,106.4 196.7,105.9 232.9,98.7 269.1,90.9 305.3,86.5 341.5,79.3 377.6,79.2 413.8,64.4 450.0,60.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,114.8 124.4,108.1 160.5,98.8 196.7,92.2 232.9,93.5 269.1,78.3 305.3,80.7 341.5,66.5 377.6,59.0 413.8,57.1 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,108.0 124.4,106.1 160.5,98.3 196.7,80.8 232.9,78.3 269.1,74.0 305.3,65.4 341.5,57.8 377.6,50.1 413.8,40.0 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.4 124.4,110.6 160.5,87.6 196.7,77.9 232.9,80.6 269.1,65.4 305.3,60.0 341.5,54.8 377.6,46.5 413.8,33.9 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.73 µs | 2.98 µs | 3.19 µs | 3.59 µs |
| D38 | 4.04 ns | 2.49 µs | 2.78 µs | 4.35 µs | 4.86 µs |
| D57 | 369 ns | 3.26 µs | 7.52 µs | 9.28 µs | 6.7 µs |
| D76 | 262 ns | 4.34 µs | 8.21 µs | 7.82 µs | 17.8 µs |
| D115 | 1.05 µs | 8.77 µs | 14.5 µs | 27.8 µs | 37.3 µs |
| D153 | 807 ns | 13.4 µs | 16.2 µs | 38 µs | 28.1 µs |
| D230 | 1.42 µs | 20.3 µs | 44.2 µs | 62.4 µs | 118 µs |
| D307 | 2.08 µs | 40 µs | 49.1 µs | 146 µs | 244 µs |
| D462 | 2.03 µs | 61.7 µs | 147 µs | 288 µs | 398 µs |
| D616 | 2.44 µs | 72.2 µs | 293 µs | 558 µs | 810 µs |
| D924 | 5.23 µs | 267 µs | 371 µs | 1.47 ms | 2.5 ms |
| D1232 | 7.25 µs | 370 µs | 1.23 ms | 1.82 ms | 5.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,192.7 124.4,136.7 160.5,140.9 196.7,123.6 232.9,127.0 269.1,119.9 305.3,115.2 341.5,115.5 377.6,113.2 413.8,103.7 450.0,99.7 450.0,16.8 413.8,27.2 377.6,41.2 341.5,50.0 305.3,56.1 269.1,65.0 232.9,82.9 196.7,79.4 160.5,88.6 124.4,100.7 88.2,104.7 52.0,108.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,192.7 124.4,136.7 160.5,140.9 196.7,123.6 232.9,127.0 269.1,119.9 305.3,115.2 341.5,115.5 377.6,113.2 413.8,103.7 450.0,99.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,113.0 124.4,109.6 160.5,106.1 196.7,97.3 232.9,92.1 269.1,86.9 305.3,78.5 341.5,73.1 377.6,71.2 413.8,55.0 450.0,50.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,111.6 124.4,99.3 160.5,98.2 196.7,91.1 232.9,89.7 269.1,77.3 305.3,76.0 341.5,62.4 377.6,53.8 413.8,50.9 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,106.0 124.4,96.6 160.5,98.8 196.7,83.0 232.9,79.2 269.1,73.0 305.3,62.4 341.5,54.0 377.6,45.8 413.8,33.8 450.0,31.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.7 124.4,100.7 160.5,88.6 196.7,79.4 232.9,82.9 269.1,65.0 305.3,56.1 341.5,50.0 377.6,41.2 413.8,27.2 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.55 ns | 1.18 µs | 2.7 µs | 2.65 µs | 3.05 µs |
| D38 | 4.99 ns | 1.97 µs | 2.06 µs | 3.74 µs | 3.83 µs |
| D57 | 2.3 ns | 2.2 µs | 5.04 µs | 6.01 µs | 5.23 µs |
| D76 | 1.37 ns | 2.85 µs | 5.29 µs | 4.87 µs | 11.4 µs |
| D115 | 13 ns | 3.18 µs | 7.85 µs | 14.3 µs | 19.7 µs |
| D153 | 14 ns | 5.4 µs | 7.07 µs | 19 µs | 17.6 µs |
| D230 | 40.7 ns | 8.9 µs | 19.9 µs | 32.8 µs | 70.7 µs |
| D307 | 73.2 ns | 12.4 µs | 16.3 µs | 70.4 µs | 117 µs |
| D462 | 107 ns | 16.1 µs | 61 µs | 135 µs | 185 µs |
| D616 | 118 ns | 19.5 µs | 123 µs | 256 µs | 363 µs |
| D924 | 189 ns | 74.3 µs | 140 µs | 615 µs | 1.05 ms |
| D1232 | 353 ns | 102 µs | 430 µs | 750 µs | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,190.1 124.4,199.6 160.5,206.1 196.7,178.1 232.9,177.3 269.1,164.0 305.3,156.7 341.5,152.0 377.6,150.8 413.8,144.9 450.0,137.2 450.0,27.5 413.8,38.0 377.6,51.2 341.5,59.5 305.3,65.2 269.1,71.4 232.9,88.7 196.7,87.3 160.5,94.1 124.4,103.8 88.2,107.6 52.0,110.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,190.1 124.4,199.6 160.5,206.1 196.7,178.1 232.9,177.3 269.1,164.0 305.3,156.7 341.5,152.0 377.6,150.8 413.8,144.9 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,115.9 124.4,114.5 160.5,111.3 196.7,109.9 232.9,103.4 269.1,97.2 305.3,93.1 341.5,89.8 377.6,87.4 413.8,70.8 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,115.3 124.4,104.2 160.5,103.6 196.7,98.7 232.9,100.0 269.1,87.2 305.3,89.6 341.5,73.3 377.6,64.6 413.8,63.0 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.2 88.2,107.9 124.4,102.0 160.5,104.6 196.7,91.3 232.9,87.7 269.1,81.0 305.3,71.5 341.5,63.4 377.6,55.5 413.8,44.6 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,107.6 124.4,103.8 160.5,94.1 196.7,87.3 232.9,88.7 269.1,71.4 305.3,65.2 341.5,59.5 377.6,51.2 413.8,38.0 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.71 µs | 3.61 µs | 3.58 µs | 4.11 µs |
| D38 | 4.04 ns | 2.76 µs | 2.83 µs | 4.82 µs | 4.87 µs |
| D57 | 2.61 ns | 3.43 µs | 7.54 µs | 8.44 µs | 6.14 µs |
| D76 | 1.69 ns | 4.35 µs | 7.57 µs | 7.23 µs | 15.3 µs |
| D115 | 10.9 ns | 8.99 µs | 9.74 µs | 23.8 µs | 28.9 µs |
| D153 | 13.2 ns | 7.88 µs | 11.6 µs | 24.5 µs | 20.7 µs |
| D230 | 40.8 ns | 12.4 µs | 24.5 µs | 38.2 µs | 80.7 µs |
| D307 | 75.8 ns | 16.4 µs | 30.9 µs | 80.8 µs | 125 µs |
| D462 | 108 ns | 25.5 µs | 81.4 µs | 158 µs | 198 µs |
| D616 | 119 ns | 26.6 µs | 135 µs | 269 µs | 351 µs |
| D924 | 201 ns | 87.4 µs | 147 µs | 608 µs | 923 µs |
| D1232 | 357 ns | 115 µs | 418 µs | 649 µs | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,192.7 124.4,198.1 160.5,203.5 196.7,180.4 232.9,178.0 269.1,164.0 305.3,156.3 341.5,152.0 377.6,150.7 413.8,144.2 450.0,137.1 450.0,25.7 413.8,39.6 377.6,51.6 341.5,58.7 305.3,64.4 269.1,69.8 232.9,86.7 196.7,82.6 160.5,90.4 124.4,101.8 88.2,104.6 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,192.7 124.4,198.1 160.5,203.5 196.7,180.4 232.9,178.0 269.1,164.0 305.3,156.3 341.5,152.0 377.6,150.7 413.8,144.2 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,111.7 124.4,109.0 160.5,106.0 196.7,97.0 232.9,98.7 269.1,93.0 305.3,89.6 341.5,84.1 377.6,83.6 413.8,68.8 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,111.4 124.4,99.2 160.5,99.2 196.7,96.0 232.9,93.9 269.1,84.6 305.3,81.7 341.5,69.7 377.6,63.4 413.8,62.3 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,104.8 124.4,97.8 160.5,99.7 196.7,85.0 232.9,84.6 269.1,79.1 305.3,69.8 341.5,61.5 377.6,54.9 413.8,44.7 450.0,43.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,104.6 124.4,101.8 160.5,90.4 196.7,82.6 232.9,86.7 269.1,69.8 305.3,64.4 341.5,58.7 377.6,51.6 413.8,39.6 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.33 ns | 1.08 µs | 2.53 µs | 2.61 µs | 3 µs |
| D38 | 4.36 ns | 1.82 µs | 1.94 µs | 3.58 µs | 3.79 µs |
| D57 | 2.02 ns | 2.05 µs | 4.97 µs | 5.9 µs | 5.23 µs |
| D76 | 1.43 ns | 2.68 µs | 5.02 µs | 4.79 µs | 11 µs |
| D115 | 13 ns | 3.06 µs | 7.72 µs | 14.2 µs | 19.8 µs |
| D153 | 13.9 ns | 5.1 µs | 6.93 µs | 19.3 µs | 17 µs |
| D230 | 40.7 ns | 8.92 µs | 19.9 µs | 33 µs | 66.8 µs |
| D307 | 67.1 ns | 11.9 µs | 15.7 µs | 66.4 µs | 114 µs |
| D462 | 98.6 ns | 16.4 µs | 58.6 µs | 133 µs | 184 µs |
| D616 | 115 ns | 19.2 µs | 121 µs | 252 µs | 358 µs |
| D924 | 197 ns | 71.9 µs | 139 µs | 608 µs | 1.05 ms |
| D1232 | 346 ns | 103 µs | 428 µs | 724 µs | 2.43 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.1 88.2,191.7 124.4,201.3 160.5,205.6 196.7,178.1 232.9,177.4 269.1,164.0 305.3,157.8 341.5,153.0 377.6,151.1 413.8,144.4 450.0,137.5 450.0,27.5 413.8,38.0 377.6,51.3 341.5,59.6 305.3,65.5 269.1,72.1 232.9,89.1 196.7,87.2 160.5,94.5 124.4,103.8 88.2,107.7 52.0,110.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.1 88.2,191.7 124.4,201.3 160.5,205.6 196.7,178.1 232.9,177.4 269.1,164.0 305.3,157.8 341.5,153.0 377.6,151.1 413.8,144.4 450.0,137.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,116.9 124.4,115.4 160.5,112.1 196.7,110.4 232.9,104.1 269.1,97.1 305.3,93.5 341.5,89.6 377.6,87.6 413.8,71.2 450.0,66.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,116.1 124.4,104.4 160.5,104.3 196.7,98.9 232.9,100.3 269.1,87.2 305.3,90.1 341.5,73.8 377.6,64.7 413.8,63.1 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,108.5 124.4,102.3 160.5,104.9 196.7,91.4 232.9,87.6 269.1,80.9 305.3,72.2 341.5,63.6 377.6,55.7 413.8,44.7 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.6 88.2,107.7 124.4,103.8 160.5,94.5 196.7,87.2 232.9,89.1 269.1,72.1 305.3,65.5 341.5,59.6 377.6,51.3 413.8,38.0 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.38 ns | 1.73 µs | 3.62 µs | 3.6 µs | 4.12 µs |
| D38 | 4.04 ns | 2.77 µs | 2.83 µs | 4.83 µs | 4.88 µs |
| D57 | 8.37 ns | 3.43 µs | 7.55 µs | 8.48 µs | 6.15 µs |
| D76 | 6.93 ns | 4.46 µs | 7.6 µs | 7.23 µs | 15.4 µs |
| D115 | 11.3 ns | 8.98 µs | 9.62 µs | 23.7 µs | 28.7 µs |
| D153 | 13 ns | 7.95 µs | 11.3 µs | 24.5 µs | 20.7 µs |
| D230 | 39.9 ns | 12.6 µs | 24.6 µs | 38.3 µs | 80.9 µs |
| D307 | 65.2 ns | 16.4 µs | 30.6 µs | 80.9 µs | 125 µs |
| D462 | 101 ns | 25.5 µs | 81.6 µs | 158 µs | 199 µs |
| D616 | 113 ns | 24 µs | 135 µs | 269 µs | 353 µs |
| D924 | 199 ns | 88 µs | 148 µs | 607 µs | 922 µs |
| D1232 | 377 ns | 116 µs | 419 µs | 643 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.2 88.2,192.7 124.4,183.6 160.5,186.0 196.7,180.0 232.9,178.2 269.1,164.3 305.3,158.2 341.5,152.7 377.6,151.4 413.8,144.3 450.0,136.4 450.0,25.6 413.8,39.6 377.6,51.5 341.5,58.6 305.3,64.4 269.1,69.8 232.9,86.7 196.7,82.6 160.5,90.4 124.4,101.7 88.2,104.6 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.2 88.2,192.7 124.4,183.6 160.5,186.0 196.7,180.0 232.9,178.2 269.1,164.3 305.3,158.2 341.5,152.7 377.6,151.4 413.8,144.3 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,111.7 124.4,109.0 160.5,105.7 196.7,97.1 232.9,98.6 269.1,92.8 305.3,89.6 341.5,84.1 377.6,84.8 413.8,68.7 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,111.4 124.4,99.2 160.5,99.1 196.7,96.2 232.9,94.2 269.1,84.6 305.3,81.8 341.5,69.7 377.6,63.4 413.8,62.3 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.7 124.4,97.8 160.5,99.7 196.7,85.0 232.9,84.6 269.1,79.0 305.3,69.8 341.5,61.4 377.6,54.9 413.8,44.8 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,104.6 124.4,101.7 160.5,90.4 196.7,82.6 232.9,86.7 269.1,69.8 305.3,64.4 341.5,58.6 377.6,51.5 413.8,39.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.27 ns | 2.12 µs | 4.29 µs | 4.28 µs | 4.94 µs |
| D38 | 4.36 ns | 3.37 µs | 3.57 µs | 5.93 µs | 6.2 µs |
| D57 | 2.6 ns | 2.72 µs | 6.33 µs | 7.82 µs | 5.22 µs |
| D76 | 2.01 ns | 3.52 µs | 6.91 µs | 6.18 µs | 13.8 µs |
| D115 | 10.5 ns | 4.04 µs | 9.82 µs | 17.3 µs | 24.1 µs |
| D153 | 13.5 ns | 7.11 µs | 9.06 µs | 23.7 µs | 20.1 µs |
| D230 | 41.9 ns | 11.4 µs | 23.7 µs | 37.3 µs | 76.2 µs |
| D307 | 62.8 ns | 14.2 µs | 18.3 µs | 75.5 µs | 126 µs |
| D462 | 94 ns | 19.9 µs | 66.8 µs | 146 µs | 199 µs |
| D616 | 103 ns | 22.3 µs | 135 µs | 276 µs | 386 µs |
| D924 | 157 ns | 81.7 µs | 153 µs | 661 µs | 1.12 ms |
| D1232 | 365 ns | 115 µs | 463 µs | 915 µs | 2.58 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.3 88.2,191.7 124.4,198.1 160.5,201.3 196.7,180.8 232.9,177.7 269.1,163.6 305.3,158.6 341.5,153.6 377.6,152.5 413.8,147.2 450.0,136.8 450.0,26.8 413.8,37.2 377.6,50.4 341.5,58.6 305.3,64.3 269.1,70.5 232.9,87.0 196.7,84.8 160.5,91.7 124.4,103.8 88.2,101.7 52.0,104.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.3 88.2,191.7 124.4,198.1 160.5,201.3 196.7,180.8 232.9,177.7 269.1,163.6 305.3,158.6 341.5,153.6 377.6,152.5 413.8,147.2 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,109.2 124.4,111.9 160.5,108.7 196.7,107.0 232.9,100.0 269.1,94.1 305.3,91.3 341.5,87.2 377.6,85.8 413.8,69.7 450.0,65.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,108.5 124.4,101.4 160.5,100.3 196.7,95.9 232.9,96.9 269.1,85.0 305.3,88.2 341.5,72.1 377.6,63.4 413.8,61.9 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,102.2 124.4,98.8 160.5,101.7 196.7,88.9 232.9,85.0 269.1,79.4 305.3,70.6 341.5,62.4 377.6,54.5 413.8,43.7 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.5 88.2,101.7 124.4,103.8 160.5,91.7 196.7,84.8 232.9,87.0 269.1,70.5 305.3,64.3 341.5,58.6 377.6,50.4 413.8,37.2 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.39 ns | 1.76 µs | 3.81 µs | 3.75 µs | 4.33 µs |
| D38 | 4.04 ns | 2.93 µs | 2.85 µs | 4.85 µs | 4.92 µs |
| D57 | 2.35 µs | 3.53 µs | 7.9 µs | 8.84 µs | 6.37 µs |
| D76 | 1.66 µs | 4.58 µs | 7.97 µs | 7.52 µs | 15.9 µs |
| D115 | 5.7 µs | 9.48 µs | 10.4 µs | 24.5 µs | 30.3 µs |
| D153 | 2.04 µs | 8.38 µs | 11.9 µs | 25.3 µs | 21.4 µs |
| D230 | 3.18 µs | 12.8 µs | 25.3 µs | 39.1 µs | 82 µs |
| D307 | 3.2 µs | 17.2 µs | 31.9 µs | 82.4 µs | 127 µs |
| D462 | 3.12 µs | 26.1 µs | 82.9 µs | 160 µs | 200 µs |
| D616 | 2.42 µs | 24.4 µs | 137 µs | 272 µs | 358 µs |
| D924 | 3.79 µs | 89.2 µs | 150 µs | 614 µs | 931 µs |
| D1232 | 4.46 µs | 119 µs | 422 µs | 657 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,199.2 88.2,192.7 124.4,113.7 160.5,118.0 196.7,102.7 232.9,115.4 269.1,109.9 305.3,109.9 341.5,110.2 377.6,113.3 413.8,107.8 450.0,105.7 450.0,25.6 413.8,39.5 377.6,51.3 341.5,58.6 305.3,64.1 269.1,69.6 232.9,86.3 196.7,82.0 160.5,90.0 124.4,101.3 88.2,104.5 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,199.2 88.2,192.7 124.4,113.7 160.5,118.0 196.7,102.7 232.9,115.4 269.1,109.9 305.3,109.9 341.5,110.2 377.6,113.3 413.8,107.8 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,110.9 124.4,108.6 160.5,105.4 196.7,96.4 232.9,97.9 269.1,92.7 305.3,89.0 341.5,83.8 377.6,84.6 413.8,68.6 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,111.3 124.4,98.6 160.5,98.5 196.7,95.2 232.9,93.6 269.1,84.2 305.3,81.3 341.5,69.5 377.6,63.3 413.8,62.1 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,104.7 124.4,97.2 160.5,99.3 196.7,84.6 232.9,84.2 269.1,78.8 305.3,69.5 341.5,61.3 377.6,54.7 413.8,44.6 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,104.5 124.4,101.3 160.5,90.0 196.7,82.0 232.9,86.3 269.1,69.6 305.3,64.1 341.5,58.6 377.6,51.3 413.8,39.5 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.25 ns | 137 ns | 173 ns | 162 ns | 178 ns |
| D38 | 3.42 ns | 138 ns | 121 ns | 202 ns | 187 ns |
| D57 | 148 ns | 172 ns | 350 ns | 351 ns | 244 ns |
| D76 | 96.3 ns | 210 ns | 331 ns | 284 ns | 562 ns |
| D115 | 470 ns | 383 ns | 542 ns | 895 ns | 993 ns |
| D153 | 265 ns | 579 ns | 485 ns | 975 ns | 587 ns |
| D230 | 569 ns | 745 ns | 1.11 µs | 1.17 µs | 1.86 µs |
| D307 | 903 ns | 1.28 µs | 990 ns | 2.19 µs | 2.87 µs |
| D462 | 946 ns | 1.6 µs | 2.22 µs | 3.11 µs | 3.33 µs |
| D616 | 779 ns | 1.21 µs | 2.94 µs | 4.2 µs | 4.83 µs |
| D924 | 1.66 µs | 3.03 µs | 2.56 µs | 7.66 µs | 10.4 µs |
| D1232 | 2.49 µs | 3.36 µs | 7.22 µs | 7.59 µs | 30.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,188.6 124.4,123.2 160.5,130.6 196.7,103.1 232.9,113.1 269.1,99.8 305.3,91.8 341.5,91.0 377.6,94.3 413.8,81.2 450.0,74.2 450.0,30.7 413.8,49.2 377.6,62.6 341.5,69.1 305.3,71.7 269.1,79.2 232.9,99.2 196.7,90.1 160.5,100.0 124.4,114.5 88.2,119.1 52.0,119.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,188.6 124.4,123.2 160.5,130.6 196.7,103.1 232.9,113.1 269.1,99.8 305.3,91.8 341.5,91.0 377.6,94.3 413.8,81.2 450.0,74.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,124.4 124.4,120.6 160.5,117.1 196.7,106.7 232.9,99.5 269.1,95.1 305.3,85.7 341.5,81.8 377.6,86.7 413.8,70.7 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.5 88.2,126.7 124.4,108.3 160.5,109.2 196.7,100.7 232.9,102.6 269.1,88.3 305.3,90.2 341.5,76.1 377.6,71.3 413.8,73.7 450.0,55.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,117.8 124.4,108.2 160.5,111.9 196.7,91.9 232.9,90.4 269.1,87.3 305.3,76.4 341.5,70.3 377.6,65.1 413.8,54.6 450.0,54.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.9 88.2,119.1 124.4,114.5 160.5,100.0 196.7,90.1 232.9,99.2 269.1,79.2 305.3,71.7 341.5,69.1 377.6,62.6 413.8,49.2 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.45 ns | 155 ns | 211 ns | 203 ns | 218 ns |
| D38 | 0.935 ns | 181 ns | 166 ns | 214 ns | 201 ns |
| D57 | 201 ns | 197 ns | 411 ns | 433 ns | 279 ns |
| D76 | 131 ns | 241 ns | 401 ns | 327 ns | 658 ns |
| D115 | 603 ns | 450 ns | 622 ns | 1.08 µs | 1.08 µs |
| D153 | 357 ns | 718 ns | 568 ns | 1.05 µs | 672 ns |
| D230 | 781 ns | 882 ns | 1.25 µs | 1.32 µs | 2.03 µs |
| D307 | 1.2 µs | 1.49 µs | 1.16 µs | 2.45 µs | 3.14 µs |
| D462 | 1.19 µs | 1.82 µs | 2.52 µs | 3.46 µs | 3.65 µs |
| D616 | 1.06 µs | 1.34 µs | 3.31 µs | 4.55 µs | 5.19 µs |
| D924 | 2.12 µs | 3.43 µs | 2.8 µs | 8.21 µs | 11 µs |
| D1232 | 3.05 µs | 3.86 µs | 7.66 µs | 8.13 µs | 31.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,163.7 88.2,177.6 124.4,99.9 160.5,106.1 196.7,84.0 232.9,91.6 269.1,80.2 305.3,74.0 341.5,74.1 377.6,75.8 413.8,65.8 450.0,60.5 450.0,26.8 413.8,42.0 377.6,52.8 341.5,57.9 305.3,60.1 269.1,66.4 232.9,82.4 196.7,75.6 160.5,82.7 124.4,95.1 88.2,99.9 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,163.7 88.2,177.6 124.4,99.9 160.5,106.1 196.7,84.0 232.9,91.6 269.1,80.2 305.3,74.0 341.5,74.1 377.6,75.8 413.8,65.8 450.0,60.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,101.4 124.4,100.2 160.5,97.3 196.7,88.2 232.9,81.5 269.1,78.5 305.3,70.9 341.5,68.0 377.6,72.4 413.8,58.8 450.0,57.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.2 88.2,102.7 124.4,89.5 160.5,89.9 196.7,83.5 232.9,84.9 269.1,73.5 305.3,74.5 341.5,63.3 377.6,59.4 413.8,61.8 450.0,47.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.8 88.2,99.0 124.4,88.8 160.5,92.9 196.7,75.6 232.9,75.9 269.1,72.6 305.3,63.7 341.5,58.7 377.6,54.7 413.8,46.2 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,99.9 124.4,95.1 160.5,82.7 196.7,75.6 232.9,82.4 269.1,66.4 305.3,60.1 341.5,57.9 377.6,52.8 413.8,42.0 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
