# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.2 ns | 9.92 µs | 27.7 µs | 32.4 µs | 36.8 µs |
| D38 | 4.29 µs | 7.52 µs | 7.19 µs | 11.8 µs | 16 µs |
| D57 | 4.71 µs | 9.2 µs | 12 µs | 14.6 µs | 17.2 µs |
| D76 | 4.66 µs | 8.49 µs | 16.2 µs | 17.3 µs | 19.7 µs |
| D115 | 5.17 µs | 12.2 µs | 25.2 µs | 29.4 µs | 40.9 µs |
| D153 | 4.29 µs | 14.2 µs | 28.6 µs | 43.3 µs | 58.4 µs |
| D230 | 4.74 µs | 23.6 µs | 40.6 µs | 60.1 µs | 119 µs |
| D307 | 4.84 µs | 26.7 µs | 64 µs | 120 µs | 166 µs |
| D462 | 5.22 µs | 41.8 µs | 132 µs | 235 µs | 369 µs |
| D616 | 4.89 µs | 59.1 µs | 201 µs | 414 µs | 697 µs |
| D924 | 5.38 µs | 121 µs | 441 µs | 930 µs | 1.65 ms |
| D1232 | 5.32 µs | 216 µs | 653 µs | 1.5 ms | 3.25 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.9 88.2,106.2 124.4,105.0 160.5,105.2 196.7,103.9 232.9,106.2 269.1,105.0 305.3,104.7 341.5,103.8 377.6,104.6 413.8,103.4 450.0,103.5 450.0,24.0 413.8,32.3 377.6,43.0 341.5,51.0 305.3,60.8 269.1,65.0 232.9,73.8 196.7,78.2 160.5,87.3 124.4,89.0 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.9 88.2,106.2 124.4,105.0 160.5,105.2 196.7,103.9 232.9,106.2 269.1,105.0 305.3,104.7 341.5,103.8 377.6,104.6 413.8,103.4 450.0,103.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,95.8 88.2,99.2 124.4,96.7 160.5,97.8 196.7,93.3 232.9,91.4 269.1,85.1 305.3,83.5 341.5,78.0 377.6,73.7 413.8,64.8 450.0,57.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.8 124.4,93.4 160.5,89.7 196.7,84.2 232.9,82.7 269.1,78.3 305.3,72.7 341.5,63.7 377.6,58.5 413.8,48.7 450.0,43.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.7 124.4,91.0 160.5,88.9 196.7,82.3 232.9,77.5 269.1,73.5 305.3,64.9 341.5,56.5 377.6,49.5 413.8,39.5 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,89.0 160.5,87.3 196.7,78.2 232.9,73.8 269.1,65.0 305.3,60.8 341.5,51.0 377.6,43.0 413.8,32.3 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 25.8 µs | 13.7 µs | 33.2 µs | 39 µs | 35.9 µs |
| D38 | 25.7 µs | 35.5 µs | 41.2 µs | 53.4 µs | 66.1 µs |
| D57 | 3.24 µs | 4.73 µs | 5.41 µs | 6.01 µs | 7.42 µs |
| D76 | 3.33 µs | 4.6 µs | 6.31 µs | 7.72 µs | 9.2 µs |
| D115 | 6.58 µs | 9.57 µs | 13.4 µs | 15.2 µs | 21.6 µs |
| D153 | 5.48 µs | 9.69 µs | 16.1 µs | 23.3 µs | 29.6 µs |
| D230 | 8.1 µs | 15.7 µs | 26.5 µs | 36.7 µs | 67.9 µs |
| D307 | 12.3 µs | 26.4 µs | 50.5 µs | 83.3 µs | 122 µs |
| D462 | 13.1 µs | 36.3 µs | 89.5 µs | 161 µs | 262 µs |
| D616 | 20.7 µs | 69.2 µs | 166 µs | 306 µs | 547 µs |
| D924 | 32.8 µs | 151 µs | 390 µs | 792 µs | 1.43 ms |
| D1232 | 41.9 µs | 278 µs | 677 µs | 1.44 ms | 2.91 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,139.4 88.2,139.5 124.4,184.5 160.5,183.9 196.7,169.1 232.9,173.0 269.1,164.6 305.3,155.4 341.5,154.1 377.6,144.2 413.8,134.2 450.0,128.9 450.0,36.8 413.8,52.2 377.6,73.1 341.5,89.1 305.3,105.8 269.1,118.4 232.9,136.5 196.7,143.3 160.5,161.8 124.4,166.5 88.2,119.0 52.0,132.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,139.4 88.2,139.5 124.4,184.5 160.5,183.9 196.7,169.1 232.9,173.0 269.1,164.6 305.3,155.4 341.5,154.1 377.6,144.2 413.8,134.2 450.0,128.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,153.1 88.2,132.5 124.4,176.3 160.5,176.9 196.7,161.0 232.9,160.7 269.1,150.2 305.3,138.9 341.5,132.0 377.6,118.0 413.8,101.0 450.0,87.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.9 88.2,129.2 124.4,173.3 160.5,170.0 196.7,153.6 232.9,149.7 269.1,138.9 305.3,124.9 341.5,112.4 377.6,99.0 413.8,80.4 450.0,68.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,123.6 124.4,171.0 160.5,165.6 196.7,151.0 232.9,141.6 269.1,131.8 305.3,114.0 341.5,99.7 377.6,85.7 413.8,65.1 450.0,52.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.3 88.2,119.0 124.4,166.5 160.5,161.8 196.7,143.3 232.9,136.5 269.1,118.4 305.3,105.8 341.5,89.1 377.6,73.1 413.8,52.2 450.0,36.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 9.91 µs | 27.7 µs | 32.4 µs | 36.8 µs |
| D38 | 4.23 µs | 7.5 µs | 7.13 µs | 11.7 µs | 16 µs |
| D57 | 4.62 µs | 9.18 µs | 12 µs | 14.5 µs | 17.3 µs |
| D76 | 4.59 µs | 8.54 µs | 16.2 µs | 17.3 µs | 19.7 µs |
| D115 | 5.09 µs | 12.1 µs | 25.3 µs | 29.4 µs | 41.1 µs |
| D153 | 4.21 µs | 14.2 µs | 29 µs | 44.1 µs | 59 µs |
| D230 | 4.77 µs | 24.2 µs | 40.7 µs | 60.3 µs | 119 µs |
| D307 | 4.76 µs | 27 µs | 65 µs | 120 µs | 166 µs |
| D462 | 5.19 µs | 42.2 µs | 130 µs | 237 µs | 368 µs |
| D616 | 4.82 µs | 59.8 µs | 200 µs | 414 µs | 699 µs |
| D924 | 5.34 µs | 122 µs | 441 µs | 929 µs | 1.65 ms |
| D1232 | 5.26 µs | 215 µs | 654 µs | 1.5 ms | 3.24 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,106.4 124.4,105.3 160.5,105.4 196.7,104.1 232.9,106.5 269.1,104.9 305.3,104.9 341.5,103.8 377.6,104.8 413.8,103.5 450.0,103.7 450.0,24.0 413.8,32.3 377.6,43.0 341.5,51.0 305.3,60.9 269.1,65.0 232.9,73.7 196.7,78.2 160.5,87.3 124.4,88.9 88.2,89.9 52.0,79.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,106.4 124.4,105.3 160.5,105.4 196.7,104.1 232.9,106.5 269.1,104.9 305.3,104.9 341.5,103.8 377.6,104.8 413.8,103.5 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,95.8 88.2,99.3 124.4,96.8 160.5,97.7 196.7,93.4 232.9,91.4 269.1,84.8 305.3,83.4 341.5,77.8 377.6,73.5 413.8,64.7 450.0,57.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.1 88.2,99.9 124.4,93.5 160.5,89.7 196.7,84.2 232.9,82.5 269.1,78.3 305.3,72.5 341.5,63.8 377.6,58.5 413.8,48.7 450.0,43.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.1 88.2,93.7 124.4,91.1 160.5,88.9 196.7,82.3 232.9,77.3 269.1,73.4 305.3,64.9 341.5,56.4 377.6,49.5 413.8,39.5 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.5 88.2,89.9 124.4,88.9 160.5,87.3 196.7,78.2 232.9,73.7 269.1,65.0 305.3,60.9 341.5,51.0 377.6,43.0 413.8,32.3 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 9.68 µs | 27.1 µs | 31.8 µs | 36.1 µs |
| D38 | 3.74 ns | 29 µs | 33.7 µs | 39.9 µs | 55.2 µs |
| D57 | 1.96 ns | 5.86 µs | 7.88 µs | 8.9 µs | 11.6 µs |
| D76 | 2.03 ns | 6.25 µs | 9.52 µs | 11.8 µs | 13.9 µs |
| D115 | 12.7 ns | 14 µs | 20.6 µs | 22.6 µs | 32.7 µs |
| D153 | 14.8 ns | 14.8 µs | 24 µs | 36.5 µs | 43.2 µs |
| D230 | 27.8 ns | 23.6 µs | 40.8 µs | 55.3 µs | 93.7 µs |
| D307 | 45 ns | 37.4 µs | 71.4 µs | 114 µs | 149 µs |
| D462 | 75.2 ns | 54.4 µs | 126 µs | 194 µs | 313 µs |
| D616 | 79.3 ns | 104 µs | 224 µs | 400 µs | 660 µs |
| D924 | 115 ns | 227 µs | 530 µs | 931 µs | 1.59 ms |
| D1232 | 144 ns | 390 µs | 865 µs | 1.63 ms | 2.88 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,201.2 196.7,178.4 232.9,176.5 269.1,168.8 305.3,162.8 341.5,156.4 377.6,155.7 413.8,151.1 450.0,148.4 450.0,25.4 413.8,32.8 377.6,43.7 341.5,53.0 305.3,62.2 269.1,67.9 232.9,77.6 196.7,81.0 160.5,91.6 124.4,93.9 88.2,74.5 52.0,79.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,201.2 196.7,178.4 232.9,176.5 269.1,168.8 305.3,162.8 341.5,156.4 377.6,155.7 413.8,151.1 450.0,148.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,96.1 88.2,82.5 124.4,102.3 160.5,101.5 196.7,91.5 232.9,90.9 269.1,85.0 305.3,79.4 341.5,74.7 377.6,66.6 413.8,56.9 450.0,50.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.4 88.2,80.6 124.4,98.7 160.5,96.3 196.7,86.7 232.9,84.9 269.1,78.3 305.3,71.3 341.5,64.3 377.6,57.1 413.8,46.4 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.4 88.2,78.5 124.4,97.2 160.5,93.7 196.7,85.6 232.9,79.7 269.1,74.5 305.3,65.6 341.5,58.9 377.6,50.0 413.8,39.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.8 88.2,74.5 124.4,93.9 160.5,91.6 196.7,81.0 232.9,77.6 269.1,67.9 305.3,62.2 341.5,53.0 377.6,43.7 413.8,32.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.47 µs | 2.56 µs | 2.93 µs | 3.05 µs |
| D38 | 5.4 µs | 8.92 µs | 5.91 µs | 10.3 µs | 14 µs |
| D57 | 3.83 µs | 7.83 µs | 10.4 µs | 13 µs | 5.01 µs |
| D76 | 3.82 µs | 7.37 µs | 14.2 µs | 15.3 µs | 17.6 µs |
| D115 | 4.23 µs | 10.5 µs | 22.6 µs | 26.3 µs | 37.2 µs |
| D153 | 3.5 µs | 12.3 µs | 22.4 µs | 39 µs | 54 µs |
| D230 | 3.91 µs | 20.9 µs | 37.5 µs | 55 µs | 111 µs |
| D307 | 3.94 µs | 24.2 µs | 53.2 µs | 112 µs | 154 µs |
| D462 | 3.15 µs | 34.1 µs | 111 µs | 207 µs | 316 µs |
| D616 | 4 µs | 54.2 µs | 187 µs | 387 µs | 661 µs |
| D924 | 4.44 µs | 112 µs | 412 µs | 886 µs | 1.57 ms |
| D1232 | 4.4 µs | 202 µs | 615 µs | 1.44 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,103.4 124.4,107.6 160.5,107.7 196.7,106.4 232.9,108.7 269.1,107.4 305.3,107.3 341.5,110.1 377.6,107.1 413.8,105.8 450.0,105.9 450.0,24.5 413.8,33.0 377.6,43.7 341.5,52.9 305.3,61.8 269.1,65.9 232.9,74.8 196.7,79.4 160.5,88.7 124.4,104.3 88.2,91.5 52.0,110.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,103.4 124.4,107.6 160.5,107.7 196.7,106.4 232.9,108.7 269.1,107.4 305.3,107.3 341.5,110.1 377.6,107.1 413.8,105.8 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.5 88.2,97.1 124.4,98.8 160.5,99.5 196.7,95.2 232.9,93.1 269.1,86.6 305.3,84.8 341.5,80.5 377.6,74.8 413.8,65.7 450.0,58.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,102.2 124.4,95.2 160.5,91.3 196.7,85.6 232.9,85.7 269.1,79.3 305.3,75.0 341.5,65.8 377.6,59.4 413.8,49.6 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,95.3 124.4,92.5 160.5,90.4 196.7,83.7 232.9,78.8 269.1,74.6 305.3,65.7 341.5,58.1 377.6,50.4 413.8,40.1 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,91.5 124.4,104.3 160.5,88.7 196.7,79.4 232.9,74.8 269.1,65.9 305.3,61.8 341.5,52.9 377.6,43.7 413.8,33.0 450.0,24.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 3.91 µs | 8.83 µs | 10.3 µs | 10.6 µs |
| D38 | 3.74 ns | 9.4 µs | 11 µs | 13 µs | 16.1 µs |
| D57 | 497 ns | 5.95 µs | 7.33 µs | 8.4 µs | 10.9 µs |
| D76 | 503 ns | 5.97 µs | 8.59 µs | 11.2 µs | 13.8 µs |
| D115 | 1.28 µs | 13 µs | 19.3 µs | 22.5 µs | 33.8 µs |
| D153 | 890 ns | 13.4 µs | 23.9 µs | 36.3 µs | 48.6 µs |
| D230 | 1.33 µs | 22.8 µs | 40.5 µs | 62.1 µs | 115 µs |
| D307 | 2.19 µs | 40.3 µs | 83.2 µs | 142 µs | 215 µs |
| D462 | 2.4 µs | 56.2 µs | 152 µs | 287 µs | 471 µs |
| D616 | 3.83 µs | 111 µs | 290 µs | 543 µs | 1 ms |
| D924 | 5.83 µs | 255 µs | 701 µs | 1.45 ms | 2.66 ms |
| D1232 | 7.52 µs | 478 µs | 1.22 ms | 2.68 ms | 5.49 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,132.9 160.5,132.8 196.7,121.2 232.9,125.7 269.1,120.8 305.3,114.6 341.5,113.4 377.6,107.6 413.8,102.4 450.0,99.2 450.0,17.4 413.8,26.4 377.6,38.5 341.5,47.9 305.3,57.6 269.1,65.4 232.9,76.1 196.7,80.6 160.5,91.7 124.4,94.7 88.2,89.8 52.0,95.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,132.9 160.5,132.8 196.7,121.2 232.9,125.7 269.1,120.8 305.3,114.6 341.5,113.4 377.6,107.6 413.8,102.4 450.0,99.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.4 88.2,96.5 124.4,102.2 160.5,102.1 196.7,92.5 232.9,92.1 269.1,85.5 305.3,78.4 341.5,74.3 377.6,65.9 413.8,55.6 450.0,47.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.3 88.2,94.5 124.4,99.6 160.5,97.6 196.7,87.6 232.9,84.9 269.1,78.4 305.3,69.4 341.5,61.9 377.6,53.9 413.8,43.0 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.3 88.2,92.4 124.4,97.9 160.5,94.3 196.7,85.6 232.9,79.7 269.1,73.1 305.3,62.8 341.5,54.1 377.6,46.1 413.8,34.0 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.0 88.2,89.8 124.4,94.7 160.5,91.7 196.7,80.6 232.9,76.1 269.1,65.4 305.3,57.6 341.5,47.9 377.6,38.5 413.8,26.4 450.0,17.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.3 µs | 5.55 µs | 6.42 µs | 6.49 µs |
| D38 | 4.98 ns | 5.97 µs | 6.75 µs | 7.92 µs | 9.71 µs |
| D57 | 2.18 ns | 3.67 µs | 4.66 µs | 5.1 µs | 8.68 µs |
| D76 | 3.43 ns | 3.83 µs | 5.59 µs | 7.02 µs | 8.81 µs |
| D115 | 16.8 ns | 4.73 µs | 9.99 µs | 12.5 µs | 18.2 µs |
| D153 | 19.7 ns | 5.14 µs | 10.2 µs | 19.2 µs | 28.8 µs |
| D230 | 48.6 ns | 9.91 µs | 17.7 µs | 31.6 µs | 66.7 µs |
| D307 | 81 ns | 11.8 µs | 26.7 µs | 67.7 µs | 101 µs |
| D462 | 177 ns | 15.5 µs | 66 µs | 131 µs | 215 µs |
| D616 | 152 ns | 29.3 µs | 120 µs | 250 µs | 451 µs |
| D924 | 208 ns | 68.5 µs | 267 µs | 608 µs | 1.12 ms |
| D1232 | 406 ns | 131 µs | 423 µs | 1.04 ms | 2.29 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.1 124.4,200.3 160.5,194.7 196.7,175.0 232.9,173.0 269.1,161.8 305.3,155.5 341.5,145.7 377.6,147.7 413.8,143.8 450.0,135.5 450.0,28.3 413.8,37.2 377.6,48.4 341.5,57.6 305.3,67.0 269.1,72.2 232.9,82.6 196.7,88.3 160.5,97.3 124.4,97.5 88.2,96.1 52.0,101.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.1 124.4,200.3 160.5,194.7 196.7,175.0 232.9,173.0 269.1,161.8 305.3,155.5 341.5,145.7 377.6,147.7 413.8,143.8 450.0,135.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.0 88.2,102.1 124.4,108.1 160.5,107.6 196.7,105.0 232.9,104.0 269.1,95.8 305.3,93.7 341.5,90.3 377.6,82.4 413.8,71.8 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.0 88.2,100.6 124.4,105.2 160.5,102.9 196.7,95.7 232.9,95.5 269.1,88.6 305.3,83.5 341.5,72.3 377.6,64.9 413.8,54.9 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,98.6 124.4,104.1 160.5,100.1 196.7,92.9 232.9,87.6 269.1,81.4 305.3,72.0 341.5,63.8 377.6,55.8 413.8,44.7 450.0,38.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.1 88.2,96.1 124.4,97.5 160.5,97.3 196.7,88.3 232.9,82.6 269.1,72.2 305.3,67.0 341.5,57.6 377.6,48.4 413.8,37.2 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 3.97 µs | 9.48 µs | 11 µs | 11.1 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 13.4 µs | 16.5 µs |
| D57 | 2.49 ns | 5.73 µs | 7.35 µs | 7.85 µs | 10 µs |
| D76 | 3.12 ns | 5.56 µs | 8.01 µs | 10.2 µs | 12 µs |
| D115 | 10.9 ns | 12.5 µs | 12.7 µs | 19.5 µs | 25.5 µs |
| D153 | 20.1 ns | 7.55 µs | 15.6 µs | 24.6 µs | 34.3 µs |
| D230 | 48.5 ns | 13.4 µs | 22.3 µs | 37.1 µs | 77.9 µs |
| D307 | 81.8 ns | 16 µs | 51.5 µs | 78.7 µs | 110 µs |
| D462 | 184 ns | 23.1 µs | 85.4 µs | 154 µs | 227 µs |
| D616 | 147 ns | 35.3 µs | 131 µs | 264 µs | 445 µs |
| D924 | 206 ns | 80.1 µs | 287 µs | 605 µs | 993 µs |
| D1232 | 404 ns | 141 µs | 410 µs | 917 µs | 2.72 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,192.7 124.4,198.7 160.5,195.9 196.7,180.4 232.9,172.7 269.1,161.8 305.3,155.4 341.5,145.3 377.6,148.1 413.8,143.9 450.0,135.5 450.0,26.2 413.8,38.7 377.6,48.6 341.5,56.9 305.3,66.0 269.1,70.2 232.9,80.4 196.7,84.1 160.5,93.5 124.4,95.7 88.2,89.5 52.0,94.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,192.7 124.4,198.7 160.5,195.9 196.7,180.4 232.9,172.7 269.1,161.8 305.3,155.4 341.5,145.3 377.6,148.1 413.8,143.9 450.0,135.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,95.5 124.4,102.6 160.5,103.0 196.7,92.9 232.9,99.2 269.1,92.1 305.3,89.9 341.5,85.3 377.6,80.1 413.8,69.9 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,94.0 124.4,99.5 160.5,98.5 196.7,92.7 232.9,90.2 269.1,85.8 305.3,75.4 341.5,69.1 377.6,63.8 413.8,54.1 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,98.7 160.5,95.5 196.7,87.4 232.9,84.6 269.1,79.5 305.3,70.1 341.5,61.8 377.6,55.1 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.4 88.2,89.5 124.4,95.7 160.5,93.5 196.7,84.1 232.9,80.4 269.1,70.2 305.3,66.0 341.5,56.9 377.6,48.6 413.8,38.7 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.19 µs | 5.36 µs | 6.34 µs | 6.41 µs |
| D38 | 4.36 ns | 5.77 µs | 6.57 µs | 7.75 µs | 9.65 µs |
| D57 | 2.18 ns | 3.41 µs | 4.48 µs | 5.05 µs | 8.66 µs |
| D76 | 3.43 ns | 3.37 µs | 5.35 µs | 6.93 µs | 8.43 µs |
| D115 | 16.8 ns | 4.52 µs | 9.96 µs | 12 µs | 17.5 µs |
| D153 | 21.1 ns | 4.8 µs | 9.93 µs | 18.5 µs | 28.1 µs |
| D230 | 48.8 ns | 9.84 µs | 18.1 µs | 31.4 µs | 65.1 µs |
| D307 | 76.4 ns | 11.5 µs | 26.1 µs | 66.2 µs | 101 µs |
| D462 | 182 ns | 14.9 µs | 65.7 µs | 131 µs | 214 µs |
| D616 | 145 ns | 28.5 µs | 119 µs | 247 µs | 448 µs |
| D924 | 197 ns | 67 µs | 267 µs | 608 µs | 1.12 ms |
| D1232 | 410 ns | 128 µs | 421 µs | 1.03 ms | 2.27 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,200.3 160.5,194.7 196.7,175.0 232.9,172.2 269.1,161.8 305.3,156.2 341.5,145.4 377.6,148.3 413.8,144.4 450.0,135.4 450.0,28.4 413.8,37.2 377.6,48.5 341.5,57.7 305.3,67.0 269.1,72.5 232.9,82.9 196.7,88.7 160.5,97.8 124.4,97.5 88.2,96.2 52.0,101.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,200.3 160.5,194.7 196.7,175.0 232.9,172.2 269.1,161.8 305.3,156.2 341.5,145.4 377.6,148.3 413.8,144.4 450.0,135.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.6 88.2,102.5 124.4,109.1 160.5,109.2 196.7,105.6 232.9,104.8 269.1,95.9 305.3,94.0 341.5,90.7 377.6,82.7 413.8,72.1 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,103.5 88.2,100.9 124.4,105.7 160.5,103.5 196.7,95.8 232.9,95.8 269.1,88.3 305.3,83.8 341.5,72.4 377.6,65.0 413.8,54.9 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.4 88.2,98.9 124.4,104.2 160.5,100.3 196.7,93.5 232.9,88.1 269.1,81.5 305.3,72.3 341.5,63.8 377.6,55.9 413.8,44.8 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.2 88.2,96.2 124.4,97.5 160.5,97.8 196.7,88.7 232.9,82.9 269.1,72.5 305.3,67.0 341.5,57.7 377.6,48.5 413.8,37.2 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 3.98 µs | 9.46 µs | 11 µs | 11.1 µs |
| D38 | 4.05 ns | 10.2 µs | 11.5 µs | 13.4 µs | 16.5 µs |
| D57 | 10.6 ns | 5.75 µs | 7.42 µs | 7.88 µs | 10 µs |
| D76 | 10.5 ns | 5.57 µs | 8.05 µs | 10.2 µs | 12 µs |
| D115 | 11.3 ns | 12.6 µs | 12.4 µs | 19.5 µs | 25.2 µs |
| D153 | 20.2 ns | 7.57 µs | 15.7 µs | 24 µs | 34.3 µs |
| D230 | 49.6 ns | 13.5 µs | 22.4 µs | 37 µs | 77.7 µs |
| D307 | 76.3 ns | 16.1 µs | 51.4 µs | 78.6 µs | 110 µs |
| D462 | 178 ns | 23.9 µs | 85.7 µs | 154 µs | 227 µs |
| D616 | 145 ns | 35.3 µs | 131 µs | 264 µs | 443 µs |
| D924 | 214 ns | 79.8 µs | 287 µs | 606 µs | 993 µs |
| D1232 | 394 ns | 142 µs | 410 µs | 915 µs | 2.71 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.7 124.4,180.7 160.5,180.8 196.7,180.0 232.9,172.7 269.1,161.6 305.3,156.2 341.5,145.7 377.6,148.2 413.8,143.4 450.0,135.8 450.0,26.2 413.8,38.7 377.6,48.7 341.5,57.0 305.3,66.0 269.1,70.3 232.9,80.4 196.7,84.2 160.5,93.5 124.4,95.7 88.2,89.5 52.0,94.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.7 124.4,180.7 160.5,180.8 196.7,180.0 232.9,172.7 269.1,161.6 305.3,156.2 341.5,145.7 377.6,148.2 413.8,143.4 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.2 88.2,95.5 124.4,102.6 160.5,103.0 196.7,92.9 232.9,99.2 269.1,92.0 305.3,89.8 341.5,84.9 377.6,80.1 413.8,69.9 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.4 88.2,94.0 124.4,99.4 160.5,98.4 196.7,93.1 232.9,90.1 269.1,85.7 305.3,75.4 341.5,69.1 377.6,63.8 413.8,54.0 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.5 88.2,92.1 124.4,98.7 160.5,95.4 196.7,87.4 232.9,84.9 269.1,79.5 305.3,70.1 341.5,61.7 377.6,55.1 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.4 88.2,89.5 124.4,95.7 160.5,93.5 196.7,84.2 232.9,80.4 269.1,70.3 305.3,66.0 341.5,57.0 377.6,48.7 413.8,38.7 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 4.33 µs | 9.97 µs | 11.7 µs | 11.8 µs |
| D38 | 4.36 ns | 10.8 µs | 12.2 µs | 14.4 µs | 17.7 µs |
| D57 | 2.81 ns | 4.52 µs | 5.99 µs | 6.74 µs | 8.53 µs |
| D76 | 3.55 ns | 4.41 µs | 7.2 µs | 8.89 µs | 10.6 µs |
| D115 | 16.4 ns | 5.96 µs | 12.6 µs | 14.6 µs | 21.1 µs |
| D153 | 20.4 ns | 6.5 µs | 12.6 µs | 22.3 µs | 33 µs |
| D230 | 47.5 ns | 12.1 µs | 21.8 µs | 35.8 µs | 73.9 µs |
| D307 | 74.5 ns | 14.3 µs | 30.8 µs | 74.7 µs | 112 µs |
| D462 | 164 ns | 18.3 µs | 73.1 µs | 146 µs | 231 µs |
| D616 | 175 ns | 33.3 µs | 131 µs | 272 µs | 485 µs |
| D924 | 203 ns | 75.2 µs | 290 µs | 652 µs | 1.19 ms |
| D1232 | 369 ns | 140 µs | 456 µs | 1.1 ms | 2.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.3 196.7,175.3 232.9,172.6 269.1,162.1 305.3,156.5 341.5,146.7 377.6,145.9 413.8,144.1 450.0,136.6 450.0,27.7 413.8,36.4 377.6,47.6 341.5,56.8 305.3,65.8 269.1,70.9 232.9,80.9 196.7,86.5 160.5,95.0 124.4,97.7 88.2,88.6 52.0,93.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,197.2 160.5,194.3 196.7,175.3 232.9,172.6 269.1,162.1 305.3,156.5 341.5,146.7 377.6,145.9 413.8,144.1 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,106.1 88.2,94.8 124.4,105.6 160.5,105.9 196.7,102.1 232.9,101.1 269.1,93.3 305.3,91.3 341.5,88.2 377.6,80.8 413.8,70.7 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.8 88.2,93.2 124.4,102.1 160.5,99.8 196.7,92.8 232.9,92.8 269.1,86.0 305.3,81.8 341.5,71.0 377.6,63.8 413.8,53.9 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.8 88.2,91.2 124.4,100.6 160.5,97.2 196.7,91.0 232.9,85.8 269.1,79.9 305.3,70.8 341.5,62.4 377.6,54.7 413.8,43.9 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,93.7 88.2,88.6 124.4,97.7 160.5,95.0 196.7,86.5 232.9,80.9 269.1,70.9 305.3,65.8 341.5,56.8 377.6,47.6 413.8,36.4 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 4 µs | 9.62 µs | 11.2 µs | 11.3 µs |
| D38 | 3.74 ns | 10.4 µs | 11.5 µs | 13.4 µs | 16.6 µs |
| D57 | 2.58 µs | 5.87 µs | 7.59 µs | 8.18 µs | 10.3 µs |
| D76 | 2.62 µs | 5.68 µs | 8.25 µs | 10.6 µs | 12.3 µs |
| D115 | 5.93 µs | 12.9 µs | 12.5 µs | 20 µs | 25.9 µs |
| D153 | 2.56 µs | 7.88 µs | 16.3 µs | 24.9 µs | 35.1 µs |
| D230 | 2.83 µs | 14.8 µs | 22.9 µs | 37.8 µs | 79.3 µs |
| D307 | 3.14 µs | 16.5 µs | 52.2 µs | 79.6 µs | 111 µs |
| D462 | 3.73 µs | 24 µs | 86.8 µs | 155 µs | 230 µs |
| D616 | 3.42 µs | 36.1 µs | 133 µs | 267 µs | 449 µs |
| D924 | 4.06 µs | 81.9 µs | 291 µs | 608 µs | 1 ms |
| D1232 | 4.47 µs | 144 µs | 414 µs | 924 µs | 2.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,112.5 160.5,112.3 196.7,102.2 232.9,112.6 269.1,111.4 305.3,110.1 341.5,108.0 377.6,109.0 413.8,106.9 450.0,105.7 450.0,26.1 413.8,38.5 377.6,48.5 341.5,56.8 305.3,65.8 269.1,70.0 232.9,80.1 196.7,83.9 160.5,93.2 124.4,95.3 88.2,89.5 52.0,94.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,112.5 160.5,112.3 196.7,102.2 232.9,112.6 269.1,111.4 305.3,110.1 341.5,108.0 377.6,109.0 413.8,106.9 450.0,105.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.1 88.2,95.3 124.4,102.3 160.5,102.7 196.7,92.5 232.9,98.7 269.1,90.9 305.3,89.5 341.5,84.8 377.6,79.8 413.8,69.6 450.0,62.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.2 88.2,93.9 124.4,99.1 160.5,98.1 196.7,93.0 232.9,89.6 269.1,85.4 305.3,75.2 341.5,68.9 377.6,63.6 413.8,53.9 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.3 88.2,92.1 124.4,98.2 160.5,95.0 196.7,87.1 232.9,84.4 269.1,79.2 305.3,70.0 341.5,61.7 377.6,55.0 413.8,44.7 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,94.2 88.2,89.5 124.4,95.3 160.5,93.2 196.7,83.9 232.9,80.1 269.1,70.0 305.3,65.8 341.5,56.8 377.6,48.5 413.8,38.5 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.28 µs | 3.23 µs | 3.8 µs | 3.85 µs |
| D38 | 4.05 ns | 3.47 µs | 4 µs | 4.72 µs | 5.85 µs |
| D57 | 177 ns | 318 ns | 328 ns | 304 ns | 401 ns |
| D76 | 177 ns | 279 ns | 335 ns | 391 ns | 411 ns |
| D115 | 379 ns | 522 ns | 673 ns | 629 ns | 764 ns |
| D153 | 296 ns | 498 ns | 698 ns | 853 ns | 910 ns |
| D230 | 496 ns | 782 ns | 930 ns | 1.12 µs | 1.71 µs |
| D307 | 812 ns | 1.15 µs | 1.61 µs | 2.01 µs | 2.46 µs |
| D462 | 995 ns | 1.33 µs | 2.31 µs | 2.95 µs | 3.79 µs |
| D616 | 1.07 µs | 1.76 µs | 2.79 µs | 4.01 µs | 5.96 µs |
| D924 | 1.6 µs | 2.65 µs | 4.72 µs | 7.45 µs | 11.2 µs |
| D1232 | 2.18 µs | 4.07 µs | 6.73 µs | 10.8 µs | 29.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,187.1 88.2,185.7 124.4,120.1 160.5,120.0 196.7,106.9 232.9,111.1 269.1,102.2 305.3,93.6 341.5,90.1 377.6,88.8 413.8,81.9 450.0,76.5 450.0,31.0 413.8,48.1 377.6,59.0 341.5,66.8 305.3,74.4 269.1,80.7 232.9,91.6 196.7,94.7 160.5,105.4 124.4,105.9 88.2,59.3 52.0,66.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,187.1 88.2,185.7 124.4,120.1 160.5,120.0 196.7,106.9 232.9,111.1 269.1,102.2 305.3,93.6 341.5,90.1 377.6,88.8 413.8,81.9 450.0,76.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,85.8 88.2,68.4 124.4,109.9 160.5,112.2 196.7,101.3 232.9,102.1 269.1,94.3 305.3,87.5 341.5,85.1 377.6,80.2 413.8,73.1 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,69.6 88.2,65.9 124.4,109.4 160.5,109.0 196.7,96.9 232.9,96.2 269.1,91.3 305.3,81.7 341.5,75.5 377.6,72.2 413.8,63.1 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.8 88.2,63.1 124.4,110.7 160.5,106.3 196.7,98.1 232.9,92.8 269.1,88.0 305.3,77.8 341.5,71.2 377.6,65.9 413.8,55.1 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,66.6 88.2,59.3 124.4,105.9 160.5,105.4 196.7,94.7 232.9,91.6 269.1,80.7 305.3,74.4 341.5,66.8 377.6,59.0 413.8,48.1 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 135 ns | 187 ns | 206 ns | 188 ns |
| D38 | 4.36 ns | 203 ns | 187 ns | 189 ns | 207 ns |
| D57 | 277 ns | 428 ns | 434 ns | 424 ns | 502 ns |
| D76 | 277 ns | 379 ns | 456 ns | 515 ns | 535 ns |
| D115 | 639 ns | 715 ns | 855 ns | 826 ns | 959 ns |
| D153 | 485 ns | 727 ns | 906 ns | 1.05 µs | 1.17 µs |
| D230 | 891 ns | 1.14 µs | 1.24 µs | 1.43 µs | 2.07 µs |
| D307 | 1.37 µs | 1.64 µs | 2.17 µs | 2.58 µs | 2.97 µs |
| D462 | 1.55 µs | 1.78 µs | 2.89 µs | 3.53 µs | 4.43 µs |
| D616 | 1.74 µs | 2.27 µs | 3.47 µs | 4.74 µs | 6.76 µs |
| D924 | 2.59 µs | 3.43 µs | 5.67 µs | 8.47 µs | 12.2 µs |
| D1232 | 3.4 µs | 5.29 µs | 7.96 µs | 12.2 µs | 31.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,112.3 160.5,112.3 196.7,97.8 232.9,102.6 269.1,92.0 305.3,84.5 341.5,82.4 377.6,80.3 413.8,73.4 450.0,68.7 450.0,30.1 413.8,46.5 377.6,56.8 341.5,64.1 305.3,71.1 269.1,77.3 232.9,87.3 196.7,90.7 160.5,100.9 124.4,102.0 88.2,117.3 52.0,119.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,112.3 160.5,112.3 196.7,97.8 232.9,102.6 269.1,92.0 305.3,84.5 341.5,82.4 377.6,80.3 413.8,73.4 450.0,68.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.8 88.2,117.7 124.4,104.8 160.5,106.8 196.7,95.8 232.9,95.5 269.1,87.7 305.3,81.4 341.5,79.9 377.6,75.7 413.8,68.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.1 88.2,119.1 124.4,104.5 160.5,103.7 196.7,92.7 232.9,91.7 269.1,86.2 305.3,76.5 341.5,71.5 377.6,68.4 413.8,59.9 450.0,54.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.5 88.2,118.9 124.4,104.9 160.5,101.5 196.7,93.3 232.9,89.2 269.1,83.8 305.3,73.6 341.5,68.1 377.6,63.0 413.8,52.9 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.0 88.2,117.3 124.4,102.0 160.5,100.9 196.7,90.7 232.9,87.3 269.1,77.3 305.3,71.1 341.5,64.1 377.6,56.8 413.8,46.5 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
