# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.21 ns | 2.09 µs | 3.21 µs | 3.21 µs | 3.77 µs |
| D38 | 1.41 µs | 4.62 µs | 4.1 µs | 8.61 µs | 10.4 µs |
| D57 | 1.54 µs | 6.56 µs | 8.73 µs | 10.6 µs | 15 µs |
| D76 | 1.58 µs | 7.16 µs | 8.2 µs | 8.45 µs | 18.4 µs |
| D115 | 1.45 µs | 8.82 µs | 20 µs | 32.4 µs | 42.3 µs |
| D153 | 1.52 µs | 10.6 µs | 27.3 µs | 25.9 µs | 37.2 µs |
| D230 | 1.68 µs | 17.8 µs | 45.5 µs | 60.4 µs | 133 µs |
| D307 | 1.6 µs | 26.8 µs | 65.6 µs | 116 µs | 193 µs |
| D462 | 1.56 µs | 45.6 µs | 134 µs | 200 µs | 374 µs |
| D616 | 1.57 µs | 61.7 µs | 178 µs | 427 µs | 708 µs |
| D924 | 1.81 µs | 137 µs | 451 µs | 762 µs | 1.68 ms |
| D1232 | 2.01 µs | 205 µs | 590 µs | 1.51 ms | 2.33 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.3 88.2,120.0 124.4,118.9 160.5,118.6 196.7,119.7 232.9,119.1 269.1,117.9 305.3,118.5 341.5,118.8 377.6,118.7 413.8,116.9 450.0,115.6 450.0,28.1 413.8,32.1 377.6,42.9 341.5,50.8 305.3,59.0 269.1,63.6 232.9,79.4 196.7,77.8 160.5,88.1 124.4,90.6 88.2,95.3 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.3 88.2,120.0 124.4,118.9 160.5,118.6 196.7,119.7 232.9,119.1 269.1,117.9 305.3,118.5 341.5,118.8 377.6,118.7 413.8,116.9 450.0,115.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.1 88.2,105.3 124.4,100.9 160.5,99.9 196.7,97.3 232.9,95.0 269.1,88.5 305.3,83.5 341.5,76.9 377.6,73.1 413.8,63.3 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.8 124.4,97.4 160.5,98.2 196.7,87.1 232.9,83.2 269.1,76.9 305.3,72.4 341.5,63.5 377.6,60.0 413.8,48.5 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,97.6 124.4,95.0 160.5,97.8 196.7,81.1 232.9,83.9 269.1,73.4 305.3,65.3 341.5,58.6 377.6,49.1 413.8,41.9 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,95.3 124.4,90.6 160.5,88.1 196.7,77.8 232.9,79.4 269.1,63.6 305.3,59.0 341.5,50.8 377.6,42.9 413.8,32.1 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 µs | 3.72 µs | 5.86 µs | 5.94 µs | 6.87 µs |
| D38 | 3.39 µs | 5.83 µs | 7.11 µs | 8.76 µs | 10.2 µs |
| D57 | 3.72 µs | 4.71 µs | 5.37 µs | 6.22 µs | 8.17 µs |
| D76 | 3.69 µs | 4.97 µs | 4.91 µs | 4.59 µs | 9.26 µs |
| D115 | 6.29 µs | 9.18 µs | 12.2 µs | 16.4 µs | 22.1 µs |
| D153 | 6.19 µs | 10.5 µs | 15.7 µs | 12.7 µs | 17.8 µs |
| D230 | 8.94 µs | 14 µs | 28.5 µs | 36.3 µs | 73.5 µs |
| D307 | 13.2 µs | 27.8 µs | 50.7 µs | 78.5 µs | 138 µs |
| D462 | 12.9 µs | 39.3 µs | 90.6 µs | 133 µs | 261 µs |
| D616 | 21.7 µs | 71.1 µs | 146 µs | 318 µs | 555 µs |
| D924 | 34 µs | 164 µs | 402 µs | 652 µs | 1.45 ms |
| D1232 | 42.8 µs | 258 µs | 591 µs | 1.44 ms | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.1 88.2,183.5 124.4,181.5 160.5,181.6 196.7,170.1 232.9,170.4 269.1,162.4 305.3,154.0 341.5,154.5 377.6,143.1 413.8,133.4 450.0,128.4 450.0,42.1 413.8,51.9 377.6,72.8 341.5,89.2 305.3,103.0 269.1,116.7 232.9,147.5 196.7,142.8 160.5,161.7 124.4,164.4 88.2,159.6 52.0,168.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.1 88.2,183.5 124.4,181.5 160.5,181.6 196.7,170.1 232.9,170.4 269.1,162.4 305.3,154.0 341.5,154.5 377.6,143.1 413.8,133.4 450.0,128.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.5 88.2,171.7 124.4,176.3 160.5,175.2 196.7,161.8 232.9,159.0 269.1,152.7 305.3,137.8 341.5,130.3 377.6,117.4 413.8,99.2 450.0,89.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.6 88.2,167.4 124.4,173.5 160.5,175.4 196.7,155.7 232.9,150.2 269.1,137.3 305.3,124.7 341.5,112.1 377.6,101.8 413.8,79.8 450.0,71.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.3 88.2,162.9 124.4,170.3 160.5,176.9 196.7,149.2 232.9,154.9 269.1,132.0 305.3,115.3 341.5,103.7 377.6,84.9 413.8,69.3 450.0,52.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.2 88.2,159.6 124.4,164.4 160.5,161.7 196.7,142.8 232.9,147.5 269.1,116.7 305.3,103.0 341.5,89.2 377.6,72.8 413.8,51.9 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.54 ns | 2.09 µs | 3.21 µs | 3.21 µs | 3.76 µs |
| D38 | 1.33 µs | 4.6 µs | 4.04 µs | 8.55 µs | 10.3 µs |
| D57 | 1.42 µs | 6.33 µs | 8.64 µs | 10.6 µs | 15 µs |
| D76 | 1.45 µs | 7.17 µs | 8.18 µs | 8.42 µs | 18.4 µs |
| D115 | 1.38 µs | 8.75 µs | 19.9 µs | 32.5 µs | 42.3 µs |
| D153 | 1.4 µs | 10.5 µs | 25.4 µs | 26.4 µs | 37.1 µs |
| D230 | 1.56 µs | 17.8 µs | 45.3 µs | 59.7 µs | 132 µs |
| D307 | 1.52 µs | 25.9 µs | 66.3 µs | 116 µs | 191 µs |
| D462 | 1.45 µs | 45.3 µs | 133 µs | 201 µs | 374 µs |
| D616 | 1.43 µs | 61.7 µs | 179 µs | 425 µs | 712 µs |
| D924 | 1.73 µs | 136 µs | 452 µs | 764 µs | 1.68 ms |
| D1232 | 1.88 µs | 205 µs | 589 µs | 1.51 ms | 2.33 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.3 88.2,120.8 124.4,119.9 160.5,119.7 196.7,120.3 232.9,120.1 269.1,118.7 305.3,119.1 341.5,119.7 377.6,119.8 413.8,117.5 450.0,116.5 450.0,28.1 413.8,32.1 377.6,42.8 341.5,50.8 305.3,59.1 269.1,63.7 232.9,79.4 196.7,77.8 160.5,88.2 124.4,90.7 88.2,95.4 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.3 88.2,120.8 124.4,119.9 160.5,119.7 196.7,120.3 232.9,120.1 269.1,118.7 305.3,119.1 341.5,119.7 377.6,119.8 413.8,117.5 450.0,116.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,105.3 124.4,101.4 160.5,99.8 196.7,97.4 232.9,95.1 269.1,88.6 305.3,83.9 341.5,77.0 377.6,73.1 413.8,63.4 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,107.0 124.4,97.5 160.5,98.2 196.7,87.2 232.9,84.1 269.1,77.0 305.3,72.3 341.5,63.6 377.6,59.9 413.8,48.4 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,97.7 124.4,95.0 160.5,97.8 196.7,81.1 232.9,83.7 269.1,73.5 305.3,65.3 341.5,58.5 377.6,49.2 413.8,41.9 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,95.4 124.4,90.7 160.5,88.2 196.7,77.8 232.9,79.4 269.1,63.7 305.3,59.1 341.5,50.8 377.6,42.8 413.8,32.1 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.76 µs | 2.55 µs | 2.6 µs | 2.97 µs |
| D38 | 4.22 ns | 2.52 µs | 3.1 µs | 3.93 µs | 4.52 µs |
| D57 | 2.81 ns | 5.86 µs | 7.91 µs | 9.08 µs | 12.6 µs |
| D76 | 3.29 ns | 6.74 µs | 7.19 µs | 6.98 µs | 13.8 µs |
| D115 | 8.1 ns | 13.5 µs | 19 µs | 24.2 µs | 33.4 µs |
| D153 | 10.6 ns | 16.3 µs | 23.2 µs | 20.3 µs | 26.7 µs |
| D230 | 19.5 ns | 21.4 µs | 44.3 µs | 53.1 µs | 102 µs |
| D307 | 28.5 ns | 40.2 µs | 71.7 µs | 103 µs | 176 µs |
| D462 | 41.4 ns | 58.4 µs | 125 µs | 161 µs | 314 µs |
| D616 | 44 ns | 103 µs | 194 µs | 404 µs | 668 µs |
| D924 | 34.4 ns | 244 µs | 532 µs | 741 µs | 1.61 ms |
| D1232 | 67.2 ns | 361 µs | 733 µs | 1.62 ms | 1.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,192.1 124.4,197.2 160.5,195.2 196.7,184.0 232.9,180.7 269.1,173.1 305.3,168.4 341.5,163.8 377.6,163.0 413.8,166.1 450.0,157.8 450.0,30.7 413.8,32.7 377.6,43.6 341.5,53.0 305.3,60.1 269.1,66.9 232.9,83.5 196.7,80.7 160.5,91.7 124.4,92.8 88.2,105.6 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,192.1 124.4,197.2 160.5,195.2 196.7,184.0 232.9,180.7 269.1,173.1 305.3,168.4 341.5,163.8 377.6,163.0 413.8,166.1 450.0,157.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,112.8 124.4,102.4 160.5,100.6 196.7,92.0 232.9,89.6 269.1,86.2 305.3,78.5 341.5,73.8 377.6,66.8 413.8,56.1 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.7 88.2,110.3 124.4,98.6 160.5,99.8 196.7,87.8 232.9,85.3 269.1,77.2 305.3,71.3 341.5,64.4 377.6,58.9 413.8,46.4 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.4 88.2,107.3 124.4,96.9 160.5,100.2 196.7,84.8 232.9,86.9 269.1,75.0 305.3,66.8 341.5,61.2 377.6,49.8 413.8,42.3 450.0,32.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,105.6 124.4,92.8 160.5,91.7 196.7,80.7 232.9,83.5 269.1,66.9 305.3,60.1 341.5,53.0 377.6,43.6 413.8,32.7 450.0,30.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.74 µs | 2.69 µs | 2.71 µs | 3.23 µs |
| D38 | 3.16 ns | 2.84 µs | 3.17 µs | 3.69 µs | 4.23 µs |
| D57 | 1.78 ns | 3.13 µs | 3.7 µs | 4.07 µs | 5.62 µs |
| D76 | 602 ns | 5.57 µs | 6.37 µs | 6.91 µs | 15.3 µs |
| D115 | 542 ns | 6.75 µs | 16.3 µs | 29.6 µs | 38.5 µs |
| D153 | 587 ns | 8.32 µs | 17.6 µs | 23.7 µs | 33.8 µs |
| D230 | 657 ns | 14.9 µs | 41.1 µs | 55.6 µs | 122 µs |
| D307 | 628 ns | 22.3 µs | 55.9 µs | 106 µs | 178 µs |
| D462 | 654 ns | 37.3 µs | 113 µs | 177 µs | 319 µs |
| D616 | 642 ns | 56.1 µs | 166 µs | 398 µs | 672 µs |
| D924 | 870 ns | 125 µs | 422 µs | 721 µs | 1.58 ms |
| D1232 | 1.01 µs | 191 µs | 552 µs | 1.45 ms | 2.21 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,195.7 124.4,202.9 160.5,130.6 196.7,131.9 232.9,130.9 269.1,129.5 305.3,130.1 341.5,129.5 377.6,129.8 413.8,126.0 450.0,124.1 450.0,28.7 413.8,32.9 377.6,43.5 341.5,52.7 305.3,60.0 269.1,64.6 232.9,80.6 196.7,79.0 160.5,90.5 124.4,102.9 88.2,106.4 52.0,109.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,195.7 124.4,202.9 160.5,130.6 196.7,131.9 232.9,130.9 269.1,129.5 305.3,130.1 341.5,129.5 377.6,129.8 413.8,126.0 450.0,124.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.4 88.2,111.3 124.4,110.1 160.5,103.0 196.7,100.6 232.9,98.0 269.1,90.7 305.3,85.8 341.5,79.4 377.6,74.3 413.8,64.4 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,110.0 124.4,108.0 160.5,101.3 196.7,89.7 232.9,88.7 269.1,78.2 305.3,74.4 341.5,65.6 377.6,60.8 413.8,49.3 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.9 88.2,108.1 124.4,106.9 160.5,100.3 196.7,82.2 232.9,85.0 269.1,74.4 305.3,66.4 341.5,60.1 377.6,50.0 413.8,42.6 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,106.4 124.4,102.9 160.5,90.5 196.7,79.0 232.9,80.6 269.1,64.6 305.3,60.0 341.5,52.7 377.6,43.5 413.8,32.9 450.0,28.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.78 µs | 2.88 µs | 2.93 µs | 3.44 µs |
| D38 | 4.22 ns | 2.85 µs | 3.57 µs | 4.36 µs | 5.12 µs |
| D57 | 496 ns | 6.08 µs | 7.49 µs | 8.76 µs | 12.1 µs |
| D76 | 510 ns | 6.59 µs | 6.8 µs | 6.66 µs | 14 µs |
| D115 | 922 ns | 12.9 µs | 17.9 µs | 25 µs | 33.7 µs |
| D153 | 939 ns | 14.6 µs | 23.6 µs | 19.3 µs | 28.4 µs |
| D230 | 1.39 µs | 20.3 µs | 43.4 µs | 60.7 µs | 126 µs |
| D307 | 2.12 µs | 42.5 µs | 83.5 µs | 134 µs | 244 µs |
| D462 | 2.04 µs | 60.2 µs | 155 µs | 239 µs | 476 µs |
| D616 | 3.57 µs | 117 µs | 249 µs | 562 µs | 1.02 ms |
| D924 | 5.3 µs | 278 µs | 720 µs | 1.2 ms | 2.7 ms |
| D1232 | 6.9 µs | 446 µs | 1.07 ms | 2.69 ms | 3.99 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.6 88.2,192.1 124.4,133.0 160.5,132.6 196.7,125.3 232.9,125.1 269.1,120.2 305.3,114.9 341.5,115.4 377.6,108.5 413.8,103.6 450.0,100.3 450.0,21.4 413.8,26.3 377.6,38.4 341.5,47.8 305.3,56.1 269.1,64.3 232.9,82.8 196.7,80.6 160.5,91.5 124.4,93.3 88.2,104.0 52.0,108.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.6 88.2,192.1 124.4,133.0 160.5,132.6 196.7,125.3 232.9,125.1 269.1,120.2 305.3,114.9 341.5,115.4 377.6,108.5 413.8,103.6 450.0,100.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,111.3 124.4,101.9 160.5,100.9 196.7,92.6 232.9,91.0 269.1,86.9 305.3,77.8 341.5,73.4 377.6,65.2 413.8,54.5 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,108.5 124.4,99.3 160.5,100.5 196.7,88.5 232.9,85.1 269.1,77.5 305.3,69.4 341.5,61.7 377.6,55.8 413.8,42.7 450.0,37.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.9 88.2,106.0 124.4,97.4 160.5,100.8 196.7,84.3 232.9,87.6 269.1,73.3 305.3,63.5 341.5,56.3 377.6,45.7 413.8,36.3 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,104.0 124.4,93.3 160.5,91.5 196.7,80.6 232.9,82.8 269.1,64.3 305.3,56.1 341.5,47.8 377.6,38.4 413.8,26.3 450.0,21.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.68 ns | 1.23 µs | 2.56 µs | 2.14 µs | 2.9 µs |
| D38 | 5.62 ns | 2.55 µs | 3.05 µs | 3.74 µs | 4.14 µs |
| D57 | 2.81 ns | 3.75 µs | 5.07 µs | 5.61 µs | 9.64 µs |
| D76 | 3.15 ns | 4.07 µs | 4.59 µs | 4.14 µs | 9.05 µs |
| D115 | 11.6 ns | 5.07 µs | 9.64 µs | 13.6 µs | 18.2 µs |
| D153 | 16.6 ns | 5.85 µs | 10.1 µs | 11.1 µs | 17.5 µs |
| D230 | 40.7 ns | 8.86 µs | 19.2 µs | 31.3 µs | 72.6 µs |
| D307 | 73.2 ns | 13 µs | 27.3 µs | 62.9 µs | 118 µs |
| D462 | 120 ns | 15.9 µs | 65.3 µs | 113 µs | 218 µs |
| D616 | 119 ns | 29.9 µs | 104 µs | 259 µs | 460 µs |
| D924 | 190 ns | 75.6 µs | 275 µs | 519 µs | 1.13 ms |
| D1232 | 371 ns | 122 µs | 384 µs | 1.04 ms | 1.65 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.8 88.2,188.6 124.4,197.2 160.5,195.8 196.7,179.6 232.9,175.2 269.1,164.0 305.3,156.7 341.5,150.6 377.6,150.7 413.8,144.9 450.0,136.6 450.0,32.3 413.8,37.0 377.6,48.2 341.5,57.5 305.3,65.0 269.1,71.1 232.9,88.8 196.7,88.3 160.5,97.0 124.4,96.2 88.2,106.7 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.8 88.2,188.6 124.4,197.2 160.5,195.8 196.7,179.6 232.9,175.2 269.1,164.0 305.3,156.7 341.5,150.6 377.6,150.7 413.8,144.9 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.7 88.2,112.7 124.4,107.9 160.5,106.9 196.7,104.1 232.9,102.4 269.1,97.2 305.3,92.4 341.5,89.9 377.6,82.1 413.8,70.6 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,110.5 124.4,104.1 160.5,105.4 196.7,96.2 232.9,95.6 269.1,87.7 305.3,83.3 341.5,72.4 377.6,66.7 413.8,54.6 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.8 88.2,107.9 124.4,102.9 160.5,106.7 196.7,91.9 232.9,94.4 269.1,81.6 305.3,72.9 341.5,65.6 377.6,55.3 413.8,46.7 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,106.7 124.4,96.2 160.5,97.0 196.7,88.3 232.9,88.8 269.1,71.1 305.3,65.0 341.5,57.5 377.6,48.2 413.8,37.0 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.77 µs | 3.36 µs | 3.08 µs | 3.82 µs |
| D38 | 4.57 ns | 3.35 µs | 3.97 µs | 4.84 µs | 5.36 µs |
| D57 | 2.81 ns | 5.96 µs | 7.63 µs | 8 µs | 11 µs |
| D76 | 4.22 ns | 6.43 µs | 6.31 µs | 6.14 µs | 12.1 µs |
| D115 | 10.3 ns | 13.1 µs | 12 µs | 21.8 µs | 26.3 µs |
| D153 | 16.2 ns | 8.36 µs | 16.5 µs | 13.6 µs | 20.8 µs |
| D230 | 40.3 ns | 13.1 µs | 24.4 µs | 37.3 µs | 85.8 µs |
| D307 | 75.8 ns | 17.2 µs | 53 µs | 76.8 µs | 125 µs |
| D462 | 105 ns | 24.9 µs | 88 µs | 133 µs | 232 µs |
| D616 | 119 ns | 36.9 µs | 116 µs | 271 µs | 449 µs |
| D924 | 206 ns | 88.7 µs | 293 µs | 507 µs | 1e+03 µs |
| D1232 | 376 ns | 135 µs | 377 µs | 922 µs | 2.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.5 88.2,191.1 124.4,197.2 160.5,192.1 196.7,181.1 232.9,175.4 269.1,164.1 305.3,156.3 341.5,152.2 377.6,150.7 413.8,143.9 450.0,136.4 450.0,29.9 413.8,38.6 377.6,48.5 341.5,56.7 305.3,64.3 269.1,69.0 232.9,86.6 196.7,83.7 160.5,93.3 124.4,94.5 88.2,103.4 52.0,107.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.5 88.2,191.1 124.4,197.2 160.5,192.1 196.7,181.1 232.9,175.4 269.1,164.1 305.3,156.3 341.5,152.2 377.6,150.7 413.8,143.9 450.0,136.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,109.3 124.4,102.1 160.5,101.2 196.7,92.4 232.9,97.9 269.1,92.4 305.3,89.0 341.5,84.4 377.6,79.5 413.8,68.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.3 88.2,107.2 124.4,99.1 160.5,101.4 196.7,93.4 232.9,89.5 269.1,84.6 305.3,75.0 341.5,68.7 377.6,65.3 413.8,53.8 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,104.7 124.4,98.5 160.5,101.8 196.7,86.0 232.9,91.9 269.1,79.4 305.3,70.4 341.5,63.6 377.6,54.8 413.8,47.0 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,103.4 124.4,94.5 160.5,93.3 196.7,83.7 232.9,86.6 269.1,69.0 305.3,64.3 341.5,56.7 377.6,48.5 413.8,38.6 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.41 ns | 1.15 µs | 2.4 µs | 2.1 µs | 2.85 µs |
| D38 | 4.92 ns | 2.39 µs | 2.89 µs | 3.57 µs | 4.11 µs |
| D57 | 2.81 ns | 3.57 µs | 4.79 µs | 5.66 µs | 9.63 µs |
| D76 | 3.52 ns | 3.87 µs | 4.42 µs | 4.11 µs | 8.71 µs |
| D115 | 11.6 ns | 4.89 µs | 9.87 µs | 13.5 µs | 18.3 µs |
| D153 | 16.9 ns | 5.58 µs | 10.5 µs | 11.1 µs | 17.1 µs |
| D230 | 40.7 ns | 8.85 µs | 19.4 µs | 30.9 µs | 70.1 µs |
| D307 | 66.7 ns | 12.9 µs | 26.4 µs | 61.5 µs | 114 µs |
| D462 | 130 ns | 15.9 µs | 62 µs | 112 µs | 216 µs |
| D616 | 115 ns | 29 µs | 104 µs | 257 µs | 456 µs |
| D924 | 192 ns | 72.7 µs | 272 µs | 507 µs | 1.13 ms |
| D1232 | 354 ns | 120 µs | 386 µs | 1.04 ms | 1.63 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.8 88.2,190.2 124.4,197.2 160.5,194.4 196.7,179.6 232.9,174.9 269.1,164.0 305.3,157.9 341.5,149.6 377.6,151.1 413.8,144.8 450.0,137.2 450.0,32.5 413.8,37.1 377.6,48.3 341.5,57.6 305.3,65.6 269.1,71.6 232.9,89.1 196.7,88.2 160.5,97.4 124.4,96.2 88.2,106.7 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.8 88.2,190.2 124.4,197.2 160.5,194.4 196.7,179.6 232.9,174.9 269.1,164.0 305.3,157.9 341.5,149.6 377.6,151.1 413.8,144.8 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.6 88.2,113.5 124.4,108.5 160.5,107.5 196.7,104.6 232.9,102.9 269.1,97.2 305.3,92.5 341.5,90.0 377.6,82.5 413.8,71.1 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.4 88.2,111.1 124.4,104.8 160.5,105.8 196.7,95.9 232.9,95.1 269.1,87.5 305.3,83.7 341.5,73.1 377.6,66.6 413.8,54.7 450.0,50.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.1 88.2,108.5 124.4,102.8 160.5,106.7 196.7,92.0 232.9,94.4 269.1,81.7 305.3,73.2 341.5,65.7 377.6,55.5 413.8,47.0 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,106.7 124.4,96.2 160.5,97.4 196.7,88.2 232.9,89.1 269.1,71.6 305.3,65.6 341.5,57.6 377.6,48.3 413.8,37.1 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.73 ns | 1.79 µs | 3.36 µs | 3.07 µs | 3.82 µs |
| D38 | 4.57 ns | 3.36 µs | 3.98 µs | 4.85 µs | 5.37 µs |
| D57 | 12.2 ns | 5.95 µs | 7.65 µs | 8.03 µs | 11 µs |
| D76 | 12.1 ns | 6.44 µs | 6.31 µs | 6.15 µs | 12.1 µs |
| D115 | 10.6 ns | 12.9 µs | 12 µs | 21.9 µs | 26.3 µs |
| D153 | 16.5 ns | 8.4 µs | 16.7 µs | 13.5 µs | 20.7 µs |
| D230 | 39.9 ns | 13.1 µs | 24.5 µs | 37.5 µs | 86.2 µs |
| D307 | 65.1 ns | 17.2 µs | 52.3 µs | 76.7 µs | 126 µs |
| D462 | 145 ns | 25 µs | 86.6 µs | 133 µs | 231 µs |
| D616 | 112 ns | 37.2 µs | 116 µs | 272 µs | 448 µs |
| D924 | 209 ns | 88.8 µs | 294 µs | 505 µs | 1 ms |
| D1232 | 396 ns | 136 µs | 377 µs | 922 µs | 2.01 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,197.6 88.2,191.1 124.4,179.0 160.5,179.1 196.7,180.7 232.9,175.2 269.1,164.3 305.3,158.2 341.5,148.3 377.6,151.5 413.8,143.7 450.0,135.8 450.0,29.9 413.8,38.6 377.6,48.5 341.5,56.7 305.3,64.3 269.1,69.0 232.9,86.7 196.7,83.7 160.5,93.3 124.4,94.5 88.2,103.4 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,197.6 88.2,191.1 124.4,179.0 160.5,179.1 196.7,180.7 232.9,175.2 269.1,164.3 305.3,158.2 341.5,148.3 377.6,151.5 413.8,143.7 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.1 88.2,109.2 124.4,102.1 160.5,101.2 196.7,92.5 232.9,97.9 269.1,92.4 305.3,89.0 341.5,84.4 377.6,79.4 413.8,68.6 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.2 88.2,107.2 124.4,99.0 160.5,101.4 196.7,93.5 232.9,89.3 269.1,84.6 305.3,75.2 341.5,68.9 377.6,65.3 413.8,53.8 450.0,50.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,104.7 124.4,98.4 160.5,101.7 196.7,86.0 232.9,92.0 269.1,79.3 305.3,70.4 341.5,63.6 377.6,54.7 413.8,47.0 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,103.4 124.4,94.5 160.5,93.3 196.7,83.7 232.9,86.7 269.1,69.0 305.3,64.3 341.5,56.7 377.6,48.5 413.8,38.6 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.36 ns | 2.18 µs | 4.02 µs | 3.78 µs | 4.66 µs |
| D38 | 4.92 ns | 4.02 µs | 4.82 µs | 5.93 µs | 6.67 µs |
| D57 | 2.81 ns | 4.72 µs | 6.38 µs | 7.39 µs | 9.88 µs |
| D76 | 3.62 ns | 5.06 µs | 5.84 µs | 5.27 µs | 10.9 µs |
| D115 | 10.9 ns | 6.33 µs | 12.3 µs | 16.8 µs | 21.7 µs |
| D153 | 16.4 ns | 7.46 µs | 12.8 µs | 13.4 µs | 20.2 µs |
| D230 | 41.2 ns | 11.1 µs | 23.2 µs | 34.7 µs | 79.7 µs |
| D307 | 62.8 ns | 15.2 µs | 31.8 µs | 69.7 µs | 128 µs |
| D462 | 102 ns | 19.7 µs | 71.4 µs | 123 µs | 233 µs |
| D616 | 117 ns | 34.1 µs | 113 µs | 284 µs | 493 µs |
| D924 | 159 ns | 82.8 µs | 297 µs | 541 µs | 1.21 ms |
| D1232 | 370 ns | 133 µs | 414 µs | 1.11 ms | 1.72 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.0 88.2,190.2 124.4,197.2 160.5,194.0 196.7,180.4 232.9,175.3 269.1,163.9 305.3,158.6 341.5,152.6 377.6,150.9 413.8,147.1 450.0,136.6 450.0,31.9 413.8,36.2 377.6,47.3 341.5,56.6 305.3,64.1 269.1,70.0 232.9,87.0 196.7,86.1 160.5,94.6 124.4,95.9 88.2,100.7 52.0,105.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.0 88.2,190.2 124.4,197.2 160.5,194.0 196.7,180.4 232.9,175.3 269.1,163.9 305.3,158.6 341.5,152.6 377.6,150.9 413.8,147.1 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.6 88.2,107.0 124.4,105.0 160.5,104.2 196.7,101.4 232.9,99.3 269.1,94.5 305.3,90.5 341.5,87.3 377.6,80.5 413.8,69.5 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,104.8 124.4,101.3 160.5,102.4 196.7,93.2 232.9,92.6 269.1,85.3 305.3,81.4 341.5,71.3 377.6,65.6 413.8,53.6 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,102.2 124.4,99.5 160.5,103.7 196.7,89.3 232.9,92.1 269.1,80.3 305.3,71.6 341.5,64.6 377.6,54.2 413.8,46.2 450.0,37.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,100.7 124.4,95.9 160.5,94.6 196.7,86.1 232.9,87.0 269.1,70.0 305.3,64.1 341.5,56.6 377.6,47.3 413.8,36.2 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.76 µs | 3.52 µs | 3.23 µs | 3.99 µs |
| D38 | 4.22 ns | 3.54 µs | 4.02 µs | 4.89 µs | 5.41 µs |
| D57 | 2.8 µs | 6.39 µs | 7.98 µs | 8.4 µs | 11.5 µs |
| D76 | 2.81 µs | 6.6 µs | 6.63 µs | 6.36 µs | 12.6 µs |
| D115 | 5.38 µs | 13.5 µs | 12.6 µs | 22.3 µs | 27.3 µs |
| D153 | 2.76 µs | 8.76 µs | 17.3 µs | 13.7 µs | 21.5 µs |
| D230 | 3.11 µs | 13.5 µs | 25.3 µs | 38.4 µs | 86.4 µs |
| D307 | 3.24 µs | 18.1 µs | 54.4 µs | 77.7 µs | 126 µs |
| D462 | 3.12 µs | 25.7 µs | 88.3 µs | 135 µs | 235 µs |
| D616 | 3.24 µs | 38.2 µs | 117 µs | 275 µs | 454 µs |
| D924 | 3.89 µs | 89.3 µs | 296 µs | 507 µs | 1.01 ms |
| D1232 | 4.19 µs | 137 µs | 382 µs | 931 µs | 2.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,192.1 124.4,111.5 160.5,111.5 196.7,103.4 232.9,111.7 269.1,110.2 305.3,109.7 341.5,110.2 377.6,109.7 413.8,107.4 450.0,106.5 450.0,29.9 413.8,38.5 377.6,48.4 341.5,56.5 305.3,64.3 269.1,69.0 232.9,86.2 196.7,83.2 160.5,92.8 124.4,94.0 88.2,103.3 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,192.1 124.4,111.5 160.5,111.5 196.7,103.4 232.9,111.7 269.1,110.2 305.3,109.7 341.5,110.2 377.6,109.7 413.8,107.4 450.0,106.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,108.6 124.4,101.3 160.5,100.9 196.7,92.0 232.9,97.4 269.1,92.0 305.3,88.4 341.5,84.0 377.6,79.1 413.8,68.5 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,107.0 124.4,98.5 160.5,100.8 196.7,92.9 232.9,88.9 269.1,84.2 305.3,74.7 341.5,68.7 377.6,65.2 413.8,53.7 450.0,50.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,104.6 124.4,97.9 160.5,101.3 196.7,85.7 232.9,91.8 269.1,79.0 305.3,70.3 341.5,63.4 377.6,54.6 413.8,47.0 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,103.3 124.4,94.0 160.5,92.8 196.7,83.2 232.9,86.2 269.1,69.0 305.3,64.3 341.5,56.5 377.6,48.4 413.8,38.5 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.54 ns | 134 ns | 157 ns | 135 ns | 163 ns |
| D38 | 4.92 ns | 162 ns | 166 ns | 199 ns | 204 ns |
| D57 | 202 ns | 335 ns | 352 ns | 333 ns | 446 ns |
| D76 | 202 ns | 328 ns | 277 ns | 244 ns | 440 ns |
| D115 | 402 ns | 650 ns | 689 ns | 798 ns | 862 ns |
| D153 | 443 ns | 709 ns | 823 ns | 502 ns | 565 ns |
| D230 | 585 ns | 772 ns | 1.11 µs | 1.15 µs | 1.94 µs |
| D307 | 932 ns | 1.39 µs | 1.73 µs | 2.04 µs | 2.84 µs |
| D462 | 909 ns | 1.6 µs | 2.37 µs | 2.72 µs | 3.93 µs |
| D616 | 1.12 µs | 1.89 µs | 2.51 µs | 4.22 µs | 6.09 µs |
| D924 | 1.69 µs | 3.05 µs | 4.97 µs | 6.46 µs | 11.4 µs |
| D1232 | 2.43 µs | 4.31 µs | 6.19 µs | 11.3 µs | 22 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,182.3 124.4,117.8 160.5,117.8 196.7,105.8 232.9,104.2 269.1,99.3 305.3,91.2 341.5,91.7 377.6,88.0 413.8,80.8 450.0,74.6 450.0,36.3 413.8,47.7 377.6,58.6 341.5,66.2 305.3,71.9 269.1,78.5 232.9,99.9 196.7,92.6 160.5,104.3 124.4,104.0 88.2,117.6 52.0,121.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,182.3 124.4,117.8 160.5,117.8 196.7,105.8 232.9,104.2 269.1,99.3 305.3,91.2 341.5,91.7 377.6,88.0 413.8,80.8 450.0,74.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.9 88.2,121.6 124.4,109.0 160.5,109.4 196.7,97.5 232.9,96.0 269.1,94.5 305.3,84.2 341.5,81.9 377.6,78.9 413.8,70.6 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.2 88.2,121.2 124.4,108.2 160.5,112.3 196.7,96.5 232.9,93.4 269.1,88.3 305.3,80.5 341.5,75.0 377.6,74.0 413.8,62.1 450.0,58.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.7 88.2,118.0 124.4,109.1 160.5,114.5 196.7,93.9 232.9,102.0 269.1,87.6 305.3,77.6 341.5,72.6 377.6,65.0 413.8,57.6 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.5 88.2,117.6 124.4,104.0 160.5,104.3 196.7,92.6 232.9,99.9 269.1,78.5 305.3,71.9 341.5,66.2 377.6,58.6 413.8,47.7 450.0,36.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.54 ns | 159 ns | 196 ns | 191 ns | 201 ns |
| D38 | 4.92 ns | 195 ns | 199 ns | 216 ns | 221 ns |
| D57 | 322 ns | 441 ns | 450 ns | 439 ns | 567 ns |
| D76 | 315 ns | 459 ns | 375 ns | 308 ns | 570 ns |
| D115 | 633 ns | 858 ns | 932 ns | 1.03 µs | 1.08 µs |
| D153 | 646 ns | 914 ns | 1.02 µs | 591 ns | 704 ns |
| D230 | 943 ns | 1.03 µs | 1.41 µs | 1.4 µs | 2.29 µs |
| D307 | 1.45 µs | 1.83 µs | 2.26 µs | 2.47 µs | 3.41 µs |
| D462 | 1.44 µs | 2.03 µs | 2.91 µs | 3.05 µs | 4.53 µs |
| D616 | 1.74 µs | 2.37 µs | 2.99 µs | 4.9 µs | 6.89 µs |
| D924 | 2.6 µs | 3.94 µs | 5.88 µs | 7.22 µs | 12.4 µs |
| D1232 | 3.38 µs | 5.15 µs | 7.06 µs | 12.5 µs | 26 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,188.0 88.2,182.3 124.4,109.7 160.5,110.1 196.7,97.9 232.9,97.6 269.1,91.0 305.3,83.5 341.5,83.6 377.6,80.4 413.8,73.4 450.0,68.8 450.0,33.4 413.8,46.2 377.6,56.5 341.5,63.7 305.3,68.7 269.1,75.6 232.9,96.1 196.7,88.7 160.5,99.8 124.4,99.9 88.2,116.2 52.0,117.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,188.0 88.2,182.3 124.4,109.7 160.5,110.1 196.7,97.9 232.9,97.6 269.1,91.0 305.3,83.5 341.5,83.6 377.6,80.4 413.8,73.4 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.0 88.2,118.4 124.4,104.2 160.5,103.5 196.7,92.7 232.9,91.6 269.1,89.5 305.3,79.5 341.5,77.7 377.6,75.0 413.8,66.2 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.3 88.2,118.1 124.4,103.9 160.5,107.0 196.7,91.2 232.9,89.6 269.1,84.1 305.3,75.8 341.5,71.5 377.6,71.0 413.8,59.2 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.7 88.2,116.6 124.4,104.3 160.5,110.5 196.7,89.4 232.9,99.1 269.1,84.2 305.3,74.3 341.5,70.6 377.6,62.4 413.8,55.7 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.9 88.2,116.2 124.4,99.9 160.5,99.8 196.7,88.7 232.9,96.1 269.1,75.6 305.3,68.7 341.5,63.7 377.6,56.5 413.8,46.2 450.0,33.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
