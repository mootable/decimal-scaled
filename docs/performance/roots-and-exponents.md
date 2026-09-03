# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 64.4 ns | 132 ns | 159 ns | 235 ns | 250 ns |
| D38 | 80.1 ns | 156 ns | 253 ns | 378 ns | 409 ns |
| D57 | 191 ns | 420 ns | 643 ns | 972 ns | 1.07 µs |
| D76 | 198 ns | 488 ns | 664 ns | 879 ns | 1.63 µs |
| D115 | 223 ns | 766 ns | 1.34 µs | 2.02 µs | 2.98 µs |
| D153 | 251 ns | 854 ns | 1.79 µs | 1.87 µs | 2.72 µs |
| D230 | 290 ns | 1.41 µs | 3.25 µs | 4.74 µs | 9.4 µs |
| D307 | 348 ns | 1.83 µs | 4.67 µs | 8.89 µs | 14.9 µs |
| D462 | 386 ns | 3.53 µs | 9.51 µs | 14.3 µs | 30.3 µs |
| D616 | 450 ns | 4.53 µs | 13.1 µs | 30.7 µs | 56.3 µs |
| D924 | 596 ns | 9.68 µs | 34.3 µs | 53.8 µs | 130 µs |
| D1232 | 1.19 µs | 15.3 µs | 45.5 µs | 119 µs | 123 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.6 88.2,173.9 124.4,158.8 160.5,158.2 196.7,156.1 232.9,154.0 269.1,151.5 305.3,148.3 341.5,146.5 377.6,143.9 413.8,139.0 450.0,126.9 450.0,46.3 413.8,45.4 377.6,60.0 341.5,70.7 305.3,83.0 269.1,91.1 232.9,112.6 196.7,111.0 160.5,121.5 124.4,128.8 88.2,145.5 52.0,154.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.6 88.2,173.9 124.4,158.8 160.5,158.2 196.7,156.1 232.9,154.0 269.1,151.5 305.3,148.3 341.5,146.5 377.6,143.9 413.8,139.0 450.0,126.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,162.2 124.4,145.1 160.5,142.5 196.7,134.6 232.9,132.7 269.1,124.1 305.3,119.5 341.5,108.1 377.6,103.8 413.8,90.6 450.0,82.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.0 88.2,153.9 124.4,137.7 160.5,137.1 196.7,125.0 232.9,119.9 269.1,109.5 305.3,103.2 341.5,90.9 377.6,85.3 413.8,68.6 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,146.9 124.4,130.5 160.5,132.2 196.7,117.8 232.9,119.1 269.1,103.0 305.3,92.0 341.5,83.7 377.6,70.5 413.8,60.8 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.1 88.2,145.5 124.4,128.8 160.5,121.5 196.7,111.0 232.9,112.6 269.1,91.1 305.3,83.0 341.5,70.7 377.6,60.0 413.8,45.4 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.66 µs | 3.23 µs | 2.96 µs | 3.69 µs |
| D38 | 1.59 ns | 3.23 µs | 3.84 µs | 4.67 µs | 5.17 µs |
| D57 | 2.81 ns | 4.18 µs | 4.76 µs | 5.75 µs | 10.1 µs |
| D76 | 3.52 ns | 6.38 µs | 5.95 µs | 5.5 µs | 11.3 µs |
| D115 | 11.9 ns | 7.09 µs | 13.3 µs | 18.1 µs | 22.5 µs |
| D153 | 16.2 ns | 7.71 µs | 17.1 µs | 12.7 µs | 20.2 µs |
| D230 | 45.3 ns | 12.2 µs | 23.7 µs | 36.9 µs | 84.4 µs |
| D307 | 85.2 ns | 16.1 µs | 38.3 µs | 75 µs | 124 µs |
| D462 | 105 ns | 23.7 µs | 84.7 µs | 132 µs | 228 µs |
| D616 | 130 ns | 35.4 µs | 113 µs | 268 µs | 444 µs |
| D924 | 217 ns | 86.1 µs | 288 µs | 499 µs | 989 µs |
| D1232 | 384 ns | 132 µs | 373 µs | 918 µs | 2.02 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,204.2 124.4,197.2 160.5,194.4 196.7,179.3 232.9,175.4 269.1,162.7 305.3,154.8 341.5,152.3 377.6,149.6 413.8,143.3 450.0,136.1 450.0,29.9 413.8,38.7 377.6,48.6 341.5,56.9 305.3,64.5 269.1,69.2 232.9,87.0 196.7,85.7 160.5,94.2 124.4,95.6 88.2,103.9 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,204.2 124.4,197.2 160.5,194.4 196.7,179.3 232.9,175.4 269.1,162.7 305.3,154.8 341.5,152.3 377.6,149.6 413.8,143.3 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.0 88.2,109.7 124.4,106.5 160.5,101.3 196.7,100.0 232.9,98.9 269.1,93.3 305.3,89.8 341.5,85.0 377.6,80.0 413.8,69.0 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,107.6 124.4,104.9 160.5,102.2 196.7,92.2 232.9,89.1 269.1,85.0 305.3,79.0 341.5,69.2 377.6,65.6 413.8,54.0 450.0,50.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,105.2 124.4,102.6 160.5,103.1 196.7,88.4 232.9,92.8 269.1,79.5 305.3,70.7 341.5,63.7 377.6,54.9 413.8,47.2 450.0,39.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,103.9 124.4,95.6 160.5,94.2 196.7,85.7 232.9,87.0 269.1,69.2 305.3,64.5 341.5,56.9 377.6,48.6 413.8,38.7 450.0,29.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.3 ns | 40.5 ns | 41.3 ns | 37.1 ns | 39.9 ns |
| D38 | 15.5 ns | 41.4 ns | 66.8 ns | 75.4 ns | 108 ns |
| D57 | 16.5 ns | 42.2 ns | 95.8 ns | 387 ns | 416 ns |
| D76 | 17.3 ns | 95 ns | 311 ns | 296 ns | 609 ns |
| D115 | 22.4 ns | 97.4 ns | 389 ns | 793 ns | 969 ns |
| D153 | 25.5 ns | 417 ns | 677 ns | 590 ns | 977 ns |
| D230 | 29 ns | 410 ns | 1.13 µs | 1.71 µs | 3.02 µs |
| D307 | 43.2 ns | 730 ns | 1.78 µs | 2.77 µs | 5.32 µs |
| D462 | 62.5 ns | 1.22 µs | 3.35 µs | 4.76 µs | 7.23 µs |
| D616 | 68.1 ns | 1.96 µs | 5.32 µs | 10.3 µs | 13.9 µs |
| D924 | 103 ns | 3.48 µs | 11 µs | 19.5 µs | 26.8 µs |
| D1232 | 106 ns | 5.6 µs | 16.5 µs | 23.8 µs | 33.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,200.5 124.4,199.1 160.5,198.1 196.7,192.5 232.9,189.6 269.1,186.8 305.3,178.2 341.5,170.2 377.6,168.3 413.8,159.4 450.0,158.6 450.0,33.9 413.8,38.6 377.6,52.9 341.5,67.0 305.3,73.7 269.1,86.0 232.9,110.5 196.7,110.7 160.5,120.8 124.4,129.1 88.2,158.4 52.0,180.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,200.5 124.4,199.1 160.5,198.1 196.7,192.5 232.9,189.6 269.1,186.8 305.3,178.2 341.5,170.2 377.6,168.3 413.8,159.4 450.0,158.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,178.7 160.5,161.1 196.7,160.6 232.9,129.0 269.1,129.4 305.3,116.8 341.5,105.8 377.6,95.3 413.8,83.0 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,168.8 124.4,160.9 160.5,135.4 196.7,130.5 232.9,118.5 269.1,107.3 305.3,97.4 341.5,83.7 377.6,73.7 413.8,57.9 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.5 88.2,166.1 124.4,130.6 160.5,136.5 196.7,115.0 232.9,121.5 269.1,98.4 305.3,87.9 341.5,76.1 377.6,59.4 413.8,45.5 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,158.4 124.4,129.1 160.5,120.8 196.7,110.7 232.9,110.5 269.1,86.0 305.3,73.7 341.5,67.0 377.6,52.9 413.8,38.6 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.14 ns | 273 ns | 379 ns | 363 ns | 380 ns |
| D38 | 2.81 ns | 370 ns | 390 ns | 412 ns | 414 ns |
| D57 | 279 ns | 493 ns | 505 ns | 487 ns | 684 ns |
| D76 | 285 ns | 499 ns | 411 ns | 345 ns | 616 ns |
| D115 | 286 ns | 517 ns | 631 ns | 944 ns | 994 ns |
| D153 | 320 ns | 558 ns | 773 ns | 639 ns | 814 ns |
| D230 | 559 ns | 595 ns | 1.16 µs | 1.11 µs | 1.92 µs |
| D307 | 784 ns | 785 ns | 1.21 µs | 1.18 µs | 11.1 µs |
| D462 | 1.19 µs | 3.14 µs | 3.65 µs | 3.62 µs | 4.97 µs |
| D616 | 1.39 µs | 1.42 µs | 1.79 µs | 2.89 µs | 3.92 µs |
| D924 | 2.19 µs | 2.25 µs | 3.11 µs | 2.98 µs | 5.14 µs |
| D1232 | 3.01 µs | 3.1 µs | 3.61 µs | 5.16 µs | 5.27 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,192.0 124.4,112.1 160.5,111.8 196.7,111.7 232.9,109.8 269.1,100.1 305.3,94.2 341.5,87.0 377.6,84.2 413.8,76.4 450.0,70.9 450.0,61.1 413.8,61.5 377.6,66.3 341.5,62.2 305.3,48.2 269.1,78.6 232.9,93.6 196.7,90.1 160.5,98.4 124.4,96.6 88.2,105.3 52.0,106.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,192.0 124.4,112.1 160.5,111.8 196.7,111.7 232.9,109.8 269.1,100.1 305.3,94.2 341.5,87.0 377.6,84.2 413.8,76.4 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.5 88.2,107.3 124.4,102.3 160.5,102.1 196.7,101.5 232.9,100.1 269.1,99.0 305.3,94.2 341.5,70.1 377.6,83.8 413.8,75.9 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,106.4 124.4,101.9 160.5,105.5 196.7,98.0 232.9,94.5 269.1,87.4 305.3,86.8 341.5,67.5 377.6,79.9 413.8,70.3 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.6 88.2,105.4 124.4,102.5 160.5,108.5 196.7,91.0 232.9,97.8 269.1,88.3 305.3,87.1 341.5,67.6 377.6,71.5 413.8,71.0 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,105.3 124.4,96.6 160.5,98.4 196.7,90.1 232.9,93.6 269.1,78.6 305.3,48.2 341.5,62.2 377.6,66.3 413.8,61.5 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.34 µs | 3.12 µs | 5.82 µs | 5.47 µs | 7 µs |
| D38 | 2.9 µs | 5.83 µs | 7.33 µs | 9.04 µs | 10.5 µs |
| D57 | 2.74 µs | 2.75 µs | 2.95 µs | 2.88 µs | 3.15 µs |
| D76 | 2.72 µs | 2.92 µs | 2.34 µs | 1.54 µs | 2.96 µs |
| D115 | 4.46 µs | 5.2 µs | 5.1 µs | 5.55 µs | 5.68 µs |
| D153 | 4.52 µs | 5.44 µs | 5.79 µs | 3.48 µs | 3.77 µs |
| D230 | 6.49 µs | 6.25 µs | 8.76 µs | 7.14 µs | 10.6 µs |
| D307 | 9.83 µs | 11.5 µs | 13.5 µs | 12.6 µs | 16.7 µs |
| D462 | 9.81 µs | 13.3 µs | 15.5 µs | 13.9 µs | 18.6 µs |
| D616 | 16.4 µs | 23.8 µs | 26.3 µs | 35.4 µs | 45.7 µs |
| D924 | 25.5 µs | 43.1 µs | 60.5 µs | 62.3 µs | 93.9 µs |
| D1232 | 32.1 µs | 61.4 µs | 81.4 µs | 115 µs | 92.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.4 88.2,179.2 124.4,180.8 160.5,181.0 196.7,166.7 232.9,166.3 269.1,155.9 305.3,143.8 341.5,143.9 377.6,129.1 413.8,116.3 450.0,109.5 450.0,79.0 413.8,78.5 377.6,99.3 341.5,125.4 305.3,128.5 269.1,141.7 232.9,171.6 196.7,159.7 160.5,178.6 124.4,176.8 88.2,142.0 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.4 88.2,179.2 124.4,180.8 160.5,181.0 196.7,166.7 232.9,166.3 269.1,155.9 305.3,143.8 341.5,143.9 377.6,129.1 413.8,116.3 450.0,109.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.0 88.2,158.9 124.4,180.7 160.5,179.0 196.7,162.3 232.9,161.0 269.1,156.9 305.3,139.3 341.5,135.1 377.6,118.2 413.8,101.0 450.0,90.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.0 88.2,152.3 124.4,178.7 160.5,185.3 196.7,162.8 232.9,159.1 269.1,147.2 305.3,134.6 341.5,130.6 377.6,115.3 413.8,91.2 450.0,82.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,160.8 88.2,146.3 124.4,179.4 160.5,197.6 196.7,160.4 232.9,173.9 269.1,153.1 305.3,136.7 341.5,133.8 377.6,106.7 413.8,90.4 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,142.0 124.4,176.8 160.5,178.6 196.7,159.7 232.9,171.6 269.1,141.7 305.3,128.5 341.5,125.4 377.6,99.3 413.8,78.5 450.0,79.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.16 ns | 2.06 µs | 3.88 µs | 3.56 µs | 4.31 µs |
| D38 | 6.48 ns | 3.86 µs | 4.47 µs | 5.35 µs | 5.87 µs |
| D57 | 62.3 ns | 4.6 µs | 4.87 µs | 4.87 µs | 5.62 µs |
| D76 | 83.4 ns | 4.5 µs | 3.97 µs | 2.88 µs | 5.59 µs |
| D115 | 136 ns | 8.72 µs | 8.91 µs | 10.2 µs | 10.8 µs |
| D153 | 173 ns | 9.08 µs | 10.4 µs | 6.57 µs | 7.14 µs |
| D230 | 340 ns | 11.6 µs | 16.3 µs | 15.3 µs | 22.9 µs |
| D307 | 428 ns | 20.8 µs | 23.1 µs | 26 µs | 34.9 µs |
| D462 | 656 ns | 77.9 µs | 148 µs | 175 µs | 262 µs |
| D616 | 753 ns | 178 µs | 293 µs | 356 µs | 574 µs |
| D924 | 1 µs | 453 µs | 490 µs | 701 µs | 1.75 ms |
| D1232 | 1.44 µs | 760 µs | 719 µs | 2.11 ms | 1.9 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,186.8 124.4,158.7 160.5,155.1 196.7,149.0 232.9,146.0 269.1,137.7 305.3,134.8 341.5,129.5 377.6,127.8 413.8,124.3 450.0,119.8 450.0,30.6 413.8,31.7 377.6,45.5 341.5,55.2 305.3,80.2 269.1,85.4 232.9,99.9 196.7,94.8 160.5,102.9 124.4,102.9 88.2,102.3 52.0,106.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,186.8 124.4,158.7 160.5,155.1 196.7,149.0 232.9,146.0 269.1,137.7 305.3,134.8 341.5,129.5 377.6,127.8 413.8,124.3 450.0,119.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.3 88.2,107.5 124.4,105.4 160.5,105.6 196.7,97.4 232.9,96.9 269.1,93.8 305.3,86.6 341.5,70.2 377.6,60.0 413.8,48.4 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,105.7 124.4,104.7 160.5,107.2 196.7,97.2 232.9,95.3 269.1,89.6 305.3,85.3 341.5,62.3 377.6,53.8 413.8,47.4 450.0,42.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,103.5 124.4,104.6 160.5,111.1 196.7,95.4 232.9,100.9 269.1,90.4 305.3,83.9 341.5,60.2 377.6,51.4 413.8,43.0 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,102.3 124.4,102.9 160.5,102.9 196.7,94.8 232.9,99.9 269.1,85.4 305.3,80.2 341.5,55.2 377.6,45.5 413.8,31.7 450.0,30.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.79 ns | 13.4 ns | 18.3 ns | 29.7 ns | 28.9 ns |
| D38 | 7.29 ns | 19.6 ns | 39.4 ns | 151 ns | 210 ns |
| D57 | 82.6 ns | 147 ns | 218 ns | 358 ns | 393 ns |
| D76 | 83.6 ns | 144 ns | 245 ns | 277 ns | 609 ns |
| D115 | 97 ns | 239 ns | 387 ns | 733 ns | 930 ns |
| D153 | 132 ns | 341 ns | 662 ns | 582 ns | 967 ns |
| D230 | 147 ns | 435 ns | 1.02 µs | 1.69 µs | 2.86 µs |
| D307 | 176 ns | 725 ns | 1.71 µs | 2.7 µs | 5.17 µs |
| D462 | 208 ns | 1.1 µs | 2.86 µs | 4.89 µs | 7.03 µs |
| D616 | 234 ns | 1.84 µs | 4.43 µs | 6.81 µs | 13.5 µs |
| D924 | 291 ns | 2.88 µs | 7.58 µs | 14.3 µs | 26.1 µs |
| D1232 | 401 ns | 4.8 µs | 12.6 µs | 24.4 µs | 31.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.8 88.2,175.5 124.4,133.3 160.5,133.1 196.7,130.5 232.9,125.2 269.1,123.3 305.3,120.2 341.5,117.3 377.6,115.2 413.8,111.4 450.0,105.9 450.0,30.1 413.8,33.3 377.6,44.8 341.5,56.1 305.3,61.5 269.1,71.7 232.9,90.6 196.7,91.3 160.5,98.6 124.4,106.2 88.2,117.1 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.8 88.2,175.5 124.4,133.3 160.5,133.1 196.7,130.5 232.9,125.2 269.1,123.3 305.3,120.2 341.5,117.3 377.6,115.2 413.8,111.4 450.0,105.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,158.3 124.4,123.3 160.5,123.7 196.7,114.9 232.9,108.7 269.1,104.4 305.3,95.6 341.5,88.4 377.6,79.4 413.8,71.6 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.5 88.2,146.2 124.4,116.5 160.5,114.4 196.7,106.5 232.9,97.2 269.1,89.6 305.3,80.7 341.5,71.8 377.6,64.1 413.8,54.8 450.0,45.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,122.8 124.4,107.8 160.5,112.3 196.7,95.4 232.9,99.4 269.1,80.9 305.3,72.8 341.5,62.4 377.6,56.7 413.8,43.7 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,117.1 124.4,106.2 160.5,98.6 196.7,91.3 232.9,90.6 269.1,71.7 305.3,61.5 341.5,56.1 377.6,44.8 413.8,33.3 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
