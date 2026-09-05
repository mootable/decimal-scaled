# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 76.8 ns | 132 ns | 157 ns | 255 ns | 254 ns |
| D38 | 82.8 ns | 156 ns | 254 ns | 294 ns | 262 ns |
| D57 | 189 ns | 430 ns | 699 ns | 1.13 µs | 825 ns |
| D76 | 198 ns | 420 ns | 800 ns | 1.28 µs | 1.62 µs |
| D115 | 225 ns | 755 ns | 1.35 µs | 2.01 µs | 2.98 µs |
| D153 | 254 ns | 877 ns | 1.74 µs | 3.1 µs | 4.3 µs |
| D230 | 278 ns | 1.41 µs | 2.13 µs | 3.36 µs | 8 µs |
| D307 | 300 ns | 1.76 µs | 4.67 µs | 8.07 µs | 14.9 µs |
| D462 | 410 ns | 3.15 µs | 8.91 µs | 17.1 µs | 33.5 µs |
| D616 | 415 ns | 4.46 µs | 12.8 µs | 23.5 µs | 56.1 µs |
| D924 | 528 ns | 9.1 µs | 21.2 µs | 53.8 µs | 117 µs |
| D1232 | 615 ns | 11.4 µs | 52.4 µs | 102 µs | 181 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.6 88.2,173.3 124.4,158.9 160.5,158.1 196.7,155.9 232.9,153.8 269.1,152.2 305.3,150.9 341.5,145.5 377.6,145.3 413.8,141.1 450.0,138.4 450.0,39.7 413.8,47.3 377.6,60.0 341.5,69.0 305.3,83.0 269.1,93.9 232.9,104.6 196.7,111.0 160.5,121.7 124.4,133.3 88.2,153.2 52.0,153.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.6 88.2,173.3 124.4,158.9 160.5,158.1 196.7,155.9 232.9,153.8 269.1,152.2 305.3,150.9 341.5,145.5 377.6,145.3 413.8,141.1 450.0,138.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,162.3 124.4,144.6 160.5,145.1 196.7,134.9 232.9,132.3 269.1,124.0 305.3,120.1 341.5,110.1 377.6,104.0 413.8,91.6 450.0,87.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.2 88.2,153.8 124.4,136.2 160.5,133.9 196.7,124.8 232.9,120.4 269.1,116.9 305.3,103.2 341.5,92.0 377.6,85.7 413.8,77.0 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,151.2 124.4,127.8 160.5,125.7 196.7,117.9 232.9,110.3 269.1,108.9 305.3,93.7 341.5,80.7 377.6,75.2 413.8,60.8 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,153.2 124.4,133.3 160.5,121.7 196.7,111.0 232.9,104.6 269.1,93.9 305.3,83.0 341.5,69.0 377.6,60.0 413.8,47.3 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.48 µs | 1.79 µs | 3.2 µs | 3.42 µs | 3.64 µs |
| D38 | 1.47 µs | 3.2 µs | 3.79 µs | 3.72 µs | 2.9 µs |
| D57 | 3.77 µs | 3.8 µs | 4.32 µs | 5.68 µs | 8.12 µs |
| D76 | 5.48 µs | 4.31 µs | 7 µs | 9.2 µs | 11.2 µs |
| D115 | 5.71 µs | 6.73 µs | 13.2 µs | 18 µs | 22.2 µs |
| D153 | 5.27 µs | 7.26 µs | 15.3 µs | 22.2 µs | 35.2 µs |
| D230 | 5.26 µs | 13.9 µs | 16.3 µs | 24.9 µs | 66.6 µs |
| D307 | 5.28 µs | 14.4 µs | 38.1 µs | 66.8 µs | 123 µs |
| D462 | 5.83 µs | 22.6 µs | 80.5 µs | 155 µs | 248 µs |
| D616 | 4.06 µs | 34.9 µs | 113 µs | 196 µs | 442 µs |
| D924 | 5.48 µs | 81 µs | 174 µs | 486 µs | 895 µs |
| D1232 | 4.24 µs | 94.3 µs | 407 µs | 773 µs | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.5 88.2,201.7 124.4,181.2 160.5,173.0 196.7,172.2 232.9,173.9 269.1,174.0 305.3,173.9 341.5,171.7 377.6,179.6 413.8,173.1 450.0,178.6 450.0,37.5 413.8,62.4 377.6,77.7 341.5,90.2 305.3,105.5 269.1,118.8 232.9,132.6 196.7,142.7 160.5,157.5 124.4,164.5 88.2,186.9 52.0,182.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.5 88.2,201.7 124.4,181.2 160.5,173.0 196.7,172.2 232.9,173.9 269.1,174.0 305.3,173.9 341.5,171.7 377.6,179.6 413.8,173.1 450.0,178.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.3 88.2,184.7 124.4,181.0 160.5,178.3 196.7,168.6 232.9,167.0 269.1,152.9 305.3,152.0 341.5,142.2 377.6,132.9 413.8,114.6 450.0,111.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.8 88.2,181.0 124.4,178.2 160.5,167.8 196.7,154.0 232.9,150.8 269.1,149.4 305.3,130.9 341.5,114.7 377.6,107.4 413.8,98.0 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.3 88.2,181.5 124.4,172.3 160.5,161.8 196.7,147.2 232.9,142.7 269.1,140.2 305.3,118.8 341.5,100.4 377.6,95.4 413.8,75.7 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,186.9 124.4,164.5 160.5,157.5 196.7,142.7 232.9,132.6 269.1,118.8 305.3,105.5 341.5,90.2 377.6,77.7 413.8,62.4 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 40.3 ns | 41.3 ns | 39.9 ns | 39.9 ns |
| D38 | 16.1 ns | 41.1 ns | 66.4 ns | 58.5 ns | 70.6 ns |
| D57 | 17.1 ns | 41.4 ns | 81.7 ns | 370 ns | 330 ns |
| D76 | 17.7 ns | 76 ns | 369 ns | 377 ns | 607 ns |
| D115 | 20.1 ns | 85.3 ns | 390 ns | 809 ns | 971 ns |
| D153 | 25.3 ns | 390 ns | 634 ns | 975 ns | 1.59 µs |
| D230 | 29.6 ns | 457 ns | 708 ns | 1.15 µs | 2.51 µs |
| D307 | 43.8 ns | 689 ns | 1.77 µs | 2.59 µs | 5.33 µs |
| D462 | 64.1 ns | 1.14 µs | 3.16 µs | 5.69 µs | 7.86 µs |
| D616 | 62.5 ns | 1.96 µs | 5.13 µs | 7.85 µs | 14 µs |
| D924 | 93.7 ns | 3.27 µs | 6.91 µs | 19.1 µs | 24.6 µs |
| D1232 | 81.7 ns | 4.49 µs | 18.7 µs | 20.2 µs | 46.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,199.6 124.4,198.3 160.5,197.6 196.7,194.8 232.9,189.9 269.1,186.5 305.3,177.9 341.5,169.7 377.6,170.2 413.8,161.4 450.0,164.4 450.0,26.5 413.8,40.4 377.6,52.7 341.5,65.2 305.3,73.7 269.1,90.0 232.9,99.9 196.7,110.6 160.5,120.8 124.4,134.1 88.2,167.6 52.0,180.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,199.6 124.4,198.3 160.5,197.6 196.7,194.8 232.9,189.9 269.1,186.5 305.3,177.9 341.5,169.7 377.6,170.2 413.8,161.4 450.0,164.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.7 88.2,179.3 124.4,179.1 160.5,166.0 196.7,163.5 232.9,130.4 269.1,127.0 305.3,118.1 341.5,107.2 377.6,95.4 413.8,84.3 450.0,77.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,168.9 124.4,164.4 160.5,131.6 196.7,130.4 232.9,119.9 269.1,117.5 305.3,97.6 341.5,85.0 377.6,74.5 413.8,68.0 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,171.7 124.4,131.6 160.5,131.2 196.7,114.6 232.9,110.6 269.1,107.0 305.3,89.4 341.5,72.2 377.6,65.3 413.8,45.9 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,167.6 124.4,134.1 160.5,120.8 196.7,110.6 232.9,99.9 269.1,90.0 305.3,73.7 341.5,65.2 377.6,52.7 413.8,40.4 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 370 ns | 339 ns | 355 ns | 355 ns |
| D38 | 11.3 ns | 371 ns | 384 ns | 335 ns | 238 ns |
| D57 | 272 ns | 508 ns | 489 ns | 510 ns | 547 ns |
| D76 | 294 ns | 354 ns | 494 ns | 616 ns | 662 ns |
| D115 | 304 ns | 487 ns | 651 ns | 996 ns | 967 ns |
| D153 | 296 ns | 518 ns | 657 ns | 996 ns | 1.29 µs |
| D230 | 519 ns | 718 ns | 703 ns | 811 ns | 1.49 µs |
| D307 | 682 ns | 616 ns | 1.25 µs | 1.12 µs | 11.3 µs |
| D462 | 1.3 µs | 1.25 µs | 1.41 µs | 1.89 µs | 2.98 µs |
| D616 | 1.16 µs | 1.43 µs | 1.87 µs | 2.08 µs | 3.88 µs |
| D924 | 1.96 µs | 2 µs | 1.89 µs | 2.96 µs | 4.64 µs |
| D1232 | 2.05 µs | 2.09 µs | 4.22 µs | 4.36 µs | 6.13 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.2 88.2,167.9 124.4,112.6 160.5,111.3 196.7,110.7 232.9,111.2 269.1,101.4 305.3,96.6 341.5,85.5 377.6,87.5 413.8,78.3 450.0,77.5 450.0,58.5 413.8,63.3 377.6,66.4 341.5,71.0 305.3,47.9 269.1,83.0 232.9,85.6 196.7,90.6 160.5,97.2 124.4,100.5 88.2,114.9 52.0,108.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.2 88.2,167.9 124.4,112.6 160.5,111.3 196.7,110.7 232.9,111.2 269.1,101.4 305.3,96.6 341.5,85.5 377.6,87.5 413.8,78.3 450.0,77.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,107.3 88.2,107.2 124.4,101.8 160.5,108.0 196.7,102.5 232.9,101.4 269.1,95.8 305.3,98.4 341.5,86.1 377.6,83.8 413.8,78.0 450.0,77.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,106.6 124.4,102.4 160.5,102.2 196.7,97.5 232.9,97.3 269.1,96.1 305.3,86.1 341.5,84.0 377.6,79.2 413.8,78.9 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,109.0 124.4,101.7 160.5,98.4 196.7,90.1 232.9,90.1 269.1,93.6 305.3,88.0 341.5,78.9 377.6,77.3 413.8,71.2 450.0,64.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.0 88.2,114.9 124.4,100.5 160.5,97.2 196.7,90.6 232.9,85.6 269.1,83.0 305.3,47.9 341.5,71.0 377.6,66.4 413.8,63.3 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.78 µs | 3.26 µs | 5.81 µs | 6.33 µs | 7 µs |
| D38 | 2.74 µs | 5.81 µs | 7.31 µs | 7.32 µs | 6.68 µs |
| D57 | 2.44 µs | 2.56 µs | 2.59 µs | 2.68 µs | 2.37 µs |
| D76 | 2.61 µs | 1.76 µs | 2.69 µs | 2.75 µs | 2.83 µs |
| D115 | 4.72 µs | 4.76 µs | 4.98 µs | 5.3 µs | 5.55 µs |
| D153 | 4.35 µs | 4.86 µs | 5.09 µs | 5.48 µs | 5.96 µs |
| D230 | 5.98 µs | 7.26 µs | 4.94 µs | 5.02 µs | 7.46 µs |
| D307 | 9.21 µs | 8.98 µs | 12.6 µs | 10.6 µs | 15.1 µs |
| D462 | 9.66 µs | 11.5 µs | 12.9 µs | 14.9 µs | 17.8 µs |
| D616 | 10.8 µs | 21.6 µs | 22.5 µs | 22.6 µs | 39.4 µs |
| D924 | 23 µs | 35 µs | 32.9 µs | 52 µs | 72.4 µs |
| D1232 | 24 µs | 41.2 µs | 75.6 µs | 83.9 µs | 109 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.4 88.2,180.9 124.4,184.2 160.5,182.2 196.7,165.0 232.9,167.5 269.1,158.2 305.3,145.7 341.5,144.3 377.6,141.0 413.8,119.2 450.0,117.9 450.0,74.3 413.8,86.0 377.6,103.7 341.5,126.7 305.3,131.4 269.1,151.8 232.9,158.3 196.7,160.4 160.5,179.9 124.4,185.0 88.2,155.0 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.4 88.2,180.9 124.4,184.2 160.5,182.2 196.7,165.0 232.9,167.5 269.1,158.2 305.3,145.7 341.5,144.3 377.6,141.0 413.8,119.2 450.0,117.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.8 88.2,159.0 124.4,182.8 160.5,193.7 196.7,164.9 232.9,164.2 269.1,152.6 305.3,146.4 341.5,139.2 377.6,121.1 413.8,107.0 450.0,102.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.0 88.2,152.4 124.4,182.5 160.5,181.3 196.7,163.5 232.9,162.9 269.1,163.8 305.3,136.7 341.5,135.9 377.6,119.8 413.8,108.8 450.0,84.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.6 88.2,152.4 124.4,181.5 160.5,180.7 196.7,161.7 232.9,160.7 269.1,163.3 305.3,141.8 341.5,131.8 377.6,119.8 413.8,95.6 450.0,81.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,155.0 124.4,185.0 160.5,179.9 196.7,160.4 232.9,158.3 269.1,151.8 305.3,131.4 341.5,126.7 377.6,103.7 413.8,86.0 450.0,74.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 387 ns | 404 ns | 1.26 µs | 1.36 µs | 1.45 µs |
| D38 | 383 ns | 1.25 µs | 1.48 µs | 1.42 µs | 804 ns |
| D57 | 1.28 µs | 1.31 µs | 1.33 µs | 1.42 µs | 1.26 µs |
| D76 | 1.38 µs | 889 ns | 1.38 µs | 1.44 µs | 1.55 µs |
| D115 | 2.42 µs | 2.37 µs | 2.59 µs | 2.74 µs | 2.89 µs |
| D153 | 2.23 µs | 2.51 µs | 2.64 µs | 2.84 µs | 3.04 µs |
| D230 | 3.01 µs | 3.54 µs | 2.59 µs | 2.62 µs | 3.86 µs |
| D307 | 4.8 µs | 4.64 µs | 6.37 µs | 5.49 µs | 7.75 µs |
| D462 | 4.97 µs | 5.92 µs | 6.67 µs | 7.58 µs | 9.21 µs |
| D616 | 5.51 µs | 9.96 µs | 10.7 µs | 10.3 µs | 18 µs |
| D924 | 11.5 µs | 16.1 µs | 13.5 µs | 22.7 µs | 30.8 µs |
| D1232 | 12 µs | 19 µs | 31.4 µs | 34.5 µs | 45.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.8 88.2,171.2 124.4,136.3 160.5,134.1 196.7,117.8 232.9,120.2 269.1,111.5 305.3,97.9 341.5,96.9 377.6,93.9 413.8,72.7 450.0,71.3 450.0,32.9 413.8,44.1 377.6,59.7 341.5,79.0 305.3,84.0 269.1,104.2 232.9,111.2 196.7,112.6 160.5,130.7 124.4,136.7 88.2,149.6 52.0,132.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.8 88.2,171.2 124.4,136.3 160.5,134.1 196.7,117.8 232.9,120.2 269.1,111.5 305.3,97.9 341.5,96.9 377.6,93.9 413.8,72.7 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.6 88.2,137.0 124.4,135.6 160.5,146.7 196.7,118.4 232.9,116.7 269.1,106.7 305.3,98.9 341.5,91.9 377.6,76.8 413.8,62.8 450.0,58.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.7 88.2,132.0 124.4,135.1 160.5,134.1 196.7,115.8 232.9,115.2 269.1,115.8 305.3,89.7 341.5,88.4 377.6,74.7 413.8,68.1 450.0,43.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.4 88.2,133.1 124.4,133.2 160.5,132.7 196.7,114.1 232.9,113.1 269.1,115.5 305.3,94.0 341.5,84.7 377.6,75.7 413.8,52.9 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.6 88.2,149.6 124.4,136.7 160.5,130.7 196.7,112.6 232.9,111.2 269.1,104.2 305.3,84.0 341.5,79.0 377.6,59.7 413.8,44.1 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 350 ns | 363 ns | 1.24 µs | 1.32 µs | 1.41 µs |
| D38 | 340 ns | 1.23 µs | 1.45 µs | 1.41 µs | 779 ns |
| D57 | 936 ns | 1.01 µs | 1.29 µs | 1.33 µs | 1.12 µs |
| D76 | 1.07 µs | 716 ns | 1.3 µs | 1.38 µs | 1.38 µs |
| D115 | 1.93 µs | 2.18 µs | 2.31 µs | 2.48 µs | 2.52 µs |
| D153 | 1.76 µs | 2.12 µs | 2.3 µs | 2.48 µs | 2.66 µs |
| D230 | 2.36 µs | 2.99 µs | 2.07 µs | 2.12 µs | 3.26 µs |
| D307 | 3.79 µs | 3.94 µs | 5.26 µs | 4.54 µs | 6.68 µs |
| D462 | 4.15 µs | 5.1 µs | 5.86 µs | 6.59 µs | 8.15 µs |
| D616 | 4.21 µs | 8.18 µs | 9.11 µs | 8.98 µs | 16 µs |
| D924 | 8.94 µs | 13.8 µs | 11.5 µs | 19.8 µs | 28.1 µs |
| D1232 | 9.38 µs | 16.2 µs | 28.3 µs | 31.4 µs | 42.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,173.7 88.2,174.6 124.4,145.3 160.5,141.4 196.7,124.3 232.9,127.0 269.1,118.5 305.3,104.7 341.5,102.1 377.6,101.7 413.8,79.9 450.0,78.5 450.0,34.8 413.8,46.7 377.6,63.0 341.5,82.6 305.3,88.3 269.1,109.1 232.9,115.0 196.7,116.6 160.5,133.9 124.4,140.1 88.2,150.6 52.0,133.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,173.7 88.2,174.6 124.4,145.3 160.5,141.4 196.7,124.3 232.9,127.0 269.1,118.5 305.3,104.7 341.5,102.1 377.6,101.7 413.8,79.9 450.0,78.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,172.7 88.2,137.3 124.4,143.0 160.5,153.0 196.7,120.7 232.9,121.5 269.1,111.6 305.3,103.6 341.5,96.2 377.6,82.5 413.8,67.3 450.0,62.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.2 88.2,132.6 124.4,135.9 160.5,135.8 196.7,119.1 232.9,119.2 269.1,122.3 305.3,95.3 341.5,92.1 377.6,79.4 413.8,72.6 450.0,46.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,135.3 88.2,133.4 124.4,135.0 160.5,134.0 196.7,117.1 232.9,117.0 269.1,121.6 305.3,99.5 341.5,88.7 377.6,79.8 413.8,56.8 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.4 88.2,150.6 124.4,140.1 160.5,133.9 196.7,116.6 232.9,115.0 269.1,109.1 305.3,88.3 341.5,82.6 377.6,63.0 413.8,46.7 450.0,34.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.5 ns | 2.26 µs | 3.83 µs | 4.03 µs | 4.27 µs |
| D38 | 13.4 ns | 3.85 µs | 4.42 µs | 4.28 µs | 3.44 µs |
| D57 | 57.8 ns | 4.19 µs | 4.46 µs | 4.58 µs | 4.29 µs |
| D76 | 83.6 ns | 2.88 µs | 4.63 µs | 5.18 µs | 5.56 µs |
| D115 | 145 ns | 8.11 µs | 8.95 µs | 10.2 µs | 10.7 µs |
| D153 | 177 ns | 8.43 µs | 9.52 µs | 10.8 µs | 11.8 µs |
| D230 | 296 ns | 13.6 µs | 9.41 µs | 10.8 µs | 18 µs |
| D307 | 391 ns | 17.3 µs | 23 µs | 24.1 µs | 34.6 µs |
| D462 | 879 ns | 70.4 µs | 138 µs | 207 µs | 285 µs |
| D616 | 607 ns | 177 µs | 288 µs | 261 µs | 573 µs |
| D924 | 1.01 µs | 417 µs | 292 µs | 682 µs | 1.59 ms |
| D1232 | 1.15 µs | 595 µs | 794 µs | 1.79 ms | 2.64 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.7 88.2,205.8 124.4,184.6 160.5,179.3 196.7,171.3 232.9,168.4 269.1,161.0 305.3,156.9 341.5,145.2 377.6,150.6 413.8,143.2 450.0,141.3 450.0,29.3 413.8,36.6 377.6,51.4 341.5,61.5 305.3,92.0 269.1,101.5 232.9,107.6 196.7,109.0 160.5,118.5 124.4,122.2 88.2,125.4 52.0,122.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.7 88.2,205.8 124.4,184.6 160.5,179.3 196.7,171.3 232.9,168.4 269.1,161.0 305.3,156.9 341.5,145.2 377.6,150.6 413.8,143.2 450.0,141.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,131.5 88.2,123.8 124.4,122.6 160.5,128.0 196.7,113.0 232.9,112.5 269.1,105.5 305.3,102.1 341.5,81.8 377.6,68.4 413.8,56.0 450.0,50.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.9 88.2,121.8 124.4,121.7 160.5,121.2 196.7,111.6 232.9,110.7 269.1,110.9 305.3,97.9 341.5,72.0 377.6,61.3 413.8,61.1 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.2 88.2,122.3 124.4,121.3 160.5,119.5 196.7,109.7 232.9,108.9 269.1,108.8 305.3,97.3 341.5,66.1 377.6,62.8 413.8,48.9 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.3 88.2,125.4 124.4,122.2 160.5,118.5 196.7,109.0 232.9,107.6 269.1,101.5 305.3,92.0 341.5,61.5 377.6,51.4 413.8,36.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.68 ns | 13.5 ns | 18.1 ns | 28.9 ns | 28.9 ns |
| D38 | 6.96 ns | 21.5 ns | 40.1 ns | 118 ns | 127 ns |
| D57 | 80.6 ns | 140 ns | 205 ns | 358 ns | 317 ns |
| D76 | 84.5 ns | 110 ns | 290 ns | 372 ns | 621 ns |
| D115 | 98.2 ns | 237 ns | 392 ns | 766 ns | 955 ns |
| D153 | 113 ns | 318 ns | 626 ns | 951 ns | 1.54 µs |
| D230 | 136 ns | 454 ns | 641 ns | 1.16 µs | 2.47 µs |
| D307 | 156 ns | 654 ns | 1.69 µs | 2.51 µs | 5.17 µs |
| D462 | 217 ns | 1.03 µs | 2.64 µs | 5.81 µs | 7.58 µs |
| D616 | 223 ns | 1.83 µs | 4.35 µs | 5.45 µs | 13.5 µs |
| D924 | 249 ns | 2.71 µs | 4.7 µs | 14.3 µs | 24.3 µs |
| D1232 | 300 ns | 3.86 µs | 13.1 µs | 20.6 µs | 46.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.8 88.2,176.3 124.4,133.8 160.5,132.9 196.7,130.3 232.9,127.8 269.1,124.6 305.3,122.2 341.5,116.5 377.6,116.1 413.8,114.2 450.0,110.9 450.0,23.3 413.8,34.6 377.6,44.8 341.5,54.8 305.3,61.4 269.1,74.3 232.9,82.5 196.7,90.8 160.5,98.3 124.4,110.0 88.2,125.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.8 88.2,176.3 124.4,133.8 160.5,132.9 196.7,130.3 232.9,127.8 269.1,124.6 305.3,122.2 341.5,116.5 377.6,116.1 413.8,114.2 450.0,110.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.8 88.2,156.7 124.4,124.1 160.5,128.4 196.7,115.0 232.9,109.9 269.1,103.7 305.3,97.4 341.5,89.4 377.6,79.5 413.8,72.7 450.0,66.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.7 88.2,145.9 124.4,117.6 160.5,111.5 196.7,106.3 232.9,98.1 269.1,97.7 305.3,80.8 341.5,73.2 377.6,64.4 413.8,63.1 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,127.2 124.4,107.8 160.5,107.2 196.7,94.6 232.9,90.9 269.1,87.5 305.3,74.0 341.5,59.4 377.6,60.6 413.8,43.8 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,125.8 124.4,110.0 160.5,98.3 196.7,90.8 232.9,82.5 269.1,74.3 305.3,61.4 341.5,54.8 377.6,44.8 413.8,34.6 450.0,23.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
