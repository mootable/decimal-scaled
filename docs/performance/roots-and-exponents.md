# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 77.4 ns | 130 ns | 158 ns | 165 ns | 205 ns |
| D38 | 68.9 ns | 155 ns | 253 ns | 296 ns | 263 ns |
| D57 | 183 ns | 427 ns | 699 ns | 1.13 µs | 1.13 µs |
| D76 | 198 ns | 352 ns | 511 ns | 1.26 µs | 1.67 µs |
| D115 | 223 ns | 715 ns | 1.38 µs | 1.69 µs | 2.99 µs |
| D153 | 255 ns | 733 ns | 1.78 µs | 2.46 µs | 4.02 µs |
| D230 | 261 ns | 1.41 µs | 3.01 µs | 5.13 µs | 8.76 µs |
| D307 | 276 ns | 1.41 µs | 4.73 µs | 8.87 µs | 11.6 µs |
| D462 | 395 ns | 3.16 µs | 8.98 µs | 18.6 µs | 33.4 µs |
| D616 | 466 ns | 4.5 µs | 12.1 µs | 30.1 µs | 43.7 µs |
| D924 | 440 ns | 9.21 µs | 29.6 µs | 50.4 µs | 84.2 µs |
| D1232 | 1.35 µs | 16 µs | 47.3 µs | 119 µs | 224 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.4 88.2,176.5 124.4,159.5 160.5,158.1 196.7,156.1 232.9,153.7 269.1,153.4 305.3,152.4 341.5,146.2 377.6,143.3 413.8,144.2 450.0,124.8 450.0,36.0 413.8,53.0 377.6,64.4 341.5,69.1 305.3,87.4 269.1,92.3 232.9,105.8 196.7,111.0 160.5,121.1 124.4,127.8 88.2,153.2 52.0,157.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.4 88.2,176.5 124.4,159.5 160.5,158.1 196.7,156.1 232.9,153.7 269.1,153.4 305.3,152.4 341.5,146.2 377.6,143.3 413.8,144.2 450.0,124.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.5 88.2,162.4 124.4,144.8 160.5,148.2 196.7,135.8 232.9,135.4 269.1,124.0 305.3,124.0 341.5,110.0 377.6,103.9 413.8,91.4 450.0,81.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.0 88.2,153.9 124.4,136.2 160.5,141.7 196.7,124.4 232.9,120.0 269.1,110.8 305.3,103.0 341.5,91.9 377.6,86.6 413.8,71.2 450.0,63.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,161.3 88.2,151.1 124.4,127.9 160.5,126.0 196.7,120.8 232.9,114.4 269.1,101.6 305.3,92.1 341.5,79.2 377.6,70.8 413.8,61.9 450.0,47.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.5 88.2,153.2 124.4,127.8 160.5,121.1 196.7,111.0 232.9,105.8 269.1,92.3 305.3,87.4 341.5,69.1 377.6,64.4 413.8,53.0 450.0,36.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 1.47 µs | 1.62 µs | 3.19 µs | 2.13 µs | 3.17 µs |
| D38 | 1.3 µs | 3.18 µs | 3.79 µs | 3.72 µs | 2.86 µs |
| D57 | 2.74 µs | 3.88 µs | 4.32 µs | 5.83 µs | 9.51 µs |
| D76 | 5.51 µs | 3.44 µs | 3.97 µs | 9.26 µs | 12.5 µs |
| D115 | 5.26 µs | 5.59 µs | 13.2 µs | 16.4 µs | 22.3 µs |
| D153 | 5.34 µs | 5.96 µs | 16.2 µs | 16.8 µs | 31.3 µs |
| D230 | 3.94 µs | 13.2 µs | 22.2 µs | 42.4 µs | 74.6 µs |
| D307 | 3.97 µs | 11.9 µs | 38.5 µs | 78.4 µs | 97.7 µs |
| D462 | 4.75 µs | 22.5 µs | 80.3 µs | 165 µs | 248 µs |
| D616 | 5.38 µs | 34.7 µs | 111 µs | 259 µs | 347 µs |
| D924 | 5.18 µs | 81.5 µs | 259 µs | 472 µs | 711 µs |
| D1232 | 5.78 µs | 140 µs | 403 µs | 922 µs | 2.82 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,201.6 88.2,204.3 124.4,188.1 160.5,173.0 196.7,173.9 232.9,173.6 269.1,180.2 305.3,180.0 341.5,176.2 377.6,173.5 413.8,174.3 450.0,171.9 450.0,37.5 413.8,67.4 377.6,83.0 341.5,90.3 305.3,110.5 269.1,116.4 232.9,135.2 196.7,142.6 160.5,155.2 124.4,161.1 88.2,187.2 52.0,184.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,201.6 88.2,204.3 124.4,188.1 160.5,173.0 196.7,173.9 232.9,173.6 269.1,180.2 305.3,180.0 341.5,176.2 377.6,173.5 413.8,174.3 450.0,171.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,199.5 88.2,184.9 124.4,180.6 160.5,183.2 196.7,172.6 232.9,171.2 269.1,154.0 305.3,156.2 341.5,142.4 377.6,133.0 413.8,114.4 450.0,102.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.8 88.2,181.1 124.4,178.2 160.5,180.1 196.7,153.9 232.9,149.5 269.1,142.7 305.3,130.8 341.5,114.8 377.6,107.7 413.8,89.3 450.0,79.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,193.6 88.2,181.5 124.4,171.7 160.5,161.7 196.7,149.3 232.9,148.7 269.1,128.6 305.3,115.3 341.5,99.1 377.6,89.4 413.8,76.3 450.0,61.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.9 88.2,187.2 124.4,161.1 160.5,155.2 196.7,142.6 232.9,135.2 269.1,116.4 305.3,110.5 341.5,90.3 377.6,83.0 413.8,67.4 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17 ns | 40.5 ns | 40.8 ns | 26.3 ns | 31.1 ns |
| D38 | 11.7 ns | 41.4 ns | 66.4 ns | 58.5 ns | 67.9 ns |
| D57 | 12.4 ns | 41.5 ns | 83 ns | 374 ns | 393 ns |
| D76 | 17.3 ns | 65 ns | 252 ns | 381 ns | 649 ns |
| D115 | 22.3 ns | 91.6 ns | 390 ns | 647 ns | 951 ns |
| D153 | 26.3 ns | 329 ns | 671 ns | 735 ns | 1.45 µs |
| D230 | 17 ns | 423 ns | 1.06 µs | 1.86 µs | 2.73 µs |
| D307 | 22 ns | 526 ns | 1.74 µs | 2.81 µs | 4.15 µs |
| D462 | 59.9 ns | 1.2 µs | 3.16 µs | 6.1 µs | 7.89 µs |
| D616 | 70.2 ns | 1.92 µs | 4.61 µs | 9.67 µs | 10.8 µs |
| D924 | 78.9 ns | 3.27 µs | 9.7 µs | 18.7 µs | 21 µs |
| D1232 | 100 ns | 6.07 µs | 16.9 µs | 23.8 µs | 48.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,206.7 124.4,205.3 160.5,198.1 196.7,192.6 232.9,189.0 269.1,198.5 305.3,192.9 341.5,171.1 377.6,167.7 413.8,165.1 450.0,160.0 450.0,25.5 413.8,43.9 377.6,58.3 341.5,65.1 305.3,79.1 269.1,88.2 232.9,101.9 196.7,111.1 160.5,119.4 124.4,130.3 88.2,168.4 52.0,185.4" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,206.7 124.4,205.3 160.5,198.1 196.7,192.6 232.9,189.0 269.1,198.5 305.3,192.9 341.5,171.1 377.6,167.7 413.8,165.1 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,179.1 160.5,169.3 196.7,161.9 232.9,134.1 269.1,128.7 305.3,123.9 341.5,106.0 377.6,95.8 413.8,84.2 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,168.9 124.4,164.0 160.5,139.9 196.7,130.5 232.9,118.7 269.1,108.7 305.3,97.9 341.5,85.0 377.6,76.8 413.8,60.7 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,189.0 88.2,171.6 124.4,131.4 160.5,131.0 196.7,119.4 232.9,116.7 269.1,96.6 305.3,87.5 341.5,70.7 377.6,60.7 413.8,46.4 450.0,41.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.4 88.2,168.4 124.4,130.3 160.5,119.4 196.7,111.1 232.9,101.9 269.1,88.2 305.3,79.1 341.5,65.1 377.6,58.3 413.8,43.9 450.0,25.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.23 ns | 272 ns | 403 ns | 264 ns | 335 ns |
| D38 | 7.32 ns | 379 ns | 390 ns | 336 ns | 267 ns |
| D57 | 218 ns | 488 ns | 482 ns | 485 ns | 606 ns |
| D76 | 295 ns | 308 ns | 292 ns | 620 ns | 763 ns |
| D115 | 295 ns | 442 ns | 622 ns | 945 ns | 983 ns |
| D153 | 320 ns | 461 ns | 757 ns | 810 ns | 1.28 µs |
| D230 | 373 ns | 662 ns | 1.05 µs | 1.29 µs | 1.81 µs |
| D307 | 455 ns | 502 ns | 1.2 µs | 1.34 µs | 9.12 µs |
| D462 | 1.21 µs | 2.83 µs | 3.35 µs | 4.3 µs | 5.4 µs |
| D616 | 1.41 µs | 1.45 µs | 1.89 µs | 2.49 µs | 3.31 µs |
| D924 | 1.7 µs | 2.05 µs | 2.58 µs | 3.04 µs | 3.63 µs |
| D1232 | 3.16 µs | 3.32 µs | 3.48 µs | 5.19 µs | 7.17 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.3 88.2,166.8 124.4,93.1 160.5,86.5 196.7,86.5 232.9,84.7 269.1,81.4 305.3,77.1 341.5,55.9 377.6,52.5 413.8,48.5 450.0,35.0 450.0,17.2 413.8,32.0 377.6,34.0 341.5,23.4 305.3,12.0 269.1,47.1 232.9,54.6 196.7,60.4 160.5,65.9 124.4,70.9 88.2,88.6 52.0,83.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.3 88.2,166.8 124.4,93.1 160.5,86.5 196.7,86.5 232.9,84.7 269.1,81.4 305.3,77.1 341.5,55.9 377.6,52.5 413.8,48.5 450.0,35.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,88.3 88.2,81.1 124.4,75.6 160.5,85.6 196.7,77.7 232.9,76.8 269.1,69.0 305.3,75.0 341.5,37.4 377.6,52.0 413.8,44.4 450.0,34.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,79.7 88.2,80.4 124.4,75.9 160.5,86.8 196.7,70.3 232.9,66.0 269.1,58.9 305.3,56.0 341.5,33.8 377.6,46.2 413.8,39.4 450.0,32.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,88.9 88.2,83.7 124.4,75.7 160.5,70.4 196.7,61.2 232.9,64.6 269.1,54.4 305.3,53.6 341.5,28.3 377.6,40.2 413.8,35.8 450.0,24.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,83.7 88.2,88.6 124.4,70.9 160.5,65.9 196.7,60.4 232.9,54.6 269.1,47.1 305.3,12.0 341.5,23.4 377.6,34.0 413.8,32.0 450.0,17.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.75 µs | 3.14 µs | 5.85 µs | 3.93 µs | 5.88 µs |
| D38 | 2.24 µs | 5.82 µs | 7.31 µs | 7.32 µs | 6.45 µs |
| D57 | 1.68 µs | 2.55 µs | 2.72 µs | 2.78 µs | 2.92 µs |
| D76 | 2.73 µs | 1.45 µs | 1.61 µs | 2.9 µs | 3.33 µs |
| D115 | 4.51 µs | 4.49 µs | 5.25 µs | 5.08 µs | 5.75 µs |
| D153 | 4.62 µs | 4.51 µs | 5.93 µs | 4.42 µs | 5.6 µs |
| D230 | 4.47 µs | 7.25 µs | 8.41 µs | 8.81 µs | 9.55 µs |
| D307 | 6.37 µs | 7.88 µs | 13.5 µs | 14.2 µs | 13.3 µs |
| D462 | 7.94 µs | 12.5 µs | 14.4 µs | 17.8 µs | 20.2 µs |
| D616 | 16.1 µs | 23.7 µs | 26 µs | 32.2 µs | 36.6 µs |
| D924 | 21.1 µs | 40.2 µs | 49.7 µs | 59.8 µs | 65.5 µs |
| D1232 | 32.6 µs | 66.7 µs | 81.8 µs | 116 µs | 155 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,180.7 88.2,186.7 124.4,195.0 160.5,180.9 196.7,166.4 232.9,165.7 269.1,166.7 305.3,156.4 341.5,150.0 377.6,129.5 413.8,121.8 450.0,109.1 450.0,64.0 413.8,88.9 377.6,105.8 341.5,123.0 305.3,135.1 269.1,144.7 232.9,160.1 196.7,159.3 160.5,175.2 124.4,179.0 88.2,156.0 52.0,158.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,180.7 88.2,186.7 124.4,195.0 160.5,180.9 196.7,166.4 232.9,165.7 269.1,166.7 305.3,156.4 341.5,150.0 377.6,129.5 413.8,121.8 450.0,109.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,176.9 88.2,159.0 124.4,182.9 160.5,199.3 196.7,166.5 232.9,166.4 269.1,152.6 305.3,150.2 341.5,136.9 377.6,118.4 413.8,103.0 450.0,88.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.9 88.2,152.4 124.4,181.1 160.5,196.3 196.7,162.0 232.9,158.5 269.1,148.3 305.3,134.5 341.5,132.8 377.6,115.6 413.8,96.9 450.0,82.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,170.3 88.2,152.4 124.4,180.4 160.5,179.2 196.7,163.0 232.9,167.0 269.1,147.0 305.3,133.1 341.5,126.7 377.6,109.5 413.8,91.6 450.0,72.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.7 88.2,156.0 124.4,179.0 160.5,175.2 196.7,159.3 232.9,160.1 269.1,144.7 305.3,135.1 341.5,123.0 377.6,105.8 413.8,88.9 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log10`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 388 ns | 385 ns | 1.27 µs | 719 ns | 1.23 µs |
| D38 | 313 ns | 1.25 µs | 1.48 µs | 1.43 µs | 784 ns |
| D57 | 839 ns | 1.31 µs | 1.33 µs | 1.42 µs | 1.45 µs |
| D76 | 1.42 µs | 731 ns | 779 ns | 1.51 µs | 1.74 µs |
| D115 | 2.25 µs | 2.24 µs | 2.57 µs | 2.6 µs | 2.83 µs |
| D153 | 2.28 µs | 2.29 µs | 2.94 µs | 2.29 µs | 2.82 µs |
| D230 | 2.28 µs | 3.37 µs | 3.94 µs | 4.18 µs | 4.46 µs |
| D307 | 3.18 µs | 3.85 µs | 6.25 µs | 6.54 µs | 6 µs |
| D462 | 3.92 µs | 5.97 µs | 6.68 µs | 8.12 µs | 9.32 µs |
| D616 | 7.91 µs | 10.1 µs | 10.8 µs | 12.8 µs | 14.3 µs |
| D924 | 10.2 µs | 16 µs | 18.3 µs | 22.3 µs | 24.1 µs |
| D1232 | 15.9 µs | 25.5 µs | 28.7 µs | 40.5 µs | 53.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,170.7 88.2,177.0 124.4,148.4 160.5,133.3 196.7,119.8 232.9,119.4 269.1,119.5 305.3,109.8 341.5,103.8 377.6,83.5 413.8,76.1 450.0,63.2 450.0,28.1 413.8,51.2 377.6,66.3 341.5,78.7 305.3,91.5 269.1,100.1 232.9,113.4 196.7,113.2 160.5,127.3 124.4,132.6 88.2,150.4 52.0,137.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,170.7 88.2,177.0 124.4,148.4 160.5,133.3 196.7,119.8 232.9,119.4 269.1,119.5 305.3,109.8 341.5,103.8 377.6,83.5 413.8,76.1 450.0,63.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,171.0 88.2,137.0 124.4,135.6 160.5,152.4 196.7,120.0 232.9,119.3 269.1,108.2 305.3,104.3 341.5,91.6 377.6,76.5 413.8,63.0 450.0,49.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.5 88.2,132.0 124.4,135.1 160.5,150.5 196.7,116.0 232.9,112.1 269.1,103.7 305.3,90.3 341.5,88.4 377.6,74.5 413.8,59.1 450.0,46.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.9 88.2,133.0 124.4,133.3 160.5,131.4 196.7,115.7 232.9,119.3 269.1,101.9 305.3,89.0 341.5,82.7 377.6,69.6 413.8,53.5 450.0,36.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.3 88.2,150.4 124.4,132.6 160.5,127.3 196.7,113.2 232.9,113.4 269.1,100.1 305.3,91.5 341.5,78.7 377.6,66.3 413.8,51.2 450.0,28.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log2`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 343 ns | 337 ns | 1.25 µs | 597 ns | 1.21 µs |
| D38 | 278 ns | 1.22 µs | 1.45 µs | 1.41 µs | 774 ns |
| D57 | 649 ns | 971 ns | 1.29 µs | 1.34 µs | 1.4 µs |
| D76 | 1.11 µs | 580 ns | 641 ns | 1.42 µs | 1.52 µs |
| D115 | 1.81 µs | 1.95 µs | 2.28 µs | 2.17 µs | 2.53 µs |
| D153 | 1.85 µs | 1.86 µs | 2.43 µs | 1.87 µs | 2.3 µs |
| D230 | 1.72 µs | 2.86 µs | 3.25 µs | 3.58 µs | 3.99 µs |
| D307 | 2.52 µs | 3.2 µs | 5.46 µs | 5.55 µs | 5.42 µs |
| D462 | 3.26 µs | 5.01 µs | 5.73 µs | 7.11 µs | 8.22 µs |
| D616 | 5.99 µs | 8.27 µs | 9.35 µs | 11.1 µs | 12.7 µs |
| D924 | 7.47 µs | 13.6 µs | 15.9 µs | 19.1 µs | 21.4 µs |
| D1232 | 12.6 µs | 22 µs | 26.2 µs | 37 µs | 49.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,174.3 88.2,180.4 124.4,155.9 160.5,140.4 196.7,126.1 232.9,125.5 269.1,127.6 305.3,116.5 341.5,109.1 377.6,91.5 413.8,85.1 450.0,70.1 450.0,30.1 413.8,54.6 377.6,69.6 341.5,82.3 305.3,94.4 269.1,103.3 232.9,119.2 196.7,116.4 160.5,131.3 124.4,133.6 88.2,150.8 52.0,137.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,174.3 88.2,180.4 124.4,155.9 160.5,140.4 196.7,126.1 232.9,125.5 269.1,127.6 305.3,116.5 341.5,109.1 377.6,91.5 413.8,85.1 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,174.8 88.2,137.5 124.4,144.2 160.5,159.1 196.7,124.0 232.9,125.3 269.1,112.9 305.3,109.7 341.5,96.7 377.6,82.2 413.8,67.8 450.0,53.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,136.8 88.2,132.6 124.4,136.1 160.5,156.2 196.7,119.4 232.9,117.7 269.1,109.2 305.3,94.2 341.5,92.8 377.6,78.6 413.8,63.3 450.0,48.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.3 88.2,133.4 124.4,134.8 160.5,133.3 196.7,120.9 232.9,125.2 269.1,106.4 305.3,93.7 341.5,86.6 377.6,73.6 413.8,57.9 450.0,38.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.8 88.2,150.8 124.4,133.6 160.5,131.3 196.7,116.4 232.9,119.2 269.1,103.3 305.3,94.4 341.5,82.3 377.6,69.6 413.8,54.6 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.5 ns | 2.05 µs | 3.85 µs | 2.56 µs | 3.73 µs |
| D38 | 10.1 ns | 3.83 µs | 4.4 µs | 4.29 µs | 3.34 µs |
| D57 | 61.5 ns | 4.15 µs | 4.56 µs | 4.67 µs | 5.07 µs |
| D76 | 80.5 ns | 2.25 µs | 2.85 µs | 5.11 µs | 6.15 µs |
| D115 | 134 ns | 7.27 µs | 8.95 µs | 9.03 µs | 10.7 µs |
| D153 | 176 ns | 7.49 µs | 10.2 µs | 8.17 µs | 10.5 µs |
| D230 | 205 ns | 12.8 µs | 15.2 µs | 18.3 µs | 20.4 µs |
| D307 | 262 ns | 14 µs | 22.9 µs | 28.7 µs | 27 µs |
| D462 | 668 ns | 69.9 µs | 137 µs | 222 µs | 282 µs |
| D616 | 768 ns | 177 µs | 277 µs | 342 µs | 449 µs |
| D924 | 810 ns | 418 µs | 435 µs | 661 µs | 1.27 ms |
| D1232 | 1.69 µs | 829 µs | 781 µs | 2.15 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="176.7" x2="450" y2="176.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="179.7" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="43.3" x2="450" y2="43.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="46.3" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,202.7 88.2,209.8 124.4,183.7 160.5,179.8 196.7,172.4 232.9,168.5 269.1,166.3 305.3,162.7 341.5,149.2 377.6,147.2 413.8,146.4 450.0,135.8 450.0,27.5 413.8,39.8 377.6,54.9 341.5,61.6 305.3,95.6 269.1,99.7 232.9,109.3 196.7,109.0 160.5,117.0 124.4,119.8 88.2,125.9 52.0,124.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,202.7 88.2,209.8 124.4,183.7 160.5,179.8 196.7,172.4 232.9,168.5 269.1,166.3 305.3,162.7 341.5,149.2 377.6,147.2 413.8,146.4 450.0,135.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,133.0 88.2,123.9 124.4,122.7 160.5,131.6 196.7,114.6 232.9,114.2 269.1,106.5 305.3,105.1 341.5,81.8 377.6,68.4 413.8,55.9 450.0,46.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,123.8 88.2,121.9 124.4,121.4 160.5,128.2 196.7,111.6 232.9,109.7 269.1,103.9 305.3,98.0 341.5,72.1 377.6,61.9 413.8,55.4 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,129.7 88.2,122.3 124.4,121.0 160.5,119.7 196.7,111.5 232.9,112.9 269.1,101.2 305.3,94.7 341.5,65.2 377.6,58.8 413.8,49.3 450.0,32.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.3 88.2,125.9 124.4,119.8 160.5,117.0 196.7,109.0 232.9,109.3 269.1,99.7 305.3,95.6 341.5,61.6 377.6,54.9 413.8,39.8 450.0,27.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.74 ns | 13.2 ns | 18.1 ns | 21.4 ns | 23.3 ns |
| D38 | 5.05 ns | 20.2 ns | 40.3 ns | 123 ns | 125 ns |
| D57 | 85.2 ns | 151 ns | 206 ns | 359 ns | 373 ns |
| D76 | 84.4 ns | 91.9 ns | 180 ns | 372 ns | 635 ns |
| D115 | 97.7 ns | 205 ns | 388 ns | 608 ns | 939 ns |
| D153 | 113 ns | 270 ns | 657 ns | 714 ns | 1.44 µs |
| D230 | 102 ns | 435 ns | 955 ns | 1.81 µs | 2.67 µs |
| D307 | 139 ns | 512 ns | 1.7 µs | 2.79 µs | 4.04 µs |
| D462 | 212 ns | 1.03 µs | 2.64 µs | 6.28 µs | 7.62 µs |
| D616 | 245 ns | 1.82 µs | 3.93 µs | 7.02 µs | 10.5 µs |
| D924 | 189 ns | 2.69 µs | 7.08 µs | 13.6 µs | 20 µs |
| D1232 | 386 ns | 5.17 µs | 13.4 µs | 24.3 µs | 48.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,179.6 88.2,181.9 124.4,132.8 160.5,132.9 196.7,130.4 232.9,127.9 269.1,129.6 305.3,124.3 341.5,117.0 377.6,114.4 413.8,118.9 450.0,106.5 450.0,22.7 413.8,38.0 377.6,49.1 341.5,54.7 305.3,65.8 269.1,72.9 232.9,83.7 196.7,91.1 160.5,97.9 124.4,107.1 88.2,126.1 52.0,155.3" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,179.6 88.2,181.9 124.4,132.8 160.5,132.9 196.7,130.4 232.9,127.9 269.1,129.6 305.3,124.3 341.5,117.0 377.6,114.4 413.8,118.9 450.0,106.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.2 88.2,157.8 124.4,122.9 160.5,131.5 196.7,117.5 232.9,112.7 269.1,104.4 305.3,101.6 341.5,89.5 377.6,79.6 413.8,72.8 450.0,61.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.7 88.2,145.8 124.4,117.4 160.5,119.8 196.7,106.4 232.9,97.3 269.1,90.8 305.3,80.8 341.5,73.1 377.6,66.2 413.8,56.0 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.8 88.2,126.4 124.4,107.8 160.5,107.2 196.7,98.6 232.9,95.9 269.1,79.7 305.3,72.2 341.5,58.1 377.6,56.1 413.8,44.6 450.0,34.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,155.3 88.2,126.1 124.4,107.1 160.5,97.9 196.7,91.1 232.9,83.7 269.1,72.9 305.3,65.8 341.5,54.7 377.6,49.1 413.8,38.0 450.0,22.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
