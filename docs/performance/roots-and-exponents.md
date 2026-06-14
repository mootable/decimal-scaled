# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.44 µs | 5.79 µs | 5.78 µs | 8.69 µs | 7.71 µs |
| D38 | 5.07 µs | 5.14 µs | 7.71 µs | 10.9 µs | 10.9 µs |
| D57 | 336 ns | 595 ns | 718 ns | 1.14 µs | 1.06 µs |
| D76 | 533 ns | 642 ns | 1.2 µs | 1.39 µs | 1.95 µs |
| D115 | 334 ns | 2.08 µs | 2.55 µs | 3.53 µs | 3.04 µs |
| D153 | 359 ns | 2.36 µs | 3.6 µs | 5.77 µs | 6.9 µs |
| D230 | 392 ns | 4.33 µs | 8.37 µs | 10.2 µs | 14.1 µs |
| D307 | 564 ns | 6.15 µs | 11.3 µs | 17.6 µs | 22.5 µs |
| D462 | 618 ns | 9.74 µs | 26 µs | 33.1 µs | 47.4 µs |
| D616 | 671 ns | 14.7 µs | 41.1 µs | 57.7 µs | 85.1 µs |
| D924 | 403 ns | 31.8 µs | 82.1 µs | 133 µs | 199 µs |
| D1232 | 838 ns | 59.8 µs | 151 µs | 249 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,127.6 88.2,124.7 124.4,183.7 160.5,173.7 196.7,183.8 232.9,182.2 269.1,180.3 305.3,172.4 341.5,170.5 377.6,168.7 413.8,179.7 450.0,163.8 450.0,33.1 413.8,45.1 377.6,63.5 341.5,76.2 305.3,92.4 269.1,102.5 232.9,118.1 196.7,135.9 160.5,145.5 124.4,158.8 88.2,108.1 52.0,115.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,127.6 88.2,124.7 124.4,183.7 160.5,173.7 196.7,183.8 232.9,182.2 269.1,180.3 305.3,172.4 341.5,170.5 377.6,168.7 413.8,179.7 450.0,163.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,121.9 88.2,124.4 124.4,171.3 160.5,169.6 196.7,144.1 232.9,141.4 269.1,128.2 305.3,120.6 341.5,110.6 377.6,101.6 413.8,84.9 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.9 88.2,115.6 124.4,167.2 160.5,156.1 196.7,139.7 232.9,132.2 269.1,113.9 305.3,107.4 341.5,89.2 377.6,79.3 413.8,64.3 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,157.2 160.5,152.8 196.7,132.6 232.9,122.0 269.1,109.5 305.3,97.7 341.5,84.0 377.6,71.9 413.8,53.8 450.0,40.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.6 88.2,108.1 124.4,158.8 160.5,145.5 196.7,135.9 232.9,118.1 269.1,102.5 305.3,92.4 341.5,76.2 377.6,63.5 413.8,45.1 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 4.79 µs | 6.8 µs | 7.31 µs | 7.27 µs |
| D38 | 1.87 ns | 6.24 µs | 7.57 µs | 9.5 µs | 10.7 µs |
| D57 | 2.49 ns | 3.58 µs | 4.1 µs | 5.4 µs | 9.66 µs |
| D76 | 3.2 ns | 5.19 µs | 6.73 µs | 9.75 µs | 10.9 µs |
| D115 | 17.4 ns | 6.83 µs | 13.6 µs | 17.2 µs | 12.7 µs |
| D153 | 21.7 ns | 5.69 µs | 15.8 µs | 21.3 µs | 37.4 µs |
| D230 | 57.6 ns | 12.6 µs | 23.1 µs | 41.2 µs | 77.1 µs |
| D307 | 84.1 ns | 14.8 µs | 33.9 µs | 81.8 µs | 121 µs |
| D462 | 133 ns | 21.8 µs | 83.7 µs | 153 µs | 228 µs |
| D616 | 144 ns | 34.6 µs | 138 µs | 264 µs | 441 µs |
| D924 | 126 ns | 79.9 µs | 263 µs | 599 µs | 984 µs |
| D1232 | 396 ns | 128 µs | 440 µs | 983 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,202.2 124.4,198.7 160.5,195.6 196.7,174.5 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.3 377.6,148.3 413.8,150.0 450.0,135.8 450.0,25.6 413.8,38.8 377.6,48.7 341.5,56.9 305.3,64.8 269.1,70.4 232.9,79.4 196.7,92.7 160.5,94.6 124.4,96.1 88.2,94.8 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,202.2 124.4,198.7 160.5,195.6 196.7,174.5 232.9,171.8 269.1,159.7 305.3,155.0 341.5,149.3 377.6,148.3 413.8,150.0 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.8 88.2,101.6 124.4,108.4 160.5,103.8 196.7,100.4 232.9,102.7 269.1,92.8 305.3,90.8 341.5,86.0 377.6,80.3 413.8,69.9 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.5 88.2,99.2 124.4,106.8 160.5,100.6 196.7,91.9 232.9,90.0 269.1,85.3 305.3,80.6 341.5,69.4 377.6,63.2 413.8,55.2 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,96.3 124.4,103.4 160.5,96.0 196.7,89.0 232.9,86.3 269.1,78.2 305.3,69.6 341.5,61.9 377.6,55.1 413.8,44.9 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,94.8 124.4,96.1 160.5,94.6 196.7,92.7 232.9,79.4 269.1,70.4 305.3,64.8 341.5,56.9 377.6,48.7 413.8,38.8 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.4 ns | 39.7 ns | 42.9 ns | 40.3 ns | 39.6 ns |
| D38 | 15.9 ns | 41.7 ns | 67 ns | 75.3 ns | 121 ns |
| D57 | 16.7 ns | 40 ns | 67.3 ns | 699 ns | 635 ns |
| D76 | 17.3 ns | 68.6 ns | 690 ns | 666 ns | 1.04 µs |
| D115 | 20.5 ns | 79.3 ns | 615 ns | 1.21 µs | 732 ns |
| D153 | 23.2 ns | 479 ns | 927 ns | 1.4 µs | 2 µs |
| D230 | 28.2 ns | 729 ns | 1.37 µs | 2.35 µs | 3.28 µs |
| D307 | 42.3 ns | 1.09 µs | 2.16 µs | 3.23 µs | 5.66 µs |
| D462 | 62.6 ns | 1.53 µs | 3.59 µs | 6.28 µs | 9.59 µs |
| D616 | 68.4 ns | 2.41 µs | 6.17 µs | 10.8 µs | 15.4 µs |
| D924 | 78 ns | 3.77 µs | 11 µs | 24.8 µs | 28.5 µs |
| D1232 | 110 ns | 6.18 µs | 20.7 µs | 27.3 µs | 51 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,199.9 124.4,198.9 160.5,198.1 196.7,194.4 232.9,191.7 269.1,187.5 305.3,178.7 341.5,170.2 377.6,168.3 413.8,165.4 450.0,158.0 450.0,24.6 413.8,37.2 377.6,50.6 341.5,60.9 305.3,72.4 269.1,84.2 232.9,95.0 196.7,116.8 160.5,109.1 124.4,119.9 88.2,155.9 52.0,180.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,199.9 124.4,198.9 160.5,198.1 196.7,194.4 232.9,191.7 269.1,187.5 305.3,178.7 341.5,170.2 377.6,168.3 413.8,165.4 450.0,158.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,179.0 124.4,179.9 160.5,168.2 196.7,165.0 232.9,126.0 269.1,116.9 305.3,108.0 341.5,100.7 377.6,90.9 413.8,81.2 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.7 124.4,168.6 160.5,118.0 196.7,120.5 232.9,111.6 269.1,103.1 305.3,93.3 341.5,82.3 377.6,70.5 413.8,57.8 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,166.1 124.4,117.8 160.5,118.8 196.7,105.9 232.9,102.7 269.1,91.5 305.3,84.5 341.5,70.1 377.6,58.4 413.8,40.3 450.0,38.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.1 88.2,155.9 124.4,119.9 160.5,109.1 196.7,116.8 232.9,95.0 269.1,84.2 305.3,72.4 341.5,60.9 377.6,50.6 413.8,37.2 450.0,24.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.14 ns | 301 ns | 387 ns | 397 ns | 377 ns |
| D38 | 2.18 ns | 359 ns | 369 ns | 401 ns | 407 ns |
| D57 | 260 ns | 438 ns | 437 ns | 439 ns | 636 ns |
| D76 | 276 ns | 450 ns | 438 ns | 632 ns | 554 ns |
| D115 | 310 ns | 481 ns | 647 ns | 918 ns | 579 ns |
| D153 | 324 ns | 415 ns | 685 ns | 925 ns | 1.36 µs |
| D230 | 543 ns | 610 ns | 1.13 µs | 1.21 µs | 1.63 µs |
| D307 | 644 ns | 648 ns | 1.02 µs | 1.36 µs | 10.1 µs |
| D462 | 1.13 µs | 2.75 µs | 3.61 µs | 3.75 µs | 4.73 µs |
| D616 | 1.41 µs | 1.39 µs | 1.79 µs | 2.65 µs | 3.56 µs |
| D924 | 1.49 µs | 1.81 µs | 2.69 µs | 3.54 µs | 4.59 µs |
| D1232 | 3.02 µs | 2.96 µs | 4.04 µs | 4.92 µs | 6.54 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,196.5 124.4,113.4 160.5,112.4 196.7,110.3 232.9,109.6 269.1,100.6 305.3,97.7 341.5,87.9 377.6,84.1 413.8,83.0 450.0,70.8 450.0,57.4 413.8,63.5 377.6,67.9 341.5,63.0 305.3,49.9 269.1,81.5 232.9,84.7 196.7,99.5 160.5,100.3 124.4,97.9 88.2,105.6 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,196.5 124.4,113.4 160.5,112.4 196.7,110.3 232.9,109.6 269.1,100.6 305.3,97.7 341.5,87.9 377.6,84.1 413.8,83.0 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,107.8 124.4,104.4 160.5,103.9 196.7,102.7 232.9,105.3 269.1,98.6 305.3,97.5 341.5,72.4 377.6,84.2 413.8,79.7 450.0,71.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,107.3 124.4,104.4 160.5,104.3 196.7,97.6 232.9,96.6 269.1,87.9 305.3,89.7 341.5,67.7 377.6,79.9 413.8,72.8 450.0,65.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.9 124.4,104.3 160.5,98.0 196.7,91.5 232.9,91.4 269.1,86.6 305.3,84.7 341.5,67.1 377.6,73.1 413.8,68.1 450.0,62.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,105.6 124.4,97.9 160.5,100.3 196.7,99.5 232.9,84.7 269.1,81.5 305.3,49.9 341.5,63.0 377.6,67.9 413.8,63.5 450.0,57.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.34 µs | 9.23 µs | 12.4 µs | 13.6 µs | 13.9 µs |
| D38 | 7.71 µs | 11.6 µs | 14.5 µs | 18.3 µs | 21.1 µs |
| D57 | 4.03 µs | 4.04 µs | 4.36 µs | 4.37 µs | 5.22 µs |
| D76 | 4.5 µs | 3.86 µs | 4.33 µs | 5.12 µs | 4.7 µs |
| D115 | 8.05 µs | 8.94 µs | 9.26 µs | 9.29 µs | 6.16 µs |
| D153 | 8.35 µs | 7.58 µs | 9.76 µs | 9.69 µs | 11.8 µs |
| D230 | 10.9 µs | 11.9 µs | 14.6 µs | 14.7 µs | 16.6 µs |
| D307 | 15.9 µs | 18.3 µs | 21.3 µs | 25.1 µs | 28.2 µs |
| D462 | 15.9 µs | 20.2 µs | 25.3 µs | 27.4 µs | 31 µs |
| D616 | 27.6 µs | 39.6 µs | 55.6 µs | 60.7 µs | 79.1 µs |
| D924 | 34.7 µs | 67.9 µs | 96 µs | 133 µs | 161 µs |
| D1232 | 56.1 µs | 105 µs | 166 µs | 218 µs | 270 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,156.5 88.2,150.9 124.4,169.7 160.5,166.5 196.7,149.6 232.9,148.5 269.1,140.8 305.3,129.9 341.5,129.8 377.6,113.9 413.8,107.3 450.0,93.4 450.0,47.9 413.8,62.9 377.6,83.5 341.5,110.6 305.3,113.3 269.1,128.6 232.9,138.5 196.7,157.3 160.5,165.2 124.4,162.2 88.2,121.6 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,156.5 88.2,150.9 124.4,169.7 160.5,166.5 196.7,149.6 232.9,148.5 269.1,140.8 305.3,129.9 341.5,129.8 377.6,113.9 413.8,107.3 450.0,93.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,139.0 124.4,169.6 160.5,170.9 196.7,146.6 232.9,151.4 269.1,138.4 305.3,125.8 341.5,123.0 377.6,103.5 413.8,87.9 450.0,75.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.1 88.2,132.5 124.4,167.4 160.5,167.5 196.7,145.6 232.9,144.0 269.1,132.3 305.3,121.4 341.5,116.4 377.6,93.6 413.8,77.9 450.0,62.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,125.8 124.4,167.3 160.5,162.7 196.7,145.5 232.9,144.2 269.1,132.1 305.3,116.7 341.5,114.2 377.6,91.1 413.8,68.4 450.0,54.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,121.6 124.4,162.2 160.5,165.2 196.7,157.3 232.9,138.5 269.1,128.6 305.3,113.3 341.5,110.6 377.6,83.5 413.8,62.9 450.0,47.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.09 ns | 5.28 µs | 7.47 µs | 8 µs | 7.94 µs |
| D38 | 6.23 ns | 6.88 µs | 8.22 µs | 10.2 µs | 11.5 µs |
| D57 | 56.3 ns | 3.96 µs | 4.32 µs | 4.59 µs | 5.47 µs |
| D76 | 80.7 ns | 3.6 µs | 4.5 µs | 5.44 µs | 5.3 µs |
| D115 | 149 ns | 8.58 µs | 9.48 µs | 9.97 µs | 6.67 µs |
| D153 | 199 ns | 7.55 µs | 10.2 µs | 10.7 µs | 13 µs |
| D230 | 341 ns | 12.4 µs | 15.9 µs | 17.7 µs | 20.7 µs |
| D307 | 367 ns | 19 µs | 21 µs | 29.7 µs | 34.4 µs |
| D462 | 618 ns | 69.7 µs | 146 µs | 204 µs | 261 µs |
| D616 | 752 ns | 177 µs | 352 µs | 354 µs | 570 µs |
| D924 | 740 ns | 423 µs | 453 µs | 845 µs | 1.74 ms |
| D1232 | 1.46 µs | 765 µs | 855 µs | 2.31 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.7 88.2,187.3 124.4,160.0 160.5,155.5 196.7,147.9 232.9,144.3 269.1,137.6 305.3,136.7 341.5,130.3 377.6,127.8 413.8,128.0 450.0,119.6 450.0,25.0 413.8,31.7 377.6,45.5 341.5,55.2 305.3,80.4 269.1,86.7 232.9,92.4 196.7,100.7 160.5,103.6 124.4,103.2 88.2,94.0 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.7 88.2,187.3 124.4,160.0 160.5,155.5 196.7,147.9 232.9,144.3 269.1,137.6 305.3,136.7 341.5,130.3 377.6,127.8 413.8,128.0 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,103.6 88.2,100.4 124.4,107.2 160.5,108.4 196.7,97.6 232.9,99.2 269.1,93.1 305.3,87.7 341.5,71.6 377.6,60.1 413.8,49.2 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.3 88.2,98.1 124.4,106.1 160.5,105.6 196.7,96.4 232.9,95.5 269.1,89.9 305.3,86.5 341.5,62.4 377.6,51.5 413.8,48.4 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,95.4 124.4,105.4 160.5,103.3 196.7,95.7 232.9,94.8 269.1,88.6 305.3,82.2 341.5,58.3 377.6,51.4 413.8,40.7 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,94.0 124.4,103.2 160.5,103.6 196.7,100.7 232.9,92.4 269.1,86.7 305.3,80.4 341.5,55.2 377.6,45.5 413.8,31.7 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.37 ns | 14.4 ns | 19.9 ns | 30.4 ns | 28.8 ns |
| D38 | 7.5 ns | 26.7 ns | 31.1 ns | 1.64 µs | 3.19 µs |
| D57 | 158 ns | 206 ns | 486 ns | 766 ns | 670 ns |
| D76 | 209 ns | 240 ns | 763 ns | 721 ns | 1.21 µs |
| D115 | 133 ns | 602 ns | 932 ns | 1.53 µs | 899 ns |
| D153 | 125 ns | 790 ns | 1.54 µs | 1.9 µs | 2.6 µs |
| D230 | 169 ns | 1.54 µs | 2.43 µs | 3.49 µs | 4.23 µs |
| D307 | 147 ns | 2.36 µs | 3.66 µs | 4.83 µs | 7.21 µs |
| D462 | 188 ns | 3.65 µs | 6.07 µs | 9.38 µs | 11.9 µs |
| D616 | 228 ns | 5.85 µs | 11.1 µs | 14.2 µs | 21 µs |
| D924 | 167 ns | 10.3 µs | 17 µs | 26.6 µs | 35.3 µs |
| D1232 | 296 ns | 15.3 µs | 28.3 µs | 40.5 µs | 62.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.8 88.2,175.0 124.4,122.1 160.5,117.2 196.7,125.0 232.9,126.1 269.1,120.9 305.3,123.4 341.5,119.1 377.6,115.7 413.8,121.1 450.0,111.1 450.0,18.1 413.8,28.1 377.6,37.1 341.5,46.9 305.3,55.7 269.1,65.0 232.9,73.4 196.7,91.8 160.5,86.7 124.4,96.9 88.2,69.8 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.8 88.2,175.0 124.4,122.1 160.5,117.2 196.7,125.0 232.9,126.1 269.1,120.9 305.3,123.4 341.5,119.1 377.6,115.7 413.8,121.1 450.0,111.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.7 88.2,152.9 124.4,117.4 160.5,114.8 196.7,98.8 232.9,94.1 269.1,82.5 305.3,75.1 341.5,67.5 377.6,59.3 413.8,49.5 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.0 88.2,150.3 124.4,102.6 160.5,94.7 196.7,91.2 232.9,82.4 269.1,74.6 305.3,67.5 341.5,58.7 377.6,48.2 413.8,40.8 450.0,31.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,81.4 124.4,94.6 160.5,95.7 196.7,82.6 232.9,78.9 269.1,68.3 305.3,62.6 341.5,51.1 377.6,43.9 413.8,33.0 450.0,25.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,69.8 124.4,96.9 160.5,86.7 196.7,91.8 232.9,73.4 269.1,65.0 305.3,55.7 341.5,46.9 377.6,37.1 413.8,28.1 450.0,18.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
