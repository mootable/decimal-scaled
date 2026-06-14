# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.07 µs | 5.13 µs | 5.15 µs | 8.69 µs | 8.7 µs |
| D38 | 5.73 µs | 2.43 µs | 7.71 µs | 9.67 µs | 8.45 µs |
| D57 | 329 ns | 595 ns | 660 ns | 1.04 µs | 1.16 µs |
| D76 | 463 ns | 760 ns | 1.19 µs | 1.36 µs | 1.94 µs |
| D115 | 334 ns | 2.09 µs | 2.5 µs | 3.54 µs | 5.28 µs |
| D153 | 528 ns | 2.59 µs | 3.57 µs | 5.75 µs | 6.94 µs |
| D230 | 548 ns | 4.1 µs | 8.55 µs | 8.66 µs | 14.1 µs |
| D307 | 563 ns | 6.15 µs | 10.6 µs | 16.7 µs | 22.4 µs |
| D462 | 482 ns | 10.5 µs | 24.2 µs | 33.5 µs | 47.2 µs |
| D616 | 673 ns | 12.7 µs | 40.5 µs | 57.8 µs | 84.9 µs |
| D924 | 707 ns | 32.4 µs | 89.6 µs | 108 µs | 198 µs |
| D1232 | 1.22 µs | 48.6 µs | 150 µs | 227 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,122.1 124.4,184.2 160.5,176.7 196.7,183.8 232.9,173.9 269.1,173.1 305.3,172.5 341.5,175.9 377.6,168.6 413.8,167.5 450.0,155.7 450.0,33.1 413.8,45.1 377.6,63.6 341.5,76.3 305.3,92.4 269.1,102.6 232.9,117.9 196.7,123.9 160.5,145.6 124.4,156.8 88.2,113.7 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,122.1 124.4,184.2 160.5,176.7 196.7,183.8 232.9,173.9 269.1,173.1 305.3,172.5 341.5,175.9 377.6,168.6 413.8,167.5 450.0,155.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,140.7 124.4,171.3 160.5,166.0 196.7,144.0 232.9,139.4 269.1,129.4 305.3,120.6 341.5,109.0 377.6,104.7 413.8,84.5 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,115.7 124.4,169.0 160.5,156.2 196.7,140.1 232.9,132.4 269.1,113.4 305.3,108.7 341.5,90.9 377.6,79.6 413.8,62.4 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.1 88.2,110.7 124.4,159.2 160.5,153.3 196.7,132.5 232.9,122.0 269.1,113.1 305.3,98.9 341.5,83.8 377.6,71.9 413.8,58.2 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,113.7 124.4,156.8 160.5,145.6 196.7,123.9 232.9,117.9 269.1,102.6 305.3,92.4 341.5,76.3 377.6,63.6 413.8,45.1 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.41 µs | 6.24 µs | 7.3 µs | 7.88 µs |
| D38 | 2.11 ns | 4.02 µs | 7.57 µs | 8.73 µs | 8.32 µs |
| D57 | 2.49 ns | 3.52 µs | 4.53 µs | 5.96 µs | 8.68 µs |
| D76 | 3.12 ns | 6.03 µs | 6.68 µs | 8.93 µs | 10.9 µs |
| D115 | 17 ns | 6.78 µs | 13.8 µs | 17.4 µs | 23.7 µs |
| D153 | 22.4 ns | 7.34 µs | 15.9 µs | 21.5 µs | 37.5 µs |
| D230 | 51.4 ns | 13.4 µs | 23.3 µs | 38.7 µs | 76.8 µs |
| D307 | 84.1 ns | 14.9 µs | 32.5 µs | 77.5 µs | 121 µs |
| D462 | 171 ns | 23.4 µs | 79.1 µs | 153 µs | 227 µs |
| D616 | 158 ns | 32.2 µs | 138 µs | 264 µs | 440 µs |
| D924 | 183 ns | 80.3 µs | 284 µs | 537 µs | 984 µs |
| D1232 | 412 ns | 138 µs | 440 µs | 903 µs | 2.83 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,200.7 124.4,198.7 160.5,195.9 196.7,174.9 232.9,171.4 269.1,161.1 305.3,155.0 341.5,146.2 377.6,147.1 413.8,145.3 450.0,135.3 450.0,25.6 413.8,38.8 377.6,48.8 341.5,56.9 305.3,64.8 269.1,70.4 232.9,79.3 196.7,85.0 160.5,94.6 124.4,97.5 88.2,98.0 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,200.7 124.4,198.7 160.5,195.9 196.7,174.9 232.9,171.4 269.1,161.1 305.3,155.0 341.5,146.2 377.6,147.1 413.8,145.3 450.0,135.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,107.0 124.4,108.7 160.5,102.0 196.7,100.5 232.9,99.6 269.1,92.0 305.3,90.7 341.5,85.2 377.6,81.2 413.8,69.9 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.6 88.2,99.2 124.4,105.5 160.5,100.7 196.7,91.7 232.9,90.0 269.1,85.2 305.3,81.1 341.5,70.1 377.6,63.2 413.8,54.2 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,97.4 124.4,102.1 160.5,97.1 196.7,88.8 232.9,86.2 269.1,78.9 305.3,70.3 341.5,61.9 377.6,55.1 413.8,46.3 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,98.0 124.4,97.5 160.5,94.6 196.7,85.0 232.9,79.3 269.1,70.4 305.3,64.8 341.5,56.9 377.6,48.8 413.8,38.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 40.2 ns | 42.4 ns | 40.3 ns | 40.1 ns |
| D38 | 15.4 ns | 37.5 ns | 67 ns | 67.1 ns | 83.6 ns |
| D57 | 16.7 ns | 40 ns | 72.9 ns | 605 ns | 702 ns |
| D76 | 17.9 ns | 74.8 ns | 692 ns | 698 ns | 1.04 µs |
| D115 | 20.5 ns | 79.3 ns | 617 ns | 1.22 µs | 1.26 µs |
| D153 | 25.2 ns | 667 ns | 916 ns | 1.4 µs | 2.06 µs |
| D230 | 29.2 ns | 643 ns | 1.37 µs | 2.05 µs | 3.29 µs |
| D307 | 41.7 ns | 1.1 µs | 1.8 µs | 3.32 µs | 5.6 µs |
| D462 | 85 ns | 1.43 µs | 3.62 µs | 6.26 µs | 9.59 µs |
| D616 | 68.8 ns | 2.08 µs | 6.16 µs | 10.8 µs | 15.4 µs |
| D924 | 103 ns | 3.79 µs | 11.3 µs | 21.6 µs | 28.6 µs |
| D1232 | 107 ns | 6.24 µs | 20.7 µs | 27.5 µs | 50.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,200.6 124.4,198.8 160.5,197.4 196.7,194.4 232.9,189.9 269.1,186.7 305.3,179.0 341.5,163.5 377.6,168.1 413.8,159.3 450.0,158.5 450.0,24.7 413.8,37.2 377.6,50.6 341.5,60.9 305.3,72.6 269.1,84.1 232.9,94.3 196.7,105.0 160.5,109.1 124.4,117.7 88.2,163.9 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,200.6 124.4,198.8 160.5,197.4 196.7,194.4 232.9,189.9 269.1,186.7 305.3,179.0 341.5,163.5 377.6,168.1 413.8,159.3 450.0,158.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.8 88.2,181.3 124.4,179.9 160.5,166.3 196.7,165.0 232.9,118.8 269.1,119.6 305.3,108.0 341.5,102.2 377.6,94.1 413.8,81.1 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.6 88.2,168.7 124.4,166.9 160.5,118.0 196.7,120.5 232.9,111.9 269.1,103.2 305.3,97.3 341.5,82.1 377.6,70.5 413.8,57.3 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,168.6 124.4,120.9 160.5,117.8 196.7,105.6 232.9,102.7 269.1,94.4 305.3,83.9 341.5,70.2 377.6,58.4 413.8,43.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,163.9 124.4,117.7 160.5,109.1 196.7,105.0 232.9,94.3 269.1,84.1 305.3,72.6 341.5,60.9 377.6,50.6 413.8,37.2 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 278 ns | 374 ns | 396 ns | 400 ns |
| D38 | 2.11 ns | 348 ns | 369 ns | 368 ns | 315 ns |
| D57 | 260 ns | 438 ns | 466 ns | 482 ns | 555 ns |
| D76 | 270 ns | 482 ns | 435 ns | 554 ns | 562 ns |
| D115 | 312 ns | 478 ns | 649 ns | 913 ns | 1.11 µs |
| D153 | 293 ns | 552 ns | 683 ns | 958 ns | 1.35 µs |
| D230 | 474 ns | 709 ns | 1.09 µs | 1.08 µs | 1.62 µs |
| D307 | 643 ns | 649 ns | 1.12 µs | 1.26 µs | 10 µs |
| D462 | 1.34 µs | 3.08 µs | 3.3 µs | 3.74 µs | 4.78 µs |
| D616 | 1.34 µs | 1.2 µs | 1.8 µs | 2.61 µs | 3.58 µs |
| D924 | 1.8 µs | 1.83 µs | 2.95 µs | 2.74 µs | 4.7 µs |
| D1232 | 2.93 µs | 3 µs | 4.05 µs | 4.58 µs | 6.49 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,197.0 124.4,113.4 160.5,112.7 196.7,110.3 232.9,111.3 269.1,103.0 305.3,97.7 341.5,84.9 377.6,84.9 413.8,79.8 450.0,71.3 450.0,57.5 413.8,63.1 377.6,67.8 341.5,62.8 305.3,50.0 269.1,81.6 232.9,84.7 196.7,88.2 160.5,100.0 124.4,100.2 88.2,110.1 52.0,105.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,197.0 124.4,113.4 160.5,112.7 196.7,110.3 232.9,111.3 269.1,103.0 305.3,97.7 341.5,84.9 377.6,84.9 413.8,79.8 450.0,71.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,108.4 124.4,104.4 160.5,102.7 196.7,102.8 232.9,100.3 269.1,96.0 305.3,97.5 341.5,70.5 377.6,86.8 413.8,79.5 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,107.3 124.4,103.3 160.5,104.5 196.7,97.5 232.9,96.6 269.1,88.5 305.3,88.1 341.5,69.3 377.6,79.8 413.8,71.2 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,107.4 124.4,102.7 160.5,100.3 196.7,91.6 232.9,90.8 269.1,88.7 305.3,86.0 341.5,67.1 377.6,73.4 413.8,72.5 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,110.1 124.4,100.2 160.5,100.0 196.7,88.2 232.9,84.7 269.1,81.6 305.3,50.0 341.5,62.8 377.6,67.8 413.8,63.1 450.0,57.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.7 µs | 8.72 µs | 11.6 µs | 13.6 µs | 14.9 µs |
| D38 | 8.15 µs | 7.4 µs | 14.5 µs | 17.1 µs | 16.4 µs |
| D57 | 4.02 µs | 4.1 µs | 4.93 µs | 4.9 µs | 4.62 µs |
| D76 | 3.93 µs | 4.91 µs | 4.34 µs | 4.55 µs | 4.68 µs |
| D115 | 8.23 µs | 8.9 µs | 9.35 µs | 9.28 µs | 10.6 µs |
| D153 | 7.53 µs | 9.1 µs | 9.98 µs | 9.83 µs | 11.6 µs |
| D230 | 10.4 µs | 12.9 µs | 14.6 µs | 12.8 µs | 16.7 µs |
| D307 | 15.9 µs | 18.2 µs | 19.5 µs | 23.8 µs | 28 µs |
| D462 | 16.7 µs | 22 µs | 23.5 µs | 27.4 µs | 31 µs |
| D616 | 27.7 µs | 34.7 µs | 55.9 µs | 60.7 µs | 79.2 µs |
| D924 | 40.2 µs | 68.2 µs | 103 µs | 110 µs | 162 µs |
| D1232 | 54 µs | 113 µs | 166 µs | 202 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,149.3 124.4,169.7 160.5,170.3 196.7,149.0 232.9,151.5 269.1,142.1 305.3,129.8 341.5,128.5 377.6,113.8 413.8,103.0 450.0,94.5 450.0,48.0 413.8,62.8 377.6,83.4 341.5,110.5 305.3,113.6 269.1,128.6 232.9,139.0 196.7,141.6 160.5,165.3 124.4,165.7 88.2,129.0 52.0,131.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,149.3 124.4,169.7 160.5,170.3 196.7,149.0 232.9,151.5 269.1,142.1 305.3,129.8 341.5,128.5 377.6,113.8 413.8,103.0 450.0,94.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.3 88.2,152.0 124.4,169.2 160.5,163.9 196.7,146.7 232.9,146.1 269.1,136.0 305.3,126.0 341.5,120.5 377.6,107.3 413.8,87.8 450.0,73.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,132.5 124.4,163.8 160.5,167.5 196.7,145.3 232.9,143.4 269.1,132.3 305.3,124.0 341.5,118.6 377.6,93.5 413.8,75.8 450.0,62.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,127.8 124.4,164.0 160.5,166.1 196.7,145.5 232.9,143.8 269.1,136.3 305.3,118.2 341.5,114.1 377.6,91.1 413.8,73.9 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.8 88.2,129.0 124.4,165.7 160.5,165.3 196.7,141.6 232.9,139.0 269.1,128.6 305.3,113.6 341.5,110.5 377.6,83.4 413.8,62.8 450.0,48.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 4.86 µs | 6.87 µs | 8.01 µs | 8.6 µs |
| D38 | 6.49 ns | 4.64 µs | 8.22 µs | 9.37 µs | 8.89 µs |
| D57 | 56.3 ns | 3.95 µs | 4.73 µs | 4.97 µs | 4.9 µs |
| D76 | 66.5 ns | 4.36 µs | 4.52 µs | 4.93 µs | 5.33 µs |
| D115 | 149 ns | 8.61 µs | 9.49 µs | 9.93 µs | 11.5 µs |
| D153 | 174 ns | 9.03 µs | 10.4 µs | 10.9 µs | 13 µs |
| D230 | 284 ns | 13.3 µs | 16.2 µs | 16.1 µs | 20.7 µs |
| D307 | 367 ns | 18.8 µs | 19.5 µs | 28.4 µs | 34.4 µs |
| D462 | 689 ns | 76.7 µs | 136 µs | 205 µs | 262 µs |
| D616 | 762 ns | 179 µs | 353 µs | 354 µs | 570 µs |
| D924 | 916 ns | 422 µs | 488 µs | 754 µs | 1.74 ms |
| D1232 | 1.41 µs | 833 µs | 855 µs | 2.11 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.8 124.4,160.0 160.5,157.9 196.7,147.9 232.9,146.0 269.1,139.9 305.3,136.7 341.5,128.9 377.6,127.7 413.8,125.4 450.0,120.1 450.0,25.0 413.8,31.7 377.6,45.5 341.5,55.2 305.3,80.4 269.1,86.7 232.9,92.5 196.7,94.0 160.5,103.5 124.4,104.6 88.2,97.2 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.8 124.4,160.0 160.5,157.9 196.7,147.9 232.9,146.0 269.1,139.9 305.3,136.7 341.5,128.9 377.6,127.7 413.8,125.4 450.0,120.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.7 88.2,105.3 124.4,107.2 160.5,106.0 196.7,97.6 232.9,97.0 269.1,92.2 305.3,87.9 341.5,70.4 377.6,59.9 413.8,49.3 450.0,40.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.4 88.2,98.1 124.4,105.0 160.5,105.6 196.7,96.4 232.9,95.3 269.1,89.8 305.3,87.4 341.5,63.4 377.6,51.5 413.8,47.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,96.5 124.4,104.4 160.5,104.5 196.7,95.8 232.9,94.6 269.1,89.8 305.3,82.8 341.5,58.3 377.6,51.4 413.8,42.1 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,97.2 124.4,104.6 160.5,103.5 196.7,94.0 232.9,92.5 269.1,86.7 305.3,80.4 341.5,55.2 377.6,45.5 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.37 ns | 15.6 ns | 19.3 ns | 30.3 ns | 30.7 ns |
| D38 | 8.96 ns | 21.7 ns | 31.1 ns | 1.47 µs | 2.47 µs |
| D57 | 158 ns | 206 ns | 434 ns | 673 ns | 764 ns |
| D76 | 197 ns | 277 ns | 770 ns | 839 ns | 1.21 µs |
| D115 | 124 ns | 602 ns | 973 ns | 1.53 µs | 1.65 µs |
| D153 | 118 ns | 1.06 µs | 1.52 µs | 1.89 µs | 2.6 µs |
| D230 | 147 ns | 1.7 µs | 2.41 µs | 3 µs | 4.26 µs |
| D307 | 150 ns | 2.3 µs | 3.33 µs | 4.67 µs | 7.18 µs |
| D462 | 210 ns | 3.88 µs | 5.84 µs | 9.33 µs | 11.9 µs |
| D616 | 225 ns | 5.3 µs | 11 µs | 14.3 µs | 20.6 µs |
| D924 | 215 ns | 10.2 µs | 17.1 µs | 23.1 µs | 35.5 µs |
| D1232 | 275 ns | 16.3 µs | 28.4 µs | 39.2 µs | 62 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,171.9 124.4,122.1 160.5,118.2 196.7,126.3 232.9,127.1 269.1,123.4 305.3,122.9 341.5,117.1 377.6,115.9 413.8,116.7 450.0,112.4 450.0,18.3 413.8,28.0 377.6,37.4 341.5,47.0 305.3,55.8 269.1,64.8 232.9,73.4 196.7,81.3 160.5,86.8 124.4,94.7 88.2,74.3 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,171.9 124.4,122.1 160.5,118.2 196.7,126.3 232.9,127.1 269.1,123.4 305.3,122.9 341.5,117.1 377.6,115.9 413.8,116.7 450.0,112.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,162.3 88.2,156.6 124.4,117.4 160.5,112.3 196.7,98.8 232.9,89.0 269.1,80.8 305.3,75.5 341.5,66.4 377.6,61.0 413.8,49.6 450.0,41.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,150.3 124.4,104.5 160.5,94.5 196.7,90.5 232.9,82.7 269.1,74.7 305.3,69.1 341.5,59.3 377.6,48.3 413.8,40.7 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,83.3 124.4,96.9 160.5,93.0 196.7,82.6 232.9,78.9 269.1,70.9 305.3,63.2 341.5,51.2 377.6,43.8 413.8,35.4 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,74.3 124.4,94.7 160.5,86.8 196.7,81.3 232.9,73.4 269.1,64.8 305.3,55.8 341.5,47.0 377.6,37.4 413.8,28.0 450.0,18.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
