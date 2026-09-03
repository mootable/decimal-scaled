# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.8 ns | 1.61 µs | 2.53 µs | 3.47 µs | 3.76 µs |
| D38 | 1.36 µs | 4.72 µs | 4.11 µs | 8.78 µs | 9.8 µs |
| D57 | 1.49 µs | 5 µs | 8.14 µs | 10.6 µs | 14 µs |
| D76 | 949 ns | 7.31 µs | 9.83 µs | 12 µs | 13.1 µs |
| D115 | 1.5 µs | 8.12 µs | 21.3 µs | 32.8 µs | 42.5 µs |
| D153 | 1.51 µs | 10.7 µs | 29 µs | 25.6 µs | 66.2 µs |
| D230 | 1.1 µs | 17.8 µs | 43.3 µs | 69.8 µs | 133 µs |
| D307 | 1.62 µs | 19.1 µs | 53.2 µs | 126 µs | 182 µs |
| D462 | 1.61 µs | 43.2 µs | 137 µs | 257 µs | 399 µs |
| D616 | 1.6 µs | 66.2 µs | 207 µs | 357 µs | 484 µs |
| D924 | 1.75 µs | 107 µs | 449 µs | 571 µs | 1.67 ms |
| D1232 | 2.19 µs | 208 µs | 669 µs | 1.54 ms | 3.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,120.5 124.4,119.4 160.5,124.9 196.7,119.3 232.9,119.2 269.1,123.1 305.3,118.3 341.5,118.4 377.6,118.4 413.8,117.4 450.0,114.6 450.0,23.5 413.8,32.2 377.6,47.6 341.5,50.0 305.3,59.7 269.1,63.6 232.9,72.3 196.7,77.7 160.5,92.4 124.4,91.5 88.2,96.0 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,120.5 124.4,119.4 160.5,124.9 196.7,119.3 232.9,119.2 269.1,123.1 305.3,118.3 341.5,118.4 377.6,118.4 413.8,117.4 450.0,114.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.4 88.2,105.0 124.4,104.3 160.5,99.6 196.7,98.3 232.9,94.9 269.1,88.6 305.3,87.7 341.5,77.6 377.6,72.3 413.8,66.3 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,106.7 124.4,98.3 160.5,95.9 196.7,86.4 232.9,82.5 269.1,77.5 305.3,75.0 341.5,63.2 377.6,58.1 413.8,48.5 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,97.3 124.4,95.0 160.5,93.5 196.7,81.0 232.9,84.1 269.1,71.6 305.3,64.3 341.5,55.4 377.6,51.3 413.8,45.5 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,96.0 124.4,91.5 160.5,92.4 196.7,77.7 232.9,72.3 269.1,63.6 305.3,59.7 341.5,50.0 377.6,47.6 413.8,32.2 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.28 µs | 2.91 µs | 4.56 µs | 6.32 µs | 6.75 µs |
| D38 | 3.28 µs | 5.79 µs | 7 µs | 8.39 µs | 9.48 µs |
| D57 | 3.27 µs | 3.47 µs | 4.94 µs | 6 µs | 7.46 µs |
| D76 | 2.01 µs | 5.03 µs | 5.83 µs | 6.7 µs | 6.59 µs |
| D115 | 6.68 µs | 9.22 µs | 12.8 µs | 17.3 µs | 22.5 µs |
| D153 | 6.2 µs | 11 µs | 16.6 µs | 13.4 µs | 34.1 µs |
| D230 | 6.33 µs | 15.1 µs | 28.3 µs | 44.1 µs | 76.4 µs |
| D307 | 12.8 µs | 18.1 µs | 38.5 µs | 83.2 µs | 131 µs |
| D462 | 12.4 µs | 36.3 µs | 88.2 µs | 166 µs | 273 µs |
| D616 | 22.4 µs | 75.6 µs | 169 µs | 268 µs | 378 µs |
| D924 | 32.7 µs | 130 µs | 400 µs | 476 µs | 1.44 ms |
| D1232 | 46.7 µs | 267 µs | 693 µs | 1.46 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,184.2 124.4,184.3 160.5,194.9 196.7,168.8 232.9,170.4 269.1,169.9 305.3,154.7 341.5,155.4 377.6,142.5 413.8,134.3 450.0,126.5 450.0,36.3 413.8,52.1 377.6,81.1 341.5,88.2 305.3,104.1 269.1,115.8 232.9,133.3 196.7,142.4 160.5,169.1 124.4,166.4 88.2,161.2 52.0,168.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,184.2 124.4,184.3 160.5,194.9 196.7,168.8 232.9,170.4 269.1,169.9 305.3,154.7 341.5,155.4 377.6,142.5 413.8,134.3 450.0,126.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,186.8 88.2,171.9 124.4,183.0 160.5,174.9 196.7,161.8 232.9,157.9 269.1,151.0 305.3,147.1 341.5,132.0 377.6,116.1 413.8,104.2 450.0,88.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.1 88.2,167.8 124.4,175.3 160.5,171.7 196.7,154.6 232.9,148.9 269.1,137.4 305.3,130.7 341.5,112.7 377.6,98.6 413.8,79.9 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.0 88.2,163.8 124.4,171.1 160.5,168.7 196.7,148.1 232.9,153.6 269.1,127.8 305.3,114.0 341.5,99.0 377.6,88.6 413.8,76.1 450.0,51.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.5 88.2,161.2 124.4,166.4 160.5,169.1 196.7,142.4 232.9,133.3 269.1,115.8 305.3,104.1 341.5,88.2 377.6,81.1 413.8,52.1 450.0,36.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.62 µs | 2.52 µs | 3.48 µs | 3.75 µs |
| D38 | 1.27 µs | 4.68 µs | 4.09 µs | 8.73 µs | 9.79 µs |
| D57 | 1.38 µs | 4.98 µs | 8.13 µs | 10.5 µs | 14 µs |
| D76 | 853 ns | 7.28 µs | 9.73 µs | 12 µs | 13.1 µs |
| D115 | 1.39 µs | 8.09 µs | 19.8 µs | 33.1 µs | 43.5 µs |
| D153 | 1.39 µs | 10.7 µs | 26.2 µs | 25.9 µs | 66.1 µs |
| D230 | 1.02 µs | 17.6 µs | 42 µs | 69.8 µs | 133 µs |
| D307 | 1.52 µs | 18.8 µs | 53.4 µs | 125 µs | 181 µs |
| D462 | 1.48 µs | 42.6 µs | 134 µs | 255 µs | 399 µs |
| D616 | 1.53 µs | 65.7 µs | 207 µs | 358 µs | 487 µs |
| D924 | 1.6 µs | 108 µs | 452 µs | 576 µs | 1.67 ms |
| D1232 | 2.07 µs | 207 µs | 666 µs | 1.53 ms | 3.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,121.3 124.4,120.3 160.5,126.3 196.7,120.2 232.9,120.2 269.1,124.1 305.3,119.1 341.5,119.4 377.6,119.0 413.8,118.4 450.0,115.2 450.0,23.5 413.8,32.2 377.6,47.5 341.5,50.0 305.3,59.8 269.1,63.6 232.9,72.3 196.7,77.5 160.5,92.4 124.4,91.5 88.2,96.0 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,121.3 124.4,120.3 160.5,126.3 196.7,120.2 232.9,120.2 269.1,124.1 305.3,119.1 341.5,119.4 377.6,119.0 413.8,118.4 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,105.1 124.4,104.4 160.5,99.7 196.7,98.3 232.9,94.9 269.1,88.7 305.3,87.9 341.5,77.7 377.6,72.4 413.8,66.2 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,106.8 124.4,98.3 160.5,96.1 196.7,87.2 232.9,83.8 269.1,77.9 305.3,74.9 341.5,63.5 377.6,58.1 413.8,48.4 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,97.4 124.4,95.1 160.5,93.5 196.7,80.9 232.9,83.9 269.1,71.6 305.3,64.4 341.5,55.5 377.6,51.3 413.8,45.4 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,96.0 124.4,91.5 160.5,92.4 196.7,77.5 232.9,72.3 269.1,63.6 305.3,59.8 341.5,50.0 377.6,47.5 413.8,32.2 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.4 µs | 2.01 µs | 2.79 µs | 2.96 µs |
| D38 | 3.74 ns | 2.51 µs | 3.08 µs | 3.75 µs | 4.19 µs |
| D57 | 1.95 ns | 4.44 µs | 7.4 µs | 9.09 µs | 11.8 µs |
| D76 | 1.04 ns | 6.85 µs | 8.79 µs | 10.2 µs | 9.89 µs |
| D115 | 13.7 ns | 12.9 µs | 19 µs | 24.8 µs | 33.4 µs |
| D153 | 15.7 ns | 16 µs | 23.6 µs | 20.1 µs | 47.3 µs |
| D230 | 22.4 ns | 21.9 µs | 42 µs | 62.7 µs | 103 µs |
| D307 | 52 ns | 26.6 µs | 56.3 µs | 116 µs | 167 µs |
| D462 | 70.1 ns | 54.8 µs | 127 µs | 210 µs | 337 µs |
| D616 | 82.9 ns | 111 µs | 226 µs | 339 µs | 444 µs |
| D924 | 103 ns | 196 µs | 528 µs | 548 µs | 1.59 ms |
| D1232 | 164 ns | 368 µs | 871 µs | 1.64 ms | 2.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,209.5 196.7,177.5 232.9,175.8 269.1,171.4 305.3,161.0 341.5,157.3 377.6,155.2 413.8,152.5 450.0,146.7 450.0,25.3 413.8,32.8 377.6,48.7 341.5,52.1 305.3,60.8 269.1,66.7 232.9,76.4 196.7,80.7 160.5,95.9 124.4,93.7 88.2,106.5 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,209.5 196.7,177.5 232.9,175.8 269.1,171.4 305.3,161.0 341.5,157.3 377.6,155.2 413.8,152.5 450.0,146.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.1 88.2,112.9 124.4,105.8 160.5,100.4 196.7,92.6 232.9,89.9 269.1,86.0 305.3,83.6 341.5,74.6 377.6,65.8 413.8,58.8 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.6 88.2,110.3 124.4,99.4 160.5,97.3 196.7,87.7 232.9,85.0 269.1,77.9 305.3,74.3 341.5,64.2 377.6,57.0 413.8,46.5 450.0,40.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,107.9 124.4,96.9 160.5,95.5 196.7,84.4 232.9,87.0 269.1,72.9 305.3,65.3 341.5,57.9 377.6,52.0 413.8,46.0 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.5 124.4,93.7 160.5,95.9 196.7,80.7 232.9,76.4 269.1,66.7 305.3,60.8 341.5,52.1 377.6,48.7 413.8,32.8 450.0,25.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.28 µs | 2.08 µs | 2.95 µs | 3.22 µs |
| D38 | 688 ns | 4.01 µs | 2.91 µs | 6.92 µs | 7.75 µs |
| D57 | 627 ns | 3.79 µs | 6.3 µs | 8.26 µs | 5.1 µs |
| D76 | 353 ns | 5.64 µs | 7.78 µs | 9.8 µs | 10.9 µs |
| D115 | 596 ns | 6.23 µs | 16.2 µs | 30 µs | 38.7 µs |
| D153 | 631 ns | 8.4 µs | 17.2 µs | 24.1 µs | 60.8 µs |
| D230 | 428 ns | 16.2 µs | 38.3 µs | 64.2 µs | 123 µs |
| D307 | 683 ns | 15.9 µs | 43 µs | 117 µs | 171 µs |
| D462 | 664 ns | 34.8 µs | 114 µs | 222 µs | 343 µs |
| D616 | 702 ns | 60.5 µs | 192 µs | 334 µs | 459 µs |
| D924 | 798 ns | 101 µs | 421 µs | 545 µs | 1.58 ms |
| D1232 | 1.1 µs | 193 µs | 631 µs | 1.47 ms | 3.24 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,128.9 124.4,130.1 160.5,137.2 196.7,130.7 232.9,130.0 269.1,134.8 305.3,129.0 341.5,129.4 377.6,128.7 413.8,127.1 450.0,123.1 450.0,24.0 413.8,32.9 377.6,48.2 341.5,51.8 305.3,60.5 269.1,64.6 232.9,73.3 196.7,78.9 160.5,94.7 124.4,104.1 88.2,98.9 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,128.9 124.4,130.1 160.5,137.2 196.7,130.7 232.9,130.0 269.1,134.8 305.3,129.0 341.5,129.4 377.6,128.7 413.8,127.1 450.0,123.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.2 88.2,107.0 124.4,107.7 160.5,102.8 196.7,101.6 232.9,97.9 269.1,89.7 305.3,90.0 341.5,80.2 377.6,73.4 413.8,67.0 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.2 88.2,111.0 124.4,101.4 160.5,98.8 196.7,89.8 232.9,89.0 269.1,79.1 305.3,77.6 341.5,65.5 377.6,59.0 413.8,49.3 450.0,44.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,100.3 124.4,98.1 160.5,96.0 196.7,82.1 232.9,84.8 269.1,72.7 305.3,65.1 341.5,57.3 377.6,52.2 413.8,46.1 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,98.9 124.4,104.1 160.5,94.7 196.7,78.9 232.9,73.3 269.1,64.6 305.3,60.5 341.5,51.8 377.6,48.2 413.8,32.9 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.44 µs | 2.25 µs | 3.15 µs | 3.42 µs |
| D38 | 3.74 ns | 2.86 µs | 3.55 µs | 4.16 µs | 4.78 µs |
| D57 | 446 ns | 4.48 µs | 6.9 µs | 8.61 µs | 11.1 µs |
| D76 | 246 ns | 6.67 µs | 8.21 µs | 9.57 µs | 9.94 µs |
| D115 | 917 ns | 13 µs | 19.2 µs | 26.8 µs | 35.4 µs |
| D153 | 948 ns | 15.8 µs | 25.5 µs | 20.6 µs | 55.8 µs |
| D230 | 969 ns | 21.5 µs | 43.4 µs | 74.3 µs | 130 µs |
| D307 | 1.97 µs | 27.2 µs | 63.8 µs | 143 µs | 232 µs |
| D462 | 1.97 µs | 56 µs | 150 µs | 296 µs | 497 µs |
| D616 | 3.47 µs | 121 µs | 295 µs | 481 µs | 691 µs |
| D924 | 4.92 µs | 214 µs | 707 µs | 875 µs | 2.68 ms |
| D1232 | 7.03 µs | 460 µs | 1.26 ms | 2.73 ms | 5.61 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,141.7 196.7,125.4 232.9,124.9 269.1,124.7 305.3,115.9 341.5,115.9 377.6,108.8 413.8,104.5 450.0,100.1 450.0,17.2 413.8,26.4 377.6,43.2 341.5,47.3 305.3,56.7 269.1,63.9 232.9,74.4 196.7,80.0 160.5,95.8 124.4,94.5 88.2,104.9 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,141.7 196.7,125.4 232.9,124.9 269.1,124.7 305.3,115.9 341.5,115.9 377.6,108.8 413.8,104.5 450.0,100.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.8 88.2,111.2 124.4,105.7 160.5,100.7 196.7,92.5 232.9,90.0 269.1,86.2 305.3,83.3 341.5,74.3 377.6,64.8 413.8,57.7 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.2 88.2,108.6 124.4,100.3 160.5,98.2 196.7,87.6 232.9,84.1 269.1,77.5 305.3,72.7 341.5,62.1 377.6,53.7 413.8,42.9 450.0,35.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,97.6 160.5,96.3 196.7,83.5 232.9,86.8 269.1,70.8 305.3,62.7 341.5,53.7 377.6,47.6 413.8,40.2 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.9 124.4,94.5 160.5,95.8 196.7,80.0 232.9,74.4 269.1,63.9 305.3,56.7 341.5,47.3 377.6,43.2 413.8,26.4 450.0,17.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 989 ns | 2.11 µs | 2.59 µs | 2.81 µs |
| D38 | 4.98 ns | 2.51 µs | 2.98 µs | 3.76 µs | 3.79 µs |
| D57 | 2.18 ns | 2.92 µs | 4.66 µs | 5.55 µs | 8.93 µs |
| D76 | 2.16 ns | 4.11 µs | 5.46 µs | 6.04 µs | 6.25 µs |
| D115 | 11.6 ns | 4.66 µs | 9.68 µs | 13.9 µs | 18.3 µs |
| D153 | 15.7 ns | 5.93 µs | 10.2 µs | 10.6 µs | 33.3 µs |
| D230 | 25.1 ns | 8.76 µs | 18.3 µs | 36.4 µs | 74.1 µs |
| D307 | 72.9 ns | 9.05 µs | 21.2 µs | 69.1 µs | 112 µs |
| D462 | 105 ns | 15.5 µs | 66.5 µs | 142 µs | 231 µs |
| D616 | 119 ns | 33.4 µs | 123 µs | 217 µs | 321 µs |
| D924 | 150 ns | 61.9 µs | 267 µs | 378 µs | 1.13 ms |
| D1232 | 354 ns | 125 µs | 435 µs | 1.06 ms | 2.4 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.1 124.4,200.3 160.5,200.4 196.7,179.6 232.9,175.8 269.1,170.0 305.3,156.8 341.5,152.2 377.6,150.7 413.8,147.8 450.0,137.2 450.0,27.7 413.8,37.1 377.6,52.7 341.5,56.8 305.3,65.7 269.1,70.9 232.9,80.8 196.7,88.2 160.5,101.5 124.4,97.1 88.2,107.7 52.0,111.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.1 124.4,200.3 160.5,200.4 196.7,179.6 232.9,175.8 269.1,170.0 305.3,156.8 341.5,152.2 377.6,150.7 413.8,147.8 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.4 88.2,112.9 124.4,111.0 160.5,106.7 196.7,105.2 232.9,102.2 269.1,97.4 305.3,97.0 341.5,90.3 377.6,80.7 413.8,73.1 450.0,64.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.0 88.2,110.7 124.4,105.2 160.5,103.2 196.7,96.1 232.9,95.5 269.1,88.2 305.3,86.4 341.5,72.2 377.6,64.6 413.8,55.0 450.0,48.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,107.8 124.4,103.0 160.5,102.0 196.7,91.6 232.9,95.0 269.1,79.7 305.3,71.7 341.5,62.8 377.6,57.5 413.8,50.7 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,107.7 124.4,97.1 160.5,101.5 196.7,88.2 232.9,80.8 269.1,70.9 305.3,65.7 341.5,56.8 377.6,52.7 413.8,37.1 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 1.49 µs | 2.85 µs | 3.59 µs | 3.82 µs |
| D38 | 4.05 ns | 3.35 µs | 3.95 µs | 4.89 µs | 4.9 µs |
| D57 | 2.81 ns | 4.59 µs | 7.3 µs | 8.15 µs | 10.5 µs |
| D76 | 2.32 ns | 6.38 µs | 7.74 µs | 8.77 µs | 8.67 µs |
| D115 | 10.3 ns | 12.3 µs | 12.4 µs | 21.7 µs | 26.3 µs |
| D153 | 15.5 ns | 8.38 µs | 16.9 µs | 13.3 µs | 39.6 µs |
| D230 | 25.1 ns | 12.4 µs | 22.9 µs | 42.8 µs | 84.7 µs |
| D307 | 75.8 ns | 12.4 µs | 41.7 µs | 80.7 µs | 118 µs |
| D462 | 106 ns | 23.8 µs | 89.4 µs | 170 µs | 251 µs |
| D616 | 119 ns | 39.8 µs | 134 µs | 228 µs | 310 µs |
| D924 | 171 ns | 69.3 µs | 290 µs | 376 µs | 994 µs |
| D1232 | 355 ns | 135 µs | 417 µs | 925 µs | 3.14 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.7 88.2,192.6 124.4,197.2 160.5,199.6 196.7,181.1 232.9,176.0 269.1,170.0 305.3,156.3 341.5,152.1 377.6,150.7 413.8,146.2 450.0,137.2 450.0,24.4 413.8,38.6 377.6,53.1 341.5,55.7 305.3,65.1 269.1,69.2 232.9,78.6 196.7,83.7 160.5,97.5 124.4,95.2 88.2,104.6 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.7 88.2,192.6 124.4,197.2 160.5,199.6 196.7,181.1 232.9,176.0 269.1,170.0 305.3,156.3 341.5,152.1 377.6,150.7 413.8,146.2 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.3 88.2,109.3 124.4,105.4 160.5,101.3 196.7,93.1 232.9,97.9 269.1,93.0 305.3,93.0 341.5,84.9 377.6,78.6 413.8,71.7 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,107.2 124.4,99.6 160.5,98.9 196.7,93.0 232.9,89.2 269.1,85.4 305.3,78.0 341.5,68.5 377.6,63.5 413.8,53.9 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.6 124.4,98.3 160.5,97.3 196.7,86.1 232.9,92.2 269.1,77.7 305.3,69.8 341.5,60.6 377.6,56.9 413.8,50.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,104.6 124.4,95.2 160.5,97.5 196.7,83.7 232.9,78.6 269.1,69.2 305.3,65.1 341.5,55.7 377.6,53.1 413.8,38.6 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 907 ns | 1.97 µs | 2.55 µs | 2.76 µs |
| D38 | 4.36 ns | 2.35 µs | 2.82 µs | 3.59 µs | 3.75 µs |
| D57 | 2.49 ns | 2.7 µs | 4.33 µs | 5.39 µs | 8.93 µs |
| D76 | 2.44 ns | 3.91 µs | 5.14 µs | 5.98 µs | 6.01 µs |
| D115 | 11.6 ns | 4.38 µs | 9.7 µs | 13.3 µs | 17.7 µs |
| D153 | 16.8 ns | 5.69 µs | 10.4 µs | 11.2 µs | 34.1 µs |
| D230 | 24.9 ns | 9.35 µs | 18.2 µs | 36.2 µs | 70.9 µs |
| D307 | 66.7 ns | 8.6 µs | 20.2 µs | 66 µs | 109 µs |
| D462 | 97.9 ns | 15.5 µs | 62.9 µs | 140 µs | 229 µs |
| D616 | 115 ns | 32.5 µs | 121 µs | 214 µs | 316 µs |
| D924 | 183 ns | 57.8 µs | 270 µs | 377 µs | 1.13 ms |
| D1232 | 347 ns | 123 µs | 434 µs | 1.05 ms | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,198.7 160.5,198.9 196.7,179.5 232.9,175.0 269.1,170.1 305.3,157.9 341.5,153.1 377.6,151.1 413.8,145.3 450.0,137.4 450.0,27.8 413.8,37.1 377.6,52.9 341.5,56.9 305.3,66.1 269.1,71.4 232.9,80.5 196.7,88.6 160.5,102.0 124.4,97.1 88.2,107.9 52.0,111.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,198.7 160.5,198.9 196.7,179.5 232.9,175.0 269.1,170.1 305.3,157.9 341.5,153.1 377.6,151.1 413.8,145.3 450.0,137.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.5 88.2,113.7 124.4,112.0 160.5,107.4 196.7,106.0 232.9,102.7 269.1,96.5 305.3,97.6 341.5,90.3 377.6,81.1 413.8,73.9 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.8 88.2,111.4 124.4,106.1 160.5,104.0 196.7,96.1 232.9,95.2 269.1,88.3 305.3,87.0 341.5,72.9 377.6,64.7 413.8,54.8 450.0,48.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,108.4 124.4,103.4 160.5,102.1 196.7,92.2 232.9,94.3 269.1,79.8 305.3,72.3 341.5,62.9 377.6,57.7 413.8,50.7 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,107.9 124.4,97.1 160.5,102.0 196.7,88.6 232.9,80.5 269.1,71.4 305.3,66.1 341.5,56.9 377.6,52.9 413.8,37.1 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.5 µs | 2.87 µs | 3.6 µs | 3.82 µs |
| D38 | 4.05 ns | 3.37 µs | 3.96 µs | 4.89 µs | 4.91 µs |
| D57 | 10.6 ns | 4.6 µs | 7.28 µs | 8.22 µs | 10.5 µs |
| D76 | 7.02 ns | 6.38 µs | 7.82 µs | 8.79 µs | 8.73 µs |
| D115 | 10.6 ns | 12.3 µs | 12.2 µs | 21.7 µs | 26.3 µs |
| D153 | 14.9 ns | 8.42 µs | 16.1 µs | 13.4 µs | 39.6 µs |
| D230 | 24.3 ns | 12.6 µs | 23 µs | 42.8 µs | 85 µs |
| D307 | 65.1 ns | 12.5 µs | 40.5 µs | 80.6 µs | 118 µs |
| D462 | 99.7 ns | 23.7 µs | 88.7 µs | 169 µs | 250 µs |
| D616 | 112 ns | 39.9 µs | 134 µs | 229 µs | 310 µs |
| D924 | 175 ns | 70 µs | 288 µs | 375 µs | 995 µs |
| D1232 | 376 ns | 135 µs | 417 µs | 923 µs | 3.14 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.6 124.4,180.7 160.5,185.8 196.7,180.7 232.9,176.5 269.1,170.4 305.3,158.2 341.5,152.9 377.6,151.4 413.8,145.9 450.0,136.4 450.0,24.4 413.8,38.6 377.6,53.1 341.5,55.8 305.3,65.1 269.1,69.2 232.9,78.6 196.7,83.7 160.5,97.4 124.4,95.1 88.2,104.5 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.6 124.4,180.7 160.5,185.8 196.7,180.7 232.9,176.5 269.1,170.4 305.3,158.2 341.5,152.9 377.6,151.4 413.8,145.9 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.2 88.2,109.2 124.4,105.3 160.5,101.3 196.7,93.1 232.9,97.8 269.1,92.9 305.3,92.9 341.5,85.0 377.6,78.6 413.8,71.6 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.2 124.4,99.7 160.5,98.8 196.7,93.2 232.9,89.8 269.1,85.4 305.3,78.4 341.5,68.6 377.6,63.6 413.8,54.0 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,104.6 124.4,98.1 160.5,97.3 196.7,86.1 232.9,92.0 269.1,77.7 305.3,69.8 341.5,60.6 377.6,56.9 413.8,50.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,104.5 124.4,95.1 160.5,97.4 196.7,83.7 232.9,78.6 269.1,69.2 305.3,65.1 341.5,55.8 377.6,53.1 413.8,38.6 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.82 µs | 3.36 µs | 4.19 µs | 4.54 µs |
| D38 | 4.36 ns | 3.97 µs | 4.72 µs | 5.98 µs | 6.1 µs |
| D57 | 2.81 ns | 3.77 µs | 5.82 µs | 7.05 µs | 9 µs |
| D76 | 2.55 ns | 5.23 µs | 6.77 µs | 7.7 µs | 7.5 µs |
| D115 | 10.9 ns | 5.79 µs | 12.4 µs | 16.4 µs | 21.8 µs |
| D153 | 15.5 ns | 7.59 µs | 12.7 µs | 13.2 µs | 37.7 µs |
| D230 | 25.6 ns | 11.3 µs | 22.1 µs | 41.4 µs | 80 µs |
| D307 | 62.7 ns | 10.6 µs | 24.3 µs | 75.3 µs | 120 µs |
| D462 | 96.5 ns | 18.7 µs | 72.2 µs | 155 µs | 248 µs |
| D616 | 154 ns | 37.4 µs | 133 µs | 234 µs | 340 µs |
| D924 | 143 ns | 67.5 µs | 293 µs | 407 µs | 1.19 ms |
| D1232 | 360 ns | 136 µs | 467 µs | 1.12 ms | 2.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,198.4 196.7,180.4 232.9,176.0 269.1,169.7 305.3,158.6 341.5,153.3 377.6,147.5 413.8,148.4 450.0,137.0 450.0,27.2 413.8,36.4 377.6,52.0 341.5,55.9 305.3,64.9 269.1,69.9 232.9,79.2 196.7,86.1 160.5,99.3 124.4,97.0 88.2,101.8 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,198.4 196.7,180.4 232.9,176.0 269.1,169.7 305.3,158.6 341.5,153.3 377.6,147.5 413.8,148.4 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,107.2 124.4,107.8 160.5,103.8 196.7,102.5 232.9,99.1 269.1,94.2 305.3,95.0 341.5,88.0 377.6,79.3 413.8,72.0 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,105.0 124.4,102.4 160.5,100.5 196.7,93.1 232.9,92.7 269.1,85.9 305.3,84.7 341.5,71.2 377.6,63.6 413.8,53.8 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,102.1 124.4,100.0 160.5,98.9 196.7,89.6 232.9,92.3 269.1,78.1 305.3,70.7 341.5,61.7 377.6,56.6 413.8,49.7 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,101.8 124.4,97.0 160.5,99.3 196.7,86.1 232.9,79.2 269.1,69.9 305.3,64.9 341.5,55.9 377.6,52.0 413.8,36.4 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.53 µs | 3 µs | 3.78 µs | 4.01 µs |
| D38 | 3.74 ns | 3.53 µs | 4 µs | 4.91 µs | 4.94 µs |
| D57 | 2.58 µs | 4.77 µs | 7.63 µs | 8.59 µs | 10.9 µs |
| D76 | 1.59 µs | 7.29 µs | 8.21 µs | 9.11 µs | 8.99 µs |
| D115 | 5.42 µs | 13 µs | 13.2 µs | 22.6 µs | 27.4 µs |
| D153 | 2.6 µs | 8.77 µs | 16.9 µs | 13.9 µs | 40.7 µs |
| D230 | 2.05 µs | 13.4 µs | 24 µs | 43.9 µs | 86.9 µs |
| D307 | 3.21 µs | 12.9 µs | 42.3 µs | 81.3 µs | 119 µs |
| D462 | 3.17 µs | 24.6 µs | 90.5 µs | 172 µs | 254 µs |
| D616 | 3.23 µs | 40.8 µs | 136 µs | 231 µs | 316 µs |
| D924 | 3.45 µs | 71.2 µs | 293 µs | 381 µs | 1 ms |
| D1232 | 4.48 µs | 138 µs | 423 µs | 930 µs | 3.15 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,112.5 160.5,118.6 196.7,103.3 232.9,112.4 269.1,115.4 305.3,109.8 341.5,110.0 377.6,109.8 413.8,108.9 450.0,105.7 450.0,24.3 413.8,38.5 377.6,52.9 341.5,55.6 305.3,65.0 269.1,68.9 232.9,78.3 196.7,83.2 160.5,97.0 124.4,94.6 88.2,104.5 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,112.5 160.5,118.6 196.7,103.3 232.9,112.4 269.1,115.4 305.3,109.8 341.5,110.0 377.6,109.8 413.8,108.9 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.0 88.2,108.6 124.4,104.9 160.5,99.6 196.7,92.5 232.9,97.3 269.1,92.1 305.3,92.6 341.5,84.6 377.6,78.3 413.8,71.4 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.6 88.2,107.1 124.4,99.1 160.5,98.2 196.7,92.2 232.9,89.2 269.1,84.8 305.3,77.8 341.5,68.4 377.6,63.4 413.8,53.8 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,104.5 124.4,97.6 160.5,96.9 196.7,85.6 232.9,91.6 269.1,77.4 305.3,69.7 341.5,60.4 377.6,56.8 413.8,50.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.5 124.4,94.6 160.5,97.0 196.7,83.2 232.9,78.3 269.1,68.9 305.3,65.0 341.5,55.6 377.6,52.9 413.8,38.5 450.0,24.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 93.5 ns | 131 ns | 160 ns | 161 ns |
| D38 | 4.36 ns | 154 ns | 159 ns | 195 ns | 183 ns |
| D57 | 181 ns | 240 ns | 320 ns | 318 ns | 409 ns |
| D76 | 106 ns | 340 ns | 333 ns | 351 ns | 310 ns |
| D115 | 397 ns | 583 ns | 707 ns | 793 ns | 875 ns |
| D153 | 413 ns | 699 ns | 820 ns | 523 ns | 1.14 µs |
| D230 | 381 ns | 756 ns | 992 ns | 1.29 µs | 1.97 µs |
| D307 | 852 ns | 829 ns | 1.27 µs | 2.08 µs | 2.58 µs |
| D462 | 864 ns | 1.38 µs | 2.29 µs | 3.24 µs | 4.18 µs |
| D616 | 1.07 µs | 2.05 µs | 2.93 µs | 3.53 µs | 4.19 µs |
| D924 | 1.47 µs | 2.38 µs | 4.88 µs | 4.68 µs | 11.3 µs |
| D1232 | 2.29 µs | 4.11 µs | 7.01 µs | 11.1 µs | 35.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,184.4 124.4,119.7 160.5,129.0 196.7,106.1 232.9,105.4 269.1,106.8 305.3,92.8 341.5,92.5 377.6,88.8 413.8,83.3 450.0,75.6 450.0,28.1 413.8,47.9 377.6,65.1 341.5,65.2 305.3,73.6 269.1,78.3 232.9,87.7 196.7,92.3 160.5,110.3 124.4,105.5 88.2,119.5 52.0,121.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,184.4 124.4,119.7 160.5,129.0 196.7,106.1 232.9,105.4 269.1,106.8 305.3,92.8 341.5,92.5 377.6,88.8 413.8,83.3 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,131.2 88.2,122.5 124.4,114.8 160.5,108.7 196.7,99.4 232.9,96.2 269.1,94.9 305.3,93.2 341.5,84.5 377.6,77.5 413.8,74.9 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.3 88.2,122.0 124.4,109.8 160.5,109.1 196.7,96.0 232.9,93.4 269.1,90.1 305.3,85.9 341.5,75.6 377.6,71.3 413.8,62.5 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,118.4 124.4,109.9 160.5,108.2 196.7,94.0 232.9,101.3 269.1,85.6 305.3,77.3 341.5,69.6 377.6,68.1 413.8,63.2 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.7 88.2,119.5 124.4,105.5 160.5,110.3 196.7,92.3 232.9,87.7 269.1,78.3 305.3,73.6 341.5,65.2 377.6,65.1 413.8,47.9 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 120 ns | 164 ns | 197 ns | 197 ns |
| D38 | 4.36 ns | 193 ns | 195 ns | 212 ns | 197 ns |
| D57 | 267 ns | 318 ns | 408 ns | 424 ns | 515 ns |
| D76 | 153 ns | 444 ns | 443 ns | 436 ns | 388 ns |
| D115 | 657 ns | 799 ns | 953 ns | 1.05 µs | 1.09 µs |
| D153 | 638 ns | 918 ns | 1.05 µs | 634 ns | 1.43 µs |
| D230 | 679 ns | 1.09 µs | 1.34 µs | 1.7 µs | 2.32 µs |
| D307 | 1.4 µs | 1.13 µs | 1.65 µs | 2.63 µs | 3.14 µs |
| D462 | 1.39 µs | 1.85 µs | 2.89 µs | 3.85 µs | 4.82 µs |
| D616 | 1.77 µs | 2.61 µs | 3.58 µs | 4.18 µs | 4.8 µs |
| D924 | 2.34 µs | 3.12 µs | 5.9 µs | 5.25 µs | 12.4 µs |
| D1232 | 3.55 µs | 5.31 µs | 8.29 µs | 12.6 µs | 36.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,112.9 160.5,122.6 196.7,97.3 232.9,97.8 269.1,96.7 305.3,84.2 341.5,84.3 377.6,80.1 413.8,75.2 450.0,68.0 450.0,27.4 413.8,46.3 377.6,62.7 341.5,62.7 305.3,70.1 269.1,75.4 232.9,83.7 196.7,88.6 160.5,106.4 124.4,101.5 88.2,118.2 52.0,118.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,112.9 160.5,122.6 196.7,97.3 232.9,97.8 269.1,96.7 305.3,84.2 341.5,84.3 377.6,80.1 413.8,75.2 450.0,68.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,126.8 88.2,118.6 124.4,109.9 160.5,104.1 196.7,93.9 232.9,91.5 269.1,88.4 305.3,87.9 341.5,79.3 377.6,73.3 413.8,70.3 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.4 88.2,118.4 124.4,105.6 160.5,104.2 196.7,90.8 232.9,89.1 269.1,84.9 305.3,81.3 341.5,71.6 377.6,67.8 413.8,59.2 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,117.0 124.4,104.9 160.5,104.4 196.7,89.1 232.9,97.9 269.1,80.8 305.3,73.2 341.5,66.6 377.6,65.2 413.8,61.2 450.0,46.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.2 124.4,101.5 160.5,106.4 196.7,88.6 232.9,83.7 269.1,75.4 305.3,70.1 341.5,62.7 377.6,62.7 413.8,46.3 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
