# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 55.7 ns | 132 ns | 160 ns | 251 ns | 263 ns |
| D38 | 72.9 ns | 158 ns | 254 ns | 372 ns | 410 ns |
| D57 | 187 ns | 423 ns | 694 ns | 1.12 µs | 1.07 µs |
| D76 | 175 ns | 490 ns | 815 ns | 1.3 µs | 1.1 µs |
| D115 | 200 ns | 756 ns | 876 ns | 2.1 µs | 3.14 µs |
| D153 | 246 ns | 755 ns | 1.72 µs | 2.88 µs | 4.47 µs |
| D230 | 276 ns | 1.4 µs | 3.25 µs | 4.67 µs | 7.41 µs |
| D307 | 326 ns | 1.83 µs | 3.03 µs | 9.62 µs | 14.9 µs |
| D462 | 382 ns | 3.36 µs | 9.42 µs | 18.4 µs | 33.4 µs |
| D616 | 466 ns | 4.78 µs | 14.4 µs | 34.1 µs | 50.3 µs |
| D924 | 356 ns | 6.76 µs | 34.2 µs | 62.6 µs | 130 µs |
| D1232 | 1.16 µs | 15.4 µs | 57.4 µs | 102 µs | 181 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.2 88.2,175.5 124.4,159.2 160.5,160.3 196.7,157.9 232.9,154.3 269.1,152.4 305.3,149.5 341.5,146.7 377.6,143.2 413.8,148.0 450.0,127.4 450.0,39.7 413.8,45.5 377.6,61.9 341.5,69.1 305.3,83.0 269.1,95.2 232.9,104.0 196.7,110.1 160.5,128.3 124.4,128.9 88.2,145.5 52.0,153.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.2 88.2,175.5 124.4,159.2 160.5,160.3 196.7,157.9 232.9,154.3 269.1,152.4 305.3,149.5 341.5,146.7 377.6,143.2 413.8,148.0 450.0,127.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.2 88.2,162.0 124.4,144.9 160.5,142.4 196.7,134.9 232.9,134.9 269.1,124.2 305.3,119.5 341.5,109.0 377.6,102.8 413.8,96.8 450.0,82.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.8 88.2,153.8 124.4,136.4 160.5,133.5 196.7,132.3 232.9,120.6 269.1,109.5 305.3,110.7 341.5,91.0 377.6,83.6 413.8,68.6 450.0,59.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.0 88.2,147.2 124.4,128.1 160.5,125.4 196.7,117.1 232.9,111.7 269.1,103.2 305.3,90.7 341.5,79.4 377.6,68.7 413.8,58.1 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.2 88.2,145.5 124.4,128.9 160.5,128.3 196.7,110.1 232.9,104.0 269.1,95.2 305.3,83.0 341.5,69.1 377.6,61.9 413.8,45.5 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.08 µs | 1.64 µs | 3.5 µs | 2.98 µs | 3.99 µs |
| D38 | 1.41 µs | 3.51 µs | 3.79 µs | 4.25 µs | 5.17 µs |
| D57 | 3.31 µs | 4.18 µs | 4.36 µs | 5.66 µs | 10.1 µs |
| D76 | 3.29 µs | 6.37 µs | 7.2 µs | 10.1 µs | 6.91 µs |
| D115 | 3.9 µs | 6.51 µs | 8.73 µs | 19 µs | 23.9 µs |
| D153 | 5.27 µs | 5.53 µs | 15.5 µs | 19.3 µs | 34.5 µs |
| D230 | 5.29 µs | 14 µs | 23.5 µs | 36.8 µs | 66.6 µs |
| D307 | 5.76 µs | 15.1 µs | 22.6 µs | 84.4 µs | 124 µs |
| D462 | 5.33 µs | 23.8 µs | 86.7 µs | 164 µs | 248 µs |
| D616 | 5.44 µs | 38.7 µs | 133 µs | 285 µs | 411 µs |
| D924 | 3.48 µs | 59.8 µs | 287 µs | 560 µs | 987 µs |
| D1232 | 5.78 µs | 131 µs | 445 µs | 773 µs | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.3 88.2,202.6 124.4,184.0 160.5,184.1 196.7,180.4 232.9,173.9 269.1,173.8 305.3,172.0 341.5,173.7 377.6,173.2 413.8,182.9 450.0,171.9 450.0,37.5 413.8,60.3 377.6,79.3 341.5,90.3 305.3,105.4 269.1,118.8 232.9,133.1 196.7,141.0 160.5,168.0 124.4,159.8 88.2,174.3 52.0,180.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.3 88.2,202.6 124.4,184.0 160.5,184.1 196.7,180.4 232.9,173.9 269.1,173.8 305.3,172.0 341.5,173.7 377.6,173.2 413.8,182.9 450.0,171.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.3 88.2,182.8 124.4,179.0 160.5,169.8 196.7,169.3 232.9,172.9 269.1,152.7 305.3,151.0 341.5,141.2 377.6,130.6 413.8,121.2 450.0,104.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,182.8 88.2,181.1 124.4,178.0 160.5,167.1 196.7,162.9 232.9,150.5 269.1,141.4 305.3,142.3 341.5,113.1 377.6,103.8 413.8,87.1 450.0,77.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,186.3 88.2,178.6 124.4,172.3 160.5,159.8 196.7,146.1 232.9,145.7 269.1,131.7 305.3,113.7 341.5,99.2 377.6,87.3 413.8,72.6 450.0,65.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.0 88.2,174.3 124.4,159.8 160.5,168.0 196.7,141.0 232.9,133.1 269.1,118.8 305.3,105.4 341.5,90.3 377.6,79.3 413.8,60.3 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 10.8 ns | 40.5 ns | 42.9 ns | 37.4 ns | 40.1 ns |
| D38 | 12.4 ns | 42.7 ns | 68.2 ns | 67.2 ns | 106 ns |
| D57 | 12.8 ns | 42.2 ns | 83.6 ns | 373 ns | 436 ns |
| D76 | 11.5 ns | 95.3 ns | 378 ns | 410 ns | 415 ns |
| D115 | 13.7 ns | 84.1 ns | 271 ns | 833 ns | 1.01 µs |
| D153 | 25.6 ns | 337 ns | 636 ns | 874 ns | 1.64 µs |
| D230 | 28.7 ns | 451 ns | 1.14 µs | 1.73 µs | 2.28 µs |
| D307 | 43.1 ns | 680 ns | 1.09 µs | 2.98 µs | 5.29 µs |
| D462 | 77 ns | 1.21 µs | 3.36 µs | 6.09 µs | 7.82 µs |
| D616 | 69.5 ns | 2.1 µs | 5.69 µs | 10.9 µs | 13 µs |
| D924 | 72.1 ns | 2.49 µs | 11 µs | 22.1 µs | 26.8 µs |
| D1232 | 99.8 ns | 5.59 µs | 20.4 µs | 20.1 µs | 46.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,208.3 88.2,205.4 124.4,204.7 160.5,207.0 196.7,203.2 232.9,189.6 269.1,187.1 305.3,178.3 341.5,165.7 377.6,167.9 413.8,167.1 450.0,160.0 450.0,26.5 413.8,38.6 377.6,54.4 341.5,65.3 305.3,73.8 269.1,92.1 232.9,99.3 196.7,109.8 160.5,129.1 124.4,128.0 88.2,158.6 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,208.3 88.2,205.4 124.4,204.7 160.5,207.0 196.7,203.2 232.9,189.6 269.1,187.1 305.3,178.3 341.5,165.7 377.6,167.9 413.8,167.1 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,178.5 124.4,178.7 160.5,161.1 196.7,163.8 232.9,133.6 269.1,127.3 305.3,118.4 341.5,105.8 377.6,93.9 413.8,90.2 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.3 124.4,163.9 160.5,131.1 196.7,138.3 232.9,119.8 269.1,107.2 305.3,108.1 341.5,83.7 377.6,72.3 413.8,57.9 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,181.4 88.2,168.6 124.4,131.4 160.5,129.4 196.7,114.0 232.9,112.9 269.1,98.1 305.3,86.3 341.5,70.8 377.6,58.1 413.8,42.7 450.0,44.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,158.6 124.4,128.0 160.5,129.1 196.7,109.8 232.9,99.3 269.1,92.1 305.3,73.8 341.5,65.3 377.6,54.4 413.8,38.6 450.0,26.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.86 ns | 273 ns | 401 ns | 379 ns | 410 ns |
| D38 | 7.84 ns | 402 ns | 383 ns | 385 ns | 417 ns |
| D57 | 223 ns | 510 ns | 497 ns | 483 ns | 706 ns |
| D76 | 210 ns | 513 ns | 508 ns | 706 ns | 358 ns |
| D115 | 229 ns | 507 ns | 356 ns | 1.05 µs | 1.1 µs |
| D153 | 320 ns | 426 ns | 661 ns | 854 ns | 1.29 µs |
| D230 | 516 ns | 717 ns | 1.13 µs | 1.23 µs | 1.66 µs |
| D307 | 756 ns | 697 ns | 739 ns | 1.47 µs | 11.3 µs |
| D462 | 1.22 µs | 3.16 µs | 3.75 µs | 4.3 µs | 5.44 µs |
| D616 | 1.43 µs | 1.68 µs | 2.05 µs | 3.12 µs | 3.69 µs |
| D924 | 1.43 µs | 1.69 µs | 3.15 µs | 3.69 µs | 5.17 µs |
| D1232 | 3.07 µs | 3.11 µs | 4.48 µs | 4.33 µs | 6.09 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.5 88.2,174.2 124.4,116.0 160.5,117.1 196.7,115.6 232.9,109.8 269.1,101.5 305.3,94.9 341.5,86.6 377.6,83.8 413.8,83.7 450.0,70.5 450.0,58.6 413.8,61.5 377.6,67.3 341.5,60.6 305.3,47.9 269.1,81.2 232.9,85.6 196.7,88.3 160.5,107.9 124.4,96.1 88.2,105.2 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.5 88.2,174.2 124.4,116.0 160.5,117.1 196.7,115.6 232.9,109.8 269.1,101.5 305.3,94.9 341.5,86.6 377.6,83.8 413.8,83.7 450.0,70.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.5 88.2,105.8 124.4,101.7 160.5,101.6 196.7,101.8 232.9,104.8 269.1,95.8 305.3,96.3 341.5,70.0 377.6,81.0 413.8,80.9 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,106.7 124.4,102.2 160.5,101.8 196.7,107.9 232.9,97.2 269.1,87.9 305.3,95.3 341.5,67.0 377.6,77.6 413.8,70.1 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,106.6 124.4,102.6 160.5,96.0 196.7,89.1 232.9,92.7 269.1,86.3 305.3,83.3 341.5,64.6 377.6,70.3 413.8,67.3 450.0,64.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,105.2 124.4,96.1 160.5,107.9 196.7,88.3 232.9,85.6 269.1,81.2 305.3,47.9 341.5,60.6 377.6,67.3 413.8,61.5 450.0,58.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.92 µs | 3.12 µs | 6.08 µs | 5.7 µs | 7.32 µs |
| D38 | 2.42 µs | 6.08 µs | 7.29 µs | 8.67 µs | 10.5 µs |
| D57 | 2.1 µs | 2.77 µs | 2.65 µs | 2.79 µs | 3.21 µs |
| D76 | 1.48 µs | 2.99 µs | 2.74 µs | 3.19 µs | 1.66 µs |
| D115 | 3.35 µs | 4.99 µs | 3.37 µs | 6.14 µs | 6.19 µs |
| D153 | 4.5 µs | 4.41 µs | 5.36 µs | 5.29 µs | 6.29 µs |
| D230 | 6.06 µs | 7.63 µs | 8.74 µs | 7.4 µs | 8.71 µs |
| D307 | 9.95 µs | 10.9 µs | 8.09 µs | 15.3 µs | 17 µs |
| D462 | 9.6 µs | 13.2 µs | 15.3 µs | 17.9 µs | 20 µs |
| D616 | 16.1 µs | 25.7 µs | 30 µs | 38.8 µs | 41.6 µs |
| D924 | 15.6 µs | 30.5 µs | 60.2 µs | 71.6 µs | 92.9 µs |
| D1232 | 32.3 µs | 62.1 µs | 96.4 µs | 98.9 µs | 130 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,191.1 88.2,184.4 124.4,188.5 160.5,198.7 196.7,175.0 232.9,166.4 269.1,157.9 305.3,143.5 341.5,144.5 377.6,129.6 413.8,130.5 450.0,109.3 450.0,69.2 413.8,78.8 377.6,102.1 341.5,123.2 305.3,127.9 269.1,147.3 232.9,156.8 196.7,157.2 160.5,195.4 124.4,176.3 88.2,142.0 52.0,152.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,191.1 88.2,184.4 124.4,188.5 160.5,198.7 196.7,175.0 232.9,166.4 269.1,157.9 305.3,143.5 341.5,144.5 377.6,129.6 413.8,130.5 450.0,109.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.1 88.2,157.7 124.4,180.5 160.5,178.3 196.7,163.4 232.9,167.0 269.1,151.2 305.3,140.7 341.5,135.4 377.6,116.0 413.8,111.0 450.0,90.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.7 88.2,152.5 124.4,181.8 160.5,180.8 196.7,174.8 232.9,161.4 269.1,147.2 305.3,149.5 341.5,131.0 377.6,111.5 413.8,91.3 450.0,77.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.6 88.2,147.5 124.4,180.3 160.5,176.5 196.7,157.4 232.9,161.7 269.1,152.1 305.3,131.1 341.5,126.5 377.6,104.0 413.8,86.3 450.0,77.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.4 88.2,142.0 124.4,176.3 160.5,195.4 196.7,157.2 232.9,156.8 269.1,147.3 305.3,127.9 341.5,123.2 377.6,102.1 413.8,78.8 450.0,69.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 255 ns | 387 ns | 1.36 µs | 905 ns | 1.57 µs |
| D38 | 336 ns | 1.36 µs | 1.49 µs | 1.67 µs | 2.05 µs |
| D57 | 1.08 µs | 1.47 µs | 1.3 µs | 1.4 µs | 1.62 µs |
| D76 | 774 ns | 1.49 µs | 1.37 µs | 1.6 µs | 833 ns |
| D115 | 1.79 µs | 2.39 µs | 1.7 µs | 3.05 µs | 3.05 µs |
| D153 | 2.25 µs | 2.27 µs | 2.71 µs | 2.64 µs | 3.09 µs |
| D230 | 2.99 µs | 3.54 µs | 4.17 µs | 3.59 µs | 4.19 µs |
| D307 | 4.94 µs | 5.35 µs | 3.79 µs | 6.89 µs | 7.66 µs |
| D462 | 4.83 µs | 6.29 µs | 6.99 µs | 8.1 µs | 9.24 µs |
| D616 | 7.91 µs | 10.7 µs | 12.2 µs | 15.2 µs | 16.6 µs |
| D924 | 7.52 µs | 12.4 µs | 21.9 µs | 26.2 µs | 33.3 µs |
| D1232 | 15.8 µs | 24.1 µs | 33.9 µs | 34.4 µs | 45.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.9 88.2,174.9 124.4,141.0 160.5,150.7 196.7,126.5 232.9,119.8 269.1,111.6 305.3,97.1 341.5,97.7 377.6,83.5 413.8,84.9 450.0,63.4 450.0,32.8 413.8,41.8 377.6,62.0 341.5,78.9 305.3,84.4 269.1,101.8 232.9,110.6 196.7,111.0 160.5,148.6 124.4,129.3 88.2,122.5 52.0,130.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.9 88.2,174.9 124.4,141.0 160.5,150.7 196.7,126.5 232.9,119.8 269.1,111.6 305.3,97.1 341.5,97.7 377.6,83.5 413.8,84.9 450.0,63.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,170.8 88.2,134.4 124.4,132.2 160.5,131.8 196.7,118.1 232.9,119.7 269.1,106.7 305.3,94.8 341.5,90.1 377.6,74.7 413.8,70.4 450.0,51.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,131.8 124.4,135.8 160.5,134.2 196.7,128.0 232.9,114.4 269.1,102.0 305.3,104.8 341.5,87.0 377.6,70.9 413.8,53.9 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,146.2 88.2,128.6 124.4,133.5 160.5,129.6 196.7,111.0 232.9,115.3 269.1,106.3 305.3,87.4 341.5,82.8 377.6,64.6 413.8,48.7 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.4 88.2,122.5 124.4,129.3 160.5,148.6 196.7,111.0 232.9,110.6 269.1,101.8 305.3,84.4 341.5,78.9 377.6,62.0 413.8,41.8 450.0,32.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 229 ns | 351 ns | 1.34 µs | 860 ns | 1.54 µs |
| D38 | 295 ns | 1.35 µs | 1.45 µs | 1.64 µs | 2.03 µs |
| D57 | 827 ns | 1.14 µs | 1.28 µs | 1.35 µs | 1.43 µs |
| D76 | 610 ns | 1.21 µs | 1.28 µs | 1.43 µs | 746 ns |
| D115 | 1.34 µs | 2.19 µs | 1.44 µs | 2.65 µs | 2.63 µs |
| D153 | 1.79 µs | 1.83 µs | 2.32 µs | 2.18 µs | 2.66 µs |
| D230 | 2.33 µs | 3.01 µs | 3.48 µs | 2.92 µs | 3.45 µs |
| D307 | 3.96 µs | 4.53 µs | 3.19 µs | 6 µs | 6.79 µs |
| D462 | 3.87 µs | 5.21 µs | 6.14 µs | 6.97 µs | 8.16 µs |
| D616 | 6.17 µs | 8.8 µs | 10.7 µs | 13.3 µs | 14.6 µs |
| D924 | 5.4 µs | 10.1 µs | 19.1 µs | 22.9 µs | 30.3 µs |
| D1232 | 12.3 µs | 20.8 µs | 30.4 µs | 31.3 µs | 42.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.0 88.2,178.6 124.4,148.8 160.5,157.7 196.7,134.8 232.9,126.5 269.1,118.9 305.3,103.5 341.5,104.1 377.6,90.6 413.8,94.5 450.0,70.8 450.0,34.8 413.8,44.6 377.6,65.6 341.5,82.6 305.3,87.9 269.1,107.5 232.9,115.0 196.7,115.4 160.5,151.8 124.4,133.0 88.2,122.9 52.0,130.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.0 88.2,178.6 124.4,148.8 160.5,157.7 196.7,134.8 232.9,126.5 269.1,118.9 305.3,103.5 341.5,104.1 377.6,90.6 413.8,94.5 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,173.7 88.2,134.7 124.4,139.6 160.5,137.9 196.7,120.7 232.9,125.8 269.1,111.5 305.3,99.6 341.5,95.6 377.6,80.4 413.8,76.3 450.0,55.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.8 88.2,132.5 124.4,136.2 160.5,136.1 196.7,132.9 232.9,119.0 269.1,107.3 305.3,109.8 341.5,90.8 377.6,74.7 413.8,58.0 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,147.7 88.2,129.0 124.4,134.7 160.5,133.0 196.7,115.1 232.9,120.8 269.1,112.3 305.3,91.4 341.5,87.1 377.6,68.3 413.8,52.6 450.0,43.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,130.9 88.2,122.9 124.4,133.0 160.5,151.8 196.7,115.4 232.9,115.0 269.1,107.5 305.3,87.9 341.5,82.6 377.6,65.6 413.8,44.6 450.0,34.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 10 ns | 2.07 µs | 4.2 µs | 3.58 µs | 4.72 µs |
| D38 | 10.8 ns | 4.18 µs | 4.41 µs | 4.89 µs | 5.88 µs |
| D57 | 62.7 ns | 4.55 µs | 4.5 µs | 4.59 µs | 5.55 µs |
| D76 | 61.4 ns | 4.51 µs | 4.75 µs | 5.62 µs | 3.23 µs |
| D115 | 112 ns | 8.17 µs | 5.56 µs | 11.1 µs | 11.6 µs |
| D153 | 176 ns | 6.98 µs | 9.5 µs | 9.65 µs | 11.8 µs |
| D230 | 295 ns | 13.5 µs | 16.3 µs | 15.2 µs | 18.5 µs |
| D307 | 469 ns | 19 µs | 13.9 µs | 30.5 µs | 34.5 µs |
| D462 | 642 ns | 77.2 µs | 149 µs | 223 µs | 283 µs |
| D616 | 786 ns | 192 µs | 329 µs | 379 µs | 531 µs |
| D924 | 797 ns | 330 µs | 486 µs | 793 µs | 1.74 ms |
| D1232 | 1.65 µs | 776 µs | 856 µs | 1.79 ms | 2.64 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,210.0 88.2,208.9 124.4,183.4 160.5,183.7 196.7,175.0 232.9,168.5 269.1,161.0 305.3,154.3 341.5,149.7 377.6,146.8 413.8,146.6 450.0,136.1 450.0,29.3 413.8,35.3 377.6,52.5 341.5,61.6 305.3,92.1 269.1,101.1 232.9,107.6 196.7,107.8 160.5,126.4 124.4,118.5 88.2,117.7 52.0,120.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,210.0 88.2,208.9 124.4,183.4 160.5,183.7 196.7,175.0 232.9,168.5 269.1,161.0 305.3,154.3 341.5,149.7 377.6,146.8 413.8,146.6 450.0,136.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,132.8 88.2,122.6 124.4,121.4 160.5,121.5 196.7,112.9 232.9,115.2 269.1,105.6 305.3,100.7 341.5,80.4 377.6,67.3 413.8,59.4 450.0,47.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,122.6 88.2,121.9 124.4,121.6 160.5,120.8 196.7,118.5 232.9,110.7 269.1,103.0 305.3,105.3 341.5,70.9 377.6,59.4 413.8,53.8 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.9 88.2,120.4 124.4,121.3 160.5,118.3 196.7,108.5 232.9,110.5 269.1,103.9 305.3,93.8 341.5,65.1 377.6,57.4 413.8,46.7 450.0,34.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,120.9 88.2,117.7 124.4,118.5 160.5,126.4 196.7,107.8 232.9,107.6 269.1,101.1 305.3,92.1 341.5,61.6 377.6,52.5 413.8,35.3 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.95 ns | 13.3 ns | 18.5 ns | 31.6 ns | 29.3 ns |
| D38 | 5.37 ns | 20.4 ns | 39.8 ns | 134 ns | 208 ns |
| D57 | 86 ns | 148 ns | 206 ns | 357 ns | 393 ns |
| D76 | 72.7 ns | 144 ns | 292 ns | 397 ns | 428 ns |
| D115 | 85.5 ns | 231 ns | 258 ns | 788 ns | 979 ns |
| D153 | 113 ns | 269 ns | 625 ns | 856 ns | 1.54 µs |
| D230 | 135 ns | 455 ns | 1.03 µs | 1.67 µs | 2.22 µs |
| D307 | 169 ns | 682 ns | 1.1 µs | 2.89 µs | 5.17 µs |
| D462 | 206 ns | 1.1 µs | 2.8 µs | 6.28 µs | 7.6 µs |
| D616 | 243 ns | 1.94 µs | 4.66 µs | 7.38 µs | 12.5 µs |
| D924 | 174 ns | 1.96 µs | 7.52 µs | 14.9 µs | 26.4 µs |
| D1232 | 378 ns | 4.8 µs | 13.9 µs | 20.6 µs | 45.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.1 88.2,180.8 124.4,132.6 160.5,135.5 196.7,132.7 232.9,127.8 269.1,124.8 305.3,120.9 341.5,117.4 377.6,114.6 413.8,120.4 450.0,106.9 450.0,23.6 413.8,33.1 377.6,46.1 341.5,54.8 305.3,61.5 269.1,76.1 232.9,82.5 196.7,90.4 160.5,104.7 124.4,106.2 88.2,117.3 52.0,151.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.1 88.2,180.8 124.4,132.6 160.5,135.5 196.7,132.7 232.9,127.8 269.1,124.8 305.3,120.9 341.5,117.4 377.6,114.6 413.8,120.4 450.0,106.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,157.6 124.4,123.2 160.5,123.6 196.7,115.4 232.9,112.8 269.1,103.7 305.3,96.6 341.5,88.4 377.6,78.5 413.8,78.3 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.3 88.2,146.0 124.4,117.5 160.5,111.4 196.7,113.5 232.9,98.2 269.1,89.5 305.3,88.4 341.5,72.1 377.6,63.2 413.8,55.0 450.0,44.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.0 88.2,124.9 124.4,107.9 160.5,106.0 196.7,94.1 232.9,92.7 269.1,81.1 305.3,71.5 341.5,58.1 377.6,55.3 413.8,43.1 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.3 88.2,117.3 124.4,106.2 160.5,104.7 196.7,90.4 232.9,82.5 269.1,76.1 305.3,61.5 341.5,54.8 377.6,46.1 413.8,33.1 450.0,23.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
