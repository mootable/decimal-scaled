# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 131 ns | 124 ns | 149 ns | 392 ns | 391 ns |
| D38 | 237 ns | 263 ns | 534 ns | 529 ns | 841 ns |
| D57 | 320 ns | 397 ns | 704 ns | 1.13 µs | 1.14 µs |
| D76 | 288 ns | 749 ns | 1.17 µs | 1.11 µs | 1.24 µs |
| D115 | 195 ns | 744 ns | 2.37 µs | 3.7 µs | 5.18 µs |
| D153 | 208 ns | 836 ns | 3.5 µs | 3.15 µs | 7.13 µs |
| D230 | 265 ns | 1.15 µs | 7.76 µs | 9.86 µs | 14.7 µs |
| D307 | 304 ns | 1.39 µs | 8.78 µs | 16.6 µs | 21.1 µs |
| D462 | 350 ns | 3.29 µs | 26.5 µs | 35.6 µs | 51.9 µs |
| D616 | 411 ns | 5.06 µs | 37 µs | 49.5 µs | 54.4 µs |
| D924 | 498 ns | 8.33 µs | 88.8 µs | 76 µs | 198 µs |
| D1232 | 904 ns | 16.7 µs | 138 µs | 228 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.2 88.2,191.3 124.4,184.8 160.5,187.0 196.7,195.5 232.9,194.1 269.1,188.9 305.3,185.8 341.5,182.8 377.6,179.3 413.8,175.1 450.0,162.2 450.0,35.2 413.8,45.2 377.6,73.2 341.5,74.2 305.3,93.7 269.1,101.7 232.9,117.4 196.7,124.3 160.5,155.3 124.4,157.2 88.2,163.8 52.0,180.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.2 88.2,191.3 124.4,184.8 160.5,187.0 196.7,195.5 232.9,194.1 269.1,188.9 305.3,185.8 341.5,182.8 377.6,179.3 413.8,175.1 450.0,162.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,205.3 88.2,189.0 124.4,180.1 160.5,166.3 196.7,166.4 232.9,163.9 269.1,156.9 305.3,152.9 341.5,134.1 377.6,124.8 413.8,114.0 450.0,98.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,201.4 88.2,173.6 124.4,167.6 160.5,156.6 196.7,141.3 232.9,132.8 269.1,115.5 305.3,112.8 341.5,88.8 377.6,81.6 413.8,62.6 450.0,53.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.4 88.2,173.8 124.4,157.3 160.5,157.7 196.7,131.6 232.9,135.1 269.1,110.3 305.3,99.0 341.5,82.4 377.6,75.3 413.8,66.0 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,180.4 88.2,163.8 124.4,157.2 160.5,155.3 196.7,124.3 232.9,117.4 269.1,101.7 305.3,93.7 341.5,74.2 377.6,73.2 413.8,45.2 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 1.43 µs | 2.76 µs | 3.46 µs | 3.68 µs |
| D38 | 1.87 ns | 3.22 µs | 3.83 µs | 4.74 µs | 4.72 µs |
| D57 | 2.49 ns | 3.06 µs | 4.34 µs | 5.64 µs | 9.1 µs |
| D76 | 1.99 ns | 6.46 µs | 7.14 µs | 7.89 µs | 8.08 µs |
| D115 | 11.9 ns | 6.66 µs | 13.3 µs | 18.3 µs | 22.3 µs |
| D153 | 15.2 ns | 7.7 µs | 16.1 µs | 13.2 µs | 38 µs |
| D230 | 31.5 ns | 11.6 µs | 22.1 µs | 41.5 µs | 83.1 µs |
| D307 | 85.6 ns | 11.6 µs | 30.4 µs | 78.3 µs | 116 µs |
| D462 | 104 ns | 22.5 µs | 85.9 µs | 166 µs | 247 µs |
| D616 | 136 ns | 38.2 µs | 131 µs | 224 µs | 307 µs |
| D924 | 177 ns | 67.7 µs | 285 µs | 370 µs | 985 µs |
| D1232 | 369 ns | 133 µs | 411 µs | 913 µs | 3.13 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,202.2 124.4,198.7 160.5,201.4 196.7,179.3 232.9,176.2 269.1,167.2 305.3,154.8 341.5,152.3 377.6,149.1 413.8,145.8 450.0,136.7 450.0,24.4 413.8,38.8 377.6,53.2 341.5,55.9 305.3,65.3 269.1,69.4 232.9,79.2 196.7,85.8 160.5,98.4 124.4,96.9 88.2,105.0 52.0,108.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,202.2 124.4,198.7 160.5,201.4 196.7,179.3 232.9,176.2 269.1,167.2 305.3,154.8 341.5,152.3 377.6,149.1 413.8,145.8 450.0,136.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,119.9 88.2,109.8 124.4,110.4 160.5,101.1 196.7,100.8 232.9,99.0 269.1,93.9 305.3,93.9 341.5,85.7 377.6,79.1 413.8,72.0 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.7 88.2,107.6 124.4,106.1 160.5,99.9 196.7,92.2 232.9,89.8 269.1,85.9 305.3,81.9 341.5,69.0 377.6,63.8 413.8,54.2 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,105.0 124.4,102.8 160.5,98.7 196.7,88.2 232.9,92.3 269.1,78.0 305.3,70.2 341.5,60.8 377.6,57.1 413.8,50.9 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.1 88.2,105.0 124.4,96.9 160.5,98.4 196.7,85.8 232.9,79.2 269.1,69.4 305.3,65.3 341.5,55.9 377.6,53.2 413.8,38.8 450.0,24.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 32 ns | 33.3 ns | 40.8 ns | 40.8 ns |
| D38 | 16.3 ns | 41.4 ns | 67.3 ns | 75.4 ns | 94.7 ns |
| D57 | 16.8 ns | 34.9 ns | 68.5 ns | 423 ns | 438 ns |
| D76 | 11.7 ns | 75.7 ns | 424 ns | 377 ns | 495 ns |
| D115 | 21.3 ns | 72.8 ns | 431 ns | 822 ns | 1.02 µs |
| D153 | 23.8 ns | 476 ns | 720 ns | 597 ns | 1.98 µs |
| D230 | 16.9 ns | 391 ns | 1.13 µs | 1.9 µs | 2.93 µs |
| D307 | 43.7 ns | 531 ns | 1.49 µs | 2.84 µs | 4.94 µs |
| D462 | 63 ns | 1.15 µs | 3.32 µs | 6.07 µs | 10.2 µs |
| D616 | 70.3 ns | 2.04 µs | 5.42 µs | 8.51 µs | 13 µs |
| D924 | 98.9 ns | 2.7 µs | 11.1 µs | 14.6 µs | 39.8 µs |
| D1232 | 110 ns | 5.57 µs | 18.8 µs | 34.7 µs | 69.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.7 88.2,199.4 124.4,198.8 160.5,206.6 196.7,193.6 232.9,191.2 269.1,198.7 305.3,178.0 341.5,170.0 377.6,167.7 413.8,160.3 450.0,157.9 450.0,17.8 413.8,30.0 377.6,54.3 341.5,59.6 305.3,75.3 269.1,86.7 232.9,95.2 196.7,109.6 160.5,125.3 124.4,127.9 88.2,161.2 52.0,179.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.7 88.2,199.4 124.4,198.8 160.5,206.6 196.7,193.6 232.9,191.2 269.1,198.7 305.3,178.0 341.5,170.0 377.6,167.7 413.8,160.3 450.0,157.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,184.8 88.2,179.1 124.4,182.8 160.5,166.0 196.7,166.9 232.9,126.1 269.1,130.4 305.3,123.7 341.5,106.9 377.6,94.5 413.8,88.4 450.0,72.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,183.9 88.2,168.6 124.4,168.2 160.5,128.6 196.7,128.3 232.9,117.1 269.1,107.4 305.3,101.3 341.5,83.9 377.6,73.3 413.8,57.8 450.0,46.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,166.1 124.4,128.7 160.5,131.2 196.7,114.3 232.9,121.2 269.1,96.1 305.3,87.4 341.5,70.8 377.6,63.5 413.8,51.8 450.0,33.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,161.2 124.4,127.9 160.5,125.3 196.7,109.6 232.9,95.2 269.1,86.7 305.3,75.3 341.5,59.6 377.6,54.3 413.8,30.0 450.0,17.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 231 ns | 317 ns | 386 ns | 395 ns |
| D38 | 2.18 ns | 374 ns | 385 ns | 422 ns | 388 ns |
| D57 | 268 ns | 394 ns | 483 ns | 487 ns | 618 ns |
| D76 | 180 ns | 513 ns | 476 ns | 537 ns | 406 ns |
| D115 | 291 ns | 506 ns | 621 ns | 944 ns | 1.07 µs |
| D153 | 280 ns | 554 ns | 760 ns | 640 ns | 1.43 µs |
| D230 | 368 ns | 573 ns | 1.04 µs | 1.31 µs | 1.96 µs |
| D307 | 768 ns | 468 ns | 923 ns | 1.35 µs | 10.4 µs |
| D462 | 1.2 µs | 2.84 µs | 3.64 µs | 4.75 µs | 5.38 µs |
| D616 | 1.41 µs | 1.59 µs | 1.88 µs | 2.41 µs | 2.65 µs |
| D924 | 1.94 µs | 1.88 µs | 3.15 µs | 2.41 µs | 5.18 µs |
| D1232 | 3.2 µs | 3.18 µs | 4.27 µs | 5.17 µs | 6.95 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,196.4 124.4,112.8 160.5,119.8 196.7,111.5 232.9,112.1 269.1,107.4 305.3,94.6 341.5,86.8 377.6,84.0 413.8,78.5 450.0,69.8 450.0,56.3 413.8,61.4 377.6,73.0 341.5,60.8 305.3,49.2 269.1,78.3 232.9,83.8 196.7,88.8 160.5,105.7 124.4,98.4 88.2,106.4 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,196.4 124.4,112.8 160.5,119.8 196.7,111.5 232.9,112.1 269.1,107.4 305.3,94.6 341.5,86.8 377.6,84.0 413.8,78.5 450.0,69.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.4 88.2,107.1 124.4,106.2 160.5,101.6 196.7,101.8 232.9,100.3 269.1,99.7 305.3,103.2 341.5,71.9 377.6,81.9 413.8,79.0 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,106.6 124.4,102.7 160.5,102.9 196.7,98.3 232.9,94.8 269.1,89.3 305.3,91.4 341.5,67.5 377.6,79.0 413.8,70.1 450.0,64.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,105.0 124.4,102.5 160.5,100.8 196.7,91.0 232.9,97.7 269.1,85.4 305.3,84.8 341.5,62.9 377.6,74.7 413.8,74.7 450.0,61.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,106.4 124.4,98.4 160.5,105.7 196.7,88.8 232.9,83.8 269.1,78.3 305.3,49.2 341.5,60.8 377.6,73.0 413.8,61.4 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.81 µs | 2.58 µs | 4.59 µs | 6.27 µs | 6.87 µs |
| D38 | 2.81 µs | 5.75 µs | 7.19 µs | 8.64 µs | 9.73 µs |
| D57 | 3.94 µs | 3.19 µs | 4.43 µs | 4.43 µs | 4.69 µs |
| D76 | 2.24 µs | 4.81 µs | 4.48 µs | 4.06 µs | 3.19 µs |
| D115 | 7.87 µs | 8.79 µs | 9 µs | 9.88 µs | 10.1 µs |
| D153 | 7.46 µs | 9.32 µs | 9.89 µs | 6.07 µs | 11.8 µs |
| D230 | 7.75 µs | 11.1 µs | 14.3 µs | 15.3 µs | 18.6 µs |
| D307 | 16.2 µs | 12.9 µs | 17.8 µs | 24 µs | 26.5 µs |
| D462 | 15.8 µs | 20.6 µs | 25.7 µs | 29.9 µs | 34 µs |
| D616 | 27.5 µs | 43 µs | 51.7 µs | 54.1 µs | 54.4 µs |
| D924 | 40.8 µs | 58.6 µs | 104 µs | 80.5 µs | 164 µs |
| D1232 | 57.3 µs | 108 µs | 157 µs | 203 µs | 252 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.1 88.2,180.0 124.4,170.3 160.5,186.7 196.7,150.3 232.9,151.8 269.1,150.7 305.3,129.3 341.5,130.1 377.6,114.1 413.8,102.6 450.0,92.8 450.0,49.9 413.8,62.4 377.6,94.3 341.5,107.9 305.3,115.1 269.1,125.3 232.9,138.6 196.7,143.0 160.5,176.4 124.4,165.3 88.2,144.1 52.0,154.2" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.1 88.2,180.0 124.4,170.3 160.5,186.7 196.7,150.3 232.9,151.8 269.1,150.7 305.3,129.3 341.5,130.1 377.6,114.1 413.8,102.6 450.0,92.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,182.5 88.2,159.4 124.4,176.4 160.5,164.6 196.7,147.1 232.9,145.4 269.1,140.3 305.3,135.9 341.5,122.3 377.6,101.1 413.8,92.2 450.0,74.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,165.9 88.2,152.9 124.4,166.9 160.5,166.6 196.7,146.4 232.9,143.7 269.1,133.0 305.3,126.7 341.5,116.1 377.6,95.8 413.8,75.5 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.9 88.2,147.6 124.4,166.9 160.5,169.4 196.7,143.7 232.9,157.8 269.1,131.1 305.3,117.9 341.5,111.6 377.6,94.5 413.8,82.9 450.0,56.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.2 88.2,144.1 124.4,165.3 160.5,176.4 196.7,143.0 232.9,138.6 269.1,125.3 305.3,115.1 341.5,107.9 377.6,94.3 413.8,62.4 450.0,49.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.02 ns | 1.83 µs | 3.3 µs | 4.07 µs | 4.32 µs |
| D38 | 6.54 ns | 3.87 µs | 4.45 µs | 5.41 µs | 5.4 µs |
| D57 | 56.5 ns | 3.5 µs | 4.56 µs | 4.75 µs | 5.18 µs |
| D76 | 61.6 ns | 4.62 µs | 4.75 µs | 4.52 µs | 3.95 µs |
| D115 | 138 ns | 8.42 µs | 9.16 µs | 10.5 µs | 11 µs |
| D153 | 165 ns | 9.16 µs | 10.4 µs | 6.66 µs | 13.2 µs |
| D230 | 200 ns | 11.4 µs | 15.4 µs | 18.5 µs | 23.1 µs |
| D307 | 451 ns | 14.2 µs | 18.2 µs | 28.5 µs | 32.3 µs |
| D462 | 627 ns | 70.3 µs | 149 µs | 224 µs | 284 µs |
| D616 | 756 ns | 193 µs | 328 µs | 301 µs | 396 µs |
| D924 | 896 ns | 353 µs | 484 µs | 513 µs | 1.74 ms |
| D1232 | 1.52 µs | 766 µs | 803 µs | 2.11 ms | 2.76 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.7 88.2,186.7 124.4,159.9 160.5,158.9 196.7,148.8 232.9,146.6 269.1,144.3 305.3,134.2 341.5,130.1 377.6,127.8 413.8,125.7 450.0,119.1 450.0,26.0 413.8,31.7 377.6,50.1 341.5,54.2 305.3,81.2 269.1,85.3 232.9,92.3 196.7,94.6 160.5,107.3 124.4,103.9 88.2,103.4 52.0,106.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.7 88.2,186.7 124.4,159.9 160.5,158.9 196.7,148.8 232.9,146.6 269.1,144.3 305.3,134.2 341.5,130.1 377.6,127.8 413.8,125.7 450.0,119.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,116.8 88.2,107.5 124.4,108.8 160.5,105.3 196.7,97.8 232.9,96.8 269.1,94.1 305.3,91.4 341.5,71.5 377.6,59.0 413.8,51.5 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.5 88.2,105.8 124.4,105.5 160.5,105.0 196.7,96.8 232.9,95.2 269.1,90.4 305.3,88.3 341.5,62.2 377.6,52.4 413.8,47.6 450.0,41.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,103.3 124.4,104.9 160.5,105.6 196.7,95.1 232.9,100.8 269.1,88.1 305.3,82.7 341.5,57.2 377.6,53.5 413.8,46.9 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,103.4 124.4,103.9 160.5,107.3 196.7,94.6 232.9,92.3 269.1,85.3 305.3,81.2 341.5,54.2 377.6,50.1 413.8,31.7 450.0,26.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.72 ns | 11.7 ns | 14.3 ns | 29.1 ns | 28.9 ns |
| D38 | 8 ns | 19.7 ns | 31.2 ns | 338 ns | 608 ns |
| D57 | 158 ns | 159 ns | 487 ns | 769 ns | 765 ns |
| D76 | 121 ns | 283 ns | 765 ns | 593 ns | 725 ns |
| D115 | 87.9 ns | 657 ns | 995 ns | 1.51 µs | 1.68 µs |
| D153 | 99.2 ns | 1.03 µs | 1.52 µs | 987 ns | 2.6 µs |
| D230 | 98.7 ns | 369 ns | 2.4 µs | 3.49 µs | 4.21 µs |
| D307 | 152 ns | 1.61 µs | 2.8 µs | 4.68 µs | 7.09 µs |
| D462 | 176 ns | 3.62 µs | 5.99 µs | 9.7 µs | 11.5 µs |
| D616 | 221 ns | 1.92 µs | 10.4 µs | 11 µs | 14.6 µs |
| D924 | 240 ns | 2.24 µs | 16.9 µs | 16.9 µs | 35.2 µs |
| D1232 | 383 ns | 15.6 µs | 27.9 µs | 40 µs | 60.7 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.7 88.2,173.9 124.4,122.1 160.5,126.8 196.7,132.2 232.9,130.1 269.1,130.2 305.3,122.7 341.5,120.2 377.6,116.2 413.8,114.8 450.0,106.7 450.0,18.7 413.8,28.1 377.6,43.4 341.5,47.5 305.3,56.0 269.1,65.0 232.9,73.4 196.7,80.9 160.5,95.6 124.4,94.6 88.2,98.6 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.7 88.2,173.9 124.4,122.1 160.5,126.8 196.7,132.2 232.9,130.1 269.1,130.2 305.3,122.7 341.5,120.2 377.6,116.2 413.8,114.8 450.0,106.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,167.3 88.2,158.2 124.4,122.0 160.5,111.9 196.7,97.3 232.9,89.5 269.1,107.3 305.3,81.7 341.5,67.7 377.6,78.7 413.8,76.0 450.0,42.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,163.8 88.2,150.2 124.4,102.5 160.5,94.6 196.7,90.1 232.9,82.8 269.1,74.8 305.3,72.1 341.5,58.9 377.6,49.4 413.8,40.9 450.0,32.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.4 88.2,108.8 124.4,94.6 160.5,99.1 196.7,82.8 232.9,90.2 269.1,68.3 305.3,63.2 341.5,50.5 377.6,48.3 413.8,40.9 450.0,25.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,98.6 124.4,94.6 160.5,95.6 196.7,80.9 232.9,73.4 269.1,65.0 305.3,56.0 341.5,47.5 377.6,43.4 413.8,28.1 450.0,18.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
