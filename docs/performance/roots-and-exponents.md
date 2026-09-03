# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 148 ns | 132 ns | 159 ns | 263 ns | 387 ns |
| D38 | 250 ns | 247 ns | 461 ns | 398 ns | 838 ns |
| D57 | 268 ns | 342 ns | 459 ns | 1.14 µs | 1.07 µs |
| D76 | 403 ns | 773 ns | 1.11 µs | 738 ns | 1.89 µs |
| D115 | 508 ns | 2.3 µs | 2.47 µs | 3.53 µs | 5.54 µs |
| D153 | 356 ns | 2.65 µs | 2.87 µs | 4.84 µs | 6.6 µs |
| D230 | 345 ns | 4.3 µs | 7.07 µs | 9.96 µs | 12 µs |
| D307 | 425 ns | 5.76 µs | 9.47 µs | 16.2 µs | 20.5 µs |
| D462 | 613 ns | 10.5 µs | 17.7 µs | 33 µs | 40.9 µs |
| D616 | 305 ns | 13.3 µs | 40.1 µs | 57.8 µs | 84.6 µs |
| D924 | 697 ns | 28.5 µs | 88.8 µs | 121 µs | 111 µs |
| D1232 | 569 ns | 55.4 µs | 87.5 µs | 142 µs | 267 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.4 88.2,190.1 124.4,188.6 160.5,179.7 196.7,174.7 232.9,182.4 269.1,183.1 305.3,178.6 341.5,170.6 377.6,185.8 413.8,167.8 450.0,172.2 450.0,38.7 413.8,57.8 377.6,63.6 341.5,79.4 305.3,94.4 269.1,106.0 232.9,119.0 196.7,122.8 160.5,146.2 124.4,158.6 88.2,163.8 52.0,180.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.4 88.2,190.1 124.4,188.6 160.5,179.7 196.7,174.7 232.9,182.4 269.1,183.1 305.3,178.6 341.5,170.6 377.6,185.8 413.8,167.8 450.0,172.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,204.0 88.2,190.4 124.4,183.3 160.5,165.6 196.7,141.9 232.9,138.8 269.1,128.3 305.3,122.0 341.5,109.0 377.6,103.9 413.8,87.2 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,199.9 88.2,176.8 124.4,176.9 160.5,157.8 196.7,140.3 232.9,137.1 269.1,117.5 305.3,111.2 341.5,97.7 377.6,79.8 413.8,62.6 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.0 88.2,180.0 124.4,157.2 160.5,166.6 196.7,132.6 232.9,125.7 269.1,110.1 305.3,99.5 341.5,84.1 377.6,71.9 413.8,55.9 450.0,52.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.6 88.2,163.8 124.4,158.6 160.5,146.2 196.7,122.8 232.9,119.0 269.1,106.0 305.3,94.4 341.5,79.4 377.6,63.6 413.8,57.8 450.0,38.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 1.65 µs | 3.56 µs | 2.97 µs | 3.67 µs |
| D38 | 1.76 ns | 2.75 µs | 4.22 µs | 3 µs | 4.75 µs |
| D57 | 2.17 ns | 2.72 µs | 3.17 µs | 5.8 µs | 10.1 µs |
| D76 | 2.5 ns | 5.9 µs | 7.71 µs | 5.5 µs | 12.4 µs |
| D115 | 11.8 ns | 6.71 µs | 13.9 µs | 18.4 µs | 22.4 µs |
| D153 | 17.9 ns | 7.24 µs | 13.7 µs | 20.6 µs | 34.6 µs |
| D230 | 35.2 ns | 13.2 µs | 19.9 µs | 42.6 µs | 73.7 µs |
| D307 | 85.3 ns | 16.6 µs | 32.7 µs | 78 µs | 115 µs |
| D462 | 110 ns | 23.8 µs | 59.4 µs | 154 µs | 195 µs |
| D616 | 104 ns | 31 µs | 141 µs | 266 µs | 442 µs |
| D924 | 178 ns | 54.6 µs | 285 µs | 553 µs | 638 µs |
| D1232 | 231 ns | 131 µs | 223 µs | 693 µs | 2.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,203.0 124.4,200.4 160.5,198.6 196.7,179.3 232.9,174.2 269.1,165.8 305.3,154.8 341.5,151.7 377.6,152.3 413.8,145.7 450.0,142.5 450.0,26.0 413.8,44.1 377.6,48.7 341.5,58.9 305.3,65.4 269.1,70.9 232.9,80.3 196.7,85.7 160.5,93.0 124.4,95.6 88.2,104.9 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,203.0 124.4,200.4 160.5,198.6 196.7,179.3 232.9,174.2 269.1,165.8 305.3,154.8 341.5,151.7 377.6,152.3 413.8,145.7 450.0,142.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.1 88.2,111.7 124.4,111.9 160.5,102.3 196.7,100.7 232.9,99.7 269.1,92.2 305.3,89.4 341.5,84.9 377.6,81.7 413.8,74.7 450.0,63.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,106.4 124.4,110.0 160.5,98.9 196.7,91.6 232.9,91.8 269.1,87.2 305.3,81.0 341.5,73.6 377.6,62.9 413.8,54.2 450.0,57.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.8 88.2,110.7 124.4,102.5 160.5,103.1 196.7,88.2 232.9,86.7 269.1,77.7 305.3,70.2 341.5,61.8 377.6,55.0 413.8,45.9 450.0,43.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,104.9 124.4,95.6 160.5,93.0 196.7,85.7 232.9,80.3 269.1,70.9 305.3,65.4 341.5,58.9 377.6,48.7 413.8,44.1 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 40.5 ns | 42.9 ns | 37.9 ns | 40.8 ns |
| D38 | 15.5 ns | 33.2 ns | 74.4 ns | 55.2 ns | 95.3 ns |
| D57 | 14.6 ns | 30.1 ns | 58.1 ns | 698 ns | 627 ns |
| D76 | 13.3 ns | 70.2 ns | 641 ns | 320 ns | 907 ns |
| D115 | 21 ns | 73.2 ns | 613 ns | 1.23 µs | 1.37 µs |
| D153 | 23.5 ns | 700 ns | 713 ns | 1.11 µs | 2.09 µs |
| D230 | 22.1 ns | 719 ns | 1.06 µs | 2.35 µs | 2.95 µs |
| D307 | 43.4 ns | 966 ns | 1.81 µs | 3.3 µs | 5.5 µs |
| D462 | 63.5 ns | 1.43 µs | 2.75 µs | 6.26 µs | 7.12 µs |
| D616 | 40.7 ns | 2.06 µs | 6.15 µs | 10.9 µs | 15.4 µs |
| D924 | 91.7 ns | 2.57 µs | 11.3 µs | 22.8 µs | 20.3 µs |
| D1232 | 86.7 ns | 6.13 µs | 11.7 µs | 19.4 µs | 44.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.2 88.2,200.5 124.4,201.8 160.5,203.9 196.7,193.9 232.9,191.4 269.1,192.8 305.3,178.1 341.5,169.9 377.6,179.5 413.8,161.9 450.0,163.1 450.0,27.6 413.8,44.6 377.6,50.7 341.5,67.4 305.3,73.0 269.1,86.5 232.9,94.0 196.7,103.1 160.5,112.1 124.4,120.1 88.2,161.1 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.2 88.2,200.5 124.4,201.8 160.5,203.9 196.7,193.9 232.9,191.4 269.1,192.8 305.3,178.1 341.5,169.9 377.6,179.5 413.8,161.9 450.0,163.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,184.0 124.4,186.0 160.5,167.7 196.7,166.8 232.9,117.7 269.1,117.2 305.3,110.8 341.5,102.2 377.6,94.3 413.8,89.5 450.0,70.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,166.4 124.4,171.8 160.5,119.7 196.7,120.6 232.9,117.3 269.1,108.8 305.3,97.1 341.5,88.0 377.6,70.6 413.8,57.3 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.0 88.2,172.9 124.4,117.8 160.5,134.7 196.7,105.4 232.9,107.7 269.1,91.4 305.3,84.1 341.5,70.2 377.6,58.1 413.8,42.1 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,161.1 124.4,120.1 160.5,112.1 196.7,103.1 232.9,94.0 269.1,86.5 305.3,73.0 341.5,67.4 377.6,50.7 413.8,44.6 450.0,27.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.95 ns | 277 ns | 405 ns | 377 ns | 390 ns |
| D38 | 2.11 ns | 316 ns | 418 ns | 312 ns | 389 ns |
| D57 | 252 ns | 361 ns | 371 ns | 504 ns | 699 ns |
| D76 | 224 ns | 479 ns | 511 ns | 351 ns | 689 ns |
| D115 | 295 ns | 496 ns | 704 ns | 981 ns | 974 ns |
| D153 | 352 ns | 516 ns | 597 ns | 895 ns | 1.29 µs |
| D230 | 448 ns | 666 ns | 985 ns | 1.3 µs | 1.56 µs |
| D307 | 753 ns | 776 ns | 957 ns | 1.38 µs | 10.4 µs |
| D462 | 1.18 µs | 3.14 µs | 2.74 µs | 4.01 µs | 4.2 µs |
| D616 | 910 ns | 1.25 µs | 1.99 µs | 2.9 µs | 3.94 µs |
| D924 | 2 µs | 1.46 µs | 3.12 µs | 3.71 µs | 3.02 µs |
| D1232 | 2.39 µs | 3.19 µs | 2.36 µs | 3.29 µs | 6.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,197.0 124.4,113.9 160.5,116.0 196.7,111.2 232.9,108.1 269.1,104.0 305.3,94.9 341.5,87.1 377.6,91.6 413.8,78.0 450.0,74.9 450.0,57.8 413.8,70.8 377.6,66.2 341.5,65.1 305.3,49.3 269.1,82.3 232.9,85.5 196.7,90.5 160.5,96.5 124.4,96.2 88.2,106.4 52.0,106.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,197.0 124.4,113.9 160.5,116.0 196.7,111.2 232.9,108.1 269.1,104.0 305.3,94.9 341.5,87.1 377.6,91.6 413.8,78.0 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,110.0 124.4,107.7 160.5,102.8 196.7,102.2 232.9,101.5 269.1,97.1 305.3,94.4 341.5,70.1 377.6,86.2 413.8,83.4 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.7 88.2,105.2 124.4,107.2 160.5,101.7 196.7,96.1 232.9,99.0 269.1,90.3 305.3,90.8 341.5,72.5 377.6,78.0 413.8,70.2 450.0,75.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,110.3 124.4,101.9 160.5,108.2 196.7,90.3 232.9,91.9 269.1,85.4 305.3,84.4 341.5,65.9 377.6,71.5 413.8,67.2 450.0,69.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.4 88.2,106.4 124.4,96.2 160.5,96.5 196.7,90.5 232.9,85.5 269.1,82.3 305.3,49.3 341.5,65.1 377.6,66.2 413.8,70.8 450.0,57.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.89 µs | 3.16 µs | 5.9 µs | 5.43 µs | 6.91 µs |
| D38 | 2.89 µs | 4.59 µs | 7.37 µs | 6.12 µs | 9.76 µs |
| D57 | 3.28 µs | 2.82 µs | 3.17 µs | 4.47 µs | 5.19 µs |
| D76 | 3.38 µs | 4.27 µs | 4.94 µs | 2.58 µs | 5.34 µs |
| D115 | 7.85 µs | 8.79 µs | 9.57 µs | 9.76 µs | 10.2 µs |
| D153 | 8.58 µs | 8.65 µs | 8.43 µs | 9.22 µs | 11.2 µs |
| D230 | 9.74 µs | 12.8 µs | 12.9 µs | 15.6 µs | 15.5 µs |
| D307 | 16.3 µs | 19.2 µs | 18.8 µs | 23.8 µs | 26.3 µs |
| D462 | 15.6 µs | 22 µs | 17.5 µs | 27.8 µs | 26.9 µs |
| D616 | 15.3 µs | 34.1 µs | 56.3 µs | 61.9 µs | 79.8 µs |
| D924 | 40.9 µs | 51.1 µs | 104 µs | 123 µs | 99 µs |
| D1232 | 48.4 µs | 106 µs | 96.6 µs | 131 µs | 222 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.3 88.2,179.3 124.4,175.6 160.5,174.7 196.7,150.4 232.9,147.8 269.1,144.1 305.3,129.2 341.5,130.4 377.6,131.0 413.8,102.6 450.0,97.7 450.0,53.5 413.8,77.0 377.6,83.2 341.5,114.7 305.3,115.3 269.1,130.7 232.9,140.2 196.7,142.8 160.5,161.5 124.4,162.3 88.2,144.0 52.0,154.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.3 88.2,179.3 124.4,175.6 160.5,174.7 196.7,150.4 232.9,147.8 269.1,144.1 305.3,129.2 341.5,130.4 377.6,131.0 413.8,102.6 450.0,97.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.7 88.2,165.9 124.4,180.0 160.5,168.0 196.7,147.1 232.9,147.5 269.1,136.2 305.3,124.5 341.5,120.6 377.6,107.8 413.8,96.1 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.6 88.2,152.2 124.4,176.6 160.5,163.8 196.7,144.6 232.9,148.3 269.1,135.9 305.3,125.1 341.5,127.2 377.6,93.3 413.8,75.5 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.0 88.2,157.5 124.4,166.7 160.5,182.5 196.7,144.0 232.9,145.7 269.1,130.4 305.3,118.2 341.5,113.7 377.6,90.6 413.8,70.6 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.1 88.2,144.0 124.4,162.3 160.5,161.5 196.7,142.8 232.9,140.2 269.1,130.7 305.3,115.3 341.5,114.7 377.6,83.2 413.8,77.0 450.0,53.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 2.08 µs | 4.25 µs | 3.57 µs | 4.31 µs |
| D38 | 6.47 ns | 3.29 µs | 4.89 µs | 3.51 µs | 5.4 µs |
| D57 | 69.6 ns | 3.04 µs | 3.35 µs | 4.78 µs | 5.61 µs |
| D76 | 76.6 ns | 4.06 µs | 5.16 µs | 2.9 µs | 6.17 µs |
| D115 | 138 ns | 8.44 µs | 9.73 µs | 10.4 µs | 11 µs |
| D153 | 198 ns | 8.57 µs | 8.62 µs | 10.2 µs | 12.1 µs |
| D230 | 266 ns | 13.1 µs | 13.9 µs | 18.6 µs | 20 µs |
| D307 | 432 ns | 20.5 µs | 19.4 µs | 28.3 µs | 32.1 µs |
| D462 | 627 ns | 77.4 µs | 104 µs | 205 µs | 221 µs |
| D616 | 456 ns | 161 µs | 350 µs | 352 µs | 570 µs |
| D924 | 921 ns | 311 µs | 484 µs | 783 µs | 1.18 ms |
| D1232 | 1.07 µs | 759 µs | 451 µs | 1.54 ms | 2.52 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,186.8 124.4,157.4 160.5,156.2 196.7,148.8 232.9,144.4 269.1,140.7 305.3,134.7 341.5,130.1 377.6,134.0 413.8,125.3 450.0,123.4 450.0,27.1 413.8,36.5 377.6,45.6 341.5,57.3 305.3,81.2 269.1,87.1 232.9,93.4 196.7,94.5 160.5,101.7 124.4,102.9 88.2,103.4 52.0,106.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,186.8 124.4,157.4 160.5,156.2 196.7,148.8 232.9,144.4 269.1,140.7 305.3,134.7 341.5,130.1 377.6,134.0 413.8,125.3 450.0,123.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.2 88.2,109.5 124.4,110.5 160.5,106.9 196.7,97.8 232.9,97.6 269.1,92.4 305.3,86.8 341.5,70.3 377.6,61.2 413.8,53.0 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,104.6 124.4,109.3 160.5,103.9 196.7,96.1 232.9,97.6 269.1,91.6 305.3,87.5 341.5,66.7 377.6,51.6 413.8,47.6 450.0,48.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.5 88.2,108.7 124.4,104.9 160.5,111.1 196.7,95.2 232.9,95.5 269.1,88.0 305.3,82.8 341.5,58.2 377.6,51.5 413.8,41.6 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,103.4 124.4,102.9 160.5,101.7 196.7,94.5 232.9,93.4 269.1,87.1 305.3,81.2 341.5,57.3 377.6,45.6 413.8,36.5 450.0,27.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.29 ns | 13.3 ns | 18.5 ns | 31.2 ns | 28.9 ns |
| D38 | 8.17 ns | 15.9 ns | 33.1 ns | 249 ns | 609 ns |
| D57 | 173 ns | 192 ns | 319 ns | 773 ns | 665 ns |
| D76 | 163 ns | 270 ns | 644 ns | 430 ns | 1.05 µs |
| D115 | 116 ns | 661 ns | 1 µs | 1.53 µs | 1.68 µs |
| D153 | 127 ns | 1.09 µs | 1.17 µs | 1.57 µs | 2.65 µs |
| D230 | 144 ns | 1.56 µs | 2.03 µs | 3.47 µs | 3.81 µs |
| D307 | 159 ns | 2.38 µs | 3.18 µs | 4.61 µs | 6.92 µs |
| D462 | 179 ns | 3.68 µs | 4.48 µs | 9.17 µs | 9.29 µs |
| D616 | 131 ns | 7.23 µs | 10.9 µs | 14.2 µs | 20.6 µs |
| D924 | 219 ns | 10.6 µs | 16.9 µs | 26.1 µs | 23.1 µs |
| D1232 | 200 ns | 15.3 µs | 15.1 µs | 26.9 µs | 52.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.1 88.2,173.5 124.4,120.5 160.5,121.5 196.7,127.5 232.9,125.9 269.1,123.7 305.3,122.0 341.5,119.9 377.6,125.4 413.8,116.4 450.0,117.9 450.0,21.1 413.8,35.5 377.6,37.4 341.5,51.3 305.3,56.4 269.1,66.8 232.9,73.1 196.7,80.9 160.5,89.2 124.4,97.1 88.2,98.6 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.1 88.2,173.5 124.4,120.5 160.5,121.5 196.7,127.5 232.9,125.9 269.1,123.7 305.3,122.0 341.5,119.9 377.6,125.4 413.8,116.4 450.0,117.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,162.0 124.4,118.6 160.5,112.8 196.7,97.2 232.9,88.5 269.1,82.3 305.3,74.9 341.5,67.4 377.6,55.6 413.8,49.1 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,149.2 124.4,109.9 160.5,97.6 196.7,90.0 232.9,87.3 269.1,77.7 305.3,69.9 341.5,64.0 377.6,48.5 413.8,40.9 450.0,42.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.2 88.2,114.2 124.4,94.5 160.5,104.7 196.7,82.6 232.9,82.1 269.1,68.4 305.3,63.4 341.5,51.5 377.6,43.9 413.8,33.4 450.0,32.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,98.6 124.4,97.1 160.5,89.2 196.7,80.9 232.9,73.1 269.1,66.8 305.3,56.4 341.5,51.3 377.6,37.4 413.8,35.5 450.0,21.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
