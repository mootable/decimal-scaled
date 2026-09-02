# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.73 µs | 2.7 µs | 5.15 µs | 3.36 µs | 7.71 µs |
| D38 | 2.64 µs | 5.15 µs | 7.71 µs | 9.67 µs | 8.45 µs |
| D57 | 340 ns | 592 ns | 505 ns | 1.06 µs | 1.08 µs |
| D76 | 519 ns | 510 ns | 742 ns | 1.01 µs | 1.91 µs |
| D115 | 336 ns | 2.3 µs | 2.37 µs | 2.94 µs | 3.94 µs |
| D153 | 526 ns | 2.25 µs | 3.48 µs | 5.55 µs | 6.7 µs |
| D230 | 391 ns | 4.32 µs | 8.18 µs | 10.6 µs | 8.94 µs |
| D307 | 564 ns | 6.16 µs | 11.1 µs | 16.5 µs | 21 µs |
| D462 | 467 ns | 9.85 µs | 25.8 µs | 33.6 µs | 51.5 µs |
| D616 | 673 ns | 15.7 µs | 40.4 µs | 49.6 µs | 85.6 µs |
| D924 | 563 ns | 30.1 µs | 82.2 µs | 121 µs | 180 µs |
| D1232 | 1.24 µs | 55.9 µs | 118 µs | 227 µs | 223 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,122.1 88.2,138.9 124.4,183.4 160.5,174.3 196.7,183.7 232.9,173.9 269.1,180.4 305.3,172.4 341.5,176.5 377.6,168.6 413.8,172.5 450.0,155.4 450.0,42.6 413.8,47.2 377.6,63.4 341.5,74.4 305.3,93.9 269.1,112.4 232.9,118.7 196.7,130.2 160.5,146.0 124.4,158.4 88.2,113.7 52.0,115.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,122.1 88.2,138.9 124.4,183.4 160.5,174.3 196.7,183.7 232.9,173.9 269.1,180.4 305.3,172.4 341.5,176.5 377.6,168.6 413.8,172.5 450.0,155.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,138.4 88.2,124.4 124.4,171.4 160.5,174.6 196.7,141.9 232.9,142.4 269.1,128.2 305.3,120.5 341.5,110.3 377.6,100.2 413.8,86.1 450.0,72.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,115.6 124.4,174.8 160.5,166.5 196.7,141.2 232.9,132.9 269.1,114.4 305.3,107.8 341.5,89.4 377.6,79.7 413.8,64.3 450.0,56.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.7 88.2,110.7 124.4,158.8 160.5,159.8 196.7,136.6 232.9,122.8 269.1,108.6 305.3,99.1 341.5,83.7 377.6,75.2 413.8,55.9 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.6 88.2,113.7 124.4,158.4 160.5,146.0 196.7,130.2 232.9,118.7 269.1,112.4 305.3,93.9 341.5,74.4 377.6,63.4 413.8,47.2 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.16 ns | 3.01 µs | 6.26 µs | 3.8 µs | 7.26 µs |
| D38 | 1.74 ns | 6.29 µs | 7.62 µs | 8.79 µs | 8.32 µs |
| D57 | 2.81 ns | 3.88 µs | 3.73 µs | 6.17 µs | 10.1 µs |
| D76 | 3.23 ns | 4.34 µs | 5.1 µs | 7.7 µs | 11.3 µs |
| D115 | 13.3 ns | 6.61 µs | 13.8 µs | 16.3 µs | 17.9 µs |
| D153 | 16.2 ns | 4.83 µs | 16.2 µs | 22.4 µs | 35.1 µs |
| D230 | 45.9 ns | 13.3 µs | 23.6 µs | 47 µs | 54.5 µs |
| D307 | 68.1 ns | 15.4 µs | 34.3 µs | 77.8 µs | 114 µs |
| D462 | 130 ns | 22.2 µs | 85.3 µs | 155 µs | 249 µs |
| D616 | 122 ns | 38.3 µs | 139 µs | 225 µs | 441 µs |
| D924 | 205 ns | 81.2 µs | 269 µs | 565 µs | 928 µs |
| D1232 | 353 ns | 132 µs | 359 µs | 926 µs | 2.15 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.7 88.2,203.1 124.4,197.2 160.5,195.5 196.7,177.9 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.6 377.6,150.4 413.8,143.9 450.0,137.2 450.0,29.1 413.8,39.5 377.6,48.7 341.5,55.8 305.3,65.5 269.1,74.7 232.9,80.1 196.7,88.5 160.5,94.2 124.4,95.6 88.2,98.0 52.0,99.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.7 88.2,203.1 124.4,197.2 160.5,195.5 196.7,177.9 232.9,175.4 269.1,162.5 305.3,157.6 341.5,149.6 377.6,150.4 413.8,143.9 450.0,137.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,110.6 88.2,101.5 124.4,107.4 160.5,106.1 196.7,100.9 232.9,104.8 269.1,92.2 305.3,90.4 341.5,85.8 377.6,79.1 413.8,69.7 450.0,63.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.5 88.2,99.1 124.4,108.0 160.5,104.1 196.7,91.8 232.9,89.7 269.1,85.1 305.3,80.4 341.5,69.1 377.6,63.0 413.8,54.9 450.0,51.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.7 88.2,97.3 124.4,101.7 160.5,99.0 196.7,89.6 232.9,85.7 269.1,76.5 305.3,70.3 341.5,61.7 377.6,57.1 413.8,45.6 450.0,39.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.7 88.2,98.0 124.4,95.6 160.5,94.2 196.7,88.5 232.9,80.1 269.1,74.7 305.3,65.5 341.5,55.8 377.6,48.7 413.8,39.5 450.0,29.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 17.3 ns | 37.7 ns | 41.4 ns | 31.4 ns | 41.4 ns |
| D38 | 13.7 ns | 41.2 ns | 66.9 ns | 66.9 ns | 82.6 ns |
| D57 | 16.3 ns | 39.9 ns | 59.3 ns | 657 ns | 627 ns |
| D76 | 17.1 ns | 57.7 ns | 379 ns | 448 ns | 1.04 µs |
| D115 | 19.8 ns | 73.4 ns | 620 ns | 845 ns | 890 ns |
| D153 | 25.4 ns | 370 ns | 1.07 µs | 1.46 µs | 2.12 µs |
| D230 | 28.4 ns | 715 ns | 1.35 µs | 2.28 µs | 2.04 µs |
| D307 | 43.4 ns | 1.06 µs | 2.12 µs | 3.28 µs | 5.47 µs |
| D462 | 63.4 ns | 1.53 µs | 3.6 µs | 6.27 µs | 9.06 µs |
| D616 | 68.6 ns | 2.38 µs | 6.16 µs | 8.72 µs | 15.9 µs |
| D924 | 102 ns | 3.75 µs | 10.9 µs | 23 µs | 28.3 µs |
| D1232 | 100 ns | 6.18 µs | 16.5 µs | 27.6 µs | 37.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.1 88.2,203.1 124.4,199.4 160.5,198.4 196.7,195.1 232.9,189.8 269.1,187.3 305.3,178.1 341.5,169.9 377.6,168.2 413.8,159.6 450.0,160.0 450.0,31.0 413.8,37.4 377.6,50.0 341.5,62.1 305.3,73.1 269.1,94.5 232.9,93.7 196.7,112.5 160.5,109.2 124.4,120.1 88.2,164.2 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.1 88.2,203.1 124.4,199.4 160.5,198.4 196.7,195.1 232.9,189.8 269.1,187.3 305.3,178.1 341.5,169.9 377.6,168.2 413.8,159.6 450.0,160.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.2 88.2,179.3 124.4,180.0 160.5,171.9 196.7,166.7 232.9,131.6 269.1,117.3 305.3,108.7 341.5,100.8 377.6,91.1 413.8,81.3 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,168.7 124.4,171.4 160.5,131.1 196.7,120.4 232.9,108.5 269.1,103.4 305.3,93.7 341.5,82.2 377.6,70.5 413.8,58.1 450.0,49.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,185.1 88.2,168.7 124.4,119.1 160.5,127.4 196.7,113.7 232.9,101.8 269.1,92.1 305.3,84.2 341.5,70.1 377.6,63.0 413.8,41.9 450.0,38.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,164.2 124.4,120.1 160.5,109.2 196.7,112.5 232.9,93.7 269.1,94.5 305.3,73.1 341.5,62.1 377.6,50.0 413.8,37.4 450.0,31.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.9 ns | 274 ns | 370 ns | 295 ns | 376 ns |
| D38 | 1.8 ns | 358 ns | 370 ns | 374 ns | 314 ns |
| D57 | 282 ns | 468 ns | 402 ns | 521 ns | 676 ns |
| D76 | 288 ns | 350 ns | 353 ns | 471 ns | 637 ns |
| D115 | 311 ns | 489 ns | 711 ns | 906 ns | 819 ns |
| D153 | 303 ns | 363 ns | 634 ns | 1.04 µs | 1.3 µs |
| D230 | 558 ns | 686 ns | 1.13 µs | 1.47 µs | 1.23 µs |
| D307 | 662 ns | 689 ns | 1.11 µs | 1.36 µs | 10.6 µs |
| D462 | 1.3 µs | 2.79 µs | 3.67 µs | 4.01 µs | 5.47 µs |
| D616 | 1.39 µs | 1.56 µs | 2.08 µs | 2.52 µs | 3.97 µs |
| D924 | 2.1 µs | 2.08 µs | 2.94 µs | 3.6 µs | 4.77 µs |
| D1232 | 3.1 µs | 3.18 µs | 3.25 µs | 5.11 µs | 5.36 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.4 88.2,199.8 124.4,112.0 160.5,111.6 196.7,110.3 232.9,110.7 269.1,100.1 305.3,97.2 341.5,85.4 377.6,84.3 413.8,77.1 450.0,70.4 450.0,60.8 413.8,62.9 377.6,66.0 341.5,60.5 305.3,49.1 269.1,86.4 232.9,85.4 196.7,93.5 160.5,97.8 124.4,96.8 88.2,110.1 52.0,107.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.4 88.2,199.8 124.4,112.0 160.5,111.6 196.7,110.3 232.9,110.7 269.1,100.1 305.3,97.2 341.5,85.4 377.6,84.3 413.8,77.1 450.0,70.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.5 88.2,107.8 124.4,103.2 160.5,108.2 196.7,102.4 232.9,107.6 269.1,96.5 305.3,96.5 341.5,72.2 377.6,82.3 413.8,77.3 450.0,69.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.3 88.2,107.3 124.4,105.8 160.5,108.1 196.7,95.9 232.9,97.9 269.1,87.8 305.3,88.2 341.5,67.4 377.6,77.3 413.8,71.3 450.0,69.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,111.2 88.2,107.1 124.4,101.3 160.5,103.1 196.7,91.7 232.9,89.3 269.1,83.3 305.3,84.6 341.5,65.9 377.6,74.0 413.8,67.7 450.0,61.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.0 88.2,110.1 124.4,96.8 160.5,97.8 196.7,93.5 232.9,85.4 269.1,86.4 305.3,49.1 341.5,60.5 377.6,66.0 413.8,62.9 450.0,60.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.17 µs | 5.64 µs | 11.6 µs | 7.3 µs | 13.9 µs |
| D38 | 5 µs | 11.7 µs | 14.6 µs | 17.2 µs | 16.4 µs |
| D57 | 4.48 µs | 4.12 µs | 3.8 µs | 4.87 µs | 5.23 µs |
| D76 | 4.42 µs | 3.02 µs | 3 µs | 3.67 µs | 4.9 µs |
| D115 | 8.42 µs | 8.68 µs | 9.46 µs | 8.85 µs | 7.61 µs |
| D153 | 7.9 µs | 6.06 µs | 9.36 µs | 10.3 µs | 11.1 µs |
| D230 | 11.4 µs | 12.5 µs | 15.2 µs | 16.5 µs | 11.2 µs |
| D307 | 15.6 µs | 18.1 µs | 21.3 µs | 24 µs | 26.1 µs |
| D462 | 16.4 µs | 20.5 µs | 25.6 µs | 27.5 µs | 33.9 µs |
| D616 | 27.1 µs | 44 µs | 56.8 µs | 54.2 µs | 80.4 µs |
| D924 | 43.1 µs | 68.7 µs | 96.6 µs | 125 µs | 150 µs |
| D1232 | 54.5 µs | 106 µs | 137 µs | 203 µs | 189 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,149.2 88.2,163.4 124.4,166.6 160.5,167.0 196.7,148.3 232.9,150.2 269.1,139.4 305.3,130.5 341.5,129.0 377.6,114.4 413.8,101.1 450.0,94.2 450.0,58.2 413.8,64.9 377.6,83.0 341.5,108.0 305.3,115.5 269.1,140.0 232.9,140.3 196.7,151.2 160.5,164.0 124.4,162.1 88.2,129.0 52.0,133.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,149.2 88.2,163.4 124.4,166.6 160.5,167.0 196.7,148.3 232.9,150.2 269.1,139.4 305.3,130.5 341.5,129.0 377.6,114.4 413.8,101.1 450.0,94.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,159.9 88.2,138.9 124.4,169.0 160.5,178.0 196.7,147.4 232.9,157.9 269.1,136.9 305.3,126.2 341.5,122.6 377.6,100.4 413.8,87.5 450.0,75.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,139.0 88.2,132.4 124.4,171.3 160.5,178.2 196.7,144.9 232.9,145.3 269.1,131.3 305.3,121.4 341.5,116.1 377.6,93.0 413.8,77.7 450.0,67.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,152.5 88.2,127.6 124.4,164.1 160.5,172.4 196.7,146.9 232.9,142.5 269.1,128.9 305.3,118.0 341.5,114.0 377.6,94.4 413.8,70.3 450.0,56.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.8 88.2,129.0 124.4,162.1 160.5,164.0 196.7,151.2 232.9,140.3 269.1,140.0 305.3,115.5 341.5,108.0 377.6,83.0 413.8,64.9 450.0,58.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 9.14 ns | 3.46 µs | 6.9 µs | 4.31 µs | 7.93 µs |
| D38 | 5.73 ns | 6.92 µs | 8.27 µs | 9.43 µs | 8.9 µs |
| D57 | 63.5 ns | 4.14 µs | 3.81 µs | 5.16 µs | 5.66 µs |
| D76 | 85.7 ns | 3.02 µs | 3.3 µs | 4.18 µs | 5.59 µs |
| D115 | 148 ns | 8.31 µs | 9.65 µs | 9.19 µs | 8.24 µs |
| D153 | 175 ns | 5.99 µs | 9.8 µs | 11.1 µs | 12.1 µs |
| D230 | 343 ns | 12.7 µs | 16.6 µs | 20 µs | 14 µs |
| D307 | 365 ns | 19.3 µs | 21.5 µs | 28.3 µs | 32.1 µs |
| D462 | 703 ns | 70.1 µs | 149 µs | 206 µs | 283 µs |
| D616 | 749 ns | 194 µs | 354 µs | 298 µs | 569 µs |
| D924 | 1e+03 ns | 425 µs | 460 µs | 800 µs | 1.64 ms |
| D1232 | 1.38 µs | 762 µs | 688 µs | 2.13 ms | 2.12 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,182.5 88.2,188.3 124.4,158.5 160.5,154.8 196.7,148.0 232.9,145.9 269.1,137.6 305.3,136.8 341.5,128.7 377.6,127.9 413.8,124.3 450.0,120.3 450.0,29.2 413.8,32.5 377.6,45.6 341.5,54.2 305.3,81.2 269.1,91.6 232.9,93.3 196.7,98.1 160.5,102.9 124.4,102.8 88.2,97.2 52.0,98.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,182.5 88.2,188.3 124.4,158.5 160.5,154.8 196.7,148.0 232.9,145.9 269.1,137.6 305.3,136.8 341.5,128.7 377.6,127.9 413.8,124.3 450.0,120.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,108.9 88.2,100.3 124.4,106.7 160.5,110.6 196.7,98.0 232.9,102.1 269.1,92.8 305.3,87.6 341.5,71.5 377.6,59.0 413.8,49.2 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,98.1 124.4,107.7 160.5,109.5 196.7,96.2 232.9,96.0 269.1,89.5 305.3,86.2 341.5,62.2 377.6,51.5 413.8,48.2 450.0,43.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.2 88.2,96.4 124.4,103.9 160.5,106.5 196.7,96.8 232.9,94.4 269.1,87.1 305.3,82.8 341.5,58.2 377.6,53.6 413.8,41.3 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.6 88.2,97.2 124.4,102.8 160.5,102.9 196.7,98.1 232.9,93.3 269.1,91.6 305.3,81.2 341.5,54.2 377.6,45.6 413.8,32.5 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.18 ns | 14 ns | 18.4 ns | 24.8 ns | 28.9 ns |
| D38 | 5.73 ns | 19.1 ns | 30.2 ns | 1.48 µs | 2.48 µs |
| D57 | 176 ns | 207 ns | 347 ns | 668 ns | 674 ns |
| D76 | 209 ns | 181 ns | 433 ns | 580 ns | 1.21 µs |
| D115 | 125 ns | 654 ns | 923 ns | 1.16 µs | 1.22 µs |
| D153 | 118 ns | 631 ns | 1.63 µs | 1.91 µs | 2.72 µs |
| D230 | 158 ns | 1.54 µs | 2.35 µs | 3.53 µs | 2.65 µs |
| D307 | 145 ns | 2.33 µs | 3.7 µs | 4.8 µs | 7.05 µs |
| D462 | 204 ns | 3.78 µs | 5.95 µs | 9.5 µs | 11.6 µs |
| D616 | 228 ns | 6.44 µs | 11.2 µs | 11 µs | 21 µs |
| D924 | 238 ns | 11.8 µs | 17.1 µs | 26.4 µs | 35 µs |
| D1232 | 274 ns | 15.3 µs | 22.9 µs | 39.6 µs | 44.5 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.4 88.2,179.7 124.4,120.2 160.5,117.2 196.7,126.1 232.9,127.2 269.1,122.0 305.3,123.5 341.5,117.6 377.6,115.7 413.8,114.9 450.0,112.5 450.0,24.1 413.8,28.2 377.6,37.1 341.5,47.5 305.3,56.1 269.1,73.1 232.9,72.6 196.7,86.5 160.5,86.6 124.4,96.8 88.2,74.3 52.0,151.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.4 88.2,179.7 124.4,120.2 160.5,117.2 196.7,126.1 232.9,127.2 269.1,122.0 305.3,123.5 341.5,117.6 377.6,115.7 413.8,114.9 450.0,112.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.2 88.2,158.8 124.4,117.4 160.5,119.7 196.7,97.4 232.9,98.0 269.1,82.5 305.3,75.3 341.5,66.9 377.6,57.7 413.8,47.2 450.0,42.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.4 88.2,150.8 124.4,108.4 160.5,104.5 196.7,91.4 232.9,81.5 269.1,75.2 305.3,67.3 341.5,59.0 377.6,48.0 413.8,40.7 450.0,35.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,154.2 88.2,83.2 124.4,97.0 160.5,99.5 196.7,87.4 232.9,78.8 269.1,68.1 305.3,62.7 341.5,50.9 377.6,48.4 413.8,33.1 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,74.3 124.4,96.8 160.5,86.6 196.7,86.5 232.9,72.6 269.1,73.1 305.3,56.1 341.5,47.5 377.6,37.1 413.8,28.2 450.0,24.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
