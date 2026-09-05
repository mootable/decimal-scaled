# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 78 ns | 141 ns | 140 ns | 253 ns | 267 ns |
| D38 | 66.3 ns | 154 ns | 254 ns | 372 ns | 328 ns |
| D57 | 186 ns | 359 ns | 390 ns | 1.13 µs | 825 ns |
| D76 | 217 ns | 418 ns | 641 ns | 1.27 µs | 1.32 µs |
| D115 | 226 ns | 798 ns | 881 ns | 2.03 µs | 3.15 µs |
| D153 | 218 ns | 845 ns | 1.37 µs | 3.21 µs | 3.56 µs |
| D230 | 281 ns | 1.41 µs | 3.06 µs | 5.15 µs | 8.79 µs |
| D307 | 304 ns | 1.36 µs | 3.7 µs | 8.78 µs | 13.5 µs |
| D462 | 347 ns | 3.13 µs | 6.85 µs | 17 µs | 28.6 µs |
| D616 | 425 ns | 5.54 µs | 15.5 µs | 30.6 µs | 43.6 µs |
| D924 | 605 ns | 7.01 µs | 34.3 µs | 63.3 µs | 117 µs |
| D1232 | 706 ns | 15.4 µs | 52.7 µs | 119 µs | 123 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.3 88.2,177.1 124.4,159.2 160.5,156.5 196.7,155.8 232.9,156.4 269.1,152.0 305.3,150.7 341.5,148.4 377.6,144.9 413.8,138.7 450.0,136.0 450.0,46.4 413.8,47.3 377.6,64.4 341.5,71.7 305.3,84.7 269.1,92.2 232.9,107.9 196.7,110.1 160.5,125.2 124.4,133.3 88.2,149.3 52.0,152.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.3 88.2,177.1 124.4,159.2 160.5,156.5 196.7,155.8 232.9,156.4 269.1,152.0 305.3,150.7 341.5,148.4 377.6,144.9 413.8,138.7 450.0,136.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.1 88.2,162.4 124.4,147.8 160.5,145.1 196.7,133.9 232.9,132.9 269.1,124.1 305.3,124.7 341.5,110.2 377.6,100.3 413.8,96.2 450.0,82.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,164.1 88.2,153.8 124.4,146.4 160.5,137.7 196.7,132.2 232.9,124.5 269.1,110.6 305.3,107.3 341.5,96.6 377.6,82.3 413.8,68.6 450.0,61.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,147.2 124.4,127.9 160.5,125.9 196.7,117.7 232.9,109.7 269.1,101.5 305.3,92.3 341.5,80.7 377.6,70.6 413.8,58.0 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.9 88.2,149.3 124.4,133.3 160.5,125.2 196.7,110.1 232.9,107.9 269.1,92.2 305.3,84.7 341.5,71.7 377.6,64.4 413.8,47.3 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.48 µs | 1.79 µs | 2.74 µs | 3.43 µs | 4.07 µs |
| D38 | 1.26 µs | 3.19 µs | 3.79 µs | 4.26 µs | 3.6 µs |
| D57 | 3.23 µs | 3.3 µs | 2.46 µs | 5.64 µs | 7.79 µs |
| D76 | 5.46 µs | 4.92 µs | 5.12 µs | 9.24 µs | 9.72 µs |
| D115 | 5.72 µs | 7.03 µs | 8.48 µs | 18.2 µs | 23.9 µs |
| D153 | 3.45 µs | 7.65 µs | 11.9 µs | 24.4 µs | 30.8 µs |
| D230 | 5.29 µs | 13.3 µs | 22.4 µs | 42.6 µs | 79.8 µs |
| D307 | 5.22 µs | 11.5 µs | 30.4 µs | 79.1 µs | 116 µs |
| D462 | 4.85 µs | 22.3 µs | 58.6 µs | 156 µs | 222 µs |
| D616 | 4.27 µs | 45 µs | 140 µs | 268 µs | 349 µs |
| D924 | 6.08 µs | 58.1 µs | 287 µs | 551 µs | 898 µs |
| D1232 | 4.94 µs | 131 µs | 407 µs | 896 µs | 2.2 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.4 88.2,204.9 124.4,184.5 160.5,173.1 196.7,172.1 232.9,183.1 269.1,173.8 305.3,174.1 341.5,175.7 377.6,178.5 413.8,170.8 450.0,175.3 450.0,42.9 413.8,62.3 377.6,82.9 341.5,92.7 305.3,106.7 269.1,114.9 232.9,135.6 196.7,141.0 160.5,160.6 124.4,165.4 88.2,182.2 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.4 88.2,204.9 124.4,184.5 160.5,173.1 196.7,172.1 232.9,183.1 269.1,173.8 305.3,174.1 341.5,175.7 377.6,178.5 413.8,170.8 450.0,175.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,197.4 88.2,184.8 124.4,184.1 160.5,175.4 196.7,167.6 232.9,165.8 269.1,153.8 305.3,156.9 341.5,142.6 377.6,127.4 413.8,121.8 450.0,104.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.1 88.2,181.1 124.4,190.5 160.5,174.5 196.7,163.6 232.9,156.3 269.1,142.5 305.3,135.9 341.5,121.6 377.6,102.8 413.8,87.1 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.2 88.2,178.5 124.4,172.4 160.5,161.7 196.7,147.0 232.9,140.7 269.1,128.5 305.3,115.1 341.5,100.3 377.6,88.6 413.8,72.9 450.0,62.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,182.2 124.4,165.4 160.5,160.6 196.7,141.0 232.9,135.6 269.1,114.9 305.3,106.7 341.5,92.7 377.6,82.9 413.8,62.3 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 40.4 ns | 38.7 ns | 39.6 ns | 40.1 ns |
| D38 | 11.7 ns | 41.1 ns | 66.4 ns | 66.9 ns | 86.3 ns |
| D57 | 12.8 ns | 32.7 ns | 66.9 ns | 370 ns | 330 ns |
| D76 | 17.8 ns | 75 ns | 298 ns | 379 ns | 512 ns |
| D115 | 20.1 ns | 98.8 ns | 268 ns | 808 ns | 1.01 µs |
| D153 | 14.2 ns | 418 ns | 499 ns | 1.03 µs | 1.31 µs |
| D230 | 29.6 ns | 425 ns | 1.07 µs | 1.85 µs | 2.79 µs |
| D307 | 44.4 ns | 514 ns | 1.37 µs | 2.83 µs | 5 µs |
| D462 | 48 ns | 1.13 µs | 2.48 µs | 5.68 µs | 7.71 µs |
| D616 | 68.8 ns | 2.43 µs | 5.89 µs | 10.2 µs | 10.8 µs |
| D924 | 104 ns | 2.62 µs | 11 µs | 22.3 µs | 25.1 µs |
| D1232 | 85.3 ns | 5.59 µs | 18.7 µs | 23.8 µs | 34.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,206.7 124.4,204.6 160.5,197.5 196.7,194.8 232.9,202.4 269.1,186.5 305.3,177.6 341.5,175.9 377.6,168.1 413.8,159.2 450.0,163.5 450.0,33.1 413.8,40.0 377.6,58.3 341.5,65.7 305.3,75.0 269.1,87.7 232.9,104.2 196.7,109.8 160.5,124.5 124.4,134.1 88.2,163.2 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,206.7 124.4,204.6 160.5,197.5 196.7,194.8 232.9,202.4 269.1,186.5 305.3,177.6 341.5,175.9 377.6,168.1 413.8,159.2 450.0,163.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.7 88.2,179.3 124.4,184.3 160.5,166.3 196.7,160.3 232.9,128.9 269.1,128.6 305.3,124.4 341.5,107.4 377.6,90.7 413.8,89.1 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.6 88.2,168.9 124.4,168.7 160.5,136.3 196.7,138.6 232.9,125.1 269.1,108.6 305.3,103.1 341.5,90.3 377.6,71.5 413.8,57.9 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,168.7 124.4,131.6 160.5,131.1 196.7,114.6 232.9,109.4 269.1,96.6 305.3,87.4 341.5,72.3 377.6,59.5 413.8,42.5 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,163.2 124.4,134.1 160.5,124.5 196.7,109.8 232.9,104.2 269.1,87.7 305.3,75.0 341.5,65.7 377.6,58.3 413.8,40.0 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.05 ns | 463 ns | 370 ns | 436 ns | 486 ns |
| D38 | 9.62 ns | 454 ns | 463 ns | 480 ns | 374 ns |
| D57 | 223 ns | 407 ns | 300 ns | 493 ns | 557 ns |
| D76 | 295 ns | 403 ns | 360 ns | 634 ns | 558 ns |
| D115 | 307 ns | 529 ns | 355 ns | 947 ns | 1.11 µs |
| D153 | 198 ns | 574 ns | 449 ns | 1.15 µs | 1.18 µs |
| D230 | 519 ns | 678 ns | 1.02 µs | 1.28 µs | 1.8 µs |
| D307 | 673 ns | 485 ns | 1.07 µs | 1.33 µs | 10.5 µs |
| D462 | 1 µs | 1.24 µs | 1.17 µs | 1.87 µs | 2.47 µs |
| D616 | 1.15 µs | 1.82 µs | 2.13 µs | 2.89 µs | 3.33 µs |
| D924 | 2.09 µs | 1.53 µs | 3.08 µs | 3.65 µs | 4.65 µs |
| D1232 | 2.28 µs | 3.15 µs | 4.26 µs | 5.07 µs | 4.71 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,170.7 124.4,116.1 160.5,111.2 196.7,110.5 232.9,118.1 269.1,101.4 305.3,96.9 341.5,89.9 377.6,87.5 413.8,77.2 450.0,75.7 450.0,63.1 413.8,63.3 377.6,69.1 341.5,74.3 305.3,49.2 269.1,79.8 232.9,87.1 196.7,88.1 160.5,100.1 124.4,100.2 88.2,107.1 52.0,102.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,170.7 124.4,116.1 160.5,111.2 196.7,110.5 232.9,118.1 269.1,101.4 305.3,96.9 341.5,89.9 377.6,87.5 413.8,77.2 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.4 88.2,103.7 124.4,105.6 160.5,105.8 196.7,101.1 232.9,99.6 269.1,96.7 305.3,102.6 341.5,86.2 377.6,79.6 413.8,82.6 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,103.4 124.4,110.9 160.5,107.7 196.7,108.0 232.9,103.9 269.1,89.6 305.3,88.8 341.5,87.3 377.6,76.9 413.8,70.5 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,104.4 88.2,102.8 124.4,102.3 160.5,97.9 196.7,91.0 232.9,87.5 269.1,85.7 305.3,85.0 341.5,79.1 377.6,71.6 413.8,67.5 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,102.5 88.2,107.1 124.4,100.2 160.5,100.1 196.7,88.1 232.9,87.1 269.1,79.8 305.3,49.2 341.5,74.3 377.6,69.1 413.8,63.3 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.8 µs | 3.27 µs | 4.87 µs | 6.33 µs | 7.3 µs |
| D38 | 2.22 µs | 5.81 µs | 7.32 µs | 8.58 µs | 7.97 µs |
| D57 | 2.06 µs | 2.15 µs | 1.61 µs | 2.67 µs | 2.38 µs |
| D76 | 2.6 µs | 2.15 µs | 1.76 µs | 2.76 µs | 2.46 µs |
| D115 | 4.63 µs | 5.17 µs | 3.18 µs | 5.34 µs | 5.88 µs |
| D153 | 2.92 µs | 5.11 µs | 4.22 µs | 5.95 µs | 5.35 µs |
| D230 | 5.95 µs | 6.81 µs | 7.79 µs | 8.22 µs | 9.06 µs |
| D307 | 9.19 µs | 7.15 µs | 9.77 µs | 12.8 µs | 14 µs |
| D462 | 7.25 µs | 11.5 µs | 9.13 µs | 14.8 µs | 14.8 µs |
| D616 | 12.1 µs | 26.7 µs | 28.8 µs | 31.1 µs | 31.8 µs |
| D924 | 24.6 µs | 25.8 µs | 51.9 µs | 60.1 µs | 70.4 µs |
| D1232 | 27.4 µs | 54.5 µs | 75.4 µs | 97.3 µs | 78.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,165.3 88.2,175.3 124.4,178.6 160.5,168.5 196.7,143.4 232.9,163.4 269.1,132.5 305.3,113.7 341.5,124.0 377.6,101.9 413.8,71.0 450.0,66.3 450.0,20.7 413.8,25.3 377.6,59.8 341.5,93.1 305.3,95.3 269.1,114.3 232.9,137.2 196.7,133.1 160.5,170.8 124.4,172.4 88.2,119.9 52.0,123.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,165.3 88.2,175.3 124.4,178.6 160.5,168.5 196.7,143.4 232.9,163.4 269.1,132.5 305.3,113.7 341.5,124.0 377.6,101.9 413.8,71.0 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,158.5 88.2,133.6 124.4,176.8 160.5,176.8 196.7,138.7 232.9,139.2 269.1,126.7 305.3,124.6 341.5,103.8 377.6,67.3 413.8,68.9 450.0,36.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,141.3 88.2,123.6 124.4,189.3 160.5,185.6 196.7,159.7 232.9,147.5 269.1,120.8 305.3,111.0 341.5,113.9 377.6,64.1 413.8,38.5 450.0,22.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,129.9 88.2,116.6 124.4,167.4 160.5,165.9 196.7,137.2 232.9,132.6 269.1,118.5 305.3,99.3 341.5,93.0 377.6,60.7 413.8,32.1 450.0,11.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.7 88.2,119.9 124.4,172.4 160.5,170.8 196.7,133.1 232.9,137.2 269.1,114.3 305.3,95.3 341.5,93.1 377.6,59.8 413.8,25.3 450.0,20.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 388 ns | 405 ns | 786 ns | 1.36 µs | 1.59 µs |
| D38 | 298 ns | 1.25 µs | 1.48 µs | 1.65 µs | 1 µs |
| D57 | 1.07 µs | 1.1 µs | 727 ns | 1.41 µs | 1.24 µs |
| D76 | 1.34 µs | 1.1 µs | 887 ns | 1.47 µs | 1.31 µs |
| D115 | 2.37 µs | 2.58 µs | 1.67 µs | 2.77 µs | 3.11 µs |
| D153 | 1.53 µs | 2.64 µs | 2.21 µs | 3.1 µs | 2.79 µs |
| D230 | 3.01 µs | 3.37 µs | 3.97 µs | 4.23 µs | 4.69 µs |
| D307 | 4.76 µs | 3.76 µs | 5.01 µs | 6.45 µs | 7.14 µs |
| D462 | 3.83 µs | 5.8 µs | 4.7 µs | 7.65 µs | 7.93 µs |
| D616 | 6.52 µs | 12.6 µs | 13.2 µs | 14.1 µs | 14.4 µs |
| D924 | 12.1 µs | 12.2 µs | 21.7 µs | 25.9 µs | 30.8 µs |
| D1232 | 14.3 µs | 24.1 µs | 31.4 µs | 40.4 µs | 32.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.7 88.2,178.4 124.4,141.5 160.5,134.8 196.7,118.3 232.9,131.1 269.1,111.5 305.3,98.1 341.5,104.5 377.6,89.1 413.8,71.2 450.0,66.2 450.0,42.4 413.8,44.1 377.6,66.2 341.5,83.4 305.3,86.4 269.1,98.6 232.9,113.6 196.7,110.5 160.5,135.6 124.4,137.0 88.2,143.3 52.0,129.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.7 88.2,178.4 124.4,141.5 160.5,134.8 196.7,118.3 232.9,131.1 269.1,111.5 305.3,98.1 341.5,104.5 377.6,89.1 413.8,71.2 450.0,66.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,169.5 88.2,136.9 124.4,140.5 160.5,140.6 196.7,115.9 232.9,115.2 269.1,108.1 305.3,105.0 341.5,92.5 377.6,70.0 413.8,70.9 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.3 88.2,132.1 124.4,152.6 160.5,146.8 196.7,128.6 232.9,120.4 269.1,103.4 305.3,96.7 341.5,98.5 377.6,68.5 413.8,54.3 450.0,43.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,128.9 124.4,133.4 160.5,132.2 196.7,113.8 232.9,110.6 269.1,101.6 305.3,89.3 341.5,84.4 377.6,66.7 413.8,49.1 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,129.9 88.2,143.3 124.4,137.0 160.5,135.6 196.7,110.5 232.9,113.6 269.1,98.6 305.3,86.4 341.5,83.4 377.6,66.2 413.8,44.1 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 346 ns | 364 ns | 784 ns | 1.31 µs | 1.56 µs |
| D38 | 268 ns | 1.23 µs | 1.45 µs | 1.63 µs | 971 ns |
| D57 | 834 ns | 873 ns | 671 ns | 1.35 µs | 1.12 µs |
| D76 | 1.07 µs | 893 ns | 788 ns | 1.38 µs | 1.24 µs |
| D115 | 1.89 µs | 2.23 µs | 1.4 µs | 2.5 µs | 2.69 µs |
| D153 | 1.19 µs | 2.15 µs | 1.79 µs | 2.7 µs | 2.34 µs |
| D230 | 2.36 µs | 2.88 µs | 3.35 µs | 3.61 µs | 4.03 µs |
| D307 | 3.76 µs | 3.16 µs | 4.21 µs | 5.59 µs | 6.39 µs |
| D462 | 3.18 µs | 5.05 µs | 4.12 µs | 6.7 µs | 7.01 µs |
| D616 | 4.6 µs | 10.3 µs | 11.4 µs | 12.2 µs | 12.7 µs |
| D924 | 9.32 µs | 9.99 µs | 19 µs | 23 µs | 27.9 µs |
| D1232 | 10.9 µs | 20.8 µs | 28.4 µs | 36.8 µs | 30.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.1 88.2,181.4 124.4,148.6 160.5,141.5 196.7,124.9 232.9,138.3 269.1,118.5 305.3,105.0 341.5,109.8 377.6,99.1 413.8,78.7 450.0,74.3 450.0,44.6 413.8,47.0 377.6,69.7 341.5,86.9 305.3,89.6 269.1,103.0 232.9,118.8 196.7,114.7 160.5,137.1 124.4,139.9 88.2,144.2 52.0,130.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.1 88.2,181.4 124.4,148.6 160.5,141.5 196.7,124.9 232.9,138.3 269.1,118.5 305.3,105.0 341.5,109.8 377.6,99.1 413.8,78.7 450.0,74.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,172.6 88.2,137.2 124.4,147.3 160.5,146.6 196.7,120.1 232.9,121.2 269.1,112.7 305.3,110.0 341.5,96.5 377.6,75.7 413.8,76.7 450.0,55.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.4 88.2,132.6 124.4,154.9 160.5,150.2 196.7,133.7 232.9,126.5 269.1,108.4 305.3,101.7 341.5,102.3 377.6,72.8 413.8,58.1 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,135.4 88.2,129.2 124.4,134.6 160.5,134.0 196.7,116.8 232.9,114.6 269.1,106.1 305.3,93.5 341.5,88.3 377.6,70.9 413.8,52.6 450.0,39.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.5 88.2,144.2 124.4,139.9 160.5,137.1 196.7,114.7 232.9,118.8 269.1,103.0 305.3,89.6 341.5,86.9 377.6,69.7 413.8,47.0 450.0,44.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.5 ns | 2.24 µs | 3.36 µs | 4.04 µs | 4.78 µs |
| D38 | 10.1 ns | 3.84 µs | 4.43 µs | 4.88 µs | 4.21 µs |
| D57 | 62.8 ns | 3.55 µs | 2.48 µs | 4.61 µs | 4.3 µs |
| D76 | 82.4 ns | 3.46 µs | 3.25 µs | 5.2 µs | 4.8 µs |
| D115 | 145 ns | 8.58 µs | 6.19 µs | 10.3 µs | 11.5 µs |
| D153 | 124 ns | 9.08 µs | 7.37 µs | 11.7 µs | 10.5 µs |
| D230 | 301 ns | 12.7 µs | 15.3 µs | 18.5 µs | 21.4 µs |
| D307 | 385 ns | 13.8 µs | 18 µs | 28.3 µs | 32 µs |
| D462 | 584 ns | 70.2 µs | 101 µs | 206 µs | 255 µs |
| D616 | 682 ns | 224 µs | 350 µs | 356 µs | 447 µs |
| D924 | 1.04 µs | 322 µs | 484 µs | 783 µs | 1.59 ms |
| D1232 | 1.21 µs | 773 µs | 793 µs | 2.12 ms | 1.87 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.7 88.2,209.8 124.4,183.4 160.5,179.5 196.7,171.3 232.9,173.5 269.1,160.7 305.3,157.1 341.5,151.1 377.6,148.9 413.8,142.8 450.0,140.5 450.0,34.3 413.8,36.6 377.6,55.0 341.5,63.1 305.3,93.2 269.1,99.0 232.9,109.4 196.7,107.9 160.5,120.6 124.4,122.2 88.2,122.5 52.0,120.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.7 88.2,209.8 124.4,183.4 160.5,179.5 196.7,171.3 232.9,173.5 269.1,160.7 305.3,157.1 341.5,151.1 377.6,148.9 413.8,142.8 450.0,140.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,131.7 88.2,123.8 124.4,125.0 160.5,125.4 196.7,112.2 232.9,111.4 269.1,106.6 305.3,105.4 341.5,81.8 377.6,65.0 413.8,59.7 450.0,47.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.8 88.2,121.8 124.4,130.2 160.5,126.2 196.7,116.9 232.9,114.4 269.1,103.9 305.3,101.5 341.5,76.5 377.6,58.5 413.8,53.8 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.1 88.2,120.4 124.4,121.2 160.5,119.5 196.7,109.5 232.9,107.7 269.1,101.1 305.3,94.9 341.5,66.2 377.6,58.3 413.8,46.9 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.7 88.2,122.5 124.4,122.2 160.5,120.6 196.7,107.9 232.9,109.4 269.1,99.0 305.3,93.2 341.5,63.1 377.6,55.0 413.8,36.6 450.0,34.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.75 ns | 13.4 ns | 18.8 ns | 29.2 ns | 30 ns |
| D38 | 4.83 ns | 20.6 ns | 39.9 ns | 136 ns | 159 ns |
| D57 | 86 ns | 135 ns | 140 ns | 357 ns | 317 ns |
| D76 | 84.8 ns | 117 ns | 227 ns | 374 ns | 500 ns |
| D115 | 98.3 ns | 240 ns | 260 ns | 767 ns | 977 ns |
| D153 | 79.1 ns | 340 ns | 484 ns | 988 ns | 1.27 µs |
| D230 | 136 ns | 446 ns | 945 ns | 1.81 µs | 2.71 µs |
| D307 | 155 ns | 502 ns | 1.32 µs | 2.73 µs | 4.93 µs |
| D462 | 182 ns | 1.04 µs | 2.03 µs | 5.8 µs | 7.44 µs |
| D616 | 231 ns | 2.27 µs | 5.09 µs | 6.72 µs | 10.5 µs |
| D924 | 272 ns | 2.1 µs | 7.48 µs | 14.9 µs | 24.3 µs |
| D1232 | 303 ns | 4.82 µs | 13.1 µs | 24.3 µs | 34.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,182.6 124.4,132.6 160.5,132.9 196.7,130.3 232.9,134.1 269.1,124.7 305.3,122.4 341.5,119.6 377.6,115.5 413.8,112.6 450.0,110.7 450.0,28.6 413.8,34.6 377.6,49.1 341.5,55.1 305.3,62.3 269.1,72.7 232.9,85.9 196.7,90.4 160.5,102.1 124.4,109.9 88.2,121.9 52.0,150.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,182.6 124.4,132.6 160.5,132.9 196.7,130.3 232.9,134.1 269.1,124.7 305.3,122.4 341.5,119.6 377.6,115.5 413.8,112.6 450.0,110.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,157.5 124.4,124.8 160.5,127.3 196.7,114.8 232.9,108.7 269.1,104.0 305.3,102.0 341.5,89.4 377.6,75.7 413.8,77.1 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.1 88.2,146.0 124.4,124.1 160.5,115.8 196.7,113.4 232.9,102.6 269.1,91.0 305.3,85.1 341.5,77.7 377.6,61.7 413.8,55.0 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.4 88.2,124.6 124.4,107.9 160.5,107.1 196.7,94.6 232.9,90.2 269.1,79.6 305.3,72.5 341.5,59.4 377.6,56.9 413.8,43.1 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.9 88.2,121.9 124.4,109.9 160.5,102.1 196.7,90.4 232.9,85.9 269.1,72.7 305.3,62.3 341.5,55.1 377.6,49.1 413.8,34.6 450.0,28.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
