# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.01 ns | 2.05 µs | 3.21 µs | 3.48 µs | 3.76 µs |
| D38 | 4.36 µs | 7.35 µs | 8.54 µs | 7.7 µs | 15.5 µs |
| D57 | 4.75 µs | 5.61 µs | 11.6 µs | 15.7 µs | 13.4 µs |
| D76 | 3.87 µs | 9.03 µs | 8.93 µs | 20.2 µs | 22.8 µs |
| D115 | 4.73 µs | 11.9 µs | 22.3 µs | 32.8 µs | 41.6 µs |
| D153 | 5.13 µs | 9.02 µs | 30 µs | 42.3 µs | 59.1 µs |
| D230 | 4.36 µs | 26.9 µs | 44.6 µs | 76 µs | 122 µs |
| D307 | 4.81 µs | 28.5 µs | 60.9 µs | 124 µs | 191 µs |
| D462 | 5.24 µs | 42.5 µs | 126 µs | 257 µs | 403 µs |
| D616 | 3.14 µs | 61 µs | 206 µs | 424 µs | 378 µs |
| D924 | 5.06 µs | 82 µs | 454 µs | 758 µs | 1.68 ms |
| D1232 | 6.04 µs | 220 µs | 658 µs | 1.52 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,106.0 124.4,105.0 160.5,107.5 196.7,105.0 232.9,104.0 269.1,106.0 305.3,104.8 341.5,103.7 377.6,110.1 413.8,104.2 450.0,102.0 450.0,23.0 413.8,32.2 377.6,50.7 341.5,49.9 305.3,59.1 269.1,64.7 232.9,73.7 196.7,78.0 160.5,85.5 124.4,92.1 88.2,90.3 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,106.0 124.4,105.0 160.5,107.5 196.7,105.0 232.9,104.0 269.1,106.0 305.3,104.8 341.5,103.7 377.6,110.1 413.8,104.2 450.0,102.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.4 88.2,99.5 124.4,102.9 160.5,97.0 196.7,93.6 232.9,97.0 269.1,83.5 305.3,82.7 341.5,77.8 377.6,73.3 413.8,69.6 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,97.7 124.4,93.8 160.5,97.1 196.7,85.8 232.9,82.1 269.1,77.2 305.3,73.3 341.5,64.2 377.6,58.1 413.8,48.4 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,99.0 124.4,90.2 160.5,87.0 196.7,81.0 232.9,77.8 269.1,70.5 305.3,64.5 341.5,55.5 377.6,49.2 413.8,42.0 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,90.3 124.4,92.1 160.5,85.5 196.7,78.0 232.9,73.7 269.1,64.7 305.3,59.1 341.5,49.9 377.6,50.7 413.8,32.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.38 µs | 3.66 µs | 5.85 µs | 6.37 µs | 6.81 µs |
| D38 | 3.38 µs | 5.8 µs | 7.15 µs | 6.31 µs | 9.62 µs |
| D57 | 3.37 µs | 2.54 µs | 5.01 µs | 6.2 µs | 5.35 µs |
| D76 | 2.69 µs | 4.5 µs | 3.39 µs | 8.39 µs | 10.1 µs |
| D115 | 6.49 µs | 9.13 µs | 11.6 µs | 17.3 µs | 22.2 µs |
| D153 | 6.96 µs | 6.61 µs | 16.5 µs | 22.5 µs | 29.1 µs |
| D230 | 7.66 µs | 17.4 µs | 29.4 µs | 47.8 µs | 71.2 µs |
| D307 | 12.4 µs | 25.9 µs | 47.7 µs | 82.5 µs | 138 µs |
| D462 | 13.2 µs | 36.2 µs | 83.9 µs | 166 µs | 273 µs |
| D616 | 14 µs | 71.6 µs | 171 µs | 320 µs | 288 µs |
| D924 | 33 µs | 93.8 µs | 409 µs | 663 µs | 1.45 ms |
| D1232 | 45.1 µs | 281 µs | 683 µs | 1.44 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,183.5 124.4,183.6 160.5,188.5 196.7,169.4 232.9,167.9 269.1,165.8 305.3,155.3 341.5,153.9 377.6,152.7 413.8,134.1 450.0,127.3 450.0,35.5 413.8,51.9 377.6,87.1 341.5,88.2 305.3,103.0 269.1,117.4 232.9,136.8 196.7,142.7 160.5,159.8 124.4,173.6 88.2,160.8 52.0,168.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,183.5 124.4,183.6 160.5,188.5 196.7,169.4 232.9,167.9 269.1,165.8 305.3,155.3 341.5,153.9 377.6,152.7 413.8,134.1 450.0,127.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.8 88.2,171.8 124.4,189.8 160.5,177.3 196.7,162.0 232.9,169.0 269.1,147.9 305.3,139.4 341.5,132.1 377.6,117.3 413.8,111.4 450.0,87.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,167.3 124.4,175.0 160.5,183.5 196.7,156.8 232.9,149.2 269.1,136.6 305.3,126.1 341.5,113.8 377.6,98.4 413.8,79.4 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.8 88.2,170.0 124.4,170.4 160.5,163.8 196.7,148.1 232.9,142.4 269.1,126.0 305.3,114.2 341.5,99.0 377.6,84.7 413.8,68.9 450.0,52.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.3 88.2,160.8 124.4,173.6 160.5,159.8 196.7,142.7 232.9,136.8 269.1,117.4 305.3,103.0 341.5,88.2 377.6,87.1 413.8,51.9 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 2.02 µs | 3.2 µs | 3.47 µs | 3.74 µs |
| D38 | 4.25 µs | 7.32 µs | 8.49 µs | 7.68 µs | 15.5 µs |
| D57 | 4.62 µs | 5.67 µs | 11.6 µs | 15.7 µs | 13.6 µs |
| D76 | 3.77 µs | 9 µs | 9.86 µs | 20.3 µs | 22.8 µs |
| D115 | 4.61 µs | 11.9 µs | 22.6 µs | 32.2 µs | 41.5 µs |
| D153 | 5 µs | 9.16 µs | 30.1 µs | 42.5 µs | 59.5 µs |
| D230 | 4.24 µs | 25.9 µs | 44.8 µs | 75.5 µs | 122 µs |
| D307 | 4.69 µs | 28.1 µs | 60.3 µs | 123 µs | 191 µs |
| D462 | 5.16 µs | 42.4 µs | 126 µs | 254 µs | 401 µs |
| D616 | 3.05 µs | 61.3 µs | 205 µs | 423 µs | 380 µs |
| D924 | 4.91 µs | 82.1 µs | 455 µs | 756 µs | 1.68 ms |
| D1232 | 5.62 µs | 220 µs | 659 µs | 1.53 ms | 3.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,106.3 124.4,105.3 160.5,107.8 196.7,105.3 232.9,104.3 269.1,106.4 305.3,105.1 341.5,103.9 377.6,110.4 413.8,104.5 450.0,102.9 450.0,23.0 413.8,32.2 377.6,50.6 341.5,49.9 305.3,59.1 269.1,64.7 232.9,73.6 196.7,78.0 160.5,85.5 124.4,91.9 88.2,90.3 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,106.3 124.4,105.3 160.5,107.8 196.7,105.3 232.9,104.3 269.1,106.4 305.3,105.1 341.5,103.9 377.6,110.4 413.8,104.5 450.0,102.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.6 88.2,99.6 124.4,102.8 160.5,97.0 196.7,93.6 232.9,96.8 269.1,83.9 305.3,82.9 341.5,77.8 377.6,73.2 413.8,69.6 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,97.7 124.4,93.9 160.5,95.9 196.7,85.6 232.9,82.0 269.1,77.1 305.3,73.4 341.5,64.3 377.6,58.2 413.8,48.3 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,99.0 124.4,90.1 160.5,87.0 196.7,81.2 232.9,77.8 269.1,70.6 305.3,64.5 341.5,55.6 377.6,49.2 413.8,42.0 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,90.3 124.4,91.9 160.5,85.5 196.7,78.0 232.9,73.6 269.1,64.7 305.3,59.1 341.5,49.9 377.6,50.6 413.8,32.2 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.74 µs | 2.55 µs | 2.78 µs | 2.99 µs |
| D38 | 4.22 ns | 2.53 µs | 3.16 µs | 2.81 µs | 4.28 µs |
| D57 | 1.95 ns | 3.25 µs | 7.44 µs | 9.16 µs | 8.61 µs |
| D76 | 1.38 ns | 6.2 µs | 5.81 µs | 13 µs | 15.1 µs |
| D115 | 14 ns | 12.7 µs | 17 µs | 24.3 µs | 33.1 µs |
| D153 | 16 ns | 9.98 µs | 24.2 µs | 33.5 µs | 41.7 µs |
| D230 | 26.2 ns | 26.1 µs | 44.6 µs | 67.2 µs | 96.1 µs |
| D307 | 44.9 ns | 38.8 µs | 68 µs | 116 µs | 177 µs |
| D462 | 74.8 ns | 54.7 µs | 118 µs | 208 µs | 337 µs |
| D616 | 53.5 ns | 105 µs | 225 µs | 398 µs | 361 µs |
| D924 | 104 ns | 149 µs | 532 µs | 737 µs | 1.6 ms |
| D1232 | 157 ns | 382 µs | 859 µs | 1.62 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,201.7 160.5,206.0 196.7,177.2 232.9,175.6 269.1,169.5 305.3,162.8 341.5,156.5 377.6,160.6 413.8,152.4 450.0,147.2 450.0,24.4 413.8,32.7 377.6,51.2 341.5,52.1 305.3,60.1 269.1,67.6 232.9,78.0 196.7,80.9 160.5,90.6 124.4,97.6 88.2,106.3 52.0,110.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,201.7 160.5,206.0 196.7,177.2 232.9,175.6 269.1,169.5 305.3,162.8 341.5,156.5 377.6,160.6 413.8,152.4 450.0,147.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,112.8 124.4,109.7 160.5,101.6 196.7,92.7 232.9,95.7 269.1,83.8 305.3,78.9 341.5,74.6 377.6,66.6 413.8,62.2 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.0 124.4,99.4 160.5,102.4 196.7,89.1 232.9,84.7 269.1,77.2 305.3,71.9 341.5,65.1 377.6,57.1 413.8,46.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,111.5 124.4,96.8 160.5,92.5 196.7,84.7 232.9,80.7 269.1,72.1 305.3,65.3 341.5,58.0 377.6,50.0 413.8,42.4 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,106.3 124.4,97.6 160.5,90.6 196.7,80.9 232.9,78.0 269.1,67.6 305.3,60.1 341.5,52.1 377.6,51.2 413.8,32.7 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.59 µs | 2.7 µs | 2.95 µs | 3.22 µs |
| D38 | 5.43 µs | 7.86 µs | 7.09 µs | 6.71 µs | 13.6 µs |
| D57 | 3.82 µs | 4.78 µs | 10.1 µs | 14 µs | 3.74 µs |
| D76 | 3.13 µs | 7.83 µs | 7.82 µs | 17.8 µs | 20.2 µs |
| D115 | 3.84 µs | 10.4 µs | 19.8 µs | 29 µs | 38.1 µs |
| D153 | 4.14 µs | 7.91 µs | 23 µs | 38.9 µs | 53.4 µs |
| D230 | 3.51 µs | 23.4 µs | 40.8 µs | 70.3 µs | 114 µs |
| D307 | 3.91 µs | 25 µs | 50.1 µs | 115 µs | 178 µs |
| D462 | 3.15 µs | 34.3 µs | 107 µs | 223 µs | 341 µs |
| D616 | 2.56 µs | 55.9 µs | 191 µs | 396 µs | 357 µs |
| D924 | 4.06 µs | 75.1 µs | 422 µs | 720 µs | 1.58 ms |
| D1232 | 4.58 µs | 203 µs | 623 µs | 1.47 ms | 3.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,103.3 124.4,107.7 160.5,110.1 196.7,107.6 232.9,106.7 269.1,108.7 305.3,107.4 341.5,110.0 377.6,112.6 413.8,106.9 450.0,105.4 450.0,23.5 413.8,32.9 377.6,51.3 341.5,51.9 305.3,60.0 269.1,65.6 232.9,74.9 196.7,79.1 160.5,87.0 124.4,107.9 88.2,91.9 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,103.3 124.4,107.7 160.5,110.1 196.7,107.6 232.9,106.7 269.1,108.7 305.3,107.4 341.5,110.0 377.6,112.6 413.8,106.9 450.0,105.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.6 88.2,98.7 124.4,104.9 160.5,98.7 196.7,95.3 232.9,98.6 269.1,85.2 305.3,84.4 341.5,80.4 377.6,74.4 413.8,70.7 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,100.0 124.4,95.6 160.5,98.8 196.7,87.2 232.9,85.4 269.1,78.3 305.3,75.7 341.5,66.3 377.6,59.1 413.8,49.3 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,100.7 124.4,91.5 160.5,88.5 196.7,82.5 232.9,78.9 269.1,71.5 305.3,65.4 341.5,57.2 377.6,50.1 413.8,42.6 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,91.9 124.4,107.9 160.5,87.0 196.7,79.1 232.9,74.9 269.1,65.6 305.3,60.0 341.5,51.9 377.6,51.3 413.8,32.9 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.77 µs | 2.9 µs | 3.17 µs | 3.45 µs |
| D38 | 4.22 ns | 2.88 µs | 3.6 µs | 3.16 µs | 4.84 µs |
| D57 | 437 ns | 3.3 µs | 7.09 µs | 8.89 µs | 7.98 µs |
| D76 | 315 ns | 5.95 µs | 5.4 µs | 12 µs | 15.1 µs |
| D115 | 943 ns | 13 µs | 16.9 µs | 26 µs | 35 µs |
| D153 | 1.06 µs | 9.29 µs | 25 µs | 35.5 µs | 47.1 µs |
| D230 | 1.06 µs | 25.4 µs | 46.9 µs | 80.9 µs | 122 µs |
| D307 | 1.86 µs | 39.3 µs | 77.4 µs | 142 µs | 244 µs |
| D462 | 2.09 µs | 55.2 µs | 143 µs | 297 µs | 496 µs |
| D616 | 2.17 µs | 116 µs | 298 µs | 574 µs | 523 µs |
| D924 | 4.94 µs | 155 µs | 723 µs | 1.21 ms | 2.7 ms |
| D1232 | 7.09 µs | 472 µs | 1.24 ms | 2.7 ms | 5.79 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,134.6 160.5,138.6 196.7,125.0 232.9,123.6 269.1,123.5 305.3,116.6 341.5,115.1 377.6,114.7 413.8,104.5 450.0,100.0 450.0,16.8 413.8,26.3 377.6,46.6 341.5,47.3 305.3,56.1 269.1,64.7 232.9,76.5 196.7,80.2 160.5,90.6 124.4,98.5 88.2,104.7 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,134.6 160.5,138.6 196.7,125.0 232.9,123.6 269.1,123.5 305.3,116.6 341.5,115.1 377.6,114.7 413.8,104.5 450.0,100.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,111.2 124.4,109.5 160.5,102.2 196.7,92.5 232.9,96.6 269.1,84.1 305.3,78.7 341.5,74.5 377.6,65.3 413.8,61.7 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,108.4 124.4,100.0 160.5,103.4 196.7,89.2 232.9,84.3 269.1,76.6 305.3,70.3 341.5,62.7 377.6,53.6 413.8,42.6 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,110.0 124.4,97.2 160.5,93.4 196.7,83.9 232.9,80.0 269.1,69.8 305.3,62.8 341.5,53.6 377.6,45.5 413.8,36.2 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.7 124.4,98.5 160.5,90.6 196.7,80.2 232.9,76.5 269.1,64.7 305.3,56.1 341.5,47.3 377.6,46.6 413.8,26.3 450.0,16.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 1.18 µs | 2.51 µs | 2.6 µs | 2.82 µs |
| D38 | 5.62 ns | 2.51 µs | 3.26 µs | 2.27 µs | 4.14 µs |
| D57 | 2.18 ns | 2.2 µs | 4.68 µs | 5.62 µs | 6.36 µs |
| D76 | 2.93 ns | 3.75 µs | 3.06 µs | 7.84 µs | 9.86 µs |
| D115 | 11.6 ns | 4.69 µs | 8.79 µs | 13.7 µs | 17.9 µs |
| D153 | 18.1 ns | 3.16 µs | 9.95 µs | 18.5 µs | 27.5 µs |
| D230 | 30.1 ns | 10.3 µs | 20 µs | 40.2 µs | 70 µs |
| D307 | 64.4 ns | 12.5 µs | 25.2 µs | 68.8 µs | 117 µs |
| D462 | 125 ns | 15.5 µs | 63.1 µs | 142 µs | 230 µs |
| D616 | 95.4 ns | 30.4 µs | 122 µs | 256 µs | 236 µs |
| D924 | 150 ns | 47.2 µs | 281 µs | 509 µs | 1.13 ms |
| D1232 | 353 ns | 130 µs | 427 µs | 1.06 ms | 2.46 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,200.3 160.5,196.7 196.7,179.6 232.9,174.0 269.1,167.7 305.3,158.3 341.5,150.1 377.6,153.4 413.8,147.8 450.0,137.2 450.0,27.4 413.8,37.0 377.6,56.5 341.5,56.8 305.3,65.2 269.1,71.6 232.9,83.2 196.7,88.5 160.5,95.9 124.4,101.3 88.2,106.7 52.0,111.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,200.3 160.5,196.7 196.7,179.6 232.9,174.0 269.1,167.7 305.3,158.3 341.5,150.1 377.6,153.4 413.8,147.8 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.2 88.2,112.9 124.4,114.5 160.5,107.9 196.7,105.1 232.9,110.0 269.1,95.3 305.3,92.9 341.5,90.3 377.6,81.9 413.8,76.5 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.9 88.2,109.6 124.4,105.1 160.5,110.4 196.7,97.3 232.9,95.8 269.1,87.1 305.3,84.2 341.5,72.9 377.6,64.7 413.8,54.3 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,114.1 124.4,102.9 160.5,98.7 196.7,91.8 232.9,88.1 269.1,78.5 305.3,71.8 341.5,62.8 377.6,55.5 413.8,46.9 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,106.7 124.4,101.3 160.5,95.9 196.7,88.5 232.9,83.2 269.1,71.6 305.3,65.2 341.5,56.8 377.6,56.5 413.8,37.0 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 1.78 µs | 3.39 µs | 3.6 µs | 3.83 µs |
| D38 | 4.57 ns | 3.37 µs | 4.35 µs | 3.09 µs | 5.41 µs |
| D57 | 3.12 ns | 3.42 µs | 7.27 µs | 8.13 µs | 7.71 µs |
| D76 | 3.62 ns | 5.93 µs | 4.56 µs | 11.2 µs | 13.3 µs |
| D115 | 9.97 ns | 12.2 µs | 11 µs | 21.5 µs | 26.1 µs |
| D153 | 18 ns | 4.67 µs | 16.3 µs | 23.4 µs | 34 µs |
| D230 | 30.3 ns | 14.6 µs | 25 µs | 47.4 µs | 79.9 µs |
| D307 | 65.1 ns | 16.7 µs | 51.3 µs | 80.3 µs | 125 µs |
| D462 | 123 ns | 23.6 µs | 81.8 µs | 168 µs | 251 µs |
| D616 | 93.2 ns | 36.2 µs | 133 µs | 272 µs | 226 µs |
| D924 | 165 ns | 61.6 µs | 292 µs | 497 µs | 999 µs |
| D1232 | 365 ns | 143 µs | 414 µs | 923 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,195.9 160.5,194.0 196.7,181.5 232.9,174.1 269.1,167.7 305.3,158.2 341.5,150.3 377.6,153.7 413.8,146.6 450.0,136.8 450.0,25.5 413.8,38.6 377.6,57.0 341.5,55.7 305.3,64.4 269.1,69.9 232.9,80.5 196.7,83.8 160.5,92.2 124.4,98.9 88.2,103.3 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,195.9 160.5,194.0 196.7,181.5 232.9,174.1 269.1,167.7 305.3,158.2 341.5,150.3 377.6,153.7 413.8,146.6 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,109.2 124.4,109.0 160.5,102.2 196.7,93.2 232.9,105.2 269.1,91.0 305.3,89.4 341.5,85.0 377.6,79.8 413.8,73.2 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.1 88.2,106.0 124.4,99.7 160.5,105.5 196.7,94.6 232.9,89.7 269.1,84.3 305.3,75.4 341.5,69.6 377.6,63.6 413.8,53.8 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,110.3 124.4,98.3 160.5,94.3 196.7,86.2 232.9,85.2 269.1,76.4 305.3,69.9 341.5,60.7 377.6,54.7 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,103.3 124.4,98.9 160.5,92.2 196.7,83.8 232.9,80.5 269.1,69.9 305.3,64.4 341.5,55.7 377.6,57.0 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.08 µs | 2.35 µs | 2.55 µs | 2.77 µs |
| D38 | 4.92 ns | 2.34 µs | 3.08 µs | 2.15 µs | 4.09 µs |
| D57 | 2.18 ns | 2.03 µs | 4.34 µs | 5.5 µs | 6.43 µs |
| D76 | 3.19 ns | 3.49 µs | 2.91 µs | 7.76 µs | 9.53 µs |
| D115 | 11.6 ns | 4.36 µs | 8.95 µs | 13.2 µs | 17.4 µs |
| D153 | 18.2 ns | 3.02 µs | 10.1 µs | 17.9 µs | 27.1 µs |
| D230 | 30.2 ns | 10.3 µs | 19.5 µs | 39.6 µs | 66.9 µs |
| D307 | 60 ns | 11.9 µs | 24.6 µs | 65.7 µs | 115 µs |
| D462 | 187 ns | 15.5 µs | 59.5 µs | 142 µs | 227 µs |
| D616 | 87 ns | 30 µs | 121 µs | 254 µs | 235 µs |
| D924 | 167 ns | 45.1 µs | 271 µs | 505 µs | 1.13 ms |
| D1232 | 357 ns | 127 µs | 424 µs | 1.05 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,190.2 124.4,200.3 160.5,195.6 196.7,179.5 232.9,174.0 269.1,167.7 305.3,159.2 341.5,145.1 377.6,154.6 413.8,146.5 450.0,137.1 450.0,27.5 413.8,37.1 377.6,56.5 341.5,57.0 305.3,65.4 269.1,72.1 232.9,83.4 196.7,88.8 160.5,96.3 124.4,101.2 88.2,106.8 52.0,111.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,190.2 124.4,200.3 160.5,195.6 196.7,179.5 232.9,174.0 269.1,167.7 305.3,159.2 341.5,145.1 377.6,154.6 413.8,146.5 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.3 88.2,113.7 124.4,115.5 160.5,108.8 196.7,106.0 232.9,110.6 269.1,95.3 305.3,93.6 341.5,90.3 377.6,82.1 413.8,77.0 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.7 88.2,110.3 124.4,106.1 160.5,111.0 196.7,97.1 232.9,95.6 269.1,87.4 305.3,84.5 341.5,73.6 377.6,64.8 413.8,54.8 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,114.8 124.4,103.1 160.5,98.9 196.7,92.3 232.9,88.5 269.1,78.6 305.3,72.3 341.5,62.8 377.6,55.6 413.8,47.0 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,106.8 124.4,101.2 160.5,96.3 196.7,88.8 232.9,83.4 269.1,72.1 305.3,65.4 341.5,57.0 377.6,56.5 413.8,37.1 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.51 ns | 1.77 µs | 3.38 µs | 3.61 µs | 3.85 µs |
| D38 | 4.57 ns | 3.37 µs | 4.36 µs | 3.1 µs | 5.41 µs |
| D57 | 10.4 ns | 3.43 µs | 7.3 µs | 8.15 µs | 7.8 µs |
| D76 | 9.54 ns | 5.93 µs | 4.57 µs | 11.3 µs | 13.3 µs |
| D115 | 10.3 ns | 12.2 µs | 10.8 µs | 21.6 µs | 26 µs |
| D153 | 17.4 ns | 4.74 µs | 16.3 µs | 23.6 µs | 34 µs |
| D230 | 29.2 ns | 14.8 µs | 24.7 µs | 47.5 µs | 80.1 µs |
| D307 | 60.5 ns | 16.8 µs | 50.3 µs | 80.6 µs | 125 µs |
| D462 | 220 ns | 23.6 µs | 81.9 µs | 169 µs | 250 µs |
| D616 | 86.8 ns | 36.2 µs | 134 µs | 271 µs | 228 µs |
| D924 | 172 ns | 53.4 µs | 292 µs | 498 µs | 999 µs |
| D1232 | 360 ns | 144 µs | 414 µs | 923 µs | 2.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.1 124.4,181.0 160.5,182.0 196.7,181.1 232.9,174.6 269.1,168.1 305.3,159.1 341.5,143.1 377.6,154.6 413.8,146.1 450.0,137.0 450.0,25.5 413.8,38.6 377.6,56.9 341.5,55.8 305.3,64.4 269.1,69.9 232.9,80.5 196.7,83.9 160.5,92.2 124.4,98.8 88.2,103.3 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.1 124.4,181.0 160.5,182.0 196.7,181.1 232.9,174.6 269.1,168.1 305.3,159.1 341.5,143.1 377.6,154.6 413.8,146.1 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,109.2 124.4,109.0 160.5,102.2 196.7,93.2 232.9,105.0 269.1,90.9 305.3,89.3 341.5,85.0 377.6,79.7 413.8,74.9 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,106.0 124.4,99.6 160.5,105.4 196.7,94.8 232.9,89.6 269.1,84.5 305.3,75.7 341.5,69.6 377.6,63.5 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,110.3 124.4,98.2 160.5,94.2 196.7,86.2 232.9,85.1 269.1,76.4 305.3,69.8 341.5,60.6 377.6,54.8 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,103.3 124.4,98.8 160.5,92.2 196.7,83.9 232.9,80.5 269.1,69.9 305.3,64.4 341.5,55.8 377.6,56.9 413.8,38.6 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 2.12 µs | 3.96 µs | 4.2 µs | 4.55 µs |
| D38 | 4.92 ns | 3.97 µs | 5.19 µs | 3.91 µs | 6.65 µs |
| D57 | 3.12 ns | 2.7 µs | 5.75 µs | 7.2 µs | 6.61 µs |
| D76 | 3.53 ns | 4.62 µs | 3.9 µs | 9.9 µs | 11.8 µs |
| D115 | 10.6 ns | 5.83 µs | 11.5 µs | 16.2 µs | 21.2 µs |
| D153 | 17.9 ns | 4.01 µs | 12.8 µs | 21.8 µs | 31.6 µs |
| D230 | 30.9 ns | 12.8 µs | 24.3 µs | 44.9 µs | 75.5 µs |
| D307 | 58.4 ns | 14.5 µs | 29.1 µs | 75.2 µs | 127 µs |
| D462 | 114 ns | 18.8 µs | 68 µs | 152 µs | 248 µs |
| D616 | 85.5 ns | 34.3 µs | 133 µs | 280 µs | 258 µs |
| D924 | 139 ns | 50.8 µs | 297 µs | 544 µs | 1.22 ms |
| D1232 | 376 ns | 143 µs | 458 µs | 1.11 ms | 2.59 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,195.9 160.5,194.3 196.7,180.7 232.9,174.2 269.1,167.4 305.3,159.5 341.5,151.2 377.6,154.8 413.8,148.8 450.0,136.4 450.0,26.8 413.8,36.1 377.6,55.4 341.5,55.9 305.3,64.2 269.1,70.6 232.9,81.4 196.7,86.4 160.5,93.6 124.4,100.9 88.2,100.8 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,195.9 160.5,194.3 196.7,180.7 232.9,174.2 269.1,167.4 305.3,159.5 341.5,151.2 377.6,154.8 413.8,148.8 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,107.2 124.4,112.0 160.5,105.3 196.7,102.4 232.9,107.1 269.1,92.7 305.3,91.1 341.5,87.9 377.6,80.4 413.8,75.5 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,103.9 124.4,102.6 160.5,107.4 196.7,94.0 232.9,92.7 269.1,84.7 305.3,82.5 341.5,71.9 377.6,63.6 413.8,53.6 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,107.4 124.4,99.8 160.5,95.8 196.7,89.7 232.9,86.0 269.1,77.1 305.3,70.7 341.5,61.9 377.6,54.4 413.8,46.1 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,100.8 124.4,100.9 160.5,93.6 196.7,86.4 232.9,81.4 269.1,70.6 305.3,64.2 341.5,55.9 377.6,55.4 413.8,36.1 450.0,26.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.8 µs | 3.56 µs | 3.77 µs | 4.01 µs |
| D38 | 4.22 ns | 3.54 µs | 4.41 µs | 3.1 µs | 5.44 µs |
| D57 | 2.58 µs | 3.65 µs | 7.61 µs | 8.57 µs | 8.28 µs |
| D76 | 2.14 µs | 6.09 µs | 4.75 µs | 11.6 µs | 13.7 µs |
| D115 | 5.36 µs | 13 µs | 11.3 µs | 22.4 µs | 27.2 µs |
| D153 | 3 µs | 4.89 µs | 17.1 µs | 24 µs | 35.1 µs |
| D230 | 2.51 µs | 15.3 µs | 25.7 µs | 48.5 µs | 81.5 µs |
| D307 | 2.97 µs | 17.4 µs | 52.6 µs | 82.1 µs | 127 µs |
| D462 | 3.35 µs | 24.5 µs | 83.3 µs | 170 µs | 253 µs |
| D616 | 2.04 µs | 37.2 µs | 137 µs | 274 µs | 230 µs |
| D924 | 3.47 µs | 57.2 µs | 297 µs | 503 µs | 1.01 ms |
| D1232 | 4.42 µs | 145 µs | 418 µs | 932 µs | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,192.1 124.4,112.5 160.5,114.8 196.7,103.5 232.9,110.6 269.1,112.9 305.3,110.8 341.5,109.3 377.6,115.4 413.8,108.9 450.0,105.9 450.0,25.4 413.8,38.5 377.6,56.8 341.5,55.6 305.3,64.2 269.1,69.7 232.9,80.1 196.7,83.3 160.5,91.8 124.4,98.1 88.2,103.3 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,192.1 124.4,112.5 160.5,114.8 196.7,103.5 232.9,110.6 269.1,112.9 305.3,110.8 341.5,109.3 377.6,115.4 413.8,108.9 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.0 88.2,108.6 124.4,108.2 160.5,101.9 196.7,92.5 232.9,104.6 269.1,90.4 305.3,88.8 341.5,84.6 377.6,79.4 413.8,74.1 450.0,62.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.9 124.4,99.1 160.5,105.0 196.7,94.2 232.9,89.0 269.1,84.0 305.3,75.1 341.5,69.4 377.6,63.3 413.8,53.7 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,110.2 124.4,97.6 160.5,93.9 196.7,85.7 232.9,84.8 269.1,76.1 305.3,69.6 341.5,60.5 377.6,54.7 413.8,47.1 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,103.3 124.4,98.1 160.5,91.8 196.7,83.3 232.9,80.1 269.1,69.7 305.3,64.2 341.5,55.6 377.6,56.8 413.8,38.5 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 137 ns | 163 ns | 163 ns | 164 ns |
| D38 | 4.92 ns | 159 ns | 174 ns | 132 ns | 199 ns |
| D57 | 182 ns | 173 ns | 320 ns | 322 ns | 286 ns |
| D76 | 131 ns | 298 ns | 190 ns | 446 ns | 485 ns |
| D115 | 398 ns | 573 ns | 630 ns | 807 ns | 848 ns |
| D153 | 485 ns | 379 ns | 829 ns | 883 ns | 914 ns |
| D230 | 452 ns | 911 ns | 1.1 µs | 1.44 µs | 1.84 µs |
| D307 | 773 ns | 1.19 µs | 1.51 µs | 2.07 µs | 2.77 µs |
| D462 | 954 ns | 1.4 µs | 2.14 µs | 3.23 µs | 4.16 µs |
| D616 | 693 ns | 1.84 µs | 2.89 µs | 4.13 µs | 3.07 µs |
| D924 | 1.43 µs | 1.63 µs | 4.84 µs | 6.13 µs | 11.3 µs |
| D1232 | 2.28 µs | 4.24 µs | 7 µs | 11.1 µs | 30.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,119.6 160.5,125.4 196.7,106.0 232.9,102.6 269.1,103.8 305.3,94.5 341.5,90.8 377.6,96.4 413.8,83.8 450.0,75.7 450.0,30.5 413.8,47.9 377.6,70.5 341.5,65.2 305.3,72.3 269.1,79.4 232.9,91.6 196.7,92.9 160.5,102.6 124.4,111.7 88.2,118.1 52.0,121.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,119.6 160.5,125.4 196.7,106.0 232.9,102.6 269.1,103.8 305.3,94.5 341.5,90.8 377.6,96.4 413.8,83.8 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.6 88.2,121.9 124.4,120.5 160.5,111.0 196.7,99.7 232.9,106.9 269.1,91.6 305.3,87.0 341.5,84.1 377.6,79.4 413.8,81.6 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.5 88.2,120.3 124.4,109.8 160.5,118.9 196.7,98.0 232.9,93.3 269.1,88.4 305.3,82.9 341.5,76.8 377.6,71.6 413.8,62.6 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.5 88.2,125.2 124.4,109.7 160.5,104.0 196.7,93.7 232.9,92.2 269.1,83.7 305.3,77.4 341.5,69.6 377.6,65.4 413.8,58.5 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.4 88.2,118.1 124.4,111.7 160.5,102.6 196.7,92.9 232.9,91.6 269.1,79.4 305.3,72.3 341.5,65.2 377.6,70.5 413.8,47.9 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 153 ns | 196 ns | 197 ns | 197 ns |
| D38 | 4.92 ns | 196 ns | 209 ns | 146 ns | 213 ns |
| D57 | 277 ns | 229 ns | 424 ns | 445 ns | 367 ns |
| D76 | 203 ns | 407 ns | 253 ns | 576 ns | 600 ns |
| D115 | 642 ns | 793 ns | 821 ns | 1.03 µs | 1.07 µs |
| D153 | 741 ns | 469 ns | 1.07 µs | 1.1 µs | 1.17 µs |
| D230 | 730 ns | 1.28 µs | 1.45 µs | 1.86 µs | 2.22 µs |
| D307 | 1.28 µs | 1.69 µs | 2.04 µs | 2.62 µs | 3.4 µs |
| D462 | 1.49 µs | 1.82 µs | 2.69 µs | 3.87 µs | 4.82 µs |
| D616 | 1.12 µs | 2.4 µs | 3.59 µs | 4.88 µs | 3.51 µs |
| D924 | 2.4 µs | 2.26 µs | 5.89 µs | 7.01 µs | 12.4 µs |
| D1232 | 3.54 µs | 5.43 µs | 8.31 µs | 12.7 µs | 32.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,112.3 160.5,117.7 196.7,97.7 232.9,95.2 269.1,95.5 305.3,85.7 341.5,83.0 377.6,88.0 413.8,74.8 450.0,68.0 450.0,29.6 413.8,46.2 377.6,68.2 341.5,62.7 305.3,68.7 269.1,76.2 232.9,87.3 196.7,88.8 160.5,98.9 124.4,107.4 88.2,116.9 52.0,118.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,112.3 160.5,117.7 196.7,97.7 232.9,95.2 269.1,95.5 305.3,85.7 341.5,83.0 377.6,88.0 413.8,74.8 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.6 88.2,118.3 124.4,115.6 160.5,105.6 196.7,94.0 232.9,103.1 269.1,85.7 305.3,80.9 341.5,79.6 377.6,74.8 413.8,75.9 450.0,60.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,117.2 124.4,104.9 160.5,113.9 196.7,93.4 232.9,88.8 269.1,83.5 305.3,77.6 341.5,72.8 377.6,67.8 413.8,59.2 450.0,53.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,123.5 124.4,104.1 160.5,99.6 196.7,89.5 232.9,88.4 269.1,79.2 305.3,73.3 341.5,66.5 377.6,62.4 413.8,56.2 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,116.9 124.4,107.4 160.5,98.9 196.7,88.8 232.9,87.3 269.1,76.2 305.3,68.7 341.5,62.7 377.6,68.2 413.8,46.2 450.0,29.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
