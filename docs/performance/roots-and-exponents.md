# Performance — Roots and Exponents

Speed of the root and exponential functions by storage width and scale. See the
[Performance overview](../performance.md) for the time units, the width reference
map, and how these timings are measured.

<!-- BEGIN GENERATED:performance:body:roots -->
### `cbrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 5.07 µs | 2.43 µs | 5.15 µs | 8.7 µs | 7.71 µs |
| D38 | 5.07 µs | 5.81 µs | 7.71 µs | 9.66 µs | 10.9 µs |
| D57 | 330 ns | 533 ns | 657 ns | 1.14 µs | 1.15 µs |
| D76 | 467 ns | 801 ns | 1.12 µs | 1.35 µs | 1.92 µs |
| D115 | 333 ns | 2.09 µs | 2.42 µs | 3.17 µs | 5.35 µs |
| D153 | 296 ns | 2.65 µs | 3.55 µs | 5.75 µs | 6.62 µs |
| D230 | 549 ns | 4.51 µs | 7.9 µs | 9.14 µs | 13.9 µs |
| D307 | 565 ns | 6.13 µs | 11.8 µs | 16.4 µs | 18.1 µs |
| D462 | 483 ns | 10.1 µs | 26 µs | 33.3 µs | 47.3 µs |
| D616 | 687 ns | 14.9 µs | 37.2 µs | 57.8 µs | 84.7 µs |
| D924 | 561 ns | 33.4 µs | 89.7 µs | 132 µs | 198 µs |
| D1232 | 1.22 µs | 48.7 µs | 138 µs | 227 µs | 313 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,124.7 88.2,124.7 124.4,184.1 160.5,176.5 196.7,183.9 232.9,186.4 269.1,173.0 305.3,172.4 341.5,175.8 377.6,168.2 413.8,172.5 450.0,155.6 450.0,35.2 413.8,45.2 377.6,63.6 341.5,76.3 305.3,97.1 269.1,102.8 232.9,119.0 196.7,123.6 160.5,145.8 124.4,157.1 88.2,108.1 52.0,115.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,124.7 88.2,124.7 124.4,184.1 160.5,176.5 196.7,183.9 232.9,186.4 269.1,173.0 305.3,172.4 341.5,175.8 377.6,168.2 413.8,172.5 450.0,155.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,140.7 88.2,121.8 124.4,173.7 160.5,164.8 196.7,144.0 232.9,138.8 269.1,127.3 305.3,120.6 341.5,109.7 377.6,101.3 413.8,83.8 450.0,75.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,124.4 88.2,115.6 124.4,169.1 160.5,157.5 196.7,140.8 232.9,132.5 269.1,115.1 305.3,106.5 341.5,89.2 377.6,81.5 413.8,62.4 450.0,53.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,113.0 88.2,110.8 124.4,157.2 160.5,153.4 196.7,134.9 232.9,122.0 269.1,112.0 305.3,99.2 341.5,83.9 377.6,71.9 413.8,53.9 450.0,42.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,115.7 88.2,108.1 124.4,157.1 160.5,145.8 196.7,123.6 232.9,119.0 269.1,102.8 305.3,97.1 341.5,76.3 377.6,63.6 413.8,45.2 450.0,35.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `exp`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 3.42 ns | 2.74 µs | 6.3 µs | 7.3 µs | 7.32 µs |
| D38 | 1.87 ns | 6.77 µs | 7.58 µs | 8.73 µs | 10.7 µs |
| D57 | 2.18 ns | 3.86 µs | 4.42 µs | 5.4 µs | 8.7 µs |
| D76 | 2.88 ns | 5.51 µs | 7.45 µs | 8.93 µs | 10.7 µs |
| D115 | 16.9 ns | 6.76 µs | 13.3 µs | 16.1 µs | 21.1 µs |
| D153 | 20 ns | 6.88 µs | 15.7 µs | 22.9 µs | 33.3 µs |
| D230 | 50.9 ns | 13.5 µs | 21.4 µs | 36 µs | 76.5 µs |
| D307 | 83.6 ns | 15 µs | 37.3 µs | 77.2 µs | 108 µs |
| D462 | 161 ns | 23 µs | 83.7 µs | 150 µs | 223 µs |
| D616 | 158 ns | 33.9 µs | 128 µs | 259 µs | 439 µs |
| D924 | 215 ns | 77.6 µs | 284 µs | 596 µs | 984 µs |
| D1232 | 403 ns | 139 µs | 404 µs | 905 µs | 2.7 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,194.7 88.2,202.2 124.4,200.3 160.5,196.9 196.7,174.9 232.9,172.8 269.1,161.2 305.3,155.1 341.5,147.0 377.6,147.2 413.8,143.3 450.0,135.6 450.0,26.2 413.8,38.8 377.6,48.8 341.5,57.2 305.3,66.2 269.1,70.5 232.9,80.8 196.7,86.4 160.5,94.8 124.4,97.4 88.2,94.8 52.0,99.6" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,194.7 88.2,202.2 124.4,200.3 160.5,196.9 196.7,174.9 232.9,172.8 269.1,161.2 305.3,155.1 341.5,147.0 377.6,147.2 413.8,143.3 450.0,135.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,111.8 88.2,100.5 124.4,107.5 160.5,103.1 196.7,100.6 232.9,100.4 269.1,92.0 305.3,90.7 341.5,85.4 377.6,80.5 413.8,70.3 450.0,63.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,101.5 88.2,99.2 124.4,105.8 160.5,99.4 196.7,92.2 232.9,90.1 269.1,86.3 305.3,79.4 341.5,69.3 377.6,64.1 413.8,54.2 450.0,49.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,97.4 124.4,103.4 160.5,97.1 196.7,89.8 232.9,85.4 269.1,79.8 305.3,70.3 341.5,62.1 377.6,55.3 413.8,45.0 450.0,39.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,99.6 88.2,94.8 124.4,97.4 160.5,94.8 196.7,86.4 232.9,80.8 269.1,70.5 305.3,66.2 341.5,57.2 377.6,48.8 413.8,38.8 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `hypot`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 16.8 ns | 37.9 ns | 41.4 ns | 40.3 ns | 41.4 ns |
| D38 | 16.1 ns | 42.9 ns | 67.2 ns | 67.2 ns | 108 ns |
| D57 | 16.9 ns | 42.2 ns | 72.9 ns | 691 ns | 708 ns |
| D76 | 18.7 ns | 68.4 ns | 593 ns | 701 ns | 1.04 µs |
| D115 | 20.5 ns | 79.2 ns | 620 ns | 922 ns | 1.38 µs |
| D153 | 19.7 ns | 706 ns | 914 ns | 1.39 µs | 2.15 µs |
| D230 | 29.3 ns | 716 ns | 1.54 µs | 1.76 µs | 3.28 µs |
| D307 | 42.1 ns | 1.07 µs | 2.1 µs | 3.3 µs | 5.13 µs |
| D462 | 84.5 ns | 1.5 µs | 3.6 µs | 6.24 µs | 9.57 µs |
| D616 | 68 ns | 2.5 µs | 6.02 µs | 10.8 µs | 15.4 µs |
| D924 | 129 ns | 3.71 µs | 11.4 µs | 24.9 µs | 28.4 µs |
| D1232 | 100 ns | 6.27 µs | 19.4 µs | 27.2 µs | 50 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,198.7 88.2,199.6 124.4,198.6 160.5,196.5 196.7,194.4 232.9,195.3 269.1,186.7 305.3,178.8 341.5,163.7 377.6,168.4 413.8,154.5 450.0,159.9 450.0,25.1 413.8,37.4 377.6,50.7 341.5,60.9 305.3,74.5 269.1,84.2 232.9,93.4 196.7,103.0 160.5,109.2 124.4,117.5 88.2,158.4 52.0,179.1" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,198.7 88.2,199.6 124.4,198.6 160.5,196.5 196.7,194.4 232.9,195.3 269.1,186.7 305.3,178.8 341.5,163.7 377.6,168.4 413.8,154.5 450.0,159.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,181.1 88.2,178.4 124.4,178.7 160.5,168.3 196.7,165.1 232.9,117.5 269.1,117.2 305.3,108.5 341.5,101.2 377.6,90.1 413.8,81.5 450.0,70.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,168.6 124.4,166.8 160.5,121.4 196.7,120.4 232.9,112.0 269.1,100.6 305.3,93.9 341.5,82.2 377.6,71.0 413.8,57.2 450.0,45.6" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.7 88.2,168.6 124.4,118.0 160.5,117.7 196.7,111.8 232.9,102.8 269.1,97.7 305.3,84.1 341.5,70.2 377.6,58.4 413.8,40.2 450.0,38.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,179.1 88.2,158.4 124.4,117.5 160.5,109.2 196.7,103.0 232.9,93.4 269.1,84.2 305.3,74.5 341.5,60.9 377.6,50.7 413.8,37.4 450.0,25.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `ln`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 4.36 ns | 274 ns | 367 ns | 398 ns | 383 ns |
| D38 | 2.18 ns | 392 ns | 373 ns | 374 ns | 405 ns |
| D57 | 265 ns | 467 ns | 487 ns | 440 ns | 551 ns |
| D76 | 270 ns | 440 ns | 466 ns | 549 ns | 552 ns |
| D115 | 282 ns | 476 ns | 635 ns | 759 ns | 938 ns |
| D153 | 284 ns | 465 ns | 681 ns | 1.07 µs | 1.18 µs |
| D230 | 502 ns | 602 ns | 932 ns | 1.21 µs | 1.64 µs |
| D307 | 641 ns | 653 ns | 1.15 µs | 1.25 µs | 8.18 µs |
| D462 | 1.27 µs | 2.71 µs | 3.64 µs | 3.78 µs | 4.73 µs |
| D616 | 1.33 µs | 1.35 µs | 1.69 µs | 2.64 µs | 3.58 µs |
| D924 | 2.05 µs | 1.86 µs | 2.94 µs | 3.51 µs | 4.69 µs |
| D1232 | 3 µs | 3.03 µs | 3.95 µs | 4.7 µs | 6.25 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="160.0" x2="450" y2="160.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="163.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="110.0" x2="450" y2="110.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="113.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="60.0" x2="450" y2="60.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="63.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,178.0 88.2,193.1 124.4,88.9 160.5,88.5 196.7,87.5 232.9,87.4 269.1,75.0 305.3,69.6 341.5,54.7 377.6,53.7 413.8,44.4 450.0,36.2 450.0,20.2 413.8,26.4 377.6,32.3 341.5,26.3 305.3,14.4 269.1,49.3 232.9,56.4 196.7,61.4 160.5,72.9 124.4,72.9 88.2,79.6 52.0,80.8" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,178.0 88.2,193.1 124.4,88.9 160.5,88.5 196.7,87.5 232.9,87.4 269.1,75.0 305.3,69.6 341.5,54.7 377.6,53.7 413.8,44.4 450.0,36.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,88.1 88.2,80.3 124.4,76.5 160.5,77.8 196.7,76.1 232.9,76.6 269.1,71.0 305.3,69.3 341.5,38.3 377.6,53.5 413.8,46.5 450.0,35.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,81.7 88.2,81.4 124.4,75.6 160.5,76.6 196.7,69.9 232.9,68.3 269.1,61.5 305.3,56.9 341.5,31.9 377.6,48.6 413.8,36.6 450.0,30.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.0 88.2,81.4 124.4,77.9 160.5,73.0 196.7,66.0 232.9,58.5 269.1,55.8 305.3,55.1 341.5,31.1 377.6,38.9 413.8,32.8 450.0,26.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,80.8 88.2,79.6 124.4,72.9 160.5,72.9 196.7,61.4 232.9,56.4 269.1,49.3 305.3,14.4 341.5,26.3 377.6,32.3 413.8,26.4 450.0,20.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `log`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 7.71 µs | 5.17 µs | 11.7 µs | 13.6 µs | 14 µs |
| D38 | 7.71 µs | 12.4 µs | 14.5 µs | 17.1 µs | 21.2 µs |
| D57 | 3.85 µs | 4.58 µs | 4.84 µs | 4.32 µs | 4.59 µs |
| D76 | 3.97 µs | 4.25 µs | 4.9 µs | 4.51 µs | 4.7 µs |
| D115 | 8.29 µs | 8.92 µs | 9.27 µs | 8.4 µs | 9.74 µs |
| D153 | 6.74 µs | 8.38 µs | 9.66 µs | 10.5 µs | 10.4 µs |
| D230 | 10.2 µs | 11.8 µs | 13.6 µs | 12.7 µs | 16.5 µs |
| D307 | 15.7 µs | 18.2 µs | 22.7 µs | 23.6 µs | 23.8 µs |
| D462 | 16.7 µs | 20.4 µs | 25.5 µs | 27.5 µs | 31 µs |
| D616 | 26.8 µs | 39.5 µs | 50.5 µs | 60.6 µs | 79.1 µs |
| D924 | 42.6 µs | 67.3 µs | 104 µs | 133 µs | 162 µs |
| D1232 | 54.1 µs | 114 µs | 154 µs | 201 µs | 250 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="143.3" x2="450" y2="143.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="146.3" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="76.7" x2="450" y2="76.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="79.7" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,150.9 88.2,150.9 124.4,171.0 160.5,170.1 196.7,148.8 232.9,154.7 269.1,142.7 305.3,130.4 341.5,128.5 377.6,114.8 413.8,101.4 450.0,94.4 450.0,50.2 413.8,62.8 377.6,83.5 341.5,110.6 305.3,118.2 269.1,128.8 232.9,142.1 196.7,144.1 160.5,165.2 124.4,165.9 88.2,121.6 52.0,133.7" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,150.9 88.2,150.9 124.4,171.0 160.5,170.1 196.7,148.8 232.9,154.7 269.1,142.7 305.3,130.4 341.5,128.5 377.6,114.8 413.8,101.4 450.0,94.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,162.4 88.2,137.1 124.4,165.9 160.5,168.1 196.7,146.7 232.9,148.5 269.1,138.6 305.3,126.0 341.5,122.6 377.6,103.6 413.8,88.1 450.0,72.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,138.8 88.2,132.5 124.4,164.3 160.5,164.0 196.7,145.5 232.9,144.3 269.1,134.4 305.3,119.6 341.5,116.3 377.6,96.4 413.8,75.6 450.0,64.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,134.5 88.2,127.8 124.4,167.6 160.5,166.4 196.7,148.4 232.9,141.8 269.1,136.5 305.3,118.5 341.5,114.1 377.6,91.1 413.8,68.5 450.0,56.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,133.7 88.2,121.6 124.4,165.9 160.5,165.2 196.7,144.1 232.9,142.1 269.1,128.8 305.3,118.2 341.5,110.6 377.6,83.5 413.8,62.8 450.0,50.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `powf`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 8.71 ns | 3.18 µs | 6.93 µs | 8.01 µs | 8 µs |
| D38 | 6.54 ns | 7.45 µs | 8.22 µs | 9.37 µs | 11.5 µs |
| D57 | 56.8 ns | 4.46 µs | 4.57 µs | 4.49 µs | 4.94 µs |
| D76 | 65.2 ns | 3.88 µs | 4.92 µs | 4.96 µs | 5.33 µs |
| D115 | 148 ns | 8.71 µs | 9.65 µs | 9.19 µs | 10.6 µs |
| D153 | 152 ns | 8.44 µs | 10.3 µs | 11.6 µs | 11.9 µs |
| D230 | 280 ns | 12.1 µs | 14.8 µs | 15.1 µs | 20.5 µs |
| D307 | 371 ns | 18.8 µs | 22.7 µs | 28 µs | 29.7 µs |
| D462 | 714 ns | 69.5 µs | 147 µs | 203 µs | 257 µs |
| D616 | 786 ns | 177 µs | 326 µs | 348 µs | 566 µs |
| D924 | 1.06 µs | 420 µs | 481 µs | 841 µs | 1.74 ms |
| D1232 | 1.41 µs | 829 µs | 797 µs | 2.12 ms | 2.74 ms |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="181.4" x2="450" y2="181.4" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="184.4" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="152.9" x2="450" y2="152.9" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="155.9" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="124.3" x2="450" y2="124.3" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="127.3" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="95.7" x2="450" y2="95.7" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="98.7" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="67.1" x2="450" y2="67.1" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="70.1" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><line x1="52" y1="38.6" x2="450" y2="38.6" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="41.6" text-anchor="end" font-size="9" fill="currentColor">1 ms</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">10 ms</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,158.2 196.7,148.0 232.9,147.7 269.1,140.1 305.3,136.6 341.5,128.5 377.6,127.3 413.8,123.6 450.0,120.0 450.0,26.1 413.8,31.7 377.6,45.6 341.5,55.4 305.3,82.2 269.1,86.8 232.9,93.6 196.7,95.0 160.5,103.5 124.4,104.5 88.2,94.0 52.0,98.5" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,183.1 88.2,186.7 124.4,159.9 160.5,158.2 196.7,148.0 232.9,147.7 269.1,140.1 305.3,136.6 341.5,128.5 377.6,127.3 413.8,123.6 450.0,120.0" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,109.9 88.2,99.4 124.4,105.7 160.5,107.5 196.7,97.4 232.9,97.8 269.1,93.4 305.3,87.9 341.5,71.7 377.6,60.1 413.8,49.3 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,100.3 88.2,98.1 124.4,105.4 160.5,104.5 196.7,96.2 232.9,95.4 269.1,90.8 305.3,85.6 341.5,62.4 377.6,52.5 413.8,47.7 450.0,41.4" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,96.5 124.4,105.6 160.5,104.4 196.7,96.8 232.9,93.9 269.1,90.6 305.3,82.9 341.5,58.3 377.6,51.7 413.8,40.7 450.0,29.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,98.5 88.2,94.0 124.4,104.5 160.5,103.5 196.7,95.0 232.9,93.6 269.1,86.8 305.3,82.2 341.5,55.4 377.6,45.6 413.8,31.7 450.0,26.1" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>

### `sqrt`

<div class="grid perf-grid" markdown>

| Width | 0 | ¼ | ½ | ¾ | max |
| :-- | --: | --: | --: | --: | --: |
| D18 | 6.52 ns | 16.6 ns | 18.9 ns | 30.5 ns | 29.9 ns |
| D38 | 7.73 ns | 36.5 ns | 32.4 ns | 1.47 µs | 3.19 µs |
| D57 | 159 ns | 223 ns | 429 ns | 766 ns | 762 ns |
| D76 | 196 ns | 271 ns | 655 ns | 834 ns | 1.21 µs |
| D115 | 134 ns | 598 ns | 937 ns | 1.21 µs | 1.68 µs |
| D153 | 107 ns | 1.09 µs | 1.52 µs | 1.83 µs | 2.65 µs |
| D230 | 145 ns | 1.56 µs | 2.33 µs | 2.78 µs | 4.14 µs |
| D307 | 148 ns | 2.34 µs | 3.64 µs | 4.7 µs | 6.18 µs |
| D462 | 211 ns | 3.67 µs | 6.08 µs | 9.42 µs | 11.9 µs |
| D616 | 231 ns | 5.86 µs | 10.5 µs | 14.1 µs | 20.4 µs |
| D924 | 239 ns | 10.6 µs | 16.9 µs | 26.7 µs | 35.2 µs |
| D1232 | 258 ns | 16.8 µs | 27.7 µs | 39.4 µs | 60.3 µs |

<figure>
<svg viewBox="0 0 460 240" width="100%" style="height:auto;color:var(--md-default-fg-color--light)" xmlns="http://www.w3.org/2000/svg"><line x1="52" y1="210.0" x2="450" y2="210.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="213.0" text-anchor="end" font-size="9" fill="currentColor">1 ns</text><line x1="52" y1="170.0" x2="450" y2="170.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="173.0" text-anchor="end" font-size="9" fill="currentColor">10 ns</text><line x1="52" y1="130.0" x2="450" y2="130.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="133.0" text-anchor="end" font-size="9" fill="currentColor">100 ns</text><line x1="52" y1="90.0" x2="450" y2="90.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="93.0" text-anchor="end" font-size="9" fill="currentColor">1 µs</text><line x1="52" y1="50.0" x2="450" y2="50.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="53.0" text-anchor="end" font-size="9" fill="currentColor">10 µs</text><line x1="52" y1="10.0" x2="450" y2="10.0" stroke="currentColor" stroke-opacity="0.15"/><text x="46" y="13.0" text-anchor="end" font-size="9" fill="currentColor">100 µs</text><text x="52.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">18</text><text x="88.2" y="222" text-anchor="middle" font-size="8" fill="currentColor">38</text><text x="124.4" y="222" text-anchor="middle" font-size="8" fill="currentColor">57</text><text x="160.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">76</text><text x="196.7" y="222" text-anchor="middle" font-size="8" fill="currentColor">115</text><text x="232.9" y="222" text-anchor="middle" font-size="8" fill="currentColor">153</text><text x="269.1" y="222" text-anchor="middle" font-size="8" fill="currentColor">230</text><text x="305.3" y="222" text-anchor="middle" font-size="8" fill="currentColor">307</text><text x="341.5" y="222" text-anchor="middle" font-size="8" fill="currentColor">462</text><text x="377.6" y="222" text-anchor="middle" font-size="8" fill="currentColor">616</text><text x="413.8" y="222" text-anchor="middle" font-size="8" fill="currentColor">924</text><text x="450.0" y="222" text-anchor="middle" font-size="8" fill="currentColor">1232</text><polygon points="52.0,177.4 88.2,174.5 124.4,121.9 160.5,118.3 196.7,125.0 232.9,128.8 269.1,123.6 305.3,123.2 341.5,117.0 377.6,115.4 413.8,114.9 450.0,113.5 450.0,18.8 413.8,28.2 377.6,37.6 341.5,47.0 305.3,58.4 269.1,65.3 232.9,73.1 196.7,81.0 160.5,86.6 124.4,94.7 88.2,69.8 52.0,151.0" fill="var(--md-primary-fg-color)" fill-opacity="0.10"/><polyline points="52.0,177.4 88.2,174.5 124.4,121.9 160.5,118.3 196.7,125.0 232.9,128.8 269.1,123.6 305.3,123.2 341.5,117.0 377.6,115.4 413.8,114.9 450.0,113.5" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><polyline points="52.0,161.2 88.2,147.5 124.4,116.0 160.5,112.7 196.7,98.9 232.9,88.4 269.1,82.3 305.3,75.3 341.5,67.4 377.6,59.3 413.8,49.1 450.0,40.9" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,159.0 88.2,149.6 124.4,104.7 160.5,97.3 196.7,91.1 232.9,82.7 269.1,75.3 305.3,67.6 341.5,58.6 377.6,49.1 413.8,40.9 450.0,32.3" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,150.6 88.2,83.3 124.4,94.6 160.5,93.2 196.7,86.6 232.9,79.5 269.1,72.3 305.3,63.1 341.5,51.0 377.6,44.0 413.8,32.9 450.0,26.2" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.0" stroke-dasharray="3 3"/><polyline points="52.0,151.0 88.2,69.8 124.4,94.7 160.5,86.6 196.7,81.0 232.9,73.1 269.1,65.3 305.3,58.4 341.5,47.0 377.6,37.6 413.8,28.2 450.0,18.8" fill="none" stroke="var(--md-primary-fg-color)" stroke-width="1.6"/><line x1="52" y1="10" x2="52" y2="210" stroke="currentColor" stroke-opacity="0.4"/><line x1="52" y1="210" x2="450" y2="210" stroke="currentColor" stroke-opacity="0.4"/></svg>
<figcaption>Median time vs width (log scale). Solid: scale 0 and max; dashed: the intermediate band-edge scales.</figcaption>
</figure>

</div>
<!-- END GENERATED:performance:body:roots -->
