# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 64.3 ns | 132 ns | 158 ns | 254 ns | 263 ns |
| D38 | 80.2 ns | 156 ns | 255 ns | 379 ns | 366 ns |
| D57 | 192 ns | 428 ns | 513 ns | 595 ns | 817 ns |
| D76 | 197 ns | 489 ns | 542 ns | 1.28 µs | 1.6 µs |
| D115 | 215 ns | 638 ns | 1.35 µs | 2.02 µs | 3.13 µs |
| D153 | 269 ns | 871 ns | 1.76 µs | 3.22 µs | 4.54 µs |
| D230 | 291 ns | 1.06 µs | 2.6 µs | 4.79 µs | 7.41 µs |
| D307 | 317 ns | 1.17 µs | 4.67 µs | 8.84 µs | 15 µs |
| D462 | 380 ns | 2.67 µs | 8.94 µs | 15 µs | 33.4 µs |
| D616 | 478 ns | 3.82 µs | 14.6 µs | 34.2 µs | 56.2 µs |
| D924 | 288 ns | 7.58 µs | 23.7 µs | 69.9 µs | 115 µs |
| D1232 | 968 ns | 12.6 µs | 52 µs | 102 µs | 200 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.7 88.2,173.8 124.4,158.6 160.5,158.2 196.7,156.7 232.9,152.8 269.1,151.4 305.3,149.9 341.5,146.8 377.6,142.8 413.8,151.6 450.0,130.6 450.0,38.0 413.8,47.5 377.6,60.0 341.5,69.0 305.3,83.0 269.1,95.2 232.9,103.7 196.7,110.2 160.5,121.8 124.4,133.5 88.2,147.5 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.7 88.2,173.8 124.4,158.6 160.5,158.2 196.7,156.7 232.9,152.8 269.1,151.4 305.3,149.9 341.5,146.8 377.6,142.8 413.8,151.6 450.0,130.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.1 88.2,162.2 124.4,144.8 160.5,142.4 196.7,137.8 232.9,132.4 269.1,129.0 305.3,127.2 341.5,112.9 377.6,106.7 413.8,94.8 450.0,86.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.1 88.2,153.8 124.4,141.6 160.5,140.6 196.7,124.8 232.9,120.2 269.1,113.4 305.3,103.2 341.5,91.9 377.6,83.5 413.8,75.0 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,146.9 124.4,139.0 160.5,125.7 196.7,117.8 232.9,109.7 269.1,102.8 305.3,92.1 341.5,83.0 377.6,68.7 413.8,56.2 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,147.5 124.4,133.5 160.5,121.8 196.7,110.2 232.9,103.7 269.1,95.2 305.3,83.0 341.5,69.0 377.6,60.0 413.8,47.5 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.46 ns | 1.62 µs | 3.23 µs | 3.45 µs | 3.98 µs |
| D38 | 1.59 ns | 3.19 µs | 3.79 µs | 4.67 µs | 3.9 µs |
| D57 | 2.49 ns | 3.8 µs | 3.56 µs | 3.29 µs | 7.6 µs |
| D76 | 3.46 ns | 6.34 µs | 4.34 µs | 9.59 µs | 11.4 µs |
| D115 | 10.3 ns | 4.97 µs | 13.8 µs | 18.2 µs | 23.7 µs |
| D153 | 15.8 ns | 7.24 µs | 16 µs | 23.5 µs | 38.1 µs |
| D230 | 45.9 ns | 9.77 µs | 19.5 µs | 37.2 µs | 64.2 µs |
| D307 | 85.4 ns | 10 µs | 38.1 µs | 79.5 µs | 123 µs |
| D462 | 131 ns | 17.8 µs | 80.1 µs | 130 µs | 247 µs |
| D616 | 143 ns | 30.6 µs | 132 µs | 285 µs | 443 µs |
| D924 | 107 ns | 71.2 µs | 202 µs | 604 µs | 913 µs |
| D1232 | 358 ns | 112 µs | 413 µs | 773 µs | 2.67 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.9 88.2,204.2 124.4,198.7 160.5,194.6 196.7,181.1 232.9,175.8 269.1,162.5 305.3,154.8 341.5,149.5 377.6,148.4 413.8,152.0 450.0,137.0 450.0,26.4 413.8,39.7 377.6,48.7 341.5,55.9 305.3,64.6 269.1,72.6 232.9,79.1 196.7,85.0 160.5,94.1 124.4,99.1 88.2,107.4 52.0,107.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.9 88.2,204.2 124.4,198.7 160.5,194.6 196.7,181.1 232.9,175.8 269.1,162.5 305.3,154.8 341.5,149.5 377.6,148.4 413.8,152.0 450.0,137.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,109.9 124.4,107.7 160.5,101.4 196.7,104.4 232.9,99.7 269.1,96.0 305.3,95.7 341.5,88.6 377.6,81.8 413.8,71.4 450.0,65.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,107.7 124.4,108.5 160.5,106.1 196.7,91.7 232.9,89.9 269.1,87.4 305.3,79.1 341.5,69.9 377.6,63.7 413.8,58.4 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,105.2 124.4,109.5 160.5,96.2 196.7,88.3 232.9,85.1 269.1,79.4 305.3,70.0 341.5,63.9 377.6,54.1 413.8,44.8 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.2 88.2,107.4 124.4,99.1 160.5,94.1 196.7,85.0 232.9,79.1 269.1,72.6 305.3,64.6 341.5,55.9 377.6,48.7 413.8,39.7 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 13.4 ns | 39.6 ns | 41.6 ns | 40.6 ns | 40.1 ns |
| D38 | 15.4 ns | 42 ns | 66.4 ns | 75.2 ns | 94.6 ns |
| D57 | 16.6 ns | 41.1 ns | 87 ns | 258 ns | 337 ns |
| D76 | 17.2 ns | 95.8 ns | 255 ns | 396 ns | 625 ns |
| D115 | 17.1 ns | 79.6 ns | 417 ns | 936 ns | 1.02 µs |
| D153 | 18.3 ns | 394 ns | 677 ns | 1.03 µs | 1.71 µs |
| D230 | 29 ns | 331 ns | 888 ns | 1.71 µs | 2.36 µs |
| D307 | 43.2 ns | 441 ns | 1.84 µs | 2.81 µs | 5.3 µs |
| D462 | 62.1 ns | 891 ns | 3.17 µs | 5.18 µs | 7.86 µs |
| D616 | 84.9 ns | 1.65 µs | 5.66 µs | 10.9 µs | 13.9 µs |
| D924 | 56.4 ns | 2.73 µs | 8.03 µs | 24.4 µs | 23.1 µs |
| D1232 | 111 ns | 4.85 µs | 18.7 µs | 20.2 µs | 45 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,203.7 88.2,200.6 124.4,198.9 160.5,198.2 196.7,198.4 232.9,196.9 269.1,186.9 305.3,178.2 341.5,170.4 377.6,163.5 413.8,172.4 450.0,157.7 450.0,27.4 413.8,41.8 377.6,52.9 341.5,65.2 305.3,73.8 269.1,91.4 232.9,98.4 196.7,109.6 160.5,120.2 124.4,133.6 88.2,161.2 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,203.7 88.2,200.6 124.4,198.9 160.5,198.2 196.7,198.4 232.9,196.9 269.1,186.9 305.3,178.2 341.5,170.4 377.6,163.5 413.8,172.4 450.0,157.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.1 88.2,178.9 124.4,179.3 160.5,160.9 196.7,164.9 232.9,130.2 269.1,134.0 305.3,127.8 341.5,112.5 377.6,99.2 413.8,88.2 450.0,75.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.0 88.2,168.9 124.4,163.0 160.5,139.7 196.7,129.0 232.9,118.5 269.1,112.6 305.3,96.8 341.5,85.0 377.6,72.4 413.8,64.8 450.0,46.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.6 88.2,166.2 124.4,139.4 160.5,130.1 196.7,111.4 232.9,109.4 269.1,98.4 305.3,87.6 341.5,74.3 377.6,58.1 413.8,40.6 450.0,44.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,161.2 124.4,133.6 160.5,120.2 196.7,109.6 232.9,98.4 269.1,91.4 305.3,73.8 341.5,65.2 377.6,52.9 413.8,41.8 450.0,27.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.14 ns | 277 ns | 392 ns | 374 ns | 409 ns |
| D38 | 2.81 ns | 372 ns | 378 ns | 414 ns | 361 ns |
| D57 | 276 ns | 465 ns | 404 ns | 297 ns | 457 ns |
| D76 | 283 ns | 508 ns | 301 ns | 678 ns | 624 ns |
| D115 | 238 ns | 372 ns | 687 ns | 995 ns | 1.1 µs |
| D153 | 260 ns | 512 ns | 763 ns | 1.11 µs | 1.45 µs |
| D230 | 560 ns | 443 ns | 965 ns | 1.23 µs | 1.59 µs |
| D307 | 739 ns | 412 ns | 1.19 µs | 1.34 µs | 11.2 µs |
| D462 | 1.16 µs | 2.22 µs | 3.35 µs | 3.37 µs | 5.36 µs |
| D616 | 1.53 µs | 1.44 µs | 1.97 µs | 3.07 µs | 3.93 µs |
| D924 | 1.22 µs | 1.89 µs | 2.16 µs | 3.98 µs | 4.64 µs |
| D1232 | 3.25 µs | 2.6 µs | 4.27 µs | 4.5 µs | 6.77 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.1 88.2,192.0 124.4,112.4 160.5,112.0 196.7,114.9 232.9,113.4 269.1,100.1 305.3,95.3 341.5,87.4 377.6,82.7 413.8,86.5 450.0,69.5 450.0,56.8 413.8,63.4 377.6,66.2 341.5,60.8 305.3,48.0 269.1,81.9 232.9,83.5 196.7,88.4 160.5,98.2 124.4,103.6 88.2,107.7 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.1 88.2,192.0 124.4,112.4 160.5,112.0 196.7,114.9 232.9,113.4 269.1,100.1 305.3,95.3 341.5,87.4 377.6,82.7 413.8,86.5 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.3 88.2,107.2 124.4,103.3 160.5,101.8 196.7,107.2 232.9,101.6 269.1,104.2 305.3,105.4 341.5,76.1 377.6,83.6 413.8,79.0 450.0,73.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.3 88.2,106.9 124.4,105.8 160.5,110.9 196.7,96.5 232.9,94.7 269.1,90.6 305.3,87.0 341.5,69.0 377.6,78.3 413.8,76.6 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,105.3 124.4,111.1 160.5,96.7 196.7,90.1 232.9,88.1 269.1,86.4 305.3,85.0 341.5,68.9 377.6,70.5 413.8,66.0 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,107.7 124.4,103.6 160.5,98.2 196.7,88.4 232.9,83.5 269.1,81.9 305.3,48.0 341.5,60.8 377.6,66.2 413.8,63.4 450.0,56.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.34 µs | 3.12 µs | 5.84 µs | 6.34 µs | 7.29 µs |
| D38 | 2.89 µs | 5.81 µs | 7.3 µs | 9.02 µs | 8.27 µs |
| D57 | 2.55 µs | 2.54 µs | 2.11 µs | 1.51 µs | 2.22 µs |
| D76 | 2.79 µs | 2.92 µs | 1.47 µs | 2.99 µs | 2.95 µs |
| D115 | 4.04 µs | 3.83 µs | 5.57 µs | 5.6 µs | 6.36 µs |
| D153 | 3.79 µs | 4.92 µs | 5.73 µs | 6.37 µs | 6.88 µs |
| D230 | 6.44 µs | 5.29 µs | 7.29 µs | 7.38 µs | 7.75 µs |
| D307 | 9.78 µs | 6.91 µs | 13.4 µs | 14.3 µs | 16.6 µs |
| D462 | 9.72 µs | 8.8 µs | 14.3 µs | 13.6 µs | 19.8 µs |
| D616 | 16.9 µs | 20.6 µs | 30.1 µs | 39.4 µs | 45.7 µs |
| D924 | 14.1 µs | 33.8 µs | 42.4 µs | 76.3 µs | 84.9 µs |
| D1232 | 33.8 µs | 52.3 µs | 89.2 µs | 98.8 µs | 141 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.3 88.2,179.3 124.4,182.9 160.5,180.3 196.7,169.6 232.9,171.4 269.1,156.1 305.3,144.0 341.5,144.2 377.6,128.1 413.8,133.4 450.0,108.1 450.0,66.7 413.8,81.4 377.6,99.3 341.5,123.6 305.3,128.7 269.1,150.7 232.9,154.2 196.7,156.4 160.5,178.7 124.4,187.0 88.2,148.8 52.0,152.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.3 88.2,179.3 124.4,182.9 160.5,180.3 196.7,169.6 232.9,171.4 269.1,156.1 305.3,144.0 341.5,144.2 377.6,128.1 413.8,133.4 450.0,108.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.1 88.2,159.1 124.4,183.0 160.5,179.0 196.7,171.1 232.9,163.9 269.1,161.8 305.3,154.0 341.5,147.0 377.6,122.4 413.8,108.1 450.0,95.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,152.4 124.4,188.3 160.5,198.7 196.7,160.3 232.9,159.4 269.1,152.5 305.3,134.9 341.5,133.0 377.6,111.5 413.8,101.5 450.0,80.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,146.3 124.4,198.1 160.5,178.3 196.7,160.1 232.9,156.4 269.1,152.1 305.3,133.1 341.5,134.4 377.6,103.6 413.8,84.5 450.0,77.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,148.8 124.4,187.0 160.5,178.7 196.7,156.4 232.9,154.2 269.1,150.7 305.3,128.7 341.5,123.6 377.6,99.3 413.8,81.4 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.8 ns | 2.04 µs | 3.87 µs | 4.07 µs | 4.66 µs |
| D38 | 13.1 ns | 3.83 µs | 4.41 µs | 5.36 µs | 4.5 µs |
| D57 | 74.2 ns | 4.06 µs | 3.67 µs | 2.62 µs | 4.11 µs |
| D76 | 78.8 ns | 4.46 µs | 2.65 µs | 5.31 µs | 5.56 µs |
| D115 | 123 ns | 6.45 µs | 9.61 µs | 10.3 µs | 11.7 µs |
| D153 | 149 ns | 8.44 µs | 10.3 µs | 11.7 µs | 13 µs |
| D230 | 371 ns | 9.39 µs | 13.4 µs | 15.6 µs | 17.5 µs |
| D307 | 467 ns | 12 µs | 22.9 µs | 28.3 µs | 34.5 µs |
| D462 | 696 ns | 54.5 µs | 138 µs | 178 µs | 284 µs |
| D616 | 848 ns | 151 µs | 326 µs | 381 µs | 569 µs |
| D924 | 629 ns | 353 µs | 346 µs | 846 µs | 1.62 ms |
| D1232 | 1.69 µs | 650 µs | 798 µs | 1.8 ms | 2.72 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,206.4 88.2,206.1 124.4,181.0 160.5,180.1 196.7,173.7 232.9,170.9 269.1,157.7 305.3,154.4 341.5,148.6 377.6,145.7 413.8,150.0 450.0,135.7 450.0,28.9 413.8,36.3 377.6,51.5 341.5,61.5 305.3,92.1 269.1,101.9 232.9,106.2 196.7,107.8 160.5,118.5 124.4,122.9 88.2,121.5 52.0,121.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,206.4 88.2,206.1 124.4,181.0 160.5,180.1 196.7,173.7 232.9,170.9 269.1,157.7 305.3,154.4 341.5,148.6 377.6,145.7 413.8,150.0 450.0,135.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,123.9 124.4,123.1 160.5,121.7 196.7,116.3 232.9,112.5 269.1,110.9 305.3,107.3 341.5,85.4 377.6,70.7 413.8,58.4 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.7 88.2,121.8 124.4,124.5 160.5,129.2 196.7,110.6 232.9,109.5 269.1,105.8 305.3,98.0 341.5,72.0 377.6,59.6 413.8,58.7 450.0,46.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.0 88.2,119.0 124.4,129.4 160.5,119.2 196.7,109.6 232.9,107.7 269.1,103.5 305.3,95.0 341.5,68.3 377.6,57.3 413.8,45.7 450.0,34.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.0 88.2,121.5 124.4,122.9 160.5,118.5 196.7,107.8 232.9,106.2 269.1,101.9 305.3,92.1 341.5,61.5 377.6,51.5 413.8,36.3 450.0,28.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.79 ns | 13.4 ns | 18.4 ns | 29.5 ns | 29.3 ns |
| D38 | 7.23 ns | 22.1 ns | 30.2 ns | 152 ns | 172 ns |
| D57 | 105 ns | 140 ns | 171 ns | 247 ns | 315 ns |
| D76 | 90.5 ns | 144 ns | 193 ns | 392 ns | 626 ns |
| D115 | 92.2 ns | 179 ns | 410 ns | 732 ns | 976 ns |
| D153 | 108 ns | 319 ns | 661 ns | 995 ns | 1.65 µs |
| D230 | 149 ns | 319 ns | 794 ns | 1.72 µs | 2.23 µs |
| D307 | 163 ns | 438 ns | 1.79 µs | 2.74 µs | 5.18 µs |
| D462 | 199 ns | 828 ns | 2.64 µs | 5.38 µs | 7.61 µs |
| D616 | 258 ns | 1.5 µs | 4.65 µs | 7.4 µs | 13.5 µs |
| D924 | 138 ns | 2.25 µs | 5.49 µs | 16.2 µs | 22.9 µs |
| D1232 | 434 ns | 4.06 µs | 13.1 µs | 20.6 µs | 44.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.8 88.2,175.6 124.4,129.1 160.5,131.7 196.7,131.4 232.9,128.7 269.1,123.0 305.3,121.5 341.5,118.1 377.6,113.5 413.8,124.4 450.0,104.5 450.0,24.0 413.8,35.6 377.6,44.8 341.5,54.7 305.3,61.4 269.1,76.0 232.9,81.3 196.7,90.4 160.5,98.1 124.4,110.0 88.2,120.5 52.0,151.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.8 88.2,175.6 124.4,129.1 160.5,131.7 196.7,131.4 232.9,128.7 269.1,123.0 305.3,121.5 341.5,118.1 377.6,113.5 413.8,124.4 450.0,104.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,156.2 124.4,124.2 160.5,123.7 196.7,119.9 232.9,109.8 269.1,109.8 305.3,104.4 341.5,93.3 377.6,83.0 413.8,75.9 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,150.8 124.4,120.7 160.5,118.6 196.7,105.5 232.9,97.2 269.1,94.0 305.3,79.9 341.5,73.1 377.6,63.3 413.8,60.4 450.0,45.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.2 88.2,122.8 124.4,114.3 160.5,106.3 196.7,95.4 232.9,90.1 269.1,80.6 305.3,72.5 341.5,60.8 377.6,55.2 413.8,41.6 450.0,37.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,120.5 124.4,110.0 160.5,98.1 196.7,90.4 232.9,81.3 269.1,76.0 305.3,61.4 341.5,54.7 377.6,44.8 413.8,35.6 450.0,24.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
