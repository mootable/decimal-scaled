# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 77.3 ns | 131 ns | 160 ns | 262 ns | 196 ns |
| D38 | 56.3 ns | 158 ns | 264 ns | 376 ns | 410 ns |
| D57 | 189 ns | 281 ns | 719 ns | 1.14 µs | 1.14 µs |
| D76 | 199 ns | 417 ns | 802 ns | 1.05 µs | 1.25 µs |
| D115 | 214 ns | 618 ns | 784 ns | 1.84 µs | 3 µs |
| D153 | 252 ns | 884 ns | 1.75 µs | 3.2 µs | 4.32 µs |
| D230 | 289 ns | 1.38 µs | 3.26 µs | 5.54 µs | 8.75 µs |
| D307 | 305 ns | 1.18 µs | 3.18 µs | 8.89 µs | 13.7 µs |
| D462 | 385 ns | 3.18 µs | 8.95 µs | 17.6 µs | 27.7 µs |
| D616 | 502 ns | 4.48 µs | 12.1 µs | 20.5 µs | 50.4 µs |
| D924 | 541 ns | 9.11 µs | 31.1 µs | 70.1 µs | 117 µs |
| D1232 | 1.35 µs | 16 µs | 39.1 µs | 132 µs | 200 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.5 88.2,180.0 124.4,158.9 160.5,158.1 196.7,156.8 232.9,153.9 269.1,151.6 305.3,150.6 341.5,146.6 377.6,142.0 413.8,140.7 450.0,124.7 450.0,38.0 413.8,47.3 377.6,61.9 341.5,72.3 305.3,84.5 269.1,92.3 232.9,104.6 196.7,110.9 160.5,126.2 124.4,127.7 88.2,145.5 52.0,158.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.5 88.2,180.0 124.4,158.9 160.5,158.1 196.7,156.8 232.9,153.9 269.1,151.6 305.3,150.6 341.5,146.6 377.6,142.0 413.8,140.7 450.0,124.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.3 88.2,162.1 124.4,152.0 160.5,145.2 196.7,138.4 232.9,132.1 269.1,124.4 305.3,127.2 341.5,109.9 377.6,103.9 413.8,91.6 450.0,81.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,153.1 124.4,135.7 160.5,133.8 196.7,134.2 232.9,120.3 269.1,109.5 305.3,109.9 341.5,91.9 377.6,86.7 413.8,70.3 450.0,66.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,147.0 124.4,127.8 160.5,129.1 196.7,119.4 232.9,109.8 269.1,100.3 305.3,92.0 341.5,80.2 377.6,77.5 413.8,56.2 450.0,45.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.3 88.2,145.5 124.4,127.7 160.5,126.2 196.7,110.9 232.9,104.6 269.1,92.3 305.3,84.5 341.5,72.3 377.6,61.9 413.8,47.3 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.47 µs | 1.62 µs | 3.51 µs | 3.76 µs | 2.68 µs |
| D38 | 1.23 µs | 3.19 µs | 4.18 µs | 4.25 µs | 5.19 µs |
| D57 | 3.79 µs | 2.14 µs | 4.43 µs | 5.74 µs | 9.26 µs |
| D76 | 5.05 µs | 4.96 µs | 7.71 µs | 7.82 µs | 8.06 µs |
| D115 | 4.64 µs | 5.47 µs | 8.08 µs | 15.1 µs | 22.4 µs |
| D153 | 5.81 µs | 7.2 µs | 16 µs | 23.4 µs | 35.8 µs |
| D230 | 5.79 µs | 13 µs | 23.6 µs | 46 µs | 78.9 µs |
| D307 | 5.29 µs | 10.4 µs | 23.9 µs | 79.3 µs | 116 µs |
| D462 | 5.38 µs | 22.5 µs | 80.6 µs | 156 µs | 211 µs |
| D616 | 4.76 µs | 35.2 µs | 111 µs | 174 µs | 412 µs |
| D924 | 5.55 µs | 80.8 µs | 266 µs | 605 µs | 906 µs |
| D1232 | 5.75 µs | 141 µs | 322 µs | 987 µs | 2.69 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.6 88.2,205.5 124.4,181.1 160.5,174.8 196.7,176.7 232.9,171.8 269.1,171.9 305.3,173.8 341.5,173.4 377.6,176.1 413.8,172.8 450.0,172.0 450.0,38.5 413.8,62.1 377.6,79.2 341.5,93.8 305.3,106.7 269.1,115.1 232.9,132.3 196.7,142.5 160.5,164.7 124.4,161.7 88.2,174.2 52.0,188.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.6 88.2,205.5 124.4,181.1 160.5,174.8 196.7,176.7 232.9,171.8 269.1,171.9 305.3,173.8 341.5,173.4 377.6,176.1 413.8,172.8 450.0,172.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.5 88.2,184.8 124.4,193.5 160.5,175.2 196.7,173.1 232.9,167.1 269.1,154.3 305.3,159.0 341.5,142.4 377.6,132.7 413.8,114.6 450.0,102.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.7 88.2,179.0 124.4,177.7 160.5,165.6 196.7,164.6 232.9,149.8 269.1,141.3 305.3,141.1 341.5,114.7 377.6,107.7 413.8,88.7 450.0,84.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.3 88.2,178.6 124.4,172.0 160.5,165.3 196.7,151.0 232.9,141.6 269.1,126.9 305.3,115.0 341.5,100.4 377.6,97.9 413.8,70.9 450.0,60.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,188.6 88.2,174.2 124.4,161.7 160.5,164.7 196.7,142.5 232.9,132.3 269.1,115.1 305.3,106.7 341.5,93.8 377.6,79.2 413.8,62.1 450.0,38.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 40.5 ns | 42.9 ns | 40.3 ns | 31 ns |
| D38 | 9.72 ns | 41.1 ns | 74.4 ns | 66.9 ns | 106 ns |
| D57 | 16.3 ns | 26.8 ns | 83.6 ns | 375 ns | 398 ns |
| D76 | 18.2 ns | 75 ns | 398 ns | 322 ns | 473 ns |
| D115 | 15.6 ns | 76.6 ns | 253 ns | 679 ns | 953 ns |
| D153 | 23.1 ns | 390 ns | 671 ns | 1.03 µs | 1.59 µs |
| D230 | 29 ns | 421 ns | 1.13 µs | 2.01 µs | 2.8 µs |
| D307 | 44.5 ns | 449 ns | 1.19 µs | 2.82 µs | 4.92 µs |
| D462 | 62 ns | 1.15 µs | 3.14 µs | 5.83 µs | 6.69 µs |
| D616 | 63.2 ns | 1.96 µs | 4.63 µs | 6.79 µs | 13 µs |
| D924 | 93.3 ns | 3.28 µs | 10.4 µs | 24.5 µs | 24.7 µs |
| D1232 | 101 ns | 6.05 µs | 14.1 µs | 25.9 µs | 45 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,161.0 88.2,170.5 124.4,161.5 160.5,159.6 196.7,162.3 232.9,155.4 269.1,151.5 305.3,144.1 341.5,138.3 377.6,138.0 413.8,131.2 450.0,129.9 450.0,23.9 413.8,34.3 377.6,45.5 341.5,57.0 305.3,62.3 269.1,72.1 232.9,81.9 196.7,90.8 160.5,103.0 124.4,106.0 88.2,128.9 52.0,150.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,161.0 88.2,170.5 124.4,161.5 160.5,159.6 196.7,162.3 232.9,155.4 269.1,151.5 305.3,144.1 341.5,138.3 377.6,138.0 413.8,131.2 450.0,129.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,145.7 88.2,145.4 124.4,152.9 160.5,135.0 196.7,134.6 232.9,106.4 269.1,105.0 305.3,103.9 341.5,87.6 377.6,78.3 413.8,69.4 450.0,58.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,144.7 88.2,135.1 124.4,133.1 160.5,106.0 196.7,113.9 232.9,96.9 269.1,87.8 305.3,87.0 341.5,70.1 377.6,63.4 413.8,49.4 450.0,44.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,145.8 88.2,137.0 124.4,107.1 160.5,109.7 196.7,96.7 232.9,89.5 269.1,77.9 305.3,72.0 341.5,59.4 377.6,56.7 413.8,34.4 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.4 88.2,128.9 124.4,106.0 160.5,103.0 196.7,90.8 232.9,81.9 269.1,72.1 305.3,62.3 341.5,57.0 377.6,45.5 413.8,34.3 450.0,23.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.23 ns | 281 ns | 400 ns | 406 ns | 310 ns |
| D38 | 6.24 ns | 387 ns | 414 ns | 380 ns | 417 ns |
| D57 | 277 ns | 305 ns | 507 ns | 528 ns | 616 ns |
| D76 | 278 ns | 408 ns | 539 ns | 548 ns | 410 ns |
| D115 | 241 ns | 418 ns | 333 ns | 888 ns | 995 ns |
| D153 | 347 ns | 516 ns | 784 ns | 1.11 µs | 1.27 µs |
| D230 | 581 ns | 653 ns | 1.14 µs | 1.47 µs | 1.8 µs |
| D307 | 660 ns | 417 ns | 776 ns | 1.35 µs | 10.6 µs |
| D462 | 1.21 µs | 2.86 µs | 3.4 µs | 3.98 µs | 4.73 µs |
| D616 | 1.45 µs | 1.42 µs | 1.79 µs | 1.89 µs | 3.71 µs |
| D924 | 2.01 µs | 2.06 µs | 2.84 µs | 3.94 µs | 4.75 µs |
| D1232 | 3.15 µs | 3.37 µs | 3.17 µs | 5.39 µs | 6.82 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.2 88.2,178.2 124.4,112.3 160.5,112.2 196.7,114.7 232.9,108.4 269.1,99.4 305.3,97.2 341.5,86.7 377.6,83.5 413.8,77.9 450.0,70.1 450.0,56.6 413.8,62.9 377.6,67.2 341.5,63.0 305.3,49.0 269.1,79.8 232.9,85.9 196.7,90.1 160.5,105.5 124.4,98.4 88.2,105.2 52.0,110.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.2 88.2,178.2 124.4,112.3 160.5,112.2 196.7,114.7 232.9,108.4 269.1,99.4 305.3,97.2 341.5,86.7 377.6,83.5 413.8,77.9 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.1 88.2,106.5 124.4,110.6 160.5,105.6 196.7,105.1 232.9,101.5 269.1,97.4 305.3,105.2 341.5,71.8 377.6,84.0 413.8,77.5 450.0,68.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,105.3 124.4,101.8 160.5,100.7 196.7,109.1 232.9,94.2 269.1,87.7 305.3,94.4 341.5,68.7 377.6,79.9 413.8,71.8 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.7 88.2,106.8 124.4,101.1 160.5,100.5 196.7,92.1 232.9,88.2 269.1,83.3 305.3,84.8 341.5,66.0 377.6,79.0 413.8,66.2 450.0,60.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.3 88.2,105.2 124.4,98.4 160.5,105.5 196.7,90.1 232.9,85.9 269.1,79.8 305.3,49.0 341.5,63.0 377.6,67.2 413.8,62.9 450.0,56.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.75 µs | 3.12 µs | 6.08 µs | 6.64 µs | 5.13 µs |
| D38 | 1.9 µs | 5.83 µs | 7.64 µs | 8.62 µs | 10.5 µs |
| D57 | 2.46 µs | 1.38 µs | 2.71 µs | 2.84 µs | 2.97 µs |
| D76 | 2.46 µs | 2.27 µs | 2.98 µs | 2.47 µs | 1.96 µs |
| D115 | 4.1 µs | 4.5 µs | 3.16 µs | 4.7 µs | 5.85 µs |
| D153 | 4.96 µs | 4.99 µs | 5.85 µs | 6.36 µs | 6.28 µs |
| D230 | 6.5 µs | 7.21 µs | 8.91 µs | 9.56 µs | 10 µs |
| D307 | 9.55 µs | 6.93 µs | 8 µs | 14.3 µs | 15.7 µs |
| D462 | 9.8 µs | 12.6 µs | 14.4 µs | 16.9 µs | 17.2 µs |
| D616 | 13.9 µs | 23.5 µs | 26.1 µs | 27.1 µs | 41.8 µs |
| D924 | 24.1 µs | 39.9 µs | 55.4 µs | 77 µs | 85.4 µs |
| D1232 | 32.8 µs | 66.7 µs | 69.5 µs | 126 µs | 142 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.7 88.2,191.4 124.4,183.9 160.5,183.9 196.7,169.2 232.9,163.6 269.1,155.8 305.3,144.7 341.5,143.9 377.6,133.7 413.8,117.9 450.0,108.9 450.0,66.6 413.8,81.2 377.6,101.9 341.5,127.7 305.3,130.2 269.1,143.3 232.9,156.8 196.7,158.9 160.5,190.6 124.4,178.5 88.2,142.0 52.0,162.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.7 88.2,191.4 124.4,183.9 160.5,183.9 196.7,169.2 232.9,163.6 269.1,155.8 305.3,144.7 341.5,143.9 377.6,133.7 413.8,117.9 450.0,108.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.0 88.2,158.9 124.4,200.7 160.5,186.3 196.7,166.4 232.9,163.5 269.1,152.8 305.3,154.0 341.5,136.6 377.6,118.6 413.8,103.3 450.0,88.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.7 88.2,151.1 124.4,181.1 160.5,178.4 196.7,176.7 232.9,158.8 269.1,146.7 305.3,149.8 341.5,132.7 377.6,115.5 413.8,93.7 450.0,87.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.2 88.2,147.6 124.4,179.8 160.5,183.8 196.7,165.2 232.9,156.4 269.1,144.6 305.3,133.0 341.5,128.1 377.6,114.4 413.8,84.2 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.7 88.2,142.0 124.4,178.5 160.5,190.6 196.7,158.9 232.9,156.8 269.1,143.3 305.3,130.2 341.5,127.7 377.6,101.9 413.8,81.2 450.0,66.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 393 ns | 390 ns | 1.35 µs | 1.48 µs | 748 ns |
| D38 | 294 ns | 1.25 µs | 1.62 µs | 1.65 µs | 2.05 µs |
| D57 | 1.2 µs | 721 ns | 1.32 µs | 1.47 µs | 1.56 µs |
| D76 | 1.27 µs | 1.14 µs | 1.46 µs | 1.26 µs | 999 ns |
| D115 | 2.13 µs | 2.25 µs | 1.55 µs | 2.5 µs | 2.84 µs |
| D153 | 2.45 µs | 2.48 µs | 2.91 µs | 3.07 µs | 3.1 µs |
| D230 | 3.2 µs | 3.35 µs | 4.21 µs | 4.52 µs | 4.65 µs |
| D307 | 4.78 µs | 3.32 µs | 3.81 µs | 6.59 µs | 7.34 µs |
| D462 | 5.01 µs | 5.85 µs | 6.66 µs | 7.69 µs | 7.9 µs |
| D616 | 6.8 µs | 9.91 µs | 10.7 µs | 9.33 µs | 16.7 µs |
| D924 | 11.6 µs | 16 µs | 20.2 µs | 28 µs | 30.9 µs |
| D1232 | 15.8 µs | 25.2 µs | 25.8 µs | 43.7 µs | 49.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.3 88.2,178.7 124.4,138.0 160.5,136.4 196.7,121.4 232.9,117.4 269.1,109.6 305.3,98.0 341.5,96.7 377.6,87.9 413.8,72.4 450.0,63.5 450.0,30.5 413.8,44.0 377.6,61.8 341.5,83.5 305.3,85.6 269.1,98.9 232.9,110.6 196.7,113.1 160.5,143.4 124.4,130.4 88.2,122.6 52.0,151.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.3 88.2,178.7 124.4,138.0 160.5,136.4 196.7,121.4 232.9,117.4 269.1,109.6 305.3,98.0 341.5,96.7 377.6,87.9 413.8,72.4 450.0,63.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.6 88.2,136.8 124.4,152.8 160.5,139.5 196.7,119.8 232.9,117.1 269.1,108.3 305.3,108.6 341.5,92.2 377.6,76.9 413.8,63.0 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.6 88.2,129.4 124.4,135.2 160.5,132.4 196.7,130.6 232.9,112.4 269.1,101.7 305.3,104.6 341.5,88.4 377.6,74.7 413.8,56.3 450.0,49.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.0 88.2,128.9 124.4,132.2 160.5,136.7 196.7,116.8 232.9,110.9 269.1,99.6 305.3,88.7 341.5,84.3 377.6,78.7 413.8,46.9 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,122.6 124.4,130.4 160.5,143.4 196.7,113.1 232.9,110.6 269.1,98.9 305.3,85.6 341.5,83.5 377.6,61.8 413.8,44.0 450.0,30.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 341 ns | 339 ns | 1.34 µs | 1.44 µs | 729 ns |
| D38 | 228 ns | 1.24 µs | 1.58 µs | 1.63 µs | 2.03 µs |
| D57 | 950 ns | 556 ns | 1.27 µs | 1.35 µs | 1.39 µs |
| D76 | 948 ns | 916 ns | 1.29 µs | 1.11 µs | 883 ns |
| D115 | 1.66 µs | 1.92 µs | 1.28 µs | 2.04 µs | 2.6 µs |
| D153 | 1.91 µs | 2.08 µs | 2.45 µs | 2.7 µs | 2.73 µs |
| D230 | 2.54 µs | 2.83 µs | 3.54 µs | 3.74 µs | 4.01 µs |
| D307 | 3.81 µs | 2.77 µs | 3.2 µs | 5.58 µs | 6.43 µs |
| D462 | 3.95 µs | 4.97 µs | 5.8 µs | 6.69 µs | 6.9 µs |
| D616 | 5.41 µs | 8.25 µs | 9.14 µs | 8.5 µs | 14.8 µs |
| D924 | 8.92 µs | 13.7 µs | 17.8 µs | 24.6 µs | 28.1 µs |
| D1232 | 12.6 µs | 22 µs | 23.1 µs | 39.7 µs | 46 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.5 88.2,186.1 124.4,144.8 160.5,144.9 196.7,128.6 232.9,124.6 269.1,116.3 305.3,104.6 341.5,103.6 377.6,94.4 413.8,80.0 450.0,69.9 450.0,32.5 413.8,46.8 377.6,65.2 341.5,87.4 305.3,89.5 269.1,103.2 232.9,114.2 196.7,115.7 160.5,146.9 124.4,133.8 88.2,122.9 52.0,152.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.5 88.2,186.1 124.4,144.8 160.5,144.9 196.7,128.6 232.9,124.6 269.1,116.3 305.3,104.6 341.5,103.6 377.6,94.4 413.8,80.0 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.6 88.2,137.2 124.4,160.3 160.5,145.9 196.7,124.5 232.9,122.1 269.1,113.2 305.3,113.8 341.5,96.9 377.6,82.3 413.8,67.5 450.0,53.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.9 88.2,130.0 124.4,136.3 160.5,136.0 196.7,136.2 232.9,117.4 269.1,106.7 305.3,109.7 341.5,92.4 377.6,79.3 413.8,60.0 450.0,52.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,132.7 88.2,129.2 124.4,134.7 160.5,140.3 196.7,122.8 232.9,114.5 269.1,105.1 305.3,93.6 341.5,88.3 377.6,81.4 413.8,50.5 450.0,36.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,122.9 124.4,133.8 160.5,146.9 196.7,115.7 232.9,114.2 269.1,103.2 305.3,89.5 341.5,87.4 377.6,65.2 413.8,46.8 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 2.06 µs | 4.24 µs | 4.42 µs | 3.21 µs |
| D38 | 8.49 ns | 3.84 µs | 4.85 µs | 4.88 µs | 5.89 µs |
| D57 | 58.2 ns | 2.37 µs | 4.56 µs | 4.67 µs | 5.12 µs |
| D76 | 69.1 ns | 3.49 µs | 5.06 µs | 4.35 µs | 3.92 µs |
| D115 | 123 ns | 7.27 µs | 5.4 µs | 8.66 µs | 10.6 µs |
| D153 | 196 ns | 8.35 µs | 10.3 µs | 11.9 µs | 12 µs |
| D230 | 362 ns | 12.6 µs | 16.4 µs | 19.7 µs | 21.4 µs |
| D307 | 387 ns | 12.1 µs | 13.5 µs | 28.5 µs | 32.2 µs |
| D462 | 662 ns | 71 µs | 138 µs | 208 µs | 242 µs |
| D616 | 851 ns | 175 µs | 276 µs | 230 µs | 533 µs |
| D924 | 1.01 µs | 418 µs | 456 µs | 848 µs | 1.59 ms |
| D1232 | 1.66 µs | 829 µs | 606 µs | 2.3 ms | 2.66 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,175.0 88.2,183.5 124.4,159.6 160.5,157.4 196.7,150.2 232.9,144.5 269.1,136.9 305.3,136.1 341.5,129.4 377.6,126.3 413.8,124.2 450.0,118.0 450.0,26.4 413.8,32.8 377.6,46.4 341.5,56.2 305.3,81.2 269.1,86.3 232.9,93.5 196.7,95.0 160.5,107.3 124.4,104.0 88.2,102.3 52.0,109.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,175.0 88.2,183.5 124.4,159.6 160.5,157.4 196.7,150.2 232.9,144.5 269.1,136.9 305.3,136.1 341.5,129.4 377.6,126.3 413.8,124.2 450.0,118.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.3 88.2,107.6 124.4,113.6 160.5,108.8 196.7,99.7 232.9,98.0 269.1,92.9 305.3,93.3 341.5,71.4 377.6,60.2 413.8,49.4 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.4 88.2,104.7 124.4,105.4 160.5,104.2 196.7,103.4 232.9,95.3 269.1,89.6 305.3,92.0 341.5,63.2 377.6,54.6 413.8,48.3 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,104.6 124.4,105.2 160.5,106.0 196.7,97.5 232.9,93.6 269.1,87.3 305.3,82.7 341.5,58.0 377.6,56.8 413.8,40.6 450.0,28.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.8 88.2,102.3 124.4,104.0 160.5,107.3 196.7,95.0 232.9,93.5 269.1,86.3 305.3,81.2 341.5,56.2 377.6,46.4 413.8,32.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.2 ns | 13.4 ns | 18.5 ns | 29.7 ns | 24.9 ns |
| D38 | 4.31 ns | 19.3 ns | 29.3 ns | 141 ns | 208 ns |
| D57 | 88 ns | 97.4 ns | 208 ns | 358 ns | 372 ns |
| D76 | 82.4 ns | 117 ns | 309 ns | 315 ns | 455 ns |
| D115 | 92 ns | 189 ns | 244 ns | 648 ns | 924 ns |
| D153 | 110 ns | 320 ns | 658 ns | 990 ns | 1.55 µs |
| D230 | 143 ns | 428 ns | 1.02 µs | 1.95 µs | 2.72 µs |
| D307 | 157 ns | 428 ns | 1.1 µs | 2.75 µs | 4.88 µs |
| D462 | 199 ns | 1.02 µs | 2.65 µs | 5.9 µs | 6.38 µs |
| D616 | 245 ns | 1.83 µs | 3.97 µs | 4.55 µs | 12.4 µs |
| D924 | 241 ns | 2.72 µs | 6.85 µs | 16.2 µs | 24.2 µs |
| D1232 | 388 ns | 5.3 µs | 11.1 µs | 26.4 µs | 44.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.3 88.2,184.6 124.4,132.2 160.5,133.4 196.7,131.4 232.9,128.4 269.1,123.8 305.3,122.2 341.5,118.0 377.6,114.4 413.8,114.8 450.0,106.5 450.0,24.0 413.8,34.6 377.6,46.2 341.5,57.8 305.3,62.5 269.1,72.6 232.9,82.4 196.7,91.4 160.5,103.7 124.4,107.2 88.2,117.3 52.0,154.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.3 88.2,184.6 124.4,132.2 160.5,133.4 196.7,131.4 232.9,128.4 269.1,123.8 305.3,122.2 341.5,118.0 377.6,114.4 413.8,114.8 450.0,106.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,158.6 124.4,130.5 160.5,127.3 196.7,118.9 232.9,109.8 269.1,104.7 305.3,104.7 341.5,89.7 377.6,79.5 413.8,72.6 450.0,61.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,151.3 124.4,117.3 160.5,110.4 196.7,114.5 232.9,97.3 269.1,89.7 305.3,88.3 341.5,73.1 377.6,66.0 413.8,56.6 450.0,48.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,124.0 124.4,107.8 160.5,110.1 196.7,97.5 232.9,90.2 269.1,78.4 305.3,72.4 341.5,59.2 377.6,63.7 413.8,41.6 450.0,33.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.2 88.2,117.3 124.4,107.2 160.5,103.7 196.7,91.4 232.9,82.4 269.1,72.6 305.3,62.5 341.5,57.8 377.6,46.2 413.8,34.6 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
