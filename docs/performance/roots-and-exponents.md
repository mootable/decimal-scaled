# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 77.2 ns | 83.2 ns | 139 ns | 262 ns | 252 ns |
| D38 | 65.6 ns | 158 ns | 305 ns | 291 ns | 410 ns |
| D57 | 191 ns | 424 ns | 648 ns | 1.04 µs | 1.06 µs |
| D76 | 198 ns | 366 ns | 812 ns | 1.3 µs | 1.24 µs |
| D115 | 214 ns | 769 ns | 1.33 µs | 2.05 µs | 3.14 µs |
| D153 | 255 ns | 871 ns | 1.83 µs | 3.11 µs | 4.56 µs |
| D230 | 298 ns | 1.44 µs | 3.12 µs | 5.18 µs | 8.82 µs |
| D307 | 279 ns | 1.48 µs | 4.44 µs | 9.58 µs | 13.7 µs |
| D462 | 390 ns | 3.24 µs | 9.11 µs | 16.4 µs | 33.5 µs |
| D616 | 464 ns | 4.61 µs | 15.6 µs | 34.3 µs | 56.2 µs |
| D924 | 535 ns | 9.15 µs | 34.5 µs | 54.6 µs | 116 µs |
| D1232 | 967 ns | 16 µs | 52.5 µs | 120 µs | 132 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.5 88.2,177.3 124.4,158.8 160.5,158.1 196.7,156.8 232.9,153.8 269.1,151.0 305.3,152.2 341.5,146.4 377.6,143.3 413.8,140.9 450.0,130.6 450.0,45.1 413.8,47.4 377.6,60.0 341.5,69.0 305.3,84.6 269.1,92.2 232.9,103.6 196.7,110.1 160.5,126.3 124.4,128.9 88.2,145.5 52.0,153.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.5 88.2,177.3 124.4,158.8 160.5,158.1 196.7,156.8 232.9,153.8 269.1,151.0 305.3,152.2 341.5,146.4 377.6,143.3 413.8,140.9 450.0,130.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.2 88.2,162.0 124.4,144.9 160.5,147.5 196.7,134.6 232.9,132.4 269.1,123.6 305.3,123.2 341.5,109.6 377.6,103.5 413.8,91.6 450.0,81.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.2 88.2,150.6 124.4,137.5 160.5,133.6 196.7,125.1 232.9,119.5 269.1,110.3 305.3,104.1 341.5,91.6 377.6,82.2 413.8,68.5 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.3 88.2,151.5 124.4,129.2 160.5,125.5 196.7,117.5 232.9,110.3 269.1,101.4 305.3,90.7 341.5,81.4 377.6,68.6 413.8,60.5 450.0,46.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,145.5 124.4,128.9 160.5,126.3 196.7,110.1 232.9,103.6 269.1,92.2 305.3,84.6 341.5,69.0 377.6,60.0 413.8,47.4 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.21 µs | 3.13 µs | 3.76 µs | 3.62 µs |
| D38 | 0.896 ns | 3.5 µs | 4.86 µs | 3.01 µs | 5.17 µs |
| D57 | 2.47 ns | 4.18 µs | 4.75 µs | 6.13 µs | 10.2 µs |
| D76 | 3.16 ns | 3.43 µs | 7.11 µs | 10.2 µs | 8.03 µs |
| D115 | 10.3 ns | 6.66 µs | 12.2 µs | 17.7 µs | 23.6 µs |
| D153 | 18 ns | 7.19 µs | 15.2 µs | 22.1 µs | 38 µs |
| D230 | 46 ns | 14 µs | 22.4 µs | 42.8 µs | 79.5 µs |
| D307 | 50.6 ns | 13.6 µs | 34.8 µs | 85.1 µs | 117 µs |
| D462 | 117 ns | 22 µs | 79.8 µs | 146 µs | 249 µs |
| D616 | 128 ns | 36.1 µs | 141 µs | 286 µs | 444 µs |
| D924 | 195 ns | 80.8 µs | 288 µs | 475 µs | 916 µs |
| D1232 | 360 ns | 140 µs | 411 µs | 915 µs | 1.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">0.1 ns</text><line x1="52" y1="185.0" x2="450" y2="185.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="188.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="135.0" x2="450" y2="135.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="138.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="85.0" x2="450" y2="85.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="88.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="35.0" x2="450" y2="35.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="38.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,172.5 88.2,186.2 124.4,175.2 160.5,172.5 196.7,159.7 232.9,153.6 269.1,143.4 305.3,142.4 341.5,133.3 377.6,132.3 413.8,127.7 450.0,121.1 450.0,29.2 413.8,36.0 377.6,43.8 341.5,50.1 305.3,58.3 269.1,62.5 232.9,70.5 196.7,75.7 160.5,87.4 124.4,84.8 88.2,92.2 52.0,96.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,172.5 88.2,186.2 124.4,175.2 160.5,172.5 196.7,159.7 232.9,153.6 269.1,143.4 305.3,142.4 341.5,133.3 377.6,132.3 413.8,127.7 450.0,121.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.9 88.2,96.4 124.4,94.5 160.5,96.6 196.7,89.4 232.9,88.6 269.1,81.4 305.3,81.7 341.5,76.4 377.6,71.1 413.8,62.3 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,92.8 124.4,93.1 160.5,88.7 196.7,82.8 232.9,80.4 269.1,76.2 305.3,71.5 341.5,62.5 377.6,56.3 413.8,48.5 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,95.6 88.2,98.0 124.4,90.3 160.5,84.8 196.7,78.8 232.9,76.4 269.1,69.2 305.3,61.8 341.5,55.9 377.6,48.6 413.8,43.1 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,96.0 88.2,92.2 124.4,84.8 160.5,87.4 196.7,75.7 232.9,70.5 269.1,62.5 305.3,58.3 341.5,50.1 377.6,43.8 413.8,36.0 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 25.8 ns | 36.7 ns | 40.3 ns | 39.5 ns |
| D38 | 11.4 ns | 42.8 ns | 86.8 ns | 55.2 ns | 108 ns |
| D57 | 16.5 ns | 42.2 ns | 95.7 ns | 398 ns | 414 ns |
| D76 | 17.2 ns | 65 ns | 370 ns | 401 ns | 475 ns |
| D115 | 16.3 ns | 84 ns | 369 ns | 780 ns | 1.01 µs |
| D153 | 23.1 ns | 391 ns | 633 ns | 974 ns | 1.71 µs |
| D230 | 28.9 ns | 448 ns | 1.08 µs | 1.87 µs | 2.79 µs |
| D307 | 21.8 ns | 593 ns | 1.63 µs | 2.98 µs | 4.93 µs |
| D462 | 68.4 ns | 1.14 µs | 3.18 µs | 5.33 µs | 7.84 µs |
| D616 | 71 ns | 2.03 µs | 5.9 µs | 10.9 µs | 13.9 µs |
| D924 | 93.9 ns | 3.27 µs | 11 µs | 19.3 µs | 24.5 µs |
| D1232 | 111 ns | 6.06 µs | 18.7 µs | 23.9 µs | 25.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.1 88.2,207.2 124.4,199.2 160.5,198.2 196.7,199.4 232.9,191.8 269.1,187.0 305.3,193.1 341.5,168.2 377.6,167.4 413.8,161.4 450.0,157.8 450.0,39.8 413.8,40.6 377.6,52.9 341.5,65.3 305.3,75.3 269.1,87.7 232.9,98.4 196.7,109.8 160.5,126.1 124.4,129.2 88.2,158.4 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.1 88.2,207.2 124.4,199.2 160.5,198.2 196.7,199.4 232.9,191.8 269.1,187.0 305.3,193.1 341.5,168.2 377.6,167.4 413.8,161.4 450.0,157.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,189.4 88.2,178.4 124.4,178.8 160.5,169.4 196.7,163.8 232.9,130.4 269.1,127.4 305.3,121.3 341.5,107.1 377.6,94.6 413.8,84.3 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.8 88.2,163.1 124.4,161.0 160.5,131.6 196.7,131.7 232.9,119.9 269.1,108.3 305.3,99.4 341.5,84.9 377.6,71.4 413.8,58.0 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,172.9 124.4,130.0 160.5,129.8 196.7,115.4 232.9,110.6 269.1,96.4 305.3,86.3 341.5,73.7 377.6,58.1 413.8,45.8 450.0,41.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,158.4 124.4,129.2 160.5,126.1 196.7,109.8 232.9,98.4 269.1,87.7 305.3,75.3 341.5,65.3 377.6,52.9 413.8,40.6 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.04 ns | 195 ns | 367 ns | 411 ns | 385 ns |
| D38 | 2.17 ns | 400 ns | 477 ns | 306 ns | 414 ns |
| D57 | 269 ns | 506 ns | 520 ns | 511 ns | 685 ns |
| D76 | 285 ns | 296 ns | 487 ns | 689 ns | 410 ns |
| D115 | 252 ns | 493 ns | 556 ns | 955 ns | 1.09 µs |
| D153 | 356 ns | 514 ns | 643 ns | 990 ns | 1.4 µs |
| D230 | 587 ns | 707 ns | 1.02 µs | 1.3 µs | 1.85 µs |
| D307 | 436 ns | 628 ns | 1.17 µs | 1.54 µs | 10.4 µs |
| D462 | 1.21 µs | 2.88 µs | 3.29 µs | 3.5 µs | 5.48 µs |
| D616 | 1.34 µs | 1.43 µs | 2.08 µs | 3.1 µs | 4.04 µs |
| D924 | 1.97 µs | 2.05 µs | 3.11 µs | 3.33 µs | 4.74 µs |
| D1232 | 3.18 µs | 3.21 µs | 4.21 µs | 5.15 µs | 4.09 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,196.6 124.4,112.8 160.5,111.8 196.7,114.0 232.9,107.9 269.1,99.3 305.3,104.4 341.5,86.7 377.6,85.0 413.8,78.2 450.0,69.9 450.0,65.5 413.8,63.0 377.6,65.8 341.5,60.4 305.3,49.3 269.1,79.3 232.9,84.1 196.7,88.4 160.5,105.5 124.4,96.6 88.2,105.3 52.0,106.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,196.6 124.4,112.8 160.5,111.8 196.7,114.0 232.9,107.9 269.1,99.3 305.3,104.4 341.5,86.7 377.6,85.0 413.8,78.2 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.4 88.2,105.9 124.4,101.8 160.5,111.2 196.7,102.3 232.9,101.6 269.1,96.0 305.3,98.1 341.5,71.6 377.6,83.7 413.8,77.5 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,102.8 124.4,101.4 160.5,102.5 196.7,100.2 232.9,97.7 269.1,89.7 305.3,87.2 341.5,69.3 377.6,77.3 413.8,70.3 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,110.6 124.4,101.6 160.5,96.5 196.7,90.8 232.9,90.2 269.1,85.4 305.3,82.5 341.5,68.2 377.6,70.4 413.8,69.1 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.6 88.2,105.3 124.4,96.6 160.5,105.5 196.7,88.4 232.9,84.1 269.1,79.3 305.3,49.3 341.5,60.4 377.6,65.8 413.8,63.0 450.0,65.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.88 µs | 2.14 µs | 5.41 µs | 6.64 µs | 6.95 µs |
| D38 | 2.21 µs | 6.07 µs | 8.91 µs | 6.27 µs | 10.4 µs |
| D57 | 2.5 µs | 2.84 µs | 2.93 µs | 3 µs | 3.17 µs |
| D76 | 2.77 µs | 1.45 µs | 2.79 µs | 3.15 µs | 1.93 µs |
| D115 | 4.09 µs | 5.02 µs | 4.63 µs | 5.54 µs | 6.2 µs |
| D153 | 4.97 µs | 5.08 µs | 5.39 µs | 5.83 µs | 6.86 µs |
| D230 | 6.59 µs | 7.82 µs | 8.24 µs | 8.95 µs | 9.86 µs |
| D307 | 6.38 µs | 9.1 µs | 12.8 µs | 14.9 µs | 15.8 µs |
| D462 | 9.68 µs | 12.5 µs | 14.3 µs | 14.9 µs | 19.8 µs |
| D616 | 14.5 µs | 23.6 µs | 33 µs | 39.4 µs | 46.1 µs |
| D924 | 24.1 µs | 39.9 µs | 59.7 µs | 61.3 µs | 85.2 µs |
| D1232 | 34.5 µs | 68.1 µs | 89 µs | 116 µs | 95.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.3 88.2,187.0 124.4,183.5 160.5,180.5 196.7,169.2 232.9,163.6 269.1,155.4 305.3,156.4 341.5,144.3 377.6,132.6 413.8,117.8 450.0,107.5 450.0,78.0 413.8,81.3 377.6,99.1 341.5,123.5 305.3,130.1 269.1,143.8 232.9,154.2 196.7,157.2 160.5,190.9 124.4,176.6 88.2,142.1 52.0,153.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.3 88.2,187.0 124.4,183.5 160.5,180.5 196.7,169.2 232.9,163.6 269.1,155.4 305.3,156.4 341.5,144.3 377.6,132.6 413.8,117.8 450.0,107.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,188.0 88.2,157.8 124.4,179.8 160.5,199.2 196.7,163.3 232.9,162.9 269.1,150.4 305.3,146.1 341.5,137.0 377.6,118.4 413.8,103.3 450.0,87.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.1 88.2,146.7 124.4,178.9 160.5,180.3 196.7,165.6 232.9,161.2 269.1,148.9 305.3,136.2 341.5,132.9 377.6,108.8 413.8,91.6 450.0,80.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,156.8 124.4,178.2 160.5,176.8 196.7,160.5 232.9,158.9 269.1,146.6 305.3,131.9 341.5,131.8 377.6,103.6 413.8,90.8 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,142.1 124.4,176.6 160.5,190.9 196.7,157.2 232.9,154.2 269.1,143.8 305.3,130.1 341.5,123.5 377.6,99.1 413.8,81.3 450.0,78.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.2 ns | 1.54 µs | 3.73 µs | 4.42 µs | 4.26 µs |
| D38 | 4.76 ns | 4.19 µs | 5.64 µs | 3.54 µs | 5.89 µs |
| D57 | 57.4 ns | 4.65 µs | 4.88 µs | 5.12 µs | 5.61 µs |
| D76 | 83.2 ns | 2.32 µs | 4.69 µs | 5.74 µs | 3.96 µs |
| D115 | 126 ns | 8.27 µs | 8.31 µs | 10.4 µs | 11.7 µs |
| D153 | 194 ns | 8.59 µs | 9.78 µs | 11 µs | 13.1 µs |
| D230 | 347 ns | 13.8 µs | 15.3 µs | 18.5 µs | 21.6 µs |
| D307 | 243 ns | 16.1 µs | 21.7 µs | 30.3 µs | 32.3 µs |
| D462 | 648 ns | 70.6 µs | 137 µs | 200 µs | 283 µs |
| D616 | 706 ns | 176 µs | 353 µs | 381 µs | 573 µs |
| D924 | 894 ns | 425 µs | 487 µs | 665 µs | 1.63 ms |
| D1232 | 1.67 µs | 831 µs | 800 µs | 2.12 ms | 1.6 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,190.6 124.4,159.7 160.5,155.1 196.7,150.0 232.9,144.6 269.1,137.4 305.3,141.8 341.5,129.7 377.6,128.6 413.8,125.7 450.0,117.9 450.0,32.7 413.8,32.5 377.6,45.5 341.5,54.2 305.3,81.1 269.1,86.2 232.9,92.4 196.7,93.7 160.5,107.2 124.4,102.9 88.2,102.3 52.0,106.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,190.6 124.4,159.7 160.5,155.1 196.7,150.0 232.9,144.6 269.1,137.4 305.3,141.8 341.5,129.7 377.6,128.6 413.8,125.7 450.0,117.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.0 88.2,106.5 124.4,105.2 160.5,113.9 196.7,98.1 232.9,97.6 269.1,91.7 305.3,89.8 341.5,71.5 377.6,60.1 413.8,49.2 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.9 88.2,102.8 124.4,104.6 160.5,105.1 196.7,98.0 232.9,96.0 269.1,90.5 305.3,86.1 341.5,63.3 377.6,51.5 413.8,47.5 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.8 88.2,108.6 124.4,104.0 160.5,102.6 196.7,95.3 232.9,94.5 269.1,88.1 305.3,81.9 341.5,58.6 377.6,50.6 413.8,43.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,102.3 124.4,102.9 160.5,107.2 196.7,93.7 232.9,92.4 269.1,86.2 305.3,81.1 341.5,54.2 377.6,45.5 413.8,32.5 450.0,32.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.17 ns | 9.99 ns | 15.9 ns | 29.7 ns | 29 ns |
| D38 | 4.77 ns | 20.7 ns | 52 ns | 106 ns | 206 ns |
| D57 | 80 ns | 147 ns | 217 ns | 384 ns | 390 ns |
| D76 | 84.1 ns | 91.5 ns | 288 ns | 396 ns | 455 ns |
| D115 | 93.6 ns | 220 ns | 388 ns | 732 ns | 973 ns |
| D153 | 114 ns | 316 ns | 632 ns | 945 ns | 1.65 µs |
| D230 | 155 ns | 478 ns | 948 ns | 1.84 µs | 2.75 µs |
| D307 | 132 ns | 582 ns | 1.6 µs | 2.88 µs | 4.88 µs |
| D462 | 214 ns | 1.03 µs | 2.64 µs | 5.57 µs | 7.66 µs |
| D616 | 246 ns | 1.88 µs | 5.03 µs | 7.38 µs | 13.5 µs |
| D924 | 245 ns | 2.71 µs | 7.48 µs | 12.7 µs | 24.1 µs |
| D1232 | 456 ns | 5.17 µs | 13.1 µs | 24.5 µs | 25.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,182.9 124.4,133.9 160.5,133.0 196.7,131.2 232.9,127.7 269.1,122.3 305.3,125.2 341.5,116.8 377.6,114.4 413.8,114.4 450.0,103.7 450.0,33.8 413.8,34.7 377.6,44.8 341.5,54.6 305.3,62.4 269.1,72.4 232.9,81.2 196.7,90.5 160.5,103.7 124.4,106.4 88.2,117.4 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,182.9 124.4,133.9 160.5,133.0 196.7,131.2 232.9,127.7 269.1,122.3 305.3,125.2 341.5,116.8 377.6,114.4 413.8,114.4 450.0,103.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.0 88.2,157.3 124.4,123.3 160.5,131.6 196.7,116.3 232.9,110.0 269.1,102.8 305.3,99.4 341.5,89.5 377.6,79.1 413.8,72.7 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.9 88.2,141.4 124.4,116.5 160.5,111.6 196.7,106.5 232.9,98.0 269.1,90.9 305.3,81.9 341.5,73.1 377.6,61.9 413.8,55.0 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,129.1 124.4,106.6 160.5,106.1 196.7,95.4 232.9,91.0 269.1,79.4 305.3,71.6 341.5,60.2 377.6,55.3 413.8,45.9 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,117.4 124.4,106.4 160.5,103.7 196.7,90.5 232.9,81.2 269.1,72.4 305.3,62.4 341.5,54.6 377.6,44.8 413.8,34.7 450.0,33.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
