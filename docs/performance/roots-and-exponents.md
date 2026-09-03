# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 136 ns | 159 ns | 392 ns | 392 ns |
| D38 | 236 ns | 201 ns | 293 ns | 597 ns | 554 ns |
| D57 | 320 ns | 307 ns | 387 ns | 608 ns | 702 ns |
| D76 | 446 ns | 444 ns | 1.18 µs | 1.24 µs | 1.27 µs |
| D115 | 228 ns | 754 ns | 2.37 µs | 3.55 µs | 5.26 µs |
| D153 | 244 ns | 857 ns | 3.45 µs | 5.66 µs | 6.8 µs |
| D230 | 280 ns | 1.38 µs | 8.17 µs | 10.6 µs | 14.2 µs |
| D307 | 317 ns | 1.19 µs | 7.11 µs | 16.3 µs | 20.6 µs |
| D462 | 400 ns | 3.16 µs | 24.1 µs | 34.3 µs | 47.2 µs |
| D616 | 380 ns | 4.51 µs | 40.2 µs | 62.9 µs | 67.4 µs |
| D924 | 532 ns | 9.64 µs | 82.1 µs | 121 µs | 198 µs |
| D1232 | 1.16 µs | 15.5 µs | 138 µs | 195 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.2 88.2,191.3 124.4,184.7 160.5,177.5 196.7,192.1 232.9,190.6 269.1,187.6 305.3,184.9 341.5,179.9 377.6,181.0 413.8,173.7 450.0,156.8 450.0,35.2 413.8,45.1 377.6,68.6 341.5,76.3 305.3,94.3 269.1,102.4 232.9,118.4 196.7,124.0 160.5,154.8 124.4,167.7 88.2,172.8 52.0,180.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.2 88.2,191.3 124.4,184.7 160.5,177.5 196.7,192.1 232.9,190.6 269.1,187.6 305.3,184.9 341.5,179.9 377.6,181.0 413.8,173.7 450.0,156.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,203.3 88.2,194.9 124.4,185.6 160.5,177.7 196.7,166.1 232.9,163.3 269.1,153.0 305.3,156.2 341.5,135.0 377.6,127.3 413.8,110.8 450.0,100.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,200.0 88.2,186.6 124.4,180.6 160.5,156.5 196.7,141.3 232.9,133.1 269.1,114.4 305.3,117.4 341.5,90.9 377.6,79.8 413.8,64.3 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.3 88.2,171.2 124.4,170.8 160.5,155.4 196.7,132.5 232.9,122.4 269.1,108.8 305.3,99.4 341.5,83.3 377.6,70.1 413.8,55.9 450.0,45.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.3 88.2,172.8 124.4,167.7 160.5,154.8 196.7,124.0 232.9,118.4 269.1,102.4 305.3,94.3 341.5,76.3 377.6,68.6 413.8,45.1 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.77 µs | 3.51 µs | 3.42 µs | 3.65 µs |
| D38 | 1.56 ns | 2.02 µs | 2.38 µs | 4.31 µs | 3.78 µs |
| D57 | 2.18 ns | 2.13 µs | 2.71 µs | 3.44 µs | 6.55 µs |
| D76 | 3.43 ns | 3.8 µs | 7.27 µs | 8.66 µs | 8.37 µs |
| D115 | 13.4 ns | 6.66 µs | 13.7 µs | 18.2 µs | 22.6 µs |
| D153 | 18 ns | 7.3 µs | 15.5 µs | 22 µs | 38.7 µs |
| D230 | 39.7 ns | 13.1 µs | 23.7 µs | 46.1 µs | 78.5 µs |
| D307 | 98.4 ns | 10.3 µs | 22.6 µs | 79.3 µs | 117 µs |
| D462 | 133 ns | 22 µs | 80 µs | 159 µs | 232 µs |
| D616 | 110 ns | 34.2 µs | 140 µs | 286 µs | 402 µs |
| D924 | 192 ns | 85.8 µs | 268 µs | 557 µs | 992 µs |
| D1232 | 381 ns | 133 µs | 411 µs | 773 µs | 2.68 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,204.5 124.4,200.3 160.5,194.7 196.7,177.8 232.9,174.2 269.1,164.3 305.3,153.1 341.5,149.3 377.6,151.7 413.8,144.8 450.0,136.2 450.0,26.3 413.8,38.7 377.6,49.9 341.5,56.7 305.3,65.2 269.1,70.1 232.9,78.9 196.7,85.6 160.5,97.9 124.4,101.0 88.2,107.8 52.0,108.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,204.5 124.4,200.3 160.5,194.7 196.7,177.8 232.9,174.2 269.1,164.3 305.3,153.1 341.5,149.3 377.6,151.7 413.8,144.8 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,117.2 88.2,115.5 124.4,114.9 160.5,107.7 196.7,100.8 232.9,99.6 269.1,92.4 305.3,95.3 341.5,85.9 377.6,80.4 413.8,69.0 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.7 88.2,113.5 124.4,111.9 160.5,99.7 196.7,91.8 232.9,90.3 269.1,85.0 305.3,85.6 341.5,69.9 377.6,63.0 413.8,54.9 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.0 88.2,106.2 124.4,108.9 160.5,97.5 196.7,88.3 232.9,86.0 269.1,76.8 305.3,70.0 341.5,61.4 377.6,54.1 413.8,45.8 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.2 88.2,107.8 124.4,101.0 160.5,97.9 196.7,85.6 232.9,78.9 269.1,70.1 305.3,65.2 341.5,56.7 377.6,49.9 413.8,38.7 450.0,26.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17 ns | 39.7 ns | 42.9 ns | 41.1 ns | 41.4 ns |
| D38 | 15.9 ns | 27.2 ns | 48.1 ns | 67.2 ns | 92.6 ns |
| D57 | 17 ns | 26.7 ns | 49.3 ns | 259 ns | 306 ns |
| D76 | 18.2 ns | 56.4 ns | 371 ns | 352 ns | 490 ns |
| D115 | 20.2 ns | 71.7 ns | 414 ns | 789 ns | 960 ns |
| D153 | 23.1 ns | 390 ns | 631 ns | 976 ns | 1.72 µs |
| D230 | 29.3 ns | 432 ns | 1.14 µs | 2.01 µs | 2.82 µs |
| D307 | 43.8 ns | 444 ns | 1.1 µs | 2.82 µs | 4.93 µs |
| D462 | 64.3 ns | 1.14 µs | 3.14 µs | 5.98 µs | 7.22 µs |
| D616 | 59.3 ns | 1.95 µs | 5.94 µs | 10.9 µs | 13.6 µs |
| D924 | 95.6 ns | 3.47 µs | 10.4 µs | 22.5 µs | 26.9 µs |
| D1232 | 102 ns | 5.62 µs | 18.7 µs | 20.1 µs | 45.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,199.9 124.4,198.5 160.5,197.0 196.7,194.7 232.9,191.8 269.1,186.6 305.3,177.9 341.5,169.6 377.6,171.4 413.8,161.0 450.0,159.6 450.0,27.2 413.8,38.5 377.6,53.3 341.5,67.1 305.3,75.4 269.1,87.5 232.9,98.2 196.7,110.9 160.5,125.5 124.4,135.7 88.2,161.7 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,199.9 124.4,198.5 160.5,197.0 196.7,194.7 232.9,191.8 269.1,186.6 305.3,177.9 341.5,169.6 377.6,171.4 413.8,161.0 450.0,159.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.0 88.2,188.2 124.4,188.7 160.5,172.4 196.7,167.2 232.9,130.5 269.1,128.2 305.3,127.6 341.5,107.2 377.6,95.5 413.8,83.0 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,175.9 124.4,175.4 160.5,131.5 196.7,129.1 232.9,120.0 269.1,107.1 305.3,107.9 341.5,85.1 377.6,71.3 413.8,59.1 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.3 88.2,168.6 124.4,139.3 160.5,132.7 196.7,115.2 232.9,110.5 269.1,94.9 305.3,87.5 341.5,71.1 377.6,58.1 413.8,42.3 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,161.7 124.4,135.7 160.5,125.5 196.7,110.9 232.9,98.2 269.1,87.5 305.3,75.4 341.5,67.1 377.6,53.3 413.8,38.5 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.67 ns | 300 ns | 399 ns | 376 ns | 383 ns |
| D38 | 2.18 ns | 257 ns | 270 ns | 382 ns | 351 ns |
| D57 | 272 ns | 293 ns | 308 ns | 306 ns | 386 ns |
| D76 | 280 ns | 293 ns | 500 ns | 557 ns | 415 ns |
| D115 | 314 ns | 490 ns | 720 ns | 975 ns | 998 ns |
| D153 | 351 ns | 529 ns | 644 ns | 993 ns | 1.42 µs |
| D230 | 516 ns | 660 ns | 1.17 µs | 1.45 µs | 1.83 µs |
| D307 | 764 ns | 403 ns | 738 ns | 1.37 µs | 10.6 µs |
| D462 | 1.32 µs | 2.81 µs | 3.31 µs | 4.39 µs | 5.01 µs |
| D616 | 1.27 µs | 1.4 µs | 2.08 µs | 2.99 µs | 3.11 µs |
| D924 | 2 µs | 2.18 µs | 2.92 µs | 3.72 µs | 5.17 µs |
| D1232 | 3.11 µs | 3.12 µs | 4.21 µs | 4.34 µs | 6.72 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.2 88.2,196.5 124.4,112.6 160.5,112.1 196.7,110.1 232.9,108.2 269.1,101.5 305.3,94.7 341.5,85.2 377.6,85.9 413.8,77.9 450.0,70.3 450.0,56.9 413.8,61.5 377.6,70.3 341.5,62.0 305.3,49.0 269.1,79.5 232.9,83.9 196.7,90.0 160.5,105.3 124.4,106.5 88.2,108.2 52.0,106.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.2 88.2,196.5 124.4,112.6 160.5,112.1 196.7,110.1 232.9,108.2 269.1,101.5 305.3,94.7 341.5,85.2 377.6,85.9 413.8,77.9 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.9 88.2,113.6 124.4,111.3 160.5,111.3 196.7,102.4 232.9,101.1 269.1,97.2 305.3,105.8 341.5,72.1 377.6,84.2 413.8,76.5 450.0,70.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.0 88.2,112.7 124.4,110.5 160.5,102.0 196.7,95.7 232.9,97.7 269.1,87.2 305.3,95.3 341.5,69.2 377.6,77.3 413.8,71.4 450.0,65.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,106.7 124.4,110.6 160.5,100.2 196.7,90.4 232.9,90.1 269.1,83.6 305.3,84.5 341.5,64.3 377.6,71.0 413.8,67.2 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.7 88.2,108.2 124.4,106.5 160.5,105.3 196.7,90.0 232.9,83.9 269.1,79.5 305.3,49.0 341.5,62.0 377.6,70.3 413.8,61.5 450.0,56.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.76 µs | 3.25 µs | 6.07 µs | 6.31 µs | 6.96 µs |
| D38 | 2.74 µs | 3.79 µs | 4.67 µs | 8.6 µs | 8.05 µs |
| D57 | 2.51 µs | 1.35 µs | 1.48 µs | 1.47 µs | 1.88 µs |
| D76 | 2.47 µs | 1.44 µs | 2.73 µs | 2.64 µs | 1.98 µs |
| D115 | 4.72 µs | 4.98 µs | 5.39 µs | 5.51 µs | 5.7 µs |
| D153 | 4.82 µs | 4.98 µs | 5.31 µs | 5.78 µs | 6.87 µs |
| D230 | 6.24 µs | 7.18 µs | 8.72 µs | 9.45 µs | 9.86 µs |
| D307 | 10.1 µs | 6.96 µs | 8.01 µs | 14.1 µs | 15.6 µs |
| D462 | 10.2 µs | 12.3 µs | 14.2 µs | 17.2 µs | 18.4 µs |
| D616 | 13.5 µs | 23.4 µs | 32.3 µs | 39 µs | 37.8 µs |
| D924 | 24 µs | 42.6 µs | 55.3 µs | 70.9 µs | 92.2 µs |
| D1232 | 32.6 µs | 61.8 µs | 89.1 µs | 98.9 µs | 141 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.6 88.2,180.8 124.4,183.3 160.5,183.8 196.7,165.0 232.9,164.5 269.1,157.0 305.3,143.2 341.5,142.8 377.6,134.7 413.8,118.0 450.0,109.1 450.0,66.7 413.8,79.0 377.6,104.8 341.5,125.7 305.3,130.5 269.1,143.8 232.9,154.2 196.7,159.6 160.5,190.2 124.4,191.7 88.2,149.6 52.0,153.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.6 88.2,180.8 124.4,183.3 160.5,183.8 196.7,165.0 232.9,164.5 269.1,157.0 305.3,143.2 341.5,142.8 377.6,134.7 413.8,118.0 450.0,109.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,175.9 88.2,171.4 124.4,201.3 160.5,199.5 196.7,163.5 232.9,163.5 269.1,152.9 305.3,153.8 341.5,137.4 377.6,118.7 413.8,101.4 450.0,90.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.8 88.2,165.4 124.4,198.6 160.5,180.9 196.7,161.2 232.9,161.6 269.1,147.3 305.3,149.7 341.5,133.1 377.6,109.4 413.8,93.8 450.0,80.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.7 88.2,147.7 124.4,198.9 160.5,181.8 196.7,160.6 232.9,159.2 269.1,145.0 305.3,133.4 341.5,127.6 377.6,103.9 413.8,86.6 450.0,77.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,149.6 124.4,191.7 160.5,190.2 196.7,159.6 232.9,154.2 269.1,143.8 305.3,130.5 341.5,125.7 377.6,104.8 413.8,79.0 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 2.24 µs | 4.2 µs | 4.05 µs | 4.29 µs |
| D38 | 6.54 ns | 2.47 µs | 2.84 µs | 4.93 µs | 4.41 µs |
| D57 | 56.5 ns | 2.36 µs | 2.6 µs | 2.64 µs | 3.58 µs |
| D76 | 74.2 ns | 2.3 µs | 4.85 µs | 4.82 µs | 4.09 µs |
| D115 | 146 ns | 8.22 µs | 9.63 µs | 10.3 µs | 10.7 µs |
| D153 | 198 ns | 8.47 µs | 9.63 µs | 10.9 µs | 13.1 µs |
| D230 | 282 ns | 12.6 µs | 16.5 µs | 19.8 µs | 21.3 µs |
| D307 | 451 ns | 12.5 µs | 13.6 µs | 28.6 µs | 32.3 µs |
| D462 | 718 ns | 70.5 µs | 138 µs | 216 µs | 266 µs |
| D616 | 617 ns | 176 µs | 353 µs | 380 µs | 515 µs |
| D924 | 927 ns | 453 µs | 459 µs | 791 µs | 1.74 ms |
| D1232 | 1.46 µs | 763 µs | 796 µs | 1.8 ms | 2.72 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,156.6 196.7,148.1 232.9,144.4 269.1,140.0 305.3,134.2 341.5,128.4 377.6,130.3 413.8,125.2 450.0,119.6 450.0,26.1 413.8,31.7 377.6,46.8 341.5,55.0 305.3,81.2 269.1,86.3 232.9,92.4 196.7,94.9 160.5,106.8 124.4,108.5 88.2,105.9 52.0,106.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,156.6 196.7,148.1 232.9,144.4 269.1,140.0 305.3,134.2 341.5,128.4 377.6,130.3 413.8,125.2 450.0,119.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,114.3 88.2,113.1 124.4,113.6 160.5,113.9 196.7,98.1 232.9,97.8 269.1,92.8 305.3,93.0 341.5,71.5 377.6,60.1 413.8,48.4 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,111.4 124.4,112.4 160.5,104.7 196.7,96.2 232.9,96.2 269.1,89.5 305.3,91.9 341.5,63.2 377.6,51.5 413.8,48.2 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,104.5 124.4,112.2 160.5,104.8 196.7,95.3 232.9,94.6 269.1,87.2 305.3,82.7 341.5,57.6 377.6,50.6 413.8,41.5 450.0,31.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,105.9 124.4,108.5 160.5,106.8 196.7,94.9 232.9,92.4 269.1,86.3 305.3,81.2 341.5,55.0 377.6,46.8 413.8,31.7 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.91 ns | 13.5 ns | 18.5 ns | 28.6 ns | 28.9 ns |
| D38 | 7.07 ns | 14.3 ns | 22.2 ns | 419 ns | 354 ns |
| D57 | 157 ns | 157 ns | 273 ns | 425 ns | 476 ns |
| D76 | 191 ns | 155 ns | 767 ns | 778 ns | 739 ns |
| D115 | 94.7 ns | 662 ns | 922 ns | 1.51 µs | 1.68 µs |
| D153 | 105 ns | 1.1 µs | 1.6 µs | 1.87 µs | 2.68 µs |
| D230 | 135 ns | 432 ns | 2.37 µs | 3.47 µs | 4.27 µs |
| D307 | 155 ns | 1.34 µs | 2.29 µs | 4.69 µs | 6.95 µs |
| D462 | 198 ns | 3.59 µs | 5.84 µs | 9.59 µs | 11.7 µs |
| D616 | 188 ns | 1.84 µs | 11 µs | 13.8 µs | 18.3 µs |
| D924 | 240 ns | 2.85 µs | 16.5 µs | 25.1 µs | 35.4 µs |
| D1232 | 330 ns | 15.6 µs | 27.3 µs | 32.1 µs | 58.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.1 88.2,176.0 124.4,122.1 160.5,118.7 196.7,131.0 232.9,129.1 269.1,124.8 305.3,122.3 341.5,118.1 377.6,119.0 413.8,114.8 450.0,109.3 450.0,19.3 413.8,28.1 377.6,39.5 341.5,47.2 305.3,56.3 269.1,64.8 232.9,72.9 196.7,81.0 160.5,95.3 124.4,102.9 88.2,108.0 52.0,151.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.1 88.2,176.0 124.4,122.1 160.5,118.7 196.7,131.0 232.9,129.1 269.1,124.8 305.3,122.3 341.5,118.1 377.6,119.0 413.8,114.8 450.0,109.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.8 88.2,163.8 124.4,122.2 160.5,122.4 196.7,97.2 232.9,88.3 269.1,104.6 305.3,84.9 341.5,67.8 377.6,79.4 413.8,71.8 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,156.2 124.4,112.6 160.5,94.6 196.7,91.4 232.9,81.8 269.1,75.0 305.3,75.6 341.5,59.3 377.6,48.4 413.8,41.3 450.0,32.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.7 88.2,105.1 124.4,104.9 160.5,94.4 196.7,82.8 232.9,79.1 269.1,68.4 305.3,63.1 341.5,50.7 377.6,44.4 413.8,34.0 450.0,29.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.6 88.2,108.0 124.4,102.9 160.5,95.3 196.7,81.0 232.9,72.9 269.1,64.8 305.3,56.3 341.5,47.2 377.6,39.5 413.8,28.1 450.0,19.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
