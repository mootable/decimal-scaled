# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 125 ns | 103 ns | 153 ns | 390 ns | 319 ns |
| D38 | 239 ns | 264 ns | 287 ns | 386 ns | 576 ns |
| D57 | 341 ns | 402 ns | 703 ns | 1.14 µs | 1.14 µs |
| D76 | 450 ns | 787 ns | 1.14 µs | 1.36 µs | 1.27 µs |
| D115 | 224 ns | 766 ns | 2.36 µs | 3.52 µs | 3.75 µs |
| D153 | 246 ns | 879 ns | 2.61 µs | 5.61 µs | 6.67 µs |
| D230 | 295 ns | 1.47 µs | 7.88 µs | 10.1 µs | 9.94 µs |
| D307 | 304 ns | 1.48 µs | 11.1 µs | 16.5 µs | 22.6 µs |
| D462 | 400 ns | 3.14 µs | 24.9 µs | 35.6 µs | 51.7 µs |
| D616 | 498 ns | 4.5 µs | 40.1 µs | 57.4 µs | 77 µs |
| D924 | 531 ns | 8.8 µs | 89.6 µs | 103 µs | 180 µs |
| D1232 | 973 ns | 15.4 µs | 151 µs | 141 µs | 311 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,205.1 88.2,191.1 124.4,183.3 160.5,177.3 196.7,192.5 232.9,190.5 269.1,186.5 305.3,185.8 341.5,179.9 377.6,175.1 413.8,173.8 450.0,160.6 450.0,35.4 413.8,47.2 377.6,65.7 341.5,74.3 305.3,92.3 269.1,110.1 232.9,118.8 196.7,131.3 160.5,154.8 124.4,157.2 88.2,172.0 52.0,184.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,205.1 88.2,191.1 124.4,183.3 160.5,177.3 196.7,192.5 232.9,190.5 269.1,186.5 305.3,185.8 341.5,179.9 377.6,175.1 413.8,173.8 450.0,160.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,209.4 88.2,188.9 124.4,179.8 160.5,165.2 196.7,165.8 232.9,162.8 269.1,151.7 305.3,151.5 341.5,135.1 377.6,127.3 413.8,112.8 450.0,100.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.7 88.2,187.1 124.4,167.7 160.5,157.1 196.7,141.3 232.9,139.2 269.1,115.2 305.3,107.8 341.5,90.2 377.6,79.9 413.8,62.4 450.0,51.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.4 88.2,180.7 124.4,157.2 160.5,153.4 196.7,132.6 232.9,122.6 269.1,109.8 305.3,99.1 341.5,82.4 377.6,72.0 413.8,59.3 450.0,52.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.8 88.2,172.0 124.4,157.2 160.5,154.8 196.7,131.3 232.9,118.8 269.1,110.1 305.3,92.3 341.5,74.3 377.6,65.7 413.8,47.2 450.0,35.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.56 ns | 1.01 µs | 3.24 µs | 3.5 µs | 4.01 µs |
| D38 | 1.87 ns | 3.19 µs | 2.33 µs | 2.97 µs | 4.12 µs |
| D57 | 2.81 ns | 3.24 µs | 4.42 µs | 5.71 µs | 9.23 µs |
| D76 | 3.43 ns | 5.89 µs | 7.8 µs | 9.52 µs | 8.31 µs |
| D115 | 11.8 ns | 6.64 µs | 14.5 µs | 17.6 µs | 17 µs |
| D153 | 16.2 ns | 7.19 µs | 12.9 µs | 22.1 µs | 34.7 µs |
| D230 | 46 ns | 13.1 µs | 22.3 µs | 42.3 µs | 58.1 µs |
| D307 | 68.4 ns | 13.6 µs | 35.4 µs | 79.4 µs | 123 µs |
| D462 | 127 ns | 22.7 µs | 81 µs | 165 µs | 247 µs |
| D616 | 149 ns | 35.3 µs | 142 µs | 267 µs | 412 µs |
| D924 | 169 ns | 77.2 µs | 290 µs | 499 µs | 916 µs |
| D1232 | 381 ns | 133 µs | 443 µs | 639 µs | 2.66 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.3 88.2,202.2 124.4,197.2 160.5,194.7 196.7,179.3 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.9 377.6,147.9 413.8,146.4 450.0,136.3 450.0,26.5 413.8,39.7 377.6,49.6 341.5,55.9 305.3,64.5 269.1,73.9 232.9,80.3 196.7,89.2 160.5,98.0 124.4,96.7 88.2,106.7 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.3 88.2,202.2 124.4,197.2 160.5,194.7 196.7,179.3 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.9 377.6,147.9 413.8,146.4 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.1 88.2,109.9 124.4,109.7 160.5,102.3 196.7,100.8 232.9,99.8 269.1,92.3 305.3,91.9 341.5,85.5 377.6,80.1 413.8,70.4 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,113.8 124.4,105.8 160.5,98.8 196.7,91.1 232.9,92.6 269.1,85.8 305.3,80.0 341.5,69.8 377.6,62.8 413.8,53.9 450.0,48.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.8 88.2,110.8 124.4,102.7 160.5,96.3 196.7,88.7 232.9,85.9 269.1,77.8 305.3,70.0 341.5,61.0 377.6,55.0 413.8,47.2 450.0,44.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,106.7 124.4,96.7 160.5,98.0 196.7,89.2 232.9,80.3 269.1,73.9 305.3,64.5 341.5,55.9 377.6,49.6 413.8,39.7 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.9 ns | 25.5 ns | 41 ns | 39.2 ns | 40.1 ns |
| D38 | 15.9 ns | 42 ns | 46.6 ns | 52.6 ns | 83.7 ns |
| D57 | 17.6 ns | 32.5 ns | 68 ns | 373 ns | 393 ns |
| D76 | 17.9 ns | 68.6 ns | 392 ns | 376 ns | 488 ns |
| D115 | 22.6 ns | 70.5 ns | 412 ns | 789 ns | 789 ns |
| D153 | 25.3 ns | 387 ns | 549 ns | 962 ns | 1.58 µs |
| D230 | 28.7 ns | 422 ns | 1.08 µs | 1.88 µs | 2.19 µs |
| D307 | 44.9 ns | 590 ns | 1.63 µs | 2.81 µs | 5.3 µs |
| D462 | 63.7 ns | 1.13 µs | 3.15 µs | 6.09 µs | 7.86 µs |
| D616 | 73.1 ns | 1.95 µs | 5.9 µs | 10.2 µs | 13 µs |
| D924 | 93.5 ns | 3.29 µs | 11 µs | 19.9 µs | 23.6 µs |
| D1232 | 121 ns | 5.59 µs | 20.4 µs | 18.4 µs | 45 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.5 88.2,199.9 124.4,197.8 160.5,197.3 196.7,192.3 232.9,189.8 269.1,187.1 305.3,177.4 341.5,169.8 377.6,166.8 413.8,161.5 450.0,155.9 450.0,27.3 413.8,41.4 377.6,54.4 341.5,65.2 305.3,73.8 269.1,93.0 232.9,100.1 196.7,115.1 160.5,125.6 124.4,130.3 88.2,163.9 52.0,179.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.5 88.2,199.9 124.4,197.8 160.5,197.3 196.7,192.3 232.9,189.8 269.1,187.1 305.3,177.4 341.5,169.8 377.6,166.8 413.8,161.5 450.0,155.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,189.7 88.2,178.8 124.4,184.4 160.5,168.2 196.7,167.6 232.9,130.6 269.1,128.7 305.3,121.5 341.5,107.3 377.6,95.5 413.8,84.1 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.4 88.2,176.6 124.4,168.4 160.5,130.3 196.7,129.3 232.9,123.0 269.1,108.3 305.3,99.4 341.5,85.1 377.6,71.5 413.8,57.9 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.3 88.2,174.0 124.4,131.4 160.5,131.3 196.7,115.2 232.9,110.8 269.1,96.3 305.3,87.5 341.5,70.8 377.6,59.5 413.8,45.1 450.0,46.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.8 88.2,163.9 124.4,130.3 160.5,125.6 196.7,115.1 232.9,100.1 269.1,93.0 305.3,73.8 341.5,65.2 377.6,54.4 413.8,41.4 450.0,27.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.1 ns | 170 ns | 387 ns | 375 ns | 410 ns |
| D38 | 2.18 ns | 373 ns | 263 ns | 308 ns | 336 ns |
| D57 | 282 ns | 399 ns | 481 ns | 490 ns | 617 ns |
| D76 | 286 ns | 482 ns | 512 ns | 631 ns | 419 ns |
| D115 | 295 ns | 489 ns | 718 ns | 951 ns | 806 ns |
| D153 | 294 ns | 519 ns | 510 ns | 992 ns | 1.29 µs |
| D230 | 562 ns | 649 ns | 1.07 µs | 1.3 µs | 1.43 µs |
| D307 | 645 ns | 624 ns | 1.1 µs | 1.34 µs | 11.3 µs |
| D462 | 1.32 µs | 2.79 µs | 3.34 µs | 4.33 µs | 5.41 µs |
| D616 | 1.54 µs | 1.4 µs | 2.06 µs | 2.89 µs | 3.72 µs |
| D924 | 2.02 µs | 1.77 µs | 3.14 µs | 3.19 µs | 4.74 µs |
| D1232 | 3.23 µs | 3.16 µs | 4.43 µs | 3.26 µs | 6.77 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.4 88.2,196.5 124.4,112.0 160.5,111.8 196.7,111.2 232.9,111.3 269.1,100.0 305.3,97.6 341.5,85.2 377.6,82.5 413.8,77.7 450.0,69.6 450.0,56.8 413.8,63.0 377.6,67.2 341.5,60.7 305.3,47.9 269.1,83.8 232.9,85.6 196.7,93.7 160.5,105.1 124.4,98.4 88.2,108.9 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.4 88.2,196.5 124.4,112.0 160.5,111.8 196.7,111.2 232.9,111.3 269.1,100.0 305.3,97.6 341.5,85.2 377.6,82.5 413.8,77.7 450.0,69.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.8 88.2,107.2 124.4,106.0 160.5,102.7 196.7,102.4 232.9,101.4 269.1,97.5 305.3,98.2 341.5,72.1 377.6,84.1 413.8,80.1 450.0,70.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,113.2 124.4,102.7 160.5,101.6 196.7,95.7 232.9,101.7 269.1,88.9 305.3,88.4 341.5,69.1 377.6,77.4 413.8,70.1 450.0,64.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,110.4 124.4,102.4 160.5,98.0 196.7,90.9 232.9,90.1 269.1,85.5 305.3,84.9 341.5,64.5 377.6,71.6 413.8,69.9 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,108.9 124.4,98.4 160.5,105.1 196.7,93.7 232.9,85.6 269.1,83.8 305.3,47.9 341.5,60.7 377.6,67.2 413.8,63.0 450.0,56.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.3 µs | 1.98 µs | 5.85 µs | 6.32 µs | 7.29 µs |
| D38 | 2.74 µs | 5.75 µs | 4.57 µs | 6.16 µs | 8.54 µs |
| D57 | 2.86 µs | 2.18 µs | 2.7 µs | 2.85 µs | 2.97 µs |
| D76 | 2.53 µs | 2.65 µs | 3 µs | 2.87 µs | 2.01 µs |
| D115 | 4.59 µs | 4.95 µs | 5.64 µs | 5.56 µs | 4.74 µs |
| D153 | 4.5 µs | 5.06 µs | 4.29 µs | 5.86 µs | 6.27 µs |
| D230 | 6.58 µs | 7.17 µs | 8.31 µs | 8.74 µs | 7.15 µs |
| D307 | 9.53 µs | 9.27 µs | 12.7 µs | 14.1 µs | 16.7 µs |
| D462 | 10.1 µs | 12.6 µs | 14.5 µs | 17.9 µs | 20.1 µs |
| D616 | 17.2 µs | 23.4 µs | 32.7 µs | 35.5 µs | 41.4 µs |
| D924 | 24.1 µs | 35.6 µs | 59.9 µs | 62.8 µs | 85.3 µs |
| D1232 | 33.7 µs | 61.5 µs | 96.1 µs | 75.3 µs | 141 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.9 88.2,180.8 124.4,179.6 160.5,183.2 196.7,165.9 232.9,166.4 269.1,155.5 305.3,144.7 341.5,143.0 377.6,127.7 413.8,117.8 450.0,108.2 450.0,66.7 413.8,81.3 377.6,102.2 341.5,123.1 305.3,128.5 269.1,153.0 232.9,156.8 196.7,165.0 160.5,189.7 124.4,178.5 88.2,147.9 52.0,152.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.9 88.2,180.8 124.4,179.6 160.5,183.2 196.7,165.9 232.9,166.4 269.1,155.5 305.3,144.7 341.5,143.0 377.6,127.7 413.8,117.8 450.0,108.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,190.2 88.2,159.3 124.4,187.4 160.5,181.8 196.7,163.7 232.9,163.1 269.1,153.0 305.3,145.5 341.5,136.7 377.6,118.7 413.8,106.6 450.0,90.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,166.0 124.4,181.2 160.5,178.2 196.7,159.9 232.9,167.8 269.1,148.7 305.3,136.4 341.5,132.6 377.6,109.0 413.8,91.5 450.0,77.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.6 88.2,157.3 124.4,179.7 160.5,179.4 196.7,160.3 232.9,158.8 269.1,147.2 305.3,133.3 341.5,126.5 377.6,106.7 413.8,90.1 450.0,84.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,147.9 124.4,178.5 160.5,189.7 196.7,165.0 232.9,156.8 269.1,153.0 305.3,128.5 341.5,123.1 377.6,102.2 413.8,81.3 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.12 ns | 1.33 µs | 3.9 µs | 4.07 µs | 4.69 µs |
| D38 | 6.54 ns | 3.81 µs | 2.79 µs | 3.48 µs | 4.69 µs |
| D57 | 63.7 ns | 3.62 µs | 4.58 µs | 4.72 µs | 5.17 µs |
| D76 | 74.7 ns | 4.08 µs | 5.16 µs | 5.25 µs | 4.13 µs |
| D115 | 146 ns | 8.33 µs | 9.7 µs | 10.4 µs | 8.57 µs |
| D153 | 176 ns | 8.62 µs | 8 µs | 11 µs | 12 µs |
| D230 | 347 ns | 12.8 µs | 15.2 µs | 18.3 µs | 16 µs |
| D307 | 377 ns | 16.2 µs | 21.8 µs | 28.7 µs | 34.7 µs |
| D462 | 683 ns | 71.1 µs | 138 µs | 223 µs | 285 µs |
| D616 | 810 ns | 176 µs | 350 µs | 355 µs | 531 µs |
| D924 | 943 ns | 424 µs | 490 µs | 696 µs | 1.62 ms |
| D1232 | 1.49 µs | 775 µs | 863 µs | 1.45 ms | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.6 88.2,186.7 124.4,158.4 160.5,156.5 196.7,148.2 232.9,145.9 269.1,137.4 305.3,136.4 341.5,129.0 377.6,126.9 413.8,125.0 450.0,119.3 450.0,26.1 413.8,32.6 377.6,46.4 341.5,54.2 305.3,80.3 269.1,89.9 232.9,93.4 196.7,97.6 160.5,106.7 124.4,103.9 88.2,105.1 52.0,105.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.6 88.2,186.7 124.4,158.4 160.5,156.5 196.7,148.2 232.9,145.9 269.1,137.4 305.3,136.4 341.5,129.0 377.6,126.9 413.8,125.0 450.0,119.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,120.8 88.2,107.7 124.4,108.3 160.5,106.8 196.7,98.0 232.9,97.6 269.1,92.7 305.3,89.7 341.5,71.4 377.6,60.1 413.8,49.2 450.0,41.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,111.6 124.4,105.4 160.5,103.9 196.7,96.1 232.9,98.5 269.1,90.5 305.3,86.1 341.5,63.1 377.6,51.6 413.8,47.4 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,108.8 124.4,105.0 160.5,103.7 196.7,95.3 232.9,94.5 269.1,88.2 305.3,82.6 341.5,57.2 377.6,51.4 413.8,43.1 450.0,33.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.1 88.2,105.1 124.4,103.9 160.5,106.7 196.7,97.6 232.9,93.4 269.1,89.9 305.3,80.3 341.5,54.2 377.6,46.4 413.8,32.6 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.53 ns | 8.72 ns | 18.4 ns | 29.3 ns | 30.2 ns |
| D38 | 6.98 ns | 19.7 ns | 21.9 ns | 238 ns | 394 ns |
| D57 | 177 ns | 200 ns | 492 ns | 772 ns | 770 ns |
| D76 | 193 ns | 268 ns | 683 ns | 834 ns | 741 ns |
| D115 | 92 ns | 656 ns | 920 ns | 1.52 µs | 1.12 µs |
| D153 | 107 ns | 1.09 µs | 1.15 µs | 1.88 µs | 2.66 µs |
| D230 | 146 ns | 427 ns | 2.37 µs | 3.4 µs | 3.14 µs |
| D307 | 144 ns | 2 µs | 3.75 µs | 4.76 µs | 7.26 µs |
| D462 | 205 ns | 3.73 µs | 5.99 µs | 9.83 µs | 11.4 µs |
| D616 | 256 ns | 1.84 µs | 11 µs | 13.9 µs | 19.9 µs |
| D924 | 247 ns | 2.57 µs | 17 µs | 22.4 µs | 32.6 µs |
| D1232 | 389 ns | 15.3 µs | 28 µs | 26.7 µs | 58.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.8 88.2,176.2 124.4,120.1 160.5,118.6 196.7,131.4 232.9,128.8 269.1,123.4 305.3,123.7 341.5,117.6 377.6,113.6 413.8,114.3 450.0,106.4 450.0,19.3 413.8,29.5 377.6,38.0 341.5,47.7 305.3,55.6 269.1,70.1 232.9,73.0 196.7,88.1 160.5,95.2 124.4,94.5 88.2,106.2 52.0,150.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.8 88.2,176.2 124.4,120.1 160.5,118.6 196.7,131.4 232.9,128.8 269.1,123.4 305.3,123.7 341.5,117.6 377.6,113.6 413.8,114.3 450.0,106.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,172.4 88.2,158.2 124.4,118.0 160.5,112.9 196.7,97.3 232.9,88.4 269.1,104.8 305.3,77.9 341.5,67.1 377.6,79.4 413.8,73.6 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,156.4 124.4,102.3 160.5,96.6 196.7,91.4 232.9,87.6 269.1,75.0 305.3,67.0 341.5,58.9 377.6,48.4 413.8,40.8 450.0,32.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.4 88.2,115.0 124.4,94.5 160.5,93.2 196.7,82.8 232.9,79.0 269.1,68.7 305.3,62.9 341.5,50.3 377.6,44.3 413.8,36.0 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.8 88.2,106.2 124.4,94.5 160.5,95.2 196.7,88.1 232.9,73.0 269.1,70.1 305.3,55.6 341.5,47.7 377.6,38.0 413.8,29.5 450.0,19.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
