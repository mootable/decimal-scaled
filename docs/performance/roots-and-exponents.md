# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 65.4 ns | 130 ns | 158 ns | 252 ns | 201 ns |
| D38 | 79.9 ns | 157 ns | 262 ns | 376 ns | 323 ns |
| D57 | 190 ns | 459 ns | 714 ns | 1.05 µs | 596 ns |
| D76 | 198 ns | 490 ns | 803 ns | 1.27 µs | 1.07 µs |
| D115 | 215 ns | 766 ns | 1.37 µs | 2.09 µs | 3.16 µs |
| D153 | 258 ns | 858 ns | 1.78 µs | 2.14 µs | 4.52 µs |
| D230 | 294 ns | 1.46 µs | 2.52 µs | 3.51 µs | 9.38 µs |
| D307 | 303 ns | 1.85 µs | 4.72 µs | 8.87 µs | 15 µs |
| D462 | 379 ns | 3.22 µs | 6.89 µs | 18.6 µs | 19.8 µs |
| D616 | 462 ns | 4.54 µs | 12.2 µs | 30.8 µs | 50.6 µs |
| D924 | 357 ns | 7.6 µs | 30.9 µs | 70.2 µs | 130 µs |
| D1232 | 695 ns | 11.3 µs | 57.3 µs | 119 µs | 159 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.4 88.2,173.9 124.4,158.9 160.5,158.2 196.7,156.7 232.9,153.5 269.1,151.3 305.3,150.8 341.5,146.9 377.6,143.4 413.8,147.9 450.0,136.3 450.0,41.9 413.8,45.4 377.6,61.8 341.5,78.1 305.3,83.0 269.1,91.1 232.9,103.8 196.7,110.0 160.5,128.8 124.4,139.0 88.2,149.6 52.0,157.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.4 88.2,173.9 124.4,158.9 160.5,158.2 196.7,156.7 232.9,153.5 269.1,151.3 305.3,150.8 341.5,146.9 377.6,143.4 413.8,147.9 450.0,136.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,165.5 88.2,162.2 124.4,143.5 160.5,142.4 196.7,134.6 232.9,132.7 269.1,123.4 305.3,119.3 341.5,109.7 377.6,103.7 413.8,94.8 450.0,87.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.1 88.2,153.2 124.4,135.8 160.5,133.8 196.7,124.5 232.9,120.0 269.1,113.9 305.3,103.1 341.5,96.5 377.6,86.6 413.8,70.4 450.0,59.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.9 88.2,147.0 124.4,129.1 160.5,125.8 196.7,117.2 232.9,116.8 269.1,108.2 305.3,92.1 341.5,79.2 377.6,70.5 413.8,56.1 450.0,46.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,157.9 88.2,149.6 124.4,139.0 160.5,128.8 196.7,110.0 232.9,103.8 269.1,91.1 305.3,83.0 341.5,78.1 377.6,61.8 413.8,45.4 450.0,41.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.52 ns | 1.63 µs | 3.23 µs | 3.46 µs | 2.68 µs |
| D38 | 1.59 ns | 3.23 µs | 4.16 µs | 4.31 µs | 3.54 µs |
| D57 | 2.18 ns | 3.86 µs | 4.57 µs | 6.07 µs | 5.51 µs |
| D76 | 3.52 ns | 6.38 µs | 7.61 µs | 9.36 µs | 6.74 µs |
| D115 | 10.3 ns | 6.77 µs | 13.3 µs | 16.5 µs | 23.8 µs |
| D153 | 13.2 ns | 6.72 µs | 15.2 µs | 14.2 µs | 38.1 µs |
| D230 | 45.8 ns | 13.7 µs | 17 µs | 27.3 µs | 83.4 µs |
| D307 | 68.3 ns | 15.4 µs | 38.2 µs | 79 µs | 123 µs |
| D462 | 105 ns | 22.8 µs | 58.7 µs | 166 µs | 152 µs |
| D616 | 133 ns | 35.4 µs | 112 µs | 269 µs | 413 µs |
| D924 | 117 ns | 68.3 µs | 269 µs | 605 µs | 992 µs |
| D1232 | 237 ns | 95.9 µs | 444 µs | 916 µs | 2.54 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.5 88.2,204.2 124.4,200.3 160.5,194.4 196.7,181.1 232.9,178.0 269.1,162.5 305.3,157.6 341.5,152.2 377.6,149.3 413.8,150.9 450.0,142.1 450.0,27.0 413.8,38.7 377.6,49.5 341.5,62.0 305.3,64.5 269.1,69.4 232.9,79.1 196.7,85.0 160.5,100.6 124.4,103.1 88.2,108.6 52.0,112.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.5 88.2,204.2 124.4,200.3 160.5,194.4 196.7,181.1 232.9,178.0 269.1,162.5 305.3,157.6 341.5,152.2 377.6,149.3 413.8,150.9 450.0,142.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,118.3 88.2,109.7 124.4,107.5 160.5,101.3 196.7,100.6 232.9,100.6 269.1,91.8 305.3,90.4 341.5,85.5 377.6,80.0 413.8,71.9 450.0,67.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,106.6 124.4,105.4 160.5,99.1 196.7,92.2 232.9,90.5 269.1,89.1 305.3,79.1 341.5,73.8 377.6,65.7 413.8,54.9 450.0,48.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,108.9 88.2,106.2 124.4,101.9 160.5,96.5 196.7,89.5 232.9,91.3 269.1,83.2 305.3,70.1 341.5,60.9 377.6,54.9 413.8,44.8 450.0,39.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,112.0 88.2,108.6 124.4,103.1 160.5,100.6 196.7,85.0 232.9,79.1 269.1,69.4 305.3,64.5 341.5,62.0 377.6,49.5 413.8,38.7 450.0,27.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 12.9 ns | 40.5 ns | 41.3 ns | 40.8 ns | 32.2 ns |
| D38 | 15.5 ns | 41.5 ns | 74.4 ns | 68 ns | 82.2 ns |
| D57 | 17.1 ns | 41.1 ns | 83.3 ns | 402 ns | 274 ns |
| D76 | 17.2 ns | 95.9 ns | 397 ns | 380 ns | 415 ns |
| D115 | 15.6 ns | 84.2 ns | 390 ns | 786 ns | 1.01 µs |
| D153 | 15.7 ns | 385 ns | 662 ns | 661 ns | 1.75 µs |
| D230 | 28.9 ns | 451 ns | 826 ns | 1.26 µs | 2.95 µs |
| D307 | 44.1 ns | 691 ns | 1.76 µs | 2.83 µs | 5.3 µs |
| D462 | 62 ns | 1.15 µs | 2.58 µs | 6.09 µs | 5.29 µs |
| D616 | 68.1 ns | 1.97 µs | 4.65 µs | 10.3 µs | 12.9 µs |
| D924 | 71 ns | 2.71 µs | 10.4 µs | 24.6 µs | 26.9 µs |
| D1232 | 82.8 ns | 4.5 µs | 20.5 µs | 22.8 µs | 44.1 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,204.5 88.2,200.5 124.4,198.3 160.5,198.2 196.7,200.3 232.9,200.2 269.1,187.0 305.3,177.8 341.5,170.4 377.6,168.3 413.8,167.4 450.0,164.1 450.0,27.8 413.8,38.5 377.6,54.4 341.5,73.8 305.3,73.8 269.1,86.5 232.9,97.8 196.7,109.7 160.5,129.1 124.4,138.1 88.2,164.2 52.0,184.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,204.5 88.2,200.5 124.4,198.3 160.5,198.2 196.7,200.3 232.9,200.2 269.1,187.0 305.3,177.8 341.5,170.4 377.6,168.3 413.8,167.4 450.0,164.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,179.6 88.2,179.1 124.4,179.3 160.5,160.9 196.7,163.7 232.9,130.8 269.1,127.3 305.3,118.0 341.5,106.9 377.6,95.3 413.8,88.4 450.0,77.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.2 88.2,166.4 124.4,164.0 160.5,130.0 196.7,130.4 232.9,119.0 269.1,114.2 305.3,97.8 341.5,89.4 377.6,76.6 413.8,59.2 450.0,44.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.5 88.2,168.4 124.4,129.8 160.5,131.0 196.7,115.2 232.9,119.0 269.1,104.9 305.3,87.4 341.5,70.8 377.6,59.4 413.8,40.4 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,184.6 88.2,164.2 124.4,138.1 160.5,129.1 196.7,109.7 232.9,97.8 269.1,86.5 305.3,73.8 341.5,73.8 377.6,54.4 413.8,38.5 450.0,27.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.09 ns | 271 ns | 380 ns | 378 ns | 316 ns |
| D38 | 2.81 ns | 371 ns | 409 ns | 381 ns | 324 ns |
| D57 | 271 ns | 478 ns | 479 ns | 519 ns | 341 ns |
| D76 | 285 ns | 502 ns | 512 ns | 614 ns | 347 ns |
| D115 | 241 ns | 505 ns | 611 ns | 845 ns | 1.08 µs |
| D153 | 229 ns | 476 ns | 666 ns | 695 ns | 1.39 µs |
| D230 | 562 ns | 730 ns | 845 ns | 940 ns | 1.91 µs |
| D307 | 662 ns | 695 ns | 1.19 µs | 1.34 µs | 11.3 µs |
| D462 | 1.21 µs | 2.85 µs | 2.6 µs | 4.31 µs | 3.25 µs |
| D616 | 1.38 µs | 1.43 µs | 1.78 µs | 2.89 µs | 3.66 µs |
| D924 | 1.38 µs | 1.89 µs | 2.96 µs | 3.97 µs | 5.16 µs |
| D1232 | 2.3 µs | 2.14 µs | 4.5 µs | 5.26 µs | 6.14 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,190.4 88.2,192.0 124.4,112.7 160.5,111.8 196.7,114.7 232.9,115.6 269.1,100.0 305.3,97.2 341.5,86.7 377.6,84.4 413.8,84.4 450.0,75.5 450.0,58.5 413.8,61.5 377.6,67.5 341.5,69.5 305.3,47.8 269.1,78.7 232.9,84.2 196.7,88.6 160.5,108.4 124.4,108.7 88.2,109.6 52.0,110.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,190.4 88.2,192.0 124.4,112.7 160.5,111.8 196.7,114.7 232.9,115.6 269.1,100.0 305.3,97.2 341.5,86.7 377.6,84.4 413.8,84.4 450.0,75.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.7 88.2,107.2 124.4,102.8 160.5,102.0 196.7,101.9 232.9,102.9 269.1,95.5 305.3,96.3 341.5,71.8 377.6,83.8 413.8,78.9 450.0,76.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.8 88.2,105.5 124.4,102.8 160.5,101.6 196.7,98.6 232.9,97.1 269.1,92.9 305.3,87.0 341.5,73.4 377.6,80.0 413.8,71.2 450.0,63.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,106.8 124.4,101.4 160.5,98.5 196.7,92.9 232.9,96.3 269.1,91.1 305.3,84.9 341.5,64.6 377.6,71.6 413.8,66.1 450.0,61.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,110.0 88.2,109.6 124.4,108.7 160.5,108.4 196.7,88.6 232.9,84.2 269.1,78.7 305.3,47.8 341.5,69.5 377.6,67.5 413.8,61.5 450.0,58.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 2.22 µs | 3.09 µs | 5.82 µs | 6.34 µs | 5.25 µs |
| D38 | 2.9 µs | 5.83 µs | 7.64 µs | 8.59 µs | 7.71 µs |
| D57 | 2.57 µs | 2.61 µs | 2.74 µs | 2.97 µs | 1.55 µs |
| D76 | 2.78 µs | 2.89 µs | 2.96 µs | 2.9 µs | 1.59 µs |
| D115 | 4.15 µs | 4.86 µs | 5.1 µs | 4.98 µs | 6.14 µs |
| D153 | 3.47 µs | 4.5 µs | 5.32 µs | 3.67 µs | 6.84 µs |
| D230 | 6.46 µs | 7.57 µs | 6.01 µs | 5.43 µs | 10.6 µs |
| D307 | 9.66 µs | 11.1 µs | 13.6 µs | 14.3 µs | 16.7 µs |
| D462 | 9.79 µs | 12.5 µs | 10.3 µs | 17.7 µs | 11.6 µs |
| D616 | 16.1 µs | 23.9 µs | 25.8 µs | 35.4 µs | 41.4 µs |
| D924 | 15.5 µs | 34.2 µs | 55.3 µs | 77.8 µs | 92.4 µs |
| D1232 | 28.5 µs | 47.9 µs | 96.7 µs | 115 µs | 132 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,186.9 88.2,179.2 124.4,182.6 160.5,180.4 196.7,168.8 232.9,173.9 269.1,156.0 305.3,144.3 341.5,143.9 377.6,129.5 413.8,130.7 450.0,113.0 450.0,68.6 413.8,79.0 377.6,102.2 341.5,139.1 305.3,128.5 269.1,141.6 232.9,154.3 196.7,157.5 160.5,196.6 124.4,197.2 88.2,150.9 52.0,162.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,186.9 88.2,179.2 124.4,182.6 160.5,180.4 196.7,168.8 232.9,173.9 269.1,156.0 305.3,144.3 341.5,143.9 377.6,129.5 413.8,130.7 450.0,113.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,177.3 88.2,158.9 124.4,182.3 160.5,179.3 196.7,164.2 232.9,166.4 269.1,151.4 305.3,140.3 341.5,136.8 377.6,118.1 413.8,107.7 450.0,97.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.0 88.2,151.1 124.4,180.8 160.5,178.6 196.7,162.8 232.9,161.6 269.1,158.1 305.3,134.5 341.5,142.5 377.6,115.9 413.8,93.8 450.0,77.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,156.5 88.2,147.7 124.4,178.5 160.5,179.2 196.7,163.5 232.9,172.4 269.1,161.0 305.3,133.0 341.5,126.7 377.6,106.7 413.8,84.0 450.0,72.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,162.0 88.2,150.9 124.4,197.2 160.5,196.6 196.7,157.5 232.9,154.3 269.1,141.6 305.3,128.5 341.5,139.1 377.6,102.2 413.8,79.0 450.0,68.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7 ns | 2.04 µs | 3.88 µs | 4.07 µs | 3.25 µs |
| D38 | 6.48 ns | 3.88 µs | 4.83 µs | 4.93 µs | 3.99 µs |
| D57 | 56.5 ns | 4.2 µs | 4.58 µs | 5.13 µs | 2.9 µs |
| D76 | 83.4 ns | 4.49 µs | 5.12 µs | 5.18 µs | 3.21 µs |
| D115 | 127 ns | 8.31 µs | 8.98 µs | 9.38 µs | 11.6 µs |
| D153 | 129 ns | 7.87 µs | 9.6 µs | 7.43 µs | 13.1 µs |
| D230 | 340 ns | 13.6 µs | 11.4 µs | 11.7 µs | 23 µs |
| D307 | 359 ns | 19.5 µs | 23.4 µs | 28.6 µs | 34.9 µs |
| D462 | 630 ns | 70.9 µs | 102 µs | 224 µs | 199 µs |
| D616 | 778 ns | 178 µs | 277 µs | 357 µs | 533 µs |
| D924 | 699 ns | 354 µs | 462 µs | 857 µs | 1.75 ms |
| D1232 | 1.09 µs | 596 µs | 862 µs | 2.12 ms | 2.5 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,185.9 88.2,186.8 124.4,160.0 160.5,155.1 196.7,149.9 232.9,149.7 269.1,137.7 305.3,137.0 341.5,130.0 377.6,127.4 413.8,128.7 450.0,123.3 450.0,27.2 413.8,31.6 377.6,46.4 341.5,58.6 305.3,80.2 269.1,85.4 232.9,92.3 196.7,93.9 160.5,109.8 124.4,111.1 88.2,107.1 52.0,109.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,185.9 88.2,186.8 124.4,160.0 160.5,155.1 196.7,149.9 232.9,149.7 269.1,137.7 305.3,137.0 341.5,130.0 377.6,127.4 413.8,128.7 450.0,123.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,115.4 88.2,107.5 124.4,106.5 160.5,105.7 196.7,98.0 232.9,98.7 269.1,91.9 305.3,87.4 341.5,71.4 377.6,60.0 413.8,51.4 450.0,45.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,107.5 88.2,104.8 124.4,105.4 160.5,104.0 196.7,97.0 232.9,96.2 269.1,94.1 305.3,85.2 341.5,66.9 377.6,54.5 413.8,48.2 450.0,40.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.9 88.2,104.5 124.4,104.0 160.5,103.9 196.7,96.5 232.9,99.4 269.1,93.8 305.3,82.7 341.5,57.1 377.6,51.3 413.8,40.5 450.0,29.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,109.7 88.2,107.1 124.4,111.1 160.5,109.8 196.7,93.9 232.9,92.3 269.1,85.4 305.3,80.2 341.5,58.6 377.6,46.4 413.8,31.6 450.0,27.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.51 ns | 13.4 ns | 18.2 ns | 29.1 ns | 25.4 ns |
| D38 | 7.29 ns | 19.3 ns | 44.4 ns | 135 ns | 154 ns |
| D57 | 81.4 ns | 140 ns | 205 ns | 388 ns | 248 ns |
| D76 | 83.7 ns | 144 ns | 309 ns | 371 ns | 395 ns |
| D115 | 92.8 ns | 223 ns | 386 ns | 736 ns | 984 ns |
| D153 | 91.2 ns | 303 ns | 637 ns | 627 ns | 1.65 µs |
| D230 | 148 ns | 459 ns | 746 ns | 1.21 µs | 2.86 µs |
| D307 | 153 ns | 679 ns | 1.7 µs | 2.75 µs | 5.17 µs |
| D462 | 198 ns | 1.04 µs | 2.13 µs | 6.26 µs | 5.06 µs |
| D616 | 236 ns | 1.84 µs | 3.92 µs | 6.85 µs | 12.5 µs |
| D924 | 174 ns | 2.24 µs | 6.82 µs | 16.3 µs | 26.2 µs |
| D1232 | 290 ns | 3.81 µs | 13.8 µs | 23.5 µs | 40.2 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.8 88.2,175.5 124.4,133.6 160.5,133.1 196.7,131.3 232.9,131.6 269.1,123.1 305.3,122.6 341.5,118.1 377.6,115.1 413.8,120.4 450.0,111.5 450.0,25.8 413.8,33.3 377.6,46.2 341.5,61.8 305.3,61.4 269.1,71.7 232.9,81.3 196.7,90.3 160.5,106.1 124.4,114.2 88.2,122.5 52.0,153.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.8 88.2,175.5 124.4,133.6 160.5,133.1 196.7,131.3 232.9,131.6 269.1,123.1 305.3,122.6 341.5,118.1 377.6,115.1 413.8,120.4 450.0,111.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,164.9 88.2,158.6 124.4,124.2 160.5,123.7 196.7,116.0 232.9,110.7 269.1,103.5 305.3,96.7 341.5,89.3 377.6,79.4 413.8,76.0 450.0,66.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.5 88.2,144.1 124.4,117.5 160.5,110.4 196.7,106.5 232.9,97.8 269.1,95.1 305.3,80.8 341.5,76.9 377.6,66.3 413.8,56.7 450.0,44.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.5 88.2,124.8 124.4,106.4 160.5,107.2 196.7,95.3 232.9,98.1 269.1,86.6 305.3,72.4 341.5,58.1 377.6,56.6 413.8,41.5 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,153.8 88.2,122.5 124.4,114.2 160.5,106.1 196.7,90.3 232.9,81.3 269.1,71.7 305.3,61.4 341.5,61.8 377.6,46.2 413.8,33.3 450.0,25.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
