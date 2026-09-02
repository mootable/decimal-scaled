# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.01 ns | 1.35 µs | 2.52 µs | 3.12 µs | 3.75 µs |
| D38 | 3.38 µs | 7.31 µs | 7.4 µs | 11.3 µs | 14.7 µs |
| D57 | 5.13 µs | 9.66 µs | 11.8 µs | 15.5 µs | 10.9 µs |
| D76 | 3.99 µs | 9.89 µs | 15.1 µs | 14.9 µs | 20.9 µs |
| D115 | 5.15 µs | 11.7 µs | 26.8 µs | 34.1 µs | 44.6 µs |
| D153 | 5.12 µs | 15.5 µs | 28.2 µs | 45.1 µs | 65.5 µs |
| D230 | 5.29 µs | 22.2 µs | 31.7 µs | 70.3 µs | 122 µs |
| D307 | 4.8 µs | 26.3 µs | 65.9 µs | 116 µs | 180 µs |
| D462 | 3.72 µs | 26.2 µs | 134 µs | 240 µs | 400 µs |
| D616 | 5.37 µs | 67.4 µs | 204 µs | 425 µs | 711 µs |
| D924 | 4.54 µs | 128 µs | 451 µs | 832 µs | 1.68 ms |
| D1232 | 4.55 µs | 205 µs | 657 µs | 1.3 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.2 88.2,109.2 124.4,104.0 160.5,107.1 196.7,104.0 232.9,104.0 269.1,103.6 305.3,104.8 341.5,108.0 377.6,103.4 413.8,105.5 450.0,105.5 450.0,23.6 413.8,32.2 377.6,42.8 341.5,49.9 305.3,59.8 269.1,64.7 232.9,72.4 196.7,77.2 160.5,86.6 124.4,94.6 88.2,90.9 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.2 88.2,109.2 124.4,104.0 160.5,107.1 196.7,104.0 232.9,104.0 269.1,103.6 305.3,104.8 341.5,108.0 377.6,103.4 413.8,105.5 450.0,105.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.5 88.2,99.6 124.4,96.1 160.5,95.8 196.7,93.7 232.9,90.3 269.1,85.8 305.3,83.7 341.5,83.8 377.6,72.0 413.8,64.1 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,99.5 124.4,93.7 160.5,90.6 196.7,83.5 232.9,82.8 269.1,81.4 305.3,72.3 341.5,63.5 377.6,58.3 413.8,48.4 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,94.2 124.4,90.3 160.5,90.8 196.7,80.5 232.9,77.0 269.1,71.5 305.3,65.3 341.5,56.3 377.6,49.2 413.8,40.9 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,90.9 124.4,94.6 160.5,86.6 196.7,77.2 232.9,72.4 269.1,64.7 305.3,59.8 341.5,49.9 377.6,42.8 413.8,32.2 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.38 µs | 2.43 µs | 4.5 µs | 5.68 µs | 6.78 µs |
| D38 | 2.63 µs | 5.81 µs | 7 µs | 8.22 µs | 9.47 µs |
| D57 | 3.66 µs | 4.69 µs | 4.99 µs | 6.13 µs | 4.46 µs |
| D76 | 2.78 µs | 5.01 µs | 5.77 µs | 6.23 µs | 9.24 µs |
| D115 | 6.94 µs | 8.87 µs | 13.8 µs | 18.7 µs | 24.2 µs |
| D153 | 6.94 µs | 10.4 µs | 15.3 µs | 24.3 µs | 34.6 µs |
| D230 | 9.43 µs | 14.9 µs | 17.5 µs | 44.7 µs | 71.3 µs |
| D307 | 12.6 µs | 22 µs | 50.4 µs | 74.9 µs | 132 µs |
| D462 | 8.97 µs | 19.6 µs | 87.9 µs | 158 µs | 272 µs |
| D616 | 22.9 µs | 75.5 µs | 172 µs | 314 µs | 550 µs |
| D924 | 27.6 µs | 155 µs | 404 µs | 710 µs | 1.45 ms |
| D1232 | 35.9 µs | 262 µs | 684 µs | 1.21 ms | 2.93 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,189.0 124.4,181.8 160.5,187.8 196.7,167.9 232.9,167.9 269.1,161.3 305.3,154.9 341.5,162.4 377.6,142.0 413.8,137.9 450.0,132.3 450.0,36.7 413.8,52.0 377.6,73.0 341.5,88.3 305.3,104.0 269.1,117.3 232.9,133.1 196.7,140.8 160.5,161.7 124.4,177.5 88.2,161.2 52.0,168.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,189.0 124.4,181.8 160.5,187.8 196.7,167.9 232.9,167.9 269.1,161.3 305.3,154.9 341.5,162.4 377.6,142.0 413.8,137.9 450.0,132.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.7 88.2,171.8 124.4,176.4 160.5,175.0 196.7,162.6 232.9,159.2 269.1,151.3 305.3,142.9 341.5,145.4 377.6,116.1 413.8,100.4 450.0,89.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,177.4 88.2,167.8 124.4,175.1 160.5,172.0 196.7,153.0 232.9,150.8 269.1,147.8 305.3,124.9 341.5,112.8 377.6,98.2 413.8,79.7 450.0,68.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,172.3 88.2,164.3 124.4,170.6 160.5,170.3 196.7,146.5 232.9,140.7 269.1,127.5 305.3,116.3 341.5,100.0 377.6,85.2 413.8,67.4 450.0,55.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.4 88.2,161.2 124.4,177.5 160.5,161.7 196.7,140.8 232.9,133.1 269.1,117.3 305.3,104.0 341.5,88.3 377.6,73.0 413.8,52.0 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 1.34 µs | 2.46 µs | 3.1 µs | 3.73 µs |
| D38 | 3.35 µs | 7.31 µs | 7.34 µs | 11.3 µs | 14.9 µs |
| D57 | 5 µs | 9.64 µs | 11.8 µs | 15.5 µs | 10.9 µs |
| D76 | 3.95 µs | 9.89 µs | 15 µs | 15 µs | 20.8 µs |
| D115 | 5.03 µs | 11.7 µs | 26.8 µs | 34.5 µs | 45.1 µs |
| D153 | 5 µs | 15.5 µs | 28.5 µs | 46.2 µs | 66.1 µs |
| D230 | 5.13 µs | 22.3 µs | 31 µs | 69.7 µs | 122 µs |
| D307 | 4.69 µs | 26 µs | 65.8 µs | 115 µs | 180 µs |
| D462 | 3.7 µs | 26.5 µs | 132 µs | 241 µs | 398 µs |
| D616 | 5.19 µs | 66.6 µs | 205 µs | 426 µs | 715 µs |
| D924 | 4.42 µs | 127 µs | 453 µs | 834 µs | 1.68 ms |
| D1232 | 4.46 µs | 206 µs | 657 µs | 1.3 ms | 3.35 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,109.3 124.4,104.3 160.5,107.2 196.7,104.2 232.9,104.3 269.1,104.0 305.3,105.1 341.5,108.1 377.6,103.9 413.8,105.8 450.0,105.7 450.0,23.6 413.8,32.2 377.6,42.7 341.5,50.0 305.3,59.9 269.1,64.7 232.9,72.3 196.7,77.0 160.5,86.6 124.4,94.7 88.2,90.8 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,109.3 124.4,104.3 160.5,107.2 196.7,104.2 232.9,104.3 269.1,104.0 305.3,105.1 341.5,108.1 377.6,103.9 413.8,105.8 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.7 88.2,99.6 124.4,96.2 160.5,95.9 196.7,93.8 232.9,90.3 269.1,85.7 305.3,83.9 341.5,83.6 377.6,72.2 413.8,64.2 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.1 88.2,99.6 124.4,93.7 160.5,90.7 196.7,83.5 232.9,82.7 269.1,81.7 305.3,72.3 341.5,63.7 377.6,58.3 413.8,48.4 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,94.2 124.4,90.3 160.5,90.7 196.7,80.3 232.9,76.7 269.1,71.6 305.3,65.4 341.5,56.2 377.6,49.2 413.8,40.8 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,90.8 124.4,94.7 160.5,86.6 196.7,77.0 232.9,72.3 269.1,64.7 305.3,59.9 341.5,50.0 377.6,42.7 413.8,32.2 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.17 µs | 1.97 µs | 2.54 µs | 2.95 µs |
| D38 | 3.27 ns | 2.53 µs | 3.08 µs | 3.66 µs | 4.19 µs |
| D57 | 2.11 ns | 5.89 µs | 7.44 µs | 9.1 µs | 7.02 µs |
| D76 | 1.48 ns | 6.81 µs | 8.72 µs | 9.58 µs | 14.1 µs |
| D115 | 12.4 ns | 12.7 µs | 20.2 µs | 26.4 µs | 36.9 µs |
| D153 | 16 ns | 15 µs | 22 µs | 36.6 µs | 48.1 µs |
| D230 | 31.6 ns | 21.6 µs | 27.6 µs | 62.6 µs | 96 µs |
| D307 | 48.4 ns | 33 µs | 74.3 µs | 103 µs | 166 µs |
| D462 | 67.9 ns | 31.6 µs | 126 µs | 197 µs | 339 µs |
| D616 | 96.3 ns | 114 µs | 224 µs | 402 µs | 670 µs |
| D924 | 79.1 ns | 232 µs | 532 µs | 795 µs | 1.6 ms |
| D1232 | 124 ns | 364 µs | 869 µs | 1.38 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,195.3 124.4,200.7 160.5,205.1 196.7,178.7 232.9,175.6 269.1,167.1 305.3,161.9 341.5,157.7 377.6,153.3 413.8,155.8 450.0,150.2 450.0,25.4 413.8,32.8 377.6,43.5 341.5,52.0 305.3,60.8 269.1,67.6 232.9,76.2 196.7,79.5 160.5,91.5 124.4,100.1 88.2,106.5 52.0,110.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,195.3 124.4,200.7 160.5,205.1 196.7,178.7 232.9,175.6 269.1,167.1 305.3,161.9 341.5,157.7 377.6,153.3 413.8,155.8 450.0,150.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.3 88.2,112.8 124.4,102.3 160.5,100.5 196.7,92.7 232.9,90.7 269.1,86.2 305.3,80.9 341.5,81.4 377.6,65.6 413.8,56.7 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.9 88.2,110.3 124.4,99.4 160.5,97.4 196.7,87.0 232.9,85.9 269.1,83.1 305.3,70.8 341.5,64.3 377.6,57.2 413.8,46.4 450.0,40.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,108.2 124.4,96.9 160.5,96.2 196.7,83.6 232.9,79.6 269.1,73.0 305.3,66.7 341.5,58.7 377.6,49.9 413.8,41.4 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,106.5 124.4,100.1 160.5,91.5 196.7,79.5 232.9,76.2 269.1,67.6 305.3,60.8 341.5,52.0 377.6,43.5 413.8,32.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.4 ns | 1.08 µs | 2.08 µs | 2.64 µs | 3.21 µs |
| D38 | 4.28 µs | 7.94 µs | 6.12 µs | 9.85 µs | 12.9 µs |
| D57 | 4.13 µs | 8.44 µs | 10.2 µs | 13.7 µs | 3.01 µs |
| D76 | 3.22 µs | 8.55 µs | 13.1 µs | 13.2 µs | 18.6 µs |
| D115 | 4.38 µs | 10.1 µs | 23.3 µs | 31.1 µs | 41 µs |
| D153 | 4.15 µs | 13.5 µs | 22.4 µs | 40.6 µs | 60.1 µs |
| D230 | 4.23 µs | 19.9 µs | 27.9 µs | 64.9 µs | 114 µs |
| D307 | 3.9 µs | 23.1 µs | 54 µs | 107 µs | 167 µs |
| D462 | 2.2 µs | 21.1 µs | 112 µs | 211 µs | 338 µs |
| D616 | 4.28 µs | 60.4 µs | 191 µs | 395 µs | 671 µs |
| D924 | 3.7 µs | 117 µs | 425 µs | 796 µs | 1.59 ms |
| D1232 | 3.73 µs | 191 µs | 622 µs | 1.24 ms | 3.21 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,106.3 124.4,106.7 160.5,109.8 196.7,106.0 232.9,106.6 269.1,106.4 305.3,107.4 341.5,114.5 377.6,106.2 413.8,108.1 450.0,108.0 450.0,24.1 413.8,32.8 377.6,43.5 341.5,52.0 305.3,60.8 269.1,65.5 232.9,73.5 196.7,78.2 160.5,88.0 124.4,110.6 88.2,92.6 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,106.3 124.4,106.7 160.5,109.8 196.7,106.0 232.9,106.6 269.1,106.4 305.3,107.4 341.5,114.5 377.6,106.2 413.8,108.1 450.0,108.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.4 88.2,98.6 124.4,97.8 160.5,97.7 196.7,95.6 232.9,92.0 269.1,87.2 305.3,85.3 341.5,86.4 377.6,73.4 413.8,65.2 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.2 88.2,101.8 124.4,95.5 160.5,92.4 196.7,85.2 232.9,85.7 269.1,83.0 305.3,74.8 341.5,65.7 377.6,59.1 413.8,49.2 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,95.9 124.4,91.8 160.5,92.3 196.7,81.6 232.9,78.3 269.1,72.5 305.3,66.3 341.5,57.9 377.6,50.1 413.8,41.4 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,92.6 124.4,110.6 160.5,88.0 196.7,78.2 232.9,73.5 269.1,65.5 305.3,60.8 341.5,52.0 377.6,43.5 413.8,32.8 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.19 µs | 2.23 µs | 2.85 µs | 3.43 µs |
| D38 | 3.27 ns | 2.88 µs | 3.56 µs | 4.12 µs | 4.79 µs |
| D57 | 529 ns | 6.03 µs | 7.09 µs | 8.85 µs | 6.71 µs |
| D76 | 325 ns | 6.56 µs | 8.02 µs | 9.15 µs | 14 µs |
| D115 | 1.13 µs | 12.6 µs | 20.3 µs | 28.6 µs | 37.7 µs |
| D153 | 1.09 µs | 14.3 µs | 23.4 µs | 37.9 µs | 56.4 µs |
| D230 | 1.39 µs | 21.3 µs | 27.4 µs | 74.4 µs | 122 µs |
| D307 | 1.84 µs | 33 µs | 81.7 µs | 127 µs | 234 µs |
| D462 | 1.46 µs | 30.4 µs | 150 µs | 282 µs | 491 µs |
| D616 | 3.69 µs | 122 µs | 305 µs | 563 µs | 1.01 ms |
| D924 | 4.04 µs | 263 µs | 720 µs | 1.31 ms | 2.7 ms |
| D1232 | 5.67 µs | 449 µs | 1.23 ms | 2.25 ms | 5.52 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,195.3 124.4,132.2 160.5,138.2 196.7,122.7 232.9,123.2 269.1,120.2 305.3,116.7 341.5,119.6 377.6,108.1 413.8,107.0 450.0,102.8 450.0,17.4 413.8,26.2 377.6,38.4 341.5,47.4 305.3,56.6 269.1,64.7 232.9,74.3 196.7,79.2 160.5,91.6 124.4,100.7 88.2,104.8 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,195.3 124.4,132.2 160.5,138.2 196.7,122.7 232.9,123.2 269.1,120.2 305.3,116.7 341.5,119.6 377.6,108.1 413.8,107.0 450.0,102.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.1 88.2,111.2 124.4,102.0 160.5,101.0 196.7,92.8 232.9,91.3 269.1,86.3 305.3,80.9 341.5,81.9 377.6,64.7 413.8,55.2 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.3 88.2,108.5 124.4,100.0 160.5,98.4 196.7,86.9 232.9,85.2 269.1,83.2 305.3,69.6 341.5,62.1 377.6,53.3 413.8,42.7 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,106.7 124.4,97.2 160.5,96.8 196.7,82.7 232.9,79.2 269.1,70.8 305.3,64.2 341.5,54.3 377.6,45.7 413.8,35.2 450.0,28.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.8 124.4,100.7 160.5,91.6 196.7,79.2 232.9,74.3 269.1,64.7 305.3,56.6 341.5,47.4 377.6,38.4 413.8,26.2 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.76 ns | 864 ns | 1.7 µs | 2.03 µs | 2.82 µs |
| D38 | 4.36 ns | 2.51 µs | 2.98 µs | 3.45 µs | 3.79 µs |
| D57 | 2.81 ns | 3.78 µs | 4.76 µs | 5.59 µs | 5.18 µs |
| D76 | 3.01 ns | 4.07 µs | 5.46 µs | 5.8 µs | 9.18 µs |
| D115 | 13 ns | 4.66 µs | 10.1 µs | 14.5 µs | 19.2 µs |
| D153 | 18.2 ns | 5.47 µs | 9.92 µs | 19.1 µs | 32.7 µs |
| D230 | 40.7 ns | 8.7 µs | 11.6 µs | 36.2 µs | 69.1 µs |
| D307 | 64.3 ns | 12 µs | 27.6 µs | 62.6 µs | 111 µs |
| D462 | 107 ns | 9.64 µs | 65.4 µs | 134 µs | 229 µs |
| D616 | 198 ns | 33.1 µs | 122 µs | 260 µs | 454 µs |
| D924 | 110 ns | 71.9 µs | 273 µs | 560 µs | 1.14 ms |
| D1232 | 286 ns | 122 µs | 423 µs | 883 µs | 2.38 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,191.7 124.4,197.2 160.5,196.3 196.7,178.1 232.9,174.0 269.1,164.0 305.3,158.3 341.5,152.0 377.6,144.4 413.8,151.7 450.0,139.8 450.0,27.8 413.8,36.9 377.6,48.4 341.5,56.9 305.3,65.9 269.1,71.7 232.9,81.0 196.7,87.6 160.5,96.8 124.4,103.9 88.2,107.8 52.0,111.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,191.7 124.4,197.2 160.5,196.3 196.7,178.1 232.9,174.0 269.1,164.0 305.3,158.3 341.5,152.0 377.6,144.4 413.8,151.7 450.0,139.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,126.1 88.2,112.9 124.4,107.8 160.5,106.9 196.7,105.2 232.9,103.2 269.1,97.4 305.3,93.4 341.5,96.2 377.6,80.8 413.8,71.2 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.7 88.2,110.7 124.4,104.9 160.5,103.2 196.7,95.6 232.9,95.8 269.1,93.8 305.3,83.1 341.5,72.4 377.6,64.7 413.8,54.7 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.5 88.2,108.9 124.4,102.9 160.5,102.5 196.7,91.1 232.9,87.7 269.1,79.8 305.3,73.0 341.5,63.5 377.6,55.3 413.8,45.8 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,107.8 124.4,103.9 160.5,96.8 196.7,87.6 232.9,81.0 269.1,71.7 305.3,65.9 341.5,56.9 377.6,48.4 413.8,36.9 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.26 µs | 2.39 µs | 2.94 µs | 3.83 µs |
| D38 | 3.54 ns | 3.37 µs | 3.99 µs | 4.45 µs | 4.93 µs |
| D57 | 3.16 ns | 5.98 µs | 7.26 µs | 8.13 µs | 6.19 µs |
| D76 | 3.71 ns | 6.34 µs | 7.68 µs | 8.55 µs | 12.2 µs |
| D115 | 10.9 ns | 12.2 µs | 13.1 µs | 23.5 µs | 28.6 µs |
| D153 | 18 ns | 7.85 µs | 15.7 µs | 24.6 µs | 40 µs |
| D230 | 40.4 ns | 12.4 µs | 15 µs | 43.5 µs | 80.6 µs |
| D307 | 65.2 ns | 15.3 µs | 53.2 µs | 75.2 µs | 118 µs |
| D462 | 119 ns | 14.1 µs | 87 µs | 157 µs | 250 µs |
| D616 | 134 ns | 40.1 µs | 134 µs | 270 µs | 450 µs |
| D924 | 116 ns | 83.1 µs | 291 µs | 552 µs | 995 µs |
| D1232 | 304 ns | 134 µs | 412 µs | 779 µs | 2.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,194.3 124.4,195.7 160.5,193.7 196.7,180.4 232.9,174.1 269.1,164.1 305.3,158.2 341.5,150.7 377.6,149.2 413.8,151.0 450.0,139.1 450.0,25.0 413.8,38.6 377.6,48.5 341.5,55.8 305.3,65.1 269.1,69.8 232.9,78.5 196.7,82.7 160.5,93.3 124.4,101.7 88.2,104.5 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,194.3 124.4,195.7 160.5,193.7 196.7,180.4 232.9,174.1 269.1,164.1 305.3,158.2 341.5,150.7 377.6,149.2 413.8,151.0 450.0,139.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.4 88.2,109.2 124.4,102.1 160.5,101.4 196.7,93.2 232.9,98.7 269.1,93.1 305.3,90.4 341.5,91.5 377.6,78.5 413.8,69.4 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.5 88.2,107.1 124.4,99.7 160.5,99.0 196.7,92.4 232.9,90.1 269.1,90.7 305.3,75.0 341.5,68.9 377.6,63.5 413.8,53.9 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,105.8 124.4,98.3 160.5,97.7 196.7,85.1 232.9,84.5 269.1,77.5 305.3,70.7 341.5,61.5 377.6,54.8 413.8,45.9 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,104.5 124.4,101.7 160.5,93.3 196.7,82.7 232.9,78.5 269.1,69.8 305.3,65.1 341.5,55.8 377.6,48.5 413.8,38.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 765 ns | 1.56 µs | 2 µs | 2.77 µs |
| D38 | 3.82 ns | 2.34 µs | 2.82 µs | 3.28 µs | 3.75 µs |
| D57 | 2.81 ns | 3.58 µs | 4.42 µs | 5.62 µs | 5.18 µs |
| D76 | 3.3 ns | 3.87 µs | 5.19 µs | 5.59 µs | 8.87 µs |
| D115 | 13 ns | 4.33 µs | 10.4 µs | 14.3 µs | 19.1 µs |
| D153 | 18.2 ns | 5.13 µs | 9.71 µs | 19 µs | 32.4 µs |
| D230 | 40.8 ns | 8.92 µs | 12.5 µs | 35.9 µs | 66.1 µs |
| D307 | 60 ns | 10.9 µs | 26.6 µs | 61.2 µs | 108 µs |
| D462 | 101 ns | 9.62 µs | 62.3 µs | 133 µs | 229 µs |
| D616 | 125 ns | 32 µs | 120 µs | 254 µs | 456 µs |
| D924 | 110 ns | 69.9 µs | 270 µs | 555 µs | 1.14 ms |
| D1232 | 280 ns | 121 µs | 423 µs | 879 µs | 2.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,193.4 124.4,197.2 160.5,195.2 196.7,178.1 232.9,174.0 269.1,164.0 305.3,159.2 341.5,152.7 377.6,150.1 413.8,151.7 450.0,140.1 450.0,27.9 413.8,36.9 377.6,48.3 341.5,56.9 305.3,66.2 269.1,72.3 232.9,81.1 196.7,87.7 160.5,97.2 124.4,103.9 88.2,107.9 52.0,111.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,193.4 124.4,197.2 160.5,195.2 196.7,178.1 232.9,174.0 269.1,164.0 305.3,159.2 341.5,152.7 377.6,150.1 413.8,151.7 450.0,140.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,127.6 88.2,113.7 124.4,108.5 160.5,107.5 196.7,106.1 232.9,104.0 269.1,97.1 305.3,94.7 341.5,96.2 377.6,81.3 413.8,71.6 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.7 88.2,111.4 124.4,105.8 160.5,103.9 196.7,95.2 232.9,96.1 269.1,92.9 305.3,83.6 341.5,73.0 377.6,64.9 413.8,54.8 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,109.5 124.4,102.9 160.5,102.9 196.7,91.2 232.9,87.8 269.1,79.8 305.3,73.2 341.5,63.6 377.6,55.6 413.8,45.9 450.0,40.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,107.9 124.4,103.9 160.5,97.2 196.7,87.7 232.9,81.1 269.1,72.3 305.3,66.2 341.5,56.9 377.6,48.3 413.8,36.9 450.0,27.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.27 µs | 2.39 µs | 2.95 µs | 3.86 µs |
| D38 | 3.54 ns | 3.37 µs | 3.97 µs | 4.47 µs | 4.94 µs |
| D57 | 12.3 ns | 5.97 µs | 7.26 µs | 8.13 µs | 6.21 µs |
| D76 | 9.82 ns | 6.33 µs | 7.71 µs | 8.55 µs | 12.2 µs |
| D115 | 11.3 ns | 12.1 µs | 12.7 µs | 23.4 µs | 28.4 µs |
| D153 | 17.4 ns | 7.92 µs | 15.2 µs | 25.1 µs | 39.3 µs |
| D230 | 39.9 ns | 12.4 µs | 15.1 µs | 43.5 µs | 80.6 µs |
| D307 | 60.5 ns | 15.3 µs | 53.1 µs | 75.4 µs | 118 µs |
| D462 | 103 ns | 14 µs | 87.7 µs | 158 µs | 251 µs |
| D616 | 135 ns | 40.1 µs | 134 µs | 271 µs | 448 µs |
| D924 | 125 ns | 83.5 µs | 292 µs | 552 µs | 996 µs |
| D1232 | 288 ns | 133 µs | 412 µs | 778 µs | 2.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,194.3 124.4,178.9 160.5,181.7 196.7,180.0 232.9,174.6 269.1,164.2 305.3,159.1 341.5,152.5 377.6,149.1 413.8,150.1 450.0,139.7 450.0,25.0 413.8,38.6 377.6,48.5 341.5,55.7 305.3,65.1 269.1,69.8 232.9,78.7 196.7,82.7 160.5,93.2 124.4,101.6 88.2,104.5 52.0,107.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,194.3 124.4,178.9 160.5,181.7 196.7,180.0 232.9,174.6 269.1,164.2 305.3,159.1 341.5,152.5 377.6,149.1 413.8,150.1 450.0,139.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.3 88.2,109.2 124.4,102.1 160.5,101.4 196.7,93.3 232.9,98.6 269.1,93.0 305.3,90.4 341.5,91.5 377.6,78.5 413.8,69.4 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.5 88.2,107.2 124.4,99.7 160.5,98.9 196.7,92.7 232.9,90.5 269.1,90.6 305.3,75.0 341.5,68.8 377.6,63.5 413.8,53.9 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,105.7 124.4,98.3 160.5,97.7 196.7,85.2 232.9,84.3 269.1,77.5 305.3,70.6 341.5,61.5 377.6,54.8 413.8,45.9 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,104.5 124.4,101.6 160.5,93.2 196.7,82.7 232.9,78.7 269.1,69.8 305.3,65.1 341.5,55.7 377.6,48.5 413.8,38.6 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 1.57 µs | 2.91 µs | 3.59 µs | 4.54 µs |
| D38 | 3.82 ns | 3.97 µs | 4.72 µs | 5.48 µs | 6.1 µs |
| D57 | 2.81 ns | 4.8 µs | 5.81 µs | 7.15 µs | 5.25 µs |
| D76 | 3.63 ns | 5.11 µs | 6.88 µs | 7.33 µs | 11 µs |
| D115 | 10.4 ns | 5.91 µs | 13.2 µs | 17.5 µs | 23.9 µs |
| D153 | 18.1 ns | 6.82 µs | 12.4 µs | 22.8 µs | 37.8 µs |
| D230 | 41.1 ns | 11.4 µs | 14.2 µs | 41.6 µs | 76.1 µs |
| D307 | 58.4 ns | 13.3 µs | 31.8 µs | 70.3 µs | 120 µs |
| D462 | 136 ns | 11.7 µs | 70.9 µs | 146 µs | 248 µs |
| D616 | 120 ns | 37.7 µs | 133 µs | 280 µs | 487 µs |
| D924 | 107 ns | 78.6 µs | 300 µs | 596 µs | 1.21 ms |
| D1232 | 274 ns | 133 µs | 457 µs | 948 µs | 2.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,193.4 124.4,197.2 160.5,194.0 196.7,180.9 232.9,174.1 269.1,163.9 305.3,159.5 341.5,149.0 377.6,150.6 413.8,152.0 450.0,140.3 450.0,27.3 413.8,36.2 377.6,47.5 341.5,55.9 305.3,64.9 269.1,70.5 232.9,79.2 196.7,84.9 160.5,94.6 124.4,103.7 88.2,101.9 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,193.4 124.4,197.2 160.5,194.0 196.7,180.9 232.9,174.1 269.1,163.9 305.3,159.5 341.5,149.0 377.6,150.6 413.8,152.0 450.0,140.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,107.2 124.4,104.8 160.5,104.0 196.7,102.2 232.9,100.5 269.1,94.1 305.3,92.2 341.5,93.7 377.6,79.3 413.8,70.1 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,105.0 124.4,102.5 160.5,100.3 196.7,92.2 232.9,93.1 269.1,91.4 305.3,81.4 341.5,71.4 377.6,63.6 413.8,53.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.4 88.2,103.2 124.4,99.9 160.5,99.6 196.7,88.8 232.9,85.5 269.1,78.0 305.3,71.5 341.5,62.4 377.6,54.4 413.8,45.0 450.0,39.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,101.9 124.4,103.7 160.5,94.6 196.7,84.9 232.9,79.2 269.1,70.5 305.3,64.9 341.5,55.9 377.6,47.5 413.8,36.2 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.29 µs | 2.53 µs | 3.1 µs | 4.01 µs |
| D38 | 3.27 ns | 3.56 µs | 4.01 µs | 4.47 µs | 4.96 µs |
| D57 | 2.83 µs | 6.16 µs | 7.55 µs | 8.58 µs | 6.4 µs |
| D76 | 2.21 µs | 6.54 µs | 8.09 µs | 8.76 µs | 12.7 µs |
| D115 | 5.73 µs | 12.8 µs | 13.4 µs | 24.1 µs | 29.8 µs |
| D153 | 3 µs | 8.31 µs | 16.1 µs | 25.1 µs | 40.4 µs |
| D230 | 3.11 µs | 12.8 µs | 15.8 µs | 44.5 µs | 81.5 µs |
| D307 | 2.98 µs | 16 µs | 55.6 µs | 76.7 µs | 119 µs |
| D462 | 2.37 µs | 14.8 µs | 88.4 µs | 160 µs | 255 µs |
| D616 | 3.59 µs | 40.8 µs | 136 µs | 273 µs | 454 µs |
| D924 | 3.13 µs | 84.4 µs | 295 µs | 557 µs | 1.01 ms |
| D1232 | 3.66 µs | 136 µs | 417 µs | 785 µs | 3.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,195.3 124.4,111.4 160.5,114.4 196.7,102.6 232.9,110.7 269.1,110.2 305.3,110.7 341.5,113.6 377.6,108.4 413.8,110.1 450.0,108.2 450.0,24.9 413.8,38.5 377.6,48.4 341.5,55.5 305.3,65.0 269.1,69.7 232.9,78.4 196.7,82.2 160.5,92.8 124.4,101.2 88.2,104.4 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,195.3 124.4,111.4 160.5,114.4 196.7,102.6 232.9,110.7 269.1,110.2 305.3,110.7 341.5,113.6 377.6,108.4 413.8,110.1 450.0,108.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.2 88.2,108.5 124.4,101.7 160.5,101.0 196.7,92.6 232.9,98.0 269.1,92.6 305.3,89.9 341.5,90.9 377.6,78.3 413.8,69.3 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,107.1 124.4,99.2 160.5,98.3 196.7,92.1 232.9,89.8 269.1,90.1 305.3,74.4 341.5,68.7 377.6,63.4 413.8,53.7 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.2 88.2,105.7 124.4,97.6 160.5,97.4 196.7,84.8 232.9,84.3 269.1,77.2 305.3,70.4 341.5,61.3 377.6,54.7 413.8,45.8 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.4 124.4,101.2 160.5,92.8 196.7,82.2 232.9,78.4 269.1,69.7 305.3,65.0 341.5,55.5 377.6,48.4 413.8,38.5 450.0,24.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 79.3 ns | 113 ns | 132 ns | 164 ns |
| D38 | 3.82 ns | 159 ns | 167 ns | 181 ns | 189 ns |
| D57 | 199 ns | 341 ns | 321 ns | 320 ns | 243 ns |
| D76 | 134 ns | 331 ns | 330 ns | 332 ns | 442 ns |
| D115 | 463 ns | 558 ns | 772 ns | 860 ns | 955 ns |
| D153 | 470 ns | 618 ns | 739 ns | 987 ns | 1.15 µs |
| D230 | 571 ns | 747 ns | 642 ns | 1.3 µs | 1.82 µs |
| D307 | 769 ns | 1.06 µs | 1.68 µs | 1.98 µs | 2.63 µs |
| D462 | 626 ns | 782 ns | 2.3 µs | 3.01 µs | 4.12 µs |
| D616 | 1.17 µs | 2.02 µs | 2.89 µs | 4.16 µs | 6.06 µs |
| D924 | 1.2 µs | 2.77 µs | 4.88 µs | 6.83 µs | 11.3 µs |
| D1232 | 1.84 µs | 4.14 µs | 6.96 µs | 9.29 µs | 33.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,186.7 124.4,118.1 160.5,124.9 196.7,103.4 232.9,103.1 269.1,99.7 305.3,94.6 341.5,98.1 377.6,87.3 413.8,86.9 450.0,79.4 450.0,29.1 413.8,47.9 377.6,58.7 341.5,65.4 305.3,73.2 269.1,79.6 232.9,87.6 196.7,90.8 160.5,104.2 124.4,114.6 88.2,119.0 52.0,121.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,186.7 124.4,118.1 160.5,124.9 196.7,103.4 232.9,103.1 269.1,99.7 305.3,94.6 341.5,98.1 377.6,87.3 413.8,86.9 450.0,79.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,134.0 88.2,122.0 124.4,108.7 160.5,109.2 196.7,100.1 232.9,98.4 269.1,95.1 305.3,89.0 341.5,94.3 377.6,77.8 413.8,72.3 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,127.9 88.2,121.1 124.4,109.7 160.5,109.2 196.7,94.5 232.9,95.3 269.1,97.7 305.3,81.0 341.5,75.5 377.6,71.6 413.8,62.5 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.1 88.2,119.7 124.4,109.8 160.5,109.2 196.7,92.6 232.9,90.2 269.1,85.5 305.3,78.1 341.5,70.9 377.6,65.2 413.8,56.6 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.4 88.2,119.0 124.4,114.6 160.5,104.2 196.7,90.8 232.9,87.6 269.1,79.6 305.3,73.2 341.5,65.4 377.6,58.7 413.8,47.9 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 101 ns | 147 ns | 170 ns | 197 ns |
| D38 | 3.82 ns | 196 ns | 196 ns | 198 ns | 199 ns |
| D57 | 311 ns | 445 ns | 425 ns | 440 ns | 317 ns |
| D76 | 210 ns | 427 ns | 443 ns | 416 ns | 556 ns |
| D115 | 726 ns | 780 ns | 1 µs | 1.14 µs | 1.18 µs |
| D153 | 742 ns | 856 ns | 987 ns | 1.18 µs | 1.42 µs |
| D230 | 996 ns | 1.06 µs | 833 ns | 1.69 µs | 2.2 µs |
| D307 | 1.3 µs | 1.47 µs | 2.23 µs | 2.45 µs | 3.18 µs |
| D462 | 995 ns | 986 ns | 2.91 µs | 3.56 µs | 4.84 µs |
| D616 | 1.91 µs | 2.6 µs | 3.58 µs | 4.87 µs | 6.86 µs |
| D924 | 1.95 µs | 3.71 µs | 5.87 µs | 7.7 µs | 12.4 µs |
| D1232 | 2.84 µs | 5.27 µs | 8.23 µs | 10.5 µs | 34.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,186.7 124.4,110.3 160.5,117.1 196.7,95.6 232.9,95.2 269.1,90.1 305.3,85.5 341.5,90.1 377.6,78.7 413.8,78.4 450.0,71.9 450.0,28.3 413.8,46.3 377.6,56.6 341.5,62.6 305.3,69.9 269.1,76.3 232.9,83.9 196.7,87.1 160.5,100.2 124.4,110.0 88.2,118.0 52.0,118.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,186.7 124.4,110.3 160.5,117.1 196.7,95.6 232.9,95.2 269.1,90.1 305.3,85.5 341.5,90.1 377.6,78.7 413.8,78.4 450.0,71.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,129.8 88.2,118.3 124.4,104.1 160.5,104.8 196.7,94.3 232.9,92.7 269.1,89.0 305.3,83.3 341.5,90.2 377.6,73.4 413.8,67.2 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.3 88.2,118.3 124.4,104.9 160.5,104.1 196.7,89.9 232.9,90.2 269.1,93.2 305.3,76.1 341.5,71.5 377.6,67.8 413.8,59.2 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,118.1 124.4,104.3 160.5,105.2 196.7,87.7 232.9,87.1 269.1,80.9 305.3,74.4 341.5,67.9 377.6,62.5 413.8,54.6 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.2 88.2,118.0 124.4,110.0 160.5,100.2 196.7,87.1 232.9,83.9 269.1,76.3 305.3,69.9 341.5,62.6 377.6,56.6 413.8,46.3 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
