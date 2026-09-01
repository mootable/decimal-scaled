# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.08 µs | 5.14 µs | 5.8 µs | 8.7 µs | 8.7 µs |
| D38 | 5.08 µs | 2.25 µs | 7.71 µs | 9.67 µs | 10.9 µs |
| D57 | 310 ns | 574 ns | 489 ns | 803 ns | 1.16 µs |
| D76 | 536 ns | 432 ns | 866 ns | 1.37 µs | 1.48 µs |
| D115 | 512 ns | 1.35 µs | 2.42 µs | 3.74 µs | 5.29 µs |
| D153 | 524 ns | 2.63 µs | 2.65 µs | 5.75 µs | 6.83 µs |
| D230 | 549 ns | 4.09 µs | 8.46 µs | 9.25 µs | 14.9 µs |
| D307 | 418 ns | 4.97 µs | 11.7 µs | 17.9 µs | 22.7 µs |
| D462 | 635 ns | 10.4 µs | 26.1 µs | 32.9 µs | 52 µs |
| D616 | 473 ns | 15.3 µs | 37.7 µs | 58.1 µs | 85.3 µs |
| D924 | 584 ns | 25.8 µs | 82.7 µs | 121 µs | 128 µs |
| D1232 | 525 ns | 57.2 µs | 103 µs | 228 µs | 345 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,124.7 124.4,185.4 160.5,173.6 196.7,174.5 232.9,174.0 269.1,173.0 305.3,178.9 341.5,169.9 377.6,176.3 413.8,171.7 450.0,174.0 450.0,33.1 413.8,54.7 377.6,63.5 341.5,74.2 305.3,92.2 269.1,101.4 232.9,118.3 196.7,123.8 160.5,151.5 124.4,156.9 88.2,108.1 52.0,113.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,124.7 124.4,185.4 160.5,173.6 196.7,174.5 232.9,174.0 269.1,173.0 305.3,178.9 341.5,169.9 377.6,176.3 413.8,171.7 450.0,174.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,124.5 88.2,142.3 124.4,172.0 160.5,178.2 196.7,153.5 232.9,139.0 269.1,129.4 305.3,125.2 341.5,109.1 377.6,100.7 413.8,89.4 450.0,72.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,121.8 88.2,115.6 124.4,175.5 160.5,163.1 196.7,140.8 232.9,138.8 269.1,113.6 305.3,106.6 341.5,89.2 377.6,81.2 413.8,64.1 450.0,59.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,110.7 124.4,164.8 160.5,153.2 196.7,131.4 232.9,122.0 269.1,111.7 305.3,97.4 341.5,84.2 377.6,71.8 413.8,55.8 450.0,42.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,108.1 124.4,156.9 160.5,151.5 196.7,123.8 232.9,118.3 269.1,101.4 305.3,92.2 341.5,74.2 377.6,63.5 413.8,54.7 450.0,33.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.11 ns | 4.42 µs | 6.79 µs | 8.54 µs | 7.88 µs |
| D38 | 1.56 ns | 3.54 µs | 7.57 µs | 8.72 µs | 10.8 µs |
| D57 | 2.3 ns | 3.31 µs | 3.11 µs | 4.48 µs | 8.75 µs |
| D76 | 3.22 ns | 3.27 µs | 5.63 µs | 8.85 µs | 9.28 µs |
| D115 | 17.4 ns | 3.62 µs | 12.6 µs | 18.4 µs | 23.4 µs |
| D153 | 22.5 ns | 7.28 µs | 11.7 µs | 21.5 µs | 34.6 µs |
| D230 | 51.3 ns | 13.4 µs | 23.7 µs | 37.6 µs | 82.4 µs |
| D307 | 105 ns | 13.3 µs | 37.5 µs | 83.5 µs | 121 µs |
| D462 | 135 ns | 23.2 µs | 84.5 µs | 150 µs | 248 µs |
| D616 | 202 ns | 34.9 µs | 130 µs | 266 µs | 441 µs |
| D924 | 201 ns | 73.7 µs | 264 µs | 548 µs | 683 µs |
| D1232 | 253 ns | 129 µs | 306 µs | 883 µs | 2.84 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,195.9 88.2,204.5 124.4,199.7 160.5,195.5 196.7,174.5 232.9,171.4 269.1,161.1 305.3,152.2 341.5,149.2 377.6,144.1 413.8,144.2 450.0,141.3 450.0,25.6 413.8,43.3 377.6,48.7 341.5,55.9 305.3,64.8 269.1,69.5 232.9,80.3 196.7,85.2 160.5,96.6 124.4,97.4 88.2,94.8 52.0,98.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,195.9 88.2,204.5 124.4,199.7 160.5,195.5 196.7,174.5 232.9,171.4 269.1,161.1 305.3,152.2 341.5,149.2 377.6,144.1 413.8,144.2 450.0,141.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,105.9 88.2,108.6 124.4,109.4 160.5,109.6 196.7,108.3 232.9,99.6 269.1,92.1 305.3,92.2 341.5,85.3 377.6,80.2 413.8,70.9 450.0,64.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.5 88.2,99.2 124.4,110.2 160.5,102.8 196.7,92.9 232.9,93.7 269.1,85.0 305.3,79.3 341.5,69.2 377.6,63.9 413.8,55.1 450.0,53.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.7 88.2,97.4 124.4,105.7 160.5,97.2 196.7,88.1 232.9,86.2 269.1,79.3 305.3,69.4 341.5,62.1 377.6,55.0 413.8,46.0 450.0,40.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.7 88.2,94.8 124.4,97.4 160.5,96.6 196.7,85.2 232.9,80.3 269.1,69.5 305.3,64.8 341.5,55.9 377.6,48.7 413.8,43.3 450.0,25.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.7 ns | 39.5 ns | 42.9 ns | 40.3 ns | 40.1 ns |
| D38 | 15.5 ns | 32.1 ns | 67.2 ns | 66.9 ns | 108 ns |
| D57 | 15.3 ns | 37.2 ns | 61.4 ns | 485 ns | 718 ns |
| D76 | 17.1 ns | 50 ns | 480 ns | 709 ns | 709 ns |
| D115 | 21.3 ns | 47.2 ns | 702 ns | 1.12 µs | 1.27 µs |
| D153 | 25.9 ns | 608 ns | 626 ns | 1.39 µs | 2.09 µs |
| D230 | 29.4 ns | 637 ns | 1.36 µs | 1.93 µs | 3.18 µs |
| D307 | 43.8 ns | 750 ns | 2.03 µs | 3.23 µs | 5.63 µs |
| D462 | 68.4 ns | 1.65 µs | 3.69 µs | 6.28 µs | 9.06 µs |
| D616 | 65 ns | 2.43 µs | 6.01 µs | 10.8 µs | 15.4 µs |
| D924 | 109 ns | 3.42 µs | 11 µs | 23.2 µs | 21.4 µs |
| D1232 | 78.2 ns | 6.08 µs | 14.5 µs | 27.1 µs | 50.9 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.8 88.2,200.5 124.4,200.8 160.5,198.4 196.7,193.6 232.9,189.3 269.1,186.6 305.3,178.0 341.5,168.2 377.6,169.4 413.8,158.0 450.0,165.3 450.0,24.7 413.8,43.5 377.6,50.7 341.5,62.1 305.3,72.5 269.1,84.9 232.9,94.0 196.7,104.9 160.5,117.5 124.4,117.2 88.2,158.4 52.0,179.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.8 88.2,200.5 124.4,200.8 160.5,198.4 196.7,193.6 232.9,189.3 269.1,186.6 305.3,178.0 341.5,168.2 377.6,169.4 413.8,158.0 450.0,165.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,180.1 88.2,184.7 124.4,181.5 160.5,175.1 196.7,176.3 232.9,120.8 269.1,119.8 305.3,116.2 341.5,99.1 377.6,90.7 413.8,83.3 450.0,70.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,178.4 88.2,168.6 124.4,170.6 160.5,126.0 196.7,117.7 232.9,120.2 269.1,103.3 305.3,94.6 341.5,81.6 377.6,71.1 413.8,57.9 450.0,51.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,168.7 124.4,125.7 160.5,117.5 196.7,107.6 232.9,102.9 269.1,95.7 305.3,84.5 341.5,70.1 377.6,58.3 413.8,41.7 450.0,38.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.9 88.2,158.4 124.4,117.2 160.5,117.5 196.7,104.9 232.9,94.0 269.1,84.9 305.3,72.5 341.5,62.1 377.6,50.7 413.8,43.5 450.0,24.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 273 ns | 387 ns | 397 ns | 401 ns |
| D38 | 2.18 ns | 286 ns | 372 ns | 373 ns | 404 ns |
| D57 | 246 ns | 423 ns | 338 ns | 379 ns | 561 ns |
| D76 | 276 ns | 275 ns | 372 ns | 551 ns | 504 ns |
| D115 | 285 ns | 282 ns | 558 ns | 1.04 µs | 1.05 µs |
| D153 | 300 ns | 535 ns | 390 ns | 956 ns | 1.2 µs |
| D230 | 491 ns | 718 ns | 1.11 µs | 1.2 µs | 1.8 µs |
| D307 | 766 ns | 610 ns | 1.14 µs | 1.44 µs | 10.4 µs |
| D462 | 1.13 µs | 3.07 µs | 3.68 µs | 3.74 µs | 5.18 µs |
| D616 | 1.34 µs | 1.34 µs | 1.66 µs | 2.63 µs | 3.59 µs |
| D924 | 2.02 µs | 1.53 µs | 2.75 µs | 3.38 µs | 3.06 µs |
| D1232 | 1.94 µs | 2.97 µs | 2.87 µs | 4.56 µs | 6.61 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,184.4 88.2,196.5 124.4,114.3 160.5,112.4 196.7,111.8 232.9,110.9 269.1,102.3 305.3,94.6 341.5,87.9 377.6,84.9 413.8,77.8 450.0,78.5 450.0,57.2 413.8,70.6 377.6,67.8 341.5,61.4 305.3,49.4 269.1,79.8 232.9,86.8 196.7,89.1 160.5,101.9 124.4,100.0 88.2,105.8 52.0,105.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,184.4 88.2,196.5 124.4,114.3 160.5,112.4 196.7,111.8 232.9,110.9 269.1,102.3 305.3,94.6 341.5,87.9 377.6,84.9 413.8,77.8 450.0,78.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,112.5 88.2,111.7 124.4,104.9 160.5,112.4 196.7,112.0 232.9,100.9 269.1,95.8 305.3,98.6 341.5,70.5 377.6,84.9 413.8,82.7 450.0,71.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.5 88.2,107.2 124.4,108.9 160.5,107.2 196.7,100.1 232.9,106.4 269.1,88.3 305.3,87.7 341.5,67.4 377.6,81.2 413.8,72.4 450.0,71.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,106.1 88.2,107.1 124.4,106.8 160.5,100.4 196.7,89.3 232.9,90.8 269.1,86.8 305.3,83.7 341.5,67.1 377.6,73.2 413.8,68.8 450.0,63.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,105.9 88.2,105.8 124.4,100.0 160.5,101.9 196.7,89.1 232.9,86.8 269.1,79.8 305.3,49.4 341.5,61.4 377.6,67.8 413.8,70.6 450.0,57.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.71 µs | 8.73 µs | 12.4 µs | 13.6 µs | 14.8 µs |
| D38 | 7.72 µs | 6.68 µs | 14.5 µs | 17.2 µs | 21.2 µs |
| D57 | 3.72 µs | 3.92 µs | 3.11 µs | 3.79 µs | 4.69 µs |
| D76 | 4.45 µs | 2.47 µs | 3.84 µs | 4.6 µs | 4.1 µs |
| D115 | 7.6 µs | 5.41 µs | 8.72 µs | 10.3 µs | 10.7 µs |
| D153 | 7.75 µs | 9.09 µs | 6.97 µs | 9.83 µs | 10.7 µs |
| D230 | 10.5 µs | 12.9 µs | 14.9 µs | 12.4 µs | 17.9 µs |
| D307 | 16.5 µs | 15.3 µs | 22.9 µs | 25.6 µs | 28.7 µs |
| D462 | 16.1 µs | 25.7 µs | 25.2 µs | 27.5 µs | 33.8 µs |
| D616 | 23.6 µs | 40.1 µs | 51.3 µs | 61.2 µs | 80.2 µs |
| D924 | 43.3 µs | 59.5 µs | 95.1 µs | 123 µs | 111 µs |
| D1232 | 40.3 µs | 106 µs | 119 µs | 202 µs | 271 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,150.8 124.4,172.0 160.5,166.8 196.7,151.3 232.9,150.7 269.1,142.0 305.3,128.9 341.5,129.5 377.6,118.4 413.8,100.9 450.0,103.0 450.0,47.8 413.8,73.6 377.6,83.0 341.5,108.1 305.3,112.8 269.1,126.4 232.9,141.5 196.7,141.5 160.5,169.2 124.4,165.3 88.2,121.6 52.0,131.9" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,150.8 124.4,172.0 160.5,166.8 196.7,151.3 232.9,150.7 269.1,142.0 305.3,128.9 341.5,129.5 377.6,118.4 413.8,100.9 450.0,103.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,147.3 88.2,155.0 124.4,170.5 160.5,183.9 196.7,161.1 232.9,146.1 269.1,136.0 305.3,130.9 341.5,116.0 377.6,103.1 413.8,91.7 450.0,74.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,137.1 88.2,132.5 124.4,177.2 160.5,171.1 196.7,147.3 232.9,153.8 269.1,131.7 305.3,119.4 341.5,116.6 377.6,96.0 413.8,78.1 450.0,71.7" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,127.7 124.4,171.4 160.5,165.8 196.7,142.4 232.9,143.8 269.1,137.1 305.3,116.2 341.5,114.0 377.6,90.9 413.8,70.6 450.0,56.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,131.9 88.2,121.6 124.4,165.3 160.5,169.2 196.7,141.5 232.9,141.5 269.1,126.4 305.3,112.8 341.5,108.1 377.6,83.0 413.8,73.6 450.0,47.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 4.86 µs | 7.46 µs | 8.01 µs | 8.6 µs |
| D38 | 6.54 ns | 4.05 µs | 8.23 µs | 9.37 µs | 11.5 µs |
| D57 | 52.1 ns | 3.89 µs | 3.25 µs | 3.84 µs | 4.97 µs |
| D76 | 72.8 ns | 2.31 µs | 3.79 µs | 4.98 µs | 4.52 µs |
| D115 | 138 ns | 5.22 µs | 8.93 µs | 10.9 µs | 11.5 µs |
| D153 | 176 ns | 9.08 µs | 7.4 µs | 10.9 µs | 11.9 µs |
| D230 | 286 ns | 13.2 µs | 16.1 µs | 15.5 µs | 22.4 µs |
| D307 | 442 ns | 15.7 µs | 22.9 µs | 30 µs | 35 µs |
| D462 | 612 ns | 90.2 µs | 147 µs | 201 µs | 282 µs |
| D616 | 711 ns | 176 µs | 324 µs | 355 µs | 572 µs |
| D924 | 993 ns | 420 µs | 455 µs | 782 µs | 1.23 ms |
| D1232 | 1 µs | 770 µs | 599 µs | 2.1 ms | 2.98 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.7 124.4,161.0 160.5,156.8 196.7,148.8 232.9,145.8 269.1,139.8 305.3,134.4 341.5,130.4 377.6,128.5 413.8,124.4 450.0,124.2 450.0,25.0 413.8,36.0 377.6,45.5 341.5,54.3 305.3,80.2 269.1,85.7 232.9,93.5 196.7,94.0 160.5,105.6 124.4,104.4 88.2,94.0 52.0,97.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.7 124.4,161.0 160.5,156.8 196.7,148.8 232.9,145.8 269.1,139.8 305.3,134.4 341.5,130.4 377.6,128.5 413.8,124.4 450.0,124.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,104.7 88.2,106.9 124.4,107.4 160.5,113.9 196.7,103.8 232.9,96.9 269.1,92.3 305.3,90.1 341.5,68.4 377.6,60.1 413.8,49.3 450.0,41.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.3 88.2,98.1 124.4,109.7 160.5,107.7 196.7,97.1 232.9,99.5 269.1,89.8 305.3,85.4 341.5,62.3 377.6,52.6 413.8,48.3 450.0,44.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,96.5 124.4,107.6 160.5,104.4 196.7,94.6 232.9,94.6 269.1,90.2 305.3,82.1 341.5,58.5 377.6,51.4 413.8,41.6 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,97.6 88.2,94.0 124.4,104.4 160.5,105.6 196.7,94.0 232.9,93.5 269.1,85.7 305.3,80.2 341.5,54.3 377.6,45.5 413.8,36.0 450.0,25.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.38 ns | 14.8 ns | 19.9 ns | 30.3 ns | 30.7 ns |
| D38 | 7.92 ns | 17.3 ns | 31.3 ns | 1.48 µs | 3.19 µs |
| D57 | 147 ns | 195 ns | 328 ns | 542 ns | 768 ns |
| D76 | 208 ns | 168 ns | 503 ns | 844 ns | 817 ns |
| D115 | 113 ns | 333 ns | 992 ns | 1.51 µs | 1.66 µs |
| D153 | 125 ns | 1.03 µs | 1.02 µs | 1.88 µs | 2.68 µs |
| D230 | 152 ns | 1.7 µs | 2.49 µs | 2.97 µs | 4.18 µs |
| D307 | 167 ns | 1.97 µs | 3.71 µs | 4.77 µs | 7.33 µs |
| D462 | 173 ns | 3.71 µs | 6.22 µs | 9.21 µs | 11.8 µs |
| D616 | 258 ns | 6.03 µs | 10.6 µs | 14.4 µs | 20.6 µs |
| D924 | 240 ns | 8.94 µs | 17 µs | 26.1 µs | 25.2 µs |
| D1232 | 207 ns | 15.6 µs | 20.6 µs | 39.6 µs | 62.4 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.8 88.2,174.1 124.4,123.4 160.5,117.3 196.7,127.9 232.9,126.1 269.1,122.7 305.3,121.1 341.5,120.4 377.6,113.5 413.8,114.8 450.0,117.4 450.0,18.2 413.8,33.9 377.6,37.5 341.5,47.1 305.3,55.4 269.1,65.2 232.9,72.9 196.7,81.2 160.5,93.5 124.4,94.6 88.2,69.8 52.0,150.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.8 88.2,174.1 124.4,123.4 160.5,117.3 196.7,127.9 232.9,126.1 269.1,122.7 305.3,121.1 341.5,120.4 377.6,113.5 413.8,114.8 450.0,117.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,163.2 88.2,160.5 124.4,118.4 160.5,120.9 196.7,109.1 232.9,89.5 269.1,80.8 305.3,78.2 341.5,67.2 377.6,58.8 413.8,51.9 450.0,42.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,158.0 88.2,150.2 124.4,109.3 160.5,102.0 196.7,90.1 232.9,89.6 269.1,74.2 305.3,67.2 341.5,58.3 377.6,48.9 413.8,40.8 450.0,37.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.7 88.2,83.2 124.4,100.6 160.5,92.9 196.7,82.8 232.9,79.0 269.1,71.1 305.3,62.8 341.5,51.4 377.6,43.7 413.8,33.3 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.5 88.2,69.8 124.4,94.6 160.5,93.5 196.7,81.2 232.9,72.9 269.1,65.2 305.3,55.4 341.5,47.1 377.6,37.5 413.8,33.9 450.0,18.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
