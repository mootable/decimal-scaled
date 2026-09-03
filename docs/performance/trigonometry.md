# Performance — Trigonometry

Speed of the trigonometric and hyperbolic functions by storage width and scale. See
the [Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:trig -->
### `acos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.14 ns | 2.13 µs | 2.78 µs | 3.77 µs | 3.77 µs |
| D38 | 1.41 µs | 3.38 µs | 4.03 µs | 5.74 µs | 9.08 µs |
| D57 | 1.56 µs | 5.75 µs | 8.05 µs | 6.11 µs | 14 µs |
| D76 | 1.57 µs | 5.53 µs | 10.5 µs | 15.3 µs | 18.6 µs |
| D115 | 1.47 µs | 8.2 µs | 20.1 µs | 33 µs | 42.8 µs |
| D153 | 1.5 µs | 9.37 µs | 25.7 µs | 42.1 µs | 54 µs |
| D230 | 1.65 µs | 16.6 µs | 45.2 µs | 75.2 µs | 122 µs |
| D307 | 914 ns | 15 µs | 66 µs | 103 µs | 170 µs |
| D462 | 1.57 µs | 45.7 µs | 126 µs | 243 µs | 407 µs |
| D616 | 1.65 µs | 54.6 µs | 210 µs | 305 µs | 706 µs |
| D924 | 1.03 µs | 135 µs | 423 µs | 872 µs | 1.17 ms |
| D1232 | 1.61 µs | 210 µs | 615 µs | 1.53 ms | 3.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.0 88.2,120.0 124.4,118.7 160.5,118.7 196.7,119.5 232.9,119.2 269.1,118.1 305.3,125.4 341.5,118.7 377.6,118.1 413.8,123.9 450.0,118.3 450.0,23.0 413.8,36.6 377.6,42.9 341.5,49.7 305.3,60.6 269.1,64.7 232.9,74.8 196.7,77.7 160.5,88.0 124.4,91.5 88.2,96.9 52.0,107.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.0 88.2,120.0 124.4,118.7 160.5,118.7 196.7,119.5 232.9,119.2 269.1,118.1 305.3,125.4 341.5,118.7 377.6,118.1 413.8,123.9 450.0,118.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.9 88.2,109.2 124.4,102.6 160.5,103.1 196.7,98.2 232.9,96.5 269.1,89.5 305.3,90.7 341.5,76.9 377.6,74.7 413.8,63.4 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,107.0 124.4,98.4 160.5,95.1 196.7,87.1 232.9,84.0 269.1,77.0 305.3,72.3 341.5,64.3 377.6,57.9 413.8,49.3 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,102.6 124.4,101.8 160.5,90.5 196.7,80.9 232.9,77.9 269.1,70.7 305.3,66.7 341.5,56.1 377.6,53.3 413.8,40.3 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.8 88.2,96.9 124.4,91.5 160.5,88.0 196.7,77.7 232.9,74.8 269.1,64.7 305.3,60.6 341.5,49.7 377.6,42.9 413.8,36.6 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `acosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.22 µs | 3.78 µs | 5.3 µs | 6.69 µs | 6.85 µs |
| D38 | 3.39 µs | 4.56 µs | 7.09 µs | 6.71 µs | 9.36 µs |
| D57 | 3.77 µs | 4.34 µs | 4.93 µs | 3.54 µs | 7.38 µs |
| D76 | 3.73 µs | 3.66 µs | 6.22 µs | 8.38 µs | 9.37 µs |
| D115 | 6.19 µs | 8.59 µs | 12.2 µs | 16.5 µs | 21.7 µs |
| D153 | 6.16 µs | 8.88 µs | 15.6 µs | 21 µs | 26.7 µs |
| D230 | 9.03 µs | 13.6 µs | 28.4 µs | 46 µs | 69.2 µs |
| D307 | 8.16 µs | 14.8 µs | 51.6 µs | 66.2 µs | 123 µs |
| D462 | 12.9 µs | 38.9 µs | 86.1 µs | 161 µs | 276 µs |
| D616 | 22.8 µs | 58.8 µs | 172 µs | 213 µs | 563 µs |
| D924 | 20.1 µs | 160 µs | 382 µs | 755 µs | 1.02 ms |
| D1232 | 36.1 µs | 266 µs | 631 µs | 1.46 ms | 3.09 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.6 88.2,183.5 124.4,181.2 160.5,181.4 196.7,170.4 232.9,170.5 269.1,162.2 305.3,164.4 341.5,154.5 377.6,142.1 413.8,144.8 450.0,132.1 450.0,35.5 413.8,59.6 377.6,72.5 341.5,88.0 305.3,105.5 269.1,118.0 232.9,138.7 196.7,143.2 160.5,161.4 124.4,166.6 88.2,161.4 52.0,168.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.6 88.2,183.5 124.4,181.2 160.5,181.4 196.7,170.4 232.9,170.5 269.1,162.2 305.3,164.4 341.5,154.5 377.6,142.1 413.8,144.8 450.0,132.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.1 88.2,177.0 124.4,178.1 160.5,181.8 196.7,163.3 232.9,162.6 269.1,153.3 305.3,151.5 341.5,130.5 377.6,121.5 413.8,99.8 450.0,88.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,173.8 88.2,167.5 124.4,175.4 160.5,170.3 196.7,155.6 232.9,150.3 269.1,137.4 305.3,124.4 341.5,113.2 377.6,98.2 413.8,80.9 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.7 88.2,168.7 124.4,182.6 160.5,163.8 196.7,149.1 232.9,143.9 269.1,126.9 305.3,119.0 341.5,99.6 377.6,93.5 413.8,66.1 450.0,51.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,168.2 88.2,161.4 124.4,166.6 160.5,161.4 196.7,143.2 232.9,138.7 269.1,118.0 305.3,105.5 341.5,88.0 377.6,72.5 413.8,59.6 450.0,35.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.11 µs | 2.87 µs | 3.75 µs | 3.76 µs |
| D38 | 1.32 µs | 3.38 µs | 3.96 µs | 5.74 µs | 9.07 µs |
| D57 | 1.45 µs | 5.74 µs | 8.01 µs | 6.08 µs | 14 µs |
| D76 | 1.47 µs | 5.45 µs | 10.5 µs | 15.2 µs | 18.5 µs |
| D115 | 1.37 µs | 8.1 µs | 19.8 µs | 32.8 µs | 42.8 µs |
| D153 | 1.36 µs | 9.31 µs | 25.5 µs | 43 µs | 53.9 µs |
| D230 | 1.54 µs | 16.8 µs | 46 µs | 74.8 µs | 122 µs |
| D307 | 837 ns | 15 µs | 65.3 µs | 102 µs | 169 µs |
| D462 | 1.4 µs | 46.8 µs | 125 µs | 242 µs | 405 µs |
| D616 | 1.56 µs | 53.9 µs | 207 µs | 302 µs | 710 µs |
| D924 | 975 ns | 133 µs | 423 µs | 874 µs | 1.17 ms |
| D1232 | 1.58 µs | 208 µs | 615 µs | 1.53 ms | 3.51 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,120.9 124.4,119.7 160.5,119.5 196.7,120.4 232.9,120.5 269.1,119.0 305.3,126.5 341.5,120.1 377.6,118.7 413.8,124.6 450.0,118.6 450.0,23.0 413.8,36.6 377.6,42.8 341.5,49.8 305.3,60.6 269.1,64.7 232.9,74.8 196.7,77.7 160.5,88.1 124.4,91.6 88.2,96.9 52.0,107.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,120.9 124.4,119.7 160.5,119.5 196.7,120.4 232.9,120.5 269.1,119.0 305.3,126.5 341.5,120.1 377.6,118.7 413.8,124.6 450.0,118.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.0 88.2,109.2 124.4,102.6 160.5,103.2 196.7,98.3 232.9,96.6 269.1,89.3 305.3,90.7 341.5,76.6 377.6,74.8 413.8,63.6 450.0,58.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.2 124.4,98.5 160.5,95.1 196.7,87.2 232.9,84.1 269.1,76.8 305.3,72.4 341.5,64.3 377.6,58.1 413.8,49.2 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.6 124.4,101.9 160.5,90.5 196.7,81.0 232.9,77.6 269.1,70.7 305.3,66.9 341.5,56.2 377.6,53.4 413.8,40.2 450.0,33.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,96.9 124.4,91.6 160.5,88.1 196.7,77.7 232.9,74.8 269.1,64.7 305.3,60.6 341.5,49.8 377.6,42.8 413.8,36.6 450.0,23.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `asinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.28 µs | 2.96 µs | 2.97 µs |
| D38 | 4.22 ns | 1.99 µs | 3.08 µs | 2.98 µs | 4.16 µs |
| D57 | 2.11 ns | 5.36 µs | 7.26 µs | 5.29 µs | 11.5 µs |
| D76 | 2.54 ns | 5.02 µs | 9.24 µs | 12.8 µs | 14 µs |
| D115 | 14 ns | 12.6 µs | 19.2 µs | 24.7 µs | 33.7 µs |
| D153 | 16.5 ns | 13.8 µs | 23.9 µs | 33.1 µs | 39.1 µs |
| D230 | 31.5 ns | 20.9 µs | 44.9 µs | 67 µs | 94.5 µs |
| D307 | 31 ns | 21.4 µs | 72.5 µs | 88.1 µs | 151 µs |
| D462 | 69.6 ns | 58.6 µs | 120 µs | 198 µs | 337 µs |
| D616 | 87.5 ns | 84.6 µs | 226 µs | 292 µs | 668 µs |
| D924 | 71 ns | 241 µs | 497 µs | 871 µs | 1.08 ms |
| D1232 | 143 ns | 367 µs | 765 µs | 1.63 ms | 3.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,200.7 160.5,198.4 196.7,177.2 232.9,175.2 269.1,167.2 305.3,167.4 341.5,157.4 377.6,154.5 413.8,157.1 450.0,148.4 450.0,24.4 413.8,37.6 377.6,43.6 341.5,52.1 305.3,62.0 269.1,67.8 232.9,78.8 196.7,80.6 160.5,91.6 124.4,94.0 88.2,106.6 52.0,110.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,200.7 160.5,198.4 196.7,177.2 232.9,175.2 269.1,167.2 305.3,167.4 341.5,157.4 377.6,154.5 413.8,157.1 450.0,148.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.9 88.2,115.8 124.4,103.5 160.5,104.3 196.7,92.8 232.9,91.7 269.1,86.5 305.3,86.3 341.5,73.8 377.6,69.2 413.8,56.2 450.0,51.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,114.1 88.2,110.3 124.4,99.7 160.5,96.7 196.7,87.6 232.9,84.9 269.1,77.1 305.3,71.1 341.5,64.9 377.6,57.0 413.8,47.2 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,110.7 124.4,103.6 160.5,92.6 196.7,84.5 232.9,80.9 269.1,72.1 305.3,68.7 341.5,58.7 377.6,53.8 413.8,40.3 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,106.6 124.4,94.0 160.5,91.6 196.7,80.6 232.9,78.8 269.1,67.8 305.3,62.0 341.5,52.1 377.6,43.6 413.8,37.6 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 1.64 µs | 2.33 µs | 3.18 µs | 3.22 µs |
| D38 | 723 ns | 2.94 µs | 2.86 µs | 4.48 µs | 7.17 µs |
| D57 | 577 ns | 4.47 µs | 6.24 µs | 4.79 µs | 5.23 µs |
| D76 | 606 ns | 4.48 µs | 8.22 µs | 12.6 µs | 15.5 µs |
| D115 | 533 ns | 6.21 µs | 16.6 µs | 29.4 µs | 39.2 µs |
| D153 | 555 ns | 7.29 µs | 16.9 µs | 37.7 µs | 49.3 µs |
| D230 | 645 ns | 13.8 µs | 40.4 µs | 69.3 µs | 113 µs |
| D307 | 344 ns | 12.7 µs | 54.9 µs | 93 µs | 158 µs |
| D462 | 605 ns | 37 µs | 107 µs | 212 µs | 341 µs |
| D616 | 725 ns | 50.5 µs | 192 µs | 282 µs | 671 µs |
| D924 | 460 ns | 124 µs | 396 µs | 827 µs | 1.11 ms |
| D1232 | 825 ns | 192 µs | 583 µs | 1.47 ms | 3.36 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,128.3 124.4,131.1 160.5,130.5 196.7,132.1 232.9,131.6 269.1,129.7 305.3,137.5 341.5,130.5 377.6,128.3 413.8,133.9 450.0,126.7 450.0,23.5 413.8,37.2 377.6,43.5 341.5,51.9 305.3,61.5 269.1,65.6 232.9,75.9 196.7,78.8 160.5,90.2 124.4,103.8 88.2,99.8 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,128.3 124.4,131.1 160.5,130.5 196.7,132.1 232.9,131.6 269.1,129.7 305.3,137.5 341.5,130.5 377.6,128.3 413.8,133.9 450.0,126.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.1 88.2,110.9 124.4,105.7 160.5,105.7 196.7,101.6 232.9,99.6 269.1,91.7 305.3,92.8 341.5,79.5 377.6,75.6 413.8,64.4 450.0,59.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.8 88.2,111.2 124.4,101.6 160.5,98.2 196.7,89.4 232.9,89.2 269.1,78.4 305.3,74.6 341.5,66.4 377.6,59.1 413.8,50.1 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,105.7 124.4,104.9 160.5,92.9 196.7,82.3 232.9,79.3 269.1,71.7 305.3,68.0 341.5,57.8 377.6,54.3 413.8,40.9 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,99.8 124.4,103.8 160.5,90.2 196.7,78.8 232.9,75.9 269.1,65.6 305.3,61.5 341.5,51.9 377.6,43.5 413.8,37.2 450.0,23.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `atanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.82 µs | 2.59 µs | 3.31 µs | 3.44 µs |
| D38 | 4.22 ns | 2.23 µs | 3.57 µs | 3.35 µs | 4.71 µs |
| D57 | 538 ns | 5.7 µs | 6.9 µs | 5.04 µs | 11.1 µs |
| D76 | 526 ns | 4.92 µs | 8.72 µs | 12 µs | 14.2 µs |
| D115 | 901 ns | 11.6 µs | 17.7 µs | 25.4 µs | 33.7 µs |
| D153 | 909 ns | 12.2 µs | 23.5 µs | 32.6 µs | 43 µs |
| D230 | 1.38 µs | 19.8 µs | 44.1 µs | 76.2 µs | 118 µs |
| D307 | 1.27 µs | 22.4 µs | 85.3 µs | 115 µs | 219 µs |
| D462 | 1.95 µs | 60.6 µs | 149 µs | 290 µs | 501 µs |
| D616 | 3.57 µs | 96.1 µs | 301 µs | 394 µs | 1.03 ms |
| D924 | 3.03 µs | 269 µs | 680 µs | 1.39 ms | 1.9 ms |
| D1232 | 5.56 µs | 457 µs | 1.15 ms | 2.73 ms | 5.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,132.0 160.5,132.3 196.7,125.6 232.9,125.5 269.1,120.3 305.3,121.3 341.5,116.0 377.6,108.5 413.8,110.5 450.0,103.0 450.0,16.7 413.8,30.6 377.6,38.2 341.5,47.2 305.3,57.4 269.1,65.1 232.9,77.6 196.7,80.6 160.5,91.4 124.4,94.5 88.2,105.1 52.0,109.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,132.0 160.5,132.3 196.7,125.6 232.9,125.5 269.1,120.3 305.3,121.3 341.5,116.0 377.6,108.5 413.8,110.5 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,114.3 124.4,102.7 160.5,104.5 196.7,93.8 232.9,93.2 269.1,87.2 305.3,85.7 341.5,73.4 377.6,67.6 413.8,54.9 450.0,48.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.5 88.2,108.5 124.4,100.3 160.5,97.4 196.7,88.6 232.9,85.1 269.1,77.3 305.3,69.1 341.5,62.2 377.6,53.5 413.8,43.4 450.0,36.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.4 88.2,109.3 124.4,104.2 160.5,93.4 196.7,84.2 232.9,81.1 269.1,70.5 305.3,65.4 341.5,53.9 377.6,50.1 413.8,34.5 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,105.1 124.4,94.5 160.5,91.4 196.7,80.6 232.9,77.6 269.1,65.1 305.3,57.4 341.5,47.2 377.6,38.2 413.8,30.6 450.0,16.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cos`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.98 ns | 1.25 µs | 1.97 µs | 2.82 µs | 2.89 µs |
| D38 | 5.62 ns | 1.76 µs | 3.01 µs | 2.43 µs | 3.08 µs |
| D57 | 2.81 ns | 3.6 µs | 4.63 µs | 3.16 µs | 9.01 µs |
| D76 | 3.17 ns | 3.19 µs | 5.87 µs | 7.81 µs | 9.05 µs |
| D115 | 11.6 ns | 4.64 µs | 9.65 µs | 13.9 µs | 19 µs |
| D153 | 16.9 ns | 4.98 µs | 9.97 µs | 18.2 µs | 25.8 µs |
| D230 | 40.7 ns | 8.38 µs | 19.6 µs | 40 µs | 68.9 µs |
| D307 | 43.7 ns | 7.32 µs | 27.4 µs | 53.3 µs | 103 µs |
| D462 | 104 ns | 16 µs | 62.5 µs | 135 µs | 233 µs |
| D616 | 144 ns | 25.8 µs | 123 µs | 183 µs | 460 µs |
| D924 | 99.9 ns | 73.7 µs | 256 µs | 575 µs | 824 µs |
| D1232 | 288 ns | 123 µs | 413 µs | 1.06 ms | 2.47 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,188.6 124.4,197.2 160.5,195.7 196.7,179.6 232.9,175.0 269.1,164.0 305.3,163.1 341.5,152.3 377.6,148.3 413.8,152.9 450.0,139.7 450.0,27.4 413.8,41.0 377.6,48.2 341.5,56.7 305.3,66.8 269.1,71.8 232.9,84.0 196.7,87.8 160.5,97.0 124.4,97.0 88.2,110.3 52.0,111.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,188.6 124.4,197.2 160.5,195.7 196.7,179.6 232.9,175.0 269.1,164.0 305.3,163.1 341.5,152.3 377.6,148.3 413.8,152.9 450.0,139.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.5 88.2,117.3 124.4,108.4 160.5,109.9 196.7,105.2 232.9,104.4 269.1,97.9 305.3,99.6 341.5,89.9 377.6,84.0 413.8,70.9 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.9 88.2,110.6 124.4,105.3 160.5,102.3 196.7,96.2 232.9,95.7 269.1,87.4 305.3,83.2 341.5,73.0 377.6,64.6 413.8,55.5 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.4 88.2,113.3 124.4,110.0 160.5,98.8 196.7,91.7 232.9,88.3 269.1,78.5 305.3,74.9 341.5,63.4 377.6,59.6 413.8,45.4 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.1 88.2,110.3 124.4,97.0 160.5,97.0 196.7,87.8 232.9,84.0 269.1,71.8 305.3,66.8 341.5,56.7 377.6,48.2 413.8,41.0 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `cosh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.73 ns | 1.9 µs | 2.77 µs | 3.88 µs | 3.84 µs |
| D38 | 4.57 ns | 2.62 µs | 3.91 µs | 3.21 µs | 4.17 µs |
| D57 | 3.16 ns | 5.4 µs | 7.11 µs | 4.76 µs | 10.3 µs |
| D76 | 3.52 ns | 4.95 µs | 8.13 µs | 11.1 µs | 12.1 µs |
| D115 | 10.3 ns | 12.3 µs | 12.4 µs | 21.7 µs | 26.3 µs |
| D153 | 16.5 ns | 7.25 µs | 16.5 µs | 23.1 µs | 31.4 µs |
| D230 | 40.8 ns | 11.9 µs | 24.6 µs | 47.2 µs | 79.8 µs |
| D307 | 43.4 ns | 9.96 µs | 52.1 µs | 64.9 µs | 113 µs |
| D462 | 105 ns | 25.4 µs | 82.5 µs | 158 µs | 251 µs |
| D616 | 137 ns | 31.8 µs | 135 µs | 198 µs | 450 µs |
| D924 | 112 ns | 87.2 µs | 270 µs | 562 µs | 720 µs |
| D1232 | 278 ns | 135 µs | 410 µs | 925 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,193.7 88.2,191.1 124.4,195.7 160.5,194.4 196.7,181.1 232.9,175.2 269.1,164.0 305.3,163.2 341.5,152.2 377.6,149.0 413.8,151.5 450.0,140.2 450.0,25.6 413.8,42.6 377.6,48.5 341.5,55.7 305.3,65.6 269.1,69.9 232.9,81.5 196.7,83.7 160.5,93.4 124.4,95.4 88.2,106.6 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,193.7 88.2,191.1 124.4,195.7 160.5,194.4 196.7,181.1 232.9,175.2 269.1,164.0 305.3,163.2 341.5,152.2 377.6,149.0 413.8,151.5 450.0,140.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,112.3 124.4,103.4 160.5,104.4 196.7,93.2 232.9,99.7 269.1,93.6 305.3,95.8 341.5,84.2 377.6,81.4 413.8,68.8 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,107.4 124.4,100.0 160.5,98.3 196.7,93.1 232.9,89.5 269.1,84.6 305.3,75.2 341.5,69.5 377.6,63.5 413.8,54.8 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,109.8 124.4,104.9 160.5,94.4 196.7,86.1 232.9,85.3 269.1,76.5 305.3,72.5 341.5,61.5 377.6,58.7 413.8,45.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,106.6 124.4,95.4 160.5,93.4 196.7,83.7 232.9,81.5 269.1,69.9 305.3,65.6 341.5,55.7 377.6,48.5 413.8,42.6 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sin`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 1.13 µs | 1.82 µs | 2.78 µs | 2.87 µs |
| D38 | 4.92 ns | 1.64 µs | 2.84 µs | 2.3 µs | 3.04 µs |
| D57 | 2.81 ns | 3.23 µs | 4.43 µs | 3.11 µs | 8.97 µs |
| D76 | 3.52 ns | 2.98 µs | 5.6 µs | 7.78 µs | 8.78 µs |
| D115 | 11.6 ns | 4.46 µs | 10.1 µs | 14.4 µs | 18.7 µs |
| D153 | 17.1 ns | 4.78 µs | 9.96 µs | 18.1 µs | 25.5 µs |
| D230 | 40.7 ns | 8.7 µs | 19.6 µs | 39.6 µs | 66.1 µs |
| D307 | 38.7 ns | 7.67 µs | 26.4 µs | 53.3 µs | 102 µs |
| D462 | 98.7 ns | 16.1 µs | 59.5 µs | 136 µs | 230 µs |
| D616 | 126 ns | 25 µs | 121 µs | 181 µs | 457 µs |
| D924 | 100 ns | 73 µs | 252 µs | 572 µs | 819 µs |
| D1232 | 278 ns | 123 µs | 412 µs | 1.05 ms | 2.45 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.7 88.2,190.2 124.4,197.2 160.5,194.4 196.7,179.6 232.9,174.8 269.1,164.0 305.3,164.6 341.5,153.0 377.6,150.0 413.8,152.8 450.0,140.2 450.0,27.4 413.8,41.0 377.6,48.3 341.5,56.8 305.3,66.9 269.1,72.3 232.9,84.1 196.7,87.9 160.5,97.3 124.4,97.1 88.2,110.5 52.0,111.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.7 88.2,190.2 124.4,197.2 160.5,194.4 196.7,179.6 232.9,174.8 269.1,164.0 305.3,164.6 341.5,153.0 377.6,150.0 413.8,152.8 450.0,140.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,122.8 88.2,118.1 124.4,109.7 160.5,110.8 196.7,105.7 232.9,104.9 269.1,97.4 305.3,99.0 341.5,89.8 377.6,84.3 413.8,71.0 450.0,64.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.9 88.2,111.3 124.4,105.8 160.5,102.9 196.7,95.6 232.9,95.8 269.1,87.4 305.3,83.7 341.5,73.6 377.6,64.8 413.8,55.7 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,113.9 124.4,110.2 160.5,98.8 196.7,91.2 232.9,88.4 269.1,78.6 305.3,75.0 341.5,63.4 377.6,59.8 413.8,45.5 450.0,37.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,110.5 124.4,97.1 160.5,97.3 196.7,87.9 232.9,84.1 269.1,72.3 305.3,66.9 341.5,56.8 377.6,48.3 413.8,41.0 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sinh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.91 µs | 2.79 µs | 3.89 µs | 3.84 µs |
| D38 | 4.57 ns | 2.47 µs | 3.92 µs | 3.22 µs | 4.17 µs |
| D57 | 12.3 ns | 5.44 µs | 7.11 µs | 4.89 µs | 10.3 µs |
| D76 | 12.1 ns | 4.91 µs | 8.14 µs | 11.1 µs | 12.1 µs |
| D115 | 10.6 ns | 12.2 µs | 11.9 µs | 21.8 µs | 26.5 µs |
| D153 | 15.9 ns | 7.3 µs | 16.2 µs | 23.1 µs | 31.4 µs |
| D230 | 39.9 ns | 12 µs | 24.6 µs | 47.3 µs | 80.7 µs |
| D307 | 38.4 ns | 9.73 µs | 52 µs | 64.8 µs | 113 µs |
| D462 | 101 ns | 25.2 µs | 82.7 µs | 159 µs | 251 µs |
| D616 | 135 ns | 31.1 µs | 135 µs | 195 µs | 450 µs |
| D924 | 169 ns | 88.3 µs | 271 µs | 562 µs | 720 µs |
| D1232 | 293 ns | 136 µs | 410 µs | 925 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,191.1 124.4,178.9 160.5,179.1 196.7,180.7 232.9,175.7 269.1,164.2 305.3,164.7 341.5,152.8 377.6,149.2 413.8,146.3 450.0,139.5 450.0,25.6 413.8,42.7 377.6,48.5 341.5,55.7 305.3,65.7 269.1,69.8 232.9,81.5 196.7,83.6 160.5,93.3 124.4,95.3 88.2,106.6 52.0,107.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,191.1 124.4,178.9 160.5,179.1 196.7,180.7 232.9,175.7 269.1,164.2 305.3,164.7 341.5,152.8 377.6,149.2 413.8,146.3 450.0,139.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.3 88.2,113.1 124.4,103.3 160.5,104.5 196.7,93.3 232.9,99.6 269.1,93.5 305.3,96.1 341.5,84.2 377.6,81.7 413.8,68.7 450.0,63.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.6 88.2,107.3 124.4,99.9 160.5,98.3 196.7,93.5 232.9,89.7 269.1,84.5 305.3,75.3 341.5,69.5 377.6,63.4 413.8,54.8 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,109.8 124.4,104.6 160.5,94.4 196.7,86.0 232.9,85.3 269.1,76.4 305.3,72.5 341.5,61.4 377.6,58.8 413.8,45.7 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,106.6 124.4,95.3 160.5,93.3 196.7,83.6 232.9,81.5 269.1,69.8 305.3,65.7 341.5,55.7 377.6,48.5 413.8,42.7 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tan`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 2.29 µs | 3.39 µs | 4.57 µs | 4.65 µs |
| D38 | 4.92 ns | 3.01 µs | 4.78 µs | 4.22 µs | 5.4 µs |
| D57 | 2.81 ns | 4.35 µs | 5.98 µs | 4.12 µs | 9.21 µs |
| D76 | 4.22 ns | 3.96 µs | 7.44 µs | 9.99 µs | 11.1 µs |
| D115 | 10.9 ns | 5.83 µs | 12.5 µs | 17 µs | 22.4 µs |
| D153 | 16.4 ns | 6.36 µs | 13.2 µs | 21.7 µs | 29.8 µs |
| D230 | 41.1 ns | 10 µs | 23.6 µs | 44.7 µs | 74.9 µs |
| D307 | 38.3 ns | 9.08 µs | 31.5 µs | 60.4 µs | 112 µs |
| D462 | 95.4 ns | 19.8 µs | 68.5 µs | 148 µs | 250 µs |
| D616 | 135 ns | 29.9 µs | 134 µs | 206 µs | 494 µs |
| D924 | 89 ns | 81.5 µs | 276 µs | 616 µs | 867 µs |
| D1232 | 297 ns | 134 µs | 441 µs | 1.13 ms | 2.6 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.9 88.2,190.2 124.4,197.2 160.5,192.1 196.7,180.4 232.9,175.3 269.1,163.9 305.3,164.8 341.5,153.4 377.6,149.1 413.8,154.3 450.0,139.4 450.0,26.7 413.8,40.3 377.6,47.3 341.5,55.8 305.3,65.7 269.1,70.7 232.9,82.2 196.7,85.7 160.5,94.4 124.4,96.7 88.2,103.4 52.0,105.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.9 88.2,190.2 124.4,197.2 160.5,192.1 196.7,180.4 232.9,175.3 269.1,163.9 305.3,164.8 341.5,153.4 377.6,149.1 413.8,154.3 450.0,139.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.0 88.2,110.6 124.4,106.0 160.5,107.2 196.7,102.4 232.9,101.3 269.1,95.7 305.3,96.9 341.5,87.3 377.6,82.1 413.8,69.7 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.1 88.2,104.9 124.4,102.1 160.5,99.4 196.7,92.9 232.9,92.3 269.1,85.0 305.3,81.5 341.5,71.8 377.6,63.5 413.8,54.5 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.4 88.2,106.4 124.4,106.7 160.5,95.7 196.7,89.1 232.9,86.1 269.1,77.1 305.3,73.4 341.5,62.2 377.6,58.2 413.8,44.6 450.0,37.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,103.4 124.4,96.7 160.5,94.4 196.7,85.7 232.9,82.2 269.1,70.7 305.3,65.7 341.5,55.8 377.6,47.3 413.8,40.3 450.0,26.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `tanh`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.95 µs | 3.03 µs | 4.1 µs | 4.01 µs |
| D38 | 4.22 ns | 2.59 µs | 3.95 µs | 3.22 µs | 4.18 µs |
| D57 | 2.79 µs | 5.65 µs | 7.47 µs | 4.93 µs | 10.8 µs |
| D76 | 2.83 µs | 5.13 µs | 8.48 µs | 11.5 µs | 12.6 µs |
| D115 | 5.32 µs | 12.8 µs | 12.7 µs | 22.6 µs | 27.4 µs |
| D153 | 2.77 µs | 7.62 µs | 17.4 µs | 24.5 µs | 32.3 µs |
| D230 | 3.1 µs | 12.8 µs | 25.5 µs | 48.6 µs | 81.4 µs |
| D307 | 1.84 µs | 10.2 µs | 54.1 µs | 64 µs | 114 µs |
| D462 | 3.16 µs | 25.9 µs | 83.9 µs | 160 µs | 256 µs |
| D616 | 3.49 µs | 32.5 µs | 138 µs | 204 µs | 454 µs |
| D924 | 2.23 µs | 90 µs | 274 µs | 568 µs | 727 µs |
| D1232 | 3.49 µs | 138 µs | 414 µs | 930 µs | 2.85 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,192.1 124.4,111.6 160.5,111.4 196.7,103.5 232.9,111.7 269.1,110.2 305.3,116.7 341.5,110.0 377.6,108.8 413.8,114.3 450.0,108.8 450.0,25.6 413.8,42.5 377.6,48.4 341.5,55.5 305.3,65.5 269.1,69.7 232.9,81.2 196.7,83.2 160.5,92.8 124.4,94.8 88.2,106.5 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,192.1 124.4,111.6 160.5,111.4 196.7,103.5 232.9,111.7 269.1,110.2 305.3,116.7 341.5,110.0 377.6,108.8 413.8,114.3 450.0,108.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.0 88.2,112.5 124.4,102.8 160.5,104.0 196.7,92.6 232.9,99.1 269.1,92.7 305.3,95.5 341.5,83.9 377.6,81.1 413.8,68.5 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.5 88.2,107.2 124.4,99.3 160.5,97.8 196.7,92.8 232.9,88.8 269.1,84.1 305.3,74.8 341.5,69.3 377.6,63.2 413.8,54.6 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,109.8 124.4,104.5 160.5,94.0 196.7,85.6 232.9,84.6 269.1,76.1 305.3,72.7 341.5,61.3 377.6,58.3 413.8,45.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,106.5 124.4,94.8 160.5,92.8 196.7,83.2 232.9,81.2 269.1,69.7 305.3,65.5 341.5,55.5 377.6,48.4 413.8,42.5 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_degrees`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 140 ns | 136 ns | 172 ns | 163 ns |
| D38 | 4.92 ns | 115 ns | 162 ns | 135 ns | 184 ns |
| D57 | 197 ns | 307 ns | 320 ns | 186 ns | 409 ns |
| D76 | 201 ns | 231 ns | 353 ns | 443 ns | 432 ns |
| D115 | 404 ns | 570 ns | 696 ns | 770 ns | 851 ns |
| D153 | 406 ns | 494 ns | 817 ns | 863 ns | 883 ns |
| D230 | 597 ns | 711 ns | 1.08 µs | 1.42 µs | 1.81 µs |
| D307 | 490 ns | 670 ns | 1.66 µs | 1.64 µs | 2.55 µs |
| D462 | 889 ns | 1.6 µs | 2.17 µs | 3.03 µs | 4.2 µs |
| D616 | 1.19 µs | 1.53 µs | 2.88 µs | 3 µs | 6.05 µs |
| D924 | 926 ns | 2.95 µs | 4.5 µs | 7.02 µs | 8.2 µs |
| D1232 | 1.87 µs | 4.1 µs | 6.54 µs | 11.1 µs | 30.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,182.3 124.4,118.2 160.5,117.8 196.7,105.8 232.9,105.7 269.1,99.0 305.3,102.4 341.5,92.0 377.6,87.0 413.8,91.3 450.0,79.1 450.0,30.8 413.8,53.5 377.6,58.7 341.5,65.1 305.3,73.7 269.1,79.7 232.9,92.2 196.7,92.8 160.5,104.6 124.4,105.5 88.2,119.4 52.0,121.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,182.3 124.4,118.2 160.5,117.8 196.7,105.8 232.9,105.7 269.1,99.0 305.3,102.4 341.5,92.0 377.6,87.0 413.8,91.3 450.0,79.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.1 88.2,127.6 124.4,110.5 160.5,115.5 196.7,99.8 232.9,102.3 269.1,95.9 305.3,97.0 341.5,81.8 377.6,82.6 413.8,71.2 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.7 88.2,121.6 124.4,109.8 160.5,108.1 196.7,96.3 232.9,93.5 269.1,88.6 305.3,81.2 341.5,76.5 377.6,71.6 413.8,63.9 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.6 88.2,124.8 124.4,119.2 160.5,104.1 196.7,94.5 232.9,92.6 269.1,83.9 305.3,81.4 341.5,70.7 377.6,70.9 413.8,56.1 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.6 88.2,119.4 124.4,105.5 160.5,104.6 196.7,92.8 232.9,92.2 269.1,79.7 305.3,73.7 341.5,65.1 377.6,58.7 413.8,53.5 450.0,30.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `to_radians`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 168 ns | 172 ns | 216 ns | 198 ns |
| D38 | 4.92 ns | 155 ns | 199 ns | 162 ns | 194 ns |
| D57 | 311 ns | 399 ns | 419 ns | 250 ns | 519 ns |
| D76 | 305 ns | 313 ns | 486 ns | 567 ns | 568 ns |
| D115 | 623 ns | 772 ns | 903 ns | 1.02 µs | 1.06 µs |
| D153 | 602 ns | 680 ns | 1.01 µs | 1.07 µs | 1.08 µs |
| D230 | 934 ns | 951 ns | 1.39 µs | 1.83 µs | 2.15 µs |
| D307 | 822 ns | 934 ns | 2.23 µs | 2.11 µs | 3.08 µs |
| D462 | 1.44 µs | 2.03 µs | 2.76 µs | 3.67 µs | 4.9 µs |
| D616 | 1.87 µs | 1.89 µs | 3.59 µs | 3.52 µs | 6.9 µs |
| D924 | 1.41 µs | 3.91 µs | 5.37 µs | 7.92 µs | 8.88 µs |
| D1232 | 2.9 µs | 5.38 µs | 7.54 µs | 12.6 µs | 31.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,182.3 124.4,110.3 160.5,110.6 196.7,98.2 232.9,98.8 269.1,91.2 305.3,93.4 341.5,83.7 377.6,79.1 413.8,84.1 450.0,71.5 450.0,30.1 413.8,52.1 377.6,56.4 341.5,62.4 305.3,70.5 269.1,76.7 232.9,88.7 196.7,89.0 160.5,99.8 124.4,101.4 88.2,118.5 52.0,118.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,182.3 124.4,110.3 160.5,110.6 196.7,98.2 232.9,98.8 269.1,91.2 305.3,93.4 341.5,83.7 377.6,79.1 413.8,84.1 450.0,71.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.0 88.2,122.4 124.4,106.0 160.5,110.2 196.7,94.5 232.9,96.7 269.1,90.9 305.3,91.2 341.5,77.7 377.6,78.9 413.8,66.3 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.6 88.2,118.0 124.4,105.1 160.5,102.5 196.7,91.8 232.9,89.8 269.1,84.3 305.3,76.1 341.5,72.4 377.6,67.8 413.8,60.8 450.0,54.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,116.7 88.2,121.6 124.4,114.1 160.5,99.9 196.7,89.7 232.9,88.9 269.1,79.5 305.3,77.0 341.5,67.4 377.6,68.1 413.8,54.1 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,118.1 88.2,118.5 124.4,101.4 160.5,99.8 196.7,89.0 232.9,88.7 269.1,76.7 305.3,70.5 341.5,62.4 377.6,56.4 413.8,52.1 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:trig -->
