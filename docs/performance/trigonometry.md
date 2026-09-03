# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.83 ns | 2.12 µs | 3.49 µs | 3.49 µs | 3.78 µs |
| D38 | 1.3 µs | 2.83 µs | 2.37 µs | 8.09 µs | 7.78 µs |
| D57 | 1.44 µs | 3.67 µs | 4.89 µs | 6.14 µs | 9.75 µs |
| D76 | 1.45 µs | 4.15 µs | 10.4 µs | 13.8 µs | 13.4 µs |
| D115 | 1.59 µs | 8.37 µs | 20.1 µs | 32.5 µs | 41.9 µs |
| D153 | 1.58 µs | 9.87 µs | 24.5 µs | 41.8 µs | 65.8 µs |
| D230 | 1.48 µs | 19.4 µs | 45.7 µs | 75.5 µs | 123 µs |
| D307 | 1.55 µs | 15.9 µs | 40.6 µs | 123 µs | 180 µs |
| D462 | 1.61 µs | 42.3 µs | 124 µs | 250 µs | 377 µs |
| D616 | 1.44 µs | 62.5 µs | 216 µs | 445 µs | 616 µs |
| D924 | 1.71 µs | 134 µs | 425 µs | 879 µs | 1.68 ms |
| D1232 | 1.99 µs | 207 µs | 662 µs | 1.3 ms | 3.25 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,121.1 124.4,119.8 160.5,119.7 196.7,118.5 232.9,118.6 269.1,119.4 305.3,118.8 341.5,118.4 377.6,119.8 413.8,117.6 450.0,115.8 450.0,23.9 413.8,32.1 377.6,44.6 341.5,50.7 305.3,59.9 269.1,64.6 232.9,72.3 196.7,77.9 160.5,92.1 124.4,96.0 88.2,98.8 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,121.1 124.4,119.8 160.5,119.7 196.7,118.5 232.9,118.6 269.1,119.4 305.3,118.8 341.5,118.4 377.6,119.8 413.8,117.6 450.0,115.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,111.4 124.4,108.2 160.5,106.6 196.7,97.9 232.9,95.9 269.1,87.5 305.3,89.9 341.5,77.8 377.6,73.0 413.8,63.5 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,113.6 124.4,104.6 160.5,95.3 196.7,87.1 232.9,84.6 269.1,76.9 305.3,78.3 341.5,64.5 377.6,57.6 413.8,49.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,98.3 124.4,101.8 160.5,91.7 196.7,81.1 232.9,78.0 269.1,70.6 305.3,64.5 341.5,55.8 377.6,48.6 413.8,40.2 450.0,35.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,98.8 124.4,96.0 160.5,92.1 196.7,77.9 232.9,72.3 269.1,64.6 305.3,59.9 341.5,50.7 377.6,44.6 413.8,32.1 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.23 µs | 3.77 µs | 6.07 µs | 6.41 µs | 6.85 µs |
| D38 | 3.21 µs | 3.87 µs | 4.88 µs | 8.36 µs | 8.35 µs |
| D57 | 3.36 µs | 2.51 µs | 2.94 µs | 3.57 µs | 5.25 µs |
| D76 | 3.37 µs | 2.65 µs | 5.85 µs | 7.16 µs | 6.9 µs |
| D115 | 6.66 µs | 8.57 µs | 13.3 µs | 16.5 µs | 21.2 µs |
| D153 | 6.68 µs | 9.72 µs | 14.6 µs | 21.6 µs | 32.9 µs |
| D230 | 8.47 µs | 15.9 µs | 28.5 µs | 46.3 µs | 69.1 µs |
| D307 | 13.2 µs | 16.4 µs | 30.3 µs | 85.8 µs | 134 µs |
| D462 | 13.2 µs | 37.4 µs | 86.5 µs | 166 µs | 264 µs |
| D616 | 18.2 µs | 73 µs | 182 µs | 330 µs | 490 µs |
| D924 | 32.6 µs | 164 µs | 384 µs | 757 µs | 1.44 ms |
| D1232 | 43.7 µs | 265 µs | 681 µs | 1.22 ms | 2.88 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.5 88.2,184.7 124.4,183.7 160.5,183.6 196.7,168.8 232.9,168.8 269.1,163.6 305.3,154.0 341.5,154.0 377.6,147.0 413.8,134.4 450.0,128.0 450.0,37.0 413.8,52.1 377.6,75.5 341.5,88.9 305.3,103.7 269.1,118.0 232.9,134.1 196.7,143.7 160.5,168.1 124.4,174.0 88.2,163.9 52.0,168.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.5 88.2,184.7 124.4,183.7 160.5,183.6 196.7,168.8 232.9,168.8 269.1,163.6 305.3,154.0 341.5,154.0 377.6,147.0 413.8,134.4 450.0,128.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,180.6 124.4,190.0 160.5,188.8 196.7,163.3 232.9,160.6 269.1,150.0 305.3,149.2 341.5,131.3 377.6,116.8 413.8,99.3 450.0,88.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.8 88.2,175.6 124.4,186.6 160.5,171.6 196.7,153.8 232.9,151.8 269.1,137.2 305.3,135.9 341.5,113.2 377.6,97.0 413.8,80.8 450.0,68.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,169.7 88.2,163.9 124.4,182.4 160.5,167.3 196.7,149.1 232.9,143.3 269.1,126.7 305.3,113.3 341.5,98.9 377.6,84.1 413.8,66.1 450.0,55.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.2 88.2,163.9 124.4,174.0 160.5,168.1 196.7,143.7 232.9,134.1 269.1,118.0 305.3,103.7 341.5,88.9 377.6,75.5 413.8,52.1 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.11 µs | 3.48 µs | 3.5 µs | 3.77 µs |
| D38 | 1.18 µs | 2.82 µs | 2.34 µs | 8.06 µs | 7.76 µs |
| D57 | 1.3 µs | 3.66 µs | 4.89 µs | 6.25 µs | 9.72 µs |
| D76 | 1.33 µs | 4.34 µs | 10.4 µs | 13.8 µs | 13.4 µs |
| D115 | 1.46 µs | 8.35 µs | 20 µs | 33 µs | 41.7 µs |
| D153 | 1.49 µs | 9.85 µs | 25.1 µs | 41.5 µs | 65.6 µs |
| D230 | 1.36 µs | 19.5 µs | 45 µs | 76.9 µs | 124 µs |
| D307 | 1.47 µs | 15.5 µs | 42.5 µs | 124 µs | 179 µs |
| D462 | 1.52 µs | 42.2 µs | 125 µs | 249 µs | 377 µs |
| D616 | 1.32 µs | 61.4 µs | 218 µs | 446 µs | 616 µs |
| D924 | 1.55 µs | 134 µs | 426 µs | 873 µs | 1.68 ms |
| D1232 | 1.87 µs | 207 µs | 663 µs | 1.29 ms | 3.26 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,122.2 124.4,121.0 160.5,120.7 196.7,119.6 232.9,119.3 269.1,120.5 305.3,119.5 341.5,119.0 377.6,120.9 413.8,118.9 450.0,116.5 450.0,23.9 413.8,32.1 377.6,44.6 341.5,50.7 305.3,59.9 269.1,64.5 232.9,72.4 196.7,78.0 160.5,92.1 124.4,96.1 88.2,98.9 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,122.2 124.4,121.0 160.5,120.7 196.7,119.6 232.9,119.3 269.1,120.5 305.3,119.5 341.5,119.0 377.6,120.9 413.8,118.9 450.0,116.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,111.4 124.4,108.2 160.5,106.1 196.7,97.9 232.9,95.9 269.1,87.4 305.3,90.3 341.5,77.9 377.6,73.2 413.8,63.5 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,113.7 124.4,104.6 160.5,95.3 196.7,87.1 232.9,84.3 269.1,77.0 305.3,77.8 341.5,64.3 377.6,57.5 413.8,49.2 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,98.4 124.4,101.5 160.5,91.7 196.7,80.9 232.9,78.1 269.1,70.4 305.3,64.5 341.5,55.8 377.6,48.6 413.8,40.3 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,98.9 124.4,96.1 160.5,92.1 196.7,78.0 232.9,72.4 269.1,64.5 305.3,59.9 341.5,50.7 377.6,44.6 413.8,32.1 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.66 µs | 2.82 µs | 2.97 µs |
| D38 | 3.73 ns | 1.67 µs | 2.26 µs | 3.71 µs | 3.7 µs |
| D57 | 1.96 ns | 3.22 µs | 4.45 µs | 5.37 µs | 8.19 µs |
| D76 | 2.34 ns | 3.74 µs | 8.72 µs | 10.9 µs | 10.3 µs |
| D115 | 12.4 ns | 12.6 µs | 21 µs | 24.5 µs | 33.4 µs |
| D153 | 16 ns | 14.8 µs | 21.7 µs | 33.9 µs | 47.2 µs |
| D230 | 27.5 ns | 24.3 µs | 44.7 µs | 66.7 µs | 95.2 µs |
| D307 | 55 ns | 24 µs | 43.2 µs | 115 µs | 165 µs |
| D462 | 85 ns | 54.8 µs | 118 µs | 204 µs | 318 µs |
| D616 | 62.5 ns | 103 µs | 243 µs | 436 µs | 569 µs |
| D924 | 102 ns | 240 µs | 496 µs | 871 µs | 1.6 ms |
| D1232 | 155 ns | 368 µs | 857 µs | 1.38 ms | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,199.4 196.7,178.7 232.9,175.6 269.1,168.9 305.3,160.3 341.5,154.9 377.6,158.7 413.8,152.6 450.0,147.4 450.0,25.6 413.8,32.7 377.6,45.6 341.5,52.8 305.3,60.9 269.1,67.8 232.9,76.5 196.7,80.7 160.5,95.3 124.4,98.2 88.2,108.1 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,201.7 160.5,199.4 196.7,178.7 232.9,175.6 269.1,168.9 305.3,160.3 341.5,154.9 377.6,158.7 413.8,152.6 450.0,147.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,118.0 124.4,109.8 160.5,107.9 196.7,92.8 232.9,90.9 269.1,84.7 305.3,84.9 341.5,74.6 377.6,66.7 413.8,56.3 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.1 88.2,114.2 124.4,105.8 160.5,97.4 196.7,86.5 232.9,86.1 269.1,77.1 305.3,77.6 341.5,65.1 377.6,56.1 413.8,47.3 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,108.0 124.4,103.4 160.5,94.7 196.7,84.6 232.9,80.6 269.1,72.2 305.3,65.4 341.5,58.3 377.6,48.9 413.8,40.3 450.0,34.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,108.1 124.4,98.2 160.5,95.3 196.7,80.7 232.9,76.5 269.1,67.8 305.3,60.9 341.5,52.8 377.6,45.6 413.8,32.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.64 µs | 2.9 µs | 2.96 µs | 3.22 µs |
| D38 | 638 ns | 2.4 µs | 1.7 µs | 6.25 µs | 6.19 µs |
| D57 | 542 ns | 2.85 µs | 3.78 µs | 4.94 µs | 3.53 µs |
| D76 | 526 ns | 3.32 µs | 8.15 µs | 11.5 µs | 11.1 µs |
| D115 | 615 ns | 6.57 µs | 17 µs | 29.2 µs | 38 µs |
| D153 | 621 ns | 7.7 µs | 16.7 µs | 37.9 µs | 61 µs |
| D230 | 569 ns | 16.2 µs | 40.5 µs | 69.1 µs | 115 µs |
| D307 | 619 ns | 13.2 µs | 34.6 µs | 114 µs | 168 µs |
| D462 | 674 ns | 34.1 µs | 105 µs | 218 µs | 322 µs |
| D616 | 555 ns | 56.1 µs | 203 µs | 419 µs | 583 µs |
| D924 | 760 ns | 124 µs | 396 µs | 831 µs | 1.58 ms |
| D1232 | 993 ns | 193 µs | 627 µs | 1.24 ms | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,129.9 124.4,131.9 160.5,132.3 196.7,130.3 232.9,130.2 269.1,131.3 305.3,130.2 341.5,129.2 377.6,131.6 413.8,127.7 450.0,124.4 450.0,24.4 413.8,32.9 377.6,45.3 341.5,52.6 305.3,60.7 269.1,65.5 232.9,73.3 196.7,79.2 160.5,94.4 124.4,108.6 88.2,101.7 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,129.9 124.4,131.9 160.5,132.3 196.7,130.3 232.9,130.2 269.1,131.3 305.3,130.2 341.5,129.2 377.6,131.6 413.8,127.7 450.0,124.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.1 88.2,113.4 124.4,111.3 160.5,109.4 196.7,100.9 232.9,99.0 269.1,89.8 305.3,92.3 341.5,80.5 377.6,74.3 413.8,64.4 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,117.7 124.4,107.8 160.5,98.3 196.7,89.2 232.9,89.3 269.1,78.3 305.3,80.3 341.5,66.5 377.6,58.3 413.8,50.1 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,101.5 124.4,104.5 160.5,93.9 196.7,82.4 232.9,79.2 269.1,71.7 305.3,65.6 341.5,57.5 377.6,49.4 413.8,40.9 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,101.7 124.4,108.6 160.5,94.4 196.7,79.2 232.9,73.3 269.1,65.5 305.3,60.7 341.5,52.6 377.6,45.3 413.8,32.9 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.99 µs | 3.16 µs | 3.43 µs |
| D38 | 3.73 ns | 1.94 µs | 2.43 µs | 4.16 µs | 4.23 µs |
| D57 | 445 ns | 3.27 µs | 4.15 µs | 5.08 µs | 7.86 µs |
| D76 | 444 ns | 3.52 µs | 8.13 µs | 10.4 µs | 10.4 µs |
| D115 | 1.05 µs | 12.3 µs | 19.2 µs | 25.2 µs | 33.2 µs |
| D153 | 1.05 µs | 13.6 µs | 21.8 µs | 34.3 µs | 54.3 µs |
| D230 | 1.35 µs | 23.1 µs | 43.9 µs | 76.8 µs | 118 µs |
| D307 | 1.99 µs | 24.7 µs | 49.6 µs | 148 µs | 238 µs |
| D462 | 2.06 µs | 58.4 µs | 149 µs | 299 µs | 480 µs |
| D616 | 2.58 µs | 119 µs | 316 µs | 589 µs | 899 µs |
| D924 | 4.86 µs | 275 µs | 689 µs | 1.4 ms | 2.68 ms |
| D1232 | 6.7 µs | 458 µs | 1.23 ms | 2.27 ms | 5.42 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,134.4 196.7,123.7 232.9,123.7 269.1,120.6 305.3,115.7 341.5,115.3 377.6,112.5 413.8,104.7 450.0,100.7 450.0,17.6 413.8,26.3 377.6,39.9 341.5,47.7 305.3,56.4 269.1,65.1 232.9,74.7 196.7,80.8 160.5,95.2 124.4,98.7 88.2,106.4 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,134.3 160.5,134.4 196.7,123.7 232.9,123.7 269.1,120.6 305.3,115.7 341.5,115.3 377.6,112.5 413.8,104.7 450.0,100.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,116.1 124.4,109.6 160.5,108.7 196.7,93.2 232.9,91.9 269.1,85.3 305.3,84.5 341.5,73.8 377.6,65.0 413.8,54.6 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.7 88.2,113.3 124.4,106.6 160.5,98.3 196.7,87.6 232.9,86.0 269.1,77.4 305.3,75.9 341.5,62.2 377.6,52.9 413.8,43.2 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,104.1 160.5,95.2 196.7,84.2 232.9,80.4 269.1,70.4 305.3,62.3 341.5,53.6 377.6,45.1 413.8,34.4 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,106.4 124.4,98.7 160.5,95.2 196.7,80.8 232.9,74.7 269.1,65.1 305.3,56.4 341.5,47.7 377.6,39.9 413.8,26.3 450.0,17.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.29 ns | 1.25 µs | 2.71 µs | 2.63 µs | 2.85 µs |
| D38 | 4.98 ns | 1.49 µs | 1.81 µs | 3.53 µs | 2.89 µs |
| D57 | 2.18 ns | 2.21 µs | 2.71 µs | 3.22 µs | 6.1 µs |
| D76 | 3.12 ns | 2.4 µs | 5.52 µs | 6.92 µs | 6.42 µs |
| D115 | 13 ns | 4.89 µs | 10 µs | 14.5 µs | 18.5 µs |
| D153 | 18.3 ns | 5.47 µs | 9.53 µs | 18.1 µs | 32.5 µs |
| D230 | 36.9 ns | 9.55 µs | 19.3 µs | 39.5 µs | 69.3 µs |
| D307 | 73.2 ns | 7.53 µs | 15.9 µs | 68.7 µs | 111 µs |
| D462 | 123 ns | 15.7 µs | 61.9 µs | 139 µs | 222 µs |
| D616 | 120 ns | 30.1 µs | 128 µs | 270 µs | 412 µs |
| D924 | 154 ns | 75.4 µs | 258 µs | 575 µs | 1.14 ms |
| D1232 | 368 ns | 123 µs | 429 µs | 890 µs | 2.3 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,189.3 88.2,190.1 124.4,200.3 160.5,195.9 196.7,178.2 232.9,174.0 269.1,165.2 305.3,156.7 341.5,150.3 377.6,150.5 413.8,147.5 450.0,136.7 450.0,28.2 413.8,36.9 377.6,49.6 341.5,57.3 305.3,65.8 269.1,71.7 232.9,81.1 196.7,88.1 160.5,101.2 124.4,101.8 88.2,111.1 52.0,111.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,189.3 88.2,190.1 124.4,200.3 160.5,195.9 196.7,178.2 232.9,174.0 269.1,165.2 305.3,156.7 341.5,150.3 377.6,150.5 413.8,147.5 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.5 88.2,119.4 124.4,114.5 160.5,113.4 196.7,104.6 232.9,103.2 269.1,96.3 305.3,99.2 341.5,90.2 377.6,82.1 413.8,70.7 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.9 88.2,116.9 124.4,111.9 160.5,103.1 196.7,95.7 232.9,96.3 269.1,87.6 305.3,90.0 341.5,73.1 377.6,64.1 413.8,55.4 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.3 88.2,108.6 124.4,109.8 160.5,100.3 196.7,91.1 232.9,88.4 269.1,78.7 305.3,71.8 341.5,63.1 377.6,54.8 413.8,45.4 450.0,40.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.3 88.2,111.1 124.4,101.8 160.5,101.2 196.7,88.1 232.9,81.1 269.1,71.7 305.3,65.8 341.5,57.3 377.6,49.6 413.8,36.9 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.89 µs | 3.64 µs | 3.56 µs | 3.79 µs |
| D38 | 4.05 ns | 2.16 µs | 2.47 µs | 4.47 µs | 3.94 µs |
| D57 | 3.12 ns | 3.43 µs | 4.32 µs | 4.85 µs | 7.25 µs |
| D76 | 3.43 ns | 3.68 µs | 7.57 µs | 9.44 µs | 8.98 µs |
| D115 | 10.9 ns | 12.3 µs | 12.7 µs | 21.8 µs | 26.4 µs |
| D153 | 18 ns | 7.93 µs | 15.2 µs | 22.9 µs | 39.6 µs |
| D230 | 37.3 ns | 14 µs | 24.5 µs | 48.5 µs | 80 µs |
| D307 | 75.8 ns | 10.7 µs | 32.5 µs | 81 µs | 119 µs |
| D462 | 123 ns | 23.2 µs | 81.7 µs | 165 µs | 236 µs |
| D616 | 129 ns | 35.6 µs | 142 µs | 291 µs | 408 µs |
| D924 | 169 ns | 87.7 µs | 272 µs | 564 µs | 1 ms |
| D1232 | 365 ns | 136 µs | 417 µs | 780 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,192.7 124.4,195.9 160.5,194.7 196.7,180.4 232.9,174.2 269.1,165.1 305.3,156.3 341.5,150.3 377.6,149.7 413.8,146.3 450.0,136.8 450.0,26.3 413.8,38.6 377.6,49.7 341.5,56.5 305.3,65.0 269.1,69.9 232.9,78.6 196.7,83.7 160.5,97.0 124.4,99.7 88.2,107.3 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,192.7 124.4,195.9 160.5,194.7 196.7,180.4 232.9,174.2 269.1,165.1 305.3,156.3 341.5,150.3 377.6,149.7 413.8,146.3 450.0,136.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.4 88.2,114.7 124.4,109.0 160.5,108.1 196.7,93.2 232.9,98.6 269.1,91.5 305.3,94.9 341.5,85.3 377.6,80.0 413.8,68.8 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,113.1 124.4,106.1 160.5,99.2 196.7,92.7 232.9,90.5 269.1,84.6 305.3,81.1 341.5,69.6 377.6,62.8 413.8,54.7 450.0,49.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.7 124.4,104.7 160.5,96.4 196.7,86.0 232.9,85.4 269.1,76.1 305.3,69.8 341.5,60.9 377.6,53.9 413.8,45.7 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,107.3 124.4,99.7 160.5,97.0 196.7,83.7 232.9,78.6 269.1,69.9 305.3,65.0 341.5,56.5 377.6,49.7 413.8,38.6 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.13 µs | 2.53 µs | 2.57 µs | 2.79 µs |
| D38 | 4.36 ns | 1.37 µs | 1.7 µs | 3.35 µs | 2.87 µs |
| D57 | 2.19 ns | 2.04 µs | 2.62 µs | 3.21 µs | 6.1 µs |
| D76 | 3.43 ns | 2.21 µs | 5.28 µs | 6.81 µs | 6.21 µs |
| D115 | 13 ns | 4.6 µs | 9.95 µs | 13.5 µs | 18 µs |
| D153 | 18.2 ns | 5.09 µs | 9.77 µs | 17.9 µs | 31.6 µs |
| D230 | 37.2 ns | 9.69 µs | 19.6 µs | 40.6 µs | 66.7 µs |
| D307 | 66.7 ns | 7.52 µs | 17.1 µs | 66 µs | 109 µs |
| D462 | 127 ns | 15.3 µs | 59.4 µs | 138 µs | 219 µs |
| D616 | 94.9 ns | 29.6 µs | 125 µs | 269 µs | 410 µs |
| D924 | 167 ns | 72.7 µs | 255 µs | 572 µs | 1.14 ms |
| D1232 | 363 ns | 120 µs | 429 µs | 884 µs | 2.28 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,191.7 124.4,200.3 160.5,194.7 196.7,178.2 232.9,174.0 269.1,165.1 305.3,157.9 341.5,149.8 377.6,153.5 413.8,146.5 450.0,136.9 450.0,28.3 413.8,37.0 377.6,49.6 341.5,57.4 305.3,66.1 269.1,72.2 232.9,81.4 196.7,88.4 160.5,101.6 124.4,101.8 88.2,111.2 52.0,111.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,191.7 124.4,200.3 160.5,194.7 196.7,178.2 232.9,174.0 269.1,165.1 305.3,157.9 341.5,149.8 377.6,153.5 413.8,146.5 450.0,136.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.8 88.2,120.4 124.4,115.5 160.5,114.4 196.7,105.4 232.9,104.1 269.1,96.1 305.3,99.2 341.5,90.4 377.6,82.3 413.8,71.1 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.8 88.2,117.7 124.4,112.4 160.5,103.6 196.7,95.8 232.9,96.0 269.1,87.4 305.3,89.0 341.5,73.6 377.6,64.4 413.8,55.5 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.6 88.2,109.3 124.4,109.8 160.5,100.5 196.7,92.0 232.9,88.5 269.1,78.3 305.3,72.3 341.5,63.1 377.6,54.9 413.8,45.5 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,111.2 124.4,101.8 160.5,101.6 196.7,88.4 232.9,81.4 269.1,72.2 305.3,66.1 341.5,57.4 377.6,49.6 413.8,37.0 450.0,28.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.89 µs | 3.64 µs | 3.56 µs | 3.78 µs |
| D38 | 4.05 ns | 2.14 µs | 2.47 µs | 4.47 µs | 3.95 µs |
| D57 | 10.4 ns | 3.43 µs | 4.64 µs | 4.87 µs | 7.26 µs |
| D76 | 10.4 ns | 3.68 µs | 7.59 µs | 9.46 µs | 9.01 µs |
| D115 | 11.2 ns | 12.4 µs | 12.8 µs | 22.1 µs | 26.9 µs |
| D153 | 17.4 ns | 7.98 µs | 15.4 µs | 23 µs | 39.4 µs |
| D230 | 37 ns | 14 µs | 24.8 µs | 47.7 µs | 80.3 µs |
| D307 | 65.1 ns | 10.5 µs | 32.1 µs | 80.9 µs | 119 µs |
| D462 | 124 ns | 23.3 µs | 81.8 µs | 164 µs | 236 µs |
| D616 | 101 ns | 36.3 µs | 142 µs | 289 µs | 406 µs |
| D924 | 173 ns | 87.6 µs | 272 µs | 565 µs | 1e+03 µs |
| D1232 | 381 ns | 136 µs | 415 µs | 779 µs | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,192.7 124.4,181.0 160.5,180.9 196.7,180.0 232.9,174.6 269.1,165.2 305.3,158.2 341.5,150.2 377.6,152.8 413.8,146.1 450.0,136.2 450.0,26.2 413.8,38.6 377.6,49.8 341.5,56.5 305.3,65.0 269.1,69.9 232.9,78.7 196.7,83.4 160.5,97.0 124.4,99.7 88.2,107.2 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,192.7 124.4,181.0 160.5,180.9 196.7,180.0 232.9,174.6 269.1,165.2 305.3,158.2 341.5,150.2 377.6,152.8 413.8,146.1 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.4 88.2,114.9 124.4,109.0 160.5,108.1 196.7,93.0 232.9,98.5 269.1,91.5 305.3,95.1 341.5,85.2 377.6,79.7 413.8,68.8 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.3 88.2,113.1 124.4,105.2 160.5,99.1 196.7,92.7 232.9,90.4 269.1,84.5 305.3,81.2 341.5,69.6 377.6,62.8 413.8,54.7 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,105.7 124.4,104.6 160.5,96.4 196.7,85.9 232.9,85.4 269.1,76.3 305.3,69.8 341.5,61.0 377.6,54.0 413.8,45.7 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,107.2 124.4,99.7 160.5,97.0 196.7,83.4 232.9,78.7 269.1,69.9 305.3,65.0 341.5,56.5 377.6,49.8 413.8,38.6 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 2.28 µs | 4.29 µs | 4.31 µs | 4.61 µs |
| D38 | 4.36 ns | 2.55 µs | 3.14 µs | 5.58 µs | 5.1 µs |
| D57 | 3.12 ns | 2.69 µs | 3.48 µs | 4.21 µs | 6.17 µs |
| D76 | 3.74 ns | 2.91 µs | 7.12 µs | 8.75 µs | 7.92 µs |
| D115 | 10.4 ns | 5.96 µs | 12.7 µs | 16 µs | 21.6 µs |
| D153 | 18 ns | 6.81 µs | 12.1 µs | 21.5 µs | 36.8 µs |
| D230 | 36.4 ns | 12.4 µs | 23.8 µs | 45.5 µs | 75.3 µs |
| D307 | 62.8 ns | 8.98 µs | 18.9 µs | 75.3 µs | 120 µs |
| D462 | 114 ns | 18.6 µs | 67.3 µs | 155 µs | 238 µs |
| D616 | 89.5 ns | 34.5 µs | 141 µs | 292 µs | 442 µs |
| D924 | 138 ns | 82.3 µs | 279 µs | 614 µs | 1.21 ms |
| D1232 | 371 ns | 134 µs | 464 µs | 944 µs | 2.41 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,191.7 124.4,195.9 160.5,193.6 196.7,180.9 232.9,174.2 269.1,165.4 305.3,158.6 341.5,151.2 377.6,154.2 413.8,148.9 450.0,136.6 450.0,27.6 413.8,36.3 377.6,48.7 341.5,56.4 305.3,64.9 269.1,70.7 232.9,79.5 196.7,86.2 160.5,98.6 124.4,101.7 88.2,104.1 52.0,105.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,191.7 124.4,195.9 160.5,193.6 196.7,180.9 232.9,174.2 269.1,165.4 305.3,158.6 341.5,151.2 377.6,154.2 413.8,148.9 450.0,136.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.1 88.2,112.7 124.4,112.0 160.5,111.1 196.7,102.1 232.9,100.5 269.1,93.0 305.3,97.1 341.5,88.0 377.6,80.4 413.8,69.6 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,110.1 124.4,108.8 160.5,99.9 196.7,92.8 232.9,93.3 269.1,85.0 305.3,87.8 341.5,72.1 377.6,62.9 413.8,54.4 450.0,48.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,102.9 124.4,106.5 160.5,97.4 196.7,89.9 232.9,86.2 269.1,76.9 305.3,70.7 341.5,61.7 377.6,53.9 413.8,44.6 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.3 88.2,104.1 124.4,101.7 160.5,98.6 196.7,86.2 232.9,79.5 269.1,70.7 305.3,64.9 341.5,56.4 377.6,48.7 413.8,36.3 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.92 µs | 3.82 µs | 3.73 µs | 3.97 µs |
| D38 | 3.74 ns | 2.17 µs | 2.76 µs | 4.49 µs | 3.97 µs |
| D57 | 2.56 µs | 4 µs | 4.48 µs | 4.95 µs | 7.51 µs |
| D76 | 2.58 µs | 3.8 µs | 8.09 µs | 9.91 µs | 9.42 µs |
| D115 | 5.68 µs | 12.8 µs | 13.7 µs | 22.6 µs | 27.2 µs |
| D153 | 2.97 µs | 8.35 µs | 15.7 µs | 23.7 µs | 40.1 µs |
| D230 | 2.83 µs | 14.6 µs | 25.3 µs | 49.1 µs | 81.9 µs |
| D307 | 3.17 µs | 11.1 µs | 33.2 µs | 82.3 µs | 120 µs |
| D462 | 3.49 µs | 24.2 µs | 83.5 µs | 167 µs | 239 µs |
| D616 | 2.87 µs | 36.6 µs | 144 µs | 294 µs | 410 µs |
| D924 | 3.45 µs | 89.2 µs | 276 µs | 571 µs | 1.01 ms |
| D1232 | 4.25 µs | 138 µs | 420 µs | 786 µs | 2.73 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,193.6 124.4,112.6 160.5,112.5 196.7,102.7 232.9,110.8 269.1,111.4 305.3,110.0 341.5,108.8 377.6,111.2 413.8,108.9 450.0,106.3 450.0,26.1 413.8,38.4 377.6,49.6 341.5,56.3 305.3,64.8 269.1,69.6 232.9,78.5 196.7,83.3 160.5,96.5 124.4,99.3 88.2,107.2 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,193.6 124.4,112.6 160.5,112.5 196.7,102.7 232.9,110.8 269.1,111.4 305.3,110.0 341.5,108.8 377.6,111.2 413.8,108.9 450.0,106.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.2 88.2,114.7 124.4,107.1 160.5,107.7 196.7,92.7 232.9,98.0 269.1,91.0 305.3,94.4 341.5,84.7 377.6,79.6 413.8,68.6 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,111.7 124.4,105.7 160.5,98.3 196.7,91.8 232.9,90.1 269.1,84.2 305.3,80.8 341.5,69.4 377.6,62.6 413.8,54.6 450.0,49.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,105.6 124.4,104.5 160.5,95.8 196.7,85.6 232.9,85.0 269.1,76.0 305.3,69.6 341.5,60.8 377.6,53.7 413.8,45.5 450.0,41.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,107.2 124.4,99.3 160.5,96.5 196.7,83.3 232.9,78.5 269.1,69.6 305.3,64.8 341.5,56.3 377.6,49.6 413.8,38.4 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 142 ns | 171 ns | 160 ns | 162 ns |
| D38 | 4.67 ns | 99.2 ns | 101 ns | 203 ns | 155 ns |
| D57 | 178 ns | 173 ns | 190 ns | 188 ns | 284 ns |
| D76 | 178 ns | 170 ns | 334 ns | 377 ns | 324 ns |
| D115 | 464 ns | 542 ns | 775 ns | 774 ns | 855 ns |
| D153 | 486 ns | 624 ns | 722 ns | 865 ns | 1.15 µs |
| D230 | 504 ns | 826 ns | 1.1 µs | 1.42 µs | 1.79 µs |
| D307 | 882 ns | 733 ns | 957 ns | 2.13 µs | 2.64 µs |
| D462 | 993 ns | 1.38 µs | 2.2 µs | 3.25 µs | 3.93 µs |
| D616 | 922 ns | 1.87 µs | 3.14 µs | 4.42 µs | 5.52 µs |
| D924 | 1.43 µs | 2.94 µs | 4.46 µs | 6.97 µs | 11.2 µs |
| D1232 | 2.19 µs | 4.14 µs | 7 µs | 9.27 µs | 28.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,183.2 124.4,119.9 160.5,120.0 196.7,103.4 232.9,102.5 269.1,101.9 305.3,92.2 341.5,90.1 377.6,91.4 413.8,83.8 450.0,76.4 450.0,31.7 413.8,48.0 377.6,60.3 341.5,66.2 305.3,73.1 269.1,79.9 232.9,87.5 196.7,92.7 160.5,109.6 124.4,111.8 88.2,122.4 52.0,121.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,183.2 124.4,119.9 160.5,120.0 196.7,103.4 232.9,102.5 269.1,101.9 305.3,92.2 341.5,90.1 377.6,91.4 413.8,83.8 450.0,76.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,123.9 88.2,130.1 124.4,120.4 160.5,120.8 196.7,100.6 232.9,98.2 269.1,93.3 305.3,95.4 341.5,84.4 377.6,79.1 413.8,71.3 450.0,65.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,129.8 124.4,118.8 160.5,109.1 196.7,94.4 232.9,95.6 269.1,88.3 305.3,90.8 341.5,76.3 377.6,70.1 413.8,64.0 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,117.7 124.4,119.1 160.5,106.9 196.7,94.4 232.9,92.5 269.1,84.0 305.3,76.8 341.5,69.5 377.6,64.2 413.8,56.3 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,122.4 124.4,111.8 160.5,109.6 196.7,92.7 232.9,87.5 269.1,79.9 305.3,73.1 341.5,66.2 377.6,60.3 413.8,48.0 450.0,31.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 168 ns | 211 ns | 198 ns | 198 ns |
| D38 | 4.36 ns | 133 ns | 139 ns | 196 ns | 178 ns |
| D57 | 271 ns | 250 ns | 246 ns | 252 ns | 353 ns |
| D76 | 276 ns | 233 ns | 442 ns | 496 ns | 404 ns |
| D115 | 712 ns | 764 ns | 964 ns | 1.02 µs | 1.07 µs |
| D153 | 717 ns | 848 ns | 958 ns | 1.06 µs | 1.42 µs |
| D230 | 855 ns | 1.16 µs | 1.42 µs | 1.83 µs | 2.17 µs |
| D307 | 1.44 µs | 1.06 µs | 1.27 µs | 2.74 µs | 3.23 µs |
| D462 | 1.52 µs | 1.86 µs | 2.78 µs | 3.85 µs | 4.6 µs |
| D616 | 1.43 µs | 2.39 µs | 3.88 µs | 5.2 µs | 6.18 µs |
| D924 | 2.34 µs | 3.87 µs | 5.38 µs | 7.95 µs | 12.4 µs |
| D1232 | 3.43 µs | 5.24 µs | 8.2 µs | 10.5 µs | 30.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,184.4 124.4,112.7 160.5,112.4 196.7,95.9 232.9,95.8 269.1,92.7 305.3,83.6 341.5,82.7 377.6,83.8 413.8,75.2 450.0,68.6 450.0,30.8 413.8,46.3 377.6,58.4 341.5,63.5 305.3,69.6 269.1,76.5 232.9,83.9 196.7,88.8 160.5,105.8 124.4,108.1 88.2,120.0 52.0,118.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,184.4 124.4,112.7 160.5,112.4 196.7,95.9 232.9,95.8 269.1,92.7 305.3,83.6 341.5,82.7 377.6,83.8 413.8,75.2 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.0 88.2,125.1 124.4,114.1 160.5,115.3 196.7,94.7 232.9,92.9 269.1,87.4 305.3,89.0 341.5,79.2 377.6,74.9 413.8,66.5 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,117.0 88.2,124.2 124.4,114.4 160.5,104.2 196.7,90.6 232.9,90.7 269.1,83.9 305.3,85.8 341.5,72.3 377.6,66.4 413.8,60.8 450.0,53.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,118.3 124.4,113.9 160.5,102.2 196.7,89.6 232.9,89.0 269.1,79.5 305.3,72.5 341.5,66.6 377.6,61.4 413.8,54.0 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,120.0 124.4,108.1 160.5,105.8 196.7,88.8 232.9,83.9 269.1,76.5 305.3,69.6 341.5,63.5 377.6,58.4 413.8,46.3 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
