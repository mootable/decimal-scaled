# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 77.2 ns | 129 ns | 156 ns | 252 ns | 264 ns |
| D38 | 80.3 ns | 156 ns | 263 ns | 376 ns | 399 ns |
| D57 | 190 ns | 422 ns | 719 ns | 1.13 µs | 1.11 µs |
| D76 | 199 ns | 490 ns | 807 ns | 1.19 µs | 1.62 µs |
| D115 | 223 ns | 507 ns | 1.33 µs | 2.14 µs | 3.15 µs |
| D153 | 247 ns | 742 ns | 1.77 µs | 3.09 µs | 4.36 µs |
| D230 | 287 ns | 1.4 µs | 3.05 µs | 3.6 µs | 8.83 µs |
| D307 | 313 ns | 1.81 µs | 3.76 µs | 8.8 µs | 13.6 µs |
| D462 | 400 ns | 3.36 µs | 8.93 µs | 17.1 µs | 33.4 µs |
| D616 | 349 ns | 4.8 µs | 14.5 µs | 30.8 µs | 56.3 µs |
| D924 | 600 ns | 9.09 µs | 30.8 µs | 54.6 µs | 81.3 µs |
| D1232 | 1.17 µs | 15.3 µs | 57.1 µs | 102 µs | 124 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.5 88.2,173.8 124.4,158.9 160.5,158.0 196.7,156.1 232.9,154.3 269.1,151.7 305.3,150.2 341.5,145.9 377.6,148.3 413.8,138.9 450.0,127.3 450.0,46.3 413.8,53.6 377.6,60.0 341.5,69.0 305.3,84.7 269.1,92.2 232.9,104.4 196.7,110.1 160.5,121.6 124.4,128.3 88.2,145.9 52.0,153.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.5 88.2,173.8 124.4,158.9 160.5,158.0 196.7,156.1 232.9,154.3 269.1,151.7 305.3,150.2 341.5,145.9 377.6,148.3 413.8,138.9 450.0,127.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.5 88.2,162.2 124.4,145.0 160.5,142.4 196.7,141.8 232.9,135.2 269.1,124.1 305.3,119.7 341.5,108.9 377.6,102.8 413.8,91.7 450.0,82.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.2 88.2,153.2 124.4,135.7 160.5,133.7 196.7,125.0 232.9,120.1 269.1,110.6 305.3,107.0 341.5,92.0 377.6,83.5 413.8,70.4 450.0,59.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.0 88.2,147.0 124.4,127.9 160.5,126.9 196.7,116.8 232.9,110.4 269.1,107.8 305.3,92.2 341.5,80.7 377.6,70.4 413.8,60.5 450.0,49.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.1 88.2,145.9 124.4,128.3 160.5,121.6 196.7,110.1 232.9,104.4 269.1,92.2 305.3,84.7 341.5,69.0 377.6,60.0 413.8,53.6 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 1.62 µs | 3.23 µs | 3.46 µs | 3.98 µs |
| D38 | 1.56 ns | 3.23 µs | 4.16 µs | 4.31 µs | 4.76 µs |
| D57 | 2.18 ns | 4.2 µs | 4.47 µs | 5.78 µs | 10.3 µs |
| D76 | 3.12 ns | 5.02 µs | 7.13 µs | 7.86 µs | 11.3 µs |
| D115 | 11.8 ns | 3.94 µs | 13.2 µs | 18.9 µs | 23.7 µs |
| D153 | 16.2 ns | 5.95 µs | 17.2 µs | 22.1 µs | 34.6 µs |
| D230 | 45.3 ns | 13.2 µs | 22.1 µs | 28.3 µs | 78.1 µs |
| D307 | 68.2 ns | 15.2 µs | 30.2 µs | 80.2 µs | 117 µs |
| D462 | 136 ns | 23.7 µs | 79.9 µs | 155 µs | 248 µs |
| D616 | 97.1 ns | 38.4 µs | 133 µs | 267 µs | 443 µs |
| D924 | 206 ns | 81.9 µs | 269 µs | 472 µs | 633 µs |
| D1232 | 383 ns | 132 µs | 445 µs | 829 µs | 1.6 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,204.5 124.4,200.3 160.5,195.9 196.7,179.3 232.9,175.4 269.1,162.7 305.3,157.6 341.5,149.0 377.6,153.2 413.8,143.9 450.0,136.2 450.0,32.7 413.8,44.2 377.6,48.7 341.5,55.9 305.3,65.2 269.1,70.2 232.9,80.3 196.7,85.0 160.5,94.2 124.4,95.3 88.2,104.9 52.0,107.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,204.5 124.4,200.3 160.5,195.9 196.7,179.3 232.9,175.4 269.1,162.7 305.3,157.6 341.5,149.0 377.6,153.2 413.8,143.9 450.0,136.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,109.8 124.4,106.5 160.5,104.3 196.7,107.3 232.9,102.2 269.1,92.3 305.3,90.5 341.5,85.0 377.6,79.0 413.8,69.6 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,106.6 124.4,105.7 160.5,99.9 196.7,92.3 232.9,89.0 269.1,85.9 305.3,82.0 341.5,69.9 377.6,63.6 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,106.2 124.4,102.5 160.5,98.7 196.7,87.8 232.9,85.9 269.1,82.8 305.3,69.9 341.5,61.7 377.6,54.9 413.8,47.9 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.1 88.2,104.9 124.4,95.3 160.5,94.2 196.7,85.0 232.9,80.3 269.1,70.2 305.3,65.2 341.5,55.9 377.6,48.7 413.8,44.2 450.0,32.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.1 ns | 40.5 ns | 41.3 ns | 40.8 ns | 40.1 ns |
| D38 | 16.4 ns | 41.4 ns | 74.4 ns | 68.1 ns | 95.6 ns |
| D57 | 16.8 ns | 42.2 ns | 85.8 ns | 374 ns | 413 ns |
| D76 | 18.3 ns | 90 ns | 369 ns | 342 ns | 606 ns |
| D115 | 22.4 ns | 65.5 ns | 390 ns | 834 ns | 1.01 µs |
| D153 | 25 ns | 343 ns | 676 ns | 988 ns | 1.63 µs |
| D230 | 28.9 ns | 421 ns | 1.1 µs | 1.26 µs | 2.82 µs |
| D307 | 43.8 ns | 687 ns | 1.37 µs | 2.8 µs | 4.9 µs |
| D462 | 63.7 ns | 1.23 µs | 3.13 µs | 5.71 µs | 7.82 µs |
| D616 | 46.2 ns | 2.1 µs | 5.94 µs | 10.3 µs | 13.8 µs |
| D924 | 103 ns | 3.26 µs | 10.3 µs | 19 µs | 20.4 µs |
| D1232 | 99.3 ns | 5.6 µs | 20.4 µs | 22.4 µs | 24.6 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.4 88.2,199.2 124.4,198.8 160.5,196.9 196.7,192.5 232.9,190.1 269.1,186.9 305.3,177.9 341.5,169.8 377.6,176.8 413.8,159.3 450.0,160.2 450.0,40.5 413.8,44.5 377.6,53.0 341.5,65.3 305.3,75.5 269.1,87.5 232.9,99.4 196.7,109.7 160.5,120.9 124.4,129.2 88.2,161.0 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.4 88.2,199.2 124.4,198.8 160.5,196.9 196.7,192.5 232.9,190.1 269.1,186.9 305.3,177.9 341.5,169.8 377.6,176.8 413.8,159.3 450.0,160.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,178.7 160.5,162.3 196.7,169.2 232.9,133.3 269.1,128.8 305.3,118.1 341.5,105.5 377.6,93.9 413.8,84.4 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,166.4 124.4,163.3 160.5,131.7 196.7,130.5 232.9,118.5 269.1,108.0 305.3,103.2 341.5,85.2 377.6,71.3 413.8,59.3 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,168.4 124.4,131.4 160.5,133.3 196.7,113.9 232.9,110.3 269.1,105.1 305.3,87.6 341.5,72.2 377.6,59.4 413.8,46.0 450.0,42.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,161.0 124.4,129.2 160.5,120.9 196.7,109.7 232.9,99.4 269.1,87.5 305.3,75.5 341.5,65.3 377.6,53.0 413.8,44.5 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 271 ns | 396 ns | 379 ns | 410 ns |
| D38 | 2.49 ns | 386 ns | 409 ns | 381 ns | 380 ns |
| D57 | 271 ns | 498 ns | 473 ns | 512 ns | 712 ns |
| D76 | 273 ns | 406 ns | 476 ns | 455 ns | 649 ns |
| D115 | 285 ns | 310 ns | 654 ns | 1.05 µs | 1.12 µs |
| D153 | 311 ns | 446 ns | 765 ns | 1.02 µs | 1.29 µs |
| D230 | 557 ns | 650 ns | 1.02 µs | 918 ns | 1.78 µs |
| D307 | 644 ns | 680 ns | 1.05 µs | 1.31 µs | 10.3 µs |
| D462 | 1.31 µs | 3.23 µs | 3.47 µs | 4 µs | 5.45 µs |
| D616 | 1.03 µs | 1.55 µs | 1.94 µs | 2.84 µs | 3.99 µs |
| D924 | 2.13 µs | 2.08 µs | 2.93 µs | 3.38 µs | 3.11 µs |
| D1232 | 3.04 µs | 3.11 µs | 4.38 µs | 4.19 µs | 3.78 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,194.1 124.4,112.7 160.5,112.6 196.7,111.8 232.9,110.3 269.1,100.2 305.3,97.6 341.5,85.4 377.6,89.4 413.8,76.9 450.0,70.7 450.0,66.9 413.8,70.3 377.6,66.0 341.5,60.6 305.3,49.4 269.1,80.0 232.9,85.5 196.7,88.1 160.5,97.5 124.4,95.9 88.2,106.8 52.0,105.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,194.1 124.4,112.7 160.5,112.6 196.7,111.8 232.9,110.3 269.1,100.2 305.3,97.6 341.5,85.4 377.6,89.4 413.8,76.9 450.0,70.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.7 88.2,106.5 124.4,102.1 160.5,105.6 196.7,110.3 232.9,104.0 269.1,97.5 305.3,96.7 341.5,69.6 377.6,82.3 413.8,77.3 450.0,70.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,105.5 124.4,103.0 160.5,102.9 196.7,97.4 232.9,94.7 269.1,89.7 305.3,89.2 341.5,68.4 377.6,78.5 413.8,71.3 450.0,64.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,106.8 124.4,101.6 160.5,103.7 196.7,89.1 232.9,89.6 269.1,91.5 305.3,85.3 341.5,65.9 377.6,71.8 413.8,68.8 450.0,65.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.5 88.2,106.8 124.4,95.9 160.5,97.5 196.7,88.1 232.9,85.5 269.1,80.0 305.3,49.4 341.5,60.6 377.6,66.0 413.8,70.3 450.0,66.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.74 µs | 3.12 µs | 5.85 µs | 6.34 µs | 7.3 µs |
| D38 | 2.75 µs | 5.85 µs | 7.64 µs | 8.59 µs | 9.96 µs |
| D57 | 2.5 µs | 2.75 µs | 2.69 µs | 2.75 µs | 3.18 µs |
| D76 | 2.49 µs | 2.1 µs | 2.71 µs | 2.25 µs | 2.96 µs |
| D115 | 4.45 µs | 3.1 µs | 5.13 µs | 5.91 µs | 6.09 µs |
| D153 | 4.57 µs | 4.56 µs | 5.72 µs | 5.75 µs | 6.18 µs |
| D230 | 6.48 µs | 7.16 µs | 8.17 µs | 5.41 µs | 9.84 µs |
| D307 | 9.43 µs | 11 µs | 10.6 µs | 14.2 µs | 15.6 µs |
| D462 | 10.2 µs | 13.2 µs | 14.3 µs | 16.6 µs | 20 µs |
| D616 | 10.2 µs | 25.3 µs | 29.9 µs | 35.5 µs | 45.9 µs |
| D924 | 25.2 µs | 40 µs | 55.5 µs | 61 µs | 55.3 µs |
| D1232 | 32.6 µs | 61.4 µs | 98 µs | 103 µs | 90.8 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.8 88.2,180.8 124.4,183.5 160.5,183.6 196.7,166.8 232.9,166.0 269.1,155.9 305.3,145.0 341.5,142.7 377.6,142.7 413.8,116.5 450.0,109.1 450.0,79.5 413.8,93.8 377.6,99.2 341.5,123.3 305.3,130.5 269.1,143.8 232.9,157.3 196.7,157.7 160.5,178.6 124.4,176.5 88.2,143.5 52.0,152.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.8 88.2,180.8 124.4,183.5 160.5,183.6 196.7,166.8 232.9,166.0 269.1,155.9 305.3,145.0 341.5,142.7 377.6,142.7 413.8,116.5 450.0,109.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.0 88.2,158.8 124.4,180.7 160.5,188.5 196.7,177.2 232.9,166.1 269.1,153.0 305.3,140.6 341.5,135.4 377.6,116.5 413.8,103.2 450.0,90.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.8 88.2,151.1 124.4,181.4 160.5,181.1 196.7,162.6 232.9,159.5 269.1,149.2 305.3,141.5 341.5,133.0 377.6,111.6 413.8,93.7 450.0,77.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,147.7 124.4,180.7 160.5,186.6 196.7,158.5 232.9,159.3 269.1,161.1 305.3,133.2 341.5,128.7 377.6,106.7 413.8,91.0 450.0,75.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,143.5 124.4,176.5 160.5,178.6 196.7,157.7 232.9,157.3 269.1,143.8 305.3,130.5 341.5,123.3 377.6,99.2 413.8,93.8 450.0,79.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 2.04 µs | 3.9 µs | 4.07 µs | 4.67 µs |
| D38 | 6.54 ns | 3.89 µs | 4.83 µs | 4.94 µs | 5.42 µs |
| D57 | 56.7 ns | 4.6 µs | 4.56 µs | 4.67 µs | 5.65 µs |
| D76 | 75.3 ns | 3.47 µs | 4.72 µs | 4.27 µs | 5.61 µs |
| D115 | 136 ns | 5.33 µs | 8.92 µs | 11.2 µs | 11.7 µs |
| D153 | 173 ns | 7.64 µs | 10.4 µs | 11 µs | 12 µs |
| D230 | 342 ns | 12.8 µs | 15.3 µs | 11.6 µs | 21.3 µs |
| D307 | 360 ns | 19.2 µs | 18 µs | 28.7 µs | 32.3 µs |
| D462 | 747 ns | 77.6 µs | 137 µs | 206 µs | 282 µs |
| D616 | 550 ns | 193 µs | 330 µs | 357 µs | 573 µs |
| D924 | 1.01 µs | 420 µs | 461 µs | 666 µs | 1.12 ms |
| D1232 | 1.37 µs | 760 µs | 857 µs | 1.95 ms | 1.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,156.4 196.7,149.0 232.9,146.1 269.1,137.6 305.3,137.0 341.5,127.9 377.6,131.7 413.8,124.2 450.0,120.3 450.0,33.5 413.8,37.2 377.6,45.5 341.5,54.3 305.3,81.2 269.1,86.3 232.9,93.4 196.7,93.8 160.5,102.9 124.4,102.8 88.2,103.3 52.0,105.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,156.4 196.7,149.0 232.9,146.1 269.1,137.6 305.3,137.0 341.5,127.9 377.6,131.7 413.8,124.2 450.0,120.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.4 88.2,107.4 124.4,105.4 160.5,108.9 196.7,103.5 232.9,99.1 269.1,92.7 305.3,87.6 341.5,70.3 377.6,59.0 413.8,49.3 450.0,42.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.4 88.2,104.8 124.4,105.5 160.5,105.0 196.7,97.1 232.9,95.2 269.1,90.4 305.3,88.4 341.5,63.2 377.6,52.3 413.8,48.2 450.0,40.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,104.5 124.4,105.2 160.5,106.3 196.7,94.3 232.9,94.6 269.1,93.9 305.3,82.6 341.5,58.2 377.6,51.3 413.8,43.6 450.0,30.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.2 88.2,103.3 124.4,102.8 160.5,102.9 196.7,93.8 232.9,93.4 269.1,86.3 305.3,81.2 341.5,54.3 377.6,45.5 413.8,37.2 450.0,33.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.75 ns | 13.3 ns | 18.2 ns | 29.1 ns | 29.7 ns |
| D38 | 7.1 ns | 19.5 ns | 44.4 ns | 135 ns | 192 ns |
| D57 | 79.7 ns | 147 ns | 205 ns | 364 ns | 391 ns |
| D76 | 82 ns | 128 ns | 288 ns | 323 ns | 608 ns |
| D115 | 96.7 ns | 146 ns | 386 ns | 791 ns | 978 ns |
| D153 | 112 ns | 269 ns | 663 ns | 945 ns | 1.57 µs |
| D230 | 148 ns | 434 ns | 968 ns | 1.26 µs | 2.73 µs |
| D307 | 154 ns | 659 ns | 1.33 µs | 2.74 µs | 4.85 µs |
| D462 | 221 ns | 1.1 µs | 2.67 µs | 5.82 µs | 7.59 µs |
| D616 | 184 ns | 1.93 µs | 4.68 µs | 6.78 µs | 13.5 µs |
| D924 | 265 ns | 2.69 µs | 6.84 µs | 12.7 µs | 17.9 µs |
| D1232 | 395 ns | 4.81 µs | 13.8 µs | 23.5 µs | 23.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,176.0 124.4,133.9 160.5,133.5 196.7,130.6 232.9,128.1 269.1,123.2 305.3,122.5 341.5,116.2 377.6,119.4 413.8,113.1 450.0,106.2 450.0,35.2 413.8,39.9 377.6,44.8 341.5,54.8 305.3,62.6 269.1,72.5 232.9,82.2 196.7,90.4 160.5,98.7 124.4,106.3 88.2,118.7 52.0,151.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,176.0 124.4,133.9 160.5,133.5 196.7,130.6 232.9,128.1 269.1,123.2 305.3,122.5 341.5,116.2 377.6,119.4 413.8,113.1 450.0,106.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.0 88.2,158.4 124.4,123.3 160.5,125.6 196.7,123.5 232.9,112.8 269.1,104.5 305.3,97.2 341.5,88.3 377.6,78.5 413.8,72.8 450.0,62.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.6 88.2,144.1 124.4,117.5 160.5,111.6 196.7,106.5 232.9,97.1 269.1,90.6 305.3,85.1 341.5,72.9 377.6,63.2 413.8,56.6 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,124.8 124.4,107.6 160.5,109.6 196.7,94.1 232.9,91.0 269.1,86.0 305.3,72.5 341.5,59.4 377.6,56.8 413.8,45.9 450.0,35.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.1 88.2,118.7 124.4,106.3 160.5,98.7 196.7,90.4 232.9,82.2 269.1,72.5 305.3,62.6 341.5,54.8 377.6,44.8 413.8,39.9 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
