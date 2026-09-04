# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 64.2 ns | 134 ns | 155 ns | 258 ns | 260 ns |
| D38 | 77.6 ns | 157 ns | 255 ns | 379 ns | 397 ns |
| D57 | 190 ns | 422 ns | 509 ns | 897 ns | 1.14 µs |
| D76 | 201 ns | 489 ns | 799 ns | 1.3 µs | 1.65 µs |
| D115 | 226 ns | 769 ns | 1.35 µs | 2.1 µs | 3.02 µs |
| D153 | 246 ns | 700 ns | 1.11 µs | 3.2 µs | 3.58 µs |
| D230 | 278 ns | 1.41 µs | 3.29 µs | 5.13 µs | 9.44 µs |
| D307 | 325 ns | 1.49 µs | 4.43 µs | 8.78 µs | 15 µs |
| D462 | 389 ns | 2.28 µs | 5.56 µs | 17.2 µs | 30.3 µs |
| D616 | 500 ns | 4.77 µs | 14.5 µs | 20.5 µs | 43.7 µs |
| D924 | 526 ns | 9.16 µs | 34.4 µs | 54.5 µs | 130 µs |
| D1232 | 997 ns | 16 µs | 51.8 µs | 76.9 µs | 207 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.7 88.2,174.4 124.4,158.8 160.5,157.9 196.7,155.8 232.9,154.3 269.1,152.2 305.3,149.5 341.5,146.4 377.6,142.0 413.8,141.2 450.0,130.1 450.0,37.4 413.8,45.4 377.6,64.4 341.5,70.7 305.3,82.9 269.1,91.0 232.9,107.8 196.7,110.8 160.5,121.3 124.4,127.8 88.2,146.1 52.0,153.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.7 88.2,174.4 124.4,158.8 160.5,157.9 196.7,155.8 232.9,154.3 269.1,152.2 305.3,149.5 341.5,146.4 377.6,142.0 413.8,141.2 450.0,130.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,162.2 124.4,145.0 160.5,142.4 196.7,134.6 232.9,136.2 269.1,124.0 305.3,123.1 341.5,115.7 377.6,102.9 413.8,91.5 450.0,81.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.3 88.2,153.7 124.4,141.7 160.5,133.9 196.7,124.7 232.9,128.2 269.1,109.3 305.3,104.1 341.5,100.2 377.6,83.5 413.8,68.5 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.5 88.2,146.9 124.4,131.9 160.5,125.4 196.7,117.1 232.9,109.8 269.1,101.6 305.3,92.3 341.5,80.6 377.6,77.6 413.8,60.5 450.0,54.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.4 88.2,146.1 124.4,127.8 160.5,121.3 196.7,110.8 232.9,107.8 269.1,91.0 305.3,82.9 341.5,70.7 377.6,64.4 413.8,45.4 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.78 µs | 3.2 µs | 3.43 µs | 3.64 µs |
| D38 | 1.87 ns | 3.2 µs | 3.8 µs | 4.68 µs | 4.71 µs |
| D57 | 2.81 ns | 4.27 µs | 3.75 µs | 5.35 µs | 9.25 µs |
| D76 | 2.62 ns | 6.44 µs | 7.69 µs | 10.1 µs | 11.3 µs |
| D115 | 13.2 ns | 6.66 µs | 13.7 µs | 19 µs | 20.7 µs |
| D153 | 17.9 ns | 5.2 µs | 9.48 µs | 23.5 µs | 30.4 µs |
| D230 | 37.9 ns | 13.7 µs | 23.7 µs | 42.3 µs | 82.9 µs |
| D307 | 66.3 ns | 13.7 µs | 35.5 µs | 79.2 µs | 124 µs |
| D462 | 108 ns | 14.6 µs | 44.4 µs | 157 µs | 234 µs |
| D616 | 114 ns | 38.5 µs | 133 µs | 182 µs | 347 µs |
| D924 | 177 ns | 80.5 µs | 285 µs | 474 µs | 991 µs |
| D1232 | 381 ns | 141 µs | 415 µs | 634 µs | 2.66 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,202.2 124.4,197.2 160.5,198.0 196.7,178.0 232.9,174.2 269.1,164.9 305.3,157.9 341.5,151.9 377.6,151.2 413.8,145.8 450.0,136.3 450.0,26.4 413.8,38.7 377.6,51.7 341.5,56.6 305.3,64.5 269.1,69.5 232.9,81.9 196.7,86.7 160.5,94.2 124.4,96.7 88.2,105.1 52.0,108.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,202.2 124.4,197.2 160.5,198.0 196.7,178.0 232.9,174.2 269.1,164.9 305.3,157.9 341.5,151.9 377.6,151.2 413.8,145.8 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,109.9 124.4,106.3 160.5,101.2 196.7,100.8 232.9,103.8 269.1,91.8 305.3,91.8 341.5,91.0 377.6,79.0 413.8,69.8 450.0,62.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.9 88.2,107.7 124.4,107.9 160.5,99.0 196.7,91.8 232.9,96.4 269.1,85.0 305.3,80.0 341.5,77.2 377.6,63.6 413.8,54.1 450.0,49.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,105.1 124.4,103.5 160.5,95.5 196.7,87.8 232.9,85.1 269.1,77.8 305.3,70.0 341.5,61.6 377.6,59.7 413.8,47.8 450.0,44.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.2 88.2,105.1 124.4,96.7 160.5,94.2 196.7,86.7 232.9,81.9 269.1,69.5 305.3,64.5 341.5,56.6 377.6,51.7 413.8,38.7 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.4 ns | 39.7 ns | 42.6 ns | 40.8 ns | 40.8 ns |
| D38 | 16.1 ns | 41.1 ns | 66.9 ns | 75.4 ns | 95.1 ns |
| D57 | 16.6 ns | 42.2 ns | 74.4 ns | 375 ns | 390 ns |
| D76 | 14.6 ns | 96.1 ns | 397 ns | 415 ns | 608 ns |
| D115 | 20.1 ns | 85.3 ns | 420 ns | 834 ns | 963 ns |
| D153 | 24.2 ns | 316 ns | 402 ns | 1.03 µs | 1.31 µs |
| D230 | 20.8 ns | 449 ns | 1.14 µs | 1.88 µs | 2.94 µs |
| D307 | 33.5 ns | 596 ns | 1.63 µs | 2.8 µs | 5.3 µs |
| D462 | 62.1 ns | 757 ns | 2.01 µs | 5.72 µs | 7.05 µs |
| D616 | 69.9 ns | 2.09 µs | 5.67 µs | 6.76 µs | 10.8 µs |
| D924 | 93.1 ns | 3.31 µs | 11.1 µs | 19.1 µs | 27 µs |
| D1232 | 110 ns | 6.06 µs | 18.6 µs | 18.7 µs | 44.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,199.7 124.4,199.0 160.5,201.7 196.7,194.8 232.9,190.8 269.1,194.1 305.3,183.8 341.5,170.3 377.6,167.8 413.8,161.5 450.0,157.9 450.0,27.4 413.8,38.4 377.6,58.4 341.5,67.6 305.3,73.8 269.1,86.6 232.9,104.2 196.7,110.8 160.5,120.8 124.4,130.4 88.2,161.1 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,199.7 124.4,199.0 160.5,201.7 196.7,194.8 232.9,190.8 269.1,194.1 305.3,183.8 341.5,170.3 377.6,167.8 413.8,161.5 450.0,157.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,179.3 124.4,178.7 160.5,160.9 196.7,163.5 232.9,135.0 269.1,127.4 305.3,121.2 341.5,116.1 377.6,94.0 413.8,84.0 450.0,70.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.5 88.2,168.7 124.4,166.4 160.5,130.1 196.7,128.9 232.9,129.8 269.1,107.1 305.3,99.4 341.5,94.9 377.6,72.3 413.8,57.7 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,166.1 124.4,131.3 160.5,129.1 196.7,113.9 232.9,109.4 269.1,96.2 305.3,87.6 341.5,72.1 377.6,68.5 413.8,46.0 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,161.1 124.4,130.4 160.5,120.8 196.7,110.8 232.9,104.2 269.1,86.6 305.3,73.8 341.5,67.6 377.6,58.4 413.8,38.4 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.06 ns | 294 ns | 377 ns | 380 ns | 382 ns |
| D38 | 2.18 ns | 379 ns | 385 ns | 414 ns | 391 ns |
| D57 | 285 ns | 514 ns | 403 ns | 449 ns | 611 ns |
| D76 | 237 ns | 507 ns | 525 ns | 709 ns | 635 ns |
| D115 | 317 ns | 497 ns | 714 ns | 1.02 µs | 866 ns |
| D153 | 358 ns | 381 ns | 369 ns | 1.11 µs | 1.21 µs |
| D230 | 447 ns | 710 ns | 1.24 µs | 1.28 µs | 1.97 µs |
| D307 | 577 ns | 632 ns | 1.13 µs | 1.31 µs | 11.2 µs |
| D462 | 1.2 µs | 1.92 µs | 1.98 µs | 4.03 µs | 5.08 µs |
| D616 | 1.4 µs | 1.56 µs | 1.97 µs | 2.02 µs | 3.29 µs |
| D924 | 2.02 µs | 2.1 µs | 3.13 µs | 3.3 µs | 5.13 µs |
| D1232 | 3.21 µs | 3.23 µs | 4.22 µs | 3.4 µs | 6.71 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.6 88.2,196.5 124.4,111.8 160.5,115.0 196.7,110.0 232.9,107.9 269.1,104.0 305.3,99.6 341.5,86.8 377.6,84.1 413.8,77.8 450.0,69.7 450.0,56.9 413.8,61.6 377.6,69.3 341.5,61.8 305.3,48.0 269.1,78.2 232.9,86.7 196.7,92.5 160.5,97.9 124.4,98.6 88.2,106.3 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.6 88.2,196.5 124.4,111.8 160.5,115.0 196.7,110.0 232.9,107.9 269.1,104.0 305.3,99.6 341.5,86.8 377.6,84.1 413.8,77.8 450.0,69.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,111.2 88.2,106.8 124.4,101.6 160.5,101.8 196.7,102.1 232.9,106.7 269.1,95.9 305.3,98.0 341.5,78.7 377.6,82.3 413.8,77.1 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,106.6 124.4,105.8 160.5,101.2 196.7,95.9 232.9,107.3 269.1,86.3 305.3,87.9 341.5,78.2 377.6,78.2 413.8,70.2 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,105.3 124.4,103.9 160.5,96.0 196.7,89.6 232.9,88.3 269.1,85.7 305.3,85.3 341.5,65.8 377.6,77.8 413.8,69.2 450.0,68.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,106.3 124.4,98.6 160.5,97.9 196.7,92.5 232.9,86.7 269.1,78.2 305.3,48.0 341.5,61.8 377.6,69.3 413.8,61.6 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.35 µs | 3.26 µs | 5.78 µs | 6.36 µs | 7.02 µs |
| D38 | 2.75 µs | 5.79 µs | 7.29 µs | 9.03 µs | 9.94 µs |
| D57 | 2.75 µs | 2.85 µs | 2.33 µs | 2.46 µs | 2.93 µs |
| D76 | 1.9 µs | 2.97 µs | 2.95 µs | 3.14 µs | 3.01 µs |
| D115 | 4.81 µs | 4.95 µs | 5.57 µs | 6 µs | 5.26 µs |
| D153 | 4.87 µs | 3.81 µs | 3.44 µs | 6.22 µs | 5.54 µs |
| D230 | 5.07 µs | 7.57 µs | 8.77 µs | 8.75 µs | 10.7 µs |
| D307 | 7.9 µs | 9.15 µs | 12.8 µs | 14.2 µs | 17 µs |
| D462 | 9.81 µs | 7.79 µs | 8.32 µs | 16.8 µs | 18.9 µs |
| D616 | 16.8 µs | 25.5 µs | 30.1 µs | 23.3 µs | 37 µs |
| D924 | 24.1 µs | 39.8 µs | 59.9 µs | 61 µs | 93 µs |
| D1232 | 33.4 µs | 67.1 µs | 88.3 µs | 85.9 µs | 140 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.3 88.2,180.7 124.4,180.7 160.5,191.4 196.7,164.5 232.9,164.1 269.1,163.0 305.3,150.2 341.5,143.9 377.6,128.4 413.8,117.9 450.0,108.4 450.0,67.0 413.8,78.8 377.6,105.4 341.5,124.9 305.3,128.0 269.1,141.4 232.9,160.4 196.7,161.9 160.5,178.1 124.4,178.8 88.2,143.5 52.0,153.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.3 88.2,180.7 124.4,180.7 160.5,191.4 196.7,164.5 232.9,164.1 269.1,163.0 305.3,150.2 341.5,143.9 377.6,128.4 413.8,117.9 450.0,108.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.8 88.2,159.1 124.4,179.7 160.5,178.5 196.7,163.7 232.9,171.2 269.1,151.4 305.3,145.9 341.5,150.6 377.6,116.3 413.8,103.3 450.0,88.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.2 88.2,152.5 124.4,185.5 160.5,178.7 196.7,160.3 232.9,174.3 269.1,147.1 305.3,136.1 341.5,148.7 377.6,111.5 413.8,91.5 450.0,80.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.4 88.2,146.3 124.4,184.0 160.5,176.9 196.7,158.1 232.9,157.1 269.1,147.2 305.3,133.2 341.5,128.3 377.6,118.8 413.8,91.0 450.0,81.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.6 88.2,143.5 124.4,178.8 160.5,178.1 196.7,161.9 232.9,160.4 269.1,141.4 305.3,128.0 341.5,124.9 377.6,105.4 413.8,78.8 450.0,67.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.8 ns | 2.24 µs | 3.84 µs | 4.04 µs | 4.28 µs |
| D38 | 13.4 ns | 3.84 µs | 4.42 µs | 5.38 µs | 5.36 µs |
| D57 | 67.3 ns | 4.61 µs | 3.79 µs | 4.28 µs | 5.12 µs |
| D76 | 56.6 ns | 4.51 µs | 5.09 µs | 5.66 µs | 5.66 µs |
| D115 | 144 ns | 8.16 µs | 9.5 µs | 11 µs | 9.88 µs |
| D153 | 198 ns | 6.47 µs | 6.07 µs | 11.8 µs | 10.5 µs |
| D230 | 248 ns | 13.5 µs | 16.3 µs | 18.1 µs | 23 µs |
| D307 | 362 ns | 16.3 µs | 21.4 µs | 28.6 µs | 34.7 µs |
| D462 | 650 ns | 47.2 µs | 75.6 µs | 209 µs | 268 µs |
| D616 | 801 ns | 192 µs | 324 µs | 233 µs | 450 µs |
| D924 | 1 µs | 416 µs | 488 µs | 665 µs | 1.75 ms |
| D1232 | 1.68 µs | 832 µs | 801 µs | 1.53 ms | 2.66 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,206.4 88.2,205.8 124.4,182.4 160.5,184.9 196.7,171.4 232.9,166.8 269.1,163.5 305.3,158.0 341.5,149.6 377.6,146.5 413.8,143.3 450.0,135.8 450.0,29.2 413.8,35.3 377.6,54.9 341.5,62.4 305.3,92.0 269.1,98.0 232.9,109.3 196.7,110.2 160.5,118.2 124.4,119.7 88.2,119.0 52.0,122.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,206.4 88.2,205.8 124.4,182.4 160.5,184.9 196.7,171.4 232.9,166.8 269.1,163.5 305.3,158.0 341.5,149.6 377.6,146.5 413.8,143.3 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,131.7 88.2,123.9 124.4,121.2 160.5,121.5 196.7,112.9 232.9,116.3 269.1,105.7 305.3,103.0 341.5,87.5 377.6,67.3 413.8,56.0 450.0,46.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.8 88.2,121.8 124.4,124.1 160.5,119.8 196.7,110.7 232.9,117.2 269.1,102.9 305.3,99.0 341.5,80.7 377.6,59.7 413.8,53.7 450.0,46.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.1 88.2,119.0 124.4,122.3 160.5,118.2 196.7,108.6 232.9,107.6 269.1,101.4 305.3,94.8 341.5,66.0 377.6,64.4 413.8,49.2 450.0,37.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.3 88.2,119.0 124.4,119.7 160.5,118.2 196.7,110.2 232.9,109.3 269.1,98.0 305.3,92.0 341.5,62.4 377.6,54.9 413.8,35.3 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.79 ns | 13.4 ns | 18.4 ns | 29.4 ns | 28.9 ns |
| D38 | 6.89 ns | 19 ns | 30 ns | 150 ns | 191 ns |
| D57 | 81.9 ns | 148 ns | 177 ns | 360 ns | 373 ns |
| D76 | 87.3 ns | 147 ns | 310 ns | 398 ns | 618 ns |
| D115 | 98.7 ns | 223 ns | 410 ns | 788 ns | 919 ns |
| D153 | 110 ns | 250 ns | 395 ns | 986 ns | 1.27 µs |
| D230 | 123 ns | 459 ns | 1.03 µs | 1.83 µs | 2.86 µs |
| D307 | 161 ns | 587 ns | 1.6 µs | 2.77 µs | 5.16 µs |
| D462 | 208 ns | 696 ns | 1.64 µs | 5.86 µs | 6.76 µs |
| D616 | 236 ns | 1.93 µs | 4.65 µs | 4.64 µs | 10.5 µs |
| D924 | 246 ns | 2.71 µs | 7.5 µs | 12.7 µs | 26.2 µs |
| D1232 | 443 ns | 5.16 µs | 13 µs | 19.1 µs | 44.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.8 88.2,176.5 124.4,133.5 160.5,132.4 196.7,130.2 232.9,128.3 269.1,126.4 305.3,121.8 341.5,117.3 377.6,115.1 413.8,114.4 450.0,104.1 450.0,24.0 413.8,33.2 377.6,49.1 341.5,56.8 305.3,61.5 269.1,71.7 232.9,85.9 196.7,91.5 160.5,98.3 124.4,107.2 88.2,118.7 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.8 88.2,176.5 124.4,133.5 160.5,132.4 196.7,130.2 232.9,128.3 269.1,126.4 305.3,121.8 341.5,117.3 377.6,115.1 413.8,114.4 450.0,104.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,158.9 124.4,123.1 160.5,123.3 196.7,116.1 232.9,114.1 269.1,103.5 305.3,99.3 341.5,96.3 377.6,78.6 413.8,72.7 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,150.9 124.4,120.1 160.5,110.4 196.7,105.5 232.9,106.1 269.1,89.5 305.3,81.8 341.5,81.4 377.6,63.3 413.8,55.0 450.0,45.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,123.0 124.4,107.8 160.5,106.0 196.7,94.1 232.9,90.2 269.1,79.5 305.3,72.3 341.5,59.3 377.6,63.4 413.8,45.9 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,118.7 124.4,107.2 160.5,98.3 196.7,91.5 232.9,85.9 269.1,71.7 305.3,61.5 341.5,56.8 377.6,49.1 413.8,33.2 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
