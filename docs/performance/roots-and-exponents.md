# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 79.1 ns | 132 ns | 160 ns | 254 ns | 263 ns |
| D38 | 77 ns | 157 ns | 252 ns | 377 ns | 308 ns |
| D57 | 188 ns | 427 ns | 643 ns | 1.14 µs | 1.06 µs |
| D76 | 199 ns | 506 ns | 807 ns | 1.3 µs | 1.07 µs |
| D115 | 222 ns | 812 ns | 1.23 µs | 2 µs | 2.03 µs |
| D153 | 246 ns | 907 ns | 1.6 µs | 2.77 µs | 3.49 µs |
| D230 | 292 ns | 1.42 µs | 3.31 µs | 4.28 µs | 7.98 µs |
| D307 | 322 ns | 1.28 µs | 4.68 µs | 9.57 µs | 13.6 µs |
| D462 | 408 ns | 3.13 µs | 9.45 µs | 17 µs | 25.9 µs |
| D616 | 475 ns | 3.63 µs | 14.5 µs | 30.1 µs | 56.2 µs |
| D924 | 372 ns | 8.96 µs | 34.1 µs | 62.9 µs | 130 µs |
| D1232 | 524 ns | 16.1 µs | 52.4 µs | 132 µs | 207 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.1 88.2,174.5 124.4,159.0 160.5,158.0 196.7,156.1 232.9,154.4 269.1,151.4 305.3,149.7 341.5,145.6 377.6,142.9 413.8,147.2 450.0,141.2 450.0,37.4 413.8,45.5 377.6,60.0 341.5,73.4 305.3,84.7 269.1,93.9 232.9,108.3 196.7,117.7 160.5,128.8 124.4,129.0 88.2,150.4 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.1 88.2,174.5 124.4,159.0 160.5,158.0 196.7,156.1 232.9,154.4 269.1,151.4 305.3,149.7 341.5,145.6 377.6,142.9 413.8,147.2 450.0,141.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.2 88.2,162.2 124.4,144.8 160.5,141.8 196.7,133.6 232.9,131.7 269.1,124.0 305.3,125.7 341.5,110.2 377.6,107.6 413.8,91.9 450.0,81.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,153.9 124.4,137.7 160.5,133.7 196.7,126.5 232.9,121.9 269.1,109.2 305.3,103.2 341.5,91.0 377.6,83.5 413.8,68.7 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,146.9 124.4,127.8 160.5,125.4 196.7,118.0 232.9,112.3 269.1,104.7 305.3,90.8 341.5,80.8 377.6,70.9 413.8,58.1 450.0,45.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,150.4 124.4,129.0 160.5,128.8 196.7,117.7 232.9,108.3 269.1,93.9 305.3,84.7 341.5,73.4 377.6,60.0 413.8,45.5 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.48 µs | 1.63 µs | 3.5 µs | 3.47 µs | 3.99 µs |
| D38 | 1.46 µs | 3.19 µs | 3.78 µs | 4.25 µs | 3.31 µs |
| D57 | 3.66 µs | 4.19 µs | 4.78 µs | 5.78 µs | 10.2 µs |
| D76 | 5.06 µs | 5.99 µs | 7.08 µs | 10 µs | 6.73 µs |
| D115 | 5.44 µs | 6.6 µs | 11.5 µs | 17.9 µs | 14.6 µs |
| D153 | 5.29 µs | 7.26 µs | 13.1 µs | 18.7 µs | 27.1 µs |
| D230 | 4.63 µs | 13.4 µs | 23.6 µs | 37.2 µs | 66.6 µs |
| D307 | 5.91 µs | 9.84 µs | 37.9 µs | 84.6 µs | 116 µs |
| D462 | 5.84 µs | 21.8 µs | 85.6 µs | 154 µs | 193 µs |
| D616 | 5.39 µs | 27 µs | 132 µs | 259 µs | 442 µs |
| D924 | 3.47 µs | 80.8 µs | 285 µs | 561 µs | 986 µs |
| D1232 | 3.62 µs | 141 µs | 412 µs | 992 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.5 88.2,201.7 124.4,181.8 160.5,174.8 196.7,173.2 232.9,173.8 269.1,176.7 305.3,171.4 341.5,171.7 377.6,173.4 413.8,183.0 450.0,182.1 450.0,37.4 413.8,60.3 377.6,77.7 341.5,95.8 305.3,106.8 269.1,118.8 232.9,138.4 196.7,151.8 160.5,168.6 124.4,159.5 88.2,184.0 52.0,180.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.5 88.2,201.7 124.4,181.8 160.5,174.8 196.7,173.2 232.9,173.8 269.1,176.7 305.3,171.4 341.5,171.7 377.6,173.4 413.8,183.0 450.0,182.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.3 88.2,184.8 124.4,178.9 160.5,171.1 196.7,169.0 232.9,167.0 269.1,153.7 305.3,160.4 341.5,143.0 377.6,138.5 413.8,114.6 450.0,102.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.8 88.2,181.1 124.4,176.0 160.5,167.5 196.7,156.9 232.9,154.1 269.1,141.4 305.3,131.1 341.5,113.4 377.6,104.0 413.8,87.2 450.0,79.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.0 88.2,178.6 124.4,171.9 160.5,159.9 196.7,147.3 232.9,146.4 269.1,131.5 305.3,113.6 341.5,100.6 377.6,89.4 413.8,72.6 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,184.0 124.4,159.5 160.5,168.6 196.7,151.8 232.9,138.4 269.1,118.8 305.3,106.8 341.5,95.8 377.6,77.7 413.8,60.3 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17 ns | 40.8 ns | 42.9 ns | 41.1 ns | 40.1 ns |
| D38 | 16.1 ns | 41.1 ns | 66.9 ns | 67 ns | 79.4 ns |
| D57 | 17.3 ns | 42.2 ns | 96 ns | 370 ns | 419 ns |
| D76 | 18.7 ns | 82.8 ns | 370 ns | 407 ns | 414 ns |
| D115 | 22.6 ns | 84.8 ns | 354 ns | 807 ns | 621 ns |
| D153 | 25.6 ns | 403 ns | 559 ns | 791 ns | 1.28 µs |
| D230 | 22.5 ns | 421 ns | 1.13 µs | 1.58 µs | 2.52 µs |
| D307 | 43.2 ns | 453 ns | 2 µs | 2.97 µs | 4.89 µs |
| D462 | 64.1 ns | 1.14 µs | 3.35 µs | 5.76 µs | 6.78 µs |
| D616 | 69.9 ns | 1.54 µs | 5.92 µs | 9.68 µs | 13.9 µs |
| D924 | 65.7 ns | 3.26 µs | 11 µs | 22.3 µs | 27.5 µs |
| D1232 | 68.2 ns | 6.07 µs | 18.9 µs | 25.9 µs | 45.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,199.7 124.4,198.1 160.5,196.4 196.7,192.3 232.9,189.6 269.1,192.4 305.3,178.2 341.5,169.7 377.6,167.8 413.8,169.1 450.0,168.3 450.0,27.0 413.8,38.0 377.6,52.8 341.5,68.4 305.3,75.5 269.1,89.9 232.9,104.6 196.7,120.3 160.5,129.2 124.4,128.9 88.2,165.0 52.0,179.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,199.7 124.4,198.1 160.5,196.4 196.7,192.3 232.9,189.6 269.1,192.4 305.3,178.2 341.5,169.7 377.6,167.8 413.8,169.1 450.0,168.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.5 88.2,179.3 124.4,178.7 160.5,164.1 196.7,163.6 232.9,129.7 269.1,128.8 305.3,127.2 341.5,107.2 377.6,100.6 413.8,84.4 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.7 124.4,160.9 160.5,131.6 196.7,132.5 232.9,122.6 269.1,107.3 305.3,94.9 341.5,83.8 377.6,71.4 413.8,57.9 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.3 88.2,168.7 124.4,131.6 160.5,129.5 196.7,114.7 232.9,115.1 269.1,100.0 305.3,86.4 341.5,72.0 377.6,60.7 413.8,42.6 450.0,39.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,165.0 124.4,128.9 160.5,129.2 196.7,120.3 232.9,104.6 269.1,89.9 305.3,75.5 341.5,68.4 377.6,52.8 413.8,38.0 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.91 ns | 273 ns | 401 ns | 380 ns | 410 ns |
| D38 | 8.23 ns | 380 ns | 385 ns | 388 ns | 309 ns |
| D57 | 272 ns | 506 ns | 502 ns | 488 ns | 699 ns |
| D76 | 282 ns | 475 ns | 491 ns | 733 ns | 358 ns |
| D115 | 292 ns | 492 ns | 490 ns | 937 ns | 727 ns |
| D153 | 306 ns | 515 ns | 526 ns | 943 ns | 1.07 µs |
| D230 | 447 ns | 651 ns | 1.14 µs | 1.24 µs | 1.45 µs |
| D307 | 745 ns | 414 ns | 1.22 µs | 1.5 µs | 10.4 µs |
| D462 | 1.34 µs | 2.85 µs | 3.71 µs | 4.12 µs | 4.36 µs |
| D616 | 1.4 µs | 1.2 µs | 1.99 µs | 2.48 µs | 3.88 µs |
| D924 | 1.31 µs | 2.06 µs | 3.05 µs | 3.63 µs | 5.22 µs |
| D1232 | 1.89 µs | 3.33 µs | 4.27 µs | 5.39 µs | 6.86 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.1 88.2,173.4 124.4,112.6 160.5,112.0 196.7,111.4 232.9,110.6 269.1,104.0 305.3,95.1 341.5,84.9 377.6,84.2 413.8,85.3 450.0,79.0 450.0,56.5 413.8,61.3 377.6,66.5 341.5,64.4 305.3,49.3 269.1,83.6 232.9,88.8 196.7,95.5 160.5,107.8 124.4,96.2 88.2,110.4 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.1 88.2,173.4 124.4,112.6 160.5,112.0 196.7,111.4 232.9,110.6 269.1,104.0 305.3,95.1 341.5,84.9 377.6,84.2 413.8,85.3 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.6 88.2,106.8 124.4,101.8 160.5,102.9 196.7,102.3 232.9,101.5 269.1,97.5 305.3,105.3 341.5,71.8 377.6,86.8 413.8,77.5 450.0,69.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,106.6 124.4,102.0 160.5,102.4 196.7,102.4 232.9,101.2 269.1,87.8 305.3,86.6 341.5,67.2 377.6,78.0 413.8,70.6 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,106.4 124.4,102.5 160.5,95.4 196.7,91.1 232.9,91.0 269.1,86.3 305.3,83.0 341.5,65.4 377.6,74.2 413.8,67.6 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,110.4 124.4,96.2 160.5,107.8 196.7,95.5 232.9,88.8 269.1,83.6 305.3,49.3 341.5,64.4 377.6,66.5 413.8,61.3 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.76 µs | 3.13 µs | 6.08 µs | 6.36 µs | 7.32 µs |
| D38 | 2.76 µs | 5.82 µs | 7.31 µs | 8.61 µs | 7.33 µs |
| D57 | 2.53 µs | 2.8 µs | 2.88 µs | 2.8 µs | 3.23 µs |
| D76 | 2.47 µs | 2.67 µs | 2.75 µs | 3.18 µs | 1.62 µs |
| D115 | 4.45 µs | 5 µs | 4.39 µs | 5.63 µs | 3.78 µs |
| D153 | 4.54 µs | 5.11 µs | 4.46 µs | 4.81 µs | 4.82 µs |
| D230 | 5.51 µs | 7.21 µs | 9 µs | 7.86 µs | 8.08 µs |
| D307 | 9.92 µs | 6.91 µs | 13.5 µs | 15.1 µs | 15.7 µs |
| D462 | 10.1 µs | 12.5 µs | 15.3 µs | 16.5 µs | 15.1 µs |
| D616 | 16 µs | 17.2 µs | 29.9 µs | 32 µs | 45.9 µs |
| D924 | 15.8 µs | 39.7 µs | 59.9 µs | 71.2 µs | 92.2 µs |
| D1232 | 21.1 µs | 67.5 µs | 89.6 µs | 126 µs | 142 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.6 88.2,180.6 124.4,183.1 160.5,183.9 196.7,166.8 232.9,166.2 269.1,160.6 305.3,143.6 341.5,143.2 377.6,129.6 413.8,130.1 450.0,121.7 450.0,66.6 413.8,79.0 377.6,99.2 341.5,131.4 305.3,130.3 269.1,149.5 232.9,164.5 196.7,171.5 160.5,196.1 124.4,176.0 88.2,152.3 52.0,152.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.6 88.2,180.6 124.4,183.1 160.5,183.9 196.7,166.8 232.9,166.2 269.1,160.6 305.3,143.6 341.5,143.2 377.6,129.6 413.8,130.1 450.0,121.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.9 88.2,159.0 124.4,180.2 160.5,181.6 196.7,163.4 232.9,162.8 269.1,152.8 305.3,154.1 341.5,136.9 377.6,127.6 413.8,103.4 450.0,88.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.7 88.2,152.4 124.4,179.4 160.5,180.7 196.7,167.1 232.9,166.7 269.1,146.4 305.3,134.6 341.5,131.0 377.6,111.6 413.8,91.5 450.0,79.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.4 88.2,147.7 124.4,180.2 160.5,176.5 196.7,160.0 232.9,164.5 269.1,150.3 305.3,131.4 341.5,128.8 377.6,109.6 413.8,86.5 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.4 88.2,152.3 124.4,176.0 160.5,196.1 196.7,171.5 232.9,164.5 269.1,149.5 305.3,130.3 341.5,131.4 377.6,99.2 413.8,79.0 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 384 ns | 383 ns | 1.36 µs | 1.41 µs | 1.56 µs |
| D38 | 385 ns | 1.25 µs | 1.48 µs | 1.66 µs | 913 ns |
| D57 | 1.25 µs | 1.45 µs | 1.42 µs | 1.44 µs | 1.65 µs |
| D76 | 1.23 µs | 1.31 µs | 1.4 µs | 1.62 µs | 834 ns |
| D115 | 2.21 µs | 2.37 µs | 2.19 µs | 2.73 µs | 1.99 µs |
| D153 | 2.32 µs | 2.5 µs | 2.27 µs | 2.51 µs | 2.47 µs |
| D230 | 2.82 µs | 3.36 µs | 4.24 µs | 3.75 µs | 3.93 µs |
| D307 | 4.91 µs | 3.41 µs | 6.37 µs | 6.88 µs | 7.19 µs |
| D462 | 4.98 µs | 5.76 µs | 7.05 µs | 7.62 µs | 7.03 µs |
| D616 | 7.87 µs | 7.27 µs | 12.3 µs | 12.7 µs | 18 µs |
| D924 | 7.52 µs | 16 µs | 21.5 µs | 25.8 µs | 33.2 µs |
| D1232 | 10.2 µs | 25.4 µs | 31.8 µs | 43.8 µs | 49.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.0 88.2,170.9 124.4,137.0 160.5,137.3 196.7,120.3 232.9,118.9 269.1,113.3 305.3,97.3 341.5,96.9 377.6,83.6 413.8,84.9 450.0,76.1 450.0,30.1 413.8,41.9 377.6,59.6 341.5,86.9 305.3,86.2 269.1,103.7 232.9,117.1 196.7,123.4 160.5,148.6 124.4,128.7 88.2,146.0 52.0,130.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.0 88.2,170.9 124.4,137.0 160.5,137.3 196.7,120.3 232.9,118.9 269.1,113.3 305.3,97.3 341.5,96.9 377.6,83.6 413.8,84.9 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.1 88.2,136.9 124.4,132.5 160.5,135.5 196.7,118.3 232.9,116.7 269.1,108.3 305.3,107.8 341.5,92.6 377.6,85.9 413.8,63.0 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,132.0 124.4,133.2 160.5,133.6 196.7,120.6 232.9,119.6 269.1,101.5 305.3,89.7 341.5,86.8 377.6,70.7 413.8,54.5 450.0,43.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.3 88.2,128.7 124.4,132.8 160.5,129.3 196.7,114.2 232.9,116.7 269.1,105.1 305.3,87.5 341.5,84.5 377.6,69.8 413.8,49.2 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.4 88.2,146.0 124.4,128.7 160.5,148.6 196.7,123.4 232.9,117.1 269.1,103.7 305.3,86.2 341.5,86.9 377.6,59.6 413.8,41.9 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 348 ns | 346 ns | 1.34 µs | 1.37 µs | 1.54 µs |
| D38 | 338 ns | 1.23 µs | 1.44 µs | 1.64 µs | 901 ns |
| D57 | 969 ns | 1.14 µs | 1.3 µs | 1.34 µs | 1.49 µs |
| D76 | 941 ns | 1.05 µs | 1.27 µs | 1.42 µs | 737 ns |
| D115 | 1.78 µs | 2.16 µs | 1.84 µs | 2.48 µs | 1.7 µs |
| D153 | 1.81 µs | 2.15 µs | 1.86 µs | 2.04 µs | 2 µs |
| D230 | 2.18 µs | 2.89 µs | 3.51 µs | 3.18 µs | 3.27 µs |
| D307 | 4.12 µs | 2.81 µs | 5.33 µs | 5.88 µs | 6.38 µs |
| D462 | 4.09 µs | 4.98 µs | 6.04 µs | 6.59 µs | 6.18 µs |
| D616 | 6 µs | 5.88 µs | 10.5 µs | 11.1 µs | 16 µs |
| D924 | 5.39 µs | 13.6 µs | 18.8 µs | 22.9 µs | 30.2 µs |
| D1232 | 7.72 µs | 22 µs | 28.4 µs | 39.9 µs | 46.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,173.9 88.2,174.8 124.4,144.2 160.5,145.1 196.7,126.6 232.9,126.2 269.1,120.7 305.3,102.3 341.5,102.6 377.6,91.4 413.8,94.6 450.0,84.2 450.0,32.3 413.8,44.7 377.6,63.0 341.5,90.6 305.3,89.7 269.1,109.0 232.9,123.2 196.7,127.9 160.5,152.2 124.4,131.9 88.2,146.4 52.0,130.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,173.9 88.2,174.8 124.4,144.2 160.5,145.1 196.7,126.6 232.9,126.2 269.1,120.7 305.3,102.3 341.5,102.6 377.6,91.4 413.8,94.6 450.0,84.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.0 88.2,137.4 124.4,139.6 160.5,142.0 196.7,121.0 232.9,121.1 269.1,112.6 305.3,113.4 341.5,96.8 377.6,92.0 413.8,67.8 450.0,53.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.8 88.2,132.7 124.4,135.8 160.5,136.5 196.7,125.7 232.9,125.4 269.1,106.9 305.3,94.9 341.5,91.3 377.6,75.1 413.8,58.3 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.2 88.2,129.1 124.4,134.8 160.5,133.2 196.7,117.0 232.9,122.7 269.1,109.9 305.3,92.0 341.5,88.7 377.6,73.6 413.8,52.7 450.0,36.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.9 88.2,146.4 124.4,131.9 160.5,152.2 196.7,127.9 232.9,123.2 269.1,109.0 305.3,89.7 341.5,90.6 377.6,63.0 413.8,44.7 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.2 ns | 2.05 µs | 4.21 µs | 4.09 µs | 4.68 µs |
| D38 | 13.4 ns | 3.82 µs | 4.41 µs | 4.89 µs | 3.86 µs |
| D57 | 57.3 ns | 4.61 µs | 4.79 µs | 4.69 µs | 5.61 µs |
| D76 | 69.3 ns | 4.02 µs | 4.66 µs | 5.7 µs | 3.24 µs |
| D115 | 135 ns | 8.17 µs | 7.53 µs | 10.2 µs | 6.89 µs |
| D153 | 177 ns | 8.47 µs | 8.26 µs | 8.9 µs | 9.12 µs |
| D230 | 280 ns | 12.7 µs | 16.5 µs | 16.1 µs | 17.9 µs |
| D307 | 464 ns | 12.4 µs | 23.1 µs | 30.4 µs | 32 µs |
| D462 | 765 ns | 69.6 µs | 149 µs | 204 µs | 221 µs |
| D616 | 768 ns | 139 µs | 326 µs | 341 µs | 570 µs |
| D924 | 732 ns | 424 µs | 484 µs | 794 µs | 1.74 ms |
| D1232 | 1.04 µs | 836 µs | 802 µs | 2.32 ms | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.0 88.2,205.8 124.4,184.7 160.5,182.0 196.7,172.3 232.9,168.4 269.1,161.8 305.3,154.4 341.5,147.2 377.6,147.2 413.8,147.9 450.0,142.7 450.0,28.7 413.8,35.4 377.6,51.5 341.5,65.2 305.3,93.2 269.1,101.5 232.9,111.3 196.7,115.4 160.5,126.3 124.4,118.4 88.2,123.8 52.0,121.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.0 88.2,205.8 124.4,184.7 160.5,182.0 196.7,172.3 232.9,168.4 269.1,161.8 305.3,154.4 341.5,147.2 377.6,147.2 413.8,147.9 450.0,142.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,132.9 88.2,123.9 124.4,121.2 160.5,123.2 196.7,112.9 232.9,112.4 269.1,106.5 305.3,106.9 341.5,81.9 377.6,71.9 413.8,55.8 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.5 88.2,121.8 124.4,120.7 160.5,121.1 196.7,114.1 232.9,112.8 269.1,102.8 305.3,97.9 341.5,70.9 377.6,59.5 413.8,53.8 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.0 88.2,120.4 124.4,121.0 160.5,118.1 196.7,109.7 232.9,111.7 269.1,103.1 305.3,93.9 341.5,66.3 377.6,58.9 413.8,46.7 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.0 88.2,123.8 124.4,118.4 160.5,126.3 196.7,115.4 232.9,111.3 269.1,101.5 305.3,93.2 341.5,65.2 377.6,51.5 413.8,35.4 450.0,28.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.74 ns | 13.3 ns | 18.5 ns | 28.6 ns | 29.3 ns |
| D38 | 6.79 ns | 19 ns | 29.7 ns | 135 ns | 145 ns |
| D57 | 79.3 ns | 149 ns | 217 ns | 356 ns | 395 ns |
| D76 | 82.8 ns | 141 ns | 292 ns | 397 ns | 396 ns |
| D115 | 98 ns | 226 ns | 341 ns | 739 ns | 611 ns |
| D153 | 112 ns | 317 ns | 547 ns | 791 ns | 1.24 µs |
| D230 | 132 ns | 425 ns | 1.02 µs | 1.5 µs | 2.48 µs |
| D307 | 174 ns | 447 ns | 1.95 µs | 2.89 µs | 4.87 µs |
| D462 | 212 ns | 1.02 µs | 2.86 µs | 5.89 µs | 6.61 µs |
| D616 | 237 ns | 1.45 µs | 4.65 µs | 7.07 µs | 13.5 µs |
| D924 | 167 ns | 2.68 µs | 7.51 µs | 14.2 µs | 26.3 µs |
| D1232 | 266 ns | 5.16 µs | 13.1 µs | 26.4 µs | 45.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,176.7 124.4,134.0 160.5,133.3 196.7,130.4 232.9,128.0 269.1,125.2 305.3,120.4 341.5,117.0 377.6,115.0 413.8,121.1 450.0,113.0 450.0,23.8 413.8,33.2 377.6,44.8 341.5,57.2 305.3,62.5 269.1,74.3 232.9,86.3 196.7,98.6 160.5,106.1 124.4,106.2 88.2,123.6 52.0,151.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,176.7 124.4,134.0 160.5,133.3 196.7,130.4 232.9,128.0 269.1,125.2 305.3,120.4 341.5,117.0 377.6,115.0 413.8,121.1 450.0,113.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,158.9 124.4,123.1 160.5,124.0 196.7,115.9 232.9,110.0 269.1,104.9 305.3,104.0 341.5,89.7 377.6,83.6 413.8,72.9 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,151.1 124.4,116.5 160.5,111.4 196.7,108.7 232.9,100.5 269.1,89.7 305.3,78.4 341.5,71.7 377.6,63.3 413.8,55.0 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,124.8 124.4,108.0 160.5,106.1 196.7,95.3 232.9,94.1 269.1,82.9 305.3,71.6 341.5,59.2 377.6,56.0 413.8,43.9 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,123.6 124.4,106.2 160.5,106.1 196.7,98.6 232.9,86.3 269.1,74.3 305.3,62.5 341.5,57.2 377.6,44.8 413.8,33.2 450.0,23.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
