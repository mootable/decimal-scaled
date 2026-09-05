# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 77.1 ns | 131 ns | 150 ns | 255 ns | 256 ns |
| D38 | 79.3 ns | 158 ns | 229 ns | 350 ns | 398 ns |
| D57 | 187 ns | 421 ns | 707 ns | 1.15 µs | 1.07 µs |
| D76 | 131 ns | 488 ns | 798 ns | 1.04 µs | 1.6 µs |
| D115 | 224 ns | 769 ns | 1.05 µs | 2.07 µs | 3.17 µs |
| D153 | 222 ns | 854 ns | 1.77 µs | 3.13 µs | 4.59 µs |
| D230 | 278 ns | 1 µs | 3.24 µs | 5.09 µs | 8.7 µs |
| D307 | 329 ns | 1.59 µs | 4.67 µs | 8.91 µs | 12.7 µs |
| D462 | 391 ns | 3.38 µs | 9.05 µs | 18.5 µs | 26.1 µs |
| D616 | 496 ns | 3.76 µs | 14.5 µs | 30.6 µs | 50.3 µs |
| D924 | 473 ns | 9 µs | 30.6 µs | 69.9 µs | 130 µs |
| D1232 | 965 ns | 12.5 µs | 52.4 µs | 132 µs | 173 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.5 88.2,174.0 124.4,159.1 160.5,165.4 196.7,156.0 232.9,156.2 269.1,152.3 305.3,149.3 341.5,146.3 377.6,142.2 413.8,143.0 450.0,130.6 450.0,40.4 413.8,45.5 377.6,61.9 341.5,73.3 305.3,85.9 269.1,92.4 232.9,103.5 196.7,110.0 160.5,121.8 124.4,128.8 88.2,146.0 52.0,153.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.5 88.2,174.0 124.4,159.1 160.5,165.4 196.7,156.0 232.9,156.2 269.1,152.3 305.3,149.3 341.5,146.3 377.6,142.2 413.8,143.0 450.0,130.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.3 88.2,162.0 124.4,145.0 160.5,142.5 196.7,134.6 232.9,132.7 269.1,130.0 305.3,121.9 341.5,108.8 377.6,107.0 413.8,91.8 450.0,86.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.0 88.2,155.6 124.4,136.0 160.5,133.9 196.7,129.1 232.9,120.1 269.1,109.6 305.3,103.2 341.5,91.7 377.6,83.6 413.8,70.6 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,148.2 124.4,127.6 160.5,129.3 196.7,117.4 232.9,110.2 269.1,101.7 305.3,92.0 341.5,79.3 377.6,70.6 413.8,56.2 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.7 88.2,146.0 124.4,128.8 160.5,121.8 196.7,110.0 232.9,103.5 269.1,92.4 305.3,85.9 341.5,73.3 377.6,61.9 413.8,45.5 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.46 µs | 1.63 µs | 2.8 µs | 3.42 µs | 3.63 µs |
| D38 | 1.61 µs | 3.19 µs | 3.07 µs | 3.58 µs | 4.69 µs |
| D57 | 3.71 µs | 4.2 µs | 4.32 µs | 5.77 µs | 10.2 µs |
| D76 | 2.8 µs | 6.3 µs | 7.62 µs | 7.94 µs | 11.5 µs |
| D115 | 5.2 µs | 6.6 µs | 10.7 µs | 16.5 µs | 24.2 µs |
| D153 | 3.34 µs | 7.65 µs | 16.2 µs | 22.2 µs | 38 µs |
| D230 | 4.71 µs | 8.54 µs | 24.3 µs | 41.1 µs | 78.7 µs |
| D307 | 4.67 µs | 13.3 µs | 38.3 µs | 78.8 µs | 100 µs |
| D462 | 5.43 µs | 24.6 µs | 80.8 µs | 167 µs | 194 µs |
| D616 | 5.85 µs | 30.8 µs | 132 µs | 268 µs | 412 µs |
| D924 | 4.71 µs | 80.5 µs | 267 µs | 603 µs | 982 µs |
| D1232 | 6.58 µs | 110 µs | 407 µs | 991 µs | 2.19 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.7 88.2,199.6 124.4,181.5 160.5,187.7 196.7,174.2 232.9,183.8 269.1,176.4 305.3,176.6 341.5,173.3 377.6,171.7 413.8,176.4 450.0,169.1 450.0,42.9 413.8,60.4 377.6,79.3 341.5,95.6 305.3,109.9 269.1,115.2 232.9,131.0 196.7,140.8 160.5,157.0 124.4,159.5 88.2,176.4 52.0,182.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.7 88.2,199.6 124.4,181.5 160.5,187.7 196.7,174.2 232.9,183.8 269.1,176.4 305.3,176.6 341.5,173.3 377.6,171.7 413.8,176.4 450.0,169.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.5 88.2,184.8 124.4,178.8 160.5,170.0 196.7,169.0 232.9,165.8 269.1,163.4 305.3,153.8 341.5,140.5 377.6,135.6 413.8,114.7 450.0,107.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,187.6 88.2,185.7 124.4,178.2 160.5,165.9 196.7,158.5 232.9,149.5 269.1,140.7 305.3,130.8 341.5,114.6 377.6,104.0 413.8,88.7 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.3 88.2,182.3 124.4,171.9 160.5,165.0 196.7,149.1 232.9,142.7 269.1,129.3 305.3,115.2 341.5,98.9 377.6,88.6 413.8,71.0 450.0,60.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.0 88.2,176.4 124.4,159.5 160.5,157.0 196.7,140.8 232.9,131.0 269.1,115.2 305.3,109.9 341.5,95.6 377.6,79.3 413.8,60.4 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.1 ns | 40.5 ns | 39.5 ns | 40.8 ns | 40.8 ns |
| D38 | 15.4 ns | 41.4 ns | 59.6 ns | 66.3 ns | 95.2 ns |
| D57 | 16.8 ns | 42.2 ns | 82.9 ns | 370 ns | 420 ns |
| D76 | 10.5 ns | 96.5 ns | 397 ns | 322 ns | 618 ns |
| D115 | 22.7 ns | 84 ns | 316 ns | 793 ns | 1.01 µs |
| D153 | 13.5 ns | 418 ns | 683 ns | 973 ns | 1.75 µs |
| D230 | 22.4 ns | 284 ns | 1.13 µs | 1.85 µs | 2.78 µs |
| D307 | 33.3 ns | 604 ns | 1.74 µs | 2.82 µs | 4.87 µs |
| D462 | 62.8 ns | 1.23 µs | 3.15 µs | 6.1 µs | 6.09 µs |
| D616 | 85.4 ns | 1.65 µs | 5.71 µs | 10.3 µs | 13.1 µs |
| D924 | 73.9 ns | 3.25 µs | 10.3 µs | 24.4 µs | 26.9 µs |
| D1232 | 110 ns | 4.76 µs | 18.7 µs | 25.8 µs | 38.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.3 88.2,200.6 124.4,198.7 160.5,208.9 196.7,192.2 232.9,203.5 269.1,192.5 305.3,183.9 341.5,170.1 377.6,163.4 413.8,166.6 450.0,157.8 450.0,31.0 413.8,38.5 377.6,54.2 341.5,70.8 305.3,75.6 269.1,87.8 232.9,97.8 196.7,109.7 160.5,120.5 124.4,128.8 88.2,161.1 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.3 88.2,200.6 124.4,198.7 160.5,208.9 196.7,192.2 232.9,203.5 269.1,192.5 305.3,183.9 341.5,170.1 377.6,163.4 413.8,166.6 450.0,157.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,178.7 160.5,160.8 196.7,163.8 232.9,128.9 269.1,137.4 305.3,121.0 341.5,105.6 377.6,99.1 413.8,84.4 450.0,76.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.2 88.2,171.2 124.4,164.1 160.5,130.1 196.7,135.0 232.9,118.3 269.1,107.3 305.3,98.0 341.5,85.1 377.6,72.2 413.8,59.3 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,168.9 124.4,131.6 160.5,134.6 196.7,115.0 232.9,110.6 269.1,96.7 305.3,87.5 341.5,70.7 377.6,59.4 413.8,40.6 450.0,39.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,161.1 124.4,128.8 160.5,120.5 196.7,109.7 232.9,97.8 269.1,87.8 305.3,75.6 341.5,70.8 377.6,54.2 413.8,38.5 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.91 ns | 270 ns | 372 ns | 381 ns | 383 ns |
| D38 | 9.52 ns | 379 ns | 347 ns | 365 ns | 403 ns |
| D57 | 272 ns | 525 ns | 487 ns | 508 ns | 721 ns |
| D76 | 164 ns | 510 ns | 509 ns | 536 ns | 621 ns |
| D115 | 287 ns | 506 ns | 405 ns | 828 ns | 1.13 µs |
| D153 | 195 ns | 560 ns | 789 ns | 1.02 µs | 1.4 µs |
| D230 | 440 ns | 378 ns | 1.12 µs | 1.3 µs | 1.79 µs |
| D307 | 580 ns | 557 ns | 1.22 µs | 1.31 µs | 8.97 µs |
| D462 | 1.19 µs | 3.21 µs | 3.39 µs | 4.37 µs | 4.26 µs |
| D616 | 1.63 µs | 1.42 µs | 1.96 µs | 2.94 µs | 3.66 µs |
| D924 | 1.64 µs | 2.02 µs | 2.88 µs | 3.98 µs | 5.01 µs |
| D1232 | 3.21 µs | 2.65 µs | 4.28 µs | 5.42 µs | 5.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.4 88.2,161.1 124.4,88.2 160.5,99.3 196.7,87.1 232.9,95.5 269.1,77.8 305.3,71.8 341.5,56.2 377.6,49.3 413.8,49.3 450.0,34.7 450.0,21.8 413.8,25.0 377.6,31.9 341.5,28.6 305.3,12.4 269.1,47.3 232.9,52.7 196.7,57.3 160.5,70.4 124.4,67.1 88.2,79.7 52.0,80.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.4 88.2,161.1 124.4,88.2 160.5,99.3 196.7,87.1 232.9,95.5 269.1,77.8 305.3,71.8 341.5,56.2 377.6,49.3 413.8,49.3 450.0,34.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,88.4 88.2,81.1 124.4,74.0 160.5,74.6 196.7,74.8 232.9,72.6 269.1,81.1 305.3,72.7 341.5,34.7 377.6,52.4 413.8,44.7 450.0,38.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.5 88.2,83.0 124.4,75.6 160.5,74.7 196.7,79.6 232.9,65.2 269.1,57.5 305.3,55.6 341.5,33.5 377.6,45.4 413.8,37.1 450.0,28.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.9 88.2,81.9 124.4,74.7 160.5,73.5 196.7,64.1 232.9,59.5 269.1,54.3 305.3,54.1 341.5,28.0 377.6,36.6 413.8,30.0 450.0,23.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.8 88.2,79.7 124.4,67.1 160.5,70.4 196.7,57.3 232.9,52.7 269.1,47.3 305.3,12.4 341.5,28.6 377.6,31.9 413.8,25.0 450.0,21.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.75 µs | 3.12 µs | 5.25 µs | 6.36 µs | 6.98 µs |
| D38 | 2.9 µs | 5.81 µs | 5.82 µs | 7.17 µs | 9.98 µs |
| D57 | 2.45 µs | 2.72 µs | 2.59 µs | 2.67 µs | 3.03 µs |
| D76 | 1.35 µs | 2.8 µs | 2.83 µs | 2.32 µs | 2.86 µs |
| D115 | 4.35 µs | 4.77 µs | 3.75 µs | 4.7 µs | 5.94 µs |
| D153 | 2.82 µs | 5.19 µs | 5.5 µs | 5.45 µs | 6.41 µs |
| D230 | 5.3 µs | 4.32 µs | 8.14 µs | 7.98 µs | 9 µs |
| D307 | 7.42 µs | 8.39 µs | 12.7 µs | 12.9 µs | 12 µs |
| D462 | 9.34 µs | 12.3 µs | 13.1 µs | 16 µs | 14.1 µs |
| D616 | 16.3 µs | 18.4 µs | 26.5 µs | 31.1 µs | 36.1 µs |
| D924 | 19 µs | 35.1 µs | 47.4 µs | 64.5 µs | 77.7 µs |
| D1232 | 33.4 µs | 45.9 µs | 75.2 µs | 106 µs | 101 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.7 88.2,179.1 124.4,184.0 160.5,201.2 196.7,167.4 232.9,180.0 269.1,161.7 305.3,152.0 341.5,145.3 377.6,129.1 413.8,124.8 450.0,108.4 450.0,76.3 413.8,84.0 377.6,106.2 341.5,133.5 305.3,138.1 269.1,146.4 232.9,156.2 196.7,158.4 160.5,179.5 124.4,177.9 88.2,143.4 52.0,153.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.7 88.2,179.1 124.4,184.0 160.5,201.2 196.7,167.4 232.9,180.0 269.1,161.7 305.3,152.0 341.5,145.3 377.6,129.1 413.8,124.8 450.0,108.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.0 88.2,159.0 124.4,181.1 160.5,180.2 196.7,164.8 232.9,162.3 269.1,167.6 305.3,148.4 341.5,137.3 377.6,125.7 413.8,107.0 450.0,99.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.0 88.2,159.0 124.4,182.5 160.5,179.9 196.7,171.7 232.9,160.7 269.1,149.3 305.3,136.3 341.5,135.5 377.6,115.1 413.8,98.3 450.0,84.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.4 88.2,153.0 124.4,181.5 160.5,185.6 196.7,165.2 232.9,160.9 269.1,149.9 305.3,135.9 341.5,129.7 377.6,110.5 413.8,89.4 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,143.4 124.4,177.9 160.5,179.5 196.7,158.4 232.9,156.2 269.1,146.4 305.3,138.1 341.5,133.5 377.6,106.2 413.8,84.0 450.0,76.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 379 ns | 384 ns | 831 ns | 1.38 µs | 1.46 µs |
| D38 | 404 ns | 1.24 µs | 879 ns | 997 ns | 1.87 µs |
| D57 | 1.29 µs | 1.43 µs | 1.31 µs | 1.48 µs | 1.61 µs |
| D76 | 687 ns | 1.47 µs | 1.49 µs | 1.26 µs | 1.53 µs |
| D115 | 2.26 µs | 2.45 µs | 2 µs | 2.48 µs | 3.08 µs |
| D153 | 1.47 µs | 2.8 µs | 2.88 µs | 2.87 µs | 3.36 µs |
| D230 | 2.82 µs | 2.18 µs | 4.24 µs | 4.1 µs | 4.64 µs |
| D307 | 3.91 µs | 4.34 µs | 6.25 µs | 6.36 µs | 6.23 µs |
| D462 | 5.01 µs | 6.21 µs | 6.62 µs | 8.18 µs | 7.32 µs |
| D616 | 8.28 µs | 8.57 µs | 12.1 µs | 14 µs | 16.6 µs |
| D924 | 9.72 µs | 15.8 µs | 20.2 µs | 28 µs | 33.3 µs |
| D1232 | 16.6 µs | 20.5 µs | 31.7 µs | 43.7 µs | 42 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,171.4 88.2,169.6 124.4,136.0 160.5,154.2 196.7,119.7 232.9,132.1 269.1,113.3 305.3,103.9 341.5,96.7 377.6,82.1 413.8,77.5 450.0,62.0 450.0,35.1 413.8,41.9 377.6,62.0 341.5,85.7 305.3,90.4 269.1,98.9 232.9,108.2 196.7,110.7 160.5,131.1 124.4,129.5 88.2,125.3 52.0,132.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,171.4 88.2,169.6 124.4,136.0 160.5,154.2 196.7,119.7 232.9,132.1 269.1,113.3 305.3,103.9 341.5,96.7 377.6,82.1 413.8,77.5 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.0 88.2,137.0 124.4,133.0 160.5,132.3 196.7,117.4 232.9,113.5 269.1,120.7 305.3,100.8 341.5,90.5 377.6,81.1 413.8,63.4 450.0,55.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,148.7 88.2,147.1 124.4,135.6 160.5,131.7 196.7,123.2 232.9,112.7 269.1,101.5 305.3,90.3 341.5,88.6 377.6,71.0 413.8,56.3 450.0,43.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.1 88.2,143.4 124.4,131.9 160.5,136.7 196.7,117.0 232.9,112.8 269.1,102.5 305.3,89.8 341.5,82.5 377.6,66.9 413.8,46.9 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.4 88.2,125.3 124.4,129.5 160.5,131.1 196.7,110.7 232.9,108.2 269.1,98.9 305.3,90.4 341.5,85.7 377.6,62.0 413.8,41.9 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 338 ns | 338 ns | 806 ns | 1.32 µs | 1.41 µs |
| D38 | 364 ns | 1.23 µs | 849 ns | 981 ns | 1.84 µs |
| D57 | 921 ns | 1.13 µs | 1.25 µs | 1.34 µs | 1.44 µs |
| D76 | 497 ns | 1.15 µs | 1.29 µs | 1.11 µs | 1.41 µs |
| D115 | 1.86 µs | 2.25 µs | 1.69 µs | 2.14 µs | 2.69 µs |
| D153 | 1.13 µs | 2.23 µs | 2.38 µs | 2.48 µs | 2.82 µs |
| D230 | 2.2 µs | 1.79 µs | 3.47 µs | 3.5 µs | 3.99 µs |
| D307 | 3.15 µs | 3.56 µs | 5.46 µs | 5.54 µs | 5.42 µs |
| D462 | 4 µs | 5.22 µs | 5.73 µs | 7 µs | 6.52 µs |
| D616 | 6.47 µs | 7.1 µs | 10.5 µs | 12.2 µs | 14.6 µs |
| D924 | 6.99 µs | 13.4 µs | 17.8 µs | 24.6 µs | 30.3 µs |
| D1232 | 12.7 µs | 17.8 µs | 28.3 µs | 39.7 µs | 39.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.7 88.2,172.6 124.4,145.7 160.5,163.6 196.7,125.4 232.9,139.8 269.1,120.4 305.3,110.1 341.5,103.2 377.6,89.3 413.8,87.0 450.0,69.7 450.0,37.1 413.8,44.6 377.6,65.7 341.5,89.0 305.3,94.4 269.1,103.2 232.9,113.3 196.7,114.6 160.5,133.3 124.4,132.8 88.2,125.6 52.0,133.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.7 88.2,172.6 124.4,145.7 160.5,163.6 196.7,125.4 232.9,139.8 269.1,120.4 305.3,110.1 341.5,103.2 377.6,89.3 413.8,87.0 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.7 88.2,137.4 124.4,139.7 160.5,139.4 196.7,119.9 232.9,120.1 269.1,126.4 305.3,106.5 341.5,95.5 377.6,86.6 413.8,68.2 450.0,59.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,149.6 88.2,148.1 124.4,136.8 160.5,135.9 196.7,128.1 232.9,118.2 269.1,107.3 305.3,94.2 341.5,92.8 377.6,75.3 413.8,60.0 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,135.2 88.2,143.9 124.4,134.8 160.5,140.4 196.7,121.3 232.9,117.0 269.1,107.1 305.3,93.8 341.5,87.0 377.6,70.8 413.8,50.6 450.0,36.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.4 88.2,125.6 124.4,132.8 160.5,133.3 196.7,114.6 232.9,113.3 269.1,103.2 305.3,94.4 341.5,89.0 377.6,65.7 413.8,44.6 450.0,37.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 2.05 µs | 3.42 µs | 4.04 µs | 4.26 µs |
| D38 | 13.1 ns | 3.83 µs | 3.54 µs | 4.19 µs | 5.35 µs |
| D57 | 57.3 ns | 4.57 µs | 4.48 µs | 4.68 µs | 5.58 µs |
| D76 | 54.4 ns | 4.49 µs | 5.04 µs | 4.33 µs | 5.62 µs |
| D115 | 134 ns | 8.15 µs | 6.57 µs | 9.31 µs | 11.5 µs |
| D153 | 118 ns | 8.95 µs | 10.3 µs | 10.9 µs | 13 µs |
| D230 | 276 ns | 8.18 µs | 16.4 µs | 18.2 µs | 21.2 µs |
| D307 | 364 ns | 15.7 µs | 23 µs | 28.4 µs | 28.1 µs |
| D462 | 664 ns | 78.5 µs | 138 µs | 223 µs | 224 µs |
| D616 | 867 ns | 151 µs | 328 µs | 356 µs | 531 µs |
| D924 | 803 ns | 423 µs | 456 µs | 845 µs | 1.74 ms |
| D1232 | 1.82 µs | 652 µs | 794 µs | 2.31 ms | 2.32 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.5 88.2,206.1 124.4,184.7 160.5,185.5 196.7,172.4 232.9,174.3 269.1,161.9 305.3,158.0 341.5,149.3 377.6,145.4 413.8,146.5 450.0,134.7 450.0,31.2 413.8,35.3 377.6,52.5 341.5,65.0 305.3,95.0 269.1,99.1 232.9,106.2 196.7,108.0 160.5,118.3 124.4,118.5 88.2,119.0 52.0,122.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.5 88.2,206.1 124.4,184.7 160.5,185.5 196.7,172.4 232.9,174.3 269.1,161.9 305.3,158.0 341.5,149.3 377.6,145.4 413.8,146.5 450.0,134.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,123.9 124.4,121.3 160.5,121.6 196.7,113.0 232.9,111.6 269.1,112.9 305.3,103.4 341.5,80.2 377.6,70.7 413.8,55.8 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,125.5 88.2,125.0 124.4,121.6 160.5,119.9 196.7,116.1 232.9,109.6 269.1,102.8 305.3,97.9 341.5,72.0 377.6,59.5 413.8,54.7 450.0,46.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.1 88.2,122.6 124.4,121.0 160.5,122.1 196.7,111.0 232.9,108.7 269.1,101.3 305.3,94.9 341.5,65.0 377.6,58.3 413.8,45.8 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.3 88.2,119.0 124.4,118.5 160.5,118.3 196.7,108.0 232.9,106.2 269.1,99.1 305.3,95.0 341.5,65.0 377.6,52.5 413.8,35.3 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.75 ns | 13.3 ns | 19.9 ns | 29.6 ns | 29.4 ns |
| D38 | 7.21 ns | 21.5 ns | 29.4 ns | 127 ns | 194 ns |
| D57 | 79.5 ns | 149 ns | 206 ns | 358 ns | 397 ns |
| D76 | 55.6 ns | 145 ns | 309 ns | 320 ns | 616 ns |
| D115 | 104 ns | 225 ns | 301 ns | 758 ns | 978 ns |
| D153 | 77.5 ns | 342 ns | 659 ns | 941 ns | 1.65 µs |
| D230 | 131 ns | 274 ns | 1.02 µs | 1.82 µs | 2.7 µs |
| D307 | 160 ns | 579 ns | 1.7 µs | 2.73 µs | 4.66 µs |
| D462 | 206 ns | 1.11 µs | 2.67 µs | 6.35 µs | 5.94 µs |
| D616 | 263 ns | 1.5 µs | 4.64 µs | 6.72 µs | 12.5 µs |
| D924 | 202 ns | 2.7 µs | 6.62 µs | 16.2 µs | 26.2 µs |
| D1232 | 430 ns | 4.04 µs | 13.4 µs | 26.3 µs | 37.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,175.7 124.4,134.0 160.5,140.2 196.7,129.3 232.9,134.4 269.1,125.3 305.3,121.9 341.5,117.5 377.6,113.2 413.8,117.8 450.0,104.7 450.0,27.1 413.8,33.3 377.6,46.1 341.5,59.0 305.3,63.3 269.1,72.8 232.9,81.3 196.7,90.4 160.5,98.4 124.4,106.0 88.2,118.5 52.0,151.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,175.7 124.4,134.0 160.5,140.2 196.7,129.3 232.9,134.4 269.1,125.3 305.3,121.9 341.5,117.5 377.6,113.2 413.8,117.8 450.0,104.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,156.7 124.4,123.1 160.5,123.5 196.7,115.9 232.9,108.6 269.1,112.5 305.3,99.5 341.5,88.2 377.6,83.0 413.8,72.7 450.0,65.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.1 88.2,151.2 124.4,117.5 160.5,110.4 196.7,110.8 232.9,97.2 269.1,89.7 305.3,80.8 341.5,73.0 377.6,63.3 413.8,57.2 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.2 88.2,125.9 124.4,107.9 160.5,109.8 196.7,94.8 232.9,91.1 269.1,79.6 305.3,72.6 341.5,57.9 377.6,56.9 413.8,41.6 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,118.5 124.4,106.0 160.5,98.4 196.7,90.4 232.9,81.3 269.1,72.8 305.3,63.3 341.5,59.0 377.6,46.1 413.8,33.3 450.0,27.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
