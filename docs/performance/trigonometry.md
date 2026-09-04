# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.94 ns | 2 µs | 3.49 µs | 3.19 µs | 3.77 µs |
| D38 | 1.41 µs | 5.18 µs | 4.48 µs | 8.68 µs | 10.4 µs |
| D57 | 1.44 µs | 5.68 µs | 8.19 µs | 10.6 µs | 11.5 µs |
| D76 | 1.35 µs | 7.19 µs | 10.3 µs | 12.2 µs | 19.1 µs |
| D115 | 1.57 µs | 8.43 µs | 19.2 µs | 32.4 µs | 53.1 µs |
| D153 | 1.48 µs | 9.87 µs | 24.7 µs | 42.2 µs | 60.9 µs |
| D230 | 1.62 µs | 19.8 µs | 28.1 µs | 45.8 µs | 124 µs |
| D307 | 1.45 µs | 26.1 µs | 66.2 µs | 124 µs | 133 µs |
| D462 | 1.58 µs | 33.7 µs | 134 µs | 224 µs | 402 µs |
| D616 | 1.56 µs | 52.2 µs | 208 µs | 305 µs | 660 µs |
| D924 | 1.42 µs | 125 µs | 424 µs | 876 µs | 1.17 ms |
| D1232 | 2.08 µs | 207 µs | 713 µs | 1.65 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.3 88.2,120.0 124.4,119.7 160.5,120.5 196.7,118.7 232.9,119.4 269.1,118.3 305.3,119.7 341.5,118.6 377.6,118.8 413.8,119.9 450.0,115.2 450.0,25.4 413.8,36.7 377.6,43.7 341.5,49.9 305.3,63.6 269.1,64.5 232.9,73.3 196.7,75.0 160.5,87.7 124.4,94.0 88.2,95.3 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.3 88.2,120.0 124.4,119.7 160.5,120.5 196.7,118.7 232.9,119.4 269.1,118.3 305.3,119.7 341.5,118.6 377.6,118.8 413.8,119.9 450.0,115.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.7 88.2,103.9 124.4,102.7 160.5,99.8 196.7,97.8 232.9,95.9 269.1,87.2 305.3,83.8 341.5,80.6 377.6,75.2 413.8,64.3 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,105.7 124.4,98.2 160.5,95.3 196.7,87.6 232.9,84.5 269.1,82.9 305.3,72.3 341.5,63.5 377.6,58.1 413.8,49.2 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,97.5 124.4,95.0 160.5,93.3 196.7,81.1 232.9,77.9 269.1,76.8 305.3,64.4 341.5,57.2 377.6,53.3 413.8,40.2 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,95.3 124.4,94.0 160.5,87.7 196.7,75.0 232.9,73.3 269.1,64.5 305.3,63.6 341.5,49.9 377.6,43.7 413.8,36.7 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.39 µs | 3.61 µs | 6.07 µs | 5.91 µs | 6.83 µs |
| D38 | 3.41 µs | 6.07 µs | 7.42 µs | 8.75 µs | 10.2 µs |
| D57 | 3.42 µs | 3.99 µs | 5.06 µs | 6.16 µs | 6.19 µs |
| D76 | 3.14 µs | 5.12 µs | 5.91 µs | 6.55 µs | 9.52 µs |
| D115 | 6.73 µs | 8.47 µs | 12.3 µs | 16.5 µs | 27.3 µs |
| D153 | 6.28 µs | 9.86 µs | 14.5 µs | 21.4 µs | 30.4 µs |
| D230 | 8.93 µs | 15.8 µs | 17.8 µs | 26.1 µs | 69.1 µs |
| D307 | 12.8 µs | 27.8 µs | 51.6 µs | 85.7 µs | 95.1 µs |
| D462 | 12.4 µs | 25.4 µs | 89.6 µs | 150 µs | 276 µs |
| D616 | 21.6 µs | 57.8 µs | 170 µs | 225 µs | 519 µs |
| D924 | 27.7 µs | 152 µs | 378 µs | 752 µs | 1.02 ms |
| D1232 | 46 µs | 260 µs | 720 µs | 1.54 ms | 2.57 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.5 88.2,183.4 124.4,183.3 160.5,185.1 196.7,168.6 232.9,170.1 269.1,162.5 305.3,154.7 341.5,155.4 377.6,143.2 413.8,137.9 450.0,126.9 450.0,39.5 413.8,59.6 377.6,74.3 341.5,88.0 305.3,111.1 269.1,118.0 232.9,135.9 196.7,138.2 160.5,161.1 124.4,170.4 88.2,159.6 52.0,168.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.5 88.2,183.4 124.4,183.3 160.5,185.1 196.7,168.6 232.9,170.1 269.1,162.5 305.3,154.7 341.5,155.4 377.6,143.2 413.8,137.9 450.0,126.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.2 88.2,170.8 124.4,179.9 160.5,174.5 196.7,163.6 232.9,160.3 269.1,150.1 305.3,137.8 341.5,139.8 377.6,121.9 413.8,100.9 450.0,89.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.9 88.2,166.5 124.4,174.8 160.5,171.4 196.7,155.5 232.9,151.9 269.1,147.5 305.3,124.4 341.5,112.4 377.6,98.5 413.8,81.1 450.0,67.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,171.4 88.2,162.9 124.4,170.5 160.5,169.2 196.7,149.1 232.9,143.5 269.1,139.2 305.3,113.4 341.5,101.2 377.6,92.4 413.8,66.2 450.0,50.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.3 88.2,159.6 124.4,170.4 160.5,161.1 196.7,138.2 232.9,135.9 269.1,118.0 305.3,111.1 341.5,88.0 377.6,74.3 413.8,59.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.58 ns | 1.99 µs | 3.49 µs | 3.18 µs | 3.76 µs |
| D38 | 1.31 µs | 5.17 µs | 4.42 µs | 8.65 µs | 10.3 µs |
| D57 | 1.31 µs | 5.68 µs | 8.15 µs | 10.6 µs | 11.5 µs |
| D76 | 1.22 µs | 7.17 µs | 10.3 µs | 12.1 µs | 19.1 µs |
| D115 | 1.46 µs | 8.37 µs | 19.1 µs | 32.6 µs | 52.1 µs |
| D153 | 1.36 µs | 9.83 µs | 24.7 µs | 42 µs | 60.9 µs |
| D230 | 1.53 µs | 20.7 µs | 28.3 µs | 45.4 µs | 123 µs |
| D307 | 1.33 µs | 25.5 µs | 66.2 µs | 125 µs | 134 µs |
| D462 | 1.39 µs | 33.8 µs | 134 µs | 222 µs | 399 µs |
| D616 | 1.42 µs | 52.8 µs | 206 µs | 305 µs | 659 µs |
| D924 | 1.34 µs | 125 µs | 424 µs | 874 µs | 1.16 ms |
| D1232 | 1.97 µs | 206 µs | 712 µs | 1.65 ms | 2.89 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,120.9 124.4,120.9 160.5,121.8 196.7,119.6 232.9,120.5 269.1,119.0 305.3,120.7 341.5,120.2 377.6,119.9 413.8,120.6 450.0,115.9 450.0,25.4 413.8,36.8 377.6,43.7 341.5,50.0 305.3,63.5 269.1,64.6 232.9,73.3 196.7,75.2 160.5,87.7 124.4,94.0 88.2,95.3 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,120.9 124.4,120.9 160.5,121.8 196.7,119.6 232.9,120.5 269.1,119.0 305.3,120.7 341.5,120.2 377.6,119.9 413.8,120.6 450.0,115.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.8 88.2,103.9 124.4,102.7 160.5,99.8 196.7,97.9 232.9,95.9 269.1,86.7 305.3,84.1 341.5,80.6 377.6,75.1 413.8,64.4 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,105.8 124.4,98.2 160.5,95.4 196.7,87.7 232.9,84.5 269.1,82.8 305.3,72.3 341.5,63.5 377.6,58.2 413.8,49.2 450.0,42.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,97.5 124.4,95.0 160.5,93.4 196.7,81.1 232.9,77.9 269.1,77.0 305.3,64.4 341.5,57.2 377.6,53.3 413.8,40.2 450.0,32.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,95.3 124.4,94.0 160.5,87.7 196.7,75.2 232.9,73.3 269.1,64.6 305.3,63.5 341.5,50.0 377.6,43.7 413.8,36.8 450.0,25.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.72 µs | 2.66 µs | 2.59 µs | 2.97 µs |
| D38 | 4.22 ns | 2.67 µs | 3.26 µs | 3.93 µs | 4.49 µs |
| D57 | 2.5 ns | 5.04 µs | 7.28 µs | 8.87 µs | 9.61 µs |
| D76 | 2.56 ns | 6.75 µs | 8.91 µs | 10.1 µs | 14.2 µs |
| D115 | 8.16 ns | 12.6 µs | 18.6 µs | 24.6 µs | 42.4 µs |
| D153 | 10.3 ns | 14.6 µs | 22.2 µs | 33 µs | 44.1 µs |
| D230 | 19.5 ns | 24.5 µs | 26.7 µs | 39.6 µs | 95.9 µs |
| D307 | 24.5 ns | 40.6 µs | 72.2 µs | 117 µs | 119 µs |
| D462 | 43.3 ns | 38.6 µs | 125 µs | 179 µs | 336 µs |
| D616 | 43.7 ns | 84.2 µs | 225 µs | 284 µs | 612 µs |
| D924 | 32.5 ns | 226 µs | 491 µs | 869 µs | 1.07 ms |
| D1232 | 76.6 ns | 359 µs | 924 µs | 1.76 ms | 2.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,198.6 160.5,198.3 196.7,184.0 232.9,181.0 269.1,173.1 305.3,170.3 341.5,163.2 377.6,163.1 413.8,166.8 450.0,156.2 450.0,27.7 413.8,37.7 377.6,44.7 341.5,52.1 305.3,65.0 269.1,67.7 232.9,77.3 196.7,77.8 160.5,91.4 124.4,96.2 88.2,105.7 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,198.6 160.5,198.3 196.7,184.0 232.9,181.0 269.1,173.1 305.3,170.3 341.5,163.2 377.6,163.1 413.8,166.8 450.0,156.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.6 88.2,112.1 124.4,104.2 160.5,100.6 196.7,92.9 232.9,91.0 269.1,84.6 305.3,78.3 341.5,79.0 377.6,69.3 413.8,57.0 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,109.6 124.4,99.7 160.5,97.1 196.7,88.0 232.9,85.8 269.1,83.5 305.3,71.2 341.5,64.3 377.6,57.1 413.8,47.4 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,107.3 124.4,97.2 160.5,95.6 196.7,84.5 232.9,80.9 269.1,78.6 305.3,65.2 341.5,59.9 377.6,54.2 413.8,40.3 450.0,31.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,105.7 124.4,96.2 160.5,91.4 196.7,77.8 232.9,77.3 269.1,67.7 305.3,65.0 341.5,52.1 377.6,44.7 413.8,37.7 450.0,27.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.39 ns | 1.57 µs | 2.91 µs | 2.69 µs | 3.23 µs |
| D38 | 3.16 ns | 2.96 µs | 3.42 µs | 3.7 µs | 4.25 µs |
| D57 | 1.7 ns | 2.65 µs | 3.54 µs | 4.19 µs | 4.19 µs |
| D76 | 468 ns | 5.59 µs | 8.08 µs | 9.96 µs | 15.9 µs |
| D115 | 595 ns | 6.44 µs | 16.2 µs | 30.7 µs | 48.4 µs |
| D153 | 554 ns | 7.73 µs | 16.2 µs | 38.9 µs | 55.5 µs |
| D230 | 659 ns | 17.1 µs | 25.6 µs | 42.3 µs | 114 µs |
| D307 | 573 ns | 21.9 µs | 54.5 µs | 115 µs | 123 µs |
| D462 | 596 ns | 27.2 µs | 112 µs | 194 µs | 339 µs |
| D616 | 627 ns | 48.8 µs | 191 µs | 285 µs | 624 µs |
| D924 | 671 ns | 115 µs | 396 µs | 829 µs | 1.11 ms |
| D1232 | 1.06 µs | 192 µs | 673 µs | 1.59 ms | 2.77 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.6 88.2,195.7 124.4,203.4 160.5,133.7 196.7,130.7 232.9,131.6 269.1,129.5 305.3,131.2 341.5,130.7 377.6,130.1 413.8,129.2 450.0,123.6 450.0,25.9 413.8,37.3 377.6,44.4 341.5,52.0 305.3,64.6 269.1,65.5 232.9,74.5 196.7,76.1 160.5,90.0 124.4,106.5 88.2,106.3 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.6 88.2,195.7 124.4,203.4 160.5,133.7 196.7,130.7 232.9,131.6 269.1,129.5 305.3,131.2 341.5,130.7 377.6,130.1 413.8,129.2 450.0,123.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.7 88.2,110.8 124.4,112.2 160.5,102.9 196.7,101.2 232.9,98.9 269.1,89.1 305.3,86.0 341.5,83.3 377.6,76.0 413.8,65.4 450.0,59.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.0 88.2,109.0 124.4,108.6 160.5,98.4 196.7,89.7 232.9,89.7 269.1,84.0 305.3,74.7 341.5,65.7 377.6,59.1 413.8,50.1 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,108.0 124.4,106.5 160.5,95.8 196.7,81.8 232.9,78.9 269.1,77.8 305.3,65.4 341.5,58.9 377.6,54.2 413.8,40.9 450.0,32.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,106.3 124.4,106.5 160.5,90.0 196.7,76.1 232.9,74.5 269.1,65.5 305.3,64.6 341.5,52.0 377.6,44.4 413.8,37.3 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.72 µs | 3 µs | 2.89 µs | 3.42 µs |
| D38 | 4.22 ns | 2.99 µs | 3.75 µs | 4.36 µs | 5.11 µs |
| D57 | 455 ns | 5.19 µs | 7.17 µs | 8.89 µs | 9.33 µs |
| D76 | 374 ns | 6.76 µs | 8.42 µs | 9.43 µs | 14.4 µs |
| D115 | 1.04 µs | 11.7 µs | 17.5 µs | 25.1 µs | 42.7 µs |
| D153 | 902 ns | 14 µs | 22.1 µs | 33.6 µs | 49.6 µs |
| D230 | 1.41 µs | 23.2 µs | 27.1 µs | 44.9 µs | 118 µs |
| D307 | 1.95 µs | 42.7 µs | 84.6 µs | 147 µs | 166 µs |
| D462 | 2.01 µs | 39.9 µs | 153 µs | 269 µs | 501 µs |
| D616 | 3.57 µs | 91.7 µs | 304 µs | 398 µs | 952 µs |
| D924 | 4.38 µs | 255 µs | 669 µs | 1.39 ms | 1.88 ms |
| D1232 | 7.16 µs | 449 µs | 1.3 ms | 2.87 ms | 4.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,192.1 124.4,134.0 160.5,136.5 196.7,123.8 232.9,125.6 269.1,120.1 305.3,116.0 341.5,115.6 377.6,108.5 413.8,106.0 450.0,99.9 450.0,19.0 413.8,30.7 377.6,39.2 341.5,47.1 305.3,60.9 269.1,65.1 232.9,75.8 196.7,77.7 160.5,91.2 124.4,96.6 88.2,104.0 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,192.1 124.4,134.0 160.5,136.5 196.7,123.8 232.9,125.6 269.1,120.1 305.3,116.0 341.5,115.6 377.6,108.5 413.8,106.0 450.0,99.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,110.7 124.4,103.9 160.5,100.6 196.7,93.7 232.9,91.5 269.1,85.3 305.3,77.7 341.5,78.6 377.6,68.2 413.8,55.5 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,107.9 124.4,99.8 160.5,97.9 196.7,88.7 232.9,85.9 269.1,83.3 305.3,69.2 341.5,61.9 377.6,53.4 413.8,43.6 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,106.0 124.4,97.2 160.5,96.4 196.7,84.3 232.9,80.7 269.1,77.1 305.3,62.3 341.5,54.8 377.6,50.0 413.8,34.5 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,104.0 124.4,96.6 160.5,91.2 196.7,77.7 232.9,75.8 269.1,65.1 305.3,60.9 341.5,47.1 377.6,39.2 413.8,30.7 450.0,19.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.75 ns | 1.18 µs | 2.7 µs | 2.13 µs | 2.84 µs |
| D38 | 5.62 ns | 2.7 µs | 3.23 µs | 3.74 µs | 4.13 µs |
| D57 | 2.59 ns | 3.37 µs | 4.7 µs | 5.62 µs | 7.22 µs |
| D76 | 3.17 ns | 4.1 µs | 5.59 µs | 6.22 µs | 9.29 µs |
| D115 | 13 ns | 4.72 µs | 10 µs | 13.6 µs | 22.2 µs |
| D153 | 16.8 ns | 5.42 µs | 9.73 µs | 18.2 µs | 29.2 µs |
| D230 | 40.8 ns | 9.96 µs | 12.1 µs | 22.7 µs | 70.1 µs |
| D307 | 64.6 ns | 13 µs | 27.6 µs | 70.8 µs | 81.1 µs |
| D462 | 105 ns | 11.3 µs | 66.4 µs | 125 µs | 231 µs |
| D616 | 119 ns | 25.6 µs | 122 µs | 186 µs | 430 µs |
| D924 | 218 ns | 71.6 µs | 261 µs | 579 µs | 831 µs |
| D1232 | 370 ns | 123 µs | 459 µs | 1.13 ms | 2.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.7 88.2,188.6 124.4,198.2 160.5,195.7 196.7,178.2 232.9,175.0 269.1,164.0 305.3,158.3 341.5,152.2 377.6,150.7 413.8,143.2 450.0,136.6 450.0,29.4 413.8,40.9 377.6,49.0 341.5,56.7 305.3,69.7 269.1,71.6 232.9,82.4 196.7,85.8 160.5,96.6 124.4,99.7 88.2,106.7 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.7 88.2,188.6 124.4,198.2 160.5,195.7 196.7,178.2 232.9,175.0 269.1,164.0 305.3,158.3 341.5,152.2 377.6,150.7 413.8,143.2 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.2 88.2,112.0 124.4,109.2 160.5,106.8 196.7,105.0 232.9,103.3 269.1,95.8 305.3,92.4 341.5,94.2 377.6,84.1 413.8,71.3 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,109.7 124.4,105.1 160.5,102.9 196.7,95.7 232.9,96.1 269.1,93.4 305.3,83.1 341.5,72.2 377.6,64.6 413.8,55.3 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.9 88.2,107.9 124.4,102.9 160.5,101.6 196.7,91.9 232.9,88.3 269.1,85.5 305.3,71.4 341.5,64.4 377.6,59.5 413.8,45.4 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,106.7 124.4,99.7 160.5,96.6 196.7,85.8 232.9,82.4 269.1,71.6 305.3,69.7 341.5,56.7 377.6,49.0 413.8,40.9 450.0,29.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.73 µs | 3.63 µs | 3.05 µs | 3.77 µs |
| D38 | 4.57 ns | 3.62 µs | 4.3 µs | 4.84 µs | 5.37 µs |
| D57 | 3.25 ns | 5.14 µs | 7.05 µs | 7.91 µs | 8.5 µs |
| D76 | 3.44 ns | 6.41 µs | 7.83 µs | 8.71 µs | 12.3 µs |
| D115 | 10.9 ns | 12.1 µs | 12.3 µs | 22 µs | 33.4 µs |
| D153 | 16.2 ns | 7.87 µs | 15.4 µs | 22.8 µs | 35.7 µs |
| D230 | 40.8 ns | 14.1 µs | 15.1 µs | 26.4 µs | 79.7 µs |
| D307 | 65.5 ns | 17.2 µs | 53 µs | 81.1 µs | 85.3 µs |
| D462 | 105 ns | 17.7 µs | 87.3 µs | 151 µs | 250 µs |
| D616 | 119 ns | 31.2 µs | 135 µs | 201 µs | 417 µs |
| D924 | 184 ns | 82.6 µs | 273 µs | 570 µs | 719 µs |
| D1232 | 355 ns | 135 µs | 450 µs | 995 µs | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,195.4 160.5,194.7 196.7,180.4 232.9,175.4 269.1,164.0 305.3,158.1 341.5,152.2 377.6,150.7 413.8,145.3 450.0,137.1 450.0,27.2 413.8,42.7 377.6,49.4 341.5,55.8 305.3,69.1 269.1,70.0 232.9,79.9 196.7,80.8 160.5,93.1 124.4,97.7 88.2,103.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,195.4 160.5,194.7 196.7,180.4 232.9,175.4 269.1,164.0 305.3,158.1 341.5,152.2 377.6,150.7 413.8,145.3 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,108.3 124.4,104.0 160.5,101.2 196.7,93.3 232.9,98.7 269.1,91.4 305.3,89.0 341.5,88.6 377.6,81.6 413.8,69.5 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,106.2 124.4,100.1 160.5,98.8 196.7,93.1 232.9,90.3 269.1,90.6 305.3,75.0 341.5,68.8 377.6,63.5 413.8,54.7 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,104.7 124.4,98.6 160.5,97.4 196.7,85.9 232.9,85.5 269.1,83.7 305.3,69.7 341.5,62.0 377.6,58.5 413.8,45.6 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,103.4 124.4,97.7 160.5,93.1 196.7,80.8 232.9,79.9 269.1,70.0 305.3,69.1 341.5,55.8 377.6,49.4 413.8,42.7 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.22 ns | 1.07 µs | 2.53 µs | 2.09 µs | 2.8 µs |
| D38 | 4.92 ns | 2.53 µs | 3.06 µs | 3.56 µs | 4.09 µs |
| D57 | 2.19 ns | 3.13 µs | 4.44 µs | 5.53 µs | 7.2 µs |
| D76 | 3.61 ns | 3.89 µs | 5.32 µs | 6.3 µs | 8.97 µs |
| D115 | 13 ns | 4.43 µs | 9.78 µs | 13.4 µs | 22 µs |
| D153 | 16.9 ns | 5.09 µs | 9.55 µs | 17.9 µs | 27.9 µs |
| D230 | 40.7 ns | 10 µs | 11.5 µs | 22.4 µs | 66.6 µs |
| D307 | 60.8 ns | 12.4 µs | 26.3 µs | 67.1 µs | 79 µs |
| D462 | 100 ns | 11.1 µs | 63 µs | 123 µs | 228 µs |
| D616 | 113 ns | 25.1 µs | 120 µs | 186 µs | 427 µs |
| D924 | 158 ns | 68.5 µs | 255 µs | 574 µs | 813 µs |
| D1232 | 361 ns | 120 µs | 458 µs | 1.12 ms | 2.08 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.1 88.2,190.2 124.4,200.3 160.5,194.1 196.7,178.1 232.9,174.9 269.1,164.0 305.3,159.0 341.5,152.8 377.6,151.3 413.8,147.2 450.0,136.9 450.0,29.5 413.8,41.1 377.6,49.1 341.5,56.9 305.3,70.1 269.1,72.2 232.9,83.0 196.7,85.9 160.5,97.1 124.4,99.8 88.2,106.8 52.0,111.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.1 88.2,190.2 124.4,200.3 160.5,194.1 196.7,178.1 232.9,174.9 269.1,164.0 305.3,159.0 341.5,152.8 377.6,151.3 413.8,147.2 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.5 88.2,112.8 124.4,110.1 160.5,107.4 196.7,105.8 232.9,104.1 269.1,95.7 305.3,93.0 341.5,94.4 377.6,84.3 413.8,71.8 450.0,64.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,110.4 124.4,105.8 160.5,103.6 196.7,96.0 232.9,96.3 269.1,93.9 305.3,83.7 341.5,72.9 377.6,64.8 413.8,55.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.1 88.2,108.5 124.4,103.1 160.5,101.5 196.7,92.0 232.9,88.5 269.1,85.7 305.3,72.1 341.5,64.5 377.6,59.5 413.8,45.5 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.5 88.2,106.8 124.4,99.8 160.5,97.1 196.7,85.9 232.9,83.0 269.1,72.2 305.3,70.1 341.5,56.9 377.6,49.1 413.8,41.1 450.0,29.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.52 ns | 1.73 µs | 3.63 µs | 3.05 µs | 3.76 µs |
| D38 | 4.57 ns | 3.63 µs | 4.31 µs | 4.85 µs | 5.37 µs |
| D57 | 10.4 ns | 5.17 µs | 7.09 µs | 7.95 µs | 8.51 µs |
| D76 | 8.67 ns | 6.39 µs | 7.86 µs | 8.73 µs | 12.4 µs |
| D115 | 11.2 ns | 12.3 µs | 11.9 µs | 21.9 µs | 33.7 µs |
| D153 | 16.2 ns | 7.92 µs | 15.3 µs | 22.9 µs | 35.6 µs |
| D230 | 39.9 ns | 14.2 µs | 15.2 µs | 26.6 µs | 79.9 µs |
| D307 | 60.3 ns | 17.3 µs | 53.4 µs | 81.5 µs | 86.3 µs |
| D462 | 99.7 ns | 17.8 µs | 86.8 µs | 152 µs | 251 µs |
| D616 | 112 ns | 31.2 µs | 135 µs | 200 µs | 417 µs |
| D924 | 168 ns | 82.7 µs | 273 µs | 567 µs | 715 µs |
| D1232 | 357 ns | 136 µs | 449 µs | 996 µs | 2.48 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.4 88.2,191.2 124.4,181.0 160.5,183.2 196.7,180.0 232.9,175.4 269.1,164.3 305.3,159.1 341.5,152.9 377.6,151.5 413.8,146.4 450.0,137.1 450.0,27.3 413.8,42.7 377.6,49.4 341.5,55.7 305.3,69.0 269.1,69.9 232.9,80.0 196.7,80.6 160.5,93.0 124.4,97.7 88.2,103.4 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.4 88.2,191.2 124.4,181.0 160.5,183.2 196.7,180.0 232.9,175.4 269.1,164.3 305.3,159.1 341.5,152.9 377.6,151.5 413.8,146.4 450.0,137.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.5 88.2,108.3 124.4,103.9 160.5,101.3 196.7,93.2 232.9,98.6 269.1,91.4 305.3,88.9 341.5,88.6 377.6,81.6 413.8,69.5 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,106.2 124.4,100.0 160.5,98.7 196.7,93.5 232.9,90.4 269.1,90.5 305.3,74.9 341.5,68.9 377.6,63.4 413.8,54.7 450.0,48.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.4 88.2,104.7 124.4,98.6 160.5,97.4 196.7,86.0 232.9,85.4 269.1,83.6 305.3,69.7 341.5,62.0 377.6,58.6 413.8,45.6 450.0,38.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,103.4 124.4,97.7 160.5,93.0 196.7,80.6 232.9,80.0 269.1,69.9 305.3,69.0 341.5,55.7 377.6,49.4 413.8,42.7 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.25 ns | 2.13 µs | 4.29 µs | 3.76 µs | 4.62 µs |
| D38 | 4.92 ns | 4.29 µs | 5.13 µs | 5.92 µs | 6.64 µs |
| D57 | 2.92 ns | 4.21 µs | 5.95 µs | 7.27 µs | 7.25 µs |
| D76 | 3.46 ns | 5.15 µs | 7.16 µs | 7.85 µs | 11.2 µs |
| D115 | 10.4 ns | 5.86 µs | 12.2 µs | 16.3 µs | 26.3 µs |
| D153 | 16.5 ns | 6.86 µs | 12.2 µs | 21.7 µs | 32.7 µs |
| D230 | 41.1 ns | 12.7 µs | 14.2 µs | 26.1 µs | 75.5 µs |
| D307 | 58.8 ns | 15.8 µs | 31.6 µs | 75.6 µs | 86 µs |
| D462 | 94.3 ns | 13.9 µs | 71.7 µs | 136 µs | 248 µs |
| D616 | 117 ns | 29 µs | 134 µs | 203 µs | 462 µs |
| D924 | 151 ns | 77.6 µs | 280 µs | 617 µs | 857 µs |
| D1232 | 379 ns | 134 µs | 490 µs | 1.2 ms | 2.19 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,192.0 88.2,190.2 124.4,196.7 160.5,194.6 196.7,180.9 232.9,175.2 269.1,163.9 305.3,159.4 341.5,153.6 377.6,151.0 413.8,147.8 450.0,136.3 450.0,28.9 413.8,40.5 377.6,48.2 341.5,55.9 305.3,69.0 269.1,70.6 232.9,81.0 196.7,83.7 160.5,94.3 124.4,99.7 88.2,100.8 52.0,105.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,192.0 88.2,190.2 124.4,196.7 160.5,194.6 196.7,180.9 232.9,175.2 269.1,163.9 305.3,159.4 341.5,153.6 377.6,151.0 413.8,147.8 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,106.2 124.4,106.4 160.5,104.0 196.7,102.3 232.9,100.4 269.1,92.7 305.3,90.0 341.5,91.6 377.6,82.5 413.8,70.3 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,104.0 124.4,102.2 160.5,99.9 196.7,93.2 232.9,93.2 269.1,91.4 305.3,81.4 341.5,71.3 377.6,63.5 413.8,54.4 450.0,47.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.2 124.4,99.7 160.5,98.7 196.7,89.7 232.9,86.1 269.1,83.8 305.3,70.6 341.5,63.3 377.6,58.3 413.8,44.6 450.0,36.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,100.8 124.4,99.7 160.5,94.3 196.7,83.7 232.9,81.0 269.1,70.6 305.3,69.0 341.5,55.9 377.6,48.2 413.8,40.5 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.76 µs | 3.83 µs | 3.2 µs | 3.95 µs |
| D38 | 4.22 ns | 3.81 µs | 4.36 µs | 4.88 µs | 5.42 µs |
| D57 | 2.58 µs | 5.31 µs | 7.54 µs | 8.35 µs | 8.76 µs |
| D76 | 2.31 µs | 6.58 µs | 8.24 µs | 9.04 µs | 12.9 µs |
| D115 | 5.78 µs | 12.6 µs | 12.6 µs | 22.8 µs | 34.8 µs |
| D153 | 2.77 µs | 8.32 µs | 16.2 µs | 23.5 µs | 36.5 µs |
| D230 | 3.07 µs | 14.7 µs | 15.5 µs | 27.1 µs | 81.2 µs |
| D307 | 2.92 µs | 18.4 µs | 54.8 µs | 82.8 µs | 86.7 µs |
| D462 | 3.06 µs | 18.5 µs | 88.6 µs | 153 µs | 254 µs |
| D616 | 3.14 µs | 31.9 µs | 137 µs | 202 µs | 421 µs |
| D924 | 3.12 µs | 85 µs | 277 µs | 573 µs | 727 µs |
| D1232 | 4.67 µs | 137 µs | 455 µs | 1.01 ms | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,192.1 124.4,112.5 160.5,113.9 196.7,102.5 232.9,111.6 269.1,110.4 305.3,111.0 341.5,110.4 377.6,110.1 413.8,110.2 450.0,105.2 450.0,27.2 413.8,42.5 377.6,49.3 341.5,55.6 305.3,68.9 269.1,69.7 232.9,79.6 196.7,80.2 160.5,92.6 124.4,97.4 88.2,103.3 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,192.1 124.4,112.5 160.5,113.9 196.7,102.5 232.9,111.6 269.1,110.4 305.3,111.0 341.5,110.4 377.6,110.1 413.8,110.2 450.0,105.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.3 88.2,107.7 124.4,103.6 160.5,100.9 196.7,92.8 232.9,98.0 269.1,90.9 305.3,88.2 341.5,88.1 377.6,81.3 413.8,69.2 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,106.0 124.4,99.2 160.5,98.1 196.7,92.8 232.9,89.8 269.1,90.3 305.3,74.6 341.5,68.6 377.6,63.2 413.8,54.5 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,104.6 124.4,98.0 160.5,97.0 196.7,85.5 232.9,85.1 269.1,83.3 305.3,69.5 341.5,61.9 377.6,58.4 413.8,45.5 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,103.3 124.4,97.4 160.5,92.6 196.7,80.2 232.9,79.6 269.1,69.7 305.3,68.9 341.5,55.6 377.6,49.3 413.8,42.5 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 131 ns | 171 ns | 134 ns | 162 ns |
| D38 | 4.92 ns | 169 ns | 176 ns | 200 ns | 204 ns |
| D57 | 178 ns | 290 ns | 342 ns | 330 ns | 334 ns |
| D76 | 144 ns | 328 ns | 328 ns | 344 ns | 438 ns |
| D115 | 483 ns | 591 ns | 727 ns | 817 ns | 1.15 µs |
| D153 | 395 ns | 623 ns | 733 ns | 840 ns | 998 ns |
| D230 | 578 ns | 837 ns | 663 ns | 784 ns | 1.82 µs |
| D307 | 847 ns | 1.4 µs | 1.71 µs | 2.15 µs | 1.93 µs |
| D462 | 924 ns | 1.04 µs | 2.38 µs | 2.98 µs | 4.27 µs |
| D616 | 1.14 µs | 1.59 µs | 2.95 µs | 3.06 µs | 5.74 µs |
| D924 | 1.36 µs | 2.88 µs | 4.61 µs | 7.1 µs | 8.17 µs |
| D1232 | 2.48 µs | 4.28 µs | 7.62 µs | 12.1 µs | 27.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,120.0 160.5,123.6 196.7,102.7 232.9,106.1 269.1,99.5 305.3,92.9 341.5,91.4 377.6,87.7 413.8,84.6 450.0,74.3 450.0,32.5 413.8,53.5 377.6,59.6 341.5,64.8 305.3,78.6 269.1,79.6 232.9,90.0 196.7,87.5 160.5,104.3 124.4,109.0 88.2,117.6 52.0,121.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,120.0 160.5,123.6 196.7,102.7 232.9,106.1 269.1,99.5 305.3,92.9 341.5,91.4 377.6,87.7 413.8,84.6 450.0,74.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,125.3 88.2,120.9 124.4,111.5 160.5,109.4 196.7,99.1 232.9,98.2 269.1,93.1 305.3,84.2 341.5,89.3 377.6,82.0 413.8,71.7 450.0,64.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,120.2 124.4,108.6 160.5,109.4 196.7,95.5 232.9,95.4 269.1,97.1 305.3,80.7 341.5,74.9 377.6,71.2 413.8,63.5 450.0,54.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.9 88.2,118.0 124.4,109.3 160.5,108.5 196.7,93.5 232.9,93.0 269.1,94.2 305.3,76.7 341.5,71.0 377.6,70.6 413.8,56.0 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.7 88.2,117.6 124.4,109.0 160.5,104.3 196.7,87.5 232.9,90.0 269.1,79.6 305.3,78.6 341.5,64.8 377.6,59.6 413.8,53.5 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.57 ns | 155 ns | 212 ns | 188 ns | 202 ns |
| D38 | 4.92 ns | 209 ns | 217 ns | 216 ns | 221 ns |
| D57 | 242 ns | 346 ns | 390 ns | 400 ns | 384 ns |
| D76 | 201 ns | 407 ns | 422 ns | 405 ns | 534 ns |
| D115 | 601 ns | 711 ns | 807 ns | 974 ns | 1.33 µs |
| D153 | 526 ns | 742 ns | 832 ns | 955 ns | 1.16 µs |
| D230 | 766 ns | 994 ns | 707 ns | 910 ns | 2 µs |
| D307 | 1.12 µs | 1.64 µs | 2 µs | 2.48 µs | 2.16 µs |
| D462 | 1.18 µs | 1.15 µs | 2.65 µs | 3.31 µs | 4.64 µs |
| D616 | 1.47 µs | 1.71 µs | 3.32 µs | 3.35 µs | 6.17 µs |
| D924 | 1.75 µs | 3.27 µs | 5.05 µs | 7.62 µs | 8.57 µs |
| D1232 | 3.01 µs | 4.68 µs | 8.09 µs | 12.9 µs | 28.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.6 88.2,182.3 124.4,114.6 160.5,117.8 196.7,98.8 232.9,101.2 269.1,94.6 305.3,88.0 341.5,87.1 377.6,83.4 413.8,80.3 450.0,70.9 450.0,31.9 413.8,52.7 377.6,58.4 341.5,63.4 305.3,76.6 269.1,78.0 232.9,87.4 196.7,85.1 160.5,100.9 124.4,106.6 88.2,116.2 52.0,117.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.6 88.2,182.3 124.4,114.6 160.5,117.8 196.7,98.8 232.9,101.2 269.1,94.6 305.3,88.0 341.5,87.1 377.6,83.4 413.8,80.3 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.4 88.2,117.2 124.4,108.4 160.5,105.6 196.7,95.9 232.9,95.2 269.1,90.1 305.3,81.4 341.5,87.6 377.6,80.7 413.8,69.4 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.0 88.2,116.6 124.4,106.3 160.5,105.0 196.7,93.7 232.9,93.2 269.1,96.0 305.3,78.0 341.5,73.1 377.6,69.1 413.8,61.9 450.0,53.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,119.0 88.2,116.6 124.4,105.9 160.5,105.7 196.7,90.5 232.9,90.8 269.1,91.6 305.3,74.2 341.5,69.2 377.6,69.0 413.8,54.7 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.8 88.2,116.2 124.4,106.6 160.5,100.9 196.7,85.1 232.9,87.4 269.1,78.0 305.3,76.6 341.5,63.4 377.6,58.4 413.8,52.7 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
